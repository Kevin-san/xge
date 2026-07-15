//! Shadow Mapping - Shadow map rendering and cascaded shadow maps (CSM)
//!
//! Implements shadow map generation, sampling, and cascaded shadow maps
//! for large outdoor scene shadow rendering.

use engine_math::{Mat4, Vec3, Vec4};

/// Number of cascades for cascaded shadow maps
pub const MAX_CASCADES: usize = 4;

/// Create a look-at view matrix (right-handed)
///
/// # Arguments
/// * `eye` - Camera/eye position
/// * `target` - Look-at target position
/// * `up` - Up vector
pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let f = (target - eye).normalize_or_zero(); // forward
    let s = f.cross(up).normalize_or_zero(); // right
    let u = s.cross(f); // up (recomputed)

    Mat4 {
        cols: [
            [s.x, u.x, -f.x, 0.0],
            [s.y, u.y, -f.y, 0.0],
            [s.z, u.z, -f.z, 0.0],
            [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0],
        ],
    }
}

/// Create an orthographic projection matrix
///
/// # Arguments
/// * `left` - Left plane
/// * `right` - Right plane
/// * `bottom` - Bottom plane
/// * `top` - Top plane
/// * `near` - Near plane
/// * `far` - Far plane
pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let rpl = right - left;
    let tpb = top - bottom;
    let fpn = far - near;
    // Guard against zero division
    let rpl = if rpl.abs() < 1e-10 { 1.0 } else { rpl };
    let tpb = if tpb.abs() < 1e-10 { 1.0 } else { tpb };
    let fpn = if fpn.abs() < 1e-10 { 1.0 } else { fpn };

    Mat4 {
        cols: [
            [2.0 / rpl, 0.0, 0.0, 0.0],
            [0.0, 2.0 / tpb, 0.0, 0.0],
            [0.0, 0.0, -2.0 / fpn, 0.0],
            [
                -(right + left) / rpl,
                -(top + bottom) / tpb,
                -(far + near) / fpn,
                1.0,
            ],
        ],
    }
}

/// Shadow map configuration
#[derive(Clone, Debug)]
pub struct ShadowMapConfig {
    /// Shadow map texture resolution (width = height)
    pub resolution: u32,
    /// Number of cascades (1 = single shadow map, up to MAX_CASCADES)
    pub cascade_count: u32,
    /// Shadow bias to prevent acne
    pub bias: f32,
    /// Normal bias to reduce peter-panning
    pub normal_bias: f32,
    /// Maximum shadow distance from camera
    pub max_distance: f32,
    /// Split ratio between cascades (0-1, e.g., 0.1 = 10% per cascade)
    pub cascade_split_lambda: f32,
    /// Whether to use PCF (percentage-closer filtering)
    pub pcf_enabled: bool,
    /// PCF kernel size (3 = 3x3, 5 = 5x5)
    pub pcf_kernel_size: u32,
}

impl Default for ShadowMapConfig {
    fn default() -> Self {
        Self {
            resolution: 2048,
            cascade_count: 4,
            bias: 0.005,
            normal_bias: 0.02,
            max_distance: 100.0,
            cascade_split_lambda: 0.75,
            pcf_enabled: true,
            pcf_kernel_size: 3,
        }
    }
}

impl ShadowMapConfig {
    /// Create a new shadow map config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create config for a single shadow map (no cascades)
    pub fn single_cascade(resolution: u32) -> Self {
        Self {
            resolution,
            cascade_count: 1,
            ..Self::default()
        }
    }

    /// Create config for cascaded shadow maps
    pub fn cascaded(resolution: u32, cascade_count: u32) -> Self {
        Self {
            resolution,
            cascade_count: cascade_count.min(MAX_CASCADES as u32),
            ..Self::default()
        }
    }
}

/// A single shadow cascade
#[derive(Clone, Debug, Default)]
pub struct ShadowCascade {
    /// View-projection matrix for this cascade
    pub view_proj: Mat4,
    /// Split distance (near plane of this cascade in view space)
    pub split_near: f32,
    /// Split distance (far plane of this cascade in view space)
    pub split_far: f32,
    /// Texel size in world space
    pub texel_size: f32,
    /// Cascade index
    pub index: u32,
}

impl ShadowCascade {
    /// Create a new shadow cascade
    pub fn new(index: u32, split_near: f32, split_far: f32) -> Self {
        Self {
            view_proj: Mat4::IDENTITY,
            split_near,
            split_far,
            texel_size: 0.0,
            index,
        }
    }

