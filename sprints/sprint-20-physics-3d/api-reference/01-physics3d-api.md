# Sprint 20 · API 参考

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)

```rust
// 主入口
pub use world::PhysicsWorld3D;
pub use rigidbody::{RigidBody, RigidBodyType, SleepState};
pub use collider::Collider;
pub use shape::{Shape, BoxShape, SphereShape, CapsuleShape, CylinderShape, ConvexHullShape, TrimeshShape, ShapeType};
pub use joint::{Joint, FixedJoint, HingeJoint, BallSocketJoint, SliderJoint, ConeTwistJoint, BreakableJoint};
pub use contact::{ContactManifold, ContactPoint};
pub use character::{CharacterController, CharacterMovement, CharacterCollisionInfo};

// Broad Phase
pub use broadphase::{DynamicAabbTree, Bvh, SahConfig};

// Narrow Phase
pub use narrowphase::{GjkResult, EpaResult};

// 数学
pub use math::{Transform, Aabb, Quaternion, Vec3};
```

## 关键 API

```rust
impl PhysicsWorld3D {
    pub fn new(config: PhysicsConfig) -> Self;
    pub fn step(&mut self, real_dt: f32);
    pub fn add_body(&mut self, body: RigidBody) -> BodyHandle;
    pub fn add_collider(&mut self, collider: Collider, body: BodyHandle) -> ColliderHandle;
    pub fn add_joint(&mut self, joint: Joint) -> JointHandle;
    pub fn ray_cast(&self, ray: Ray3) -> Option<RayCastHit>;
    pub fn overlap_test(&self, shape: &dyn Shape, pos: Vec3, rot: Quat) -> Vec<BodyHandle>;
    pub fn sweep_test(&self, shape: &dyn Shape, from: Vec3, to: Vec3) -> Vec<BodyHandle>;
}
```
