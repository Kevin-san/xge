# Module 05 — HDR 渲染目标与 Tone Mapping

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)

## 1. 目标

HDR 渲染流程：
- HDR 多重渲染目标 (MRT)：albedo / normal / material / depth
- ACES Filmic Tone Mapping
- FXAA / TAA 抗锯齿

## 2. HDR Target

```rust
pub struct HdrTarget {
    pub albedo: GpuTexture,     // RGBA8
    pub normal: GpuTexture,     // RG16F
    pub material: GpuTexture,   // RGBA8
    pub depth: GpuTexture,      // D32F
    pub size: (u32, u32),
}

impl HdrTarget {
    pub fn new(device: &GpuDevice, width: u32, height: u32) -> Self;
    pub fn resize(&mut self, device: &GpuDevice, width: u32, height: u32);
}
```

## 3. ACES Filmic Tone Mapping

```glsl
vec3 ACESFilm(vec3 x) {
    float a = 2.51;
    float b = 0.03;
    float c = 2.43;
    float d = 0.59;
    float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

vec3 tone_map(vec3 hdr, float exposure) {
    return ACESFilm(hdr * exposure);
}
```

## 4. 曝光控制

```rust
pub struct Exposure {
    pub auto: bool,
    pub ev100: f32,  // 100 ISO 下曝光值
    pub min: f32,
    pub max: f32,
    pub current: f32,
}

impl Exposure {
    pub fn update(&mut self, avg_luminance: f32, dt: f32) {
        // 自动曝光
        let target = 0.18;  // 中灰
        let diff = target / (avg_luminance + 1e-5);
        self.current = self.current * 0.95 + diff * 0.05;
        self.current = self.current.clamp(self.min, self.max);
    }
}
```

## 5. FXAA

```rust
pub struct FxaaPass {
    pub pipeline: GpuPipelineState,
    pub sampler: GpuSampler,
}

impl FxaaPass {
    pub fn new(device: &GpuDevice) -> Self;
    pub fn apply(&self, ctx: &mut RenderContext, input: &GpuTexture, output: &GpuTexture);
}
```

## 6. TAA（可选）

```rust
pub struct TaaPass {
    pub history: GpuTexture,  // 前一帧
    pub jitter: HaltonSequence,
    pub pipeline: GpuPipelineState,
}

impl TaaPass {
    pub fn apply(&self, ctx: &mut RenderContext, current: &GpuTexture, velocity: &GpuTexture);
}

pub struct HaltonSequence {
    base: (u32, u32),  // (2, 3)
    frame: u32,
}

impl HaltonSequence {
    pub fn next(&mut self) -> Vec2 {
        self.frame += 1;
        Vec2::new(
            halton(self.base.0, self.frame),
            halton(self.base.1, self.frame),
        ) * 2.0 - 1.0
    }
}
```

## 7. Gamma 校正

```glsl
vec3 linear_to_srgb(vec3 linear) {
    return mix(
        1.055 * pow(linear, vec3(1.0 / 2.4)) - 0.055,
        linear * 12.92,
        step(linear, vec3(0.0031308))
    );
}
```

## 8. 验收

- [ ] HDR 渲染目标线性空间
- [ ] ACES Filmic 视觉对比
- [ ] FXAA 边缘平滑无 ghosting
- [ ] TAA 时间稳定（jitter 8 frame 收敛）
- [ ] 曝光自动调节 < 1 frame 响应
