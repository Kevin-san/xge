//! 关节模块
//!
//! 提供 3D 物理关节实现，包括固定关节、旋转关节、滑块关节、球关节等。

use engine_math::{Quat, Vec3};

/// 关节类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType3D {
    /// 固定关节
    Fixed,
    /// 旋转关节
    Revolute,
    /// 滑块关节
    Prismatic,
    /// 球关节
    Ball,
    /// 距离关节
    Distance,
    /// 绳索关节
    Rope,
    /// 球面关节
    Spherical,
}

/// 关节句柄
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JointHandle {
    /// 索引
    pub index: u32,
    /// 版本号
    pub generation: u32,
}

impl JointHandle {
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

impl Default for JointHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

/// 马达模型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotorModel {
    /// 力控制
    #[default]
    ForceBased,
    /// 速度控制
    VelocityBased,
    /// 位置控制
    PositionBased,
}

/// 关节基类
#[derive(Debug, Clone)]
pub struct Joint3D {
    /// 关节类型
    joint_type: JointType3D,
    /// 连接的刚体 A 索引
    body_a: usize,
    /// 连接的刚体 B 索引
    body_b: usize,
    /// 是否启用
    enabled: bool,
    /// 碰撞连接的两个物体
    collide_connected: bool,
}

impl Joint3D {
    /// 创建新的关节
    pub fn new(joint_type: JointType3D, body_a: usize, body_b: usize) -> Self {
        Self {
            joint_type,
            body_a,
            body_b,
            enabled: true,
            collide_connected: false,
        }
    }

    /// 获取关节类型
    pub fn joint_type(&self) -> JointType3D {
        self.joint_type
    }

    /// 获取刚体 A 索引
    pub fn body_a(&self) -> usize {
        self.body_a
    }

    /// 获取刚体 B 索引
    pub fn body_b(&self) -> usize {
        self.body_b
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 检查是否与连接物体碰撞
    pub fn collide_connected(&self) -> bool {
        self.collide_connected
    }

    /// 设置是否与连接物体碰撞
    pub fn set_collide_connected(&mut self, collide: bool) {
        self.collide_connected = collide;
    }
}

/// 固定关节
///
/// 完全固定两个刚体之间的相对位置和旋转。
#[derive(Debug, Clone)]
pub struct FixedJoint {
    /// 基类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 刚体A的局部基准
    local_basis1: Quat,
    /// 刚体B的局部基准
    local_basis2: Quat,
}

impl FixedJoint {
    /// 创建新的固定关节
    pub fn new(body_a: usize, body_b: usize) -> Self {
        Self {
            base: Joint3D::new(JointType3D::Fixed, body_a, body_b),
            local_anchor1: Vec3::ZERO,
            local_anchor2: Vec3::ZERO,
            local_basis1: Quat::IDENTITY,
            local_basis2: Quat::IDENTITY,
        }
    }

    /// 获取刚体A的局部锚点
    pub fn local_anchor1(&self) -> Vec3 {
        self.local_anchor1
    }

    /// 设置刚体A的局部锚点
    pub fn set_local_anchor1(&mut self, anchor: Vec3) {
        self.local_anchor1 = anchor;
    }

    /// 获取刚体B的局部锚点
    pub fn local_anchor2(&self) -> Vec3 {
        self.local_anchor2
    }

    /// 设置刚体B的局部锚点
    pub fn set_local_anchor2(&mut self, anchor: Vec3) {
        self.local_anchor2 = anchor;
    }

    /// 获取刚体A的局部基准
    pub fn local_basis1(&self) -> Quat {
        self.local_basis1
    }

    /// 设置刚体A的局部基准
    pub fn set_local_basis1(&mut self, basis: Quat) {
        self.local_basis1 = basis.normalize();
    }

    /// 获取刚体B的局部基准
    pub fn local_basis2(&self) -> Quat {
        self.local_basis2
    }

