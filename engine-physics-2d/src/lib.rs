//! engine-physics-2d crate - 2D 物理引擎
//!
//! 提供 2D 物理仿真，包括刚体、碰撞检测、关节等。

#![warn(missing_docs)]

pub mod collider;
pub mod collision;
pub mod collision_group;
pub mod debug_renderer;
pub mod joint;
pub mod query;
pub mod rigidbody;
pub mod physics_material;
pub mod world;

pub use collider::{Collider2D, Collider2DBuilder, ColliderShape};
pub use physics_material::PhysicsMaterial;
pub use collision::{CollisionEvent, Contact, Manifold};
pub use collision_group::CollisionGroup;
pub use debug_renderer::PhysicsDebugRenderer;
pub use joint::{
    DistanceJoint, Joint2D, JointType, MotorJoint, PrismaticJoint, RevoluteJoint, SpringJoint,
    WeldJoint,
};
pub use query::{PointQuery, RayCast2D, RayCastHit2D, ShapeCast2D, Transform2D};
pub use rigidbody::{RigidBody2D, RigidBody2DBuilder, RigidBodyType};
pub use world::{PhysicsWorld2D, QueryFilter, ShapeCastHit2D};
