# 模块六：色调映射与色彩分级

## 模块名称与概述

**模块名称**：Tone Mapping and Color Grading  
**模块路径**：`engine-postfx` crate  
**功能概述**：实现 HDR 到 LDR 的色调映射（ACES/Reinhard/Linear/Neutral/Filmic）以及完整色彩分级（ LUT/白平衡/饱和度/对比度/色调/曲线），为画面提供电影级调色能力。

---

## 需求编号

对应原需求文档编号：**119-120, 138-142, 186-191, 434-444, 528-556, 564-565**

---

## 功能描述

---

## Part A：ToneMapping Pass

### A.1 色调映射模式

```rust
pub enum ToneMappingMode {
    Linear,                              // 无操作（已在线性空间）
    Reinhard,                            // L / (1 + L)
    ReinhardExtended { white: f32 },     // 扩展版，可指定白点
    ACES,                                 // Academy Color Encoding System
    Neutral,                              // 中性调
    Filmic,                               // Uncharted 2 风格
}
```

### A.2 核心结构

```rust
pub struct ToneMappingPass {
    mode: ToneMappingMode,
    exposure: f32,  // 曝光值（默认 1.0）
}

impl ToneMappingPass {
    pub fn new(mode: ToneMappingMode) -> Self;
    pub fn exposure(&self) -> f32;           // 默认 1.0
    pub fn set_exposure(&mut self, e: f32);
    pub fn apply(&self, hdr_input: &Texture, ldr_output: &mut Texture);
}
```

### A.3 算法实现

```rust
// Reinhard
fn reinhard(luminance: f32) -> f32 {
    luminance / (1.0 + luminance)
}

// Reinhard Extended
fn reinhard_extended(luminance: f32, white: f32) -> f32 {
    (luminance * (1.0 + luminance / (white * white))) / (1.0 + luminance)
}

// ACES（约等于 S-curve）
fn tonemapping_aces(x: f32) -> f32 {
    // fitted ACES function
    const a: f32 = 2.51;
    const b: f32 = 0.03;
    const c: f32 = 2.43;
    const d: f32 = 0.59;
    const e: f32 = 0.14;
    (x * (a * x + b)) / (x * (c * x + d) + e)
}

// Neutral（中性调，保持色相）
fn tonemapping_neutral(x: f32) -> f32 {
    // 线性压缩暗部，保持高光
    x / (x + 0.6)
}

// Filmic（Uncharted 2）
fn tonemapping_filmic(x: f32) -> f32 {
    const a: f32 = 0.15;
    const b: f32 = 0.50;
    const c: f32 = 0.10;
    const d: f32 = 0.20;
    const e: f32 = 0.02;
    const f: f32 = 0.30;
    ((x * (a * x + c * b) + d * e) - x * (f * b)) / (x * (a * x + b) + d * f)
}
```

### A.4 API 签名

```rust
impl ToneMappingPass {
    pub fn new(mode: ToneMappingMode) -> Self;
    pub fn with_exposure(exposure: f32) -> Self;
    pub fn mode(&self) -> &ToneMappingMode;
    pub fn exposure(&self) -> f32;
    pub fn set_exposure(&mut self, e: f32);
    pub fn apply(&self, hdr_input: &Texture, ldr_output: &mut Texture);
}
```

**输入**：HDR 纹理（R11G11B10 或 RGBA16F）  
**输出**：LDR 纹理（RGBA8 sRGB）

---

## Part B：ColorGrading Pass

### B.1 核心结构

```rust
pub struct ColorGradingPass {
    lut: Option<Handle<Texture3D>>,      // 32x32x32 LUT
    white_balance: (f32, f32),           // (温度, 色调)
    saturation: f32,                    // 饱和度（默认 1.0）
    contrast: f32,                      // 对比度（默认 1.0）
    hue_shift: f32,                     // 色调偏移（弧度）
    lift: Vec3,                         // 阴影色（默认 0）
    gamma: Vec3,                        // 中灰（默认 1）
    gain: Vec3,                         // 高光（默认 1）
    tone_curve: Option<Curve<f32>>,     // 色调曲线
}
```

### B.2 API 签名

```rust
impl ColorGradingPass {
    pub fn new() -> Self;
    pub fn lut(&self) -> Option<Handle<Texture3D>>;  // 32x32x32 LUT
    pub fn set_lut(&mut self, lut: Option<Handle<Texture3D>>);
    pub fn white_balance(&self) -> (f32, f32);     // (温度, 色调)
    pub fn saturation(&self) -> f32;               // 默认 1.0
    pub fn contrast(&self) -> f32;                  // 默认 1.0
    pub fn hue_shift(&self) -> f32;                 // 弧度
    pub fn lift(&self) -> Vec3;                      // 阴影色
    pub fn gamma(&self) -> Vec3;                     // 中灰
    pub fn gain(&self) -> Vec3;                      // 高光
    pub fn tone_curve(&self) -> Option<&Curve<f32>>;
    pub fn global_tone_curve(&self) -> Curve<f32>;
    pub fn apply(&self, input: &Texture, output: &mut Texture);
}
```

