//! 关节模块
//!
//! 提供 2D 物理关节实现，包括距离关节、旋转关节、滑块关节等。

use crate::world::PhysicsWorld2D;
use engine_math::Vec2;

/// 关节类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType {
    /// 距离关节
    Distance,
    /// 旋转关节
    Revolute,
    /// 滑块关节
    Prismatic,
    /// 绳索关节
    Rope,
    /// 弹簧关节
    Spring,
}

/// 关节基类
#[derive(Debug, Clone)]
pub struct Joint2D {
    /// 关节类型
    joint_type: JointType,
    /// 连接的刚体 A 索引
    body_a: usize,
    /// 连接的刚体 B 索引
    body_b: usize,
    /// 关节锚点 A（世界坐标）
    anchor_a: Vec2,
    /// 关节锚点 B（世界坐标）
    anchor_b: Vec2,
    /// 是否启用
    enabled: bool,
    /// 碰撞连接的两个物体
    collide_connected: bool,
}

impl Joint2D {
    /// 创建新的关节
    pub fn new(joint_type: JointType, body_a: usize, body_b: usize) -> Self {
        Self {
            joint_type,
            body_a,
            body_b,
            anchor_a: Vec2::ZERO,
            anchor_b: Vec2::ZERO,
            enabled: true,
            collide_connected: false,
        }
    }

    /// 获取关节类型
    pub fn joint_type(&self) -> JointType {
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

    /// 获取锚点 A
    pub fn anchor_a(&self) -> Vec2 {
        self.anchor_a
    }

    /// 获取锚点 B
    pub fn anchor_b(&self) -> Vec2 {
        self.anchor_b
    }

    /// 设置锚点 A
    pub fn set_anchor_a(&mut self, anchor: Vec2) {
        self.anchor_a = anchor;
    }

    /// 设置锚点 B
    pub fn set_anchor_b(&mut self, anchor: Vec2) {
        self.anchor_b = anchor;
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

    #[allow(dead_code)]
    fn pre_solve(&mut self, _world: &PhysicsWorld2D, _dt: f32) {}

    #[allow(dead_code)]
    fn solve(&mut self, _world: &PhysicsWorld2D) {}

    #[allow(dead_code)]
    fn post_solve(&mut self) {}
}

/// 距离关节
///
/// 保持两个锚点之间的距离不变。
#[derive(Debug, Clone)]
pub struct DistanceJoint {
    /// 基类
    base: Joint2D,
    /// 目标距离
    length: f32,
    /// 最小距离
    min_length: f32,
    /// 最大距离
    max_length: f32,
    /// 刚度（0-1）
    stiffness: f32,
    /// 阻尼（0-1）
    damping: f32,
}

impl DistanceJoint {
    /// 创建新的距离关节
    pub fn new(body_a: usize, body_b: usize, anchor_a: Vec2, anchor_b: Vec2) -> Self {
        let length = (anchor_b - anchor_a).length();
        Self {
            base: Joint2D::new(JointType::Distance, body_a, body_b),
            length,
            min_length: length,
            max_length: length,
            stiffness: 1.0,
            damping: 0.0,
        }
    }

    /// 获取目标距离
    pub fn length(&self) -> f32 {
        self.length
    }

    /// 设置目标距离
    pub fn set_length(&mut self, length: f32) {
        self.length = length;
        self.min_length = self.min_length.min(length);
        self.max_length = self.max_length.max(length);
    }

    /// 获取最小距离
    pub fn min_length(&self) -> f32 {
        self.min_length
    }

    /// 设置最小距离
    pub fn set_min_length(&mut self, length: f32) {
        self.min_length = length;
    }

    /// 获取最大距离
    pub fn max_length(&self) -> f32 {
        self.max_length
    }

    /// 设置最大距离
    pub fn set_max_length(&mut self, length: f32) {
        self.max_length = length;
    }

    /// 获取刚度
    pub fn stiffness(&self) -> f32 {
        self.stiffness
    }

    /// 设置刚度
    pub fn set_stiffness(&mut self, stiffness: f32) {
        self.stiffness = stiffness.clamp(0.0, 1.0);
    }

    /// 获取阻尼
    pub fn damping(&self) -> f32 {
        self.damping
    }

    /// 设置阻尼
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping.clamp(0.0, 1.0);
    }

    /// 获取基类
    pub fn base(&self) -> &Joint2D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint2D {
        &mut self.base
    }
}

/// 旋转关节
///
/// 两个刚体围绕锚点旋转。
#[derive(Debug, Clone)]
pub struct RevoluteJoint {
    /// 基类
    base: Joint2D,
    /// 关节角度
    angle: f32,
    /// 最小角度限制
    min_angle: Option<f32>,
    /// 最大角度限制
    max_angle: Option<f32>,
    /// 是否启用马达
    motor_enabled: bool,
    /// 马达目标速度
    motor_speed: f32,
    /// 马达最大扭矩
    motor_max_torque: f32,
    /// 弹簧刚度
    spring_stiffness: f32,
    /// 弹簧阻尼
    spring_damping: f32,
}

impl RevoluteJoint {
    /// 创建新的旋转关节
    pub fn new(body_a: usize, body_b: usize, _anchor: Vec2) -> Self {
        Self {
            base: Joint2D::new(JointType::Revolute, body_a, body_b),
            angle: 0.0,
            min_angle: None,
            max_angle: None,
            motor_enabled: false,
            motor_speed: 0.0,
            motor_max_torque: 0.0,
            spring_stiffness: 0.0,
            spring_damping: 0.0,
        }
    }

