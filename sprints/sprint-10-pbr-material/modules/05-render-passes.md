# 模块五：渲染 Pass 需求

## 5.1 模块概述

渲染 Pass 系统定义了 PBR 材质从绘制到输出的完整渲染管线。包含 PBR 主 Pass、阴影 Pass、HDR 管线、后期处理（Tonemapping/ColorGrading）以及 Compute Pass 支持。

**对应原需求编号**：91-107, 117-132, 341-412

---

## 5.2 核心类型定义

### 5.2.1 PbrPipeline

```rust
pub struct PbrPipeline {
    pipeline: Pipeline,
    bind_group_layout: BindGroupLayout,
}

impl PbrPipeline {
    pub fn new(renderer: &Renderer, key: PbrShaderKey) -> Result<Self>;
    pub fn bind(&self, renderer: &mut Renderer, camera: &Camera, lights: &LightBag, env: &EnvironmentMap);
    pub fn draw_mesh(&self, renderer: &mut Renderer, mesh: &Mesh, material: &PbrMaterial, transform: &Transform);
}
```

### 5.2.2 PbrPass

```rust
pub struct PbrPass {
    opaque_pass: PbrPipeline,
    transparent_pass: PbrPipeline,
    shadow_pass: ShadowMapPass,
}

impl PbrPass {
    pub fn new(renderer: &Renderer) -> Result<Self>;
    pub fn draw(&self, renderer: &mut Renderer, scene: &Scene, camera: &Camera, lights: &LightBag, env: &EnvironmentMap);
}
```

### 5.2.3 ShadowMapPass

```rust
pub struct ShadowMapPass {
    directional: DirectionalShadowMap,
    point: Option<PointShadowCubemap>,
}

impl ShadowMapPass {
    pub fn new(renderer: &Renderer, size: u32) -> Result<Self>;
    pub fn draw_directional(&self, renderer: &mut Renderer, light: &Light, scene: &Scene);
    pub fn draw_point(&self, renderer: &mut Renderer, light: &Light, scene: &Scene);
    pub fn texture(&self) -> &Texture;
}
```

### 5.2.4 ShadowQuality 枚举

```rust
pub enum ShadowQuality {
    Low(512),    // 512x512
    Medium(1024), // 1024x1024
    High(2048),   // 2048x2048
    Ultra(4096),  // 4096x4096
}
```

### 5.2.5 CascadedShadowMap

```rust
pub struct CascadedShadowMap {
    cascades: [ShadowMap; 4],
}
```

### 5.2.6 HdrPipeline

```rust
pub struct HdrPipeline {
    render_target: Texture,
    tonemap_pass: TonemapPass,
}

impl HdrPipeline {
    pub fn new(renderer: &Renderer, size: UVec2) -> Result<Self>;
    pub fn render_target(&self) -> &Texture;
    pub fn tonemap(&self, renderer: &mut Renderer, tonemapper: Tonemapper, color_grading: &ColorGrading);
}
```

### 5.2.7 Tonemapper 枚举

```rust
pub enum Tonemapper {
    Aces,
    Reinhard,
    Filmic,
    None,
}

impl Tonemapper {
    pub fn apply(&self, hdr_color: Vec3) -> Vec3;
}
```

### 5.2.8 ColorGrading

```rust
pub struct ColorGrading {
    exposure: f32,
    contrast: f32,
    saturation: f32,
    temperature: f32,
}

impl ColorGrading {
    pub fn exposure(&self) -> f32;
    pub fn set_exposure(&mut self, v: f32);
    pub fn contrast(&self) -> f32;
    pub fn set_contrast(&mut self, v: f32);
    pub fn saturation(&self) -> f32;
    pub fn set_saturation(&mut self, v: f32);
    pub fn temperature(&self) -> f32;
    pub fn set_temperature(&mut self, v: f32);
}
```

---

