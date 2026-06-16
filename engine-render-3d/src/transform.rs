//! 变换模块
//!
//! 提供 Transform3D 3D 变换类型，包含位置、旋转、缩放。

use engine_math::{Mat4, Quat, Vec3};

/// 3D 变换
///
/// 包含位置、旋转、缩放的完整 3D 变换。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
    /// 位置
    translation: Vec3,
    /// 旋转（四元数）
    rotation: Quat,
    /// 缩放
    scale: Vec3,
}

impl Transform3D {
    /// 创建单位变换
    pub fn new() -> Self {
        Self::IDENTITY
    }

    /// 从位置创建
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    /// 从旋转创建
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    /// 从缩放创建
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        }
    }

    /// 获取变换矩阵
    pub fn matrix(&self) -> Mat4 {
        let scale_matrix = Mat4::from_scale(self.scale);
        let rotation_matrix = Mat4::from_quat(self.rotation);
        let translation_matrix = Mat4::from_translation(self.translation);
        translation_matrix * rotation_matrix * scale_matrix
    }

    /// 获取逆变换矩阵
    pub fn inverse_matrix(&self) -> Option<Mat4> {
        self.matrix().inverse()
    }

    /// 获取位置
    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    /// 获取旋转
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    /// 获取缩放
    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    /// 设置位置
    pub fn set_translation(&mut self, translation: Vec3) {
        self.translation = translation;
    }

    /// 设置旋转
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    /// 设置缩放
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    /// 平移
    pub fn translate(&mut self, delta: Vec3) {
        self.translation = self.translation + delta;
    }

    /// 旋转
    pub fn rotate(&mut self, delta: Quat) {
        self.rotation = delta * self.rotation;
    }

    /// 缩放
    pub fn scale_by(&mut self, delta: Vec3) {
        self.scale = self.scale * delta;
    }

    /// 看向目标点
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.translation).normalize();
        let right = up.cross(forward).normalize();
        let new_up = forward.cross(right);

        // 从方向向量构建四元数
        let dot = Vec3::new(0.0, 0.0, 1.0).dot(forward);
        if dot < -0.999 {
            self.rotation = Quat::new(0.0, 0.0, 1.0, 0.0);
            return;
        }
        if dot > 0.999 {
            self.rotation = Quat::IDENTITY;
            return;
        }

        let cross = Vec3::new(0.0, 0.0, 1.0).cross(forward);
        self.rotation = Quat::new(cross.x, cross.y, cross.z, 1.0 + dot).normalize();
    }

    /// 线性插值
    pub fn lerp(a: Transform3D, b: Transform3D, t: f32) -> Self {
        Self {
            translation: a.translation + (b.translation - a.translation) * t,
            rotation: a.rotation.slerp(b.rotation, t),
            scale: a.scale + (b.scale - a.scale) * t,
        }
    }

    /// 变换点
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let m = self.matrix();
        let transformed = m.mul_vec4(engine_math::Vec4::new(point.x, point.y, point.z, 1.0));
        Vec3::new(transformed.x, transformed.y, transformed.z)
    }

    /// 变换向量（不考虑平移）
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        let m = self.matrix();
        let transformed = m.mul_vec4(engine_math::Vec4::new(vector.x, vector.y, vector.z, 0.0));
        Vec3::new(transformed.x, transformed.y, transformed.z)
    }

    /// 变换方向向量（不考虑平移和缩放）
    pub fn transform_direction(&self, direction: Vec3) -> Vec3 {
        self.rotation * direction
    }

    /// 分解矩阵到变换分量
    pub fn from_matrix(matrix: &Mat4) -> Option<Self> {
        // 提取缩放
        let scale_x = Vec3::new(matrix.cols[0][0], matrix.cols[0][1], matrix.cols[0][2]).length();
        let scale_y = Vec3::new(matrix.cols[1][0], matrix.cols[1][1], matrix.cols[1][2]).length();
        let scale_z = Vec3::new(matrix.cols[2][0], matrix.cols[2][1], matrix.cols[2][2]).length();

        if scale_x < 0.0001 || scale_y < 0.0001 || scale_z < 0.0001 {
            return None;
        }

        // 提取位置
        let translation = Vec3::new(matrix.cols[3][0], matrix.cols[3][1], matrix.cols[3][2]);

        // 提取旋转（归一化缩放后的矩阵）
        let mut rotation_matrix = *matrix;
        for j in 0..3 {
            rotation_matrix.cols[j][0] /= scale_x;
            rotation_matrix.cols[j][1] /= scale_y;
            rotation_matrix.cols[j][2] /= scale_z;
        }

        let rotation = Quat::from_matrix(&rotation_matrix)?;

        Some(Self {
            translation,
            rotation,
            scale: Vec3::new(scale_x, scale_y, scale_z),
        })
    }

    /// 获取前向方向（默认朝 +Z）
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, 0.0, 1.0)
    }

    /// 获取后向方向
    pub fn backward(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, 0.0, -1.0)
    }

    /// 获取右方向
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::new(1.0, 0.0, 0.0)
    }

    /// 获取左方向
    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::new(-1.0, 0.0, 0.0)
    }

    /// 获取上方向
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, 1.0, 0.0)
    }

    /// 获取下方向
    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, -1.0, 0.0)
    }

    /// 单位变换常量
    pub const IDENTITY: Transform3D = Transform3D {
        translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        rotation: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
    };
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_identity() {
        let t = Transform3D::IDENTITY;
        assert_eq!(t.translation(), Vec3::ZERO);
        assert_eq!(t.scale(), Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_transform_matrix() {
        let t = Transform3D::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let m = t.matrix();
        let point = Vec3::new(0.0, 0.0, 0.0);
        let transformed = t.transform_point(point);
        assert_eq!(transformed, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_transform_lerp() {
        let a = Transform3D::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let b = Transform3D::from_translation(Vec3::new(10.0, 10.0, 10.0));
        let t = Transform3D::lerp(a, b, 0.5);
        assert_eq!(t.translation(), Vec3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_transform_directions() {
        let mut t = Transform3D::new();
        t.rotate(Quat::from_rotation_y(90.0_f32.to_radians()));
        let right = t.right();
        assert!((right.x - 0.0).abs() < 0.01 || (right.z - 1.0).abs() < 0.01);
    }
}
