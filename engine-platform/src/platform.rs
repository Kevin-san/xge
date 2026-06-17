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
        matches!(self, Platform::Windows)
    }

    pub fn is_macos(&self) -> bool {
        matches!(self, Platform::MacOS)
    }

    pub fn is_linux(&self) -> bool {
        matches!(self, Platform::Linux)
    }

    pub fn is_android(&self) -> bool {
        matches!(self, Platform::Android)
    }

    pub fn is_ios(&self) -> bool {
        matches!(self, Platform::IOS)
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

#[macro_export]
macro_rules! target_os_cfg {
    ($($os:tt => $block:expr),* $(,)?) => {{
        $(
            #[cfg(target_os = $os)]
            { $block }
        )*
        #[cfg(not(any($(target_os = $os),*)))]
        {
            #[cfg(target_os = "windows")]
            { $block }
            #[cfg(target_os = "linux")]
            { $block }
            #[cfg(target_os = "macos")]
            { $block }
            #[cfg(target_os = "android")]
            { $block }
            #[cfg(target_os = "ios")]
            { $block }
            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            { $block }
        }
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderBackend {
    OpenGL,
    Vulkan,
    WebGPU,
    Metal,
    D3D11,
    D3D12,
    Unknown,
}

impl RenderBackend {
    pub fn name(&self) -> &'static str {
        match self {
            RenderBackend::OpenGL => "opengl",
            RenderBackend::Vulkan => "vulkan",
            RenderBackend::WebGPU => "webgpu",
            RenderBackend::Metal => "metal",
            RenderBackend::D3D11 => "d3d11",
            RenderBackend::D3D12 => "d3d12",
            RenderBackend::Unknown => "unknown",
        }
    }
}

pub struct Feature;

impl Feature {
    pub fn enabled(name: &str) -> bool {
        match name {
            "render-vulkan" => cfg!(feature = "render-vulkan"),
            "render-gl" => cfg!(feature = "render-gl"),
            "render-webgpu" => cfg!(feature = "render-webgpu"),
            "render-metal" => cfg!(feature = "render-metal"),
            "audio" => cfg!(feature = "audio"),
            "network" => cfg!(feature = "network"),
            "editor" => cfg!(feature = "editor"),
            _ => false,
        }
    }

    pub fn list() -> Vec<&'static str> {
        let mut features = Vec::new();
        if cfg!(feature = "render-vulkan") {
            features.push("render-vulkan");
        }
        if cfg!(feature = "render-gl") {
            features.push("render-gl");
        }
        if cfg!(feature = "render-webgpu") {
            features.push("render-webgpu");
        }
        if cfg!(feature = "render-metal") {
            features.push("render-metal");
        }
        if cfg!(feature = "audio") {
            features.push("audio");
        }
        if cfg!(feature = "network") {
            features.push("network");
        }
        if cfg!(feature = "editor") {
            features.push("editor");
        }
        features
    }

    pub fn render_backend() -> RenderBackend {
        #[cfg(feature = "render-vulkan")]
        {
            RenderBackend::Vulkan
        }
        #[cfg(feature = "render-metal")]
        {
            RenderBackend::Metal
        }
        #[cfg(feature = "render-webgpu")]
        {
            RenderBackend::WebGPU
        }
        #[cfg(feature = "render-gl")]
        {
            RenderBackend::OpenGL
        }
        #[cfg(not(any(
            feature = "render-vulkan",
            feature = "render-metal",
            feature = "render-webgpu",
            feature = "render-gl"
        )))]
        {
            #[cfg(target_os = "windows")]
            {
                RenderBackend::D3D11
            }
            #[cfg(target_os = "macos")]
            {
                RenderBackend::Metal
            }
            #[cfg(target_os = "linux")]
            {
                RenderBackend::OpenGL
            }
            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            {
                RenderBackend::WebGPU
            }
            #[cfg(not(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "linux",
                all(target_arch = "wasm32", target_os = "unknown")
            )))]
            {
                RenderBackend::Unknown
            }
        }
    }

    pub fn render_backend_name() -> &'static str {
        Self::render_backend().name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_current() {
        let platform = Platform::current();
        assert_ne!(platform, Platform::Unknown);
    }

    #[test]
    fn test_platform_categories() {
        assert!(Platform::Windows.is_desktop());
        assert!(Platform::Linux.is_desktop());
        assert!(Platform::MacOS.is_desktop());

        assert!(Platform::Android.is_mobile());
        assert!(Platform::IOS.is_mobile());

        assert!(Platform::Web.is_web());
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
    fn test_platform_specific_checks() {
        assert!(Platform::Windows.is_windows());
        assert!(!Platform::Windows.is_linux());
        assert!(!Platform::Windows.is_macos());

        assert!(Platform::Linux.is_linux());
        assert!(!Platform::Linux.is_windows());

        assert!(Platform::Web.is_web());
        assert!(!Platform::Web.is_desktop());
    }

    #[test]
    fn test_feature_enabled() {
        let _ = Feature::enabled("render-gl");
        let _ = Feature::enabled("audio");
    }

    #[test]
    fn test_feature_list() {
        let features = Feature::list();
        assert!(features.len() >= 0);
    }

    #[test]
    fn test_feature_render_backend() {
        let backend = Feature::render_backend();
        let name = Feature::render_backend_name();
        assert_eq!(backend.name(), name);
    }

    #[test]
    fn test_render_backend_name() {
        assert_eq!(RenderBackend::OpenGL.name(), "opengl");
        assert_eq!(RenderBackend::Vulkan.name(), "vulkan");
        assert_eq!(RenderBackend::WebGPU.name(), "webgpu");
        assert_eq!(RenderBackend::Metal.name(), "metal");
        assert_eq!(RenderBackend::D3D11.name(), "d3d11");
        assert_eq!(RenderBackend::D3D12.name(), "d3d12");
        assert_eq!(RenderBackend::Unknown.name(), "unknown");
    }
}