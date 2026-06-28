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
                [
                    1.0 - 2.0 * (y * y + z * z),
                    2.0 * (x * y + z * w),
                    2.0 * (x * z - y * w),
                    0.0,
                ],
                [
                    2.0 * (x * y - z * w),
                    1.0 - 2.0 * (x * x + z * z),
                    2.0 * (y * z + x * w),
                    0.0,
                ],
                [
                    2.0 * (x * z + y * w),
                    2.0 * (y * z - x * w),
                    1.0 - 2.0 * (x * x + y * y),
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn mul_vec4(&self, v: Vec4) -> Vec4 {
        let x = self.cols[0][0] * v.x
            + self.cols[1][0] * v.y
            + self.cols[2][0] * v.z
            + self.cols[3][0] * v.w;
        let y = self.cols[0][1] * v.x
            + self.cols[1][1] * v.y
            + self.cols[2][1] * v.z
            + self.cols[3][1] * v.w;
        let z = self.cols[0][2] * v.x
            + self.cols[1][2] * v.y
            + self.cols[2][2] * v.z
            + self.cols[3][2] * v.w;
        let w = self.cols[0][3] * v.x
            + self.cols[1][3] * v.y
            + self.cols[2][3] * v.z
            + self.cols[3][3] * v.w;
        Vec4::new(x, y, z, w)
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        Self {
            cols: [
                [
                    self.cols[0][0],
                    self.cols[0][1],
                    self.cols[0][2],
                    self.cols[0][3],
                ],
                [
                    self.cols[1][0],
                    self.cols[1][1],
                    self.cols[1][2],
                    self.cols[1][3],
                ],
                [
                    self.cols[2][0],
                    self.cols[2][1],
                    self.cols[2][2],
                    self.cols[2][3],
                ],
                [
                    self.cols[3][0],
                    self.cols[3][1],
                    self.cols[3][2],
                    self.cols[3][3],
                ],
            ],
        }
    }

    #[inline]
    #[allow(clippy::needless_range_loop, clippy::only_used_in_recursion)]
    pub fn inverse(&self) -> Option<Self> {
        // Column-major matrix: cols[column][row]
        // This is equivalent to the transpose of the standard row-major representation.
        // For a column-major matrix M, M.cols[j][i] gives element at row i, column j.
        //
        // We use Gauss-Jordan elimination to compute the inverse.
        // Start with [M | I] and transform to [I | M^-1].

        let mut aug = [
            [self.cols[0][0], self.cols[1][0], self.cols[2][0], self.cols[3][0], 1.0, 0.0, 0.0, 0.0],
            [self.cols[0][1], self.cols[1][1], self.cols[2][1], self.cols[3][1], 0.0, 1.0, 0.0, 0.0],
            [self.cols[0][2], self.cols[1][2], self.cols[2][2], self.cols[3][2], 0.0, 0.0, 1.0, 0.0],
            [self.cols[0][3], self.cols[1][3], self.cols[2][3], self.cols[3][3], 0.0, 0.0, 0.0, 1.0],
        ];

        // Forward elimination with partial pivoting
        for col in 0..4 {
            // Find pivot row
            let mut max_row = col;
            for row in (col + 1)..4 {
                if aug[row][col].abs() > aug[max_row][col].abs() {
                    max_row = row;
                }
            }

            // Check for singular matrix
            if aug[max_row][col].abs() < 1e-6 {
                return None;
            }

            // Swap rows if needed
            if max_row != col {
                for j in 0..8 {
                    let temp = aug[col][j];
                    aug[col][j] = aug[max_row][j];
                    aug[max_row][j] = temp;
                }
            }

            // Scale pivot row
            let pivot = aug[col][col];
            for j in 0..8 {
                aug[col][j] /= pivot;
            }

            // Eliminate column entries in other rows
            for row in 0..4 {
                if row != col {
                    let factor = aug[row][col];
                    for j in 0..8 {
                        aug[row][j] -= factor * aug[col][j];
                    }
                }
            }
        }

        // Result is in columns 4-7, but we need to transpose back to column-major format
        Some(Self {
            cols: [
                [aug[0][4], aug[1][4], aug[2][4], aug[3][4]],
                [aug[0][5], aug[1][5], aug[2][5], aug[3][5]],
                [aug[0][6], aug[1][6], aug[2][6], aug[3][6]],
                [aug[0][7], aug[1][7], aug[2][7], aug[3][7]],
            ],
        })
    }

    #[inline]
    pub fn to_cols_array(&self) -> [f32; 16] {
        [
            self.cols[0][0],
            self.cols[0][1],
            self.cols[0][2],
            self.cols[0][3],
            self.cols[1][0],
            self.cols[1][1],
            self.cols[1][2],
            self.cols[1][3],
            self.cols[2][0],
            self.cols[2][1],
            self.cols[2][2],
            self.cols[2][3],
            self.cols[3][0],
            self.cols[3][1],
            self.cols[3][2],
            self.cols[3][3],
        ]
    }
}

impl core::ops::Mul for Mat4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Self::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                result.cols[j][i] = self.cols[0][i] * other.cols[j][0]
                    + self.cols[1][i] * other.cols[j][1]
                    + self.cols[2][i] * other.cols[j][2]
                    + self.cols[3][i] * other.cols[j][3];
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

