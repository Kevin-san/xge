//! engine-ecs crate - ECS (Entity Component System) 核心
//!
//! 提供游戏引擎的 ECS 架构实现，包括 World、Entity、Component、System 等核心概念。

#![warn(missing_docs)]

pub mod bundle;
pub mod component;
pub mod entity;
pub mod event;
pub mod query;
pub mod resource;
pub mod system;
pub mod world;

pub use bundle::Bundle;
pub use component::Component;
pub use entity::{Entity, EntityMut, EntityRef};
pub use event::{Event, EventReader, EventWriter};
pub use query::Query;
pub use resource::{Resource, Resources};
pub use system::{IntoSystem, System, SystemParam};
pub use world::World;
