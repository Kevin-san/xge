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

// ===== Slider 滑块控件 =====

/// 滑块方向
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SliderDirection {
    /// 水平
    Horizontal,
    /// 垂直
    Vertical,
}

impl Default for SliderDirection {
    fn default() -> Self {
        SliderDirection::Horizontal
    }
}

/// 滑块控件
///
/// 提供数值范围选择，支持水平/垂直方向、最小/最大值、步长、拖拽交互。
pub struct Slider {
    /// 当前值
    value: f32,
    /// 最小值
    min: f32,
    /// 最大值
    max: f32,
    /// 步长（0 表示连续）
    step: f32,
    /// 方向
    direction: SliderDirection,
    /// 是否正在拖拽
    is_dragging: bool,
    /// 是否悬停
    is_hovered: bool,
    /// 是否禁用
    disabled: bool,
    /// 滑块手柄大小（像素）
    handle_size: f32,
    /// 轨道高度（像素）
    track_thickness: f32,
}

impl Slider {
    /// 创建新的滑块
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        let mut slider = Self {
            value: 0.0,
            min,
            max,
            step: 0.0,
            direction: SliderDirection::Horizontal,
            is_dragging: false,
            is_hovered: false,
            disabled: false,
            handle_size: 16.0,
            track_thickness: 4.0,
        };
        slider.set_value(value);
        slider
    }

    /// 创建水平滑块
    pub fn horizontal(min: f32, max: f32, value: f32) -> Self {
        let mut slider = Self::new(min, max, value);
        slider.direction = SliderDirection::Horizontal;
        slider
    }

    /// 创建垂直滑块
    pub fn vertical(min: f32, max: f32, value: f32) -> Self {
        let mut slider = Self::new(min, max, value);
        slider.direction = SliderDirection::Vertical;
        slider
    }

    /// 获取当前值
    pub fn value(&self) -> f32 {
        self.value
    }

    /// 设置当前值（自动 clamp 并应用步长，向下取整到最近步长）
    pub fn set_value(&mut self, value: f32) {
        let mut v = value.clamp(self.min, self.max);
        if self.step > 0.0 {
            v = self.min + ((v - self.min) / self.step).floor() * self.step;
            v = v.clamp(self.min, self.max);
        }
        self.value = v;
    }

    /// 获取最小值
    pub fn min(&self) -> f32 {
        self.min
    }

    /// 设置最小值
    pub fn set_min(&mut self, min: f32) {
        self.min = min;
        if self.max < min {
            self.max = min;
        }
        self.set_value(self.value);
    }

    /// 获取最大值
    pub fn max(&self) -> f32 {
        self.max
    }

    /// 设置最大值
    pub fn set_max(&mut self, max: f32) {
        self.max = max;
        if self.min > max {
            self.min = max;
        }
        self.set_value(self.value);
    }

    /// 获取步长
    pub fn step(&self) -> f32 {
        self.step
    }

    /// 设置步长
    pub fn set_step(&mut self, step: f32) {
        self.step = step.max(0.0);
        self.set_value(self.value);
    }

    /// 获取方向
    pub fn direction(&self) -> SliderDirection {
        self.direction
    }

    /// 设置方向
    pub fn set_direction(&mut self, direction: SliderDirection) {
        self.direction = direction;
    }

    /// 是否正在拖拽
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// 设置拖拽状态
    pub fn set_dragging(&mut self, dragging: bool) {
        self.is_dragging = dragging;
    }

    /// 是否悬停
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    /// 设置悬停状态
    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    /// 是否禁用
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// 设置禁用状态
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// 获取手柄大小
    pub fn handle_size(&self) -> f32 {
        self.handle_size
    }

    /// 设置手柄大小
    pub fn set_handle_size(&mut self, size: f32) {
        self.handle_size = size.max(0.0);
    }

    /// 获取轨道厚度
    pub fn track_thickness(&self) -> f32 {
        self.track_thickness
    }

    /// 设置轨道厚度
    pub fn set_track_thickness(&mut self, thickness: f32) {
        self.track_thickness = thickness.max(0.0);
    }

    /// 获取进度比例（0.0~1.0）
    pub fn ratio(&self) -> f32 {
        if (self.max - self.min).abs() < f32::EPSILON {
            return 0.0;
        }
        (self.value - self.min) / (self.max - self.min)
    }

    /// 根据鼠标位置（相对滑块矩形）设置值
    pub fn set_value_from_position(&mut self, pos: engine_math::Vec2, slider_rect: engine_math::Rect) {
        let ratio = match self.direction {
            SliderDirection::Horizontal => {
                if slider_rect.w > 0.0 {
                    ((pos.x - slider_rect.x) / slider_rect.w).clamp(0.0, 1.0)
                } else {
                    0.0
                }
            }
            SliderDirection::Vertical => {
                if slider_rect.h > 0.0 {
                    // 垂直滑块：上=最大值，下=最小值
                    1.0 - ((pos.y - slider_rect.y) / slider_rect.h).clamp(0.0, 1.0)
                } else {
                    0.0
                }
            }
        };
        let new_value = self.min + ratio * (self.max - self.min);
        self.set_value(new_value);
    }

    /// 增加值
    pub fn increment(&mut self) {
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        self.set_value(self.value + step);
    }

    /// 减少值
    pub fn decrement(&mut self) {
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        self.set_value(self.value - step);
    }
}