use super::{Quat, Vec3, Vec4};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Mat4::IDENTITY;
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let result = m.mul_vec4(v);
        assert_eq!(result, v);
    }

    #[test]
    fn test_translation() {
        let m = Mat4::from_translation(Vec3::new(5.0, 10.0, 15.0));
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let result = m.mul_vec4(v);
        assert_eq!(result.x, 6.0);
        assert_eq!(result.y, 12.0);
        assert_eq!(result.z, 18.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_scale() {
        let m = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let result = m.mul_vec4(v);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 12.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_rotation_x() {
        let angle = std::f32::consts::FRAC_PI_2;
        let m = Mat4::from_rotation_x(angle);
        let v = Vec4::new(0.0, 1.0, 0.0, 1.0);
        let result = m.mul_vec4(v);
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 0.0).abs() < 1e-6);
        assert!((result.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotation_y() {
        let angle = std::f32::consts::FRAC_PI_2;
        let m = Mat4::from_rotation_y(angle);
        let v = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let result = m.mul_vec4(v);
        // Rotation around Y axis: X axis rotates to -Z axis (for positive angle)
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 0.0).abs() < 1e-6);
        assert!((result.z + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotation_z() {
        let angle = std::f32::consts::FRAC_PI_2;
        let m = Mat4::from_rotation_z(angle);
        let v = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let result = m.mul_vec4(v);
        assert!((result.x - 0.0).abs() < 1e-6);
        assert!((result.y - 1.0).abs() < 1e-6);
        assert!((result.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_transpose() {
        let m = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let t = m.transpose();
        // Check that transpose twice returns original
        let t2 = t.transpose();
        assert_eq!(m.cols, t2.cols);
    }

    #[test]
    fn test_inverse_identity() {
        let m = Mat4::IDENTITY;
        // Identity matrix inverse should work
        assert!(m.inverse().is_some());
    }

    #[test]
    fn test_inverse_translation() {
        let m = Mat4::from_translation(Vec3::new(5.0, 10.0, 15.0));
        // Translation matrix inverse should work
        assert!(m.inverse().is_some());
    }

    #[test]
    fn test_inverse_scale() {
        let m = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        // Scale matrix inverse should work
        assert!(m.inverse().is_some());
    }

    #[test]
    fn test_inverse_zero_scale_returns_none() {
        let m = Mat4::from_scale(Vec3::new(0.0, 1.0, 1.0));
        // Zero scale makes matrix non-invertible
        assert!(m.inverse().is_none());
    }

    #[test]
    fn test_matrix_multiplication_identity() {
        let m = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let result = Mat4::IDENTITY * m;
        assert_eq!(result.cols, m.cols);

        let result2 = m * Mat4::IDENTITY;
        assert_eq!(result2.cols, m.cols);
    }

    #[test]
    fn test_matrix_multiplication_combined() {
        let t = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let s = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let combined = t * s;

        let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
        // Scale first (2,2,2), then translate (3,2,2)
        let result = combined.mul_vec4(v);
        assert!((result.x - 3.0).abs() < 1e-6);
        assert!((result.y - 2.0).abs() < 1e-6);
        assert!((result.z - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_from_quat_identity() {
        let q = Quat::IDENTITY;
        let m = Mat4::from_quat(q);
        // Identity quaternion should produce identity matrix
        assert_eq!(m.cols[0][0], 1.0);
        assert_eq!(m.cols[1][1], 1.0);
        assert_eq!(m.cols[2][2], 1.0);
        assert_eq!(m.cols[3][3], 1.0);
    }

    #[test]
    fn test_to_cols_array() {
        let m = Mat4::IDENTITY;
        let arr = m.to_cols_array();
        assert_eq!(arr.len(), 16);
        assert_eq!(arr[0], 1.0); // col0 row0
        assert_eq!(arr[5], 1.0); // col1 row1
        assert_eq!(arr[10], 1.0); // col2 row2
        assert_eq!(arr[15], 1.0); // col3 row3
    }

    #[test]
    fn test_zero_matrix() {
        let m = Mat4::ZERO;
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let result = m.mul_vec4(v);
        assert_eq!(result, Vec4::ZERO);
    }

    #[test]
    fn test_inverse_translation_verbose() {
        let t = Mat4::from_translation(Vec3::new(3.0, -5.0, 2.0));
        let inv = t.inverse().expect("translation should be invertible");
        let result = t * inv;
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.cols[i][j] - expected).abs() < 1e-5,
                    "M * M^-1 should be identity at [{}][{}]: got {} expected {}",
                    i, j, result.cols[i][j], expected);
            }
        }
    }

    #[test]
    fn test_inverse_scale_verbose() {
        let s = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let inv = s.inverse().expect("scale should be invertible");
        let result = s * inv;
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.cols[i][j] - expected).abs() < 1e-5,
                    "M * M^-1 should be identity at [{}][{}]: got {} expected {}",
                    i, j, result.cols[i][j], expected);
            }
        }
    }

    #[test]
    fn test_inverse_rotation_verbose() {
        let r = Mat4::from_rotation_y(std::f32::consts::FRAC_PI_4);
        let inv = r.inverse().expect("rotation should be invertible");
        let result = r * inv;
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.cols[i][j] - expected).abs() < 1e-5,
                    "M * M^-1 should be identity at [{}][{}]: got {} expected {}",
                    i, j, result.cols[i][j], expected);
            }
        }
    }

    #[test]
    fn test_inverse_combined_transform_verbose() {
        let t = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let r = Mat4::from_rotation_z(std::f32::consts::FRAC_PI_6);
        let s = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let combined = t * r * s;
        let inv = combined.inverse().expect("combined transform should be invertible");
        let result = combined * inv;
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.cols[i][j] - expected).abs() < 1e-4,
                    "M * M^-1 should be identity at [{}][{}]: got {} expected {}",
                    i, j, result.cols[i][j], expected);
            }
        }
    }
}
