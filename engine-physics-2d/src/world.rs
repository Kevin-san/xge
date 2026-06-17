//! 物理世界模块
//!
//! 管理所有物理实体、碰撞检测和仿真步进。

use std::collections::{HashMap, VecDeque};

use crate::{Collider2D, CollisionEvent, Contact, Joint2D, Manifold, RigidBody2D, RigidBodyType};
use engine_math::{Rect, Vec2};

#[derive(Debug, Clone)]
pub struct PhysicsWorldConfig {
    pub gravity: Vec2,
    pub timestep: f32,
    pub max_substeps: usize,
    pub velocity_iterations: usize,
    pub position_iterations: usize,
    pub default_restitution: f32,
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

pub struct PhysicsWorld2D {
    config: PhysicsWorldConfig,
    bodies: Vec<RigidBody2D>,
    colliders: Vec<Collider2D>,
    joints: Vec<Joint2D>,
    collision_pairs: Vec<(usize, usize)>,
    manifolds: HashMap<(usize, usize), Manifold>,
    collision_events: VecDeque<CollisionEvent>,
    simulation_time: f32,
    accumulator: f32,
    collision_detection_enabled: bool,
    simulation_enabled: bool,
}

impl PhysicsWorld2D {
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

    pub fn with_default_config() -> Self {
        Self::new(PhysicsWorldConfig::default())
    }

    pub fn add_body(&mut self, body: RigidBody2D) -> usize {
        let index = self.bodies.len();
        self.bodies.push(body);
        index
    }

    pub fn remove_body(&mut self, index: usize) {
        if index >= self.bodies.len() {
            return;
        }
        
        let collider_indices = self.bodies[index].collider_indices().to_vec();
        
        for &collider_idx in &collider_indices {
            if collider_idx < self.colliders.len() {
                self.colliders.remove(collider_idx);
            }
        }
        
        self.bodies.remove(index);
        self.reindex_colliders_and_pairs();
    }
    
    fn reindex_colliders_and_pairs(&mut self) {
        self.collision_pairs.retain(|(i, j)| {
            *i < self.colliders.len() && *j < self.colliders.len()
        });
        
        self.manifolds.retain(|key, _| {
            let (i, j) = *key;
            i < self.colliders.len() && j < self.colliders.len()
        });
    }

    pub fn get_body(&self, index: usize) -> Option<&RigidBody2D> {
        self.bodies.get(index)
    }

    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut RigidBody2D> {
        self.bodies.get_mut(index)
    }

    pub fn add_collider(&mut self, collider: Collider2D, body_index: usize) -> usize {
        let index = self.colliders.len();
        if body_index < self.bodies.len() {
            self.bodies[body_index].add_collider_index(index);
        }
        self.colliders.push(collider);
        index
    }

    pub fn remove_collider(&mut self, index: usize) {
        if index >= self.colliders.len() {
            return;
        }
        
        for body in &mut self.bodies {
            body.remove_collider_index(index);
        }
        
        self.colliders.remove(index);
        
        for body in &mut self.bodies {
            body.update_collider_indices_after_remove(index);
        }
        
        self.reindex_colliders_and_pairs();
    }

    pub fn get_collider(&self, index: usize) -> Option<&Collider2D> {
        self.colliders.get(index)
    }

    pub fn get_collider_mut(&mut self, index: usize) -> Option<&mut Collider2D> {
        self.colliders.get_mut(index)
    }

    pub fn add_joint(&mut self, joint: Joint2D) -> usize {
        let index = self.joints.len();
        self.joints.push(joint);
        index
    }

    pub fn remove_joint(&mut self, index: usize) {
        if index < self.joints.len() {
            self.joints.remove(index);
        }
    }

    pub fn get_joint(&self, index: usize) -> Option<&Joint2D> {
        self.joints.get(index)
    }

