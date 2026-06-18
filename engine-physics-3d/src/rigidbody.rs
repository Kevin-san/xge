//! 刚体模块
//!
//! 提供 3D 刚体实现，包括动态、静态、运动刚体类型。

use engine_math::{Quat, Vec3};

#[allow(unused_imports)]
use crate::constants::{DEFAULT_ANGULAR_DAMPING, DEFAULT_GRAVITY, DEFAULT_LINEAR_DAMPING};

/// 刚体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidBodyType3D {
    /// 动态刚体 - 受物理力影响
    #[default]
    Dynamic,
    /// 静态刚体 - 不受物理力影响，位置固定
    Static,
    /// 基于位置的运动刚体 - 位置由代码控制
    KinematicPositionBased,
    /// 基于速度的运动刚体 - 速度由代码控制
    KinematicVelocityBased,
    /// 固定刚体 - 完全固定，不参与物理仿真
    Fixed,
}

impl RigidBodyType3D {
    /// 检查是否是动态刚体
    pub fn is_dynamic(&self) -> bool {
        matches!(self, RigidBodyType3D::Dynamic)
    }

    /// 检查是否是静态刚体
    pub fn is_static(&self) -> bool {
        matches!(self, RigidBodyType3D::Static)
    }

    /// 检查是否是运动刚体
    pub fn is_kinematic(&self) -> bool {
        matches!(
            self,
            RigidBodyType3D::KinematicPositionBased | RigidBodyType3D::KinematicVelocityBased
        )
    }

    /// 检查是否是固定刚体
    pub fn is_fixed(&self) -> bool {
        matches!(self, RigidBodyType3D::Fixed)
    }

    /// 获取默认质量
    pub fn default_mass(&self) -> f32 {
        if self.is_dynamic() {
            1.0
        } else {
            0.0
        }
    }
}

/// 刚体句柄
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RigidBodyHandle {
    /// 索引
    pub index: u32,
    /// 版本号
    pub generation: u32,
}

impl RigidBodyHandle {
    /// 创建新的句柄
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// 无效句柄
    pub const INVALID: Self = Self {
        index: u32::MAX,
        generation: u32::MAX,
    };

    /// 检查是否有效
    pub fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }
}

impl Default for RigidBodyHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

/// 刚体状态
#[derive(Debug, Clone)]
pub struct RigidBodyState3D {
    /// 位置
    pub position: Vec3,
    /// 旋转（四元数）
    pub rotation: Quat,
    /// 线性速度
    pub linear_velocity: Vec3,
    /// 角速度
    pub angular_velocity: Vec3,
}

impl Default for RigidBodyState3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
        }
    }
}

/// 刚体
///
/// 代表一个具有质量和惯性的3D物理实体。
#[derive(Debug, Clone)]
pub struct RigidBody3D {
    /// 刚体类型
    body_type: RigidBodyType3D,
    /// 质量
    mass: f32,
    /// 逆质量（用于计算，0 表示无限质量）
    inverse_mass: f32,
    /// 位置
    position: Vec3,
    /// 旋转（四元数）
    rotation: Quat,
    /// 线性速度
    linear_velocity: Vec3,
    /// 角速度
    angular_velocity: Vec3,
    /// 力累加器
    force: Vec3,
    /// 扭矩累加器
    torque: Vec3,
    /// 线性阻尼
    linear_damping: f32,
    /// 角阻尼
    angular_damping: f32,
    /// 重力缩放
    gravity_scale: f32,
    /// 是否启用CCD（连续碰撞检测）
    ccd_enabled: bool,
    /// 是否正在睡眠
    is_sleeping: bool,
    /// 是否启用
    enabled: bool,
    /// 碰撞体索引列表
    collider_indices: Vec<usize>,
    /// 主导级分组
    dominance_group: i8,
    /// 变换是否需要更新
    transform_dirty: bool,
    /// 锁定平移轴
    locked_translations: (bool, bool, bool),
    /// 锁定旋转轴
    locked_rotations: (bool, bool, bool),
}

impl RigidBody3D {
    /// 创建新的刚体
    pub fn new(body_type: RigidBodyType3D) -> Self {
        let mass = body_type.default_mass();
        Self {
            body_type,
            mass,
            inverse_mass: if mass > 0.0 { 1.0 / mass } else { 0.0 },
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
            linear_damping: DEFAULT_LINEAR_DAMPING,
            angular_damping: DEFAULT_ANGULAR_DAMPING,
            gravity_scale: 1.0,
            ccd_enabled: false,
            is_sleeping: false,
            enabled: true,
            collider_indices: Vec::new(),
            dominance_group: 0,
            transform_dirty: true,
            locked_translations: (false, false, false),
            locked_rotations: (false, false, false),
        }
    }

