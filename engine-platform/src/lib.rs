//! Game engine platform abstraction library
//!
//! Provides time management, file system abstraction, thread pool, and platform detection.

mod platform;
mod thread_pool;
mod time;

pub mod filesystem;
pub mod macros;

pub use filesystem::{FsError, FileSystem};
pub use macros::{IS_DESKTOP, IS_LINUX, IS_MACOS, IS_WINDOWS, TARGET_OS};
pub use platform::{Feature, Platform};
pub use thread_pool::ThreadPool;
pub use time::{FixedTimestepSteps, Stopwatch, Time};
