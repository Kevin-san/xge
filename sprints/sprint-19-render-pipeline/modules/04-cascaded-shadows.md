# Module 04 — 级联阴影映射 CSM

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)

## 1. 目标

**Cascaded Shadow Maps**：
- 4 级联分割（默认 0.1 / 1 / 10 / 50 米）
- 视锥拟合 + 稳定化（避免阴影跳变）
- 阴影贴图 Atlas 打包
- PCF 软阴影 / VSM 方差阴影

## 2. Cascade Splitter

```rust
pub enum CascadeSplitScheme {
    Linear,
    Logarithmic,
    PSSM { lambda: f32 },  // Practical Split
    Manual(Vec<f32>),       // 用户指定
}

pub struct CascadedShadowMap {
    pub cascades: Vec<Cascade>,
    pub atlas_size: u32,    // 4096
    pub split_scheme: CascadeSplitScheme,
    pub light: DirectionalLight,
    pub stabilizer: Stabilizer,
}

pub struct Cascade {
    pub view_proj: Mat4,
    pub near: f32,
    pub far: f32,
    pub atlas_offset: Vec2,  // 在 atlas 中的位置
    pub atlas_size: u32,     // 单个 cascade 大小
}

impl CascadedShadowMap {
    pub fn compute_splits(&mut self, camera: &Camera3D);
    pub fn fit_to_cascade(&mut self, camera: &Camera3D, cascade_idx: usize);
    pub fn stabilize(&mut self, texel_size: f32);
}
```

## 3. 视锥拟合

```rust
pub fn fit_cascade_to_frustum(
    light_view: &Mat4,
    frustum_corners: &[Vec3; 8],
) -> Mat4 {
    // 1. 8 个角点投影到光源空间
    // 2. 计算 AABB
    // 3. 正交投影覆盖 AABB
    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);
    for &corner in frustum_corners {
        let p = light_view.transform_point(corner);
        min = min.min(p);
        max = max.max(p);
    }
    Mat4::orthographic_rh(min.x, max.x, min.y, max.y, min.z, max.z)
}
```

## 4. 稳定化

```rust
pub fn stabilize_orthographic(
    proj: &mut Mat4,
    texel_size: f32,
) {
    // 投影矩阵的平移按 texel_size 对齐
    let mut inv = proj.inverse().unwrap();
    let translation = inv.cols[3];
    let snapped = Vec3::new(
        (translation.x / texel_size).round() * texel_size,
        (translation.y / texel_size).round() * texel_size,
        translation.z,
    );
    inv.cols[3] = snapped;
    *proj = inv.inverse().unwrap();
}
```

## 5. 阴影 Shader

```glsl
// PBR fragment shader
float sample_shadow(vec3 world_pos, vec3 normal, vec3 light_dir) {
    float bias = max(0.005 * (1.0 - dot(normal, light_dir)), 0.0005);
    
    // 1. 选择 cascade
    float view_z = -(view * vec4(world_pos, 1.0)).z;
    uint cascade_idx = select_cascade(view_z);
    
    // 2. 投影到 cascade 空间
    vec4 shadow_pos = cascades[cascade_idx].view_proj * vec4(world_pos, 1.0);
    shadow_pos.xyz /= shadow_pos.w;
    vec3 uv = shadow_pos.xyz * 0.5 + 0.5;
    
    // 3. PCF 软阴影
    float shadow = 0.0;
    vec2 texel_size = 1.0 / vec2(textureSize(shadow_map, 0));
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            vec2 offset = vec2(x, y) * texel_size;
            shadow += texture(shadow_map, uv.xy + offset).r;
        }
    }
    shadow /= 9.0;
    
    return shadow > uv.z - bias ? 1.0 : 0.0;
}
```

## 6. 验收

- [ ] 4 级联 @ 4096² 总贴图 < 2 ms GPU
- [ ] 自阴影瑕疵 < 0.1% 像素
- [ ] 远距离阴影过渡平滑
- [ ] 阴影跳变 < 1 texel
- [ ] 示例：4 级联分屏对比
