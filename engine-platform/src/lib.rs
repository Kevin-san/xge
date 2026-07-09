//! Game engine platform abstraction library
//!
//! Provides time management, file system abstraction, thread pool, and platform detection.

mod file_system;
mod platform;
mod thread_pool;
mod time;

pub use file_system::FileSystem;
pub use platform::{Feature, Platform};
pub use thread_pool::ThreadPool;
pub use time::{FixedTimestepSteps, Stopwatch, Time};
