//! Color Grading - Post-processing color adjustments

use engine_math::Vec3;
use serde::{Deserialize, Serialize};

/// Color grading parameters for post-processing
///
/// Controls exposure, contrast, saturation, temperature, and gamma adjustments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorGrading {
    /// Exposure adjustment (EV value)
    pub exposure: f32,
    /// Contrast multiplier
    pub contrast: f32,
    /// Saturation multiplier
    pub saturation: f32,
    /// Color temperature (warm/cold shift)
    pub temperature: f32,
    /// Gamma correction value
    pub gamma: f32,
    /// Tint adjustment (magenta/green shift)
    pub tint: f32,
    /// Brightness offset
    pub brightness: f32,
}

impl Default for ColorGrading {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            temperature: 0.0,
            gamma: 1.0,
            tint: 0.0,
            brightness: 0.0,
        }
    }
}

impl ColorGrading {
    /// Create default color grading (no adjustments)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create color grading with specific exposure
    pub fn with_exposure(exposure: f32) -> Self {
        Self {
            exposure,
            ..Self::default()
        }
    }

    /// Apply color grading to a color
    ///
    /// Returns the adjusted color in linear space
    pub fn apply(&self, color: Vec3) -> Vec3 {
        let mut result = color;

        // Apply exposure (2^EV)
        result *= 2.0_f32.powf(self.exposure);

        // Apply brightness
        result += Vec3::new(self.brightness, self.brightness, self.brightness);

        // Apply contrast
        result = (result - Vec3::new(0.5, 0.5, 0.5)) * self.contrast + Vec3::new(0.5, 0.5, 0.5);

        // Apply temperature (warm/cold shift)
        result = self.apply_temperature(result);

        // Apply tint (magenta/green shift)
        result = self.apply_tint(result);

        // Apply saturation
        result = self.apply_saturation(result);

        // Apply gamma
        result = Vec3::new(
            result.x.powf(1.0 / self.gamma),
            result.y.powf(1.0 / self.gamma),
            result.z.powf(1.0 / self.gamma),
        );

        result
    }

    /// Apply temperature adjustment
    ///
    /// Positive values warm the image (more orange/red)
    /// Negative values cool the image (more blue)
    fn apply_temperature(&self, color: Vec3) -> Vec3 {
        let t = self.temperature;
        Vec3::new(color.x + t * 0.1, color.y, color.z - t * 0.1)
    }

    /// Apply tint adjustment
    ///
    /// Positive values shift towards magenta
    /// Negative values shift towards green
    fn apply_tint(&self, color: Vec3) -> Vec3 {
        let t = self.tint;
        Vec3::new(color.x + t * 0.05, color.y - t * 0.1, color.z + t * 0.05)
    }

    /// Apply saturation adjustment
    ///
    /// Uses luminance-preserving saturation formula
    fn apply_saturation(&self, color: Vec3) -> Vec3 {
        // Calculate luminance (approximate)
        let luminance = 0.2126 * color.x + 0.7152 * color.y + 0.0722 * color.z;

        // Blend between luminance and original color based on saturation
        Vec3::new(
            luminance + (color.x - luminance) * self.saturation,
            luminance + (color.y - luminance) * self.saturation,
            luminance + (color.z - luminance) * self.saturation,
        )
    }

    // Getter methods

    /// Get exposure value
    pub fn exposure(&self) -> f32 {
        self.exposure
    }

    /// Set exposure value
    pub fn set_exposure(&mut self, v: f32) {
        self.exposure = v;
    }

    /// Get contrast value
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    /// Set contrast value
    pub fn set_contrast(&mut self, v: f32) {
        self.contrast = v;
    }

    /// Get saturation value
    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    /// Set saturation value
    pub fn set_saturation(&mut self, v: f32) {
        self.saturation = v;
    }

    /// Get temperature value
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    /// Set temperature value
    pub fn set_temperature(&mut self, v: f32) {
        self.temperature = v;
    }

    /// Get gamma value
    pub fn gamma(&self) -> f32 {
        self.gamma
    }

    /// Set gamma value
    pub fn set_gamma(&mut self, v: f32) {
        self.gamma = v;
    }

