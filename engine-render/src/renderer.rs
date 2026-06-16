//! Renderer 模块 - 渲染器 trait 与 RenderContext
//!
//! 提供 Renderer trait 定义和 RenderContext 全局渲染上下文。

use engine_math::{Mat4, Vec2};
use parking_lot::RwLock;
use std::sync::Arc;

use super::{
    BlendMode, Camera2D, Color, DrawParams, OrthographicCamera, RenderStats, TextureHandle,
};
use crate::sprite::Rect;

/// 渲染器 trait
///
/// 定义渲染器的统一接口。
pub trait Renderer {
    /// 初始化渲染器
    fn init(window: &crate::RenderContext) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// 获取默认后端名称
    fn default_backend() -> &'static str
    where
        Self: Sized;

    /// 获取后端信息
    fn backend_info(&self) -> String;

    /// 开始帧
    fn begin_frame(&mut self) -> anyhow::Result<()>;

    /// 结束帧
    fn end_frame(&mut self) -> anyhow::Result<()>;

    /// 显示帧（交换缓冲区）
    fn present(&mut self);

    // region: 状态设置

    /// 设置清除颜色
    fn set_clear_color(&mut self, color: Color);

    /// 设置垂直同步
    fn set_vsync(&mut self, enabled: bool);

    /// 设置分辨率
    fn set_resolution(&mut self, width: u32, height: u32);

    /// 调整大小
    fn resize(&mut self, width: u32, height: u32);

    // endregion

    // region: 变换与裁剪

    /// 推送变换矩阵
    fn push_transform(&mut self, matrix: Mat4);

    /// 弹出变换矩阵
    fn pop_transform(&mut self);

    /// 推送裁剪矩形
    fn push_scissor_rect(&mut self, rect: Rect);

    /// 弹出裁剪矩形
    fn pop_scissor_rect(&mut self);

    // endregion

    // region: 混合模式

    /// 设置混合模式
    fn set_blend_mode(&mut self, mode: BlendMode);

    /// 重置混合模式
    fn reset_blend_mode(&mut self);

    // endregion

    // region: 相机

    /// 获取当前相机
    fn camera(&self) -> Option<&Camera2D>;

    /// 设置相机
    fn set_camera(&mut self, camera: Camera2D);

    // endregion

    // region: 绘制命令

    /// 绘制四边形
    fn draw_quad(&mut self, quad: &super::shader::Mesh2D);

    /// 绘制纹理
    fn draw_texture(&mut self, texture: TextureHandle, x: f32, y: f32, color: Color);

    /// 绘制纹理（扩展参数）
    fn draw_texture_ex(&mut self, texture: TextureHandle, x: f32, y: f32, params: DrawParams);

    /// 绘制纹理（PRO 版本，支持源/目标矩形、原点、旋转）
    fn draw_texture_pro(
        &mut self,
        texture: TextureHandle,
        source: Option<Rect>,
        dest: Rect,
        origin: Vec2,
        rotation: f32,
        color: Color,
    );

    /// 绘制旋转纹理
    fn draw_texture_rotated(
        &mut self,
        texture: TextureHandle,
        x: f32,
        y: f32,
        angle: f32,
        color: Color,
    );

    /// 绘制纹理矩形
    fn draw_texture_rect(&mut self, texture: TextureHandle, source: Rect, dest: Rect, color: Color);

    /// 绘制矩形
    fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color);

    /// 绘制矩形边框
    fn draw_rectangle_lines(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        thickness: f32,
        color: Color,
    );

    /// 绘制旋转矩形
    fn draw_rectangle_rotated(&mut self, x: f32, y: f32, w: f32, h: f32, angle: f32, color: Color);

    /// 绘制圆形
    fn draw_circle(&mut self, x: f32, y: f32, r: f32, color: Color);

    /// 绘制圆形边框
    fn draw_circle_lines(&mut self, x: f32, y: f32, r: f32, thickness: f32, color: Color);

    /// 绘制线段
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color);

    /// 绘制三角形
    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color);

    /// 绘制三角形边框
    fn draw_triangle_lines(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, thickness: f32, color: Color);

    /// 绘制多边形
    fn draw_poly(&mut self, x: f32, y: f32, sides: u32, radius: f32, rotation: f32, color: Color);

    /// 绘制多边形边框
    fn draw_poly_lines(
        &mut self,
        x: f32,
        y: f32,
        sides: u32,
        radius: f32,
        rotation: f32,
        thickness: f32,
        color: Color,
    );

    /// 绘制文本（留位）
    fn draw_text(&mut self, _text: &str, _x: f32, _y: f32, _font_size: u32, _color: Color) {
        // Placeholder for text rendering
    }

    // endregion

    // region: 渲染控制

    /// 刷新所有待处理的绘制命令
    fn flush(&mut self);

    /// 获取渲染统计
    fn stats(&self) -> RenderStats;

    // endregion
}

