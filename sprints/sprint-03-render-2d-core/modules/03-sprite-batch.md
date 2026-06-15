# 模块三：Sprite 与批处理需求

## 1. 模块概述

本模块提供 2D 精灵（Sprite）绘制能力，包括基础 `Sprite`、动画精灵 `AnimatedSprite`、精灵批处理 `SpriteBatch` 以及自动合批渲染器 `BatchRenderer`。

**核心目标**：支持精灵的灵活绘制、多纹理合批减少 draw call、基于图集的帧动画。

---

## 2. 需求清单

### 2.1 Sprite 基础（需求 24-31, 50-57, 206-220, 258-260）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 24 | Sprite 结构体 | `struct Sprite { texture, source_rect, color_tint, flip_x, flip_y, anchor, origin }` | - | - | 结构完整 | 需求 16 | P0 |
| 25 | 从纹理创建 | `Sprite::from_texture(handle: TextureHandle) -> Self` | handle | Sprite | 创建成功 | 需求 24 | P0 |
| 26 | 设置源矩形 | `Sprite::with_source_rect(rect: Rect) -> Self` | rect | Self | 链式设置 | 需求 24 | P0 |
| 27 | 设置颜色 | `Sprite::with_color(color: Color) -> Self` | color | Self | 链式设置 | 需求 24 | P0 |
| 28 | 设置水平翻转 | `Sprite::with_flip_x(bool) -> Self` | bool | Self | 链式设置 | 需求 24 | P0 |
| 29 | 设置垂直翻转 | `Sprite::with_flip_y(bool) -> Self` | bool | Self | 链式设置 | 需求 24 | P0 |
| 30 | 设置锚点 | `Sprite::with_anchor(anchor: Vec2) -> Self` | Vec2 | Self | 链式设置 | 需求 24 | P0 |
| 31 | 绘制精灵 | `Sprite::draw(&self, transform: Vec2, ctx: &mut RenderContext)` | transform, ctx | - | 绘制成功 | 需求 24 | P0 |
| 50 | Sprite::from_texture | `Sprite::from_texture(handle) -> Self` | handle | Self | 同需求 25 | 需求 25 | P0 |
| 51 | Sprite::with_source_rect | `Sprite::with_source_rect(rect) -> Self` | rect | Self | 同需求 26 | 需求 26 | P0 |
| 52 | Sprite::with_color | `Sprite::with_color(color) -> Self` | color | Self | 同需求 27 | 需求 27 | P0 |
| 53 | Sprite::with_flip_x | `Sprite::with_flip_x(bool) -> Self` | bool | Self | 同需求 28 | 需求 28 | P0 |
| 54 | Sprite::with_flip_y | `Sprite::with_flip_y(bool) -> Self` | bool | Self | 同需求 29 | 需求 29 | P0 |
| 55 | Sprite::with_anchor | `Sprite::with_anchor(Vec2) -> Self` | Vec2 | Self | 同需求 30 | 需求 30 | P0 |
| 56 | Sprite::draw | `Sprite::draw(&self, transform, ctx)` | transform, ctx | - | 同需求 31 | 需求 31 | P0 |
| 206 | 创建 Sprite | `Sprite::new(texture: TextureHandle) -> Self` | texture | Self | 同需求 25 | 需求 25 | P0 |
| 207 | 从纹理矩形创建 | `Sprite::from_texture_rect(texture: TextureHandle, rect: Rect) -> Self` | texture, rect | Self | 指定区域 | 需求 25 | P0 |
| 208 | 获取源矩形 | `Sprite::source_rect(&self) -> Option<Rect>` | - | Option<Rect> | 返回区域 | 需求 26 | P0 |
| 209 | 设置源矩形 | `Sprite::set_source_rect(&mut self, rect: Rect)` | rect | - | 设置成功 | 需求 208 | P0 |
| 210 | 获取颜色 | `Sprite::color(&self) -> Color` | - | Color | 返回颜色 | 需求 27 | P0 |
| 211 | 设置颜色 | `Sprite::set_color(&mut self, color: Color)` | color | - | 设置成功 | 需求 210 | P0 |
| 212 | 获取翻转 X | `Sprite::flip_x(&self) -> bool` | - | bool | 返回状态 | 需求 28 | P0 |
| 213 | 设置翻转 X | `Sprite::set_flip_x(&mut self, flip: bool)` | bool | - | 设置成功 | 需求 212 | P0 |
| 214 | 获取翻转 Y | `Sprite::flip_y(&self) -> bool` | - | bool | 返回状态 | 需求 29 | P0 |
| 215 | 设置翻转 Y | `Sprite::set_flip_y(&mut self, flip: bool)` | bool | - | 设置成功 | 需求 214 | P0 |
| 216 | 获取锚点 | `Sprite::anchor(&self) -> Vec2` | - | Vec2 | 返回锚点 | 需求 30 | P0 |
| 217 | 设置锚点 | `Sprite::set_anchor(&mut self, anchor: Vec2)` | anchor | - | 设置成功 | 需求 216 | P0 |
| 218 | 获取尺寸 | `Sprite::size(&self) -> Vec2` | - | Vec2 | 返回宽高 | 需求 15 | P0 |
| 219 | 绘制精灵 | `Sprite::draw(&self, ctx: &mut RenderContext, position: Vec2)` | ctx, position | - | 绘制成功 | 需求 31 | P0 |
| 220 | 高级绘制 | `Sprite::draw_ex(&self, ctx: &mut RenderContext, params: DrawParams)` | ctx, params | - | 按参数绘制 | 需求 68 | P0 |

