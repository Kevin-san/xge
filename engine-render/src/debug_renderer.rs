//! DebugRenderer 模块 - 调试图形渲染
//!
//! 提供 DebugRenderer 类型，用于绘制调试线条、矩形、圆形等。

use super::{Color, RenderContext};
use engine_math::Vec2;

/// 调试渲染器
///
/// 用于绘制调试图形（线条、矩形、圆形等）。
#[derive(Clone, Debug)]
pub struct DebugRenderer {
    /// 线条列表 (start, end, color)
    lines: Vec<(Vec2, Vec2, Color)>,
    /// 矩形列表 (rect, color)
    rects: Vec<(super::Rect, Color)>,
    /// 圆形列表 (center, radius, color)
    circles: Vec<(Vec2, f32, Color)>,
    /// 文本列表 (text, position, color) - 后续完善
    texts: Vec<(String, Vec2, Color)>,
    /// 是否启用
    enabled: bool,
}

impl Default for DebugRenderer {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            rects: Vec::new(),
            circles: Vec::new(),
            texts: Vec::new(),
            enabled: true, // Default to enabled
        }
    }
}

impl DebugRenderer {
    /// 创建新的调试渲染器
    pub fn new() -> Self {
        Self::default()
    }

    /// 绘制线段
    pub fn line(&mut self, start: Vec2, end: Vec2, color: Color) {
        self.lines.push((start, end, color));
    }

    /// 绘制矩形（填充）
    pub fn rect(&mut self, rect: super::Rect, color: Color) {
        self.rects.push((rect, color));
    }

    /// 绘制矩形边框
    pub fn rect_lines(&mut self, rect: super::Rect, color: Color) {
        let min = Vec2::new(rect.x, rect.y);
        let max = Vec2::new(rect.right(), rect.bottom());

        self.line(Vec2::new(min.x, min.y), Vec2::new(max.x, min.y), color);
        self.line(Vec2::new(max.x, min.y), Vec2::new(max.x, max.y), color);
        self.line(Vec2::new(max.x, max.y), Vec2::new(min.x, max.y), color);
        self.line(Vec2::new(min.x, max.y), Vec2::new(min.x, min.y), color);
    }

    /// 绘制圆形（填充，近似多边形）
    pub fn circle(&mut self, center: Vec2, radius: f32, color: Color) {
        self.circles.push((center, radius, color));
    }