    pub fn step(&mut self, dt: f32) {
        self.accumulator += dt;

        let max_time = self.config.timestep * self.config.max_substeps as f32;
        if self.accumulator > max_time {
            self.accumulator = max_time;
        }

        while self.accumulator >= self.config.timestep {
            let step_dt = self.config.timestep;

            if self.simulation_enabled {
                self.apply_gravity();
                self.update_velocities(step_dt);

                if self.collision_detection_enabled {
                    self.broad_phase();
                    self.narrow_phase();
                }

                self.resolve_collisions();
                self.update_positions(step_dt);
                self.solve_joints();
                self.correct_positions();
            }

            self.accumulator -= step_dt;
            self.simulation_time += step_dt;
        }
    }

    fn apply_gravity(&mut self) {
        let gravity = self.config.gravity;
        for body in &mut self.bodies {
            if body.body_type() == RigidBodyType::Dynamic {
                body.apply_force(gravity * body.mass());
            }
        }
    }

    fn update_velocities(&mut self, dt: f32) {
        for body in &mut self.bodies {
            if body.body_type() == RigidBodyType::Dynamic {
                body.update_velocity(dt);
            }
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for body in &mut self.bodies {
            if body.body_type() == RigidBodyType::Dynamic {
                body.update_position(dt);
            }
        }
    }

    fn broad_phase(&mut self) {
        self.collision_pairs.clear();

        let n = self.bodies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if self.bodies[i].is_static() && self.bodies[j].is_static() {
                    continue;
                }

                if self.check_aabb_overlap(i, j) {
                    self.collision_pairs.push((i, j));
                }
            }
        }
    }

    fn check_aabb_overlap(&self, body_a: usize, body_b: usize) -> bool {
        let body_a = &self.bodies[body_a];
        let body_b = &self.bodies[body_b];

        let aabb_a = self.compute_body_aabb(body_a);
        let aabb_b = self.compute_body_aabb(body_b);

        aabb_a.intersects(&aabb_b)
    }

    fn compute_body_aabb(&self, body: &RigidBody2D) -> Rect {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for &collider_idx in body.collider_indices() {
            if let Some(collider) = self.colliders.get(collider_idx) {
                let world_pos = collider.world_position(body.position(), body.rotation());
                let world_rot = collider.world_rotation(body.rotation());
                let aabb = collider.shape().compute_aabb(world_pos, world_rot);
                
                min_x = min_x.min(aabb.x);
                min_y = min_y.min(aabb.y);
                max_x = max_x.max(aabb.x + aabb.w);
                max_y = max_y.max(aabb.y + aabb.h);
            }
        }

        if min_x > max_x {
            let pos = body.position();
            Rect::new(pos.x, pos.y, 0.0, 0.0)
        } else {
            Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
        }
    }

    fn narrow_phase(&mut self) {
        let previous_manifolds = std::mem::take(&mut self.manifolds);
        self.collision_events.clear();

        for &(body_a_idx, body_b_idx) in &self.collision_pairs {
            let body_a = &self.bodies[body_a_idx];
            let body_b = &self.bodies[body_b_idx];

            for &collider_a_idx in body_a.collider_indices() {
                for &collider_b_idx in body_b.collider_indices() {
                    let collider_a = &self.colliders[collider_a_idx];
                    let collider_b = &self.colliders[collider_b_idx];

                    if let Some(manifold) = self.generate_contact(
                        collider_a, 
                        body_a, 
                        collider_b, 
                        body_b
                    ) {
                        let key = (collider_a_idx.min(collider_b_idx), collider_a_idx.max(collider_b_idx));
                        
                        let prev_manifold = previous_manifolds.get(&key);
                        if prev_manifold.is_none() {
                            self.collision_events.push_back(CollisionEvent::Started {
                                body_a: body_a_idx,
                                body_b: body_b_idx,
                                manifold: manifold.clone(),
                            });
                        }

                        self.manifolds.insert(key, manifold);
                    }
                }
            }
        }

        for (key, _) in previous_manifolds {
            if !self.manifolds.contains_key(&key) {
                let (a, b) = key;
                let body_a = self.find_body_for_collider(a);
                let body_b = self.find_body_for_collider(b);
                
                if let (Some(body_a), Some(body_b)) = (body_a, body_b) {
                    self.collision_events.push_back(CollisionEvent::Ended {
                        body_a,
                        body_b,
                    });
                }
            }
        }
    }