impl Component for Slider {}

impl Default for Slider {
    fn default() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
}

// ===== Grid 网格控件 =====

/// 网格布局方向
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GridFlow {
    /// 行优先（先填满一行再换行）
    Row,
    /// 列优先（先填满一列再换列）
    Column,
}

impl Default for GridFlow {
    fn default() -> Self {
        GridFlow::Row
    }
}

/// 网格控件
///
/// 将子项按网格排列，支持固定列数/行数、间距、对齐。
pub struct Grid {
    /// 列数（0 表示自动）
    columns: usize,
    /// 行数（0 表示自动）
    rows: usize,
    /// 单元格间距
    spacing: f32,
    /// 内边距
    padding: engine_math::Vec2,
    /// 布局方向
    flow: GridFlow,
    /// 单元格固定宽度（None 表示自动）
    cell_width: Option<f32>,
    /// 单元格固定高度（None 表示自动）
    cell_height: Option<f32>,
}

impl Grid {
    /// 创建新的网格
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建指定列数的网格
    pub fn with_columns(columns: usize) -> Self {
        Self {
            columns: columns.max(1),
            ..Default::default()
        }
    }

    /// 创建指定行数的网格
    pub fn with_rows(rows: usize) -> Self {
        Self {
            rows: rows.max(1),
            ..Default::default()
        }
    }

    /// 获取列数
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// 设置列数
    pub fn set_columns(&mut self, columns: usize) {
        self.columns = columns;
    }

    /// 获取行数
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// 设置行数
    pub fn set_rows(&mut self, rows: usize) {
        self.rows = rows;
    }

    /// 获取间距
    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    /// 设置间距
    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing.max(0.0);
    }

    /// 获取内边距
    pub fn padding(&self) -> engine_math::Vec2 {
        self.padding
    }

    /// 设置内边距
    pub fn set_padding(&mut self, padding: engine_math::Vec2) {
        self.padding = padding;
    }

    /// 获取布局方向
    pub fn flow(&self) -> GridFlow {
        self.flow
    }

    /// 设置布局方向
    pub fn set_flow(&mut self, flow: GridFlow) {
        self.flow = flow;
    }

    /// 获取单元格固定宽度
    pub fn cell_width(&self) -> Option<f32> {
        self.cell_width
    }

    /// 设置单元格固定宽度
    pub fn set_cell_width(&mut self, width: Option<f32>) {
        self.cell_width = width.map(|w| w.max(0.0));
    }

    /// 获取单元格固定高度
    pub fn cell_height(&self) -> Option<f32> {
        self.cell_height
    }

    /// 设置单元格固定高度
    pub fn set_cell_height(&mut self, height: Option<f32>) {
        self.cell_height = height.map(|h| h.max(0.0));
    }

    /// 计算指定索引的单元格矩形
    ///
    /// `index`：子项索引
    /// `grid_rect`：网格容器矩形
    /// `item_count`：总子项数量
    pub fn cell_rect(
        &self,
        index: usize,
        grid_rect: engine_math::Rect,
        item_count: usize,
    ) -> engine_math::Rect {
        if item_count == 0 {
            return engine_math::Rect::new(0.0, 0.0, 0.0, 0.0);
        }

        let (col, row) = match self.flow {
            GridFlow::Row => {
                let cols = if self.columns > 0 {
                    self.columns
                } else {
                    // 自动计算列数：根据行数
                    let rows = if self.rows > 0 { self.rows } else { 1 };
                    (item_count + rows - 1) / rows
                };
                (index % cols, index / cols)
            }
            GridFlow::Column => {
                let rows = if self.rows > 0 {
                    self.rows
                } else {
                    let cols = if self.columns > 0 { self.columns } else { 1 };
                    (item_count + cols - 1) / cols
                };
                (index / rows, index % rows)
            }
        };

        let total_cols = match self.flow {
            GridFlow::Row => {
                if self.columns > 0 {
                    self.columns
                } else {
                    let rows = if self.rows > 0 { self.rows } else { 1 };
                    (item_count + rows - 1) / rows
                }
            }
            GridFlow::Column => {
                if self.columns > 0 {
                    self.columns
                } else {
                    1
                }
            }
        };

        let total_rows = match self.flow {
            GridFlow::Row => {
                if self.rows > 0 {
                    self.rows
                } else {
                    1
                }
            }
            GridFlow::Column => {
                if self.rows > 0 {
                    self.rows
                } else {
                    let cols = if self.columns > 0 { self.columns } else { 1 };
                    (item_count + cols - 1) / cols
                }
            }
        };

        let inner_w = (grid_rect.w - self.padding.x * 2.0).max(0.0);
        let inner_h = (grid_rect.h - self.padding.y * 2.0).max(0.0);

        let cell_w = self.cell_width.unwrap_or_else(|| {
            if total_cols > 0 {
                let total_spacing = self.spacing * (total_cols - 1) as f32;
                (inner_w - total_spacing) / total_cols as f32
            } else {
                0.0
            }
        });
        let cell_h = self.cell_height.unwrap_or_else(|| {
            if total_rows > 0 {
                let total_spacing = self.spacing * (total_rows - 1) as f32;
                (inner_h - total_spacing) / total_rows as f32
            } else {
                0.0
            }
        });

        let x = grid_rect.x + self.padding.x + col as f32 * (cell_w + self.spacing);
        let y = grid_rect.y + self.padding.y + row as f32 * (cell_h + self.spacing);

        engine_math::Rect::new(x, y, cell_w.max(0.0), cell_h.max(0.0))
    }

    /// 计算网格需要的总尺寸
    pub fn measure_size(&self, item_count: usize) -> engine_math::Vec2 {
        if item_count == 0 {
            return engine_math::Vec2::ZERO;
        }

        let (cols, rows) = match self.flow {
            GridFlow::Row => {
                let c = if self.columns > 0 {
                    self.columns
                } else {
                    let r = if self.rows > 0 { self.rows } else { 1 };
                    (item_count + r - 1) / r
                };
                let r = (item_count + c - 1) / c;
                (c, r)
            }
            GridFlow::Column => {
                let r = if self.rows > 0 {
                    self.rows
                } else {
                    let c = if self.columns > 0 { self.columns } else { 1 };
                    (item_count + c - 1) / c
                };
                let c = (item_count + r - 1) / r;
                (c, r)
            }
        };

        let cell_w = self.cell_width.unwrap_or(0.0);
        let cell_h = self.cell_height.unwrap_or(0.0);
        let total_w = cols as f32 * cell_w + self.spacing * cols.saturating_sub(1) as f32 + self.padding.x * 2.0;
        let total_h = rows as f32 * cell_h + self.spacing * rows.saturating_sub(1) as f32 + self.padding.y * 2.0;

        engine_math::Vec2::new(total_w, total_h)
    }
}

