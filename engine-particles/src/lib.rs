//! engine-particles crate - 粒子系统与后期特效栈
//!
//! 提供完整的粒子系统（ParticleSystem、ParticleEmitter、ParticleModule）
//! 和后期特效栈（PostProcessingStack、Bloom、DOF、SSAO、SSR等）。
//!
//! # 核心模块
//!
//! ## 粒子系统
//! - [`Particle`] - 单个粒子数据结构
//! - [`ParticlePool`] - 粒子池（SoA 结构优化）
//! - [`ParticleSystem`] - 粒子系统管理器
//! - [`ParticleEmitter`] - 粒子发射器
//! - [`EmitShape`] - 发射形状（Point、Box、Sphere、Cone等）
//! - [`ParticleModule`] - 粒子模块trait（Gravity、Force、Color等）
//!
//! ## 后期特效
//! - [`PostProcessingStack`] - 后期特效栈
//! - [`IPostProcessPass`] - 后期特效Pass trait
//! - [`BloomPass`] - Bloom发光效果
//! - [`DOFPass`] - 景深效果
//! - [`SSAOPass`] - 屏幕空间环境光遮蔽
//! - [`SSRPass`] - 屏幕空间反射
//! - [`ToneMappingPass`] - 色调映射
//! - [`VignettePass`] - 暗角效果
//! - [`ChromaticAberrationPass`] - 色差效果

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use core::fmt;
use engine_math::{Vec2, Vec3};

// ============================================================================
// 常量定义
// ============================================================================

/// 默认最大粒子数
const DEFAULT_MAX_PARTICLES: usize = 1000;

/// 默认重力加速度 (m/s²)
const DEFAULT_GRAVITY: f32 = -9.8;

/// 默认粒子生命周期 (秒)
const DEFAULT_LIFETIME: f32 = 1.0;

/// 默认发射速率 (粒子/秒)
const DEFAULT_EMISSION_RATE: f32 = 10.0;

/// SSAO 默认采样核大小
const SSAO_DEFAULT_KERNEL_SIZE: u32 = 32;

/// SSAO 默认半径
const SSAO_DEFAULT_RADIUS: f32 = 0.5;

/// Bloom 默认阈值
const BLOOM_DEFAULT_THRESHOLD: f32 = 1.0;

/// Bloom 默认强度
const BLOOM_DEFAULT_INTENSITY: f32 = 0.5;

// ============================================================================
// 颜色结构
// ============================================================================

/// RGBA颜色（线性空间）
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }
}

// ============================================================================
// 粒子核心数据结构
// ============================================================================

/// 单个粒子数据
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Particle {
    /// 位置
    pub position: Vec3,
    /// 速度
    pub velocity: Vec3,
    /// 旋转角度（2D为z轴旋转，3D可用Quat）
    pub rotation: f32,
    /// 尺寸
    pub size: Vec2,
    /// 颜色
    pub color: Rgba,
    /// 当前年龄
    pub age: f32,
    /// 总生命周期
    pub lifetime: f32,
}

impl Particle {
    /// 创建新粒子
    #[inline]
    pub fn new(
        position: Vec3,
        velocity: Vec3,
        rotation: f32,
        size: Vec2,
        color: Rgba,
        lifetime: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            rotation,
            size,
            color,
            age: 0.0,
            lifetime,
        }
    }

    /// 粒子是否存活
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.age < self.lifetime
    }

    /// 获取归一化年龄 (0.0 ~ 1.0)
    #[inline]
    pub fn normalized_age(&self) -> f32 {
        if self.lifetime > 0.0 {
            (self.age / self.lifetime).min(1.0)
        } else {
            1.0
        }
    }

    /// 更新粒子状态
    #[inline]
    pub fn update(&mut self, dt: f32) {
        self.position = self.position + self.velocity * dt;
        self.age += dt;
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            rotation: 0.0,
            size: Vec2::ONE,
            color: Rgba::WHITE,
            age: 0.0,
            lifetime: DEFAULT_LIFETIME,
        }
    }
}

// ============================================================================
// 粒子池
// ============================================================================

/// 粒子池（紧凑存储，支持高效spawn/kill）
pub struct ParticlePool {
    /// 粒子数据数组
    particles: Vec<Particle>,
    /// 存活粒子数量
    alive_count: usize,
    /// 最大容量
    max_count: usize,
}

impl ParticlePool {
    /// 创建新粒子池
    pub fn new(max_count: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_count),
            alive_count: 0,
            max_count,
        }
    }

    /// 尝试生成新粒子
    pub fn spawn(&mut self, particle: Particle) -> bool {
        if self.alive_count >= self.max_count {
            return false;
        }
        if self.particles.len() > self.alive_count {
            self.particles[self.alive_count] = particle;
        } else {
            self.particles.push(particle);
        }
        self.alive_count += 1;
        true
    }

    /// 杀死指定索引的粒子（swap-remove）
    pub fn kill(&mut self, index: usize) {
        if index < self.alive_count {
            self.alive_count -= 1;
            if index != self.alive_count {
                self.particles.swap(index, self.alive_count);
            }
        }
    }

    /// 存活粒子数量
    #[inline]
    pub fn alive_count(&self) -> usize {
        self.alive_count
    }

    /// 死亡粒子数量
    #[inline]
    pub fn dead_count(&self) -> usize {
        self.max_count - self.alive_count
    }

    /// 最大容量
    #[inline]
    pub fn max_count(&self) -> usize {
        self.max_count
    }

    /// 获取存活粒子切片
    #[inline]
    pub fn alive_particles(&self) -> &[Particle] {
        &self.particles[..self.alive_count]
    }

    /// 获取可变存活粒子切片
    #[inline]
    pub fn alive_particles_mut(&mut self) -> &mut [Particle] {
        &mut self.particles[..self.alive_count]
    }

    /// 清空所有粒子
    pub fn clear(&mut self) {
        self.alive_count = 0;
    }
}

// ============================================================================
// 发射形状
// ============================================================================

/// 球体发射模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SphereEmitMode {
    /// 从体积内部发射
    Volume,
    /// 从表面发射
    Shell,
}

/// 圆锥发射模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConeEmitMode {
    /// 从底面发射
    Base,
    /// 从体积内部发射
    Volume,
    /// 从外壳发射
    Shell,
}

/// 发射形状
#[derive(Clone, Debug)]
pub enum EmitShape {
    /// 点发射
    Point,
    /// 盒子发射 (size)
    Box(Vec3),
    /// 球体发射 (radius, mode)
    Sphere(f32, SphereEmitMode),
    /// 半球体发射 (radius)
    Hemisphere(f32),
    /// 圆锥发射 (angle, base_radius, length, mode)
    Cone(f32, f32, f32, ConeEmitMode),
    /// 圆形发射 (radius, axis)
    Circle(f32, Vec3),
    /// 边缘发射 (start, end)
    Edge(Vec3, Vec3),
}

impl Default for EmitShape {
    fn default() -> Self {
        Self::Point
    }
}

/// 简单随机数生成器trait
pub trait Rng {
    fn range(&mut self, min: f32, max: f32) -> f32;
}

/// 简单线性同余随机数生成器
#[derive(Clone)]
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
}

impl Rng for SimpleRng {
    fn range(&mut self, min: f32, max: f32) -> f32 {
        let u = self.next();
        let f = (u as f32) / (u64::MAX as f32);
        min + f * (max - min)
    }
}

impl Default for SimpleRng {
    fn default() -> Self {
        Self::new(12345)
    }
}

/// 随机方向生成
fn random_direction(rng: &mut impl Rng) -> Vec3 {
    let theta = rng.range(0.0, core::f32::consts::TAU);
    let phi = rng.range(-1.0, 1.0);
    let phi_angle = phi.acos();
    Vec3::new(
        phi_angle.sin() * theta.cos(),
        phi_angle.cos(),
        phi_angle.sin() * theta.sin(),
    )
}

