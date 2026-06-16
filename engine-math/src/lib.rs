//! Game engine math library
//!
//! Provides vector, matrix, quaternion and transform types for game development.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod mat4;
mod quat;
mod vec2;
mod vec3;
mod vec4;
mod rect;

pub use mat4::Mat4;
pub use quat::Quat;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
pub use rect::Rect;
