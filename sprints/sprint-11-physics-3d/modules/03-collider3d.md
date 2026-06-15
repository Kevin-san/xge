# 3D 碰撞体模块

## 模块概述

`Collider3D` 模块提供 3D 物理模拟中的碰撞体抽象，支持多种几何形状：球体 (Ball)、盒体 (Cuboid)、胶囊体 (Capsule)、圆柱体 (Cylinder)、圆锥体 (Cone)、凸包 (ConvexHull)、三角网格 (Trimesh)、高度场 (Heightfield)、线段 (Segment)、三角形 (Triangle)、半空间 (Halfspace) 等。该模块通过 `ColliderBuilder` 建造者模式提供流畅的配置接口，支持设置摩擦力、弹力、碰撞分组、传感器等属性。

## 需求编号

对应原需求清单：**88-158, 319-388**

| 编号 | 功能描述 | 优先级 |
|------|----------|--------|
| 88 | `Collider3D`：各种形状 | P0 |
| 89 | `ColliderBuilder::ball(radius) -> Self` | P0 |
| 90 | `ColliderBuilder::cuboid(hx, hy, hz) -> Self` | P0 |
| 91 | `ColliderBuilder::capsule(half_h, radius, axis) -> Self` | P0 |
| 92 | `ColliderBuilder::cylinder(half_h, radius) -> Self` | P0 |
| 93 | `ColliderBuilder::cone(half_h, radius) -> Self` | P0 |
| 94 | `ColliderBuilder::convex_hull(points) -> Option<Self>` | P0 |
| 95 | `ColliderBuilder::trimesh(vertices, indices) -> Self` | P0 |
| 96 | `ColliderBuilder::heightfield(heights, scale) -> Self` | P0 |
| 97 | `ColliderBuilder::polyline(vertices) -> Self` | P2 |
| 98 | `ColliderBuilder::segment(a, b) -> Self` | P0 |
| 99 | `ColliderBuilder::triangle(a, b, c) -> Self` | P0 |
| 100 | `ColliderBuilder::halfspace(outward_normal) -> Self` | P0 |
| 101 | `ColliderBuilder::translation(v) -> Self` | P0 |
| 102 | `ColliderBuilder::rotation(q) -> Self` | P0 |
| 103 | `ColliderBuilder::density(v) -> Self` | P0 |
| 104 | `ColliderBuilder::mass(v) -> Self` | P0 |
| 105 | `ColliderBuilder::mass_properties(mp) -> Self` | P1 |
| 106 | `ColliderBuilder::friction(v) -> Self` | P0 |
| 107 | `ColliderBuilder::friction_combine_rule(rule) -> Self` | P1 |
| 108 | `ColliderBuilder::restitution(v) -> Self` | P0 |
| 109 | `ColliderBuilder::restitution_combine_rule(rule) -> Self` | P1 |
| 110 | `ColliderBuilder::collision_groups(groups) -> Self` | P0 |
| 111 | `ColliderBuilder::solver_groups(groups) -> Self` | P2 |
| 112 | `ColliderBuilder::sensor(b) -> Self` | P0 |
| 113 | `ColliderBuilder::contact_force_event_threshold(v) -> Self` | P1 |
| 114 | `ColliderBuilder::contact_skin(v) -> Self` | P2 |
| 115 | `ColliderBuilder::build(&self) -> Collider3D` | P0 |
| 116 | `PhysicsWorld3D::insert_collider(&mut self, collider, parent_body) -> ColliderHandle` | P0 |
| 117 | `PhysicsWorld3D::remove_collider(&mut self, handle)` | P0 |
| 118 | `PhysicsWorld3D::collider(&self, handle) -> &Collider3D` | P0 |
| 119 | `PhysicsWorld3D::collider_mut(&mut self, handle) -> &mut Collider3D` | P0 |
| 120 | `CollisionGroups::new(memberships, filter)` | P0 |
| 121 | `CollisionGroups::ALL` | P0 |
| 122 | `CollisionGroups::NONE` | P0 |
| 123 | `Collider3D::type(&self) -> ColliderType` | P0 |
| 124 | `Collider3D::aabb(&self, body) -> AABB` | P0 |
| 125 | `Collider3D::mass(&self) -> f32` | P0 |
| 126 | `Collider3D::friction(&self) -> f32` | P0 |
| 127 | `Collider3D::set_friction(&mut self, v)` | P0 |
| 128 | `Collider3D::restitution(&self) -> f32` | P0 |
| 129 | `Collider3D::set_restitution(&mut self, v)` | P0 |
| 130 | `Collider3D::is_sensor(&self) -> bool` | P0 |
| 131 | `Collider3D::set_sensor(&mut self, b)` | P0 |
| 320 | `ColliderBuilder::ball(r)` | P0 |
| 321 | `ColliderBuilder::cuboid(hx, hy, hz)` | P0 |
| 322 | `ColliderBuilder::capsule(half_h, r, axis)` | P0 |
| 323 | `ColliderBuilder::cylinder(half_h, r)` | P0 |
| 324 | `ColliderBuilder::cone(half_h, r)` | P0 |
| 325 | `ColliderBuilder::convex_hull(points)` | P0 |
| 326 | `ColliderBuilder::trimesh(vertices, indices)` | P0 |
| 327 | `ColliderBuilder::heightfield(heights, scale)` | P0 |
| 328 | `ColliderBuilder::segment(a, b)` | P0 |
| 329 | `ColliderBuilder::triangle(a, b, c)` | P0 |
| 330 | `ColliderBuilder::halfspace(n)` | P0 |
| 331 | `ColliderBuilder::translation(v) -> Self` | P0 |
| 332 | `ColliderBuilder::rotation(q) -> Self` | P0 |
| 333 | `ColliderBuilder::density(v) -> Self` | P0 |
| 334 | `ColliderBuilder::mass(v) -> Self` | P0 |
| 335 | `ColliderBuilder::friction(v) -> Self` | P0 |
| 336 | `ColliderBuilder::restitution(v) -> Self` | P0 |
| 337 | `ColliderBuilder::collision_groups(g) -> Self` | P0 |
| 338 | `ColliderBuilder::sensor(b) -> Self` | P0 |
| 339 | `ColliderBuilder::build(&self) -> Collider3D` | P0 |
| 340 | `Collider3D::aabb(&self, body_transform) -> AABB` | P0 |
| 341 | `Collider3D::mass(&self) -> f32` | P0 |
| 342 | `Collider3D::friction(&self) -> f32` | P0 |
| 343 | `Collider3D::set_friction(&mut self, v)` | P0 |
| 344 | `Collider3D::restitution(&self) -> f32` | P0 |
| 345 | `Collider3D::set_restitution(&mut self, v)` | P0 |
| 346 | `Collider3D::is_sensor(&self) -> bool` | P0 |
| 347 | `Collider3D::set_sensor(&mut self, b)` | P0 |
| 348 | `Collider3D::shape(&self) -> ColliderShape` | P0 |
| 349 | `ColliderShape::Ball(r) / Cuboid(hx, hy, hz) / Capsule(half_h, r) / Cylinder(half_h, r) / Cone(half_h, r) / Trimesh(verts, idx) / Heightfield(heights, scale) / ConvexHull(points) / Segment(a, b) / Triangle(a, b, c) / Halfspace(n)` | P0 |

