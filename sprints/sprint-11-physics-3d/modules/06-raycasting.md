# 射线检测模块

## 模块概述

`Query3D` 模块提供 3D 物理查询功能，支持射线检测 (Ray Cast)、形状检测 (Shape Cast)、点查询 (Point Test) 和 AABB 查询。该模块是物理引擎中实现选中、拾取、碰撞检测等功能的基石。查询系统支持灵活的过滤机制，允许用户根据碰撞分组、刚体类型等条件筛选查询结果。同时提供接触事件系统，用于监听碰撞开始/结束等事件。

## 需求编号

对应原需求清单：**155-196, 391-466**

| 编号 | 功能描述 | 优先级 |
|------|----------|--------|
| 155 | `Query3D::cast_ray(&self, world, ray, max_toi, solid, filter) -> Option<(Entity, toi, normal)>` | P0 |
| 156 | `Query3D::cast_ray_and_get_normal(...)` | P0 |
| 157 | `Query3D::cast_shape(&self, world, shape, pos, dir, max_toi, filter) -> Option<(Entity, toi, normal)>` | P0 |
| 158 | `Query3D::intersection_with_shape(&self, world, shape, pos, filter) -> Vec<Entity>` | P0 |
| 159 | `Query3D::point_intersections(&self, world, point, filter, cb)` | P0 |
| 160 | `Query3D::intersections_with_aabb(&self, world, aabb, filter, cb)` | P0 |
| 161 | `QueryFilter::new() / only_dynamic() / exclude_sensors() / groups(collision_groups)` | P0 |
| 162 | `Ray3::new(origin, dir)` | P0 |
| 163 | `Ray3::point_at(&self, t) -> Vec3` | P0 |
| 164 | `RayCastHit::entity() / toi() / normal() / point()` | P0 |
| 165 | `ContactEvent::Started(a, b) / Stopped(a, b)` | P0 |
| 166 | `ContactForceEvent::with_threshold(threshold)` | P1 |
| 167 | `PhysicsWorld3D::contact_events(&self) -> &[ContactEvent]` | P0 |
| 168 | `PhysicsWorld3D::intersection_events(&self) -> &[IntersectionEvent]` | P0 |
| 169 | `PhysicsWorld3D::contact_force_events(&self) -> &[ContactForceEvent]` | P1 |
| 170 | `PhysicsWorld3D::contact_pair(&self, a, b) -> Option<ContactPair>` | P1 |
| 391 | `Ray3::new(origin, dir)` | P0 |
| 392 | `Ray3::point_at(&self, t) -> Vec3` | P0 |
| 393 | `Query3D::cast_ray(world, ray, max_toi, solid, filter) -> Option<(Entity, f32, Vec3)>` | P0 |
| 394 | `Query3D::cast_ray_and_get_normal(world, ray, max_toi, solid, filter) -> Option<RayCastHit>` | P0 |
| 395 | `Query3D::cast_shape(world, shape, pos, dir, max_toi, filter) -> Option<ShapeCastHit>` | P0 |
| 396 | `Query3D::intersection_with_shape(world, shape, pos, filter) -> Vec<Entity>` | P0 |
| 397 | `Query3D::point_intersections(world, point, filter, cb)` | P0 |
| 398 | `Query3D::intersections_with_aabb(world, aabb, filter, cb)` | P0 |
| 399 | `QueryFilter::new()` | P0 |
| 400 | `QueryFilter::only_dynamic()` | P0 |
| 401 | `QueryFilter::exclude_sensors()` | P0 |
| 402 | `QueryFilter::groups(collision_groups)` | P0 |
| 403 | `QueryFilter::exclude_fixed()` | P0 |
| 404 | `QueryFilter::exclude(body)` | P0 |
| 405 | `RayCastHit::entity() -> Entity` | P0 |
| 406 | `RayCastHit::toi(&self) -> f32` | P0 |
| 407 | `RayCastHit::point(&self) -> Vec3` | P0 |
| 408 | `RayCastHit::normal(&self) -> Vec3` | P0 |
| 409 | `ContactEvent::Started(a, b) / Stopped(a, b)` | P0 |
| 410 | `IntersectionEvent::Started(a, b) / Stopped(a, b)` | P0 |
| 411 | `ContactForceEvent::total_force(&self) -> Vec3` | P1 |
| 412 | `ContactForceEvent::total_magnitude(&self) -> f32` | P1 |
| 413 | `PhysicsWorld3D::contact_pair(&self, a, b) -> Option<ContactPair>` | P1 |
| 414 | `ContactPair::normal(&self) -> Vec3` | P1 |
| 415 | `ContactPair::points(&self) -> &[ContactPoint]` | P1 |
| 416 | `ContactPoint::point(&self) -> Vec3` | P1 |
| 417 | `ContactPoint::penetration(&self) -> f32` | P1 |

## API 签名

### Ray3

