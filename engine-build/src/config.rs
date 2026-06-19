//! Build configuration types
//!
//! Provides configuration for build pipeline including platform targets,
//! profiles, permissions, and build settings.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Platform target for building
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlatformTarget {
    Windows,
    MacOS,
    Linux,
    Android,
    Ios,
    Web,
    MiniApp(MiniAppPlatform),
}

impl PlatformTarget {
    /// Get current host platform
    pub fn current() -> PlatformTarget {
        #[cfg(target_os = "windows")]
        {
            PlatformTarget::Windows
        }
        #[cfg(target_os = "macos")]
        {
            PlatformTarget::MacOS
        }
        #[cfg(target_os = "linux")]
        {
            PlatformTarget::Linux
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            PlatformTarget::Web
        }
    }

    /// Check if this target can be built on current host
    pub fn supported(&self) -> bool {
        let current = PlatformTarget::current();
        match self {
            PlatformTarget::Windows => {
                current == PlatformTarget::Windows || current == PlatformTarget::Linux
            }
            PlatformTarget::MacOS => current == PlatformTarget::MacOS,
            PlatformTarget::Linux => current == PlatformTarget::Linux,
            PlatformTarget::Android => {
                current == PlatformTarget::Linux
                    || current == PlatformTarget::Windows
                    || current == PlatformTarget::MacOS
            }
            PlatformTarget::Ios => current == PlatformTarget::MacOS,
            PlatformTarget::Web => true,
            PlatformTarget::MiniApp(_) => true,
        }
    }

    /// Get target triple for Rust compilation
    pub fn target_triple(&self) -> &'static str {
        match self {
            PlatformTarget::Windows => "x86_64-pc-windows-msvc",
            PlatformTarget::MacOS => "x86_64-apple-darwin",
            PlatformTarget::Linux => "x86_64-unknown-linux-gnu",
            PlatformTarget::Android => "aarch64-linux-android",
            PlatformTarget::Ios => "aarch64-apple-ios",
            PlatformTarget::Web => "wasm32-unknown-unknown",
            PlatformTarget::MiniApp(_) => "wasm32-unknown-unknown",
        }
    }
}

impl Default for PlatformTarget {
    fn default() -> Self {
        PlatformTarget::current()
    }
}

/// Mini app platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MiniAppPlatform {
    WeChat,
    ByteDance,
    QQ,
}

/// Build profile configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    #[default]
    Debug,
    Release,
    Ship,
}

impl Profile {
    /// Get optimization level (0-3)
    pub fn optimization_level(&self) -> u8 {
        match self {
            Profile::Debug => 0,
            Profile::Release => 2,
            Profile::Ship => 3,
        }
    }

    /// Get optimization level string for Cargo
    pub fn opt_level(&self) -> String {
        self.optimization_level().to_string()
    }

    /// Check if debug info should be included
    pub fn debug_info(&self) -> bool {
        match self {
            Profile::Debug => true,
            Profile::Release | Profile::Ship => false,
        }
    }

    /// Alias for debug_info
    pub fn debug(&self) -> bool {
        self.debug_info()
    }

    /// Check if symbols should be stripped
    pub fn strip_symbols(&self) -> bool {
        match self {
            Profile::Debug => false,
            Profile::Release | Profile::Ship => true,
        }
    }

    /// Alias for strip_symbols
    pub fn strip(&self) -> bool {
        self.strip_symbols()
    }

    /// Check if LTO should be enabled
    pub fn lto(&self) -> bool {
        match self {
            Profile::Debug | Profile::Release => false,
            Profile::Ship => true,
        }
    }

    /// Get Cargo build arguments
    pub fn cargo_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        match self {
            Profile::Debug => args.push("--debug".to_string()),
            Profile::Release => args.push("--release".to_string()),
            Profile::Ship => {
                args.push("--release".to_string());
                args.push("--config".to_string());
                args.push("profile.release.lto=true".to_string());
            }
        }
        args
    }
}

/// Screen orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
    Auto,
}

/// Permission type for app capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Internet,
    Storage,
    Camera,
    Microphone,
    Location,
    Bluetooth,
    NFC,
}

