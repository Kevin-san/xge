use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub const IDENTITY: Self = Self {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub const ZERO: Self = Self {
        cols: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ],
    };

    #[inline]
    pub const fn from_translation(v: Vec3) -> Self {
        Self {
            cols: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [v.x, v.y, v.z, 1.0],
            ],
        }
    }

    #[inline]
    pub const fn from_scale(v: Vec3) -> Self {
        Self {
            cols: [
                [v.x, 0.0, 0.0, 0.0],
                [0.0, v.y, 0.0, 0.0],
                [0.0, 0.0, v.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = (angle.sin(), angle.cos());
        Self {
            cols: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, s, 0.0],
                [0.0, -s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = (angle.sin(), angle.cos());
        Self {
            cols: [
                [c, 0.0, -s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = (angle.sin(), angle.cos());
        Self {
            cols: [
                [c, s, 0.0, 0.0],
                [-s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub const fn from_quat(q: Quat) -> Self {
        let x = q.x;
        let y = q.y;
        let z = q.z;
        let w = q.w;
        Self {
            cols: [
                [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + z * w), 2.0 * (x * z - y * w), 0.0],
                [2.0 * (x * y - z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + x * w), 0.0],
                [2.0 * (x * z + y * w), 2.0 * (y * z - x * w), 1.0 - 2.0 * (x * x + y * y), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn mul_vec4(&self, v: Vec4) -> Vec4 {
        let x = self.cols[0][0] * v.x + self.cols[1][0] * v.y + self.cols[2][0] * v.z + self.cols[3][0] * v.w;
        let y = self.cols[0][1] * v.x + self.cols[1][1] * v.y + self.cols[2][1] * v.z + self.cols[3][1] * v.w;
        let z = self.cols[0][2] * v.x + self.cols[1][2] * v.y + self.cols[2][2] * v.z + self.cols[3][2] * v.w;
        let w = self.cols[0][3] * v.x + self.cols[1][3] * v.y + self.cols[2][3] * v.z + self.cols[3][3] * v.w;
        Vec4::new(x, y, z, w)
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            cols: [
                [self.cols[0][0], self.cols[0][1], self.cols[0][2], self.cols[0][3]],
                [self.cols[1][0], self.cols[1][1], self.cols[1][2], self.cols[1][3]],
                [self.cols[2][0], self.cols[2][1], self.cols[2][2], self.cols[2][3]],
                [self.cols[3][0], self.cols[3][1], self.cols[3][2], self.cols[3][3]],
            ],
        }
    }

    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let m = self.cols;
        let mut inv = [[0.0; 4]; 4];

        inv[0][0] = m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3] + m[2][1] * m[1][3] * m[3][2] + m[3][1] * m[1][2] * m[2][3] - m[3][1] * m[1][3] * m[2][2];
        inv[1][0] = -m[1][0] * m[2][2] * m[3][3] + m[1][0] * m[2][3] * m[3][2] + m[2][0] * m[1][2] * m[3][3] - m[2][0] * m[1][3] * m[3][2] - m[3][0] * m[1][2] * m[2][3] + m[3][0] * m[1][3] * m[2][2];
        inv[2][0] = m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3] + m[2][0] * m[1][3] * m[3][1] + m[3][0] * m[1][1] * m[2][3] - m[3][0] * m[1][3] * m[2][1];
        inv[3][0] = -m[1][0] * m[2][1] * m[3][2] + m[1][0] * m[2][2] * m[3][1] + m[2][0] * m[1][1] * m[3][2] - m[2][0] * m[1][2] * m[3][1] - m[3][0] * m[1][1] * m[2][2] + m[3][0] * m[1][2] * m[2][1];

        inv[0][1] = -m[1][1] * m[2][2] * m[3][3] + m[1][1] * m[2][3] * m[3][2] + m[2][1] * m[1][2] * m[3][3] - m[2][1] * m[1][3] * m[3][2] - m[3][1] * m[1][2] * m[2][3] + m[3][1] * m[1][3] * m[2][2];
        inv[1][1] = m[1][0] * m[2][2] * m[3][3] - m[1][0] * m[2][3] * m[3][2] - m[2][0] * m[1][2] * m[3][3] + m[2][0] * m[1][3] * m[3][2] + m[3][0] * m[1][2] * m[2][3] - m[3][0] * m[1][3] * m[2][2];
        inv[2][1] = -m[1][0] * m[2][1] * m[3][3] + m[1][0] * m[2][3] * m[3][1] + m[2][0] * m[1][1] * m[3][3] - m[2][0] * m[1][3] * m[3][1] - m[3][0] * m[1][1] * m[2][3] + m[3][0] * m[1][3] * m[2][1];
        inv[3][1] = m[1][0] * m[2][1] * m[3][2] - m[1][0] * m[2][2] * m[3][1] - m[2][0] * m[1][1] * m[3][2] + m[2][0] * m[1][2] * m[3][1] + m[3][0] * m[1][1] * m[2][2] - m[3][0] * m[1][2] * m[2][1];

        inv[0][2] = m[1][1] * m[2][3] * m[3][0] - m[1][1] * m[2][0] * m[3][3] - m[2][1] * m[1][3] * m[3][0] + m[2][1] * m[1][0] * m[3][3] + m[3][1] * m[1][3] * m[2][0] - m[3][1] * m[1][0] * m[2][3];
        inv[1][2] = -m[1][0] * m[2][3] * m[3][0] + m[1][0] * m[2][0] * m[3][3] + m[2][0] * m[1][3] * m[3][0] - m[2][0] * m[1][0] * m[3][3] - m[3][0] * m[1][3] * m[2][0] + m[3][0] * m[1][0] * m[2][3];
        inv[2][2] = m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3] + m[2][0] * m[1][3] * m[3][1] + m[3][0] * m[1][1] * m[2][3] - m[3][0] * m[1][3] * m[2][1];
        inv[3][2] = -m[1][0] * m[2][1] * m[3][2] + m[1][0] * m[2][2] * m[3][1] + m[2][0] * m[1][1] * m[3][2] - m[2][0] * m[1][2] * m[3][1] - m[3][0] * m[1][1] * m[2][2] + m[3][0] * m[1][2] * m[2][1];

        inv[0][3] = -m[1][1] * m[2][3] * m[3][1] + m[1][1] * m[2][1] * m[3][3] + m[2][1] * m[1][3] * m[3][1] - m[2][1] * m[1][1] * m[3][3] - m[3][1] * m[1][3] * m[2][1] + m[3][1] * m[1][1] * m[2][3];
        inv[1][3] = m[1][0] * m[2][3] * m[3][1] - m[1][0] * m[2][1] * m[3][3] - m[2][0] * m[1][3] * m[3][1] + m[2][0] * m[1][1] * m[3][3] + m[3][0] * m[1][3] * m[2][1] - m[3][0] * m[1][1] * m[2][3];
        inv[2][3] = -m[1][0] * m[2][1] * m[3][3] + m[1][0] * m[2][3] * m[3][1] + m[2][0] * m[1][1] * m[3][3] - m[2][0] * m[1][3] * m[3][1] - m[3][0] * m[1][1] * m[2][3] + m[3][0] * m[1][3] * m[2][1];
        inv[3][3] = m[1][0] * m[2][1] * m[3][2] - m[1][0] * m[2][2] * m[3][1] - m[2][0] * m[1][1] * m[3][2] + m[2][0] * m[1][2] * m[3][1] + m[3][0] * m[1][1] * m[2][2] - m[3][0] * m[1][2] * m[2][1];

        let mut det = m[0][0] * inv[0][0] + m[1][0] * inv[0][1] + m[2][0] * inv[0][2] + m[3][0] * inv[0][3];

        if det.abs() < 1e-10 {
            return None;
        }

        det = 1.0 / det;

        let mut result = Mat4::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                result.cols[i][j] = inv[i][j] * det;
            }
        }

        Some(result)
    }

    #[inline]
    pub fn to_cols_array(&self) -> [f32; 16] {
        [
            self.cols[0][0], self.cols[0][1], self.cols[0][2], self.cols[0][3],
            self.cols[1][0], self.cols[1][1], self.cols[1][2], self.cols[1][3],
            self.cols[2][0], self.cols[2][1], self.cols[2][2], self.cols[2][3],
            self.cols[3][0], self.cols[3][1], self.cols[3][2], self.cols[3][3],
        ]
    }
}

impl core::ops::Mul for Mat4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                result.cols[j][i] = 
                    self.cols[0][i] * other.cols[j][0] +
                    self.cols[1][i] * other.cols[j][1] +
                    self.cols[2][i] * other.cols[j][2] +
                    self.cols[3][i] * other.cols[j][3];
            }
        }
        result
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mat4(\n  [{:.2}, {:.2}, {:.2}, {:.2}]\n  [{:.2}, {:.2}, {:.2}, {:.2}]\n  [{:.2}, {:.2}, {:.2}, {:.2}]\n  [{:.2}, {:.2}, {:.2}, {:.2}])",
            self.cols[0][0], self.cols[1][0], self.cols[2][0], self.cols[3][0],
            self.cols[0][1], self.cols[1][1], self.cols[2][1], self.cols[3][1],
            self.cols[0][2], self.cols[1][2], self.cols[2][2], self.cols[3][2],
            self.cols[0][3], self.cols[1][3], self.cols[2][3], self.cols[3][3],
        )
    }
}

use super::{Vec3, Vec4, Quat};