impl EmitShape {
    /// 在形状内采样位置
    pub fn sample_position(&self, rng: &mut impl Rng) -> Vec3 {
        match self {
            Self::Point => Vec3::ZERO,
            Self::Box(size) => Vec3::new(
                rng.range(-size.x / 2.0, size.x / 2.0),
                rng.range(-size.y / 2.0, size.y / 2.0),
                rng.range(-size.z / 2.0, size.z / 2.0),
            ),
            Self::Sphere(radius, mode) => {
                let dir = random_direction(rng);
                let r = *radius;
                match mode {
                    SphereEmitMode::Volume => dir * rng.range(0.0, r),
                    SphereEmitMode::Shell => dir * r,
                }
            }
            Self::Hemisphere(radius) => {
                let dir = random_direction(rng);
                let hemi_dir = Vec3::new(dir.x, dir.y.abs(), dir.z).normalize();
                let r = *radius;
                hemi_dir * rng.range(0.0, r)
            }
            Self::Cone(angle, base_radius, length, mode) => {
                let angle_rad = angle.to_radians();
                let br = *base_radius;
                let len = *length;
                match mode {
                    ConeEmitMode::Base => {
                        let r = rng.range(0.0, br);
                        let theta = rng.range(0.0, core::f32::consts::TAU);
                        Vec3::new(r * theta.cos(), 0.0, r * theta.sin())
                    }
                    ConeEmitMode::Volume | ConeEmitMode::Shell => {
                        let t = rng.range(0.0, 1.0);
                        let y = t * len;
                        let r_at_y = br + y * angle_rad.tan();
                        let r = if *mode == ConeEmitMode::Shell {
                            r_at_y
                        } else {
                            rng.range(0.0, r_at_y)
                        };
                        let theta = rng.range(0.0, core::f32::consts::TAU);
                        Vec3::new(r * theta.cos(), y, r * theta.sin())
                    }
                }
            }
            Self::Circle(radius, axis) => {
                let theta = rng.range(0.0, core::f32::consts::TAU);
                let r = rng.range(0.0, *radius);
                if axis.y > 0.5 {
                    Vec3::new(r * theta.cos(), 0.0, r * theta.sin())
                } else if axis.x > 0.5 {
                    Vec3::new(0.0, r * theta.cos(), r * theta.sin())
                } else {
                    Vec3::new(r * theta.cos(), r * theta.sin(), 0.0)
                }
            }
            Self::Edge(start, end) => {
                let t = rng.range(0.0, 1.0);
                start.lerp(*end, t)
            }
        }
    }

    /// 采样初始方向
    pub fn sample_direction(&self, rng: &mut impl Rng, position: Vec3) -> Vec3 {
        match self {
            Self::Point => random_direction(rng),
            Self::Box(_) => random_direction(rng),
            Self::Sphere(_, SphereEmitMode::Shell) => position.normalize(),
            Self::Sphere(_, SphereEmitMode::Volume) => random_direction(rng),
            Self::Hemisphere(_) => {
                let dir = random_direction(rng);
                Vec3::new(dir.x, dir.y.abs(), dir.z).normalize()
            }
            Self::Cone(angle, _, _, _) => {
                let base_dir = Vec3::Y;
                let spread = angle.to_radians() / 2.0;
                let offset = random_direction(rng);
                let spread_dir = Vec3::new(
                    offset.x * spread.sin(),
                    base_dir.y + offset.y * spread.cos(),
                    offset.z * spread.sin(),
                );
                spread_dir.normalize()
            }
            Self::Circle(_, axis) => axis.normalize(),
            Self::Edge(_, _) => random_direction(rng),
        }
    }
}

// ============================================================================
// 发射模式
// ============================================================================

/// 爆发配置
#[derive(Clone, Debug)]
pub struct BurstConfig {
    /// 触发时间
    pub time: f32,
    /// 粒子数量
    pub count: u32,
    /// 循环次数
    pub cycles: u32,
    /// 循环间隔
    pub interval: f32,
    /// 已触发次数
    triggered_count: u32,
}

impl BurstConfig {
    pub fn new(time: f32, count: u32, cycles: u32, interval: f32) -> Self {
        Self {
            time,
            count,
            cycles,
            interval,
            triggered_count: 0,
        }
    }

    /// 检查是否应该触发
    pub fn should_fire(&mut self, current_time: f32) -> bool {
        if self.triggered_count >= self.cycles {
            return false;
        }
        let trigger_time = self.time + self.triggered_count as f32 * self.interval;
        if current_time >= trigger_time {
            self.triggered_count += 1;
            return true;
        }
        false
    }

    /// 重置触发状态
    pub fn reset(&mut self) {
        self.triggered_count = 0;
    }
}

impl Default for BurstConfig {
    fn default() -> Self {
        Self::new(0.0, 10, 1, 0.0)
    }
}

/// 发射模式
#[derive(Clone, Debug)]
pub enum EmissionMode {
    /// 持续发射 (rate: 粒子/秒)
    Continuous(f32),
    /// 爆发发射
    Burst(Vec<BurstConfig>),
    /// 混合模式 (rate + bursts)
    Mixed(f32, Vec<BurstConfig>),
}

impl Default for EmissionMode {
    fn default() -> Self {
        Self::Continuous(DEFAULT_EMISSION_RATE)
    }
}

// ============================================================================
// 粒子渲染模式
// ============================================================================

/// 粒子渲染模式
#[derive(Clone, Debug, PartialEq)]
pub enum ParticleRenderMode {
    /// Sprite Billboard（始终面向相机）
    SpriteBillboard,
    /// Mesh Billboard（使用mesh，整体朝向相机）
    MeshBillboard,
    /// 拉伸Billboard（沿速度方向拉伸）
    StretchedBillboard(f32, f32), // length_scale, speed_scale
    /// 水平Billboard（绕Y轴对齐）
    HorizontalBillboard,
    /// 垂直Billboard（绕相机up对齐）
    VerticalBillboard,
}

impl Default for ParticleRenderMode {
    fn default() -> Self {
        Self::SpriteBillboard
    }
}

/// 粒子材质混合模式
#[derive(Clone, Debug, PartialEq)]
pub enum ParticleBlendMode {
    Opaque,
    Masked,
    Translucent,
    Additive,
}

impl Default for ParticleBlendMode {
    fn default() -> Self {
        Self::Translucent
    }
}

// ============================================================================
// 模块系统
// ============================================================================

/// 模块上下文
pub struct ModuleContext {
    /// 时间增量
    pub dt: f32,
    /// 总时间
    pub total_time: f32,
    /// 发射器变换
    pub emitter_position: Vec3,
    /// 随机数生成器
    pub rng: SimpleRng,
}

impl ModuleContext {
    pub fn new(dt: f32, total_time: f32, emitter_position: Vec3) -> Self {
        Self {
            dt,
            total_time,
            emitter_position,
            rng: SimpleRng::default(),
        }
    }
}

/// 粒子模块trait
pub trait ParticleModule: fmt::Debug {
    /// 模块优先级（执行顺序）
    fn priority(&self) -> i32 {
        0
    }

    /// 应用模块效果
    fn apply(&self, particle: &mut Particle, dt: f32, ctx: &mut ModuleContext);
}

// ============================================================================
// 具体模块实现
// ============================================================================

/// 初始速度模块
#[derive(Clone, Debug)]
pub struct InitialVelocityModule {
    min_speed: f32,
    max_speed: f32,
}

impl InitialVelocityModule {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min_speed: min,
            max_speed: max,
        }
    }

    pub fn speed(&self) -> (f32, f32) {
        (self.min_speed, self.max_speed)
    }
}

impl ParticleModule for InitialVelocityModule {
    fn priority(&self) -> i32 {
        -100
    }
    fn apply(&self, particle: &mut Particle, _dt: f32, ctx: &mut ModuleContext) {
        let speed = ctx.rng.range(self.min_speed, self.max_speed);
        particle.velocity = particle.velocity.normalize() * speed;
    }
}

/// 速度随生命周期变化模块
#[derive(Clone, Debug)]
pub struct VelocityOverLifeModule {
    /// 速度曲线 (key: normalized_age, value: velocity multiplier)
    curve: Vec<(f32, f32)>,
}

impl VelocityOverLifeModule {
    pub fn new(curve: Vec<(f32, f32)>) -> Self {
        Self { curve }
    }

    fn sample(&self, t: f32) -> f32 {
        if self.curve.is_empty() {
            return 1.0;
        }
        let t_clamped = t.clamp(0.0, 1.0);
        for i in 0..self.curve.len() - 1 {
            if t_clamped >= self.curve[i].0 && t_clamped <= self.curve[i + 1].0 {
                let t0 = self.curve[i].0;
                let t1 = self.curve[i + 1].0;
                let v0 = self.curve[i].1;
                let v1 = self.curve[i + 1].1;
                let blend = (t_clamped - t0) / (t1 - t0);
                return v0 + (v1 - v0) * blend;
            }
        }
        self.curve.last().map(|(_, v)| v).copied().unwrap_or(1.0)
    }
}

impl ParticleModule for VelocityOverLifeModule {
    fn priority(&self) -> i32 {
        10
    }
    fn apply(&self, particle: &mut Particle, _dt: f32, _ctx: &mut ModuleContext) {
        let t = particle.normalized_age();
        let multiplier = self.sample(t);
        particle.velocity = particle.velocity * multiplier;
    }
}

/// 颜色随生命周期变化模块
#[derive(Clone, Debug)]
pub struct ColorOverLifeModule {
    /// 颜色渐变
    gradient: Vec<(f32, Rgba)>,
}

