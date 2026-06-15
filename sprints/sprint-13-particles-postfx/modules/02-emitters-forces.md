# 模块二：发射器与力模块

## 模块名称与概述

**模块名称**：Emitters and Force Modules  
**模块路径**：`engine-particles` crate  
**功能概述**：粒子发射源（ParticleEmitter）管理发射形状（EmitShape）、发射模式（Burst/Continuous/Mixed）以及 20+ 种生命周期模块（Force/Gravity/Drag/Turbulence/Noise/Orbital/Attractor/Collision/SubEmitter/Kill 等）。

---

## 需求编号

对应原需求文档编号：**19-38, 66-95, 97-100, 214-270, 284-366, 415-430, 432-438**

---

## 功能描述

---

## Part A：ParticleEmitter 组件

### A.1 基本结构

```rust
pub struct ParticleEmitter {
    shape: EmitShape,
    emission_mode: EmissionMode,
    material: Handle<Material>,
    modules: Vec<Box<dyn ParticleModule>>,
    render_mode: ParticleRenderMode,
    // 时间与循环控制
    duration: f32,
    looping: bool,
    delay: f32,
    time: f32,
    emitting: bool,
    emitted_count: u64,
}

impl ParticleEmitter {
    pub fn new(shape: EmitShape, emission_mode: EmissionMode, material: Handle<Material>) -> Self;
    
    // Builder pattern
    pub fn with_rate(rate: f32) -> Self;
    pub fn with_duration(duration: f32, looping: bool) -> Self;
    pub fn with_max_particles(max: usize) -> Self;
    pub fn with_burst(burst: BurstConfig) -> Self;
    pub fn with_render_mode(mode: ParticleRenderMode) -> Self;
    pub fn with_material(material: Handle<Material>) -> Self;
    
    // Getters
    pub fn shape(&self) -> &EmitShape;
    pub fn set_shape(&mut self, shape: EmitShape);
    pub fn rate(&self) -> f32;  // 每秒发射数
    pub fn set_rate(&mut self, rate: f32);
    pub fn burst(&self) -> Option<BurstConfig>;
    pub fn set_burst(&mut self, burst: Option<BurstConfig>);
    pub fn duration(&self) -> f32;
    pub fn set_duration(&mut self, duration: f32);
    pub fn is_looping(&self) -> bool;
    pub fn set_looping(&mut self, b: bool);
    pub fn is_emitting(&self) -> bool;
    pub fn play(&mut self);
    pub fn stop(&mut self);
    pub fn modules(&self) -> &[ParticleModule];
    pub fn add_module(&mut self, module: Box<dyn ParticleModule>);
    pub fn render_mode(&self) -> ParticleRenderMode;
    pub fn set_render_mode(&mut self, mode: ParticleRenderMode);
    pub fn material(&self) -> Handle<Material>;
    pub fn set_material(&mut self, material: Handle<Material>);
    
    // 统计
    pub fn time(&self) -> f32;
    pub fn emitted_count(&self) -> u64;
    pub fn alive_count(&self) -> usize;
    pub fn active_particles(&self) -> &[Particle];
    
    // 发射逻辑
    pub fn spawn(&mut self, count: usize);
    pub fn emit_one(&mut self, ctx: &ModuleContext) -> Option<Particle>;
    pub fn update(&mut self, dt: f32, ctx: &ModuleContext) -> Vec<Particle>;
    
    // Burst 管理
    pub fn burst_list(&self) -> &[BurstConfig];
    pub fn add_burst(&mut self, burst: BurstConfig);
    pub fn remove_burst(&mut self, index: usize);
    pub fn emission_rate_over_time(&self) -> Option<&Curve<f32>>;
    pub fn set_emission_rate_over_time(&mut self, curve: Option<Curve<f32>>);
    pub fn delay(&self) -> f32;
    pub fn set_delay(&mut self, seconds: f32);
}
```

**输入**：dt, ModuleContext（rng/transform/event_queue）  
**输出**：新生成粒子列表

---

## Part B：EmitShape 发射形状

### B.1 形状类型