    fn find_body_for_collider(&self, collider_idx: usize) -> Option<usize> {
        self.bodies.iter().enumerate().find(|(_, body)| {
            body.collider_indices().contains(&collider_idx)
        }).map(|(i, _)| i)
    }

    fn generate_contact(
        &self,
        collider_a: &Collider2D,
        body_a: &RigidBody2D,
        collider_b: &Collider2D,
        body_b: &RigidBody2D,
    ) -> Option<Manifold> {
        let pos_a = collider_a.world_position(body_a.position(), body_a.rotation());
        let pos_b = collider_b.world_position(body_b.position(), body_b.rotation());
        let rot_a = collider_a.world_rotation(body_a.rotation());
        let rot_b = collider_b.world_rotation(body_b.rotation());

        match (collider_a.shape(), collider_b.shape()) {
            (crate::ColliderShape::Circle { radius: r1 }, crate::ColliderShape::Circle { radius: r2 }) => {
                self.circle_circle_collision(pos_a, *r1, pos_b, *r2)
            }
            (crate::ColliderShape::Circle { radius }, crate::ColliderShape::Rectangle { half_extents }) => {
                self.circle_rectangle_collision(pos_a, *radius, pos_b, half_extents, rot_b)
            }
            (crate::ColliderShape::Rectangle { half_extents }, crate::ColliderShape::Circle { radius }) => {
                let manifold = self.circle_rectangle_collision(pos_b, *radius, pos_a, half_extents, rot_a);
                manifold.map(|mut m| {
                    m.normal = -m.normal;
                    m
                })
            }
            (crate::ColliderShape::Aabb { half_extents: h1 }, crate::ColliderShape::Aabb { half_extents: h2 }) => {
                self.aabb_aabb_collision(pos_a, h1, pos_b, h2)
            }
            (crate::ColliderShape::Rectangle { half_extents: h1 }, crate::ColliderShape::Rectangle { half_extents: h2 }) => {
                self.rectangle_rectangle_collision(pos_a, h1, rot_a, pos_b, h2, rot_b)
            }
            _ => None,
        }
    }

    fn circle_circle_collision(&self, pos_a: Vec2, r1: f32, pos_b: Vec2, r2: f32) -> Option<Manifold> {
        let delta = pos_b - pos_a;
        let distance_squared = delta.length_squared();
        let radius_sum = r1 + r2;

        if distance_squared > radius_sum * radius_sum {
            return None;
        }

        let distance = distance_squared.sqrt();
        let normal = if distance > 0.0 { delta / distance } else { Vec2::new(1.0, 0.0) };
        let penetration = radius_sum - distance;

        let contact = Contact {
            position: pos_a + normal * r1,
            normal,
            penetration,
            restitution: self.config.default_restitution,
            friction: self.config.default_friction,
        };

        Some(Manifold {
            contacts: vec![contact],
            body_a: 0,
            body_b: 0,
        })
    }

    fn circle_rectangle_collision(
        &self,
        circle_pos: Vec2,
        circle_radius: f32,
        rect_pos: Vec2,
        rect_half_extents: Vec2,
        rect_rotation: f32,
    ) -> Option<Manifold> {
        let cos = rect_rotation.cos();
        let sin = rect_rotation.sin();

        let local_circle = Vec2::new(
            (circle_pos.x - rect_pos.x) * cos + (circle_pos.y - rect_pos.y) * sin,
            -(circle_pos.x - rect_pos.x) * sin + (circle_pos.y - rect_pos.y) * cos,
        );

        let clamped_x = local_circle.x.clamp(-rect_half_extents.x, rect_half_extents.x);
        let clamped_y = local_circle.y.clamp(-rect_half_extents.y, rect_half_extents.y);

        let closest_point_local = Vec2::new(clamped_x, clamped_y);
        let closest_point = Vec2::new(
            closest_point_local.x * cos - closest_point_local.y * sin + rect_pos.x,
            closest_point_local.x * sin + closest_point_local.y * cos + rect_pos.y,
        );

        let delta = circle_pos - closest_point;
        let distance_squared = delta.length_squared();

        if distance_squared > circle_radius * circle_radius {
            return None;
        }

        let distance = distance_squared.sqrt();
        let normal = if distance > 0.0 { delta / distance } else { Vec2::new(1.0, 0.0) };
        let penetration = circle_radius - distance;

        let contact = Contact {
            position: closest_point,
            normal,
            penetration,
            restitution: self.config.default_restitution,
            friction: self.config.default_friction,
        };

        Some(Manifold {
            contacts: vec![contact],
            body_a: 0,
            body_b: 0,
        })
    }

