//! Error types for build pipeline
//!
//! Provides comprehensive error handling for build operations.

use std::io;
use std::path::{PathBuf, StripPrefixError};
use thiserror::Error;

/// Build error types
#[derive(Debug, Error)]
pub enum BuildError {
    /// IO error
    #[error("IO error: {0}")]
    Io(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),

    /// Crypto error
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Signature verification error
    #[error("Signature error: {0}")]
    Signature(String),

    /// Unsupported platform
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),

    /// Asset not found
    #[error("Asset not found: {0}")]
    AssetNotFound(PathBuf),

    /// Build failed
    #[error("Build failed at stage {stage}: {message}")]
    BuildFailed {
        stage: String,
        message: String,
        file: Option<PathBuf>,
    },

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Path error
    #[error("Path error: {0}")]
    Path(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> Self {
        BuildError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for BuildError {
    fn from(err: serde_json::Error) -> Self {
        BuildError::Parse(err.to_string())
    }
}

impl From<toml::de::Error> for BuildError {
    fn from(err: toml::de::Error) -> Self {
        BuildError::Parse(err.to_string())
    }
}

impl From<walkdir::Error> for BuildError {
    fn from(err: walkdir::Error) -> Self {
        BuildError::Io(err.to_string())
    }
}

impl From<StripPrefixError> for BuildError {
    fn from(err: StripPrefixError) -> Self {
        BuildError::Path(err.to_string())
    }
}

impl BuildError {
    /// Create IO error
    pub fn io_error(msg: String) -> Self {
        BuildError::Io(msg)
    }

    /// Create parse error
    pub fn parse_error(msg: String) -> Self {
        BuildError::Parse(msg)
    }

    /// Create crypto error
    pub fn crypto_error(msg: String) -> Self {
        BuildError::Crypto(msg)
    }

    /// Create signature error
    pub fn signature_error(msg: String) -> Self {
        BuildError::Signature(msg)
    }

    /// Create unsupported platform error
    pub fn unsupported_platform(platform: crate::PlatformTarget) -> Self {
        BuildError::UnsupportedPlatform(format!("{:?}", platform))
    }

    /// Create asset not found error
    pub fn asset_not_found(path: PathBuf) -> Self {
        BuildError::AssetNotFound(path)
    }

    /// Create build failed error
    pub fn build_failed(stage: &str, message: &str, file: Option<PathBuf>) -> Self {
        BuildError::BuildFailed {
            stage: stage.to_string(),
            message: message.to_string(),
            file,
        }
    }

    /// Create config error
    pub fn config_error(msg: String) -> Self {
        BuildError::Config(msg)
    }

    /// Check if IO error
    pub fn is_io(&self) -> bool {
        matches!(self, BuildError::Io(_))
    }

    /// Check if parse error
    pub fn is_parse(&self) -> bool {
        matches!(self, BuildError::Parse(_))
    }

    /// Get error code
    pub fn code(&self) -> &'static str {
        match self {
            BuildError::Io(_) => "IO_001",
            BuildError::Parse(_) => "PARSE_001",
            BuildError::Crypto(_) => "CRYPTO_001",
            BuildError::Signature(_) => "SIG_001",
            BuildError::UnsupportedPlatform(_) => "PLATFORM_001",
            BuildError::AssetNotFound(_) => "ASSET_001",
            BuildError::BuildFailed { .. } => "BUILD_001",
            BuildError::Config(_) => "CONFIG_001",
            BuildError::Path(_) => "PATH_001",
            BuildError::Other(_) => "OTHER_001",
        }
    }
}

/// Build result type
pub type BuildResult<T> = std::result::Result<T, BuildError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_error_io() {
        let err = BuildError::io_error("test io error".to_string());
        assert!(err.is_io());
        assert_eq!(err.code(), "IO_001");
    }

    #[test]
    fn test_build_error_parse() {
        let err = BuildError::parse_error("test parse error".to_string());
        assert!(err.is_parse());
        assert_eq!(err.code(), "PARSE_001");
    }

    #[test]
    fn test_build_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let build_err: BuildError = io_err.into();
        assert!(build_err.is_io());
        assert_eq!(build_err.code(), "IO_001");
    }

    #[test]
    fn test_build_error_unsupported_platform() {
        let err = BuildError::unsupported_platform(crate::PlatformTarget::Android);
        assert_eq!(err.code(), "PLATFORM_001");
    }

    #[test]
    fn test_build_error_asset_not_found() {
        let err = BuildError::asset_not_found(PathBuf::from("test.png"));
        assert_eq!(err.code(), "ASSET_001");
    }

    #[test]
    fn test_build_error_crypto() {
        let err = BuildError::crypto_error("crypto failed".to_string());
        assert_eq!(err.code(), "CRYPTO_001");
    }

    #[test]
    fn test_build_error_signature() {
        let err = BuildError::signature_error("bad signature".to_string());
        assert_eq!(err.code(), "SIG_001");
    }

    #[test]
    fn test_build_error_build_failed() {
        let err = BuildError::build_failed("compile", "type error", None);
        assert_eq!(err.code(), "BUILD_001");
    }

    #[test]
    fn test_build_error_config() {
        let err = BuildError::config_error("missing field".to_string());
        assert_eq!(err.code(), "CONFIG_001");
    }

    #[test]
    fn test_build_error_path() {
        let err = BuildError::Path("invalid path".to_string());
        assert_eq!(err.code(), "PATH_001");
    }

    #[test]
    fn test_build_error_other() {
        let err = BuildError::Other("generic".to_string());
        assert_eq!(err.code(), "OTHER_001");
    }

    #[test]
    fn test_build_error_debug_display() {
        let err = BuildError::io_error("io issue".to_string());
        let debug_s = format!("{:?}", err);
        assert!(debug_s.contains("Io"));
        let display_s = format!("{}", err);
        assert!(display_s.contains("IO error"));
    }

    #[test]
    fn test_build_error_from_strip_prefix() {
        use std::path::Path;
        // 触发 strip_prefix 错误
        let p1 = Path::new("/a/b");
        let p2 = Path::new("/c/d");
        if let Err(e) = p1.strip_prefix(p2) {
            let be: BuildError = e.into();
            assert_eq!(be.code(), "PATH_001");
        }
    }
}
