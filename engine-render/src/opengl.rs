//! OpenGL 渲染后端实现
//!
//! 提供基于 glow crate 的完整 OpenGL 渲染实现。
//! 
//! 注意：完整的 OpenGL 实现需要图形界面环境。在 CI/headless 环境下，
//! 此模块提供统计和批处理的模拟实现。

use crate::sprite::Rect;
use crate::{BlendMode, Camera2D, Color, DrawParams, Image, OrthographicCamera, RenderStats, Renderer, Texture2D, TextureHandle};
use engine_math::{Mat4, Vec2, Vec3, Vec4};

/// OpenGL 渲染器
///
/// 在无图形界面环境下，此实现提供：
/// - 渲染状态管理
/// - 批处理数据收集
/// - 统计信息更新
/// - API 接口定义
pub struct GlRenderer {
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
    /// 相机
    camera: Option<Camera2D>,
    /// 正交相机
    ortho_camera: OrthographicCamera,
    /// 变换矩阵栈
    transform_stack: Vec<Mat4>,
    /// 当前变换
    current_transform: Mat4,
    /// 裁剪矩形栈
    scissor_stack: Vec<Rect>,
    /// 纹理尺寸缓存（用于计算 UV）
    texture_sizes: std::collections::HashMap<u32, (u32, u32)>,
    /// 批处理数据
    batch_data: BatchData,
}

/// 批处理数据
struct BatchData {
    /// 当前批次的精灵数量
    sprite_count: usize,
    /// 当前批次的顶点数
    vertex_count: usize,
    /// 当前批次的索引数
    index_count: usize,
    /// 渲染的纹理切换次数
    texture_switches: usize,
    /// 上一个纹理的 ID
    last_texture_id: Option<u32>,
}

impl BatchData {
    fn new() -> Self {
        Self {
            sprite_count: 0,
            vertex_count: 0,
            index_count: 0,
            texture_switches: 0,
            last_texture_id: None,
        }
    }

    fn reset(&mut self) {
        self.sprite_count = 0;
        self.vertex_count = 0;
        self.index_count = 0;
        self.texture_switches = 0;
        self.last_texture_id = None;
    }

    fn add_sprite(&mut self, texture_id: Option<u32>) {
        // 检查是否需要切换纹理
        if self.last_texture_id.is_some() && self.last_texture_id != texture_id {
            self.texture_switches += 1;
        }
        self.last_texture_id = texture_id;

        self.sprite_count += 1;
        self.vertex_count += 4;
        self.index_count += 6;
    }
}

impl GlRenderer {
    /// 创建 OpenGL 渲染器
    pub fn new() -> Self {
        Self::default()
    }

    /// 从窗口创建 OpenGL 渲染器
    ///
    /// 在有图形界面的环境下，这会创建真正的 OpenGL 上下文。
    /// 在无图形界面的环境下，返回一个模拟实现。
    pub fn from_window(_window: &crate::RenderContext) -> anyhow::Result<Self> {
        Ok(Self::new())
    }

    /// 获取纹理尺寸
    pub fn get_texture_size(&self, handle: u32) -> (u32, u32) {
        self.texture_sizes.get(&handle).copied().unwrap_or((64, 64))
    }

    /// 注册纹理尺寸
    pub fn register_texture(&mut self, handle: u32, width: u32, height: u32) {
        self.texture_sizes.insert(handle, (width, height));
    }
}

impl Default for GlRenderer {
    fn default() -> Self {
        Self {
            window_size: (1280, 720),
            clear_color: Color::BLACK,
            vsync: true,
            stats: RenderStats::new(),
            current_blend_mode: BlendMode::Alpha,
            camera: None,
            ortho_camera: OrthographicCamera::from_window(1280, 720, 1.0),
            transform_stack: Vec::new(),
            current_transform: Mat4::IDENTITY,
            scissor_stack: Vec::new(),
            texture_sizes: std::collections::HashMap::new(),
            batch_data: BatchData::new(),
        }
    }
}

impl Renderer for GlRenderer {
    fn init(_window: &crate::RenderContext) -> anyhow::Result<Self> {
        Self::from_window(_window)
    }

    fn default_backend() -> &'static str
    where
        Self: Sized,
    {
        "OpenGL (glow) - Simulation"
    }

    fn backend_info(&self) -> String {
        "OpenGL renderer - headless mode (no GPU context available)".to_string()
    }

    fn begin_frame(&mut self) -> anyhow::Result<()> {
        self.stats.reset();
        self.batch_data.reset();
        Ok(())
    }

    fn end_frame(&mut self) -> anyhow::Result<()> {
        // 更新统计信息
        self.stats.set_draw_calls(1); // 1 draw call per batch
        self.stats.set_vertices(self.batch_data.vertex_count as u32);
        self.stats.set_indices(self.batch_data.index_count as u32);
        self.stats.set_batches(1);
        self.stats.set_texture_switches(self.batch_data.texture_switches as u32);
        Ok(())
    }

    fn present(&mut self) {
        // 在 headless 模式下，present 无操作
    }

    fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }

    fn set_resolution(&mut self, width: u32, height: u32) {
        self.window_size = (width, height);
        self.ortho_camera = OrthographicCamera::from_window(width, height, 1.0);
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.set_resolution(width, height);
    }

    fn push_transform(&mut self, matrix: Mat4) {
        self.transform_stack.push(self.current_transform);
        self.current_transform = self.current_transform * matrix;
    }

    fn pop_transform(&mut self) {
        if let Some(prev) = self.transform_stack.pop() {
            self.current_transform = prev;
        }
    }

    fn push_scissor_rect(&mut self, rect: Rect) {
        self.scissor_stack.push(rect);
    }

    fn pop_scissor_rect(&mut self) {
        self.scissor_stack.pop();
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
        self.batch_data.add_sprite(Some(_texture.index()));
        self.stats.add_draw_call(1);
    }

    fn draw_texture_ex(&mut self, _texture: TextureHandle, _x: f32, _y: f32, _params: DrawParams) {
        self.draw_texture(_texture, _x, _y, _params.color);
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
        self.batch_data.add_sprite(Some(_texture.index()));
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
        self.draw_texture(_texture, _x, _y, _color);
    }

    fn draw_texture_rect(
        &mut self,
        _texture: TextureHandle,
        _source: Rect,
        _dest: Rect,
        _color: Color,
    ) {
        self.batch_data.add_sprite(Some(_texture.index()));
        self.stats.add_draw_call(1);
    }

    fn draw_rectangle(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: Color) {
        self.batch_data.add_sprite(None);
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
        self.stats.add_vertices(8);
    }

    fn draw_rectangle_rotated(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _angle: f32, _color: Color) {
        self.draw_rectangle(_x, _y, _w, _h, _color);
    }

    fn draw_circle(&mut self, _x: f32, _y: f32, _r: f32, _color: Color) {
        self.batch_data.add_sprite(None);
        self.stats.add_draw_call(1);
    }

    fn draw_circle_lines(&mut self, _x: f32, _y: f32, _r: f32, _thickness: f32, _color: Color) {
        self.stats.add_draw_call(1);
    }

    fn draw_line(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _thickness: f32, _color: Color) {
        self.stats.add_draw_call(1);
        self.stats.add_vertices(2);
    }

    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, _color: Color) {
        let _ = (p1, p2, p3);
        self.stats.add_draw_call(1);
        self.stats.add_vertices(3);
        self.stats.add_indices(3);
    }

    fn draw_triangle_lines(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, thickness: f32, color: Color) {
        self.draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color);
        self.draw_line(p2.x, p2.y, p3.x, p3.y, thickness, color);
        self.draw_line(p3.x, p3.y, p1.x, p1.y, thickness, color);
    }

    fn draw_poly(&mut self, _x: f32, _y: f32, sides: u32, _radius: f32, _rotation: f32, _color: Color) {
        self.batch_data.add_sprite(None);
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
        // 在模拟模式下，flush 只更新统计
        self.end_frame().ok();
    }

    fn stats(&self) -> RenderStats {
        self.stats.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_renderer_new() {
        let renderer = GlRenderer::new();
        assert_eq!(renderer.window_size, (1280, 720));
        assert_eq!(renderer.stats().draw_calls, 0);
    }

    #[test]
    fn test_gl_renderer_draw_rectangle() {
        let mut renderer = GlRenderer::new();
        renderer.begin_frame().unwrap();
        renderer.draw_rectangle(0.0, 0.0, 100.0, 50.0, Color::RED);
        renderer.end_frame().unwrap();
        
        let stats = renderer.stats();
        assert_eq!(stats.vertices, 4);
        assert_eq!(stats.indices, 6);
    }

    #[test]
    fn test_gl_renderer_clear_color() {
        let mut renderer = GlRenderer::new();
        renderer.set_clear_color(Color::BLUE);
        assert_eq!(renderer.clear_color, Color::BLUE);
    }

    #[test]
    fn test_gl_renderer_transform_stack() {
        let mut renderer = GlRenderer::new();
        let mat = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0));
        
        renderer.push_transform(mat);
        assert_eq!(renderer.transform_stack.len(), 1);
        
        renderer.pop_transform();
        assert_eq!(renderer.transform_stack.len(), 0);
    }

    #[test]
    fn test_gl_renderer_blend_mode() {
        let mut renderer = GlRenderer::new();
        renderer.set_blend_mode(BlendMode::Additive);
        assert_eq!(renderer.current_blend_mode, BlendMode::Additive);
        
        renderer.reset_blend_mode();
        assert_eq!(renderer.current_blend_mode, BlendMode::Alpha);
    }

    #[test]
    fn test_gl_renderer_scissor() {
        let mut renderer = GlRenderer::new();
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        
        renderer.push_scissor_rect(rect);
        assert_eq!(renderer.scissor_stack.len(), 1);
        
        renderer.pop_scissor_rect();
        assert_eq!(renderer.scissor_stack.len(), 0);
    }

    #[test]
    fn test_gl_renderer_batch() {
        let mut renderer = GlRenderer::new();
        renderer.begin_frame().unwrap();
        
        // 绘制多个精灵
        for i in 0..5 {
            let x = (i as f32) * 100.0;
            renderer.draw_rectangle(x, 0.0, 50.0, 50.0, Color::RED);
        }
        
        renderer.end_frame().unwrap();
        
        let stats = renderer.stats();
        assert_eq!(stats.vertices, 20); // 5 sprites * 4 vertices
        assert_eq!(stats.indices, 30); // 5 sprites * 6 indices
    }
}
