use core::fmt;

use super::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    #[inline]
    pub const fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
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

    #[inline]
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_translation(self.translation) * Mat4::from_quat(self.rotation) * Mat4::from_scale(self.scale)
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let inv_rotation = self.rotation.inverse();
        let inv_scale = Vec3::new(
            if self.scale.x.abs() > 1e-10 { 1.0 / self.scale.x } else { 0.0 },
            if self.scale.y.abs() > 1e-10 { 1.0 / self.scale.y } else { 0.0 },
            if self.scale.z.abs() > 1e-10 { 1.0 / self.scale.z } else { 0.0 },
        );
        let inv_translation = -(inv_rotation * (self.translation * inv_scale));
        Self {
            translation: inv_translation,
            rotation: inv_rotation,
            scale: inv_scale,
        }
    }

    #[inline]
    pub fn mul_transform(&self, other: &Self) -> Self {
        let new_rotation = self.rotation * other.rotation;
        let new_scale = self.scale * other.scale;
        let new_translation = self.translation + self.rotation * (self.scale * other.translation);
        Self {
            translation: new_translation,
            rotation: new_rotation,
            scale: new_scale,
        }
    }

    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.translation + self.rotation * (self.scale * point)
    }

    #[inline]
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (self.scale * vector)
    }

    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            translation: self.translation.lerp(other.translation, t),
            rotation: self.rotation.slerp(other.rotation, t),
            scale: self.scale.lerp(other.scale, t),
        }
    }

    #[inline]
    pub fn is_identity(&self) -> bool {
        self.translation == Vec3::ZERO
            && self.rotation == Quat::IDENTITY
            && self.scale == Vec3::ONE
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transform(pos: ({:.2}, {:.2}, {:.2}), rot: ({:.2}, {:.2}, {:.2}, {:.2}), scale: ({:.2}, {:.2}, {:.2}))",
            self.translation.x, self.translation.y, self.translation.z,
            self.rotation.x, self.rotation.y, self.rotation.z, self.rotation.w,
            self.scale.x, self.scale.y, self.scale.z
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::IDENTITY;
        assert_eq!(t.translation, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
        assert!(t.is_identity());
    }

    #[test]
    fn test_from_translation() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.translation, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_from_rotation() {
        let q = Quat::from_rotation_x(0.5);
        let t = Transform::from_rotation(q);
        assert_eq!(t.translation, Vec3::ZERO);
        assert_eq!(t.rotation, q);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_from_scale() {
        let s = Vec3::new(2.0, 3.0, 4.0);
        let t = Transform::from_scale(s);
        assert_eq!(t.translation, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, s);
    }

    #[test]
    fn test_matrix_translation() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        let v = Vec3::new(0.0, 0.0, 0.0);
        let result = m.mul_vec4(v.extend(1.0)).xyz();
        assert!((result.x - 1.0).abs() < 1e-6);
        assert!((result.y - 2.0).abs() < 1e-6);
        assert!((result.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_transform_point() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let p = Vec3::new(4.0, 5.0, 6.0);
        let result = t.transform_point(p);
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_transform_vector() {
        let t = Transform::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = t.transform_vector(v);
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_lerp() {
        let a = Transform::IDENTITY;
        let b = Transform::new(
            Vec3::new(10.0, 20.0, 30.0),
            Quat::IDENTITY,
            Vec3::new(2.0, 2.0, 2.0),
        );
        let result = a.lerp(&b, 0.5);
        assert!((result.translation.x - 5.0).abs() < 1e-6);
        assert!((result.translation.y - 10.0).abs() < 1e-6);
        assert!((result.translation.z - 15.0).abs() < 1e-6);
        assert!((result.scale.x - 1.5).abs() < 1e-6);
    }

    #[test]
    fn test_mul_transform() {
        let a = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let b = Transform::from_translation(Vec3::new(0.0, 2.0, 0.0));
        let result = a.mul_transform(&b);
        assert!((result.translation.x - 1.0).abs() < 1e-6);
        assert!((result.translation.y - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse() {
        let t = Transform::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_y(0.5),
            Vec3::new(2.0, 2.0, 2.0),
        );
        let inv = t.inverse();
        let result = t.mul_transform(&inv);
        assert!(result.translation.length() < 1e-4);
        assert!((result.scale.x - 1.0).abs() < 1e-4);
        assert!((result.scale.y - 1.0).abs() < 1e-4);
        assert!((result.scale.z - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_new() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let rot = Quat::from_rotation_x(0.5);
        let scale = Vec3::new(2.0, 3.0, 4.0);
        let t = Transform::new(pos, rot, scale);
        assert_eq!(t.translation, pos);
        assert_eq!(t.rotation, rot);
        assert_eq!(t.scale, scale);
    }

    #[test]
    fn test_default_is_identity() {
        let t: Transform = Default::default();
        assert!(t.is_identity());
    }
}
