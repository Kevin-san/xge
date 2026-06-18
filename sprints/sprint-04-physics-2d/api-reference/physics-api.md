# 物理 API 参考（Physics API Reference）

## 概述

本文档汇总 Sprint 04 中 `engine-physics-2d` crate 的所有公开 API，包含 World2D、RigidBody2D、Collider2D、Joint2D 及相关类型的完整签名。公开 API 数量目标 ≤ 120。

---

## 1. World2D 物理世界

```rust
// 构造与配置
pub fn new(gravity: Vec2) -> Self
pub fn new_default() -> Self  // gravity = (0, -9.81)

pub fn set_gravity(&mut self, v: Vec2)
pub fn gravity(&self) -> Vec2

// 仿真控制
pub fn step(&mut self, dt: f32)
pub fn step_with_iterations(&mut self, dt: f32, velocity_iter: u32, position_iter: u32)
pub fn set_paused(&mut self, paused: bool)

// 刚体管理
pub fn insert_body(&mut self, body: RigidBody2D) -> BodyHandle
pub fn remove_body(&mut self, handle: BodyHandle)
pub fn get_body(&self, handle: BodyHandle) -> &RigidBody2D
pub fn get_body_mut(&mut self, handle: BodyHandle) -> &mut RigidBody2D
pub fn bodies(&self) -> impl Iterator<Item = &RigidBody2D>

// 碰撞体管理
pub fn insert_collider(&mut self, collider: Collider2D, body_handle: BodyHandle) -> ColliderHandle
pub fn remove_collider(&mut self, handle: ColliderHandle)
pub fn colliders(&self) -> impl Iterator<Item = &Collider2D>

// 关节管理
pub fn insert_joint(&mut self, joint: Joint2D) -> JointHandle
pub fn remove_joint(&mut self, handle: JointHandle)
pub fn joints(&self) -> impl Iterator<Item = &Joint2D>

// 接触信息
pub fn contacts(&self) -> Vec<Contact2D>
pub fn events(&self) -> impl Iterator<Item = &ContactEvent>
pub fn contact_manifolds(&self) -> impl Iterator<Item = &ContactManifold>

// 空间查询
pub fn ray_cast(&self, origin: Vec2, dir: Vec2, max_toi: f32, filter: QueryFilter) -> Option<RayCastHit2D>
pub fn shape_cast(&self, shape: &dyn Shape, origin: Vec2, dir: Vec2, max_toi: f32) -> Option<ShapeCastHit2D>
pub fn point_overlap(&self, point: Vec2, filter: QueryFilter) -> Vec<BodyHandle>
pub fn aabb_overlap(&self, aabb: AABB, filter: QueryFilter) -> Vec<BodyHandle>

// 清理
pub fn clear(&mut self)
```

---

## 2. RigidBody2D 刚体

```rust
// 类型
pub enum BodyType { Dynamic, Static, Kinematic, Sensor }

// 变换
pub fn translation(&self) -> Vec2
pub fn set_translation(&mut self, v: Vec2)
pub fn rotation(&self) -> f32
pub fn set_rotation(&mut self, rad: f32)
pub fn linvel(&self) -> Vec2
pub fn set_linvel(&mut self, v: Vec2)
pub fn angvel(&self) -> f32
pub fn set_angvel(&mut self, v: f32)

// 力与冲量
pub fn apply_force(&mut self, force: Vec2, point: Vec2)
pub fn apply_force_at_center(&mut self, force: Vec2)
pub fn apply_torque(&mut self, torque: f32)
pub fn apply_impulse(&mut self, impulse: Vec2, point: Vec2)
pub fn apply_impulse_at_center(&mut self, impulse: Vec2)

// 质量
pub fn mass(&self) -> f32
pub fn inertia(&self) -> f32
pub fn set_mass(&mut self, mass: f32)
pub fn local_center_of_mass(&self) -> Vec2
pub fn mass_properties(&self) -> MassProperties2D

// 物理参数
pub fn gravity_scale(&self) -> f32
pub fn set_gravity_scale(&mut self, scale: f32)
pub fn linear_damping(&self) -> f32
pub fn set_linear_damping(&mut self, v: f32)
pub fn angular_damping(&self) -> f32
pub fn set_angular_damping(&mut self, v: f32)

// 休眠
pub fn can_sleep(&self) -> bool
pub fn set_can_sleep(&mut self, can_sleep: bool)
pub fn sleeping(&self) -> bool
pub fn wake_up(&mut self)

// 类型查询
pub fn type_(&self) -> BodyType
pub fn is_dynamic(&self) -> bool
pub fn is_static(&self) -> bool
pub fn is_kinematic(&self) -> bool
pub fn ccd_enabled(&self) -> bool
pub fn handle(&self) -> BodyHandle
```