| 形状 | API | 参数 |
|------|-----|------|
| Point | `EmitShape::Point` | - |
| Box | `EmitShape::Box(Vec3)` | 长方体尺寸 |
| Sphere | `EmitShape::Sphere(f32, SphereEmitMode)` | 半径 + 发射模式 |
| Hemisphere | `EmitShape::Hemisphere(f32)` | 半径 |
| Cone | `EmitShape::Cone(angle, base_radius, length, ConeEmitMode)` | 角度/半径/长度/模式 |
| Circle | `EmitShape::Circle(f32, axis)` | 半径 + 轴向 |
| Edge | `EmitShape::Edge(Vec3, Vec3)` | 起点 + 终点 |
| Mesh | `EmitShape::Mesh(Handle<Mesh3D>, MeshEmitMode)` | mesh句柄 + 发射模式 |
| SkinnedMesh | `EmitShape::SkinnedMesh(Handle<SkinnedMesh>, MeshEmitMode)` | 蒙皮mesh + 模式 |

### B.2 发射模式枚举

```rust
pub enum SphereEmitMode { Volume, Shell }
pub enum ConeEmitMode { Base, Volume, Shell }
pub enum MeshEmitMode { Vertex, Edge, Triangle, VolumeApprox }
```

### B.3 EmitShape API

```rust
impl EmitShape {
    pub fn sample_position(&self, rng: &mut dyn Rng) -> Vec3;
    pub fn sample_direction(&self, rng: &mut dyn Rng, position: Vec3) -> Vec3;
    pub fn sample(&self, rng: &mut dyn Rng) -> (Vec3, Vec3);  // (position, direction)
    pub fn surface_area(&self) -> f32;  // 用于 mesh 概率采样
    pub fn aabb(&self, transform: &Transform) -> Aabb;
    pub fn scale(&self, s: f32) -> EmitShape;
    pub fn transform(&self, mat: &Mat4) -> EmitShape;
    pub fn rotate(&self, quat: &Quat) -> EmitShape;
}
```

**输入**：rng, 可选 transform  
**输出**：采样位置/方向，AABB

---

## Part C：EmissionMode 发射模式

```rust
pub enum EmissionMode {
    Continuous(f32),           // 固定速率
    Burst(Vec<BurstConfig>),    // 爆发配置列表
    Mixed {
        rate: f32,              // 持续速率
        bursts: Vec<BurstConfig>,  // 叠加爆发
    },
}

pub struct BurstConfig {
    time: f32,         // 触发时间
    count: u32,         // 发射数量
    cycles: u32,       // 循环次数
    interval: f32,      // 循环间隔
}

impl BurstConfig {
    pub fn new(time: f32, count: u32, cycles: u32, interval: f32) -> Self;
    pub fn should_fire(&self, current_time: f32) -> bool;
    pub fn reset(&mut self);
}
```

---

## Part D：ParticleModule 模块系统

### D.1 Module Trait

```rust
pub trait ParticleModule: Any + Send + Sync {
    fn priority(&self) -> i32;  // 执行顺序
    fn apply(&self, particle: &mut Particle, dt: f32, ctx: &ModuleContext);
}

pub struct ModuleContext<'a> {
    pub dt: f32,
    pub total_time: f32,
    pub emitter_transform: &'a Transform,
    pub rng: &'a mut dyn Rng,
    pub event_queue: &'a mut ParticleEventQueue,
}

impl ModuleContext {
    pub fn emit(&mut self, event: ParticleEvent);
}
```

### D.2 模块清单

