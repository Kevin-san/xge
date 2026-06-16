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

mod config;
mod pipeline;
mod asset;
mod hot_update;
mod cache;
mod hash;
mod compress;
mod logger;
mod error;

pub use config::{
    BuildConfig, MiniAppPlatform, Orientation, Permission, PlatformTarget, Profile,
};
pub use pipeline::{BuildArtifact, BuildPipeline, Package, PackageFormat};
pub use asset::{
    AssetCompress, AssetEncrypt, AssetEntry, AssetKind, AssetManifest, AssetPipeline,
    DiffResult,
};
pub use hot_update::{FileChange, HotUpdate, HotUpdatePatch};
pub use cache::BuildCache;
pub use hash::Hash;
pub use compress::{Compress, Encrypt};
pub use logger::{BuildLogger, BuildProgress, BuildReport, BuildStage};
pub use error::{BuildError, BuildResult};