//! 3D Transform component

use engine_math::{Mat4, Quat, Vec3};

/// 3D Transform representing position, rotation, and scale
#[derive(Clone, Copy, Debug)]
pub struct Transform3D {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Transform3D {
    /// Identity transform
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    #[inline]
    pub fn new() -> Self {
        Self::IDENTITY
    }

    #[inline]
    pub fn from_translation(v: Vec3) -> Self {
        Self {
            translation: v,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_rotation(q: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: q,
            scale: Vec3::ONE,
        }
    }

    #[inline]
    pub fn from_scale(v: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: v,
        }
    }

    /// Compute the transformation matrix (TRS order)
    #[inline]
    pub fn matrix(&self) -> Mat4 {
        let t = Mat4::from_translation(self.translation);
        let r = Mat4::from_quat(self.rotation);
        let s = Mat4::from_scale(self.scale);
        t * r * s
    }

    /// Compute the inverse transformation matrix
    #[inline]
    pub fn inverse_matrix(&self) -> Mat4 {
        let inv_scale = Mat4::from_scale(Vec3::new(
            1.0 / self.scale.x,
            1.0 / self.scale.y,
            1.0 / self.scale.z,
        ));
        let inv_rot = Mat4::from_quat(self.rotation.inverse());
        let inv_trans = Mat4::from_translation(-self.translation);
        inv_scale * inv_rot * inv_trans
    }

    #[inline]
    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    #[inline]
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    #[inline]
    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    #[inline]
    pub fn set_translation(&mut self, v: Vec3) {
        self.translation = v;
    }

    #[inline]
    pub fn set_rotation(&mut self, q: Quat) {
        self.rotation = q;
    }

    #[inline]
    pub fn set_scale(&mut self, v: Vec3) {
        self.scale = v;
    }

    #[inline]
    pub fn translate(&mut self, v: Vec3) {
        self.translation += v;
    }

    #[inline]
    pub fn rotate(&mut self, q: Quat) {
        self.rotation = q * self.rotation;
    }

    #[inline]
    pub fn scale_by(&mut self, v: Vec3) {
        self.scale = Vec3::new(self.scale.x * v.x, self.scale.y * v.y, self.scale.z * v.z);
    }

    /// Orient transform to look at target
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let dir = (target - self.translation).normalize();
        self.look_to(dir, up);
    }

    /// Orient transform to look in direction
    pub fn look_to(&mut self, dir: Vec3, up: Vec3) {
        let forward = dir.normalize();
        let right = up.cross(forward).normalize();
        let new_up = forward.cross(right).normalize();

        // Build rotation matrix and convert to quaternion
        let mat = Mat4 {
            cols: [
                [right.x, right.y, right.z, 0.0],
                [new_up.x, new_up.y, new_up.z, 0.0],
                [forward.x, forward.y, forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        self.rotation = mat_to_quat(mat);
    }

    /// Linear interpolation between two transforms
    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            translation: a.translation.lerp(b.translation, t),
            rotation: a.rotation.nlerp(b.rotation, t),
            scale: Vec3::new(
                a.scale.x + (b.scale.x - a.scale.x) * t,
                a.scale.y + (b.scale.y - a.scale.y) * t,
                a.scale.z + (b.scale.z - a.scale.z) * t,
            ),
        }
    }

    /// Transform a point (applies translation)
    #[inline]
    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        self.rotation * (p * self.scale) + self.translation
    }

    /// Transform a vector (no translation)
    #[inline]
    pub fn transform_vector(&self, v: Vec3) -> Vec3 {
        self.rotation * (v * self.scale)
    }

    /// Transform a direction (no translation, no scale)
    #[inline]
    pub fn transform_direction(&self, v: Vec3) -> Vec3 {
        self.rotation * v
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert rotation matrix to quaternion
fn mat_to_quat(mat: Mat4) -> Quat {
    let trace = mat.cols[0][0] + mat.cols[1][1] + mat.cols[2][2];

    if trace > 0.0 {
        let s = 0.5 / (trace + 1.0).sqrt();
        Quat {
            x: (mat.cols[2][1] - mat.cols[1][2]) * s,
            y: (mat.cols[0][2] - mat.cols[2][0]) * s,
            z: (mat.cols[1][0] - mat.cols[0][1]) * s,
            w: 0.25 / s,
        }
    } else if mat.cols[0][0] > mat.cols[1][1] && mat.cols[0][0] > mat.cols[2][2] {
        let s = 2.0 * (1.0 + mat.cols[0][0] - mat.cols[1][1] - mat.cols[2][2]).sqrt();
        Quat {
            x: 0.25 * s,
            y: (mat.cols[0][1] + mat.cols[1][0]) / s,
            z: (mat.cols[0][2] + mat.cols[2][0]) / s,
            w: (mat.cols[2][1] - mat.cols[1][2]) / s,
        }
    } else if mat.cols[1][1] > mat.cols[2][2] {
        let s = 2.0 * (1.0 + mat.cols[1][1] - mat.cols[0][0] - mat.cols[2][2]).sqrt();
        Quat {
            x: (mat.cols[0][1] + mat.cols[1][0]) / s,
            y: 0.25 * s,
            z: (mat.cols[1][2] + mat.cols[2][1]) / s,
            w: (mat.cols[0][2] - mat.cols[2][0]) / s,
        }
    } else {
        let s = 2.0 * (1.0 + mat.cols[2][2] - mat.cols[0][0] - mat.cols[1][1]).sqrt();
        Quat {
            x: (mat.cols[0][2] + mat.cols[2][0]) / s,
            y: (mat.cols[1][2] + mat.cols[2][1]) / s,
            z: 0.25 * s,
            w: (mat.cols[1][0] - mat.cols[0][1]) / s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_identity() {
        let t = Transform3D::IDENTITY;
        let m = t.matrix();
        assert_eq!(m, Mat4::IDENTITY);
    }

    #[test]
    fn test_transform_translation() {
        let t = Transform3D::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let p = t.transform_point(Vec3::ZERO);
        assert_eq!(p, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_transform_inverse() {
        let t = Transform3D::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        let inv = t.inverse_matrix();
        let result = m * inv;
        // Check if result is approximately identity
        let id = Mat4::IDENTITY;
        for i in 0..4 {
            for j in 0..4 {
                assert!((result.cols[i][j] - id.cols[i][j]).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn test_transform_lerp() {
        let a = Transform3D::from_translation(Vec3::ZERO);
        let b = Transform3D::from_translation(Vec3::ONE);
        let mid = Transform3D::lerp(&a, &b, 0.5);
        assert_eq!(mid.translation(), Vec3::splat(0.5));
    }
}