impl Permission {
    /// Convert to Android permission string
    pub fn to_android_string(&self) -> &'static str {
        match self {
            Permission::Internet => "android.permission.INTERNET",
            Permission::Storage => "android.permission.READ_EXTERNAL_STORAGE",
            Permission::Camera => "android.permission.CAMERA",
            Permission::Microphone => "android.permission.RECORD_AUDIO",
            Permission::Location => "android.permission.ACCESS_FINE_LOCATION",
            Permission::Bluetooth => "android.permission.BLUETOOTH",
            Permission::NFC => "android.permission.NFC",
        }
    }

    /// Convert to iOS permission string
    pub fn to_ios_string(&self) -> &'static str {
        match self {
            Permission::Internet => "NSInternetPermission",
            Permission::Storage => "NSPhotoLibraryUsageDescription",
            Permission::Camera => "NSCameraUsageDescription",
            Permission::Microphone => "NSMicrophoneUsageDescription",
            Permission::Location => "NSLocationWhenInUseUsageDescription",
            Permission::Bluetooth => "NSBluetoothAlwaysUsageDescription",
            Permission::NFC => "NSNFCUsageDescription",
        }
    }
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Application name
    pub app_name: String,
    /// Application identifier (e.g., com.example.myapp)
    pub app_id: String,
    /// Version string (e.g., 1.0.0)
    pub version: String,
    /// Version code (integer)
    pub version_code: i32,
    /// Icon paths
    pub icons: Vec<PathBuf>,
    /// Splash screen path
    pub splash_screen: Option<PathBuf>,
    /// Required permissions
    pub permissions: Vec<Permission>,
    /// Screen orientation
    pub orientation: Orientation,
    /// Target platform
    pub platform_target: PlatformTarget,
    /// Build profile
    pub profile: Profile,
    /// Assets directory
    pub assets_dir: PathBuf,
    /// Output directory
    pub output_dir: PathBuf,
    /// Temporary directory
    pub temp_dir: PathBuf,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            app_name: "MyApp".to_string(),
            app_id: "com.example.myapp".to_string(),
            version: "1.0.0".to_string(),
            version_code: 1,
            icons: Vec::new(),
            splash_screen: None,
            permissions: Vec::new(),
            orientation: Orientation::default(),
            platform_target: PlatformTarget::default(),
            profile: Profile::default(),
            assets_dir: PathBuf::from("assets"),
            output_dir: PathBuf::from("build"),
            temp_dir: PathBuf::from("build/temp"),
        }
    }
}

impl BuildConfig {
    /// Create new build config
    pub fn new() -> Self {
        Self::default()
    }

    /// Get application name
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Get application ID
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get version string
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get version code
    pub fn version_code(&self) -> i32 {
        self.version_code
    }

    /// Get icon paths
    pub fn icons(&self) -> &[PathBuf] {
        &self.icons
    }

    /// Get splash screen path
    pub fn splash(&self) -> Option<&PathBuf> {
        self.splash_screen.as_ref()
    }

    /// Get permissions
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Get orientation
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Get output directory
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    /// Get temp directory
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Get assets directory
    pub fn assets_dir(&self) -> &Path {
        &self.assets_dir
    }

    /// Get platform target
    pub fn platform_target(&self) -> PlatformTarget {
        self.platform_target
    }

    /// Get profile
    pub fn profile(&self) -> Profile {
        self.profile
    }

    /// Load from TOML file
    pub fn from_toml(path: impl AsRef<Path>) -> crate::BuildResult<Self> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let config: BuildConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Alias for from_toml
    pub fn from_file(path: impl AsRef<Path>) -> crate::BuildResult<Self> {
        Self::from_toml(path)
    }

