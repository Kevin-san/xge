# 渲染 API 清单

## 1. 概述

本文档列出 `engine-render` crate 所有公开 API，按模块分类。API 数量控制在 80 个以内（需求 382），doc comment 覆盖率 100%（需求 383）。

---

## 2. Renderer 模块

### 2.1 Renderer Trait

```rust
// 创建与生命周期
fn new(window: &Window) -> Result<Self>;
fn default_backend() -> &'static str;
fn backend_info(&self) -> String;

// 帧管理
fn begin_frame(&mut self) -> Result<()>;
fn end_frame(&mut self) -> Result<()>;
fn present(&mut self);

// 配置
fn set_clear_color(&mut self, color: Color);
fn set_vsync(&mut self, mode: VsyncMode);
fn set_resolution(&mut self, w: u32, h: u32);
fn resize(&mut self, w: u32, h: u32);

// 相机
fn camera(&self) -> Option<&Camera2D>;
fn set_camera(&mut self, camera: Camera2D);

// 状态栈
fn push_scissor_rect(&mut self, rect: Rect);
fn pop_scissor_rect(&mut self);
fn push_transform(&mut self, mat: Mat4);
fn pop_transform(&mut self);

// 混合模式
fn set_blend_mode(&mut self, mode: BlendMode);
fn reset_blend_mode(&mut self);

// 绘制
fn draw_quad(&mut self, quad: Quad);
fn draw_texture(&mut self, tex: TextureHandle, x: f32, y: f32, color: Color);
fn draw_texture_ex(&mut self, tex: TextureHandle, x: f32, y: f32, params: DrawParams);
fn draw_texture_pro(&mut self, tex: TextureHandle, source: Rect, dest: Rect, origin: Vec2, rot: f32, color: Color);
fn draw_texture_rotated(&mut self, tex: TextureHandle, x: f32, y: f32, angle: f32, color: Color);
fn draw_texture_rect(&mut self, tex: TextureHandle, source: Rect, dest: Rect, color: Color);
fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color);
fn draw_rectangle_lines(&mut self, x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color);
fn draw_rectangle_rotated(&mut self, x: f32, y: f32, w: f32, h: f32, angle: f32, color: Color);
fn draw_circle(&mut self, x: f32, y: f32, r: f32, color: Color);
fn draw_circle_lines(&mut self, x: f32, y: f32, r: f32, thickness: f32, color: Color);
fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color);
fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color);
fn draw_triangle_lines(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, thickness: f32, color: Color);
fn draw_poly(&mut self, x: f32, y: f32, sides: u32, radius: f32, rotation: f32, color: Color);
fn draw_poly_lines(&mut self, x: f32, y: f32, sides: u32, radius: f32, rotation: f32, thickness: f32, color: Color);
fn draw_text(&mut self, text: &str, x: f32, y: f32, font_size: u32, color: Color); // placeholder

// 统计
fn flush(&mut self);
fn stats(&self) -> RenderStats;
```

### 2.2 RenderStats

```rust
pub struct RenderStats {
    pub draw_calls: usize,
    pub vertices: usize,
    pub indices: usize,
    pub batches: usize,
    pub texture_switches: usize,
}

impl Default for RenderStats { ... }
impl RenderStats {
    pub fn reset(&mut self);
}
```

---

## 3. Texture / Image 模块

### 3.1 Texture2D

```rust
// 创建
fn from_image(ctx: &RenderContext, image: &Image) -> Result<Self>;
fn empty(ctx: &RenderContext, w: u32, h: u32, format: TextureFormat) -> Result<Self>;
fn from_file(ctx: &RenderContext, path: &str) -> Result<Self>;
fn from_bytes(ctx: &RenderContext, bytes: &[u8]) -> Result<Self>;

// 属性
fn width(&self) -> u32;
fn height(&self) -> u32;
fn size(&self) -> (u32, u32);
fn format(&self) -> TextureFormat;
fn handle(&self) -> TextureHandle;

// 更新
fn set_filter(&mut self, filter: FilterMode);
fn set_wrap(&mut self, wrap: WrapMode);
fn update(&mut self, rect: Rect, data: &[u8]);
fn generate_mipmaps(&mut self);
```

### 3.2 Image

