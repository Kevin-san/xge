//! Game engine build pipeline and asset processing library
//!
//! Provides cross-platform build pipeline, asset processing, hot update,
//! and build configuration management.
//!
//! # Modules
//!
//! - [`config`] - Build configuration, profiles, platform targets
//! - [`pipeline`] - Build pipeline and artifacts
//! - [`asset`] - Asset pipeline, manifest, and processing
//! - [`hot_update`] - Hot update and differential patching
//! - [`cache`] - Build cache for incremental builds
//! - [`hash`] - Hash computation utilities
//! - [`compress`] - Compression algorithms
//! - [`logger`] - Build logging and reporting
//! - [`error`] - Error types

mod asset;
mod cache;
mod compress;
mod config;
mod error;
mod hash;
mod hot_update;
mod logger;
mod pipeline;

pub use asset::{
    AssetCompress, AssetEncrypt, AssetEntry, AssetKind, AssetManifest, AssetPipeline, DiffResult,
};
pub use cache::BuildCache;
pub use compress::{Compress, Encrypt};
pub use config::{BuildConfig, MiniAppPlatform, Orientation, Permission, PlatformTarget, Profile};
pub use error::{BuildError, BuildResult};
pub use hash::Hash;
pub use hot_update::{FileChange, HotUpdate, HotUpdatePatch};
pub use logger::{BuildLogger, BuildProgress, BuildReport, BuildStage};
pub use pipeline::{BuildArtifact, BuildPipeline, Package, PackageFormat};