impl ColorOverLifeModule {
    pub fn new(gradient: Vec<(f32, Rgba)>) -> Self {
        Self { gradient }
    }

    fn sample(&self, t: f32) -> Rgba {
        if self.gradient.is_empty() {
            return Rgba::WHITE;
        }
        let t_clamped = t.clamp(0.0, 1.0);
        for i in 0..self.gradient.len() - 1 {
            if t_clamped >= self.gradient[i].0 && t_clamped <= self.gradient[i + 1].0 {
                let t0 = self.gradient[i].0;
                let t1 = self.gradient[i + 1].0;
                let c0 = self.gradient[i].1;
                let c1 = self.gradient[i + 1].1;
                let blend = (t_clamped - t0) / (t1 - t0);
                return c0.lerp(c1, blend);
            }
        }
        self.gradient
            .last()
            .map(|(_, c)| c)
            .copied()
            .unwrap_or(Rgba::WHITE)
    }
}

impl ParticleModule for ColorOverLifeModule {
    fn priority(&self) -> i32 {
        20
    }
    fn apply(&self, particle: &mut Particle, _dt: f32, _ctx: &mut ModuleContext) {
        let t = particle.normalized_age();
        particle.color = self.sample(t);
    }
}

/// 尺寸随生命周期变化模块
#[derive(Clone, Debug)]
pub struct SizeOverLifeModule {
    /// 尺寸曲线
    curve: Vec<(f32, f32)>,
}

impl SizeOverLifeModule {
    pub fn new(curve: Vec<(f32, f32)>) -> Self {
        Self { curve }
    }

    fn sample(&self, t: f32) -> f32 {
        if self.curve.is_empty() {
            return 1.0;
        }
        let t_clamped = t.clamp(0.0, 1.0);
        for i in 0..self.curve.len() - 1 {
            if t_clamped >= self.curve[i].0 && t_clamped <= self.curve[i + 1].0 {
                let t0 = self.curve[i].0;
                let t1 = self.curve[i + 1].0;
                let v0 = self.curve[i].1;
                let v1 = self.curve[i + 1].1;
                let blend = (t_clamped - t0) / (t1 - t0);
                return v0 + (v1 - v0) * blend;
            }
        }
        self.curve.last().map(|(_, v)| v).copied().unwrap_or(1.0)
    }
}

impl ParticleModule for SizeOverLifeModule {
    fn priority(&self) -> i32 {
        30
    }
    fn apply(&self, particle: &mut Particle, _dt: f32, _ctx: &mut ModuleContext) {
        let t = particle.normalized_age();
        let scale = self.sample(t);
        particle.size = particle.size * scale;
    }
}

/// 旋转随生命周期变化模块
#[derive(Clone, Debug)]
pub struct RotationOverLifeModule {
    /// 旋转速度 (弧度/秒)
    rotation_speed: f32,
}

impl RotationOverLifeModule {
    pub fn new(rotation_speed: f32) -> Self {
        Self { rotation_speed }
    }
}

impl ParticleModule for RotationOverLifeModule {
    fn priority(&self) -> i32 {
        40
    }
    fn apply(&self, particle: &mut Particle, dt: f32, _ctx: &mut ModuleContext) {
        particle.rotation += self.rotation_speed * dt;
    }
}

/// 力模块
#[derive(Clone, Debug)]
pub struct ForceModule {
    force: Vec3,
}

impl ForceModule {
    pub fn new(force: Vec3) -> Self {
        Self { force }
    }

    pub fn force(&self) -> Vec3 {
        self.force
    }
}

impl ParticleModule for ForceModule {
    fn priority(&self) -> i32 {
        50
    }
    fn apply(&self, particle: &mut Particle, dt: f32, _ctx: &mut ModuleContext) {
        particle.velocity = particle.velocity + self.force * dt;
    }
}

/// 重力模块
#[derive(Clone, Debug)]
pub struct GravityModule {
    gravity: f32,
}

impl GravityModule {
    pub fn new(gravity: f32) -> Self {
        Self { gravity }
    }

    pub fn default_gravity() -> Self {
        Self::new(DEFAULT_GRAVITY)
    }
}

impl Default for GravityModule {
    fn default() -> Self {
        Self::default_gravity()
    }
}

impl ParticleModule for GravityModule {
    fn priority(&self) -> i32 {
        51
    }
    fn apply(&self, particle: &mut Particle, dt: f32, _ctx: &mut ModuleContext) {
        particle.velocity.y += self.gravity * dt;
    }
}

/// 阻力模块
#[derive(Clone, Debug)]
pub struct DragModule {
    drag: f32,
}

impl DragModule {
    pub fn new(drag: f32) -> Self {
        Self { drag }
    }

    pub fn drag(&self) -> f32 {
        self.drag
    }
}

impl ParticleModule for DragModule {
    fn priority(&self) -> i32 {
        60
    }
    fn apply(&self, particle: &mut Particle, dt: f32, _ctx: &mut ModuleContext) {
        let drag_factor = 1.0 - self.drag * dt;
        particle.velocity = particle.velocity * drag_factor.max(0.0);
    }
}

/// 湍流模块
#[derive(Clone, Debug)]
pub struct TurbulenceModule {
    intensity: f32,
    frequency: f32,
}

impl TurbulenceModule {
    pub fn new(intensity: f32, frequency: f32) -> Self {
        Self {
            intensity,
            frequency,
        }
    }
}

impl ParticleModule for TurbulenceModule {
    fn priority(&self) -> i32 {
        70
    }
    fn apply(&self, particle: &mut Particle, dt: f32, ctx: &mut ModuleContext) {
        let noise_x = ctx.rng.range(-1.0, 1.0) * self.intensity;
        let noise_y = ctx.rng.range(-1.0, 1.0) * self.intensity;
        let noise_z = ctx.rng.range(-1.0, 1.0) * self.intensity;
        let noise = Vec3::new(noise_x, noise_y, noise_z) * self.frequency;
        particle.velocity = particle.velocity + noise * dt;
    }
}

/// 吸引器模块
#[derive(Clone, Debug)]
pub struct AttractorModule {
    center: Vec3,
    strength: f32,
    falloff_radius: f32,
}

impl AttractorModule {
    pub fn new(center: Vec3, strength: f32, falloff_radius: f32) -> Self {
        Self {
            center,
            strength,
            falloff_radius,
        }
    }

    pub fn attract(&self, position: Vec3) -> Vec3 {
        let dir = self.center - position;
        let dist = dir.length();
        if dist < 0.001 {
            return Vec3::ZERO;
        }
        let falloff = if dist < self.falloff_radius {
            dist / self.falloff_radius
        } else {
            1.0 / (dist / self.falloff_radius)
        };
        dir.normalize() * self.strength * falloff
    }
}

impl ParticleModule for AttractorModule {
    fn priority(&self) -> i32 {
        80
    }
    fn apply(&self, particle: &mut Particle, dt: f32, _ctx: &mut ModuleContext) {
        let force = self.attract(particle.position);
        particle.velocity = particle.velocity + force * dt;
    }
}

/// 碰撞器类型
#[derive(Clone, Debug)]
pub enum ParticleCollider {
    Plane(Vec3, f32),  // normal, offset
    Sphere(Vec3, f32), // center, radius
    Box(Vec3, Vec3),   // center, half_size
}

/// 碰撞模块
#[derive(Clone, Debug)]
pub struct CollisionModule {
    colliders: Vec<ParticleCollider>,
    bounce: f32,
    friction: f32,
    #[allow(dead_code)]
    kill_threshold: f32,
}

impl CollisionModule {
    pub fn new(colliders: Vec<ParticleCollider>) -> Self {
        Self {
            colliders,
            bounce: 0.5,
            friction: 0.1,
            kill_threshold: 0.01,
        }
    }

    pub fn colliders(&self) -> &[ParticleCollider] {
        &self.colliders
    }

    fn collide_plane(&self, particle: &mut Particle, normal: Vec3, offset: f32) -> bool {
        let dist = particle.position.dot(normal) - offset;
        if dist < 0.0 {
            let dot = particle.velocity.dot(normal);
            if dot < 0.0 {
                particle.velocity = particle.velocity - normal * (2.0 * dot);
                particle.velocity = particle.velocity * self.bounce;
                let tangent_vel = particle.velocity - normal * particle.velocity.dot(normal);
                particle.velocity = particle.velocity - tangent_vel * self.friction;
                particle.position = particle.position + normal * (-dist);
                return true;
            }
        }
        false
    }

    fn collide_sphere(&self, particle: &mut Particle, center: Vec3, radius: f32) -> bool {
        let dist_vec = particle.position - center;
        let dist = dist_vec.length();
        if dist < radius {
            let normal = dist_vec.normalize();
            let dot = particle.velocity.dot(normal);
            if dot < 0.0 {
                particle.velocity = particle.velocity - normal * (2.0 * dot);
                particle.velocity = particle.velocity * self.bounce;
                particle.position = center + normal * radius;
                return true;
            }
        }
        false
    }
}

