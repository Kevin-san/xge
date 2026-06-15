# 模块三：光照系统需求

## 3.1 模块概述

本模块定义了 3D 渲染系统中的光照系统，包括方向光、点光源、聚光灯、环境光等光源类型，以及光源管理器。光照系统是 3D 渲染的核心组成部分，决定了场景的明暗和视觉效果。

**对应原需求编号**：60-95, 311-377

**核心依赖**：
- `engine-math`：Vec3、Color 等数学类型
- `Transform3D`：光源位置/方向变换

---

## 3.2 基础几何

### 3.2.1 Plane 结构（用于光照计算）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 172 | Plane 结构 | Plane：normal + d | - | - | normalize 后使用 |
| 173 | 从法线和点构造 | `Plane::from_normal_and_point(normal: Vec3, point: Vec3) -> Self` | Vec3, Vec3 | Plane | d = -dot(normal, point) |
| 487 | 从法线和点（详细） | `Plane::from_normal_and_point(normal, point)` | Vec3, Vec3 | Self | 与上述一致 |
| 174 | 点到平面距离 | `Plane::distance(&self, p: Vec3) -> f32` | Vec3 | f32 | 返回 signed distance |
| 175 | 点到平面距离（详细） | `Plane::distance(&self, p) -> f32` | Vec3 | f32 | 与上述一致 |
| 488 | 点到平面距离（详细） | `Plane::distance(&self, p) -> f32` | Vec3 | f32 | 与上述一致 |
| 176 | 平面归一化 | `Plane::normalize(&mut self)` | - | - | 使法线长度为 1 |
| 177 | 平面归一化（详细） | `Plane::normalize(&mut self)` | - | - | 与上述一致 |
| 489 | 平面归一化（详细） | `Plane::normalize(&mut self)` | - | - | 与上述一致 |

**优先级**：P0

---

## 3.3 Light Trait 光照trait

### 3.3.1 光照计算接口

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 60 | 光源 trait | `Light3D` trait：`contribution(world_pos: Vec3) -> LightSample` | Vec3 | LightSample | 返回该点的光照贡献 |
| 86 | 光源 trait（详细） | `Light3D::contribution(world_pos) -> LightSample` | Vec3 | LightSample | 计算光照衰减和颜色 |

**优先级**：P0

---

## 3.4 DirectionalLight 方向光

