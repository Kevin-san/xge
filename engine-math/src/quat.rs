use core::fmt;
use core::ops::{Add, Mul};

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

    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    #[inline]
    pub fn conjugate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

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

    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half = angle / 2.0;
        let (s, c) = half.sin_cos();
        let axis = axis.normalize();
        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: c,
        }
    }

    #[inline]
    pub fn from_euler(euler: crate::Euler) -> Self {
        let rad = euler.to_radians();
        let (sx, cx) = (rad.x * 0.5).sin_cos();
        let (sy, cy) = (rad.y * 0.5).sin_cos();
        let (sz, cz) = (rad.z * 0.5).sin_cos();

        Self {
            x: sx * cy * cz + cx * sy * sz,
            y: cx * sy * cz - sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            w: cx * cy * cz + sx * sy * sz,
        }
    }

    #[inline]
    pub fn to_euler(&self) -> crate::Euler {
        // ZXY intrinsic convention (matches from_euler)
        let sin_roll = 2.0 * (self.w * self.x - self.y * self.z);
        let roll = if sin_roll.abs() >= 1.0 {
            core::f32::consts::FRAC_PI_2.copysign(sin_roll)
        } else {
            sin_roll.asin()
        };

        let pitch = (2.0 * (self.x * self.z + self.w * self.y))
            .atan2(1.0 - 2.0 * (self.x * self.x + self.y * self.y));
        let yaw = (2.0 * (self.x * self.y + self.w * self.z))
            .atan2(1.0 - 2.0 * (self.x * self.x + self.z * self.z));

        crate::Euler::new(
            roll * 180.0 / core::f32::consts::PI,
            pitch * 180.0 / core::f32::consts::PI,
            yaw * 180.0 / core::f32::consts::PI,
        )
    }

    #[inline]
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let q = self.normalize();
        let angle = 2.0 * q.w.acos();
        let s = (1.0 - q.w * q.w).sqrt();
        let axis = if s < 1e-6 {
            Vec3::X
        } else {
            Vec3::new(q.x / s, q.y / s, q.z / s)
        };
        (axis, angle)
    }

    #[inline]
    pub fn squad(q0: Self, q1: Self, q2: Self, t: f32) -> Self {
        let s1 = q0.slerp(q1, t);
        let s2 = q1.slerp(q2, t);
        s1.slerp(s2, t)
    }

    #[inline]
    pub fn log(self) -> Self {
        let w = self.w.clamp(-1.0, 1.0);
        let half_angle = w.acos();
        let sin_half = (1.0 - w * w).sqrt();

        let v = if sin_half > 1e-10 {
            Vec3::new(self.x, self.y, self.z) / sin_half * half_angle
        } else if half_angle < 1e-10 {
            Vec3::new(self.x, self.y, self.z)
        } else {
            Vec3::ZERO
        };

        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }

    #[inline]
    pub fn exp(self) -> Self {
        let v = Vec3::new(self.x, self.y, self.z);
        let len = v.length();

        if len < 1e-10 {
            let len_sq = len * len;
            Self {
                x: self.x * (1.0 - len_sq / 6.0),
                y: self.y * (1.0 - len_sq / 6.0),
                z: self.z * (1.0 - len_sq / 6.0),
                w: 1.0 - len_sq * 0.5,
            }
        } else {
            let s = len.sin();
            let c = len.cos();
            let v = v / len * s;
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
                w: c,
            }
        }
    }

    #[inline]
    pub fn swing_twist_decompose(self, twist_axis: Vec3) -> (Quat, Quat) {
        let axis_unit = twist_axis.normalize();
        let v = Vec3::new(self.x, self.y, self.z);
        let axis_component = axis_unit * v.dot(axis_unit);
        let twist = Quat {
            x: axis_component.x,
            y: axis_component.y,
            z: axis_component.z,
            w: self.w,
        }
        .normalize();
        let swing = self * twist.inverse();
        (swing, twist)
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

impl Add for Quat {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
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
        let q = Quat {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let n = q.normalize();

        let len = (n.x * n.x + n.y * n.y + n.z * n.z + n.w * n.w).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero() {
        let q = Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
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
        let len =
            (result.x * result.x + result.y * result.y + result.z * result.z + result.w * result.w)
                .sqrt();
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

    #[test]
    fn test_from_axis_angle() {
        // 90 degree rotation around Z axis
        let q = Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2);
        let v = Vec3::X;
        let result = q * v;
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
        assert!((result.z - 0.0).abs() < 1e-6);

        // 180 degree rotation around Y axis
        let q2 = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI);
        let v2 = Vec3::X;
        let result2 = q2 * v2;
        assert!((result2.x + 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_from_euler() {
        use crate::Euler;

        // Zero euler angles should produce identity quaternion
        let q = Quat::from_euler(Euler::new(0.0, 0.0, 0.0));
        assert!((q.x - 0.0).abs() < 1e-6);
        assert!((q.y - 0.0).abs() < 1e-6);
        assert!((q.z - 0.0).abs() < 1e-6);
        assert!((q.w - 1.0).abs() < 1e-6);

        // 90 degree rotation around Z
        let q2 = Quat::from_euler(Euler::new(0.0, 0.0, 90.0));
        let v = Vec3::X;
        let result = q2 * v;
        assert!((result.x - 0.0).abs() < 1e-5);
        assert!((result.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_to_euler() {
        use crate::Euler;

        // Identity quaternion should give zero euler angles
        let q = Quat::IDENTITY;
        let euler = q.to_euler();
        assert!((euler.x - 0.0).abs() < 1e-5);
        assert!((euler.y - 0.0).abs() < 1e-5);
        assert!((euler.z - 0.0).abs() < 1e-5);

        // Roundtrip: euler -> quat -> euler
        let euler_in = Euler::new(30.0, 45.0, 60.0);
        let q = Quat::from_euler(euler_in);
        let euler_out = q.to_euler();
        assert!((euler_out.x - euler_in.x).abs() < 1e-4);
        assert!((euler_out.y - euler_in.y).abs() < 1e-4);
        assert!((euler_out.z - euler_in.z).abs() < 1e-4);
    }

    #[test]
    fn test_to_axis_angle() {
        // Identity quaternion: any axis, zero angle
        let (_axis, angle) = Quat::IDENTITY.to_axis_angle();
        assert!((angle - 0.0).abs() < 1e-6);

        // 90 degree rotation around Z
        let q = Quat::from_axis_angle(Vec3::Z, std::f32::consts::FRAC_PI_2);
        let (axis, angle) = q.to_axis_angle();
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        assert!((axis.z - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_quat_identity_euler() {
        let q = Quat::IDENTITY;
        let euler = q.to_euler();
        assert!((euler.x).abs() < 0.001, "identity roll should be 0");
        assert!((euler.y).abs() < 0.001, "identity pitch should be 0");
        assert!((euler.z).abs() < 0.001, "identity yaw should be 0");
    }

    #[test]
    fn test_quat_90_degree_rotation() {
        let q = Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_2);
        let v = q * Vec3::X;
        // 90 degree Y rotation of X gives -Z (right-hand rule)
        assert!((v.z + 1.0).abs() < 0.001, "90 degree Y rotation of X should give -Z, got {:?}", v);
    }

    #[test]
    fn test_quat_double_rotation_is_half_angle() {
        let q = Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_2);
        let q2 = q * q; // Should be 180 degree rotation around Y
        let v = q2 * Vec3::X;
        assert!((v.x + 1.0).abs() < 0.001, "double rotation should give -X, got {:?}", v);
    }

    #[test]
    fn test_squad_identity() {
        let result = Quat::squad(Quat::IDENTITY, Quat::IDENTITY, Quat::IDENTITY, 0.5);
        assert!((result.x - Quat::IDENTITY.x).abs() < 1e-6);
        assert!((result.y - Quat::IDENTITY.y).abs() < 1e-6);
        assert!((result.z - Quat::IDENTITY.z).abs() < 1e-6);
        assert!((result.w - Quat::IDENTITY.w).abs() < 1e-6);
    }

    #[test]
    fn test_squad_at_extremes() {
        let q0 = Quat::from_axis_angle(Vec3::X, 0.3);
        let q1 = Quat::from_axis_angle(Vec3::Y, 1.2);
        let q2 = Quat::from_axis_angle(Vec3::Z, 2.0);

        let r0 = Quat::squad(q0, q1, q2, 0.0);
        assert!((r0.x - q0.x).abs() < 1e-5);
        assert!((r0.y - q0.y).abs() < 1e-5);
        assert!((r0.z - q0.z).abs() < 1e-5);
        assert!((r0.w - q0.w).abs() < 1e-5);

        let r2 = Quat::squad(q0, q1, q2, 1.0);
        assert!((r2.x - q2.x).abs() < 1e-5);
        assert!((r2.y - q2.y).abs() < 1e-5);
        assert!((r2.z - q2.z).abs() < 1e-5);
        assert!((r2.w - q2.w).abs() < 1e-5);
    }

    #[test]
    fn test_log_exp_roundtrip() {
        let test_angles = [0.01, 0.1, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        for &angle in &test_angles {
            let q = Quat::from_axis_angle(Vec3::new(1.0, 2.0, 3.0).normalize(), angle);
            let l = q.log();
            let e = l.exp();
            assert!(
                (e.x - q.x).abs() < 1e-4,
                "log/exp roundtrip failed for angle={}: x: {} vs {}",
                angle, e.x, q.x
            );
            assert!((e.y - q.y).abs() < 1e-4);
            assert!((e.z - q.z).abs() < 1e-4);
            assert!((e.w - q.w).abs() < 1e-4);
        }
    }

    #[test]
    fn test_swing_twist_decompose() {
        let q = Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_3);
        let (swing, twist) = q.swing_twist_decompose(Vec3::Y);

        let combined = swing * twist;
        assert!((combined.x - q.x).abs() < 1e-5);
        assert!((combined.y - q.y).abs() < 1e-5);
        assert!((combined.z - q.z).abs() < 1e-5);
        assert!((combined.w - q.w).abs() < 1e-5);

        let twist_vec = Vec3::new(twist.x, twist.y, twist.z);
        let twist_dir = twist_vec.normalize();
        assert!(
            (twist_dir.y - 1.0).abs() < 1e-5 || (twist_dir.y + 1.0).abs() < 1e-5,
            "twist should be about Y axis"
        );
    }

    #[test]
    fn test_swing_twist_identity() {
        let (swing, twist) = Quat::IDENTITY.swing_twist_decompose(Vec3::Y);
        assert!((swing.x - Quat::IDENTITY.x).abs() < 1e-6);
        assert!((swing.y - Quat::IDENTITY.y).abs() < 1e-6);
        assert!((swing.z - Quat::IDENTITY.z).abs() < 1e-6);
        assert!((swing.w - Quat::IDENTITY.w).abs() < 1e-6);
        assert!((twist.x - Quat::IDENTITY.x).abs() < 1e-6);
        assert!((twist.y - Quat::IDENTITY.y).abs() < 1e-6);
        assert!((twist.z - Quat::IDENTITY.z).abs() < 1e-6);
        assert!((twist.w - Quat::IDENTITY.w).abs() < 1e-6);
    }
}
