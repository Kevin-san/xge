# 物理世界（World2D）模块需求

## 模块概述

物理世界（World2D）是 2D 物理引擎的核心，负责管理所有刚体、碰撞体、关节，执行物理仿真（重力、碰撞检测、接触求解），并提供空间查询接口（射线检测、形状投射、点/矩形重叠查询）。

本模块对应 `engine-physics-2d` crate，是 Sprint 04 的核心交付之一。

---

## 需求清单

### 1. 基础生命周期

| 编号 | 需求 | 描述 |
|------|------|------|
| 1 | crate 建立 | 建立 `engine-physics-2d` crate |
| 2 | `World2D::new(gravity)` | 创建物理世界，指定重力向量 |
| 3 | `World2D::set_gravity(&mut self, v)` | 设置重力向量 |
| 4 | `World2D::gravity(&self) -> Vec2` | 获取当前重力向量 |
| 5 | `World2D::step(&mut self, dt)` | 执行一步物理仿真，默认 dt=1/60 |
| 6 | `World2D::step_with_iterations(&mut self, dt, velocity_iter, position_iter)` | 指定迭代次数执行仿真 |
| 47 | `World2D::set_paused(&mut self, bool)` | 暂停/恢复物理仿真 |
| 218 | `World2D::clear(&mut self)` | 清空物理世界（移除所有 body、collider、joint） |

### 2. 刚体管理

| 编号 | 需求 | 描述 |
|------|------|------|
| 7 | `World2D::insert_body(body) -> BodyHandle` | 将刚体插入世界，返回句柄 |
| 8 | `World2D::remove_body(handle)` | 根据句柄移除刚体 |
| 11 | `World2D::get_body(&self, handle) -> &RigidBody2D` | 获取不可变引用 |
| 12 | `World2D::get_body_mut(&mut self, handle) -> &mut RigidBody2D` | 获取可变引用 |
| 13 | `World2D::bodies(&self) -> 迭代器` | 遍历所有刚体 |
| 214 | `World2D::insert_body` 后 handle 可用 | 插入后立即可通过 handle 访问 |
| 215 | `World2D::remove_body` 后 handle 失效 | 移除后 handle 不再可用 |

### 3. 碰撞体管理

| 编号 | 需求 | 描述 |
|------|------|------|
| 9 | `World2D::insert_collider(collider, handle)` | 将碰撞体附加到指定刚体 |
| 10 | `World2D::remove_collider(handle)` | 移除碰撞体 |
| 14 | `World2D::colliders(&self) -> 迭代器` | 遍历所有碰撞体 |

### 4. 关节管理

| 编号 | 需求 | 描述 |
|------|------|------|
| 266 | `World2D::insert_joint(joint) -> JointHandle` | 插入关节，返回句柄 |
| 267 | `World2D::remove_joint(handle)` | 移除关节 |
| 268 | `World2D::joints(&self) -> 迭代器` | 遍历所有关节 |

### 5. 接触信息

| 编号 | 需求 | 描述 |
|------|------|------|
| 15 | `World2D::contacts(&self) -> Vec<Contact2D>` | 获取所有接触点信息 |
| 69 | `Contact2D` 结构体 | 包含 body_a / body_b / point / normal / penetration |
| 70 | `ContactEvent::Started(Contact2D) / Ended(Contact2D)` | 接触开始/结束事件 |
| 98 | `ContactManifold` | 多点接触信息 |
| 216 | `World2D::events(&self) -> 迭代所有 contact events` | 遍历接触事件 |
| 217 | `World2D::contact_manifolds(&self) -> 迭代所有 manifold` | 遍历接触流形 |

### 6. 空间查询

| 编号 | 需求 | 描述 |
|------|------|------|
| 16 | `World2D::ray_cast(origin, dir, max_toi, filter) -> Option<RayCastHit2D>` | 射线投射 |
| 17 | `World2D::shape_cast(shape, origin, dir, max_toi) -> Option<ShapeCastHit2D>` | 形状投射 |
| 18 | `World2D::point_overlap(point, filter) -> Vec<BodyHandle>` | 点重叠查询 |
| 19 | `World2D::aabb_overlap(aabb, filter) -> Vec<BodyHandle>` | AABB 重叠查询 |
| 79 | `RayCastHit2D` | 包含 handle / point / normal / toi |
| 80 | `QueryFilter` | 过滤选项（跳过指定 handle、是否包含传感器） |
| 211 | `World2D::ray_cast` 支持过滤 sensor | 射线检测可配置是否穿透传感器 |
| 212 | `World2D::point_overlap` 找到包含该点的所有传感器 | 点查询可返回传感器 |
| 213 | `World2D::aabb_overlap` 找到与 AABB 相交的全部 | AABB 查询返回所有相交物体 |

### 7. 仿真参数与高级特性

| 编号 | 需求 | 描述 |
|------|------|------|
| 201 | `World2D::new()` 支持默认重力 (0, -9.81) | 默认重力配置 |
| 202 | `World2D::step(dt)` 在 dt 过大时分多步 | 自适应子步进 |
| 203 | `World2D::step(dt)` 分 velocity / position 两个阶段 | 分离求解器 |
| 204 | `World2D` 维护 active body 列表 | 活跃刚体优化 |
| 205 | `World2D` 支持 sleep / awake（energy threshold） | 休眠机制 |
| 206 | `World2D` 支持 broad phase：AABB tree / sweep and prune | 宽相碰撞检测 |
| 207 | `World2D` 支持 narrow phase：GJK / SAT | 窄相碰撞检测 |
| 208 | `World2D` 支持 contact manifold 解算：sequential impulse | 顺序冲动求解 |
| 209 | `World2D` 支持 friction 模型：Coulomb friction | 库仑摩擦模型 |
| 210 | `World2D` 支持 restitution 模型：速度阈值以上生效 | 弹性恢复模型 |

