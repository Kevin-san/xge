use core::fmt;

use super::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const ZERO: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

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
    pub fn from_translation(v: Vec3) -> Self {
        Self {
            translation: v,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_rotation(q: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: q,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_scale(v: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: v,
        }
    }

    #[inline]
    pub fn matrix(&self) -> Mat4 {
        let scale = Mat4::from_scale(self.scale);
        let rotation = Mat4::from_quat(self.rotation);
        let translation = Mat4::from_translation(self.translation);
        translation * rotation * scale
    }

    #[inline]
    pub fn translate(&mut self, v: Vec3) {
        self.translation += v;
    }

    #[inline]
    pub fn rotate(&mut self, q: Quat) {
        self.rotation = q * self.rotation;
    }

    #[inline]
    pub fn rotate_local(&mut self, q: Quat) {
        self.rotation = self.rotation * q;
    }

    #[inline]
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.translation).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right).normalize();

        let m = Mat4 {
            cols: [
                [right.x, up.x, forward.x, 0.0],
                [right.y, up.y, forward.y, 0.0],
                [right.z, up.z, forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        self.rotation = quat_from_mat4(&m);
    }
}

fn quat_from_mat4(m: &Mat4) -> Quat {
    let trace = m.cols[0][0] + m.cols[1][1] + m.cols[2][2];

    if trace > 0.0 {
        let s = 0.5 / (trace + 1.0).sqrt();
        Quat {
            x: (m.cols[2][1] - m.cols[1][2]) * s,
            y: (m.cols[0][2] - m.cols[2][0]) * s,
            z: (m.cols[1][0] - m.cols[0][1]) * s,
            w: 0.25 / s,
        }
    } else if m.cols[0][0] > m.cols[1][1] && m.cols[0][0] > m.cols[2][2] {
        let s = 2.0 * (1.0 + m.cols[0][0] - m.cols[1][1] - m.cols[2][2]).sqrt();
        Quat {
            x: 0.25 * s,
            y: (m.cols[0][1] + m.cols[1][0]) / s,
            z: (m.cols[0][2] + m.cols[2][0]) / s,
            w: (m.cols[2][1] - m.cols[1][2]) / s,
        }
    } else if m.cols[1][1] > m.cols[2][2] {
        let s = 2.0 * (1.0 + m.cols[1][1] - m.cols[0][0] - m.cols[2][2]).sqrt();
        Quat {
            x: (m.cols[0][1] + m.cols[1][0]) / s,
            y: 0.25 * s,
            z: (m.cols[1][2] + m.cols[2][1]) / s,
            w: (m.cols[0][2] - m.cols[2][0]) / s,
        }
    } else {
        let s = 2.0 * (1.0 + m.cols[2][2] - m.cols[0][0] - m.cols[1][1]).sqrt();
        Quat {
            x: (m.cols[0][2] + m.cols[2][0]) / s,
            y: (m.cols[1][2] + m.cols[2][1]) / s,
            z: 0.25 * s,
            w: (m.cols[1][0] - m.cols[0][1]) / s,
        }
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
    fn test_identity() {
        let t = Transform::IDENTITY;
        let m = t.matrix();
        assert_eq!(m.cols[0][0], 1.0);
        assert_eq!(m.cols[1][1], 1.0);
        assert_eq!(m.cols[2][2], 1.0);
        assert_eq!(m.cols[3][3], 1.0);
    }

    #[test]
    fn test_translation() {
        let t = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        assert_eq!(m.cols[3][0], 1.0);
        assert_eq!(m.cols[3][1], 2.0);
        assert_eq!(m.cols[3][2], 3.0);
    }

    #[test]
    fn test_scale() {
        let t = Transform::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let m = t.matrix();
        assert_eq!(m.cols[0][0], 2.0);
        assert_eq!(m.cols[1][1], 3.0);
        assert_eq!(m.cols[2][2], 4.0);
    }

    #[test]
    fn test_rotation() {
        let angle = std::f32::consts::FRAC_PI_2;
        let q = Quat::from_rotation_x(angle);
        let t = Transform::from_rotation(q);
        let m = t.matrix();

        let v = Vec3::Y;
        let rotated = q * v;
        let transformed = m.mul_vec4(Vec4::new(v.x, v.y, v.z, 1.0)).xyz();

        assert!((rotated.x - transformed.x).abs() < 1e-6);
        assert!((rotated.y - transformed.y).abs() < 1e-6);
        assert!((rotated.z - transformed.z).abs() < 1e-6);
    }

    #[test]
    fn test_composed_transform() {
        let t = Transform::new(
            Vec3::new(1.0, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::new(2.0, 2.0, 2.0),
        );

        let m = t.matrix();
        let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let result = m.mul_vec4(v);

        assert!((result.x - 3.0).abs() < 1e-6);
        assert!((result.y - 2.0).abs() < 1e-6);
        assert!((result.z - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_translate() {
        let mut t = Transform::IDENTITY;
        t.translate(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.translation, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_rotate() {
        let mut t = Transform::IDENTITY;
        let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        t.rotate(q);

        let v = Vec3::Y;
        let rotated = t.rotation * v;
        assert!((rotated.z - 1.0).abs() < 1e-6);
    }
}