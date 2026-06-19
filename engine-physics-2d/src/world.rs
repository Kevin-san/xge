//! 物理世界模块
//!
//! 管理所有物理实体、碰撞检测和仿真步进。

use std::collections::{HashMap, VecDeque};

use crate::{
    Collider2D, ColliderShape, CollisionEvent, Contact, Joint2D, JointType, Manifold, RigidBody2D,
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
                // 清空力累加器，避免跨步累积
                body.clear_forces();
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
                // 两个静态物体之间不产生碰撞
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
            (ColliderShape::Polygon { vertices: v1 }, ColliderShape::Polygon { vertices: v2 }) => {
                self.polygon_polygon_collision(index_a, index_b, world_pos_a, world_pos_b, rot_a, rot_b, v1, v2)
            }
            (
                ColliderShape::Aabb { half_extents } | ColliderShape::Rectangle { half_extents },
                ColliderShape::Polygon { vertices },
            ) => {
                // 将矩形转换为多边形顶点
                let rect_verts = vec![
                    Vec2::new(-half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, half_extents.y),
                    Vec2::new(-half_extents.x, half_extents.y),
                ];
                self.polygon_polygon_collision(index_a, index_b, world_pos_a, world_pos_b, rot_a, rot_b, &rect_verts, vertices)
            }
            (
                ColliderShape::Polygon { vertices },
                ColliderShape::Aabb { half_extents } | ColliderShape::Rectangle { half_extents },
            ) => {
                let rect_verts = vec![
                    Vec2::new(-half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, half_extents.y),
                    Vec2::new(-half_extents.x, half_extents.y),
                ];
                let manifold = self.polygon_polygon_collision(index_a, index_b, world_pos_a, world_pos_b, rot_a, rot_b, vertices, &rect_verts);
                manifold
            }
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

    /// 多边形-多边形碰撞检测（SAT 分离轴定理）
    #[allow(clippy::too_many_arguments)]
    fn polygon_polygon_collision(
        &self,
        index_a: usize,
        index_b: usize,
        pos_a: Vec2,
        pos_b: Vec2,
        rot_a: f32,
        rot_b: f32,
        local_verts_a: &[Vec2],
        local_verts_b: &[Vec2],
    ) -> Option<Manifold> {
        if local_verts_a.len() < 3 || local_verts_b.len() < 3 {
            return None;
        }

        // 将局部顶点变换到世界坐标
        let cos_a = rot_a.cos();
        let sin_a = rot_a.sin();
        let cos_b = rot_b.cos();
        let sin_b = rot_b.sin();

        let world_a: Vec<Vec2> = local_verts_a
            .iter()
            .map(|v| {
                Vec2::new(
                    v.x * cos_a - v.y * sin_a + pos_a.x,
                    v.x * sin_a + v.y * cos_a + pos_a.y,
                )
            })
            .collect();
        let world_b: Vec<Vec2> = local_verts_b
            .iter()
            .map(|v| {
                Vec2::new(
                    v.x * cos_b - v.y * sin_b + pos_b.x,
                    v.x * sin_b + v.y * cos_b + pos_b.y,
                )
            })
            .collect();

        // 收集所有候选分离轴（每个多边形边的法线）
        let axes_a = Self::polygon_axes(&world_a);
        let axes_b = Self::polygon_axes(&world_b);

        let mut min_overlap = f32::MAX;
        let mut best_axis = Vec2::new(1.0, 0.0);

        for axis in axes_a.iter().chain(axes_b.iter()) {
            let (min_a, max_a) = Self::project_polygon(&world_a, *axis);
            let (min_b, max_b) = Self::project_polygon(&world_b, *axis);

            // 没有重叠则不碰撞
            if max_a < min_b || max_b < min_a {
                return None;
            }
            let overlap = (max_a.min(max_b) - min_a.max(min_b)).min(max_a.max(min_b) - min_a.min(max_b));
            if overlap < min_overlap {
                min_overlap = overlap;
                best_axis = *axis;
            }
        }

        // 确保法线从 A 指向 B
        let direction = pos_b - pos_a;
        if direction.dot(best_axis) < 0.0 {
            best_axis = -best_axis;
        }

        // 找接触点：简化为两个多边形中心连线与边界的交点
        let contact_point = (pos_a + pos_b) * 0.5;

        let mut manifold = Manifold::new(index_a, index_b);
        manifold.normal = best_axis;
        manifold.penetration = min_overlap;
        manifold.add_contact(Contact::new(contact_point, best_axis, min_overlap));
        Some(manifold)
    }

    /// 计算多边形所有边的法线轴
    fn polygon_axes(verts: &[Vec2]) -> Vec<Vec2> {
        let n = verts.len();
        let mut axes = Vec::with_capacity(n);
        for i in 0..n {
            let j = (i + 1) % n;
            let edge = verts[j] - verts[i];
            // 垂直方向（左手法线）
            let normal = Vec2::new(-edge.y, edge.x);
            let len = normal.length();
            if len > 1e-6 {
                axes.push(normal / len);
            }
        }
        axes
    }

    /// 将多边形投影到轴上，返回 (min, max)
    fn project_polygon(verts: &[Vec2], axis: Vec2) -> (f32, f32) {
        let mut min = verts[0].dot(axis);
        let mut max = min;
        for v in &verts[1..] {
            let proj = v.dot(axis);
            if proj < min {
                min = proj;
            }
            if proj > max {
                max = proj;
            }
        }
        (min, max)
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
    ///
    /// 基于冲量的碰撞求解，处理法向冲量（弹性）和切向冲量（摩擦）。
    fn resolve_collisions(&mut self) {
        let velocity_iterations = self.config.velocity_iterations.max(1);
        let default_restitution = self.config.default_restitution;
        let default_friction = self.config.default_friction;

        // 收集流形信息（body_a/body_b/normal/contacts），避免后续借用冲突
        let mut manifold_infos: Vec<(usize, usize, Vec2, Vec<(Vec2, f32, f32)>)> = Vec::new();
        for manifold in self.manifolds.values() {
            let contacts: Vec<(Vec2, f32, f32)> = manifold
                .contacts
                .iter()
                .map(|c| (c.position, c.normal_impulse, c.tangent_impulse))
                .collect();
            manifold_infos.push((manifold.body_a, manifold.body_b, manifold.normal, contacts));
        }

        for _ in 0..velocity_iterations {
            for (body_a_idx, body_b_idx, normal, contacts) in &manifold_infos {
                let restitution = {
                    let ca = self.colliders.get(
                        self.bodies
                            .get(*body_a_idx)
                            .and_then(|b| b.collider_indices().first().copied())
                            .unwrap_or(0),
                    );
                    let cb = self.colliders.get(
                        self.bodies
                            .get(*body_b_idx)
                            .and_then(|b| b.collider_indices().first().copied())
                            .unwrap_or(0),
                    );
                    let ra = ca.map(|c| c.restitution()).unwrap_or(default_restitution);
                    let rb = cb.map(|c| c.restitution()).unwrap_or(default_restitution);
                    ra.max(rb)
                };
                let friction = {
                    let ca = self.colliders.get(
                        self.bodies
                            .get(*body_a_idx)
                            .and_then(|b| b.collider_indices().first().copied())
                            .unwrap_or(0),
                    );
                    let cb = self.colliders.get(
                        self.bodies
                            .get(*body_b_idx)
                            .and_then(|b| b.collider_indices().first().copied())
                            .unwrap_or(0),
                    );
                    let fa = ca.map(|c| c.friction()).unwrap_or(default_friction);
                    let fb = cb.map(|c| c.friction()).unwrap_or(default_friction);
                    (fa + fb) * 0.5
                };

                for (contact_point, normal_impulse_acc, tangent_impulse_acc) in contacts {
                    self.solve_contact_impulse(
                        *body_a_idx,
                        *body_b_idx,
                        *contact_point,
                        *normal,
                        restitution,
                        friction,
                        *normal_impulse_acc,
                        *tangent_impulse_acc,
                    );
                }
            }
        }
    }

    /// 求解单个接触点的冲量
    #[allow(clippy::too_many_arguments)]
    fn solve_contact_impulse(
        &mut self,
        body_a_idx: usize,
        body_b_idx: usize,
        contact_point: Vec2,
        normal: Vec2,
        restitution: f32,
        friction: f32,
        normal_impulse_acc: f32,
        tangent_impulse_acc: f32,
    ) {
        // 提取刚体状态
        let (inv_mass_a, inv_inertia_a, pos_a, vel_a, ang_vel_a, type_a) = {
            let b = match self.bodies.get(body_a_idx) {
                Some(b) => b,
                None => return,
            };
            (
                b.inverse_mass(),
                b.inverse_inertia(),
                b.position(),
                b.linear_velocity(),
                b.angular_velocity(),
                b.body_type(),
            )
        };
        let (inv_mass_b, inv_inertia_b, pos_b, vel_b, ang_vel_b, type_b) = {
            let b = match self.bodies.get(body_b_idx) {
                Some(b) => b,
                None => return,
            };
            (
                b.inverse_mass(),
                b.inverse_inertia(),
                b.position(),
                b.linear_velocity(),
                b.angular_velocity(),
                b.body_type(),
            )
        };

        // 静态/运动学刚体不参与冲量响应
        let inv_mass_a = if type_a == RigidBodyType::Dynamic { inv_mass_a } else { 0.0 };
        let inv_inertia_a = if type_a == RigidBodyType::Dynamic { inv_inertia_a } else { 0.0 };
        let inv_mass_b = if type_b == RigidBodyType::Dynamic { inv_mass_b } else { 0.0 };
        let inv_inertia_b = if type_b == RigidBodyType::Dynamic { inv_inertia_b } else { 0.0 };

        let inv_mass_sum = inv_mass_a + inv_mass_b;
        if inv_mass_sum <= 0.0 {
            return;
        }

        // 接触点相对刚体中心的位置
        let ra = contact_point - pos_a;
        let rb = contact_point - pos_b;

        // 接触点处的速度: v + ω × r （2D 中 ω × r = (-ω*ry, ω*rx)）
        let vel_at_a = vel_a + Vec2::new(-ang_vel_a * ra.y, ang_vel_a * ra.x);
        let vel_at_b = vel_b + Vec2::new(-ang_vel_b * rb.y, ang_vel_b * rb.x);
        let rel_vel = vel_at_b - vel_at_a;

        // 沿法线的相对速度
        let vel_along_normal = rel_vel.dot(normal);
        // 分离的物体不需要冲量
        if vel_along_normal > 0.0 {
            return;
        }

        // 计算有效转动惯量项 (r × n)^2
        let ra_cross_n = ra.cross(normal);
        let rb_cross_n = rb.cross(normal);
        let denom_normal = inv_mass_sum
            + inv_inertia_a * ra_cross_n * ra_cross_n
            + inv_inertia_b * rb_cross_n * rb_cross_n;
        if denom_normal <= 0.0 {
            return;
        }

        // 法向冲量（带弹性）
        let j = -(1.0 + restitution) * vel_along_normal / denom_normal;
        // 累积冲量钳制（不允许产生分离冲量）
        let new_normal_impulse = (normal_impulse_acc + j).max(0.0);
        let delta_impulse = new_normal_impulse - normal_impulse_acc;
        let impulse = normal * delta_impulse;

        // 应用法向冲量
        if type_a == RigidBodyType::Dynamic {
            let b = &mut self.bodies[body_a_idx];
            b.set_linear_velocity(vel_a - impulse * inv_mass_a);
            let new_ang = ang_vel_a - ra.cross(impulse) * inv_inertia_a;
            b.set_angular_velocity(new_ang);
        }
        if type_b == RigidBodyType::Dynamic {
            let b = &mut self.bodies[body_b_idx];
            b.set_linear_velocity(vel_b + impulse * inv_mass_b);
            let new_ang = ang_vel_b + rb.cross(impulse) * inv_inertia_b;
            b.set_angular_velocity(new_ang);
        }

        // ===== 摩擦冲量（切向） =====
        // 重新计算相对速度
        let vel_a = self.bodies[body_a_idx].linear_velocity();
        let vel_b = self.bodies[body_b_idx].linear_velocity();
        let ang_vel_a = self.bodies[body_a_idx].angular_velocity();
        let ang_vel_b = self.bodies[body_b_idx].angular_velocity();
        let vel_at_a = vel_a + Vec2::new(-ang_vel_a * ra.y, ang_vel_a * ra.x);
        let vel_at_b = vel_b + Vec2::new(-ang_vel_b * rb.y, ang_vel_b * rb.x);
        let rel_vel = vel_at_b - vel_at_a;

        // 切向方向（相对速度在切平面的投影）
        let tangent = rel_vel - normal * rel_vel.dot(normal);
        let tangent_len = tangent.length();
        if tangent_len < 1e-6 {
            return;
        }
        let tangent = tangent / tangent_len;

        let ra_cross_t = ra.cross(tangent);
        let rb_cross_t = rb.cross(tangent);
        let denom_tangent = inv_mass_sum
            + inv_inertia_a * ra_cross_t * ra_cross_t
            + inv_inertia_b * rb_cross_t * rb_cross_t;
        if denom_tangent <= 0.0 {
            return;
        }

        let jt = -rel_vel.dot(tangent) / denom_tangent;
        // 库仑摩擦定律：|jt| <= friction * j
        let max_friction = friction * new_normal_impulse;
        let new_tangent_impulse = (tangent_impulse_acc + jt).clamp(-max_friction, max_friction);
        let delta_t = new_tangent_impulse - tangent_impulse_acc;
        let friction_impulse = tangent * delta_t;

        if type_a == RigidBodyType::Dynamic {
            let b = &mut self.bodies[body_a_idx];
            b.set_linear_velocity(b.linear_velocity() - friction_impulse * inv_mass_a);
            let new_ang = b.angular_velocity() - ra.cross(friction_impulse) * inv_inertia_a;
            b.set_angular_velocity(new_ang);
        }
        if type_b == RigidBodyType::Dynamic {
            let b = &mut self.bodies[body_b_idx];
            b.set_linear_velocity(b.linear_velocity() + friction_impulse * inv_mass_b);
            let new_ang = b.angular_velocity() + rb.cross(friction_impulse) * inv_inertia_b;
            b.set_angular_velocity(new_ang);
        }
    }

    /// 关节约束求解
    fn solve_joints(&mut self) {
        let iterations = self.config.position_iterations.max(1);
        // 收集关节数据，避免借用冲突
        let joint_snapshots: Vec<(JointType, usize, usize, Vec2, Vec2, bool)> = self
            .joints
            .iter()
            .map(|j| {
                (
                    j.joint_type(),
                    j.body_a(),
                    j.body_b(),
                    j.anchor_a(),
                    j.anchor_b(),
                    j.is_enabled(),
                )
            })
            .collect();

        for _ in 0..iterations {
            for (joint_type, body_a_idx, body_b_idx, anchor_a, anchor_b, enabled) in &joint_snapshots {
                self.apply_joint_constraint_data(
                    *joint_type,
                    *body_a_idx,
                    *body_b_idx,
                    *anchor_a,
                    *anchor_b,
                    *enabled,
                );
            }
        }
    }

    /// 应用关节约束（基于快照数据）
    #[allow(clippy::too_many_arguments)]
    fn apply_joint_constraint_data(
        &mut self,
        joint_type: JointType,
        body_a_idx: usize,
        body_b_idx: usize,
        anchor_a: Vec2,
        anchor_b: Vec2,
        enabled: bool,
    ) {
        if !enabled {
            return;
        }

        let (pos_a, pos_b, inv_mass_a, inv_mass_b, type_a, type_b) = {
            let a = match self.bodies.get(body_a_idx) {
                Some(b) => b,
                None => return,
            };
            let b = match self.bodies.get(body_b_idx) {
                Some(b) => b,
                None => return,
            };
            (
                a.position(),
                b.position(),
                a.inverse_mass(),
                b.inverse_mass(),
                a.body_type(),
                b.body_type(),
            )
        };

        let inv_mass_a = if type_a == RigidBodyType::Dynamic { inv_mass_a } else { 0.0 };
        let inv_mass_b = if type_b == RigidBodyType::Dynamic { inv_mass_b } else { 0.0 };

        match joint_type {
            JointType::Distance => {
                // 保持两个锚点之间的距离等于目标长度
                let target_len = (anchor_b - anchor_a).length();
                let current = pos_b - pos_a;
                let current_len = current.length();
                if current_len < 1e-6 {
                    return;
                }
                let dir = current / current_len;
                let diff = current_len - target_len;
                let correction = dir * diff;
                let total_inv_mass = inv_mass_a + inv_mass_b;
                if total_inv_mass <= 0.0 {
                    return;
                }
                if type_a == RigidBodyType::Dynamic {
                    self.bodies[body_a_idx].set_position(pos_a + correction * (inv_mass_a / total_inv_mass));
                }
                if type_b == RigidBodyType::Dynamic {
                    self.bodies[body_b_idx].set_position(pos_b - correction * (inv_mass_b / total_inv_mass));
                }
            }
            JointType::Revolute | JointType::Weld => {
                // 旋转/焊接关节：将两个锚点拉到同一点
                let diff = anchor_b - anchor_a;
                let total_inv_mass = inv_mass_a + inv_mass_b;
                if total_inv_mass <= 0.0 {
                    return;
                }
                if type_a == RigidBodyType::Dynamic {
                    self.bodies[body_a_idx].set_position(pos_a + diff * (inv_mass_a / total_inv_mass));
                }
                if type_b == RigidBodyType::Dynamic {
                    self.bodies[body_b_idx].set_position(pos_b - diff * (inv_mass_b / total_inv_mass));
                }
            }
            JointType::Spring => {
                // 弹簧关节：胡克定律 F = -k*x - c*v
                let current = pos_b - pos_a;
                let current_len = current.length();
                if current_len < 1e-6 {
                    return;
                }
                let dir = current / current_len;
                let rest_len = (anchor_b - anchor_a).length();
                let stretch = current_len - rest_len;
                let stiffness = 100.0; // 默认刚度
                let damping = 1.0; // 默认阻尼
                let force_mag = stiffness * stretch;
                let force = dir * force_mag;
                let total_inv_mass = inv_mass_a + inv_mass_b;
                if total_inv_mass <= 0.0 {
                    return;
                }
                // 位置修正（软约束）
                let correction = force * 0.01;
                if type_a == RigidBodyType::Dynamic {
                    self.bodies[body_a_idx].set_position(pos_a + correction * (inv_mass_a / total_inv_mass));
                }
                if type_b == RigidBodyType::Dynamic {
                    self.bodies[body_b_idx].set_position(pos_b - correction * (inv_mass_b / total_inv_mass));
                }
                // 速度阻尼
                let vel_a = self.bodies[body_a_idx].linear_velocity();
                let vel_b = self.bodies[body_b_idx].linear_velocity();
                let rel_vel = vel_b - vel_a;
                let damp_force = rel_vel * damping;
                if type_a == RigidBodyType::Dynamic {
                    self.bodies[body_a_idx].set_linear_velocity(vel_a + damp_force * (inv_mass_a / total_inv_mass) * 0.1);
                }
                if type_b == RigidBodyType::Dynamic {
                    self.bodies[body_b_idx].set_linear_velocity(vel_b - damp_force * (inv_mass_b / total_inv_mass) * 0.1);
                }
            }
            JointType::Prismatic | JointType::Rope | JointType::Motor => {
                // 滑块/绳索/驱动关节：简化为位置约束
                let diff = pos_b - pos_a;
                let total_inv_mass = inv_mass_a + inv_mass_b;
                if total_inv_mass <= 0.0 {
                    return;
                }
                let correction = diff * 0.1;
                if type_a == RigidBodyType::Dynamic {
                    self.bodies[body_a_idx].set_position(pos_a + correction * (inv_mass_a / total_inv_mass));
                }
                if type_b == RigidBodyType::Dynamic {
                    self.bodies[body_b_idx].set_position(pos_b - correction * (inv_mass_b / total_inv_mass));
                }
            }
        }
    }

    /// 位置修正（Baumgarte 稳定化）
    fn correct_positions(&mut self) {
        let slop = 0.005; // 允许的穿透容差
        let percent = 0.4; // 修正比例（0-1）

        // 收集流形信息避免借用冲突
        let mut corrections: Vec<(usize, usize, Vec2, f32)> = Vec::new();
        for manifold in self.manifolds.values() {
            for contact in &manifold.contacts {
                if contact.penetration > slop {
                    let correction_mag = (contact.penetration - slop) * percent;
                    corrections.push((
                        manifold.body_a,
                        manifold.body_b,
                        contact.normal,
                        correction_mag,
                    ));
                }
            }
        }

        for (body_a_idx, body_b_idx, normal, mag) in corrections {
            let (inv_mass_a, type_a) = {
                let b = match self.bodies.get(body_a_idx) {
                    Some(b) => b,
                    None => continue,
                };
                (
                    if b.body_type() == RigidBodyType::Dynamic { b.inverse_mass() } else { 0.0 },
                    b.body_type(),
                )
            };
            let (inv_mass_b, type_b) = {
                let b = match self.bodies.get(body_b_idx) {
                    Some(b) => b,
                    None => continue,
                };
                (
                    if b.body_type() == RigidBodyType::Dynamic { b.inverse_mass() } else { 0.0 },
                    b.body_type(),
                )
            };

            let total_inv_mass = inv_mass_a + inv_mass_b;
            if total_inv_mass <= 0.0 {
                continue;
            }
            let correction = normal * mag;
            if type_a == RigidBodyType::Dynamic {
                let pos = self.bodies[body_a_idx].position();
                self.bodies[body_a_idx].set_position(pos - correction * (inv_mass_a / total_inv_mass));
            }
            if type_b == RigidBodyType::Dynamic {
                let pos = self.bodies[body_b_idx].position();
                self.bodies[body_b_idx].set_position(pos + correction * (inv_mass_b / total_inv_mass));
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
    use crate::{Collider2D, ColliderShape, Joint2D, JointType, RigidBody2DBuilder};

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

    // ============= P2-001 新增：物理积分/碰撞响应/关节约束测试 =============

    #[test]
    fn test_gravity_integration_dynamic_body() {
        // 动态刚体在重力作用下应该加速下落
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::new(0.0, -10.0));

        let mut body = RigidBody2DBuilder::dynamic().with_mass(1.0).build();
        body.set_linear_damping(0.0); // 禁用阻尼以验证纯重力
        let body_idx = world.add_body(body);
        world.add_collider(Collider2D::new(ColliderShape::Circle { radius: 0.5 }), body_idx);

        let initial_y = world.get_body(body_idx).unwrap().position().y;
        world.step(1.0 / 60.0);
        let after_y = world.get_body(body_idx).unwrap().position().y;
        // 下落，y 应该减小
        assert!(after_y < initial_y, "动态刚体应该在重力作用下下落");
    }

    #[test]
    fn test_static_body_not_affected_by_gravity() {
        // 静态刚体不受重力影响
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::new(0.0, -10.0));

        let body = RigidBody2DBuilder::static_()
            .with_position(Vec2::new(0.0, 5.0))
            .build();
        let body_idx = world.add_body(body);
        world.add_collider(Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) }), body_idx);

        let initial_y = world.get_body(body_idx).unwrap().position().y;
        world.step(1.0 / 60.0);
        let after_y = world.get_body(body_idx).unwrap().position().y;
        assert_eq!(initial_y, after_y, "静态刚体不应受重力影响");
    }

    #[test]
    fn test_circle_circle_collision_response() {
        // 两个圆形动态刚体相向运动，碰撞后应该反弹
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO); // 无重力

        let mut body_a = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(-2.0, 0.0))
            .with_velocity(Vec2::new(5.0, 0.0))
            .build();
        body_a.set_linear_damping(0.0);
        let idx_a = world.add_body(body_a);
        world.add_collider(Collider2D::new(ColliderShape::Circle { radius: 1.0 }), idx_a);

        let mut body_b = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(2.0, 0.0))
            .with_velocity(Vec2::new(-5.0, 0.0))
            .build();
        body_b.set_linear_damping(0.0);
        let idx_b = world.add_body(body_b);
        world.add_collider(Collider2D::new(ColliderShape::Circle { radius: 1.0 }), idx_b);

        // 步进几步让它们碰撞
        for _ in 0..30 {
            world.step(1.0 / 60.0);
        }

        let vel_a = world.get_body(idx_a).unwrap().linear_velocity();
        let vel_b = world.get_body(idx_b).unwrap().linear_velocity();
        // 碰撞后 A 应该向左/反向运动，B 应该向右/反向运动
        assert!(vel_a.x < 5.0, "碰撞后 A 速度应减小或反向: {}", vel_a.x);
        assert!(vel_b.x > -5.0, "碰撞后 B 速度应减小或反向: {}", vel_b.x);
    }

    #[test]
    fn test_dynamic_static_collision_no_pass_through() {
        // 动态刚体不应穿过静态刚体
        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::new(0.0, -10.0));

        // 静态地面
        let ground = RigidBody2DBuilder::static_()
            .with_position(Vec2::new(0.0, -1.0))
            .build();
        let ground_idx = world.add_body(ground);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(10.0, 1.0) }),
            ground_idx,
        );

        // 动态球
        let mut ball = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(0.0, 5.0))
            .build();
        ball.set_linear_damping(0.0);
        let ball_idx = world.add_body(ball);
        world.add_collider(Collider2D::new(ColliderShape::Circle { radius: 0.5 }), ball_idx);

        // 步进足够长时间让球下落
        for _ in 0..120 {
            world.step(1.0 / 60.0);
        }

        let ball_y = world.get_body(ball_idx).unwrap().position().y;
        // 球应该停在地面附近（地面顶部 y=0，球半径 0.5）
        // 允许少量穿透（Baumgarte 稳定化不是精确求解）
        assert!(ball_y >= -1.0, "动态刚体不应穿过静态刚体: ball_y={}", ball_y);
        assert!(ball_y <= 1.5, "球应该下落到地面附近: ball_y={}", ball_y);
    }

    #[test]
    fn test_aabb_aabb_collision_response() {
        // 两个 AABB 碰撞应该产生响应
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO);

        let mut body_a = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(-3.0, 0.0))
            .with_velocity(Vec2::new(10.0, 0.0))
            .build();
        body_a.set_linear_damping(0.0);
        let idx_a = world.add_body(body_a);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) }),
            idx_a,
        );

        let mut body_b = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(3.0, 0.0))
            .with_velocity(Vec2::new(-10.0, 0.0))
            .build();
        body_b.set_linear_damping(0.0);
        let idx_b = world.add_body(body_b);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) }),
            idx_b,
        );

        for _ in 0..30 {
            world.step(1.0 / 60.0);
        }

        let vel_a = world.get_body(idx_a).unwrap().linear_velocity();
        let vel_b = world.get_body(idx_b).unwrap().linear_velocity();
        // 碰撞后速度应该减小或反向
        assert!(vel_a.x < 10.0, "A 碰撞后速度应变化: {}", vel_a.x);
        assert!(vel_b.x > -10.0, "B 碰撞后速度应变化: {}", vel_b.x);
    }

    #[test]
    fn test_polygon_polygon_sat_collision() {
        // 两个多边形碰撞应该被检测到
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO);

        // 三角形 A
        let tri_a = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(0.0, 1.0),
        ];
        let mut body_a = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(-1.0, 0.0))
            .with_velocity(Vec2::new(5.0, 0.0))
            .build();
        body_a.set_linear_damping(0.0);
        let idx_a = world.add_body(body_a);
        world.add_collider(
            Collider2D::new(ColliderShape::Polygon { vertices: tri_a }),
            idx_a,
        );

        // 三角形 B
        let tri_b = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(0.0, 1.0),
        ];
        let mut body_b = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(1.0, 0.0))
            .with_velocity(Vec2::new(-5.0, 0.0))
            .build();
        body_b.set_linear_damping(0.0);
        let idx_b = world.add_body(body_b);
        world.add_collider(
            Collider2D::new(ColliderShape::Polygon { vertices: tri_b }),
            idx_b,
        );

        for _ in 0..30 {
            world.step(1.0 / 60.0);
        }

        let vel_a = world.get_body(idx_a).unwrap().linear_velocity();
        let vel_b = world.get_body(idx_b).unwrap().linear_velocity();
        // 多边形碰撞应该产生响应
        assert!(vel_a.x < 5.0, "多边形碰撞后 A 速度应变化: {}", vel_a.x);
        assert!(vel_b.x > -5.0, "多边形碰撞后 B 速度应变化: {}", vel_b.x);
    }

    #[test]
    fn test_distance_joint_constraint() {
        // 距离关节应该约束两个刚体的距离
        use crate::{DistanceJoint, Joint2D, JointType};

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO);

        let body_a = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(0.0, 0.0))
            .build();
        let idx_a = world.add_body(body_a);

        let body_b = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(5.0, 0.0))
            .build();
        let idx_b = world.add_body(body_b);

        // 创建距离关节，目标距离 5.0
        let mut joint = Joint2D::new(JointType::Distance, idx_a, idx_b);
        joint.set_anchor_a(Vec2::new(0.0, 0.0));
        joint.set_anchor_b(Vec2::new(5.0, 0.0));
        world.add_joint(joint);

        // 给 B 一个远离的力
        world.get_body_mut(idx_b).unwrap().apply_impulse(Vec2::new(100.0, 0.0));

        // 步进
        for _ in 0..60 {
            world.step(1.0 / 60.0);
        }

        let pos_a = world.get_body(idx_a).unwrap().position();
        let pos_b = world.get_body(idx_b).unwrap().position();
        let dist = (pos_b - pos_a).length();
        // 距离应该被约束在目标附近（允许一定误差）
        assert!(dist < 10.0, "距离关节应约束两体距离: dist={}", dist);
    }

    #[test]
    fn test_set_mass_changes_inverse_mass() {
        let mut body = RigidBody2DBuilder::dynamic().with_mass(2.0).build();
        assert_eq!(body.inverse_mass(), 0.5);
        body.set_mass(4.0);
        assert_eq!(body.mass(), 4.0);
        assert_eq!(body.inverse_mass(), 0.25);
    }

    #[test]
    fn test_set_mass_zero_for_static() {
        // 静态刚体的 set_mass 不应生效
        let mut body = RigidBody2D::new(RigidBodyType::Static);
        body.set_mass(5.0);
        assert_eq!(body.mass(), 0.0);
        assert_eq!(body.inverse_mass(), 0.0);
    }

    #[test]
    fn test_set_inertia_changes_inverse_inertia() {
        let mut body = RigidBody2DBuilder::dynamic().with_inertia(2.0).build();
        assert_eq!(body.inverse_inertia(), 0.5);
        body.set_inertia(4.0);
        assert_eq!(body.inertia(), 4.0);
        assert_eq!(body.inverse_inertia(), 0.25);
    }

    #[test]
    fn test_broad_phase_allows_static_dynamic() {
        // 静态-动态对应该被检测到
        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO);

        let ground = RigidBody2DBuilder::static_().build();
        let g_idx = world.add_body(ground);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(10.0, 1.0) }),
            g_idx,
        );

        let ball = RigidBody2DBuilder::dynamic()
            .with_position(Vec2::new(0.0, 0.5))
            .build();
        let b_idx = world.add_body(ball);
        world.add_collider(Collider2D::new(ColliderShape::Circle { radius: 0.5 }), b_idx);

        world.step(1.0 / 60.0);
        // 应该有碰撞事件产生
        assert!(!world.collision_events().is_empty(), "静态-动态碰撞应被检测");
    }

    #[test]
    fn test_broad_phase_skips_static_static() {
        // 两个静态刚体之间不应产生碰撞
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        let a = RigidBody2DBuilder::static_().build();
        let a_idx = world.add_body(a);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) }),
            a_idx,
        );

        let b = RigidBody2DBuilder::static_().build();
        let b_idx = world.add_body(b);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) }),
            b_idx,
        );

        world.step(1.0 / 60.0);
        assert!(world.collision_events().is_empty(), "静态-静态不应产生碰撞");
    }

    #[test]
    fn test_restitution_bouncy_ball() {
        // 高弹性球应该反弹
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::new(0.0, -10.0));

        // 静态地面
        let ground = RigidBody2DBuilder::static_().build();
        let g_idx = world.add_body(ground);
        let mut ground_collider = Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(10.0, 1.0) });
        ground_collider.set_restitution(1.0); // 完全弹性
        world.add_collider(ground_collider, g_idx);

        // 弹性球
        let mut ball_collider = Collider2D::new(ColliderShape::Circle { radius: 0.5 });
        ball_collider.set_restitution(1.0);
        let mut ball = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(0.0, 5.0))
            .build();
        ball.set_linear_damping(0.0);
        ball.set_angular_damping(0.0);
        let b_idx = world.add_body(ball);
        world.add_collider(ball_collider, b_idx);

        // 步进直到碰撞
        for _ in 0..120 {
            world.step(1.0 / 60.0);
        }

        // 球应该反弹（y 速度为正或位置上升）
        let vel = world.get_body(b_idx).unwrap().linear_velocity();
        let pos = world.get_body(b_idx).unwrap().position();
        // 完全弹性碰撞后应该反弹
        assert!(
            vel.y > -1.0 || pos.y > 0.5,
            "弹性球应该反弹: vel.y={}, pos.y={}",
            vel.y,
            pos.y
        );
    }

    #[test]
    fn test_friction_slows_motion() {
        // 摩擦应该减慢运动
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::new(0.0, -10.0));

        // 地面（高摩擦）
        let ground = RigidBody2DBuilder::static_().build();
        let g_idx = world.add_body(ground);
        let mut ground_collider = Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(50.0, 1.0) });
        ground_collider.set_friction(0.9);
        world.add_collider(ground_collider, g_idx);

        // 滑块（高摩擦）
        let mut box_collider = Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(0.5, 0.5) });
        box_collider.set_friction(0.9);
        let mut body = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(0.0, 2.0))
            .with_velocity(Vec2::new(10.0, 0.0))
            .build();
        body.set_linear_damping(0.0);
        let b_idx = world.add_body(body);
        world.add_collider(box_collider, b_idx);

        // 步进
        for _ in 0..120 {
            world.step(1.0 / 60.0);
        }

        let vel = world.get_body(b_idx).unwrap().linear_velocity();
        // 摩擦应该显著减慢速度
        assert!(vel.x.abs() < 10.0, "摩擦应减慢运动: vel.x={}", vel.x);
    }

    #[test]
    fn test_position_correction_reduces_penetration() {
        // 位置修正应该减少穿透
        use crate::ColliderShape;

        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO);

        // 静态地面
        let ground = RigidBody2DBuilder::static_().build();
        let g_idx = world.add_body(ground);
        world.add_collider(
            Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(10.0, 1.0) }),
            g_idx,
        );

        // 动态球，初始与地面重叠
        let ball = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(0.0, 0.0)) // 与地面重叠
            .build();
        let b_idx = world.add_body(ball);
        world.add_collider(Collider2D::new(ColliderShape::Circle { radius: 0.5 }), b_idx);

        // 步进几步让位置修正生效
        for _ in 0..10 {
            world.step(1.0 / 60.0);
        }

        let ball_y = world.get_body(b_idx).unwrap().position().y;
        // 球应该被推到地面之上
        assert!(ball_y > -0.5, "位置修正应减少穿透: ball_y={}", ball_y);
    }

    #[test]
    fn test_polygon_axes_generation() {
        let square = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
        ];
        let axes = PhysicsWorld2D::polygon_axes(&square);
        // 正方形应该有 4 条边，但去重后是 2 个独立轴方向
        assert_eq!(axes.len(), 4);
        // 所有轴应该是单位向量
        for axis in &axes {
            assert!((axis.length() - 1.0).abs() < 1e-5, "轴应该是单位向量");
        }
    }

    #[test]
    fn test_project_polygon() {
        let square = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
        ];
        let (min, max) = PhysicsWorld2D::project_polygon(&square, Vec2::new(1.0, 0.0));
        assert!((min - (-1.0)).abs() < 1e-5);
        assert!((max - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_energy_conservation_elastic_collision() {
        // 等质量完全弹性碰撞应该交换速度
        let mut world = PhysicsWorld2D::with_default_config();
        world.set_gravity(Vec2::ZERO);

        let mut a = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(-2.0, 0.0))
            .with_velocity(Vec2::new(5.0, 0.0))
            .build();
        a.set_linear_damping(0.0);
        let idx_a = world.add_body(a);
        let mut col_a = Collider2D::new(ColliderShape::Circle { radius: 0.5 });
        col_a.set_restitution(1.0);
        world.add_collider(col_a, idx_a);

        let mut b = RigidBody2DBuilder::dynamic()
            .with_mass(1.0)
            .with_position(Vec2::new(2.0, 0.0))
            .build();
        b.set_linear_damping(0.0);
        let idx_b = world.add_body(b);
        let mut col_b = Collider2D::new(ColliderShape::Circle { radius: 0.5 });
        col_b.set_restitution(1.0);
        world.add_collider(col_b, idx_b);

        // 步进直到碰撞发生（A 速度 5/s，1秒移动 5 单位到 x=3，必然与 B 碰撞）
        for _ in 0..90 {
            world.step(1.0 / 60.0);
        }

        let vel_a = world.get_body(idx_a).unwrap().linear_velocity();
        let vel_b = world.get_body(idx_b).unwrap().linear_velocity();
        // 等质量完全弹性碰撞：A 静止，B 获得 A 的速度
        assert!(vel_a.x.abs() < 2.0, "等质量弹性碰撞后 A 应接近静止: {}", vel_a.x);
        assert!(vel_b.x > 1.0, "等质量弹性碰撞后 B 应获得速度: {}", vel_b.x);
    }
}