impl ParticleModule for CollisionModule {
    fn priority(&self) -> i32 {
        90
    }
    fn apply(&self, particle: &mut Particle, _dt: f32, _ctx: &mut ModuleContext) {
        for collider in &self.colliders {
            match collider {
                ParticleCollider::Plane(normal, offset) => {
                    self.collide_plane(particle, *normal, *offset);
                }
                ParticleCollider::Sphere(center, radius) => {
                    self.collide_sphere(particle, *center, *radius);
                }
                ParticleCollider::Box(center, half_size) => {
                    let rel_pos = particle.position - *center;
                    if rel_pos.x.abs() < half_size.x
                        && rel_pos.y.abs() < half_size.y
                        && rel_pos.z.abs() < half_size.z
                    {
                        let axes = [
                            (Vec3::X, half_size.x - rel_pos.x.abs()),
                            (Vec3::Y, half_size.y - rel_pos.y.abs()),
                            (Vec3::Z, half_size.z - rel_pos.z.abs()),
                        ];
                        let (normal, _) = axes
                            .iter()
                            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                            .unwrap();
                        let sign = if rel_pos.dot(*normal) > 0.0 {
                            1.0
                        } else {
                            -1.0
                        };
                        let collision_normal = *normal * sign;
                        let dot = particle.velocity.dot(collision_normal);
                        if dot < 0.0 {
                            particle.velocity = particle.velocity - collision_normal * (2.0 * dot);
                            particle.velocity = particle.velocity * self.bounce;
                        }
                    }
                }
            }
        }
    }
}

/// 杀死条件
#[derive(Clone, Debug)]
pub enum KillCondition {
    OutsideAabb(Vec3, Vec3), // min, max
    MinSpeed(f32),
    MaxDistance(Vec3, f32), // origin, max_distance
}

/// 杀死模块
#[derive(Clone, Debug)]
pub struct KillModule {
    condition: KillCondition,
}

impl KillModule {
    pub fn by_outside_aabb(min: Vec3, max: Vec3) -> Self {
        Self {
            condition: KillCondition::OutsideAabb(min, max),
        }
    }

    pub fn by_min_speed(v: f32) -> Self {
        Self {
            condition: KillCondition::MinSpeed(v),
        }
    }

    pub fn by_max_distance(origin: Vec3, d: f32) -> Self {
        Self {
            condition: KillCondition::MaxDistance(origin, d),
        }
    }

    fn should_kill(&self, particle: &Particle) -> bool {
        match &self.condition {
            KillCondition::OutsideAabb(min, max) => {
                particle.position.x < min.x
                    || particle.position.x > max.x
                    || particle.position.y < min.y
                    || particle.position.y > max.y
                    || particle.position.z < min.z
                    || particle.position.z > max.z
            }
            KillCondition::MinSpeed(v) => particle.velocity.length() < *v,
            KillCondition::MaxDistance(origin, d) => particle.position.distance(*origin) > *d,
        }
    }
}

impl ParticleModule for KillModule {
    fn priority(&self) -> i32 {
        100
    }
    fn apply(&self, particle: &mut Particle, _dt: f32, _ctx: &mut ModuleContext) {
        if self.should_kill(particle) {
            particle.age = particle.lifetime;
        }
    }
}

// ============================================================================
// 粒子发射器
// ============================================================================

/// 模拟空间
#[derive(Clone, Debug, PartialEq)]
pub enum SimulationSpace {
    Local,
    World,
}

impl Default for SimulationSpace {
    fn default() -> Self {
        Self::World
    }
}

/// 缩放模式
#[derive(Clone, Debug, PartialEq)]
pub enum ScalingMode {
    Hierarchy,
    Local,
    ShapeOnly,
}

impl Default for ScalingMode {
    fn default() -> Self {
        Self::Local
    }
}

/// 发射器句柄
pub type EmitterHandle = u32;

/// 粒子发射器
pub struct ParticleEmitter {
    /// 发射形状
    shape: EmitShape,
    /// 发射模式
    emission_mode: EmissionMode,
    /// 渲染模式
    render_mode: ParticleRenderMode,
    /// 混合模式
    #[allow(dead_code)]
    blend_mode: ParticleBlendMode,
    /// 模块列表
    modules: Vec<Box<dyn ParticleModule>>,
    /// 最大粒子数
    #[allow(dead_code)]
    max_particles: usize,
    /// 持续时间
    duration: f32,
    /// 是否循环
    looping: bool,
    /// 延迟时间
    delay: f32,
    /// 当前时间
    time: f32,
    /// 是否正在发射
    emitting: bool,
    /// 已发射总数
    emitted_count: u64,
    /// 发射累计时间
    emit_accumulator: f32,
    /// 随机种子
    #[allow(dead_code)]
    random_seed: u64,
    /// 随机数生成器
    rng: SimpleRng,
}

impl ParticleEmitter {
    pub fn new(shape: EmitShape) -> Self {
        Self {
            shape,
            emission_mode: EmissionMode::default(),
            render_mode: ParticleRenderMode::default(),
            blend_mode: ParticleBlendMode::default(),
            modules: Vec::new(),
            max_particles: DEFAULT_MAX_PARTICLES,
            duration: 5.0,
            looping: true,
            delay: 0.0,
            time: 0.0,
            emitting: true,
            emitted_count: 0,
            emit_accumulator: 0.0,
            random_seed: 12345,
            rng: SimpleRng::new(12345),
        }
    }

    pub fn with_rate(rate: f32) -> Self {
        Self {
            emission_mode: EmissionMode::Continuous(rate),
            ..Self::new(EmitShape::Point)
        }
    }

    pub fn with_duration(duration: f32, looping: bool) -> Self {
        Self {
            duration,
            looping,
            ..Self::new(EmitShape::Point)
        }
    }

    pub fn with_max_particles(max: usize) -> Self {
        Self {
            max_particles: max,
            ..Self::new(EmitShape::Point)
        }
    }

    pub fn with_burst(burst: BurstConfig) -> Self {
        Self {
            emission_mode: EmissionMode::Burst(alloc::vec![burst]),
            ..Self::new(EmitShape::Point)
        }
    }

    pub fn with_render_mode(mode: ParticleRenderMode) -> Self {
        Self {
            render_mode: mode,
            ..Self::new(EmitShape::Point)
        }
    }

    // Getter methods
    pub fn shape(&self) -> &EmitShape {
        &self.shape
    }

    pub fn set_shape(&mut self, shape: EmitShape) {
        self.shape = shape;
    }

    pub fn rate(&self) -> f32 {
        match &self.emission_mode {
            EmissionMode::Continuous(r) => *r,
            EmissionMode::Mixed(r, _) => *r,
            EmissionMode::Burst(_) => 0.0,
        }
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.emission_mode = EmissionMode::Continuous(rate);
    }

    pub fn burst(&self) -> Option<&Vec<BurstConfig>> {
        match &self.emission_mode {
            EmissionMode::Burst(b) => Some(b),
            EmissionMode::Mixed(_, b) => Some(b),
            EmissionMode::Continuous(_) => None,
        }
    }

