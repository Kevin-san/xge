//! Game engine platform abstraction library
//!
//! Provides time management, file system abstraction, thread pool, and platform detection.

mod fs;
mod platform;
mod thread_pool;
mod time;

pub use fs::{FileSystem, FileSystemError, StandardFileSystem, normalize_path, join_paths};
pub use platform::{Feature, Platform, RenderBackend};
pub use thread_pool::ThreadPool;
pub use time::{FixedTimestepSteps, Stopwatch, Time};
