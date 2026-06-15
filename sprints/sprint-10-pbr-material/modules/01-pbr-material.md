# 模块一：PBR 材质需求

## 1.1 模块概述

本模块定义了 `engine-pbr` crate 中的 `PbrMaterial` 材质系统，支持完整 PBR 金属/粗糙度工作流。材质系统管理 albedo、metallic、roughness、normal、ao、emissive、height 等贴图通道，以及 clear coat、anisotropy、sheen、subsurface scattering 等高级特性。

**对应原需求编号**：1-50, 108-119, 177-276, 429, 440, 486, 495

---

## 1.2 核心类型定义

### 1.2.1 PbrMaterial 结构体

| 属性 | 类型 | 说明 |
|------|------|------|
| `albedo_map` | `Option<Handle<Texture>>` | 基础颜色贴图 |
| `albedo` | `Color` | 基础颜色常量（与贴图相乘） |
| `metallic_map` | `Option<Handle<Texture>>` | 金属度贴图 |
| `metallic` | `f32` | 金属度常量值 [0.0, 1.0] |
| `roughness_map` | `Option<Handle<Texture>>` | 粗糙度贴图 |
| `roughness` | `f32` | 粗糙度常量值 [0.0, 1.0] |
| `normal_map` | `Option<Handle<Texture>>` | 法线贴图 |
| `normal_strength` | `f32` | 法线强度 [0.0, 1.0] |
| `ao_map` | `Option<Handle<Texture>>` | 环境光遮蔽贴图 |
| `ao_strength` | `f32` | AO 强度 [0.0, 1.0] |
| `emissive_map` | `Option<Handle<Texture>>` | 自发光贴图 |
| `emissive` | `Color` | 自发光颜色 |
| `emissive_intensity` | `f32` | 自发光强度 |
| `height_map` | `Option<Handle<Texture>>` | 高度贴图（视差） |
| `parallax_strength` | `f32` | 视差强度 [0.0, 1.0] |
| `clear_coat` | `f32` | 清漆层强度 [0.0, 1.0] |
| `clear_coat_roughness` | `f32` | 清漆层粗糙度 [0.0, 1.0] |
| `anisotropy` | `f32` | 各向异性强度 [0.0, 1.0] |
| `tangent_map` | `Option<Handle<Texture>>` | 切线贴图（可选） |
| `sheen` | `Color` | 织物光泽颜色 |
| `sheen_roughness` | `f32` | 织物光泽粗糙度 [0.0, 1.0] |
| `subsurface` | `f32` | 次表面散射强度 [0.0, 1.0] |
| `alpha_mode` | `AlphaMode` | 透明度模式 |
| `alpha_cutoff` | `f32` | Mask 模式 cutoff 值 |
| `double_sided` | `bool` | 双面渲染 |
| `casts_shadow` | `bool` | 投射阴影 |
| `receives_shadow` | `bool` | 接收阴影 |
| `flags` | `PbrMaterialFlags` | 按位标记 |

### 1.2.2 AlphaMode 枚举

```rust
pub enum AlphaMode {
    Opaque,  // 不透明
    Mask,    // 镂空遮罩
    Blend,   // 半透明混合
}
```

### 1.2.3 PbrMaterialFlags 位标记

```rust
pub struct PbrMaterialFlags(u32);

impl PbrMaterialFlags {
    pub const HAS_ALBEDO_MAP: u32 = 1 << 0;
    pub const HAS_NORMAL_MAP: u32 = 1 << 1;
    pub const HAS_METALLIC_MAP: u32 = 1 << 2;
    pub const HAS_ROUGHNESS_MAP: u32 = 1 << 3;
    pub const HAS_AO_MAP: u32 = 1 << 4;
    pub const HAS_EMISSIVE_MAP: u32 = 1 << 5;
    pub const HAS_HEIGHT_MAP: u32 = 1 << 6;
    pub const USE_IBL: u32 = 1 << 7;
    pub const USE_CLEAR_COAT: u32 = 1 << 8;
    pub const USE_ANISOTROPY: u32 = 1 << 9;
    pub const USE_SHEEN: u32 = 1 << 10;
    pub const USE_SUBSURFACE: u32 = 1 << 11;
    pub const USE_PARALLAX: u32 = 1 << 12;
}

impl PbrMaterialFlags {
    pub fn contains(self, flag: u32) -> bool;
}
```

---

## 1.3 API 签名

### 构造函数

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 46, 212 | `PbrMaterial::default() -> Self` | 全 1.0 白色 + 无贴图 |
| 47, 213 | `PbrMaterial::from_albedo(color: Color) -> Self` | 从基础颜色创建 |

### Albedo 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 214 | `PbrMaterial::albedo_map(&self) -> Option<Handle<Texture>>` | 获取 albedo 贴图 |
| 215 | `PbrMaterial::set_albedo_map(&mut self, tex: Handle<Texture>)` | 设置 albedo 贴图 |
| 216 | `PbrMaterial::albedo(&self) -> Color` | 获取 albedo 颜色 |
| 217 | `PbrMaterial::set_albedo(&mut self, color: Color)` | 设置 albedo 颜色 |

