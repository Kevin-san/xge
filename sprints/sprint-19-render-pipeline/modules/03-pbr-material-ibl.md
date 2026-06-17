# Module 03 — PBR 材质与 IBL

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)

## 1. PBR Material

```rust
pub struct PbrMaterial {
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub ao: f32,
    pub emissive: Vec3,
    pub alpha_mode: AlphaMode,
    pub two_sided: bool,
    pub albedo_texture: Option<TextureHandle>,
    pub normal_texture: Option<TextureHandle>,
    pub metallic_roughness_texture: Option<TextureHandle>,
    pub occlusion_texture: Option<TextureHandle>,
    pub emissive_texture: Option<TextureHandle>,
}

pub enum AlphaMode {
    Opaque,
    Mask { cutoff: f32 },
    Blend,
}
```

## 2. BRDF（GGX）

```glsl
// GGX 法线分布函数
float D_GGX(float NoH, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float NoH2 = NoH * NoH;
    float d = NoH2 * (a2 - 1.0) + 1.0;
    return a2 / (PI * d * d);
}

// Smith 几何遮蔽
float G_Smith(float NoV, float NoL, float roughness) {
    float a = roughness * roughness;
    float k = (a + 1.0) * (a + 1.0) / 8.0;
    float g1_v = NoV / (NoV * (1.0 - k) + k);
    float g1_l = NoL / (NoL * (1.0 - k) + k);
    return g1_v * g1_l;
}

// Schlick Fresnel
vec3 F_Schlick(float u, vec3 f0) {
    return f0 + (1.0 - f0) * pow(1.0 - u, 5.0);
}
```

## 3. IBL

```rust
pub struct IblProbe {
    pub irradiance: CubeTexture,    // 32×32×6 辐照度
    pub prefiltered: CubeTexture,  // 256×256×6 mipmap 链
    pub sh_coefficients: [Vec3; 9], // 9 阶 L2 球谐
}

impl IblProbe {
    pub fn from_hdr(texture: HdrTexture) -> Self;
    pub fn prefilter(&mut self, roughness_levels: u32);
    pub fn compute_sh(&mut self, samples: u32);
}
```

## 4. 资源管理

```rust
pub struct MaterialManager3D {
    materials: HashMap<MaterialHandle, PbrMaterial>,
    textures: HashMap<TextureHandle, CubeTexture>,
    next_handle: u32,
}

impl MaterialManager3D {
    pub fn load_pbr(&mut self, path: &str) -> MaterialHandle;
    pub fn create_default() -> MaterialHandle;
    pub fn create_error() -> MaterialHandle;  // 紫色错误材质
}
```

## 5. 验收

- [ ] Disney BRDF 视觉对比：差异 < 5% (sRGB)
- [ ] IBL 烘焙：HDR EXR → 立方贴图 < 100 ms
- [ ] 实时 uniform 更新 < 16 µs
- [ ] 示例：5 个金属/粗糙度球体展示

## 6. 性能

| 操作 | 目标 |
|------|------|
| PBR shader 编译 | < 50 ms |
| 材质切换 | < 16 µs (1 uniform buffer) |
| IBL prefilter (256² 6 mip) | < 100 ms |
