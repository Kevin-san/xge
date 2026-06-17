//! Tonemapper - HDR to LDR color mapping

use engine_math::Vec3;

/// Tonemapping algorithm for converting HDR colors to displayable LDR range
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tonemapper {
    /// ACES filmic tone mapping (industry standard)
    Aces,
    /// Reinhard simple tone mapping
    Reinhard,
    /// Unreal Engine filmic tone mapping
    Filmic,
    /// No tone mapping (pass-through)
    #[default]
    None,
}

impl Tonemapper {
    /// Apply tone mapping to HDR color
    ///
    /// Converts HDR color values to LDR range [0, 1]
    pub fn apply(&self, hdr_color: Vec3) -> Vec3 {
        match self {
            Tonemapper::Aces => self.aces(hdr_color),
            Tonemapper::Reinhard => self.reinhard(hdr_color),
            Tonemapper::Filmic => self.filmic(hdr_color),
            Tonemapper::None => hdr_color,
        }
    }

    /// ACES Filmic Tone Mapping
    ///
    /// Based on the Academy Color Encoding System standard
    fn aces(&self, color: Vec3) -> Vec3 {
        // ACES approximation by Krzysztof Narkowicz
        let a = 2.51;
        let b = 0.03;
        let c = -2.43;
        let d = -0.59;
        let e = 0.14;

        Vec3::new(
            self.clamp((color.x * (a * color.x + b)) / (color.x * (c * color.x + d) + e)),
            self.clamp((color.y * (a * color.y + b)) / (color.y * (c * color.y + d) + e)),
            self.clamp((color.z * (a * color.z + b)) / (color.z * (c * color.z + d) + e)),
        )
    }

    /// Reinhard Tone Mapping
    ///
    /// Simple L/(1+L) formula
    fn reinhard(&self, color: Vec3) -> Vec3 {
        Vec3::new(
            color.x / (1.0 + color.x),
            color.y / (1.0 + color.y),
            color.z / (1.0 + color.z),
        )
    }

    /// Filmic Tone Mapping (Unreal Engine style)
    ///
    /// Provides smooth transitions with shoulder and toe regions
    fn filmic(&self, color: Vec3) -> Vec3 {
        Vec3::new(
            self.clamp(self.filmic_curve(color.x)),
            self.clamp(self.filmic_curve(color.y)),
            self.clamp(self.filmic_curve(color.z)),
        )
    }

    /// Filmic curve helper function (Hable filmic curve approximation)
    fn filmic_curve(&self, x: f32) -> f32 {
        // Hable filmic tone mapping curve
        // A = 0.15, B = 0.50, C = 0.10, D = 0.20, E = 0.02, F = 0.30
        let a = 0.15;
        let b = 0.50;
        let c = 0.10;
        let d = 0.20;
        let e = 0.02;
        let f = 0.30;

        let x = x.max(0.0);
        ((x * (a * x + c * b) + d * e) / (x * (a * x + b) + d * f)) - e / f
    }

    /// Clamp value to [0, 1] range
    fn clamp(&self, x: f32) -> f32 {
        x.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonemapper_none() {
        let tm = Tonemapper::None;
        let color = Vec3::new(0.5, 0.7, 0.9);
        let result = tm.apply(color);
        assert_eq!(result, color);
    }

    #[test]
    fn test_tonemapper_reinhard() {
        let tm = Tonemapper::Reinhard;
        let color = Vec3::new(1.0, 2.0, 4.0);
        let result = tm.apply(color);
        // L/(1+L) formula
        assert!((result.x - 0.5).abs() < 0.001);
        assert!((result.y - 0.666).abs() < 0.01);
        assert!((result.z - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_tonemapper_aces_positive() {
        let tm = Tonemapper::Aces;
        let color = Vec3::new(0.5, 1.0, 2.0);
        let result = tm.apply(color);
        // All values should be in [0, 1]
        assert!(result.x >= 0.0 && result.x <= 1.0);
        assert!(result.y >= 0.0 && result.y <= 1.0);
        assert!(result.z >= 0.0 && result.z <= 1.0);
    }

    #[test]
    fn test_tonemapper_aces_zero() {
        let tm = Tonemapper::Aces;
        let color = Vec3::new(0.0, 0.0, 0.0);
        let result = tm.apply(color);
        assert_eq!(result, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_tonemapper_aces_high_values() {
        let tm = Tonemapper::Aces;
        let color = Vec3::new(10.0, 20.0, 50.0);
        let result = tm.apply(color);
        // Should compress to valid range
        assert!(result.x >= 0.0 && result.x <= 1.0);
        assert!(result.y >= 0.0 && result.y <= 1.0);
        assert!(result.z >= 0.0 && result.z <= 1.0);
    }

    #[test]
    fn test_tonemapper_filmic() {
        let tm = Tonemapper::Filmic;
        let color = Vec3::new(0.5, 1.0, 2.0);
        let result = tm.apply(color);
        assert!(result.x >= 0.0 && result.x <= 1.0);
        assert!(result.y >= 0.0 && result.y <= 1.0);
        assert!(result.z >= 0.0 && result.z <= 1.0);
    }
}