    /// 获取关节角度
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// 设置关节角度
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    /// 设置角度限制
    pub fn set_limits(&mut self, min: Option<f32>, max: Option<f32>) {
        self.min_angle = min;
        self.max_angle = max;
    }

    /// 启用马达
    pub fn enable_motor(&mut self, enabled: bool) {
        self.motor_enabled = enabled;
    }

    /// 设置马达速度
    pub fn set_motor_speed(&mut self, speed: f32) {
        self.motor_speed = speed;
    }

    /// 设置马达最大扭矩
    pub fn set_motor_max_torque(&mut self, torque: f32) {
        self.motor_max_torque = torque;
    }

    /// 设置弹簧参数
    pub fn set_spring(&mut self, stiffness: f32, damping: f32) {
        self.spring_stiffness = stiffness;
        self.spring_damping = damping;
    }

    /// 获取基类
    pub fn base(&self) -> &Joint2D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint2D {
        &mut self.base
    }
}

/// 滑块关节
///
/// 两个刚体沿指定轴相对滑动。
#[derive(Debug, Clone)]
pub struct PrismaticJoint {
    base: Joint2D,
    axis: Vec2,
    min_distance: Option<f32>,
    max_distance: Option<f32>,
    #[allow(dead_code)]
    motor_stiffness: f32,
    #[allow(dead_code)]
    motor_damping: f32,
}

impl PrismaticJoint {
    /// 创建新的滑块关节
    pub fn new(body_a: usize, body_b: usize, _anchor: Vec2, axis: Vec2) -> Self {
        Self {
            base: Joint2D::new(JointType::Prismatic, body_a, body_b),
            axis: axis.normalize(),
            min_distance: None,
            max_distance: None,
            motor_stiffness: 0.0,
            motor_damping: 0.0,
        }
    }

    /// 获取滑动轴
    pub fn axis(&self) -> Vec2 {
        self.axis
    }

    /// 设置滑动轴
    pub fn set_axis(&mut self, axis: Vec2) {
        self.axis = axis.normalize();
    }

    /// 设置距离限制
    pub fn set_limits(&mut self, min: Option<f32>, max: Option<f32>) {
        self.min_distance = min;
        self.max_distance = max;
    }

    /// 获取基类
    pub fn base(&self) -> &Joint2D {
        &self.base
    }

    /// 获取可变基类
    pub fn base_mut(&mut self) -> &mut Joint2D {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_joint() {
        let joint = DistanceJoint::new(0, 1, Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));

        assert_eq!(joint.length(), 10.0);
        assert_eq!(joint.min_length(), 10.0);
        assert_eq!(joint.max_length(), 10.0);
    }

    #[test]
    fn test_distance_joint_length() {
        let mut joint = DistanceJoint::new(0, 1, Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));

        joint.set_length(20.0);
        assert_eq!(joint.length(), 20.0);
        assert_eq!(joint.min_length(), 10.0);
        assert_eq!(joint.max_length(), 20.0);
    }

    #[test]
    fn test_revolute_joint() {
        let joint = RevoluteJoint::new(0, 1, Vec2::new(5.0, 5.0));

        assert_eq!(joint.angle(), 0.0);
        assert!(!joint.motor_enabled);
    }

    #[test]
    fn test_revolute_joint_limits() {
        let mut joint = RevoluteJoint::new(0, 1, Vec2::new(0.0, 0.0));

        joint.set_limits(Some(-1.0), Some(1.0));
        assert_eq!(joint.min_angle, Some(-1.0));
        assert_eq!(joint.max_angle, Some(1.0));
    }

    #[test]
    fn test_prismatic_joint() {
        let joint = PrismaticJoint::new(0, 1, Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));

        assert_eq!(joint.axis(), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_joint_enabled() {
        let mut joint = DistanceJoint::new(0, 1, Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));

        assert!(joint.base().is_enabled());
        joint.base_mut().set_enabled(false);
        assert!(!joint.base().is_enabled());
    }
}