    /// 获取刚体类型
    pub fn body_type(&self) -> RigidBodyType3D {
        self.body_type
    }

    /// 检查是否是动态刚体
    pub fn is_dynamic(&self) -> bool {
        self.body_type.is_dynamic()
    }

    /// 检查是否是静态刚体
    pub fn is_static(&self) -> bool {
        self.body_type.is_static()
    }

    /// 检查是否是运动刚体
    pub fn is_kinematic(&self) -> bool {
        self.body_type.is_kinematic()
    }

    /// 检查是否是固定刚体
    pub fn is_fixed(&self) -> bool {
        self.body_type.is_fixed()
    }

    /// 获取质量
    pub fn mass(&self) -> f32 {
        self.mass
    }

    /// 设置质量
    pub fn set_mass(&mut self, mass: f32) {
        if self.body_type.is_dynamic() && mass > 0.0 {
            self.mass = mass;
            self.inverse_mass = 1.0 / mass;
        }
    }

    /// 获取逆质量
    pub fn inverse_mass(&self) -> f32 {
        self.inverse_mass
    }

    /// 获取位置
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// 设置位置（带唤醒选项）
    pub fn set_translation(&mut self, position: Vec3, wake: bool) {
        self.position = position;
        self.transform_dirty = true;
        if wake && self.is_sleeping {
            self.wake_up(true);
        }
    }

    /// 获取旋转
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    /// 设置旋转（带唤醒选项）
    pub fn set_rotation(&mut self, rotation: Quat, wake: bool) {
        self.rotation = rotation.normalize();
        self.transform_dirty = true;
        if wake && self.is_sleeping {
            self.wake_up(true);
        }
    }

    /// 设置位置和旋转（带唤醒选项）
    pub fn set_position(&mut self, position: Vec3, rotation: Quat, wake: bool) {
        self.position = position;
        self.rotation = rotation.normalize();
        self.transform_dirty = true;
        if wake && self.is_sleeping {
            self.wake_up(true);
        }
    }

    /// 获取变换（位置 + 旋转）
    pub fn transform(&self) -> (Vec3, Quat) {
        (self.position, self.rotation)
    }

    /// 获取线性速度
    pub fn linvel(&self) -> Vec3 {
        self.linear_velocity
    }

    /// 设置线性速度（带唤醒选项）
    pub fn set_linvel(&mut self, velocity: Vec3, wake: bool) {
        self.linear_velocity = velocity;
        if wake && self.is_sleeping {
            self.wake_up(true);
        }
    }

    /// 获取角速度
    pub fn angvel(&self) -> Vec3 {
        self.angular_velocity
    }

    /// 设置角速度（带唤醒选项）
    pub fn set_angvel(&mut self, velocity: Vec3, wake: bool) {
        self.angular_velocity = velocity;
        if wake && self.is_sleeping {
            self.wake_up(true);
        }
    }

    /// 应用力（带唤醒选项）
    pub fn apply_force(&mut self, force: Vec3, wake: bool) {
        if self.body_type.is_dynamic() {
            self.force += force;
            if wake && self.is_sleeping {
                self.wake_up(true);
            }
        }
    }

    /// 应用力在某个点（带唤醒选项）
    pub fn apply_force_at_point(&mut self, force: Vec3, point: Vec3, wake: bool) {
        if self.body_type.is_dynamic() {
            self.force += force;
            let r = point - self.position;
            self.torque += r.cross(force);
            if wake && self.is_sleeping {
                self.wake_up(true);
            }
        }
    }

    /// 应用扭矩（带唤醒选项）
    pub fn apply_torque(&mut self, torque: Vec3, wake: bool) {
        if self.body_type.is_dynamic() {
            self.torque += torque;
            if wake && self.is_sleeping {
                self.wake_up(true);
            }
        }
    }

    /// 应用冲量（带唤醒选项）
    pub fn apply_impulse(&mut self, impulse: Vec3, wake: bool) {
        if self.body_type.is_dynamic() {
            self.linear_velocity += impulse * self.inverse_mass;
            if wake && self.is_sleeping {
                self.wake_up(true);
            }
        }
    }

