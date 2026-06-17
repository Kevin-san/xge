# engine-render - 2D 渲染核心

游戏引擎 2D 渲染核心库，提供精灵、纹理、批处理、图集打包和正交相机等渲染能力。

## 特性

- **Renderer Trait** - 统一的渲染器抽象，支持 OpenGL (glow) 后端
- **Texture2D** - 纹理管理，支持 RGBA8/RGBA16F/R8/BGRA8 格式
- **Sprite & SpriteBatch** - 精灵绘制和高效批处理
- **AnimatedSprite** - 基于图集的帧动画支持
- **TextureAtlas** - Skyline/Guillotine 两种 bin packing 算法
- **OrthographicCamera / Camera2D** - 正交相机与跟随相机
- **Color** - RGBA f32 颜色，24+ 常用颜色常量
- **BlendMode** - 7 种混合模式
- **DebugRenderer** - 调试图形（线条、矩形、圆形、网格）
- **Shader / Pipeline / Buffer** - 底层渲染抽象

## 快速开始

### 绘制精灵

```rust
use engine_render::{Color, Image, Sprite, Texture2D, Camera2D};

// 创建图像数据
let image_data = create_gradient_image(64, 64);
let image = Image::from_rgba(64, 64, image_data);

// 从图像创建纹理
let texture = Texture2D::from_image(&image);

// 创建精灵
let sprite = Sprite::from_texture(texture)
    .with_color(Color::WHITE)
    .with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));

// 创建相机
let camera = Camera2D::new();
```

### 批量绘制

```rust
use engine_render::{SpriteBatch, DrawParams};
use engine_math::Vec2;

// 创建精灵批次（同一纹理）
let mut batch = SpriteBatch::new(texture);

// 添加精灵
for i in 0..100 {
    let pos = Vec2::new(i as f32 * 32.0, 0.0);
    batch.add(&sprite, pos);
}

// 一次性绘制所有精灵
batch.draw(&mut ctx);
```

### 纹理图集

```rust
use engine_render::{TextureAtlas, TextureAtlasBuilder, PackAlgorithm};

// 构建图集
let mut builder = TextureAtlasBuilder::new(512);
builder.with_padding(2);
builder.with_algorithm(PackAlgorithm::Guillotine);

builder.add(image1);
builder.add(image2);
builder.add(image3);

let (atlas, rects) = builder.build(&ctx).unwrap();

// 获取图集中的区域
if let Some(rect) = atlas.get(0) {
    let sprite = Sprite::from_texture_rect(atlas.texture(), rect);
}
```

### 相机与视图

```rust
use engine_render::{OrthographicCamera, Camera2D};
use engine_math::Vec2;

// 创建正交相机
let camera = OrthographicCamera::from_window(1280, 720, 1.0);

// 创建带跟随的相机
let mut camera = Camera2D::new();
camera.set_target(Some(Vec2::new(100.0, 50.0)));
camera.set_zoom(2.0);

// 每帧更新相机位置
camera.update(delta_time);

// 坐标转换
let world_pos = camera.screen_to_world(screen_pos);
```

## 颜色与混合

```rust
use engine_render::{Color, BlendMode};

// 使用颜色常量
let red = Color::RED;
let custom = Color::from_hex("#FF8040C0").unwrap();

// 颜色插值
let lerped = Color::lerp(Color::RED, Color::BLUE, 0.5);

// 设置混合模式
ctx.set_blend_mode(BlendMode::Additive);  // 发光效果
ctx.set_blend_mode(BlendMode::Multiply);   // 正片叠底
ctx.set_blend_mode(BlendMode::Alpha);      // 标准混合
```

## 调试图形

```rust
use engine_render::{DebugRenderer, Rect};
use engine_math::Vec2;

let mut debug = DebugRenderer::new();

// 绘制调试图形
debug.line(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color::RED);
debug.rect(Rect::new(50.0, 50.0, 100.0, 50.0), Color::BLUE);
debug.circle(Vec2::new(200.0, 200.0), 30.0, Color::GREEN);
debug.cross(Vec2::new(100.0, 100.0), 20.0, Color::WHITE);
debug.grid(Vec2::ZERO, 50.0, 5, 5, Color::GRAY);

// 刷新渲染
debug.flush(&ctx);
```

## Examples

运行示例：

```bash
# 绘制单个精灵
cargo run --example sprite_draw

# 绘制多个精灵
cargo run --example multi_sprite

# 批量绘制
cargo run --example batch_draw

# 图集动画
cargo run --example atlas_animation

# 相机跟随
cargo run --example camera_follow

# 绘制基本图形
cargo run --example shape_draw

# 调试图形
cargo run --example debug_draw

# 混合模式
cargo run --example blend_mode

# 剪刀裁剪
cargo run --example scissor

# 变换栈
cargo run --example transform_stack

# 着色器热重载
cargo run --example hot_shader --features gl
```

## 着色器与 Pipeline

引擎使用内置的 2D 精灵着色器：

```glsl
// sprite.vert
#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;
uniform mat4 uViewProjection;
out vec2 vTexCoord;
out vec4 vColor;

void main() {
    gl_Position = uViewProjection * vec4(aPos, 0.0, 1.0);
    vTexCoord = aTexCoord;
    vColor = aColor;
}
```

```glsl
// sprite.frag
#version 330 core
in vec2 vTexCoord;
in vec4 vColor;
uniform sampler2D uTexture;
out vec4 fragColor;

void main() {
    fragColor = texture(uTexture, vTexCoord) * vColor;
}
```

## 图集打包

TextureAtlas 支持两种 bin packing 算法：

- **Guillotine** (默认) - 更高的空间利用率
- **Skyline** - 打包速度快

```rust
let mut builder = TextureAtlasBuilder::new(1024); // 最大尺寸 1024x1024
builder.with_padding(2);
builder.with_algorithm(PackAlgorithm::Skyline);
```

## 渲染统计

```rust
use engine_render::RenderStats;

// 每帧获取统计
let stats = ctx.stats();
println!("Draw calls: {}", stats.draw_calls);
println!("Vertices: {}", stats.vertices);
println!("Indices: {}", stats.indices);
println!("Batches: {}", stats.batches);
```

## 模块列表

| 模块 | 说明 |
|------|------|
| `renderer` | Renderer trait 与 RenderContext |
| `texture` | Texture2D, Sampler, TextureFormat |
| `image` | CPU 端像素数据，加载/保存图像 |
| `sprite` | Sprite 精灵，Rect 矩形区域 |
| `sprite_batch` | SpriteBatch 高效批处理 |
| `animated_sprite` | AnimatedSprite 帧动画 |
| `texture_atlas` | TextureAtlas 图集打包 |
| `camera` | OrthographicCamera, Camera2D, View, Viewport |
| `color` | Color RGBA，BlendMode 混合模式 |
| `draw_params` | DrawParams 绘制参数 |
| `shader` | Shader, Pipeline, Buffer, BindGroup |
| `debug_renderer` | 调试图形渲染 |
| `render_stats` | RenderStats 渲染统计 |

## License

MIT OR Apache-2.0
