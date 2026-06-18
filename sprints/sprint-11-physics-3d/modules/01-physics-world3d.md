# 3D 物理世界模块

## 模块概述

`PhysicsWorld3D` 是 3D 物理引擎的核心模块，负责管理整个物理模拟空间。该模块提供物理世界的创建、配置、步进控制，以及 body、collider、joint 的注册与管理功能。支持重力配置、CCD（连续碰撞检测）、碰撞分组、查询管道等高级特性。

## 需求编号

对应原需求清单：**236-303**

| 编号 | 功能描述 | 优先级 |
|------|----------|--------|
| 236 | `PhysicsWorld3D::new(gravity)` 初始化所有内部状态 | P0 |
| 237 | `PhysicsWorld3D::step(dt)` 执行一次模拟 | P0 |
| 238 | `PhysicsWorld3D::step_with_substeps(dt, substeps)` 细分模拟 | P0 |
| 239 | `PhysicsWorld3D::gravity(&self) -> Vec3` | P0 |
| 240 | `PhysicsWorld3D::set_gravity(&mut self, g)` | P0 |
| 241 | `PhysicsWorld3D::set_paused(paused)` 暂停后 step 不更新位置 | P1 |
| 242 | `PhysicsWorld3D::paused(&self) -> bool` | P1 |
| 243 | `PhysicsWorld3D::insert_body(body) -> RigidBodyHandle` | P0 |
| 244 | `PhysicsWorld3D::remove_body(handle)` | P0 |
| 245 | `PhysicsWorld3D::body(&self, handle) -> &RigidBody3D` | P0 |
| 246 | `PhysicsWorld3D::body_mut(&mut self, handle) -> &mut RigidBody3D` | P0 |
| 247 | `PhysicsWorld3D::insert_collider(collider, parent) -> ColliderHandle` | P0 |
| 248 | `PhysicsWorld3D::remove_collider(handle)` | P0 |
| 249 | `PhysicsWorld3D::collider(&self, handle) -> &Collider3D` | P0 |
| 250 | `PhysicsWorld3D::collider_mut(&mut self, handle) -> &mut Collider3D` | P0 |
| 251 | `PhysicsWorld3D::insert_joint(body1, body2, joint) -> JointHandle` | P0 |
| 252 | `PhysicsWorld3D::remove_joint(handle)` | P0 |
| 253 | `PhysicsWorld3D::num_bodies(&self) -> usize` | P1 |
| 254 | `PhysicsWorld3D::num_colliders(&self) -> usize` | P1 |
| 255 | `PhysicsWorld3D::num_joints(&self) -> usize` | P1 |
| 256 | `PhysicsWorld3D::ccd_enabled(&self) -> bool` | P1 |
| 257 | `PhysicsWorld3D::set_ccd_enabled(&mut self, bool)` | P1 |
| 258 | `PhysicsWorld3D::gravity_scale(&self) -> f32` | P1 |
| 259 | `PhysicsWorld3D::set_gravity_scale(&mut self, v)` | P1 |
| 260 | `PhysicsWorld3D::contact_events(&self) -> &[ContactEvent]` | P0 |
| 261 | `PhysicsWorld3D::intersection_events(&self) -> &[IntersectionEvent]` | P0 |
| 262 | `PhysicsWorld3D::contact_force_events(&self) -> &[ContactForceEvent]` | P1 |
| 263 | `PhysicsWorld3D::contact_pair(&self, a, b) -> Option<ContactPair>` | P1 |
| 264 | `PhysicsWorld3D::bodies_iter(&self) -> impl Iterator<Item=(RigidBodyHandle, &RigidBody3D)>` | P1 |
| 265 | `PhysicsWorld3D::colliders_iter(&self) -> impl Iterator<Item=(ColliderHandle, &Collider3D)>` | P1 |
| 266 | `PhysicsWorld3D::query_pipeline(&self) -> &QueryPipeline` | P0 |
| 267 | `PhysicsWorld3D::clear(&mut self)` 清空所有 body/collider/joint | P1 |

## API 签名

### 构造函数

```rust
pub fn new(gravity: Vec3) -> Self
```

### 世界步进

