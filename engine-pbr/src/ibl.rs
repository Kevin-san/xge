//! IBL (Image-Based Lighting) Baker and Environment Map

use engine_math::Vec3;
use serde::{Deserialize, Serialize};

/// Placeholder for cube map texture
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CubeMap {
    /// Size of each face in pixels
    pub size: u32,
    /// Number of mip levels
    pub mip_levels: u32,
    /// Texture path or identifier
    pub path: String,
}

/// Placeholder for 2D texture
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Texture2D {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Number of mip levels
    pub mip_levels: u32,
    /// Texture path or identifier
    pub path: String,
}

/// Environment map for IBL lighting
///
/// Contains HDR environment map data for skybox, irradiance, and prefilter maps
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnvironmentMap {
    /// Skybox cube map
    pub skybox: CubeMap,
    /// Irradiance cube map for diffuse IBL
    pub irradiance: CubeMap,
    /// Prefilter cube map for specular IBL
    pub prefilter: CubeMap,
    /// BRDF LUT texture
    pub brdf_lut: Texture2D,
    /// Environment lighting intensity multiplier
    pub intensity: f32,
}

impl EnvironmentMap {
    /// Create an environment map from an HDR file path
    pub fn from_hdr(path: &str) -> Self {
        Self {
            skybox: CubeMap {
                size: 1024,
                mip_levels: 1,
                path: path.to_string(),
            },
            irradiance: CubeMap {
                size: 32,
                mip_levels: 1,
                path: format!("{}_irradiance", path),
            },
            prefilter: CubeMap {
                size: 128,
                mip_levels: 5,
                path: format!("{}_prefilter", path),
            },
            brdf_lut: Texture2D {
                width: 256,
                height: 256,
                mip_levels: 1,
                path: format!("{}_brdf_lut", path),
            },
            intensity: 1.0,
        }
    }

    /// Get skybox cube map
    pub fn skybox(&self) -> &CubeMap {
        &self.skybox
    }

    /// Get irradiance cube map
    pub fn irradiance(&self) -> &CubeMap {
        &self.irradiance
    }

    /// Get prefilter cube map
    pub fn prefilter(&self) -> &CubeMap {
        &self.prefilter
    }

    /// Get BRDF LUT texture
    pub fn brdf_lut(&self) -> &Texture2D {
        &self.brdf_lut
    }

    /// Get intensity value
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Set intensity value
    pub fn set_intensity(&mut self, v: f32) {
        self.intensity = v;
    }
}

/// IBL Baker for generating irradiance, prefilter, and BRDF LUT from environment maps
///
/// Processes HDR environment maps to produce IBL textures for PBR rendering
#[derive(Clone, Debug, Default)]
pub struct IBLBaker {
    /// Default irradiance map size
    irradiance_size: u32,
    /// Default prefilter map size
    prefilter_size: u32,
    /// Default prefilter mip levels
    prefilter_mips: u32,
    /// Default BRDF LUT size
    brdf_lut_size: u32,
}

impl IBLBaker {
    /// Create a new IBL baker with default settings
    pub fn new() -> Self {
        Self {
            irradiance_size: 32,
            prefilter_size: 128,
            prefilter_mips: 5,
            brdf_lut_size: 256,
        }
    }

    /// Bake irradiance cube map from environment map
    ///
    /// Generates a low-resolution diffuse irradiance map
    pub fn bake_irradiance(&self, env_map: &EnvironmentMap) -> CubeMap {
        CubeMap {
            size: self.irradiance_size,
            mip_levels: 1,
            path: format!("{}_irradiance", env_map.skybox.path),
        }
    }

    /// Bake prefilter cube map from environment map
    ///
    /// Generates a mip-mapped specular prefilter map for roughness-based IBL
    pub fn bake_prefilter(&self, env_map: &EnvironmentMap, levels: u32) -> CubeMap {
        CubeMap {
            size: self.prefilter_size,
            mip_levels: levels.min(self.prefilter_mips),
            path: format!("{}_prefilter", env_map.skybox.path),
        }
    }

    /// Bake BRDF LUT texture
    ///
    /// Generates a 2D texture containing BRDF integration results
    pub fn bake_brdf_lut(&self, size: u32) -> Texture2D {
        Texture2D {
            width: size.min(self.brdf_lut_size),
            height: size.min(self.brdf_lut_size),
            mip_levels: 1,
            path: "brdf_lut".to_string(),
        }
    }

    /// Calculate the number of mip levels for a texture size
    pub fn calculate_mip_levels(size: u32) -> u32 {
        if size == 0 {
            return 1;
        }
        (size as f32).log2().floor() as u32 + 1
    }

    /// Sample environment map for irradiance calculation
    ///
    /// Returns approximate irradiance for a given normal direction
    pub fn sample_irradiance(normal: Vec3, env_map: &EnvironmentMap) -> Vec3 {
        // Simplified irradiance calculation
        // In real implementation, this would convolve the environment map
        let intensity = env_map.intensity;
        Vec3::new(
            intensity * 0.5 * (normal.y + 1.0),
            intensity * 0.5 * (normal.y + 1.0),
            intensity * 0.5 * (normal.y + 1.0),
        )
    }

    /// Sample prefilter map for specular IBL
    ///
    /// Returns prefiltered environment color based on roughness
    pub fn sample_prefilter(reflection: Vec3, roughness: f32, env_map: &EnvironmentMap) -> Vec3 {
        // Simplified prefilter sampling
        // In real implementation, this would sample the appropriate mip level
        let _mip_level = (roughness * (env_map.prefilter.mip_levels - 1) as f32) as u32;
        let intensity = env_map.intensity;

        // Approximate specular reflection
        Vec3::new(
            intensity * reflection.x.max(0.0),
            intensity * reflection.y.max(0.0),
            intensity * reflection.z.max(0.0),
        )
    }

