//! HDR Framebuffer - High dynamic range rendering pipeline
//!
//! Provides HDR framebuffer configuration, exposure control,
//! and HDR-to-LDR conversion with tone mapping.

use engine_math::Vec3;

use crate::Tonemapper;

/// HDR framebuffer format
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HdrFormat {
    /// 16-bit float (half precision)
    R16G16B16A16Float,
    /// 32-bit float (full precision)
    R32G32B32A32Float,
    /// 11-bit float (RGB, no alpha)
    R11G11B10Float,
    /// 10-bit unsigned (RGB, no alpha)
    #[default]
    R10G10B10A2Unorm,
}

impl HdrFormat {
    /// Get the number of bits per pixel
    pub fn bits_per_pixel(&self) -> u32 {
        match self {
            HdrFormat::R16G16B16A16Float => 64,
            HdrFormat::R32G32B32A32Float => 128,
            HdrFormat::R11G11B10Float => 32,
            HdrFormat::R10G10B10A2Unorm => 32,
        }
    }

    /// Check if the format supports alpha channel
    pub fn has_alpha(&self) -> bool {
        matches!(
            self,
            HdrFormat::R16G16B16A16Float
                | HdrFormat::R32G32B32A32Float
                | HdrFormat::R10G10B10A2Unorm
        )
    }

    /// Check if the format is floating point
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            HdrFormat::R16G16B16A16Float | HdrFormat::R32G32B32A32Float | HdrFormat::R11G11B10Float
        )
    }
}

/// Exposure settings for HDR rendering
#[derive(Clone, Copy, Debug)]
pub struct ExposureSettings {
    /// Exposure value (EV), higher = brighter
    pub exposure: f32,
    /// Whether to use auto-exposure
    pub auto_exposure: bool,
    /// Target luminance for auto-exposure
    pub target_luminance: f32,
    /// Adaptation speed for auto-exposure (0-1)
    pub adaptation_speed: f32,
    /// Minimum exposure value
    pub min_exposure: f32,
    /// Maximum exposure value
    pub max_exposure: f32,
}

impl Default for ExposureSettings {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            auto_exposure: false,
            target_luminance: 0.5,
            adaptation_speed: 0.5,
            min_exposure: 0.1,
            max_exposure: 10.0,
        }
    }
}

impl ExposureSettings {
    /// Create new exposure settings with a specific exposure value
    pub fn new(exposure: f32) -> Self {
        Self {
            exposure,
            ..Self::default()
        }
    }

    /// Create auto-exposure settings
    pub fn auto() -> Self {
        Self {
            auto_exposure: true,
            ..Self::default()
        }
    }

    /// Apply exposure to an HDR color
    pub fn apply_exposure(&self, color: Vec3) -> Vec3 {
        color * self.exposure
    }

    /// Update auto-exposure based on average luminance
    ///
    /// # Arguments
    /// * `avg_luminance` - Average scene luminance
    /// * `delta_time` - Time elapsed since last update (seconds)
    pub fn update_auto_exposure(&mut self, avg_luminance: f32, delta_time: f32) {
        if !self.auto_exposure || avg_luminance <= 0.0 {
            return;
        }

        // Calculate target exposure based on target luminance
        let target_exposure = self.target_luminance / avg_luminance;
        let target_exposure = target_exposure.clamp(self.min_exposure, self.max_exposure);

        // Smoothly adapt towards target
        let t = (self.adaptation_speed * delta_time).clamp(0.0, 1.0);
        self.exposure = self.exposure + (target_exposure - self.exposure) * t;
    }
}

/// HDR framebuffer configuration
#[derive(Clone, Debug)]
pub struct HdrFramebuffer {
    /// Framebuffer width in pixels
    pub width: u32,
    /// Framebuffer height in pixels
    pub height: u32,
    /// HDR texture format
    pub format: HdrFormat,
    /// Whether to use multisampling
    pub samples: u32,
    /// Exposure settings
    pub exposure: ExposureSettings,
    /// Tone mapping algorithm
    pub tonemapper: Tonemapper,
    /// Whether to generate mipmaps for the HDR buffer
    pub generate_mipmaps: bool,
}

