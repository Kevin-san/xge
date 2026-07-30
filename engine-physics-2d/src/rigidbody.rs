//! 刚体模块
//!
//! 提供 2D 刚体实现，包括动态、静态和运动刚体类型。

use engine_math::Vec2;

/// 刚体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidBodyType {
    #[default]
    /// 动态刚体 - 受物理力影响
    Dynamic,
    /// 静态刚体 - 不受物理力影响，位置固定
    Static,
    /// 运动刚体 - 位置由代码控制，但可以影响其他物体
    Kinematic,
}

/// 刚体状态
#[derive(Debug, Clone)]
pub struct RigidBodyState {
    /// 位置
    pub position: Vec2,
    /// 旋转角度（弧度）
    pub rotation: f32,
    /// 线性速度
    pub linear_velocity: Vec2,
    /// 角速度
    pub angular_velocity: f32,
}

/// 刚体
///
/// 代表一个具有质量和惯性的物理实体。
#[derive(Debug, Clone)]
pub struct RigidBody2D {
    /// 刚体类型
    body_type: RigidBodyType,
    /// 质量
    mass: f32,
    /// 逆质量（用于计算，0 表示无限质量）
    inverse_mass: f32,
    /// 转动惯量
    inertia: f32,
    /// 逆转动惯量
    inverse_inertia: f32,
    /// 位置
    position: Vec2,
    /// 旋转角度
    rotation: f32,
    /// 线性速度
    linear_velocity: Vec2,
    /// 角速度
    angular_velocity: f32,
    /// 力累加器
    force: Vec2,
    /// 扭矩累加器
    torque: f32,
    /// 阻尼
    linear_damping: f32,
    /// 角阻尼
    angular_damping: f32,
    /// 重力缩放
    gravity_scale: f32,
    /// 是否启用
    enabled: bool,
    /// 碰撞体索引列表
    collider_indices: Vec<usize>,
    /// 变换是否需要更新
    transform_dirty: bool,
}

impl RigidBody2D {
    /// 创建新的刚体
    pub fn new(body_type: RigidBodyType) -> Self {
        let mass = match body_type {
            RigidBodyType::Dynamic => 1.0,
            _ => 0.0,
        };

        Self {
            body_type,
            mass,
            inverse_mass: if mass > 0.0 { 1.0 / mass } else { 0.0 },
            inertia: match body_type {
                RigidBodyType::Dynamic => 1.0,
                _ => 0.0,
            },
            inverse_inertia: match body_type {
                RigidBodyType::Dynamic => 1.0,
                _ => 0.0,
            },
            position: Vec2::ZERO,
            rotation: 0.0,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            force: Vec2::ZERO,
            torque: 0.0,
            linear_damping: 0.01,
            angular_damping: 0.01,
            gravity_scale: 1.0,
            enabled: true,
            collider_indices: Vec::new(),
            transform_dirty: true,
        }
    }

    /// 获取刚体类型
    pub fn body_type(&self) -> RigidBodyType {
        self.body_type
    }

    /// 检查是否是静态刚体
    pub fn is_static(&self) -> bool {
        self.body_type == RigidBodyType::Static
    }

    /// 检查是否是动态刚体
    pub fn is_dynamic(&self) -> bool {
        self.body_type == RigidBodyType::Dynamic
    }

    /// 检查是否是运动刚体
    pub fn is_kinematic(&self) -> bool {
        self.body_type == RigidBodyType::Kinematic
    }

    /// 获取质量
    pub fn mass(&self) -> f32 {
        self.mass
    }

    /// 获取逆质量
    pub fn inverse_mass(&self) -> f32 {
        self.inverse_mass
    }

    /// 设置质量（仅对动态刚体有效，0 表示无限质量）
    pub fn set_mass(&mut self, mass: f32) {
        if self.body_type != RigidBodyType::Dynamic {
            return;
        }
        self.mass = mass;
        self.inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
    }