### 2.2 AnimatedSprite（需求 32-35, 58-61, 221-235, 274）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 32 | AnimatedSprite 结构 | `struct AnimatedSprite` | - | - | 结构完整 | 需求 24 | P0 |
| 33 | 播放动画 | `AnimatedSprite::play(&mut self)` | - | - | 开始播放 | 需求 32 | P0 |
| 34 | 暂停动画 | `AnimatedSprite::pause(&mut self)` | - | - | 暂停 | 需求 32 | P0 |
| 35 | 停止动画 | `AnimatedSprite::stop(&mut self)` | - | - | 停止并重置 | 需求 32 | P0 |
| 36 | 播放状态查询 | `AnimatedSprite::is_playing(&self) -> bool` | - | bool | 返回状态 | 需求 32 | P0 |
| 58 | AnimatedSprite 帧 | `AnimatedSprite::current_frame` | - | - | 当前帧索引 | 需求 32 | P0 |
| 59 | AnimatedSprite fps | `AnimatedSprite::fps / total_frames` | - | - | 属性完整 | 需求 32 | P0 |
| 221 | 创建 AnimatedSprite | `AnimatedSprite::new(atlas: TextureAtlasHandle, fps: f32, frames: Vec<Rect>) -> Self` | atlas, fps, frames | Self | 创建成功 | 需求 32 | P0 |
| 222 | 播放 | `AnimatedSprite::play(&mut self)` | - | - | 同需求 33 | 需求 33 | P0 |
| 223 | 暂停 | `AnimatedSprite::pause(&mut self)` | - | - | 同需求 34 | 需求 34 | P0 |
| 224 | 停止 | `AnimatedSprite::stop(&mut self)` | - | - | 同需求 35 | 需求 35 | P0 |
| 225 | 播放状态 | `AnimatedSprite::is_playing(&self) -> bool` | - | bool | 同需求 36 | 需求 36 | P0 |
| 226 | 当前帧 | `AnimatedSprite::current_frame(&self) -> usize` | - | usize | 返回帧索引 | 需求 58 | P0 |
| 227 | 设置帧 | `AnimatedSprite::set_frame(&mut self, idx: usize)` | idx | - | 跳转帧 | 需求 226 | P0 |
| 228 | 总帧数 | `AnimatedSprite::total_frames(&self) -> usize` | - | usize | 返回总数 | 需求 59 | P0 |
| 229 | 获取 fps | `AnimatedSprite::fps(&self) -> f32` | - | f32 | 返回帧率 | 需求 59 | P0 |
| 230 | 设置 fps | `AnimatedSprite::set_fps(&mut self, fps: f32)` | fps | - | 修改帧率 | 需求 229 | P0 |
| 231 | 设置循环模式 | `AnimatedSprite::set_loop(&mut self, mode: LoopMode)` | mode | - | 设置循环 | 需求 274 | P0 |
| 232 | 获取循环模式 | `AnimatedSprite::loop_mode(&self) -> LoopMode` | - | LoopMode | 返回模式 | 需求 274 | P0 |
| 233 | 更新动画 | `AnimatedSprite::update(&mut self, dt: f32)` | dt | - | 推进帧 | 需求 32 | P0 |
| 234 | 绘制动画 | `AnimatedSprite::draw(&self, ctx: &mut RenderContext, position: Vec2)` | ctx, position | - | 绘制当前帧 | 需求 221 | P0 |
| 274 | LoopMode 枚举 | `enum LoopMode { Once, Loop, PingPong }` | - | - | 正确枚举 | 需求 231 | P0 |

