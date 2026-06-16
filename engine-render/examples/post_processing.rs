//! Post-processing module - Screen-space effects pipeline
//!
//! Provides post-processing effects including:
//! - Bloom/Glow
//! - Vignette
//! - Chromatic aberration
//! - Color grading

/// Post-processing pipeline configuration
#[derive(Debug, Clone)]
pub struct PostProcessConfig {
    /// Bloom intensity (0.0 - 1.0)
    pub bloom_intensity: f32,
    /// Bloom threshold
    pub bloom_threshold: f32,
    /// Vignette intensity (0.0 - 1.0)
    pub vignette_intensity: f32,
    /// Vignette radius
    pub vignette_radius: f32,
    /// Chromatic aberration offset
    pub chromatic_offset: f32,
    /// Color saturation adjustment
    pub saturation: f32,
    /// Contrast adjustment
    pub contrast: f32,
    /// Brightness adjustment
    pub brightness: f32,
}

impl Default for PostProcessConfig {
    fn default() -> Self {
        Self {
            bloom_intensity: 0.3,
            bloom_threshold: 0.8,
            vignette_intensity: 0.4,
            vignette_radius: 0.9,
            chromatic_offset: 0.003,
            saturation: 1.0,
            contrast: 1.0,
            brightness: 1.0,
        }
    }
}

/// Post-processing pipeline
pub struct PostProcessingPipeline {
    config: PostProcessConfig,
}

impl PostProcessingPipeline {
    /// Create new post-processing pipeline
    pub fn new() -> Self {
        Self {
            config: PostProcessConfig::default(),
        }
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: PostProcessConfig) {
        self.config = config;
    }
    
    /// Get configuration
    pub fn config(&self) -> &PostProcessConfig {
        &self.config
    }
}

impl Default for PostProcessingPipeline {
    fn default() -> Self {
        Self::new()
    }
}
