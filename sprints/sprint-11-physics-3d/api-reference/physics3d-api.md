# 3D 物理 API 清单

## 概述

本文档列出 `engine-physics-3d` crate 中所有公开的 API，按模块组织。每个 API 包含签名、说明和对应的需求编号。

## 需求编号对照表

| 编号范围 | 模块 |
|----------|------|
| 1-24 | 后端抽象与 crate 建立 |
| 25-115 | RigidBody (基础) |
| 116-159 | Collider 与 Query (基础) |
| 160-196 | Joint, Query, Event (基础) |
| 236-303 | PhysicsWorld3D (详细) |
| 268-358 | RigidBody3D (详细) |
| 319-388 | Collider3D (详细) |
| 350-413 | Joints3D (详细) |
| 371-436 | CharacterController3D (详细) |
| 391-466 | Query3D (详细) |
| 417-479 | 调试可视化 |
| 480-547 | ECS 集成、编辑器、测试 |

---

## 1. Crate 与后端抽象

### 1.1 Crate 建立

```rust
// 需求 1
pub mod engine_physics_3d {
    // engine-physics-3d crate 建立
}
```

### 1.2 PhysicsBackend Trait

```rust
// 需求 2
pub trait PhysicsBackend {
    fn new_world(gravity: Vec3) -> Self;
    fn step(&mut self, dt: f32);
    fn insert_body(&mut self, body: RigidBody3D) -> RigidBodyHandle;
    fn remove_body(&mut self, handle: RigidBodyHandle);
    fn insert_collider(&mut self, collider: Collider3D, parent: RigidBodyHandle) -> ColliderHandle;
    fn remove_collider(&mut self, handle: ColliderHandle);
    fn insert_joint(&mut self, body1: RigidBodyHandle, body2: RigidBodyHandle, joint: Joint3D) -> JointHandle;
    fn remove_joint(&mut self, handle: JointHandle);
    fn cast_ray(&self, ray: &Ray3, max_toi: f32, solid: bool) -> Option<RayCastHit>;
    fn query_aabb(&self, aabb: AABB) -> Vec<Entity>;
    fn point_test(&self, point: Vec3) -> Option<Entity>;
    fn debug_draw(&self, renderer: &mut dyn DebugRenderer);
}
```

### 1.3 后端实现

```rust
// 需求 3
pub struct RapierBackend { /* ... */ }
impl PhysicsBackend for RapierBackend { /* ... */ }

// 需求 4
pub struct NullBackend { /* ... */ }
impl PhysicsBackend for NullBackend { /* ... */ }
```

---

## 2. PhysicsWorld3D

### 2.1 构造函数与步进

```rust
// 需求 5
pub fn new(gravity: Vec3) -> Self;

// 需求 6
pub fn set_gravity(&mut self, g: Vec3);

// 需求 7
pub fn gravity(&self) -> Vec3;

// 需求 8
pub fn step(&mut self, dt: f32);

// 需求 9
pub fn step_with_substeps(&mut self, dt: f32, substeps: u32);

// 需求 10
pub fn paused(&self) -> bool;

// 需求 11
pub fn set_paused(&mut self, bool);

// 需求 12
pub fn num_bodies(&self) -> usize;

// 需求 13
pub fn num_colliders(&self) -> usize;

// 需求 14
pub fn num_joints(&self) -> usize;

// 需求 15
pub fn query_pipeline(&self) -> &QueryPipeline;

// 需求 16
pub fn ccd_enabled(&self) -> bool;

// 需求 17
pub fn set_ccd_enabled(&mut self, bool);

// 需求 18
pub fn gravity_scale(&self) -> f32;

// 需求 19
pub fn set_gravity_scale(&mut self, v: f32);

// 需求 20
pub fn collision_groups() -> CollisionGroups;

// 需求 21
pub fn max_velocity_iterations(&self) -> usize;

// 需求 22
pub fn set_max_velocity_iterations(&mut self, n: usize);

// 需求 23
pub fn max_position_iterations(&self) -> usize;

// 需求 24
pub fn set_max_position_iterations(&mut self, n: usize);
```

