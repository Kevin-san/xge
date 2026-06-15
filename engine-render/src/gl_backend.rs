//! OpenGL (glow) 后端实现
//!
//! 提供基于 glow crate 的 OpenGL 渲染后端实现。

#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(feature = "gl")]
use glow::HasContext;

use crate::sprite::Rect;
use crate::{BlendMode, Camera2D, Color, DrawParams, RenderStats, Renderer, TextureHandle};
use engine_math::{Mat4, Vec2};

/// OpenGL 渲染器实现
#[cfg(feature = "gl")]
pub struct GlRenderer {
    /// glow 上下文（使用 opaque 类型）
    #[allow(dead_code)]
    gl: (),
    /// 窗口尺寸
    window_size: (u32, u32),
    /// 清除颜色
    clear_color: Color,
    /// 垂直同步
    vsync: bool,
    /// 统计信息
    stats: RenderStats,
    /// 当前混合模式
    current_blend_mode: BlendMode,
    /// 正交相机
    ortho_camera: crate::OrthographicCamera,
    /// 相机
    camera: Option<Camera2D>,
}

#[cfg(feature = "gl")]
impl GlRenderer {
    /// 创建新的 GL 渲染器
    #[allow(dead_code)]
    pub fn new(_gl: ()) -> Self {
        Self {
            gl: _gl,
            window_size: (1280, 720),
            clear_color: Color::BLACK,
            vsync: true,
            stats: RenderStats::new(),
            current_blend_mode: BlendMode::Alpha,
            ortho_camera: crate::OrthographicCamera::from_window(1280, 720, 1.0),
            camera: None,
        }
    }

    /// 设置清除颜色
    #[allow(dead_code)]
    fn apply_clear_color(&self) {
        // Would apply clear color to GL context
        let _ = self.clear_color;
    }

    /// 应用混合模式
    #[allow(dead_code)]
    fn apply_blend_mode(&self, mode: BlendMode) {
        let _ = mode;
        // Would apply blend mode to GL context
    }
}

#[cfg(feature = "gl")]
impl Renderer for GlRenderer {
    fn init(_window: &engine_window::Window) -> anyhow::Result<Self> {
        // This would create actual GL context from window
        // For now, return a placeholder
        Err(anyhow::anyhow!("GL renderer requires OpenGL context"))
    }

    fn default_backend() -> &'static str
    where
        Self: Sized,
    {
        "OpenGL (glow)"
    }

    fn backend_info(&self) -> String {
        "OpenGL renderer using glow".to_string()
    }

    fn begin_frame(&mut self) -> anyhow::Result<()> {
        self.stats.reset();
        Ok(())
    }

    fn end_frame(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn present(&mut self) {
        // Swap buffers would happen through window system
    }

    fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }

    fn set_resolution(&mut self, width: u32, height: u32) {
        self.window_size = (width, height);
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.set_resolution(width, height);
    }

    fn push_transform(&mut self, _matrix: Mat4) {
        // Would push to transform stack
    }

    fn pop_transform(&mut self) {
        // Would pop from transform stack
    }

    fn push_scissor_rect(&mut self, rect: Rect) {
        let _ = rect;
        // Would set scissor
    }

    fn pop_scissor_rect(&mut self) {
        // Would restore previous scissor
    }

    fn set_blend_mode(&mut self, mode: BlendMode) {
        self.current_blend_mode = mode;
    }

    fn reset_blend_mode(&mut self) {
        self.current_blend_mode = BlendMode::Alpha;
    }

    fn camera(&self) -> Option<&Camera2D> {
        self.camera.as_ref()
    }

    fn set_camera(&mut self, camera: Camera2D) {
        self.camera = Some(camera);
    }

    fn draw_quad(&mut self, _quad: &crate::shader::Mesh2D) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(4);
        self.stats.add_indices(6);
    }

    fn draw_texture(&mut self, _texture: TextureHandle, _x: f32, _y: f32, _color: Color) {
        self.draw_texture_ex(_texture, _x, _y, DrawParams::default().with_color(_color));
    }

    fn draw_texture_ex(&mut self, _texture: TextureHandle, _x: f32, _y: f32, _params: DrawParams) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(4);
        self.stats.add_indices(6);
    }

    fn draw_texture_pro(
        &mut self,
        _texture: TextureHandle,
        _source: Option<Rect>,
        _dest: Rect,
        _origin: Vec2,
        _rotation: f32,
        _color: Color,
    ) {
        self.stats.add_draw_call(1);
    }

    fn draw_texture_rotated(
        &mut self,
        _texture: TextureHandle,
        _x: f32,
        _y: f32,
        _angle: f32,
        _color: Color,
    ) {
        self.stats.add_draw_call(1);
    }

    fn draw_texture_rect(
        &mut self,
        _texture: TextureHandle,
        _source: Rect,
        _dest: Rect,
        _color: Color,
    ) {
        self.draw_texture_pro(_texture, Some(_source), _dest, Vec2::ZERO, 0.0, _color);
    }

    fn draw_rectangle(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: Color) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(4);
        self.stats.add_indices(6);
    }

    fn draw_rectangle_lines(
        &mut self,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
        _thickness: f32,
        _color: Color,
    ) {
        self.stats.add_draw_call(1);
    }

    fn draw_rectangle_rotated(
        &mut self,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
        _angle: f32,
        _color: Color,
    ) {
        self.draw_rectangle(_x, _y, _w, _h, _color);
    }

    fn draw_circle(&mut self, _x: f32, _y: f32, _r: f32, _color: Color) {
        self.stats.add_draw_call(1);
    }

    fn draw_circle_lines(&mut self, _x: f32, _y: f32, _r: f32, _thickness: f32, _color: Color) {
        self.stats.add_draw_call(1);
    }

    fn draw_line(
        &mut self,
        _x1: f32,
        _y1: f32,
        _x2: f32,
        _y2: f32,
        _thickness: f32,
        _color: Color,
    ) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(2);
    }

    fn draw_triangle(&mut self, _p1: Vec2, _p2: Vec2, _p3: Vec2, _color: Color) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(3);
        self.stats.add_indices(3);
    }

    fn draw_triangle_lines(
        &mut self,
        _p1: Vec2,
        _p2: Vec2,
        _p3: Vec2,
        _thickness: f32,
        _color: Color,
    ) {
        self.draw_line(_p1.x, _p1.y, _p2.x, _p2.y, _thickness, _color);
        self.draw_line(_p2.x, _p2.y, _p3.x, _p3.y, _thickness, _color);
        self.draw_line(_p3.x, _p3.y, _p1.x, _p1.y, _thickness, _color);
    }

    fn draw_poly(
        &mut self,
        _x: f32,
        _y: f32,
        sides: u32,
        _radius: f32,
        _rotation: f32,
        _color: Color,
    ) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(sides + 2);
        self.stats.add_indices(sides * 3);
    }

    fn draw_poly_lines(
        &mut self,
        _x: f32,
        _y: f32,
        sides: u32,
        _radius: f32,
        _rotation: f32,
        _thickness: f32,
        _color: Color,
    ) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(sides);
    }

    fn flush(&mut self) {
        // Flush pending draw calls
    }

    fn stats(&self) -> RenderStats {
        self.stats.clone()
    }
}

// Suppress warnings for unused code in non-gl builds
#[cfg(not(feature = "gl"))]
pub struct GlRenderer;

#[cfg(not(feature = "gl"))]
impl GlRenderer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}
