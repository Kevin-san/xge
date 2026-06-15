#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod handle;
pub mod arena;
pub mod event_bus;
pub mod resource_manager;
pub mod asset_id;

pub use handle::Handle;
pub use arena::Arena;
pub use event_bus::{EventBus, SubscriptionHandle};
pub use resource_manager::ResourceManager;
pub use asset_id::AssetId;
