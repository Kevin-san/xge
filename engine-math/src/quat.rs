#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn from_rotation_x(angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        Self { x: s, y: 0.0, z: 0.0, w: c }
    }

    pub const fn from_rotation_y(angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        Self { x: 0.0, y: s, z: 0.0, w: c }
    }

    pub const fn from_rotation_z(angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();
        let c = half_angle.cos();
        Self { x: 0.0, y: 0.0, z: s, w: c }
    }

    pub const fn from_euler(euler: crate::Euler) -> Self {
        let (pitch, yaw, roll) = euler.to_radians();
        let half_pitch = pitch * 0.5;
        let half_yaw = yaw * 0.5;
        let half_roll = roll * 0.5;

        let sin_p = half_pitch.sin();
        let cos_p = half_pitch.cos();
        let sin_y = half_yaw.sin();
        let cos_y = half_yaw.cos();
        let sin_r = half_roll.sin();
        let cos_r = half_roll.cos();

        Self {
            x: sin_r * cos_p * cos_y - cos_r * sin_p * sin_y,
            y: cos_r * sin_p * cos_y + sin_r * cos_p * sin_y,
            z: cos_r * cos_p * sin_y - sin_r * sin_p * cos_y,
            w: cos_r * cos_p * cos_y + sin_r * sin_p * sin_y,
        }
    }

    pub fn to_euler(&self) -> crate::Euler {
        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        let sinp = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if sinp.abs() >= 1.0 {
            core::f32::consts::FRAC_PI_2 * sinp.signum()
        } else {
            sinp.asin()
        };

        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        crate::Euler::from_radians(pitch, yaw, roll)
    }

    pub fn mul(self, other: Self) -> Self {
        Self {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }

    pub fn inverse(&self) -> Self {
        let len_sq = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if len_sq == 0.0 {
            Self::IDENTITY
        } else {
            let inv_len = 1.0 / len_sq;
            Self {
                x: -self.x * inv_len,
                y: -self.y * inv_len,
                z: -self.z * inv_len,
                w: self.w * inv_len,
            }
        }
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if len == 0.0 {
            Self::IDENTITY
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        }
    }

    pub fn slerp(self, other: Self, t: f32) -> Self {
        let mut dot = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        
        let (other, dot) = if dot < 0.0 {
            (Self {
                x: -other.x,
                y: -other.y,
                z: -other.z,
                w: -other.w,
            }, -dot)
        } else {
            (other, dot)
        };

        let (scale0, scale1) = if dot > 0.9995 {
            (1.0 - t, t)
        } else {
            let theta = dot.acos();
            let sin_theta = theta.sin();
            let t_theta = t * theta;
            (
                ((1.0 - t) * theta).sin() / sin_theta,
                t_theta.sin() / sin_theta,
            )
        };

        Self {
            x: scale0 * self.x + scale1 * other.x,
            y: scale0 * self.y + scale1 * other.y,
            z: scale0 * self.z + scale1 * other.z,
            w: scale0 * self.w + scale1 * other.w,
        }
    }

    pub fn nlerp(self, other: Self, t: f32) -> Self {
        let result = Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            w: self.w + (other.w - self.w) * t,
        };
        result.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quat_identity() {
        assert_eq!(Quat::IDENTITY, Quat::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn quat_mul_identity() {
        let q = Quat::from_rotation_x(core::f32::consts::FRAC_PI_2);
        assert_eq!(q.mul(Quat::IDENTITY), q);
    }

    #[test]
    fn quat_inverse() {
        let q = Quat::from_rotation_x(core::f32::consts::FRAC_PI_2);
        let inv = q.inverse();
        let result = q.mul(inv);
        assert!((result.w - 1.0).abs() < 1e-5);
        assert!(result.x.abs() < 1e-5);
        assert!(result.y.abs() < 1e-5);
        assert!(result.z.abs() < 1e-5);
    }

    #[test]
    fn quat_slerp() {
        let a = Quat::IDENTITY;
        let b = Quat::from_rotation_x(core::f32::consts::PI);
        let q = a.slerp(b, 0.5);
        assert!((q.w - 0.0).abs() < 1e-5);
        assert!((q.x - 1.0).abs() < 1e-5);
    }
}