```rust
// 创建
fn from_pixels(width: u32, height: u32, data: Vec<u8>) -> Self;
fn from_file(path: &str) -> Result<Self>;
fn from_bytes(bytes: &[u8]) -> Result<Self>;

// 属性
fn width(&self) -> u32;
fn height(&self) -> u32;
fn size(&self) -> (u32, u32);
fn pixels(&self) -> &[u8];
fn pixels_mut(&mut self) -> &mut [u8];

// 操作
fn save(&self, path: &str) -> Result<()>;
fn crop(&self, rect: Rect) -> Image;
fn flip_horizontal(&mut self);
fn flip_vertical(&mut self);
fn rotate_90(&mut self);
fn rotate_180(&mut self);
fn rotate_270(&mut self);
fn resize(&mut self, new_w: u32, new_h: u32);
```

### 3.3 Sampler / SamplerBuilder

```rust
// SamplerBuilder
fn new() -> Self;
fn with_filter(mag: FilterMode, min: FilterMode) -> Self;
fn with_wrap(s: WrapMode, t: WrapMode) -> Self;
fn with_mipmap_filter(mode: FilterMode) -> Self;
fn with_anisotropy(level: u8) -> Self;
fn build(&self, ctx: &RenderContext) -> Sampler;

// Sampler
fn handle(&self) -> SamplerHandle;
```

### 3.4 TextureManager

```rust
fn new() -> Self;
fn load(&mut self, path: &str) -> Result<TextureHandle>;
fn get(&self, handle: TextureHandle) -> Option<&Texture2D>;
fn unload(&mut self, handle: TextureHandle);
fn reload(&mut self, handle: TextureHandle) -> Result<()>;
fn iter(&self) -> impl Iterator<Item = &Texture2D>;
```

---

## 4. Sprite / SpriteBatch 模块

### 4.1 Sprite

```rust
// 创建
fn new(texture: TextureHandle) -> Self;
fn from_texture(texture: TextureHandle) -> Self;
fn from_texture_rect(texture: TextureHandle, rect: Rect) -> Self;

// 属性
fn source_rect(&self) -> Option<Rect>;
fn color(&self) -> Color;
fn flip_x(&self) -> bool;
fn flip_y(&self) -> bool;
fn anchor(&self) -> Vec2;
fn size(&self) -> Vec2;

// 设置
fn set_source_rect(&mut self, rect: Rect);
fn set_color(&mut self, color: Color);
fn set_flip_x(&mut self, flip: bool);
fn set_flip_y(&mut self, flip: bool);
fn set_anchor(&mut self, anchor: Vec2);

// 绘制
fn draw(&self, ctx: &mut RenderContext, position: Vec2);
fn draw_ex(&self, ctx: &mut RenderContext, params: DrawParams);
```

### 4.2 AnimatedSprite

```rust
// 创建
fn new(atlas: TextureAtlasHandle, fps: f32, frames: Vec<Rect>) -> Self;

// 播放控制
fn play(&mut self);
fn pause(&mut self);
fn stop(&mut self);
fn is_playing(&self) -> bool;

// 帧
fn current_frame(&self) -> usize;
fn set_frame(&mut self, idx: usize);
fn total_frames(&self) -> usize;
fn fps(&self) -> f32;
fn set_fps(&mut self, fps: f32);

// 循环
fn set_loop(&mut self, mode: LoopMode);
fn loop_mode(&self) -> LoopMode;

// 更新与绘制
fn update(&mut self, dt: f32);
fn draw(&self, ctx: &mut RenderContext, position: Vec2);
```

### 4.3 SpriteBatch

```rust
// 创建
fn new(texture: TextureHandle) -> Self;
fn with_capacity(texture: TextureHandle, cap: usize) -> Self;

// 操作
fn add(&mut self, sprite: &Sprite, position: Vec2) -> usize;
fn add_ex(&mut self, sprite: &Sprite, params: DrawParams) -> usize;
fn set(&mut self, idx: usize, sprite: &Sprite, position: Vec2);
fn remove(&mut self, idx: usize);
fn clear(&mut self);

// 属性
fn len(&self) -> usize;
fn is_empty(&self) -> bool;
fn capacity(&self) -> usize;
fn texture(&self) -> TextureHandle;
fn set_texture(&mut self, tex: TextureHandle);

// 绘制
fn draw(&self, ctx: &mut RenderContext);
fn draw_at(&self, ctx: &mut RenderContext, x: f32, y: f32);
```

### 4.4 BatchRenderer

```rust
fn new() -> Self;
fn begin(&mut self);
fn draw(&mut self, sprite: &Sprite, transform: Vec2);
fn end(&mut self, ctx: &mut RenderContext);
fn flush(&mut self, ctx: &mut RenderContext);
fn batches(&self) -> usize;
```

