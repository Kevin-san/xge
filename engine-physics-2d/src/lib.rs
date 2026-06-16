//! engine-physics-2d crate - 2D 物理引擎
//!
//! 提供 2D 物理仿真，包括刚体、碰撞检测、关节等。

#![warn(missing_docs)]

pub mod world;
pub mod rigidbody;
pub mod collider;
pub mod joint;
pub mod collision;
pub mod query;

pub use world::PhysicsWorld2D;
pub use rigidbody::{RigidBody2D, RigidBody2DBuilder, RigidBodyType};
pub use collider::{Collider2D, Collider2DBuilder, ColliderShape};
pub use joint::{Joint2D, DistanceJoint, RevoluteJoint, PrismaticJoint};
pub use collision::{Contact, Manifold, CollisionEvent};
pub use query::{RayCast2D, RayCastHit2D, ShapeCast2D, PointQuery, Transform2D};