    fn aabb_aabb_collision(&self, pos_a: Vec2, h1: &Vec2, pos_b: Vec2, h2: &Vec2) -> Option<Manifold> {
        let dx = (pos_a.x + h1.x) - (pos_b.x + h2.x);
        let px = (h1.x + h2.x) - dx.abs();
        
        let dy = (pos_a.y + h1.y) - (pos_b.y + h2.y);
        let py = (h1.y + h2.y) - dy.abs();

        if px < 0.0 || py < 0.0 {
            return None;
        }

        let (normal, penetration) = if px < py {
            (Vec2::new(-dx.signum(), 0.0), px)
        } else {
            (Vec2::new(0.0, -dy.signum()), py)
        };

        let contact = Contact {
            position: pos_a + normal * penetration,
            normal,
            penetration,
            restitution: self.config.default_restitution,
            friction: self.config.default_friction,
        };

        Some(Manifold {
            contacts: vec![contact],
            body_a: 0,
            body_b: 0,
        })
    }

    fn rectangle_rectangle_collision(
        &self,
        pos_a: Vec2,
        h1: &Vec2,
        rot_a: f32,
        pos_b: Vec2,
        h2: &Vec2,
        rot_b: f32,
    ) -> Option<Manifold> {
        self.gjk_collision(pos_a, h1, rot_a, pos_b, h2, rot_b)
    }

    fn gjk_collision(
        &self,
        pos_a: Vec2,
        h1: &Vec2,
        rot_a: f32,
        pos_b: Vec2,
        h2: &Vec2,
        rot_b: f32,
    ) -> Option<Manifold> {
        let support_a = |d: Vec2| self.support_rectangle(d, pos_a, h1, rot_a);
        let support_b = |d: Vec2| self.support_rectangle(d, pos_b, h2, rot_b);

        let mut direction = pos_b - pos_a;
        if direction.length_squared() < 0.0001 {
            direction = Vec2::new(1.0, 0.0);
        }

        let mut simplex = Vec::new();
        simplex.push(support_a(direction) - support_b(direction));
        direction = -direction;

        loop {
            let a = support_a(direction) - support_b(direction);
            
            if a.dot(direction) < 0.0 {
                return None;
            }

            simplex.push(a);

            if self.do_simplex(&mut simplex, &mut direction) {
                let manifold = self.epa(&simplex, support_a, support_b);
                return manifold;
            }
        }
    }

    fn support_rectangle(&self, d: Vec2, pos: Vec2, h: &Vec2, rot: f32) -> Vec2 {
        let cos = rot.cos();
        let sin = rot.sin();

        let axes = [
            Vec2::new(cos, sin),
            Vec2::new(-sin, cos),
        ];

        let mut max_proj = f32::MIN;
        let mut max_point = Vec2::ZERO;

        for i in 0..4 {
            let x = if i & 1 != 0 { h.x } else { -h.x };
            let y = if i & 2 != 0 { h.y } else { -h.y };
            let vertex = Vec2::new(
                x * cos - y * sin + pos.x,
                x * sin + y * cos + pos.y,
            );
            let proj = vertex.dot(d);
            if proj > max_proj {
                max_proj = proj;
                max_point = vertex;
            }
        }

        max_point
    }

