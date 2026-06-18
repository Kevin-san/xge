# 模块二：Shader 系统需求

## 2.1 模块概述

本模块定义了 Shader 编译、管理和热重载系统。支持多种着色器语言（WGSL/GLSL/HLSL/MSL），通过 Naga 进行跨语言交叉编译。提供 ShaderPermutation 和 ShaderKey 机制支持基于宏组合的编译缓存，以及 ShaderHotReload 实现开发期文件监听和自动重建。

**对应原需求编号**：26-45, 53-72, 241-319

---

## 2.2 核心类型定义

### 2.2.1 ShaderStage 枚举

```rust
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}
```

### 2.2.2 ShaderLanguage 枚举

```rust
pub enum ShaderLanguage {
    WGSL,
    GLSL,
    HLSL,
    MSL,
}
```

### 2.2.3 Diagnostic 与 Level

```rust
pub enum Level {
    Error,
    Warning,
    Info,
}

pub struct Diagnostic {
    level: Level,
    message: String,
    span: Option<Range<usize>>,
}

impl Diagnostic {
    pub fn level(&self) -> Level;
    pub fn message(&self) -> &str;
    pub fn span(&self) -> Option<Range<usize>>;
}
```

### 2.2.4 ShaderModule

```rust
pub struct ShaderModule {
    source: String,
    lang: ShaderLanguage,
    stage: ShaderStage,
}

impl ShaderModule {
    pub fn from_wgsl(src: &str) -> Result<Self>;
    pub fn from_glsl(src: &str, stage: ShaderStage) -> Result<Self>;
    pub fn entry_points(&self) -> Vec<&str>;
    pub fn stage(&self) -> ShaderStage;
}
```

### 2.2.5 ShaderSource

```rust
pub struct ShaderSource {
    source: String,
    include_paths: Vec<PathBuf>,
}
```

### 2.2.6 ShaderKey

```rust
#[derive(Hash)]
pub struct ShaderKey {
    base_name: String,
    permutations: Vec<(String, bool)>,
}

impl ShaderKey {
    pub fn hash(&self) -> u64;
}
```

### 2.2.7 ShaderPermutation

```rust
pub struct ShaderPermutation {
    base_source: String,
    macros: Vec<(String, bool)>,
}

impl ShaderPermutation {
    pub fn new(base_source: String, macros: &[(&str, bool)]) -> Self;
    pub fn compile(&self, compiler: &ShaderCompiler) -> Result<ShaderModule>;
    pub fn key(&self) -> ShaderKey;
}
```

---

## 2.3 ShaderCompiler

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 27, 53, 54 | `ShaderCompiler::new() -> Self` | 创建编译器 |
| 55 | `ShaderCompiler::compile_wgsl(src: &str) -> Result<ShaderModule>` | 编译 WGSL |
| 56 | `ShaderCompiler::compile_glsl(src: &str, stage: ShaderStage) -> Result<ShaderModule>` | 编译 GLSL |
| 57 | `ShaderCompiler::inspect_errors(src: &str) -> Vec<Diagnostic>` | 语法错误定位 |
| 246 | `ShaderCompiler::compile(&self, source: &ShaderSource, lang: ShaderLanguage, stage: ShaderStage) -> Result<ShaderModule>` | 通用编译接口 |
| 247 | `ShaderCompiler::diagnostics(&self) -> Vec<Diagnostic>` | 获取诊断信息 |

---

## 2.4 ShaderHotReload

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 45, 71 | `ShaderHotReload::watch(path: &Path, callback: F)` | 监听文件变化 |
| 45, 72 | `ShaderHotReload::tick(&mut self)` | 轮询检查变化 |

---

## 2.5 ShaderLibrary

Shader 库提供常用头文件与工具函数。

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 33, 59 | `ShaderLibrary::include(name: &str) -> &str` | 获取 include 内容 |
| 33, 60 | `ShaderLibrary::brdf_functions() -> &str` | BRDF 函数库 |
| 33, 61 | `ShaderLibrary::pbr_common() -> &str` | PBR 通用代码 |
| 33, 62 | `ShaderLibrary::utils() -> &str` | 工具函数 |

---

## 2.6 PBR 工具函数

以下函数在 ShaderLibrary 中提供，供 PBR 着色器使用：

| 需求编号 | 函数签名 | 说明 |
|----------|----------|------|
| 34, 63 | `pow5(x: f32) -> f32` | x 的 5 次方 |
| 34, 64 | `saturate(x: f32) -> f32` |  clamp(x, 0.0, 1.0) |
| 34, 65 | `fresnel_schlick(cos_theta: f32, f0: f32) -> f32` | Fresnel Schlick 近似 |
| 34, 66 | `fresnel_schlick_roughness(cos_theta: f32, f0: f32, roughness: f32) -> f32` | 带粗糙度的 Fresnel |
| 34, 67 | `ndf_ggx(n_dot_h: f32, roughness: f32) -> f32` | GGX 法线分布函数 |
| 34, 68 | `geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32` | Schlick GGX 几何函数 |
| 34, 69 | `geometry_smith(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32` | Smith 几何遮蔽 |
| 34, 70 | `diffuse_lambert() -> f32` | Lambert 漫反射系数 |
| 34, 71 | `cook_torrance(n_dot_v: f32, n_dot_l: f32, n_dot_h: f32, v_dot_h: f32, roughness: f32, f0: f32) -> f32` | Cook-Torrance BRDF |
| 34, 72 | `tangent_bitangent(normal: vec3, uv: vec2) -> mat3` | TBN 矩阵构造 |
| 34 | `normal_sample(normal_map: texture_2d, uv: vec2, tbn: mat3) -> vec3` | 法线贴图采样 |

