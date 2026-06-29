//! engine-ecs crate - ECS (Entity Component System) 核心
//!
//! 提供游戏引擎的 ECS 架构实现，包括 World、Entity、Component、System 等核心概念。

pub mod archetype;
pub mod bundle;
pub mod change_tracker;
pub mod component;
pub mod entity;
pub mod event;
pub mod hierarchy;
pub mod query;
pub mod resource;
pub mod schedule;
pub mod storage;
pub mod system;
pub mod system_param;
pub mod world;

pub use archetype::{Archetype, ArchetypeStorage};
pub use bundle::{Bundle, BundleError};
pub use change_tracker::{ChangeTrackers, Ref, Tick};
pub use component::Component;
pub use entity::{Entity, EntityMut, EntityRef};
pub use event::{EntityDespawned, EntitySpawned, Event, EventReader, EventWriter, Events};
pub use hierarchy::{Children, HierarchyCommandsExt, Parent, WorldHierarchyExt};
pub use query::{
    AccessMode, Added, Changed, ComponentAccess, NoneFilter, Query, QueryFilter, QueryItem,
    QueryItemMut, QueryIter, QueryState, With, Without,
};
pub use resource::{Resource, Resources};
pub use schedule::{FnSystem, Schedule, System, SystemStage};
pub use storage::{HashMapStorage, SparseSet, StorageLabel, StorageType};
pub use system::{IntoSystem, System as SystemTrait, SystemParam};
pub use system_param::{Commands, Res, ResMut};
pub use world::{ResourceError, World};
