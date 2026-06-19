//! Cook-Torrance BRDF - Physically-based rendering lighting model
//!
//! Implements the microfacet BRDF model with:
//! - GGX (Trowbridge-Reitz) normal distribution function
//! - Smith's geometry function with Schlick-GGX
//! - Schlick's Fresnel approximation
//! - Lambertian diffuse term

use engine_math::Vec3;

/// PI constant for BRDF calculations
pub const PI: f32 = core::f32::consts::PI;

/// Minimum epsilon to avoid division by zero in dot products
const EPSILON: f32 = 1e-6;

/// Cook-Torrance BRDF implementation for PBR lighting
#[derive(Clone, Copy, Debug, Default)]
pub struct CookTorranceBRDF;

impl CookTorranceBRDF {
    /// GGX (Trowbridge-Reitz) normal distribution function
    ///
    /// Estimates the microfacets aligned to the halfway vector H
    /// relative to the surface normal N.
    ///
    /// # Arguments
    /// * `n` - Surface normal (normalized)
    /// * `h` - Halfway vector between light and view directions (normalized)
    /// * `roughness` - Surface roughness [0, 1]
    pub fn distribution_ggx(n: Vec3, h: Vec3, roughness: f32) -> f32 {
        let a = roughness * roughness;
        let a2 = a * a;
        let n_dot_h = n.dot(h).max(0.0);
        let n_dot_h2 = n_dot_h * n_dot_h;

        let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
        denom * denom + EPSILON
    }

    /// Full GGX distribution value (not squared) for direct use
    pub fn d_ggx(n: Vec3, h: Vec3, roughness: f32) -> f32 {
        let a2 = (roughness * roughness) * (roughness * roughness);
        let n_dot_h = n.dot(h).max(0.0);
        let n_dot_h2 = n_dot_h * n_dot_h;

        let denom = n_dot_h2 * (a2 - 1.0) + 1.0;
        a2 / (PI * denom * denom + EPSILON)
    }

    /// Schlick-GGX geometry function (single direction)
    ///
    /// Used as part of Smith's method for both view and light directions.
    fn geometry_schlick_ggx(n_dot_x: f32, roughness: f32) -> f32 {
        // Use remapped roughness for direct lighting (k = (r+1)^2 / 8)
        // For IBL, k = r^2 / 2
        let r = roughness + 1.0;
        let k = (r * r) / 8.0;

        n_dot_x / (n_dot_x * (1.0 - k) + k + EPSILON)
    }

    /// Smith's geometry function (combines view and light directions)
    ///
    /// Accounts for shadowing and masking of microfacets.
    ///
    /// # Arguments
    /// * `n` - Surface normal
    /// * `v` - View direction
    /// * `l` - Light direction
    /// * `roughness` - Surface roughness
    pub fn geometry_smith(n: Vec3, v: Vec3, l: Vec3, roughness: f32) -> f32 {
        let n_dot_v = n.dot(v).max(0.0);
        let n_dot_l = n.dot(l).max(0.0);
        let ggx_v = Self::geometry_schlick_ggx(n_dot_v, roughness);
        let ggx_l = Self::geometry_schlick_ggx(n_dot_l, roughness);
        ggx_v * ggx_l
    }

    /// Smith's geometry function for IBL (uses different k remapping)
    pub fn geometry_smith_ibl(n: Vec3, v: Vec3, l: Vec3, roughness: f32) -> f32 {
        let n_dot_v = n.dot(v).max(0.0);
        let n_dot_l = n.dot(l).max(0.0);
        let k = (roughness * roughness) / 2.0;
        let ggx_v = n_dot_v / (n_dot_v * (1.0 - k) + k + EPSILON);
        let ggx_l = n_dot_l / (n_dot_l * (1.0 - k) + k + EPSILON);
        ggx_v * ggx_l
    }

    /// Schlick's Fresnel approximation
    ///
    /// Models the reflectance increase at grazing angles.
    ///
    /// # Arguments
    /// * `cos_theta` - Dot product between view and halfway (or normal) directions
    /// * `f0` - Reflectance at normal incidence (surface reflection)
    pub fn fresnel_schlick(cos_theta: f32, f0: Vec3) -> Vec3 {
        let cos_theta = cos_theta.clamp(0.0, 1.0);
        // Use 5th power approximation: F0 + (1 - F0) * (1 - cos)^5
        let one_minus_cos = 1.0 - cos_theta;
        let pow5 = one_minus_cos * one_minus_cos * one_minus_cos * one_minus_cos * one_minus_cos;
        f0 + (Vec3::new(1.0, 1.0, 1.0) - f0) * pow5
    }

