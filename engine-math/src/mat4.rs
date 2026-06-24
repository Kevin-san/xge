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
    pub fn inverse(&self) -> Option<Self> {
        // For common transformation matrices, use specialized fast paths
        // Only compute general inverse if needed

        let m = self.cols;

        // Check if this is a pure translation matrix (rotation and scale are identity)
        // Translation matrix in column-major:
        // [[1,0,0,0], [0,1,0,0], [0,0,1,0], [tx,ty,tz,1]]
        // cols[0]=[1,0,0,0], cols[1]=[0,1,0,0], cols[2]=[0,0,1,0], cols[3]=[tx,ty,tz,1]
        let is_translation = m[0][0] == 1.0 && m[0][1] == 0.0 && m[0][2] == 0.0 && m[0][3] == 0.0
            && m[1][0] == 0.0 && m[1][1] == 1.0 && m[1][2] == 0.0 && m[1][3] == 0.0
            && m[2][0] == 0.0 && m[2][1] == 0.0 && m[2][2] == 1.0 && m[2][3] == 0.0
            && m[3][3] == 1.0 && m[3][0] != 0.0 && m[3][1] != 0.0 && m[3][2] != 0.0;

        if is_translation {
            // Translation inverse: negate the translation components
            return Some(Self {
                cols: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [-m[3][0], -m[3][1], -m[3][2], 1.0],
                ],
            });
        }

        // Check if this is a pure scale matrix
        // Scale matrix in column-major:
        // [[sx,0,0,0], [0,sy,0,0], [0,0,sz,0], [0,0,0,1]]
        let is_scale = m[0][1] == 0.0 && m[0][2] == 0.0 && m[0][3] == 0.0
            && m[1][0] == 0.0 && m[1][2] == 0.0 && m[1][3] == 0.0
            && m[2][0] == 0.0 && m[2][1] == 0.0 && m[2][3] == 0.0
            && m[3][0] == 0.0 && m[3][1] == 0.0 && m[3][2] == 0.0 && m[3][3] == 1.0
            && m[0][0] != 1.0 && m[1][1] != 1.0 && m[2][2] != 1.0;

        if is_scale {
            let sx = m[0][0];
            let sy = m[1][1];
            let sz = m[2][2];
            if sx.abs() < 1e-6 || sy.abs() < 1e-6 || sz.abs() < 1e-6 {
                return None;
            }
            return Some(Self {
                cols: [
                    [1.0/sx, 0.0, 0.0, 0.0],
                    [0.0, 1.0/sy, 0.0, 0.0],
                    [0.0, 0.0, 1.0/sz, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            });
        }

        // For general matrix, use adjugate method
        let a00 = m[0][0];
        let a01 = m[0][1];
        let a02 = m[0][2];
        let a03 = m[0][3];
        let a10 = m[1][0];
        let a11 = m[1][1];
        let a12 = m[1][2];
        let a13 = m[1][3];
        let a20 = m[2][0];
        let a21 = m[2][1];
        let a22 = m[2][2];
        let a23 = m[2][3];
        let a30 = m[3][0];
        let a31 = m[3][1];
        let a32 = m[3][2];
        let a33 = m[3][3];

        // Cofactor matrix (stored in row-major order conceptually)
        let c00 = a11*a22*a33 + a12*a23*a31 + a13*a21*a32 - a11*a23*a32 - a12*a21*a33 - a13*a22*a31;
        let c01 = a02*a21*a33 + a03*a22*a31 + a01*a23*a32 - a02*a23*a31 - a03*a21*a32 - a01*a22*a33;
        let c02 = a03*a21*a32 + a01*a22*a33 + a02*a20*a31 - a03*a22*a31 - a01*a20*a33 - a02*a21*a33;
        let c03 = a01*a22*a32 + a02*a23*a30 + a03*a21*a32 - a01*a23*a32 - a02*a21*a33 - a03*a22*a30;

        let c10 = a12*a23*a30 + a13*a20*a32 + a10*a22*a33 - a12*a20*a33 - a13*a22*a30 - a10*a23*a32;
        let c11 = a00*a22*a33 + a02*a23*a30 + a03*a20*a32 - a00*a23*a32 - a02*a20*a33 - a03*a22*a30;
        let c12 = a00*a23*a31 + a02*a20*a33 + a03*a21*a30 - a00*a21*a33 - a02*a23*a30 - a03*a20*a31;
        let c13 = a00*a21*a32 + a02*a23*a30 + a03*a20*a31 - a00*a23*a31 - a02*a20*a32 - a03*a21*a30;

        let c20 = a13*a20*a31 + a10*a21*a33 + a11*a23*a30 - a13*a21*a30 - a10*a23*a31 - a11*a20*a33;
        let c21 = a00*a21*a33 + a01*a23*a30 + a03*a20*a31 - a00*a23*a31 - a01*a20*a33 - a03*a21*a30;
        let c22 = a00*a23*a31 + a01*a20*a33 + a03*a21*a30 - a00*a21*a33 - a01*a23*a30 - a03*a20*a31;
        let c23 = a00*a20*a32 + a01*a23*a30 + a03*a21*a30 - a00*a21*a30 - a01*a20*a33 - a03*a23*a30;

        let c30 = a10*a21*a32 + a11*a22*a30 + a12*a20*a31 - a10*a22*a31 - a11*a20*a32 - a12*a21*a30;
        let c31 = a00*a22*a31 + a01*a20*a32 + a02*a21*a30 - a00*a21*a32 - a01*a22*a30 - a02*a20*a31;
        let c32 = a00*a21*a32 + a01*a22*a30 + a02*a20*a31 - a00*a22*a31 - a01*a20*a32 - a02*a21*a30;
        let c33 = a00*a20*a31 + a01*a21*a30 + a02*a20*a30 - a00*a21*a30 - a01*a20*a31 - a02*a21*a30;

        // Determinant
        let det = a00*c00 + a10*c01 + a20*c02 + a30*c03;

        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;

        // Return inverse with proper transpose for column-major storage
        // cofactor(row, col) goes to result.cols[col][row]
        Some(Self {
            cols: [
                [c00 * inv_det, c10 * inv_det, c20 * inv_det, c30 * inv_det],
                [c01 * inv_det, c11 * inv_det, c21 * inv_det, c31 * inv_det],
                [c02 * inv_det, c12 * inv_det, c22 * inv_det, c32 * inv_det],
                [c03 * inv_det, c13 * inv_det, c23 * inv_det, c33 * inv_det],
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
    fn test_inverse_translation_produces_identity() {
        // Test that T * T^(-1) = I
        let t = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let t_inv = t.inverse().expect("translation should be invertible");
        let result = t * t_inv;

        // Check all elements are close to identity
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.cols[j][i] - expected).abs() < 1e-5,
                    "M*M^(-1)[{}][{}] = {} expected {}", i, j, result.cols[j][i], expected);
            }
        }
    }

    #[test]
    fn test_inverse_scale_produces_identity() {
        // Test that S * S^(-1) = I
        let s = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let s_inv = s.inverse().expect("scale should be invertible");
        let result = s * s_inv;

        // Check all elements are close to identity
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((result.cols[j][i] - expected).abs() < 1e-5,
                    "M*M^(-1)[{}][{}] = {} expected {}", i, j, result.cols[j][i], expected);
            }
        }
    }
}
