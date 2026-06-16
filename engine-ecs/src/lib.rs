//! engine-ecs crate - ECS (Entity Component System) 核心
//!
//! 提供游戏引擎的 ECS 架构实现，包括 World、Entity、Component、System 等核心概念。

#![warn(missing_docs)]

pub mod world;
pub mod entity;
pub mod component;
pub mod bundle;
pub mod resource;
pub mod event;
pub mod query;
pub mod system;

pub use world::World;
pub use entity::{Entity, EntityRef, EntityMut};
pub use component::Component;
pub use bundle::Bundle;
pub use resource::{Resource, Resources};
pub use event::{Event, EventWriter, EventReader};
pub use query::Query;
pub use system::{System, SystemParam, IntoSystem};
