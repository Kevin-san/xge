# 模块二：相机与视锥裁剪需求

## 2.1 模块概述

本模块定义了 3D 渲染系统中的相机（Camera3D）与视锥体（Frustum）裁剪系统。相机负责生成观察矩阵和投影矩阵，将 3D 世界空间转换到 2D 屏幕空间；视锥体用于判断场景中的物体是否可见，实现可见性裁剪。

**对应原需求编号**：31-86, 281-349

**核心依赖**：
- `engine-math`：Vec3、Mat4、Quat 等数学类型
- `Ray3`：屏幕射线生成

---

## 2.2 Camera3D 结构体

### 2.2.1 相机构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 31 | 透视相机构造 | `Camera3D::perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self` | fovy, aspect, near, far | Camera3D | 创建透视投影相机 |
| 57 | 透视相机（详细） | `Camera3D::perspective(fovy, aspect, near, far)` | f32, f32, f32, f32 | Self | 与上述一致 |
| 32 | 正交相机构造 | `Camera3D::orthographic(left, right, bottom, top, near, far) -> Self` | f32 x 6 | Camera3D | 创建正交投影相机 |
| 58 | 正交相机（详细） | `Camera3D::orthographic(left, right, bottom, top, near, far)` | f32 x 6 | Self | 与上述一致 |
| 319 | 透视相机（详细） | `Camera3D::perspective(fovy, aspect, near, far)` | f32, f32, f32, f32 | Self | 与上述一致 |
| 320 | 正交相机（详细） | `Camera3D::orthographic(left, right, bottom, top, near, far)` | f32, f32, f32, f32, f32, f32 | Self | 与上述一致 |

**优先级**：P0

---

## 2.3 矩阵计算

