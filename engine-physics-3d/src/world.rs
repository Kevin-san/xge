//! 物理世界模块
//!
//! 管理所有 3D 物理实体、碰撞检测和仿真步进。

use std::collections::{HashMap, VecDeque};

use engine_math::Vec3;

use crate::{
    collider::{Collider3D, ColliderHandle},
    collision::{ContactEvent, ContactForceEvent, ContactPair, IntersectionEvent},
    constants::{
        DEFAULT_GRAVITY, DEFAULT_POSITION_ITERATIONS, DEFAULT_TIMESTEP,
        DEFAULT_VELOCITY_ITERATIONS, MAX_SUBSTEPS,
    },
    joint::{Joint3D, JointHandle},
    query::{Query3D, QueryPipeline},
    rigidbody::{RigidBody3D, RigidBodyHandle, RigidBodyType3D},
};

/// 物理世界配置
#[derive(Debug, Clone)]
pub struct PhysicsWorldConfig3D {
    /// 重力加速度
    pub gravity: Vec3,
    /// 物理步长（秒）
    pub timestep: f32,
    /// 最大子步数
    pub max_substeps: usize,
    /// 速度迭代次数
    pub velocity_iterations: usize,
    /// 位置迭代次数
    pub position_iterations: usize,
    /// 默认弹性系数
    pub default_restitution: f32,
    /// 默认摩擦系数
    pub default_friction: f32,
    /// 是否启用CCD
    pub ccd_enabled: bool,
}

impl Default for PhysicsWorldConfig3D {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, DEFAULT_GRAVITY, 0.0),
            timestep: DEFAULT_TIMESTEP,
            max_substeps: MAX_SUBSTEPS,
            velocity_iterations: DEFAULT_VELOCITY_ITERATIONS,
            position_iterations: DEFAULT_POSITION_ITERATIONS,
            default_restitution: 0.3,
            default_friction: 0.5,
            ccd_enabled: false,
        }
    }
}

impl PhysicsWorldConfig3D {
    /// 创建默认配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置重力
    pub fn with_gravity(mut self, gravity: Vec3) -> Self {
        self.gravity = gravity;
        self
    }

    /// 设置物理步长
    pub fn with_timestep(mut self, timestep: f32) -> Self {
        self.timestep = timestep;
        self
    }

    /// 设置迭代次数
    pub fn with_iterations(mut self, velocity: usize, position: usize) -> Self {
        self.velocity_iterations = velocity;
        self.position_iterations = position;
        self
    }
}

/// 刚体版本号（用于检测无效句柄）
struct BodyVersion {
    generation: u32,
    is_valid: bool,
}

/// 碰撞体版本号
struct ColliderVersion {
    generation: u32,
    is_valid: bool,
}

/// 关节版本号
struct JointVersion {
    generation: u32,
    is_valid: bool,
}

/// 3D 物理世界
///
/// 管理所有物理实体（刚体、碰撞体、关节），并执行物理仿真。
pub struct PhysicsWorld3D {
    /// 配置
    config: PhysicsWorldConfig3D,
    /// 刚体列表
    bodies: Vec<RigidBody3D>,
    /// 刚体版本号列表
    body_versions: Vec<BodyVersion>,
    /// 碰撞体列表
    colliders: Vec<Collider3D>,
    /// 碰撞体版本号列表
    collider_versions: Vec<ColliderVersion>,
    /// 关节列表
    joints: Vec<Joint3D>,
    /// 关节版本号列表
    joint_versions: Vec<JointVersion>,
    /// 碰撞对索引（用于 Broad Phase）
    collision_pairs: Vec<(usize, usize)>,
    /// 接触流形
    manifolds: HashMap<(usize, usize), ContactPair>,
    /// 接触事件
    contact_events: VecDeque<ContactEvent>,
    /// 相交事件
    intersection_events: VecDeque<IntersectionEvent>,
    /// 接触力事件
    contact_force_events: VecDeque<ContactForceEvent>,
    /// 查询管道
    query_pipeline: QueryPipeline,
    /// 查询对象
    query: Query3D,
    /// 仿真时间
    simulation_time: f32,
    /// 累积时间
    accumulator: f32,
    /// 是否暂停
    paused: bool,
    /// 启用碰撞检测
    collision_detection_enabled: bool,
    /// 启用物理仿真
    simulation_enabled: bool,
    /// 重力缩放
    gravity_scale: f32,
}