### 2.2 Body 管理

```rust
// 需求 51
pub fn insert_body(&mut self, body: RigidBody3D) -> RigidBodyHandle;

// 需求 52
pub fn remove_body(&mut self, handle: RigidBodyHandle);

// 需求 53
pub fn body(&self, handle: RigidBodyHandle) -> &RigidBody3D;

// 需求 54
pub fn body_mut(&mut self, handle: RigidBodyHandle) -> &mut RigidBody3D;
```

### 2.3 Collider 管理

```rust
// 需求 116
pub fn insert_collider(&mut self, collider: Collider3D, parent_body: RigidBodyHandle) -> ColliderHandle;

// 需求 117
pub fn remove_collider(&mut self, handle: ColliderHandle);

// 需求 118
pub fn collider(&self, handle: ColliderHandle) -> &Collider3D;

// 需求 119
pub fn collider_mut(&mut self, handle: ColliderHandle) -> &mut Collider3D;
```

### 2.4 Joint 管理

```rust
// 需求 140
pub fn insert_joint(&mut self, body1: RigidBodyHandle, body2: RigidBodyHandle, joint: Joint3D) -> JointHandle;

// 需求 141
pub fn remove_joint(&mut self, handle: JointHandle);
```

### 2.5 事件

```rust
// 需求 167
pub fn contact_events(&self) -> &[ContactEvent];

// 需求 168
pub fn intersection_events(&self) -> &[IntersectionEvent];

// 需求 169
pub fn contact_force_events(&self) -> &[ContactForceEvent];

// 需求 170
pub fn contact_pair(&self, a: ColliderHandle, b: ColliderHandle) -> Option<ContactPair>;
```

### 2.6 迭代器

```rust
// 需求 264
pub fn bodies_iter(&self) -> impl Iterator<Item = (RigidBodyHandle, &RigidBody3D)>;

// 需求 265
pub fn colliders_iter(&self) -> impl Iterator<Item = (ColliderHandle, &Collider3D)>;

// 需求 267
pub fn clear(&mut self);
```

---

## 3. RigidBody3D

### 3.1 RigidBodyType

```rust
// 需求 25, 292
pub enum RigidBodyType {
    Dynamic,
    Static,
    KinematicPositionBased,
    KinematicVelocityBased,
    Fixed,
}
```

### 3.2 RigidBodyHandle

```rust
// 需求 50
pub struct RigidBodyHandle(/* ... */);
```

### 3.3 RigidBodyBuilder

```rust
// 需求 26-30
impl RigidBodyBuilder {
    pub fn dynamic() -> Self;
    pub fn static_() -> Self;
    pub fn kinematic_position_based() -> Self;
    pub fn kinematic_velocity_based() -> Self;
    pub fn fixed() -> Self;
}

// 需求 31-48
impl RigidBodyBuilder {
    pub fn translation(v: Vec3) -> Self;
    pub fn rotation(q: Quat) -> Self;
    pub fn linvel(v: Vec3) -> Self;
    pub fn angvel(v: Vec3) -> Self;
    pub fn mass(v: f32) -> Self;
    pub fn mass_properties(mp: MassProperties) -> Self;
    pub fn center_of_mass(v: Vec3) -> Self;
    pub fn principal_inertia(v: Vec3) -> Self;
    pub fn linear_damping(v: f32) -> Self;
    pub fn angular_damping(v: f32) -> Self;
    pub fn gravity_scale(v: f32) -> Self;
    pub fn ccd_enabled(b: bool) -> Self;
    pub fn sleeping(b: bool) -> Self;
    pub fn dominance_group(i8) -> Self;
    pub fn additional_mass(v: f32) -> Self;
    pub fn lock_translations() -> Self;
    pub fn lock_rotations() -> Self;
    pub fn restrict_rotations(x: bool, y: bool, z: bool) -> Self;
}

// 需求 49
pub fn build(&self) -> RigidBody3D;
```