    /// Compute the shadow coordinate for a world position
    ///
    /// Returns homogeneous shadow coordinate [x, y, z, w]
    /// where x,y are in [0,1] UV space and z is the depth.
    pub fn compute_shadow_coord(&self, world_pos: Vec3) -> Vec4 {
        let coord = self
            .view_proj
            .mul_vec4(Vec4::new(world_pos.x, world_pos.y, world_pos.z, 1.0));
        // Convert from NDC [-1, 1] to UV [0, 1]
        Vec4::new(
            coord.x * 0.5 + 0.5,
            coord.y * 0.5 + 0.5,
            coord.z * 0.5 + 0.5,
            coord.w,
        )
    }
}

/// Shadow map renderer for generating and sampling shadow maps
#[derive(Clone, Debug)]
pub struct ShadowMapRenderer {
    /// Shadow map configuration
    pub config: ShadowMapConfig,
    /// Cascade data
    pub cascades: Vec<ShadowCascade>,
    /// Light view matrix
    pub light_view: Mat4,
    /// Light projection matrix
    pub light_proj: Mat4,
}

impl Default for ShadowMapRenderer {
    fn default() -> Self {
        Self::new(ShadowMapConfig::default())
    }
}

impl ShadowMapRenderer {
    /// Create a new shadow map renderer with the given config
    pub fn new(config: ShadowMapConfig) -> Self {
        let cascade_count = config.cascade_count as usize;
        let cascades = (0..cascade_count)
            .map(|i| ShadowCascade::new(i as u32, 0.0, 0.0))
            .collect();
        Self {
            config,
            cascades,
            light_view: Mat4::IDENTITY,
            light_proj: Mat4::IDENTITY,
        }
    }

    /// Calculate cascade split distances based on camera near/far planes
    ///
    /// Uses a logarithmic-linear blend controlled by `cascade_split_lambda`.
    ///
    /// # Arguments
    /// * `near` - Camera near plane
    /// * `far` - Camera far plane (or shadow max distance)
    pub fn calculate_cascade_splits(&self, near: f32, far: f32) -> Vec<f32> {
        let count = self.config.cascade_count as usize;
        let mut splits = Vec::with_capacity(count + 1);
        splits.push(near);

        let lambda = self.config.cascade_split_lambda.clamp(0.0, 1.0);
        // Guard against near <= 0: logarithmic split is undefined (division by zero).
        // In that case fall back to pure uniform splits regardless of lambda.
        let log_valid = near > 0.0 && far > near;

        for i in 1..=count {
            let p = i as f32 / count as f32;
            // Uniform split
            let uniform_split = near + (far - near) * p;
            // Logarithmic split (only valid when near > 0)
            let log_split = if log_valid {
                near * (far / near).powf(p)
            } else {
                uniform_split
            };
            // Blend
            let split = lambda * log_split + (1.0 - lambda) * uniform_split;
            splits.push(split);
        }

        splits
    }

    /// Update cascade split distances
    ///
    /// # Arguments
    /// * `near` - Camera near plane
    /// * `far` - Camera far plane (or shadow max distance)
    pub fn update_cascade_splits(&mut self, near: f32, far: f32) {
        let splits = self.calculate_cascade_splits(near, far);
        for (i, cascade) in self.cascades.iter_mut().enumerate() {
            if i + 1 < splits.len() {
                cascade.split_near = splits[i];
                cascade.split_far = splits[i + 1];
            }
        }
    }

    /// Set the light direction and position to compute light view matrix
    ///
    /// # Arguments
    /// * `light_dir` - Direction from light to scene (normalized)
    /// * `scene_center` - Center of the scene to focus shadows on
    /// * `scene_radius` - Radius of the scene bounds
    pub fn set_light_view(&mut self, light_dir: Vec3, scene_center: Vec3, scene_radius: f32) {
        let light_pos = scene_center - light_dir * scene_radius;
        self.light_view = look_at(light_pos, scene_center, Vec3::new(0.0, 1.0, 0.0));

        // Orthographic projection for directional light
        let r = scene_radius;
        self.light_proj = orthographic(-r, r, -r, r, -r * 2.0, r * 2.0);
    }