### Metallic 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 218 | `PbrMaterial::metallic_map(&self) -> Option<Handle<Texture>>` | 获取 metallic 贴图 |
| 219 | `PbrMaterial::set_metallic_map(&mut self, tex: Handle<Texture>)` | 设置 metallic 贴图 |
| 220 | `PbrMaterial::metallic(&self) -> f32` | 获取 metallic 值 |
| 221 | `PbrMaterial::set_metallic(&mut self, v: f32)` | 设置 metallic 值 |

### Roughness 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 222 | `PbrMaterial::roughness_map(&self) -> Option<Handle<Texture>>` | 获取 roughness 贴图 |
| 223 | `PbrMaterial::set_roughness_map(&mut self, tex: Handle<Texture>)` | 设置 roughness 贴图 |
| 224 | `PbrMaterial::roughness(&self) -> f32` | 获取 roughness 值 |
| 225 | `PbrMaterial::set_roughness(&mut self, v: f32)` | 设置 roughness 值 |

### Normal 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 226 | `PbrMaterial::normal_map(&self) -> Option<Handle<Texture>>` | 获取 normal 贴图 |
| 227 | `PbrMaterial::set_normal_map(&mut self, tex: Handle<Texture>)` | 设置 normal 贴图 |
| 228 | `PbrMaterial::normal_strength(&self) -> f32` | 获取 normal 强度 |
| 229 | `PbrMaterial::set_normal_strength(&mut self, v: f32)` | 设置 normal 强度 |

### AO 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 230 | `PbrMaterial::ao_map(&self) -> Option<Handle<Texture>>` | 获取 ao 贴图 |
| 231 | `PbrMaterial::set_ao_map(&mut self, tex: Handle<Texture>)` | 设置 ao 贴图 |
| 232 | `PbrMaterial::ao_strength(&self) -> f32` | 获取 ao 强度 |
| 233 | `PbrMaterial::set_ao_strength(&mut self, v: f32)` | 设置 ao 强度 |

### Emissive 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 234 | `PbrMaterial::emissive_map(&self) -> Option<Handle<Texture>>` | 获取 emissive 贴图 |
| 235 | `PbrMaterial::set_emissive_map(&mut self, tex: Handle<Texture>)` | 设置 emissive 贴图 |
| 236 | `PbrMaterial::emissive(&self) -> Color` | 获取 emissive 颜色 |
| 237 | `PbrMaterial::set_emissive(&mut self, color: Color)` | 设置 emissive 颜色 |
| 238 | `PbrMaterial::emissive_intensity(&self) -> f32` | 获取 emissive 强度 |
| 239 | `PbrMaterial::set_emissive_intensity(&mut self, v: f32)` | 设置 emissive 强度 |

### Height/Parallax 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 240 | `PbrMaterial::height_map(&self) -> Option<Handle<Texture>>` | 获取 height 贴图 |
| 241 | `PbrMaterial::set_height_map(&mut self, tex: Handle<Texture>)` | 设置 height 贴图 |
| 242 | `PbrMaterial::parallax_strength(&self) -> f32` | 获取 parallax 强度 |
| 243 | `PbrMaterial::set_parallax_strength(&mut self, v: f32)` | 设置 parallax 强度 |

### Clear Coat 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 244 | `PbrMaterial::clear_coat(&self) -> f32` | 获取 clear coat 值 |
| 245 | `PbrMaterial::set_clear_coat(&mut self, v: f32)` | 设置 clear coat 值 |
| 246 | `PbrMaterial::clear_coat_roughness(&self) -> f32` | 获取 clear coat roughness |
| 247 | `PbrMaterial::set_clear_coat_roughness(&mut self, v: f32)` | 设置 clear coat roughness |

### Anisotropy 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 248 | `PbrMaterial::anisotropy(&self) -> f32` | 获取 anisotropy 值 |
| 249 | `PbrMaterial::set_anisotropy(&mut self, v: f32)` | 设置 anisotropy 值 |

### Sheen 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 250 | `PbrMaterial::sheen(&self) -> Color` | 获取 sheen 颜色 |
| 251 | `PbrMaterial::set_sheen(&mut self, color: Color)` | 设置 sheen 颜色 |
| 252 | `PbrMaterial::sheen_roughness(&self) -> f32` | 获取 sheen roughness |
| 253 | `PbrMaterial::set_sheen_roughness(&mut self, v: f32)` | 设置 sheen roughness |

### Subsurface 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 254 | `PbrMaterial::subsurface(&self) -> f32` | 获取 subsurface 值 |
| 255 | `PbrMaterial::set_subsurface(&mut self, v: f32)` | 设置 subsurface 值 |

### Alpha 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 256 | `PbrMaterial::alpha_mode(&self) -> AlphaMode` | 获取 alpha mode |
| 257 | `PbrMaterial::set_alpha_mode(&mut self, mode: AlphaMode)` | 设置 alpha mode |
| 258 | `PbrMaterial::alpha_cutoff(&self) -> f32` | 获取 alpha cutoff |
| 259 | `PbrMaterial::set_alpha_cutoff(&mut self, v: f32)` | 设置 alpha cutoff |

