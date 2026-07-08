//! Game engine utility library
//!
//! Provides handle, arena, and resource management types.

extern crate alloc;

mod arena;
mod asset_id;
mod event_bus;
mod handle;
mod resource_manager;

pub use arena::{Arena, ArenaIter};
pub use asset_id::AssetId;
pub use event_bus::EventBus;
pub use handle::Handle;
pub use resource_manager::ResourceManager;
