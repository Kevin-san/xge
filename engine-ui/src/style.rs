//! 样式模块
//!
//! 定义 UI 控件的样式属性。

use engine_render::Color;

/// UI样式
pub struct Style {
    /// 背景颜色
    pub background_color: Color,
    /// 边框颜色
    pub border_color: Color,
    /// 边框宽度
    pub border_width: f32,
    /// 圆角半径
    pub corner_radius: f32,
    /// 阴影颜色
    pub shadow_color: Color,
    /// 阴影偏移
    pub shadow_offset: (f32, f32),
    /// 阴影模糊
    pub shadow_blur: f32,
    /// 不透明度
    pub opacity: f32,
}

impl Style {
    /// 创建新的样式
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

    /// 设置背景颜色
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// 设置边框颜色和宽度
    pub fn with_border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    /// 设置圆角半径
    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// 设置阴影
    pub fn with_shadow(mut self, color: Color, offset: (f32, f32), blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = offset;
        self.shadow_blur = blur;
        self
    }

    /// 设置不透明度
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

/// 文本样式
pub struct TextStyle {
    /// 文本颜色
    pub color: Color,
    /// 字体大小
    pub font_size: f32,
    /// 字体族
    pub font_family: String,
    /// 是否加粗
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
}

impl TextStyle {
    /// 创建新的文本样式
    pub fn new() -> Self {
        Self {
            color: Color::BLACK,
            font_size: 16.0,
            font_family: "Arial".to_string(),
            bold: false,
            italic: false,
        }
    }

    /// 设置文本颜色
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 设置字体大小
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// 设置字体族
    pub fn with_font_family(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }

    /// 设置加粗
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// 设置斜体
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

/// 按钮样式
pub struct ButtonStyle {
    /// 普通状态样式
    pub normal: Style,
    /// 悬停状态样式
    pub hover: Style,
    /// 按下状态样式
    pub pressed: Style,
    /// 禁用状态样式
    pub disabled: Style,
    /// 文本样式
    pub text_style: TextStyle,
}

impl ButtonStyle {
    /// 创建新的按钮样式
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
    use engine_render::Color;

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

    #[test]
    fn test_style_with_shadow() {
        let style = Style::new().with_shadow(Color::GRAY, (2.0, 3.0), 4.0);
        assert_eq!(style.shadow_color, Color::GRAY);
        assert_eq!(style.shadow_offset, (2.0, 3.0));
        assert_eq!(style.shadow_blur, 4.0);
    }

    #[test]
    fn test_style_with_opacity_clamped_zero() {
        let style = Style::new().with_opacity(-5.0);
        assert!(style.opacity >= 0.0);
    }

    #[test]
    fn test_style_with_opacity_clamped_one() {
        let style = Style::new().with_opacity(5.0);
        assert!(style.opacity <= 1.0);
    }

    #[test]
    fn test_style_with_opacity_mid() {
        let style = Style::new().with_opacity(0.5);
        assert_eq!(style.opacity, 0.5);
    }

    #[test]
    fn test_text_style_with_color() {
        let text_style = TextStyle::new().with_color(Color::BLUE);
        assert_eq!(text_style.color, Color::BLUE);
    }

    #[test]
    fn test_text_style_with_font_size() {
        let text_style = TextStyle::new().with_font_size(24.0);
        assert_eq!(text_style.font_size, 24.0);
    }

    #[test]
    fn test_text_style_with_font_family() {
        let text_style = TextStyle::new().with_font_family("Verdana");
        assert_eq!(text_style.font_family, "Verdana");
    }

    #[test]
    fn test_text_style_bold() {
        let text_style = TextStyle::new().bold();
        assert!(text_style.bold);
        assert!(!text_style.italic);
    }

    #[test]
    fn test_text_style_italic() {
        let text_style = TextStyle::new().italic();
        assert!(text_style.italic);
    }

    #[test]
    fn test_button_style_states_exist() {
        let bs = ButtonStyle::new();
        assert_ne!(bs.normal.background_color, Color::TRANSPARENT);
        assert_ne!(bs.hover.background_color, Color::TRANSPARENT);
        assert_ne!(bs.pressed.background_color, Color::TRANSPARENT);
        assert_ne!(bs.disabled.background_color, Color::TRANSPARENT);
    }

    #[test]
    fn test_style_border_color() {
        let style = Style::new().with_border(Color::RED, 3.0);
        assert_eq!(style.border_color, Color::RED);
        assert_eq!(style.border_width, 3.0);
    }

    #[test]
    fn test_style_default_is_new() {
        let s1 = Style::new();
        let s2 = Style::default();
        assert_eq!(s1.background_color, s2.background_color);
        assert_eq!(s1.opacity, s2.opacity);
    }

    #[test]
    fn test_text_style_chain_builder() {
        let ts = TextStyle::new()
            .with_color(Color::GREEN)
            .with_font_size(20.0)
            .with_font_family("Times")
            .bold()
            .italic();
        assert_eq!(ts.color, Color::GREEN);
        assert_eq!(ts.font_size, 20.0);
        assert_eq!(ts.font_family, "Times");
        assert!(ts.bold);
        assert!(ts.italic);
    }
}
