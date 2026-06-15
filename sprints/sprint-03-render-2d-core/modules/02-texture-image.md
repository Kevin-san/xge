# 模块二：纹理与图像需求

## 1. 模块概述

本模块提供 2D 纹理和图像处理能力，包括 `Texture2D` GPU 纹理资源管理、`Image` CPU 端像素数据处理、`Sampler` 采样器配置以及 `TextureManager` 纹理资源管理器。

**核心目标**：支持从文件加载纹理、CPU 端图像处理、GPU 纹理上传与管理。

---

## 2. 需求清单

### 2.1 Texture2D 核心（需求 10-18, 37-41, 161-174, 199-205）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 10 | Texture2D 创建销毁 | `Texture2D::new(ctx, desc) / drop` | ctx, desc | Texture2D | 正常创建释放 | 需求 2 | P0 |
| 11 | 纹理格式支持 | `enum TextureFormat { RGBA8, RGBA16, R8, BGRA8 }` | - | - | 格式正确 | 需求 10 | P0 |
| 12 | 从文件加载纹理 | `Texture2D::from_file(ctx, path) -> Result<Self>` | ctx, path | Texture2D | PNG/JPG/BMP/GIF 加载 | 需求 10 | P0 |
| 13 | 纹理尺寸查询 | `Texture2D::width(&self) -> u32` | - | u32 | 返回宽度 | 需求 10 | P0 |
| 14 | 纹理尺寸查询 | `Texture2D::height(&self) -> u32` | - | u32 | 返回高度 | 需求 10 | P0 |
| 15 | 纹理尺寸元组 | `Texture2D::size(&self) -> (u32, u32)` | - | (u32, u32) | 返回宽高 | 需求 13-14 | P0 |
| 16 | 更新纹理区域 | `Texture2D::update(&mut self, rect: Rect, data: &[u8])` | rect, data | - | 区域更新成功 | 需求 10 | P0 |
| 17 | Texture2DBuilder | `struct Texture2DBuilder` | - | - | 链式 API | 需求 10 | P1 |
| 18 | 采样器创建 | `Sampler::new(ctx, desc)` | ctx, desc | Sampler | 创建成功 | 需求 42 | P0 |
| 37 | RGBA8 格式 | `TextureFormat::RGBA8` | - | - | 支持 | 需求 11 | P0 |
| 38 | RGBA16 格式 | `TextureFormat::RGBA16F` | - | - | 支持 | 需求 11 | P0 |
| 39 | R8 格式 | `TextureFormat::R8` | - | - | 支持 | 需求 11 | P0 |
| 40 | BGRA8 格式 | `TextureFormat::BGRA8` | - | - | 支持 | 需求 11 | P0 |
| 41 | PNG/JPG/BMP/GIF 加载 | 使用 `image` crate | - | - | 格式支持 | 需求 12 | P0 |
| 42 | Sampler 过滤器 | `enum FilterMode { Linear, Nearest }` | - | - | 正确枚举 | 需求 18 | P0 |
| 43 | Sampler 环绕模式 | `enum WrapMode { Clamp, Repeat, MirrorRepeat }` | - | - | 正确枚举 | 需求 18 | P0 |
| 161 | 从 Image 创建 | `Texture2D::from_image(ctx, image: &Image) -> Result<Self>` | ctx, image | Texture2D | 上传成功 | 需求 44 | P0 |
| 162 | 创建空纹理 | `Texture2D::empty(ctx, w: u32, h: u32, format: TextureFormat) -> Result<Self>` | ctx, w, h, format | Texture2D | 空纹理创建 | 需求 10 | P0 |
| 163 | 从文件创建 | `Texture2D::from_file(ctx, path) -> Result<Self>` | ctx, path | Texture2D | 同需求 12 | 需求 12 | P0 |
| 164 | 从字节创建 | `Texture2D::from_bytes(ctx, bytes: &[u8]) -> Result<Self>` | ctx, bytes | Texture2D | 字节加载 | 需求 12 | P0 |
| 165 | 宽度获取 | `Texture2D::width(&self) -> u32` | - | u32 | 同需求 13 | 需求 13 | P0 |
| 166 | 高度获取 | `Texture2D::height(&self) -> u32` | - | u32 | 同需求 14 | 需求 14 | P0 |
| 167 | 尺寸获取 | `Texture2D::size(&self) -> (u32, u32)` | - | (u32, u32) | 同需求 15 | 需求 15 | P0 |
| 168 | 格式获取 | `Texture2D::format(&self) -> TextureFormat` | - | TextureFormat | 返回格式 | 需求 11 | P0 |
| 169 | 设置过滤器 | `Texture2D::set_filter(&mut self, filter: FilterMode)` | filter | - | 过滤器生效 | 需求 42 | P1 |
| 170 | 设置环绕模式 | `Texture2D::set_wrap(&mut self, wrap: WrapMode)` | wrap | - | 环绕生效 | 需求 43 | P1 |
| 171 | 更新区域 | `Texture2D::update(&mut self, rect: Rect, data: &[u8])` | rect, data | - | 同需求 16 | 需求 16 | P0 |
| 172 | 生成多级渐远纹理 | `Texture2D::generate_mipmaps(&mut self)` | - | - | Mipmap 生成 | 需求 10 | P1 |
| 173 | 获取句柄 | `Texture2D::handle(&self) -> TextureHandle` | - | TextureHandle | 返回句柄 | 需求 10 | P1 |
| 174 | 纹理格式枚举 | `TextureFormat::RGBA8 / RGBA16F / R8 / BGRA8` | - | - | 同需求 11 | 需求 11 | P0 |
| 199 | Sampler 句柄 | `Sampler::handle(&self) -> SamplerHandle` | - | SamplerHandle | 返回句柄 | 需求 18 | P1 |

