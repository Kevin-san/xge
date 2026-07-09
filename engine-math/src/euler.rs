use core::fmt;

use super::{Quat, Vec3};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Euler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Euler {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn from_quat(q: Quat) -> Self {
        let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
        let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        let sinp = 2.0 * (q.w * q.y - q.z * q.x);
        let pitch = if sinp.abs() >= 1.0 {
            sinp.signum() * core::f32::consts::FRAC_PI_2
        } else {
            sinp.asin()
        };

        let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
        let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        Self {
            x: roll,
            y: pitch,
            z: yaw,
        }
    }

    #[inline]
    pub fn to_quat(self) -> Quat {
        Quat::from_euler(self)
    }

    #[inline]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }
}

impl From<Vec3> for Euler {
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Euler> for Vec3 {
    fn from(e: Euler) -> Self {
        Vec3::new(e.x, e.y, e.z)
    }
}

impl fmt::Display for Euler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Euler(roll: {:.2}, pitch: {:.2}, yaw: {:.2})",
            self.x, self.y, self.z
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let e = Euler::ZERO;
        assert_eq!(e.x, 0.0);
        assert_eq!(e.y, 0.0);
        assert_eq!(e.z, 0.0);
    }

    #[test]
    fn test_from_quat_identity() {
        let q = Quat::IDENTITY;
        let e = Euler::from_quat(q);
        assert!(e.x.abs() < 1e-6);
        assert!(e.y.abs() < 1e-6);
        assert!(e.z.abs() < 1e-6);
    }

    #[test]
    fn test_to_quat_and_back() {
        let original = Euler::new(0.3, 0.5, 0.7);
        let q = original.to_quat();
        let back = Euler::from_quat(q);
        assert!((back.x - original.x).abs() < 1e-5);
        assert!((back.y - original.y).abs() < 1e-5);
        assert!((back.z - original.z).abs() < 1e-5);
    }

    #[test]
    fn test_from_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let e: Euler = v.into();
        assert_eq!(e.x, 1.0);
        assert_eq!(e.y, 2.0);
        assert_eq!(e.z, 3.0);
    }

    #[test]
    fn test_to_vec3() {
        let e = Euler::new(1.0, 2.0, 3.0);
        let v: Vec3 = e.into();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_lerp() {
        let a = Euler::ZERO;
        let b = Euler::new(1.0, 2.0, 3.0);
        let result = a.lerp(b, 0.5);
        assert!((result.x - 0.5).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
        assert!((result.z - 1.5).abs() < 1e-6);
    }
}