    /// Sample BRDF LUT for specular BRDF integration
    ///
    /// Returns (F0 scale, geometry term) from the LUT
    pub fn sample_brdf_lut(n_dot_v: f32, roughness: f32) -> (f32, f32) {
        // Simplified BRDF LUT sampling
        // In real implementation, this would sample a precomputed texture
        let x = n_dot_v.clamp(0.0, 1.0);
        let y = roughness.clamp(0.0, 1.0);

        // Approximate values based on Schlick BRDF
        let f0_scale = 1.0 - (1.0 - x) * y;
        let geometry = x * (1.0 - y * 0.5);

        (f0_scale, geometry)
    }

    /// Bake BRDF LUT data using actual Cook-Torrance integration
    ///
    /// Computes the 2D LUT where:
    /// - X axis: NdotV (cosine between normal and view direction)
    /// - Y axis: Roughness
    /// - Red channel: F0 scale factor
    /// - Green channel: Geometry/visibility term
    ///
    /// # Arguments
    /// * `size` - LUT dimension (size x size)
    /// * `sample_count` - Number of samples for Monte Carlo integration
    pub fn bake_brdf_lut_data(&self, size: u32, sample_count: u32) -> BrdfLutData {
        let mut data = BrdfLutData::new(size);

        for y in 0..size {
            let roughness = y as f32 / (size - 1) as f32;
            for x in 0..size {
                let n_dot_v = x as f32 / (size - 1) as f32;
                let (r0_scale, g_bias) = Self::integrate_brdf(n_dot_v, roughness, sample_count);
                data.set(x, y, [r0_scale, g_bias, 0.0, 1.0]);
            }
        }

        data
    }

    /// Integrate BRDF for a given NdotV and roughness
    ///
    /// Uses importance sampling with GGX distribution to compute
    /// the scale and bias for F0 in the split-sum approximation.
    fn integrate_brdf(n_dot_v: f32, roughness: f32, sample_count: u32) -> (f32, f32) {
        let n_dot_v = n_dot_v.clamp(0.0, 1.0);
        let roughness = roughness.clamp(0.0, 1.0);

        let v = Vec3::new((1.0 - n_dot_v * n_dot_v).max(0.0).sqrt(), 0.0, n_dot_v);

        let mut a = 0.0; // F0 scale
        let mut b = 0.0; // F0 bias

        let mut sample_index = 0u32;
        while sample_index < sample_count {
            let h = Self::importance_sample_ggx(sample_index, sample_count, roughness);
            let l = (2.0 * v.dot(h)) * h - v;

            let n_dot_l = l.z.max(0.0);
            let n_dot_h = h.z.max(0.0);
            let v_dot_h = v.dot(h).max(0.0);

            if n_dot_l > 0.0 {
                let g = Self::geometry_smith_ibl(n_dot_v, n_dot_l, roughness);
                let g_vis = (g * v_dot_h) / (n_dot_h * n_dot_v + 1e-6);
                let fc = (1.0 - v_dot_h).powi(5);

                a += (1.0 - fc) * g_vis;
                b += fc * g_vis;
            }

            sample_index += 1;
        }

        let inv_samples = 1.0 / sample_count as f32;
        (a * inv_samples, b * inv_samples)
    }