    pub fn set_burst(&mut self, burst: BurstConfig) {
        self.emission_mode = EmissionMode::Burst(alloc::vec![burst]);
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn is_looping(&self) -> bool {
        self.looping
    }

    pub fn set_looping(&mut self, b: bool) {
        self.looping = b;
    }

    pub fn is_emitting(&self) -> bool {
        self.emitting
    }

    pub fn render_mode(&self) -> ParticleRenderMode {
        self.render_mode.clone()
    }

    pub fn set_render_mode(&mut self, mode: ParticleRenderMode) {
        self.render_mode = mode;
    }

    pub fn modules(&self) -> &[Box<dyn ParticleModule>] {
        &self.modules
    }

    pub fn add_module(&mut self, module: Box<dyn ParticleModule>) {
        self.modules.push(module);
        self.modules.sort_by_key(|m| m.priority());
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn emitted_count(&self) -> u64 {
        self.emitted_count
    }

    /// 开始发射
    pub fn play(&mut self) {
        self.emitting = true;
        self.time = 0.0;
        self.emit_accumulator = 0.0;
        self.reset_bursts();
    }

    /// 停止发射
    pub fn stop(&mut self) {
        self.emitting = false;
    }

    fn reset_bursts(&mut self) {
        match &mut self.emission_mode {
            EmissionMode::Burst(bursts) => {
                for b in bursts.iter_mut() {
                    b.reset();
                }
            }
            EmissionMode::Mixed(_, bursts) => {
                for b in bursts.iter_mut() {
                    b.reset();
                }
            }
            EmissionMode::Continuous(_) => {}
        }
    }

    /// 发射单个粒子
    pub fn emit_one(&mut self, emitter_position: Vec3) -> Particle {
        let position = self.shape.sample_position(&mut self.rng);
        let direction = self.shape.sample_direction(&mut self.rng, position);
        let world_pos = position + emitter_position;

        let mut particle = Particle::default();
        particle.position = world_pos;
        particle.velocity = direction;
        particle.lifetime = DEFAULT_LIFETIME;

        let mut ctx = ModuleContext::new(0.0, self.time, emitter_position);
        for module in &self.modules {
            if module.priority() < 0 {
                module.apply(&mut particle, 0.0, &mut ctx);
            }
        }

        self.emitted_count += 1;
        particle
    }

    /// 更新发射器，返回新生成的粒子
    pub fn update(&mut self, dt: f32, emitter_position: Vec3) -> Vec<Particle> {
        if !self.emitting {
            return alloc::vec![];
        }

        if self.time < self.delay {
            self.time += dt;
            return alloc::vec![];
        }

        let effective_time = self.time - self.delay;
        let mut new_particles = alloc::vec![];

        // 持续发射
        let rate = match &self.emission_mode {
            EmissionMode::Continuous(r) => *r,
            EmissionMode::Mixed(r, _) => *r,
            EmissionMode::Burst(_) => 0.0,
        };

        if rate > 0.0 {
            self.emit_accumulator += dt * rate;
            while self.emit_accumulator >= 1.0 {
                new_particles.push(self.emit_one(emitter_position));
                self.emit_accumulator -= 1.0;
            }
        }

        // 爁发发射
        self.process_bursts(effective_time, emitter_position, &mut new_particles);

        self.time += dt;

        if self.time >= self.duration && self.looping {
            self.time = self.delay;
            self.emit_accumulator = 0.0;
            self.reset_bursts();
        }

        new_particles
    }

    fn process_bursts(
        &mut self,
        effective_time: f32,
        emitter_position: Vec3,
        new_particles: &mut Vec<Particle>,
    ) {
        let bursts = match &mut self.emission_mode {
            EmissionMode::Burst(b) => b,
            EmissionMode::Mixed(_, b) => b,
            EmissionMode::Continuous(_) => return,
        };

        // 先收集需要触发的burst信息
        let mut triggers: Vec<(usize, u32)> = Vec::new();
        for (i, burst) in bursts.iter_mut().enumerate() {
            if burst.should_fire(effective_time) {
                triggers.push((i, burst.count));
            }
        }

        // 然后发射粒子
        for (_, count) in triggers {
            for _ in 0..count {
                new_particles.push(self.emit_one(emitter_position));
            }
        }
    }
}

// ============================================================================
// 粒子系统
// ============================================================================

/// 粒子系统
pub struct ParticleSystem {
    /// 发射器列表
    emitters: Vec<ParticleEmitter>,
    /// 粒子池
    pool: ParticlePool,
    /// 当前时间
    time: f32,
    /// 是否正在播放
    playing: bool,
    /// 模拟空间
    simulation_space: SimulationSpace,
    /// 缩放模式
    #[allow(dead_code)]
    scaling_mode: ScalingMode,
    /// 重力修正系数
    gravity_modifier: f32,
    /// 预热时间
    prewarm: bool,
    prewarm_time: f32,
    /// 时间缩放
    delta_time_scale: f32,
    /// 随机种子
    #[allow(dead_code)]
    random_seed: u64,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            emitters: Vec::new(),
            pool: ParticlePool::new(DEFAULT_MAX_PARTICLES),
            time: 0.0,
            playing: false,
            simulation_space: SimulationSpace::default(),
            scaling_mode: ScalingMode::default(),
            gravity_modifier: 1.0,
            prewarm: false,
            prewarm_time: 0.0,
            delta_time_scale: 1.0,
            random_seed: 12345,
        }
    }

    pub fn with_max_particles(max: usize) -> Self {
        Self {
            pool: ParticlePool::new(max),
            ..Self::new()
        }
    }

    /// 添加发射器
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) -> EmitterHandle {
        let handle = self.emitters.len() as EmitterHandle;
        self.emitters.push(emitter);
        handle
    }

    /// 移除发射器
    pub fn remove_emitter(&mut self, handle: EmitterHandle) {
        if handle < self.emitters.len() as u32 {
            self.emitters.remove(handle as usize);
        }
    }

    /// 获取发射器列表
    pub fn emitters(&self) -> &[ParticleEmitter] {
        &self.emitters
    }

    /// 粒子数量
    pub fn particle_count(&self) -> usize {
        self.pool.alive_count()
    }

    /// 最大粒子数
    pub fn max_particles(&self) -> usize {
        self.pool.max_count()
    }

    /// 设置最大粒子数
    pub fn set_max_particles(&mut self, n: usize) {
        self.pool = ParticlePool::new(n);
    }

    /// 播放
    pub fn play(&mut self) {
        self.playing = true;
        for emitter in self.emitters.iter_mut() {
            emitter.play();
        }
        if self.prewarm && self.prewarm_time > 0.0 {
            let dt = 1.0 / 60.0;
            let steps = (self.prewarm_time / dt) as usize;
            for _ in 0..steps {
                self.update(dt, Vec3::ZERO);
            }
        }
    }

    /// 暂停
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// 停止
    pub fn stop(&mut self) {
        self.playing = false;
        self.pool.clear();
        self.time = 0.0;
        for emitter in self.emitters.iter_mut() {
            emitter.stop();
        }
    }

    /// 清空
    pub fn clear(&mut self) {
        self.pool.clear();
    }

    /// 获取粒子池
    pub fn pool(&self) -> &ParticlePool {
        &self.pool
    }

    /// 当前时间
    pub fn time(&self) -> f32 {
        self.time
    }

    /// 是否正在播放
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// 模拟空间
    pub fn simulation_space(&self) -> SimulationSpace {
        self.simulation_space.clone()
    }

    pub fn set_simulation_space(&mut self, space: SimulationSpace) {
        self.simulation_space = space;
    }

    /// 重力修正
    pub fn gravity_modifier(&self) -> f32 {
        self.gravity_modifier
    }

    pub fn set_gravity_modifier(&mut self, g: f32) {
        self.gravity_modifier = g;
    }

    /// 时间缩放
    pub fn delta_time_scale(&self) -> f32 {
        self.delta_time_scale
    }

    pub fn set_delta_time_scale(&mut self, s: f32) {
        self.delta_time_scale = s;
    }

    /// 更新粒子系统
    pub fn update(&mut self, dt: f32, emitter_position: Vec3) {
        if !self.playing {
            return;
        }

        let scaled_dt = dt * self.delta_time_scale;
        self.time += scaled_dt;

        // 发射新粒子
        for emitter in self.emitters.iter_mut() {
            let new_particles = emitter.update(scaled_dt, emitter_position);
            for particle in new_particles {
                self.pool.spawn(particle);
            }
        }

        // 更新现有粒子
        let mut ctx = ModuleContext::new(scaled_dt, self.time, emitter_position);
        for i in 0..self.pool.alive_count() {
            let particle = &mut self.pool.particles[i];
            particle.update(scaled_dt);

            for emitter in &self.emitters {
                for module in emitter.modules() {
                    if module.priority() >= 0 {
                        module.apply(particle, scaled_dt, &mut ctx);
                    }
                }
            }
        }

        // 移除死亡粒子
        let mut i = 0;
        while i < self.pool.alive_count() {
            if !self.pool.particles[i].is_alive() {
                self.pool.kill(i);
            } else {
                i += 1;
            }
        }
    }

    /// 获取存活粒子
    pub fn alive_particles(&self) -> &[Particle] {
        self.pool.alive_particles()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 粒子事件
// ============================================================================

/// 粒子事件
#[derive(Clone, Debug)]
pub enum ParticleEvent {
    Born {
        index: usize,
    },
    Died {
        index: usize,
        age: f32,
        position: Vec3,
    },
    Collided {
        index: usize,
        normal: Vec3,
        depth: f32,
    },
}

/// 粒子事件队列
pub struct ParticleEventQueue {
    events: Vec<ParticleEvent>,
}

impl ParticleEventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            events: Vec::with_capacity(n),
        }
    }

    pub fn push(&mut self, event: ParticleEvent) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<ParticleEvent> {
        self.events.pop()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = ParticleEvent> + '_ {
        self.events.drain(..)
    }
}

impl Default for ParticleEventQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 后期特效栈
// ============================================================================

/// 后期特效Pass trait
pub trait IPostProcessPass: fmt::Debug {
    /// Pass名称
    fn name(&self) -> &str;

    /// 是否启用
    fn enabled(&self) -> bool;

