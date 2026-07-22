use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    #[inline]
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn from_translation(v: Vec3) -> Self {
        Self {
            position: v,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_rotation(q: Quat) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: q,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_scale(v: Vec3) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: v,
        }
    }

    #[inline]
    pub fn matrix(&self) -> Mat4 {
        let scale_mat = Mat4::from_scale(self.scale);
        let rotation_mat = Mat4::from_quat(self.rotation);
        let translation_mat = Mat4::from_translation(self.position);
        translation_mat * rotation_mat * scale_mat
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        Self {
            position: -(self.rotation.inverse() * self.position),
            rotation: self.rotation.inverse(),
            scale: Vec3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z),
        }
    }

    #[inline]
    pub fn mul_point(&self, point: Vec3) -> Vec3 {
        self.rotation * (self.scale * point) + self.position
    }

    #[inline]
    pub fn mul_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (self.scale * vector)
    }

    #[inline]
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.position).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        let rotation = Mat4 {
            cols: [
                [right.x, up.x, -forward.x, 0.0],
                [right.y, up.y, -forward.y, 0.0],
                [right.z, up.z, -forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        self.rotation = Quat::from_euler(Euler::new(0.0, 0.0, 0.0));
        if let Some(inv) = rotation.inverse() {
            self.rotation = quat_from_mat4(&inv);
        }
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            position: self.position.lerp(other.position, t),
            rotation: self.rotation.nlerp(other.rotation, t),
            scale: self.scale.lerp(other.scale, t),
        }
    }
}

fn quat_from_mat4(m: &Mat4) -> Quat {
    let trace = m.cols[0][0] + m.cols[1][1] + m.cols[2][2] + 1.0;

    if trace > 0.0 {
        let s = 0.5 / trace.sqrt();
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
            "Transform(pos={}, rot={}, scale={})",
            self.position, self.rotation, self.scale
        )
    }
}

use super::{Euler, Mat4, Quat, Vec3};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::IDENTITY;
        assert_eq!(t.position, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_new() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let rot = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        let scale = Vec3::new(2.0, 2.0, 2.0);

        let t = Transform::new(pos, rot, scale);
        assert_eq!(t.position, pos);
        assert_eq!(t.rotation, rot);
        assert_eq!(t.scale, scale);
    }

    #[test]
    fn test_from_translation() {
        let pos = Vec3::new(5.0, 10.0, 15.0);
        let t = Transform::from_translation(pos);

        assert_eq!(t.position, pos);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_from_rotation() {
        let rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
        let t = Transform::from_rotation(rot);

        assert_eq!(t.position, Vec3::ZERO);
        assert_eq!(t.rotation, rot);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_from_scale() {
        let scale = Vec3::new(2.0, 3.0, 4.0);
        let t = Transform::from_scale(scale);

        assert_eq!(t.position, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, scale);
    }

    #[test]
    fn test_matrix_identity() {
        let t = Transform::IDENTITY;
        let m = t.matrix();
        assert_eq!(m, Mat4::IDENTITY);
    }

    #[test]
    fn test_matrix_translation() {
        let t = Transform::from_translation(Vec3::new(5.0, 10.0, 15.0));
        let m = t.matrix();

        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let result = m.mul_vec4(v);

        assert!((result.x - 6.0).abs() < 1e-5);
        assert!((result.y - 12.0).abs() < 1e-5);
        assert!((result.z - 18.0).abs() < 1e-5);
        assert!((result.w - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_mul_point() {
        let t = Transform {
            position: Vec3::new(1.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(2.0, 2.0, 2.0),
        };

        let point = Vec3::new(1.0, 2.0, 3.0);
        let result = t.mul_point(point);

        assert_eq!(result, Vec3::new(3.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vector() {
        let t = Transform {
            position: Vec3::new(100.0, 100.0, 100.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(2.0, 2.0, 2.0),
        };

        let vector = Vec3::new(1.0, 2.0, 3.0);
        let result = t.mul_vector(vector);

        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_inverse_identity() {
        let t = Transform::IDENTITY;
        let inv = t.inverse();
        assert_eq!(inv, Transform::IDENTITY);
    }

    #[test]
    fn test_inverse_translation() {
        let t = Transform::from_translation(Vec3::new(5.0, 10.0, 15.0));
        let inv = t.inverse();

        assert_eq!(inv.position, Vec3::new(-5.0, -10.0, -15.0));
        assert_eq!(inv.rotation, Quat::IDENTITY);
        assert_eq!(inv.scale, Vec3::ONE);
    }

    #[test]
    fn test_lerp() {
        let a = Transform::IDENTITY;
        let b = Transform {
            position: Vec3::new(2.0, 4.0, 6.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(2.0, 2.0, 2.0),
        };

        let result = a.lerp(b, 0.5);
        assert_eq!(result.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(result.scale, Vec3::new(1.5, 1.5, 1.5));
    }

    #[test]
    fn test_look_at() {
        let mut t = Transform::IDENTITY;
        t.position = Vec3::new(0.0, 0.0, 5.0);
        t.look_at(Vec3::ZERO, Vec3::Y);

        let forward = -(t.rotation * Vec3::Z);
        assert!((forward.z + 1.0).abs() < 0.1);
    }

    use super::super::Vec4;
}