    /// 应用冲量在某个点（带唤醒选项）
    pub fn apply_impulse_at_point(&mut self, impulse: Vec3, point: Vec3, wake: bool) {
        if self.body_type.is_dynamic() {
            self.linear_velocity += impulse * self.inverse_mass;
            let r = point - self.position;
            self.angular_velocity += r.cross(impulse) * self.inverse_mass;
            if wake && self.is_sleeping {
                self.wake_up(true);
            }
        }
    }

    /// 应用扭矩冲量（带唤醒选项）
    pub fn apply_torque_impulse(&mut self, torque_impulse: Vec3, wake: bool) {
        if self.body_type.is_dynamic() {
            self.angular_velocity += torque_impulse * self.inverse_mass;
            if wake && self.is_sleeping {
                self.wake_up(true);
            }
        }
    }

    /// 获取力
    pub fn force(&self) -> Vec3 {
        self.force
    }

    /// 获取扭矩
    pub fn torque(&self) -> Vec3 {
        self.torque
    }

    /// 清空力
    pub fn clear_forces(&mut self) {
        self.force = Vec3::ZERO;
        self.torque = Vec3::ZERO;
    }

    /// 获取线性阻尼
    pub fn linear_damping(&self) -> f32 {
        self.linear_damping
    }

    /// 设置线性阻尼
    pub fn set_linear_damping(&mut self, damping: f32) {
        self.linear_damping = damping;
    }

    /// 获取角阻尼
    pub fn angular_damping(&self) -> f32 {
        self.angular_damping
    }

    /// 设置角阻尼
    pub fn set_angular_damping(&mut self, damping: f32) {
        self.angular_damping = damping;
    }

    /// 获取重力缩放
    pub fn gravity_scale(&self) -> f32 {
        self.gravity_scale
    }

    /// 设置重力缩放
    pub fn set_gravity_scale(&mut self, scale: f32) {
        self.gravity_scale = scale;
    }

    /// 检查是否正在睡眠
    pub fn is_sleeping(&self) -> bool {
        self.is_sleeping
    }

    /// 唤醒刚体
    pub fn wake_up(&mut self, strong: bool) {
        self.is_sleeping = false;
        if strong {
            // 强唤醒会重置睡眠计时器
        }
    }

    /// 使刚体睡眠
    pub fn sleep(&mut self) {
        if self.body_type.is_dynamic() {
            self.is_sleeping = true;
            self.linear_velocity = Vec3::ZERO;
            self.angular_velocity = Vec3::ZERO;
        }
    }

    /// 检查CCD是否启用
    pub fn ccd_enabled(&self) -> bool {
        self.ccd_enabled
    }

