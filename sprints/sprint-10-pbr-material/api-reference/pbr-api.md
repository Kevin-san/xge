# PBR API 清单

## 概述

本文档列出 `engine-pbr` crate 的完整公开 API 接口。

---

## 1. PbrMaterial API

### 1.1 构造函数

| API | 签名 | 需求编号 |
|-----|------|----------|
| `PbrMaterial::default` | `fn default() -> Self` | 46, 212 |
| `PbrMaterial::from_albedo` | `fn from_albedo(color: Color) -> Self` | 47, 213 |

### 1.2 Albedo

| API | 签名 | 需求编号 |
|-----|------|----------|
| `albedo_map` | `fn albedo_map(&self) -> Option<Handle<Texture>>` | 214 |
| `set_albedo_map` | `fn set_albedo_map(&mut self, tex: Handle<Texture>)` | 215 |
| `albedo` | `fn albedo(&self) -> Color` | 216 |
| `set_albedo` | `fn set_albedo(&mut self, color: Color)` | 217 |

### 1.3 Metallic

| API | 签名 | 需求编号 |
|-----|------|----------|
| `metallic_map` | `fn metallic_map(&self) -> Option<Handle<Texture>>` | 218 |
| `set_metallic_map` | `fn set_metallic_map(&mut self, tex: Handle<Texture>)` | 219 |
| `metallic` | `fn metallic(&self) -> f32` | 220 |
| `set_metallic` | `fn set_metallic(&mut self, v: f32)` | 221 |

### 1.4 Roughness

| API | 签名 | 需求编号 |
|-----|------|----------|
| `roughness_map` | `fn roughness_map(&self) -> Option<Handle<Texture>>` | 222 |
| `set_roughness_map` | `fn set_roughness_map(&mut self, tex: Handle<Texture>)` | 223 |
| `roughness` | `fn roughness(&self) -> f32` | 224 |
| `set_roughness` | `fn set_roughness(&mut self, v: f32)` | 225 |

### 1.5 Normal

| API | 签名 | 需求编号 |
|-----|------|----------|
| `normal_map` | `fn normal_map(&self) -> Option<Handle<Texture>>` | 226 |
| `set_normal_map` | `fn set_normal_map(&mut self, tex: Handle<Texture>)` | 227 |
| `normal_strength` | `fn normal_strength(&self) -> f32` | 228 |
| `set_normal_strength` | `fn set_normal_strength(&mut self, v: f32)` | 229 |

### 1.6 AO

| API | 签名 | 需求编号 |
|-----|------|----------|
| `ao_map` | `fn ao_map(&self) -> Option<Handle<Texture>>` | 230 |
| `set_ao_map` | `fn set_ao_map(&mut self, tex: Handle<Texture>)` | 231 |
| `ao_strength` | `fn ao_strength(&self) -> f32` | 232 |
| `set_ao_strength` | `fn set_ao_strength(&mut self, v: f32)` | 233 |

### 1.7 Emissive

| API | 签名 | 需求编号 |
|-----|------|----------|
| `emissive_map` | `fn emissive_map(&self) -> Option<Handle<Texture>>` | 234 |
| `set_emissive_map` | `fn set_emissive_map(&mut self, tex: Handle<Texture>)` | 235 |
| `emissive` | `fn emissive(&self) -> Color` | 236 |
| `set_emissive` | `fn set_emissive(&mut self, color: Color)` | 237 |
| `emissive_intensity` | `fn emissive_intensity(&self) -> f32` | 238 |
| `set_emissive_intensity` | `fn set_emissive_intensity(&mut self, v: f32)` | 239 |

### 1.8 Height/Parallax

| API | 签名 | 需求编号 |
|-----|------|----------|
| `height_map` | `fn height_map(&self) -> Option<Handle<Texture>>` | 240 |
| `set_height_map` | `fn set_height_map(&mut self, tex: Handle<Texture>)` | 241 |
| `parallax_strength` | `fn parallax_strength(&self) -> f32` | 242 |
| `set_parallax_strength` | `fn set_parallax_strength(&mut self, v: f32)` | 243 |

### 1.9 Clear Coat