impl Default for HdrFramebuffer {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            format: HdrFormat::R16G16B16A16Float,
            samples: 1,
            exposure: ExposureSettings::default(),
            tonemapper: Tonemapper::Aces,
            generate_mipmaps: false,
        }
    }
}

impl HdrFramebuffer {
    /// Create a new HDR framebuffer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Self::default()
        }
    }

    /// Create an HDR framebuffer with a specific format
    pub fn with_format(width: u32, height: u32, format: HdrFormat) -> Self {
        Self {
            width,
            height,
            format,
            ..Self::default()
        }
    }

    /// Resize the framebuffer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Get the aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            1.0
        } else {
            self.width as f32 / self.height as f32
        }
    }

    /// Get the total pixel count
    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    /// Get the memory size in bytes for the HDR buffer
    pub fn memory_size_bytes(&self) -> u64 {
        (self.pixel_count() as u64 * self.format.bits_per_pixel() as u64) / 8
    }

    /// Convert an HDR color to LDR using the configured tone mapper and exposure
    ///
    /// # Arguments
    /// * `hdr_color` - Input HDR color
    pub fn tonemap(&self, hdr_color: Vec3) -> Vec3 {
        let exposed = self.exposure.apply_exposure(hdr_color);
        self.tonemapper.apply(exposed)
    }

    /// Convert an HDR color to LDR without applying exposure
    pub fn tonemap_only(&self, hdr_color: Vec3) -> Vec3 {
        self.tonemapper.apply(hdr_color)
    }

    /// Set the exposure value
    pub fn set_exposure(&mut self, exposure: f32) {
        self.exposure.exposure = exposure;
    }

    /// Get the current exposure value
    pub fn exposure(&self) -> f32 {
        self.exposure.exposure
    }

    /// Set the tone mapper
    pub fn set_tonemapper(&mut self, tonemapper: Tonemapper) {
        self.tonemapper = tonemapper;
    }

    /// Enable or disable auto-exposure
    pub fn set_auto_exposure(&mut self, enabled: bool) {
        self.exposure.auto_exposure = enabled;
    }

    /// Update auto-exposure (call once per frame)
    pub fn update_auto_exposure(&mut self, avg_luminance: f32, delta_time: f32) {
        self.exposure
            .update_auto_exposure(avg_luminance, delta_time);
    }
}

/// HDR pixel data stored in CPU memory (for testing and CPU-side processing)
#[derive(Clone, Debug)]
pub struct HdrImageData {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixel data as RGBA floats (linear, HDR)
    pub pixels: Vec<[f32; 4]>,
}