    /// Serialize to TOML string
    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_default()
    }

    /// Save to TOML file
    pub fn save(&self, path: impl AsRef<Path>) -> crate::BuildResult<()> {
        let content = self.to_toml();
        std::fs::write(path.as_ref(), content)?;
        Ok(())
    }

    /// Set assets directory (builder pattern)
    pub fn with_assets_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.assets_dir = dir.as_ref().to_path_buf();
        self
    }

    /// Set output directory (builder pattern)
    pub fn with_output_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.output_dir = dir.as_ref().to_path_buf();
        self
    }

    /// Set temp directory (builder pattern)
    pub fn with_temp_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.temp_dir = dir.as_ref().to_path_buf();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_current() {
        let current = PlatformTarget::current();
        assert!(current.supported());
    }

    #[test]
    fn test_platform_target_triple() {
        assert_eq!(PlatformTarget::Windows.target_triple(), "x86_64-pc-windows-msvc");
        assert_eq!(PlatformTarget::MacOS.target_triple(), "x86_64-apple-darwin");
        assert_eq!(PlatformTarget::Linux.target_triple(), "x86_64-unknown-linux-gnu");
        assert_eq!(PlatformTarget::Web.target_triple(), "wasm32-unknown-unknown");
    }

    #[test]
    fn test_platform_target_debug() {
        let s = format!("{:?}", PlatformTarget::Windows);
        assert!(s.contains("Windows"));
    }

    #[test]
    fn test_miniapp_platform_debug() {
        let s = format!("{:?}", MiniAppPlatform::WeChat);
        assert!(s.contains("WeChat"));
    }

    #[test]
    fn test_profile_optimization() {
        assert_eq!(Profile::Debug.optimization_level(), 0);
        assert_eq!(Profile::Release.optimization_level(), 2);
        assert_eq!(Profile::Ship.optimization_level(), 3);
    }

    #[test]
    fn test_profile_opt_level_string() {
        assert_eq!(Profile::Debug.opt_level(), "0");
        assert_eq!(Profile::Release.opt_level(), "2");
        assert_eq!(Profile::Ship.opt_level(), "3");
    }

    #[test]
    fn test_profile_debug_info() {
        assert!(Profile::Debug.debug_info());
        assert!(!Profile::Release.debug_info());
        assert!(!Profile::Ship.debug_info());
    }

    #[test]
    fn test_profile_strip_symbols() {
        assert!(!Profile::Debug.strip_symbols());
        assert!(Profile::Release.strip_symbols());
        assert!(Profile::Ship.strip_symbols());
    }

    #[test]
    fn test_profile_lto() {
        assert!(!Profile::Debug.lto());
        assert!(!Profile::Release.lto());
        assert!(Profile::Ship.lto());
    }

    #[test]
    fn test_profile_cargo_args() {
        let debug_args = Profile::Debug.cargo_args();
        assert!(debug_args.iter().any(|a| a.contains("debug")));
        let release_args = Profile::Release.cargo_args();
        assert!(release_args.iter().any(|a| a.contains("release")));
        let ship_args = Profile::Ship.cargo_args();
        assert!(ship_args.iter().any(|a| a.contains("lto")));
    }

    #[test]
    fn test_profile_debug() {
        let s = format!("{:?}", Profile::Release);
        assert!(s.contains("Release"));
    }

    #[test]
    fn test_profile_default() {
        let p: Profile = Default::default();
        assert_eq!(p, Profile::Debug);
    }

    #[test]
    fn test_orientation_default() {
        let o: Orientation = Default::default();
        assert_eq!(o, Orientation::Portrait);
    }

    #[test]
    fn test_orientation_debug() {
        let s = format!("{:?} {:?}", Orientation::Portrait, Orientation::Landscape);
        assert!(s.contains("Portrait"));
        assert!(s.contains("Landscape"));
    }

    #[test]
    fn test_permission_android() {
        assert_eq!(
            Permission::Internet.to_android_string(),
            "android.permission.INTERNET"
        );
        assert_eq!(
            Permission::Storage.to_android_string(),
            "android.permission.READ_EXTERNAL_STORAGE"
        );
        assert_eq!(
            Permission::Camera.to_android_string(),
            "android.permission.CAMERA"
        );
        assert_eq!(
            Permission::Microphone.to_android_string(),
            "android.permission.RECORD_AUDIO"
        );
        assert_eq!(
            Permission::Location.to_android_string(),
            "android.permission.ACCESS_FINE_LOCATION"
        );
        assert_eq!(
            Permission::Bluetooth.to_android_string(),
            "android.permission.BLUETOOTH"
        );
        assert_eq!(Permission::NFC.to_android_string(), "android.permission.NFC");
    }

    #[test]
    fn test_permission_ios() {
        assert_eq!(
            Permission::Internet.to_ios_string(),
            "NSInternetPermission"
        );
        assert_eq!(
            Permission::Camera.to_ios_string(),
            "NSCameraUsageDescription"
        );
        assert_eq!(
            Permission::Location.to_ios_string(),
            "NSLocationWhenInUseUsageDescription"
        );
        assert_eq!(
            Permission::Bluetooth.to_ios_string(),
            "NSBluetoothAlwaysUsageDescription"
        );
    }

    #[test]
    fn test_permission_debug_clone() {
        let p = Permission::Internet;
        let p2 = p;
        assert_eq!(p, p2);
        let s = format!("{:?}", p);
        assert!(s.contains("Internet"));
    }

    #[test]
    fn test_build_config_default() {
        let config = BuildConfig::default();
        assert_eq!(config.app_name(), "MyApp");
        assert_eq!(config.version(), "1.0.0");
        assert_eq!(config.assets_dir(), std::path::Path::new("assets"));
        assert_eq!(config.output_dir(), std::path::Path::new("build"));
        assert_eq!(config.temp_dir(), std::path::Path::new("build/temp"));
    }

    #[test]
    fn test_build_config_builder() {
        let config = BuildConfig::new()
            .with_assets_dir("my_assets")
            .with_output_dir("my_output")
            .with_temp_dir("my_temp");
        assert_eq!(config.assets_dir(), Path::new("my_assets"));
        assert_eq!(config.output_dir(), Path::new("my_output"));
        assert_eq!(config.temp_dir(), Path::new("my_temp"));
    }

    #[test]
    fn test_build_config_debug_fields() {
        let config = BuildConfig::new();
        let s = format!("{:?}", config);
        assert!(s.contains("BuildConfig"));
    }

    #[test]
    fn test_build_config_version_code() {
        let config = BuildConfig::new();
        assert_eq!(config.version_code(), 1);
    }

    #[test]
    fn test_build_config_permissions_default() {
        let config = BuildConfig::new();
        assert!(config.permissions().is_empty());
    }

    #[test]
    fn test_build_config_icons_default() {
        let config = BuildConfig::new();
        assert!(config.icons().is_empty());
    }

    #[test]
    fn test_build_config_splash_default() {
        let config = BuildConfig::new();
        assert!(config.splash().is_none());
    }

    #[test]
    fn test_build_config_profile_default() {
        let config = BuildConfig::new();
        assert_eq!(config.profile(), Profile::Debug);
    }

    #[test]
    fn test_build_config_toml_roundtrip() {
        let config = BuildConfig::default();
        let toml_str = config.to_toml();
        let parsed: BuildConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.app_name, parsed.app_name);
        assert_eq!(config.version, parsed.version);
    }
}
