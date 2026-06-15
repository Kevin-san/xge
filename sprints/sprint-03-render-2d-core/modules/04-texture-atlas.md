# 模块四：纹理图集需求

## 1. 模块概述

本模块提供纹理图集（TextureAtlas）功能，将多张纹理合成为一张大纹理，并提供 UV 坐标查询和图集打包算法支持。

**核心目标**：支持多张图像合成一张大图、UV 坐标映射、打包算法（Skyline/Guillotine）。

---

## 2. 需求清单

### 2.1 TextureAtlas 核心（需求 47-57, 72-78, 256-272, 297-313）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 47 | TextureAtlas 结构 | `struct TextureAtlas` | - | - | 结构完整 | 需求 11 | P0 |
| 48 | TextureAtlasBuilder | `struct TextureAtlasBuilder` | - | - | 构建器完整 | 需求 47 | P0 |
| 49 | 按 max_size/padding/algorithm 合并 | `TextureAtlasBuilder::with_padding(pixels)` | pixels | Self | 链式调用 | 需求 48 | P0 |
| 50 | 从 images 合成 | `TextureAtlas::from_images(images: Vec<Image>) -> (Atlas, Vec<(index, Rect)>)` | images | (Atlas, rects) | 合成成功 | 需求 44 | P0 |
| 51 | 获取 UV | `TextureAtlas::get_uv(index: usize) -> Option<(Vec2, Vec2)>` | index | Option<(UV0, UV1)> | 返回 UV | 需求 47 | P0 |
| 52 | 获取矩形 | `TextureAtlas::get_rect(index: usize) -> Option<Rect>` | index | Option<Rect> | 返回像素矩形 | 需求 47 | P0 |
| 53 | 纹理数量 | `TextureAtlas::num_textures(&self) -> usize` | - | usize | 返回数量 | 需求 47 | P0 |
| 54 | 图集尺寸 | `TextureAtlas::size(&self) -> (u32, u32)` | - | (u32, u32) | 返回宽高 | 需求 47 | P0 |
| 72 | Skyline 算法 | `PackAlgorithm::Skyline` | - | - | 正确实现 | 需求 48 | P0 |
| 73 | Guillotine 算法 | `PackAlgorithm::Guillotine` | - | - | 正确实现 | 需求 48 | P0 |
| 74 | 图集打包 Skyline | `SkylinePacker` | - | - | 实现正确 | 需求 72 | P0 |
| 75 | 图集打包 Guillotine | `GuillotinePacker` | - | - | 实现正确 | 需求 73 | P0 |
| 76 | Skyline 合并 | `Skyline::add(width, height) -> Rect` | w, h | Rect | 返回位置 | 需求 74 | P0 |
| 77 | Guillotine 合并 | `Guillotine::add(width, height) -> Rect` | w, h | Rect | 返回位置 | 需求 75 | P0 |
| 78 | PackResult 碰撞检测 | `PackResult::contains_collisions(&self) -> bool` | - | bool | 检测正确 | 需求 48 | P1 |
| 256 | 创建构建器 | `TextureAtlasBuilder::new(max_size: u32) -> Self` | max_size | Self | 创建成功 | 需求 48 | P0 |
| 257 | 设置边距 | `TextureAtlasBuilder::with_padding(pixels: u32) -> Self` | pixels | Self | 同需求 49 | 需求 256 | P0 |
| 258 | 设置算法 | `TextureAtlasBuilder::with_algorithm(algorithm: PackAlgorithm) -> Self` | algorithm | Self | 切换算法 | 需求 72-73 | P0 |
| 259 | 添加图像 | `TextureAtlasBuilder::add(&mut self, image: &Image) -> usize` | image | usize | 返回索引 | 需求 256 | P0 |
| 260 | 从文件添加 | `TextureAtlasBuilder::add_from_file(&mut self, path: &str) -> Result<usize>` | path | Result<usize> | 加载并添加 | 需求 259 | P0 |
| 261 | 构建图集 | `TextureAtlasBuilder::build(&self, ctx: &RenderContext) -> Result<(TextureAtlas, Vec<Rect>)>` | ctx | (Atlas, rects) | 构建成功 | 需求 256 | P0 |
| 262 | 获取纹理句柄 | `TextureAtlas::texture(&self) -> TextureHandle` | - | TextureHandle | 返回大图 | 需求 47 | P0 |
| 263 | 获取尺寸 | `TextureAtlas::size(&self) -> (u32, u32)` | - | (u32, u32) | 同需求 54 | 需求 54 | P0 |
| 264 | 图集长度 | `TextureAtlas::len(&self) -> usize` | - | usize | 返回子图数量 | 需求 53 | P0 |
| 265 | 是否为空 | `TextureAtlas::is_empty(&self) -> bool` | - | bool | 返回状态 | 需求 264 | P0 |
| 266 | 按索引获取矩形 | `TextureAtlas::get(&self, idx: usize) -> Option<Rect>` | idx | Option<Rect> | 同需求 52 | 需求 52 | P0 |
| 267 | 按索引获取 UV | `TextureAtlas::get_uv(&self, idx: usize) -> Option<(Vec2, Vec2)>` | idx | Option<(Vec2, Vec2)> | 同需求 51 | 需求 51 | P0 |
| 268 | 按索引获取 Sprite | `TextureAtlas::get_sprite(&self, idx: usize) -> Sprite` | idx | Sprite | 创建精灵 | 需求 24 | P0 |
| 269 | 打包算法枚举 | `enum PackAlgorithm { Skyline, Guillotine }` | - | - | 同需求 72-73 | 需求 72-73 | P0 |
| 270 | 碰撞检测 | `PackResult::contains_collisions(&self) -> bool` | - | bool | 同需求 78 | 需求 78 | P1 |
| 297 | TextureAtlasBuilder::new | `TextureAtlasBuilder::new(max_size)` | max_size | Self | 同需求 256 | 需求 256 | P0 |
| 298 | with_padding | `TextureAtlasBuilder::with_padding(pixels)` | pixels | Self | 同需求 257 | 需求 257 | P0 |
| 299 | with_algorithm | `TextureAtlasBuilder::with_algorithm(algorithm)` | algorithm | Self | 同需求 258 | 需求 258 | P0 |
| 300 | add | `TextureAtlasBuilder::add(image) -> usize` | image | usize | 同需求 259 | 需求 259 | P0 |
| 301 | add_from_file | `TextureAtlasBuilder::add_from_file(path) -> Result<usize>` | path | Result<usize> | 同需求 260 | 需求 260 | P0 |
| 302 | build | `TextureAtlasBuilder::build(ctx) -> Result<(TextureAtlas, Vec<Rect>)>` | ctx | Result | 同需求 261 | 需求 261 | P0 |
| 303 | texture | `TextureAtlas::texture() -> TextureHandle` | - | TextureHandle | 同需求 262 | 需求 262 | P0 |
| 304 | size | `TextureAtlas::size() -> (u32, u32)` | - | (u32, u32) | 同需求 263 | 需求 263 | P0 |
| 305 | len | `TextureAtlas::len() -> usize` | - | usize | 同需求 264 | 需求 264 | P0 |
| 306 | is_empty | `TextureAtlas::is_empty() -> bool` | - | bool | 同需求 265 | 需求 265 | P0 |
| 307 | get | `TextureAtlas::get(idx) -> Option<Rect>` | idx | Option<Rect> | 同需求 266 | 需求 266 | P0 |
| 308 | get_uv | `TextureAtlas::get_uv(idx) -> Option<(Vec2, Vec2)>` | idx | Option<(Vec2, Vec2)> | 同需求 267 | 需求 267 | P0 |
| 309 | get_sprite | `TextureAtlas::get_sprite(idx) -> Sprite` | idx | Sprite | 同需求 268 | 需求 268 | P0 |
| 310 | PackAlgorithm::Skyline | `PackAlgorithm::Skyline` | - | - | 同需求 72 | 需求 72 | P0 |
| 311 | PackAlgorithm::Guillotine | `PackAlgorithm::Guillotine` | - | - | 同需求 73 | 需求 73 | P0 |
| 312 | contains_collisions | `PackResult::contains_collisions() -> bool` | - | bool | 同需求 270 | 需求 270 | P1 |
| 313 | PackResult | `struct PackResult` | - | - | 结构完整 | 需求 48 | P0 |

