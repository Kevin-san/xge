//! 物理世界模块
//!
//! 管理所有物理实体、碰撞检测和仿真步进。

use std::collections::{HashMap, VecDeque};

use crate::{
    Collider2D, ColliderShape, CollisionEvent, Contact, Joint2D, Manifold, RigidBody2D,
    RigidBodyType,
};
use engine_math::{Rect, Vec2};

/// 物理世界配置
#[derive(Debug, Clone)]
pub struct PhysicsWorldConfig {
    /// 重力加速度
    pub gravity: Vec2,
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
}

impl Default for PhysicsWorldConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -9.81),
            timestep: 1.0 / 60.0,
            max_substeps: 4,
            velocity_iterations: 8,
            position_iterations: 3,
            default_restitution: 0.3,
            default_friction: 0.5,
        }
    }
}

/// 物理世界
///
/// 管理所有物理实体（刚体、碰撞体、关节），并执行物理仿真。
pub struct PhysicsWorld2D {
    /// 配置
    config: PhysicsWorldConfig,
    /// 刚体列表
    bodies: Vec<RigidBody2D>,
    /// 碰撞体列表
    colliders: Vec<Collider2D>,
    /// 关节列表
    joints: Vec<Joint2D>,
    /// 碰撞对索引（用于 Broad Phase）
    collision_pairs: Vec<(usize, usize)>,
    /// 接触流形
    manifolds: HashMap<(usize, usize), Manifold>,
    /// 碰撞事件
    collision_events: VecDeque<CollisionEvent>,
    /// 仿真时间
    simulation_time: f32,
    /// 累积时间
    accumulator: f32,
    /// 启用碰撞检测
    collision_detection_enabled: bool,
    /// 启用物理仿真
    simulation_enabled: bool,
}

impl PhysicsWorld2D {
    /// 创建新的物理世界
    pub fn new(config: PhysicsWorldConfig) -> Self {
        Self {
            config,
            bodies: Vec::new(),
            colliders: Vec::new(),
            joints: Vec::new(),
            collision_pairs: Vec::new(),
            manifolds: HashMap::new(),
            collision_events: VecDeque::new(),
            simulation_time: 0.0,
            accumulator: 0.0,
            collision_detection_enabled: true,
            simulation_enabled: true,
        }
    }

    /// 创建物理世界（使用默认配置）
    pub fn with_default_config() -> Self {
        Self::new(PhysicsWorldConfig::default())
    }

    /// 添加刚体
    pub fn add_body(&mut self, body: RigidBody2D) -> usize {
        let index = self.bodies.len();
        self.bodies.push(body);
        index
    }

    /// 移除刚体
    pub fn remove_body(&mut self, index: usize) {
        if index >= self.bodies.len() {
            return;
        }

        // 首先收集所有要移除的碰撞体索引（按降序排列避免索引偏移问题）
        let mut collider_indices = self.bodies[index].collider_indices().to_vec();
        collider_indices.sort_by(|a, b| b.cmp(a)); // 降序: [2, 1, 0]

        // 从高索引到低索引移除，避免索引偏移
        for collider_idx in collider_indices {
            if collider_idx < self.colliders.len() {
                self.colliders.remove(collider_idx);
            }
        }

        self.bodies.remove(index);
        self.reindex_colliders_and_pairs();
    }

    /// 重新索引碰撞体和清理碰撞对
    fn reindex_colliders_and_pairs(&mut self) {
        // 重建 collision_pairs，移除涉及已删除碰撞体的对
        self.collision_pairs
            .retain(|(i, j)| *i < self.colliders.len() && *j < self.colliders.len());

        // 清理 manifolds 中涉及已删除碰撞体的条目
        self.manifolds.retain(|key, _| {
            let (i, j) = *key;
            i < self.colliders.len() && j < self.colliders.len()
        });
    }

    /// 获取刚体
    pub fn get_body(&self, index: usize) -> Option<&RigidBody2D> {
        self.bodies.get(index)
    }

    /// 获取可变刚体
    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut RigidBody2D> {
        self.bodies.get_mut(index)
    }

    /// 添加碰撞体
    pub fn add_collider(&mut self, collider: Collider2D, body_index: usize) -> usize {
        let index = self.colliders.len();
        if body_index < self.bodies.len() {
            self.bodies[body_index].add_collider_index(index);
        }
        self.colliders.push(collider);
        index
    }