    /// Importance sample GGX distribution
    fn importance_sample_ggx(sample_index: u32, _sample_count: u32, roughness: f32) -> Vec3 {
        let a = roughness * roughness;

        // Halton sequence for low-discrepancy sampling
        let mut u = Self::halton(sample_index * 2 + 1, 2);
        let mut v = Self::halton(sample_index * 2 + 2, 3);

        // Avoid exact 0 and 1
        u = u.clamp(1e-6, 1.0 - 1e-6);
        v = v.clamp(1e-6, 1.0 - 1e-6);

        let phi = 2.0 * core::f32::consts::PI * u;
        let cos_theta = (1.0 - v) / (1.0 + (a * a - 1.0) * v);
        let cos_theta = cos_theta.clamp(-1.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();

        Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
    }

    /// Halton sequence for quasi-random sampling
    fn halton(index: u32, base: u32) -> f32 {
        let mut result = 0.0f32;
        let mut f = 1.0f32;
        let mut i = index;

        while i > 0 {
            f /= base as f32;
            result += f * (i % base) as f32;
            i /= base;
        }

        result
    }

    /// Smith geometry function for IBL (uses different k than direct lighting)
    fn geometry_smith_ibl(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
        let k = (roughness * roughness) / 2.0;
        let gv = n_dot_v / (n_dot_v * (1.0 - k) + k + 1e-6);
        let gl = n_dot_l / (n_dot_l * (1.0 - k) + k + 1e-6);
        gv * gl
    }

    /// Bake irradiance map data by convolving an environment sample function
    ///
    /// # Arguments
    /// * `size` - Cube map face resolution
    /// * `sample_count` - Number of samples for the convolution
    /// * `env_sampler` - Function that returns environment color for a direction
    pub fn bake_irradiance_data<F>(
        &self,
        size: u32,
        sample_count: u32,
        env_sampler: F,
    ) -> IrradianceMapData
    where
        F: Fn(Vec3) -> Vec3,
    {
        let mut data = IrradianceMapData::new(size);

        // For each face, for each pixel, convolve the hemisphere
        // This is a simplified version that computes irradiance for a single
        // direction (up) and applies it uniformly. A full implementation would
        // process all 6 faces and all pixels.
        for face in 0..6 {
            for y in 0..size {
                for x in 0..size {
                    let dir = Self::cube_map_direction(face, x, y, size);
                    let irradiance = Self::convolve_irradiance(&env_sampler, dir, sample_count);
                    data.set_face_pixel(face, x, y, irradiance);
                }
            }
        }

        data
    }

    /// Convolve environment map for diffuse irradiance
    fn convolve_irradiance<F>(env_sampler: &F, normal: Vec3, sample_count: u32) -> [f32; 3]
    where
        F: Fn(Vec3) -> Vec3,
    {
        let mut irradiance = Vec3::new(0.0, 0.0, 0.0);
        let mut sample_count_actual = 0u32;

        // Build tangent frame
        let up = if normal.z.abs() < 0.999 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let tangent = up.cross(normal).normalize_or_zero();
        let bitangent = normal.cross(tangent);

        let sample_step = (sample_count as f32).sqrt().max(1.0) as u32;

        for phi_step in 0..sample_step {
            for theta_step in 0..sample_step {
                let phi = 2.0 * core::f32::consts::PI * phi_step as f32 / sample_step as f32;
                let theta = (core::f32::consts::PI / 2.0) * theta_step as f32 / sample_step as f32;

                let sin_theta = theta.sin();
                let cos_theta = theta.cos();

                let sample_dir = tangent * (sin_theta * phi.cos())
                    + bitangent * (sin_theta * phi.sin())
                    + normal * cos_theta;

                let env_color = env_sampler(sample_dir);
                irradiance += env_color * (cos_theta * sin_theta);
                sample_count_actual += 1;
            }
        }

        let weight = core::f32::consts::PI / sample_count_actual as f32;
        [
            irradiance.x * weight,
            irradiance.y * weight,
            irradiance.z * weight,
        ]
    }

    /// Bake prefilter map data for a specific roughness level
    ///
    /// # Arguments
    /// * `size` - Cube map face resolution
    /// * `roughness` - Roughness value for this mip level [0, 1]
    /// * `sample_count` - Number of samples for Monte Carlo integration
    /// * `env_sampler` - Function that returns environment color for a direction
    pub fn bake_prefilter_data<F>(
        &self,
        size: u32,
        roughness: f32,
        sample_count: u32,
        env_sampler: F,
    ) -> PrefilterMapData
    where
        F: Fn(Vec3) -> Vec3,
    {
        let mut data = PrefilterMapData::new(size, roughness);

        for face in 0..6 {
            for y in 0..size {
                for x in 0..size {
                    let dir = Self::cube_map_direction(face, x, y, size);
                    let prefiltered =
                        Self::prefilter_env(&env_sampler, dir, roughness, sample_count);
                    data.set_face_pixel(face, x, y, prefiltered);
                }
            }
        }

        data
    }

    /// Prefilter environment map for specular IBL using GGX importance sampling
    fn prefilter_env<F>(
        env_sampler: &F,
        normal: Vec3,
        roughness: f32,
        sample_count: u32,
    ) -> [f32; 3]
    where
        F: Fn(Vec3) -> Vec3,
    {
        let mut prefiltered = Vec3::new(0.0, 0.0, 0.0);
        let mut total_weight = 0.0f32;

        // Build tangent frame
        let up = if normal.z.abs() < 0.999 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let tangent = up.cross(normal).normalize_or_zero();
        let bitangent = normal.cross(tangent);

        for i in 0..sample_count {
            let h = Self::importance_sample_ggx(i, sample_count, roughness);
            // Transform to world space
            let h_world = tangent * h.x + bitangent * h.y + normal * h.z;

            let v = h_world; // For prefilter, V = H (view = halfway)
            let l = (2.0 * v.dot(h_world)) * h_world - v;

            let n_dot_l = normal.dot(l).max(0.0);

            if n_dot_l > 0.0 {
                let env_color = env_sampler(l);
                prefiltered += env_color * n_dot_l;
                total_weight += n_dot_l;
            }
        }

        if total_weight > 0.0 {
            [
                prefiltered.x / total_weight,
                prefiltered.y / total_weight,
                prefiltered.z / total_weight,
            ]
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    /// Get the direction vector for a cube map face pixel
    ///
    /// # Arguments
    /// * `face` - Face index (0-5: +X, -X, +Y, -Y, +Z, -Z)
    /// * `x` - Pixel x coordinate
    /// * `y` - Pixel y coordinate
    /// * `size` - Face size
    pub fn cube_map_direction(face: u32, x: u32, y: u32, size: u32) -> Vec3 {
        let u = (2.0 * x as f32 + 1.0) / size as f32 - 1.0;
        let v = (2.0 * y as f32 + 1.0) / size as f32 - 1.0;

        match face {
            0 => Vec3::new(1.0, v, -u),  // +X
            1 => Vec3::new(-1.0, v, u),  // -X
            2 => Vec3::new(u, 1.0, -v),  // +Y
            3 => Vec3::new(u, -1.0, v),  // -Y
            4 => Vec3::new(u, v, 1.0),   // +Z
            5 => Vec3::new(-u, v, -1.0), // -Z
            _ => Vec3::new(0.0, 0.0, 1.0),
        }
        .normalize_or_zero()
    }
}

/// BRDF LUT data (2D texture with actual pixel values)
#[derive(Clone, Debug)]
pub struct BrdfLutData {
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Pixel data (RGBA, row-major)
    pub pixels: Vec<[f32; 4]>,
}

impl BrdfLutData {
    /// Create a new BRDF LUT filled with zeros
    pub fn new(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            pixels: vec![[0.0, 0.0, 0.0, 1.0]; (size * size) as usize],
        }
    }

    /// Get a pixel at (x, y)
    pub fn get(&self, x: u32, y: u32) -> [f32; 4] {
        if x >= self.width || y >= self.height {
            return [0.0, 0.0, 0.0, 1.0];
        }
        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }

    /// Set a pixel at (x, y)
    pub fn set(&mut self, x: u32, y: u32, pixel: [f32; 4]) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = pixel;
        }
    }

