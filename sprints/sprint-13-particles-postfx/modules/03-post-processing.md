# 模块三：后期处理栈

## 模块名称与概述

**模块名称**：Post-Processing Stack  
**模块路径**：`engine-postfx` crate  
**功能概述**：建立模块化后期处理管线 PostProcessStack，按顺序执行一系列 PostProcessPass（FXAA/TAA/Bloom/DOF/SSAO/SSR/ToneMapping/ColorGrading/Vignette/ChromaticAberration/MotionBlur/Grain/LensDistortion/ScreenSpaceShadows），支持 HDR/LDR 双路径与 render graph 集成。

---

## 需求编号

对应原需求文档编号：**2, 49, 102, 103-179, 185-228, 364-380, 444-476, 488-507, 510-528**

---

## 功能描述

---

## Part A：PostProcessStack 组件

### A.1 核心结构

```rust
pub struct PostProcessStack {
    passes: Vec<Box<dyn IPostProcessPass>>,
    enabled: bool,
    hdr: bool,
    render_target_pool: RenderTargetPool,
}

impl PostProcessStack {
    pub fn new() -> Self;
    pub fn add_pass(&mut self, pass: Box<dyn IPostProcessPass>);
    pub fn insert_pass(&mut self, index: usize, pass: Box<dyn IPostProcessPass>);
    pub fn remove_pass(&mut self, index: usize);
    pub fn passes(&self) -> &[Box<dyn IPostProcessPass>];
    pub fn order(&self) -> Vec<usize>;
    pub fn reorder(&mut self, new_order: Vec<usize>);
    pub fn enabled(&self) -> bool;
    pub fn set_enabled(&mut self, b: bool);
    pub fn hdr(&self) -> bool;
    pub fn set_hdr(&mut self, b: bool);
    
    // 核心执行
    pub fn apply(&mut self, ctx: PostProcessContext, scene_color: &Texture, output: &mut Texture);
}
```

### A.2 Pass 管理

- `add_pass`：追加到链尾
- `insert_pass`：插入指定索引
- `remove_pass`：移除指定索引
- `reorder`：调整 Pass 执行顺序

---

## Part B：IPostProcessPass Trait

### B.1 Trait 定义

```rust
pub trait IPostProcessPass: Any + Send + Sync {
    fn name(&self) -> &str;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, b: bool);
    fn declare_resources(&self, graph_builder: &mut RenderGraphBuilder);
    fn apply(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture);
}
```

### B.2 Pass 类型枚举

```rust
pub enum PostProcessPass {
    FXAA(FXAAPass),
    TAA(TAAPass),
    Bloom(BloomPass),
    DepthOfField(DOFPass),
    SSAO(SSAOPass),
    HBAO(HBAOPass),
    SSR(SSRPass),
    ToneMapping(ToneMappingPass),
    ColorGrading(ColorGradingPass),
    Vignette(VignettePass),
    ChromaticAberration(ChromaticAberrationPass),
    MotionBlur(MotionBlurPass),
    Grain(GrainPass),
    LensDistortion(LensDistortionPass),
    ScreenSpaceShadows(ScreenSpaceShadowsPass),
}
```

---

## Part C：PostProcessContext

```rust
pub struct PostProcessContext<'a> {
    pub renderer: &'a Renderer,
    pub camera: &'a Camera,
    pub time: f32,
    pub viewport: Rect,
    pub depth_rt: Option<Handle<Texture>>,
    pub normal_rt: Option<Handle<Texture>>,
    pub motion_rt: Option<Handle<Texture>>,
    pub hdr: bool,
}
```

---

## Part D：渲染管线与资源管理

### D.1 RenderTargetPool

```rust
pub struct RenderTargetPool {
    pool: HashMap<(UVec2, TextureFormat), Vec<Handle<Texture>>>,
}

impl RenderTargetPool {
    pub fn acquire(&mut self, size: UVec2, format: TextureFormat) -> Handle<Texture>;
    pub fn release(&mut self, handle: Handle<Texture>);
    pub fn on_frame_end(&mut self);  // 回收上一帧资源
}
```

### D.2 Pass 之间 Ping-Pong