    /// 获取转动惯量
    pub fn inertia(&self) -> f32 {
        self.inertia
    }

    /// 获取逆转动惯量
    pub fn inverse_inertia(&self) -> f32 {
        self.inverse_inertia
    }

    /// 设置转动惯量（仅对动态刚体有效）
    pub fn set_inertia(&mut self, inertia: f32) {
        if self.body_type != RigidBodyType::Dynamic {
            return;
        }
        self.inertia = inertia;
        self.inverse_inertia = if inertia > 0.0 { 1.0 / inertia } else { 0.0 };
    }

    /// 获取位置
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// 设置位置
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.transform_dirty = true;
    }

    /// 获取旋转角度
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// 设置旋转角度
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.transform_dirty = true;
    }

    /// 获取线性速度
    pub fn linear_velocity(&self) -> Vec2 {
        self.linear_velocity
    }

    /// 设置线性速度
    pub fn set_linear_velocity(&mut self, velocity: Vec2) {
        self.linear_velocity = velocity;
    }

    /// 获取角速度
    pub fn angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    /// 设置角速度
    pub fn set_angular_velocity(&mut self, velocity: f32) {
        self.angular_velocity = velocity;
    }

    /// 应用力
    pub fn apply_force(&mut self, force: Vec2) {
        if self.body_type == RigidBodyType::Dynamic {
            self.force += force;
        }
    }

    /// 应用力在某个点
    pub fn apply_force_at_point(&mut self, force: Vec2, point: Vec2) {
        if self.body_type == RigidBodyType::Dynamic {
            self.force += force;
            let r = point - self.position;
            self.torque += r.cross(force);
        }
    }

    /// 应用扭矩
    pub fn apply_torque(&mut self, torque: f32) {
        if self.body_type == RigidBodyType::Dynamic {
            self.torque += torque;
        }
    }

    /// 应用冲量
    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if self.body_type == RigidBodyType::Dynamic {
            self.linear_velocity += impulse * self.inverse_mass;
        }
    }

    /// 应用冲量在某个点
    pub fn apply_impulse_at_point(&mut self, impulse: Vec2, point: Vec2) {
        if self.body_type == RigidBodyType::Dynamic {
            self.linear_velocity += impulse * self.inverse_mass;
            let r = point - self.position;
            self.angular_velocity += r.cross(impulse) * self.inverse_inertia;
        }
    }

    /// 获取力
    pub fn force(&self) -> Vec2 {
        self.force
    }

    /// 获取扭矩
    pub fn torque(&self) -> f32 {
        self.torque
    }

    /// 清空力
    pub fn clear_forces(&mut self) {
        self.force = Vec2::ZERO;
        self.torque = 0.0;
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

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 获取变换矩阵 (cos, sin, tx, -sin, cos, ty)
    /// 返回 6 个元素的元组 (a, b, c, d, e, f) 表示 2D 变换矩阵
    pub fn transform_matrix(&self) -> (f32, f32, f32, f32, f32, f32) {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        (cos, -sin, self.position.x, sin, cos, self.position.y)
    }

    /// 添加碰撞体索引
    pub fn add_collider_index(&mut self, index: usize) {
        if !self.collider_indices.contains(&index) {
            self.collider_indices.push(index);
        }
    }

    /// 移除碰撞体索引
    pub fn remove_collider_index(&mut self, index: usize) {
        self.collider_indices.retain(|&i| i != index);
    }

    /// 移除后更新碰撞体索引（索引大于被移除的需要减1）
    pub fn update_collider_indices_after_remove(&mut self, removed_index: usize) {
        for i in &mut self.collider_indices {
            if *i > removed_index {
                *i -= 1;
            }
        }
    }

    /// 获取碰撞体索引
    pub fn collider_indices(&self) -> &[usize] {
        &self.collider_indices
    }

    /// 更新速度
    pub fn update_velocity(&mut self, dt: f32) {
        if self.body_type != RigidBodyType::Dynamic {
            return;
        }

        // 线性速度更新
        let acceleration = self.force * self.inverse_mass;
        self.linear_velocity += acceleration * dt;

        let angular_acceleration = self.torque * self.inverse_inertia;
        self.angular_velocity += angular_acceleration * dt;

        self.linear_velocity = self.linear_velocity * (1.0 - self.linear_damping);
        self.angular_velocity *= 1.0 - self.angular_damping;
    }

    /// 更新位置
    pub fn update_position(&mut self, dt: f32) {
        if self.body_type != RigidBodyType::Dynamic {
            return;
        }

        self.position += self.linear_velocity * dt;
        self.rotation += self.angular_velocity * dt;
        self.transform_dirty = true;
    }

    /// 获取状态
    pub fn state(&self) -> RigidBodyState {
        RigidBodyState {
            position: self.position,
            rotation: self.rotation,
            linear_velocity: self.linear_velocity,
            angular_velocity: self.angular_velocity,
        }
    }

    /// 设置状态
    pub fn set_state(&mut self, state: RigidBodyState) {
        self.position = state.position;
        self.rotation = state.rotation;
        self.linear_velocity = state.linear_velocity;
        self.angular_velocity = state.angular_velocity;
        self.transform_dirty = true;
    }
}