impl Component for Grid {}

impl Default for Grid {
    fn default() -> Self {
        Self {
            columns: 0,
            rows: 0,
            spacing: 0.0,
            padding: engine_math::Vec2::ZERO,
            flow: GridFlow::Row,
            cell_width: None,
            cell_height: None,
        }
    }
}

// ===== ScrollPanel 滚动面板控件 =====

/// 滚动面板控件
///
/// 提供可滚动的内容区域，支持垂直/水平滚动条、滚轮交互、内容偏移。
pub struct ScrollPanel {
    /// 内容偏移 X
    scroll_x: f32,
    /// 内容偏移 Y
    scroll_y: f32,
    /// 内容总宽度
    content_width: f32,
    /// 内容总高度
    content_height: f32,
    /// 可视区域宽度
    viewport_width: f32,
    /// 可视区域高度
    viewport_height: f32,
    /// 是否显示水平滚动条
    show_horizontal: bool,
    /// 是否显示垂直滚动条
    show_vertical: bool,
    /// 滚动条厚度
    scrollbar_thickness: f32,
    /// 是否正在拖拽垂直滚动条
    dragging_vertical: bool,
    /// 是否正在拖拽水平滚动条
    dragging_horizontal: bool,
    /// 滚轮步长
    wheel_step: f32,
}

impl ScrollPanel {
    /// 创建新的滚动面板
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建指定视口大小的滚动面板
    pub fn with_viewport(width: f32, height: f32) -> Self {
        Self {
            viewport_width: width,
            viewport_height: height,
            ..Default::default()
        }
    }

    /// 获取水平滚动偏移
    pub fn scroll_x(&self) -> f32 {
        self.scroll_x
    }

    /// 设置水平滚动偏移（自动 clamp）
    pub fn set_scroll_x(&mut self, x: f32) {
        let max = self.max_scroll_x();
        self.scroll_x = x.clamp(0.0, max);
    }

    /// 获取垂直滚动偏移
    pub fn scroll_y(&self) -> f32 {
        self.scroll_y
    }

    /// 设置垂直滚动偏移（自动 clamp）
    pub fn set_scroll_y(&mut self, y: f32) {
        let max = self.max_scroll_y();
        self.scroll_y = y.clamp(0.0, max);
    }

    /// 获取内容宽度
    pub fn content_width(&self) -> f32 {
        self.content_width
    }

    /// 设置内容宽度
    pub fn set_content_width(&mut self, width: f32) {
        self.content_width = width.max(0.0);
        // 重新 clamp 滚动偏移
        let max = self.max_scroll_x();
        if self.scroll_x > max {
            self.scroll_x = max;
        }
    }

    /// 获取内容高度
    pub fn content_height(&self) -> f32 {
        self.content_height
    }

    /// 设置内容高度
    pub fn set_content_height(&mut self, height: f32) {
        self.content_height = height.max(0.0);
        let max = self.max_scroll_y();
        if self.scroll_y > max {
            self.scroll_y = max;
        }
    }

    /// 获取视口宽度
    pub fn viewport_width(&self) -> f32 {
        self.viewport_width
    }

    /// 设置视口宽度
    pub fn set_viewport_width(&mut self, width: f32) {
        self.viewport_width = width.max(0.0);
        let max = self.max_scroll_x();
        if self.scroll_x > max {
            self.scroll_x = max;
        }
    }

    /// 获取视口高度
    pub fn viewport_height(&self) -> f32 {
        self.viewport_height
    }