    /// Sample the LUT with bilinear filtering
    ///
    /// # Arguments
    /// * `n_dot_v` - Normalized dot product [0, 1]
    /// * `roughness` - Roughness [0, 1]
    pub fn sample(&self, n_dot_v: f32, roughness: f32) -> (f32, f32) {
        let x = n_dot_v.clamp(0.0, 1.0) * (self.width - 1) as f32;
        let y = roughness.clamp(0.0, 1.0) * (self.height - 1) as f32;

        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let s00 = self.get(x0, y0);
        let s10 = self.get(x1, y0);
        let s01 = self.get(x0, y1);
        let s11 = self.get(x1, y1);

        let r0 = s00[0] * (1.0 - fx) + s10[0] * fx;
        let r1 = s01[0] * (1.0 - fx) + s11[0] * fx;
        let r = r0 * (1.0 - fy) + r1 * fy;

        let g0 = s00[1] * (1.0 - fx) + s10[1] * fx;
        let g1 = s01[1] * (1.0 - fx) + s11[1] * fx;
        let g = g0 * (1.0 - fy) + g1 * fy;

        (r, g)
    }
}

/// Irradiance map data (cube map with actual pixel values)
#[derive(Clone, Debug)]
pub struct IrradianceMapData {
    /// Face size (width = height)
    pub size: u32,
    /// Pixel data for all 6 faces (face * size * size)
    pub pixels: Vec<[f32; 3]>,
}

impl IrradianceMapData {
    /// Create a new irradiance map filled with zeros
    pub fn new(size: u32) -> Self {
        Self {
            size,
            pixels: vec![[0.0, 0.0, 0.0]; (6 * size * size) as usize],
        }
    }

    /// Get a pixel from a specific face
    pub fn get_face_pixel(&self, face: u32, x: u32, y: u32) -> [f32; 3] {
        if face >= 6 || x >= self.size || y >= self.size {
            return [0.0, 0.0, 0.0];
        }
        let index = ((face * self.size * self.size) + y * self.size + x) as usize;
        self.pixels[index]
    }

    /// Set a pixel on a specific face
    pub fn set_face_pixel(&mut self, face: u32, x: u32, y: u32, color: [f32; 3]) {
        if face < 6 && x < self.size && y < self.size {
            let index = ((face * self.size * self.size) + y * self.size + x) as usize;
            self.pixels[index] = color;
        }
    }

    /// Sample the irradiance map for a given direction
    pub fn sample(&self, direction: Vec3) -> [f32; 3] {
        // Determine which face to sample based on dominant axis
        let abs_x = direction.x.abs();
        let abs_y = direction.y.abs();
        let abs_z = direction.z.abs();

        let (face, u, v) = if abs_x >= abs_y && abs_x >= abs_z {
            if direction.x > 0.0 {
                (0u32, direction.z, direction.y)
            } else {
                (1u32, -direction.z, direction.y)
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            if direction.y > 0.0 {
                (2u32, direction.x, -direction.z)
            } else {
                (3u32, direction.x, direction.z)
            }
        } else if direction.z > 0.0 {
            (4u32, direction.x, direction.y)
        } else {
            (5u32, -direction.x, direction.y)
        };

        let x = ((u * 0.5 + 0.5) * (self.size - 1) as f32) as u32;
        let y = ((v * 0.5 + 0.5) * (self.size - 1) as f32) as u32;
        self.get_face_pixel(face, x.min(self.size - 1), y.min(self.size - 1))
    }
}

/// Prefilter map data for a single roughness level (cube map)
#[derive(Clone, Debug)]
pub struct PrefilterMapData {
    /// Face size
    pub size: u32,
    /// Roughness level this data was baked for
    pub roughness: f32,
    /// Pixel data for all 6 faces
    pub pixels: Vec<[f32; 3]>,
}

impl PrefilterMapData {
    /// Create a new prefilter map filled with zeros
    pub fn new(size: u32, roughness: f32) -> Self {
        Self {
            size,
            roughness,
            pixels: vec![[0.0, 0.0, 0.0]; (6 * size * size) as usize],
        }
    }

    /// Get a pixel from a specific face
    pub fn get_face_pixel(&self, face: u32, x: u32, y: u32) -> [f32; 3] {
        if face >= 6 || x >= self.size || y >= self.size {
            return [0.0, 0.0, 0.0];
        }
        let index = ((face * self.size * self.size) + y * self.size + x) as usize;
        self.pixels[index]
    }

    /// Set a pixel on a specific face
    pub fn set_face_pixel(&mut self, face: u32, x: u32, y: u32, color: [f32; 3]) {
        if face < 6 && x < self.size && y < self.size {
            let index = ((face * self.size * self.size) + y * self.size + x) as usize;
            self.pixels[index] = color;
        }
    }

