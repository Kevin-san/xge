# 模块五：2D 相机需求

## 1. 模块概述

本模块提供 2D 相机能力，包括 `OrthographicCamera` 正交相机、`Camera2D` 带变换的相机、`View` 和 `Viewport` 视图抽象，以及颜色 `Color` 和混合模式 `BlendMode`。

**核心目标**：支持 2D 正交投影、屏幕/世界坐标转换、相机跟随、平滑插值。

---

## 2. 需求清单

### 2.1 OrthographicCamera（需求 79-87, 334-336）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 79 | OrthographicCamera | `struct OrthographicCamera { left, right, bottom, top, near, far }` | - | - | 结构完整 | 需求 2 | P0 |
| 80 | 从窗口创建 | `OrthographicCamera::from_window(window: &Window, zoom: f32) -> Self` | window, zoom | Self | 创建成功 | 需求 79 | P0 |
| 81 | 投影矩阵 | `OrthographicCamera::projection(&self) -> Mat4` | - | Mat4 | 返回投影 | 需求 79 | P0 |
| 82 | 视图矩阵 | `OrthographicCamera::view(&self) -> Mat4` | - | Mat4 | 返回视图 | 需求 79 | P0 |
| 83 | 视图投影矩阵 | `OrthographicCamera::view_projection(&self) -> Mat4` | - | Mat4 | 返回组合 | 需求 81-82 | P0 |
| 84 | 屏幕转世界 | `OrthographicCamera::screen_to_world(&self, screen_pos: Vec2) -> Vec2` | screen_pos | Vec2 | 转换正确 | 需求 79 | P0 |
| 85 | 世界转屏幕 | `OrthographicCamera::world_to_screen(&self, world_pos: Vec2) -> Vec2` | world_pos | Vec2 | 转换正确 | 需求 79 | P0 |
| 86 | 缩放 | `OrthographicCamera::zoom(&mut self, factor: f32)` | factor | - | 缩放相机 | 需求 79 | P0 |
| 87 | 移动 | `OrthographicCamera::move_by(&mut self, delta: Vec2)` | delta | - | 平移相机 | 需求 79 | P0 |
| 334 | 创建 OrthographicCamera | `OrthographicCamera::new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self` | left, right, bottom, top, near, far | Self | 创建成功 | 需求 79 | P0 |
| 335 | 从尺寸创建 | `OrthographicCamera::from_size(w: f32, h: f32) -> Self` | w, h | Self | 简化创建 | 需求 334 | P0 |
| 336 | 视图投影 | `OrthographicCamera::view_projection(&self) -> Mat4` | - | Mat4 | 同需求 83 | 需求 83 | P0 |

