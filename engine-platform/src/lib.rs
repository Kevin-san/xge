//! Game engine platform abstraction library
//!
//! Provides time management, file system abstraction, thread pool, and platform detection.

mod fs;
mod macros;
mod platform;
mod thread_pool;
mod time;

pub use fs::FileSystem;
pub use platform::{Feature, Platform};
pub use thread_pool::ThreadPool;
pub use time::{FixedTimestepSteps, Stopwatch, Time};
