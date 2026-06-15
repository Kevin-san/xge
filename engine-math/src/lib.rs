#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat4;
pub mod quat;
pub mod transform;
pub mod euler;
pub mod rect;
pub mod aabb;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
pub use mat4::Mat4;
pub use quat::Quat;
pub use transform::Transform;
pub use euler::Euler;
pub use rect::Rect;
pub use aabb::AABB;