### 2.2 Image CPU 端处理（需求 19-27, 44-49, 213-229）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 19 | 从文件加载 Image | `Image::from_file(path) -> Result<Image>` | path | Image | 文件加载 | 需求 12 | P0 |
| 20 | 从字节加载 | `Image::from_bytes(bytes: &[u8]) -> Result<Image>` | bytes | Image | 字节加载 | 需求 12 | P0 |
| 21 | 从 RGBA 数据创建 | `Image::from_rgba(width: u32, height: u32, data: Vec<u8>) -> Self` | width, height, data | Image | RGBA 创建 | 需求 19 | P0 |
| 22 | 保存到文件 | `Image::save(&self, path: &str) -> Result<()>` | path | Result | 保存成功 | 需求 19 | P0 |
| 23 | 裁剪区域 | `Image::region(&self, x: u32, y: u32, w: u32, h: u32) -> Image` | x, y, w, h | Image | 裁剪成功 | 需求 19 | P0 |
| 44 | Image 未上传 GPU | `struct Image` | - | - | CPU 端数据 | 需求 19 | P0 |
| 45 | Image::from_file | `Image::from_file(path) -> Result<Image>` | path | Result<Image> | 同需求 19 | 需求 19 | P0 |
| 46 | Image::from_bytes | `Image::from_bytes(bytes) -> Result<Image>` | bytes | Result<Image> | 同需求 20 | 需求 20 | P0 |
| 47 | Image::from_rgba | `Image::from_rgba(width, height, data) -> Self` | width, height, data | Image | 同需求 21 | 需求 21 | P0 |
| 48 | Image::save | `Image::save(path) -> Result<()>` | path | Result | 同需求 22 | 需求 22 | P0 |
| 49 | Image::region | `Image::region(x, y, w, h) -> Image` | x, y, w, h | Image | 同需求 23 | 需求 23 | P0 |
| 213 | 从像素创建 | `Image::from_pixels(width: u32, height: u32, data: Vec<u8>) -> Self` | width, height, data | Self | 同需求 21 | 需求 21 | P0 |
| 214 | 从文件创建 | `Image::from_file(path) -> Result<Self>` | path | Result<Self> | 同需求 45 | 需求 45 | P0 |
| 215 | 从字节创建 | `Image::from_bytes(bytes: &[u8]) -> Result<Self>` | bytes | Result<Self> | 同需求 46 | 需求 46 | P0 |
| 216 | 宽度获取 | `Image::width(&self) -> u32` | - | u32 | 返回宽度 | 需求 21 | P0 |
| 217 | 高度获取 | `Image::height(&self) -> u32` | - | u32 | 返回高度 | 需求 21 | P0 |
| 218 | 尺寸获取 | `Image::size(&self) -> (u32, u32)` | - | (u32, u32) | 返回宽高 | 需求 216-217 | P0 |
| 219 | 保存图像 | `Image::save(&self, path: &str) -> Result<()>` | path | Result | 同需求 48 | 需求 48 | P0 |
| 220 | 裁剪图像 | `Image::crop(&self, rect: Rect) -> Image` | rect | Image | 同需求 49 | 需求 49 | P0 |
| 221 | 水平翻转 | `Image::flip_horizontal(&mut self)` | - | - | 翻转成功 | 需求 19 | P0 |
| 222 | 垂直翻转 | `Image::flip_vertical(&mut self)` | - | - | 翻转成功 | 需求 19 | P0 |
| 223 | 旋转 90 度 | `Image::rotate_90(&mut self)` | - | - | 顺时针 90° | 需求 19 | P0 |
| 224 | 旋转 180 度 | `Image::rotate_180(&mut self)` | - | - | 180° 旋转 | 需求 19 | P0 |
| 225 | 旋转 270 度 | `Image::rotate_270(&mut self)` | - | - | 逆时针 90° | 需求 19 | P0 |
| 226 | 缩放图像 | `Image::resize(&mut self, new_w: u32, new_h: u32)` | new_w, new_h | - | 尺寸改变 | 需求 19 | P1 |
| 227 | 获取像素只读引用 | `Image::pixels(&self) -> &[u8]` | - | &[u8] | 返回像素数据 | 需求 21 | P0 |
| 228 | 获取像素可变引用 | `Image::pixels_mut(&mut self) -> &mut [u8]` | - | &mut [u8] | 可修改像素 | 需求 21 | P0 |

