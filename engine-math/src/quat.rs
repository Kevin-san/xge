use core::fmt;
use core::ops::Mul;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let half = angle / 2.0;
        Self {
            x: half.sin(),
            y: 0.0,
            z: 0.0,
            w: half.cos(),
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let half = angle / 2.0;
        Self {
            x: 0.0,
            y: half.sin(),
            z: 0.0,
            w: half.cos(),
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let half = angle / 2.0;
        Self {
            x: 0.0,
            y: 0.0,
            z: half.sin(),
            w: half.cos(),
        }
    }

    #[inline]
    pub fn inverse(self) -> Self {
        let len_sq = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if len_sq > 0.0 {
            let inv = 1.0 / len_sq;
            Self {
                x: -self.x * inv,
                y: -self.y * inv,
                z: -self.z * inv,
                w: self.w * inv,
            }
        } else {
            Self::IDENTITY
        }
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            Self::IDENTITY
        }
    }

    #[inline]
    pub fn slerp(self, other: Self, t: f32) -> Self {
        let mut cos_half =
            self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;

        if cos_half < 0.0 {
            cos_half = -cos_half;
            let (ox, oy, oz, ow) = (-other.x, -other.y, -other.z, -other.w);
            Self {
                x: ox,
                y: oy,
                z: oz,
                w: ow,
            }
            .slerp_impl(self, t, cos_half)
        } else {
            self.slerp_impl(other, t, cos_half)
        }
    }

    fn slerp_impl(self, other: Self, t: f32, cos_half: f32) -> Self {
        let half;
        let sin_half = (1.0 - cos_half * cos_half).sqrt();

        if sin_half < 0.001 {
            Self {
                x: self.x * (1.0 - t) + other.x * t,
                y: self.y * (1.0 - t) + other.y * t,
                z: self.z * (1.0 - t) + other.z * t,
                w: self.w * (1.0 - t) + other.w * t,
            }
        } else {
            half = cos_half.acos();
            let a = ((1.0 - t) * half).sin() / sin_half;
            let b = (t * half).sin() / sin_half;
            Self {
                x: self.x * a + other.x * b,
                y: self.y * a + other.y * b,
                z: self.z * a + other.z * b,
                w: self.w * a + other.w * b,
            }
        }
    }

    #[inline]
    pub fn nlerp(self, other: Self, t: f32) -> Self {
        let result = Self {
            x: self.x * (1.0 - t) + other.x * t,
            y: self.y * (1.0 - t) + other.y * t,
            z: self.z * (1.0 - t) + other.z * t,
            w: self.w * (1.0 - t) + other.w * t,
        };
        result.normalize()
    }
}

impl Mul for Quat {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        let qv = Vec3::new(self.x, self.y, self.z);
        let uv = qv.cross(v);
        let uuv = qv.cross(uv);
        uv * (2.0 * self.w) + uuv * 2.0 + v
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Quat({:.2}, {:.2}, {:.2}, {:.2})",
            self.x, self.y, self.z, self.w
        )
    }
}

use super::Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let q = Quat::IDENTITY;
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
        assert_eq!(q.w, 1.0);
    }

    #[test]
    fn test_from_rotation_x() {
        let angle = std::f32::consts::FRAC_PI_2;
        let q = Quat::from_rotation_x(angle);
        
        // Rotate Y axis by 90 degrees around X should give Z axis
        let v = Vec3::Y;
        let result = q * v;
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 0.0).abs() < 1e-6);
        assert!((result.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_rotation_y() {
        let angle = std::f32::consts::FRAC_PI_2;
        let q = Quat::from_rotation_y(angle);
        
        // Rotate X axis by 90 degrees around Y should give -Z axis
        let v = Vec3::X;
        let result = q * v;
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 0.0).abs() < 1e-6);
        assert!((result.z + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_rotation_z() {
        let angle = std::f32::consts::FRAC_PI_2;
        let q = Quat::from_rotation_z(angle);
        
        // Rotate X axis by 90 degrees around Z should give Y axis
        let v = Vec3::X;
        let result = q * v;
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
        assert!((result.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse_identity() {
        let q = Quat::IDENTITY;
        let inv = q.inverse();
        assert_eq!(inv.x, 0.0);
        assert_eq!(inv.y, 0.0);
        assert_eq!(inv.z, 0.0);
        assert_eq!(inv.w, 1.0);
    }

    #[test]
    fn test_inverse_rotation() {
        let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
        let inv = q.inverse();
        
        // q * inv should be identity
        let combined = q * inv;
        assert!((combined.x - 0.0).abs() < 1e-6);
        assert!((combined.y - 0.0).abs() < 1e-6);
        assert!((combined.z - 0.0).abs() < 1e-6);
        assert!((combined.w - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize() {
        let q = Quat { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        let n = q.normalize();
        
        let len = (n.x * n.x + n.y * n.y + n.z * n.z + n.w * n.w).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero() {
        let q = Quat { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let n = q.normalize();
        assert_eq!(n, Quat::IDENTITY);
    }

    #[test]
    fn test_quat_multiplication_identity() {
        let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
        let result = Quat::IDENTITY * q;
        assert!((result.x - q.x).abs() < 1e-6);
        assert!((result.y - q.y).abs() < 1e-6);
        assert!((result.z - q.z).abs() < 1e-6);
        assert!((result.w - q.w).abs() < 1e-6);
    }

    #[test]
    fn test_quat_multiplication_combined() {
        let q1 = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        let q2 = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let combined = q1 * q2;
        
        // Apply combined rotation to X axis
        let v = Vec3::X;
        let result = combined * v;
        // The combined rotation should produce a unit vector
        assert!((result.length() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_slerp_identity() {
        let q1 = Quat::IDENTITY;
        let q2 = Quat::IDENTITY;
        let result = q1.slerp(q2, 0.5);
        assert_eq!(result, Quat::IDENTITY);
    }

    #[test]
    fn test_slerp_halfway() {
        let q1 = Quat::IDENTITY;
        let q2 = Quat::from_rotation_x(std::f32::consts::PI);
        let result = q1.slerp(q2, 0.5);
        
        // Halfway rotation should be 90 degrees around X
        let v = Vec3::Y;
        let rotated = result * v;
        // Y rotated by 90 deg around X -> Z
        assert!((rotated.z - 1.0).abs() < 1e-5 || (rotated.z + 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_nlerp() {
        let q1 = Quat::IDENTITY;
        let q2 = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        let result = q1.nlerp(q2, 0.5);
        
        // nlerp result should be normalized
        let len = (result.x * result.x + result.y * result.y + result.z * result.z + result.w * result.w).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotate_vector() {
        let q = Quat::IDENTITY;
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = q * v;
        assert_eq!(result, v);
    }

    #[test]
    fn test_double_rotation() {
        let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
        let q2 = q * q;
        
        // Two 45 degree rotations = 90 degree rotation
        let v = Vec3::Y;
        let result = q2 * v;
        assert!((result.z - 1.0).abs() < 1e-6);
    }
}