    fn do_simplex(&self, simplex: &mut Vec<Vec2>, direction: &mut Vec2) -> bool {
        let a = *simplex.last().unwrap();

        if simplex.len() == 2 {
            let b = simplex[0];
            let ab = b - a;
            let ao = -a;

            if self.triple_product(ab, ao, ab).dot(ao) >= 0.0 {
                *direction = self.triple_product(ab, ao, ab);
            } else {
                *direction = ao;
                *simplex = vec![a];
            }
        } else {
            let b = simplex[1];
            let c = simplex[0];
            let ab = b - a;
            let ac = c - a;
            let ao = -a;

            let ab_perp = self.triple_product(ac, ab, ab);
            let ac_perp = self.triple_product(ab, ac, ac);

            if ab_perp.dot(ao) > 0.0 {
                *simplex = vec![a, b];
                *direction = ab_perp;
            } else if ac_perp.dot(ao) > 0.0 {
                *simplex = vec![a, c];
                *direction = ac_perp;
            } else {
                return true;
            }
        }

        false
    }

    fn triple_product(&self, a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
        Vec2::new(
            a.y * b.dot(c) - b.y * a.dot(c),
            b.x * a.dot(c) - a.x * b.dot(c),
        )
    }

    fn epa(
        &self,
        simplex: &[Vec2],
        support_a: impl Fn(Vec2) -> Vec2,
        support_b: impl Fn(Vec2) -> Vec2,
    ) -> Option<Manifold> {
        let mut poly = simplex.to_vec();
        
        for _ in 0..20 {
            let (closest_edge, min_dist, normal) = self.find_closest_edge(&poly);
            
            if min_dist < 0.0001 {
                let contact = Contact {
                    position: Vec2::ZERO,
                    normal,
                    penetration: min_dist,
                    restitution: self.config.default_restitution,
                    friction: self.config.default_friction,
                };

                return Some(Manifold {
                    contacts: vec![contact],
                    body_a: 0,
                    body_b: 0,
                });
            }

            let support = support_a(normal) - support_b(normal);
            
            if support.dot(normal) - min_dist < 0.0001 {
                let contact = Contact {
                    position: Vec2::ZERO,
                    normal,
                    penetration: min_dist,
                    restitution: self.config.default_restitution,
                    friction: self.config.default_friction,
                };

                return Some(Manifold {
                    contacts: vec![contact],
                    body_a: 0,
                    body_b: 0,
                });
            }

            poly.insert(closest_edge, support);
        }

        None
    }

    fn find_closest_edge(&self, poly: &[Vec2]) -> (usize, f32, Vec2) {
        let mut min_dist = f32::MAX;
        let mut closest_edge = 0;
        let mut normal = Vec2::ZERO;

        for i in 0..poly.len() {
            let j = (i + 1) % poly.len();
            let a = poly[i];
            let b = poly[j];
            let ab = b - a;
            
            let t = (-a).dot(ab) / ab.length_squared();
            let t_clamped = t.clamp(0.0, 1.0);
            let closest = a + ab * t_clamped;
            let dist = closest.length();

            let edge_normal = Vec2::new(-ab.y, ab.x).normalize();
            if closest.dot(edge_normal) < 0.0 {
                edge_normal * -1.0;
            }

            if dist < min_dist {
                min_dist = dist;
                closest_edge = j;
                normal = edge_normal;
            }
        }

        (closest_edge, min_dist, normal)
    }

    fn resolve_collisions(&mut self) {
        for ((collider_a_idx, collider_b_idx), manifold) in &mut self.manifolds {
            let body_a_idx = self.find_body_for_collider(*collider_a_idx);
            let body_b_idx = self.find_body_for_collider(*collider_b_idx);

            if let (Some(body_a_idx), Some(body_b_idx)) = (body_a_idx, body_b_idx) {
                let body_a = &mut self.bodies[body_a_idx];
                let body_b = &mut self.bodies[body_b_idx];

                if body_a.is_static() && body_b.is_static() {
                    continue;
                }

                for contact in &manifold.contacts {
                    self.resolve_single_contact(body_a, body_b, contact);
                }
            }
        }
    }