---

## 5. TextureAtlas 模块

### 5.1 TextureAtlasBuilder

```rust
fn new(max_size: u32) -> Self;
fn with_padding(pixels: u32) -> Self;
fn with_algorithm(algorithm: PackAlgorithm) -> Self;
fn add(&mut self, image: &Image) -> usize;
fn add_from_file(&mut self, path: &str) -> Result<usize>;
fn build(&self, ctx: &RenderContext) -> Result<(TextureAtlas, Vec<Rect>)>;
```

### 5.2 TextureAtlas

```rust
fn texture(&self) -> TextureHandle;
fn size(&self) -> (u32, u32);
fn len(&self) -> usize;
fn is_empty(&self) -> bool;
fn get(&self, idx: usize) -> Option<Rect>;
fn get_uv(&self, idx: usize) -> Option<(Vec2, Vec2)>;
fn get_sprite(&self, idx: usize) -> Sprite;
```

---

## 6. Camera / Color 模块

### 6.1 OrthographicCamera

```rust
fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self;
fn from_size(w: f32, h: f32) -> Self;
fn from_window(window: &Window, zoom: f32) -> Self;
fn projection(&self) -> Mat4;
fn view(&self) -> Mat4;
fn view_projection(&self) -> Mat4;
fn screen_to_world(&self, screen_pos: Vec2) -> Vec2;
fn world_to_screen(&self, world_pos: Vec2) -> Vec2;
fn zoom(&mut self, factor: f32);
fn move_by(&mut self, delta: Vec2);
```

### 6.2 Camera2D

```rust
fn new() -> Self;
fn from_window(window: &Window, zoom: f32) -> Self;
fn position(&self) -> Vec2;
fn set_position(&mut self, pos: Vec2);
fn rotation(&self) -> f32;
fn set_rotation(&mut self, angle: f32);
fn zoom(&self) -> f32;
fn set_zoom(&mut self, zoom: f32);
fn target(&self) -> Option<Vec2>;
fn set_target(&mut self, target: Vec2);
fn offset(&self) -> Vec2;
fn set_offset(&mut self, offset: Vec2);
fn projection(&self) -> Mat4;
fn view(&self) -> Mat4;
fn view_projection(&self) -> Mat4;
fn screen_to_world(&self, screen_pos: Vec2) -> Vec2;
fn world_to_screen(&self, world_pos: Vec2) -> Vec2;
fn update(&mut self, dt: f32);
```

### 6.3 View / Viewport

```rust
// View
fn new(camera: Camera2D, viewport: Viewport) -> Self;

// Viewport
fn new(x: i32, y: i32, w: u32, h: u32) -> Self;
fn x(&self) -> i32;
fn y(&self) -> i32;
fn w(&self) -> u32;
fn h(&self) -> u32;
```

### 6.4 Color

```rust
// 构造
fn new(r: f32, g: f32, b: f32, a: f32) -> Self;
fn from_rgb(r: f32, g: f32, b: f32) -> Self;
fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self;
fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self;
fn from_hex(hex: &str) -> Result<Self>;

// 转换
fn to_hex(&self) -> String;
fn to_vec4(&self) -> [f32; 4];
fn to_array(&self) -> [f32; 4];

// 插值
fn lerp(a: Color, b: Color, t: f32) -> Color;

// 常量
const RED: Color;
const GREEN: Color;
const BLUE: Color;
const WHITE: Color;
const BLACK: Color;
const TRANSPARENT: Color;
const YELLOW: Color;
const CYAN: Color;
const MAGENTA: Color;
const ORANGE: Color;
const GRAY: Color;
const LIGHTGRAY: Color;
const DARKGRAY: Color;
const GOLD: Color;
const LIME: Color;
const PINK: Color;
const PURPLE: Color;
const TEAL: Color;
const MAROON: Color;
const NAVY: Color;
const OLIVE: Color;
const BROWN: Color;
```

### 6.5 BlendMode

```rust
enum BlendMode {
    Alpha,
    Additive,
    Subtract,
    Multiply,
    Replace,
    Invert,
    PreMultiplied,
}

impl BlendMode {
    fn to_gl_enum(&self) -> u32;
}
```

### 6.6 DrawParams

```rust
struct DrawParams {
    pub color_tint: Color,
    pub blend_mode: BlendMode,
    pub z_order: f32,
}
```

---

## 7. Shader / Pipeline / Buffer 模块