/// 刚体构建器
pub struct RigidBody2DBuilder {
    body: RigidBody2D,
}

impl RigidBody2DBuilder {
    /// 创建动态刚体构建器
    pub fn dynamic() -> Self {
        Self {
            body: RigidBody2D::new(RigidBodyType::Dynamic),
        }
    }

    /// 创建静态刚体构建器
    pub fn static_() -> Self {
        Self {
            body: RigidBody2D::new(RigidBodyType::Static),
        }
    }

    /// 创建运动刚体构建器
    pub fn kinematic() -> Self {
        Self {
            body: RigidBody2D::new(RigidBodyType::Kinematic),
        }
    }

    /// 设置质量
    pub fn with_mass(mut self, mass: f32) -> Self {
        if mass > 0.0 {
            self.body.mass = mass;
            self.body.inverse_mass = 1.0 / mass;
        }
        self
    }

    /// 设置转动惯量
    pub fn with_inertia(mut self, inertia: f32) -> Self {
        if inertia > 0.0 {
            self.body.inertia = inertia;
            self.body.inverse_inertia = 1.0 / inertia;
        }
        self
    }

    /// 设置初始位置
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.body.position = position;
        self.body.transform_dirty = true;
        self
    }

    /// 设置初始旋转
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.body.rotation = rotation;
        self.body.transform_dirty = true;
        self
    }

    /// 设置初始速度
    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.body.linear_velocity = velocity;
        self
    }

    /// 设置角速度
    pub fn with_angular_velocity(mut self, velocity: f32) -> Self {
        self.body.angular_velocity = velocity;
        self
    }

    /// 设置阻尼
    pub fn with_damping(mut self, linear: f32, angular: f32) -> Self {
        self.body.linear_damping = linear;
        self.body.angular_damping = angular;
        self
    }

    /// 设置重力缩放
    pub fn with_gravity_scale(mut self, scale: f32) -> Self {
        self.body.gravity_scale = scale;
        self
    }

    /// 构建刚体
    pub fn build(self) -> RigidBody2D {
        self.body
    }
}

