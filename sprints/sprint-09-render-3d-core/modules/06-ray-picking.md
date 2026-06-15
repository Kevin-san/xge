# 模块六：Ray 拾取与几何需求

## 6.1 模块概述

本模块定义了 3D 渲染系统中的射线（Ray3）拾取功能与几何结构（AABB、Sphere、Plane）。射线拾取用于鼠标点击选中 3D 场景中的物体；几何结构用于碰撞检测和视锥裁剪。

**对应原需求编号**：158-200, 414-490

**核心依赖**：
- `engine-math`：Vec3、Mat4 等数学类型
- `Mesh3D`：射线检测的目标网格

---

## 6.2 Ray3 射线结构

### 6.2.1 射线构造与属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 158 | 射线结构 | Ray3：origin + direction | - | - | 无限长射线 |
| 184 | 射线结构（详细） | Ray3：origin + direction | - | - | 与上述一致 |
| 414 | 创建射线 | `Ray3::new(origin: Vec3, direction: Vec3) -> Self` | Vec3, Vec3 | Self | direction 应为单位向量 |
| 464 | 创建射线（详细） | `Ray3::new(origin, direction)` | Vec3, Vec3 | Self | 与上述一致 |
| 159 | 射线点计算 | `Ray3::at(&self, t: f32) -> Vec3` | f32 | Vec3 | 返回 origin + direction * t |
| 185 | 射线点计算（详细） | `Ray3::at(&self, t) -> Vec3` | f32 | Vec3 | 与上述一致 |
| 415 | 射线点计算（详细） | `Ray3::at(&self, t) -> Vec3` | f32 | Vec3 | 与上述一致 |

**优先级**：P0

---

## 6.3 Ray3 命中检测

### 6.3.1 AABB 命中

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 160 | 射线命中 AABB | `Ray3::hit_aabb(&self, aabb: &AABB) -> Option<f32>`（slab method） | &AABB | Option<f32> | 返回命中距离 t |
| 186 | 射线命中 AABB（详细） | `Ray3::hit_aabb(&self, aabb) -> Option<f32>` | &AABB | Option<f32> | 与上述一致 |
| 416 | 射线命中 AABB（详细） | `Ray3::hit_aabb(&self, aabb) -> Option<f32>` | &AABB | Option<f32> | 与上述一致 |
| 466 | 射线命中 AABB（详细） | `Ray3::hit_aabb(&self, aabb) -> Option<f32>` | &AABB | Option<f32> | 与上述一致 |

### 6.3.2 球体命中

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 161 | 射线命中球体 | `Ray3::hit_sphere(&self, sphere: &Sphere) -> Option<f32>` | &Sphere | Option<f32> | 返回命中距离 t |
| 187 | 射线命中球体（详细） | `Ray3::hit_sphere(&self, sphere) -> Option<f32>` | &Sphere | Option<f32> | 与上述一致 |
| 417 | 射线命中球体（详细） | `Ray3::hit_sphere(&self, sphere) -> Option<f32>` | &Sphere | Option<f32> | 与上述一致 |
| 467 | 射线命中球体（详细） | `Ray3::hit_sphere(&self, sphere) -> Option<f32>` | &Sphere | Option<f32> | 与上述一致 |

### 6.3.3 三角形命中

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 162 | 射线命中三角形 | `Ray3::hit_triangle(&self, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32>`（Möller–Trumbore） | Vec3, Vec3, Vec3 | Option<f32> | 返回命中距离 t |
| 188 | 射线命中三角形（详细） | `Ray3::hit_triangle(&self, v0, v1, v2) -> Option<f32>` | Vec3, Vec3, Vec3 | Option<f32> | 与上述一致 |
| 418 | 射线命中三角形（详细） | `Ray3::hit_triangle(&self, v0, v1, v2) -> Option<f32>` | Vec3, Vec3, Vec3 | Option<f32> | 与上述一致 |
| 468 | 射线命中三角形（详细） | `Ray3::hit_triangle(&self, v0, v1, v2) -> Option<f32>` | Vec3, Vec3, Vec3 | Option<f32> | Möller–Trumbore 算法 |

### 6.3.4 平面命中

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 419 | 射线命中平面 | `Ray3::hit_plane(&self, plane: &Plane) -> Option<f32>` | &Plane | Option<f32> | 返回命中距离 t |
| 469 | 射线命中平面（详细） | `Ray3::hit_plane(&self, plane) -> Option<f32>` | &Plane | Option<f32> | 与上述一致 |