    /// 设置视口高度
    pub fn set_viewport_height(&mut self, height: f32) {
        self.viewport_height = height.max(0.0);
        let max = self.max_scroll_y();
        if self.scroll_y > max {
            self.scroll_y = max;
        }
    }

    /// 是否显示水平滚动条
    pub fn show_horizontal(&self) -> bool {
        self.show_horizontal
    }

    /// 设置是否显示水平滚动条
    pub fn set_show_horizontal(&mut self, show: bool) {
        self.show_horizontal = show;
    }

    /// 是否显示垂直滚动条
    pub fn show_vertical(&self) -> bool {
        self.show_vertical
    }

    /// 设置是否显示垂直滚动条
    pub fn set_show_vertical(&mut self, show: bool) {
        self.show_vertical = show;
    }

    /// 获取滚动条厚度
    pub fn scrollbar_thickness(&self) -> f32 {
        self.scrollbar_thickness
    }

    /// 设置滚动条厚度
    pub fn set_scrollbar_thickness(&mut self, thickness: f32) {
        self.scrollbar_thickness = thickness.max(0.0);
    }

    /// 获取滚轮步长
    pub fn wheel_step(&self) -> f32 {
        self.wheel_step
    }

    /// 设置滚轮步长
    pub fn set_wheel_step(&mut self, step: f32) {
        self.wheel_step = step.max(0.0);
    }

    /// 最大水平滚动偏移
    pub fn max_scroll_x(&self) -> f32 {
        (self.content_width - self.viewport_width).max(0.0)
    }

    /// 最大垂直滚动偏移
    pub fn max_scroll_y(&self) -> f32 {
        (self.content_height - self.viewport_height).max(0.0)
    }

    /// 水平滚动比例（0.0~1.0）
    pub fn horizontal_ratio(&self) -> f32 {
        let max = self.max_scroll_x();
        if max > 0.0 {
            self.scroll_x / max
        } else {
            0.0
        }
    }

    /// 垂直滚动比例（0.0~1.0）
    pub fn vertical_ratio(&self) -> f32 {
        let max = self.max_scroll_y();
        if max > 0.0 {
            self.scroll_y / max
        } else {
            0.0
        }
    }

    /// 是否可以水平滚动
    pub fn can_scroll_horizontal(&self) -> bool {
        self.content_width > self.viewport_width
    }

    /// 是否可以垂直滚动
    pub fn can_scroll_vertical(&self) -> bool {
        self.content_height > self.viewport_height
    }

    /// 处理滚轮事件
    ///
    /// 标准滚动行为：delta.y 为正（向下滚动）→ scroll_y 增加；为负（向上滚动）→ scroll_y 减少。
    pub fn process_wheel(&mut self, delta: engine_math::Vec2) {
        if self.can_scroll_vertical() {
            self.set_scroll_y(self.scroll_y + delta.y * self.wheel_step);
        }
        if self.can_scroll_horizontal() {
            self.set_scroll_x(self.scroll_x + delta.x * self.wheel_step);
        }
    }

    /// 滚动到顶部
    pub fn scroll_to_top(&mut self) {
        self.scroll_y = 0.0;
    }