### 2.3 SpriteBatch（需求 36-43, 63-68, 236-255, 275-295）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 36 | SpriteBatch 合批 | `struct SpriteBatch` | - | - | 结构完整 | 需求 24 | P0 |
| 37 | 创建批处理 | `SpriteBatch::new(texture: TextureHandle) -> Self` | texture | Self | 创建成功 | 需求 36 | P0 |
| 38 | 添加精灵 | `SpriteBatch::add(sprite: &Sprite, transform: Vec2) -> usize` | sprite, transform | usize | 返回索引 | 需求 36 | P0 |
| 39 | 绘制批处理 | `SpriteBatch::draw(&self, ctx: &mut RenderContext)` | ctx | - | 批量绘制 | 需求 36 | P0 |
| 40 | 清空批处理 | `SpriteBatch::clear(&mut self)` | - | - | 清空队列 | 需求 36 | P0 |
| 41 | 容量查询 | `SpriteBatch::capacity(&self) -> usize` | - | usize | 返回容量 | 需求 36 | P1 |
| 42 | 数量查询 | `SpriteBatch::len(&self) -> usize` | - | usize | 返回数量 | 需求 36 | P1 |
| 43 | 内部顶点和索引缓冲 | 使用 Triangles 2x3 per sprite | - | - | 索引正确 | 需求 36 | P0 |
| 63 | BatchRenderer 自动合批 | `struct BatchRenderer` | - | - | 按纹理合批 | 需求 36 | P0 |
| 64 | Quad 结构体 | `struct Quad { x, y, w, h, u0, v0, u1, v1, color }` | - | - | 结构完整 | 需求 70 | P0 |
| 236 | 创建 SpriteBatch | `SpriteBatch::new(texture) -> Self` | texture | Self | 同需求 37 | 需求 37 | P0 |
| 237 | 带容量创建 | `SpriteBatch::with_capacity(texture: TextureHandle, cap: usize) -> Self` | texture, cap | Self | 预分配容量 | 需求 236 | P1 |
| 238 | 添加精灵 | `SpriteBatch::add(&mut self, sprite: &Sprite, position: Vec2) -> usize` | sprite, position | usize | 同需求 38 | 需求 38 | P0 |
| 239 | 高级添加 | `SpriteBatch::add_ex(&mut self, sprite: &Sprite, params: DrawParams) -> usize` | sprite, params | usize | 按参数添加 | 需求 238 | P0 |
| 240 | 设置索引 | `SpriteBatch::set(&mut self, idx: usize, sprite: &Sprite, position: Vec2)` | idx, sprite, position | - | 修改指定索引 | 需求 238 | P1 |
| 241 | 移除索引 | `SpriteBatch::remove(&mut self, idx: usize)` | idx | - | 移除指定项 | 需求 238 | P1 |
| 242 | 清空 | `SpriteBatch::clear(&mut self)` | - | - | 同需求 40 | 需求 40 | P0 |
| 243 | 长度 | `SpriteBatch::len(&self) -> usize` | - | usize | 同需求 42 | 需求 42 | P1 |
| 244 | 是否为空 | `SpriteBatch::is_empty(&self) -> bool` | - | bool | 返回状态 | 需求 243 | P1 |
| 245 | 容量 | `SpriteBatch::capacity(&self) -> usize` | - | usize | 同需求 41 | 需求 41 | P1 |
| 246 | 绘制 | `SpriteBatch::draw(&self, ctx: &mut RenderContext)` | ctx | - | 同需求 39 | 需求 39 | P0 |
| 247 | 指定位置绘制 | `SpriteBatch::draw_at(&self, ctx: &mut RenderContext, x: f32, y: f32)` | ctx, x, y | - | 偏移绘制 | 需求 246 | P1 |
| 248 | 获取纹理句柄 | `SpriteBatch::texture(&self) -> TextureHandle` | - | TextureHandle | 返回纹理 | 需求 236 | P0 |
| 249 | 设置纹理 | `SpriteBatch::set_texture(&mut self, tex: TextureHandle)` | tex | - | 切换纹理 | 需求 248 | P0 |
| 275 | SpriteBatch::new | `SpriteBatch::new(texture) -> Self` | texture | Self | 同需求 37 | 需求 37 | P0 |
| 276 | SpriteBatch::with_capacity | `SpriteBatch::with_capacity(texture, cap) -> Self` | texture, cap | Self | 同需求 237 | 需求 237 | P1 |
| 277 | SpriteBatch::add | `SpriteBatch::add(sprite, position) -> usize` | sprite, position | usize | 同需求 238 | 需求 238 | P0 |
| 278 | SpriteBatch::add_ex | `SpriteBatch::add_ex(sprite, params) -> usize` | sprite, params | usize | 同需求 239 | 需求 239 | P0 |
| 279 | SpriteBatch::set | `SpriteBatch::set(idx, sprite, position)` | idx, sprite, position | - | 同需求 240 | 需求 240 | P1 |
| 280 | SpriteBatch::remove | `SpriteBatch::remove(idx)` | idx | - | 同需求 241 | 需求 241 | P1 |
| 281 | SpriteBatch::clear | `SpriteBatch::clear()` | - | - | 同需求 242 | 需求 242 | P0 |
| 282 | SpriteBatch::len | `SpriteBatch::len() -> usize` | - | usize | 同需求 243 | 需求 243 | P1 |
| 283 | SpriteBatch::is_empty | `SpriteBatch::is_empty() -> bool` | - | bool | 同需求 244 | 需求 244 | P1 |
| 284 | SpriteBatch::capacity | `SpriteBatch::capacity() -> usize` | - | usize | 同需求 245 | 需求 245 | P1 |
| 285 | SpriteBatch::draw | `SpriteBatch::draw(ctx)` | ctx | - | 同需求 246 | 需求 246 | P0 |
| 286 | SpriteBatch::draw_at | `SpriteBatch::draw_at(ctx, x, y)` | ctx, x, y | - | 同需求 247 | 需求 247 | P1 |
| 287 | SpriteBatch::texture | `SpriteBatch::texture() -> TextureHandle` | - | TextureHandle | 同需求 248 | 需求 248 | P0 |
| 288 | SpriteBatch::set_texture | `SpriteBatch::set_texture(tex)` | tex | - | 同需求 249 | 需求 249 | P0 |