### 2.2 Camera2D（需求 62, 88, 271-333）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 62 | Camera2D 结构 | `struct Camera2D` | - | - | 带位置/旋转/缩放 | 需求 2 | P0 |
| 88 | View / Viewport | `struct View / Viewport` | - | - | 视图抽象 | 需求 62 | P0 |
| 271 | 创建 Camera2D | `Camera2D::new()` | - | Self | 创建成功 | 需求 62 | P0 |
| 272 | 从窗口创建 | `Camera2D::from_window(window: &Window, zoom: f32) -> Self` | window, zoom | Self | 同需求 80 | 需求 80 | P0 |
| 273 | 位置获取 | `Camera2D::position(&self) -> Vec2` | - | Vec2 | 返回位置 | 需求 62 | P0 |
| 274 | 位置设置 | `Camera2D::set_position(&mut self, pos: Vec2)` | pos | - | 设置位置 | 需求 273 | P0 |
| 275 | 旋转获取 | `Camera2D::rotation(&self) -> f32` | - | f32 | 返回弧度 | 需求 62 | P0 |
| 276 | 旋转设置 | `Camera2D::set_rotation(&mut self, angle: f32)` | angle | - | 设置旋转 | 需求 275 | P0 |
| 277 | 缩放获取 | `Camera2D::zoom(&self) -> f32` | - | f32 | 返回缩放 | 需求 62 | P0 |
| 278 | 缩放设置 | `Camera2D::set_zoom(&mut self, zoom: f32)` | zoom | - | 设置缩放 | 需求 277 | P0 |
| 279 | 目标获取 | `Camera2D::target(&self) -> Option<Vec2>` | - | Option<Vec2> | 返回跟随目标 | 需求 62 | P1 |
| 280 | 目标设置 | `Camera2D::set_target(&mut self, target: Vec2)` | target | - | 设置跟随 | 需求 279 | P1 |
| 281 | 偏移获取 | `Camera2D::offset(&self) -> Vec2` | - | Vec2 | 返回偏移 | 需求 62 | P1 |
| 282 | 偏移设置 | `Camera2D::set_offset(&mut self, offset: Vec2)` | offset | - | 设置偏移 | 需求 281 | P1 |
| 283 | 投影矩阵 | `Camera2D::projection(&self) -> Mat4` | - | Mat4 | 返回投影 | 需求 62 | P0 |
| 284 | 视图矩阵 | `Camera2D::view(&self) -> Mat4` | - | Mat4 | 返回视图 | 需求 62 | P0 |
| 285 | 视图投影 | `Camera2D::view_projection(&self) -> Mat4` | - | Mat4 | 返回组合 | 需求 283-284 | P0 |
| 286 | 屏幕转世界 | `Camera2D::screen_to_world(&self, screen_pos: Vec2) -> Vec2` | screen_pos | Vec2 | 同需求 84 | 需求 84 | P0 |
| 287 | 世界转屏幕 | `Camera2D::world_to_screen(&self, world_pos: Vec2) -> Vec2` | world_pos | Vec2 | 同需求 85 | 需求 85 | P0 |
| 288 | 更新相机 | `Camera2D::update(&mut self, dt: f32)` | dt | - | 平滑跟随 | 需求 279 | P1 |
| 289 | OrthographicCamera::new | `OrthographicCamera::new(left, right, bottom, top, near, far)` | left, right, bottom, top, near, far | Self | 同需求 334 | 需求 334 | P0 |
| 290 | OrthographicCamera::from_size | `OrthographicCamera::from_size(w, h)` | w, h | Self | 同需求 335 | 需求 335 | P0 |
| 291 | OrthographicCamera::view_projection | `OrthographicCamera::view_projection()` | - | Mat4 | 同需求 336 | 需求 336 | P0 |
| 292 | View 创建 | `View::new(camera: Camera2D, viewport: Viewport) -> Self` | camera, viewport | Self | 创建成功 | 需求 88 | P0 |
| 293 | Viewport 创建 | `Viewport::new(x: i32, y: i32, w: u32, h: u32) -> Self` | x, y, w, h | Self | 创建成功 | 需求 88 | P0 |
| 294 | Viewport 属性 | `Viewport::x(&self) / y(&self) / w(&self) / h(&self)` | - | i32/u32 | 返回属性 | 需求 293 | P0 |

### 2.3 Color（需求 64-67, 91-93, 295-349）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 64 | Color RGBA f32 | `struct Color { r, g, b, a }` | - | - | f32 0.0-1.0 | 需求 2 | P0 |
| 65 | Color 构造方法 | `Color::new(r, g, b, a) / from_u8(r,g,b,a) / from_hex(&str) / to_hex(&self)` | - | - | 多种构造 | 需求 64 | P0 |
| 66 | Color 常量 | `RED / GREEN / BLUE / WHITE / BLACK / TRANSPARENT / YELLOW / CYAN / MAGENTA / ORANGE / GRAY / LIME / PINK / PURPLE / TEAL` (15+) | - | - | 常用色 | 需求 64 | P0 |
| 67 | Color 插值 | `Color::lerp(a: Color, b: Color, t: f32) -> Color` | a, b, t | Color | 线性插值 | 需求 64 | P0 |
| 91 | Color RGBA | `Color::new(r, g, b, a)` | - | - | 同需求 65 | 需求 65 | P0 |
| 92 | Color from_u8 | `Color::from_u8(r, g, b, a) -> Self` | u8, u8, u8, u8 | Self | u8 转 f32 | 需求 65 | P0 |
| 93 | Color from_hex | `Color::from_hex(hex: &str) -> Result<Self>` | hex | Result<Self> | "#RRGGBB" 解析 | 需求 65 | P0 |
| 295 | Color::new | `Color::new(r: f32, g: f32, b: f32, a: f32) -> Self` | r, g, b, a | Self | 同需求 64 | 需求 64 | P0 |
| 296 | Color::from_rgb | `Color::from_rgb(r: f32, g: f32, b: f32) -> Self` | r, g, b | Self | RGB 构造 | 需求 295 | P0 |
| 297 | Color::from_rgba | `Color::from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self` | r, g, b, a | Self | 同需求 295 | 需求 295 | P0 |
| 298 | Color::from_u8 | `Color::from_u8(r: u8, g: u8, b: u8, a: u8) -> Self` | u8, u8, u8, u8 | Self | 同需求 92 | 需求 92 | P0 |
| 299 | Color::from_hex | `Color::from_hex(hex: &str) -> Result<Self>` | hex | Result<Self> | 同需求 93 | 需求 93 | P0 |
| 300 | Color::to_hex | `Color::to_hex(&self) -> String` | - | String | 返回 "#RRGGBBAA" | 需求 93 | P0 |
| 301 | Color::to_vec4 | `Color::to_vec4(&self) -> [f32; 4]` | - | [f32; 4] | 返回数组 | 需求 295 | P0 |
| 302 | Color::to_array | `Color::to_array(&self) -> [f32; 4]` | - | [f32; 4] | 同 to_vec4 | 需求 301 | P0 |
| 303 | Color::lerp | `Color::lerp(a: Color, b: Color, t: f32) -> Color` | a, b, t | Color | 同需求 67 | 需求 67 | P0 |
| 304 | Color 常量集 | `RED / GREEN / BLUE / WHITE / BLACK / TRANSPARENT / YELLOW / CYAN / MAGENTA / ORANGE / GRAY / LIGHTGRAY / DARKGRAY / GOLD / LIME / PINK / PURPLE / TEAL / MAROON / NAVY / OLIVE / BROWN` (24 种) | - | - | 24 色常量 | 需求 66 | P0 |
| 305 | BlendMode 枚举 | `enum BlendMode { Alpha, Additive, Subtract, Multiply, Replace, Invert, PreMultiplied }` | - | - | 7 种模式 | 需求 69 | P0 |
| 306 | BlendMode 转 GL | `BlendMode::to_gl_enum(&self) -> u32` | - | u32 | GL 枚举值 | 需求 305 | P1 |