    /// 滚动到底部
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_y = self.max_scroll_y();
    }

    /// 滚动到左侧
    pub fn scroll_to_left(&mut self) {
        self.scroll_x = 0.0;
    }

    /// 滚动到右侧
    pub fn scroll_to_right(&mut self) {
        self.scroll_x = self.max_scroll_x();
    }

    /// 是否正在拖拽垂直滚动条
    pub fn is_dragging_vertical(&self) -> bool {
        self.dragging_vertical
    }

    /// 设置垂直滚动条拖拽状态
    pub fn set_dragging_vertical(&mut self, dragging: bool) {
        self.dragging_vertical = dragging;
    }

    /// 是否正在拖拽水平滚动条
    pub fn is_dragging_horizontal(&self) -> bool {
        self.dragging_horizontal
    }

    /// 设置水平滚动条拖拽状态
    pub fn set_dragging_horizontal(&mut self, dragging: bool) {
        self.dragging_horizontal = dragging;
    }

    /// 获取内容偏移（用于渲染时平移内容）
    pub fn content_offset(&self) -> engine_math::Vec2 {
        engine_math::Vec2::new(-self.scroll_x, -self.scroll_y)
    }

    /// 计算垂直滚动条矩形
    pub fn vertical_scrollbar_rect(&self, panel_rect: engine_math::Rect) -> engine_math::Rect {
        if !self.show_vertical || !self.can_scroll_vertical() {
            return engine_math::Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        let x = panel_rect.x + panel_rect.w - self.scrollbar_thickness;
        let y = panel_rect.y;
        let h = panel_rect.h;
        engine_math::Rect::new(x, y, self.scrollbar_thickness, h)
    }

    /// 计算水平滚动条矩形
    pub fn horizontal_scrollbar_rect(&self, panel_rect: engine_math::Rect) -> engine_math::Rect {
        if !self.show_horizontal || !self.can_scroll_horizontal() {
            return engine_math::Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        let x = panel_rect.x;
        let y = panel_rect.y + panel_rect.h - self.scrollbar_thickness;
        let w = panel_rect.w;
        engine_math::Rect::new(x, y, w, self.scrollbar_thickness)
    }

    /// 计算垂直滚动条手柄矩形
    pub fn vertical_handle_rect(&self, panel_rect: engine_math::Rect) -> engine_math::Rect {
        if !self.can_scroll_vertical() {
            return engine_math::Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        let bar = self.vertical_scrollbar_rect(panel_rect);
        let ratio = self.viewport_height / self.content_height;
        let handle_h = (bar.h * ratio).max(self.scrollbar_thickness * 1.5);
        let max_handle_y = bar.h - handle_h;
        let handle_y = bar.y + self.vertical_ratio() * max_handle_y;
        engine_math::Rect::new(bar.x, handle_y, bar.w, handle_h)
    }

    /// 计算水平滚动条手柄矩形
    pub fn horizontal_handle_rect(&self, panel_rect: engine_math::Rect) -> engine_math::Rect {
        if !self.can_scroll_horizontal() {
            return engine_math::Rect::new(0.0, 0.0, 0.0, 0.0);
        }
        let bar = self.horizontal_scrollbar_rect(panel_rect);
        let ratio = self.viewport_width / self.content_width;
        let handle_w = (bar.w * ratio).max(self.scrollbar_thickness * 1.5);
        let max_handle_x = bar.w - handle_w;
        let handle_x = bar.x + self.horizontal_ratio() * max_handle_x;
        engine_math::Rect::new(handle_x, bar.y, handle_w, bar.h)
    }
}

impl Component for ScrollPanel {}

impl Default for ScrollPanel {
    fn default() -> Self {
        Self {
            scroll_x: 0.0,
            scroll_y: 0.0,
            content_width: 0.0,
            content_height: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            show_horizontal: true,
            show_vertical: true,
            scrollbar_thickness: 12.0,
            dragging_vertical: false,
            dragging_horizontal: false,
            wheel_step: 30.0,
        }
    }
}

// ===== ProgressBar 进度条控件 =====

/// 进度条控件
pub struct ProgressBar {
    /// 当前进度（0.0~1.0）
    value: f32,
    /// 最小值
    min: f32,
    /// 最大值
    max: f32,
    /// 方向
    direction: SliderDirection,
    /// 是否显示文本
    show_text: bool,
    /// 是否不确定（动画）
    indeterminate: bool,
}

impl ProgressBar {
    /// 创建新的进度条
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            direction: SliderDirection::Horizontal,
            show_text: false,
            indeterminate: false,
        }
    }

    /// 获取当前值
    pub fn value(&self) -> f32 {
        self.value
    }

    /// 设置当前值
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// 获取最小值
    pub fn min(&self) -> f32 {
        self.min
    }

    /// 设置最小值
    pub fn set_min(&mut self, min: f32) {
        self.min = min;
        if self.max < min {
            self.max = min;
        }
        self.set_value(self.value);
    }

    /// 获取最大值
    pub fn max(&self) -> f32 {
        self.max
    }

    /// 设置最大值
    pub fn set_max(&mut self, max: f32) {
        self.max = max;
        if self.min > max {
            self.min = max;
        }
        self.set_value(self.value);
    }

    /// 获取进度比例（0.0~1.0）
    pub fn ratio(&self) -> f32 {
        if (self.max - self.min).abs() < f32::EPSILON {
            return 0.0;
        }
        (self.value - self.min) / (self.max - self.min)
    }

    /// 获取方向
    pub fn direction(&self) -> SliderDirection {
        self.direction
    }

    /// 设置方向
    pub fn set_direction(&mut self, direction: SliderDirection) {
        self.direction = direction;
    }

    /// 是否显示文本
    pub fn show_text(&self) -> bool {
        self.show_text
    }

    /// 设置是否显示文本
    pub fn set_show_text(&mut self, show: bool) {
        self.show_text = show;
    }

    /// 是否不确定
    pub fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }

    /// 设置不确定状态
    pub fn set_indeterminate(&mut self, indeterminate: bool) {
        self.indeterminate = indeterminate;
    }

    /// 获取百分比文本
    pub fn percentage_text(&self) -> String {
        format!("{:.0}%", self.ratio() * 100.0)
    }
}