    /// 移除碰撞体
    pub fn remove_collider(&mut self, index: usize) {
        if index >= self.colliders.len() {
            return;
        }

        // 首先从所有刚体中移除此碰撞体的索引引用
        for body in &mut self.bodies {
            body.remove_collider_index(index);
        }

        // 然后再实际移除碰撞体
        self.colliders.remove(index);

        // 最后更新所有刚体的索引（将大于被移除索引的减1）
        for body in &mut self.bodies {
            body.update_collider_indices_after_remove(index);
        }

        // 清理相关的碰撞对和流形
        self.reindex_colliders_and_pairs();
    }

    /// 获取碰撞体
    pub fn get_collider(&self, index: usize) -> Option<&Collider2D> {
        self.colliders.get(index)
    }

    /// 添加关节
    pub fn add_joint(&mut self, joint: Joint2D) -> usize {
        let index = self.joints.len();
        self.joints.push(joint);
        index
    }

    /// 移除关节
    pub fn remove_joint(&mut self, index: usize) {
        if index < self.joints.len() {
            self.joints.remove(index);
        }
    }

    /// 获取关节
    pub fn get_joint(&self, index: usize) -> Option<&Joint2D> {
        self.joints.get(index)
    }

    /// 执行物理步进
    pub fn step(&mut self, dt: f32) {
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
                self.resolve_collisions();

                // 更新位置
                self.update_positions(step_dt);

                // 关节约束
                self.solve_joints();

                // 位置修正
                self.correct_positions();
            }

