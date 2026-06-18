# Module 06 — 后处理效果

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)
> 文件位置: `engine-postfx/src/`

## 1. Bloom

```rust
pub struct BloomPass {
    pub threshold: f32,    // 默认 1.0
    pub intensity: f32,    // 默认 0.5
    pub levels: u32,       // 默认 5
    pub downsample_pipelines: Vec<GpuPipelineState>,
    pub upsample_pipelines: Vec<GpuPipelineState>,
}

impl BloomPass {
    pub fn apply(
        &self,
        ctx: &mut RenderContext,
        input: &GpuTexture,
        output: &mut GpuTexture,
    ) {
        // 1. Threshold pass：提取高亮
        // 2. Downsample：1/2, 1/4, 1/8, 1/16, 1/32
        // 3. Upsample + composite：与原图混合
    }
}
```

```glsl
// downsample
vec4 downsample(sampler2D tex, vec2 uv, vec2 texel_size) {
    vec4 sum = texture(tex, uv) * 4.0;
    sum += texture(tex, uv + vec2(-1, -1) * texel_size);
    sum += texture(tex, uv + vec2( 1, -1) * texel_size);
    sum += texture(tex, uv + vec2(-1,  1) * texel_size);
    sum += texture(tex, uv + vec2( 1,  1) * texel_size);
    return sum / 8.0;
}

// upsample with bilinear
vec4 upsample(sampler2D tex, vec2 uv, vec2 texel_size, float radius) {
    vec4 sum = vec4(0.0);
    sum += texture(tex, uv + vec2(-1, -1) * texel_size * radius);
    sum += texture(tex, uv + vec2( 0, -1) * texel_size * radius) * 2.0;
    sum += texture(tex, uv + vec2( 1, -1) * texel_size * radius);
    sum += texture(tex, uv + vec2(-1,  0) * texel_size * radius) * 2.0;
    sum += texture(tex, uv) * 4.0;
    sum += texture(tex, uv + vec2( 1,  0) * texel_size * radius) * 2.0;
    sum += texture(tex, uv + vec2(-1,  1) * texel_size * radius);
    sum += texture(tex, uv + vec2( 0,  1) * texel_size * radius) * 2.0;
    sum += texture(tex, uv + vec2( 1,  1) * texel_size * radius);
    return sum / 16.0;
}
```

## 2. Depth of Field (DoF)

```rust
pub struct DofPass {
    pub focal_distance: f32,
    pub focal_range: f32,    // 焦距范围
    pub bokeh_radius: f32,
    pub max_blur: f32,
    pub coc_buffer: GpuTexture,    // Circle of Confusion
    pub pipeline: GpuPipelineState,
}

impl DofPass {
    pub fn compute_coc(&self, depth: &GpuTexture) -> GpuTexture;
    pub fn bokeh_blur(&self, scene: &GpuTexture, coc: &GpuTexture) -> GpuTexture;
}
```

```glsl
// CoC 计算
float coc = (depth - focal_distance) / focal_range;
coc = clamp(coc, -1.0, 1.0);

// Bokeh 散景
vec3 bokeh_sample(sampler2D tex, vec2 uv, float coc, vec2 texel_size) {
    vec3 sum = vec3(0.0);
    float total = 0.0;
    for (int i = 0; i < 32; i++) {
        vec2 offset = bokeh_offsets[i] * abs(coc) * bokeh_radius * texel_size;
        sum += texture(tex, uv + offset).rgb;
        total += 1.0;
    }
    return sum / total;
}
```

## 3. SSAO

```rust
pub enum SsaoAlgorithm {
    Classic,   // 经典 SSAO
    HbaoPlus,  // Horizon-based
    Gtao,      // Ground Truth AO
}

pub struct SsaoPass {
    pub algorithm: SsaoAlgorithm,
    pub radius: f32,
    pub bias: f32,
    pub samples: u32,
    pub noise_texture: GpuTexture,
    pub pipeline: GpuPipelineState,
    pub blur_pipeline: GpuPipelineState,
}
```

```glsl
// SSAO sample
float ao(vec3 pos, vec3 normal, vec3 sample_pos, float radius) {
    vec3 diff = sample_pos - pos;
    float dist = length(diff);
    vec3 dir = normalize(diff);
    return max(0.0, dot(normal, dir)) * (1.0 - smoothstep(0.0, radius, dist));
}
```

## 4. SSR (Screen Space Reflection)

```rust
pub struct SsrPass {
    pub max_steps: u32,      // 默认 64
    pub thickness: f32,      // 默认 0.5
    pub stride: u32,         // 默认 1
    pub max_distance: f32,
    pub hi_z_pyramid: HiZPyramid,
    pub pipeline: GpuPipelineState,
}

pub struct HiZPyramid {
    pub mip_levels: Vec<GpuTexture>,
}
```

```glsl
// SSR ray march
vec4 ssr_trace(vec3 ro, vec3 rd, sampler2D hi_z, mat4 proj) {
    float t = 0.0;
    vec3 curr = ro;
    for (int i = 0; i < max_steps; i++) {
        curr = ro + rd * t;
        vec4 clip = proj * vec4(curr, 1.0);
        vec2 uv = clip.xy / clip.w * 0.5 + 0.5;
        float scene_depth = sample_hi_z(hi_z, uv);
        float curr_depth = clip.z / clip.w;
        if (curr_depth > scene_depth + thickness) {
            return vec4(curr, t);
        }
        t += stride;
    }
    return vec4(0.0);  // miss
}
```

## 5. Color Grading

```rust
pub struct ColorGrading {
    pub lift: Vec3,    // 阴影
    pub gamma: Vec3,   // 中间调
    pub gain: Vec3,    // 高光
    pub saturation: f32,
    pub contrast: f32,
    pub lut: Option<Lut>,
}

pub struct Lut {
    pub data: Vec<Vec3>,  // 33³ × 3
    pub size: u32,        // 33
}
```

```glsl
vec3 color_grading(vec3 color) {
    color = color * gain + lift;
    color = pow(color, 1.0 / gamma);
    color = mix(vec3(dot(color, vec3(0.299, 0.587, 0.114))), color, saturation);
    color = (color - 0.5) * contrast + 0.5;
    if (use_lut) {
        color = texture(lut_3d, color).rgb;
    }
    return color;
}
```

## 6. 验收

- [ ] Bloom 5 级 < 0.5 ms GPU @ 1080p
- [ ] SSAO 16 采样 < 0.5 ms GPU
- [ ] SSR 64 步 < 1 ms GPU
- [ ] 色彩分级 LUT 256³ < 0.1 ms
- [ ] DoF 散景视觉自然
- [ ] Bloom 阈值 / 强度可调