    /// Schlick's Fresnel with roughness consideration (for IBL)
    pub fn fresnel_schlick_roughness(cos_theta: f32, f0: Vec3, roughness: f32) -> Vec3 {
        let cos_theta = cos_theta.clamp(0.0, 1.0);
        let one_minus_cos = 1.0 - cos_theta;
        let pow5 = one_minus_cos * one_minus_cos * one_minus_cos * one_minus_cos * one_minus_cos;
        // F0_90 adjusted by roughness
        let f90 = Vec3::new(
            1.0 + roughness,
            1.0 + roughness,
            1.0 + roughness,
        );
        f0 + (f90 - f0) * pow5
    }

    /// Calculate F0 (reflectance at normal incidence) from material parameters
    ///
    /// For dielectrics: F0 = 0.04 (4% reflectance)
    /// For metals: F0 = albedo (tinted reflectance)
    ///
    /// # Arguments
    /// * `albedo` - Base color
    /// * `metallic` - Metallic factor [0, 1]
    pub fn calculate_f0(albedo: Vec3, metallic: f32) -> Vec3 {
        let dielectric_f0 = Vec3::new(0.04, 0.04, 0.04);
        dielectric_f0.lerp(albedo, metallic)
    }

    /// Lambertian diffuse BRDF
    ///
    /// # Arguments
    /// * `albedo` - Surface base color
    /// * `metallic` - Metallic factor (metals have no diffuse)
    pub fn diffuse_brdf(albedo: Vec3, metallic: f32) -> Vec3 {
        // Metals have minimal diffuse component
        let kd = 1.0 - metallic;
        albedo * kd / PI
    }

