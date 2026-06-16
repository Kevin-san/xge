//! 相机模块
//!
//! 提供 Camera3D 透视/正交相机和 Frustum 视锥裁剪。

use engine_math::{Mat4, Quat, Vec3};
use crate::geometry::{AABB, Plane, Ray3, Sphere};

/// 相机类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraType {
    /// 透视相机
    Perspective,
    /// 正交相机
    Orthographic,
}

/// 3D 相机
#[derive(Debug, Clone)]
pub struct Camera3D {
    /// 相机类型
    camera_type: CameraType,
    /// 位置
    position: Vec3,
    /// 旋转（四元数）
    rotation: Quat,
    /// 透视参数
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    /// 正交参数
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl Camera3D {
    /// 创建透视相机
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            camera_type: CameraType::Perspective,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            fov,
            aspect,
            near,
            far,
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
            top: 0.0,
        }
    }

    /// 创建正交相机
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            camera_type: CameraType::Orthographic,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            fov: 0.0,
            aspect: (right - left) / (top - bottom),
            near,
            far,
            left,
            right,
            bottom,
            top,
        }
    }

    /// 获取视角矩阵
    pub fn view_matrix(&self) -> Mat4 {
        let forward = self.forward();
        let right = self.right();
        let up = self.up();

        Mat4 {
            cols: [
                [right.x, up.x, -forward.x, 0.0],
                [right.y, up.y, -forward.y, 0.0],
                [right.z, up.z, -forward.z, 0.0],
                [-right.dot(self.position), -up.dot(self.position), forward.dot(self.position), 1.0],
            ],
        }
    }

    /// 获取投影矩阵
    pub fn projection_matrix(&self) -> Mat4 {
        match self.camera_type {
            CameraType::Perspective => self.perspective_matrix(),
            CameraType::Orthographic => self.orthographic_matrix(),
        }
    }

    /// 获取视角投影矩阵
    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// 获取逆视角矩阵
    pub fn inverse_view(&self) -> Option<Mat4> {
        self.view_matrix().inverse()
    }

    /// 获取逆投影矩阵
    pub fn inverse_projection(&self) -> Option<Mat4> {
        self.projection_matrix().inverse()
    }

    /// 获取逆视角投影矩阵
    pub fn inverse_view_projection(&self) -> Option<Mat4> {
        self.view_projection().inverse()
    }

    /// 获取位置
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// 获取前进方向
    pub fn forward(&self) -> Vec3 {
        let q = self.rotation;
        // 相机默认朝 -Z 方向
        Vec3::new(-2.0 * (q.y * q.z - q.x * q.w), 2.0 * (q.x * q.z + q.y * q.w), 1.0 - 2.0 * (q.x * q.x + q.y * q.y)).normalize()
    }

    /// 获取右方向
    pub fn right(&self) -> Vec3 {
        let q = self.rotation;
        Vec3::new(1.0 - 2.0 * (q.y * q.y + q.z * q.z), 2.0 * (q.x * q.y + q.z * q.w), 2.0 * (q.y * q.z - q.x * q.w)).normalize()
    }

    /// 获取上方向
    pub fn up(&self) -> Vec3 {
        let q = self.rotation;
        Vec3::new(2.0 * (q.x * q.y - q.z * q.w), 1.0 - 2.0 * (q.x * q.x + q.z * q.z), 2.0 * (q.y * q.z + q.x * q.w)).normalize()
    }

    /// 获取 FOV
    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// 获取宽高比
    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    /// 获取近裁剪面
    pub fn near(&self) -> f32 {
        self.near
    }

    /// 获取远裁剪面
    pub fn far(&self) -> f32 {
        self.far
    }

    /// 设置 FOV
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    /// 设置宽高比
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    /// 设置近裁剪面
    pub fn set_near(&mut self, near: f32) {
        self.near = near;
    }

    /// 设置远裁剪面
    pub fn set_far(&mut self, far: f32) {
        self.far = far;
    }

    /// 设置位置
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// 设置旋转
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    /// 看向目标点
    pub fn look_at(&mut self, target: Vec3) {
        let forward = (target - self.position).normalize();
        // 计算四元数（简化版本）
        let up = Vec3::new(0.0, 1.0, 0.0);
        let right = up.cross(forward).normalize();
        let new_up = forward.cross(right);

        // 从方向向量构建旋转
        let dot = Vec3::new(0.0, 0.0, 1.0).dot(forward);
        if dot < -0.999 {
            self.rotation = Quat::new(0.0, 0.0, 1.0, 0.0); // 180度翻转
            return;
        }
        if dot > 0.999 {
            self.rotation = Quat::IDENTITY;
            return;
        }

        let cross = Vec3::new(0.0, 0.0, 1.0).cross(forward);
        self.rotation = Quat::new(cross.x, cross.y, cross.z, 1.0 + dot).normalize();
    }

    /// 朝指定方向看
    pub fn look_to(&mut self, dir: Vec3, up: Vec3) {
        self.position = Vec3::ZERO;
        self.look_at(dir);
    }

    /// 屏幕坐标转世界射线
    pub fn screen_to_world_ray(&self, screen_pos: engine_math::Vec2, screen_size: engine_math::Vec2) -> Ray3 {
        // 将屏幕坐标归一化到 [-1, 1]
        let ndc = engine_math::Vec2::new(
            2.0 * screen_pos.x / screen_size.x - 1.0,
            2.0 * screen_pos.y / screen_size.y - 1.0,
        );

        let inv_proj = match self.inverse_projection() {
            Some(m) => m,
            None => return Ray3::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0)),
        };

        let inv_view = match self.inverse_view() {
            Some(m) => m,
            None => return Ray3::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0)),
        };

        // 近平面点
        let near_point = inv_proj.mul_vec4(engine_math::Vec4::new(ndc.x, ndc.y, -1.0, 1.0));
        let near_point = Vec3::new(near_point.x, near_point.y, near_point.z) / near_point.w;

        // 远平面点
        let far_point = inv_proj.mul_vec4(engine_math::Vec4::new(ndc.x, ndc.y, 1.0, 1.0));
        let far_point = Vec3::new(far_point.x, far_point.y, far_point.z) / far_point.w;

        // 转换到世界空间
        let near_world = inv_view.mul_vec4(engine_math::Vec4::new(near_point.x, near_point.y, near_point.z, 1.0));
        let far_world = inv_view.mul_vec4(engine_math::Vec4::new(far_point.x, far_point.y, far_point.z, 1.0));

        let near_world = Vec3::new(near_world.x, near_world.y, near_world.z);
        let far_world = Vec3::new(far_world.x, far_world.y, far_world.z);

        let direction = (far_world - near_world).normalize();

        Ray3::new(near_world, direction)
    }

    /// 世界坐标转屏幕坐标
    pub fn world_to_screen(&self, world_pos: Vec3, screen_size: engine_math::Vec2) -> engine_math::Vec2 {
        let clip = self.view_projection().mul_vec4(engine_math::Vec4::new(world_pos.x, world_pos.y, world_pos.z, 1.0));
        let ndc = engine_math::Vec2::new(clip.x / clip.w, clip.y / clip.w);
        engine_math::Vec2::new(
            (ndc.x + 1.0) * 0.5 * screen_size.x,
            (1.0 - ndc.y) * 0.5 * screen_size.y,
        )
    }

    /// 获取透视投影矩阵
    fn perspective_matrix(&self) -> Mat4 {
        let f = 1.0 / (self.fov * std::f32::consts::PI / 360.0).tan();
        let nf = 1.0 / (self.near - self.far);

        Mat4 {
            cols: [
                [f / self.aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (self.far + self.near) * nf, -1.0],
                [0.0, 0.0, 2.0 * self.far * self.near * nf, 0.0],
            ],
        }
    }

    /// 获取正交投影矩阵
    fn orthographic_matrix(&self) -> Mat4 {
        let rl = self.right - self.left;
        let tb = self.top - self.bottom;
        let fn_ = self.far - self.near;

        Mat4 {
            cols: [
                [2.0 / rl, 0.0, 0.0, 0.0],
                [0.0, 2.0 / tb, 0.0, 0.0],
                [0.0, 0.0, -2.0 / fn_, 0.0],
                [-(self.right + self.left) / rl, -(self.top + self.bottom) / tb, -(self.far + self.near) / fn_, 1.0],
            ],
        }
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::perspective(60.0, 16.0 / 9.0, 0.1, 1000.0)
    }
}

