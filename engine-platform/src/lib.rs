//! Game engine platform abstraction library
//!
//! Provides time management, file system abstraction, thread pool, and platform detection.

mod time;
mod thread_pool;
mod platform;

pub use time::{Time, FixedTimestepSteps, Stopwatch};
pub use thread_pool::ThreadPool;
pub use platform::{Platform, Feature};