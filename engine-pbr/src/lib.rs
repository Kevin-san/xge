//! Game engine PBR material and shader system
//!
//! Provides PBR material, shader graph, IBL baker, tonemapper and color grading.

mod color;
mod flags;
mod graph;
mod ibl;
mod material;
mod tonemap;

pub use color::ColorGrading;
pub use flags::PbrMaterialFlags;
pub use graph::{
    CycleError, EdgeId, NodeId, NodeKind, ShaderGraph, ShaderGraphNode, ShaderNodeType,
};
pub use ibl::{CubeMap, EnvironmentMap, IBLBaker, Texture2D};
pub use material::{AlphaMode, PbrMaterial};
pub use tonemap::Tonemapper;

// Re-export engine-math types for convenience
pub use engine_math::{Vec2, Vec3, Vec4};