```rust
pub struct PingPongRT {
    ping: Handle<Texture>,
    pong: Handle<Texture>,
    current: usize,  // 0 = ping, 1 = pong
}

impl PingPongRT {
    pub fn read(&self) -> Handle<Texture>;
    pub fn write(&mut self) -> Handle<Texture>;
    pub fn flip(&mut self);
}
```

### D.3 Pipeline 构建

```rust
pub struct PostProcessPipeline {
    stack: PostProcessStack,
    scene_input: NodeHandle,
}

impl PostProcessPipeline {
    pub fn build(graph_builder: &mut RenderGraphBuilder, stack: PostProcessStack, scene_input: NodeHandle) -> NodeHandle;
    pub fn requires_depth(&self) -> bool;
    pub fn requires_normal(&self) -> bool;
    pub fn requires_motion(&self) -> bool;
}
```

---

## Part E：FXAA Pass

```rust
pub struct FXAAPass {
    threshold: f32,        // 默认 0.063
    edge_threshold: f32,    // 默认 0.0312
    quality: FXAAQuality,
}

pub enum FXAAQuality { Low, Medium, High, Ultra }

impl FXAAPass {
    pub fn new() -> Self;
    pub fn with_quality(quality: FXAAQuality) -> Self;
    pub fn with_threshold(threshold: f32) -> Self;
    pub fn threshold(&self) -> f32;
    pub fn edge_threshold(&self) -> f32;
    pub fn apply(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture);
}
```

**算法**：luma-based edge detection + subpixel AA

---

## Part F：TAA Pass（Temporal Anti-Aliasing）

```rust
pub struct TAAPass {
    history_buffer: Handle<Texture>,  // double-buffered
    jitter: Vec<Vec2>,                 // Halton 序列
    feedback: f32,                     // 默认 0.9
}

impl TAAPass {
    pub fn new() -> Self;
    pub fn history_buffer(&self) -> Handle<Texture>;
    pub fn jitter(&self) -> Vec<Vec2>;
    pub fn feedback(&self) -> f32;
    pub fn clamp_neighborhood(&self, color_tex: &Texture, position_tex: &Texture);
    pub fn apply(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture);
}
```

---

## Part G：Vignette Pass

```rust
pub struct VignettePass {
    intensity: f32,      // 默认 0.45
    smoothness: f32,     // 默认 0.2
    roundness: f32,      // 默认 1.0（0 为方形）
    center: Vec2,        // 默认 (0.5, 0.5)
    color: Rgba,         // 默认纯黑
}

impl VignettePass {
    pub fn new() -> Self;
    pub fn with_intensity(i: f32) -> Self;
    pub fn intensity(&self) -> f32;
    pub fn smoothness(&self) -> f32;
    pub fn roundness(&self) -> f32;
    pub fn center(&self) -> Vec2;
    pub fn color(&self) -> Rgba;
    pub fn apply(&self, input: &Texture, output: &mut Texture);
}
```

---

## Part H：ChromaticAberration Pass

```rust
pub struct ChromaticAberrationPass {
    strength: f32,       // 默认 0.3
    max_offset: f32,     // 默认 0.02
}

impl ChromaticAberrationPass {
    pub fn new() -> Self;
    pub fn with_strength(s: f32) -> Self;
    pub fn strength(&self) -> f32;
    pub fn max_offset(&self) -> f32;
    pub fn apply(&self, input: &Texture, output: &mut Texture);  // RGB 通道径向偏移
}
```

---

## Part I：MotionBlur Pass

```rust
pub struct MotionBlurPass {
    sample_count: u32,       // 默认 12
    shutter_angle: f32,       // 默认 270°（弧度制 3π/4）
    velocity_scale: f32,
    max_velocity: f32,
}

impl MotionBlurPass {
    pub fn new() -> Self;
    pub fn sample_count(&self) -> u32;
    pub fn shutter_angle(&self) -> f32;
    pub fn velocity_scale(&self) -> f32;
    pub fn max_velocity(&self) -> f32;
    pub fn apply(&self, input: &Texture, motion_vector: &Texture, output: &mut Texture);
}

pub struct MotionVectorTexture;
impl MotionVectorTexture {
    pub fn generate(prev_vp: &Mat4, curr_vp: &Mat4, depth: &Texture) -> Texture;
}
```

---

## Part J：Grain Pass

