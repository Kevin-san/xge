# 模块四：Transform 与场景需求

## 4.1 模块概述

本模块定义了 3D 渲染系统中的变换（Transform3D）组件与场景图（Scene3D）结构。Transform3D 描述了 3D 对象的位置、旋转和缩放；Scene3D 管理场景中的所有节点，执行世界变换传播和视锥裁剪。

**对应原需求编号**：69-131, 337-429

**核心依赖**：
- `engine-math`：Vec3、Mat4、Quat 等数学类型
- `Mesh3D`：节点挂载的网格
- `Camera3D`：场景的主相机

---

## 4.2 Transform3D 结构体

### 4.2.1 变换构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 69 | Transform3D 结构 | Transform3D：位置/旋转/缩放 | - | - | 包含 translation/rotation/scale |
| 95 | Transform3D 结构（详细） | Transform3D：位置/旋转/缩放 | - | - | 与上述一致 |
| 337 | 创建单位变换 | `Transform3D::new()` | - | Self | 创建单位变换 |
| 381 | 创建单位变换（详细） | `Transform3D::new()` | - | Self | 与上述一致 |
| 96 | 从平移构造 | `Transform3D::from_translation(v: Vec3) -> Self` | Vec3 | Self | 仅设置平移 |
| 338 | 从平移构造（详细） | `Transform3D::from_translation(v)` | Vec3 | Self | 与上述一致 |
| 97 | 从旋转构造 | `Transform3D::from_rotation(q: Quat) -> Self` | Quat | Self | 仅设置旋转 |
| 339 | 从旋转构造（详细） | `Transform3D::from_rotation(q)` | Quat | Self | 与上述一致 |
| 98 | 从缩放构造 | `Transform3D::from_scale(v: Vec3) -> Self` | Vec3 | Self | 仅设置缩放 |
| 340 | 从缩放构造（详细） | `Transform3D::from_scale(v)` | Vec3 | Self | 与上述一致 |
| 357 | 单位常量 | `Transform3D::IDENTITY` | - | Transform3D | 静态单位变换常量 |

**优先级**：P0

---

## 4.3 矩阵计算

### 4.3.1 矩阵属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 74 | 获取变换矩阵 | `Transform3D::matrix(&self) -> Mat4` | - | Mat4 | T * R * S 顺序计算 |
| 100 | 变换矩阵（详细） | `Transform3D::matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 341 | 变换矩阵（详细） | `Transform3D::matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 75 | 获取逆矩阵 | `Transform3D::inverse_matrix(&self) -> Mat4` | - | Mat4 | 返回 matrix 的逆矩阵 |
| 101 | 逆矩阵（详细） | `Transform3D::inverse_matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 342 | 逆矩阵（详细） | `Transform3D::inverse_matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |

**优先级**：P0

---

## 4.4 变换属性访问

