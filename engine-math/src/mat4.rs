#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    pub const ZERO: Self = Self([
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);

    pub const fn from_translation(v: crate::Vec3) -> Self {
        Self([
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn from_scale(v: crate::Vec3) -> Self {
        Self([
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn from_rotation_x(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, -s, 0.0],
            [0.0, s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn from_rotation_y(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn from_rotation_z(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self([
            [c, -s, 0.0, 0.0],
            [s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn from_quat(q: crate::Quat) -> Self {
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;

        Self([
            [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
            [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
            [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn look_at_rh(eye: crate::Vec3, target: crate::Vec3, up: crate::Vec3) -> Self {
        let z = (target - eye).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x);

        Self([
            [x.x, y.x, z.x, 0.0],
            [x.y, y.y, z.y, 0.0],
            [x.z, y.z, z.z, 0.0],
            [-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0],
        ])
    }

    pub fn perspective_rh(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fovy / 2.0).tan();
        let nf = 1.0 / (near - far);

        Self([
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) * nf, -1.0],
            [0.0, 0.0, 2.0 * far * near * nf, 0.0],
        ])
    }

    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let lr = 1.0 / (left - right);
        let bt = 1.0 / (bottom - top);
        let nf = 1.0 / (near - far);

        Self([
            [-2.0 * lr, 0.0, 0.0, 0.0],
            [0.0, -2.0 * bt, 0.0, 0.0],
            [0.0, 0.0, 2.0 * nf, 0.0],
            [(left + right) * lr, (bottom + top) * bt, (near + far) * nf, 1.0],
        ])
    }

    pub fn inverse(&self) -> Option<Self> {
        let m = &self.0;
        let (
            (m00, m01, m02, m03),
            (m10, m11, m12, m13),
            (m20, m21, m22, m23),
            (m30, m31, m32, m33),
        ) = (
            (m[0][0], m[0][1], m[0][2], m[0][3]),
            (m[1][0], m[1][1], m[1][2], m[1][3]),
            (m[2][0], m[2][1], m[2][2], m[2][3]),
            (m[3][0], m[3][1], m[3][2], m[3][3]),
        );

        let cofactor00 = m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31) + m13 * (m21 * m32 - m22 * m31);
        let cofactor01 = -(m10 * (m22 * m33 - m23 * m32) - m12 * (m20 * m33 - m23 * m30) + m13 * (m20 * m32 - m22 * m30));
        let cofactor02 = m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30) + m13 * (m20 * m31 - m21 * m30);
        let cofactor03 = -(m10 * (m21 * m32 - m22 * m31) - m11 * (m20 * m32 - m22 * m30) + m12 * (m20 * m31 - m21 * m30));

        let det = m00 * cofactor00 + m01 * cofactor01 + m02 * cofactor02 + m03 * cofactor03;
        
        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(Self([
            [cofactor00 * inv_det, cofactor01 * inv_det, cofactor02 * inv_det, cofactor03 * inv_det],
            [
                -(m01 * (m22 * m33 - m23 * m32) - m02 * (m21 * m33 - m23 * m31) + m03 * (m21 * m32 - m22 * m31)) * inv_det,
                (m00 * (m22 * m33 - m23 * m32) - m02 * (m20 * m33 - m23 * m30) + m03 * (m20 * m32 - m22 * m30)) * inv_det,
                -(m00 * (m21 * m33 - m23 * m31) - m01 * (m20 * m33 - m23 * m30) + m03 * (m20 * m31 - m21 * m30)) * inv_det,
                (m00 * (m21 * m32 - m22 * m31) - m01 * (m20 * m32 - m22 * m30) + m02 * (m20 * m31 - m21 * m30)) * inv_det,
            ],
            [
                (m01 * (m12 * m33 - m13 * m32) - m02 * (m11 * m33 - m13 * m31) + m03 * (m11 * m32 - m12 * m31)) * inv_det,
                -(m00 * (m12 * m33 - m13 * m32) - m02 * (m10 * m33 - m13 * m30) + m03 * (m10 * m32 - m12 * m30)) * inv_det,
                (m00 * (m11 * m33 - m13 * m31) - m01 * (m10 * m33 - m13 * m30) + m03 * (m10 * m31 - m11 * m30)) * inv_det,
                -(m00 * (m11 * m32 - m12 * m31) - m01 * (m10 * m32 - m12 * m30) + m02 * (m10 * m31 - m11 * m30)) * inv_det,
            ],
            [
                -(m01 * (m12 * m23 - m13 * m22) - m02 * (m11 * m23 - m13 * m21) + m03 * (m11 * m22 - m12 * m21)) * inv_det,
                (m00 * (m12 * m23 - m13 * m22) - m02 * (m10 * m23 - m13 * m20) + m03 * (m10 * m22 - m12 * m20)) * inv_det,
                -(m00 * (m11 * m23 - m13 * m21) - m01 * (m10 * m23 - m13 * m20) + m03 * (m10 * m21 - m11 * m20)) * inv_det,
                (m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20) + m02 * (m10 * m21 - m11 * m20)) * inv_det,
            ],
        ]))
    }

    pub fn transpose(&self) -> Self {
        let m = &self.0;
        Self([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ])
    }

    pub fn mul_vec4(&self, v: crate::Vec4) -> crate::Vec4 {
        let m = &self.0;
        crate::Vec4::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3] * v.w,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3] * v.w,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3] * v.w,
            m[3][0] * v.x + m[3][1] * v.y + m[3][2] * v.z + m[3][3] * v.w,
        )
    }

    pub fn to_cols_array(&self) -> [f32; 16] {
        let m = &self.0;
        [
            m[0][0], m[1][0], m[2][0], m[3][0],
            m[0][1], m[1][1], m[2][1], m[3][1],
            m[0][2], m[1][2], m[2][2], m[3][2],
            m[0][3], m[1][3], m[2][3], m[3][3],
        ]
    }
}

impl core::ops::Mul for Mat4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let a = &self.0;
        let b = &other.0;
        let mut result = Self::ZERO;

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.0[i][j] += a[i][k] * b[k][j];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat4_identity() {
        assert_eq!(Mat4::IDENTITY, Mat4::IDENTITY);
    }

    #[test]
    fn mat4_translation() {
        let m = Mat4::from_translation(crate::Vec3::new(1.0, 2.0, 3.0));
        let v = crate::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let result = m.mul_vec4(v);
        assert_eq!(result, crate::Vec4::new(1.0, 2.0, 3.0, 1.0));
    }

    #[test]
    fn mat4_transpose() {
        let m = Mat4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let t = m.transpose();
        assert_eq!(t.0[0][1], 5.0);
        assert_eq!(t.0[1][0], 2.0);
    }

    #[test]
    fn mat4_inverse_identity() {
        let inv = Mat4::IDENTITY.inverse();
        assert!(inv.is_some());
        assert_eq!(inv.unwrap(), Mat4::IDENTITY);
    }

    #[test]
    fn mat4_mul() {
        let a = Mat4::from_translation(crate::Vec3::new(1.0, 0.0, 0.0));
        let b = Mat4::from_translation(crate::Vec3::new(0.0, 1.0, 0.0));
        let c = a * b;
        let v = crate::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let result = c.mul_vec4(v);
        assert_eq!(result, crate::Vec4::new(1.0, 1.0, 0.0, 1.0));
    }
}