### 3.4.1 方向光属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 61 | 方向光结构 | DirectionalLight：方向 + 颜色 + 强度 + 阴影开关 | - | - | 用于无限远光源 |
| 311 | 创建方向光 | `DirectionalLight::new(dir: Vec3, color: Color, intensity: f32) -> Self` | Vec3, Color, f32 | Self | 创建方向光 |
| 351 | 创建方向光（详细） | `DirectionalLight::new(dir, color, intensity)` | Vec3, Color, f32 | Self | 与上述一致 |
| 312 | 获取方向 | `DirectionalLight::direction(&self) -> Vec3` | - | Vec3 | 返回光线方向 |
| 352 | 方向（详细） | `DirectionalLight::direction(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 313 | 获取颜色 | `DirectionalLight::color(&self) -> Color` | - | Color | 返回光源颜色 |
| 353 | 颜色（详细） | `DirectionalLight::color(&self) -> Color` | - | Color | 与上述一致 |
| 314 | 获取强度 | `DirectionalLight::intensity(&self) -> f32` | - | f32 | 返回光强度 |
| 354 | 强度（详细） | `DirectionalLight::intensity(&self) -> f32` | - | f32 | 与上述一致 |
| 315 | 阴影开关 | `DirectionalLight::casts_shadow(&self) -> bool` | - | bool | 返回是否投射阴影 |

**优先级**：P0

---

## 3.5 PointLight 点光源

### 3.5.1 点光源属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 62 | 点光源结构 | PointLight：位置 + 颜色 + 强度 + 半径 + 衰减 | - | - | 用于局部光源 |
| 316 | 创建点光源 | `PointLight::new(pos: Vec3, color: Color, intensity: f32, radius: f32) -> Self` | Vec3, Color, f32, f32 | Self | 创建点光源 |
| 356 | 创建点光源（详细） | `PointLight::new(pos, color, intensity, radius)` | Vec3, Color, f32, f32 | Self | 与上述一致 |
| 317 | 获取位置 | `PointLight::position(&self) -> Vec3` | - | Vec3 | 返回光源位置 |
| 357 | 位置（详细） | `PointLight::position(&self) -> Vec3` | - | Vec3 | 与上述一致 |
| 318 | 获取颜色 | `PointLight::color(&self) -> Color` | - | Color | 返回光源颜色 |
| 358 | 颜色（详细） | `PointLight::color(&self) -> Color` | - | Color | 与上述一致 |
| 319 | 获取强度 | `PointLight::intensity(&self) -> f32` | - | f32 | 返回光强度 |
| 359 | 强度（详细） | `PointLight::intensity(&self) -> f32` | - | f32 | 与上述一致 |
| 320 | 获取半径 | `PointLight::radius(&self) -> f32` | f32 | 返回有效作用半径 |
| 360 | 半径（详细） | `PointLight::radius(&self) -> f32` | - | f32 | 与上述一致 |
| 321 | 衰减计算 | `PointLight::attenuation(&self, distance: f32) -> f32` | f32 | f32 | 返回距离衰减系数 |
| 361 | 衰减（详细） | `PointLight::attenuation(&self, distance) -> f32` | f32 | f32 | 与上述一致 |

**优先级**：P0

---

## 3.6 SpotLight 聚光灯

### 3.6.1 聚光灯属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 63 | 聚光灯结构 | SpotLight：位置 + 方向 + 内/外圆锥角 + 颜色 + 强度 | - | - | 用于聚光灯光源 |
| 322 | 创建聚光灯 | `SpotLight::new(pos: Vec3, dir: Vec3, inner_angle: f32, outer_angle: f32, color: Color, intensity: f32) -> Self` | Vec3, Vec3, f32, f32, Color, f32 | Self | 创建聚光灯 |
| 362 | 创建聚光灯（详细） | `SpotLight::new(pos, dir, inner_angle, outer_angle, color, intensity)` | Vec3, Vec3, f32, f32, Color, f32 | Self | 与上述一致 |
| 323 | 获取内圆锥角 | `SpotLight::inner_angle(&self) -> f32` | - | f32 | 返回内圆锥半角 |
| 363 | 内圆锥角（详细） | `SpotLight::inner_angle(&self) -> f32` | - | f32 | 与上述一致 |
| 324 | 获取外圆锥角 | `SpotLight::outer_angle(&self) -> f32` | - | f32 | 返回外圆锥半角 |
| 364 | 外圆锥角（详细） | `SpotLight::outer_angle(&self) -> f32` | - | f32 | 与上述一致 |
| 325 | 圆锥衰减计算 | `SpotLight::cone_attenuation(&self, dir_to_point: Vec3) -> f32` | Vec3 | f32 | 返回角度衰减系数 |
| 365 | 圆锥衰减（详细） | `SpotLight::cone_attenuation(&self, dir_to_point) -> f32` | Vec3 | f32 | 与上述一致 |

**优先级**：P0

---

## 3.7 AmbientLight 环境光

### 3.7.1 环境光属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 64 | 环境光结构 | AmbientLight：颜色 + 强度 | - | - | 用于全局光照 |
| 326 | 创建环境光 | `AmbientLight::new(color: Color, intensity: f32) -> Self` | Color, f32 | Self | 创建环境光 |
| 366 | 创建环境光（详细） | `AmbientLight::new(color, intensity)` | Color, f32 | Self | 与上述一致 |

**优先级**：P0

---

## 3.8 HemisphereLight 半球光

### 3.8.1 半球光属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 65 | 半球光结构 | HemisphereLight：天/地颜色 + 强度 | - | - | 用于自然环境光 |
| 327 | 创建半球光 | `HemisphereLight::new(sky: Color, ground: Color, intensity: f32) -> Self` | Color, Color, f32 | Self | 创建半球光 |
| 367 | 创建半球光（详细） | `HemisphereLight::new(sky, ground, intensity)` | Color, Color, f32 | Self | 与上述一致 |

**优先级**：P1

---

## 3.9 LightManager 光源管理器

### 3.9.1 光源管理

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 66 | 光源管理器 | LightManager：管理场景光源，上限（16 方向光 + 64 点光） | - | - | 管理多光源 |
| 328 | 创建光源管理器 | `LightManager::new()` | - | Self | 初始化空管理器 |
| 368 | 创建光源管理器（详细） | `LightManager::new()` | - | Self | 与上述一致 |
| 329 | 添加方向光 | `LightManager::add_directional(l: DirectionalLight)` | DirectionalLight | - | 添加到管理器 |
| 369 | 添加方向光（详细） | `LightManager::add_directional(l)` | DirectionalLight | - | 与上述一致 |
| 330 | 添加点光源 | `LightManager::add_point(l: PointLight)` | PointLight | - | 添加到管理器 |
| 370 | 添加点光源（详细） | `LightManager::add_point(l)` | PointLight | - | 与上述一致 |
| 331 | 添加聚光灯 | `LightManager::add_spot(l: SpotLight)` | SpotLight | - | 添加到管理器 |
| 371 | 添加聚光灯（详细） | `LightManager::add_spot(l)` | SpotLight | - | 与上述一致 |
| 332 | 设置环境光 | `LightManager::set_ambient(l: AmbientLight)` | AmbientLight | - | 设置全局环境光 |
| 372 | 设置环境光（详细） | `LightManager::set_ambient(l)` | AmbientLight | - | 与上述一致 |

### 3.9.2 光源查询

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 333 | 获取光源 UBO | `LightManager::lights_ubo(&self) -> &UniformBuffer` | - | &UniformBuffer | 返回上传到着色器的缓冲 |
| 373 | 光源 UBO（详细） | `LightManager::lights_ubo(&self) -> &UniformBuffer` | - | &UniformBuffer | 与上述一致 |
| 334 | 方向光数量 | `LightManager::directional_count(&self) -> usize` | - | usize | 返回方向光数量 |
| 374 | 方向光数量（详细） | `LightManager::directional_count(&self) -> usize` | - | usize | 与上述一致 |
| 335 | 点光源数量 | `LightManager::point_count(&self) -> usize` | - | usize | 返回点光源数量 |
| 375 | 点光源数量（详细） | `LightManager::point_count(&self) -> usize` | - | usize | 与上述一致 |
| 336 | 聚光灯数量 | `LightManager::spot_count(&self) -> usize` | - | usize | 返回聚光灯数量 |
| 376 | 聚光灯数量（详细） | `LightManager::spot_count(&self) -> usize` | - | usize | 与上述一致 |

**优先级**：P0

---

## 3.10 光源 UniformBuffer

### 3.10.1 GPU 数据上传

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 67 | 光源 UBO | LightUniformBuffer：UBO 上传到着色器 | - | - | 每帧更新光源数据 |
| 93 | 光源 UBO（详细） | `LightUniformBuffer` | - | - | 包含所有光源数据 |
| 68 | 光源排序 | 每帧自动排序光源（按距离摄像机远近） | - | - | 优先使用最近的光源 |

**优先级**：P0

---

## 3.11 依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                      engine-math                         │
│                    (Vec3, Color)                         │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                    engine-render-3d                       │
│                      (Light System)                      │
└─────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│Scene3D      │     │RenderPipeline│    │Material3D   │
│光源挂载到节点│     │光源Uniform上传│    │计算光照着色  │
└─────────────┘     └─────────────┘     └─────────────┘
```