```rust
pub struct Ray3 {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray3 {
    pub fn new(origin: Vec3, dir: Vec3) -> Self;
    pub fn point_at(&self, t: f32) -> Vec3;
}
```

### RayCastHit

```rust
pub struct RayCastHit {
    pub entity: Entity,
    pub toi: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl RayCastHit {
    pub fn entity(&self) -> Entity;
    pub fn toi(&self) -> f32;
    pub fn point(&self) -> Vec3;
    pub fn normal(&self) -> Vec3;
}
```

### ShapeCastHit

```rust
pub struct ShapeCastHit {
    pub entity: Entity,
    pub toi: f32,
    pub point: Vec3,
    pub normal: Vec3,
}
```

### QueryFilter

```rust
pub struct QueryFilter {
    // 内部状态
}

impl QueryFilter {
    pub fn new() -> Self;
    pub fn only_dynamic(mut self) -> Self;
    pub fn exclude_sensors(mut self) -> Self;
    pub fn groups(mut self, groups: CollisionGroups) -> Self;
    pub fn exclude_fixed(mut self) -> Self;
    pub fn exclude(mut self, entity: Entity) -> Self;
}
```

### Query3D

```rust
pub struct Query3D {
    // 内部状态
}

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
        mut cb: impl FnMut(Entity),
    );
    
    pub fn intersections_with_aabb(
        &self,
        world: &PhysicsWorld3D,
        aabb: AABB,
        filter: QueryFilter,
        mut cb: impl FnMut(Entity),
    );
}
```

### ContactEvent / IntersectionEvent

```rust
pub enum ContactEvent {
    Started(ColliderHandle, ColliderHandle),
    Stopped(ColliderHandle, ColliderHandle),
}

pub enum IntersectionEvent {
    Started(ColliderHandle, ColliderHandle),
    Stopped(ColliderHandle, ColliderHandle),
}

pub struct ContactForceEvent {
    pub handles: (ColliderHandle, ColliderHandle),
    pub total_force: Vec3,
    pub total_magnitude: f32,
}

impl ContactForceEvent {
    pub fn with_threshold(threshold: f32) -> Self { /* ... */ }
    pub fn total_force(&self) -> Vec3;
    pub fn total_magnitude(&self) -> f32;
}
```

### ContactPair

```rust
pub struct ContactPair {
    // 内部状态
}

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

## 输入/输出

| 方法 | 输入 | 输出 |
|------|------|------|
| `Ray3::new(origin, dir)` | `Vec3`, `Vec3` | `Ray3` |
| `Ray3::point_at(t)` | `f32` | `Vec3` |
| `cast_ray(world, ray, max_toi, solid, filter)` | `&PhysicsWorld3D`, `&Ray3`, `f32`, `bool`, `QueryFilter` | `Option<(Entity, f32, Vec3)>` |
| `cast_ray_and_get_normal(...)` | 同上 | `Option<RayCastHit>` |
| `cast_shape(world, shape, pos, dir, max_toi, filter)` | `&PhysicsWorld3D`, `&ColliderShape`, `Vec3`, `Vec3`, `f32`, `QueryFilter` | `Option<ShapeCastHit>` |
| `intersection_with_shape(world, shape, pos, filter)` | 同上 | `Vec<Entity>` |
| `contact_events()` | - | `&[ContactEvent]` |
| `contact_pair(a, b)` | `ColliderHandle`, `ColliderHandle` | `Option<ContactPair>` |

## 验收标准

1. `cast_ray` 返回射线击中的第一个物体的 (Entity, toi, normal)
2. `cast_ray_and_get_normal` 返回包含完整击中信息的 `RayCastHit`
3. 射线命中静态物体时返回 `toi > 0`（需求209）
4. 射线错过动态物体（在其外）时返回 `None`（需求210）
5. `CollisionGroups` 过滤正确，命中时正确排除（需求211）
6. `Ray3` 平行于平面时不崩溃（需求245）
7. `intersection_with_shape` 返回与形状相交的所有实体
8. `point_intersections` 回调正确触发
9. `intersections_with_aabb` 返回 AABB 内的所有实体
10. `QueryFilter::exclude_sensors` 正确排除传感器碰撞体
11. `QueryFilter::exclude(body)` 排除指定实体
12. `ContactEvent::Started` 在两碰撞体开始接触时触发
13. `ContactEvent::Stopped` 在两碰撞体结束接触时触发
14. `IntersectionEvent` 在传感器进入/离开时触发
15. `contact_pair` 返回两碰撞体间的接触对信息

## 依赖关系

- **内部依赖**: `PhysicsWorld3D`, `ColliderShape`, `CollisionGroups`
- **外部依赖**: `nalgebra` (Vec3), `bevy_ecs` (Entity)
- **被依赖**: `PhysicsModule`, `PhysicsDebugRenderer3D`

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 结束前完成
- **P1**: 重要功能，应在 Sprint 结束前完成
- **P2**: 增强功能，可延后到后续 Sprint