| API | 签名 | 需求编号 |
|-----|------|----------|
| `clear_coat` | `fn clear_coat(&self) -> f32` | 244 |
| `set_clear_coat` | `fn set_clear_coat(&mut self, v: f32)` | 245 |
| `clear_coat_roughness` | `fn clear_coat_roughness(&self) -> f32` | 246 |
| `set_clear_coat_roughness` | `fn set_clear_coat_roughness(&mut self, v: f32)` | 247 |

### 1.10 Anisotropy

| API | 签名 | 需求编号 |
|-----|------|----------|
| `anisotropy` | `fn anisotropy(&self) -> f32` | 248 |
| `set_anisotropy` | `fn set_anisotropy(&mut self, v: f32)` | 249 |

### 1.11 Sheen

| API | 签名 | 需求编号 |
|-----|------|----------|
| `sheen` | `fn sheen(&self) -> Color` | 250 |
| `set_sheen` | `fn set_sheen(&mut self, color: Color)` | 251 |
| `sheen_roughness` | `fn sheen_roughness(&self) -> f32` | 252 |
| `set_sheen_roughness` | `fn set_sheen_roughness(&mut self, v: f32)` | 253 |

### 1.12 Subsurface

| API | 签名 | 需求编号 |
|-----|------|----------|
| `subsurface` | `fn subsurface(&self) -> f32` | 254 |
| `set_subsurface` | `fn set_subsurface(&mut self, v: f32)` | 255 |

### 1.13 Alpha Mode

| API | 签名 | 需求编号 |
|-----|------|----------|
| `alpha_mode` | `fn alpha_mode(&self) -> AlphaMode` | 256 |
| `set_alpha_mode` | `fn set_alpha_mode(&mut self, mode: AlphaMode)` | 257 |
| `alpha_cutoff` | `fn alpha_cutoff(&self) -> f32` | 258 |
| `set_alpha_cutoff` | `fn set_alpha_cutoff(&mut self, v: f32)` | 259 |

### 1.14 Rendering Flags

| API | 签名 | 需求编号 |
|-----|------|----------|
| `double_sided` | `fn double_sided(&self) -> bool` | 260 |
| `set_double_sided` | `fn set_double_sided(&mut self, bool)` | 261 |
| `casts_shadow` | `fn casts_shadow(&self) -> bool` | 262 |
| `set_casts_shadow` | `fn set_casts_shadow(&mut self, bool)` | 263 |
| `receives_shadow` | `fn receives_shadow(&self) -> bool` | 264 |
| `set_receives_shadow` | `fn set_receives_shadow(&mut self, bool)` | 265 |
| `flags` | `fn flags(&self) -> PbrMaterialFlags` | 266 |

### 1.15 GPU Binding

| API | 签名 | 需求编号 |
|-----|------|----------|
| `bind_group_layout` | `fn bind_group_layout(renderer: &Renderer) -> BindGroupLayout` | 267 |
| `bind_group` | `fn bind_group(&self, renderer: &Renderer) -> BindGroup` | 268 |

### 1.16 Serialization

| API | 签名 | 需求编号 |
|-----|------|----------|
| `to_json` | `fn to_json(&self) -> String` | 108, 234 |
| `from_json` | `fn from_json(json: &str) -> Result<Self>` | 109, 235 |
| `save` | `fn save(&self, path: &Path) -> Result<()>` | 110, 236 |
| `load` | `fn load(path: &Path) -> Result<Self>` | 111, 237 |

---

## 2. AlphaMode

```rust
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}
```

---

## 3. PbrMaterialFlags

```rust
pub struct PbrMaterialFlags(u32);

impl PbrMaterialFlags {
    pub const HAS_ALBEDO_MAP: u32;
    pub const HAS_NORMAL_MAP: u32;
    pub const HAS_METALLIC_MAP: u32;
    pub const HAS_ROUGHNESS_MAP: u32;
    pub const HAS_AO_MAP: u32;
    pub const HAS_EMISSIVE_MAP: u32;
    pub const HAS_HEIGHT_MAP: u32;
    pub const USE_IBL: u32;
    pub const USE_CLEAR_COAT: u32;
    pub const USE_ANISOTROPY: u32;
    pub const USE_SHEEN: u32;
    pub const USE_SUBSURFACE: u32;
    pub const USE_PARALLAX: u32;
    
    pub fn contains(self, flag: u32) -> bool;
}
```

**需求编号**：45, 274, 275

---

