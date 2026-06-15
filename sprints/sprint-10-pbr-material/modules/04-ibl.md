# 模块四：IBL 环境光照需求

## 4.1 模块概述

Image-Based Lighting (IBL) 系统提供基于环境贴图的光照支持。包含 IBL Baker 离线烘焙工具、EnvironmentMap HDR 贴图处理、Skybox 天空盒渲染，以及与 PBR 着色器的集成。

**对应原需求编号**：60-75, 86-101, 324-385

---

## 4.2 核心类型定义

### 4.2.1 EnvironmentMap

```rust
pub struct EnvironmentMap {
    skybox: CubeMap,
    irradiance: CubeMap,
    prefilter: CubeMap,
    brdf_lut: Texture2D,
    intensity: f32,
}

impl EnvironmentMap {
    pub fn from_hdr(path: &Path) -> Result<Self>;
    pub fn from_equirectangular(texture: Texture) -> Result<Self>;
    pub fn skybox(&self) -> CubeMap;
    pub fn irradiance(&self) -> CubeMap;
    pub fn prefilter(&self) -> CubeMap;
    pub fn brdf_lut(&self) -> Texture2D;
    pub fn intensity(&self) -> f32;
    pub fn set_intensity(&mut self, v: f32);
}
```

### 4.2.2 IBLBaker

```rust
pub struct IBLBaker {
    renderer: Renderer,
}

impl IBLBaker {
    pub fn new(renderer: &Renderer) -> Result<Self>;
    pub fn bake_irradiance(&self, env_map: &EnvironmentMap) -> CubeMap;
    pub fn bake_prefilter(&self, env_map: &EnvironmentMap, levels: u32) -> CubeMap;
    pub fn bake_brdf_lut(&self, size: u32) -> Texture2D;
    pub fn save_cache(&self, dir: &Path) -> Result<()>;
    pub fn load_cache(&mut self, dir: &Path) -> Result<()>;
}
```

### 4.2.3 SkyboxRenderer

```rust
pub struct SkyboxRenderer {
    pipeline: Pipeline,
    bind_group: BindGroup,
}

impl SkyboxRenderer {
    pub fn new(renderer: &Renderer) -> Result<Self>;
    pub fn draw(&self, renderer: &mut Renderer, camera: &Camera, env: &EnvironmentMap);
    pub fn set_skybox_texture(&mut self, cube_map: CubeMap);
}
```

---

## 4.3 IBLBaker API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 60, 86, 324 | `IBLBaker::new(renderer: &Renderer) -> Result<Self>` | 创建 Baker |
| 61, 87, 325 | `IBLBaker::bake_irradiance(&self, env_map: &EnvironmentMap) -> CubeMap` | 烘焙辐照度贴图 |
| 62, 88, 326 | `IBLBaker::bake_prefilter(&self, env_map: &EnvironmentMap, levels: u32) -> CubeMap` | 烘焙预滤波贴图 |
| 63, 89, 327 | `IBLBaker::bake_brdf_lut(&self, size: u32) -> Texture2D` | 烘焙 BRDF LUT |
| 64, 90 | `IBLBaker::save_cache(&self, dir: &Path) -> Result<()>` | 保存烘焙缓存 |
| 65, 91 | `IBLBaker::load_cache(&mut self, dir: &Path) -> Result<()>` | 加载烘焙缓存 |
| 66, 92 | 开发期实时烘焙 | 从 HDR env 贴图实时烘焙 |
| 67, 93 | Release 预烘焙 | 预烘焙二进制缓存 |

---

## 4.4 EnvironmentMap API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 66, 92, 330 | `EnvironmentMap::from_hdr(path: &Path) -> Result<Self>` | 从 HDR 文件加载 |
| 66, 93, 331 | `EnvironmentMap::from_equirectangular(texture: Texture) -> Result<Self>` | 从等距柱状投影纹理创建 |
| 67, 94, 332 | `EnvironmentMap::skybox(&self) -> CubeMap` | 获取天空盒贴图 |
| 68, 95, 333 | `EnvironmentMap::irradiance(&self) -> CubeMap` | 获取辐照度贴图 |
| 69, 96, 334 | `EnvironmentMap::prefilter(&self) -> CubeMap` | 获取预滤波贴图 |
| 70, 97, 335 | `EnvironmentMap::brdf_lut(&self) -> Texture2D` | 获取 BRDF LUT |
| 336 | `EnvironmentMap::intensity(&self) -> f32` | 获取光照强度 |
| 337 | `EnvironmentMap::set_intensity(&mut self, v: f32)` | 设置光照强度 |

---

## 4.5 SkyboxRenderer API

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 71, 98, 338 | `SkyboxRenderer::new(renderer: &Renderer) -> Result<Self>` | 创建天空盒渲染器 |
| 72, 99, 339 | `SkyboxRenderer::draw(&self, renderer: &mut Renderer, camera: &Camera, env: &EnvironmentMap)` | 绘制天空盒 |
| 73, 100 | `SkyboxRenderer::set_skybox_texture(&mut self, cube_map: CubeMap)` | 设置天空盒纹理 |
| 74, 101 | 关闭深度写入 | 天空盒绘制在远平面 |

---

## 4.6 HDR 贴图支持

| 需求编号 | 特性 | 说明 |
|----------|------|------|
| 66 | .hdr / .exr 格式 | HDR 贴图输入 |
| 105 | HDR 纹理使用 RGBA16F / RGBA32F | 高动态范围纹理格式 |

---

## 4.7 输入与输出

### 输入
- HDR 环境贴图文件（.hdr）
- 等距柱状投影纹理
- 环境光照强度参数

### 输出
- `CubeMap`: 天空盒用立方体贴图
- `CubeMap`: 辐照度贴图（IBL 漫反射）
- `CubeMap`: 预滤波环境贴图（IBL 高光）
- `Texture2D`: BRDF LUT 查找表

---

## 4.8 验收标准

| 编号 | 标准 |
|------|------|
| 60 | `IBLBaker::new()` 成功创建 |
| 61 | `bake_irradiance()` 生成辐照度贴图 |
| 62 | `bake_prefilter()` 生成预滤波贴图 |
| 63 | `bake_brdf_lut()` 生成 BRDF LUT |
| 66 | `EnvironmentMap::from_hdr()` 正确加载 HDR |
| 67 | skybox/irradiance/prefilter/brdf_lut 正确返回 |
| 71 | `SkyboxRenderer` 正确渲染天空盒 |
| 74 | 天空盒关闭深度写入，绘制在远平面 |
| 436 | 单测 `IBLBaker::bake_brdf_lut` 生成非空贴图 |
| 420 | `examples/pbr_ibl` 显示 IBL 效果 |

---

## 4.9 依赖关系

### 依赖模块
- `TextureCompiler`: 纹理处理
- `PbrShader`: IBL 着色器
- `ComputePass`: IBL 烘焙计算

### 被依赖模块
- `engine-render-passes`: PBR Pass

---

## 4.10 优先级

| 优先级 | 需求编号 | 说明 |
|--------|----------|------|
| P0 | 60-63, 66-70, 86-97, 324-335 | 核心 IBL 功能 |
| P1 | 64-65, 98-101, 338-339 | 缓存与天空盒渲染 |
| P2 | 71-75 | 开发期实时烘焙 |