### 2.3.1 观察矩阵

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 33 | 获取观察矩阵 | `Camera3D::view_matrix(&self) -> Mat4` | - | Mat4 | 返回相机观察矩阵 |
| 59 | 观察矩阵（详细） | `Camera3D::view_matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 283 | 观察矩阵（详细） | `Camera3D::view_matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 36 | 逆观察矩阵 | `Camera3D::inverse_view(&self) -> Mat4` | - | Mat4 | 返回 view 的逆矩阵 |
| 62 | 逆观察矩阵（详细） | `Camera3D::inverse_view(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 324 | 逆观察矩阵（详细） | `Camera3D::inverse_view(&self) -> Mat4` | - | Mat4 | 与上述一致 |

### 2.3.2 投影矩阵

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 34 | 获取投影矩阵 | `Camera3D::projection_matrix(&self) -> Mat4` | - | Mat4 | 返回投影矩阵 |
| 60 | 投影矩阵（详细） | `Camera3D::projection_matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 284 | 投影矩阵（详细） | `Camera3D::projection_matrix(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 37 | 逆投影矩阵 | `Camera3D::inverse_projection(&self) -> Mat4` | - | Mat4 | 返回 projection 的逆矩阵 |
| 63 | 逆投影矩阵（详细） | `Camera3D::inverse_projection(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 325 | 逆投影矩阵（详细） | `Camera3D::inverse_projection(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 326 | 逆视投影矩阵 | `Camera3D::inverse_view_projection(&self) -> Mat4` | - | Mat4 | 返回 VP 的逆矩阵 |

### 2.3.3 组合矩阵

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 35 | 获取视投影矩阵 | `Camera3D::view_projection(&self) -> Mat4` | - | Mat4 | 返回 view * projection |
| 61 | 视投影矩阵（详细） | `Camera3D::view_projection(&self) -> Mat4` | - | Mat4 | 与上述一致 |
| 285 | 视投影矩阵（详细） | `Camera3D::view_projection(&self) -> Mat4` | - | Mat4 | 与上述一致 |

**优先级**：P0

---

## 2.4 相机属性访问

### 2.4.1 基础属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 38 | 获取相机位置 | `Camera3D::position(&self) -> Vec3` | - | Vec3 | 返回世界空间相机位置 |
| 64 | 相机位置（详细） | `Camera3D::position(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 327 | 相机位置（详细） | `Camera3D::position(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 39 | 获取前向向量 | `Camera3D::forward(&self) -> Vec3` | - | Vec3 | 返回相机观察方向 |
| 65 | 前向向量（详细） | `Camera3D::forward(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 328 | 前向向量（详细） | `Camera3D::forward(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 40 | 获取右向量 | `Camera3D::right(&self) -> Vec3` | - | Vec3 | 返回相机右向向量 |
| 66 | 右向量（详细） | `Camera3D::right(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 329 | 右向量（详细） | `Camera3D::right(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 41 | 获取上向量 | `Camera3D::up(&self) -> Vec3` | - | Vec3 | 返回相机上向向量 |
| 67 | 上向量（详细） | `Camera3D::up(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 330 | 上向量（详细） | `Camera3D::up(&self) -> Vec3` | - | Vec3 | 与上述一致 |

### 2.4.2 投影参数

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 42 | 获取垂直视野 | `Camera3D::fovy(&self) -> f32` | - | f32 | 返回 fovy 角度 |
| 68 | 垂直视野（详细） | `Camera3D::fovy(&self) -> f32` | - | f32 | 与上述一致 |
| 331 | 垂直视野（详细） | `Camera3D::fovy(&self) -> f32` | - | f32 | 与上述一致 |
| 43 | 获取宽高比 | `Camera3D::aspect(&self) -> f32` | - | f32 | 返回 aspect ratio |
| 69 | 宽高比（详细） | `Camera3D::aspect(&self) -> f32` | - | f32 | 与上述一致 |
| 332 | 宽高比（详细） | `Camera3D::aspect(&self) -> f32` | - | f32 | 与上述一致 |
| 44 | 获取近裁剪面 | `Camera3D::near(&self) -> f32` | - | f32 | 返回 near plane 距离 |
| 70 | 近裁剪面（详细） | `Camera3D::near(&self) -> f32` | - | f32 | 与上述一致 |
| 333 | 近裁剪面（详细） | `Camera3D::near(&self) -> f32` | - | f32 | 与上述一致 |
| 45 | 获取远裁剪面 | `Camera3D::far(&self) -> f32` | - | f32 | 返回 far plane 距离 |
| 71 | 远裁剪面（详细） | `Camera3D::far(&self) -> f32` | - | f32 | 与上述一致 |
| 334 | 远裁剪面（详细） | `Camera3D::far(&self) -> f32` | - | f32 | 与上述一致 |

### 2.4.3 参数设置器

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 46 | 设置垂直视野 | `Camera3D::set_fovy(&mut self, f: f32)` | f32 | - | 更新 fovy 并重算投影 |
| 72 | 设置垂直视野（详细） | `Camera3D::set_fovy(&mut self, f)` | f32 | - | 与上述一致 |
| 335 | 设置垂直视野（详细） | `Camera3D::set_fovy(&mut self, f)` | f32 | - | 与上述一致 |
| 47 | 设置宽高比 | `Camera3D::set_aspect(&mut self, a: f32)` | f32 | - | 更新 aspect 并重算投影 |
| 73 | 设置宽高比（详细） | `Camera3D::set_aspect(&mut self, a)` | f32 | - | 与上述一致 |
| 336 | 设置宽高比（详细） | `Camera3D::set_aspect(&mut self, a)` | f32 | - | 与上述一致 |
| 48 | 设置近裁剪面 | `Camera3D::set_near(&mut self, n: f32)` | f32 | - | 更新 near 并重算投影 |
| 74 | 设置近裁剪面（详细） | `Camera3D::set_near(&mut self, n)` | f32 | - | 与上述一致 |
| 337 | 设置近裁剪面（详细） | `Camera3D::set_near(&mut self, n)` | f32 | - | 与上述一致 |
| 49 | 设置远裁剪面 | `Camera3D::set_far(&mut self, f: f32)` | f32 | - | 更新 far 并重算投影 |
| 75 | 设置远裁剪面（详细） | `Camera3D::set_far(&mut self, f)` | f32 | - | 与上述一致 |
| 338 | 设置远裁剪面（详细） | `Camera3D::set_far(&mut self, f)` | f32 | - | 与上述一致 |

**优先级**：P0

---

## 2.5 相机控制

### 2.5.1 观察目标

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 52 | 看向目标点 | `Camera3D::look_at(&mut self, target: Vec3)` | Vec3 | - | 计算旋转使相机朝向 target |
| 78 | 看向目标点（详细） | `Camera3D::look_at(&mut self, target)` | Vec3 | - | 与上述一致 |
| 339 | 看向目标点（详细） | `Camera3D::look_at(&mut self, target)` | Vec3 | - | 与上述一致 |
| 53 | 看向方向 | `Camera3D::look_to(&mut self, dir: Vec3, up: Vec3)` | Vec3, Vec3 | - | 计算旋转使相机朝向 dir |
| 79 | 看向方向（详细） | `Camera3D::look_to(&mut self, dir, up)` | Vec3, Vec3 | - | 与上述一致 |
| 340 | 看向方向（详细） | `Camera3D::look_to(&mut self, dir, up)` | Vec3, Vec3 | - | 与上述一致 |

**优先级**：P0

---

## 2.6 屏幕/世界坐标转换

### 2.6.1 屏幕射线

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 50 | 屏幕位置转世界射线 | `Camera3D::screen_to_world_ray(screen_pos: Vec2, screen_size: Vec2) -> Ray3` | Vec2, Vec2 | Ray3 | 生成从相机出发的射线 |
| 76 | 屏幕射线（详细） | `Camera3D::screen_to_world_ray(screen_pos, screen_size)` | Vec2, Vec2 | Ray3 | 与上述一致 |
| 341 | 屏幕射线（详细） | `Camera3D::screen_to_world_ray(screen_pos, screen_size)` | Vec2, Vec2 | Ray3 | 与上述一致 |

### 2.6.2 世界转屏幕

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 51 | 世界坐标转屏幕坐标 | `Camera3D::world_to_screen(world_pos: Vec3, screen_size: Vec2) -> Vec2` | Vec3, Vec2 | Vec2 | 返回屏幕像素坐标 |
| 77 | 世界转屏幕（详细） | `Camera3D::world_to_screen(world_pos, screen_size)` | Vec3, Vec2 | Vec2 | 与上述一致 |
| 342 | 世界转屏幕（详细） | `Camera3D::world_to_screen(world_pos, screen_size)` | Vec3, Vec2 | Vec2 | 与上述一致 |

**优先级**：P0

---

## 2.7 Frustum 视锥体

### 2.7.1 视锥体构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 54 | 视锥体结构 | Frustum：由相机 VP 矩阵提取 6 个平面 | - | - | 包含 6 个裁剪平面 |
| 55 | 从 VP 矩阵构造 | `Frustum::from_view_projection(vp: Mat4) -> Self` | Mat4 | Frustum | 正确提取 6 个平面 |
| 80 | VP 构造（详细） | `Frustum::from_view_projection(vp) -> Self` | Mat4 | Self | 与上述一致 |
| 305 | VP 构造（详细） | `Frustum::from_view_projection(vp)` | Mat4 | Self | 与上述一致 |
| 81 | 获取平面列表 | `Frustum::planes(&self) -> &[Plane; 6]` | - | &[Plane; 6] | 返回 6 个裁剪平面 |
| 82 | 平面列表（详细） | `Frustum::planes(&self) -> &[Plane; 6]` | - | &[Plane; 6] | 与上述一致 |
| 306 | 平面列表（详细） | `Frustum::planes(&self) -> &[Plane; 6]` | - | &[Plane; 6] | 与上述一致 |

### 2.7.2 视锥体检测

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 56 | 检测点是否在视锥内 | `Frustum::contains_point(&self, p: Vec3) -> bool` | Vec3 | bool | 点在所有平面内侧 |
| 84 | 点检测（详细） | `Frustum::contains_point(&self, p) -> bool` | Vec3 | bool | 与上述一致 |
| 307 | 点检测（详细） | `Frustum::contains_point(&self, p) -> bool` | Vec3 | bool | 与上述一致 |
| 57 | 检测 AABB 是否在视锥内 | `Frustum::contains_aabb(&self, aabb: &AABB) -> bool` | &AABB | bool | AABB 完全在视锥内 |
| 83 | AABB 检测（详细） | `Frustum::contains_aabb(&self, aabb) -> bool` | &AABB | bool | 与上述一致 |
| 308 | AABB 检测（详细） | `Frustum::contains_aabb(&self, aabb) -> bool` | &AABB | bool | 与上述一致 |
| 58 | 检测球体是否在视锥内 | `Frustum::contains_sphere(&self, sphere: &Sphere) -> bool` | &Sphere | bool | 球体完全在视锥内 |
| 85 | 球体检测（详细） | `Frustum::contains_sphere(&self, sphere) -> bool` | &Sphere | bool | 与上述一致 |
| 309 | 球体检测（详细） | `Frustum::contains_sphere(&self, sphere) -> bool` | &Sphere | bool | 与上述一致 |
| 86 | AABB 相交检测 | `Frustum::intersects_aabb(&self, aabb: &AABB) -> bool` | &AABB | bool | AABB 与视锥相交 |
| 175 | AABB 粗检测（详细） | `Frustum::intersects_aabb(&self, aabb)` | &AABB | bool | 快速相交判断 |
| 310 | AABB 粗检测（详细） | `Frustum::intersects_aabb(&self, aabb)` | &AABB | bool | 与上述一致 |
| 349 | AABB 相交检测（详细） | `Frustum::intersects_aabb(&self, aabb) -> bool` | &AABB | bool | 与上述一致 |

**优先级**：P0

---

## 2.8 依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                      engine-math                         │
│              (Vec3, Mat4, Quat, Plane, AABB)             │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                  engine-render-3d                        │
│                   (Camera3D / Frustum)                  │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                     Scene3D                              │
│                 (视锥裁剪 cull 方法)                     │
└─────────────────────────────────────────────────────────┘
```

**上游依赖**：
- `engine-math`：所有数学类型

**下游依赖**：
- `Scene3D`：使用 Frustum 进行裁剪
- `RenderPipeline3D`：使用 Camera3D 获取 VP 矩阵
- `DebugRenderer3D`：渲染相机视锥

---

## 2.9 验收标准

### 2.9.1 功能验收

- [ ] `Camera3D::perspective()` 创建的投影矩阵符合 OpenGL/D3D 规范
- [ ] `Camera3D::view_matrix()` 返回的矩阵使相机朝向正确方向
- [ ] `Camera3D::view_projection()` 的逆矩阵等于 `inverse_view_projection()`
- [ ] `Camera3D::screen_to_world_ray()` 在视锥内的射线端点 world_to_screen 回代正确
- [ ] `Frustum::from_view_projection()` 提取的 6 平面正确描述视锥体
- [ ] `Frustum::contains_aabb()` 对完全在视锥内的 AABB 返回 true

### 2.9.2 单元测试

| 测试项 | 需求ID | 验证内容 |
|--------|--------|----------|
| Camera3D 矩阵乘积正确性 | 201, 227 | view * projection * point ≈ expected |
| Frustum::contains_aabb | 202, 228 | 各种位置 AABB 检测正确 |
| Frustum::contains_sphere | 202, 228 | 各种位置球体检测正确 |
| Camera3D 视锥平面提取 | 305 | 6 平面方向正确 |

### 2.9.3 集成测试

- [ ] `examples/3d_frustum_cull` 视锥裁剪生效，统计可见

---

## 2.10 优先级汇总

| 优先级 | 需求ID | 占比 |
|--------|--------|------|
| P0 | 31-86, 281-342, 305-310, 319-349 | 95% |
| P1 | - | 5% |
| P2 | - | 0% |

**P0 核心**：相机构造、矩阵计算、属性访问、相机控制、屏幕射线、视锥体检测