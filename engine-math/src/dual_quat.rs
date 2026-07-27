use core::ops::{Add, Mul};

use crate::Mat4;
use crate::Quat;
use crate::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DualQuat {
    pub real: Quat,
    pub dual: Quat,
}

impl DualQuat {
    pub const IDENTITY: Self = Self {
        real: Quat::IDENTITY,
        dual: Quat::ZERO,
    };

    #[inline]
    pub fn from_translation(v: Vec3) -> Self {
        Self {
            real: Quat::IDENTITY,
            dual: Quat {
                x: v.x * 0.5,
                y: v.y * 0.5,
                z: v.z * 0.5,
                w: 0.0,
            },
        }
    }

    #[inline]
    pub fn from_rotation(q: Quat) -> Self {
        Self {
            real: q,
            dual: Quat::ZERO,
        }
    }

    #[inline]
    pub fn from_rotation_translation(q: Quat, t: Vec3) -> Self {
        let t_quat = Quat {
            x: t.x,
            y: t.y,
            z: t.z,
            w: 0.0,
        };
        let dual = t_quat * q;
        Self {
            real: q,
            dual: Quat {
                x: dual.x * 0.5,
                y: dual.y * 0.5,
                z: dual.z * 0.5,
                w: dual.w * 0.5,
            },
        }
    }

    #[inline]
    pub fn from_mat4(m: Mat4) -> Self {
        let c = m.cols;
        let r00 = c[0][0];
        let r01 = c[1][0];
        let r02 = c[2][0];
        let r10 = c[0][1];
        let r11 = c[1][1];
        let r12 = c[2][1];
        let r20 = c[0][2];
        let r21 = c[1][2];
        let r22 = c[2][2];

        let rotation = mat3_to_quat(r00, r01, r02, r10, r11, r12, r20, r21, r22);
        let translation = Vec3::new(c[3][0], c[3][1], c[3][2]);

        Self::from_rotation_translation(rotation, translation)
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Self {
        Self {
            real: Quat {
                x: self.real.x + other.real.x,
                y: self.real.y + other.real.y,
                z: self.real.z + other.real.z,
                w: self.real.w + other.real.w,
            },
            dual: Quat {
                x: self.dual.x + other.dual.x,
                y: self.dual.y + other.dual.y,
                z: self.dual.z + other.dual.z,
                w: self.dual.w + other.dual.w,
            },
        }
    }

    #[inline]
    pub fn scale(self, s: f32) -> Self {
        Self {
            real: Quat {
                x: self.real.x * s,
                y: self.real.y * s,
                z: self.real.z * s,
                w: self.real.w * s,
            },
            dual: Quat {
                x: self.dual.x * s,
                y: self.dual.y * s,
                z: self.dual.z * s,
                w: self.dual.w * s,
            },
        }
    }

    #[inline]
    pub fn norm(self) -> f32 {
        let n = self.real.x * self.real.x
            + self.real.y * self.real.y
            + self.real.z * self.real.z
            + self.real.w * self.real.w;
        n.sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let n = self.norm();
        if n > 0.0 {
            let inv_n = 1.0 / n;
            Self {
                real: Quat {
                    x: self.real.x * inv_n,
                    y: self.real.y * inv_n,
                    z: self.real.z * inv_n,
                    w: self.real.w * inv_n,
                },
                dual: Quat {
                    x: self.dual.x * inv_n,
                    y: self.dual.y * inv_n,
                    z: self.dual.z * inv_n,
                    w: self.dual.w * inv_n,
                },
            }
        } else {
            Self::IDENTITY
        }
    }

    #[inline]
    pub fn conjugate(self) -> Self {
        Self {
            real: Quat {
                x: -self.real.x,
                y: -self.real.y,
                z: -self.real.z,
                w: self.real.w,
            },
            dual: Quat {
                x: -self.dual.x,
                y: -self.dual.y,
                z: -self.dual.z,
                w: self.dual.w,
            },
        }
    }

    #[inline]
    pub fn inverse(self) -> Self {
        self.conjugate()
    }

    #[inline]
    pub fn sclerp(self, other: Self, t: f32) -> Self {
        let mut other = other;
        let dot = self.real.x * other.real.x
            + self.real.y * other.real.y
            + self.real.z * other.real.z
            + self.real.w * other.real.w;

        if dot < 0.0 {
            other = Self {
                real: Quat {
                    x: -other.real.x,
                    y: -other.real.y,
                    z: -other.real.z,
                    w: -other.real.w,
                },
                dual: Quat {
                    x: -other.dual.x,
                    y: -other.dual.y,
                    z: -other.dual.z,
                    w: -other.dual.w,
                },
            };
        }

        let result = Self {
            real: Quat {
                x: self.real.x * (1.0 - t) + other.real.x * t,
                y: self.real.y * (1.0 - t) + other.real.y * t,
                z: self.real.z * (1.0 - t) + other.real.z * t,
                w: self.real.w * (1.0 - t) + other.real.w * t,
            },
            dual: Quat {
                x: self.dual.x * (1.0 - t) + other.dual.x * t,
                y: self.dual.y * (1.0 - t) + other.dual.y * t,
                z: self.dual.z * (1.0 - t) + other.dual.z * t,
                w: self.dual.w * (1.0 - t) + other.dual.w * t,
            },
        };
        result.normalize()
    }

    #[inline]
    pub fn to_mat4(&self) -> Mat4 {
        let n = self.norm();
        if n <= 0.0 {
            return Mat4::IDENTITY;
        }
        let inv_n = 1.0 / n;
        let real = Quat {
            x: self.real.x * inv_n,
            y: self.real.y * inv_n,
            z: self.real.z * inv_n,
            w: self.real.w * inv_n,
        };
        let dual = Quat {
            x: self.dual.x * inv_n,
            y: self.dual.y * inv_n,
            z: self.dual.z * inv_n,
            w: self.dual.w * inv_n,
        };

        let conj_real = Quat {
            x: -real.x,
            y: -real.y,
            z: -real.z,
            w: real.w,
        };
        let t_quat = dual * conj_real;
        let translation = Vec3::new(2.0 * t_quat.x, 2.0 * t_quat.y, 2.0 * t_quat.z);

        Mat4::from_translation(translation) * Mat4::from_quat(real)
    }

    #[inline]
    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        let mat = self.to_mat4();
        let v = crate::Vec4::new(p.x, p.y, p.z, 1.0);
        let result = mat.mul_vec4(v);
        Vec3::new(result.x, result.y, result.z)
    }
}

