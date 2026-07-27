//! Game engine math library
//!
//! Provides vector, matrix, quaternion and transform types for game development.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod aabb;
mod dual_quat;
mod euler;
mod frustum;
mod mat4;
mod quat;
mod rect;
mod simd;
mod soa;
mod vec2;
mod vec3;
mod vec4;

pub use aabb::AABB;
pub use dual_quat::DualQuat;
pub use euler::Euler;
pub use frustum::{FrustResult, Frustum, Plane};
pub use mat4::Mat4;
pub use quat::Quat;
pub use rect::Rect;
pub use simd::{f32x4, f32x8};
#[cfg(target_arch = "x86_64")]
pub use simd::x86 as simd_x86;
#[cfg(target_arch = "aarch64")]
pub use simd::aarch64_backend as simd_aarch64;
pub use soa::{aos_to_soa_quat, aos_to_soa_vec3, SoaQuat, SoaVec3, soa_to_aos_quat, soa_to_aos_vec3};
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