## 4. MaterialSystemPbr

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new() -> Self` | 22 |
| `load` | `fn load(path: &Path) -> Result<Handle<PbrMaterial>>` | 23, 49 |
| `save` | `fn save(handle: Handle<PbrMaterial>, path: &Path) -> Result<()>` | 24, 50 |
| `recompile` | `fn recompile(handle: Handle<PbrMaterial>) -> Result<()>` | 25, 51 |

---

## 5. ShaderCompiler

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new() -> Self` | 27, 53, 245 |
| `compile_wgsl` | `fn compile_wgsl(src: &str) -> Result<ShaderModule>` | 55 |
| `compile_glsl` | `fn compile_glsl(src: &str, stage: ShaderStage) -> Result<ShaderModule>` | 56 |
| `inspect_errors` | `fn inspect_errors(src: &str) -> Vec<Diagnostic>` | 57 |
| `compile` | `fn compile(&self, source: &ShaderSource, lang: ShaderLanguage, stage: ShaderStage) -> Result<ShaderModule>` | 246 |
| `diagnostics` | `fn diagnostics(&self) -> Vec<Diagnostic>` | 247 |

---

## 6. ShaderModule

| API | 签名 | 需求编号 |
|-----|------|----------|
| `from_wgsl` | `fn from_wgsl(src: &str) -> Result<Self>` | 241 |
| `from_glsl` | `fn from_glsl(src: &str, stage: ShaderStage) -> Result<Self>` | 242 |
| `entry_points` | `fn entry_points(&self) -> Vec<&str>` | 243 |
| `stage` | `fn stage(&self) -> ShaderStage` | 244 |

---

## 7. ShaderHotReload

| API | 签名 | 需求编号 |
|-----|------|----------|
| `watch` | `fn watch(path: &Path, callback: F)` | 255 |
| `tick` | `fn tick(&mut self)` | 256 |

---

## 8. ShaderLibrary

| API | 签名 | 需求编号 |
|-----|------|----------|
| `include` | `fn include(name: &str) -> &str` | 257 |
| `brdf_functions` | `fn brdf_functions() -> &str` | 258 |
| `pbr_common` | `fn pbr_common() -> &str` | 259 |
| `utils` | `fn utils() -> &str` | 260 |

---

## 9. ShaderGraph

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new() -> Self` | 47, 281 |
| `name` | `fn name(&self) -> &str` | 323 |
| `set_name` | `fn set_name(&mut self, name: String)` | 324 |
| `nodes` | `fn nodes(&self) -> &[ShaderGraphNode]` | 325 |
| `edges` | `fn edges(&self) -> &[Edge]` | 326 |
| `add_node` | `fn add_node(&mut self, kind: NodeKind) -> NodeId` | 327 |
| `remove_node` | `fn remove_node(&mut self, id: NodeId)` | 328 |
| `add_edge` | `fn add_edge(&mut self, from: NodeId, to: NodeId) -> EdgeId` | 329 |
| `remove_edge` | `fn remove_edge(&mut self, id: EdgeId)` | 330 |
| `topological_order` | `fn topological_order(&self) -> Result<Vec<NodeId>, CycleError>` | 331 |
| `compile` | `fn compile(&self) -> Result<ShaderSource>` | 332 |
| `validate` | `fn validate(&self) -> Result<()>` | 333 |
| `to_json` | `fn to_json(&self) -> String` | 334 |
| `from_json` | `fn from_json(json: &str) -> Result<Self>` | 335 |
| `generate_wgsl` | `fn generate_wgsl(&self) -> String` | 357 |
| `generate_glsl` | `fn generate_glsl(&self) -> String` | 358 |

---

## 10. ShaderGraphEditor

| API | 签名 | 需求编号 |
|-----|------|----------|
| `open` | `fn open(&mut self, graph: Handle<ShaderGraph>)` | 359 |
| `close` | `fn close(&mut self)` | 361 |
| `select` | `fn select(&mut self, node_id: NodeId)` | 362 |
| `draw` | `fn draw(&mut self, ui: &mut Ui)` | 363 |
| `preview` | `fn preview(&mut self, renderer: &mut Renderer)` | 364 |

---

## 11. IBLBaker

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer) -> Result<Self>` | 324 |
| `bake_irradiance` | `fn bake_irradiance(&self, env_map: &EnvironmentMap) -> CubeMap` | 325 |
| `bake_prefilter` | `fn bake_prefilter(&self, env_map: &EnvironmentMap, levels: u32) -> CubeMap` | 326 |
| `bake_brdf_lut` | `fn bake_brdf_lut(&self, size: u32) -> Texture2D` | 327 |
| `save_cache` | `fn save_cache(&self, dir: &Path) -> Result<()>` | 328 |
| `load_cache` | `fn load_cache(&mut self, dir: &Path) -> Result<()>` | 329 |