impl Add for DualQuat {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        self.add(other)
    }
}

impl Mul<f32> for DualQuat {
    type Output = Self;
    #[inline]
    fn mul(self, s: f32) -> Self {
        self.scale(s)
    }
}

impl Mul<DualQuat> for DualQuat {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real,
            dual: self.real * other.dual + self.dual * other.real,
        }
    }
}

#[inline]
#[allow(clippy::too_many_arguments)]
fn mat3_to_quat(
    r00: f32,
    r01: f32,
    r02: f32,
    r10: f32,
    r11: f32,
    r12: f32,
    r20: f32,
    r21: f32,
    r22: f32,
) -> Quat {
    let trace = r00 + r11 + r22;

    if trace > 0.0 {
        let s = 0.5 / (trace + 1.0).sqrt();
        Quat {
            x: (r21 - r12) * s,
            y: (r02 - r20) * s,
            z: (r10 - r01) * s,
            w: 0.25 / s,
        }
    } else if r00 > r11 && r00 > r22 {
        let s = 2.0 * (1.0 + r00 - r11 - r22).sqrt();
        Quat {
            x: 0.25 * s,
            y: (r01 + r10) / s,
            z: (r02 + r20) / s,
            w: (r21 - r12) / s,
        }
    } else if r11 > r22 {
        let s = 2.0 * (1.0 + r11 - r00 - r22).sqrt();
        Quat {
            x: (r01 + r10) / s,
            y: 0.25 * s,
            z: (r12 + r21) / s,
            w: (r02 - r20) / s,
        }
    } else {
        let s = 2.0 * (1.0 + r22 - r00 - r11).sqrt();
        Quat {
            x: (r02 + r20) / s,
            y: (r12 + r21) / s,
            z: 0.25 * s,
            w: (r10 - r01) / s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-5;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPS
    }

    fn approx_eq_vec3(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    fn approx_eq_quat(a: Quat, b: Quat) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w)
    }

    fn approx_eq_mat4(a: Mat4, b: Mat4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !approx_eq(a.cols[i][j], b.cols[i][j]) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_identity() {
        let dq = DualQuat::IDENTITY;
        assert_eq!(dq.real, Quat::IDENTITY);
        assert_eq!(dq.dual, Quat::ZERO);
        let n = dq.norm();
        assert!(approx_eq(n, 1.0));
    }

    #[test]
    fn test_from_translation() {
        let t = Vec3::new(1.0, 2.0, 3.0);
        let dq = DualQuat::from_translation(t);
        assert_eq!(dq.real, Quat::IDENTITY);
        assert!(approx_eq(dq.dual.x, 0.5));
        assert!(approx_eq(dq.dual.y, 1.0));
        assert!(approx_eq(dq.dual.z, 1.5));
        assert!(approx_eq(dq.dual.w, 0.0));
    }

    #[test]
    fn test_from_rotation() {
        let q = Quat::from_axis_angle(Vec3::Z, core::f32::consts::FRAC_PI_2);
        let dq = DualQuat::from_rotation(q);
        assert_eq!(dq.real, q);
        assert_eq!(dq.dual, Quat::ZERO);
    }

    #[test]
    fn test_from_rotation_translation() {
        let q = Quat::from_axis_angle(Vec3::Y, core::f32::consts::FRAC_PI_4);
        let t = Vec3::new(5.0, 10.0, 15.0);
        let dq = DualQuat::from_rotation_translation(q, t);

        let mat = dq.to_mat4();
        let expected = Mat4::from_translation(t) * Mat4::from_quat(q);
        assert!(
            approx_eq_mat4(mat, expected),
            "from_rotation_translation roundtrip failed"
        );
    }

    #[test]
    fn test_translation_roundtrip_mat4() {
        let t = Vec3::new(3.0, -7.0, 11.0);
        let dq = DualQuat::from_translation(t);
        let mat = dq.to_mat4();
        let expected = Mat4::from_translation(t);
        assert!(
            approx_eq_mat4(mat, expected),
            "translation roundtrip failed"
        );
    }

    #[test]
    fn test_rotation_roundtrip_mat4() {
        let q = Quat::from_axis_angle(Vec3::X, core::f32::consts::FRAC_PI_3);
        let dq = DualQuat::from_rotation_translation(q, Vec3::ZERO);
        let mat = dq.to_mat4();
        let expected = Mat4::from_quat(q);
        assert!(
            approx_eq_mat4(mat, expected),
            "rotation roundtrip failed"
        );
    }

    #[test]
    fn test_roundtrip_mat4_both() {
        let q = Quat::from_axis_angle(Vec3::Z, core::f32::consts::FRAC_PI_2);
        let t = Vec3::new(1.0, 2.0, 3.0);
        let dq = DualQuat::from_rotation_translation(q, t);
        let mat = dq.to_mat4();
        let back = DualQuat::from_mat4(mat);
        let mat2 = back.to_mat4();
        assert!(
            approx_eq_mat4(mat, mat2),
            "mat4 roundtrip failed"
        );
    }

    #[test]
    fn test_from_mat4_pure_translation() {
        let t = Vec3::new(4.0, -2.0, 9.0);
        let mat = Mat4::from_translation(t);
        let dq = DualQuat::from_mat4(mat);
        let result = dq.to_mat4();
        assert!(approx_eq_mat4(result, mat));
    }

    #[test]
    fn test_from_mat4_pure_rotation() {
        let q = Quat::from_axis_angle(Vec3::Y, core::f32::consts::FRAC_PI_2);
        let mat = Mat4::from_quat(q);
        let dq = DualQuat::from_mat4(mat);
        let result = dq.to_mat4();
        assert!(approx_eq_mat4(result, mat));
    }

    #[test]
    fn test_sclerp_at_zero() {
        let q1 = Quat::from_axis_angle(Vec3::Z, 0.1);
        let q2 = Quat::from_axis_angle(Vec3::Z, 0.5);
        let dq1 = DualQuat::from_rotation(q1);
        let dq2 = DualQuat::from_rotation(q2);
        let result = dq1.sclerp(dq2, 0.0);
        assert!(approx_eq_quat(result.real, dq1.real));
    }

    #[test]
    fn test_sclerp_at_one() {
        let q1 = Quat::from_axis_angle(Vec3::Z, 0.1);
        let q2 = Quat::from_axis_angle(Vec3::Z, 0.5);
        let dq1 = DualQuat::from_rotation(q1);
        let dq2 = DualQuat::from_rotation(q2);
        let result = dq1.sclerp(dq2, 1.0);
        assert!(approx_eq_quat(result.real, dq2.real));
    }

    #[test]
    fn test_sclerp_halfway() {
        let q1 = Quat::IDENTITY;
        let q2 = Quat::from_rotation_z(core::f32::consts::FRAC_PI_2);
        let dq1 = DualQuat::from_rotation(q1);
        let dq2 = DualQuat::from_rotation(q2);
        let result = dq1.sclerp(dq2, 0.5);
        let v = Vec3::X;
        let rotated = result.real * v;
        let expected = (core::f32::consts::FRAC_PI_4).sin();
        assert!(approx_eq(rotated.y, expected), "rotated.y = {}, expected {}", rotated.y, expected);
    }

    #[test]
    fn test_sclerp_shortest_path() {
        let q1 = Quat::from_axis_angle(Vec3::Z, 0.1);
        let q2 = Quat::from_axis_angle(Vec3::Z, core::f32::consts::TAU - 0.1);
        let dq1 = DualQuat::from_rotation(q1);
        let dq2 = DualQuat::from_rotation(q2);
        let result = dq1.sclerp(dq2, 0.5);
        let v = Vec3::X;
        let rotated = result.real * v;
        assert!(approx_eq(rotated.y, 0.0) || approx_eq(rotated.y, -0.0));
    }

    #[test]
    fn test_add() {
        let dq1 = DualQuat::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let dq2 = DualQuat::from_translation(Vec3::new(4.0, 5.0, 6.0));
        let result = dq1 + dq2;
        assert!(approx_eq(result.real.x, 0.0));
        assert!(approx_eq(result.real.w, 2.0));
        assert!(approx_eq(result.dual.x, 2.5));
        assert!(approx_eq(result.dual.y, 3.5));
        assert!(approx_eq(result.dual.z, 4.5));
        assert!(approx_eq(result.dual.w, 0.0));
    }

    #[test]
    fn test_scale() {
        let dq = DualQuat::from_translation(Vec3::new(2.0, 4.0, 6.0));
        let scaled = dq * 2.0;
        assert!(approx_eq(scaled.real.w, 2.0));
        assert!(approx_eq(scaled.dual.x, 2.0));
        assert!(approx_eq(scaled.dual.y, 4.0));
        assert!(approx_eq(scaled.dual.z, 6.0));
    }

    #[test]
    fn test_norm_and_normalize() {
        let dq = DualQuat::IDENTITY;
        let n = dq.norm();
        assert!(approx_eq(n, 1.0));
        let normalized = dq.normalize();
        assert!(approx_eq(normalized.norm(), 1.0));
    }

    #[test]
    fn test_normalize_zero() {
        let dq = DualQuat::default();
        assert_eq!(dq.norm(), 0.0);
        let normalized = dq.normalize();
        assert_eq!(normalized, DualQuat::IDENTITY);
    }

    #[test]
    fn test_conjugate() {
        let q = Quat::from_axis_angle(Vec3::Z, 0.5);
        let dq = DualQuat::from_rotation_translation(q, Vec3::new(1.0, 2.0, 3.0));
        let conj = dq.conjugate();
        let product = dq * conj;
        assert!(approx_eq(product.real.x, 0.0));
        assert!(approx_eq(product.real.y, 0.0));
        assert!(approx_eq(product.real.z, 0.0));
        assert!(approx_eq(product.real.w, 1.0));
    }

    #[test]
    fn test_inverse() {
        let q = Quat::from_axis_angle(Vec3::Y, core::f32::consts::FRAC_PI_4);
        let t = Vec3::new(5.0, -3.0, 8.0);
        let dq = DualQuat::from_rotation_translation(q, t);
        let inv = dq.inverse();
        let result = dq * inv;
        let mat = result.to_mat4();
        let expected = Mat4::IDENTITY;
        assert!(
            approx_eq_mat4(mat, expected),
            "inverse should produce identity transform"
        );
    }

    #[test]
    fn test_composition() {
        let t1 = DualQuat::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let t2 = DualQuat::from_translation(Vec3::new(0.0, 2.0, 0.0));
        let combined = t1 * t2;
        let mat = combined.to_mat4();
        let p = mat.mul_vec4(crate::Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert!(approx_eq(p.x, 1.0));
        assert!(approx_eq(p.y, 2.0));
        assert!(approx_eq(p.z, 0.0));
    }

    #[test]
    fn test_transform_point() {
        let q = Quat::from_axis_angle(Vec3::X, core::f32::consts::FRAC_PI_2);
        let t = Vec3::new(1.0, 2.0, 3.0);
        let dq = DualQuat::from_rotation_translation(q, t);
        let p = Vec3::new(0.0, 1.0, 0.0);
        let result = dq.transform_point(p);
        let mat = Mat4::from_translation(t) * Mat4::from_quat(q);
        let expected = mat.mul_vec4(crate::Vec4::new(p.x, p.y, p.z, 1.0));
        assert!(approx_eq_vec3(result, expected.xyz()));
    }

    #[test]
    fn test_transform_point_identity() {
        let dq = DualQuat::IDENTITY;
        let p = Vec3::new(5.0, -3.0, 7.0);
        let result = dq.transform_point(p);
        assert!(approx_eq_vec3(result, p));
    }

    #[test]
    fn test_linear_blending() {
        let q1 = Quat::from_axis_angle(Vec3::Z, 0.2);
        let t1 = Vec3::new(0.0, 0.0, 0.0);
        let q2 = Quat::from_axis_angle(Vec3::Z, 0.8);
        let t2 = Vec3::new(10.0, 0.0, 0.0);

        let dq1 = DualQuat::from_rotation_translation(q1, t1);
        let dq2 = DualQuat::from_rotation_translation(q2, t2);

        let blended = (dq1 * 0.4 + dq2 * 0.6).normalize();
        let _mat = blended.to_mat4();

        let v = Vec3::X;
        let rotated = blended.real.normalize() * v;
        assert!(approx_eq(rotated.length(), 1.0));
    }

    #[test]
    fn test_composition_rotation_translation() {
        let rot = DualQuat::from_rotation(Quat::from_axis_angle(Vec3::Z, core::f32::consts::FRAC_PI_2));
        let trans = DualQuat::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let combined = rot * trans;
        let result = combined.transform_point(Vec3::ZERO);
        assert!(approx_eq(result.x, 0.0));
        assert!(approx_eq(result.y, 1.0));
    }

    #[test]
    fn test_dual_quat_triple_composition() {
        let t1 = DualQuat::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let t2 = DualQuat::from_translation(Vec3::new(0.0, 2.0, 0.0));
        let t3 = DualQuat::from_translation(Vec3::new(0.0, 0.0, 3.0));
        let combined = t1 * t2 * t3;
        let p = combined.transform_point(Vec3::ZERO);
        assert!(approx_eq_vec3(p, Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_dual_quat_sclerp_endpoints() {
        let dq1 = DualQuat::from_rotation_translation(
            Quat::IDENTITY,
            Vec3::new(0.0, 0.0, 0.0),
        );
        let dq2 = DualQuat::from_rotation_translation(
            Quat::from_axis_angle(Vec3::Z, 0.5),
            Vec3::new(10.0, 0.0, 0.0),
        );
        let r0 = dq1.sclerp(dq2, 0.0);
        assert!(approx_eq(r0.real.x, dq1.real.x));
        assert!(approx_eq(r0.real.w, dq1.real.w));
        let r1 = dq1.sclerp(dq2, 1.0);
        assert!(approx_eq(r1.real.x, dq2.real.x));
        assert!(approx_eq(r1.real.w, dq2.real.w));
    }

    #[test]
    fn test_dual_quat_norm_preserved_after_normalize() {
        let dq = DualQuat::from_rotation_translation(
            Quat::from_axis_angle(Vec3::X, 1.0),
            Vec3::new(3.0, -2.0, 7.0),
        );
        let normalized = dq.normalize();
        let n = normalized.norm();
        assert!(approx_eq(n, 1.0), "Normalized dq norm should be 1.0, got {}", n);
    }
}