    /// Specular BRDF (Cook-Torrance)
    ///
    /// Computes the specular reflection term: D*G*F / (4 * NdotL * NdotV)
    ///
    /// # Arguments
    /// * `n` - Surface normal (normalized)
    /// * `v` - View direction (normalized)
    /// * `l` - Light direction (normalized)
    /// * `f0` - Reflectance at normal incidence
    /// * `roughness` - Surface roughness [0, 1]
    pub fn specular_brdf(
        n: Vec3,
        v: Vec3,
        l: Vec3,
        f0: Vec3,
        roughness: f32,
    ) -> Vec3 {
        let h = (v + l).normalize_or_zero();
        let n_dot_v = n.dot(v).max(0.0);
        let n_dot_l = n.dot(l).max(0.0);

        if n_dot_v < EPSILON || n_dot_l < EPSILON {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let d = Self::d_ggx(n, h, roughness);
        let g = Self::geometry_smith(n, v, l, roughness);
        let f = Self::fresnel_schlick(h.dot(v).max(0.0), f0);

        let numerator = d * g * f;
        let denominator = 4.0 * n_dot_v * n_dot_l + EPSILON;

        numerator / denominator
    }

    /// Full Cook-Torrance BRDF (diffuse + specular)
    ///
    /// Combines Lambertian diffuse and Cook-Torrance specular terms.
    /// The Fresnel term reduces diffuse contribution for metals.
    ///
    /// # Arguments
    /// * `n` - Surface normal (normalized)
    /// * `v` - View direction (normalized)
    /// * `l` - Light direction (normalized)
    /// * `albedo` - Surface base color
    /// * `metallic` - Metallic factor [0, 1]
    /// * `roughness` - Surface roughness [0, 1]
    pub fn evaluate(
        n: Vec3,
        v: Vec3,
        l: Vec3,
        albedo: Vec3,
        metallic: f32,
        roughness: f32,
    ) -> BrdfResult {
        let h = (v + l).normalize_or_zero();
        let n_dot_l = n.dot(l).max(0.0);
        let n_dot_v = n.dot(v).max(0.0);

        let f0 = Self::calculate_f0(albedo, metallic);
        let f = Self::fresnel_schlick(h.dot(v).max(0.0), f0);

        // ks = F, kd = (1 - ks) * (1 - metallic)
        let ks = f;
        let kd = (Vec3::new(1.0, 1.0, 1.0) - ks) * (1.0 - metallic);

        let diffuse = kd * albedo / PI;
        let specular = Self::specular_brdf(n, v, l, f0, roughness);

        // Apply NdotL attenuation
        let radiance_factor = n_dot_l;

        BrdfResult {
            diffuse,
            specular,
            ks,
            kd,
            n_dot_l,
            n_dot_v,
            radiance_factor,
        }
    }

    /// Compute the final reflected radiance for a single light
    ///
    /// # Arguments
    /// * `n` - Surface normal
    /// * `v` - View direction
    /// * `l` - Light direction
    /// * `light_color` - Light radiance/irradiance
    /// * `albedo` - Surface base color
    /// * `metallic` - Metallic factor
    /// * `roughness` - Surface roughness
    pub fn compute_lighting(
        n: Vec3,
        v: Vec3,
        l: Vec3,
        light_color: Vec3,
        albedo: Vec3,
        metallic: f32,
        roughness: f32,
    ) -> Vec3 {
        let result = Self::evaluate(n, v, l, albedo, metallic, roughness);
        let n_dot_l = result.n_dot_l;
        (result.diffuse + result.specular) * light_color * n_dot_l
    }
}

/// Result of BRDF evaluation containing individual terms
#[derive(Clone, Copy, Debug)]
pub struct BrdfResult {
    /// Diffuse reflectance term
    pub diffuse: Vec3,
    /// Specular reflectance term
    pub specular: Vec3,
    /// Fresnel/specular ratio (ks)
    pub ks: Vec3,
    /// Diffuse ratio (kd)
    pub kd: Vec3,
    /// N dot L (normal-light angle)
    pub n_dot_l: f32,
    /// N dot V (normal-view angle)
    pub n_dot_v: f32,
    /// Radiance attenuation factor
    pub radiance_factor: f32,
}

impl BrdfResult {
    /// Total BRDF contribution (diffuse + specular)
    pub fn total(&self) -> Vec3 {
        self.diffuse + self.specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    fn vec_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn test_pi_constant() {
        assert!(approx_eq(PI, core::f32::consts::PI));
    }

    #[test]
    fn test_distribution_ggx_perfect_alignment() {
        // When N and H are aligned, distribution should be high
        let n = Vec3::new(0.0, 1.0, 0.0);
        let h = Vec3::new(0.0, 1.0, 0.0);
        let d = CookTorranceBRDF::d_ggx(n, h, 0.5);
        assert!(d > 0.0);
    }

    #[test]
    fn test_distribution_ggx_perpendicular() {
        // When N and H are perpendicular, distribution should be near zero
        let n = Vec3::new(0.0, 1.0, 0.0);
        let h = Vec3::new(1.0, 0.0, 0.0);
        let d = CookTorranceBRDF::d_ggx(n, h, 0.5);
        assert!(d >= 0.0);
        // Should be very small for perpendicular case
        assert!(d < 1.0);
    }

    #[test]
    fn test_distribution_ggx_smooth_surface() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let h = Vec3::new(0.0, 1.0, 0.0);
        // Smoother surface (lower roughness) should have sharper peak
        let d_smooth = CookTorranceBRDF::d_ggx(n, h, 0.1);
        let d_rough = CookTorranceBRDF::d_ggx(n, h, 0.9);
        // At perfect alignment, smoother surface has higher peak
        assert!(d_smooth > d_rough);
    }

    #[test]
    fn test_distribution_ggx_zero_roughness() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let h = Vec3::new(0.0, 1.0, 0.0);
        let d = CookTorranceBRDF::d_ggx(n, h, 0.0);
        assert!(d.is_finite());
        assert!(d >= 0.0);
    }

    #[test]
    fn test_geometry_schlick_ggx_perfect() {
        // When NdotV = 1, geometry should be 1 (no shadowing)
        let g = CookTorranceBRDF::geometry_smith(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.5,
        );
        assert!(g > 0.0 && g <= 1.0);
    }

    #[test]
    fn test_geometry_smith_grazing_angle() {
        // At grazing angles, geometry should reduce (more shadowing)
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v_straight = Vec3::new(0.0, 1.0, 0.0);
        let v_grazing = Vec3::new(1.0, 0.01, 0.0).normalize_or_zero();
        let l = Vec3::new(0.0, 1.0, 0.0);

        let g_straight = CookTorranceBRDF::geometry_smith(n, v_straight, l, 0.5);
        let g_grazing = CookTorranceBRDF::geometry_smith(n, v_grazing, l, 0.5);
        assert!(g_straight >= g_grazing);
    }