| 模块 | API | 功能 |
|------|-----|------|
| InitialVelocity | `InitialVelocityModule::new(min: Vec3, max: Vec3)` | 初始速度区间 |
| VelocityOverLife | `VelocityOverLifeModule::new(curve: Curve<Vec3>)` | 速度随生命周期变化 |
| ColorOverLife | `ColorOverLifeModule::new(gradient: ColorGradient)` | 颜色渐变 |
| SizeOverLife | `SizeOverLifeModule::new(curve: Curve<f32>)` | 尺寸随生命周期变化 |
| RotationOverLife | `RotationOverLifeModule::new(curve: Curve<f32>)` | 旋转随生命周期变化 |
| Force | `ForceModule::new(force: Vec3)` | 恒定外力 |
| Gravity | `GravityModule::new(gravity: Vec3)` | 重力（默认 0,-9.8,0） |
| Drag | `DragModule::new(drag: f32)` | 空气阻力 |
| Turbulence | `TurbulenceModule::new(intensity, frequency, speed)` | 湍流扰动 |
| Noise | `NoiseModule::new(frequency, octaves, seed, amplitude)` | 噪声采样 |
| Orbital | `OrbitalModule::new(center, axis, angular_speed)` | 轨道运动 |
| Attractor | `AttractorModule::new(center, strength, falloff_radius)` | 引力吸引 |
| Collision | `CollisionModule::new(colliders, bounce, friction, kill_threshold)` | 碰撞检测 |
| SubEmitter | `SubEmitterModule::new(emitter_handle, trigger)` | 子发射器触发 |
| Kill | `KillModule::new(condition)` | 粒子消亡条件 |
| LifetimeByVelocity | `LifetimeByVelocityModule::new(min, max)` | 速度映射生命周期 |
| RotationBySpeed | `RotationBySpeedModule::new(scale)` | 速度映射旋转 |

### D.3 模块详细 API

```rust
// InitialVelocityModule
pub struct InitialVelocityModule { min: Vec3, max: Vec3 }
impl InitialVelocityModule { pub fn new(min, max) -> Self; pub fn speed(&self) -> (Vec3, Vec3); }

// VelocityOverLifeModule
pub struct VelocityOverLifeModule { curve: Curve<Vec3> }
impl VelocityOverLifeModule { pub fn new(curve) -> Self; pub fn curve(&self) -> &Curve<Vec3>; }

// ColorOverLifeModule
pub struct ColorOverLifeModule { gradient: ColorGradient }
impl ColorOverLifeModule { pub fn new(gradient) -> Self; }

// ColorGradient
pub struct ColorGradient { stops: Vec<(f32, Rgba)> }
impl ColorGradient {
    pub fn new(stops: Vec<(f32, Rgba)>) -> Self;
    pub fn sample(&self, t: f32) -> Rgba;  // t 自动 clamp 到 [0,1]
}

// SizeOverLifeModule / RotationOverLifeModule
pub struct SizeOverLifeModule { curve: Curve<f32> }
pub struct RotationOverLifeModule { curve: Curve<f32> }

// ForceModule / GravityModule
pub struct ForceModule { force: Vec3 }
pub struct GravityModule { gravity: Vec3 }
impl GravityModule {
    pub fn new(g: Vec3) -> Self;
    pub fn default() -> Self;  // (0, -9.8, 0)
}

// DragModule
pub struct DragModule { drag: f32 }
impl DragModule { pub fn new(drag: f32) -> Self; pub fn drag(&self) -> f32; }

// TurbulenceModule / NoiseModule
pub struct TurbulenceModule { intensity, frequency, speed }
pub struct NoiseModule { frequency, octaves, seed, amplitude }
impl NoiseModule { pub fn sample(&self, position: Vec3, time: f32) -> Vec3; }

// OrbitalModule
pub struct OrbitalModule { center: Vec3, axis: Vec3, angular_speed: f32 }

// AttractorModule
pub struct AttractorModule { center: Vec3, strength: f32, falloff_radius: f32 }
impl AttractorModule { pub fn attract(&self, position: Vec3) -> Vec3; }

// CollisionModule
pub enum ParticleCollider {
    Plane { normal: Vec3, offset: f32 },
    Sphere { center: Vec3, radius: f32 },
    Box { center: Vec3, half_size: Vec3 },
    Mesh { mesh: Handle<Mesh3D>, mode: MeshCollisionMode },
}
pub enum MeshCollisionMode { DistanceFieldApprox, TriangleTest }

pub struct CollisionModule {
    colliders: Vec<ParticleCollider>,
    bounce: f32,
    friction: f32,
    kill_threshold: f32,
}
impl CollisionModule {
    pub fn new(colliders) -> Self;
    pub fn colliders(&self) -> &[ParticleCollider];
    pub fn bounce(&self) -> f32;
    pub fn friction(&self) -> f32;
    pub fn kill_threshold(&self) -> f32;
}

// SubEmitterModule
pub enum SubEmitterTrigger { OnBirth, OnDeath, OnCollision, OnTime(f32) }
pub struct SubEmitterModule { emitter_handle: EmitterHandle, trigger: SubEmitterTrigger }
impl SubEmitterModule {
    pub fn new(emitter_handle: EmitterHandle, trigger: SubEmitterTrigger) -> Self;
    pub fn inherit_position(&self) -> bool;
    pub fn inherit_rotation(&self) -> bool;
    pub fn inherit_velocity(&self) -> f32;  // 0~1 传递系数
}

// KillModule
pub enum KillCondition { OutsideAabb(Aabb), MinSpeed(f32), MaxDistance { origin: Vec3, distance: f32 } }
pub struct KillModule { condition: KillCondition }
impl KillModule {
    pub fn by_outside_aabb(aabb: Aabb) -> Self;
    pub fn by_min_speed(v: f32) -> Self;
    pub fn by_max_distance(origin: Vec3, d: f32) -> Self;
}

// LifetimeByVelocityModule / RotationBySpeedModule
pub struct LifetimeByVelocityModule { min: f32, max: f32 }
pub struct RotationBySpeedModule { scale: f32 }
```