### 2.3 SamplerBuilder（需求 17, 43, 229-235）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 17 | Texture2DBuilder Fluent API | `Texture2DBuilder::new()` | - | Self | 链式构建 | 需求 10 | P1 |
| 43 | SamplerBuilder | `struct SamplerBuilder` | - | - | 采样器构建 | 需求 18 | P1 |
| 229 | SamplerBuilder 新实例 | `SamplerBuilder::new()` | - | Self | 创建成功 | 需求 43 | P1 |
| 230 | 设置过滤器 | `SamplerBuilder::with_filter(mag: FilterMode, min: FilterMode) -> Self` | mag, min | Self | 链式调用 | 需求 42 | P1 |
| 231 | 设置环绕模式 | `SamplerBuilder::with_wrap(s: WrapMode, t: WrapMode) -> Self` | s, t | Self | 链式调用 | 需求 43 | P1 |
| 232 | 设置 mipmap 过滤器 | `SamplerBuilder::with_mipmap_filter(mode: FilterMode) -> Self` | mode | Self | 链式调用 | 需求 42 | P1 |
| 233 | 设置各向异性 | `SamplerBuilder::with_anisotropy(level: u8) -> Self` | level | Self | 链式调用 | 需求 43 | P1 |
| 234 | 构建 Sampler | `SamplerBuilder::build(&self, ctx: &RenderContext) -> Sampler` | ctx | Sampler | 创建成功 | 需求 43 | P1 |
| 235 | Sampler 句柄 | `Sampler::handle(&self) -> SamplerHandle` | - | SamplerHandle | 同需求 199 | 需求 199 | P1 |

### 2.4 TextureManager（需求 200-205）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 200 | TextureManager 创建 | `TextureManager::new()` | - | Self | 创建成功 | 需求 10 | P1 |
| 201 | 加载纹理 | `TextureManager::load(&mut self, path: &str) -> Result<TextureHandle>` | path | Result<TextureHandle> | 加载并缓存 | 需求 12 | P1 |
| 202 | 获取纹理 | `TextureManager::get(&self, handle: TextureHandle) -> Option<&Texture2D>` | handle | Option<&Texture2D> | 返回引用 | 需求 10 | P1 |
| 203 | 卸载纹理 | `TextureManager::unload(&mut self, handle: TextureHandle)` | handle | - | 释放显存 | 需求 10 | P1 |
| 204 | 重新加载纹理 | `TextureManager::reload(&mut self, handle: TextureHandle) -> Result<()>` | handle | Result | 重新加载 | 需求 203 | P1 |
| 205 | 迭代器 | `TextureManager::iter(&self) -> impl Iterator` | - | Iterator | 遍历所有纹理 | 需求 200 | P1 |

### 2.5 FilterMode 与 WrapMode（需求 211-212）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 211 | 线性过滤 | `FilterMode::Linear` | - | - | 正确枚举值 | 需求 42 | P0 |
| 212 | 最近邻过滤 | `FilterMode::Nearest` | - | - | 正确枚举值 | 需求 42 | P0 |
| 176 | 环绕 Clamp | `WrapMode::Clamp` | - | - | 正确枚举值 | 需求 43 | P0 |
| 177 | 环绕 Repeat | `WrapMode::Repeat` | - | - | 正确枚举值 | 需求 43 | P0 |
| 178 | 环绕 MirrorRepeat | `WrapMode::MirrorRepeat` | - | - | 正确枚举值 | 需求 43 | P0 |

---

## 3. 验收标准

### 3.1 功能验收

- [ ] `Texture2D::from_file()` 成功加载 PNG/JPG/BMP/GIF
- [ ] `Texture2D::update()` 正确更新纹理区域
- [ ] `Image` 支持裁剪、翻转、旋转操作
- [ ] `Sampler` 支持 Linear/Nearest 过滤和 Clamp/Repeat/MirrorRepeat 环绕
- [ ] `TextureManager` 正确管理纹理生命周期

### 3.2 质量验收

- [ ] `Image::from_bytes` 单元测试通过（需求 362）
- [ ] `Texture::update` 单元测试通过（需求 363）
- [ ] clippy 无 warning
- [ ] fmt 检查通过

---

## 4. 依赖关系图

```
┌─────────────────────────────────────┐
│            Image (CPU)              │
│  ├── from_file / from_bytes        │
│  ├── crop / flip / rotate / resize │
│  └── pixels / pixels_mut           │
└─────────────────────────────────────┘
                  │ upload()
                  ▼
┌─────────────────────────────────────┐
│          Texture2D (GPU)            │
│  ├── from_image / from_file        │
│  ├── update / generate_mipmaps    │
│  └── handle / format / size        │
└─────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│      TextureManager                 │
│  ├── load / get / unload          │
│  └── reload / iter                 │
└─────────────────────────────────────┘
```

---

## 5. 备注

- 需求 113（纹理压缩 ETC1/BCn）标记为留位
- `Image` 作为 CPU 端数据，不直接绑定到渲染管线
- `TextureManager` 提供引用计数式生命周期管理