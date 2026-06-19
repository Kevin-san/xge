//! 控件模块
//!
//! 定义各种 UI 控件类型。

use engine_ecs::Component;
use engine_render::Color;
use unicode_segmentation::UnicodeSegmentation;

use crate::style::{ButtonStyle, TextStyle};

/// 按钮控件
pub struct Button {
    text: String,
    style: ButtonStyle,
    is_pressed: bool,
    is_hovered: bool,
}

impl Button {
    /// 创建新的按钮
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: ButtonStyle::new(),
            is_pressed: false,
            is_hovered: false,
        }
    }

    /// 获取按钮文本
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 设置按钮文本
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// 获取按钮样式
    pub fn style(&self) -> &ButtonStyle {
        &self.style
    }

    /// 获取可变按钮样式
    pub fn style_mut(&mut self) -> &mut ButtonStyle {
        &mut self.style
    }

    /// 是否按下
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    /// 设置按下状态
    pub fn set_pressed(&mut self, pressed: bool) {
        self.is_pressed = pressed;
    }

    /// 是否悬停
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    /// 设置悬停状态
    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    /// 根据状态获取当前样式
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

/// 标签控件
pub struct Label {
    text: String,
    text_style: TextStyle,
}

impl Label {
    /// 创建新的标签
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            text_style: TextStyle::new(),
        }
    }

    /// 获取标签文本
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 设置标签文本
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// 获取文本样式
    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    /// 获取可变文本样式
    pub fn text_style_mut(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    /// 获取字符数量
    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    /// 获取字素数量
    pub fn grapheme_count(&self) -> usize {
        self.text.graphemes(true).count()
    }
}

impl Component for Label {}

/// 文本框控件
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
    /// 创建新的文本框
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

    /// 获取文本内容
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 设置文本内容
    pub fn set_text(&mut self, text: &str) {
        if let Some(max_len) = self.max_length {
            self.text = text.chars().take(max_len).collect();
        } else {
            self.text = text.to_string();
        }
        self.cursor_position = self.text.len();
    }

    /// 获取占位符文本
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// 设置占位符文本
    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.placeholder = placeholder.to_string();
    }

    /// 获取文本样式
    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    /// 获取可变文本样式
    pub fn text_style_mut(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    /// 获取光标位置
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// 设置光标位置
    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_position = position.clamp(0, self.text.len());
    }

    /// 是否获得焦点
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// 设置焦点状态
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// 获取最大长度限制
    pub fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    /// 设置最大长度限制
    pub fn set_max_length(&mut self, max_length: Option<usize>) {
        self.max_length = max_length;
        if let Some(max_len) = max_length {
            self.text = self.text.chars().take(max_len).collect();
            self.cursor_position = self.cursor_position.min(self.text.len());
        }
    }

    /// 是否为密码模式
    pub fn is_password(&self) -> bool {
        self.is_password
    }

    /// 设置密码模式
    pub fn set_password(&mut self, is_password: bool) {
        self.is_password = is_password;
    }

    /// 获取显示文本
    pub fn display_text(&self) -> String {
        if self.is_password {
            "*".repeat(self.text.len())
        } else {
            self.text.clone()
        }
    }

    /// 插入字符
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

    /// 删除光标前的字符
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let mut chars: Vec<char> = self.text.chars().collect();
            chars.remove(self.cursor_position - 1);
            self.text = chars.into_iter().collect();
            self.cursor_position -= 1;
        }
    }

    /// 删除光标后的字符
    pub fn delete_char_forward(&mut self) {
        if self.cursor_position < self.text.len() {
            let mut chars: Vec<char> = self.text.chars().collect();
            chars.remove(self.cursor_position);
            self.text = chars.into_iter().collect();
        }
    }

    /// 光标左移
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// 光标右移
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }

    /// 光标移到开头
    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    /// 光标移到末尾
    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.text.len();
    }
}

impl Component for TextBox {}

/// 面板控件
pub struct Panel {
    background_color: Color,
    border_color: Color,
    border_width: f32,
    corner_radius: f32,
}

impl Panel {
    /// 创建新的面板
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取背景颜色
    pub fn background_color(&self) -> Color {
        self.background_color
    }

    /// 设置背景颜色
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// 获取边框颜色
    pub fn border_color(&self) -> Color {
        self.border_color
    }

    /// 设置边框颜色
    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    /// 获取边框宽度
    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    /// 设置边框宽度
    pub fn set_border_width(&mut self, width: f32) {
        self.border_width = width;
    }

    /// 获取圆角半径
    pub fn corner_radius(&self) -> f32 {
        self.corner_radius
    }

    /// 设置圆角半径
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

/// 复选框控件
pub struct CheckBox {
    is_checked: bool,
    text: String,
}

impl CheckBox {
    /// 创建新的复选框
    pub fn new(text: &str) -> Self {
        Self {
            is_checked: false,
            text: text.to_string(),
        }
    }

