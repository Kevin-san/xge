//! 角色控制器模块
//!
//! 提供 3D 角色控制器实现，用于处理角色移动、碰撞检测和地面检测。

use engine_math::Vec3;

use crate::{Collider3D, CollisionGroups, QueryFilter, RigidBody3D};
use crate::constants::DEFAULT_FRICTION;

/// 角色移动结果
#[derive(Debug, Clone)]
pub struct CharacterMovement {
    /// 实际移动的位移
    translation: Vec3,
    /// 是否在地面上
    grounded: bool,
    /// 是否碰到天花板
    hit_ceil: bool,
    /// 是否碰到墙壁
    hit_wall: bool,
    /// 地面法线
    ground_normal: Vec3,
    /// 碰撞点列表
    collision_points: Vec<Vec3>,
}

impl CharacterMovement {
    /// 创建新的角色移动结果
    pub fn new() -> Self {
        Self {
            translation: Vec3::ZERO,
            grounded: false,
            hit_ceil: false,
            hit_wall: false,
            ground_normal: Vec3::Y,
            collision_points: Vec::new(),
        }
    }

    /// 获取实际移动的位移
    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    /// 设置实际移动的位移
    pub fn set_translation(&mut self, translation: Vec3) {
        self.translation = translation;
    }

    /// 检查是否在地面上
    pub fn grounded(&self) -> bool {
        self.grounded
    }

    /// 设置地面状态
    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded;
    }

    /// 检查是否碰到天花板
    pub fn hit_ceil(&self) -> bool {
        self.hit_ceil
    }

    /// 设置天花板碰撞状态
    pub fn set_hit_ceil(&mut self, hit_ceil: bool) {
        self.hit_ceil = hit_ceil;
    }

    /// 检查是否碰到墙壁
    pub fn hit_wall(&self) -> bool {
        self.hit_wall
    }

    /// 设置墙壁碰撞状态
    pub fn set_hit_wall(&mut self, hit_wall: bool) {
        self.hit_wall = hit_wall;
    }

    /// 获取地面法线
    pub fn ground_normal(&self) -> Vec3 {
        self.ground_normal
    }

    /// 设置地面法线
    pub fn set_ground_normal(&mut self, normal: Vec3) {
        self.ground_normal = normal.normalize();
    }

    /// 添加碰撞点
    pub fn add_collision_point(&mut self, point: Vec3) {
        self.collision_points.push(point);
    }

    /// 获取碰撞点列表
    pub fn collision_points(&self) -> &[Vec3] {
        &self.collision_points
    }
}

impl Default for CharacterMovement {
    fn default() -> Self {
        Self::new()
    }
}

/// 角色控制器配置
#[derive(Debug, Clone)]
pub struct CharacterControllerConfig {
    /// 偏移量（用于调整碰撞体位置）
    pub offset: Vec3,
    /// 向上方向
    pub up_dir: Vec3,
    /// 最大爬坡角度（弧度）
    pub max_slope_climb_angle: f32,
    /// 最大滑行角度（弧度）
    pub max_slide_angle: f32,
    /// 是否对动态刚体施加冲量
    pub apply_impulse_to_dynamic_bodies: bool,
    /// 最大地面检测距离
    pub max_distance_to_ground: f32,
    /// 自动步进高度
    pub auto_step_height: f32,
    /// 自动步进最小宽度
    pub auto_step_min_width: f32,
    /// 是否启用自动步进
    pub auto_step_enabled: bool,
}

impl Default for CharacterControllerConfig {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            up_dir: Vec3::Y,
            max_slope_climb_angle: 45.0 * std::f32::consts::PI / 180.0, // 45度
            max_slide_angle: 45.0 * std::f32::consts::PI / 180.0,      // 45度
            apply_impulse_to_dynamic_bodies: true,
            max_distance_to_ground: 0.1,
            auto_step_height: 0.3,
            auto_step_min_width: 0.1,
            auto_step_enabled: false,
        }
    }
}

/// 3D 角色控制器
///
/// 用于处理角色移动、碰撞检测和地面检测。
#[derive(Debug, Clone)]
pub struct CharacterController3D {
    /// 配置
    config: CharacterControllerConfig,
    /// 上一次移动结果
    last_movement: CharacterMovement,
    /// 当前速度
    velocity: Vec3,
    /// 是否启用
    enabled: bool,
}

impl CharacterController3D {
    /// 创建新的角色控制器
    pub fn new(
        offset: Vec3,
        up_dir: Vec3,
        max_slope_climb_angle: f32,
        max_slide_angle: f32,
    ) -> Self {
        Self {
            config: CharacterControllerConfig {
                offset,
                up_dir: up_dir.normalize(),
                max_slope_climb_angle,
                max_slide_angle,
                ..Default::default()
            },
            last_movement: CharacterMovement::new(),
            velocity: Vec3::ZERO,
            enabled: true,
        }
    }

    /// 使用默认配置创建角色控制器
    pub fn with_default_config() -> Self {
        Self {
            config: CharacterControllerConfig::default(),
            last_movement: CharacterMovement::new(),
            velocity: Vec3::ZERO,
            enabled: true,
        }
    }