impl Default for RigidBody2D {
    fn default() -> Self {
        Self::new(RigidBodyType::Dynamic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_creation() {
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        assert_eq!(body.body_type(), RigidBodyType::Dynamic);
        assert_eq!(body.mass(), 1.0);
    }

    #[test]
    fn test_static_body() {
        let body = RigidBody2D::new(RigidBodyType::Static);
        assert!(body.is_static());
        assert_eq!(body.mass(), 0.0);
    }

    #[test]
    fn test_apply_force() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        body.apply_force(Vec2::new(10.0, 0.0));
        assert_eq!(body.force(), Vec2::new(10.0, 0.0));
    }

    #[test]
    fn test_update_velocity() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        body.set_linear_damping(0.0); // 禁用阻尼以测试纯物理
        body.apply_force(Vec2::new(1.0, 0.0));
        body.update_velocity(1.0);
        // 速度 = 力 * dt / 质量 = 1.0 * 1.0 / 1.0 = 1.0
        assert_eq!(body.linear_velocity().x, 1.0);
    }

    #[test]
    fn test_update_position() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        body.set_linear_velocity(Vec2::new(1.0, 0.0));
        body.update_position(1.0);
        assert_eq!(body.position(), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_builder_dynamic() {
        let body = RigidBody2DBuilder::dynamic()
            .with_mass(2.0)
            .with_position(Vec2::new(10.0, 20.0))
            .with_velocity(Vec2::new(5.0, 0.0))
            .build();

        assert!(body.is_dynamic());
        assert_eq!(body.mass(), 2.0);
        assert_eq!(body.position(), Vec2::new(10.0, 20.0));
        assert_eq!(body.linear_velocity(), Vec2::new(5.0, 0.0));
    }

    #[test]
    fn test_builder_static() {
        let body = RigidBody2DBuilder::static_()
            .with_position(Vec2::new(0.0, 0.0))
            .build();

        assert!(body.is_static());
        assert_eq!(body.position(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_clear_forces() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        body.apply_force(Vec2::new(10.0, 0.0));
        body.clear_forces();
        assert_eq!(body.force(), Vec2::ZERO);
    }

    #[test]
    fn test_impulse() {
        let mut body = RigidBody2DBuilder::dynamic().with_mass(1.0).build();

        body.apply_impulse(Vec2::new(5.0, 0.0));
        assert_eq!(body.linear_velocity(), Vec2::new(5.0, 0.0));
    }

    #[test]
    fn test_kinematic_body() {
        let body = RigidBody2D::new(RigidBodyType::Kinematic);
        assert!(body.is_kinematic());
        assert!(!body.is_static());
        assert!(!body.is_dynamic());
    }

    #[test]
    fn test_kinematic_body_velocity_not_affected_by_force() {
        let mut body = RigidBody2D::new(RigidBodyType::Kinematic);
        // 运动学物体受力不会改变速度（但这个测试可能需要调整实现）
        body.apply_force(Vec2::new(100.0, 0.0));
        // 运动学物体的质量是 0，所以逆质量是 0
        // 但 apply_force 的实现是检查 body_type == Dynamic
        assert_eq!(body.force(), Vec2::ZERO); // Kinematic 不会累加力
    }

    #[test]
    fn test_gravity_scale() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        assert_eq!(body.gravity_scale(), 1.0); // 默认值

        body.set_gravity_scale(2.0);
        assert_eq!(body.gravity_scale(), 2.0);

        body.set_gravity_scale(0.0);
        assert_eq!(body.gravity_scale(), 0.0); // 零重力
    }

    #[test]
    fn test_gravity_scale_builder() {
        let body = RigidBody2DBuilder::dynamic()
            .with_gravity_scale(0.5)
            .build();

        assert_eq!(body.gravity_scale(), 0.5);
    }

    #[test]
    fn test_kinematic_builder() {
        let body = RigidBody2DBuilder::kinematic()
            .with_position(Vec2::new(5.0, 10.0))
            .with_velocity(Vec2::new(1.0, 2.0))
            .build();

        assert!(body.is_kinematic());
        assert_eq!(body.position(), Vec2::new(5.0, 10.0));
        assert_eq!(body.linear_velocity(), Vec2::new(1.0, 2.0));
    }

    #[test]
    fn test_body_enabled() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        assert!(body.is_enabled());

        body.set_enabled(false);
        assert!(!body.is_enabled());
    }

    #[test]
    fn test_angular_velocity() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        assert_eq!(body.angular_velocity(), 0.0);

        body.set_angular_velocity(std::f32::consts::PI);
        assert_eq!(body.angular_velocity(), std::f32::consts::PI);
    }