### 6.3.5 网格命中

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 163 | 射线命中网格 | `Ray3::hit_mesh(&self, mesh: &Mesh3D, transform: &Transform3D) -> Option<HitResult>` | &Mesh3D, &Transform3D | Option<HitResult> | 返回完整命中结果 |
| 189 | 射线命中网格（详细） | `Ray3::hit_mesh(&self, mesh, transform) -> Option<HitResult>` | &Mesh3D, &Transform3D | Option<HitResult> | 与上述一致 |
| 420 | 射线命中网格（详细） | `Ray3::hit_mesh(&self, mesh, transform) -> Option<HitResult>` | &Mesh3D, &Transform3D | Option<HitResult> | 与上述一致 |
| 470 | 射线命中网格（详细） | `Ray3::hit_mesh(&self, mesh, transform) -> Option<HitResult>` | &Mesh3D, &Transform3D | Option<HitResult> | 与上述一致 |

**优先级**：P0

---

## 6.4 HitResult 命中结果

### 6.4.1 命中信息

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 164 | 命中结果结构 | HitResult：t, point, normal, uv, mesh, primitive_index | - | - | 包含完整命中信息 |
| 190 | 命中结果（详细） | HitResult：t, point, normal, uv, mesh, primitive_index | - | - | 与上述一致 |
| 421 | 命中结果属性 | `HitResult::t / point / normal / uv / primitive_index` | - | - | 各项命中数据 |
| 471 | 命中结果属性（详细） | `HitResult::t / point / normal / uv / primitive_index` | - | - | 与上述一致 |

**优先级**：P0

---

## 6.5 PickResult 拾取结果

### 6.5.1 拾取结果

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 165 | 拾取结果结构 | PickResult：实体集合，可按 t 排序取最近 | - | - | 包含多个命中结果 |

**优先级**：P0

---

## 6.6 AABB 包围盒

### 6.6.1 AABB 构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 170 | AABB 结构 | AABB：`min/max`、`merge(other)`、`transform_by(mat)` | - | - | 轴对齐包围盒 |
| 196 | AABB 结构（详细） | AABB：`min/max`、`merge(other)`、`transform_by(mat)` | - | - | 与上述一致 |
| 472 | 创建 AABB | `AABB::new(min: Vec3, max: Vec3) -> Self` | Vec3, Vec3 | Self | min <= max |
| 422 | 创建 AABB（详细） | `AABB::new(min, max)` | Vec3, Vec3 | Self | 与上述一致 |
| 473 | 从点创建 AABB | `AABB::from_points(points: &[Vec3]) -> Self` | &[Vec3] | Self | 从点集创建最小包围盒 |
| 423 | 从点创建（详细） | `AABB::from_points(points)` | &[Vec3] | Self | 与上述一致 |