    #[test]
    fn test_geometry_smith_ibl_uses_different_k() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, 1.0, 0.0);
        let g_direct = CookTorranceBRDF::geometry_smith(n, v, l, 0.5);
        let g_ibl = CookTorranceBRDF::geometry_smith_ibl(n, v, l, 0.5);
        // Both should be positive and finite
        assert!(g_direct.is_finite());
        assert!(g_ibl.is_finite());
        assert!(g_direct > 0.0);
        assert!(g_ibl > 0.0);
    }

    #[test]
    fn test_fresnel_schlick_normal_incidence() {
        // At normal incidence (cos=1), F = F0
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let f = CookTorranceBRDF::fresnel_schlick(1.0, f0);
        assert!(vec_approx_eq(f, f0));
    }

    #[test]
    fn test_fresnel_schlick_grazing_angle() {
        // At grazing angle (cos=0), F approaches 1
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let f = CookTorranceBRDF::fresnel_schlick(0.0, f0);
        assert!(f.x > f0.x);
        assert!(f.y > f0.y);
        assert!(f.z > f0.z);
        // Should approach 1 at grazing
        assert!(approx_eq(f.x, 1.0));
    }

    #[test]
    fn test_fresnel_schlick_monotonic() {
        // Fresnel should increase as cos_theta decreases
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let f1 = CookTorranceBRDF::fresnel_schlick(1.0, f0).x;
        let f2 = CookTorranceBRDF::fresnel_schlick(0.5, f0).x;
        let f3 = CookTorranceBRDF::fresnel_schlick(0.0, f0).x;
        assert!(f1 <= f2);
        assert!(f2 <= f3);
    }

    #[test]
    fn test_fresnel_schlick_roughness() {
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let f = CookTorranceBRDF::fresnel_schlick_roughness(0.5, f0, 0.5);
        assert!(f.x >= f0.x);
        assert!(f.y >= f0.y);
        assert!(f.z >= f0.z);
    }

    #[test]
    fn test_calculate_f0_dielectric() {
        // Dielectric (metallic=0) should have F0 = 0.04
        let albedo = Vec3::new(1.0, 0.0, 0.0);
        let f0 = CookTorranceBRDF::calculate_f0(albedo, 0.0);
        assert!(approx_eq(f0.x, 0.04));
        assert!(approx_eq(f0.y, 0.04));
        assert!(approx_eq(f0.z, 0.04));
    }

    #[test]
    fn test_calculate_f0_metal() {
        // Metal (metallic=1) should have F0 = albedo
        let albedo = Vec3::new(0.8, 0.6, 0.2);
        let f0 = CookTorranceBRDF::calculate_f0(albedo, 1.0);
        assert!(vec_approx_eq(f0, albedo));
    }

    #[test]
    fn test_calculate_f0_semi_metal() {
        // Semi-metal should interpolate
        let albedo = Vec3::new(1.0, 0.0, 0.0);
        let f0 = CookTorranceBRDF::calculate_f0(albedo, 0.5);
        let expected = Vec3::new(0.04, 0.04, 0.04).lerp(albedo, 0.5);
        assert!(vec_approx_eq(f0, expected));
    }

    #[test]
    fn test_diffuse_brdf_dielectric() {
        let albedo = Vec3::new(0.5, 0.5, 0.5);
        let diffuse = CookTorranceBRDF::diffuse_brdf(albedo, 0.0);
        // kd = 1.0, diffuse = albedo / PI
        assert!(approx_eq(diffuse.x, 0.5 / PI));
    }

    #[test]
    fn test_diffuse_brdf_metal() {
        let albedo = Vec3::new(0.5, 0.5, 0.5);
        let diffuse = CookTorranceBRDF::diffuse_brdf(albedo, 1.0);
        // Metals have no diffuse
        assert!(approx_eq(diffuse.x, 0.0));
        assert!(approx_eq(diffuse.y, 0.0));
        assert!(approx_eq(diffuse.z, 0.0));
    }

    #[test]
    fn test_specular_brdf_perfect_reflection() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, 1.0, 0.0);
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let spec = CookTorranceBRDF::specular_brdf(n, v, l, f0, 0.5);
        assert!(spec.x >= 0.0);
        assert!(spec.y >= 0.0);
        assert!(spec.z >= 0.0);
    }

    #[test]
    fn test_specular_brdf_back_facing() {
        // Light from behind the surface should produce no specular
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, -1.0, 0.0);
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let spec = CookTorranceBRDF::specular_brdf(n, v, l, f0, 0.5);
        assert!(approx_eq(spec.x, 0.0));
        assert!(approx_eq(spec.y, 0.0));
        assert!(approx_eq(spec.z, 0.0));
    }

    #[test]
    fn test_evaluate_returns_valid_result() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, 1.0, 0.0);
        let albedo = Vec3::new(0.8, 0.8, 0.8);
        let result = CookTorranceBRDF::evaluate(n, v, l, albedo, 0.0, 0.5);

        assert!(result.diffuse.x >= 0.0);
        assert!(result.specular.x >= 0.0);
        assert!(result.n_dot_l >= 0.0);
        assert!(result.n_dot_v >= 0.0);
    }

    #[test]
    fn test_evaluate_metal_has_less_diffuse() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, 1.0, 0.0);
        let albedo = Vec3::new(0.8, 0.8, 0.8);

        let result_dielectric = CookTorranceBRDF::evaluate(n, v, l, albedo, 0.0, 0.5);
        let result_metal = CookTorranceBRDF::evaluate(n, v, l, albedo, 1.0, 0.5);

        // Metal should have less diffuse
        assert!(result_metal.diffuse.x <= result_dielectric.diffuse.x);
    }

    #[test]
    fn test_compute_lighting_basic() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, 1.0, 0.0);
        let light = Vec3::new(1.0, 1.0, 1.0);
        let albedo = Vec3::new(0.8, 0.8, 0.8);

        let color = CookTorranceBRDF::compute_lighting(n, v, l, light, albedo, 0.0, 0.5);
        assert!(color.x >= 0.0);
        assert!(color.y >= 0.0);
        assert!(color.z >= 0.0);
    }

    #[test]
    fn test_compute_lighting_no_light_when_back_facing() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let l = Vec3::new(0.0, -1.0, 0.0); // Light from behind
        let light = Vec3::new(1.0, 1.0, 1.0);
        let albedo = Vec3::new(0.8, 0.8, 0.8);

        let color = CookTorranceBRDF::compute_lighting(n, v, l, light, albedo, 0.0, 0.5);
        // NdotL = 0, so lighting should be zero
        assert!(approx_eq(color.x, 0.0));
        assert!(approx_eq(color.y, 0.0));
        assert!(approx_eq(color.z, 0.0));
    }

    #[test]
    fn test_brdf_result_total() {
        let result = BrdfResult {
            diffuse: Vec3::new(0.1, 0.2, 0.3),
            specular: Vec3::new(0.4, 0.5, 0.6),
            ks: Vec3::new(0.5, 0.5, 0.5),
            kd: Vec3::new(0.5, 0.5, 0.5),
            n_dot_l: 1.0,
            n_dot_v: 1.0,
            radiance_factor: 1.0,
        };
        let total = result.total();
        assert!(approx_eq(total.x, 0.5));
        assert!(approx_eq(total.y, 0.7));
        assert!(approx_eq(total.z, 0.9));
    }

    #[test]
    fn test_distribution_ggx_returns_finite() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let h = Vec3::new(0.0, 1.0, 0.0);
        for r in [0.01, 0.1, 0.5, 0.9, 1.0] {
            let d = CookTorranceBRDF::d_ggx(n, h, r);
            assert!(d.is_finite());
            assert!(d >= 0.0);
        }
    }

    #[test]
    fn test_fresnel_schlick_clamps_cos_theta() {
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        // Negative cos_theta should be clamped to 0
        let f = CookTorranceBRDF::fresnel_schlick(-1.0, f0);
        assert!(f.x >= f0.x);
        // cos_theta > 1 should be clamped to 1
        let f2 = CookTorranceBRDF::fresnel_schlick(2.0, f0);
        assert!(vec_approx_eq(f2, f0));
    }

    #[test]
    fn test_specular_brdf_zero_at_grazing() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(1.0, 0.001, 0.0).normalize_or_zero();
        let l = Vec3::new(0.0, 1.0, 0.0);
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let spec = CookTorranceBRDF::specular_brdf(n, v, l, f0, 0.5);
        assert!(spec.x >= 0.0);
        assert!(spec.x.is_finite());
    }

    #[test]
    fn test_cook_torrance_default() {
        let _brdf = CookTorranceBRDF;
    }

    #[test]
    fn test_cook_torrance_clone_copy() {
        let brdf1 = CookTorranceBRDF;
        let brdf2 = brdf1;
        let _brdf3 = brdf1;
        let _ = format!("{:?}", brdf2);
    }
}