            self.accumulator -= step_dt;
            self.simulation_time += step_dt;
        }
    }

    /// 应用重力
    fn apply_gravity(&mut self) {
        let gravity = self.config.gravity;
        for body in &mut self.bodies {
            if body.body_type() == RigidBodyType::Dynamic {
                body.apply_force(gravity * body.mass());
            }
        }
    }

    /// 更新速度
    fn update_velocities(&mut self, dt: f32) {
        for body in &mut self.bodies {
            if body.body_type() == RigidBodyType::Dynamic {
                body.update_velocity(dt);
            }
        }
    }

    /// 更新位置
    fn update_positions(&mut self, dt: f32) {
        for body in &mut self.bodies {
            if body.body_type() == RigidBodyType::Dynamic {
                body.update_position(dt);
            }
        }
    }

    /// Broad Phase 碰撞检测
    fn broad_phase(&mut self) {
        self.collision_pairs.clear();

        let n = self.bodies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                // 静态物体不参与碰撞检测
                if self.bodies[i].is_static() || self.bodies[j].is_static() {
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
        self.collision_events.clear();

        for &(i, j) in &self.collision_pairs {
            if let Some(manifold) = self.generate_contact(i, j) {
                let key = (i.min(j), i.max(j));
                self.manifolds.insert(key, manifold.clone());

                // 生成碰撞事件
                self.collision_events.push_back(CollisionEvent::Started {
                    body_a: i,
                    body_b: j,
                    manifold,
                });
            }
        }
    }

    /// 生成接触流形
    fn generate_contact(&self, index_a: usize, index_b: usize) -> Option<Manifold> {
        let body_a = self.bodies.get(index_a)?;
        let body_b = self.bodies.get(index_b)?;

        // 获取碰撞体（假设每个刚体最多有一个碰撞体）
        let collider_a_idx = body_a.collider_indices().first()?;
        let collider_b_idx = body_b.collider_indices().first()?;
        let collider_a = self.colliders.get(*collider_a_idx)?;
        let collider_b = self.colliders.get(*collider_b_idx)?;

        let pos_a = body_a.position();
        let pos_b = body_b.position();
        let rot_a = body_a.rotation();
        let rot_b = body_b.rotation();

        let world_pos_a = collider_a.world_position(pos_a, rot_a);
        let world_pos_b = collider_b.world_position(pos_b, rot_b);

        // 根据形状类型进行碰撞检测
        match (collider_a.shape(), collider_b.shape()) {
            (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
                self.circle_circle_collision(
                    index_a,
                    index_b,
                    world_pos_a,
                    world_pos_b,
                    *r1,
                    *r2,
                )
            }
            (
                ColliderShape::Circle { radius },
                ColliderShape::Aabb { half_extents } | ColliderShape::Rectangle { half_extents },
            ) => self.circle_box_collision(
                (index_a, index_b),
                world_pos_a,
                world_pos_b,
                *radius,
                *half_extents,
                collider_b.shape(),
                rot_b,
            ),
            (
                ColliderShape::Aabb { half_extents } | ColliderShape::Rectangle { half_extents },
                ColliderShape::Circle { radius },
            ) => {
                let manifold = self.circle_box_collision(
                    (index_b, index_a),
                    world_pos_b,
                    world_pos_a,
                    *radius,
                    *half_extents,
                    collider_a.shape(),
                    rot_a,
                );
                // 反转法线方向
                manifold.map(|m| {
                    let mut reversed = m;
                    reversed.normal = -reversed.normal;
                    reversed
                })
            }
            (
                ColliderShape::Aabb { half_extents: h1 },
                ColliderShape::Aabb { half_extents: h2 },
            ) => self.aabb_aabb_collision(index_a, index_b, world_pos_a, world_pos_b, *h1, *h2),
            _ => None, // 其他形状组合暂不支持
        }
    }

    /// 圆形-圆形碰撞检测
    fn circle_circle_collision(
        &self,
        index_a: usize,
        index_b: usize,
        pos_a: Vec2,
        pos_b: Vec2,
        radius_a: f32,
        radius_b: f32,
    ) -> Option<Manifold> {
        let diff = pos_b - pos_a;
        let dist_sq = diff.length_squared();
        let min_dist = radius_a + radius_b;

        if dist_sq >= min_dist * min_dist {
            return None;
        }

        let dist = dist_sq.sqrt();
        let normal = if dist > 0.0 {
            diff / dist
        } else {
            Vec2::new(1.0, 0.0) // 圆心重合时的默认法线
        };
        let penetration = min_dist - dist;
        let contact_point = pos_a + normal * radius_a;

        let mut manifold = Manifold::new(index_a, index_b);
        manifold.normal = normal;
        manifold.penetration = penetration;
        manifold.add_contact(Contact::new(contact_point, normal, penetration));
        Some(manifold)
    }

    /// 圆形-矩形碰撞检测
    fn circle_box_collision(
        &self,
        indices: (usize, usize),
        circle_pos: Vec2,
        box_pos: Vec2,
        radius: f32,
        half_extents: Vec2,
        box_shape: &ColliderShape,
        box_rotation: f32,
    ) -> Option<Manifold> {
        let (circle_index, box_index) = indices;
        // 对于 AABB，直接计算
        // 对于旋转矩形，需要将圆转换到矩形的局部坐标系
        let (local_circle_pos, is_rotated) = match box_shape {
            ColliderShape::Aabb { .. } => (circle_pos - box_pos, false),
            ColliderShape::Rectangle { .. } => {
                let cos = box_rotation.cos();
                let sin = box_rotation.sin();
                let rel = circle_pos - box_pos;
                (
                    Vec2::new(rel.x * cos + rel.y * sin, -rel.x * sin + rel.y * cos),
                    true,
                )
            }
            _ => (circle_pos - box_pos, false),
        };

        // 找到矩形上离圆心最近的点
        let closest_x = local_circle_pos.x.clamp(-half_extents.x, half_extents.x);
        let closest_y = local_circle_pos.y.clamp(-half_extents.y, half_extents.y);
        let closest_point = Vec2::new(closest_x, closest_y);

        // 检查圆心是否在矩形内
        let inside = local_circle_pos.x.abs() <= half_extents.x
            && local_circle_pos.y.abs() <= half_extents.y;

        let diff = local_circle_pos - closest_point;
        let dist_sq = diff.length_squared();

        if inside {
            // 圆心在矩形内，需要找出最近的边
            let dx = half_extents.x - local_circle_pos.x.abs();
            let dy = half_extents.y - local_circle_pos.y.abs();

            let (penetration, normal_local) = if dx < dy {
                (dx, Vec2::new(local_circle_pos.x.signum(), 0.0))
            } else {
                (dy, Vec2::new(0.0, local_circle_pos.y.signum()))
            };

            // 将法线转换回世界坐标系
            let normal = if is_rotated {
                let cos = box_rotation.cos();
                let sin = box_rotation.sin();
                Vec2::new(
                    normal_local.x * cos - normal_local.y * sin,
                    normal_local.x * sin + normal_local.y * cos,
                )
            } else {
                normal_local
            };

            let contact_point = box_pos + normal * half_extents;

            let mut manifold = Manifold::new(circle_index, box_index);
            manifold.normal = normal;
            manifold.penetration = penetration + radius;
            manifold.add_contact(Contact::new(contact_point, normal, manifold.penetration));
            Some(manifold)
        } else if dist_sq < radius * radius {
            let dist = dist_sq.sqrt();
            let normal_local = if dist > 0.0 {
                diff / dist
            } else {
                Vec2::new(1.0, 0.0)
            };

            // 将法线转换回世界坐标系
            let normal = if is_rotated {
                let cos = box_rotation.cos();
                let sin = box_rotation.sin();
                Vec2::new(
                    normal_local.x * cos - normal_local.y * sin,
                    normal_local.x * sin + normal_local.y * cos,
                )
            } else {
                normal_local
            };

            let penetration = radius - dist;
            let contact_point_world = if is_rotated {
                let cos = box_rotation.cos();
                let sin = box_rotation.sin();
                box_pos + Vec2::new(
                    closest_point.x * cos - closest_point.y * sin,
                    closest_point.x * sin + closest_point.y * cos,
                )
            } else {
                box_pos + closest_point
            };

            let mut manifold = Manifold::new(circle_index, box_index);
            manifold.normal = normal;
            manifold.penetration = penetration;
            manifold.add_contact(Contact::new(contact_point_world, normal, penetration));
            Some(manifold)
        } else {
            None
        }
    }

    /// AABB-AABB 碰撞检测
    fn aabb_aabb_collision(
        &self,
        index_a: usize,
        index_b: usize,
        pos_a: Vec2,
        pos_b: Vec2,
        half_extents_a: Vec2,
        half_extents_b: Vec2,
    ) -> Option<Manifold> {
        let diff = pos_b - pos_a;
        let overlap_x = half_extents_a.x + half_extents_b.x - diff.x.abs();
        let overlap_y = half_extents_a.y + half_extents_b.y - diff.y.abs();

        if overlap_x <= 0.0 || overlap_y <= 0.0 {
            return None;
        }

        // 选择穿透最小的轴作为碰撞法线
        let (penetration, normal) = if overlap_x < overlap_y {
            (overlap_x, Vec2::new(diff.x.signum(), 0.0))
        } else {
            (overlap_y, Vec2::new(0.0, diff.y.signum()))
        };

        let contact_point = pos_a + normal * half_extents_a;

        let mut manifold = Manifold::new(index_a, index_b);
        manifold.normal = normal;
        manifold.penetration = penetration;
        manifold.add_contact(Contact::new(contact_point, normal, penetration));
        Some(manifold)
    }

    /// 检查 AABB 重叠
    fn check_aabb_overlap(&self, index_a: usize, index_b: usize) -> bool {
        let body_a = self.bodies.get(index_a);
        let body_b = self.bodies.get(index_b);

        if body_a.is_none() || body_b.is_none() {
            return false;
        }

        let body_a = body_a.unwrap();
        let body_b = body_b.unwrap();

        // 获取碰撞体 AABB
        let collider_a_idx = body_a.collider_indices().first();
        let collider_b_idx = body_b.collider_indices().first();

        if collider_a_idx.is_none() || collider_b_idx.is_none() {
            return false;
        }

        let collider_a = self.colliders.get(*collider_a_idx.unwrap());
        let collider_b = self.colliders.get(*collider_b_idx.unwrap());

        if collider_a.is_none() || collider_b.is_none() {
            return false;
        }

        let collider_a = collider_a.unwrap();
        let collider_b = collider_b.unwrap();

        let aabb_a = collider_a.shape().compute_aabb(body_a.position(), body_a.rotation());
        let aabb_b = collider_b.shape().compute_aabb(body_b.position(), body_b.rotation());

        aabb_a.intersects(&aabb_b)
    }

    /// 碰撞响应
    fn resolve_collisions(&mut self) {
        // 简化的碰撞响应实现
        // 实际实现需要考虑弹性、摩擦等因素
        let _ = self;
    }

    #[allow(dead_code)]
    fn resolve_contact(&self, _contact: &Contact) {}

    /// 关节约束求解
    fn solve_joints(&mut self) {
        for joint in &self.joints {
            self.apply_joint_constraint(joint);
        }
    }

    /// 应用关节约束
    fn apply_joint_constraint(&self, _joint: &Joint2D) {
        // 简化的关节实现
    }

    /// 位置修正
    fn correct_positions(&mut self) {
        let slop = 0.005;
        let baumgarte = 0.2;

        for manifold in self.manifolds.values_mut() {
            for contact in &mut manifold.contacts {
                let correction = contact.normal * contact.penetration * baumgarte;
                if correction.length() > slop {
                    // 位置修正
                }
            }
        }
    }

    /// 获取碰撞事件
    pub fn collision_events(&self) -> &VecDeque<CollisionEvent> {
        &self.collision_events
    }

    /// 清空碰撞事件
    pub fn clear_collision_events(&mut self) {
        self.collision_events.clear();
    }

    /// 获取刚体数量
    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    /// 获取碰撞体数量
    pub fn collider_count(&self) -> usize {
        self.colliders.len()
    }

    /// 获取关节数量
    pub fn joint_count(&self) -> usize {
        self.joints.len()
    }

    /// 获取仿真时间
    pub fn simulation_time(&self) -> f32 {
        self.simulation_time
    }

    /// 设置重力
    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.config.gravity = gravity;
    }

    /// 获取重力
    pub fn gravity(&self) -> Vec2 {
        self.config.gravity
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
        self.colliders.clear();
        self.joints.clear();
        self.collision_pairs.clear();
        self.manifolds.clear();
        self.collision_events.clear();
        self.simulation_time = 0.0;
        self.accumulator = 0.0;
    }

    /// 创建测试用物理世界
    pub fn test_world() -> Self {
        let mut world = Self::with_default_config();
        world.set_gravity(Vec2::new(0.0, -9.81));
        world
    }
}