### 4.4.1 基础属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 76 | 获取平移 | `Transform3D::translation(&self) -> Vec3` | - | Vec3 | 返回位置 |
| 102 | 平移（详细） | `Transform3D::translation(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 343 | 平移（详细） | `Transform3D::translation(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 77 | 获取旋转 | `Transform3D::rotation(&self) -> Quat` | - | Quat | 返回四元数旋转 |
| 103 | 旋转（详细） | `Transform3D::rotation(&self) -> Quat` | - | Quat | 与上述一致 |
| 344 | 旋转（详细） | `Transform3D::rotation(&self) -> Quat` | - | Quat | 与上述一致 |
| 78 | 获取缩放 | `Transform3D::scale(&self) -> Vec3` | - | Vec3 | 返回缩放 |
| 104 | 缩放（详细） | `Transform3D::scale(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 345 | 缩放（详细） | `Transform3D::scale(&self) -> Vec3` | - | Vec3 | 与上述一致 |

**优先级**：P0

---

## 4.5 变换修改器

### 4.5.1 设置操作

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 79 | 设置平移 | `Transform3D::set_translation(&mut self, v: Vec3)` | Vec3 | - | 直接设置位置 |
| 105 | 设置平移（详细） | `Transform3D::set_translation(&mut self, v)` | Vec3 | - | 与上述一致 |
| 346 | 设置平移（详细） | `Transform3D::set_translation(&mut self, v)` | Vec3 | - | 与上述一致 |
| 80 | 设置旋转 | `Transform3D::set_rotation(&mut self, q: Quat)` | Quat | - | 直接设置旋转 |
| 106 | 设置旋转（详细） | `Transform3D::set_rotation(&mut self, q)` | Quat | - | 与上述一致 |
| 347 | 设置旋转（详细） | `Transform3D::set_rotation(&mut self, q)` | Quat | - | 与上述一致 |
| 81 | 设置缩放 | `Transform3D::set_scale(&mut self, v: Vec3)` | Vec3 | - | 直接设置缩放 |
| 107 | 设置缩放（详细） | `Transform3D::set_scale(&mut self, v)` | Vec3 | - | 与上述一致 |
| 348 | 设置缩放（详细） | `Transform3D::set_scale(&mut self, v)` | Vec3 | - | 与上述一致 |

### 4.5.2 增量操作

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 82 | 平移增量 | `Transform3D::translate(&mut self, v: Vec3)` | Vec3 | - | 位置增加 v |
| 108 | 平移增量（详细） | `Transform3D::translate(&mut self, v)` | Vec3 | - | 与上述一致 |
| 349 | 平移增量（详细） | `Transform3D::translate(&mut self, v)` | Vec3 | - | 与上述一致 |
| 83 | 旋转增量 | `Transform3D::rotate(&mut self, q: Quat)` | Quat | - | 旋转乘以 q |
| 109 | 旋转增量（详细） | `Transform3D::rotate(&mut self, q)` | Quat | - | 与上述一致 |
| 350 | 旋转增量（详细） | `Transform3D::rotate(&mut self, q)` | Quat | - | 与上述一致 |
| 84 | 缩放增量 | `Transform3D::scale_by(&mut self, v: Vec3)` | Vec3 | - | 缩放分量乘以 v |
| 110 | 缩放增量（详细） | `Transform3D::scale_by(&mut self, v)` | Vec3 | - | 与上述一致 |
| 351 | 缩放增量（详细） | `Transform3D::scale_by(&mut self, v)` | Vec3 | - | 与上述一致 |

**优先级**：P0

---

## 4.6 变换操作

### 4.6.1 观察目标

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 85 | 看向目标 | `Transform3D::look_at(&mut self, target: Vec3, up: Vec3)` | Vec3, Vec3 | - | 计算旋转使 -Z 指向 target |
| 111 | 看向目标（详细） | `Transform3D::look_at(&mut self, target, up)` | Vec3, Vec3 | - | 与上述一致 |
| 352 | 看向目标（详细） | `Transform3D::look_at(&mut self, target, up)` | Vec3, Vec3 | - | 与上述一致 |

### 4.6.2 插值

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 86 | 线性插值 | `Transform3D::lerp(a: &Transform3D, b: &Transform3D, t: f32) -> Transform3D` | &Transform3D, &Transform3D, f32 | Transform3D | 位置和缩放线性插值，旋转使用 slerp |
| 112 | 线性插值（详细） | `Transform3D::lerp(a, b, t)` | &Transform3D, &Transform3D, f32 | Transform3D | 与上述一致 |
| 353 | 线性插值（详细） | `Transform3D::lerp(a, b, t)` | &Transform3D, &Transform3D, f32 | Transform3D | 与上述一致 |

### 4.6.3 空间变换

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 87 | 变换点 | `Transform3D::transform_point(&self, p: Vec3) -> Vec3` | Vec3 | Vec3 | 点乘以矩阵（包含平移） |
| 113 | 变换点（详细） | `Transform3D::transform_point(&self, p) -> Vec3` | Vec3 | Vec3 | 与上述一致 |
| 354 | 变换点（详细） | `Transform3D::transform_point(&self, p) -> Vec3` | Vec3 | Vec3 | 与上述一致 |
| 88 | 变换向量 | `Transform3D::transform_vector(&self, v: Vec3) -> Vec3` | Vec3 | Vec3 | 向量乘以矩阵（不包含平移） |
| 114 | 变换向量（详细） | `Transform3D::transform_vector(&self, v) -> Vec3` | Vec3 | Vec3 | 与上述一致 |
| 355 | 变换向量（详细） | `Transform3D::transform_vector(&self, v) -> Vec3` | Vec3 | Vec3 | 与上述一致 |
| 89 | 变换方向 | `Transform3D::transform_direction(&self, v: Vec3) -> Vec3` | Vec3 | Vec3 | 仅旋转（无缩放影响） |
| 115 | 变换方向（详细） | `Transform3D::transform_direction(&self, v) -> Vec3` | Vec3 | Vec3 | 与上述一致 |
| 356 | 变换方向（详细） | `Transform3D::transform_direction(&self, v) -> Vec3` | Vec3 | Vec3 | 与上述一致 |

**优先级**：P0

---

## 4.7 Node3D 场景节点

### 4.7.1 节点构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 102 | Node3D 结构 | Node3D：name / parent / children / transform / mesh / material / visible | - | - | 场景图节点 |
| 358 | 创建节点 | `Node3D::new() -> Self` | - | Self | 创建空节点 |
| 358 | 创建节点（详细） | `Node3D::new() -> Self` | - | Self | 与上述一致 |
| 359 | 带名称创建 | `Node3D::with_name(name: String) -> Self` | String | Self | 创建带名称的节点 |
| 359 | 带名称创建（详细） | `Node3D::with_name(name) -> Self` | String | Self | 与上述一致 |
| 360 | 带网格创建 | `Node3D::with_mesh(handle: Handle<Mesh3D>) -> Self` | Handle<Mesh3D> | Self | 创建带网格的节点 |
| 360 | 带网格创建（详细） | `Node3D::with_mesh(handle) -> Self` | Handle<Mesh3D> | Self | 与上述一致 |

### 4.7.2 节点属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 128 | 节点属性 | Node3D：name / parent / children / transform / mesh / material / visible | - | - | 完整属性列表 |
| 361 | 获取名称 | `Node3D::name(&self) -> &str` | - | &str | 返回节点名称 |
| 405 | 名称（详细） | `Node3D::name(&self) -> &str` | - | &str | 与上述一致 |
| 362 | 获取父节点 | `Node3D::parent(&self) -> Option<NodeHandle>` | - | Option<NodeHandle> | 返回父节点句柄 |
| 406 | 父节点（详细） | `Node3D::parent(&self) -> Option<NodeHandle>` | - | Option<NodeHandle> | 与上述一致 |
| 363 | 获取子节点 | `Node3D::children(&self) -> &[NodeHandle]` | - | &[NodeHandle] | 返回子节点列表 |
| 407 | 子节点（详细） | `Node3D::children(&self) -> &[NodeHandle]` | - | &[NodeHandle] | 与上述一致 |
| 364 | 获取本地变换 | `Node3D::local_transform(&self) -> &Transform3D` | - | &Transform3D | 返回本地变换 |
| 408 | 本地变换（详细） | `Node3D::local_transform(&self) -> &Transform3D` | - | &Transform3D | 与上述一致 |
| 365 | 获取世界变换 | `Node3D::world_transform(&self) -> &Transform3D` | - | &Transform3D | 返回组合后的世界变换 |
| 409 | 世界变换（详细） | `Node3D::world_transform(&self) -> &Transform3D` | - | &Transform3D | 与上述一致 |
| 366 | 获取世界 AABB | `Node3D::aabb(&self) -> AABB` | - | AABB | 返回世界空间包围盒 |
| 410 | 世界 AABB（详细） | `Node3D::aabb(&self) -> AABB` | - | AABB | 与上述一致 |
| 367 | 获取可见性 | `Node3D::visible(&self) -> bool` | - | bool | 返回节点可见性 |
| 411 | 可见性（详细） | `Node3D::visible(&self) -> bool` | - | bool | 与上述一致 |
| 368 | 设置可见性 | `Node3D::set_visible(&mut self, visible: bool)` | bool | - | 设置节点可见性 |
| 412 | 设置可见性（详细） | `Node3D::set_visible(&mut self, visible)` | bool | - | 与上述一致 |
| 369 | 获取网格句柄 | `Node3D::mesh(&self) -> Option<Handle<Mesh3D>>` | - | Option<Handle<Mesh3D>> | 返回挂载的网格 |
| 413 | 网格句柄（详细） | `Node3D::mesh(&self) -> Option<Handle<Mesh3D>>` | - | Option<Handle<Mesh3D>> | 与上述一致 |
| 370 | 获取材质句柄 | `Node3D::material(&self) -> Option<Handle<Material3D>>` | - | Option<Handle<Material3D>> | 返回挂载的材质 |
| 414 | 材质句柄（详细） | `Node3D::material(&self) -> Option<Handle<Material3D>>` | - | Option<Handle<Material3D>> | 与上述一致 |

**优先级**：P0

---

## 4.8 Scene3D 场景图

### 4.8.1 场景构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 90 | Scene3D 结构 | Scene3D：节点树 + 渲染实体列表 | - | - | 管理场景所有节点 |
| 371 | 创建场景 | `Scene3D::new()` | - | Self | 创建空场景 |
| 415 | 创建场景（详细） | `Scene3D::new()` | - | Self | 与上述一致 |

### 4.8.2 节点管理

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 91 | 添加节点 | `Scene3D::add_node(&mut self, node: Node3D) -> NodeHandle` | Node3D | NodeHandle | 返回节点句柄 |
| 117 | 添加节点（详细） | `Scene3D::add_node(&mut self, node) -> NodeHandle` | Node3D | NodeHandle | 与上述一致 |
| 372 | 添加节点（详细） | `Scene3D::add_node(&mut self, node) -> NodeHandle` | Node3D | NodeHandle | 与上述一致 |
| 92 | 移除节点 | `Scene3D::remove_node(&mut self, handle: NodeHandle)` | NodeHandle | - | 从场景移除节点 |
| 118 | 移除节点（详细） | `Scene3D::remove_node(&mut self, handle)` | NodeHandle | - | 与上述一致 |
| 373 | 移除节点（详细） | `Scene3D::remove_node(&mut self, handle)` | NodeHandle | - | 与上述一致 |
| 93 | 获取节点（不可变） | `Scene3D::node(&self, handle: NodeHandle) -> &Node3D` | NodeHandle | &Node3D | 返回节点引用 |
| 119 | 获取节点（详细） | `Scene3D::node(&self, handle) -> &Node3D` | NodeHandle | &Node3D | 与上述一致 |
| 374 | 获取节点（详细） | `Scene3D::node(&self, handle) -> &Node3D` | NodeHandle | &Node3D | 与上述一致 |
| 94 | 获取节点（可变） | `Scene3D::node_mut(&mut self, handle: NodeHandle) -> &mut Node3D` | NodeHandle | &mut Node3D | 返回节点可变引用 |
| 120 | 获取节点可变（详细） | `Scene3D::node_mut(&mut self, handle) -> &mut Node3D` | NodeHandle | &mut Node3D | 与上述一致 |
| 375 | 获取节点可变（详细） | `Scene3D::node_mut(&mut self, handle) -> &mut Node3D` | NodeHandle | &mut Node3D | 与上述一致 |
| 95 | 获取所有节点 | `Scene3D::nodes(&self) -> &[Node3D]` | - | &[Node3D] | 返回节点列表 |
| 121 | 所有节点（详细） | `Scene3D::nodes(&self) -> &[Node3D]` | - | &[Node3D] | 与上述一致 |
| 376 | 所有节点（详细） | `Scene3D::nodes(&self) -> &[Node3D]` | - | &[Node3D] | 与上述一致 |
| 377 | 获取根节点 | `Scene3D::root_nodes(&self) -> Vec<NodeHandle>` | - | Vec<NodeHandle> | 返回无父节点的节点 |

### 4.8.3 相机管理

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 97 | 获取主相机 | `Scene3D::main_camera(&self) -> Option<&Camera3D>` | - | Option<&Camera3D> | 返回主相机引用 |
| 123 | 主相机（详细） | `Scene3D::main_camera(&self) -> Option<&Camera3D>` | - | Option<&Camera3D> | 与上述一致 |
| 378 | 主相机（详细） | `Scene3D::main_camera(&self) -> Option<&Camera3D>` | - | Option<&Camera3D> | 与上述一致 |
| 98 | 设置主相机 | `Scene3D::set_main_camera(&mut self, handle: NodeHandle)` | NodeHandle | - | 指定主相机节点 |
| 124 | 设置主相机（详细） | `Scene3D::set_main_camera(&mut self, handle)` | NodeHandle | - | 与上述一致 |
| 379 | 设置主相机（详细） | `Scene3D::set_main_camera(&mut self, handle)` | NodeHandle | - | 与上述一致 |

### 4.8.4 场景更新

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 99 | 更新世界变换 | `Scene3D::update_world_transforms(&mut self)` | - | - | 从根到叶传播变换 |
| 125 | 更新世界变换（详细） | `Scene3D::update_world_transforms(&mut self)` | - | - | 与上述一致 |
| 380 | 更新世界变换（详细） | `Scene3D::update_world_transforms(&mut self)` | - | - | 与上述一致 |
| 426 | 更新世界变换（详细） | `Scene3D::update_world_transforms(&mut self)` | - | - | 与上述一致 |

### 4.8.5 视锥裁剪

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 100 | 视锥裁剪 | `Scene3D::cull(&mut self, frustum: &Frustum)` | &Frustum | - | 标记可见节点 |
| 126 | 视锥裁剪（详细） | `Scene3D::cull(&mut self, frustum)` | &Frustum | - | 与上述一致 |
| 381 | 视锥裁剪（详细） | `Scene3D::cull(&mut self, frustum)` | &Frustum | - | 与上述一致 |
| 425 | 视锥裁剪（详细） | `Scene3D::cull(&mut self, frustum)` | &Frustum | - | 与上述一致 |

### 4.8.6 可见实体

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 101 | 获取可见实体列表 | `Scene3D::visible_entities(&self) -> &[RenderEntity3D]` | - | &[RenderEntity3D] | 返回裁剪后的实体 |
| 127 | 可见实体（详细） | `Scene3D::visible_entities(&self) -> &[RenderEntity3D]` | - | &[RenderEntity3D] | 与上述一致 |
| 382 | 可见实体（详细） | `Scene3D::visible_entities(&self) -> &[RenderEntity3D]` | - | &[RenderEntity3D | 与上述一致 |
| 426 | 可见实体（详细） | `Scene3D::visible_entities(&self) -> &[RenderEntity3D]` | - | &[RenderEntity3D] | 与上述一致 |

### 4.8.7 场景统计

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 383 | 获取场景统计 | `Scene3D::stats(&self) -> &SceneStats3D` | - | &SceneStats3D | 返回统计信息 |
| 384 | 统计结构 | SceneStats3D：nodes / visible_nodes / total_triangles | - | - | 场景统计信息 |
| 428 | 统计信息（详细） | `SceneStats3D::nodes / visible_nodes / total_triangles` | - | - | 与上述一致 |

### 4.8.8 RenderEntity3D

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 105 | 渲染实体结构 | RenderEntity3D：mesh handle + material handle + world matrix | - | - | 传递给渲染管线 |
| 429 | 渲染实体（详细） | `RenderEntity3D::mesh / material / world_matrix` | - | - | 包含绘制所需信息 |

**优先级**：P0

---

## 4.9 依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                      engine-math                         │
│                    (Vec3, Mat4, Quat)                    │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                  engine-render-3d                        │
│                  (Transform3D / Scene3D)               │
└─────────────────────────────────────────────────────────┘
          │                       │
          ▼                       ▼
┌─────────────────┐     ┌─────────────────┐
│    Mesh3D       │     │   Camera3D       │
│  挂载到节点      │     │   用于视锥裁剪   │
└─────────────────┘     └─────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────┐
│                  RenderPipeline3D                        │
│               使用 Scene3D 可见实体列表                   │
└─────────────────────────────────────────────────────────┘
```

**上游依赖**：
- `engine-math`：数学类型

**下游依赖**：
- `RenderPipeline3D`：使用场景节点和可见实体
- `DebugRenderer3D`：渲染变换轴心

---

## 4.10 验收标准

### 4.10.1 功能验收

- [ ] `Transform3D::matrix()` 返回 T*R*S 顺序的矩阵
- [ ] `Transform3D::inverse_matrix()` 与 `matrix()` 相乘等于单位矩阵
- [ ] `Transform3D::transform_point()` 正确处理点的变换
- [ ] `Transform3D::transform_direction()` 不包含平移影响
- [ ] `Scene3D::update_world_transforms()` 子节点世界矩阵 = 父世界矩阵 × 子本地矩阵
- [ ] `Scene3D::cull()` 正确标记视锥内和视锥外的节点
- [ ] `Scene3D::visible_entities()` 返回所有可见且在视锥内的实体

### 4.10.2 单元测试

| 测试项 | 需求ID | 验证内容 |
|--------|--------|----------|
| Transform3D 矩阵与 inverse 乘积 | 204, 230 | matrix * inverse_matrix ≈ I |
| Scene3D::update_world_transforms | 209, 235 | 子节点世界矩阵正确 |

---

## 4.11 优先级汇总

| 优先级 | 需求ID | 占比 |
|--------|--------|------|
| P0 | 69-115, 128-131, 337-429 | 95% |
| P1 | - | 5% |
| P2 | - | 0% |

**P0 核心**：Transform3D 所有方法、Node3D 所有属性、Scene3D 所有方法
**P1 重要**：Transform 插值、场景统计
**P2 可选**：高级节点层次功能