---

## 12. EnvironmentMap

| API | 签名 | 需求编号 |
|-----|------|----------|
| `from_hdr` | `fn from_hdr(path: &Path) -> Result<Self>` | 330 |
| `from_equirectangular` | `fn from_equirectangular(texture: Texture) -> Result<Self>` | 331 |
| `skybox` | `fn skybox(&self) -> CubeMap` | 332 |
| `irradiance` | `fn irradiance(&self) -> CubeMap` | 333 |
| `prefilter` | `fn prefilter(&self) -> CubeMap` | 334 |
| `brdf_lut` | `fn brdf_lut(&self) -> Texture2D` | 335 |
| `intensity` | `fn intensity(&self) -> f32` | 336 |
| `set_intensity` | `fn set_intensity(&mut self, v: f32)` | 337 |

---

## 13. SkyboxRenderer

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer) -> Result<Self>` | 338 |
| `draw` | `fn draw(&self, renderer: &mut Renderer, camera: &Camera, env: &EnvironmentMap)` | 339 |
| `set_skybox_texture` | `fn set_skybox_texture(&mut self, cube_map: CubeMap)` | 340 |

---

## 14. PbrPipeline

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer, key: PbrShaderKey) -> Result<Self>` | 341 |
| `bind` | `fn bind(&self, renderer: &mut Renderer, camera: &Camera, lights: &LightBag, env: &EnvironmentMap)` | 342 |
| `draw_mesh` | `fn draw_mesh(&self, renderer: &mut Renderer, mesh: &Mesh, material: &PbrMaterial, transform: &Transform)` | 343 |

---

## 15. PbrPass

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer) -> Result<Self>` | 344 |
| `draw` | `fn draw(&self, renderer: &mut Renderer, scene: &Scene, camera: &Camera, lights: &LightBag, env: &EnvironmentMap)` | 345 |

---

## 16. ShadowMapPass

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer, size: u32) -> Result<Self>` | 346 |
| `draw_directional` | `fn draw_directional(&self, renderer: &mut Renderer, light: &Light, scene: &Scene)` | 347 |
| `draw_point` | `fn draw_point(&self, renderer: &mut Renderer, light: &Light, scene: &Scene)` | 348 |
| `texture` | `fn texture(&self) -> &Texture` | 349 |

---

## 17. HdrPipeline

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer, size: UVec2) -> Result<Self>` | 362 |
| `render_target` | `fn render_target(&self) -> &Texture` | 363 |
| `tonemap` | `fn tonemap(&self, renderer: &mut Renderer, tonemapper: Tonemapper, color_grading: &ColorGrading)` | 364 |

---

## 18. Tonemapper

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

**需求编号**：82, 108, 352, 353

---

## 19. ColorGrading

| API | 签名 | 需求编号 |
|-----|------|----------|
| `exposure` | `fn exposure(&self) -> f32` | 354 |
| `set_exposure` | `fn set_exposure(&mut self, v: f32)` | 355 |
| `contrast` | `fn contrast(&self) -> f32` | 356 |
| `set_contrast` | `fn set_contrast(&mut self, v: f32)` | 357 |
| `saturation` | `fn saturation(&self) -> f32` | 358 |
| `set_saturation` | `fn set_saturation(&mut self, v: f32)` | 359 |
| `temperature` | `fn temperature(&self) -> f32` | 360 |
| `set_temperature` | `fn set_temperature(&mut self, v: f32)` | 361 |

---

## 20. TextureCompiler

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer)` | 365 |
| `compile_file` | `fn compile_file(&self, path: &Path, options: TextureOptions) -> Result<Texture>` | 366 |
| `compile_bytes` | `fn compile_bytes(&self, bytes: &[u8], options: TextureOptions) -> Result<Texture>` | 367 |
| `compile_image` | `fn compile_image(&self, image: &DynamicImage, options: TextureOptions) -> Result<Texture>` | 368 |

---

## 21. TextureOptions