### B.3 白平衡

```rust
// 温度（Kelvin）：默认 6500K（日光）
// 色调（Tint）：绿色←→品红，默认 0

fn apply_white_balance(color: Vec3, temperature: f32, tint: f32) -> Vec3 {
    // 转换为 LMS 空间
    // 温度偏移：沿着 Planckian locus 移动
    // 色调偏移：green↔magenta 轴偏移
    // 转回 RGB
}
```

### B.4 Lift/Gamma/Gain（阴影/中灰/高光）

```rust
// Lift = 阴影色（+black point）
// Gamma = 中灰（midtones）
// Gain = 高光色（+white point）

// Formula: output = gain * (input^gamma) + lift
fn apply_lgg(color: Vec3, lift: Vec3, gamma: Vec3, gain: Vec3) -> Vec3 {
    gain * color.powf(gamma) + lift
}
```

### B.5 色调曲线

```rust
// 使用 Curve<f32> 定义 RGB 统一色调曲线
// t=0 → t=1 映射输出值
// 用于 S-curve、log 转 linear 等

pub struct Curve<T> {
    keyframes: Vec<(f32, T)>,  // (t, value)
}

impl Curve<f32> {
    pub fn sample(&self, t: f32) -> f32 {
        // 线性插值 keyframes
    }
}
```

### B.6 LUT 支持

```rust
// 32x32x32 3D LUT
// 用于高精度调色（替代程序化操作）

// 生成中性 LUT
pub struct Lut3D;
impl Lut3D {
    pub fn generate_neutral(size: u32) -> Handle<Texture3D> {
        // 生成 32x32x32 纯色 LUT
        // 实际调色在 shader 中采样 LUT
    }
}

// 在 shader 中：
// vec3 graded = textureLod(lut, color * (31.0/32.0) + 0.5/32.0).rgb;
```

---

## Part C：其他色彩相关 Pass

### C.1 独立 Vignette（扩展，见模块三）

```rust
// 模块三已定义，此处补充参数说明
pub struct VignettePass {
    intensity: f32,      // 默认 0.45
    smoothness: f32,     // 默认 0.2
    roundness: f32,      // 默认 1.0
    center: Vec2,        // 默认 (0.5, 0.5)
    color: Rgba,         // 默认纯黑
}
```

### C.2 独立 ChromaticAberration（扩展，见模块三）

```rust
pub struct ChromaticAberrationPass {
    strength: f32,       // 默认 0.3
    max_offset: f32,     // 默认 0.02
}
```

---

## 输入/输出规格

### ToneMapping

| 输入 | 输出 |
|------|------|
| HDR (RGBA16F / R11G11B10) | LDR (RGBA8 sRGB) |

### ColorGrading

| 输入 | 输出 |
|------|------|
| ToneMapped 场景 | 调色后场景 |
| 可选 3D LUT | LUT 采样结果 |

---

## 验收标准

| ID | 描述 |
|----|------|
| V1 | ToneMappingMode::Reinhard 对 L=1 返回 0.5 |
| V2 | ToneMappingPass::apply 正确执行 exposure * color |
| V3 | ACES tone mapping 曲线符合 S-shape |
| V4 | ColorGradingPass::white_balance 纯灰 (0.5,0.5,0.5) 不变 |
| V5 | ColorGradingPass::saturation=0 时输出灰度 |
| V6 | ColorGradingPass::contrast=1 时保持不变 |
| V7 | Lift/Gamma/Gain 独立作用于阴影/中灰/高光 |
| V8 | Lut3D::generate_neutral 生成正确的 32x32x32 纹理 |
| V9 | ColorGradingPass 正确采样 3D LUT |
| V10 | Vignette 中心像素输出 = 输入 |
| V11 | 后期栈默认链 <= 6ms（包括 ToneMapping + ColorGrading） |

---

## 依赖关系

**前置依赖**：
- `03-post-processing`：PostProcessContext / IPostProcessPass
- `engine-gfx`：Texture3D / RenderTarget

**被依赖**：
- `examples/postfx_color`：调色演示

---

## 优先级

**P0**：
- ToneMapping 所有模式
- ColorGrading 基础参数（saturation/contrast/hue/lift/gamma/gain）

**P1**：
- LUT 支持
- White Balance
- Tone Curve

**P2**：
- 独立 Vignette / ChromaticAberration（后续可拆分独立 Pass）

---

## 性能目标

- ToneMapping 单 Pass <= 0.5ms
- ColorGrading（无 LUT）<= 1ms
- ColorGrading（有 LUT）<= 2ms
- LUT 3D 采样在 shader 中完成（无 CPU 开销）
