//! Camera 模块 - 正交相机与视图
//!
//! 提供 OrthographicCamera、Camera2D、View、Viewport 等类型。

use engine_math::{Mat4, Vec2, Vec3};

/// 视口
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Viewport {
    /// X 坐标
    x: u32,
    /// Y 坐标
    y: u32,
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
}

impl Viewport {
    /// 创建新视口
    #[inline]
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// X 坐标
    #[inline]
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Y 坐标
    #[inline]
    pub fn y(&self) -> u32 {
        self.y
    }

    /// 宽度
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 高度
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}

/// 视图
#[derive(Clone, Debug)]
pub struct View {
    /// 相机
    camera: Camera2D,
    /// 视口
    viewport: Viewport,
}

impl View {
    /// 创建新视图
    pub fn new(camera: Camera2D, viewport: Viewport) -> Self {
        Self { camera, viewport }
    }

    /// 获取相机
    pub fn camera(&self) -> &Camera2D {
        &self.camera
    }

    /// 获取视口
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }
}

/// 正交相机（简单版本，不支持旋转）
#[derive(Clone, Debug)]
pub struct OrthographicCamera {
    /// 投影矩阵
    projection: Mat4,
    /// 视图矩阵
    view: Mat4,
    /// 左边界
    left: f32,
    /// 右边界
    right: f32,
    /// 下边界
    bottom: f32,
    /// 上边界
    top: f32,
    /// 近平面
    near: f32,
    /// 远平面
    far: f32,
    /// 缩放
    zoom: f32,
    /// 位置
    position: Vec2,
}

impl OrthographicCamera {
    /// 创建新正交相机
    ///
    /// # Arguments
    /// * `left` - 左边界
    /// * `right` - 右边界
    /// * `bottom` - 下边界
    /// * `top` - 上边界
    /// * `near` - 近平面
    /// * `far` - 远平面
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut camera = Self {
            projection: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            left,
            right,
            bottom,
            top,
            near,
            far,
            zoom: 1.0,
            position: Vec2::ZERO,
        };
        camera.recalculate();
        camera
    }

    /// 从窗口尺寸创建相机
    ///
    /// # Arguments
    /// * `width` - 窗口宽度
    /// * `height` - 窗口高度
    /// * `zoom` - 缩放因子
    pub fn from_window(width: u32, height: u32, zoom: f32) -> Self {
        let w = width as f32 / zoom;
        let h = height as f32 / zoom;
        Self::new(-w / 2.0, w / 2.0, -h / 2.0, h / 2.0, -1.0, 1.0)
    }

    /// 从尺寸创建
    pub fn from_size(width: f32, height: f32) -> Self {
        Self::new(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            -1.0,
            1.0,
        )
    }

    /// 重新计算投影和视图矩阵
    fn recalculate(&mut self) {
        self.projection = Mat4::IDENTITY;

        let left = self.left / self.zoom;
        let right = self.right / self.zoom;
        let bottom = self.bottom / self.zoom;
        let top = self.top / self.zoom;

        self.projection = Self::orthographic(left, right, bottom, top, self.near, self.far);

        self.view = Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0));
    }

    /// 创建正交投影矩阵
    fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        let mut m = Mat4::IDENTITY;

        m.cols[0][0] = 2.0 / (right - left);
        m.cols[1][1] = 2.0 / (top - bottom);
        m.cols[2][2] = -2.0 / (far - near);
        m.cols[3][0] = -(right + left) / (right - left);
        m.cols[3][1] = -(top + bottom) / (top - bottom);
        m.cols[3][2] = -(far + near) / (far - near);

        m
    }

    /// 获取投影矩阵
    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    /// 获取视图矩阵
    pub fn view(&self) -> Mat4 {
        self.view
    }

    /// 获取视图投影矩阵
    pub fn view_projection(&self) -> Mat4 {
        self.projection * self.view
    }

    /// 屏幕坐标转世界坐标
    ///
    /// # Arguments
    /// * `screen_pos` - 屏幕坐标（像素，左上角为原点）
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        // NDC coordinates
        let ndc_x = screen_pos.x / (self.right - self.left) * 2.0 - 1.0;
        let ndc_y = screen_pos.y / (self.top - self.bottom) * 2.0 - 1.0;

        // World coordinates
        let world_x = ndc_x * (self.right - self.left) / 2.0;
        let world_y = ndc_y * (self.top - self.bottom) / 2.0;

        Vec2::new(world_x, world_y) + self.position
    }

    /// 世界坐标转屏幕坐标
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let screen_x = (world_pos.x - self.position.x) / (self.right - self.left) * 2.0 + 1.0;
        let screen_y = (world_pos.y - self.position.y) / (self.top - self.bottom) * 2.0 + 1.0;

        Vec2::new(
            (screen_x + 1.0) / 2.0 * (self.right - self.left),
            (screen_y + 1.0) / 2.0 * (self.top - self.bottom),
        )
    }

    /// 设置缩放
    pub fn zoom(&mut self, factor: f32) {
        self.zoom *= factor;
        self.recalculate();
    }

    /// 获取位置
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// 设置位置
    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
        self.recalculate();
    }

    /// 移动相机
    pub fn move_by(&mut self, delta: Vec2) {
        self.position = self.position + delta;
        self.recalculate();
    }
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        Self::from_window(1280, 720, 1.0)
    }
}