---

## 3. 验收标准

### 3.1 功能验收

- [ ] `TextureAtlasBuilder` 可添加多张图像并合成大图
- [ ] UV 坐标正确映射到子图区域
- [ ] Skyline 和 Guillotine 两种打包算法正确实现
- [ ] `PackResult::contains_collisions` 可检测打包冲突

### 3.2 质量验收

- [ ] `TextureAtlas` 打包不越界单元测试通过（需求 367）
- [ ] clippy 无 warning
- [ ] fmt 检查通过
- [ ] `examples/atlas_animation` 可播放动画

---

## 4. 依赖关系图

```
┌─────────────────────────────────────┐
│        TextureAtlasBuilder           │
│  ├── new(max_size)                  │
│  ├── with_padding / with_algorithm │
│  └── add / add_from_file / build    │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│          TextureAtlas                │
│  ├── texture (合成后的大图)          │
│  ├── get(index) -> Rect             │
│  ├── get_uv(index) -> (UV0, UV1)    │
│  └── get_sprite(index) -> Sprite    │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│        PackAlgorithm                 │
│  ├── Skyline (高效，适合规则图)      │
│  └── Guillotine (灵活，适合复杂图)   │
└─────────────────────────────────────┘
```

---

## 5. 备注

- 图集打包是 2D 游戏优化 draw call 的关键手段
- Skyline 算法适合宽度相近的图像，Guillotine 更灵活
- UV 坐标范围为 [0.0, 1.0]，需要根据图集实际尺寸计算
- 打包结果应检测碰撞，确保无子图重叠