---

## 2.7 PBR Shader 源

### 2.7.1 PbrShader

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 35, 310 | `PbrShader::main_vs() -> ShaderSource` | 主顶点着色器 |
| 35, 311 | `PbrShader::main_fs() -> ShaderSource` | 主片段着色器 |
| 36, 312 | `PbrShader::skybox_vs() -> ShaderSource` | 天空盒顶点着色器 |
| 36, 313 | `PbrShader::skybox_fs() -> ShaderSource` | 天空盒片段着色器 |
| 37, 314 | `PbrShader::shadow_vs() -> ShaderSource` | 阴影顶点着色器 |
| 37, 315 | `PbrShader::shadow_fs() -> ShaderSource` | 阴影片段着色器 |
| 38, 316 | `PbrShader::ibl_irradiance_cs() -> ShaderSource` | IBL 辐照度计算 Compute Shader |
| 38, 317 | `PbrShader::ibl_prefilter_cs() -> ShaderSource` | IBL 预滤波 Compute Shader |
| 38, 318 | `PbrShader::ibl_brdf_lut_cs() -> ShaderSource` | BRDF LUT Compute Shader |

### 2.7.2 PBR 主着色器特性支持

| 需求编号 | 特性 | 说明 |
|----------|------|------|
| 35 | PBR Lit 顶点/片段着色器 | 基础 PBR 光照 |
| 36 | IBL 支持 | irradiance + prefilter + brdf_lut |
| 37 | 多光源支持 | 点光源、方向光、聚光灯 |
| 38 | 阴影支持 | shadow map |
| 39 | Normal mapping | 法线贴图 |
| 40 | Emissive + Bloom | 自发光与泛光预备 |
| 41 | Parallax occlusion mapping | 视差遮蔽映射 |
| 42 | 预处理器宏 | PBR_IBL / PBR_SHADOW / PBR_PARALLAX / PBR_ANISOTROPY / PBR_CLEAR_COAT / PBR_SUBSURFACE |

---

## 2.8 Shader 代码生成

### 2.8.1 代码生成器（ShaderGraph 编译后端）

| 需求编号 | 签名 | 说明 |
|----------|------|------|
| 51, 78 | 按拓扑排序生成 | 拓扑排序后依次生成各节点代码 |
| 52, 79 | 循环/分支 DAG 展开 | 简化控制流 |
| 53, 80 | 内置节点代码生成 | 纹理采样、常量、UV、时间等 |
| 54, 81 | PBR Master 节点 | 生成完整 BRDF 主函数 |
| 357 | `ShaderGraph::generate_wgsl(&self) -> String` | 生成 WGSL 代码 |
| 358 | `ShaderGraph::generate_glsl(&self) -> String` | 生成 GLSL 代码 |

---

## 2.9 输入与输出

### 输入
- Shader 源代码（WGSL/GLSL/HLSL/MSL）
- 宏定义列表
- include 路径

### 输出
- `ShaderModule`: 编译后的着色器模块
- `ShaderKey`: 用于缓存查找的哈希键
- `Vec<Diagnostic>`: 编译诊断信息

---

## 2.10 验收标准

| 编号 | 标准 |
|------|------|
| 27 | `ShaderCompiler` 支持 WGSL/GLSL 编译 |
| 31 | `inspect_errors` 正确返回语法错误位置 |
| 33 | ShaderLibrary 包含 brdf/pbr_common/utils |
| 34 | 所有 PBR 工具函数正确实现 |
| 42 | 预处理器宏正确切换特性 |
| 43 | `ShaderPermutation` 按宏组合编译并缓存 |
| 44 | `ShaderKey` 哈希映射确保相同 permutation 复用 |
| 45 | `ShaderHotReload` 监听文件变化自动重建 |
| 285 | `ShaderCompiler::compile` 通用接口工作正常 |
| 291 | `ShaderKey::hash` 不同 feature 产生不同 hash |

---

## 2.11 依赖关系

### 依赖模块
- `naga`: 跨语言 Shader 编译
- `notify`: 文件系统监听
- `hashbrown`: 哈希映射

### 被依赖模块
- `engine-render`: Pipeline 构建
- `ShaderGraph`: 代码生成

---

## 2.12 优先级

| 优先级 | 需求编号 | 说明 |
|--------|----------|------|
| P0 | 27, 35-42, 53-54, 63-71 | 核心 Shader 编译与 PBR 函数 |
| P1 | 28-34, 43-45, 55-62, 72 | 工具函数、热重载、库 |
| P2 | 310-318 | IBL Compute Shader |
