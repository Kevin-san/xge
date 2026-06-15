use core::fmt;
use crate::{Vec3, Quat, Mat4};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
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
}
