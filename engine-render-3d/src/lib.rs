//! engine-render-3d crate - 3D 渲染核心（网格 / 相机 / 光照 / 变换）
//!
//! 提供 3D 渲染所需的核心类型，包括 Mesh3D、Camera3D、Light3D、Transform3D、
//! Scene3D、Frustum 视锥裁剪、Ray3 射线检测等。
//!
//! # 核心模块
//!
//! - [`mesh`] - Mesh3D 网格与 Vertex3D 顶点结构
//! - [`camera`] - Camera3D 透视/正交相机与 Frustum 视锥
//! - [`light`] - Light3D 光照系统（方向光/点光/聚光灯）
//! - [`transform`] - Transform3D 3D 变换
//! - [`scene`] - Scene3D 场景图与 Node3D 节点
//! - [`geometry`] - 几何类型 AABB、Sphere、Plane、Ray3
//! - [`debug`] - DebugRenderer3D 调试图形

// Re-export commonly used types
pub use camera::{Camera3D, Frustum};
pub use geometry::{AABB, HitResult, Plane, Ray3, Sphere};
pub use light::{DirectionalLight, Light3D, LightManager, LightType, PointLight, SpotLight};
pub use mesh::{Mesh3D, MeshBuilder3D, MeshManager, Primitive, Vertex3D};
pub use scene::{Node3D, Scene3D};
pub use transform::Transform3D;

// Module declarations
mod camera;
mod geometry;
mod light;
mod mesh;
mod scene;
mod transform;

// Optional OpenGL backend
#[cfg(feature = "gl")]
mod gl_backend;