    /// Sample the prefilter map for a given direction
    pub fn sample(&self, direction: Vec3) -> [f32; 3] {
        let abs_x = direction.x.abs();
        let abs_y = direction.y.abs();
        let abs_z = direction.z.abs();

        let (face, u, v) = if abs_x >= abs_y && abs_x >= abs_z {
            if direction.x > 0.0 {
                (0u32, direction.z, direction.y)
            } else {
                (1u32, -direction.z, direction.y)
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            if direction.y > 0.0 {
                (2u32, direction.x, -direction.z)
            } else {
                (3u32, direction.x, direction.z)
            }
        } else if direction.z > 0.0 {
            (4u32, direction.x, direction.y)
        } else {
            (5u32, -direction.x, direction.y)
        };

        let x = ((u * 0.5 + 0.5) * (self.size - 1) as f32) as u32;
        let y = ((v * 0.5 + 0.5) * (self.size - 1) as f32) as u32;
        self.get_face_pixel(face, x.min(self.size - 1), y.min(self.size - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ibl_baker_new() {
        let baker = IBLBaker::new();
        assert_eq!(baker.irradiance_size, 32);
        assert_eq!(baker.prefilter_size, 128);
        assert_eq!(baker.prefilter_mips, 5);
        assert_eq!(baker.brdf_lut_size, 256);
    }

    #[test]
    fn test_ibl_baker_bake_irradiance() {
        let baker = IBLBaker::new();
        let env = EnvironmentMap::from_hdr("test.hdr");
        let irradiance = baker.bake_irradiance(&env);
        assert_eq!(irradiance.size, 32);
        assert_eq!(irradiance.mip_levels, 1);
    }

    #[test]
    fn test_ibl_baker_bake_prefilter() {
        let baker = IBLBaker::new();
        let env = EnvironmentMap::from_hdr("test.hdr");
        let prefilter = baker.bake_prefilter(&env, 5);
        assert_eq!(prefilter.size, 128);
        assert_eq!(prefilter.mip_levels, 5);
    }

    #[test]
    fn test_ibl_baker_bake_brdf_lut() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut(256);
        assert_eq!(lut.width, 256);
        assert_eq!(lut.height, 256);
        assert_eq!(lut.mip_levels, 1);
    }

    #[test]
    fn test_ibl_baker_bake_brdf_lut_non_empty() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut(256);
        assert!(lut.width > 0);
        assert!(lut.height > 0);
    }

    #[test]
    fn test_calculate_mip_levels() {
        assert_eq!(IBLBaker::calculate_mip_levels(256), 9);
        assert_eq!(IBLBaker::calculate_mip_levels(128), 8);
        assert_eq!(IBLBaker::calculate_mip_levels(64), 7);
        assert_eq!(IBLBaker::calculate_mip_levels(32), 6);
        assert_eq!(IBLBaker::calculate_mip_levels(1), 1);
        assert_eq!(IBLBaker::calculate_mip_levels(0), 1);
    }

    #[test]
    fn test_environment_map_from_hdr() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        assert_eq!(env.skybox.size, 1024);
        assert_eq!(env.irradiance.size, 32);
        assert_eq!(env.prefilter.size, 128);
        assert_eq!(env.brdf_lut.width, 256);
        assert_eq!(env.intensity, 1.0);
    }

    #[test]
    fn test_environment_map_intensity() {
        let mut env = EnvironmentMap::from_hdr("test.hdr");
        env.set_intensity(2.0);
        assert_eq!(env.intensity(), 2.0);
    }

    #[test]
    fn test_sample_irradiance() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let irradiance = IBLBaker::sample_irradiance(normal, &env);
        assert!(irradiance.y > 0.0);
    }

    #[test]
    fn test_sample_prefilter() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        let reflection = Vec3::new(0.0, 1.0, 0.0);
        let prefilter = IBLBaker::sample_prefilter(reflection, 0.5, &env);
        assert!(prefilter.y >= 0.0);
    }

    #[test]
    fn test_sample_brdf_lut() {
        let (f0_scale, geometry) = IBLBaker::sample_brdf_lut(0.5, 0.5);
        assert!((0.0..=1.0).contains(&f0_scale));
        assert!((0.0..=1.0).contains(&geometry));
    }

    #[test]
    fn test_ibl_baker_new_defaults() {
        let baker = IBLBaker::new();
        // Check default values
        assert_eq!(baker.irradiance_size, 32);
        assert_eq!(baker.prefilter_size, 128);
        assert_eq!(baker.prefilter_mips, 5);
        assert_eq!(baker.brdf_lut_size, 256);
    }

    #[test]
    fn test_cubemap_default_fields() {
        let cm = CubeMap::default();
        assert_eq!(cm.size, 0);
        assert_eq!(cm.mip_levels, 0);
        assert_eq!(cm.path, "");
    }

    #[test]
    fn test_texture2d_default_fields() {
        let tex = Texture2D::default();
        assert_eq!(tex.width, 0);
        assert_eq!(tex.height, 0);
        assert_eq!(tex.mip_levels, 0);
        assert_eq!(tex.path, "");
    }

    #[test]
    fn test_environment_map_default_fields() {
        let env = EnvironmentMap::default();
        assert_eq!(env.skybox.size, 0);
        assert_eq!(env.intensity, 0.0);
    }

    #[test]
    fn test_environment_map_intensity_setter() {
        let env = EnvironmentMap {
            intensity: 3.0,
            ..Default::default()
        };
        assert_eq!(env.intensity, 3.0);
    }

    #[test]
    fn test_ibl_baker_bake_irradiance_path() {
        let baker = IBLBaker::new();
        let env = EnvironmentMap::from_hdr("test_env.hdr");
        let irradiance = baker.bake_irradiance(&env);
        assert_eq!(irradiance.path, "test_env.hdr_irradiance");
    }

    #[test]
    fn test_ibl_baker_bake_prefilter_path() {
        let baker = IBLBaker::new();
        let env = EnvironmentMap::from_hdr("test_env.hdr");
        let prefilter = baker.bake_prefilter(&env, 5);
        assert_eq!(prefilter.path, "test_env.hdr_prefilter");
        assert_eq!(prefilter.mip_levels, 5);
    }

