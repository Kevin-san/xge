//! Game engine utility library
//! 
//! Provides handle, arena, and resource management types.

extern crate alloc;

mod handle;
mod arena;
mod resource_manager;
mod asset_id;

pub use handle::Handle;
pub use arena::{Arena, ArenaIter};
pub use resource_manager::ResourceManager;
pub use asset_id::AssetId;