    /// 绘制圆形边框
    pub fn circle_lines(&mut self, center: Vec2, radius: f32, color: Color) {
        // Draw circle using line segments
        let segments = 32;
        let angle_step = std::f32::consts::TAU / segments as f32;

        let mut prev = Vec2::ZERO;
        for i in 0..=segments {
            let angle = i as f32 * angle_step;
            let point = Vec2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );

            if i > 0 {
                self.line(prev, point, color);
            }
            prev = point;
        }
    }

    /// 绘制十字
    pub fn cross(&mut self, position: Vec2, size: f32, color: Color) {
        let half = size / 2.0;
        self.line(
            Vec2::new(position.x - half, position.y),
            Vec2::new(position.x + half, position.y),
            color,
        );
        self.line(
            Vec2::new(position.x, position.y - half),
            Vec2::new(position.x, position.y + half),
            color,
        );
    }

    /// 绘制网格
    pub fn grid(&mut self, origin: Vec2, cell_size: f32, cols: u32, rows: u32, color: Color) {
        for col in 0..=cols {
            let x = origin.x + col as f32 * cell_size;
            self.line(
                Vec2::new(x, origin.y),
                Vec2::new(x, origin.y + rows as f32 * cell_size),
                color,
            );
        }

        for row in 0..=rows {
            let y = origin.y + row as f32 * cell_size;
            self.line(
                Vec2::new(origin.x, y),
                Vec2::new(origin.x + cols as f32 * cell_size, y),
                color,
            );
        }
    }

    /// 绘制文本（后续完善）
    pub fn text(&mut self, _text: &str, _position: Vec2, _color: Color) {
        // Placeholder for text rendering
        // Would need font/texture atlas integration
    }

    /// 刷新渲染（发送到 GPU）
    pub fn flush(&mut self, ctx: &RenderContext) {
        // Render lines
        for (start, end, color) in &self.lines {
            self.draw_line_internal(ctx, *start, *end, *color);
        }

        // Render rects
        for (rect, color) in &self.rects {
            self.draw_rect_internal(ctx, *rect, *color);
        }

        // Render circles
        for (center, radius, color) in &self.circles {
            self.draw_circle_internal(ctx, *center, *radius, *color);
        }

        // Clear after flush
        self.clear();
    }

    /// 内部：绘制线段
    fn draw_line_internal(&self, ctx: &RenderContext, start: Vec2, end: Vec2, color: Color) {
        // Implementation would use GL_LINES
        let _ = ctx;
        let _ = (start, end, color);
    }

    /// 内部：绘制矩形
    fn draw_rect_internal(&self, ctx: &RenderContext, rect: super::Rect, color: Color) {
        let _ = ctx;
        let _ = (rect, color);
    }

    /// 内部：绘制圆形
    fn draw_circle_internal(&self, ctx: &RenderContext, center: Vec2, radius: f32, color: Color) {
        let _ = ctx;
        let _ = (center, radius, color);
    }

    /// 清空所有调试图形
    pub fn clear(&mut self) {
        self.lines.clear();
        self.rects.clear();
        self.circles.clear();
        self.texts.clear();
    }

    /// 启用/禁用调试渲染
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 获取线条数量
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// 获取矩形数量
    pub fn rect_count(&self) -> usize {
        self.rects.len()
    }

    /// 获取圆形数量
    pub fn circle_count(&self) -> usize {
        self.circles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Rect};

    #[test]
    fn test_debug_renderer_new() {
        let renderer = DebugRenderer::new();
        assert!(renderer.is_enabled());
        assert_eq!(renderer.line_count(), 0);
    }

    #[test]
    fn test_debug_renderer_line() {
        let mut renderer = DebugRenderer::new();
        renderer.line(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color::RED);
        assert_eq!(renderer.line_count(), 1);
    }

    #[test]
    fn test_debug_renderer_rect() {
        let mut renderer = DebugRenderer::new();
        let rect = Rect::new(0.0, 0.0, 100.0, 50.0);
        renderer.rect(rect, Color::BLUE);
        assert_eq!(renderer.rect_count(), 1);
    }

    #[test]
    fn test_debug_renderer_circle() {
        let mut renderer = DebugRenderer::new();
        renderer.circle(Vec2::new(100.0, 100.0), 50.0, Color::GREEN);
        assert_eq!(renderer.circle_count(), 1);
    }

    #[test]
    fn test_debug_renderer_clear() {
        let mut renderer = DebugRenderer::new();
        renderer.line(Vec2::ZERO, Vec2::ONE, Color::RED);
        renderer.rect(Rect::new(0.0, 0.0, 10.0, 10.0), Color::BLUE);
        renderer.clear();
        assert_eq!(renderer.line_count(), 0);
        assert_eq!(renderer.rect_count(), 0);
    }

    #[test]
    fn test_debug_renderer_enabled() {
        let mut renderer = DebugRenderer::new();
        assert!(renderer.is_enabled());
        renderer.set_enabled(false);
        assert!(!renderer.is_enabled());
    }

    #[test]
    fn test_debug_renderer_cross() {
        let mut renderer = DebugRenderer::new();
        renderer.cross(Vec2::new(100.0, 100.0), 20.0, Color::RED);
        // Cross draws 2 lines
        assert_eq!(renderer.line_count(), 2);
    }

    #[test]
    fn test_debug_renderer_grid() {
        let mut renderer = DebugRenderer::new();
        renderer.grid(Vec2::ZERO, 50.0, 3, 3, Color::GRAY);
        // 4 vertical lines + 4 horizontal lines = 8 lines
        assert_eq!(renderer.line_count(), 8);
    }

    #[test]
    fn test_debug_renderer_rect_lines() {
        let mut renderer = DebugRenderer::new();
        renderer.rect_lines(Rect::new(0.0, 0.0, 100.0, 100.0), Color::RED);
        // 4 lines for rectangle border
        assert_eq!(renderer.line_count(), 4);
    }
}