    /// 启用/禁用CCD
    pub fn enable_ccd(&mut self, enabled: bool) {
        self.ccd_enabled = enabled;
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 获取主导级分组
    pub fn dominance_group(&self) -> i8 {
        self.dominance_group
    }

    /// 设置主导级分组
    pub fn set_dominance_group(&mut self, group: i8) {
        self.dominance_group = group;
    }

    /// 锁定平移
    pub fn lock_translations(&mut self, x: bool, y: bool, z: bool) {
        self.locked_translations = (x, y, z);
    }

    /// 锁定旋转
    pub fn lock_rotations(&mut self, x: bool, y: bool, z: bool) {
        self.locked_rotations = (x, y, z);
    }

    /// 获取锁定平移状态
    pub fn locked_translations(&self) -> (bool, bool, bool) {
        self.locked_translations
    }

    /// 获取锁定旋转状态
    pub fn locked_rotations(&self) -> (bool, bool, bool) {
        self.locked_rotations
    }

    /// 添加碰撞体索引
    pub fn add_collider_index(&mut self, index: usize) {
        if !self.collider_indices.contains(&index) {
            self.collider_indices.push(index);
        }
    }

    /// 获取碰撞体索引
    pub fn collider_indices(&self) -> &[usize] {
        &self.collider_indices
    }

    /// 更新速度
    pub fn update_velocity(&mut self, dt: f32, gravity: Vec3) {
        if !self.body_type.is_dynamic() || self.is_sleeping {
            return;
        }

        // 应用重力
        self.linear_velocity += gravity * self.gravity_scale * dt;

        // 应用力
        let acceleration = self.force * self.inverse_mass;
        self.linear_velocity += acceleration * dt;

        // 应用扭矩
        let angular_acceleration = self.torque * self.inverse_mass;
        self.angular_velocity += angular_acceleration * dt;

        // 应用阻尼
        self.linear_velocity *= 1.0 - self.linear_damping * dt;
        self.angular_velocity *= 1.0 - self.angular_damping * dt;

        // 应用锁定
        let (lx, ly, lz) = self.locked_translations;
        if lx {
            self.linear_velocity.x = 0.0;
        }
        if ly {
            self.linear_velocity.y = 0.0;
        }
        if lz {
            self.linear_velocity.z = 0.0;
        }

        let (rx, ry, rz) = self.locked_rotations;
        if rx {
            self.angular_velocity.x = 0.0;
        }
        if ry {
            self.angular_velocity.y = 0.0;
        }
        if rz {
            self.angular_velocity.z = 0.0;
        }
    }

    /// 更新位置
    pub fn update_position(&mut self, dt: f32) {
        if !self.body_type.is_dynamic() || self.is_sleeping {
            return;
        }

        self.position += self.linear_velocity * dt;
        // 简化的旋转更新（实际应使用更精确的方法）
        let angvel_mag = self.angular_velocity.length();
        if angvel_mag > 0.0 {
            let _axis = self.angular_velocity.normalize();
            let angle = angvel_mag * dt;
            let delta_rot = Quat::from_rotation_z(angle); // 简化处理
            self.rotation = delta_rot * self.rotation;
            self.rotation = self.rotation.normalize();
        }
        self.transform_dirty = true;
    }

    /// 获取状态
    pub fn state(&self) -> RigidBodyState3D {
        RigidBodyState3D {
            position: self.position,
            rotation: self.rotation,
            linear_velocity: self.linear_velocity,
            angular_velocity: self.angular_velocity,
        }
    }

    /// 设置状态
    pub fn set_state(&mut self, state: RigidBodyState3D) {
        self.position = state.position;
        self.rotation = state.rotation.normalize();
        self.linear_velocity = state.linear_velocity;
        self.angular_velocity = state.angular_velocity;
        self.transform_dirty = true;
    }
}

impl Default for RigidBody3D {
    fn default() -> Self {
        Self::new(RigidBodyType3D::Dynamic)
    }
}

/// 刚体构建器
pub struct RigidBody3DBuilder {
    body: RigidBody3D,
}

impl RigidBody3DBuilder {
    /// 创建动态刚体构建器
    pub fn dynamic() -> Self {
        Self {
            body: RigidBody3D::new(RigidBodyType3D::Dynamic),
        }
    }

    /// 创建静态刚体构建器
    pub fn static_() -> Self {
        Self {
            body: RigidBody3D::new(RigidBodyType3D::Static),
        }
    }

    /// 创建基于位置的运动刚体构建器
    pub fn kinematic_position_based() -> Self {
        Self {
            body: RigidBody3D::new(RigidBodyType3D::KinematicPositionBased),
        }
    }

    /// 创建基于速度的运动刚体构建器
    pub fn kinematic_velocity_based() -> Self {
        Self {
            body: RigidBody3D::new(RigidBodyType3D::KinematicVelocityBased),
        }
    }

    /// 创建固定刚体构建器
    pub fn fixed() -> Self {
        Self {
            body: RigidBody3D::new(RigidBodyType3D::Fixed),
        }
    }

    /// 设置初始位置
    pub fn translation(mut self, position: Vec3) -> Self {
        self.body.position = position;
        self.body.transform_dirty = true;
        self
    }

    /// 设置初始旋转
    pub fn rotation(mut self, rotation: Quat) -> Self {
        self.body.rotation = rotation.normalize();
        self.body.transform_dirty = true;
        self
    }

    /// 设置初始线性速度
    pub fn linvel(mut self, velocity: Vec3) -> Self {
        self.body.linear_velocity = velocity;
        self
    }

    /// 设置初始角速度
    pub fn angvel(mut self, velocity: Vec3) -> Self {
        self.body.angular_velocity = velocity;
        self
    }

    /// 设置质量
    pub fn mass(mut self, mass: f32) -> Self {
        if mass > 0.0 && self.body.body_type.is_dynamic() {
            self.body.mass = mass;
            self.body.inverse_mass = 1.0 / mass;
        }
        self
    }

    /// 设置额外质量
    pub fn additional_mass(mut self, mass: f32) -> Self {
        if self.body.body_type.is_dynamic() {
            self.body.mass += mass;
            if self.body.mass > 0.0 {
                self.body.inverse_mass = 1.0 / self.body.mass;
            }
        }
        self
    }

    /// 设置线性阻尼
    pub fn linear_damping(mut self, damping: f32) -> Self {
        self.body.linear_damping = damping;
        self
    }