```rust
pub fn step(&mut self, dt: f32)
pub fn step_with_substeps(&mut self, dt: f32, substeps: u32)
```

### 重力配置

```rust
pub fn gravity(&self) -> Vec3
pub fn set_gravity(&mut self, g: Vec3)
pub fn gravity_scale(&self) -> f32
pub fn set_gravity_scale(&mut self, v: f32)
```

### 暂停控制

```rust
pub fn paused(&self) -> bool
pub fn set_paused(&mut self, paused: bool)
```

### CCD 配置

```rust
pub fn ccd_enabled(&self) -> bool
pub fn set_ccd_enabled(&mut self, enabled: bool)
```

### Body 管理

```rust
pub fn insert_body(&mut self, body: RigidBody3D) -> RigidBodyHandle
pub fn remove_body(&mut self, handle: RigidBodyHandle)
pub fn body(&self, handle: RigidBodyHandle) -> &RigidBody3D
pub fn body_mut(&mut self, handle: RigidBodyHandle) -> &mut RigidBody3D
pub fn num_bodies(&self) -> usize
pub fn bodies_iter(&self) -> impl Iterator<Item = (RigidBodyHandle, &RigidBody3D)>
```

### Collider 管理

```rust
pub fn insert_collider(&mut self, collider: Collider3D, parent_body: RigidBodyHandle) -> ColliderHandle
pub fn remove_collider(&mut self, handle: ColliderHandle)
pub fn collider(&self, handle: ColliderHandle) -> &Collider3D
pub fn collider_mut(&mut self, handle: ColliderHandle) -> &mut Collider3D
pub fn num_colliders(&self) -> usize
pub fn colliders_iter(&self) -> impl Iterator<Item = (ColliderHandle, &Collider3D)>
```

### Joint 管理

```rust
pub fn insert_joint(&mut self, body1: RigidBodyHandle, body2: RigidBodyHandle, joint: Joint3D) -> JointHandle
pub fn remove_joint(&mut self, handle: JointHandle)
pub fn num_joints(&self) -> usize
```

### 事件查询

```rust
pub fn contact_events(&self) -> &[ContactEvent]
pub fn intersection_events(&self) -> &[IntersectionEvent]
pub fn contact_force_events(&self) -> &[ContactForceEvent]
pub fn contact_pair(&self, a: ColliderHandle, b: ColliderHandle) -> Option<ContactPair>
```

### 查询管道

```rust
pub fn query_pipeline(&self) -> &QueryPipeline
```

### 清理

```rust
pub fn clear(&mut self)
```

## 输入/输出

| 方法 | 输入 | 输出 |
|------|------|------|
| `new(gravity)` | `Vec3` 重力向量 | `PhysicsWorld3D` 实例 |
| `step(dt)` | `f32` 时间步长 | `()` |
| `insert_body(body)` | `RigidBody3D` | `RigidBodyHandle` |
| `body(handle)` | `RigidBodyHandle` | `&RigidBody3D` |
| `contact_events()` | - | `&[ContactEvent]` |
| `query_pipeline()` | - | `&QueryPipeline` |

## 验收标准

1. 创建 `PhysicsWorld3D` 实例后，所有内部状态正确初始化
2. `step(dt)` 执行后，动态刚体位置根据物理定律更新
3. `step(dt)` 为 0 时不崩溃（需求242）
4. `set_paused(true)` 后 `step` 不更新任何刚体位置
5. `insert_body/remove_body` 正确管理刚体生命周期
6. `insert_collider` 将碰撞体关联到指定父刚体
7. `contact_events` 在碰撞开始/结束时返回正确的事件
8. `query_pipeline` 返回可用于射线/形状查询的管道
9. `clear()` 后 `num_bodies/num_colliders/num_joints` 均为 0
10. CCD 启用后高速小物体能命中薄物体

## 依赖关系

- **内部依赖**: `RigidBody3D`, `Collider3D`, `Joint3D`, `QueryPipeline`
- **外部依赖**: `rapier3d` v0.19+ (默认后端)
- **被依赖**: 所有其他物理模块依赖于 `PhysicsWorld3D`

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 结束前完成
- **P1**: 重要功能，应在 Sprint 结束前完成
- **P2**: 增强功能，可延后到后续 Sprint
