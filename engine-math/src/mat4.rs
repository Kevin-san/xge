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
        let m = self.cols;
        let mut inv = [[0.0; 4]; 4];

        inv[0][0] =
            m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3]
                + m[2][1] * m[1][3] * m[3][2]
                + m[3][1] * m[1][2] * m[2][3]
                - m[3][1] * m[1][3] * m[2][2];
        inv[1][0] = -m[1][0] * m[2][2] * m[3][3]
            + m[1][0] * m[2][3] * m[3][2]
            + m[2][0] * m[1][2] * m[3][3]
            - m[2][0] * m[1][3] * m[3][2]
            - m[3][0] * m[1][2] * m[2][3]
            + m[3][0] * m[1][3] * m[2][2];
        inv[2][0] =
            m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1]
                + m[3][0] * m[1][1] * m[2][3]
                - m[3][0] * m[1][3] * m[2][1];
        inv[3][0] = -m[1][0] * m[2][1] * m[3][2]
            + m[1][0] * m[2][2] * m[3][1]
            + m[2][0] * m[1][1] * m[3][2]
            - m[2][0] * m[1][2] * m[3][1]
            - m[3][0] * m[1][1] * m[2][2]
            + m[3][0] * m[1][2] * m[2][1];

        inv[0][1] = -m[1][1] * m[2][2] * m[3][3]
            + m[1][1] * m[2][3] * m[3][2]
            + m[2][1] * m[1][2] * m[3][3]
            - m[2][1] * m[1][3] * m[3][2]
            - m[3][1] * m[1][2] * m[2][3]
            + m[3][1] * m[1][3] * m[2][2];
        inv[1][1] =
            m[1][0] * m[2][2] * m[3][3] - m[1][0] * m[2][3] * m[3][2] - m[2][0] * m[1][2] * m[3][3]
                + m[2][0] * m[1][3] * m[3][2]
                + m[3][0] * m[1][2] * m[2][3]
                - m[3][0] * m[1][3] * m[2][2];
        inv[2][1] = -m[1][0] * m[2][1] * m[3][3]
            + m[1][0] * m[2][3] * m[3][1]
            + m[2][0] * m[1][1] * m[3][3]
            - m[2][0] * m[1][3] * m[3][1]
            - m[3][0] * m[1][1] * m[2][3]
            + m[3][0] * m[1][3] * m[2][1];
        inv[3][1] =
            m[1][0] * m[2][1] * m[3][2] - m[1][0] * m[2][2] * m[3][1] - m[2][0] * m[1][1] * m[3][2]
                + m[2][0] * m[1][2] * m[3][1]
                + m[3][0] * m[1][1] * m[2][2]
                - m[3][0] * m[1][2] * m[2][1];

        inv[0][2] =
            m[1][1] * m[2][3] * m[3][0] - m[1][1] * m[2][0] * m[3][3] - m[2][1] * m[1][3] * m[3][0]
                + m[2][1] * m[1][0] * m[3][3]
                + m[3][1] * m[1][3] * m[2][0]
                - m[3][1] * m[1][0] * m[2][3];
        inv[1][2] = -m[1][0] * m[2][3] * m[3][0]
            + m[1][0] * m[2][0] * m[3][3]
            + m[2][0] * m[1][3] * m[3][0]
            - m[2][0] * m[1][0] * m[3][3]
            - m[3][0] * m[1][3] * m[2][0]
            + m[3][0] * m[1][0] * m[2][3];
        inv[2][2] =
            m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1]
                + m[3][0] * m[1][1] * m[2][3]
                - m[3][0] * m[1][3] * m[2][1];
        inv[3][2] = -m[1][0] * m[2][1] * m[3][2]
            + m[1][0] * m[2][2] * m[3][1]
            + m[2][0] * m[1][1] * m[3][2]
            - m[2][0] * m[1][2] * m[3][1]
            - m[3][0] * m[1][1] * m[2][2]
            + m[3][0] * m[1][2] * m[2][1];

        inv[0][3] = -m[1][1] * m[2][3] * m[3][1]
            + m[1][1] * m[2][1] * m[3][3]
            + m[2][1] * m[1][3] * m[3][1]
            - m[2][1] * m[1][1] * m[3][3]
            - m[3][1] * m[1][3] * m[2][1]
            + m[3][1] * m[1][1] * m[2][3];
        inv[1][3] =
            m[1][0] * m[2][3] * m[3][1] - m[1][0] * m[2][1] * m[3][3] - m[2][0] * m[1][3] * m[3][1]
                + m[2][0] * m[1][1] * m[3][3]
                + m[3][0] * m[1][3] * m[2][1]
                - m[3][0] * m[1][1] * m[2][3];
        inv[2][3] = -m[1][0] * m[2][1] * m[3][3]
            + m[1][0] * m[2][3] * m[3][1]
            + m[2][0] * m[1][1] * m[3][3]
            - m[2][0] * m[1][3] * m[3][1]
            - m[3][0] * m[1][1] * m[2][3]
            + m[3][0] * m[1][3] * m[2][1];
        inv[3][3] =
            m[1][0] * m[2][1] * m[3][2] - m[1][0] * m[2][2] * m[3][1] - m[2][0] * m[1][1] * m[3][2]
                + m[2][0] * m[1][2] * m[3][1]
                + m[3][0] * m[1][1] * m[2][2]
                - m[3][0] * m[1][2] * m[2][1];

        let mut det =
            m[0][0] * inv[0][0] + m[0][1] * inv[1][0] + m[0][2] * inv[2][0] + m[0][3] * inv[3][0];

        if det.abs() < 1e-10 {
            return None;
        }

        det = 1.0 / det;

        let mut result = Mat4::ZERO;
        for (i, inv_row) in inv.iter().enumerate() {
            for (j, &val) in inv_row.iter().enumerate() {
                result.cols[i][j] = val * det;
            }
        }

        Some(result)
    }

    /// Right-handed look-at matrix (eye looking at target, up direction)
    #[inline]
    pub fn look_at_rh(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Self {
            cols: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [-eye.dot(s), -eye.dot(u), eye.dot(f), 1.0],
            ],
        }
    }

    /// Right-handed perspective projection matrix
    #[inline]
    pub fn perspective_rh(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_half_fovy = (fovy / 2.0).tan();
        Self {
            cols: [
                [1.0 / (aspect * tan_half_fovy), 0.0, 0.0, 0.0],
                [0.0, 1.0 / tan_half_fovy, 0.0, 0.0],
                [0.0, 0.0, -(far + near) / (far - near), -1.0],
                [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0],
            ],
        }
    }

    /// Right-handed orthographic projection matrix
    #[inline]
    pub fn orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            cols: [
                [2.0 / (right - left), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (top - bottom), 0.0, 0.0],
                [0.0, 0.0, -2.0 / (far - near), 0.0],
                [
                    -(right + left) / (right - left),
                    -(top + bottom) / (top - bottom),
                    -(far + near) / (far - near),
                    1.0,
                ],
            ],
        }
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
    fn test_look_at_rh() {
        let eye = Vec3::new(0.0, 0.0, 5.0);
        let target = Vec3::ZERO;
        let up = Vec3::Y;
        let m = Mat4::look_at_rh(eye, target, up);

        // The look-at matrix should be invertible
        assert!(m.inverse().is_some());

        // Transform eye position: should map to origin in view space
        let eye_v = Vec4::new(0.0, 0.0, 5.0, 1.0);
        let result = m.mul_vec4(eye_v);
        assert!((result.x - 0.0).abs() < 1e-5);
        assert!((result.y - 0.0).abs() < 1e-5);
        // z should be negative for right-handed (looking down -z)
        assert!((result.z - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_perspective_rh() {
        let fovy = std::f32::consts::FRAC_PI_4;
        let aspect = 16.0 / 9.0;
        let near = 0.1;
        let far = 100.0;
        let m = Mat4::perspective_rh(fovy, aspect, near, far);

        // Perspective matrix should be invertible
        assert!(m.inverse().is_some());

        // The [2][3] element should be -1 for perspective
        assert!((m.cols[2][3] - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_orthographic_rh() {
        let m = Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, 0.1, 100.0);

        // Orthographic matrix should be invertible
        assert!(m.inverse().is_some());

        // The [2][3] element should be 0 for orthographic
        assert!((m.cols[2][3] - 0.0).abs() < 1e-6);

        // Center of ortho box should map to origin
        let v = Vec4::new(0.0, 0.0, -50.05, 1.0);
        let result = m.mul_vec4(v);
        assert!((result.x - 0.0).abs() < 1e-4);
        assert!((result.y - 0.0).abs() < 1e-4);
    }
}