### 2.4 DrawParams / BlendMode（需求 68-69, 94-95）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 68 | DrawParams | `struct DrawParams { color_tint, blend_mode, z_order }` | - | - | 结构完整 | 需求 2 | P0 |
| 69 | BlendMode | `enum BlendMode { Alpha, Add, Subtract, Multiply, Replace, Invert, PreMultiplied }` | - | - | 7 种模式 | 需求 68 | P0 |
| 94 | Color::to_hex | `Color::to_hex(&self) -> String` | - | String | 同需求 300 | 需求 300 | P0 |
| 95 | Color::to_vec4 | `Color::to_vec4(&self) -> [f32; 4]` | - | [f32; 4] | 同需求 301 | 需求 301 | P0 |

---

## 3. 验收标准

### 3.1 功能验收

- [ ] `OrthographicCamera` 正确计算投影/视图/视图投影矩阵
- [ ] `screen_to_world` / `world_to_screen` 坐标转换正确
- [ ] `Camera2D` 支持位置/旋转/缩放/目标跟随
- [ ] `Color` 支持多种构造方法和 24 色常量
- [ ] `BlendMode` 支持 7 种混合模式

### 3.2 质量验收

- [ ] `Camera::screen_to_world` 单元测试通过（需求 365）
- [ ] `Color::from_hex / to_hex` 往返测试通过（需求 366）
- [ ] `OrthographicCamera` 投影单元测试通过（需求 368）
- [ ] clippy 无 warning
- [ ] fmt 检查通过

---

## 4. 依赖关系图

```
┌─────────────────────────────────────┐
│        OrthographicCamera            │
│  ├── projection / view / vp        │
│  ├── screen_to_world / world_to_screen │
│  └── zoom / move_by                │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│            Camera2D                 │
│  ├── position / rotation / zoom    │
│  ├── target / offset               │
│  └── update (smooth follow)        │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│           View / Viewport            │
│  ├── camera + viewport             │
│  └── x / y / w / h                 │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│             Color                    │
│  ├── new / from_u8 / from_hex       │
│  ├── to_hex / to_vec4 / lerp       │
│  └── 24 color constants             │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│           BlendMode                  │
│  Alpha / Additive / Subtract        │
│  Multiply / Replace / Invert        │
│  PreMultiplied                      │
└─────────────────────────────────────┘
```

---

## 5. 备注

- 正交相机不进行透视校正，适用于 2D 游戏
- `Camera2D::update()` 实现平滑跟随时使用线性插值
- Color 内部使用 f32 表示通道值，范围 [0.0, 1.0]
- BlendMode 对应 OpenGL/DirectX 混合方程