impl PhysicsWorld3D {
    /// 创建新的物理世界
    pub fn new(config: PhysicsWorldConfig3D) -> Self {
        Self {
            config,
            bodies: Vec::new(),
            body_versions: Vec::new(),
            colliders: Vec::new(),
            collider_versions: Vec::new(),
            joints: Vec::new(),
            joint_versions: Vec::new(),
            collision_pairs: Vec::new(),
            manifolds: HashMap::new(),
            contact_events: VecDeque::new(),
            intersection_events: VecDeque::new(),
            contact_force_events: VecDeque::new(),
            query_pipeline: QueryPipeline::new(),
            query: Query3D::new(),
            simulation_time: 0.0,
            accumulator: 0.0,
            paused: false,
            collision_detection_enabled: true,
            simulation_enabled: true,
            gravity_scale: 1.0,
        }
    }

    /// 创建物理世界（使用默认配置）
    pub fn with_default_config() -> Self {
        Self::new(PhysicsWorldConfig3D::default())
    }

    /// 设置重力
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.config.gravity = gravity;
    }

    /// 获取重力
    pub fn gravity(&self) -> Vec3 {
        self.config.gravity
    }

    /// 设置暂停状态
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// 检查是否暂停
    pub fn paused(&self) -> bool {
        self.paused
    }

    /// 设置CCD启用状态
    pub fn set_ccd_enabled(&mut self, enabled: bool) {
        self.config.ccd_enabled = enabled;
    }

    /// 检查CCD是否启用
    pub fn ccd_enabled(&self) -> bool {
        self.config.ccd_enabled
    }

    /// 设置重力缩放
    pub fn set_gravity_scale(&mut self, scale: f32) {
        self.gravity_scale = scale;
    }

    /// 获取重力缩放
    pub fn gravity_scale(&self) -> f32 {
        self.gravity_scale
    }

    /// 设置速度迭代次数
    pub fn set_max_velocity_iterations(&mut self, iterations: usize) {
        self.config.velocity_iterations = iterations;
    }

    /// 获取速度迭代次数
    pub fn max_velocity_iterations(&self) -> usize {
        self.config.velocity_iterations
    }

    /// 设置位置迭代次数
    pub fn set_max_position_iterations(&mut self, iterations: usize) {
        self.config.position_iterations = iterations;
    }

    /// 获取位置迭代次数
    pub fn max_position_iterations(&self) -> usize {
        self.config.position_iterations
    }

    /// 添加刚体
    pub fn insert_body(&mut self, body: RigidBody3D) -> RigidBodyHandle {
        // 查找可用的空位
        for (index, version) in self.body_versions.iter_mut().enumerate() {
            if !version.is_valid {
                version.is_valid = true;
                version.generation += 1;
                self.bodies[index] = body;
                return RigidBodyHandle::new(index as u32, version.generation);
            }
        }

        // 添加新刚体
        let index = self.bodies.len();
        let generation = 0;
        self.bodies.push(body);
        self.body_versions.push(BodyVersion { generation, is_valid: true });
        RigidBodyHandle::new(index as u32, generation)
    }

    /// 移除刚体
    pub fn remove_body(&mut self, handle: RigidBodyHandle) {
        if self.is_valid_body_handle(handle) {
            let index = handle.index as usize;
            self.body_versions[index].is_valid = false;
            // 清除关联的碰撞体
            self.bodies[index].clear_forces();
        }
    }

    /// 检查刚体句柄是否有效
    fn is_valid_body_handle(&self, handle: RigidBodyHandle) -> bool {
        if handle.index as usize >= self.body_versions.len() {
            return false;
        }
        let version = &self.body_versions[handle.index as usize];
        version.is_valid && version.generation == handle.generation
    }

    /// 获取刚体
    pub fn body(&self, handle: RigidBodyHandle) -> Option<&RigidBody3D> {
        if self.is_valid_body_handle(handle) {
            Some(&self.bodies[handle.index as usize])
        } else {
            None
        }
    }

    /// 获取可变刚体
    pub fn body_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody3D> {
        if self.is_valid_body_handle(handle) {
            Some(&mut self.bodies[handle.index as usize])
        } else {
            None
        }
    }

    /// 添加碰撞体
    pub fn insert_collider(&mut self, collider: Collider3D, parent_body: RigidBodyHandle) -> ColliderHandle {
        // 先检查父刚体是否有效
        let parent_is_valid = self.is_valid_body_handle(parent_body);
        
        // 查找可用的空位
        let mut found_index = None;
        let mut found_generation = 0;
        for (index, version) in self.collider_versions.iter_mut().enumerate() {
            if !version.is_valid {
                version.is_valid = true;
                version.generation += 1;
                found_index = Some(index);
                found_generation = version.generation;
                break;
            }
        }
        
        if let Some(index) = found_index {
            self.colliders[index] = collider;
            // 设置父刚体索引
            if parent_is_valid {
                self.bodies[parent_body.index as usize].add_collider_index(index);
                self.colliders[index].set_parent_body_index(Some(parent_body.index as usize));
            }
            return ColliderHandle::new(index as u32, found_generation);
        }

        // 添加新碰撞体
        let index = self.colliders.len();
        let generation = 0;
        self.colliders.push(collider);
        self.collider_versions.push(ColliderVersion { generation, is_valid: true });
        
        // 设置父刚体索引
        if parent_is_valid {
            self.bodies[parent_body.index as usize].add_collider_index(index);
            self.colliders[index].set_parent_body_index(Some(parent_body.index as usize));
        }
        
        ColliderHandle::new(index as u32, generation)
    }

    /// 移除碰撞体
    pub fn remove_collider(&mut self, handle: ColliderHandle) {
        if self.is_valid_collider_handle(handle) {
            let index = handle.index as usize;
            self.collider_versions[index].is_valid = false;
        }
    }

    /// 检查碰撞体句柄是否有效
    fn is_valid_collider_handle(&self, handle: ColliderHandle) -> bool {
        if handle.index as usize >= self.collider_versions.len() {
            return false;
        }
        let version = &self.collider_versions[handle.index as usize];
        version.is_valid && version.generation == handle.generation
    }

    /// 获取碰撞体
    pub fn collider(&self, handle: ColliderHandle) -> Option<&Collider3D> {
        if self.is_valid_collider_handle(handle) {
            Some(&self.colliders[handle.index as usize])
        } else {
            None
        }
    }

    /// 获取可变碰撞体
    pub fn collider_mut(&mut self, handle: ColliderHandle) -> Option<&mut Collider3D> {
        if self.is_valid_collider_handle(handle) {
            Some(&mut self.colliders[handle.index as usize])
        } else {
            None
        }
    }

    /// 添加关节
    pub fn insert_joint(&mut self, _body1: RigidBodyHandle, _body2: RigidBodyHandle, joint: Joint3D) -> JointHandle {
        // 查找可用的空位
        for (index, version) in self.joint_versions.iter_mut().enumerate() {
            if !version.is_valid {
                version.is_valid = true;
                version.generation += 1;
                self.joints[index] = joint;
                return JointHandle::new(index as u32, version.generation);
            }
        }

        // 添加新关节
        let index = self.joints.len();
        let generation = 0;
        self.joints.push(joint);
        self.joint_versions.push(JointVersion { generation, is_valid: true });
        JointHandle::new(index as u32, generation)
    }

    /// 移除关节
    pub fn remove_joint(&mut self, handle: JointHandle) {
        if self.is_valid_joint_handle(handle) {
            let index = handle.index as usize;
            self.joint_versions[index].is_valid = false;
        }
    }

    /// 检查关节句柄是否有效
    fn is_valid_joint_handle(&self, handle: JointHandle) -> bool {
        if handle.index as usize >= self.joint_versions.len() {
            return false;
        }
        let version = &self.joint_versions[handle.index as usize];
        version.is_valid && version.generation == handle.generation
    }

    /// 获取关节
    pub fn joint(&self, handle: JointHandle) -> Option<&Joint3D> {
        if self.is_valid_joint_handle(handle) {
            Some(&self.joints[handle.index as usize])
        } else {
            None
        }
    }

    /// 获取可变关节
    pub fn joint_mut(&mut self, handle: JointHandle) -> Option<&mut Joint3D> {
        if self.is_valid_joint_handle(handle) {
            Some(&mut self.joints[handle.index as usize])
        } else {
            None
        }
    }

    /// 执行物理步进
    pub fn step(&mut self, dt: f32) {
        if self.paused {
            return;
        }

        self.accumulator += dt;

        let max_time = self.config.timestep * self.config.max_substeps as f32;
        if self.accumulator > max_time {
            self.accumulator = max_time;
        }

        while self.accumulator >= self.config.timestep {
            let step_dt = self.config.timestep;

            if self.simulation_enabled {
                // 应用重力
                self.apply_gravity();

                // 更新刚体速度
                self.update_velocities(step_dt);

                // Broad Phase 碰撞检测
                if self.collision_detection_enabled {
                    self.broad_phase();
                    self.narrow_phase();
                }

                // 碰撞响应
                self.resolve_collisions(step_dt);

                // 更新位置
                self.update_positions(step_dt);

                // 关节约束
                self.solve_joints();

                // 位置修正
                self.correct_positions();

                // 清除力
                self.clear_forces();
            }

            self.accumulator -= step_dt;
            self.simulation_time += step_dt;
        }

        // 更新查询管道
        self.query_pipeline.update();
    }

    /// 执行物理步进（带子步）
    pub fn step_with_substeps(&mut self, dt: f32, substeps: u32) {
        let sub_dt = dt / substeps as f32;
        for _ in 0..substeps {
            self.step(sub_dt);
        }
    }

    /// 应用重力
    fn apply_gravity(&mut self) {
        let gravity = self.config.gravity * self.gravity_scale;
        for body in &mut self.bodies {
            if body.is_dynamic() && body.is_enabled() && !body.is_sleeping() {
                body.apply_force(gravity * body.mass(), false);
            }
        }
    }

    /// 更新速度
    fn update_velocities(&mut self, dt: f32) {
        let gravity = self.config.gravity * self.gravity_scale;
        for body in &mut self.bodies {
            if body.is_dynamic() && body.is_enabled() && !body.is_sleeping() {
                body.update_velocity(dt, gravity);
            }
        }
    }

    /// 更新位置
    fn update_positions(&mut self, dt: f32) {
        for body in &mut self.bodies {
            if body.is_dynamic() && body.is_enabled() && !body.is_sleeping() {
                body.update_position(dt);
            }
        }
    }

    /// 清除力
    fn clear_forces(&mut self) {
        for body in &mut self.bodies {
            body.clear_forces();
        }
    }

    /// Broad Phase 碰撞检测
    fn broad_phase(&mut self) {
        self.collision_pairs.clear();

        let n = self.bodies.len();
        for i in 0..n {
            if !self.body_versions[i].is_valid || !self.bodies[i].is_enabled() {
                continue;
            }
            for j in (i + 1)..n {
                if !self.body_versions[j].is_valid || !self.bodies[j].is_enabled() {
                    continue;
                }

                // 静态物体不参与碰撞检测
                if self.bodies[i].is_static() && self.bodies[j].is_static() {
                    continue;
                }

                // AABB 检测
                if self.check_aabb_overlap(i, j) {
                    self.collision_pairs.push((i, j));
                }
            }
        }
    }

    /// Narrow Phase 碰撞检测
    fn narrow_phase(&mut self) {
        self.manifolds.clear();
        self.contact_events.clear();
        self.intersection_events.clear();

        for &(i, j) in &self.collision_pairs {
            if let Some(pair) = self.generate_contact_pair(i, j) {
                let key = (i.min(j), i.max(j));
                self.manifolds.insert(key, pair);

                // 生成碰撞事件
                let handle_a = ColliderHandle::new(i as u32, self.collider_versions.get(i).map(|v| v.generation).unwrap_or(0));
                let handle_b = ColliderHandle::new(j as u32, self.collider_versions.get(j).map(|v| v.generation).unwrap_or(0));
                self.contact_events.push_back(ContactEvent::Started(handle_a, handle_b));
            }
        }
    }

    /// 生成接触对
    fn generate_contact_pair(&self, _index_a: usize, _index_b: usize) -> Option<ContactPair> {
        // 简化的碰撞检测实现
        // 实际实现需要根据碰撞体形状计算
        None
    }

    /// 检查 AABB 重叠
    fn check_aabb_overlap(&self, index_a: usize, index_b: usize) -> bool {
        // 简化实现：检查碰撞体的AABB
        let body_a = &self.bodies[index_a];
        let body_b = &self.bodies[index_b];

        // 获取碰撞体AABB（简化）
        let aabb_a = crate::AABB::from_center_half_extents(body_a.position(), Vec3::splat(1.0));
        let aabb_b = crate::AABB::from_center_half_extents(body_b.position(), Vec3::splat(1.0));

        aabb_a.intersects(&aabb_b)
    }

    /// 碰撞响应
    fn resolve_collisions(&mut self, _dt: f32) {
        // 简化的碰撞响应实现
        // 实际实现需要考虑弹性、摩擦等因素
    }

    /// 关节约束求解
    fn solve_joints(&mut self) {
        for joint in &self.joints {
            self.apply_joint_constraint(joint);
        }
    }

    /// 应用关节约束
    fn apply_joint_constraint(&self, _joint: &Joint3D) {
        // 简化的关节实现
    }

    /// 位置修正
    fn correct_positions(&mut self) {
        let slop = crate::constants::PENETRATION_SLOP;
        let baumgarte = crate::constants::BAUMGARTE;

        for pair in self.manifolds.values_mut() {
            for point in &mut pair.points {
                if point.penetration > slop {
                    // 位置修正
                    let correction = pair.normal * point.penetration * baumgarte;
                    // 应用修正到刚体
                    if let Some(body_a) = self.bodies.get_mut(pair.collider_a.index as usize) {
                        if body_a.is_dynamic() {
                            body_a.set_translation(body_a.position() - correction * 0.5, false);
                        }
                    }
                    if let Some(body_b) = self.bodies.get_mut(pair.collider_b.index as usize) {
                        if body_b.is_dynamic() {
                            body_b.set_translation(body_b.position() + correction * 0.5, false);
                        }
                    }
                }
            }
        }
    }

    /// 获取接触事件
    pub fn contact_events(&self) -> &VecDeque<ContactEvent> {
        &self.contact_events
    }

    /// 获取相交事件
    pub fn intersection_events(&self) -> &VecDeque<IntersectionEvent> {
        &self.intersection_events
    }

    /// 获取接触力事件
    pub fn contact_force_events(&self) -> &VecDeque<ContactForceEvent> {
        &self.contact_force_events
    }

    /// 获取接触对
    pub fn contact_pair(&self, a: ColliderHandle, b: ColliderHandle) -> Option<ContactPair> {
        let key = (a.index as usize, b.index as usize);
        self.manifolds.get(&key).cloned()
    }

    /// 获取刚体数量
    pub fn num_bodies(&self) -> usize {
        self.body_versions.iter().filter(|v| v.is_valid).count()
    }

    /// 获取碰撞体数量
    pub fn num_colliders(&self) -> usize {
        self.collider_versions.iter().filter(|v| v.is_valid).count()
    }

    /// 获取关节数量
    pub fn num_joints(&self) -> usize {
        self.joint_versions.iter().filter(|v| v.is_valid).count()
    }

    /// 获取仿真时间
    pub fn simulation_time(&self) -> f32 {
        self.simulation_time
    }

    /// 获取查询管道
    pub fn query_pipeline(&self) -> &QueryPipeline {
        &self.query_pipeline
    }

    /// 获取查询对象
    pub fn query(&self) -> &Query3D {
        &self.query
    }

    /// 清空碰撞事件
    pub fn clear_collision_events(&mut self) {
        self.contact_events.clear();
        self.intersection_events.clear();
        self.contact_force_events.clear();
    }

    /// 启用/禁用碰撞检测
    pub fn set_collision_detection(&mut self, enabled: bool) {
        self.collision_detection_enabled = enabled;
    }

    /// 启用/禁用物理仿真
    pub fn set_simulation(&mut self, enabled: bool) {
        self.simulation_enabled = enabled;
    }

    /// 清空世界
    pub fn clear(&mut self) {
        self.bodies.clear();
        self.body_versions.clear();
        self.colliders.clear();
        self.collider_versions.clear();
        self.joints.clear();
        self.joint_versions.clear();
        self.collision_pairs.clear();
        self.manifolds.clear();
        self.contact_events.clear();
        self.intersection_events.clear();
        self.contact_force_events.clear();
        self.simulation_time = 0.0;
        self.accumulator = 0.0;
    }

    /// 创建测试用物理世界
    pub fn test_world() -> Self {
        let mut world = Self::with_default_config();
        world.set_gravity(Vec3::new(0.0, DEFAULT_GRAVITY, 0.0));
        world
    }

    /// 迭代所有刚体
    pub fn bodies_iter(&self) -> impl Iterator<Item = (RigidBodyHandle, &RigidBody3D)> {
        self.body_versions.iter().enumerate().filter_map(|(index, version)| {
            if version.is_valid {
                let handle = RigidBodyHandle::new(index as u32, version.generation);
                Some((handle, &self.bodies[index]))
            } else {
                None
            }
        })
    }

    /// 迭代所有碰撞体
    pub fn colliders_iter(&self) -> impl Iterator<Item = (ColliderHandle, &Collider3D)> {
        self.collider_versions.iter().enumerate().filter_map(|(index, version)| {
            if version.is_valid {
                let handle = ColliderHandle::new(index as u32, version.generation);
                Some((handle, &self.colliders[index]))
            } else {
                None
            }
        })
    }
}