impl HdrImageData {
    /// Create a new HDR image filled with black
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![[0.0, 0.0, 0.0, 1.0]; (width * height) as usize],
        }
    }

    /// Create from raw pixel data
    pub fn from_pixels(width: u32, height: u32, pixels: Vec<[f32; 4]>) -> Self {
        Self {
            width,
            height,
            pixels,
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

    /// Calculate the average luminance of the image
    pub fn average_luminance(&self) -> f32 {
        if self.pixels.is_empty() {
            return 0.0;
        }

        let mut sum = 0.0;
        for pixel in &self.pixels {
            // Use Rec. 709 luminance coefficients
            let lum = 0.2126 * pixel[0] + 0.7152 * pixel[1] + 0.0722 * pixel[2];
            // Use log-space averaging to handle HDR values better
            sum += lum.max(1e-6).ln();
        }

        (sum / self.pixels.len() as f32).exp()
    }

    /// Calculate the maximum luminance
    pub fn max_luminance(&self) -> f32 {
        self.pixels
            .iter()
            .map(|p| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2])
            .fold(0.0f32, f32::max)
    }

    /// Apply tone mapping to the entire image
    pub fn tonemap(&self, tonemapper: Tonemapper, exposure: f32) -> Self {
        let mut result = Self::new(self.width, self.height);
        for (i, pixel) in self.pixels.iter().enumerate() {
            let hdr = Vec3::new(pixel[0], pixel[1], pixel[2]) * exposure;
            let ldr = tonemapper.apply(hdr);
            result.pixels[i] = [ldr.x, ldr.y, ldr.z, pixel[3]];
        }
        result
    }

    /// Clear the image to black
    pub fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = [0.0, 0.0, 0.0, 1.0];
        }
    }

    /// Clear to a specific color
    pub fn clear_to(&mut self, color: [f32; 4]) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdr_format_default() {
        let format = HdrFormat::default();
        assert_eq!(format, HdrFormat::R10G10B10A2Unorm);
    }

    #[test]
    fn test_hdr_format_bits_per_pixel() {
        assert_eq!(HdrFormat::R16G16B16A16Float.bits_per_pixel(), 64);
        assert_eq!(HdrFormat::R32G32B32A32Float.bits_per_pixel(), 128);
        assert_eq!(HdrFormat::R11G11B10Float.bits_per_pixel(), 32);
        assert_eq!(HdrFormat::R10G10B10A2Unorm.bits_per_pixel(), 32);
    }

    #[test]
    fn test_hdr_format_has_alpha() {
        assert!(HdrFormat::R16G16B16A16Float.has_alpha());
        assert!(HdrFormat::R32G32B32A32Float.has_alpha());
        assert!(!HdrFormat::R11G11B10Float.has_alpha());
        assert!(HdrFormat::R10G10B10A2Unorm.has_alpha());
    }

    #[test]
    fn test_hdr_format_is_float() {
        assert!(HdrFormat::R16G16B16A16Float.is_float());
        assert!(HdrFormat::R32G32B32A32Float.is_float());
        assert!(HdrFormat::R11G11B10Float.is_float());
        assert!(!HdrFormat::R10G10B10A2Unorm.is_float());
    }

    #[test]
    fn test_exposure_settings_default() {
        let settings = ExposureSettings::default();
        assert_eq!(settings.exposure, 1.0);
        assert!(!settings.auto_exposure);
        assert!(settings.min_exposure < settings.max_exposure);
    }

    #[test]
    fn test_exposure_settings_new() {
        let settings = ExposureSettings::new(2.0);
        assert_eq!(settings.exposure, 2.0);
    }

    #[test]
    fn test_exposure_settings_auto() {
        let settings = ExposureSettings::auto();
        assert!(settings.auto_exposure);
    }

    #[test]
    fn test_apply_exposure() {
        let settings = ExposureSettings::new(2.0);
        let color = Vec3::new(0.5, 0.5, 0.5);
        let result = settings.apply_exposure(color);
        assert_eq!(result, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_update_auto_exposure_disabled() {
        let mut settings = ExposureSettings::default();
        let original = settings.exposure;
        settings.update_auto_exposure(0.5, 0.016);
        assert_eq!(settings.exposure, original); // No change when disabled
    }

    #[test]
    fn test_update_auto_exposure_enabled() {
        let mut settings = ExposureSettings::auto();
        settings.update_auto_exposure(0.1, 1.0); // Low luminance should increase exposure
        assert!(settings.exposure > 1.0); // Should adapt towards brighter
    }

    #[test]
    fn test_update_auto_exposure_clamped() {
        let mut settings = ExposureSettings::auto();
        settings.min_exposure = 0.5;
        settings.max_exposure = 2.0;
        // Very low luminance would push exposure very high, but should be clamped
        settings.update_auto_exposure(0.001, 1.0);
        assert!(settings.exposure <= settings.max_exposure);
    }

    #[test]
    fn test_hdr_framebuffer_default() {
        let fb = HdrFramebuffer::default();
        assert_eq!(fb.width, 1920);
        assert_eq!(fb.height, 1080);
        assert_eq!(fb.format, HdrFormat::R16G16B16A16Float);
        assert_eq!(fb.samples, 1);
    }

    #[test]
    fn test_hdr_framebuffer_new() {
        let fb = HdrFramebuffer::new(800, 600);
        assert_eq!(fb.width, 800);
        assert_eq!(fb.height, 600);
    }

    #[test]
    fn test_hdr_framebuffer_with_format() {
        let fb = HdrFramebuffer::with_format(1024, 768, HdrFormat::R11G11B10Float);
        assert_eq!(fb.width, 1024);
        assert_eq!(fb.height, 768);
        assert_eq!(fb.format, HdrFormat::R11G11B10Float);
    }

    #[test]
    fn test_hdr_framebuffer_resize() {
        let mut fb = HdrFramebuffer::new(800, 600);
        fb.resize(1920, 1080);
        assert_eq!(fb.width, 1920);
        assert_eq!(fb.height, 1080);
    }

    #[test]
    fn test_hdr_framebuffer_aspect_ratio() {
        let fb = HdrFramebuffer::new(1920, 1080);
        let ar = fb.aspect_ratio();
        assert!((ar - 16.0 / 9.0).abs() < 0.001);
    }

    #[test]
    fn test_hdr_framebuffer_aspect_ratio_zero_height() {
        let mut fb = HdrFramebuffer::new(1920, 1080);
        fb.height = 0;
        assert_eq!(fb.aspect_ratio(), 1.0);
    }

    #[test]
    fn test_hdr_framebuffer_pixel_count() {
        let fb = HdrFramebuffer::new(1920, 1080);
        assert_eq!(fb.pixel_count(), 1920 * 1080);
    }

    #[test]
    fn test_hdr_framebuffer_memory_size() {
        let fb = HdrFramebuffer::with_format(1024, 1024, HdrFormat::R16G16B16A16Float);
        // 1024 * 1024 * 64 bits / 8 = 8388608 bytes = 8 MB
        assert_eq!(fb.memory_size_bytes(), 8388608);
    }

    #[test]
    fn test_hdr_framebuffer_tonemap() {
        let fb = HdrFramebuffer::default();
        let hdr = Vec3::new(2.0, 2.0, 2.0); // HDR value > 1
        let ldr = fb.tonemap(hdr);
        // Should be in [0, 1] range after tone mapping
        assert!(ldr.x >= 0.0 && ldr.x <= 1.0);
        assert!(ldr.y >= 0.0 && ldr.y <= 1.0);
        assert!(ldr.z >= 0.0 && ldr.z <= 1.0);
    }

    #[test]
    fn test_hdr_framebuffer_tonemap_only() {
        let mut fb = HdrFramebuffer::default();
        fb.exposure.exposure = 2.0;
        let hdr = Vec3::new(1.0, 1.0, 1.0);
        let with_exposure = fb.tonemap(hdr);
        let without_exposure = fb.tonemap_only(hdr);
        // With exposure should be brighter (before tonemapping compresses)
        // Both should be in valid range
        assert!(with_exposure.x >= 0.0 && with_exposure.x <= 1.0);
        assert!(without_exposure.x >= 0.0 && without_exposure.x <= 1.0);
    }

    #[test]
    fn test_hdr_framebuffer_set_exposure() {
        let mut fb = HdrFramebuffer::default();
        fb.set_exposure(3.0);
        assert_eq!(fb.exposure(), 3.0);
    }

    #[test]
    fn test_hdr_framebuffer_set_tonemapper() {
        let mut fb = HdrFramebuffer::default();
        fb.set_tonemapper(Tonemapper::Reinhard);
        assert_eq!(fb.tonemapper, Tonemapper::Reinhard);
    }

    #[test]
    fn test_hdr_framebuffer_set_auto_exposure() {
        let mut fb = HdrFramebuffer::default();
        fb.set_auto_exposure(true);
        assert!(fb.exposure.auto_exposure);
    }

    #[test]
    fn test_hdr_framebuffer_update_auto_exposure() {
        let mut fb = HdrFramebuffer::default();
        fb.set_auto_exposure(true);
        fb.update_auto_exposure(0.1, 0.016);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_hdr_image_data_new() {
        let img = HdrImageData::new(4, 4);
        assert_eq!(img.width, 4);
        assert_eq!(img.height, 4);
        assert_eq!(img.pixels.len(), 16);
    }

    #[test]
    fn test_hdr_image_data_get_set() {
        let mut img = HdrImageData::new(4, 4);
        img.set(1, 2, [0.5, 0.6, 0.7, 1.0]);
        let pixel = img.get(1, 2);
        assert_eq!(pixel, [0.5, 0.6, 0.7, 1.0]);
    }

    #[test]
    fn test_hdr_image_data_get_out_of_bounds() {
        let img = HdrImageData::new(4, 4);
        let pixel = img.get(10, 10);
        assert_eq!(pixel, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_hdr_image_data_set_out_of_bounds() {
        let mut img = HdrImageData::new(4, 4);
        img.set(10, 10, [1.0, 1.0, 1.0, 1.0]);
        // Should not panic
    }

    #[test]
    fn test_hdr_image_data_average_luminance_black() {
        let img = HdrImageData::new(4, 4);
        let lum = img.average_luminance();
        assert!(lum >= 0.0);
    }

    #[test]
    fn test_hdr_image_data_average_luminance_white() {
        let mut img = HdrImageData::new(4, 4);
        img.clear_to([1.0, 1.0, 1.0, 1.0]);
        let lum = img.average_luminance();
        assert!(lum > 0.0);
        // White should have luminance near 1.0
        assert!((lum - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_hdr_image_data_max_luminance() {
        let mut img = HdrImageData::new(2, 2);
        img.set(0, 0, [0.5, 0.5, 0.5, 1.0]);
        img.set(1, 0, [2.0, 2.0, 2.0, 1.0]);
        img.set(0, 1, [0.1, 0.1, 0.1, 1.0]);
        img.set(1, 1, [1.0, 1.0, 1.0, 1.0]);
        let max_lum = img.max_luminance();
        assert!((max_lum - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_hdr_image_data_tonemap() {
        let mut img = HdrImageData::new(2, 2);
        img.set(0, 0, [2.0, 2.0, 2.0, 1.0]);
        img.set(1, 0, [0.5, 0.5, 0.5, 1.0]);
        img.set(0, 1, [1.0, 1.0, 1.0, 1.0]);
        img.set(1, 1, [3.0, 3.0, 3.0, 1.0]);

        let result = img.tonemap(Tonemapper::Aces, 1.0);
        // All values should be in [0, 1] after tonemapping
        for pixel in &result.pixels {
            assert!(pixel[0] >= 0.0 && pixel[0] <= 1.0);
            assert!(pixel[1] >= 0.0 && pixel[1] <= 1.0);
            assert!(pixel[2] >= 0.0 && pixel[2] <= 1.0);
        }
    }

    #[test]
    fn test_hdr_image_data_clear() {
        let mut img = HdrImageData::new(4, 4);
        img.clear_to([1.0, 1.0, 1.0, 1.0]);
        img.clear();
        assert_eq!(img.get(0, 0), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_hdr_image_data_clear_to() {
        let mut img = HdrImageData::new(4, 4);
        img.clear_to([0.5, 0.5, 0.5, 0.5]);
        assert_eq!(img.get(0, 0), [0.5, 0.5, 0.5, 0.5]);
        assert_eq!(img.get(3, 3), [0.5, 0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_hdr_image_data_from_pixels() {
        let pixels = vec![[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]];
        let img = HdrImageData::from_pixels(2, 1, pixels);
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 1);
        assert_eq!(img.get(0, 0), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(img.get(1, 0), [0.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_hdr_image_data_clone() {
        let img1 = HdrImageData::new(2, 2);
        let img2 = img1.clone();
        assert_eq!(img1.width, img2.width);
        assert_eq!(img1.height, img2.height);
    }

    #[test]
    fn test_hdr_framebuffer_clone() {
        let fb1 = HdrFramebuffer::default();
        let fb2 = fb1.clone();
        assert_eq!(fb1.width, fb2.width);
        assert_eq!(fb1.format, fb2.format);
    }

    #[test]
    fn test_exposure_settings_clone_copy() {
        let settings1 = ExposureSettings::new(2.0);
        let settings2 = settings1;
        let _settings3 = settings1;
        assert_eq!(settings2.exposure, 2.0);
    }

    #[test]
    fn test_hdr_format_clone_copy() {
        let format1 = HdrFormat::R16G16B16A16Float;
        let format2 = format1;
        assert_eq!(format1, format2);
    }
}