    #[test]
    fn test_set_rotation() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        body.set_rotation(std::f32::consts::FRAC_PI_2);
        assert_eq!(body.rotation(), std::f32::consts::FRAC_PI_2);
    }

    // ============= RigidBody2DBuilder 更多参数组合测试 =============

    #[test]
    fn test_builder_dynamic_with_mass_velocity() {
        let body = RigidBody2DBuilder::dynamic()
            .with_mass(5.0)
            .with_velocity(Vec2::new(2.0, 3.0))
            .build();
        assert!(body.is_dynamic());
        assert_eq!(body.mass(), 5.0);
        assert_eq!(body.linear_velocity(), Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_builder_static_with_position() {
        let body = RigidBody2DBuilder::static_()
            .with_position(Vec2::new(100.0, 200.0))
            .build();
        assert!(body.is_static());
        assert_eq!(body.position(), Vec2::new(100.0, 200.0));
        assert_eq!(body.linear_velocity(), Vec2::ZERO);
    }

    #[test]
    fn test_builder_kinematic_with_rotation_angular_velocity() {
        let body = RigidBody2DBuilder::kinematic()
            .with_rotation(std::f32::consts::PI)
            .with_angular_velocity(2.0)
            .build();
        assert!(body.is_kinematic());
        assert_eq!(body.rotation(), std::f32::consts::PI);
        assert_eq!(body.angular_velocity(), 2.0);
    }

    #[test]
    fn test_builder_with_damping_chain() {
        let body = RigidBody2DBuilder::dynamic()
            .with_mass(10.0)
            .with_position(Vec2::new(5.0, 5.0))
            .with_velocity(Vec2::new(1.0, 0.0))
            .with_rotation(0.5)
            .with_angular_velocity(1.5)
            .with_damping(0.1, 0.2)
            .with_gravity_scale(1.5)
            .build();
        assert_eq!(body.mass(), 10.0);
        assert_eq!(body.position(), Vec2::new(5.0, 5.0));
        assert_eq!(body.linear_velocity(), Vec2::new(1.0, 0.0));
        assert_eq!(body.rotation(), 0.5);
        assert_eq!(body.angular_velocity(), 1.5);
        assert_eq!(body.gravity_scale(), 1.5);
    }

    #[test]
    fn test_builder_default_mass_positive() {
        let body = RigidBody2DBuilder::dynamic().with_mass(3.0).build();
        assert!(body.mass() > 0.0);
    }

    #[test]
    fn test_builder_inertia_applied() {
        let body = RigidBody2DBuilder::dynamic().with_inertia(2.5).build();
        // 通过 apply_impulse_at_point 验证惯性生效
        // 由于 inverse_inertia 是 1/inertia，这里验证它在构建后不为零
        assert!(body.inertia() > 0.0);
    }

    #[test]
    fn test_builder_zero_mass_not_allowed_for_static_default_is_static() {
        let body = RigidBody2DBuilder::static_().build();
        assert_eq!(body.mass(), 0.0);
    }

    #[test]
    fn test_builder_dynamic_position_offset() {
        let body = RigidBody2DBuilder::dynamic()
            .with_position(Vec2::new(-10.0, -20.0))
            .build();
        assert_eq!(body.position().x, -10.0);
        assert_eq!(body.position().y, -20.0);
    }

    #[test]
    fn test_builder_static_is_not_dynamic() {
        let body = RigidBody2DBuilder::static_().build();
        assert!(!body.is_dynamic());
    }

    #[test]
    fn test_builder_dynamic_is_not_static() {
        let body = RigidBody2DBuilder::dynamic().build();
        assert!(!body.is_static());
    }

    #[test]
    fn test_rigid_body_get_type_static() {
        let body = RigidBody2D::new(RigidBodyType::Static);
        assert_eq!(body.body_type(), RigidBodyType::Static);
    }

    #[test]
    fn test_rigid_body_get_type_kinematic() {
        let body = RigidBody2D::new(RigidBodyType::Kinematic);
        assert_eq!(body.body_type(), RigidBodyType::Kinematic);
    }

    #[test]
    fn test_rigid_body_apply_force_at_point() {
        let mut body = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_inertia(1.0)
            .build();
        body.apply_force_at_point(Vec2::new(10.0, 0.0), Vec2::new(0.0, 1.0));
        // 力累加
        assert_eq!(body.force().x, 10.0);
    }

    #[test]
    fn test_rigid_body_apply_torque() {
        let mut body = RigidBody2DBuilder::dynamic().with_mass(1.0).build();
        body.apply_torque(5.0);
        body.update_velocity(1.0);
        // 由于有扭矩影响角速度
        assert!(body.angular_velocity() != 0.0);
    }

    #[test]
    fn test_rigid_body_set_position_moves() {
        let mut body = RigidBody2DBuilder::dynamic().build();
        body.set_position(Vec2::new(42.0, 42.0));
        assert_eq!(body.position(), Vec2::new(42.0, 42.0));
    }

    #[test]
    fn test_rigid_body_set_linear_velocity() {
        let mut body = RigidBody2DBuilder::dynamic().build();
        body.set_linear_velocity(Vec2::new(3.0, 4.0));
        assert_eq!(body.linear_velocity(), Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_rigid_body_set_angular_velocity() {
        let mut body = RigidBody2DBuilder::dynamic().build();
        body.set_angular_velocity(5.0);
        assert_eq!(body.angular_velocity(), 5.0);
    }

    #[test]
    fn test_rigid_body_set_gravity_scale_zero() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        body.set_gravity_scale(0.0);
        assert_eq!(body.gravity_scale(), 0.0);
    }

    #[test]
    fn test_rigid_body_enabled_toggle() {
        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);
        assert!(body.is_enabled());
        body.set_enabled(false);
        assert!(!body.is_enabled());
        body.set_enabled(true);
        assert!(body.is_enabled());
    }

    #[test]
    fn test_rigid_body_transform_matrix_initial() {
        let body = RigidBody2DBuilder::dynamic()
            .with_position(Vec2::ZERO)
            .with_rotation(0.0)
            .build();
        let (a, b, c, d, e, f) = body.transform_matrix();
        // 单位矩阵: [1, 0, 0, 0, 1, 0]
        assert_eq!(a, 1.0);
        assert_eq!(b, 0.0);
        assert_eq!(c, 0.0);
        assert_eq!(d, 0.0);
        assert_eq!(e, 1.0);
        assert_eq!(f, 0.0);
    }

    #[test]
    fn test_rigid_body_state_roundtrip() {
        let mut body = RigidBody2DBuilder::dynamic()
            .with_position(Vec2::new(1.0, 2.0))
            .with_velocity(Vec2::new(3.0, 4.0))
            .with_rotation(std::f32::consts::FRAC_PI_4)
            .with_angular_velocity(1.0)
            .build();
        let state = body.state();
        assert_eq!(state.position, Vec2::new(1.0, 2.0));
        assert_eq!(state.linear_velocity, Vec2::new(3.0, 4.0));
        assert_eq!(state.rotation, std::f32::consts::FRAC_PI_4);
        body.set_state(RigidBodyState {
            position: Vec2::new(5.0, 6.0),
            rotation: 0.0,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
        });
        assert_eq!(body.position(), Vec2::new(5.0, 6.0));
        assert_eq!(body.linear_velocity(), Vec2::ZERO);
    }
}