### 3.4 RigidBody3D 实例方法

```rust
// 类型查询 - 需求 55, 85-87
pub fn type(&self) -> RigidBodyType;
pub fn is_dynamic(&self) -> bool;
pub fn is_static(&self) -> bool;
pub fn is_kinematic(&self) -> bool;

// 变换 - 需求 56-61
pub fn position(&self) -> Vec3;
pub fn rotation(&self) -> Quat;
pub fn transform(&self) -> (Vec3, Quat);
pub fn set_translation(&mut self, v: Vec3, wake: bool);
pub fn set_rotation(&mut self, q: Quat, wake: bool);
pub fn set_position(&mut self, v: Vec3, q: Quat, wake: bool);

// 速度 - 需求 62-65
pub fn linvel(&self) -> Vec3;
pub fn angvel(&self) -> Vec3;
pub fn set_linvel(&mut self, v: Vec3, wake: bool);
pub fn set_angvel(&mut self, v: Vec3, wake: bool);

// 质量 - 需求 66-67
pub fn mass(&self) -> f32;
pub fn set_mass(&mut self, mass: f32);

// 力与冲量 - 需求 68-73
pub fn apply_force(&mut self, force: Vec3, wake: bool);
pub fn apply_force_at_point(&mut self, force: Vec3, point: Vec3, wake: bool);
pub fn apply_torque(&mut self, torque: Vec3, wake: bool);
pub fn apply_impulse(&mut self, impulse: Vec3, wake: bool);
pub fn apply_impulse_at_point(&mut self, impulse: Vec3, point: Vec3, wake: bool);
pub fn apply_torque_impulse(&mut self, torque_impulse: Vec3, wake: bool);

// 阻尼 - 需求 74-77
pub fn linear_damping(&self) -> f32;
pub fn set_linear_damping(&mut self, v: f32);
pub fn angular_damping(&self) -> f32;
pub fn set_angular_damping(&mut self, v: f32);

// 重力 - 需求 78-79
pub fn gravity_scale(&self) -> f32;
pub fn set_gravity_scale(&mut self, v: f32);

// 睡眠 - 需求 80-82
pub fn is_sleeping(&self) -> bool;
pub fn wake_up(&mut self, strong: bool);
pub fn sleep(&mut self);

// CCD - 需求 83-84
pub fn ccd_enabled(&self) -> bool;
pub fn enable_ccd(&mut self, b: bool);
```

---

## 4. Collider3D

### 4.1 ColliderShape

```rust
// 需求 349
pub enum ColliderShape {
    Ball { radius: f32 },
    Cuboid { hx: f32, hy: f32, hz: f32 },
    Capsule { half_height: f32, radius: f32, axis: Axis },
    Cylinder { half_height: f32, radius: f32 },
    Cone { half_height: f32, radius: f32 },
    ConvexHull { points: Vec<Vec3> },
    Trimesh { vertices: Vec<Vec3>, indices: Vec<[u32; 3]> },
    Heightfield { heights: Vec<f32>, scale: Vec3 },
    Segment { a: Vec3, b: Vec3 },
    Triangle { a: Vec3, b: Vec3, c: Vec3 },
    Halfspace { outward_normal: Vec3 },
}
```

### 4.2 CollisionGroups

```rust
// 需求 120-122
pub struct CollisionGroups {
    pub memberships: u32,
    pub filter: u32,
}

impl CollisionGroups {
    pub fn new(memberships: u32, filter: u32) -> Self;
    pub const ALL: CollisionGroups;
    pub const NONE: CollisionGroups;
}
```

### 4.3 ColliderBuilder