impl Default for PhysicsWorld3D {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rigidbody::RigidBody3DBuilder;

    #[test]
    fn test_physics_world_creation() {
        let world = PhysicsWorld3D::with_default_config();
        assert_eq!(world.num_bodies(), 0);
        assert_eq!(world.num_colliders(), 0);
        assert_eq!(world.num_joints(), 0);
    }

    #[test]
    fn test_add_body() {
        let mut world = PhysicsWorld3D::with_default_config();
        let body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        let handle = world.insert_body(body);
        assert!(handle.is_valid());
        assert_eq!(world.num_bodies(), 1);
    }

    #[test]
    fn test_remove_body() {
        let mut world = PhysicsWorld3D::with_default_config();
        let body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        let handle = world.insert_body(body);
        world.remove_body(handle);
        assert_eq!(world.num_bodies(), 0);
    }

    #[test]
    fn test_get_body() {
        let mut world = PhysicsWorld3D::with_default_config();
        let body = RigidBody3DBuilder::dynamic()
            .translation(Vec3::new(10.0, 20.0, 30.0))
            .build();
        let handle = world.insert_body(body);
        
        let retrieved = world.body(handle);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().position(), Vec3::new(10.0, 20.0, 30.0));
    }

    #[test]
    fn test_gravity() {
        let mut world = PhysicsWorld3D::with_default_config();
        assert_eq!(world.gravity(), Vec3::new(0.0, DEFAULT_GRAVITY, 0.0));

        world.set_gravity(Vec3::new(0.0, -20.0, 0.0));
        assert_eq!(world.gravity(), Vec3::new(0.0, -20.0, 0.0));
    }

    #[test]
    fn test_step() {
        let mut world = PhysicsWorld3D::with_default_config();
        world.step(1.0 / 60.0);
        assert!(world.simulation_time() > 0.0);
    }

    #[test]
    fn test_clear() {
        let mut world = PhysicsWorld3D::with_default_config();
        let body = RigidBody3D::new(RigidBodyType3D::Dynamic);
        world.insert_body(body);
        world.clear();
        assert_eq!(world.num_bodies(), 0);
    }

    #[test]
    fn test_paused() {
        let mut world = PhysicsWorld3D::with_default_config();
        assert!(!world.paused());
        
        world.set_paused(true);
        assert!(world.paused());
        
        // 暂停时不应该步进
        world.step(1.0 / 60.0);
        assert_eq!(world.simulation_time(), 0.0);
    }

    #[test]
    fn test_iterators() {
        let mut world = PhysicsWorld3D::with_default_config();
        
        let body1 = RigidBody3DBuilder::dynamic().build();
        let body2 = RigidBody3DBuilder::static_().build();
        
        world.insert_body(body1);
        world.insert_body(body2);
        
        let count = world.bodies_iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_config() {
        let config = PhysicsWorldConfig3D::new()
            .with_gravity(Vec3::new(0.0, -5.0, 0.0))
            .with_timestep(1.0 / 30.0)
            .with_iterations(4, 2);
        
        assert_eq!(config.gravity, Vec3::new(0.0, -5.0, 0.0));
        assert_eq!(config.timestep, 1.0 / 30.0);
        assert_eq!(config.velocity_iterations, 4);
        assert_eq!(config.position_iterations, 2);
    }

    #[test]
    fn test_gravity_scale() {
        let mut world = PhysicsWorld3D::with_default_config();
        world.set_gravity_scale(2.0);
        assert_eq!(world.gravity_scale(), 2.0);
    }

    #[test]
    fn test_iterations() {
        let mut world = PhysicsWorld3D::with_default_config();
        world.set_max_velocity_iterations(16);
        world.set_max_position_iterations(8);
        assert_eq!(world.max_velocity_iterations(), 16);
        assert_eq!(world.max_position_iterations(), 8);
    }
}