    /// Update a specific cascade's view-projection matrix
    ///
    /// # Arguments
    /// * `index` - Cascade index
    /// * `view_proj` - Combined view-projection matrix
    pub fn set_cascade_view_proj(&mut self, index: usize, view_proj: Mat4) {
        if index < self.cascades.len() {
            self.cascades[index].view_proj = view_proj;
        }
    }

    /// Compute the shadow factor for a fragment
    ///
    /// Returns 0.0 (fully shadowed) to 1.0 (fully lit)
    ///
    /// # Arguments
    /// * `world_pos` - Fragment world position
    /// * `depth` - Depth value from shadow map at this position
    /// * `cascade_index` - Which cascade to sample
    pub fn compute_shadow_factor(
        &self,
        world_pos: Vec3,
        shadow_map_depth: f32,
        cascade_index: usize,
    ) -> f32 {
        if cascade_index >= self.cascades.len() {
            return 1.0; // No shadow if cascade doesn't exist
        }

        let cascade = &self.cascades[cascade_index];
        let coord = cascade.compute_shadow_coord(world_pos);

        // Check if coordinate is within shadow map bounds
        if coord.x < 0.0 || coord.x > 1.0 || coord.y < 0.0 || coord.y > 1.0 {
            return 1.0;
        }

        // Perspective divide
        let z = if coord.w > 0.0 {
            coord.z / coord.w
        } else {
            return 1.0;
        };

        // Apply bias
        let z_biased = z - self.config.bias;

        // Simple depth comparison
        if z_biased >= shadow_map_depth {
            0.0 // In shadow
        } else {
            1.0 // Lit
        }
    }

    /// Compute shadow factor with PCF (percentage-closer filtering)
    ///
    /// # Arguments
    /// * `world_pos` - Fragment world position
    /// * `depth_samples` - Array of depth samples from shadow map (kernel_size^2)
    /// * `cascade_index` - Which cascade to sample
    pub fn compute_shadow_factor_pcf(
        &self,
        world_pos: Vec3,
        depth_samples: &[f32],
        cascade_index: usize,
    ) -> f32 {
        if !self.config.pcf_enabled || depth_samples.is_empty() {
            // Fallback to single sample
            return self.compute_shadow_factor(
                world_pos,
                depth_samples.first().copied().unwrap_or(1.0),
                cascade_index,
            );
        }

        if cascade_index >= self.cascades.len() {
            return 1.0;
        }

        let cascade = &self.cascades[cascade_index];
        let coord = cascade.compute_shadow_coord(world_pos);

        if coord.x < 0.0 || coord.x > 1.0 || coord.y < 0.0 || coord.y > 1.0 {
            return 1.0;
        }

        let z = if coord.w > 0.0 {
            coord.z / coord.w
        } else {
            return 1.0;
        };

        let z_biased = z - self.config.bias;

        // Average over all samples
        let lit_count = depth_samples.iter().filter(|&&d| z_biased < d).count();

        lit_count as f32 / depth_samples.len() as f32
    }

    /// Select the appropriate cascade based on view-space depth
    ///
    /// # Arguments
    /// * `view_depth` - Fragment depth in view space
    pub fn select_cascade(&self, view_depth: f32) -> usize {
        for (i, cascade) in self.cascades.iter().enumerate() {
            if view_depth <= cascade.split_far {
                return i;
            }
        }
        // Beyond last cascade, no shadow
        self.cascades.len()
    }

    /// Get all cascades
    pub fn cascades(&self) -> &[ShadowCascade] {
        &self.cascades
    }

    /// Get the shadow map configuration
    pub fn config(&self) -> &ShadowMapConfig {
        &self.config
    }

    /// Get the light view-projection matrix (for single shadow map mode)
    pub fn light_view_proj(&self) -> Mat4 {
        self.light_proj * self.light_view
    }
}

/// Shadow map texture data (CPU-side representation)
#[derive(Clone, Debug)]
pub struct ShadowMapTexture {
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Depth values (row-major, width * height)
    pub depth_data: Vec<f32>,
}

