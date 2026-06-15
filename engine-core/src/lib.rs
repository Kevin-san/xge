#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod engine;
pub mod module;
pub mod app;
pub mod schedule;

pub use engine::{Engine, EngineConfig};
pub use module::{Module, ModuleRegistry, CycleError};
pub use app::{App, AppBuilder};
pub use schedule::Schedule;

pub const ENGINE_VERSION: &str = "0.1.0-dev";
pub const BUILD_COMMIT_HASH: &str = env!("BUILD_COMMIT_HASH");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
