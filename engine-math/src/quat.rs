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

    /// 从分量创建四元数
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
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

    /// 从矩阵创建四元数
    pub fn from_matrix(m: &Mat4) -> Option<Self> {
        let trace = m.cols[0][0] + m.cols[1][1] + m.cols[2][2];

        if trace > 0.0 {
            let s = (trace + 1.0).sqrt();
            let inv_s = 1.0 / s;
            Some(Self {
                x: (m.cols[2][1] - m.cols[1][2]) * inv_s,
                y: (m.cols[0][2] - m.cols[2][0]) * inv_s,
                z: (m.cols[1][0] - m.cols[0][1]) * inv_s,
                w: s * 0.5,
            })
        } else if m.cols[0][0] > m.cols[1][1] && m.cols[0][0] > m.cols[2][2] {
            let s = (1.0 + m.cols[0][0] - m.cols[1][1] - m.cols[2][2]).sqrt();
            let inv_s = 1.0 / s;
            Some(Self {
                x: 0.5 * s,
                y: (m.cols[1][0] + m.cols[0][1]) * inv_s,
                z: (m.cols[0][2] + m.cols[2][0]) * inv_s,
                w: (m.cols[2][1] - m.cols[1][2]) * inv_s,
            })
        } else if m.cols[1][1] > m.cols[2][2] {
            let s = (1.0 + m.cols[1][1] - m.cols[0][0] - m.cols[2][2]).sqrt();
            let inv_s = 1.0 / s;
            Some(Self {
                x: (m.cols[1][0] + m.cols[0][1]) * inv_s,
                y: 0.5 * s,
                z: (m.cols[2][1] + m.cols[1][2]) * inv_s,
                w: (m.cols[0][2] - m.cols[2][0]) * inv_s,
            })
        } else {
            let s = (1.0 + m.cols[2][2] - m.cols[0][0] - m.cols[1][1]).sqrt();
            let inv_s = 1.0 / s;
            Some(Self {
                x: (m.cols[0][2] + m.cols[2][0]) * inv_s,
                y: (m.cols[2][1] + m.cols[1][2]) * inv_s,
                z: 0.5 * s,
                w: (m.cols[1][0] - m.cols[0][1]) * inv_s,
            })
        }
    }

    /// 乘以 Vec4
    pub fn mul_vec4(&self, v: &Vec4) -> Vec4 {
        let qv = Vec3::new(self.x, self.y, self.z);
        let uv = qv.cross(Vec3::new(v.x, v.y, v.z));
        let uuv = qv.cross(uv);
        let uuv = uuv * 2.0 * self.w;
        let uv = uv * 2.0;
        let result = Vec3::new(v.x, v.y, v.z) + uuv + uv;
        Vec4::new(result.x, result.y, result.z, v.w)
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
use super::Vec4;
use super::Mat4;