## 5.3 PbrPipeline API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 88, 113, 341 | `PbrPipeline::new(renderer: &Renderer, key: PbrShaderKey) -> Result<Self>` | 创建 PBR Pipeline |
| 89, 114, 342 | `PbrPipeline::bind(&self, renderer: &mut Renderer, camera: &Camera, lights: &LightBag, env: &EnvironmentMap)` | 绑定渲染状态 |
| 90, 115, 343 | `PbrPipeline::draw_mesh(&self, renderer: &mut Renderer, mesh: &Mesh, material: &PbrMaterial, transform: &Transform)` | 绘制网格 |

---

## 5.4 PbrPass API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 91, 117, 344 | `PbrPass::new(renderer: &Renderer) -> Result<Self>` | 创建 PBR Pass |
| 92, 118, 345 | `PbrPass::draw(&self, renderer: &mut Renderer, scene: &Scene, camera: &Camera, lights: &LightBag, env: &EnvironmentMap)` | 绘制场景 |

---

## 5.5 ShadowMapPass API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 93, 119, 346 | `ShadowMapPass::new(renderer: &Renderer, size: u32) -> Result<Self>` | 创建阴影 Pass |
| 94, 120, 347 | `ShadowMapPass::draw_directional(&self, renderer: &mut Renderer, light: &Light, scene: &Scene)` | 绘制方向光阴影 |
| 95, 121, 348 | `ShadowMapPass::draw_point(&self, renderer: &mut Renderer, light: &Light, scene: &Scene)` | 绘制点光源阴影 |
| 96, 122, 349 | `ShadowMapPass::texture(&self) -> &Texture` | 获取阴影贴图 |
| 97, 123 | `ShadowQuality::Low / Medium / High / Ultra` | 阴影质量级别 |
| 98, 124 | `CascadedShadowMap` | 4 级级联阴影（扩展）|

---

## 5.6 渲染 Pass 顺序

| 需求编号 | Pass | 说明 |
|----------|------|------|
| 91, 117 | `depth_pass` | 深度预Pass（可选）|
| 91, 117 | `shadow_map_pass` | 阴影贴图生成 |
| 91, 117 | `opaque_pass` | 不透明物体渲染 |
| 91, 117 | `transparent_pass` | 半透明物体渲染 |
| 91, 117 | `skybox_pass` | 天空盒渲染 |

---

## 5.7 Tonemapping 与 ColorGrading API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 82, 108, 352 | `Tonemapper::Aces / Reinhard / Filmic / None` | 色调映射算子 |
| 83, 109, 353 | `Tonemapper::apply(&self, hdr_color: Vec3) -> Vec3` | 应用色调映射 |
| 84, 110, 354 | `ColorGrading::exposure(&self) -> f32` | 获取曝光度 |
| 85, 111, 355 | `ColorGrading::set_exposure(&mut self, v: f32)` | 设置曝光度 |
| 110, 356 | `ColorGrading::contrast(&self) -> f32` | 获取对比度 |
| 110, 357 | `ColorGrading::set_contrast(&mut self, v: f32)` | 设置对比度 |
| 110, 358 | `ColorGrading::saturation(&self) -> f32` | 获取饱和度 |
| 110, 359 | `ColorGrading::set_saturation(&mut self, v: f32)` | 设置饱和度 |
| 110, 360 | `ColorGrading::temperature(&self) -> f32` | 获取色温 |
| 110, 361 | `ColorGrading::set_temperature(&mut self, v: f32)` | 设置色温 |
| 111 | `ColorGradingLUT` | 3D LUT 颜色查找（后续）|

---

## 5.8 HdrPipeline API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 112, 149, 362 | `HdrPipeline::new(renderer: &Renderer, size: UVec2) -> Result<Self>` | 创建 HDR 管线 |
| 113, 150, 363 | `HdrPipeline::render_target(&self) -> &Texture` | 获取 HDR 渲染目标 |
| 113, 151, 364 | `HdrPipeline::tonemap(&self, renderer: &mut Renderer, tonemapper: Tonemapper, color_grading: &ColorGrading)` | 执行色调映射 |

---

