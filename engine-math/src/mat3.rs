//! 3x3 矩阵

use crate::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mat3 {
    pub cols: [[f32; 3]; 3],
}

impl Mat3 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    pub const ZERO: Self = Self {
        cols: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
    };

    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        Self { cols: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]] }
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            cols: [
                [scale.x, 0.0, 0.0],
                [0.0, scale.y, 0.0],
                [0.0, 0.0, scale.z],
            ],
        }
    }

    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            cols: [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]],
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            cols: [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]],
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            cols: [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    #[inline]
    pub fn mul_vec3(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.cols[0][0] * v.x + self.cols[1][0] * v.y + self.cols[2][0] * v.z,
            self.cols[0][1] * v.x + self.cols[1][1] * v.y + self.cols[2][1] * v.z,
            self.cols[0][2] * v.x + self.cols[1][2] * v.y + self.cols[2][2] * v.z,
        )
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            cols: [
                [self.cols[0][0], self.cols[1][0], self.cols[2][0]],
                [self.cols[0][1], self.cols[1][1], self.cols[2][1]],
                [self.cols[0][2], self.cols[1][2], self.cols[2][2]],
            ],
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.cols[0][0] * (self.cols[1][1] * self.cols[2][2] - self.cols[2][1] * self.cols[1][2])
            - self.cols[1][0] * (self.cols[0][1] * self.cols[2][2] - self.cols[2][1] * self.cols[0][2])
            + self.cols[2][0] * (self.cols[0][1] * self.cols[1][2] - self.cols[1][1] * self.cols[0][2])
    }

    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-6 {
            return None;
        }
        let inv_det = 1.0 / det;

        let m00 = (self.cols[1][1] * self.cols[2][2] - self.cols[2][1] * self.cols[1][2]) * inv_det;
        let m01 = (self.cols[0][2] * self.cols[2][1] - self.cols[0][1] * self.cols[2][2]) * inv_det;
        let m02 = (self.cols[0][1] * self.cols[1][2] - self.cols[0][2] * self.cols[1][1]) * inv_det;
        let m10 = (self.cols[1][2] * self.cols[2][0] - self.cols[1][0] * self.cols[2][2]) * inv_det;
        let m11 = (self.cols[0][0] * self.cols[2][2] - self.cols[0][2] * self.cols[2][0]) * inv_det;
        let m12 = (self.cols[0][2] * self.cols[1][0] - self.cols[0][0] * self.cols[1][2]) * inv_det;
        let m20 = (self.cols[1][0] * self.cols[2][1] - self.cols[1][1] * self.cols[2][0]) * inv_det;
        let m21 = (self.cols[0][1] * self.cols[2][0] - self.cols[0][0] * self.cols[2][1]) * inv_det;
        let m22 = (self.cols[0][0] * self.cols[1][1] - self.cols[0][1] * self.cols[1][0]) * inv_det;

        Some(Self::new(m00, m01, m02, m10, m11, m12, m20, m21, m22))
    }

    #[inline]
    pub fn to_cols_array(&self) -> [f32; 9] {
        [
            self.cols[0][0], self.cols[0][1], self.cols[0][2],
            self.cols[1][0], self.cols[1][1], self.cols[1][2],
            self.cols[2][0], self.cols[2][1], self.cols[2][2],
        ]
    }
}

impl core::ops::Mul for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..3 {
            for j in 0..3 {
                result.cols[i][j] = self.cols[0][j] * rhs.cols[i][0]
                    + self.cols[1][j] * rhs.cols[i][1]
                    + self.cols[2][j] * rhs.cols[i][2];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Mat3::IDENTITY;
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = m.mul_vec3(v);
        assert!((result.x - 1.0).abs() < 1e-6);
        assert!((result.y - 2.0).abs() < 1e-6);
        assert!((result.z - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_scale() {
        let m = Mat3::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let v = Vec3::ONE;
        let result = m.mul_vec3(v);
        assert!((result.x - 2.0).abs() < 1e-6);
        assert!((result.y - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_inverse() {
        let m = Mat3::from_rotation_x(0.5);
        let inv = m.inverse().unwrap();
        let identity = m * inv;
        assert!((identity.cols[0][0] - 1.0).abs() < 1e-5);
        assert!(identity.cols[0][1].abs() < 1e-5);
    }

    #[test]
    fn test_determinant() {
        let m = Mat3::IDENTITY;
        assert!((m.determinant() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_transpose() {
        let m = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let t = m.transpose();
        assert!((t.cols[0][1] - 4.0).abs() < 1e-6);
        assert!((t.cols[1][0] - 2.0).abs() < 1e-6);
    }
}
