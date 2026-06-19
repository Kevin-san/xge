use core::fmt;
use crate::{Vec3, Quat, Mat4};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    #[inline]
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn from_translation(v: Vec3) -> Self {
        Self {
            translation: v,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        let scale_mat = Mat4::from_scale(self.scale);
        let rotation_mat = Mat4::from_quat(self.rotation);
        let translation_mat = Mat4::from_translation(self.translation);
        
        translation_mat * rotation_mat * scale_mat
    }

    pub fn inverse(self) -> Self {
        let inv_rotation = self.rotation.inverse();
        let inv_scale = Vec3::new(
            if self.scale.x != 0.0 { 1.0 / self.scale.x } else { 0.0 },
            if self.scale.y != 0.0 { 1.0 / self.scale.y } else { 0.0 },
            if self.scale.z != 0.0 { 1.0 / self.scale.z } else { 0.0 },
        );
        
        Self {
            translation: inv_rotation * (-self.translation) * inv_scale,
            rotation: inv_rotation,
            scale: inv_scale,
        }
    }

    #[inline]
    pub fn mul_transform(self, other: Self) -> Self {
        Self {
            translation: self.translation + self.rotation * (self.scale * other.translation),
            rotation: self.rotation * other.rotation,
            scale: self.scale * other.scale,
        }
    }
}

impl core::ops::Mul for Transform {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.mul_transform(other)
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transform(translation: {}, rotation: {}, scale: {})",
            self.translation, self.rotation, self.scale
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::default();
        assert_eq!(t.translation, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_matrix() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        let v = crate::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let result = m.mul_vec4(v);
        assert_eq!(result.xyz(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_inverse() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let inv = t.inverse();
        let identity = t * inv;
        
        assert!((identity.translation - Vec3::ZERO).length() < 1e-6);
    }

    #[test]
    fn test_from_translation() {
        let t = Transform::from_translation(Vec3::new(10.0, 20.0, 30.0));
        assert_eq!(t.translation, Vec3::new(10.0, 20.0, 30.0));
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_from_rotation() {
        let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
        let t = Transform::from_rotation(q);
        assert_eq!(t.translation, Vec3::ZERO);
        assert_eq!(t.rotation, q);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_from_scale() {
        let t = Transform::from_scale(Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(t.translation, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_mul_transform_translation() {
        let t1 = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let t2 = Transform::from_translation(Vec3::new(2.0, 0.0, 0.0));
        let combined = t1 * t2;
        
        // t1 applies translation to t2's translation
        assert_eq!(combined.translation, Vec3::new(3.0, 0.0, 0.0));
    }

    #[test]
    fn test_mul_transform_scale() {
        let t1 = Transform::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let t2 = Transform::from_scale(Vec3::new(3.0, 3.0, 3.0));
        let combined = t1 * t2;
        
        assert_eq!(combined.scale, Vec3::new(6.0, 6.0, 6.0));
    }

    #[test]
    fn test_mul_transform_rotation() {
        let r1 = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
        let r2 = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
        let t1 = Transform::from_rotation(r1);
        let t2 = Transform::from_rotation(r2);
        let combined = t1 * t2;
        
        // Two 45 degree rotations = 90 degree
        let v = Vec3::Y;
        let result = combined.rotation * v;
        assert!((result.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse_with_rotation() {
        let q = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let t = Transform::from_rotation(q);
        let inv = t.inverse();
        
        let combined = t * inv;
        assert!((combined.rotation.x - 0.0).abs() < 1e-6);
        assert!((combined.rotation.w - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse_with_scale() {
        let t = Transform::from_scale(Vec3::new(2.0, 4.0, 8.0));
        let inv = t.inverse();
        
        assert_eq!(inv.scale.x, 0.5);
        assert_eq!(inv.scale.y, 0.25);
        assert_eq!(inv.scale.z, 0.125);
    }

    #[test]
    fn test_inverse_zero_scale() {
        let t = Transform::from_scale(Vec3::new(0.0, 1.0, 1.0));
        let inv = t.inverse();
        
        // Zero scale should result in zero inverse scale
        assert_eq!(inv.scale.x, 0.0);
    }

    #[test]
    fn test_matrix_combined_transform() {
        let t = Transform::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_4),
            Vec3::new(2.0, 2.0, 2.0),
        );
        let m = t.matrix();
        
        // Matrix should be valid (non-zero determinant)
        assert!(m.inverse().is_some());
    }

    #[test]
    fn test_identity_multiplication() {
        let t = Transform::from_translation(Vec3::new(5.0, 10.0, 15.0));
        let identity = Transform::default();
        
        let result1 = identity * t;
        assert_eq!(result1.translation, t.translation);
        
        let result2 = t * identity;
        assert_eq!(result2.translation, t.translation);
    }

    #[test]
    fn test_complex_transform_chain() {
        let t1 = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let t2 = Transform::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let t3 = Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
        
        let combined = t1 * t2 * t3;
        
        // Verify the combined transform produces a valid matrix
        let m = combined.matrix();
        assert!(m.inverse().is_some());
    }
}