/// 2D 相机（支持位置、旋转、缩放）
#[derive(Clone, Debug)]
pub struct Camera2D {
    /// 位置
    position: Vec2,
    /// 旋转（弧度）
    rotation: f32,
    /// 缩放
    zoom: f32,
    /// 跟随目标
    target: Option<Vec2>,
    /// 跟随平滑度
    smoothing: f32,
    /// 偏移
    offset: Vec2,
    /// 左边界
    left: f32,
    /// 右边界
    right: f32,
    /// 下边界
    bottom: f32,
    /// 上边界
    top: f32,
    /// 是否使用边界
    use_bounds: bool,
}

impl Camera2D {
    /// 创建新 2D 相机
    pub fn new() -> Self {
        Self::default()
    }

    /// 从窗口创建
    pub fn from_window(width: u32, height: u32, zoom: f32) -> Self {
        let w = width as f32 / zoom;
        let h = height as f32 / zoom;
        let mut camera = Self::new();
        camera.left = -w / 2.0;
        camera.right = w / 2.0;
        camera.bottom = -h / 2.0;
        camera.top = h / 2.0;
        camera.zoom = zoom;
        camera
    }

    // region: Getter/Setter

    /// 获取位置
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// 设置位置
    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    /// 获取旋转
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// 设置旋转
    pub fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }

    /// 获取缩放
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// 设置缩放
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.01);
    }

    /// 获取跟随目标
    pub fn target(&self) -> Option<Vec2> {
        self.target
    }

    /// 设置跟随目标
    pub fn set_target(&mut self, target: Option<Vec2>) {
        self.target = target;
    }

    /// 获取偏移
    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    /// 设置偏移
    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }

    // endregion

    // region: 矩阵计算

    /// 获取投影矩阵
    pub fn projection(&self) -> Mat4 {
        let left = self.left / self.zoom;
        let right = self.right / self.zoom;
        let bottom = self.bottom / self.zoom;
        let top = self.top / self.zoom;

        let mut m = Mat4::IDENTITY;
        m.cols[0][0] = 2.0 / (right - left);
        m.cols[1][1] = 2.0 / (top - bottom);
        m.cols[2][2] = -2.0 / (1000.0 + 1.0); // far - near, simplified
        m.cols[3][0] = -(right + left) / (right - left);
        m.cols[3][1] = -(top + bottom) / (top - bottom);
        m
    }

    /// 获取视图矩阵
    pub fn view(&self) -> Mat4 {
        let mut m = Mat4::IDENTITY;

        // Translation
        m.cols[3][0] = -self.position.x - self.offset.x;
        m.cols[3][1] = -self.position.y - self.offset.y;

        // Rotation
        if self.rotation != 0.0 {
            let (s, c) = self.rotation.sin_cos();
            let mut rot = Mat4::IDENTITY;
            rot.cols[0][0] = c;
            rot.cols[0][1] = s;
            rot.cols[1][0] = -s;
            rot.cols[1][1] = c;
            m = m * rot;
        }

        // Scale
        if self.zoom != 1.0 {
            let mut scale = Mat4::IDENTITY;
            scale.cols[0][0] = self.zoom;
            scale.cols[1][1] = self.zoom;
            m = m * scale;
        }

        m
    }

    /// 获取视图投影矩阵
    pub fn view_projection(&self) -> Mat4 {
        self.projection() * self.view()
    }

    // endregion

    // region: 坐标转换

    /// 屏幕坐标转世界坐标
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let view_proj = self.view_projection();
        let inv = view_proj.inverse().unwrap_or(Mat4::IDENTITY);

        let ndc = Vec4::new(
            screen_pos.x / (self.right - self.left) * 2.0 - 1.0,
            screen_pos.y / (self.top - self.bottom) * 2.0 - 1.0,
            0.0,
            1.0,
        );

        let world = inv.mul_vec4(ndc);
        Vec2::new(world.x, world.y)
    }

    /// 世界坐标转屏幕坐标
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let view_proj = self.view_projection();

        let clip = view_proj.mul_vec4(Vec4::new(world_pos.x, world_pos.y, 0.0, 1.0));

        let screen_x = (clip.x / clip.w + 1.0) / 2.0 * (self.right - self.left);
        let screen_y = (clip.y / clip.w + 1.0) / 2.0 * (self.top - self.bottom);

        Vec2::new(screen_x, screen_y)
    }

    // endregion

    // region: 更新

    /// 更新相机（平滑跟随）
    ///
    /// # Arguments
    /// * `dt` - 帧时间（秒）
    pub fn update(&mut self, dt: f32) {
        if let Some(target) = self.target {
            let diff = target - self.position;

            if self.smoothing > 0.0 {
                let t = 1.0 - (-dt / self.smoothing).exp();
                self.position = self.position + diff * t;
            } else {
                self.position = target;
            }
        }
    }

    /// 设置边界
    pub fn set_bounds(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.use_bounds = true;
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
    }

    /// 清除边界
    pub fn clear_bounds(&mut self) {
        self.use_bounds = false;
    }

    // endregion
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
            target: None,
            smoothing: 0.1,
            offset: Vec2::ZERO,
            left: -640.0,
            right: 640.0,
            bottom: -360.0,
            top: 360.0,
            use_bounds: false,
        }
    }
}

