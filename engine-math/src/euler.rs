use core::fmt;

use crate::Quat;
use crate::Vec3;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Euler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Euler {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn to_quat(&self) -> Quat {
        let cx = self.x.cos();
        let sx = self.x.sin();
        let cy = self.y.cos();
        let sy = self.y.sin();
        let cz = self.z.cos();
        let sz = self.z.sin();

        Quat {
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            w: cx * cy * cz + sx * sy * sz,
        }
    }
}

impl From<Quat> for Euler {
    fn from(q: Quat) -> Self {
        let q = q.normalize();

        let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
        let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        let sinp = 2.0 * (q.w * q.y - q.z * q.x);
        let pitch = if sinp.abs() >= 1.0 {
            sinp.copysign(std::f32::consts::FRAC_PI_2)
        } else {
            sinp.asin()
        };

        let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
        let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        Self {
            x: pitch,
            y: yaw,
            z: roll,
        }
    }
}

impl fmt::Display for Euler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Euler(x: {:.3}, y: {:.3}, z: {:.3})",
            self.x, self.y, self.z
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let e = Euler::new(0.0, 0.0, 0.0);
        assert_eq!(e.x, 0.0);
        assert_eq!(e.y, 0.0);
        assert_eq!(e.z, 0.0);
    }

    #[test]
    fn test_from_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let e = Euler::from_vec3(v);
        assert_eq!(e.x, 1.0);
        assert_eq!(e.y, 2.0);
        assert_eq!(e.z, 3.0);
    }

    #[test]
    fn test_to_vec3() {
        let e = Euler::new(1.0, 2.0, 3.0);
        let v = e.to_vec3();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_to_quat_identity() {
        let e = Euler::new(0.0, 0.0, 0.0);
        let q = e.to_quat();
        assert!((q.x - 0.0).abs() < 1e-6);
        assert!((q.y - 0.0).abs() < 1e-6);
        assert!((q.z - 0.0).abs() < 1e-6);
        assert!((q.w - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_to_quat_90_x() {
        let e = Euler::new(std::f32::consts::FRAC_PI_2, 0.0, 0.0);
        let q = e.to_quat();
        assert!((q.x - 1.0).abs() < 1e-6);
        assert!((q.y - 0.0).abs() < 1e-6);
        assert!((q.z - 0.0).abs() < 1e-6);
        assert!((q.w - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_quat_identity() {
        let q = Quat::IDENTITY;
        let e: Euler = q.into();
        assert!((e.x).abs() < 1e-6);
        assert!((e.y).abs() < 1e-6);
        assert!((e.z).abs() < 1e-6);
    }
}