    /// 设置刚体B的局部基准
    pub fn set_local_basis2(&mut self, basis: Quat) {
        self.local_basis2 = basis.normalize();
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 固定关节构建器
pub struct FixedJointBuilder {
    joint: FixedJoint,
}

impl FixedJointBuilder {
    /// 创建新的固定关节构建器
    pub fn new(body_a: usize, body_b: usize) -> Self {
        Self {
            joint: FixedJoint::new(body_a, body_b),
        }
    }

    /// 设置刚体A的局部锚点
    pub fn local_anchor1(mut self, anchor: Vec3) -> Self {
        self.joint.set_local_anchor1(anchor);
        self
    }

    /// 设置刚体B的局部锚点
    pub fn local_anchor2(mut self, anchor: Vec3) -> Self {
        self.joint.set_local_anchor2(anchor);
        self
    }

    /// 设置刚体A的局部基准
    pub fn local_basis1(mut self, basis: Quat) -> Self {
        self.joint.set_local_basis1(basis);
        self
    }

    /// 设置刚体B的局部基准
    pub fn local_basis2(mut self, basis: Quat) -> Self {
        self.joint.set_local_basis2(basis);
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Fixed,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

/// 旋转关节
///
/// 两个刚体围绕指定轴旋转。
#[derive(Debug, Clone)]
pub struct RevoluteJoint {
    /// 基类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 旋转轴
    axis: Vec3,
    /// 角度限制最小值
    limits_min: Option<f32>,
    /// 角度限制最大值
    limits_max: Option<f32>,
    /// 马达模型
    motor_model: MotorModel,
    /// 马达目标速度
    motor_velocity: f32,
    /// 马达因子
    motor_factor: f32,
    /// 马达目标位置
    motor_position: f32,
    /// 马达刚度
    motor_stiffness: f32,
    /// 马达阻尼
    motor_damping: f32,
}

impl RevoluteJoint {
    /// 创建新的旋转关节
    pub fn new(body_a: usize, body_b: usize, axis: Vec3) -> Self {
        Self {
            base: Joint3D::new(JointType3D::Revolute, body_a, body_b),
            local_anchor1: Vec3::ZERO,
            local_anchor2: Vec3::ZERO,
            axis: axis.normalize(),
            limits_min: None,
            limits_max: None,
            motor_model: MotorModel::default(),
            motor_velocity: 0.0,
            motor_factor: 0.0,
            motor_position: 0.0,
            motor_stiffness: 0.0,
            motor_damping: 0.0,
        }
    }

    /// 获取旋转轴
    pub fn axis(&self) -> Vec3 {
        self.axis
    }

    /// 设置旋转轴
    pub fn set_axis(&mut self, axis: Vec3) {
        self.axis = axis.normalize();
    }

    /// 获取刚体A的局部锚点
    pub fn local_anchor1(&self) -> Vec3 {
        self.local_anchor1
    }

    /// 设置刚体A的局部锚点
    pub fn set_local_anchor1(&mut self, anchor: Vec3) {
        self.local_anchor1 = anchor;
    }

    /// 获取刚体B的局部锚点
    pub fn local_anchor2(&self) -> Vec3 {
        self.local_anchor2
    }

    /// 设置刚体B的局部锚点
    pub fn set_local_anchor2(&mut self, anchor: Vec3) {
        self.local_anchor2 = anchor;
    }

    /// 设置角度限制
    pub fn set_limits(&mut self, min: f32, max: f32) {
        self.limits_min = Some(min);
        self.limits_max = Some(max);
    }

    /// 获取角度限制
    pub fn limits(&self) -> (Option<f32>, Option<f32>) {
        (self.limits_min, self.limits_max)
    }

    /// 设置马达模型
    pub fn set_motor_model(&mut self, model: MotorModel) {
        self.motor_model = model;
    }

    /// 设置马达速度
    pub fn set_motor_velocity(&mut self, velocity: f32, factor: f32) {
        self.motor_velocity = velocity;
        self.motor_factor = factor;
    }

    /// 设置马达位置
    pub fn set_motor_position(&mut self, position: f32, stiffness: f32, damping: f32) {
        self.motor_position = position;
        self.motor_stiffness = stiffness;
        self.motor_damping = damping;
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 旋转关节构建器
pub struct RevoluteJointBuilder {
    joint: RevoluteJoint,
}

impl RevoluteJointBuilder {
    /// 创建新的旋转关节构建器
    pub fn new(axis: Vec3) -> Self {
        Self {
            joint: RevoluteJoint::new(0, 1, axis),
        }
    }

    /// 设置刚体索引
    pub fn bodies(mut self, body_a: usize, body_b: usize) -> Self {
        self.joint.base.body_a = body_a;
        self.joint.base.body_b = body_b;
        self
    }

    /// 设置刚体A的局部锚点
    pub fn local_anchor1(mut self, anchor: Vec3) -> Self {
        self.joint.set_local_anchor1(anchor);
        self
    }

    /// 设置刚体B的局部锚点
    pub fn local_anchor2(mut self, anchor: Vec3) -> Self {
        self.joint.set_local_anchor2(anchor);
        self
    }

    /// 设置马达模型
    pub fn motor_model(mut self, model: MotorModel) -> Self {
        self.joint.set_motor_model(model);
        self
    }

    /// 设置角度限制
    pub fn limits(mut self, min: f32, max: f32) -> Self {
        self.joint.set_limits(min, max);
        self
    }

    /// 设置马达速度
    pub fn motor_velocity(mut self, velocity: f32, factor: f32) -> Self {
        self.joint.set_motor_velocity(velocity, factor);
        self
    }

    /// 设置马达位置
    pub fn motor_position(mut self, position: f32, stiffness: f32, damping: f32) -> Self {
        self.joint.set_motor_position(position, stiffness, damping);
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Revolute,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

/// 滑块关节
///
/// 两个刚体沿指定轴相对滑动。
#[derive(Debug, Clone)]
pub struct PrismaticJoint {
    /// 基类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 滑动轴
    axis: Vec3,
    /// 距离限制最小值
    limits_min: Option<f32>,
    /// 距离限制最大值
    limits_max: Option<f32>,
}

impl PrismaticJoint {
    /// 创建新的滑块关节
    pub fn new(body_a: usize, body_b: usize, axis: Vec3) -> Self {
        Self {
            base: Joint3D::new(JointType3D::Prismatic, body_a, body_b),
            local_anchor1: Vec3::ZERO,
            local_anchor2: Vec3::ZERO,
            axis: axis.normalize(),
            limits_min: None,
            limits_max: None,
        }
    }

    /// 获取滑动轴
    pub fn axis(&self) -> Vec3 {
        self.axis
    }

    /// 设置滑动轴
    pub fn set_axis(&mut self, axis: Vec3) {
        self.axis = axis.normalize();
    }

    /// 设置距离限制
    pub fn set_limits(&mut self, min: f32, max: f32) {
        self.limits_min = Some(min);
        self.limits_max = Some(max);
    }

    /// 获取距离限制
    pub fn limits(&self) -> (Option<f32>, Option<f32>) {
        (self.limits_min, self.limits_max)
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 滑块关节构建器
pub struct PrismaticJointBuilder {
    joint: PrismaticJoint,
}

impl PrismaticJointBuilder {
    /// 创建新的滑块关节构建器
    pub fn new(axis: Vec3) -> Self {
        Self {
            joint: PrismaticJoint::new(0, 1, axis),
        }
    }

    /// 设置刚体索引
    pub fn bodies(mut self, body_a: usize, body_b: usize) -> Self {
        self.joint.base.body_a = body_a;
        self.joint.base.body_b = body_b;
        self
    }

    /// 设置距离限制
    pub fn limits(mut self, min: f32, max: f32) -> Self {
        self.joint.set_limits(min, max);
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Prismatic,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

/// 球关节
///
/// 两个刚体围绕锚点自由旋转。
#[derive(Debug, Clone)]
pub struct BallJoint {
    /// 基类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 最大角度限制
    limits_max_angle: Option<f32>,
}

impl BallJoint {
    /// 创建新的球关节
    pub fn new(body_a: usize, body_b: usize, anchor1: Vec3, anchor2: Vec3) -> Self {
        Self {
            base: Joint3D::new(JointType3D::Ball, body_a, body_b),
            local_anchor1: anchor1,
            local_anchor2: anchor2,
            limits_max_angle: None,
        }
    }

    /// 获取刚体A的局部锚点
    pub fn local_anchor1(&self) -> Vec3 {
        self.local_anchor1
    }

    /// 获取刚体B的局部锚点
    pub fn local_anchor2(&self) -> Vec3 {
        self.local_anchor2
    }

    /// 设置角度限制
    pub fn set_limits(&mut self, max_angle: f32) {
        self.limits_max_angle = Some(max_angle);
    }

    /// 获取角度限制
    pub fn limits(&self) -> Option<f32> {
        self.limits_max_angle
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 球关节构建器
pub struct BallJointBuilder {
    joint: BallJoint,
}

impl BallJointBuilder {
    /// 创建新的球关节构建器
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self {
        Self {
            joint: BallJoint::new(0, 1, anchor1, anchor2),
        }
    }

    /// 设置刚体索引
    pub fn bodies(mut self, body_a: usize, body_b: usize) -> Self {
        self.joint.base.body_a = body_a;
        self.joint.base.body_b = body_b;
        self
    }

    /// 设置角度限制
    pub fn limits(mut self, max_angle: f32) -> Self {
        self.joint.set_limits(max_angle);
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Ball,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

/// 距离关节
///
/// 保持两个锚点之间的距离不变。
#[derive(Debug, Clone)]
pub struct DistanceJoint {
    /// 基类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 目标距离
    length: f32,
}

impl DistanceJoint {
    /// 创建新的距离关节
    pub fn new(body_a: usize, body_b: usize, anchor1: Vec3, anchor2: Vec3) -> Self {
        let length = (anchor2 - anchor1).length();
        Self {
            base: Joint3D::new(JointType3D::Distance, body_a, body_b),
            local_anchor1: anchor1,
            local_anchor2: anchor2,
            length,
        }
    }

    /// 获取目标距离
    pub fn length(&self) -> f32 {
        self.length
    }

    /// 设置目标距离
    pub fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 距离关节构建器
pub struct DistanceJointBuilder {
    joint: DistanceJoint,
}

impl DistanceJointBuilder {
    /// 创建新的距离关节构建器
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self {
        Self {
            joint: DistanceJoint::new(0, 1, anchor1, anchor2),
        }
    }

    /// 设置刚体索引
    pub fn bodies(mut self, body_a: usize, body_b: usize) -> Self {
        self.joint.base.body_a = body_a;
        self.joint.base.body_b = body_b;
        self
    }

    /// 设置目标距离
    pub fn length(mut self, length: f32) -> Self {
        self.joint.set_length(length);
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Distance,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

/// 绳索关节
///
/// 限制两个锚点之间的最大距离。
#[derive(Debug, Clone)]
pub struct RopeJoint {
    /// 基类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 最大距离
    max_length: f32,
}

impl RopeJoint {
    /// 创建新的绳索关节
    pub fn new(
        body_a: usize,
        body_b: usize,
        anchor1: Vec3,
        anchor2: Vec3,
        max_length: f32,
    ) -> Self {
        Self {
            base: Joint3D::new(JointType3D::Rope, body_a, body_b),
            local_anchor1: anchor1,
            local_anchor2: anchor2,
            max_length,
        }
    }

    /// 获取最大距离
    pub fn max_length(&self) -> f32 {
        self.max_length
    }

    /// 设置最大距离
    pub fn set_max_length(&mut self, length: f32) {
        self.max_length = length;
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 绳索关节构建器
pub struct RopeJointBuilder {
    joint: RopeJoint,
}

impl RopeJointBuilder {
    /// 创建新的绳索关节构建器
    pub fn new(anchor1: Vec3, anchor2: Vec3, max_length: f32) -> Self {
        Self {
            joint: RopeJoint::new(0, 1, anchor1, anchor2, max_length),
        }
    }

    /// 设置刚体索引
    pub fn bodies(mut self, body_a: usize, body_b: usize) -> Self {
        self.joint.base.body_a = body_a;
        self.joint.base.body_b = body_b;
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Rope,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

/// 球面关节
///
/// 允许围绕锚点自由旋转，但可以设置锥形限制。
#[derive(Debug, Clone)]
pub struct SphericalJoint {
    /// 埐类
    base: Joint3D,
    /// 刚体A的局部锚点
    local_anchor1: Vec3,
    /// 刚体B的局部锚点
    local_anchor2: Vec3,
    /// 锥形限制轴
    cone_limit_axis: Option<Vec3>,
    /// 锥形限制角度
    cone_limit_angle: Option<f32>,
}

impl SphericalJoint {
    /// 创建新的球面关节
    pub fn new(body_a: usize, body_b: usize) -> Self {
        Self {
            base: Joint3D::new(JointType3D::Spherical, body_a, body_b),
            local_anchor1: Vec3::ZERO,
            local_anchor2: Vec3::ZERO,
            cone_limit_axis: None,
            cone_limit_angle: None,
        }
    }

    /// 设置锥形限制
    pub fn set_cone_limit(&mut self, axis: Vec3, angle: f32) {
        self.cone_limit_axis = Some(axis.normalize());
        self.cone_limit_angle = Some(angle);
    }

    /// 获取锥形限制
    pub fn cone_limit(&self) -> (Option<Vec3>, Option<f32>) {
        (self.cone_limit_axis, self.cone_limit_angle)
    }

    /// 获取基类
    pub fn base(&self) -> &Joint3D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint3D {
        &mut self.base
    }
}

/// 球面关节构建器
pub struct SphericalJointBuilder {
    joint: SphericalJoint,
}

impl SphericalJointBuilder {
    /// 创建新的球面关节构建器
    pub fn new(body_a: usize, body_b: usize) -> Self {
        Self {
            joint: SphericalJoint::new(body_a, body_b),
        }
    }

    /// 设置锥形限制
    pub fn with_cone_limit(mut self, axis: Vec3, angle: f32) -> Self {
        self.joint.set_cone_limit(axis, angle);
        self
    }

    /// 构建关节
    pub fn build(self) -> Joint3D {
        Joint3D::new(
            JointType3D::Spherical,
            self.joint.base.body_a,
            self.joint.base.body_b,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_joint() {
        let joint = FixedJointBuilder::new(0, 1)
            .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
            .local_anchor2(Vec3::new(1.0, 0.0, 0.0))
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Fixed);
        assert_eq!(joint.body_a(), 0);
        assert_eq!(joint.body_b(), 1);
    }

    #[test]
    fn test_revolute_joint() {
        let joint = RevoluteJointBuilder::new(Vec3::Y)
            .bodies(0, 1)
            .limits(-1.0, 1.0)
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Revolute);
    }

    #[test]
    fn test_prismatic_joint() {
        let joint = PrismaticJointBuilder::new(Vec3::X)
            .bodies(0, 1)
            .limits(0.0, 10.0)
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Prismatic);
    }

    #[test]
    fn test_ball_joint() {
        let joint = BallJointBuilder::new(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0))
            .bodies(0, 1)
            .limits(45.0)
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Ball);
    }

    #[test]
    fn test_distance_joint() {
        let joint = DistanceJointBuilder::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0))
            .bodies(0, 1)
            .length(10.0)
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Distance);
    }

    #[test]
    fn test_rope_joint() {
        let joint = RopeJointBuilder::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 15.0)
            .bodies(0, 1)
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Rope);
    }

    #[test]
    fn test_spherical_joint() {
        let joint = SphericalJointBuilder::new(0, 1)
            .with_cone_limit(Vec3::Y, 30.0)
            .build();

        assert_eq!(joint.joint_type(), JointType3D::Spherical);
    }

    #[test]
    fn test_joint_enabled() {
        let mut joint = FixedJoint::new(0, 1);
        assert!(joint.base().is_enabled());
        joint.base_mut().set_enabled(false);
        assert!(!joint.base().is_enabled());
    }

    #[test]
    fn test_joint_handle() {
        let handle = JointHandle::new(0, 0);
        assert!(handle.is_valid());
        assert!(!JointHandle::INVALID.is_valid());
    }
}