    /// 设置启用状态
    fn set_enabled(&mut self, b: bool);

    /// 应用效果（返回是否实际执行）
    fn apply(&self, input: &[f32], output: &mut [f32], width: u32, height: u32) -> bool;
}

/// 后期特效栈
pub struct PostProcessingStack {
    passes: Vec<Box<dyn IPostProcessPass>>,
    enabled: bool,
    hdr: bool,
}

impl PostProcessingStack {
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            enabled: true,
            hdr: false,
        }
    }

    pub fn add_pass(&mut self, pass: Box<dyn IPostProcessPass>) {
        self.passes.push(pass);
    }

    pub fn insert_pass(&mut self, index: usize, pass: Box<dyn IPostProcessPass>) {
        if index <= self.passes.len() {
            self.passes.insert(index, pass);
        }
    }

    pub fn remove_pass(&mut self, index: usize) {
        if index < self.passes.len() {
            self.passes.remove(index);
        }
    }

    pub fn passes(&self) -> &[Box<dyn IPostProcessPass>] {
        &self.passes
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    pub fn hdr(&self) -> bool {
        self.hdr
    }

    pub fn set_hdr(&mut self, b: bool) {
        self.hdr = b;
    }

    /// 应用所有Pass
    pub fn apply(&mut self, input: &[f32], output: &mut [f32], width: u32, height: u32) {
        if !self.enabled {
            output.copy_from_slice(input);
            return;
        }

        let mut temp = input.to_vec();
        let mut temp2 = output.to_vec();

        for pass in &self.passes {
            if pass.enabled() {
                pass.apply(&temp, &mut temp2, width, height);
                core::mem::swap(&mut temp, &mut temp2);
            }
        }

        output.copy_from_slice(&temp);
    }
}

impl Default for PostProcessingStack {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 具体后期特效Pass实现
// ============================================================================

/// Bloom Pass
#[derive(Clone, Debug)]
pub struct BloomPass {
    enabled: bool,
    intensity: f32,
    threshold: f32,
    soft_knee: f32,
    radius: f32,
    mip_count: u32,
}

impl BloomPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            intensity: BLOOM_DEFAULT_INTENSITY,
            threshold: BLOOM_DEFAULT_THRESHOLD,
            soft_knee: 0.5,
            radius: 1.0,
            mip_count: 6,
        }
    }

    pub fn with_intensity(f: f32) -> Self {
        Self {
            intensity: f,
            ..Self::new()
        }
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    pub fn soft_knee(&self) -> f32 {
        self.soft_knee
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn mip_count(&self) -> u32 {
        self.mip_count
    }

    /// 提取高亮区域
    pub fn extract_bright(input: &[f32], output: &mut [f32], threshold: f32) {
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = if *i > threshold { *i - threshold } else { 0.0 };
        }
    }
}

impl Default for BloomPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for BloomPass {
    fn name(&self) -> &str {
        "Bloom"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        let mut bright = alloc::vec![0.0; input.len()];
        Self::extract_bright(input, &mut bright, self.threshold);
        for idx in 0..input.len() {
            output[idx] = input[idx] + bright[idx] * self.intensity;
        }
        true
    }
}

/// 景深Pass
#[derive(Clone, Debug)]
pub struct DOFPass {
    enabled: bool,
    focus_distance: f32,
    focal_length: f32,
    aperture: f32,
    max_blur: f32,
}

impl DOFPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            focus_distance: 10.0,
            focal_length: 50.0,
            aperture: 2.8,
            max_blur: 10.0,
        }
    }

    pub fn focus_distance(&self) -> f32 {
        self.focus_distance
    }
    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }
    pub fn aperture(&self) -> f32 {
        self.aperture
    }
    pub fn max_blur(&self) -> f32 {
        self.max_blur
    }

    /// 计算模糊圈大小
    pub fn circle_of_confusion(&self, depth: f32) -> f32 {
        let coc = (depth - self.focus_distance).abs() * self.aperture / self.focal_length;
        coc.min(self.max_blur)
    }
}

impl Default for DOFPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for DOFPass {
    fn name(&self) -> &str {
        "DepthOfField"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        output.copy_from_slice(input);
        true
    }
}

/// SSAO Pass
#[derive(Clone, Debug)]
pub struct SSAOPass {
    enabled: bool,
    radius: f32,
    bias: f32,
    power: f32,
    kernel_size: u32,
}

impl SSAOPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            radius: SSAO_DEFAULT_RADIUS,
            bias: 0.025,
            power: 2.0,
            kernel_size: SSAO_DEFAULT_KERNEL_SIZE,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn bias(&self) -> f32 {
        self.bias
    }
    pub fn power(&self) -> f32 {
        self.power
    }
    pub fn kernel_size(&self) -> u32 {
        self.kernel_size
    }

    /// 生成采样核
    pub fn generate_kernel(seed: u64) -> Vec<Vec3> {
        let mut rng = SimpleRng::new(seed);
        let mut kernel = Vec::with_capacity(SSAO_DEFAULT_KERNEL_SIZE as usize);
        for _ in 0..SSAO_DEFAULT_KERNEL_SIZE {
            let theta = rng.range(0.0, core::f32::consts::TAU);
            let phi = rng.range(0.0, core::f32::consts::FRAC_PI_2);
            let r = rng.range(0.0, 1.0);
            kernel.push(Vec3::new(phi.cos() * theta.cos(), phi.sin(), phi.cos() * theta.sin()) * r);
        }
        kernel
    }
}

impl Default for SSAOPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for SSAOPass {
    fn name(&self) -> &str {
        "SSAO"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = *i * 0.9;
        }
        true
    }
}

/// SSR Pass
#[derive(Clone, Debug)]
pub struct SSRPass {
    enabled: bool,
    step_count: u32,
    thickness: f32,
    #[allow(dead_code)]
    binary_search_steps: u32,
    max_distance: f32,
}

impl SSRPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            step_count: 64,
            thickness: 0.5,
            binary_search_steps: 8,
            max_distance: 100.0,
        }
    }

    pub fn step_count(&self) -> u32 {
        self.step_count
    }
    pub fn thickness(&self) -> f32 {
        self.thickness
    }
    pub fn max_distance(&self) -> f32 {
        self.max_distance
    }
}

impl Default for SSRPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for SSRPass {
    fn name(&self) -> &str {
        "SSR"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        output.copy_from_slice(input);
        true
    }
}

/// 色调映射模式
#[derive(Clone, Debug, PartialEq)]
pub enum ToneMappingMode {
    Linear,
    Reinhard,
    ReinhardExtended(f32),
    ACES,
    Neutral,
    Filmic,
}

impl Default for ToneMappingMode {
    fn default() -> Self {
        Self::ACES
    }
}

/// 色调映射Pass
#[derive(Clone, Debug)]
pub struct ToneMappingPass {
    enabled: bool,
    mode: ToneMappingMode,
    exposure: f32,
}

impl ToneMappingPass {
    pub fn new(mode: ToneMappingMode) -> Self {
        Self {
            enabled: true,
            mode,
            exposure: 1.0,
        }
    }

    pub fn exposure(&self) -> f32 {
        self.exposure
    }
    pub fn set_exposure(&mut self, e: f32) {
        self.exposure = e;
    }

    fn apply_tone_mapping(&self, v: f32) -> f32 {
        let v = v * self.exposure;
        match self.mode {
            ToneMappingMode::Linear => v,
            ToneMappingMode::Reinhard => v / (1.0 + v),
            ToneMappingMode::ReinhardExtended(white) => {
                let white_sq = white * white;
                v * (1.0 + v / white_sq) / (1.0 + v)
            }
            ToneMappingMode::ACES => {
                let a = 2.51;
                let b = 0.03;
                let c = 2.43;
                let d = 0.59;
                let e = 0.14;
                (v * (a * v + b)) / (v * (c * v + d) + e)
            }
            ToneMappingMode::Neutral => v.clamp(0.0, 1.0),
            ToneMappingMode::Filmic => {
                let a = 0.22;
                let b = 0.30;
                let c = 0.10;
                let d = 0.20;
                let e = 0.01;
                let f = 0.30;
                ((v * (a * v + c * b) + d * e) / (v * (a * v + b) + d * f)) - e / f
            }
        }
    }
}

impl Default for ToneMappingPass {
    fn default() -> Self {
        Self::new(ToneMappingMode::default())
    }
}

impl IPostProcessPass for ToneMappingPass {
    fn name(&self) -> &str {
        "ToneMapping"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        for (i, o) in input.iter().zip(output.iter_mut()) {
            *o = self.apply_tone_mapping(*i).clamp(0.0, 1.0);
        }
        true
    }
}

/// 暗角Pass
#[derive(Clone, Debug)]
pub struct VignettePass {
    enabled: bool,
    intensity: f32,
    smoothness: f32,
    roundness: f32,
    center: Vec2,
}