impl ShadowMapTexture {
    /// Create a new shadow map texture
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            depth_data: vec![1.0; (width * height) as usize],
        }
    }

    /// Sample the shadow map at the given UV coordinates
    ///
    /// # Arguments
    /// * `u` - U coordinate [0, 1]
    /// * `v` - V coordinate [0, 1]
    pub fn sample(&self, u: f32, v: f32) -> f32 {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let x = (u * (self.width - 1) as f32).round() as usize;
        let y = (v * (self.height - 1) as f32).round() as usize;
        let index = y * self.width as usize + x;
        self.depth_data.get(index).copied().unwrap_or(1.0)
    }

    /// Sample with bilinear filtering
    pub fn sample_bilinear(&self, u: f32, v: f32) -> f32 {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let x = u * (self.width - 1) as f32;
        let y = v * (self.height - 1) as f32;
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = (x0 + 1).min(self.width as usize - 1);
        let y1 = (y0 + 1).min(self.height as usize - 1);
        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let s00 = self.get(x0, y0);
        let s10 = self.get(x1, y0);
        let s01 = self.get(x0, y1);
        let s11 = self.get(x1, y1);

        let s0 = s00 * (1.0 - fx) + s10 * fx;
        let s1 = s01 * (1.0 - fx) + s11 * fx;
        s0 * (1.0 - fy) + s1 * fy
    }

    /// Get a depth value at integer coordinates
    pub fn get(&self, x: usize, y: usize) -> f32 {
        let index = y * self.width as usize + x;
        self.depth_data.get(index).copied().unwrap_or(1.0)
    }

    /// Set a depth value at integer coordinates
    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        let index = y * self.width as usize + x;
        if index < self.depth_data.len() {
            self.depth_data[index] = value;
        }
    }

    /// Clear the shadow map to maximum depth (1.0)
    pub fn clear(&mut self) {
        self.depth_data.fill(1.0);
    }

    /// Clear the shadow map to a specific depth value
    pub fn clear_to(&mut self, value: f32) {
        self.depth_data.fill(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_map_config_default() {
        let config = ShadowMapConfig::default();
        assert_eq!(config.resolution, 2048);
        assert_eq!(config.cascade_count, 4);
        assert!(config.bias > 0.0);
        assert!(config.pcf_enabled);
    }

    #[test]
    fn test_shadow_map_config_new() {
        let config = ShadowMapConfig::new();
        assert_eq!(config.cascade_count, 4);
    }

    #[test]
    fn test_shadow_map_config_single_cascade() {
        let config = ShadowMapConfig::single_cascade(1024);
        assert_eq!(config.cascade_count, 1);
        assert_eq!(config.resolution, 1024);
    }

    #[test]
    fn test_shadow_map_config_cascaded() {
        let config = ShadowMapConfig::cascaded(2048, 3);
        assert_eq!(config.cascade_count, 3);
        assert_eq!(config.resolution, 2048);
    }

    #[test]
    fn test_shadow_map_config_cascaded_clamped() {
        let config = ShadowMapConfig::cascaded(2048, 10);
        assert_eq!(config.cascade_count, MAX_CASCADES as u32);
    }

    #[test]
    fn test_shadow_cascade_new() {
        let cascade = ShadowCascade::new(0, 0.1, 10.0);
        assert_eq!(cascade.index, 0);
        assert_eq!(cascade.split_near, 0.1);
        assert_eq!(cascade.split_far, 10.0);
    }

    #[test]
    fn test_shadow_cascade_compute_coord() {
        let cascade = ShadowCascade::new(0, 0.1, 10.0);
        let coord = cascade.compute_shadow_coord(Vec3::new(0.0, 0.0, 0.0));
        // With identity matrix, the result should be the position transformed
        assert!(coord.w >= 0.0);
    }

    #[test]
    fn test_shadow_map_renderer_default() {
        let renderer = ShadowMapRenderer::default();
        assert_eq!(renderer.cascades.len(), 4);
    }

    #[test]
    fn test_shadow_map_renderer_new() {
        let config = ShadowMapConfig::cascaded(1024, 2);
        let renderer = ShadowMapRenderer::new(config);
        assert_eq!(renderer.cascades.len(), 2);
    }

    #[test]
    fn test_calculate_cascade_splits() {
        let renderer = ShadowMapRenderer::default();
        let splits = renderer.calculate_cascade_splits(0.1, 100.0);
        assert_eq!(splits.len(), 5); // 4 cascades = 5 split points
        assert_eq!(splits[0], 0.1);
        assert_eq!(splits[4], 100.0);
    }

    #[test]
    fn test_calculate_cascade_splits_monotonic() {
        let renderer = ShadowMapRenderer::default();
        let splits = renderer.calculate_cascade_splits(0.1, 100.0);
        for i in 1..splits.len() {
            assert!(splits[i] >= splits[i - 1]);
        }
    }

    #[test]
    fn test_update_cascade_splits() {
        let mut renderer = ShadowMapRenderer::default();
        renderer.update_cascade_splits(0.1, 100.0);
        assert_eq!(renderer.cascades[0].split_near, 0.1);
        assert_eq!(renderer.cascades[3].split_far, 100.0);
    }

    #[test]
    fn test_set_cascade_view_proj() {
        let mut renderer = ShadowMapRenderer::default();
        let vp = Mat4::IDENTITY;
        renderer.set_cascade_view_proj(0, vp);
        // Should not panic
    }

    #[test]
    fn test_set_cascade_view_proj_out_of_bounds() {
        let mut renderer = ShadowMapRenderer::default();
        renderer.set_cascade_view_proj(100, Mat4::IDENTITY);
        // Should not panic, just ignore
    }

    #[test]
    fn test_compute_shadow_factor_no_cascade() {
        let renderer = ShadowMapRenderer::default();
        let factor = renderer.compute_shadow_factor(Vec3::new(0.0, 0.0, 0.0), 0.5, 100);
        assert_eq!(factor, 1.0); // No shadow when cascade doesn't exist
    }

    #[test]
    fn test_compute_shadow_factor_lit() {
        let renderer = ShadowMapRenderer::default();
        // With identity VP, world origin maps to center
        // depth = 0.5, biased = 0.5 - bias
        // If shadow_map_depth > biased, then lit
        let factor = renderer.compute_shadow_factor(Vec3::new(0.0, 0.0, 0.0), 1.0, 0);
        assert!((0.0..=1.0).contains(&factor));
    }

    #[test]
    fn test_compute_shadow_factor_shadowed() {
        let renderer = ShadowMapRenderer::default();
        // If shadow_map_depth is very small (0.0), fragment is in shadow
        let factor = renderer.compute_shadow_factor(Vec3::new(0.0, 0.0, 0.0), 0.0, 0);
        // With bias, z_biased = 0.5 - 0.005 = 0.495, which is > 0.0, so shadowed
        assert_eq!(factor, 0.0);
    }

    #[test]
    fn test_compute_shadow_factor_pcf_empty() {
        let renderer = ShadowMapRenderer::default();
        let factor = renderer.compute_shadow_factor_pcf(Vec3::new(0.0, 0.0, 0.0), &[], 0);
        assert_eq!(factor, 1.0);
    }

    #[test]
    fn test_compute_shadow_factor_pcf_multiple_samples() {
        let renderer = ShadowMapRenderer::default();
        let samples = vec![1.0, 1.0, 0.0, 1.0]; // 3 lit, 1 shadowed
        let factor = renderer.compute_shadow_factor_pcf(Vec3::new(0.0, 0.0, 0.0), &samples, 0);
        // Should be average of lit samples
        assert!(factor > 0.0 && factor <= 1.0);
    }

    #[test]
    fn test_select_cascade() {
        let mut renderer = ShadowMapRenderer::default();
        renderer.update_cascade_splits(0.1, 100.0);
        let idx = renderer.select_cascade(0.5);
        assert_eq!(idx, 0); // Closest depth uses first cascade
    }

    #[test]
    fn test_select_cascade_beyond_max() {
        let mut renderer = ShadowMapRenderer::default();
        renderer.update_cascade_splits(0.1, 100.0);
        let idx = renderer.select_cascade(200.0);
        assert_eq!(idx, renderer.cascades.len()); // Beyond all cascades
    }

    #[test]
    fn test_shadow_map_texture_new() {
        let tex = ShadowMapTexture::new(64, 64);
        assert_eq!(tex.width, 64);
        assert_eq!(tex.height, 64);
        assert_eq!(tex.depth_data.len(), 64 * 64);
    }

    #[test]
    fn test_shadow_map_texture_sample() {
        let mut tex = ShadowMapTexture::new(64, 64);
        tex.set(0, 0, 0.5);
        let value = tex.sample(0.0, 0.0);
        assert_eq!(value, 0.5);
    }

    #[test]
    fn test_shadow_map_texture_sample_clamped() {
        let tex = ShadowMapTexture::new(64, 64);
        let v1 = tex.sample(-1.0, -1.0);
        let v2 = tex.sample(2.0, 2.0);
        // Should not panic, returns clamped values
        assert!((0.0..=1.0).contains(&v1));
        assert!((0.0..=1.0).contains(&v2));
    }

    #[test]
    fn test_shadow_map_texture_set_get() {
        let mut tex = ShadowMapTexture::new(64, 64);
        tex.set(10, 20, 0.7);
        assert_eq!(tex.get(10, 20), 0.7);
    }

    #[test]
    fn test_shadow_map_texture_clear() {
        let mut tex = ShadowMapTexture::new(64, 64);
        tex.set(0, 0, 0.5);
        tex.clear();
        assert_eq!(tex.get(0, 0), 1.0);
    }

    #[test]
    fn test_shadow_map_texture_clear_to() {
        let mut tex = ShadowMapTexture::new(64, 64);
        tex.clear_to(0.3);
        assert_eq!(tex.get(0, 0), 0.3);
        assert_eq!(tex.get(63, 63), 0.3);
    }

    #[test]
    fn test_shadow_map_texture_bilinear() {
        let mut tex = ShadowMapTexture::new(2, 2);
        tex.set(0, 0, 0.0);
        tex.set(1, 0, 1.0);
        tex.set(0, 1, 0.0);
        tex.set(1, 1, 1.0);
        // Sample at center should be average
        let center = tex.sample_bilinear(0.5, 0.5);
        assert!((center - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_shadow_map_texture_bilinear_corners() {
        let mut tex = ShadowMapTexture::new(2, 2);
        tex.set(0, 0, 0.0);
        tex.set(1, 0, 1.0);
        let result = tex.sample_bilinear(0.0, 0.0);
        assert!((result - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_set_light_view() {
        let mut renderer = ShadowMapRenderer::default();
        renderer.set_light_view(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 50.0);
        // Just verify it doesn't panic
        assert_eq!(renderer.cascades.len(), 4);
    }

    #[test]
    fn test_light_view_proj() {
        let renderer = ShadowMapRenderer::default();
        let vp = renderer.light_view_proj();
        let _ = format!("{:?}", vp);
    }

    #[test]
    fn test_cascades_getter() {
        let renderer = ShadowMapRenderer::default();
        assert_eq!(renderer.cascades().len(), 4);
    }

    #[test]
    fn test_config_getter() {
        let renderer = ShadowMapRenderer::default();
        assert_eq!(renderer.config().cascade_count, 4);
    }

    #[test]
    fn test_shadow_map_config_clone() {
        let config1 = ShadowMapConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.resolution, config2.resolution);
        assert_eq!(config1.cascade_count, config2.cascade_count);
    }

    #[test]
    fn test_shadow_map_renderer_clone() {
        let renderer1 = ShadowMapRenderer::default();
        let renderer2 = renderer1.clone();
        assert_eq!(renderer1.cascades.len(), renderer2.cascades.len());
    }

    #[test]
    fn test_shadow_cascade_default() {
        let cascade = ShadowCascade::default();
        assert_eq!(cascade.index, 0);
        assert_eq!(cascade.split_near, 0.0);
    }

    #[test]
    fn test_calculate_cascade_splits_single() {
        let config = ShadowMapConfig::single_cascade(1024);
        let renderer = ShadowMapRenderer::new(config);
        let splits = renderer.calculate_cascade_splits(0.1, 100.0);
        assert_eq!(splits.len(), 2); // 1 cascade = 2 split points
    }

    #[test]
    fn test_calculate_cascade_splits_lambda_zero() {
        let config = ShadowMapConfig {
            cascade_split_lambda: 0.0, // Pure uniform
            ..Default::default()
        };
        let renderer = ShadowMapRenderer::new(config);
        let splits = renderer.calculate_cascade_splits(0.0, 100.0);
        // With lambda=0, splits should be uniform
        for i in 1..splits.len() {
            let expected = 100.0 * (i as f32 / (splits.len() - 1) as f32);
            assert!((splits[i] - expected).abs() < 0.1);
        }
    }

    #[test]
    fn test_shadow_map_texture_out_of_bounds_get() {
        let tex = ShadowMapTexture::new(2, 2);
        // Out of bounds should return default (1.0)
        let v = tex.get(100, 100);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_shadow_map_texture_out_of_bounds_set() {
        let mut tex = ShadowMapTexture::new(2, 2);
        // Should not panic
        tex.set(100, 100, 0.5);
    }
}
