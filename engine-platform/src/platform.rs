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
        { Platform::Windows }

        #[cfg(target_os = "linux")]
        { Platform::Linux }

        #[cfg(target_os = "macos")]
        { Platform::MacOS }

        #[cfg(target_os = "android")]
        { Platform::Android }

        #[cfg(target_os = "ios")]
        { Platform::IOS }

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        { Platform::Web }

        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "android",
            target_os = "ios",
            all(target_arch = "wasm32", target_os = "unknown")
        )))]
        { Platform::Unknown }
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
}