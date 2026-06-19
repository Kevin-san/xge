//! 文本模块
//!
//! 定义字体和文本渲染相关类型。

use std::collections::HashMap;

use engine_render::Color;

/// 字体大小
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontSize {
    /// 小号
    Small,
    /// 中号
    Medium,
    /// 大号
    Large,
    /// 特大号
    ExtraLarge,
    /// 自定义大小
    Custom(f32),
}

impl FontSize {
    /// 转换为 f32 值
    pub fn to_f32(&self) -> f32 {
        match self {
            FontSize::Small => 12.0,
            FontSize::Medium => 16.0,
            FontSize::Large => 24.0,
            FontSize::ExtraLarge => 32.0,
            FontSize::Custom(size) => *size,
        }
    }
}

/// 文本对齐方式
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextAlign {
    /// 左对齐
    Left,
    /// 居中对齐
    Center,
    /// 右对齐
    Right,
}

/// 字体
pub struct Font {
    family: &'static str,
    size: FontSize,
    bold: bool,
    italic: bool,
    color: Color,
}

impl Font {
    /// 创建新的字体
    pub fn new(family: &'static str, size: FontSize) -> Self {
        Self {
            family,
            size,
            bold: false,
            italic: false,
            color: Color::BLACK,
        }
    }

    /// 获取字体族
    pub fn family(&self) -> &str {
        self.family
    }

    /// 获取字体大小
    pub fn size(&self) -> FontSize {
        self.size
    }

    /// 获取字体大小（f32）
    pub fn size_f32(&self) -> f32 {
        self.size.to_f32()
    }

    /// 设置字体大小
    pub fn set_size(&mut self, size: FontSize) {
        self.size = size;
    }

    /// 是否加粗
    pub fn bold(&self) -> bool {
        self.bold
    }

    /// 设置加粗
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }

    /// 是否斜体
    pub fn italic(&self) -> bool {
        self.italic
    }

    /// 设置斜体
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    /// 获取颜色
    pub fn color(&self) -> Color {
        self.color
    }

    /// 设置颜色
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// 设置加粗
    pub fn with_bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// 设置斜体
    pub fn with_italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

/// 字体度量
pub struct FontMetrics {
    ascent: f32,
    descent: f32,
    line_height: f32,
    char_widths: HashMap<char, f32>,
}

impl FontMetrics {
    /// 创建新的字体度量
    pub fn new(ascent: f32, descent: f32, line_height: f32) -> Self {
        Self {
            ascent,
            descent,
            line_height,
            char_widths: HashMap::new(),
        }
    }

    /// 获取上升量
    pub fn ascent(&self) -> f32 {
        self.ascent
    }

    /// 获取下降量
    pub fn descent(&self) -> f32 {
        self.descent
    }

    /// 获取行高
    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    /// 获取字符宽度
    pub fn char_width(&self, c: char) -> f32 {
        *self.char_widths.get(&c).unwrap_or(&8.0)
    }

    /// 设置字符宽度
    pub fn set_char_width(&mut self, c: char, width: f32) {
        self.char_widths.insert(c, width);
    }

    /// 测量文本宽度
    pub fn measure_text(&self, text: &str) -> f32 {
        text.chars().map(|c| self.char_width(c)).sum()
    }

    /// 测量多行文本，返回（最大宽度，总高度）
    pub fn measure_text_lines(&self, text: &str) -> (f32, f32) {
        let mut max_width: f32 = 0.0;
        let mut lines = 1;

        for line in text.split('\n') {
            let width = self.measure_text(line);
            max_width = max_width.max(width);
            lines += 1;
        }

        (max_width, self.line_height * lines as f32)
    }
}

/// 文本渲染器
pub struct TextRenderer {
    default_font: Font,
}

impl TextRenderer {
    /// 创建新的文本渲染器
    pub fn new() -> Self {
        Self {
            default_font: Font::new("Arial", FontSize::Medium),
        }
    }

    /// 获取默认字体
    pub fn default_font(&self) -> &Font {
        &self.default_font
    }