---

## Part E：ParticleRenderMode 渲染模式

```rust
pub enum ParticleRenderMode {
    SpriteBillboard,                    // 始终面向相机
    MeshBillboard { mesh: Handle<Mesh3D> },  // mesh 代替 sprite
    StretchedBillboard { length_scale: f32, speed_scale: f32 },  // 沿速度拉伸
    HorizontalBillboard,                  // 绕 Y 轴对齐水平
    VerticalBillboard,                   // 绕相机 up 对齐
}
```

---

## Part F：ParticleMaterial 粒子材质

```rust
pub enum ParticleMaterial {
    PBR { albedo: Rgba, metallic: f32, roughness: f32, blending: Blending },
    Custom { shader: Handle<Shader> },
}

pub enum Blending { Opaque, Masked, Translucent, Additive }
```

---

## 验收标准

| ID | 描述 |
|----|------|
| V1 | `ParticleEmitter::emit_one` 正确采样 EmitShape |
| V2 | BurstConfig::should_fire 在指定时间触发 |
| V3 | Continuous 模式按 rate 均匀发射 |
| V4 | Mixed 模式同时支持持续和爆发 |
| V5 | InitialVelocityModule 使用速度区间 |
| V6 | GravityModule 在 dt 后速度变化正确（-9.8 * dt） |
| V7 | DragModule 衰减速度收敛到 0 |
| V8 | ColorGradient::sample 线性插值正确 |
| V9 | ColorGradient::sample 边界 t<0/t>1 clamp |
| V10 | AttractorModule 在 center 处无吸引（稳定点） |
| V11 | CollisionModule::Sphere 反射法向量单位化 |
| V12 | CollisionModule::Plane 位置+法向+弹回 |
| V13 | KillModule kill 超龄粒子 |
| V14 | EmitShape::sample 在 Point 返回 (origin, forward) |
| V15 | EmitShape::Sphere sample 长度接近 radius |
| V16 | EmitShape::Box sample 在 AABB 内部 |
| V17 | SubEmitter 在 particle death 时触发 |
| V18 | SubEmitter 级联（fire → smoke → sparks） |

---

## 依赖关系

**前置依赖**：
- `01-particle-system`：Particle/ParticlePool/ModuleContext
- `engine-render`：Material/Shader/Texture handles

**被依赖**：
- 渲染系统依赖 ParticleRenderMode / ParticleMaterial
- 事件系统依赖 ParticleEventQueue

---

## 优先级

**P0**：
- EmitShape 所有形状实现
- EmissionMode（Burst/Continuous/Mixed）
- InitialVelocity / Gravity / Drag / Kill

**P1**：
- ColorOverLife / SizeOverLife / RotationOverLife
- VelocityOverLife
- Collision（Plane/Sphere/Box）
- SubEmitter

**P2**：
- Turbulence / Noise / Orbital / Attractor
- MeshCollisionMode
- LifetimeByVelocity / RotationBySpeed
