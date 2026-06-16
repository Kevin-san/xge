//! 样式模块
//!
//! 定义 UI 控件的样式属性。

use engine_render::Color;

pub struct Style {
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub corner_radius: f32,
    pub shadow_color: Color,
    pub shadow_offset: (f32, f32),
    pub shadow_blur: f32,
    pub opacity: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            background_color: Color::WHITE,
            border_color: Color::BLACK,
            border_width: 0.0,
            corner_radius: 0.0,
            shadow_color: Color::TRANSPARENT,
            shadow_offset: (0.0, 0.0),
            shadow_blur: 0.0,
            opacity: 1.0,
        }
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn with_shadow(mut self, color: Color, offset: (f32, f32), blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_blur = blur;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TextStyle {
    pub color: Color,
    pub font_size: f32,
    pub font_family: String,
    pub bold: bool,
    pub italic: bool,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            color: Color::BLACK,
            font_size: 16.0,
            font_family: "Arial".to_string(),
            bold: false,
            italic: false,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_font_family(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ButtonStyle {
    pub normal: Style,
    pub hover: Style,
    pub pressed: Style,
    pub disabled: Style,
    pub text_style: TextStyle,
}

impl ButtonStyle {
    pub fn new() -> Self {
        Self {
            normal: Style::new().with_background(Color::new(0.2, 0.5, 0.8, 1.0)),
            hover: Style::new().with_background(Color::new(0.3, 0.6, 0.9, 1.0)),
            pressed: Style::new().with_background(Color::new(0.1, 0.4, 0.7, 1.0)),
            disabled: Style::new().with_background(Color::new(0.5, 0.5, 0.5, 0.5)),
            text_style: TextStyle::new().with_color(Color::WHITE),
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_creation() {
        let style = Style::new();
        assert_eq!(style.background_color, Color::WHITE);
        assert_eq!(style.border_width, 0.0);
        assert_eq!(style.opacity, 1.0);
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .with_background(Color::RED)
            .with_border(Color::BLACK, 2.0)
            .with_corner_radius(5.0);

        assert_eq!(style.background_color, Color::RED);
        assert_eq!(style.border_width, 2.0);
        assert_eq!(style.corner_radius, 5.0);
    }

    #[test]
    fn test_text_style_default() {
        let text_style = TextStyle::default();
        assert_eq!(text_style.font_size, 16.0);
        assert_eq!(text_style.font_family, "Arial");
        assert!(!text_style.bold);
    }

    #[test]
    fn test_button_style_default() {
        let button_style = ButtonStyle::default();
        assert_eq!(button_style.text_style.color, Color::WHITE);
    }
}