impl VignettePass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            intensity: 0.45,
            smoothness: 0.2,
            roundness: 1.0,
            center: Vec2::splat(0.5),
        }
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }
    pub fn smoothness(&self) -> f32 {
        self.smoothness
    }
    pub fn roundness(&self) -> f32 {
        self.roundness
    }
    pub fn center(&self) -> Vec2 {
        self.center
    }
}

impl Default for VignettePass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for VignettePass {
    fn name(&self) -> &str {
        "Vignette"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], width: u32, height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        let w = width as f32;
        let h = height as f32;
        for (idx, (i, o)) in input.iter().zip(output.iter_mut()).enumerate() {
            let x = (idx % width as usize) as f32 / w;
            let y = (idx / width as usize) as f32 / h;
            let dx = x - self.center.x;
            let dy = y - self.center.y;
            let dist = (dx * dx + dy * dy).sqrt() * 2.0;
            let vignette = 1.0 - self.intensity * dist.powf(self.roundness);
            *o = *i * vignette.clamp(0.0, 1.0);
        }
        true
    }
}

/// 色差Pass
#[derive(Clone, Debug)]
pub struct ChromaticAberrationPass {
    enabled: bool,
    strength: f32,
    max_offset: f32,
}

impl ChromaticAberrationPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            strength: 0.3,
            max_offset: 0.02,
        }
    }

    pub fn strength(&self) -> f32 {
        self.strength
    }
    pub fn max_offset(&self) -> f32 {
        self.max_offset
    }
}

impl Default for ChromaticAberrationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for ChromaticAberrationPass {
    fn name(&self) -> &str {
        "ChromaticAberration"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        output.copy_from_slice(input);
        true
    }
}

/// 颜色分级Pass
#[derive(Clone, Debug)]
pub struct ColorGradingPass {
    enabled: bool,
    saturation: f32,
    contrast: f32,
    hue_shift: f32,
    white_balance: Vec2,
}

impl ColorGradingPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            saturation: 1.0,
            contrast: 1.0,
            hue_shift: 0.0,
            white_balance: Vec2::ZERO,
        }
    }

    pub fn saturation(&self) -> f32 {
        self.saturation
    }
    pub fn contrast(&self) -> f32 {
        self.contrast
    }
    pub fn hue_shift(&self) -> f32 {
        self.hue_shift
    }
    pub fn white_balance(&self) -> Vec2 {
        self.white_balance
    }
}

impl Default for ColorGradingPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for ColorGradingPass {
    fn name(&self) -> &str {
        "ColorGrading"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        for (i, o) in input.iter().zip(output.iter_mut()) {
            let v = *i;
            let v = (v - 0.5) * self.contrast + 0.5;
            let gray = 0.5;
            let v = gray + (v - gray) * self.saturation;
            *o = v.clamp(0.0, 1.0);
        }
        true
    }
}

/// FXAA Pass
#[derive(Clone, Debug)]
pub struct FXAAPass {
    enabled: bool,
    threshold: f32,
    edge_threshold: f32,
}

impl FXAAPass {
    pub fn new() -> Self {
        Self {
            enabled: true,
            threshold: 0.063,
            edge_threshold: 0.0312,
        }
    }

    pub fn threshold(&self) -> f32 {
        self.threshold
    }
    pub fn edge_threshold(&self) -> f32 {
        self.edge_threshold
    }
}

impl Default for FXAAPass {
    fn default() -> Self {
        Self::new()
    }
}

impl IPostProcessPass for FXAAPass {
    fn name(&self) -> &str {
        "FXAA"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled {
            return false;
        }
        output.copy_from_slice(input);
        true
    }
}

/// 调试视图模式
#[derive(Clone, Debug, PartialEq)]
pub enum DebugViewMode {
    None,
    Depth,
    Normal,
    MotionVector,
    AmbientOcclusion,
    BloomOnly,
    ColorGradingOnly,
}

impl Default for DebugViewMode {
    fn default() -> Self {
        Self::None
    }
}

/// 调试视图Pass
#[derive(Clone, Debug)]
pub struct PostProcessDebugView {
    enabled: bool,
    mode: DebugViewMode,
}

impl PostProcessDebugView {
    pub fn new(mode: DebugViewMode) -> Self {
        Self {
            enabled: true,
            mode,
        }
    }

    pub fn mode(&self) -> DebugViewMode {
        self.mode.clone()
    }

    pub fn set_mode(&mut self, mode: DebugViewMode) {
        self.mode = mode;
    }
}

impl Default for PostProcessDebugView {
    fn default() -> Self {
        Self::new(DebugViewMode::default())
    }
}

impl IPostProcessPass for PostProcessDebugView {
    fn name(&self) -> &str {
        "DebugView"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, b: bool) {
        self.enabled = b;
    }

    fn apply(&self, input: &[f32], output: &mut [f32], _width: u32, _height: u32) -> bool {
        if !self.enabled || self.mode == DebugViewMode::None {
            return false;
        }
        output.copy_from_slice(input);
        true
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let p = Particle::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0,
            Vec2::ONE,
            Rgba::WHITE,
            2.0,
        );
        assert!(p.is_alive());
        assert_eq!(p.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(p.lifetime, 2.0);
    }

