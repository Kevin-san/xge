//! Game engine PBR material and shader system
//!
//! Provides PBR material, shader graph, IBL baker, tonemapper and color grading.

mod brdf;
mod color;
mod flags;
mod graph;
mod hdr;
mod ibl;
mod material;
mod permutation;
mod shadow;
mod tonemap;

pub use brdf::{BrdfResult, CookTorranceBRDF, PI};
pub use color::ColorGrading;
pub use flags::PbrMaterialFlags;
pub use graph::{
    CycleError, EdgeId, NodeId, NodeKind, ShaderGraph, ShaderGraphNode, ShaderNodeType,
};
pub use hdr::{ExposureSettings, HdrFormat, HdrFramebuffer, HdrImageData};
pub use ibl::{
    BrdfLutData, CubeMap, EnvironmentMap, IBLBaker, IrradianceMapData, PrefilterMapData, Texture2D,
};
pub use material::{AlphaMode, PbrMaterial};
pub use permutation::{CachedShader, PermutationCache, ShaderPermutationKey};
pub use shadow::{
    look_at, orthographic, ShadowCascade, ShadowMapConfig, ShadowMapRenderer, ShadowMapTexture,
    MAX_CASCADES,
};
pub use tonemap::Tonemapper;

// Re-export engine-math types for convenience
pub use engine_math::{Vec2, Vec3, Vec4};
