//! 文本模块
//!
//! 定义字体和文本渲染相关类型。

use std::collections::HashMap;

use engine_render::Color;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(f32),
}

impl FontSize {
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub struct Font {
    family: &'static str,
    size: FontSize,
    bold: bool,
    italic: bool,
    color: Color,
}

impl Font {
    pub fn new(family: &'static str, size: FontSize) -> Self {
        Self {
            family,
            size,
            bold: false,
            italic: false,
            color: Color::BLACK,
        }
    }

    pub fn family(&self) -> &str {
        self.family
    }

    pub fn size(&self) -> FontSize {
        self.size
    }

    pub fn size_f32(&self) -> f32 {
        self.size.to_f32()
    }

    pub fn set_size(&mut self, size: FontSize) {
        self.size = size;
    }

    pub fn bold(&self) -> bool {
        self.bold
    }

    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn with_bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn with_italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub struct FontMetrics {
    ascent: f32,
    descent: f32,
    line_height: f32,
    char_widths: HashMap<char, f32>,
}

impl FontMetrics {
    pub fn new(ascent: f32, descent: f32, line_height: f32) -> Self {
        Self {
            ascent,
            descent,
            line_height,
            char_widths: HashMap::new(),
        }
    }

    pub fn ascent(&self) -> f32 {
        self.ascent
    }

    pub fn descent(&self) -> f32 {
        self.descent
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    pub fn char_width(&self, c: char) -> f32 {
        *self.char_widths.get(&c).unwrap_or(&8.0)
    }

    pub fn set_char_width(&mut self, c: char, width: f32) {
        self.char_widths.insert(c, width);
    }

    pub fn measure_text(&self, text: &str) -> f32 {
        text.chars().map(|c| self.char_width(c)).sum()
    }

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

pub struct TextRenderer {
    default_font: Font,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            default_font: Font::new("Arial", FontSize::Medium),
        }
    }

    pub fn default_font(&self) -> &Font {
        &self.default_font
    }

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
}
