//! 控件模块
//!
//! 定义各种 UI 控件类型。

use engine_ecs::Component;
use engine_render::Color;
use unicode_segmentation::UnicodeSegmentation;

use crate::style::{ButtonStyle, TextStyle};

pub struct Button {
    text: String,
    style: ButtonStyle,
    is_pressed: bool,
    is_hovered: bool,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: ButtonStyle::new(),
            is_pressed: false,
            is_hovered: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn style(&self) -> &ButtonStyle {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut ButtonStyle {
        &mut self.style
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.is_pressed = pressed;
    }

    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    pub fn current_style(&self, enabled: bool) -> &Style {
        if !enabled {
            &self.style.disabled
        } else if self.is_pressed {
            &self.style.pressed
        } else if self.is_hovered {
            &self.style.hover
        } else {
            &self.style.normal
        }
    }
}

impl Component for Button {}

pub struct Label {
    text: String,
    text_style: TextStyle,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            text_style: TextStyle::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    pub fn text_style_mut(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    pub fn grapheme_count(&self) -> usize {
        self.text.graphemes(true).count()
    }
}

impl Component for Label {}

pub struct TextBox {
    text: String,
    placeholder: String,
    text_style: TextStyle,
    cursor_position: usize,
    #[allow(dead_code)]
    selection_start: usize,
    #[allow(dead_code)]
    selection_end: usize,
    is_focused: bool,
    max_length: Option<usize>,
    is_password: bool,
}

impl TextBox {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            placeholder: String::new(),
            text_style: TextStyle::new(),
            cursor_position: text.len(),
            selection_start: 0,
            selection_end: 0,
            is_focused: false,
            max_length: None,
            is_password: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        if let Some(max_len) = self.max_length {
            self.text = text.chars().take(max_len).collect();
        } else {
            self.text = text.to_string();
        }
        self.cursor_position = self.text.len();
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.placeholder = placeholder.to_string();
    }

    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    pub fn text_style_mut(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_position = position.clamp(0, self.text.len());
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    pub fn set_max_length(&mut self, max_length: Option<usize>) {
        self.max_length = max_length;
        if let Some(max_len) = max_length {
            self.text = self.text.chars().take(max_len).collect();
            self.cursor_position = self.cursor_position.min(self.text.len());
        }
    }

    pub fn is_password(&self) -> bool {
        self.is_password
    }

    pub fn set_password(&mut self, is_password: bool) {
        self.is_password = is_password;
    }

    pub fn display_text(&self) -> String {
        if self.is_password {
            "*".repeat(self.text.len())
        } else {
            self.text.clone()
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if !c.is_control() {
            if let Some(max_len) = self.max_length {
                if self.text.len() >= max_len {
                    return;
                }
            }

            let mut chars: Vec<char> = self.text.chars().collect();
            chars.insert(self.cursor_position, c);
            self.text = chars.into_iter().collect();
            self.cursor_position += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let mut chars: Vec<char> = self.text.chars().collect();
            chars.remove(self.cursor_position - 1);
            self.text = chars.into_iter().collect();
            self.cursor_position -= 1;
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_position < self.text.len() {
            let mut chars: Vec<char> = self.text.chars().collect();
            chars.remove(self.cursor_position);
            self.text = chars.into_iter().collect();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.text.len();
    }
}

impl Component for TextBox {}

pub struct Panel {
    background_color: Color,
    border_color: Color,
    border_width: f32,
    corner_radius: f32,
}

impl Panel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn border_color(&self) -> Color {
        self.border_color
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    pub fn set_border_width(&mut self, width: f32) {
        self.border_width = width;
    }

    pub fn corner_radius(&self) -> f32 {
        self.corner_radius
    }

    pub fn set_corner_radius(&mut self, radius: f32) {
        self.corner_radius = radius;
    }
}

impl Component for Panel {}

impl Default for Panel {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            border_color: Color::BLACK,
            border_width: 0.0,
            corner_radius: 0.0,
        }
    }
}

pub struct CheckBox {
    is_checked: bool,
    text: String,
}

impl CheckBox {
    pub fn new(text: &str) -> Self {
        Self {
            is_checked: false,
            text: text.to_string(),
        }
    }

    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    pub fn toggle(&mut self) {
        self.is_checked = !self.is_checked;
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Component for CheckBox {}

use crate::style::Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Click me");
        assert_eq!(button.text(), "Click me");
        assert!(!button.is_pressed());
        assert!(!button.is_hovered());
    }

    #[test]
    fn test_button_state_styles() {
        let mut button = Button::new("Test");
        let style = button.current_style(true);
        assert_eq!(
            style.background_color,
            button.style().normal.background_color
        );

        button.set_hovered(true);
        let style = button.current_style(true);
        assert_eq!(
            style.background_color,
            button.style().hover.background_color
        );

        button.set_pressed(true);
        let style = button.current_style(true);
        assert_eq!(
            style.background_color,
            button.style().pressed.background_color
        );

        let style = button.current_style(false);
        assert_eq!(
            style.background_color,
            button.style().disabled.background_color
        );
    }

    #[test]
    fn test_label_grapheme_count() {
        let label = Label::new("Hello 世界");
        assert_eq!(label.grapheme_count(), 8);
    }

    #[test]
    fn test_text_box_insert() {
        let mut textbox = TextBox::new("");
        textbox.insert_char('H');
        textbox.insert_char('e');
        assert_eq!(textbox.text(), "He");
        assert_eq!(textbox.cursor_position(), 2);
    }

    #[test]
    fn test_text_box_delete() {
        let mut textbox = TextBox::new("Hello");
        textbox.set_cursor_position(5);
        textbox.delete_char();
        assert_eq!(textbox.text(), "Hell");
        assert_eq!(textbox.cursor_position(), 4);
    }

    #[test]
    fn test_text_box_max_length() {
        let mut textbox = TextBox::new("");
        textbox.set_max_length(Some(3));
        textbox.insert_char('A');
        textbox.insert_char('B');
        textbox.insert_char('C');
        textbox.insert_char('D');
        assert_eq!(textbox.text(), "ABC");
    }

    #[test]
    fn test_checkbox_toggle() {
        let mut checkbox = CheckBox::new("Enable");
        assert!(!checkbox.is_checked());
        checkbox.toggle();
        assert!(checkbox.is_checked());
        checkbox.toggle();
        assert!(!checkbox.is_checked());
    }
}