impl Default for PhysicsWorld2D {
    fn default() -> Self {
        Self::with_default_config()
    }
}

/// 形状投射命中结果
#[derive(Debug, Clone)]
pub struct ShapeCastHit2D {
    /// 命中点
    pub point: Vec2,
    /// 法线
    pub normal: Vec2,
    /// 命中间隔 [0, 1]
    pub time: f32,
    /// 命中的碰撞体索引
    pub collider: usize,
}

/// 查询过滤器
///
/// 用于在空间查询时过滤不需要的碰撞体。
#[derive(Debug, Clone, Default)]
pub struct QueryFilter {
    /// 跳过的刚体索引列表
    pub skip_bodies: Vec<usize>,
    /// 是否包含传感器
    pub include_sensors: bool,
}

impl QueryFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self {
            skip_bodies: Vec::new(),
            include_sensors: true,
        }
    }

    /// 设置跳过的刚体列表
    pub fn with_skip_bodies(mut self, bodies: Vec<usize>) -> Self {
        self.skip_bodies = bodies;
        self
    }

    /// 设置是否包含传感器
    pub fn with_include_sensors(mut self, include: bool) -> Self {
        self.include_sensors = include;
        self
    }
}

impl PhysicsWorld2D {
    /// 形状投射
    ///
    /// 在指定方向上投射形状，返回第一个命中的碰撞体信息。
    pub fn shape_cast(
        &self,
        shape: &ColliderShape,
        origin: Vec2,
        dir: Vec2,
        max_toi: f32,
    ) -> Option<ShapeCastHit2D> {
        let dir = if dir.length() > 0.0 {
            dir.normalize()
        } else {
            return None;
        };

        let mut closest_hit: Option<ShapeCastHit2D> = None;
        let mut closest_t = max_toi;

        for (i, collider) in self.colliders.iter().enumerate() {
            if !collider.is_enabled() {
                continue;
            }

            // 获取碰撞体世界坐标
            if let Some(body) = self.bodies.get(i) {
                let world_pos = collider.world_position(body.position(), body.rotation());

                // 简化的形状投射实现
                // 实际实现需要根据形状类型计算
                match shape {
                    ColliderShape::Circle { radius } => {
                        // 射线与圆形求交
                        let oc = origin - world_pos;
                        let a = dir.dot(dir);
                        let b = 2.0 * oc.dot(dir);
                        let c = oc.dot(oc) - radius * radius;
                        let discriminant = b * b - 4.0 * a * c;

                        if discriminant >= 0.0 {
                            let sqrt_d = discriminant.sqrt();
                            let t = (-b - sqrt_d) / (2.0 * a);
                            if t >= 0.0 && t < closest_t {
                                closest_t = t;
                                let point = origin + dir * t;
                                let normal = (point - world_pos).normalize();
                                closest_hit = Some(ShapeCastHit2D {
                                    point,
                                    normal,
                                    time: t / max_toi,
                                    collider: i,
                                });
                            }
                        }
                    }
                    _ => {
                        // 对于非圆形，使用简化的 AABB 检测
                        let aabb = shape.compute_aabb(origin, 0.0);
                        let target_pos = origin + dir * max_toi;
                        let target_aabb = shape.compute_aabb(target_pos, 0.0);

                        // 检测两个 AABB 是否相交
                        if aabb.intersects(&target_aabb) {
                            closest_hit = Some(ShapeCastHit2D {
                                point: world_pos,
                                normal: dir,
                                time: 0.5,
                                collider: i,
                            });
                        }
                    }
                }
            }
        }

        closest_hit
    }

