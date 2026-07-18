//! Game engine math library
//!
//! Provides vector, matrix, quaternion and transform types for game development.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod aabb;
mod euler;
mod mat4;
mod quat;
mod rect;
mod transform;
mod vec2;
mod vec3;
mod vec4;

pub use aabb::AABB;
pub use euler::Euler;
pub use mat4::Mat4;
pub use quat::Quat;
pub use rect::Rect;
pub use transform::Transform;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

/// Linear interpolation between two values
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Spherical linear interpolation between two quaternions
#[inline]
pub fn slerp(a: Quat, b: Quat, t: f32) -> Quat {
    Quat::slerp(a, b, t)
}

/// Normalized linear interpolation between two quaternions
#[inline]
pub fn nlerp(a: Quat, b: Quat, t: f32) -> Quat {
    Quat::nlerp(a, b, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 1e-6);
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < 1e-6);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < 1e-6);
        assert!((lerp(-5.0, 5.0, 0.5) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_slerp_free() {
        let q1 = Quat::IDENTITY;
        let q2 = Quat::from_rotation_x(core::f32::consts::FRAC_PI_2);
        // Free function should match method
        let r_free = slerp(q1, q2, 0.5);
        let r_method = q1.slerp(q2, 0.5);
        assert!((r_free.x - r_method.x).abs() < 1e-6);
        assert!((r_free.y - r_method.y).abs() < 1e-6);
        assert!((r_free.z - r_method.z).abs() < 1e-6);
        assert!((r_free.w - r_method.w).abs() < 1e-6);
    }

    #[test]
    fn test_nlerp_free() {
        let q1 = Quat::IDENTITY;
        let q2 = Quat::from_rotation_x(core::f32::consts::FRAC_PI_2);
        // Free function should match method
        let r_free = nlerp(q1, q2, 0.5);
        let r_method = q1.nlerp(q2, 0.5);
        assert!((r_free.x - r_method.x).abs() < 1e-6);
        assert!((r_free.y - r_method.y).abs() < 1e-6);
        assert!((r_free.z - r_method.z).abs() < 1e-6);
        assert!((r_free.w - r_method.w).abs() < 1e-6);
    }
}