```rust
// 需求 89-100
impl ColliderBuilder {
    pub fn ball(radius: f32) -> Self;
    pub fn cuboid(hx: f32, hy: f32, hz: f32) -> Self;
    pub fn capsule(half_h: f32, radius: f32, axis: Axis) -> Self;
    pub fn cylinder(half_h: f32, radius: f32) -> Self;
    pub fn cone(half_h: f32, radius: f32) -> Self;
    pub fn convex_hull(points: &[Vec3]) -> Option<Self>;
    pub fn trimesh(vertices: Vec<Vec3>, indices: Vec<[u32; 3]>) -> Self;
    pub fn heightfield(heights: Vec<f32>, scale: Vec3) -> Self;
    pub fn polyline(vertices: Vec<Vec3>) -> Self;
    pub fn segment(a: Vec3, b: Vec3) -> Self;
    pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Self;
    pub fn halfspace(outward_normal: Vec3) -> Self;
}

// 需求 101-114
impl ColliderBuilder {
    pub fn translation(v: Vec3) -> Self;
    pub fn rotation(q: Quat) -> Self;
    pub fn density(v: f32) -> Self;
    pub fn mass(v: f32) -> Self;
    pub fn mass_properties(mp: MassProperties) -> Self;
    pub fn friction(v: f32) -> Self;
    pub fn friction_combine_rule(rule: CombineRule) -> Self;
    pub fn restitution(v: f32) -> Self;
    pub fn restitution_combine_rule(rule: CombineRule) -> Self;
    pub fn collision_groups(groups: CollisionGroups) -> Self;
    pub fn solver_groups(groups: CollisionGroups) -> Self;
    pub fn sensor(b: bool) -> Self;
    pub fn contact_force_event_threshold(v: f32) -> Self;
    pub fn contact_skin(v: f32) -> Self;
}

// 需求 115
pub fn build(&self) -> Collider3D;
```

### 4.4 Collider3D 实例方法

```rust
// 需求 123, 348
pub fn type(&self) -> ColliderType;
pub fn shape(&self) -> ColliderShape;

// 需求 124, 340
pub fn aabb(&self, body_transform: &Isometry3<f32>) -> AABB;

// 需求 125, 341
pub fn mass(&self) -> f32;

// 需求 126-127, 342-343
pub fn friction(&self) -> f32;
pub fn set_friction(&mut self, v: f32);

// 需求 128-129, 344-345
pub fn restitution(&self) -> f32;
pub fn set_restitution(&mut self, v: f32);

// 需求 130-131, 346-347
pub fn is_sensor(&self) -> bool;
pub fn set_sensor(&mut self, b: bool);
```

---

## 5. Joint3D

### 5.1 FixedJointBuilder

```rust
// 需求 350-355
pub struct FixedJointBuilder { /* ... */ }

impl FixedJointBuilder {
    pub fn new() -> Self;
    pub fn local_anchor1(v: Vec3) -> Self;
    pub fn local_anchor2(v: Vec3) -> Self;
    pub fn local_basis1(q: Quat) -> Self;
    pub fn local_basis2(q: Quat) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### 5.2 RevoluteJointBuilder

```rust
// 需求 356-362
pub struct RevoluteJointBuilder { /* ... */ }

