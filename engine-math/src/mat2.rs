//! 2x2 矩阵

use crate::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mat2 {
    pub cols: [[f32; 2]; 2],
}

impl Mat2 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0], [0.0, 1.0]],
    };
    pub const ZERO: Self = Self {
        cols: [[0.0, 0.0], [0.0, 0.0]],
    };

    #[inline]
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Self {
            cols: [[m00, m01], [m10, m11]],
        }
    }

    #[inline]
    pub fn from_cols(x: Vec2, y: Vec2) -> Self {
        Self {
            cols: [[x.x, x.y], [y.x, y.y]],
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec2) -> Self {
        Self {
            cols: [[scale.x, 0.0], [0.0, scale.y]],
        }
    }

    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            cols: [[c, s], [-s, c]],
        }
    }

    #[inline]
    pub fn mul_vec2(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.cols[0][0] * v.x + self.cols[1][0] * v.y,
            self.cols[0][1] * v.x + self.cols[1][1] * v.y,
        )
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            cols: [
                [self.cols[0][0], self.cols[1][0]],
                [self.cols[0][1], self.cols[1][1]],
            ],
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.cols[0][0] * self.cols[1][1] - self.cols[1][0] * self.cols[0][1]
    }

    #[inline]
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-6 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Self {
            cols: [
                [self.cols[1][1] * inv_det, -self.cols[0][1] * inv_det],
                [-self.cols[1][0] * inv_det, self.cols[0][0] * inv_det],
            ],
        })
    }

    #[inline]
    pub fn to_cols_array(&self) -> [f32; 4] {
        [
            self.cols[0][0],
            self.cols[0][1],
            self.cols[1][0],
            self.cols[1][1],
        ]
    }
}

impl core::ops::Mul for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            cols: [
                [
                    self.cols[0][0] * rhs.cols[0][0] + self.cols[1][0] * rhs.cols[0][1],
                    self.cols[0][1] * rhs.cols[0][0] + self.cols[1][1] * rhs.cols[0][1],
                ],
                [
                    self.cols[0][0] * rhs.cols[1][0] + self.cols[1][0] * rhs.cols[1][1],
                    self.cols[0][1] * rhs.cols[1][0] + self.cols[1][1] * rhs.cols[1][1],
                ],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Mat2::IDENTITY;
        let v = Vec2::new(3.0, 4.0);
        let result = m.mul_vec2(v);
        assert!((result.x - 3.0).abs() < 1e-6);
        assert!((result.y - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_angle() {
        let m = Mat2::from_angle(core::f32::consts::FRAC_PI_2);
        let v = Vec2::new(1.0, 0.0);
        let result = m.mul_vec2(v);
        assert!(result.x.abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_inverse() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let inv = m.inverse().unwrap();
        let identity = m * inv;
        assert!((identity.cols[0][0] - 1.0).abs() < 1e-5);
        assert!(identity.cols[0][1].abs() < 1e-5);
    }

    #[test]
    fn test_from_scale() {
        let m = Mat2::from_scale(Vec2::new(2.0, 3.0));
        let v = Vec2::new(1.0, 1.0);
        let result = m.mul_vec2(v);
        assert!((result.x - 2.0).abs() < 1e-6);
        assert!((result.y - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_determinant() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert!((m.determinant() - (-2.0)).abs() < 1e-6);
    }
}