/// 视锥
#[derive(Debug, Clone)]
pub struct Frustum {
    planes: [Plane; 6],
}

impl Frustum {
    /// 从视角投影矩阵创建视锥
    pub fn from_view_projection(vp: &Mat4) -> Self {
        let cols = vp.to_cols_array();
        // 提取矩阵列
        let m = [
            [cols[0], cols[4], cols[8], cols[12]],
            [cols[1], cols[5], cols[9], cols[13]],
            [cols[2], cols[6], cols[10], cols[14]],
            [cols[3], cols[7], cols[11], cols[15]],
        ];

        let mut planes = [Plane::from_normal_and_point(Vec3::ZERO, Vec3::ZERO); 6];

        // 右平面
        planes[0] = Plane::from_normal_and_point(
            Vec3::new(m[0][3] - m[0][0], m[1][3] - m[1][0], m[2][3] - m[2][0]),
            Vec3::ZERO,
        );
        // 左平面
        planes[1] = Plane::from_normal_and_point(
            Vec3::new(m[0][3] + m[0][0], m[1][3] + m[1][0], m[2][3] + m[2][0]),
            Vec3::ZERO,
        );
        // 底平面
        planes[2] = Plane::from_normal_and_point(
            Vec3::new(m[0][3] + m[0][1], m[1][3] + m[1][1], m[2][3] + m[2][1]),
            Vec3::ZERO,
        );
        // 顶平面
        planes[3] = Plane::from_normal_and_point(
            Vec3::new(m[0][3] - m[0][1], m[1][3] - m[1][1], m[2][3] - m[2][1]),
            Vec3::ZERO,
        );
        // 远平面
        planes[4] = Plane::from_normal_and_point(
            Vec3::new(m[0][3] - m[0][2], m[1][3] - m[1][2], m[2][3] - m[2][2]),
            Vec3::ZERO,
        );
        // 近平面
        planes[5] = Plane::from_normal_and_point(
            Vec3::new(m[0][3] + m[0][2], m[1][3] + m[1][2], m[2][3] + m[2][2]),
            Vec3::ZERO,
        );

        // 归一化所有平面
        for plane in &mut planes {
            plane.normalize();
        }

        Self { planes }
    }