    /// 获取配置
    pub fn config(&self) -> &CharacterControllerConfig {
        &self.config
    }

    /// 设置偏移量
    pub fn set_offset(&mut self, offset: Vec3) {
        self.config.offset = offset;
    }

    /// 获取偏移量
    pub fn offset(&self) -> Vec3 {
        self.config.offset
    }

    /// 设置向上方向
    pub fn set_up(&mut self, up_dir: Vec3) {
        self.config.up_dir = up_dir.normalize();
    }

    /// 获取向上方向
    pub fn up_dir(&self) -> Vec3 {
        self.config.up_dir
    }

    /// 设置最大爬坡角度
    pub fn set_slope_climb_angle(&mut self, angle: f32) {
        self.config.max_slope_climb_angle = angle;
    }

    /// 获取最大爬坡角度
    pub fn max_slope_climb_angle(&self) -> f32 {
        self.config.max_slope_climb_angle
    }

    /// 设置最大滑行角度
    pub fn set_slide_angle(&mut self, angle: f32) {
        self.config.max_slide_angle = angle;
    }

    /// 获取最大滑行角度
    pub fn max_slide_angle(&self) -> f32 {
        self.config.max_slide_angle
    }

    /// 设置是否对动态刚体施加冲量
    pub fn set_apply_impulse_to_dynamic_bodies(&mut self, apply: bool) {
        self.config.apply_impulse_to_dynamic_bodies = apply;
    }

    /// 设置最大地面检测距离
    pub fn set_max_distance_to_ground(&mut self, distance: f32) {
        self.config.max_distance_to_ground = distance;
    }

    /// 获取最大地面检测距离
    pub fn max_distance_to_ground(&self) -> f32 {
        self.config.max_distance_to_ground
    }

    /// 获取当前速度
    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    /// 设置当前速度
    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    /// 获取上一次移动结果
    pub fn last_movement(&self) -> &CharacterMovement {
        &self.last_movement
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 移动形状
    ///
    /// 执行角色移动，检测碰撞并返回移动结果。
    pub fn move_shape(
        &mut self,
        dt: f32,
        desired_translation: Vec3,
        body: &mut RigidBody3D,
        collider: &Collider3D,
        _filter: QueryFilter,
    ) -> CharacterMovement {
        if !self.enabled {
            return CharacterMovement::new();
        }

        let mut movement = CharacterMovement::new();
        let position = body.position();
        let rotation = body.rotation();

        // 计算碰撞体的世界位置
        let collider_pos = collider.world_position(position, rotation);
        let collider_rot = collider.world_rotation(rotation);

        // 简化的碰撞检测和移动
        // 实际实现需要使用物理引擎的形状投射功能
        let shape = collider.shape();
        let _aabb = shape.compute_aabb(collider_pos + self.config.offset, collider_rot);

        // 检测地面
        let ground_check_distance = self.config.max_distance_to_ground;
        let ground_ray_start = collider_pos + self.config.offset;
        let ground_ray_dir = -self.config.up_dir;

        // 简化的地面检测
        // 实际实现需要使用射线投射
        let is_grounded = self.check_ground_simple(ground_ray_start, ground_ray_dir, ground_check_distance);

        movement.set_grounded(is_grounded);

        // 计算实际位移
        let actual_translation = desired_translation * dt;

        // 检测墙壁碰撞（简化）
        let horizontal_dir = Vec3::new(actual_translation.x, 0.0, actual_translation.z);
        if horizontal_dir.length() > 0.001 {
            // 简化的墙壁检测
            movement.set_hit_wall(false); // 实际需要形状投射检测
        }

        // 检测天花板碰撞（简化）
        if actual_translation.y > 0.0 {
            movement.set_hit_ceil(false); // 实际需要形状投射检测
        }

        // 应用坡度限制
        if is_grounded && movement.hit_wall() {
            let wall_normal = Vec3::new(-horizontal_dir.x, 0.0, -horizontal_dir.z).normalize();
            let slope_angle = wall_normal.dot(self.config.up_dir).acos();

            if slope_angle > self.config.max_slope_climb_angle {
                // 无法爬上坡度，滑行
                let slide_dir = wall_normal - wall_normal.dot(self.config.up_dir) * self.config.up_dir;
                movement.set_translation(slide_dir.normalize() * actual_translation.length());
            } else {
                movement.set_translation(actual_translation);
            }
        } else {
            movement.set_translation(actual_translation);
        }

        // 更新刚体位置
        let new_position = position + movement.translation();
        body.set_translation(new_position, true);

        // 更新速度
        self.velocity = desired_translation;

        // 保存移动结果
        self.last_movement = movement.clone();

        movement
    }

    /// 简化的地面检测
    fn check_ground_simple(&self, _start: Vec3, _dir: Vec3, _distance: f32) -> bool {
        // 简化实现：假设角色在地面上
        // 实际实现需要使用射线投射检测
        true
    }

    /// 检查是否可以爬上坡度
    pub fn can_climb_slope(&self, normal: Vec3) -> bool {
        let angle = normal.dot(self.config.up_dir).acos();
        angle <= self.config.max_slope_climb_angle
    }

    /// 计算滑行方向
    pub fn compute_slide_direction(&self, normal: Vec3) -> Vec3 {
        let slide_dir = normal - normal.dot(self.config.up_dir) * self.config.up_dir;
        slide_dir.normalize()
    }

    /// 应用重力
    pub fn apply_gravity(&mut self, gravity: Vec3, dt: f32) {
        if !self.last_movement.grounded() {
            self.velocity += gravity * dt;
        }
    }

    /// 应用跳跃
    pub fn jump(&mut self, jump_velocity: f32) {
        if self.last_movement.grounded() {
            self.velocity.y = jump_velocity;
            self.last_movement.set_grounded(false);
        }
    }

    /// 应用移动输入
    pub fn apply_movement_input(&mut self, direction: Vec3, speed: f32) {
        let horizontal_vel = direction.normalize() * speed;
        self.velocity.x = horizontal_vel.x;
        self.velocity.z = horizontal_vel.z;
    }

    /// 应用摩擦力
    pub fn apply_friction(&mut self, friction: f32, dt: f32) {
        if self.last_movement.grounded() {
            let friction_factor = 1.0 - friction * dt;
            self.velocity.x *= friction_factor;
            self.velocity.z *= friction_factor;
        }
    }

    /// 获取水平速度
    pub fn horizontal_velocity(&self) -> Vec3 {
        Vec3::new(self.velocity.x, 0.0, self.velocity.z)
    }

    /// 获取垂直速度
    pub fn vertical_velocity(&self) -> f32 {
        self.velocity.y
    }

    /// 设置水平速度
    pub fn set_horizontal_velocity(&mut self, velocity: Vec3) {
        self.velocity.x = velocity.x;
        self.velocity.z = velocity.z;
    }

    /// 设置垂直速度
    pub fn set_vertical_velocity(&mut self, velocity: f32) {
        self.velocity.y = velocity;
    }
}

impl Default for CharacterController3D {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_controller_creation() {
        let controller = CharacterController3D::with_default_config();
        assert_eq!(controller.up_dir(), Vec3::Y);
        assert!(controller.is_enabled());
    }

