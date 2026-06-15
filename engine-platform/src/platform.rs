#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Android,
    Ios,
    Web,
    Unknown,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        {
            Platform::Windows
        }
        #[cfg(target_os = "macos")]
        {
            Platform::MacOS
        }
        #[cfg(target_os = "linux")]
        {
            Platform::Linux
        }
        #[cfg(target_os = "android")]
        {
            Platform::Android
        }
        #[cfg(target_os = "ios")]
        {
            Platform::Ios
        }
        #[cfg(target_arch = "wasm32")]
        {
            Platform::Web
        }
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "ios",
            target_arch = "wasm32"
        )))]
        {
            Platform::Unknown
        }
    }

    pub fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }

    pub fn is_macos(&self) -> bool {
        matches!(self, Platform::MacOS)
    }

    pub fn is_linux(&self) -> bool {
        matches!(self, Platform::Linux)
    }

    pub fn is_web(&self) -> bool {
        matches!(self, Platform::Web)
    }

    pub fn is_android(&self) -> bool {
        matches!(self, Platform::Android)
    }

    pub fn is_ios(&self) -> bool {
        matches!(self, Platform::Ios)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::MacOS => "macOS",
            Platform::Linux => "Linux",
            Platform::Android => "Android",
            Platform::Ios => "iOS",
            Platform::Web => "Web",
            Platform::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_current() {
        let platform = Platform::current();
        assert!(matches!(platform, Platform::Linux | Platform::Windows | Platform::MacOS | Platform::Web));
    }

    #[test]
    fn platform_name() {
        assert_eq!(Platform::Windows.name(), "Windows");
        assert_eq!(Platform::MacOS.name(), "macOS");
        assert_eq!(Platform::Linux.name(), "Linux");
        assert_eq!(Platform::Web.name(), "Web");
        assert_eq!(Platform::Unknown.name(), "Unknown");
    }

    #[test]
    fn platform_is_*() {
        let windows = Platform::Windows;
        assert!(windows.is_windows());
        assert!(!windows.is_macos());
        assert!(!windows.is_linux());
        assert!(!windows.is_web());

        let linux = Platform::Linux;
        assert!(!linux.is_windows());
        assert!(!linux.is_macos());
        assert!(linux.is_linux());
        assert!(!linux.is_web());
    }
}