    fn resolve_single_contact(&mut self, body_a: &mut RigidBody2D, body_b: &mut RigidBody2D, contact: &Contact) {
        let relative_vel = body_b.linear_velocity() - body_a.linear_velocity();
        let vel_along_normal = relative_vel.dot(contact.normal);

        if vel_along_normal > 0.0 {
            return;
        }

        let restitution = contact.restitution.min(body_a.restitution()).min(body_b.restitution());

        let inv_mass_a = if body_a.is_static() { 0.0 } else { 1.0 / body_a.mass() };
        let inv_mass_b = if body_b.is_static() { 0.0 } else { 1.0 / body_b.mass() };

        let impulse_scalar = -(1.0 + restitution) * vel_along_normal / (inv_mass_a + inv_mass_b);

        let impulse = contact.normal * impulse_scalar;

        if !body_a.is_static() {
            body_a.apply_impulse(-impulse);
        }
        if !body_b.is_static() {
            body_b.apply_impulse(impulse);
        }

        let tangent = (relative_vel - contact.normal * vel_along_normal).normalize_or_zero();
        let friction = contact.friction.min(body_a.friction()).min(body_b.friction());
        let tangent_impulse = tangent * impulse_scalar * friction;

        if !body_a.is_static() {
            body_a.apply_impulse(-tangent_impulse);
        }
        if !body_b.is_static() {
            body_b.apply_impulse(tangent_impulse);
        }
    }

    fn solve_joints(&mut self) {
        for _ in 0..self.config.velocity_iterations {
            for joint in &self.joints {
                self.apply_joint_constraint(joint);
            }
        }
    }

    fn apply_joint_constraint(&self, _joint: &Joint2D) {
    }

    fn correct_positions(&mut self) {
        let slop = 0.005;
        let baumgarte = 0.2;

        for ((collider_a_idx, collider_b_idx), manifold) in &mut self.manifolds {
            let body_a_idx = self.find_body_for_collider(*collider_a_idx);
            let body_b_idx = self.find_body_for_collider(*collider_b_idx);

            if let (Some(body_a_idx), Some(body_b_idx)) = (body_a_idx, body_b_idx) {
                let body_a = &mut self.bodies[body_a_idx];
                let body_b = &mut self.bodies[body_b_idx];

                let inv_mass_a = if body_a.is_static() { 0.0 } else { 1.0 / body_a.mass() };
                let inv_mass_b = if body_b.is_static() { 0.0 } else { 1.0 / body_b.mass() };

                for contact in &manifold.contacts {
                    let correction = contact.normal * contact.penetration.clamp(0.0, slop) * baumgarte;

                    if !body_a.is_static() {
                        body_a.set_position(body_a.position() - correction * inv_mass_a);
                    }
                    if !body_b.is_static() {
                        body_b.set_position(body_b.position() + correction * inv_mass_b);
                    }
                }
            }
        }
    }

    pub fn collision_events(&self) -> &VecDeque<CollisionEvent> {
        &self.collision_events
    }

    pub fn clear_collision_events(&mut self) {
        self.collision_events.clear();
    }

    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn collider_count(&self) -> usize {
        self.colliders.len()
    }

    pub fn joint_count(&self) -> usize {
        self.joints.len()
    }

    pub fn simulation_time(&self) -> f32 {
        self.simulation_time
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.config.gravity = gravity;
    }

    pub fn gravity(&self) -> Vec2 {
        self.config.gravity
    }

    pub fn set_collision_detection(&mut self, enabled: bool) {
        self.collision_detection_enabled = enabled;
    }

    pub fn set_simulation(&mut self, enabled: bool) {
        self.simulation_enabled = enabled;
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ColliderShape;

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
        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        let body_index = world.add_body(body);

        let collider = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider, body_index);

        assert_eq!(world.body_count(), 1);
        assert_eq!(world.collider_count(), 1);

        world.remove_body(body_index);