    #[test]
    fn test_character_movement() {
        let movement = CharacterMovement::new();
        assert_eq!(movement.translation(), Vec3::ZERO);
        assert!(!movement.grounded());
        assert!(!movement.hit_ceil());
        assert!(!movement.hit_wall());
    }

    #[test]
    fn test_set_grounded() {
        let mut movement = CharacterMovement::new();
        movement.set_grounded(true);
        assert!(movement.grounded());
    }

    #[test]
    fn test_set_ground_normal() {
        let mut movement = CharacterMovement::new();
        movement.set_ground_normal(Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(movement.ground_normal(), Vec3::Y);
    }

    #[test]
    fn test_velocity() {
        let mut controller = CharacterController3D::with_default_config();
        controller.set_velocity(Vec3::new(1.0, 0.0, 2.0));
        assert_eq!(controller.velocity(), Vec3::new(1.0, 0.0, 2.0));
    }

    #[test]
    fn test_horizontal_velocity() {
        let mut controller = CharacterController3D::with_default_config();
        controller.set_velocity(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(controller.horizontal_velocity(), Vec3::new(1.0, 0.0, 3.0));
    }

    #[test]
    fn test_vertical_velocity() {
        let mut controller = CharacterController3D::with_default_config();
        controller.set_velocity(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(controller.vertical_velocity(), 2.0);
    }

    #[test]
    fn test_jump() {
        let mut controller = CharacterController3D::with_default_config();
        controller.last_movement.set_grounded(true);
        controller.jump(10.0);
        assert_eq!(controller.vertical_velocity(), 10.0);
        assert!(!controller.last_movement.grounded());
    }

    #[test]
    fn test_apply_movement_input() {
        let mut controller = CharacterController3D::with_default_config();
        controller.apply_movement_input(Vec3::new(1.0, 0.0, 0.0), 5.0);
        assert_eq!(controller.horizontal_velocity().x, 5.0);
    }

    #[test]
    fn test_slope_climb() {
        let controller = CharacterController3D::with_default_config();
        // 45度坡度
        let normal = Vec3::new(0.0, 1.0, 0.0);
        assert!(controller.can_climb_slope(normal));

        // 垂直墙面
        let wall_normal = Vec3::new(1.0, 0.0, 0.0);
        assert!(!controller.can_climb_slope(wall_normal));
    }

    #[test]
    fn test_slide_direction() {
        let controller = CharacterController3D::with_default_config();
        let normal = Vec3::new(0.5, 0.5, 0.0).normalize();
        let slide_dir = controller.compute_slide_direction(normal);
        // 滑行方向应该与上方向垂直
        assert!((slide_dir.dot(Vec3::Y)).abs() < 0.001);
    }

    #[test]
    fn test_config() {
        let controller = CharacterController3D::new(
            Vec3::ZERO,
            Vec3::Y,
            30.0,
            45.0,
        );
        assert_eq!(controller.max_slope_climb_angle(), 30.0);
        assert_eq!(controller.max_slide_angle(), 45.0);
    }
}