---

## 3. RigidBody2DBuilder 刚体构造器

```rust
pub fn new(body_type: BodyType) -> Self
pub fn dynamic() -> Self
pub fn static_() -> Self
pub fn kinematic() -> Self
pub fn sensor() -> Self

pub fn translation(mut self, v: Vec2) -> Self
pub fn rotation(mut self, rad: f32) -> Self
pub fn linvel(mut self, v: Vec2) -> Self
pub fn angvel(mut self, v: f32) -> Self
pub fn gravity_scale(mut self, f: f32) -> Self
pub fn linear_damping(mut self, f: f32) -> Self
pub fn angular_damping(mut self, f: f32) -> Self
pub fn can_sleep(mut self, bool: bool) -> Self
pub fn ccd_enabled(mut self, bool: bool) -> Self

pub fn build(self) -> RigidBody2D
```

---

## 4. Collider2D 碰撞体

```rust
pub enum ColliderShape { Circle, Rect, Polygon, Capsule, Triangle }

pub fn aabb(&self) -> AABB
pub fn mass_properties(&self, density: f32) -> MassProperties2D
pub fn handle(&self) -> ColliderHandle
pub fn body(&self) -> Option<BodyHandle>
pub fn is_sensor(&self) -> bool
pub fn material(&self) -> &PhysicsMaterial
pub fn collision_groups(&self) -> CollisionGroup
pub fn solver_groups(&self) -> CollisionGroup
```

---

## 5. ColliderBuilder 碰撞体构造器

```rust
pub fn circle(radius: f32) -> Self
pub fn rect(w: f32, h: f32) -> Self
pub fn polygon(points: Vec<Vec2>) -> Self  // 逆时针
pub fn capsule(half_h: f32, radius: f32) -> Self
pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> Self

pub fn translation(mut self, v: Vec2) -> Self
pub fn rotation(mut self, rad: f32) -> Self
pub fn sensor(mut self, bool: bool) -> Self
pub fn material(mut self, m: PhysicsMaterial) -> Self
pub fn density(mut self, d: f32) -> Self
pub fn friction(mut self, f: f32) -> Self
pub fn restitution(mut self, r: f32) -> Self
pub fn collision_group(mut self, g: CollisionGroup) -> Self
pub fn solver_groups(mut self, g: CollisionGroup) -> Self

pub fn build(self) -> Collider2D
```

---

## 6. PhysicsMaterial 物理材质

```rust
pub struct PhysicsMaterial {
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
}

pub fn default() -> Self
```

---

## 7. CollisionGroup 碰撞分组

```rust
pub struct CollisionGroup { /* ... */ }

pub fn new(memberships: u32, filters: u32) -> Self
pub fn with_all() -> Self
pub fn with_none() -> Self
pub fn memberships(&self) -> u32
pub fn filters(&self) -> u32
pub fn can_interact_with(a: CollisionGroup, b: CollisionGroup) -> bool
```

---

## 8. Contact2D / ContactEvent / ContactManifold

```rust
pub struct Contact2D {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub point: Vec2,
    pub normal: Vec2,
    pub penetration: f32,
}

pub enum ContactEvent { Started(Contact2D), Ended(Contact2D) }

pub struct ContactManifold {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub points: Vec<Contact2D>,
}
```

---

## 9. QueryFilter / RayCastHit2D / ShapeCastHit2D