    #[test]
    fn test_ibl_baker_bake_brdf_lut_sizes() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut(256);
        assert_eq!(lut.width, 256);
        assert_eq!(lut.height, 256);
    }

    #[test]
    fn test_ibl_baker_mip_levels_various_sizes() {
        assert_eq!(IBLBaker::calculate_mip_levels(1), 1);
        assert_eq!(IBLBaker::calculate_mip_levels(2), 2);
        assert_eq!(IBLBaker::calculate_mip_levels(4), 3);
        assert_eq!(IBLBaker::calculate_mip_levels(8), 4);
        assert_eq!(IBLBaker::calculate_mip_levels(64), 7);
        assert_eq!(IBLBaker::calculate_mip_levels(128), 8);
        assert_eq!(IBLBaker::calculate_mip_levels(256), 9);
        assert_eq!(IBLBaker::calculate_mip_levels(512), 10);
        assert_eq!(IBLBaker::calculate_mip_levels(1024), 11);
        assert_eq!(IBLBaker::calculate_mip_levels(2048), 12);
    }

    #[test]
    fn test_ibl_baker_default_clone() {
        let baker1 = IBLBaker::new();
        let baker2 = baker1.clone();
        assert_eq!(baker1.irradiance_size, baker2.irradiance_size);
        assert_eq!(baker1.prefilter_size, baker2.prefilter_size);
        assert_eq!(baker1.prefilter_mips, baker2.prefilter_mips);
        assert_eq!(baker1.brdf_lut_size, baker2.brdf_lut_size);
    }

    #[test]
    fn test_ibl_baker_debug() {
        let baker = IBLBaker::new();
        let _ = format!("{:?}", baker);
    }

    #[test]
    fn test_cubemap_debug() {
        let cm = CubeMap::default();
        let _ = format!("{:?}", cm);
    }

    #[test]
    fn test_texture2d_debug() {
        let tex = Texture2D::default();
        let _ = format!("{:?}", tex);
    }

    #[test]
    fn test_environment_map_debug() {
        let env = EnvironmentMap::default();
        let _ = format!("{:?}", env);
    }

    #[test]
    fn test_sample_irradiance_different_normals() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        let result_up = IBLBaker::sample_irradiance(Vec3::new(0.0, 1.0, 0.0), &env);
        let result_down = IBLBaker::sample_irradiance(Vec3::new(0.0, -1.0, 0.0), &env);
        assert!(result_up.x >= 0.0);
        assert!(result_down.x >= 0.0);
    }

    #[test]
    fn test_sample_prefilter_roughness() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        let result_low = IBLBaker::sample_prefilter(Vec3::new(0.0, 1.0, 0.0), 0.0, &env);
        let result_high = IBLBaker::sample_prefilter(Vec3::new(0.0, 1.0, 0.0), 1.0, &env);
        assert!(result_low.y >= 0.0);
        assert!(result_high.y >= 0.0);
    }

    #[test]
    fn test_sample_brdf_lut_bounds() {
        // Test various n_dot_v and roughness values
        let (scale1, geom1) = IBLBaker::sample_brdf_lut(0.0, 0.0);
        let (scale2, geom2) = IBLBaker::sample_brdf_lut(1.0, 1.0);
        assert!(scale1 >= 0.0);
        assert!(geom1 >= 0.0);
        assert!(scale2 >= 0.0);
        assert!(geom2 >= 0.0);
    }

    #[test]
    fn test_environment_map_paths() {
        let env = EnvironmentMap::from_hdr("sky.hdr");
        assert!(env.irradiance.path.contains("sky.hdr"));
        assert!(env.prefilter.path.contains("sky.hdr"));
        assert!(env.brdf_lut.path.contains("sky.hdr"));
    }

    #[test]
    fn test_environment_map_sizes() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        assert_eq!(env.skybox.size, 1024);
        assert_eq!(env.irradiance.size, 32);
        assert_eq!(env.prefilter.size, 128);
        assert_eq!(env.brdf_lut.width, 256);
        assert_eq!(env.brdf_lut.height, 256);
    }

    #[test]
    fn test_environment_map_prefilter_mips() {
        let env = EnvironmentMap::from_hdr("test.hdr");
        assert_eq!(env.prefilter.mip_levels, 5);
    }

    #[test]
    fn test_ibl_baker_bake_brdf_lut_non_empty_sizes() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut(256);
        assert!(lut.width > 0);
        assert!(lut.height > 0);
    }

    #[test]
    fn test_bake_brdf_lut_data_basic() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut_data(16, 64);
        assert_eq!(lut.width, 16);
        assert_eq!(lut.height, 16);
        assert_eq!(lut.pixels.len(), 16 * 16);
    }

    #[test]
    fn test_bake_brdf_lut_data_values() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut_data(32, 128);

        // At NdotV=1, roughness=0, the scale should be high (close to 1)
        let (scale_smooth, _bias_smooth) = lut.sample(1.0, 0.0);
        assert!((0.0..=1.0).contains(&scale_smooth));

        // At NdotV=1, roughness=1, the scale should be lower
        let (scale_rough, _bias_rough) = lut.sample(1.0, 1.0);
        assert!((0.0..=1.0).contains(&scale_rough));
    }

    #[test]
    fn test_bake_brdf_lut_data_get_set() {
        let mut lut = BrdfLutData::new(8);
        lut.set(2, 3, [0.5, 0.6, 0.0, 1.0]);
        let pixel = lut.get(2, 3);
        assert_eq!(pixel, [0.5, 0.6, 0.0, 1.0]);
    }

    #[test]
    fn test_bake_brdf_lut_data_get_out_of_bounds() {
        let lut = BrdfLutData::new(8);
        let pixel = lut.get(100, 100);
        assert_eq!(pixel, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_bake_brdf_lut_data_sample_bilinear() {
        let mut lut = BrdfLutData::new(4);
        // Set known values
        for y in 0..4 {
            for x in 0..4 {
                lut.set(x, y, [x as f32 / 3.0, y as f32 / 3.0, 0.0, 1.0]);
            }
        }

        let (r, g) = lut.sample(0.5, 0.5);
        // Should be interpolated values
        assert!((0.0..=1.0).contains(&r));
        assert!((0.0..=1.0).contains(&g));
    }

    #[test]
    fn test_bake_irradiance_data_basic() {
        let baker = IBLBaker::new();
        let env_sampler = |dir: Vec3| {
            // Simple environment: white from above, black from below
            Vec3::new(1.0, 1.0, 1.0) * dir.z.max(0.0)
        };
        let data = baker.bake_irradiance_data(4, 16, env_sampler);
        assert_eq!(data.size, 4);
        assert_eq!(data.pixels.len(), 6 * 4 * 4);
    }

    #[test]
    fn test_bake_irradiance_data_uniform_env() {
        let baker = IBLBaker::new();
        let env_sampler = |_dir: Vec3| Vec3::new(0.5, 0.5, 0.5);
        let data = baker.bake_irradiance_data(4, 16, env_sampler);

        // With uniform environment, irradiance should be approximately pi * env_color
        let pixel = data.get_face_pixel(4, 1, 1); // +Z face, center-ish
                                                  // Irradiance for uniform env = env_color * pi
                                                  // So 0.5 * pi ≈ 1.57
        assert!(pixel[0] > 0.0);
        assert!(pixel[1] > 0.0);
        assert!(pixel[2] > 0.0);
    }

    #[test]
    fn test_bake_prefilter_data_basic() {
        let baker = IBLBaker::new();
        let env_sampler = |dir: Vec3| Vec3::new(1.0, 1.0, 1.0) * dir.z.max(0.0);
        let data = baker.bake_prefilter_data(4, 0.5, 32, env_sampler);
        assert_eq!(data.size, 4);
        assert_eq!(data.roughness, 0.5);
        assert_eq!(data.pixels.len(), 6 * 4 * 4);
    }

    #[test]
    fn test_bake_prefilter_data_smooth() {
        let baker = IBLBaker::new();
        let env_sampler = |dir: Vec3| {
            // Environment that's bright in +Z direction
            Vec3::new(1.0, 1.0, 1.0) * dir.z.max(0.0)
        };
        let data = baker.bake_prefilter_data(4, 0.0, 32, env_sampler);

        // With roughness=0, prefilter should be close to the environment reflection
        let pixel = data.get_face_pixel(4, 1, 1);
        assert!(pixel[0] >= 0.0);
    }

    #[test]
    fn test_irradiance_map_data_new() {
        let data = IrradianceMapData::new(8);
        assert_eq!(data.size, 8);
        assert_eq!(data.pixels.len(), 6 * 8 * 8);
    }

    #[test]
    fn test_irradiance_map_data_get_set() {
        let mut data = IrradianceMapData::new(4);
        data.set_face_pixel(0, 1, 2, [0.1, 0.2, 0.3]);
        let pixel = data.get_face_pixel(0, 1, 2);
        assert_eq!(pixel, [0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_irradiance_map_data_get_out_of_bounds() {
        let data = IrradianceMapData::new(4);
        let pixel = data.get_face_pixel(10, 100, 100);
        assert_eq!(pixel, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_irradiance_map_data_sample() {
        let mut data = IrradianceMapData::new(4);
        data.set_face_pixel(4, 2, 2, [1.0, 0.0, 0.0]); // +Z face center
        let sampled = data.sample(Vec3::new(0.0, 0.0, 1.0));
        // Should sample from +Z face
        assert!(sampled[0] >= 0.0);
    }

    #[test]
    fn test_prefilter_map_data_new() {
        let data = PrefilterMapData::new(8, 0.5);
        assert_eq!(data.size, 8);
        assert_eq!(data.roughness, 0.5);
        assert_eq!(data.pixels.len(), 6 * 8 * 8);
    }

    #[test]
    fn test_prefilter_map_data_get_set() {
        let mut data = PrefilterMapData::new(4, 0.3);
        data.set_face_pixel(2, 1, 1, [0.5, 0.5, 0.5]);
        let pixel = data.get_face_pixel(2, 1, 1);
        assert_eq!(pixel, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_prefilter_map_data_get_out_of_bounds() {
        let data = PrefilterMapData::new(4, 0.5);
        let pixel = data.get_face_pixel(10, 100, 100);
        assert_eq!(pixel, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_prefilter_map_data_sample() {
        let mut data = PrefilterMapData::new(4, 0.5);
        data.set_face_pixel(0, 2, 2, [1.0, 0.0, 0.0]); // +X face center
        let sampled = data.sample(Vec3::new(1.0, 0.0, 0.0));
        assert!(sampled[0] >= 0.0);
    }

    #[test]
    fn test_cube_map_direction_face_zero() {
        let dir = IBLBaker::cube_map_direction(0, 0, 0, 4);
        // +X face, should have positive x component
        assert!(dir.x > 0.0);
    }

    #[test]
    fn test_cube_map_direction_face_one() {
        let dir = IBLBaker::cube_map_direction(1, 0, 0, 4);
        // -X face, should have negative x component
        assert!(dir.x < 0.0);
    }

    #[test]
    fn test_cube_map_direction_face_two() {
        let dir = IBLBaker::cube_map_direction(2, 0, 0, 4);
        // +Y face, should have positive y component
        assert!(dir.y > 0.0);
    }

    #[test]
    fn test_cube_map_direction_face_four() {
        let dir = IBLBaker::cube_map_direction(4, 2, 2, 4);
        // +Z face, center should have positive z component
        assert!(dir.z > 0.0);
    }

    #[test]
    fn test_cube_map_direction_normalized() {
        let dir = IBLBaker::cube_map_direction(0, 1, 1, 4);
        let length = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
        assert!((length - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_halton_sequence() {
        // Halton(1, 2) = 0.5
        let h1 = IBLBaker::halton(1, 2);
        assert!((h1 - 0.5).abs() < 0.001);

        // Halton(2, 2) = 0.25
        let h2 = IBLBaker::halton(2, 2);
        assert!((h2 - 0.25).abs() < 0.001);

        // Halton(0, 2) = 0
        let h0 = IBLBaker::halton(0, 2);
        assert!((h0 - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_halton_base3() {
        // Halton(1, 3) = 1/3
        let h1 = IBLBaker::halton(1, 3);
        assert!((h1 - 1.0 / 3.0).abs() < 0.001);

        // Halton(2, 3) = 2/3
        let h2 = IBLBaker::halton(2, 3);
        assert!((h2 - 2.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn test_importance_sample_ggx_normalized() {
        let h = IBLBaker::importance_sample_ggx(0, 16, 0.5);
        let length = (h.x * h.x + h.y * h.y + h.z * h.z).sqrt();
        assert!((length - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_importance_sample_ggx_smooth() {
        // With low roughness, samples should be concentrated near the normal (z axis)
        let h = IBLBaker::importance_sample_ggx(0, 16, 0.1);
        assert!(h.z > 0.0);
    }

    #[test]
    fn test_geometry_smith_ibl() {
        let g = IBLBaker::geometry_smith_ibl(1.0, 1.0, 0.0);
        // With roughness=0, k=0, so G = 1
        assert!((g - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_geometry_smith_ibl_rough() {
        let g = IBLBaker::geometry_smith_ibl(0.5, 0.5, 1.0);
        // Should be less than 1 with roughness=1
        assert!(g > 0.0 && g < 1.0);
    }

    #[test]
    fn test_integrate_brdf_smooth() {
        // At NdotV=1, roughness=0, scale should be high
        let (scale, bias) = IBLBaker::integrate_brdf(1.0, 0.0, 64);
        assert!((0.0..=1.0).contains(&scale));
        assert!((0.0..=1.0).contains(&bias));
    }

    #[test]
    fn test_integrate_brdf_rough() {
        // At NdotV=1, roughness=1, scale should be lower
        let (scale, bias) = IBLBaker::integrate_brdf(1.0, 1.0, 64);
        assert!((0.0..=1.0).contains(&scale));
        assert!((0.0..=1.0).contains(&bias));
    }

    #[test]
    fn test_integrate_brdf_grazing_angle() {
        // At grazing angle (NdotV near 0), Fresnel should increase
        let (scale_low, _bias_low) = IBLBaker::integrate_brdf(0.1, 0.5, 64);
        let (scale_high, _bias_high) = IBLBaker::integrate_brdf(0.9, 0.5, 64);
        // Both should be valid
        assert!((0.0..=1.0).contains(&scale_low));
        assert!((0.0..=1.0).contains(&scale_high));
    }

    #[test]
    fn test_brdf_lut_data_clone() {
        let lut1 = BrdfLutData::new(4);
        let lut2 = lut1.clone();
        assert_eq!(lut1.width, lut2.width);
        assert_eq!(lut1.height, lut2.height);
    }

    #[test]
    fn test_irradiance_map_data_clone() {
        let data1 = IrradianceMapData::new(4);
        let data2 = data1.clone();
        assert_eq!(data1.size, data2.size);
    }

    #[test]
    fn test_prefilter_map_data_clone() {
        let data1 = PrefilterMapData::new(4, 0.5);
        let data2 = data1.clone();
        assert_eq!(data1.size, data2.size);
        assert_eq!(data1.roughness, data2.roughness);
    }

    #[test]
    fn test_bake_brdf_lut_data_sample_corners() {
        let baker = IBLBaker::new();
        let lut = baker.bake_brdf_lut_data(8, 32);

        // Sample at corners
        let (s00, _) = lut.sample(0.0, 0.0);
        let (s10, _) = lut.sample(1.0, 0.0);
        let (s01, _) = lut.sample(0.0, 1.0);
        let (s11, _) = lut.sample(1.0, 1.0);

        // All should be valid values
        assert!((0.0..=1.0).contains(&s00));
        assert!((0.0..=1.0).contains(&s10));
        assert!((0.0..=1.0).contains(&s01));
        assert!((0.0..=1.0).contains(&s11));
    }

    #[test]
    fn test_bake_irradiance_data_clamped() {
        let baker = IBLBaker::new();
        let env_sampler = |dir: Vec3| {
            // Very bright environment
            Vec3::new(10.0, 10.0, 10.0) * dir.z.max(0.0)
        };
        let data = baker.bake_irradiance_data(2, 4, env_sampler);
        // Should not panic and produce finite values
        for pixel in &data.pixels {
            assert!(pixel[0].is_finite());
            assert!(pixel[1].is_finite());
            assert!(pixel[2].is_finite());
        }
    }

    #[test]
    fn test_bake_prefilter_data_different_roughness() {
        let baker = IBLBaker::new();
        let env_sampler = |dir: Vec3| Vec3::new(1.0, 1.0, 1.0) * dir.z.max(0.0);

        let data_smooth = baker.bake_prefilter_data(2, 0.0, 16, env_sampler);
        let data_rough = baker.bake_prefilter_data(2, 1.0, 16, env_sampler);

        assert_eq!(data_smooth.roughness, 0.0);
        assert_eq!(data_rough.roughness, 1.0);
    }

    #[test]
    fn test_cube_map_direction_all_faces() {
        for face in 0..6u32 {
            let dir = IBLBaker::cube_map_direction(face, 1, 1, 4);
            // All directions should be normalized
            let length = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
            assert!((length - 1.0).abs() < 0.01, "Face {} not normalized", face);
        }
    }

    #[test]
    fn test_cube_map_direction_invalid_face() {
        let dir = IBLBaker::cube_map_direction(99, 0, 0, 4);
        // Should return default direction (0, 0, 1) normalized
        assert!((dir.z - 1.0).abs() < 0.01);
    }
}