**上游依赖**：
- `engine-math`：数学类型

**下游依赖**：
- `Scene3D`：光源挂载到场景节点
- `RenderPipeline3D`：光源数据上传到 GPU
- `Material3D`：在着色器中计算光照

---

## 3.12 验收标准

### 3.12.1 功能验收

- [ ] `DirectionalLight` 方向固定，不随距离衰减
- [ ] `PointLight` 按 1/(a + bd + cd²) 衰减
- [ ] `SpotLight` 在内圆锥内强度为 1，外圆锥外强度为 0，中间线性衰减
- [ ] `LightManager` 能管理多种类型光源
- [ ] `LightManager::lights_ubo()` 返回的数据能被着色器正确读取

### 3.12.2 示例验收

- [ ] `examples/3d_lights` 点光/方向光/聚光正确渲染
- [ ] 多光源同时存在时场景光照正确

---

## 3.13 优先级汇总

| 优先级 | 需求ID | 占比 |
|--------|--------|------|
| P0 | 60-68, 86-95, 172-177, 311-377, 487-489 | 85% |
| P1 | 65, 175, 327, 367 | 10% |
| P2 | - | 5% |

**P0 核心**：DirectionalLight、PointLight、SpotLight、AmbientLight、LightManager
**P1 重要**：HemisphereLight、Plane 归一化
**P2 可选**：高级光照模型