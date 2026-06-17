//! Game engine 3D rendering library
//!
//! Provides Mesh3D, Camera3D, Light3D, Transform3D, Frustum, Ray3, Scene3D and related types.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod buffer;
mod camera;
mod frustum;
mod geometry;
mod light;
mod mesh;
mod ray;
mod scene;
mod transform;
mod vertex;

pub use buffer::{IndexBuffer, IndexFormat, VertexBuffer};
pub use camera::Camera3D;
pub use frustum::Frustum;
pub use geometry::{Mat4Transform3D, Plane, Sphere, AABB};
pub use light::{
    AmbientLight, DirectionalLight, HemisphereLight, Light3D, LightManager, PointLight, SpotLight,
};
pub use mesh::{Mesh3D, MeshBuilder3D, Primitive};
pub use ray::{HitResult, Ray3};
pub use scene::{Node3D, NodeHandle, RenderEntity3D, Scene3D, SceneStats3D};
pub use transform::Transform3D;
pub use vertex::{Vertex, VertexLayout};