    #[test]
    fn test_particle_update() {
        let mut p = Particle::default();
        p.velocity = Vec3::new(1.0, 0.0, 0.0);
        p.lifetime = 5.0;
        p.update(1.0);
        assert_eq!(p.position, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(p.age, 1.0);
        assert!(p.is_alive());
    }

    #[test]
    fn test_particle_normalized_age() {
        let p = Particle {
            age: 0.5,
            lifetime: 1.0,
            ..Default::default()
        };
        assert!((p.normalized_age() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_particle_pool_spawn() {
        let mut pool = ParticlePool::new(10);
        let p = Particle::default();
        assert!(pool.spawn(p));
        assert_eq!(pool.alive_count(), 1);
    }

    #[test]
    fn test_particle_pool_kill() {
        let mut pool = ParticlePool::new(10);
        pool.spawn(Particle::default());
        pool.spawn(Particle::default());
        assert_eq!(pool.alive_count(), 2);
        pool.kill(0);
        assert_eq!(pool.alive_count(), 1);
    }

    #[test]
    fn test_particle_pool_max_limit() {
        let mut pool = ParticlePool::new(2);
        assert!(pool.spawn(Particle::default()));
        assert!(pool.spawn(Particle::default()));
        assert!(!pool.spawn(Particle::default()));
        assert_eq!(pool.alive_count(), 2);
    }

    #[test]
    fn test_rgba_lerp() {
        let a = Rgba::BLACK;
        let b = Rgba::WHITE;
        let c = a.lerp(b, 0.5);
        assert!((c.r - 0.5).abs() < 1e-6);
        assert!((c.g - 0.5).abs() < 1e-6);
        assert!((c.b - 0.5).abs() < 1e-6);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_emit_shape_point() {
        let mut rng = SimpleRng::new(42);
        let shape = EmitShape::Point;
        let pos = shape.sample_position(&mut rng);
        assert_eq!(pos, Vec3::ZERO);
    }

    #[test]
    fn test_emit_shape_box() {
        let mut rng = SimpleRng::new(42);
        let shape = EmitShape::Box(Vec3::new(2.0, 2.0, 2.0));
        let pos = shape.sample_position(&mut rng);
        assert!(pos.x >= -1.0 && pos.x <= 1.0);
        assert!(pos.y >= -1.0 && pos.y <= 1.0);
        assert!(pos.z >= -1.0 && pos.z <= 1.0);
    }

    #[test]
    fn test_emit_shape_sphere() {
        let mut rng = SimpleRng::new(42);
        let shape = EmitShape::Sphere(1.0, SphereEmitMode::Shell);
        let pos = shape.sample_position(&mut rng);
        assert!((pos.length() - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_burst_config() {
        let mut burst = BurstConfig::new(0.5, 10, 2, 1.0);
        assert!(!burst.should_fire(0.0));
        assert!(burst.should_fire(0.5));
        assert_eq!(burst.triggered_count, 1);
        assert!(!burst.should_fire(1.0));
        assert!(burst.should_fire(1.5));
        assert_eq!(burst.triggered_count, 2);
        assert!(!burst.should_fire(2.5));
    }

    #[test]
    fn test_burst_config_reset() {
        let mut burst = BurstConfig::new(0.0, 5, 1, 0.0);
        burst.should_fire(0.0);
        assert_eq!(burst.triggered_count, 1);
        burst.reset();
        assert_eq!(burst.triggered_count, 0);
    }

    #[test]
    fn test_gravity_module() {
        let module = GravityModule::default();
        assert_eq!(module.gravity, DEFAULT_GRAVITY);

        let mut particle = Particle::default();
        particle.velocity = Vec3::ZERO;
        let mut ctx = ModuleContext::new(1.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 1.0, &mut ctx);
        assert!((particle.velocity.y - DEFAULT_GRAVITY).abs() < 1e-6);
    }

    #[test]
    fn test_force_module() {
        let module = ForceModule::new(Vec3::new(1.0, 2.0, 3.0));
        let mut particle = Particle::default();
        let mut ctx = ModuleContext::new(1.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 1.0, &mut ctx);
        assert_eq!(particle.velocity, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_drag_module() {
        let module = DragModule::new(0.5);
        let mut particle = Particle::default();
        particle.velocity = Vec3::new(10.0, 10.0, 10.0);
        let mut ctx = ModuleContext::new(1.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 1.0, &mut ctx);
        assert_eq!(particle.velocity, Vec3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_color_over_life_module() {
        let module = ColorOverLifeModule::new(alloc::vec![(0.0, Rgba::BLACK), (1.0, Rgba::WHITE),]);
        let mut particle = Particle::default();
        particle.age = 0.5;
        particle.lifetime = 1.0;
        let mut ctx = ModuleContext::new(0.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 0.0, &mut ctx);
        assert!((particle.color.r - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_size_over_life_module() {
        let module = SizeOverLifeModule::new(alloc::vec![(0.0, 1.0), (1.0, 0.0),]);
        let mut particle = Particle::default();
        particle.age = 0.5;
        particle.lifetime = 1.0;
        particle.size = Vec2::ONE;
        let mut ctx = ModuleContext::new(0.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 0.0, &mut ctx);
        assert!((particle.size.x - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_attractor_module() {
        let module = AttractorModule::new(Vec3::ZERO, 10.0, 5.0);
        let force = module.attract(Vec3::new(2.0, 0.0, 0.0));
        // dir = center - position = (0,0,0) - (2,0,0) = (-2,0,0)
        // normalize = (-1,0,0)
        // falloff = 2/5 = 0.4
        // force = (-1,0,0) * 10 * 0.4 = (-4,0,0)
        assert!((force.x - (-4.0)).abs() < 1e-6);
    }

    #[test]
    fn test_attractor_at_center() {
        let module = AttractorModule::new(Vec3::ZERO, 10.0, 5.0);
        let force = module.attract(Vec3::ZERO);
        assert_eq!(force, Vec3::ZERO);
    }

    #[test]
    fn test_collision_module_sphere() {
        let module = CollisionModule::new(alloc::vec![ParticleCollider::Sphere(Vec3::ZERO, 1.0),]);
        let mut particle = Particle::default();
        particle.position = Vec3::new(0.5, 0.0, 0.0);
        particle.velocity = Vec3::new(-1.0, 0.0, 0.0);
        let mut ctx = ModuleContext::new(0.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 0.0, &mut ctx);
        assert!(particle.velocity.x > 0.0);
    }

    #[test]
    fn test_kill_module_aabb() {
        let module =
            KillModule::by_outside_aabb(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let mut particle = Particle::default();
        particle.position = Vec3::new(2.0, 0.0, 0.0);
        particle.lifetime = 5.0;
        let mut ctx = ModuleContext::new(0.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 0.0, &mut ctx);
        assert!(!particle.is_alive());
    }

    #[test]
    fn test_kill_module_min_speed() {
        let module = KillModule::by_min_speed(1.0);
        let mut particle = Particle::default();
        particle.velocity = Vec3::new(0.1, 0.0, 0.0);
        particle.lifetime = 5.0;
        let mut ctx = ModuleContext::new(0.0, 0.0, Vec3::ZERO);
        module.apply(&mut particle, 0.0, &mut ctx);
        assert!(!particle.is_alive());
    }

    #[test]
    fn test_particle_system_basic() {
        let mut system = ParticleSystem::new();
        let emitter = ParticleEmitter::with_rate(10.0);
        system.add_emitter(emitter);
        system.play();
        assert!(system.is_playing());
        system.update(0.1, Vec3::ZERO);
        assert!(system.particle_count() > 0);
    }

    #[test]
    fn test_particle_system_stop() {
        let mut system = ParticleSystem::new();
        system.add_emitter(ParticleEmitter::with_rate(10.0));
        system.play();
        system.update(0.1, Vec3::ZERO);
        system.stop();
        assert!(!system.is_playing());
        assert_eq!(system.particle_count(), 0);
    }

    #[test]
    fn test_post_process_stack() {
        let mut stack = PostProcessingStack::new();
        stack.add_pass(Box::new(BloomPass::new()));
        stack.add_pass(Box::new(VignettePass::new()));
        assert_eq!(stack.passes().len(), 2);
        assert!(stack.enabled());
    }

    #[test]
    fn test_post_process_stack_remove() {
        let mut stack = PostProcessingStack::new();
        stack.add_pass(Box::new(BloomPass::new()));
        stack.add_pass(Box::new(VignettePass::new()));
        stack.remove_pass(0);
        assert_eq!(stack.passes().len(), 1);
    }

    #[test]
    fn test_post_process_stack_disabled() {
        let mut stack = PostProcessingStack::new();
        stack.add_pass(Box::new(BloomPass::new()));
        stack.set_enabled(false);
        let input = alloc::vec![0.5; 100];
        let mut output = alloc::vec![0.0; 100];
        stack.apply(&input, &mut output, 10, 10);
        assert_eq!(input, output);
    }

    #[test]
    fn test_bloom_pass_disabled() {
        let mut pass = BloomPass::new();
        pass.set_enabled(false);
        let input = alloc::vec![0.5; 100];
        let mut output = alloc::vec![0.0; 100];
        let applied = pass.apply(&input, &mut output, 10, 10);
        assert!(!applied);
    }

    #[test]
    fn test_ssao_kernel_generation() {
        let kernel = SSAOPass::generate_kernel(42);
        assert_eq!(kernel.len(), SSAO_DEFAULT_KERNEL_SIZE as usize);
        for v in &kernel {
            assert!(v.y >= 0.0);
        }
    }

    #[test]
    fn test_tone_mapping_reinhard() {
        let pass = ToneMappingPass::new(ToneMappingMode::Reinhard);
        let result = pass.apply_tone_mapping(1.0);
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_tone_mapping_linear() {
        let pass = ToneMappingPass::new(ToneMappingMode::Linear);
        let result = pass.apply_tone_mapping(0.5);
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_dof_circle_of_confusion() {
        let pass = DOFPass::new();
        let coc = pass.circle_of_confusion(pass.focus_distance);
        assert!((coc).abs() < 1e-6);
        let coc_far = pass.circle_of_confusion(pass.focus_distance + 10.0);
        assert!(coc_far > 0.0);
    }

    #[test]
    fn test_vignette_center() {
        let pass = VignettePass::new();
        let input = alloc::vec![1.0; 100];
        let mut output = alloc::vec![0.0; 100];
        pass.apply(&input, &mut output, 10, 10);
        let center_idx = 5 * 10 + 5;
        assert!((output[center_idx] - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_color_grading_saturation() {
        let pass = ColorGradingPass::new();
        let input = alloc::vec![0.5; 100];
        let mut output = alloc::vec![0.0; 100];
        pass.apply(&input, &mut output, 10, 10);
        for o in &output {
            assert!(*o >= 0.0 && *o <= 1.0);
        }
    }

    #[test]
    fn test_emitter_play_stop() {
        let mut emitter = ParticleEmitter::new(EmitShape::Point);
        emitter.play();
        assert!(emitter.is_emitting());
        emitter.stop();
        assert!(!emitter.is_emitting());
    }

    #[test]
    fn test_emitter_duration_loop() {
        let mut emitter = ParticleEmitter::with_duration(1.0, false);
        emitter.play();
        emitter.update(0.5, Vec3::ZERO);
        assert!(emitter.is_emitting());
        emitter.update(0.6, Vec3::ZERO);
        assert!(!emitter.is_emitting() || emitter.time() >= emitter.duration());
    }

    #[test]
    fn test_particle_event_queue() {
        let mut queue = ParticleEventQueue::new();
        queue.push(ParticleEvent::Born { index: 0 });
        queue.push(ParticleEvent::Died {
            index: 0,
            age: 1.0,
            position: Vec3::ZERO,
        });
        assert_eq!(queue.len(), 2);
        let event = queue.pop();
        assert!(event.is_some());
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_random_direction() {
        let mut rng = SimpleRng::new(42);
        let dir = random_direction(&mut rng);
        assert!((dir.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_simple_rng_range() {
        let mut rng = SimpleRng::new(42);
        for _ in 0..100 {
            let v = rng.range(0.0, 1.0);
            assert!(v >= 0.0 && v <= 1.0);
        }
    }
}