## 5.9 Compute Pass

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 130 | `ComputePass::new() -> Self` | 创建 Compute Pass |
| 131 | `ComputeShader::new(src: &str) -> Result<Self>` | 创建 Compute Shader |
| 132 | `ComputeShader::dispatch(renderer: &mut Renderer, groups_x: u32, groups_y: u32, groups_z: u32)` | 分发计算任务 |
| 156 | `ComputePass::new()` | IBL 烘焙用 Compute Pass |

---

## 5.10 纹理/采样器/缓冲 API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 101, 159 | `TextureCompiler::new(renderer)` | 创建纹理编译器 |
| 102, 160 | `TextureCompiler::compile_file(path, options) -> Result<Texture>` | 编译文件 |
| 103, 161 | `TextureCompiler::compile_bytes(bytes, options) -> Result<Texture>` | 编译字节 |
| 104, 162 | `TextureCompiler::compile_image(image, options) -> Result<Texture>` | 编译图像 |
| 105 | 自动生成 mipmap | Mipmap 生成 |
| 106 | BCn/ETC2/ASTC 压缩 | 平台相关压缩 |
| 107 | sRGB/Linear 选择 | 颜色空间选择 |
| 104 | Normal map BC5/RG | 法线贴图格式 |
| 105 | HDR RGBA16F/32F | HDR 纹理格式 |
| 369-375 | `TextureOptions` 构建器 | 配置纹理选项 |
| 376 | `Compression::None/BCn/ETC2/ASTC` | 压缩格式 |
| 377 | `Texture::width() -> u32` | 纹理宽度 |
| 378 | `Texture::height() -> u32` | 纹理高度 |
| 379 | `Texture::depth_or_layers() -> u32` | 纹理深度/层数 |
| 380 | `Texture::mip_levels() -> u32` | Mip 级别数 |
| 381 | `Texture::format() -> TextureFormat` | 纹理格式 |
| 382 | `Texture::is_srgb() -> bool` | 是否 sRGB |
| 383 | `Texture::generate_mipmaps(renderer)` | 生成 Mipmap |
| 384 | `Sampler::nearest(renderer) -> Handle<Sampler>` | 最近邻采样 |
| 385 | `Sampler::linear(renderer) -> Handle<Sampler>` | 双线性采样 |
| 386 | `Sampler::trilinear(renderer) -> Handle<Sampler>` | 三线性采样 |
| 387 | `Sampler::anisotropic(renderer, level) -> Handle<Sampler>` | 各向异性采样 |
| 388 | `Sampler::comparison(renderer, func) -> Handle<Sampler>` | 比较模式采样 |
| 389 | `Sampler::repeat(renderer) -> Handle<Sampler>` | 重复寻址 |
| 390 | `Buffer::new(renderer, usage, size, data) -> Result<Self>` | 创建缓冲 |
| 391 | `Buffer::write(&self, renderer, offset, data)` | 写入缓冲 |
| 392 | `Buffer::read_back(&self, renderer) -> Result<Vec<u8>>` | 读取缓冲 |
| 393 | `Buffer::size(&self) -> u64` | 缓冲大小 |

---

## 5.11 Bind Group API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 141, 167 | `BindGroupLayoutBuilder::new()` | 创建布局构建器 |
| 142, 168 | `BindGroupLayoutBuilder::add(binding: u32, ty: BindingType)` | 添加绑定 |
| 143, 169 | `BindGroupLayoutBuilder::build(&self, renderer) -> BindGroupLayout` | 构建布局 |
| 144, 170 | `BindGroupBuilder::new()` | 创建绑定组构建器 |
| 145, 171 | `BindGroupBuilder::add_buffer(binding, buffer)` | 添加缓冲 |
| 146, 172 | `BindGroupBuilder::add_sampler(binding, sampler)` | 添加采样器 |
| 147, 173 | `BindGroupBuilder::add_texture(binding, texture)` | 添加纹理 |
| 148, 174 | `BindGroupBuilder::build(&self, renderer, layout) -> BindGroup` | 构建绑定组 |
| 149, 175 | `BindGroupCache::get(&self, key) -> Option<BindGroup>` | 获取缓存 |
| 150, 176 | `BindGroupCache::insert(&mut self, key, bg)` | 插入缓存 |
| 166 | `BindingType` | uniform/storage/sampler/texture |
| 167 | `BindGroupLayoutBuilder` 流畅构建 | Builder 模式 |
| 168 | `BindGroupBuilder` 流畅构建 | Builder 模式 |