    /// 是否选中
    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    /// 设置选中状态
    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    /// 切换选中状态
    pub fn toggle(&mut self) {
        self.is_checked = !self.is_checked;
    }

    /// 获取复选框文本
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 设置复选框文本
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Component for CheckBox {}

use crate::style::Style;

#[cfg(test)]
mod tests {
    use super::*;
    use engine_render::Color;

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

    #[test]
    fn test_button_set_text() {
        let mut button = Button::new("Old");
        button.set_text("New");
        assert_eq!(button.text(), "New");
    }

    #[test]
    fn test_button_set_pressed_state() {
        let mut button = Button::new("Test");
        button.set_pressed(true);
        assert!(button.is_pressed());
        button.set_pressed(false);
        assert!(!button.is_pressed());
    }

    #[test]
    fn test_button_set_hovered_state() {
        let mut button = Button::new("Test");
        button.set_hovered(true);
        assert!(button.is_hovered());
        button.set_hovered(false);
        assert!(!button.is_hovered());
    }

    #[test]
    fn test_label_creation() {
        let label = Label::new("Hello");
        assert_eq!(label.text(), "Hello");
        assert_eq!(label.char_count(), 5);
    }

    #[test]
    fn test_label_set_text() {
        let mut label = Label::new("Hello");
        label.set_text("World");
        assert_eq!(label.text(), "World");
    }

    #[test]
    fn test_panel_default_background() {
        let panel = Panel::new();
        assert_eq!(panel.background_color(), Color::WHITE);
    }

    #[test]
    fn test_panel_set_border() {
        let mut panel = Panel::new();
        panel.set_border_color(Color::BLACK);
        panel.set_border_width(2.0);
        assert_eq!(panel.border_color(), Color::BLACK);
        assert_eq!(panel.border_width(), 2.0);
    }

    #[test]
    fn test_panel_set_corner_radius() {
        let mut panel = Panel::new();
        panel.set_corner_radius(8.0);
        assert_eq!(panel.corner_radius(), 8.0);
    }

    #[test]
    fn test_checkbox_text() {
        let mut checkbox = CheckBox::new("Opt1");
        assert_eq!(checkbox.text(), "Opt1");
        checkbox.set_text("Opt2");
        assert_eq!(checkbox.text(), "Opt2");
    }

    #[test]
    fn test_checkbox_set_checked() {
        let mut checkbox = CheckBox::new("X");
        checkbox.set_checked(true);
        assert!(checkbox.is_checked());
        checkbox.set_checked(false);
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_text_box_cursor_move() {
        let mut textbox = TextBox::new("abc");
        textbox.set_cursor_position(0);
        textbox.move_cursor_right();
        assert_eq!(textbox.cursor_position(), 1);
        textbox.move_cursor_left();
        assert_eq!(textbox.cursor_position(), 0);
    }

    #[test]
    fn test_text_box_cursor_clamp() {
        let mut textbox = TextBox::new("hi");
        textbox.set_cursor_position(999);
        assert_eq!(textbox.cursor_position(), 2);
        textbox.set_cursor_position(0);
        assert_eq!(textbox.cursor_position(), 0);
    }

    #[test]
    fn test_text_box_delete_forward() {
        let mut textbox = TextBox::new("abc");
        textbox.set_cursor_position(0);
        textbox.delete_char_forward();
        assert_eq!(textbox.text(), "bc");
    }

    #[test]
    fn test_text_box_cursor_jump() {
        let mut textbox = TextBox::new("abc");
        textbox.move_cursor_to_start();
        assert_eq!(textbox.cursor_position(), 0);
        textbox.move_cursor_to_end();
        assert_eq!(textbox.cursor_position(), 3);
    }

    #[test]
    fn test_text_box_password_display() {
        let mut textbox = TextBox::new("secret");
        textbox.set_password(true);
        assert!(textbox.is_password());
        assert_eq!(textbox.display_text(), "******");
        textbox.set_password(false);
        assert_eq!(textbox.display_text(), "secret");
    }

    #[test]
    fn test_text_box_placeholder() {
        let mut textbox = TextBox::new("");
        textbox.set_placeholder("Enter text");
        assert_eq!(textbox.placeholder(), "Enter text");
    }

    #[test]
    fn test_text_box_focus_state() {
        let mut textbox = TextBox::new("");
        assert!(!textbox.is_focused());
        textbox.set_focused(true);
        assert!(textbox.is_focused());
    }

    #[test]
    fn test_panel_set_background_color() {
        let mut panel = Panel::new();
        panel.set_background_color(Color::RED);
        assert_eq!(panel.background_color(), Color::RED);
    }
}