// Vec4 import
use engine_math::Vec4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orthographic_camera_new() {
        let camera = OrthographicCamera::new(-100.0, 100.0, -50.0, 50.0, -1.0, 1.0);
        assert!((camera.projection().cols[0][0] - 0.01).abs() < 0.001);
    }

    #[test]
    fn test_orthographic_camera_from_window() {
        let camera = OrthographicCamera::from_window(1280, 720, 1.0);
        assert_eq!(camera.position, Vec2::ZERO);
    }

    #[test]
    fn test_orthographic_camera_screen_to_world() {
        let mut camera = OrthographicCamera::from_window(1280, 720, 1.0);
        camera.set_position(Vec2::new(100.0, 50.0));

        let world = camera.screen_to_world(Vec2::new(640.0, 360.0));
        // Screen center (640, 360) maps to world center (0, 0) + camera position
        // So the result should be (100, 50)
        assert!((world.x - 100.0).abs() < 1.0);
        assert!((world.y - 50.0).abs() < 1.0);
    }

    #[test]
    fn test_orthographic_camera_zoom() {
        let mut camera = OrthographicCamera::from_window(1280, 720, 1.0);
        camera.zoom(2.0);
        // Zoom doubles, so view should change
    }

    #[test]
    fn test_orthographic_camera_move_by() {
        let mut camera = OrthographicCamera::from_window(1280, 720, 1.0);
        camera.move_by(Vec2::new(100.0, 50.0));
        assert_eq!(camera.position, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_camera2d_new() {
        let camera = Camera2D::new();
        assert_eq!(camera.position, Vec2::ZERO);
        assert_eq!(camera.rotation, 0.0);
        assert_eq!(camera.zoom, 1.0);
    }

    #[test]
    fn test_camera2d_set_position() {
        let mut camera = Camera2D::new();
        camera.set_position(Vec2::new(100.0, 200.0));
        assert_eq!(camera.position, Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_camera2d_set_target() {
        let mut camera = Camera2D::new();
        camera.set_target(Some(Vec2::new(100.0, 100.0)));
        assert_eq!(camera.target(), Some(Vec2::new(100.0, 100.0)));
    }

    #[test]
    fn test_camera2d_update_with_target() {
        let mut camera = Camera2D::new();
        camera.set_target(Some(Vec2::new(100.0, 100.0)));
        camera.set_position(Vec2::ZERO);
        camera.update(0.016); // ~60fps
                              // Position should have moved towards target
        assert!(camera.position.x > 0.0);
    }

    #[test]
    fn test_camera2d_bounds() {
        let mut camera = Camera2D::new();
        camera.set_bounds(-1000.0, 1000.0, -500.0, 500.0);
        assert_eq!(camera.left, -1000.0);
        assert_eq!(camera.right, 1000.0);
    }

    #[test]
    fn test_viewport() {
        let vp = Viewport::new(0, 0, 1280, 720);
        assert_eq!(vp.width(), 1280);
        assert_eq!(vp.height(), 720);
    }

    #[test]
    fn test_view() {
        let camera = Camera2D::new();
        let viewport = Viewport::new(0, 0, 1280, 720);
        let view = View::new(camera, viewport);
        assert_eq!(view.viewport(), viewport);
    }
}