### 6.6.2 AABB 属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 424 | 获取最小点 | `AABB::min(&self) -> Vec3` | - | Vec3 | 返回 min 角点 |
| 474 | 最小点（详细） | `AABB::min(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 425 | 获取最大点 | `AABB::max(&self) -> Vec3` | - | Vec3 | 返回 max 角点 |
| 475 | 最大点（详细） | `AABB::max(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 426 | 获取中心点 | `AABB::center(&self) -> Vec3` | - | Vec3 | 返回 (min + max) / 2 |
| 476 | 中心点（详细） | `AABB::center(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 427 | 获取半扩展 | `AABB::half_extents(&self) -> Vec3` | - | Vec3 | 返回 (max - min) / 2 |
| 477 | 半扩展（详细） | `AABB::half_extents(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 428 | 获取尺寸 | `AABB::size(&self) -> Vec3` | - | Vec3 | 返回 max - min |
| 478 | 尺寸（详细） | `AABB::size(&self) -> Vec3` | - | Vec3 | 与上述一致 |

### 6.6.3 AABB 检测

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 479 | 点包含检测 | `AABB::contains_point(&self, p: Vec3) -> bool` | Vec3 | bool | 点在 AABB 内返回 true |
| 429 | 点包含（详细） | `AABB::contains_point(&self, p) -> bool` | Vec3 | bool | 与上述一致 |
| 480 | AABB 相交检测 | `AABB::intersects_aabb(&self, other: &AABB) -> bool` | &AABB | bool | 两 AABB 相交返回 true |
| 430 | AABB 相交（详细） | `AABB::intersects_aabb(&self, other) -> bool` | &AABB | bool | 与上述一致 |

### 6.6.4 AABB 操作

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 481 | 合并 AABB | `AABB::merge(&self, other: &AABB) -> AABB` | &AABB | AABB | 返回包含两个 AABB 的最小 AABB |
| 431 | 合并（详细） | `AABB::merge(&self, other) -> AABB` | &AABB | AABB | 与上述一致 |
| 482 | 变换 AABB | `AABB::transform_by(&self, mat: Mat4) -> AABB` | Mat4 | AABB | 返回变换后的 AABB（可能更大） |
| 432 | 变换（详细） | `AABB::transform_by(&self, mat) -> AABB` | Mat4 | AABB | 与上述一致 |

**优先级**：P0

---

## 6.7 Sphere 包围球

### 6.7.1 Sphere 构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 171 | Sphere 结构 | Sphere：`center/radius`、`merge(other)` | - | - | 包围球结构 |
| 197 | Sphere 结构（详细） | Sphere：`center/radius`、`merge(other)` | - | - | 与上述一致 |
| 483 | 创建球体 | `Sphere::new(center: Vec3, radius: f32) -> Self` | Vec3, f32 | Self | radius >= 0 |
| 433 | 创建球体（详细） | `Sphere::new(center, radius)` | Vec3, f32 | Self | 与上述一致 |

### 6.7.2 Sphere 检测

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 484 | 点包含检测 | `Sphere::contains_point(&self, p: Vec3) -> bool` | Vec3 | bool | 点在球内返回 true |
| 434 | 点包含（详细） | `Sphere::contains_point(&self, p) -> bool` | Vec3 | bool | 与上述一致 |
| 485 | 球体相交检测 | `Sphere::intersects_sphere(&self, other: &Sphere) -> bool` | &Sphere | bool | 两球相交返回 true |
| 435 | 球体相交（详细） | `Sphere::intersects_sphere(&self, other) -> bool` | &Sphere | bool | 与上述一致 |

### 6.7.3 Sphere 操作

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 486 | 合并球体 | `Sphere::merge(&self, other: &Sphere) -> Sphere` | &Sphere | Sphere | 返回包含两个球的最小球 |
| 436 | 合并（详细） | `Sphere::merge(&self, other) -> Sphere` | &Sphere | Sphere | 与上述一致 |

**优先级**：P0

---

## 6.8 Plane 平面

### 6.8.1 Plane 操作

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 172 | Plane 结构 | Plane：`normal + d` | - | - | 平面方程 normal*x + d = 0 |
| 198 | Plane 结构（详细） | Plane：`normal + d` | - | - | 与上述一致 |
| 487 | 从法线和点创建 | `Plane::from_normal_and_point(normal: Vec3, point: Vec3) -> Self` | Vec3, Vec3 | Self | d = -dot(normal, point) |
| 437 | 从法线和点（详细） | `Plane::from_normal_and_point(normal, point)` | Vec3, Vec3 | Self | 与上述一致 |
| 488 | 点到平面距离 | `Plane::distance(&self, p: Vec3) -> f32` | Vec3 | f32 | 返回 signed distance |
| 438 | 距离计算（详细） | `Plane::distance(&self, p) -> f32` | Vec3 | f32 | 与上述一致 |
| 489 | 归一化平面 | `Plane::normalize(&mut self)` | - | - | 使法线长度为 1 |
| 439 | 归一化（详细） | `Plane::normalize(&mut self)` | - | - | 与上述一致 |

**优先级**：P0

---

## 6.9 依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                      engine-math                         │
│                    (Vec3, Mat4, Quat)                    │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                  engine-render-3d                        │
│                  (Ray3, AABB, Sphere, Plane)            │
└─────────────────────────────────────────────────────────┘
          │                       │
          ▼                       ▼
┌─────────────────┐     ┌─────────────────┐
│    Mesh3D       │     │   Camera3D       │
│  射线检测目标    │     │   生成屏幕射线   │
└─────────────────┘     └─────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────┐
│                   Scene3D / examples                     │
│                   拾取实现                               │
└─────────────────────────────────────────────────────────┘
```

**上游依赖**：
- `engine-math`：数学类型

**下游依赖**：
- `Scene3D`：视锥裁剪使用 AABB/Sphere
- `examples/3d_picker`：鼠标拾取示例

---

## 6.10 验收标准

### 6.10.1 功能验收

- [ ] `Ray3::hit_aabb()` 使用 slab method 正确检测
- [ ] `Ray3::hit_sphere()` 正确计算射线与球体交点
- [ ] `Ray3::hit_triangle()` 使用 Möller–Trumbore 算法
- [ ] `Ray3::hit_mesh()` 遍历所有三角形返回最近的命中
- [ ] `AABB::transform_by()` 返回正确包含变换后顶点的 AABB

### 6.10.2 单元测试

| 测试项 | 需求ID | 验证内容 |
|--------|--------|----------|
| Ray3::hit_aabb | 203, 229 | 射线与 AABB 交点计算正确 |
| Ray3::hit_sphere | 203, 229 | 射线与球体交点计算正确 |
| Ray3::hit_triangle | 203, 229 | Möller–Trumbore 算法正确 |
| AABB::transform_by | 205, 231 | 变换后 AABB 包含所有顶点 |

### 6.10.3 示例验收

- [ ] `examples/3d_picker` 点击可命中实体

---

## 6.11 优先级汇总

| 优先级 | 需求ID | 占比 |
|--------|--------|------|
| P0 | 158-165, 184-190, 414-421, 464-471, 170-172, 196-198, 422-439, 472-489 | 90% |
| P1 | - | 10% |
| P2 | - | 0% |

**P0 核心**：Ray3 所有方法、AABB/Sphere/Plane 所有方法
**P1 重要**：HitResult/PickResult
**P2 可选**：高级几何算法