## API 签名

### ColliderShape 枚举

```rust
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

### ColliderHandle 类型

```rust
pub struct ColliderHandle(/* 内部表示 */);
```

### CollisionGroups

```rust
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

### ColliderBuilder 建造者

```rust
impl ColliderBuilder {
    // 形状构造
    pub fn ball(radius: f32) -> Self;
    pub fn cuboid(hx: f32, hy: f32, hz: f32) -> Self;
    pub fn capsule(half_height: f32, radius: f32, axis: Axis) -> Self;
    pub fn cylinder(half_height: f32, radius: f32) -> Self;
    pub fn cone(half_height: f32, radius: f32) -> Self;
    pub fn convex_hull(points: &[Vec3]) -> Option<Self>;
    pub fn trimesh(vertices: Vec<Vec3>, indices: Vec<[u32; 3]>) -> Self;
    pub fn heightfield(heights: Vec<f32>, scale: Vec3) -> Self;
    pub fn polyline(vertices: Vec<Vec3>) -> Self;
    pub fn segment(a: Vec3, b: Vec3) -> Self;
    pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Self;
    pub fn halfspace(outward_normal: Vec3) -> Self;
    
    // 变换
    pub fn translation(mut self, v: Vec3) -> Self;
    pub fn rotation(mut self, q: Quat) -> Self;
    
    // 物理属性
    pub fn density(mut self, v: f32) -> Self;
    pub fn mass(mut self, v: f32) -> Self;
    pub fn mass_properties(mut self, mp: MassProperties) -> Self;
    pub fn friction(mut self, v: f32) -> Self;
    pub fn friction_combine_rule(mut self, rule: CombineRule) -> Self;
    pub fn restitution(mut self, v: f32) -> Self;
    pub fn restitution_combine_rule(mut self, rule: CombineRule) -> Self;
    
    // 分组
    pub fn collision_groups(mut self, groups: CollisionGroups) -> Self;
    pub fn solver_groups(mut self, groups: CollisionGroups) -> Self;
    
    // 传感器
    pub fn sensor(mut self, b: bool) -> Self;
    pub fn contact_force_event_threshold(mut self, v: f32) -> Self;
    pub fn contact_skin(mut self, v: f32) -> Self;
    
    pub fn build(&self) -> Collider3D;
}
```

