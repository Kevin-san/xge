/// 平台类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Android,
    IOS,
    Web,
    Unknown,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        {
            Platform::Windows
        }

        #[cfg(target_os = "linux")]
        {
            Platform::Linux
        }

        #[cfg(target_os = "macos")]
        {
            Platform::MacOS
        }

        #[cfg(target_os = "android")]
        {
            Platform::Android
        }

        #[cfg(target_os = "ios")]
        {
            Platform::IOS
        }

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            Platform::Web
        }

        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "android",
            target_os = "ios",
            all(target_arch = "wasm32", target_os = "unknown")
        )))]
        {
            Platform::Unknown
        }
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, Platform::Windows | Platform::Linux | Platform::MacOS)
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Android | Platform::IOS)
    }

    pub fn is_web(&self) -> bool {
        matches!(self, Platform::Web)
    }

    pub fn is_windows(&self) -> bool {
        *self == Platform::Windows
    }

    pub fn is_macos(&self) -> bool {
        *self == Platform::MacOS
    }

    pub fn is_linux(&self) -> bool {
        *self == Platform::Linux
    }

    pub fn name(&self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::Linux => "linux",
            Platform::MacOS => "macos",
            Platform::Android => "android",
            Platform::IOS => "ios",
            Platform::Web => "web",
            Platform::Unknown => "unknown",
        }
    }
}

/// 特性开关
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Feature {
    name: &'static str,
    enabled: bool,
}

impl Feature {
    pub fn new(name: &'static str, enabled: bool) -> Self {
        Self { name, enabled }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if a feature with given name is enabled from a feature list
    pub fn enabled(name: &str, features: &[Feature]) -> bool {
        features.iter().any(|f| f.name == name && f.enabled)
    }

    /// List all feature names
    pub fn list(features: &[Feature]) -> Vec<&'static str> {
        features.iter().map(|f| f.name).collect()
    }

    /// Get render backend from feature list
    pub fn render_backend(features: &[Feature]) -> &'static str {
        features
            .iter()
            .find(|f| f.name.starts_with("render-") && f.enabled)
            .map(|f| f.name)
            .unwrap_or("render-gl")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_current() {
        let platform = Platform::current();
        // 在任何平台上运行此测试都应该返回一个有效平台
        assert_ne!(platform, Platform::Unknown);
    }

    #[test]
    fn test_platform_categories() {
        // 测试分类方法
        assert!(Platform::Windows.is_desktop());
        assert!(Platform::Linux.is_desktop());
        assert!(Platform::MacOS.is_desktop());

        assert!(Platform::Android.is_mobile());
        assert!(Platform::IOS.is_mobile());

        assert!(Platform::Web.is_web());
    }

    #[test]
    fn test_feature() {
        let mut f = Feature::new("test_feature", false);
        assert!(!f.is_enabled());

        f.enable();
        assert!(f.is_enabled());

        f.disable();
        assert!(!f.is_enabled());
    }

    #[test]
    fn test_platform_is_specific() {
        assert!(Platform::Windows.is_windows());
        assert!(!Platform::Linux.is_windows());
        assert!(!Platform::MacOS.is_windows());

        assert!(Platform::MacOS.is_macos());
        assert!(!Platform::Windows.is_macos());
        assert!(!Platform::Linux.is_macos());

        assert!(Platform::Linux.is_linux());
        assert!(!Platform::Windows.is_linux());
        assert!(!Platform::MacOS.is_linux());
    }

    #[test]
    fn test_platform_name() {
        assert_eq!(Platform::Windows.name(), "windows");
        assert_eq!(Platform::Linux.name(), "linux");
        assert_eq!(Platform::MacOS.name(), "macos");
        assert_eq!(Platform::Android.name(), "android");
        assert_eq!(Platform::IOS.name(), "ios");
        assert_eq!(Platform::Web.name(), "web");
        assert_eq!(Platform::Unknown.name(), "unknown");
    }

    #[test]
    fn test_feature_enabled() {
        let features = vec![
            Feature::new("render-gl", true),
            Feature::new("audio", false),
            Feature::new("network", true),
        ];

        assert!(Feature::enabled("render-gl", &features));
        assert!(!Feature::enabled("audio", &features));
        assert!(Feature::enabled("network", &features));
        assert!(!Feature::enabled("nonexistent", &features));
    }

    #[test]
    fn test_feature_list() {
        let features = vec![
            Feature::new("render-gl", true),
            Feature::new("audio", false),
            Feature::new("network", true),
        ];

        let names = Feature::list(&features);
        assert_eq!(names, vec!["render-gl", "audio", "network"]);
    }

    #[test]
    fn test_feature_render_backend() {
        let features = vec![
            Feature::new("render-gl", true),
            Feature::new("audio", false),
        ];
        assert_eq!(Feature::render_backend(&features), "render-gl");

        let features = vec![
            Feature::new("render-vulkan", true),
            Feature::new("render-gl", false),
        ];
        assert_eq!(Feature::render_backend(&features), "render-vulkan");

        // No enabled render feature → fallback
        let features = vec![
            Feature::new("render-gl", false),
            Feature::new("audio", true),
        ];
        assert_eq!(Feature::render_backend(&features), "render-gl");

        // Empty list → fallback
        let features: Vec<Feature> = vec![];
        assert_eq!(Feature::render_backend(&features), "render-gl");
    }
}