```rust
impl TextureOptions {
    pub fn default() -> Self;
    pub fn with_srgb(self, b: bool) -> Self;
    pub fn with_mipmap(self, b: bool) -> Self;
    pub fn with_compression(self, c: Compression) -> Self;
    pub fn with_filter(self, f: FilterMode) -> Self;
    pub fn with_wrap(self, w: WrapMode) -> Self;
    pub fn with_hdr(self, b: bool) -> Self;
}
```

**需求编号**：369-375

---

## 22. Texture

| API | 签名 | 需求编号 |
|-----|------|----------|
| `width` | `fn width(&self) -> u32` | 377 |
| `height` | `fn height(&self) -> u32` | 378 |
| `depth_or_layers` | `fn depth_or_layers(&self) -> u32` | 379 |
| `mip_levels` | `fn mip_levels(&self) -> u32` | 380 |
| `format` | `fn format(&self) -> TextureFormat` | 381 |
| `is_srgb` | `fn is_srgb(&self) -> bool` | 382 |
| `generate_mipmaps` | `fn generate_mipmaps(&mut self, renderer: &Renderer)` | 383 |

---

## 23. Sampler

| API | 签名 | 需求编号 |
|-----|------|----------|
| `nearest` | `fn nearest(renderer: &Renderer) -> Handle<Sampler>` | 384 |
| `linear` | `fn linear(renderer: &Renderer) -> Handle<Sampler>` | 385 |
| `trilinear` | `fn trilinear(renderer: &Renderer) -> Handle<Sampler>` | 386 |
| `anisotropic` | `fn anisotropic(renderer: &Renderer, level: u8) -> Handle<Sampler>` | 387 |
| `comparison` | `fn comparison(renderer: &Renderer, func: CompareFunction) -> Handle<Sampler>` | 388 |
| `repeat` | `fn repeat(renderer: &Renderer) -> Handle<Sampler>` | 389 |

---

## 24. Buffer

| API | 签名 | 需求编号 |
|-----|------|----------|
| `new` | `fn new(renderer: &Renderer, usage: BufferUsage, size_bytes: u64, data: Option<&[u8]>) -> Result<Self>` | 390 |
| `write` | `fn write(&self, renderer: &Renderer, offset: u64, data: &[u8])` | 391 |
| `read_back` | `fn read_back(&self, renderer: &Renderer) -> Result<Vec<u8>>` | 392 |
| `size` | `fn size(&self) -> u64` | 393 |

---

## 25. BindGroup Layout/Builder

```rust
pub struct BindGroupLayoutBuilder { ... }

impl BindGroupLayoutBuilder {
    pub fn new() -> Self;
    pub fn add(self, binding: u32, ty: BindingType) -> Self;
    pub fn build(&self, renderer: &Renderer) -> BindGroupLayout;
}

pub struct BindGroupBuilder { ... }

impl BindGroupBuilder {
    pub fn new() -> Self;
    pub fn add_buffer(self, binding: u32, buffer: &Buffer) -> Self;
    pub fn add_sampler(self, binding: u32, sampler: &Sampler) -> Self;
    pub fn add_texture(self, binding: u32, texture: &Texture) -> Self;
    pub fn build(&self, renderer: &Renderer, layout: &BindGroupLayout) -> BindGroup;
}
```

**需求编号**：394-401

---

## 26. BindGroupCache

| API | 签名 | 需求编号 |
|-----|------|----------|
| `get` | `fn get(&self, key: &(impl Hash + Eq)) -> Option<BindGroup>` | 402 |
| `insert` | `fn insert(&mut self, key: K, value: BindGroup)` | 403 |

---

## 27. PipelineCache

| API | 签名 | 需求编号 |
|-----|------|----------|
| `save` | `fn save(&self, dir: &Path) -> Result<()>` | 404 |
| `load` | `fn load(&mut self, dir: &Path) -> Result<()>` | 405 |

---

## 28. RenderError

```rust
pub enum RenderError {
    ShaderCompile { msg: String, line: Option<u32> },
    Pipeline { msg: String },
    Resource { msg: String },
    Other { msg: String },
}

impl RenderError {
    pub fn shader(msg: String, line: Option<u32>) -> Self;
    pub fn to_string(&self) -> String;
    pub fn span(&self) -> Option<Range<usize>>;
}
```

**需求编号**：144-146, 170, 406-409