### Collider3D 实例方法

```rust
pub fn shape(&self) -> ColliderShape;
pub fn type(&self) -> ColliderType;
pub fn aabb(&self, body_transform: &Isometry3<f32>) -> AABB;
pub fn mass(&self) -> f32;
pub fn friction(&self) -> f32;
pub fn set_friction(&mut self, v: f32);
pub fn restitution(&self) -> f32;
pub fn set_restitution(&mut self, v: f32);
pub fn is_sensor(&self) -> bool;
pub fn set_sensor(&mut self, b: bool);
```

## 输入/输出

| 方法 | 输入 | 输出 |
|------|------|------|
| `ColliderBuilder::ball(radius)` | `f32` | `ColliderBuilder` |
| `ColliderBuilder::cuboid(hx, hy, hz)` | `f32`, `f32`, `f32` | `ColliderBuilder` |
| `ColliderBuilder::convex_hull(points)` | `&[Vec3]` | `Option<ColliderBuilder>` |
| `ColliderBuilder::build()` | - | `Collider3D` |
| `insert_collider(collider, parent)` | `Collider3D`, `RigidBodyHandle` | `ColliderHandle` |
| `collider(handle)` | `ColliderHandle` | `&Collider3D` |
| `aabb(body_transform)` | `&Isometry3<f32>` | `AABB` |

## 验收标准

1. `ColliderBuilder::ball(1.0)` 创建球半径为 1.0 的碰撞体
2. `ColliderBuilder::cuboid(0.5, 1.0, 0.5)` 创建半尺寸为 (0.5, 1.0, 0.5) 的盒碰撞体
3. `ColliderBuilder::convex_hull(points)` 对有效点集返回 `Some`，对无效点返回 `None`
4. `ColliderBuilder::trimesh` 可正确处理三角网格索引
5. `ColliderBuilder::heightfield` 可用一维高度数组创建地形碰撞体
6. `aabb()` 根据给定的 body transform 返回正确的 AABB
7. `set_friction()` 改变摩擦系数后影响碰撞响应
8. `set_sensor(true)` 使碰撞体成为传感器，触发进入/离开事件但不产生物理碰撞
9. `CollisionGroups` 过滤正确，命中时正确排除（需求212）
10. 传感器触发 `intersection_event`（需求239）

## 依赖关系

- **内部依赖**: `RigidBodyHandle`, `CollisionGroups`
- **外部依赖**: `nalgebra` (Vec3, Quat, Isometry3)
- **被依赖**: `PhysicsWorld3D`, `CharacterController3D`, `Query3D`

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 结束前完成
- **P1**: 重要功能，应在 Sprint 结束前完成
- **P2**: 增强功能，可延后到后续 Sprint