/// 全局渲染上下文
pub struct RenderContext {
    /// 当前渲染器（Arc 允许跨线程共享）
    renderer: Arc<RwLock<Option<Box<dyn Renderer + Send + Sync>>>>,
    /// 全局统计
    stats: RenderStats,
    /// 全局正交相机
    ortho_camera: OrthographicCamera,
    /// 变换矩阵栈
    transform_stack: Vec<Mat4>,
    /// 当前变换
    current_transform: Mat4,
    /// 裁剪矩形栈
    scissor_stack: Vec<Rect>,
    /// 当前混合模式
    current_blend_mode: BlendMode,
    /// 清除颜色
    clear_color: Color,
}

impl RenderContext {
    /// 创建新的渲染上下文
    pub fn new() -> Self {
        Self {
            renderer: Arc::new(RwLock::new(None)),
            stats: RenderStats::new(),
            ortho_camera: OrthographicCamera::from_window(1280, 720, 1.0),
            transform_stack: Vec::new(),
            current_transform: Mat4::IDENTITY,
            scissor_stack: Vec::new(),
            current_blend_mode: BlendMode::Alpha,
            clear_color: Color::BLACK,
        }
    }

    /// 设置渲染器
    pub fn set_renderer<R: Renderer + Send + Sync + 'static>(&self, renderer: R) {
        *self.renderer.write() = Some(Box::new(renderer));
    }

    /// 获取渲染器（如果已设置）
    pub fn renderer(&self) -> Arc<RwLock<Option<Box<dyn Renderer + Send + Sync>>>> {
        self.renderer.clone()
    }

    /// 获取统计
    pub fn stats(&self) -> RenderStats {
        self.stats.clone()
    }

    /// 重置统计
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }

    /// 获取正交相机
    pub fn ortho_camera(&self) -> &OrthographicCamera {
        &self.ortho_camera
    }

    /// 设置正交相机
    pub fn set_ortho_camera(&mut self, camera: OrthographicCamera) {
        self.ortho_camera = camera;
    }

    /// 推送变换
    pub fn push_transform(&mut self, matrix: Mat4) {
        self.transform_stack.push(self.current_transform);
        self.current_transform = self.current_transform * matrix;
    }

    /// 弹出变换
    pub fn pop_transform(&mut self) {
        if let Some(prev) = self.transform_stack.pop() {
            self.current_transform = prev;
        }
    }

    /// 获取当前变换
    pub fn current_transform(&self) -> Mat4 {
        self.current_transform
    }

    /// 推送裁剪矩形
    pub fn push_scissor(&mut self, rect: Rect) {
        self.scissor_stack.push(rect);
    }

    /// 弹出裁剪矩形
    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
    }

    /// 获取清除颜色
    pub fn clear_color(&self) -> Color {
        self.clear_color
    }

    /// 设置清除颜色
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    /// 获取混合模式
    pub fn blend_mode(&self) -> BlendMode {
        self.current_blend_mode
    }

    /// 设置混合模式
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.current_blend_mode = mode;
    }

    /// 检查是否可以批处理
    pub fn can_batch(&self, texture: TextureHandle, blend_mode: BlendMode) -> bool {
        // In a full implementation, would check if same texture and blend mode
        let _ = (texture, blend_mode);
        true
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_context_new() {
        let ctx = RenderContext::new();
        assert_eq!(ctx.stats().draw_calls, 0);
        assert_eq!(ctx.blend_mode(), BlendMode::Alpha);
    }

    #[test]
    fn test_render_context_transform_stack() {
        let mut ctx = RenderContext::new();
        let mat1 = Mat4::from_translation(engine_math::Vec3::new(10.0, 0.0, 0.0));
        let mat2 = Mat4::from_translation(engine_math::Vec3::new(0.0, 20.0, 0.0));

        ctx.push_transform(mat1);
        assert_eq!(ctx.transform_stack.len(), 1);

        ctx.push_transform(mat2);
        assert_eq!(ctx.transform_stack.len(), 2);

        ctx.pop_transform();
        assert_eq!(ctx.transform_stack.len(), 1);

        ctx.pop_transform();
        assert_eq!(ctx.transform_stack.len(), 0);
    }

    #[test]
    fn test_render_context_scissor_stack() {
        let mut ctx = RenderContext::new();
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);

        ctx.push_scissor(rect1);
        assert_eq!(ctx.scissor_stack.len(), 1);

        ctx.push_scissor(rect2);
        assert_eq!(ctx.scissor_stack.len(), 2);

        ctx.pop_scissor();
        assert_eq!(ctx.scissor_stack.len(), 1);

        ctx.pop_scissor();
        assert_eq!(ctx.scissor_stack.len(), 0);
    }

    #[test]
    fn test_render_context_clear_color() {
        let mut ctx = RenderContext::new();
        assert_eq!(ctx.clear_color(), Color::BLACK);

        ctx.set_clear_color(Color::RED);
        assert_eq!(ctx.clear_color(), Color::RED);
    }

    #[test]
    fn test_render_context_blend_mode() {
        let mut ctx = RenderContext::new();
        assert_eq!(ctx.blend_mode(), BlendMode::Alpha);

        ctx.set_blend_mode(BlendMode::Additive);
        assert_eq!(ctx.blend_mode(), BlendMode::Additive);
    }

    #[test]
    fn test_render_context_stats_reset() {
        let mut ctx = RenderContext::new();
        ctx.stats.add_draw_call(5);
        ctx.reset_stats();
        assert_eq!(ctx.stats().draw_calls, 0);
    }
}
