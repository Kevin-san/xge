//! Game engine utility library
//!
//! Provides handle, arena, and resource management types.

extern crate alloc;

mod arena;
mod asset_id;
mod handle;
mod resource_manager;
mod spin_lock;

pub use arena::{Arena, ArenaIter};
pub use asset_id::AssetId;
pub use handle::Handle;
pub use resource_manager::ResourceManager;
pub use spin_lock::{SpinLock, SpinLockGuard};