### 8. 调试渲染

| 编号 | 需求 | 描述 |
|------|------|------|
| 88 | `PhysicsDebugRenderer` | 绘制碰撞体线框（基于 DebugRenderer） |
| 225 | Debug 绘制：按 `` `键 B`` 显示/隐藏碰撞体线框 | 切换线框显示 |
| 226 | Debug 绘制：按 `` `键 P`` 暂停/继续物理 | 切换物理暂停 |
| 227 | Debug 绘制：按 `` `键 F`` 显示/隐藏 FPS / FrameTime | 显示性能信息 |

---

## API 签名

### World2D

```rust
pub struct World2D;

impl World2D {
    pub fn new(gravity: Vec2) -> Self;
    pub fn new_default() -> Self;  // gravity = (0, -9.81)
    
    pub fn set_gravity(&mut self, v: Vec2);
    pub fn gravity(&self) -> Vec2;
    
    pub fn step(&mut self, dt: f32);
    pub fn step_with_iterations(&mut self, dt: f32, velocity_iter: u32, position_iter: u32);
    
    pub fn set_paused(&mut self, paused: bool);
    
    pub fn insert_body(&mut self, body: RigidBody2D) -> BodyHandle;
    pub fn remove_body(&mut self, handle: BodyHandle);
    pub fn get_body(&self, handle: BodyHandle) -> &RigidBody2D;
    pub fn get_body_mut(&mut self, handle: BodyHandle) -> &mut RigidBody2D;
    pub fn bodies(&self) -> impl Iterator<Item = &RigidBody2D>;
    
    pub fn insert_collider(&mut self, collider: Collider2D, body_handle: BodyHandle) -> ColliderHandle;
    pub fn remove_collider(&mut self, handle: ColliderHandle);
    pub fn colliders(&self) -> impl Iterator<Item = &Collider2D>;
    
    pub fn insert_joint(&mut self, joint: Joint2D) -> JointHandle;
    pub fn remove_joint(&mut self, handle: JointHandle);
    pub fn joints(&self) -> impl Iterator<Item = &Joint2D>;
    
    pub fn contacts(&self) -> Vec<Contact2D>;
    pub fn events(&self) -> impl Iterator<Item = &ContactEvent>;
    pub fn contact_manifolds(&self) -> impl Iterator<Item = &ContactManifold>;
    
    pub fn ray_cast(&self, origin: Vec2, dir: Vec2, max_toi: f32, filter: QueryFilter) -> Option<RayCastHit2D>;
    pub fn shape_cast(&self, shape: &dyn Shape, origin: Vec2, dir: Vec2, max_toi: f32) -> Option<ShapeCastHit2D>;
    pub fn point_overlap(&self, point: Vec2, filter: QueryFilter) -> Vec<BodyHandle>;
    pub fn aabb_overlap(&self, aabb: AABB, filter: QueryFilter) -> Vec<BodyHandle>;
    
    pub fn clear(&mut self);
}
```

### Contact2D & ContactEvent

```rust
pub struct Contact2D {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub point: Vec2,
    pub normal: Vec2,
    pub penetration: f32,
}

pub enum ContactEvent {
    Started(Contact2D),
    Ended(Contact2D),
}

pub struct ContactManifold {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub points: Vec<Contact2D>,
}
```

### QueryFilter & RayCastHit2D

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

## 输入/输出

### 输入
- 重力向量 `Vec2`
- 时间步长 `dt: f32`
- 迭代次数（可选）
- 刚体/碰撞体/关节配置

### 输出
- 更新后的刚体状态（位置、旋转、速度）
- 接触事件流
- 空间查询结果

---

## 验收标准

1. ✅ `World2D::new(Vec2::ZERO)` 可创建无重力世界
2. ✅ `World2D::step(dt)` 在 dt=1/60 时仿真稳定
3. ✅ `insert_body` 返回的 handle 可立即用于 `get_body`
4. ✅ `remove_body` 后再次访问该 handle 返回 `None`
5. ✅ `contacts()` 返回当前帧所有接触点
6. ✅ `ray_cast` 正确命中目标并返回精确交点
7. ✅ `step(dt)` 支持 dt > 1/30 时自动分多步
8. ✅ 支持 Coulomb 摩擦和弹性恢复
9. ✅ DebugRenderer 可正确绘制线框
10. ✅ 按 `B` 键切换线框显示、按 `P` 切换暂停、按 `F` 显示 FPS

---

## 依赖关系

- 依赖 `math` crate（Vec2、Mat3、AABB）
- 被 `PhysicsModule` 封装集成到引擎
- 被 `Body2DNode` 使用实现节点与物理世界同步
- 示例 `ball_pit`、`dominoes`、`ray_cast`、`joints` 依赖本模块

---

## 优先级

| 优先级 | 含义 | 需求编号 |
|--------|------|----------|
| P0 | 核心功能，MVP 必须完成 | 1-19, 47, 69-70, 79-80, 88, 201-215, 225-227 |
| P1 | 重要功能，影响演示 | 6, 98, 211-213, 216-218 |
| P2 | 增强功能，提升体验 | 204-210 |
