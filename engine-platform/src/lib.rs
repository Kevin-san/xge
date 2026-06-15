#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod time;
pub mod file_system;
pub mod thread_pool;
pub mod platform;
pub mod feature;

pub use time::{Time, FixedTimestepSteps, Stopwatch};
pub use file_system::{FileSystem, NativeFileSystem};
pub use thread_pool::{ThreadPool, JoinHandle};
pub use platform::Platform;
pub use feature::Feature;