```rust
pub struct GrainPass {
    intensity: f32,              // 默认 0.15
    size: f32,                   // 默认 1.0
    luminance_contribution: f32, // 默认 0.8（暗部更明显）
    seed: f32,
}

impl GrainPass {
    pub fn new() -> Self;
    pub fn intensity(&self) -> f32;
    pub fn size(&self) -> f32;
    pub fn luminance_contribution(&self) -> f32;
    pub fn seed(&self) -> f32;
    pub fn apply(&self, input: &Texture, output: &mut Texture);
}
```

---

## Part K：LensDistortion Pass

```rust
pub struct LensDistortionPass {
    k1: f32,   // 径向畸变一阶
    k2: f32,   // 径向畸变二阶
    k3: f32,   // 径向三阶
    p1: f32,   // 切向一阶
    p2: f32,   // 切向二阶
    center: Vec2,
}

impl LensDistortionPass {
    pub fn new() -> Self;
    pub fn k1(&self) -> f32;
    pub fn k2(&self) -> f32;
    pub fn k3(&self) -> f32;
    pub fn p1(&self) -> f32;
    pub fn p2(&self) -> f32;
    pub fn center(&self) -> Vec2;
    pub fn apply(&self, input: &Texture, output: &mut Texture);  // Brown–Conrady 模型
}
```

---

## Part L：ScreenSpaceShadows Pass

```rust
pub struct ScreenSpaceShadowsPass {
    step_count: u32,
    thickness: f32,
    max_distance: f32,
}

impl ScreenSpaceShadowsPass {
    pub fn new() -> Self;
    pub fn step_count(&self) -> u32;
    pub fn thickness(&self) -> f32;
    pub fn max_distance(&self) -> f32;
    pub fn apply(&self, depth: &Texture, normal: &Texture, light_dir: Vec3, output: &mut Texture);
}
```

---

## Part M：PostProcessDebugView

```rust
pub enum DebugViewMode {
    None,
    Depth,
    Normal,
    MotionVector,
    AmbientOcclusion,
    BloomOnly,
    ColorGradingOnly,
}

pub struct PostProcessDebugView {
    mode: DebugViewMode,
}

impl PostProcessDebugView {
    pub fn new(mode: DebugViewMode) -> Self;
    pub fn mode(&self) -> DebugViewMode;
    pub fn set_mode(&mut self, mode: DebugViewMode);
    pub fn apply(&self, input: &Texture, output: &mut Texture, ctx: &PostProcessContext);
}
```

---

## 渲染管线集成

| 属性 | 值 |
|------|-----|
| render graph 阶段 | `PostProcessing`（在 ForwardLighting 之后） |
| HDR pipeline 格式 | R11G11B10 / RGBA16F |
| LDR pipeline 格式 | RGBA8 sRGB |
| 深度缓冲格式 | D32S8 / D24S8 |
| 运动向量格式 | RG16F |
| 法线图格式 | RGB10A2 |

---

## 验收标准

| ID | 描述 |
|----|------|
| V1 | PostProcessStack::add_pass/remove_pass/reorder 正确管理 Pass 列表 |
| V2 | PostProcessStack::apply 按顺序串联所有 Pass |
| V3 | FXAAPass enabled=false 时 apply 为 no-op |
| V4 | VignettePass 中心像素输出 = 输入 |
| V5 | MotionVectorTexture::generate 前向单位向量 |
| V6 | RenderTargetPool::acquire 复用已有尺寸/格式的 RT |
| V7 | Ping-pong 切换正确（read/write/flip） |
| V8 | HDR/LDR 双路径正确分支 |
| V9 | Pass 之间自动决定最小纹理尺寸（如 bloom 降采样 1/4） |
| V10 | 半分辨率 AO 提升性能 |

---

## 依赖关系

**前置依赖**：
- `engine-render`：Texture/RenderTarget/RenderGraph
- `engine-camera`：Camera/Perspective

**被依赖**：
- `examples/postfx_*`：所有示例依赖此模块

---

## 优先级

**P0**：
- PostProcessStack / IPostProcessPass 抽象
- FXAA / Bloom / ToneMapping
- Vignette

**P1**：
- TAA / DOF / SSAO / SSR
- ColorGrading
- ChromaticAberration / MotionBlur

**P2**：
- Grain / LensDistortion
- ScreenSpaceShadows
- PostProcessDebugView
