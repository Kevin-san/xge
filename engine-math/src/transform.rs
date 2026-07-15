use core::fmt;

use crate::Mat4;
use crate::Quat;
use crate::Vec3;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
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
        let translation = Mat4::from_translation(self.translation);
        let rotation = Mat4::from_quat(self.rotation);
        let scale = Mat4::from_scale(self.scale);
        translation * rotation * scale
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        Self {
            translation: -(self.rotation.inverse() * self.translation),
            rotation: self.rotation.inverse(),
            scale: Vec3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z),
        }
    }

    #[inline]
    pub fn looking_at(target: Vec3, up: Vec3) -> Self {
        let forward = (target - Vec3::ZERO).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();

        let rotation = Quat::from_mat4(&Mat4 {
            cols: [
                [right.x, right.y, right.z, 0.0],
                [up.x, up.y, up.z, 0.0],
                [-forward.x, -forward.y, -forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        });

        Self {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.rotation * (point * self.scale) + self.translation
    }

    #[inline]
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (vector * self.scale)
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transform(t: {}, r: {}, s: {})",
            self.translation, self.rotation, self.scale
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vec4;

    #[test]
    fn test_new() {
        let t = Transform::new(Vec3::X, Quat::IDENTITY, Vec3::ONE);
        assert_eq!(t.translation, Vec3::X);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
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
        let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
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
    fn test_matrix() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        let v = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let result = m.mul_vec4(v);
        assert!((result.x - 1.0).abs() < 1e-6);
        assert!((result.y - 2.0).abs() < 1e-6);
        assert!((result.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse() {
        let t = Transform::new(Vec3::X, Quat::IDENTITY, Vec3::ONE);
        let inv = t.inverse();
        assert_eq!(inv.translation, -Vec3::X);
        assert_eq!(inv.rotation, Quat::IDENTITY);
        assert_eq!(inv.scale, Vec3::ONE);
    }

    #[test]
    fn test_transform_point() {
        let t = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let p = Vec3::new(0.0, 0.0, 0.0);
        let result = t.transform_point(p);
        assert_eq!(result, Vec3::X);
    }

    #[test]
    fn test_transform_vector() {
        let t = Transform::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let v = Vec3::X;
        let result = t.transform_vector(v);
        assert_eq!(result, Vec3::new(2.0, 0.0, 0.0));
    }
}
