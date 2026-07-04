//! Game engine platform abstraction library
//!
//! Provides time management, file system abstraction, thread pool, and platform detection.

mod filesystem;
mod platform;
mod thread_pool;
mod time;

pub use filesystem::{FileSystem, NativeFileSystem};
pub use platform::{Feature, Platform};
pub use thread_pool::ThreadPool;
pub use time::{FixedTimestepSteps, Stopwatch, Time};