---

## 5.12 RenderGraph 接口

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 124, 151 | `RenderGraph::add_pass(name, deps, fn)` | 添加 Pass |
| 124, 152 | `RenderGraph::compile() -> PassOrder` | 编译图 |
| 124, 153 | `RenderGraph::execute(renderer)` | 执行图 |
| 125 | `RenderPassDescriptor` | 颜色/深度/模板附件 |
| 126 | `RenderPassEncoder` | draw/dispatch 封装 |
| 127 | `RenderGraph::add_pass` | Pass 级别依赖图 |

---

## 5.13 错误处理

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 144, 170 | `RenderError::ShaderCompile(msg, line) -> Self` | Shader 编译错误 |
| 145, 171 | `RenderError::to_string(&self) -> String` | 错误转字符串 |
| 146, 172 | `RenderError::span(&self) -> Option<Range<usize>>` | 错误位置 |
| 144, 406-408 | `RenderError` 枚举 | Shader/Pipeline/Resource/Other |
| 463 | shader 编译失败回退到 unlit | 开发模式容错 |
| 464 | `DebugMessageCallback` | GPU 调试消息 |
| 465 | validation feature | CI 中验证 |
| 466 | `gpu_debug_marker(group, name)` | RenderDoc 标记 |
| 467 | `gpu_timestamp_query` | Pass 耗时测量 |

---

## 5.14 PipelineCache

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 87, 113 | `PbrShaderKey` | 根据材质特性构建 key |
| 113 | `PipelineCache` 持久化 | 加速后续启动 |
| 454 | `PipelineCache::save(&self, dir) -> Result<()>` | 保存缓存 |
| 455 | `PipelineCache::load(&mut self, dir) -> Result<()>` | 加载缓存 |

---

## 5.15 输入与输出

### 输入
- Scene 场景数据
- Camera 相机数据
- LightBag 光源数据
- EnvironmentMap 环境光照
- PbrMaterial 材质数据

### 输出
- 渲染到 Framebuffer 的图像
- 阴影贴图
- HDR 渲染目标

---

## 5.16 验收标准

| 编号 | 标准 |
|------|------|
| 91 | `PbrPass::draw` 正确渲染场景 |
| 93 | `ShadowMapPass` 生成正确阴影 |
| 97 | `ShadowQuality` 正确切换分辨率 |
| 108 | `Tonemapper` 四种模式正确工作 |
| 112 | `HdrPipeline` HDR 渲染正确 |
| 117 | Pass 顺序：depth -> shadow -> opaque -> transparent -> skybox |
| 149 | ACES tonemapper 对 HDR 正确压缩 |
| 438 | 单测 `PbrPipeline::new` 构建成功 |
| 421 | `examples/pbr_tonemap` 切换色调映射算子 |
| 422 | `examples/pbr_shadow` 显示阴影 |

---

## 5.17 依赖关系

### 依赖模块
- `PbrMaterial`: 材质数据
- `PbrShader`: Shader 代码
- `TextureCompiler`: 纹理处理
- `IBLBaker`: 环境光照

### 被依赖模块
- `engine-render`: 底层渲染抽象

---

## 5.18 优先级

| 优先级 | 需求编号 | 说明 |
|--------|----------|------|
| P0 | 91-97, 117-123, 341-349, 388-393 | 核心渲染 Pass |
| P1 | 108-112, 352-364, 454-455 | Tonemapping 与缓存 |
| P2 | 124-132, 151-157, 170-176, 406-408 | Compute Pass 与错误处理 |