### 2.4 BatchRenderer（需求 43, 63, 289-295）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 43 | BatchRenderer | `struct BatchRenderer` | - | - | 结构完整 | 需求 63 | P0 |
| 289 | 创建 BatchRenderer | `BatchRenderer::new() -> Self` | - | Self | 创建成功 | 需求 43 | P0 |
| 290 | 开始批处理 | `BatchRenderer::begin(&mut self)` | - | - | 开始收集 | 需求 289 | P0 |
| 291 | 添加绘制 | `BatchRenderer::draw(&mut self, sprite: &Sprite, transform: Vec2)` | sprite, transform | - | 收集绘制 | 需求 290 | P0 |
| 292 | 结束批处理 | `BatchRenderer::end(&mut self, ctx: &mut RenderContext)` | ctx | - | 完成收集 | 需求 290 | P0 |
| 293 | 刷新批处理 | `BatchRenderer::flush(&mut self, ctx: &mut RenderContext)` | ctx | - | 执行绘制 | 需求 292 | P0 |
| 294 | 批次数查询 | `BatchRenderer::batches(&self) -> usize` | - | usize | 返回批次数 | 需求 289 | P1 |
| 295 | 批处理数量 | `BatchRenderer::batches() -> usize` | - | usize | 同需求 294 | 需求 294 | P1 |

---

## 3. 验收标准

### 3.1 功能验收

- [ ] `Sprite` 支持纹理、源矩形、颜色、翻转、锚点配置
- [ ] `AnimatedSprite` 支持播放/暂停/停止、帧切换、循环模式
- [ ] `SpriteBatch` 正确合批同纹理精灵，减少 draw call
- [ ] `BatchRenderer` 按纹理自动分批

### 3.2 性能验收

- [ ] `examples/batch_draw` 10k 精灵合批正常运行
- [ ] `examples/multi_sprite` 1000 随机精灵 + FPS 统计
- [ ] criterion 性能测试：10k / 100k 精灵帧率基线

### 3.3 质量验收

- [ ] `SpriteBatch::add/draw` 单元测试通过（需求 364）
- [ ] clippy 无 warning
- [ ] fmt 检查通过

---

## 4. 依赖关系图

```
┌─────────────────────────────────────┐
│              Sprite                  │
│  ├── texture / source_rect         │
│  ├── color / flip_x / flip_y       │
│  └── anchor / origin                │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│          AnimatedSprite              │
│  ├── atlas / frames / fps         │
│  ├── current_frame / loop_mode    │
│  └── play / pause / stop / update │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│           SpriteBatch                │
│  ├── texture / capacity           │
│  ├── add / set / remove / clear   │
│  └── draw / draw_at               │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│          BatchRenderer               │
│  ├── begin / draw / end / flush   │
│  └── auto-group by texture         │
└─────────────────────────────────────┘
```

---

## 5. 备注

- Sprite 内部使用顶点缓冲存储位置、UV、颜色数据
- SpriteBatch 每个精灵使用 2 个三角形（6 个顶点 + 6 个索引）
- BatchRenderer 按纹理自动分组，相同纹理连续绘制
- AnimatedSprite 帧数据来源为 TextureAtlas