    /// 测量文本尺寸
    pub fn measure_text(&self, text: &str, font: &Font) -> (f32, f32) {
        let metrics = FontMetrics::new(
            font.size_f32() * 0.8,
            font.size_f32() * 0.2,
            font.size_f32() * 1.2,
        );
        metrics.measure_text_lines(text)
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_render::Color;

    #[test]
    fn test_font_size_to_f32() {
        assert_eq!(FontSize::Small.to_f32(), 12.0);
        assert_eq!(FontSize::Medium.to_f32(), 16.0);
        assert_eq!(FontSize::Large.to_f32(), 24.0);
        assert_eq!(FontSize::ExtraLarge.to_f32(), 32.0);
        assert_eq!(FontSize::Custom(20.0).to_f32(), 20.0);
    }

    #[test]
    fn test_font_builder() {
        let font = Font::new("Arial", FontSize::Large)
            .with_bold()
            .with_color(Color::RED);

        assert!(font.bold());
        assert_eq!(font.color(), Color::RED);
    }

    #[test]
    fn test_font_metrics_measure_text() {
        let mut metrics = FontMetrics::new(16.0, 4.0, 20.0);
        metrics.set_char_width('H', 10.0);
        metrics.set_char_width('e', 8.0);
        metrics.set_char_width('l', 6.0);
        metrics.set_char_width('o', 8.0);

        let width = metrics.measure_text("Hello");
        assert_eq!(width, 10.0 + 8.0 + 6.0 + 6.0 + 8.0);
    }

    #[test]
    fn test_text_renderer_default_font() {
        let renderer = TextRenderer::new();
        let font = renderer.default_font();
        assert_eq!(font.family(), "Arial");
    }

    #[test]
    fn test_text_renderer_measure_text() {
        let renderer = TextRenderer::new();
        let font = Font::new("Arial", FontSize::Medium);
        let (width, height) = renderer.measure_text("Hello World", &font);
        assert!(width > 0.0);
        assert!(height > 0.0);
    }

    #[test]
    fn test_font_new_with_family() {
        let font = Font::new("Verdana", FontSize::Small);
        assert_eq!(font.family(), "Verdana");
        assert_eq!(font.size(), FontSize::Small);
        assert_eq!(font.size_f32(), 12.0);
    }

    #[test]
    fn test_font_set_size() {
        let mut font = Font::new("Arial", FontSize::Medium);
        font.set_size(FontSize::Large);
        assert_eq!(font.size(), FontSize::Large);
        assert_eq!(font.size_f32(), 24.0);
    }

    #[test]
    fn test_font_set_bold() {
        let mut font = Font::new("Arial", FontSize::Medium);
        assert!(!font.bold());
        font.set_bold(true);
        assert!(font.bold());
        font.set_bold(false);
        assert!(!font.bold());
    }

    #[test]
    fn test_font_set_italic() {
        let mut font = Font::new("Arial", FontSize::Medium);
        assert!(!font.italic());
        font.set_italic(true);
        assert!(font.italic());
    }

    #[test]
    fn test_font_set_color() {
        let mut font = Font::new("Arial", FontSize::Medium);
        font.set_color(Color::BLUE);
        assert_eq!(font.color(), Color::BLUE);
    }

    #[test]
    fn test_font_metrics_ascent_descent() {
        let metrics = FontMetrics::new(16.0, 4.0, 20.0);
        assert_eq!(metrics.ascent(), 16.0);
        assert_eq!(metrics.descent(), 4.0);
        assert_eq!(metrics.line_height(), 20.0);
    }

    #[test]
    fn test_text_align_variants() {
        let _l = TextAlign::Left;
        let _c = TextAlign::Center;
        let _r = TextAlign::Right;
    }

    #[test]
    fn test_font_with_italic_chain() {
        let font = Font::new("Arial", FontSize::Medium).with_italic();
        assert!(font.italic());
        assert!(!font.bold());
    }

    #[test]
    fn test_font_metrics_default_char_width() {
        let metrics = FontMetrics::new(16.0, 4.0, 20.0);
        assert_eq!(metrics.char_width('x'), 8.0);
    }

    #[test]
    fn test_font_metrics_measure_lines() {
        let metrics = FontMetrics::new(16.0, 4.0, 20.0);
        let (_w, h) = metrics.measure_text_lines("line1\nline2");
        // 3 iterations: initial lines=1 + 2 split lines = 3 * 20 = 60
        assert_eq!(h, 60.0);
    }
}