        assert_eq!(world.body_count(), 0);
        assert_eq!(world.collider_count(), 0);
    }

    #[test]
    fn test_remove_collider_updates_body_indices() {
        let mut world = PhysicsWorld2D::with_default_config();
        let body = RigidBody2D::new(RigidBodyType::Dynamic);
        let body_index = world.add_body(body);

        let collider1 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        let collider2 = Collider2D::new(ColliderShape::Circle { radius: 2.0 });
        let collider1_index = world.add_collider(collider1, body_index);
        world.add_collider(collider2, body_index);

        assert_eq!(collider1_index, 0);

        world.remove_collider(collider1_index);

        let body = world.get_body(body_index);
        assert!(body.is_some());
        assert_eq!(body.unwrap().collider_indices(), &[0]);
    }

    #[test]
    fn test_circle_circle_collision() {
        let mut world = PhysicsWorld2D::with_default_config();

        let mut body1 = RigidBody2D::new(RigidBodyType::Dynamic);
        body1.set_position(Vec2::new(0.0, 0.0));
        let body1_idx = world.add_body(body1);

        let mut body2 = RigidBody2D::new(RigidBodyType::Dynamic);
        body2.set_position(Vec2::new(1.5, 0.0));
        let body2_idx = world.add_body(body2);

        let collider1 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider1, body1_idx);

        let collider2 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider2, body2_idx);

        world.step(1.0 / 60.0);

        assert!(world.manifolds.len() > 0);
    }

    #[test]
    fn test_circle_circle_no_collision() {
        let mut world = PhysicsWorld2D::with_default_config();

        let mut body1 = RigidBody2D::new(RigidBodyType::Dynamic);
        body1.set_position(Vec2::new(0.0, 0.0));
        let body1_idx = world.add_body(body1);

        let mut body2 = RigidBody2D::new(RigidBodyType::Dynamic);
        body2.set_position(Vec2::new(5.0, 0.0));
        let body2_idx = world.add_body(body2);

        let collider1 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider1, body1_idx);

        let collider2 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider2, body2_idx);

        world.step(1.0 / 60.0);

        assert!(world.manifolds.is_empty());
    }

    #[test]
    fn test_aabb_aabb_collision() {
        let mut world = PhysicsWorld2D::with_default_config();

        let mut body1 = RigidBody2D::new(RigidBodyType::Dynamic);
        body1.set_position(Vec2::new(0.0, 0.0));
        let body1_idx = world.add_body(body1);

        let mut body2 = RigidBody2D::new(RigidBodyType::Dynamic);
        body2.set_position(Vec2::new(1.0, 0.0));
        let body2_idx = world.add_body(body2);

        let collider1 = Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) });
        world.add_collider(collider1, body1_idx);

        let collider2 = Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) });
        world.add_collider(collider2, body2_idx);

        world.step(1.0 / 60.0);

        assert!(world.manifolds.len() > 0);
    }

    #[test]
    fn test_circle_aabb_collision() {
        let mut world = PhysicsWorld2D::with_default_config();

        let mut body1 = RigidBody2D::new(RigidBodyType::Dynamic);
        body1.set_position(Vec2::new(0.0, 0.0));
        let body1_idx = world.add_body(body1);

        let mut body2 = RigidBody2D::new(RigidBodyType::Dynamic);
        body2.set_position(Vec2::new(1.5, 0.0));
        let body2_idx = world.add_body(body2);

        let collider1 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider1, body1_idx);

        let collider2 = Collider2D::new(ColliderShape::Aabb { half_extents: Vec2::new(1.0, 1.0) });
        world.add_collider(collider2, body2_idx);

        world.step(1.0 / 60.0);

        assert!(world.manifolds.len() > 0);
    }

    #[test]
    fn test_collision_response() {
        let mut world = PhysicsWorld2D::with_default_config();

        let mut body1 = RigidBody2D::new(RigidBodyType::Dynamic);
        body1.set_position(Vec2::new(0.0, 0.0));
        body1.set_linear_velocity(Vec2::new(1.0, 0.0));
        let body1_idx = world.add_body(body1);

        let mut body2 = RigidBody2D::new(RigidBodyType::Dynamic);
        body2.set_position(Vec2::new(1.5, 0.0));
        let body2_idx = world.add_body(body2);

        let collider1 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider1, body1_idx);

        let collider2 = Collider2D::new(ColliderShape::Circle { radius: 1.0 });
        world.add_collider(collider2, body2_idx);

        world.step(1.0 / 60.0);

        let body1_after = world.get_body(body1_idx).unwrap();
        let body2_after = world.get_body(body2_idx).unwrap();

        assert!(body1_after.linear_velocity().x < 1.0);
        assert!(body2_after.linear_velocity().x > 0.0);
    }
}