    /// 获取所有平面
    pub fn planes(&self) -> &[Plane; 6] {
        &self.planes
    }

    /// 检查是否包含点
    pub fn contains_point(&self, point: Vec3) -> bool {
        for plane in &self.planes {
            if plane.distance(point) < 0.0 {
                return false;
            }
        }
        true
    }

    /// 检查是否包含 AABB
    pub fn contains_aabb(&self, aabb: &AABB) -> bool {
        let center = aabb.center();
        let half = aabb.half_extents();

        for plane in &self.planes {
            let n = plane.normal();
            let d = plane.d();

            // 计算 AABB 中心到平面的距离加上包围盒朝向到平面的最大距离
            let dist = n.dot(center) + d;
            let box_dist = (n.x.abs() * half.x + n.y.abs() * half.y + n.z.abs() * half.z);

            if dist + box_dist < 0.0 {
                return false;
            }
        }
        true
    }

    /// 检查是否包含包围球
    pub fn contains_sphere(&self, sphere: &Sphere) -> bool {
        for plane in &self.planes {
            if plane.distance(sphere.center()) < -sphere.radius() {
                return false;
            }
        }
        true
    }

    /// 检查是否与 AABB 相交（粗测试）
    pub fn intersects_aabb(&self, aabb: &AABB) -> bool {
        !(!self.contains_aabb(aabb) && !self.intersects_aabb_inner(aabb))
    }

    /// AABB 相交内部检测
    fn intersects_aabb_inner(&self, aabb: &AABB) -> bool {
        let aabb_min = aabb.min();
        let aabb_max = aabb.max();
        let corners = [
            Vec3::new(aabb_min.x, aabb_min.y, aabb_min.z),
            Vec3::new(aabb_max.x, aabb_min.y, aabb_min.z),
            Vec3::new(aabb_min.x, aabb_max.y, aabb_min.z),
            Vec3::new(aabb_max.x, aabb_max.y, aabb_min.z),
            Vec3::new(aabb_min.x, aabb_min.y, aabb_max.z),
            Vec3::new(aabb_max.x, aabb_min.y, aabb_max.z),
            Vec3::new(aabb_min.x, aabb_max.y, aabb_max.z),
            Vec3::new(aabb_max.x, aabb_max.y, aabb_max.z),
        ];

        for plane in &self.planes {
            let mut all_outside = true;
            for corner in &corners {
                if plane.distance(*corner) >= 0.0 {
                    all_outside = false;
                    break;
                }
            }
            if all_outside {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_perspective() {
        let camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 1000.0);
        assert_eq!(camera.fov(), 60.0);
        assert_eq!(camera.aspect(), 16.0 / 9.0);
    }

    #[test]
    fn test_camera_orthographic() {
        let camera = Camera3D::orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 1000.0);
        assert_eq!(camera.near(), 0.1);
        assert_eq!(camera.far(), 1000.0);
    }

    #[test]
    fn test_frustum_creation() {
        let camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 1000.0);
        let vp = camera.view_projection();
        let frustum = Frustum::from_view_projection(&vp);
        assert_eq!(frustum.planes().len(), 6);
    }

    #[test]
    fn test_camera_view_matrix() {
        let mut camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 1000.0);
        camera.set_position(Vec3::new(0.0, 0.0, 5.0));
        let view = camera.view_matrix();
        // 验证矩阵的基本性质
        assert!(view.inverse().is_some());
    }
}