```rust
pub struct QueryFilter {
    pub skip_bodies: Vec<BodyHandle>,
    pub include_sensors: bool,
}

pub struct RayCastHit2D {
    pub handle: BodyHandle,
    pub point: Vec2,
    pub normal: Vec2,
    pub toi: f32,
}

pub struct ShapeCastHit2D {
    pub handle: BodyHandle,
    pub point: Vec2,
    pub normal: Vec2,
    pub toi: f32,
}
```

---

## 10. Joint2D Trait & Implementations

```rust
pub trait Joint2D {
    fn body_a(&self) -> BodyHandle;
    fn body_b(&self) -> BodyHandle;
}

// DistanceJoint
pub struct DistanceJoint { /* ... */ }
pub fn new(body_a: BodyHandle, body_b: BodyHandle, local_a: Vec2, local_b: Vec2) -> Self
pub fn length(mut self, f: f32) -> Self
pub fn stiffness(mut self, f: f32) -> Self
pub fn damping(mut self, f: f32) -> Self
pub fn build(self) -> DistanceJoint

// RevoluteJoint
pub struct RevoluteJoint { /* ... */ }
pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor: Vec2) -> Self
pub fn limits(mut self, min: f32, max: f32) -> Self
pub fn motor(mut self, velocity: f32, max_torque: f32) -> Self
pub fn build(self) -> RevoluteJoint

// PrismaticJoint
pub struct PrismaticJoint { /* ... */ }
pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor: Vec2, axis: Vec2) -> Self
pub fn limits(mut self, min: f32, max: f32) -> Self
pub fn motor(mut self, velocity: f32, max_force: f32) -> Self
pub fn build(self) -> PrismaticJoint

// WeldJoint
pub struct WeldJoint { /* ... */ }
pub fn new(body_a: BodyHandle, body_b: BodyHandle, local_a: Vec2, local_b: Vec2) -> Self
pub fn build(self) -> WeldJoint

// SpringJoint
pub struct SpringJoint { /* ... */ }
pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor_a: Vec2, anchor_b: Vec2) -> Self
pub fn stiffness(mut self, f: f32) -> Self
pub fn damping(mut self, f: f32) -> Self
pub fn rest_length(mut self, f: f32) -> Self
pub fn build(self) -> SpringJoint

// MotorJoint
pub struct MotorJoint { /* ... */ }
pub fn new(body_a: BodyHandle, body_b: BodyHandle, offset: Vec2) -> Self
pub fn motor_velocity(mut self, v: Vec2) -> Self
pub fn motor_max_force(mut self, f: f32) -> Self
pub fn build(self) -> MotorJoint
```

---

## 11. Shape Trait

```rust
pub trait Shape {
    fn aabb(&self, transform: &Mat3) -> AABB;
}

impl Shape for Circle { /* ... */ }
impl Shape for Rect { /* ... */ }
impl Shape for Polygon { /* ... */ }
impl Shape for Capsule { /* ... */ }
```

---

## 12. PhysicsDebugRenderer

```rust
pub trait PhysicsDebugRenderer {
    fn draw_circle(&mut self, pos: Vec2, radius: f32, color: Color);
    fn draw_rect(&mut self, min: Vec2, max: Vec2, color: Color);
    fn draw_polygon(&mut self, points: &[Vec2], color: Color);
    fn draw_segment(&mut self, a: Vec2, b: Vec2, color: Color);
}
```

---

## 13. PhysicsModule

```rust
pub struct PhysicsModule { /* ... */ }

impl PhysicsModule {
    pub fn new(world: World2D) -> Self;
    pub fn update(&mut self, scene: &mut SceneTree, dt: f32);
}
```

---

## API 统计

| 类别 | 数量 |
|------|------|
| World2D | 22 |
| RigidBody2D | 27 |
| RigidBody2DBuilder | 15 |
| Collider2D | 8 |
| ColliderBuilder | 14 |
| PhysicsMaterial | 1 |
| CollisionGroup | 7 |
| Contact/Event/Manifold | 3 |
| QueryFilter/RayCastHit/ShapeCastHit | 3 |
| Joint2D + 6 implementations | ~40 |
| Shape trait | ~5 |
| PhysicsDebugRenderer | 4 |
| PhysicsModule | 2 |
| **总计** | **≤ 120** |