### 7.1 Shader

```rust
fn from_source(stage: ShaderStage, source: &str) -> Result<Self>;
fn from_file(stage: ShaderStage, path: &str) -> Result<Self>;

enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}
```

### 7.2 ShaderModule

```rust
fn compile(&self, entry: &str) -> Result<CompiledShader>;
fn hot_reload(&mut self);
```

### 7.3 PipelineBuilder

```rust
fn new(shader_module: ShaderModule) -> Self;
fn with_vertex_layout(layout: VertexLayout) -> Self;
fn with_blend_mode(mode: BlendMode) -> Self;
fn with_depth_test(enabled: bool) -> Self;
fn with_cull_mode(mode: CullMode) -> Self;
fn with_winding_order(order: WindingOrder) -> Self;
fn build(&self, ctx: &RenderContext) -> Result<Pipeline>;
```

### 7.4 Pipeline

```rust
fn bind(&self, ctx: &mut RenderContext);
```

### 7.5 Buffer

```rust
fn new_vertex(ctx: &RenderContext, data: &[u8]) -> Result<Self>;
fn new_index(ctx: &RenderContext, data: &[u8]) -> Result<Self>;
fn new_uniform(ctx: &RenderContext, data: &[u8]) -> Result<Self>;
fn update(&mut self, offset: usize, data: &[u8]);
fn size(&self) -> usize;

enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    CopyDst,
}
```

### 7.6 BindGroup / BindGroupLayout

```rust
fn new(ctx: &RenderContext, layout: &BindGroupLayout, resources: &[BindingResource]) -> Result<Self>;

fn new(ctx: &RenderContext, entries: &[BindingEntry]) -> Result<Self>;
```

### 7.7 VertexLayout

```rust
fn new() -> Self;
fn push::<T>(&mut self, name: &str) -> &mut Self;
fn stride(&self) -> usize;
fn attributes(&self) -> &[VertexAttr];
```

### 7.8 Mesh2D

```rust
fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self;
fn quad(w: f32, h: f32, color: Color) -> Self;
fn draw(&self, ctx: &mut RenderContext, pipeline: &Pipeline, bind_groups: &[&BindGroup]);
```

---

## 8. Debug / Profiler 模块

### 8.1 DebugRenderer

```rust
fn new() -> Self;
fn line(&mut self, a: Vec2, b: Vec2, color: Color);
fn rect(&mut self, rect: Rect, color: Color);
fn rect_lines(&mut self, rect: Rect, color: Color);
fn circle(&mut self, c: Vec2, r: f32, color: Color);
fn circle_lines(&mut self, c: Vec2, r: f32, color: Color);
fn text(&mut self, text: &str, pos: Vec2, color: Color); // placeholder
fn cross(&mut self, pos: Vec2, size: f32, color: Color);
fn grid(&mut self, origin: Vec2, cell_size: f32, cols: u32, rows: u32, color: Color);
fn flush(&mut self, ctx: &mut RenderContext);
fn clear(&mut self);
```

### 8.2 Profiler

```rust
fn push_scope(name: &str);
fn pop_scope();
fn dump() -> String;
```

---

## 9. 枚举类型汇总

| 枚举 | 值 |
|------|-----|
| `TextureFormat` | RGBA8, RGBA16F, R8, BGRA8 |
| `FilterMode` | Linear, Nearest |
| `WrapMode` | Clamp, Repeat, MirrorRepeat |
| `BlendMode` | Alpha, Additive, Subtract, Multiply, Replace, Invert, PreMultiplied |
| `BufferUsage` | Vertex, Index, Uniform, CopyDst |
| `ShaderStage` | Vertex, Fragment, Compute |
| `PackAlgorithm` | Skyline, Guillotine |
| `LoopMode` | Once, Loop, PingPong |
| `VsyncMode` | On, Off, Adaptive |

---

## 10. API 统计

| 模块 | API 数量 |
|------|----------|
| Renderer | 35 |
| Texture/Image | 22 |
| Sprite/SpriteBatch | 26 |
| TextureAtlas | 9 |
| Camera/Color | 33 |
| Shader/Pipeline/Buffer | 20 |
| Debug/Profiler | 9 |
| **总计** | **154** |

> 注：当前设计 API 数量为 154，超过需求 382 限制的 80 个。需要进行 API 精简：
> - 合并相似方法（如 `from_texture` 与 `new`）
> - 移除冗余重载
> - 简化 Builder 模式
> - 目标：精简至 80 个以内