    /// 设置角阻尼
    pub fn angular_damping(mut self, damping: f32) -> Self {
        self.body.angular_damping = damping;
        self
    }

    /// 设置重力缩放
    pub fn gravity_scale(mut self, scale: f32) -> Self {
        self.body.gravity_scale = scale;
        self
    }

    /// 启用CCD
    pub fn ccd_enabled(mut self, enabled: bool) -> Self {
        self.body.ccd_enabled = enabled;
        self
    }

    /// 设置睡眠状态
    pub fn sleeping(mut self, sleeping: bool) -> Self {
        self.body.is_sleeping = sleeping;
        self
    }

    /// 设置主导级分组
    pub fn dominance_group(mut self, group: i8) -> Self {
        self.body.dominance_group = group;
        self
    }

    /// 锁定所有平移
    pub fn lock_translations(mut self) -> Self {
        self.body.locked_translations = (true, true, true);
        self
    }

    /// 锁定所有旋转
    pub fn lock_rotations(mut self) -> Self {
        self.body.locked_rotations = (true, true, true);
        self
    }

    /// 限制旋转轴
    pub fn restrict_rotations(mut self, x: bool, y: bool, z: bool) -> Self {
        self.body.locked_rotations = (!x, !y, !z);
        self
    }

    /// 构建刚体
    pub fn build(self) -> RigidBody3D {
        self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_creation() {
        let body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        assert_eq!(body.body_type(), RigidBodyType3D::Dynamic);
        assert_eq!(body.mass(), 1.0);
        assert!(body.is_dynamic());
    }

    #[test]
    fn test_static_body() {
        let body = RigidBody3D::new(RigidBodyType3D::Static);
        assert!(body.is_static());
        assert_eq!(body.mass(), 0.0);
        assert_eq!(body.inverse_mass(), 0.0);
    }

    #[test]
    fn test_apply_force() {
        let mut body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        body.apply_force(Vec3::new(10.0, 0.0, 0.0), false);
        assert_eq!(body.force(), Vec3::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_update_velocity() {
        let mut body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        body.set_linear_damping(0.0);
        body.apply_force(Vec3::new(1.0, 0.0, 0.0), false);
        body.update_velocity(1.0, Vec3::new(0.0, DEFAULT_GRAVITY, 0.0));
        // 速度 = 力 * dt / 质量 = 1.0 * 1.0 / 1.0 = 1.0
        assert!((body.linvel().x - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_update_position() {
        let mut body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        body.set_linvel(Vec3::new(1.0, 0.0, 0.0), false);
        body.update_position(1.0);
        assert_eq!(body.position(), Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_builder_dynamic() {
        let body = RigidBody3DBuilder::dynamic()
            .mass(2.0)
            .translation(Vec3::new(10.0, 20.0, 30.0))
            .linvel(Vec3::new(5.0, 0.0, 0.0))
            .build();

        assert!(body.is_dynamic());
        assert_eq!(body.mass(), 2.0);
        assert_eq!(body.position(), Vec3::new(10.0, 20.0, 30.0));
        assert_eq!(body.linvel(), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_builder_static() {
        let body = RigidBody3DBuilder::static_()
            .translation(Vec3::new(0.0, 0.0, 0.0))
            .build();

        assert!(body.is_static());
        assert_eq!(body.position(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_clear_forces() {
        let mut body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        body.apply_force(Vec3::new(10.0, 0.0, 0.0), false);
        body.clear_forces();
        assert_eq!(body.force(), Vec3::ZERO);
    }

    #[test]
    fn test_impulse() {
        let mut body = RigidBody3DBuilder::dynamic().mass(1.0).build();
        body.apply_impulse(Vec3::new(5.0, 0.0, 0.0), false);
        assert_eq!(body.linvel(), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_sleep_wake() {
        let mut body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        assert!(!body.is_sleeping());
        body.sleep();
        assert!(body.is_sleeping());
        body.wake_up(true);
        assert!(!body.is_sleeping());
    }

    #[test]
    fn test_lock_rotations() {
        let mut body = RigidBody3DBuilder::dynamic()
            .restrict_rotations(true, false, true)
            .build();
        assert_eq!(body.locked_rotations(), (false, true, false));
    }

    #[test]
    fn test_gravity_scale() {
        let mut body = RigidBody3DBuilder::dynamic().gravity_scale(2.0).build();
        assert_eq!(body.gravity_scale(), 2.0);
    }
}
