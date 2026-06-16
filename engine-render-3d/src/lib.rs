//! Game engine 3D rendering library
//!
//! Provides Mesh3D, Camera3D, Light3D, Transform3D, Frustum, Ray3, Scene3D and related types.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod geometry;
mod camera;
mod light;
mod transform;
mod frustum;
mod ray;
mod scene;
mod mesh;
mod vertex;
mod buffer;

pub use geometry::{AABB, Sphere, Plane, Mat4Transform3D};
pub use camera::Camera3D;
pub use light::{Light3D, DirectionalLight, PointLight, SpotLight, AmbientLight, HemisphereLight, LightManager};
pub use transform::Transform3D;
pub use frustum::Frustum;
pub use ray::{Ray3, HitResult};
pub use scene::{Scene3D, Node3D, NodeHandle, RenderEntity3D, SceneStats3D};
pub use mesh::{Mesh3D, MeshBuilder3D, Primitive};
pub use vertex::{Vertex, VertexLayout};
pub use buffer::{VertexBuffer, IndexBuffer, IndexFormat};