    /// AABB 重叠查询
    ///
    /// 返回与给定 AABB 相交的所有碰撞体索引。
    pub fn aabb_overlap(&self, aabb: Rect, filter: QueryFilter) -> Vec<usize> {
        let mut results = Vec::new();

        for (i, collider) in self.colliders.iter().enumerate() {
            if !collider.is_enabled() {
                continue;
            }

            // 检查是否在跳过列表中
            let body_idx = i; // 简化：假设碰撞体索引与刚体索引相同
            if filter.skip_bodies.contains(&body_idx) {
                continue;
            }

            // 检查传感器
            if !filter.include_sensors && collider.is_sensor() {
                continue;
            }

            // 获取碰撞体 AABB
            if let Some(body) = self.bodies.get(i) {
                let world_pos = collider.world_position(body.position(), body.rotation());
                let collider_aabb = collider.shape().compute_aabb(world_pos, body.rotation());

                if aabb.intersects(&collider_aabb) {
                    results.push(i);
                }
            }
        }

        results
    }

    /// 获取接触流形迭代器
    pub fn contact_manifolds(&self) -> impl Iterator<Item = &Manifold> {
        self.manifolds.values()
    }

    /// 获取关节迭代器
    pub fn joints_iter(&self) -> impl Iterator<Item = &Joint2D> {
        self.joints.iter()
    }