    /// Get tint value
    pub fn tint(&self) -> f32 {
        self.tint
    }

    /// Set tint value
    pub fn set_tint(&mut self, v: f32) {
        self.tint = v;
    }

    /// Get brightness value
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    /// Set brightness value
    pub fn set_brightness(&mut self, v: f32) {
        self.brightness = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_grading_default() {
        let cg = ColorGrading::default();
        assert_eq!(cg.exposure, 0.0);
        assert_eq!(cg.contrast, 1.0);
        assert_eq!(cg.saturation, 1.0);
        assert_eq!(cg.gamma, 1.0);
    }

    #[test]
    fn test_color_grading_apply_default() {
        let cg = ColorGrading::default();
        let color = Vec3::new(0.5, 0.5, 0.5);
        let result = cg.apply(color);
        // Default settings should not change the color significantly
        assert!((result.x - 0.5).abs() < 0.01);
        assert!((result.y - 0.5).abs() < 0.01);
        assert!((result.z - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_color_grading_exposure() {
        let cg = ColorGrading::with_exposure(1.0); // +1 EV = 2x brightness
        let color = Vec3::new(0.25, 0.25, 0.25);
        let result = cg.apply(color);
        // Exposure should multiply by 2^1 = 2
        assert!((result.x - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_color_grading_contrast() {
        let cg = ColorGrading {
            contrast: 2.0,
            ..ColorGrading::default()
        };
        let color = Vec3::new(0.75, 0.75, 0.75);
        let result = cg.apply(color);
        // Higher contrast should push values away from 0.5
        assert!(result.x > 0.75);
    }

    #[test]
    fn test_color_grading_saturation() {
        let cg = ColorGrading {
            saturation: 0.0,
            ..ColorGrading::default()
        };
        let color = Vec3::new(1.0, 0.5, 0.0);
        let result = cg.apply(color);
        // Zero saturation should result in grayscale
        let luminance = 0.2126 * 1.0 + 0.7152 * 0.5 + 0.0722 * 0.0;
        assert!((result.x - luminance).abs() < 0.01);
        assert!((result.y - luminance).abs() < 0.01);
        assert!((result.z - luminance).abs() < 0.01);
    }

    #[test]
    fn test_color_grading_temperature_warm() {
        let cg = ColorGrading {
            temperature: 1.0,
            ..ColorGrading::default()
        };
        let color = Vec3::new(0.5, 0.5, 0.5);
        let result = cg.apply(color);
        // Warm temperature should increase red and decrease blue
        assert!(result.x > color.x);
        assert!(result.z < color.z);
    }

    #[test]
    fn test_color_grading_temperature_cold() {
        let cg = ColorGrading {
            temperature: -1.0,
            ..ColorGrading::default()
        };
        let color = Vec3::new(0.5, 0.5, 0.5);
        let result = cg.apply(color);
        // Cold temperature should decrease red and increase blue
        assert!(result.x < color.x);
        assert!(result.z > color.z);
    }

    #[test]
    fn test_color_grading_gamma() {
        let cg = ColorGrading {
            gamma: 2.0,
            ..ColorGrading::default()
        };
        let color = Vec3::new(0.25, 0.25, 0.25);
        let result = cg.apply(color);
        // Gamma 2.0 should brighten midtones
        assert!(result.x > color.x);
    }

    #[test]
    fn test_color_grading_setters() {
        let mut cg = ColorGrading::default();
        cg.set_exposure(1.0);
        cg.set_contrast(1.5);
        cg.set_saturation(0.8);
        cg.set_temperature(0.5);
        cg.set_gamma(1.2);
        cg.set_tint(0.3);
        cg.set_brightness(0.1);

        assert_eq!(cg.exposure(), 1.0);
        assert_eq!(cg.contrast(), 1.5);
        assert_eq!(cg.saturation(), 0.8);
        assert_eq!(cg.temperature(), 0.5);
        assert_eq!(cg.gamma(), 1.2);
        assert_eq!(cg.tint(), 0.3);
        assert_eq!(cg.brightness(), 0.1);
    }
}