impl RevoluteJointBuilder {
    pub fn new(axis: Vec3) -> Self;
    pub fn local_anchor1(v: Vec3) -> Self;
    pub fn local_anchor2(v: Vec3) -> Self;
    pub fn motor_model(model: MotorModel) -> Self;
    pub fn limits(min: f32, max: f32) -> Self;
    pub fn motor_velocity(vel: f32, factor: f32) -> Self;
    pub fn motor_position(pos: f32, stiffness: f32, damping: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### 5.3 PrismaticJointBuilder

```rust
// 需求 363-364
pub struct PrismaticJointBuilder { /* ... */ }

impl PrismaticJointBuilder {
    pub fn new(axis: Vec3) -> Self;
    pub fn limits(min: f32, max: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### 5.4 BallJointBuilder

```rust
// 需求 365-366
pub struct BallJointBuilder { /* ... */ }

impl BallJointBuilder {
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self;
    pub fn limits(max_angle: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### 5.5 DistanceJointBuilder

```rust
// 需求 367-368
pub struct DistanceJointBuilder { /* ... */ }

impl DistanceJointBuilder {
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self;
    pub fn length(l: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### 5.6 RopeJointBuilder

```rust
// 需求 369
pub struct RopeJointBuilder { /* ... */ }

impl RopeJointBuilder {
    pub fn new(anchor1: Vec3, anchor2: Vec3, max_length: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### 5.7 SphericalJointBuilder

```rust
// 需求 370
pub struct SphericalJointBuilder { /* ... */ }

impl SphericalJointBuilder {
    pub fn with_cone_limit(axis: Vec3, angle: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

---

## 6. CharacterController3D

```rust
// 需求 142-150, 371-378
impl CharacterController3D {
    pub fn new(
        offset: Vec3,
        up_dir: Vec3,
        max_slope_climb_angle: f32,
        max_slide_angle: f32,
    ) -> Self;
    
    pub fn move_shape(
        &mut self,
        dt: f32,
        desired_translation: Vec3,
        body: &mut RigidBody3D,
        collider: &Collider3D,
        filter: QueryFilter,
    ) -> CharacterMovement;
    
    pub fn set_apply_impulse_to_dynamic_bodies(&mut self, b: bool);
    pub fn set_slope_climb_angle(&mut self, angle: f32);
    pub fn set_slide_angle(&mut self, angle: f32);
    pub fn set_offset(&mut self, offset: Vec3);
    pub fn set_max_distance_to_ground(&mut self, d: f32);
    pub fn set_up(&mut self, v: Vec3);
}

// 需求 151-154, 379-382
impl CharacterMovement {
    pub fn translation(&self) -> Vec3;
    pub fn grounded(&self) -> bool;
    pub fn hit_ceil(&self) -> bool;
    pub fn hit_wall(&self) -> bool;
    pub fn ground_normal(&self) -> Vec3;
}
```

---

## 7. Query3D

```rust
// 需求 162-163, 391-392
impl Ray3 {
    pub fn new(origin: Vec3, dir: Vec3) -> Self;
    pub fn point_at(&self, t: f32) -> Vec3;
}

// 需求 164, 405-408
impl RayCastHit {
    pub fn entity(&self) -> Entity;
    pub fn toi(&self) -> f32;
    pub fn point(&self) -> Vec3;
    pub fn normal(&self) -> Vec3;
}

// 需求 161, 399-404
impl QueryFilter {
    pub fn new() -> Self;
    pub fn only_dynamic() -> Self;
    pub fn exclude_sensors() -> Self;
    pub fn groups(collision_groups: CollisionGroups) -> Self;
    pub fn exclude_fixed() -> Self;
    pub fn exclude(entity: Entity) -> Self;
}

// 需求 155-160, 393-398
impl Query3D {
    pub fn cast_ray(
        &self,
        world: &PhysicsWorld3D,
        ray: &Ray3,
        max_toi: f32,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<(Entity, f32, Vec3)>;
    
    pub fn cast_ray_and_get_normal(
        &self,
        world: &PhysicsWorld3D,
        ray: &Ray3,
        max_toi: f32,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<RayCastHit>;
    
    pub fn cast_shape(
        &self,
        world: &PhysicsWorld3D,
        shape: &ColliderShape,
        pos: Vec3,
        dir: Vec3,
        max_toi: f32,
        filter: QueryFilter,
    ) -> Option<ShapeCastHit>;
    
    pub fn intersection_with_shape(
        &self,
        world: &PhysicsWorld3D,
        shape: &ColliderShape,
        pos: Vec3,
        filter: QueryFilter,
    ) -> Vec<Entity>;
    
    pub fn point_intersections(
        &self,
        world: &PhysicsWorld3D,
        point: Vec3,
        filter: QueryFilter,
        cb: impl FnMut(Entity),
    );
    
    pub fn intersections_with_aabb(
        &self,
        world: &PhysicsWorld3D,
        aabb: AABB,
        filter: QueryFilter,
        cb: impl FnMut(Entity),
    );
}
```

---

## 8. 事件类型

```rust
// 需求 165, 409
pub enum ContactEvent {
    Started(ColliderHandle, ColliderHandle),
    Stopped(ColliderHandle, ColliderHandle),
}

// 需求 168, 410
pub enum IntersectionEvent {
    Started(ColliderHandle, ColliderHandle),
    Stopped(ColliderHandle, ColliderHandle),
}

// 需求 166, 411-412
pub struct ContactForceEvent {
    pub handles: (ColliderHandle, ColliderHandle),
    pub total_force: Vec3,
    pub total_magnitude: f32,
}

impl ContactForceEvent {
    pub fn with_threshold(threshold: f32) -> Self;
    pub fn total_force(&self) -> Vec3;
    pub fn total_magnitude(&self) -> f32;
}

// 需求 170, 413-417
pub struct ContactPair { /* ... */ }

impl ContactPair {
    pub fn normal(&self) -> Vec3;
    pub fn points(&self) -> &[ContactPoint];
}

pub struct ContactPoint {
    pub point: Vec3,
    pub penetration: f32,
}

impl ContactPoint {
    pub fn point(&self) -> Vec3;
    pub fn penetration(&self) -> f32;
}
```

---

## 9. 调试可视化

```rust
// 需求 171-178
impl PhysicsDebugRenderer3D {
    pub fn new() -> Self;
    pub fn draw_world(&mut self, world: &PhysicsWorld3D, renderer: &mut dyn Renderer, view_proj: Mat4);
    pub fn set_draw_colliders(&mut self, b: bool);
    pub fn set_draw_joints(&mut self, b: bool);
    pub fn set_draw_contacts(&mut self, b: bool);
    pub fn set_draw_aabb(&mut self, b: bool);
    pub fn set_color(&mut self, color: Color);
    pub fn flush(&self, renderer: &mut dyn Renderer);
}
```

---

## 10. ECS 集成

```rust
// 需求 179-217
pub struct PhysicsModule;

impl PhysicsModule {
    pub fn register_systems(app: &mut App);
}

pub struct PhysicsConfig {
    pub gravity: Vec3,
    pub default_friction: f32,
    pub default_restitution: f32,
    pub max_substeps: u32,
    pub ccd: bool,
    pub fixed_dt: f32,
    pub debug_draw_enabled: bool,
}

impl PhysicsConfig {
    pub fn load(path: &Path) -> Result<Self>;
    pub fn save(&self, path: &Path) -> Result<()>;
}

pub struct PhysicsStats {
    pub num_bodies: usize,
    pub num_colliders: usize,
    pub num_joints: usize,
    pub step_time_ms: f32,
}

impl PhysicsStats {
    pub fn to_string(&self) -> String;
}

// 需求 219-220
pub enum PhysicsTimestepMode {
    FixedDelta(f32),
    Variable,
}
```

---

## 11. 组件与系统

```rust
// 需求 181-183
pub struct RigidBodyComponent {
    pub handle: RigidBodyHandle,
    pub dirty: bool,
}

pub struct ColliderComponent {
    pub handle: ColliderHandle,
    pub parent_body: Option<Entity>,
}

pub struct JointComponent {
    pub handle: JointHandle,
    pub entity_a: Entity,
    pub entity_b: Entity,
}

// 需求 186
pub struct PhysicsQuery<'w, 's> {
    // 内部状态
}

impl<'w, 's> PhysicsQuery<'w, 's> {
    pub fn cast_ray(&self, ray: &Ray3, max_toi: f32, filter: QueryFilter) -> Option<RayCastHit>;
    // ... 其他查询方法
}
```

---

## 12. 时间戳版本

- **版本**: 0.11.0
- **CHANGELOG 记录**: 需求 251