    /// 获取刚体迭代器
    pub fn bodies_iter(&self) -> impl Iterator<Item = &RigidBody2D> {
        self.bodies.iter()
    }

    /// 获取碰撞体迭代器
    pub fn colliders_iter(&self) -> impl Iterator<Item = &Collider2D> {
        self.colliders.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_world_creation() {
        let world = PhysicsWorld2D::with_default_config();
        assert_eq!(world.body_count(), 0);
        assert_eq!(world.collider_count(), 0);
    }

    #[test]
    fn test_add_body() {
        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        let index = world.add_body(body);
        assert_eq!(index, 0);
        assert_eq!(world.body_count(), 1);
    }

    #[test]
    fn test_remove_body() {
        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        world.add_body(body);
        world.remove_body(0);
        assert_eq!(world.body_count(), 0);
    }

    #[test]
    fn test_gravity() {
        let mut world = PhysicsWorld2D::with_default_config();
        assert_eq!(world.gravity(), Vec2::new(0.0, -9.81));

        world.set_gravity(Vec2::new(0.0, -20.0));
        assert_eq!(world.gravity(), Vec2::new(0.0, -20.0));
    }

    #[test]
    fn test_step() {
        let mut world = PhysicsWorld2D::with_default_config();
        world.step(1.0 / 60.0);
        assert!(world.simulation_time() > 0.0);
    }

    #[test]
    fn test_clear() {
        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        world.add_body(body);
        world.clear();
        assert_eq!(world.body_count(), 0);
    }

    #[test]
    fn test_remove_body_cleans_up_colliders() {
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        let body_index = world.add_body(body);

        // 添加一个碰撞体
        let collider = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider, body_index);

        assert_eq!(world.body_count(), 1);
        assert_eq!(world.collider_count(), 1);

        // 移除刚体应该同时移除其碰撞体
        world.remove_body(body_index);

        assert_eq!(world.body_count(), 0);
        assert_eq!(world.collider_count(), 0);
    }

    #[test]
    fn test_remove_collider_updates_body_indices() {
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        let body_index = world.add_body(body);

        // 添加两个碰撞体
        let collider1 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        let collider2 = Collider2D::new(ColliderShape::Circle { radius: 2.0 });
        let collider1_index = world.add_collider(collider1, body_index);
        world.add_collider(collider2, body_index);

        assert_eq!(collider1_index, 0);

        // 移除第一个碰撞体后，第二个碰撞体的索引应该更新
        world.remove_collider(collider1_index);

        // 刚体的碰撞体索引应该已更新
        let body = world.get_body(body_index);
        assert!(body.is_some());
        // 原来的索引 1 应该变成 0
        assert_eq!(body.unwrap().collider_indices(), &[0]);
    }
}
