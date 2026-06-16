//! engine-physics-2d crate - 2D 物理引擎
//!
//! 提供 2D 物理仿真，包括刚体、碰撞检测、关节等。

#![warn(missing_docs)]

pub mod collider;
pub mod collision;
pub mod joint;
pub mod query;
pub mod rigidbody;
pub mod world;

pub use collider::{Collider2D, Collider2DBuilder, ColliderShape};
pub use collision::{CollisionEvent, Contact, Manifold};
pub use joint::{DistanceJoint, Joint2D, PrismaticJoint, RevoluteJoint};
pub use query::{PointQuery, RayCast2D, RayCastHit2D, ShapeCast2D, Transform2D};
pub use rigidbody::{RigidBody2D, RigidBody2DBuilder, RigidBodyType};
pub use world::PhysicsWorld2D;