impl Component for ProgressBar {}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

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

    // ===== Slider 测试 =====

    #[test]
    fn test_slider_creation() {
        let slider = Slider::new(0.0, 100.0, 50.0);
        assert_eq!(slider.min(), 0.0);
        assert_eq!(slider.max(), 100.0);
        assert_eq!(slider.value(), 50.0);
        assert_eq!(slider.direction(), SliderDirection::Horizontal);
        assert!(!slider.is_dragging());
        assert!(!slider.is_hovered());
        assert!(!slider.is_disabled());
    }

    #[test]
    fn test_slider_horizontal_vertical() {
        let h = Slider::horizontal(0.0, 10.0, 5.0);
        assert_eq!(h.direction(), SliderDirection::Horizontal);
        let v = Slider::vertical(0.0, 10.0, 5.0);
        assert_eq!(v.direction(), SliderDirection::Vertical);
    }

    #[test]
    fn test_slider_set_value_clamp() {
        let mut slider = Slider::new(0.0, 100.0, 50.0);
        slider.set_value(150.0);
        assert_eq!(slider.value(), 100.0);
        slider.set_value(-10.0);
        assert_eq!(slider.value(), 0.0);
    }

    #[test]
    fn test_slider_set_value_with_step() {
        let mut slider = Slider::new(0.0, 100.0, 0.0);
        slider.set_step(25.0);
        slider.set_value(30.0);
        assert_eq!(slider.value(), 25.0);
        slider.set_value(60.0);
        assert_eq!(slider.value(), 50.0);
        slider.set_value(90.0);
        assert_eq!(slider.value(), 75.0);
    }

    #[test]
    fn test_slider_ratio() {
        let slider = Slider::new(0.0, 100.0, 50.0);
        assert!((slider.ratio() - 0.5).abs() < 0.001);
        let slider2 = Slider::new(0.0, 100.0, 100.0);
        assert!((slider2.ratio() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_slider_increment_decrement() {
        let mut slider = Slider::new(0.0, 100.0, 50.0);
        slider.set_step(10.0);
        slider.increment();
        assert_eq!(slider.value(), 60.0);
        slider.decrement();
        slider.decrement();
        assert_eq!(slider.value(), 40.0);
    }

    #[test]
    fn test_slider_set_value_from_position_horizontal() {
        let mut slider = Slider::new(0.0, 100.0, 0.0);
        let rect = engine_math::Rect::new(0.0, 0.0, 200.0, 20.0);
        slider.set_value_from_position(engine_math::Vec2::new(100.0, 10.0), rect);
        assert!((slider.value() - 50.0).abs() < 0.1);
        slider.set_value_from_position(engine_math::Vec2::new(50.0, 10.0), rect);
        assert!((slider.value() - 25.0).abs() < 0.1);
    }

    #[test]
    fn test_slider_set_value_from_position_vertical() {
        let mut slider = Slider::vertical(0.0, 100.0, 0.0);
        let rect = engine_math::Rect::new(0.0, 0.0, 20.0, 200.0);
        // 垂直滑块：上=最大值
        slider.set_value_from_position(engine_math::Vec2::new(10.0, 0.0), rect);
        assert!((slider.value() - 100.0).abs() < 0.1);
        slider.set_value_from_position(engine_math::Vec2::new(10.0, 200.0), rect);
        assert!((slider.value() - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_slider_set_min_max() {
        let mut slider = Slider::new(0.0, 100.0, 50.0);
        slider.set_min(20.0);
        assert_eq!(slider.min(), 20.0);
        assert_eq!(slider.value(), 50.0);
        slider.set_max(40.0);
        assert_eq!(slider.max(), 40.0);
        assert_eq!(slider.value(), 40.0); // clamp 到 max
    }

    #[test]
    fn test_slider_handle_size() {
        let mut slider = Slider::new(0.0, 100.0, 50.0);
        slider.set_handle_size(20.0);
        assert_eq!(slider.handle_size(), 20.0);
        slider.set_handle_size(-5.0);
        assert_eq!(slider.handle_size(), 0.0);
    }

    #[test]
    fn test_slider_track_thickness() {
        let mut slider = Slider::new(0.0, 100.0, 50.0);
        slider.set_track_thickness(8.0);
        assert_eq!(slider.track_thickness(), 8.0);
    }

    #[test]
    fn test_slider_disabled_state() {
        let mut slider = Slider::new(0.0, 100.0, 50.0);
        slider.set_disabled(true);
        assert!(slider.is_disabled());
    }

    #[test]
    fn test_slider_default() {
        let slider = Slider::default();
        assert_eq!(slider.min(), 0.0);
        assert_eq!(slider.max(), 1.0);
        assert_eq!(slider.value(), 0.0);
    }

    // ===== Grid 测试 =====

    #[test]
    fn test_grid_default() {
        let grid = Grid::new();
        assert_eq!(grid.columns(), 0);
        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.spacing(), 0.0);
        assert_eq!(grid.flow(), GridFlow::Row);
    }

    #[test]
    fn test_grid_with_columns() {
        let grid = Grid::with_columns(3);
        assert_eq!(grid.columns(), 3);
    }

    #[test]
    fn test_grid_with_rows() {
        let grid = Grid::with_rows(2);
        assert_eq!(grid.rows(), 2);
    }

    #[test]
    fn test_grid_setters() {
        let mut grid = Grid::new();
        grid.set_columns(4);
        grid.set_rows(2);
        grid.set_spacing(10.0);
        grid.set_padding(engine_math::Vec2::new(5.0, 5.0));
        grid.set_flow(GridFlow::Column);
        grid.set_cell_width(Some(100.0));
        grid.set_cell_height(Some(50.0));

        assert_eq!(grid.columns(), 4);
        assert_eq!(grid.rows(), 2);
        assert_eq!(grid.spacing(), 10.0);
        assert_eq!(grid.padding(), engine_math::Vec2::new(5.0, 5.0));
        assert_eq!(grid.flow(), GridFlow::Column);
        assert_eq!(grid.cell_width(), Some(100.0));
        assert_eq!(grid.cell_height(), Some(50.0));
    }

    #[test]
    fn test_grid_cell_rect_row_flow() {
        let mut grid = Grid::with_columns(3);
        grid.set_spacing(10.0);
        let rect = engine_math::Rect::new(0.0, 0.0, 300.0, 200.0);
        // 6 个子项，3 列 2 行
        let cell0 = grid.cell_rect(0, rect, 6);
        let cell1 = grid.cell_rect(1, rect, 6);
        let cell3 = grid.cell_rect(3, rect, 6);

        // 第一个单元格应在 (0, 0)
        assert_eq!(cell0.x, 0.0);
        assert_eq!(cell0.y, 0.0);
        // 单元格宽度 = (300 - 2*10) / 3 = 93.33
        assert!((cell0.w - 93.33).abs() < 0.1);
        // 第二个单元格 x = 93.33 + 10 = 103.33
        assert!((cell1.x - 103.33).abs() < 0.1);
        // 第四个单元格（第二行第一个）y = cell_h + 10
        assert!((cell3.y - (cell0.h + 10.0)).abs() < 0.1);
    }

    #[test]
    fn test_grid_cell_rect_column_flow() {
        let mut grid = Grid::with_rows(2);
        grid.set_flow(GridFlow::Column);
        grid.set_spacing(5.0);
        let rect = engine_math::Rect::new(0.0, 0.0, 200.0, 100.0);
        // 4 个子项，2 行 2 列
        let cell0 = grid.cell_rect(0, rect, 4);
        let cell1 = grid.cell_rect(1, rect, 4);
        let cell2 = grid.cell_rect(2, rect, 4);

        // 第一个单元格 (col=0, row=0)
        assert_eq!(cell0.x, 0.0);
        assert_eq!(cell0.y, 0.0);
        // 第二个单元格 (col=0, row=1)
        assert_eq!(cell1.x, 0.0);
        assert!((cell1.y - (cell0.h + 5.0)).abs() < 0.1);
        // 第三个单元格 (col=1, row=0)
        assert!((cell2.x - (cell0.w + 5.0)).abs() < 0.1);
        assert_eq!(cell2.y, 0.0);
    }

    #[test]
    fn test_grid_measure_size() {
        let mut grid = Grid::with_columns(3);
        grid.set_cell_width(Some(100.0));
        grid.set_cell_height(Some(50.0));
        grid.set_spacing(10.0);
        grid.set_padding(engine_math::Vec2::new(5.0, 5.0));
        // 6 个子项 = 3 列 2 行
        let size = grid.measure_size(6);
        // 宽度 = 3*100 + 2*10 + 2*5 = 330
        assert!((size.x - 330.0).abs() < 0.1);
        // 高度 = 2*50 + 1*10 + 2*5 = 120
        assert!((size.y - 120.0).abs() < 0.1);
    }

    #[test]
    fn test_grid_measure_size_empty() {
        let grid = Grid::new();
        assert_eq!(grid.measure_size(0), engine_math::Vec2::ZERO);
    }

    #[test]
    fn test_grid_cell_rect_empty() {
        let grid = Grid::new();
        let rect = engine_math::Rect::new(0.0, 0.0, 100.0, 100.0);
        let cell = grid.cell_rect(0, rect, 0);
        assert_eq!(cell.w, 0.0);
        assert_eq!(cell.h, 0.0);
    }

    // ===== ScrollPanel 测试 =====

    #[test]
    fn test_scroll_panel_default() {
        let panel = ScrollPanel::new();
        assert_eq!(panel.scroll_x(), 0.0);
        assert_eq!(panel.scroll_y(), 0.0);
        assert_eq!(panel.content_width(), 0.0);
        assert_eq!(panel.content_height(), 0.0);
        assert!(panel.show_horizontal());
        assert!(panel.show_vertical());
        assert!(!panel.is_dragging_vertical());
        assert!(!panel.is_dragging_horizontal());
    }

    #[test]
    fn test_scroll_panel_with_viewport() {
        let panel = ScrollPanel::with_viewport(800.0, 600.0);
        assert_eq!(panel.viewport_width(), 800.0);
        assert_eq!(panel.viewport_height(), 600.0);
    }

    #[test]
    fn test_scroll_panel_set_scroll_clamp() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        // max_scroll_x = 200 - 100 = 100
        panel.set_scroll_x(150.0);
        assert_eq!(panel.scroll_x(), 100.0);
        panel.set_scroll_x(-10.0);
        assert_eq!(panel.scroll_x(), 0.0);
    }

    #[test]
    fn test_scroll_panel_can_scroll() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        assert!(!panel.can_scroll_horizontal());
        assert!(!panel.can_scroll_vertical());
        panel.set_content_width(200.0);
        panel.set_content_height(150.0);
        assert!(panel.can_scroll_horizontal());
        assert!(panel.can_scroll_vertical());
    }

    #[test]
    fn test_scroll_panel_max_scroll() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(250.0);
        panel.set_content_height(180.0);
        assert_eq!(panel.max_scroll_x(), 150.0);
        assert_eq!(panel.max_scroll_y(), 80.0);
    }

    #[test]
    fn test_scroll_panel_ratio() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        panel.set_scroll_x(50.0);
        panel.set_scroll_y(100.0);
        assert!((panel.horizontal_ratio() - 0.5).abs() < 0.001);
        assert!((panel.vertical_ratio() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_scroll_panel_process_wheel() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        panel.set_wheel_step(20.0);
        // 向下滚动（delta.y 为正）→ scroll_y 增加
        panel.process_wheel(engine_math::Vec2::new(0.0, 1.0));
        assert_eq!(panel.scroll_y(), 20.0);
        // 向上滚动（delta.y 为负）→ scroll_y 减少
        panel.set_scroll_y(100.0);
        panel.process_wheel(engine_math::Vec2::new(0.0, -1.0));
        assert_eq!(panel.scroll_y(), 80.0);
    }

    #[test]
    fn test_scroll_panel_scroll_to() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        panel.scroll_to_bottom();
        assert_eq!(panel.scroll_y(), 100.0);
        panel.scroll_to_top();
        assert_eq!(panel.scroll_y(), 0.0);
        panel.scroll_to_right();
        assert_eq!(panel.scroll_x(), 100.0);
        panel.scroll_to_left();
        assert_eq!(panel.scroll_x(), 0.0);
    }

    #[test]
    fn test_scroll_panel_content_offset() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        panel.set_scroll_x(30.0);
        panel.set_scroll_y(40.0);
        let offset = panel.content_offset();
        assert_eq!(offset.x, -30.0);
        assert_eq!(offset.y, -40.0);
    }

    #[test]
    fn test_scroll_panel_scrollbar_rect() {
        let panel = ScrollPanel::with_viewport(100.0, 100.0);
        let panel_rect = engine_math::Rect::new(0.0, 0.0, 200.0, 200.0);
        // 内容不足，不显示滚动条
        let v_bar = panel.vertical_scrollbar_rect(panel_rect);
        assert_eq!(v_bar.w, 0.0);
    }

    #[test]
    fn test_scroll_panel_scrollbar_rect_with_content() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        let panel_rect = engine_math::Rect::new(0.0, 0.0, 200.0, 200.0);
        let v_bar = panel.vertical_scrollbar_rect(panel_rect);
        assert_eq!(v_bar.x, 200.0 - 12.0); // 200 - thickness
        assert_eq!(v_bar.w, 12.0);
        let h_bar = panel.horizontal_scrollbar_rect(panel_rect);
        assert_eq!(h_bar.y, 200.0 - 12.0);
        assert_eq!(h_bar.h, 12.0);
    }

    #[test]
    fn test_scroll_panel_handle_rect() {
        let mut panel = ScrollPanel::with_viewport(100.0, 100.0);
        panel.set_content_width(200.0);
        panel.set_content_height(200.0);
        let panel_rect = engine_math::Rect::new(0.0, 0.0, 200.0, 200.0);
        // 滚动到中间
        panel.set_scroll_y(50.0);
        let handle = panel.vertical_handle_rect(panel_rect);
        // 视口/内容 = 100/200 = 0.5，手柄高度 = 200*0.5 = 100
        assert!((handle.h - 100.0).abs() < 0.1);
        // ratio = 50/100 = 0.5，手柄 y = 0 + 0.5 * (200-100) = 50
        assert!((handle.y - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_scroll_panel_dragging_state() {
        let mut panel = ScrollPanel::new();
        panel.set_dragging_vertical(true);
        assert!(panel.is_dragging_vertical());
        panel.set_dragging_horizontal(true);
        assert!(panel.is_dragging_horizontal());
    }

    #[test]
    fn test_scroll_panel_wheel_step() {
        let mut panel = ScrollPanel::new();
        panel.set_wheel_step(50.0);
        assert_eq!(panel.wheel_step(), 50.0);
    }

    // ===== ProgressBar 测试 =====

    #[test]
    fn test_progress_bar_default() {
        let bar = ProgressBar::new();
        assert_eq!(bar.value(), 0.0);
        assert_eq!(bar.min(), 0.0);
        assert_eq!(bar.max(), 1.0);
        assert!(!bar.show_text());
        assert!(!bar.is_indeterminate());
    }

    #[test]
    fn test_progress_bar_set_value() {
        let mut bar = ProgressBar::new();
        bar.set_value(0.5);
        assert_eq!(bar.value(), 0.5);
        bar.set_value(2.0);
        assert_eq!(bar.value(), 1.0);
        bar.set_value(-1.0);
        assert_eq!(bar.value(), 0.0);
    }

    #[test]
    fn test_progress_bar_ratio() {
        let mut bar = ProgressBar::new();
        bar.set_min(0.0);
        bar.set_max(100.0);
        bar.set_value(25.0);
        assert!((bar.ratio() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_progress_bar_percentage_text() {
        let mut bar = ProgressBar::new();
        bar.set_min(0.0);
        bar.set_max(100.0);
        bar.set_value(42.0);
        assert_eq!(bar.percentage_text(), "42%");
    }

    #[test]
    fn test_progress_bar_indeterminate() {
        let mut bar = ProgressBar::new();
        bar.set_indeterminate(true);
        assert!(bar.is_indeterminate());
    }

    #[test]
    fn test_progress_bar_show_text() {
        let mut bar = ProgressBar::new();
        bar.set_show_text(true);
        assert!(bar.show_text());
    }

    #[test]
    fn test_progress_bar_direction() {
        let mut bar = ProgressBar::new();
        bar.set_direction(SliderDirection::Vertical);
        assert_eq!(bar.direction(), SliderDirection::Vertical);
    }
}