### Rendering 属性

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 260 | `PbrMaterial::double_sided(&self) -> bool` | 获取双面渲染状态 |
| 261 | `PbrMaterial::set_double_sided(&mut self, bool)` | 设置双面渲染 |
| 262 | `PbrMaterial::casts_shadow(&self) -> bool` | 获取投射阴影状态 |
| 263 | `PbrMaterial::set_casts_shadow(&mut self, bool)` | 设置投射阴影 |
| 264 | `PbrMaterial::receives_shadow(&self) -> bool` | 获取接收阴影状态 |
| 265 | `PbrMaterial::set_receives_shadow(&mut self, bool)` | 设置接收阴影 |

### 内部方法

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 266 | `PbrMaterial::flags(&self) -> PbrMaterialFlags` | 获取材质标记 |
| 267 | `PbrMaterial::bind_group_layout(renderer: &Renderer) -> BindGroupLayout` | 获取绑定组布局 |
| 268 | `PbrMaterial::bind_group(&self, renderer: &Renderer) -> BindGroup` | 获取绑定组 |

### 序列化

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 108, 234 | `PbrMaterial::to_json(&self) -> String` | 序列化为 JSON |
| 109, 235 | `PbrMaterial::from_json(json: &str) -> Result<Self>` | 从 JSON 反序列化 |
| 110, 236 | `PbrMaterial::save(&self, path: &Path) -> Result<()>` | 保存到文件 |
| 111, 237 | `PbrMaterial::load(path: &Path) -> Result<Self>` | 从文件加载 |

---

## 1.4 MaterialSystemPbr

材质系统统一管理所有 PBR 材质。

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 22 | `MaterialSystemPbr::new() -> Self` | 创建材质系统 |
| 23 | `MaterialSystemPbr::load(path: &Path) -> Result<Handle<PbrMaterial>>` | 加载 TOML/JSON 材质 |
| 24 | `MaterialSystemPbr::save(handle: Handle<PbrMaterial>, path: &Path) -> Result<()>` | 保存材质 |
| 25 | `MaterialSystemPbr::recompile(handle: Handle<PbrMaterial>) -> Result<()>` | 动态重编译 |
| 47 | `PbrMaterial::default() -> Self` | 默认材质 |
| 48 | `from_albedo(color: Color) -> Self` | 从颜色创建 |

---

## 1.5 输入与输出

### 输入
- 纹理资源（2D 贴图）
- 颜色值（RGBA）
- 浮点数值
- 材质定义文件（TOML/JSON）

### 输出
- `PbrMaterial` 实例
- `BindGroup` 用于 GPU 绑定
- `PbrMaterialFlags` 位标记

---

## 1.6 验收标准

| 编号 | 标准 |
|------|------|
| 1 | `engine-pbr` crate 建立，feature-gated |
| 46 | `PbrMaterial::default()` 返回全 1.0 白色材质 |
| 47 | `from_albedo(color)` 正确创建材质 |
| 45 | `PbrMaterialFlags` 按位标记正确工作 |
| 108-111 | JSON/TOML 序列化/反序列化往返一致 |
| 267-268 | bind group 懒构建，初次 draw 时触发 |
| 17 | transparent vs opaque 的 pipeline 区别正确 |
| 114-118 | 材质面板支持拖拽贴图、设置常量值 |
| 429 | 单测 PbrMaterial JSON 往返通过 |
| 440 | 单测 PbrMaterialFlags 位运算正确 |

---

## 1.7 依赖关系

### 依赖模块
- `engine-core`: Handle、Result、Color 类型
- `engine-render`: Texture、BindGroup、BindGroupLayout
- `engine-assets`: 资源加载框架

### 被依赖模块
- `engine-render-passes`: PBR 渲染 Pass
- `ShaderGraph`: 材质编辑器

---

## 1.8 优先级

| 优先级 | 需求编号 | 说明 |
|--------|----------|------|
| P0 | 2-19, 40-48, 108-111 | 核心 PBR 属性、序列化 |
| P1 | 45, 49-51, 114-118, 267-268 | 材质系统、编辑器集成 |
| P2 | 52-59 | 高级特性（anisotropy、sheen、subsurface） |

---

## 1.9 材质球 Demo 需求

| 需求编号 | 材质类型 | 说明 |
|----------|----------|------|
| 97-99 | Metal | 金属材质 |
| 97-99 | Plastic | 塑料材质 |
| 97-99 | Ceramic | 陶瓷材质 |
| 97-99 | Wood | 木质材质 |
| 97-99 | Concrete | 混凝土材质 |
| 97-99 | Fabric | 织物材质 |
| 97-99 | Gold | 金材质 |
| 97-99 | Copper | 铜材质 |
| 97-99 | Leather | 皮革材质 |
| 97-99 | CarPaint | 车漆材质 |
| 97-99 | Rubber | 橡胶材质 |
| 97-99 | Brushed Metal | 拉丝金属材质 |
