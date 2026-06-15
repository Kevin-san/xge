# UI API 清单

## 概述

本文档列出 `engine-ui` crate 的所有公开 API，按模块组织。所有 API 应提供完整的 doc comment，并达到 100% 覆盖率要求。

---

## 模块索引

- [核心组件](#核心组件)
- [文本与字体](#文本与字体)
- [控件库](#控件库)
- [布局系统](#布局系统)
- [事件系统](#事件系统)
- [渲染系统](#渲染系统)

---

## 核心组件

### UiNode

```rust
/// UI 节点组件，标记一个 ECS Entity 为 UI 元素
#[derive(Component, Default)]
pub struct UiNode;
```

### UiRect

```rust
/// UI 矩形组件，定义节点的位置和尺寸
#[derive(Component)]
pub struct UiRect {
    /// 相对父节点的偏移
    pub position: Vec2,
    /// 宽高尺寸
    pub size: Vec2,
    /// 锚点位置
    pub anchor: Anchor,
    /// 外边距
    pub margin: UiMargin,
    /// 内边距
    pub padding: UiPadding,
    /// 绘制顺序
    pub z_index: i32,
}

impl UiRect {
    /// 计算期望尺寸
    pub fn desired_size(&self) -> Vec2;
    /// 基于父 rect 计算最终 rect
    pub fn final_rect(&self, parent_rect: Rect) -> Rect;
    /// 获取可见性状态
    pub fn visible(&self) -> bool;
}
```

### UiStyle

```rust
/// UI 样式组件
#[derive(Component)]
pub struct UiStyle {
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub box_shadow: Option<BoxShadow>,
    pub opacity: f32,
}

impl UiStyle {
    /// 合并两个样式
    pub fn merge(&self, other: &UiStyle) -> UiStyle;
}
```

### UiTheme

```rust
/// UI 主题
pub struct UiTheme {
    pub name: String,
    pub styles: HashMap<String, UiStyle>,
}

impl UiTheme {
    /// 创建亮色主题
    pub fn default_light() -> Self;
    /// 创建暗色主题
    pub fn default_dark() -> Self;
    /// 应用主题到节点
    pub fn apply(&self, ui_node: Entity);
    /// 获取样式
    pub fn get_style(&self, class: &str) -> &UiStyle;
}
```

### UiCanvas

```rust
/// UI 画布，根组件
#[derive(Component)]
pub struct UiCanvas {
    pub size: Vec2,
    pub dpi: f64,
    pub safe_area: Rect,
}

impl UiCanvas {
    pub fn size(&self) -> Vec2;
    pub fn dpi(&self) -> f64;
    pub fn safe_area(&self) -> Rect;
}
```

### 枚举类型

```rust
/// 锚点位置
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom(Vec2),
}

/// UI 尺寸模式
pub enum UiSize {
    Pixels(f32),
    Percent(f32),
    Auto,
}

/// 外边距
pub struct UiMargin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

/// 可见性状态
pub enum UiVisibility {
    Visible,
    Hidden,
    Collapsed,
}
```

---

## 文本与字体

### Font

```rust
/// 字体句柄
pub struct Font;

impl Font {
    /// 从文件加载字体
    pub fn from_file(path: &str) -> Result<Font>;
    /// 从字节数组加载
    pub fn load_from_bytes(bytes: &[u8]) -> Result<Font>;
    /// 获取字体名称
    pub fn name(&self) -> &str;
    /// 检查是否包含字形
    pub fn has_glyph(&self, ch: char) -> bool;
    /// 获取行高
    pub fn line_height(&self, size: f32) -> f32;
    /// 获取字形
    pub fn get_glyph(&self, ch: char, size: f32) -> Glyph;
    /// 获取字距
    pub fn get_kerning(&self, a: char, b: char) -> f32;
    /// 测量文本尺寸
    pub fn measure(&self, text: &str, size: f32) -> Vec2;
}
```

### Glyph

```rust
/// 字形信息
pub struct Glyph {
    pub index: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub uv_rect: Rect,
}
```

### FontAtlasBuilder

```rust
/// 字形图集构建器
pub struct FontAtlasBuilder;

impl FontAtlasBuilder {
    pub fn new() -> Self;
    pub fn add_font(&mut self, font: &Font, size: f32, charset: &str);
    pub fn build(&self, ctx: &mut Context) -> Result<FontAtlas>;
}
```

### FontAtlas

```rust
/// 字形图集
pub struct FontAtlas;

impl FontAtlas {
    pub fn texture(&self) -> TextureHandle;
    pub fn get_uv(&self, ch: char) -> Option<Rect>;
    pub fn get_glyph(&self, ch: char) -> Option<Glyph>;
    pub fn font_size(&self) -> f32;
    pub fn get_kerning(&self, a: char, b: char) -> f32;
}
```

### TextLayout

```rust
/// 文本布局
pub struct TextLayout;

impl TextLayout {
    pub fn new(
        font: &Font,
        size: f32,
        text: &str,
        max_width: f32,
        align: TextAlign,
    ) -> Self;
    pub fn glyphs(&self) -> &[Glyph];
    pub fn lines(&self) -> &[Line];
    pub fn size(&self) -> Vec2;
    pub fn char_index_at(&self, pos: Vec2) -> usize;
}
```

### RichText

```rust
/// 富文本
pub struct RichText {
    sections: Vec<TextSection>,
}

impl RichText {
    pub fn new() -> Self;
    pub fn push(&mut self, section: TextSection);
}
```

### TextSection

```rust
/// 文本段落
pub struct TextSection {
    pub text: String,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub bold: bool,
    pub italic: bool,
    pub font: Option<Handle<Font>>,
}

impl TextSection {
    pub fn new(text: &str, style: &TextStyle) -> Self;
    pub fn with_color(mut self, color: Color) -> Self;
    pub fn with_size(mut self, size: f32) -> Self;
    pub fn with_bold(mut self) -> Self;
    pub fn with_italic(mut self) -> Self;
    pub fn with_font(mut self, font: Handle<Font>) -> Self;
}
```

### 枚举类型

```rust
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

pub enum TextOverflow {
    Wrap,
    Clip,
    Ellipsis,
}
```

---

## 控件库

### UiButton

```rust
pub struct UiButton {
    pub label: String,
    pub state: ButtonState,
    pub disabled: bool,
}

pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
    Selected,
}

impl UiButton {
    pub fn label(&self) -> &str;
    pub fn state(&self) -> ButtonState;
    pub fn set_state(&mut self, state: ButtonState);
    pub fn is_disabled(&self) -> bool;
    pub fn set_disabled(&mut self, disabled: bool);
}
```

### UiInput

```rust
pub struct UiInput {
    pub text: String,
    pub cursor: usize,
    pub selection: Range<usize>,
    pub max_length: Option<usize>,
    pub is_password: bool,
    pub password_char: char,
    pub placeholder: String,
    pub numeric_mode: bool,
}

impl UiInput {
    pub fn text(&self) -> &str;
    pub fn set_text(&mut self, text: &str);
    pub fn cursor(&self) -> usize;
    pub fn set_cursor(&mut self, pos: usize);
    pub fn select_range(&self) -> Range<usize>;
    pub fn insert_char(&mut self, ch: char);
    pub fn delete_backward(&mut self);
    pub fn delete_forward(&mut self);
    pub fn max_length(&self) -> Option<usize>;
    pub fn set_max_length(&mut self, opt_len: Option<usize>);
    pub fn is_password(&self) -> bool;
    pub fn password_char(&self) -> char;
    pub fn placeholder(&self) -> &str;
    pub fn set_placeholder(&mut self, text: &str);
    pub fn numeric_mode(&self) -> bool;
    pub fn set_numeric_mode(&mut self, numeric: bool);
}
```

### UiSlider

```rust
pub struct UiSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub orientation: Orientation,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl UiSlider {
    pub fn value(&self) -> f32;
    pub fn set_value(&mut self, v: f32);
    pub fn min(&self) -> f32;
    pub fn max(&self) -> f32;
    pub fn step(&self) -> f32;
    pub fn orientation(&self) -> Orientation;
}
```

### UiToggle

```rust
pub struct UiToggle {
    pub value: bool,
}

impl UiToggle {
    pub fn value(&self) -> bool;
    pub fn set_value(&mut self, value: bool);
}
```

### UiProgressBar

```rust
pub struct UiProgressBar {
    pub progress: f32,
}

impl UiProgressBar {
    pub fn progress(&self) -> f32;
    pub fn set_progress(&mut self, v: f32);
}
```

### UiPanel

```rust
pub struct UiPanel {
    pub layout_mode: LayoutMode,
}

impl UiPanel {
    pub fn layout_mode(&self) -> LayoutMode;
}
```

### UiGrid

```rust
pub struct UiGrid {
    pub rows: u32,
    pub cols: u32,
    pub gap: Vec2,
}

impl UiGrid {
    pub fn rows(&self) -> u32;
    pub fn cols(&self) -> u32;
    pub fn gap(&self) -> Vec2;
    pub fn set_rows(&mut self, rows: u32);
    pub fn set_cols(&mut self, cols: u32);
    pub fn set_gap(&mut self, gap: Vec2);
}
```

### UiList

```rust
pub struct UiList {
    pub items: Vec<Entity>,
    pub direction: Direction,
}

pub enum Direction {
    Vertical,
    Horizontal,
}

impl UiList {
    pub fn items(&self) -> &[Entity];
    pub fn add_item(&mut self, item: Entity);
    pub fn remove_item(&mut self, index: usize);
    pub fn direction(&self) -> Direction;
}
```

### UiScroll

```rust
pub struct UiScroll {
    pub content: Entity,
    pub scroll_offset: Vec2,
    pub scrollbar_visible: bool,
}

impl UiScroll {
    pub fn content(&self) -> Entity;
    pub fn scroll_offset(&self) -> Vec2;
    pub fn set_scroll_offset(&mut self, v: Vec2);
    pub fn scrollbar_visible(&self) -> bool;
}
```

### UiTooltip

```rust
pub struct UiTooltip {
    pub text: String,
    pub delay: f32,
}

impl UiTooltip {
    pub fn text(&self) -> &str;
    pub fn delay(&self) -> f32;
}
```

### UiWindow

```rust
pub struct UiWindow {
    pub title: String,
    pub draggable: bool,
    pub resizable: bool,
    pub closable: bool,
}

impl UiWindow {
    pub fn title(&self) -> &str;
    pub fn draggable(&self) -> bool;
    pub fn resizable(&self) -> bool;
    pub fn closable(&self) -> bool;
}
```

---

## 布局系统

### LayoutMode

```rust
pub enum LayoutMode {
    None,
    Vertical,
    Horizontal,
    Grid,
    Flex,
}
```

### LayoutDirection

```rust
pub enum LayoutDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}
```

### LayoutAlignment

```rust
pub enum LayoutAlignment {
    Start,
    Center,
    End,
    Fill,
}
```

### LayoutConstraints

```rust
pub struct LayoutConstraints {
    pub min_size: Vec2,
    pub max_size: Vec2,
    pub available: Vec2,
}
```

### FlexLayout

```rust
pub struct FlexLayout {
    pub gap: f32,
    pub flex_direction: LayoutDirection,
    pub justify: Justify,
    pub align_items: AlignItems,
    pub align_self: AlignItems,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: f32,
}
```

### Justify

```rust
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
```

### AlignItems

```rust
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}
```

### 布局系统

```rust
/// 布局系统，每帧在 PreUpdate 执行
pub fn layout_system(world: &mut World);
```

---

## 事件系统

### UiEvent

```rust
pub enum UiEvent {
    Click(Entity),
    Press(Entity),
    Release(Entity),
    HoverEnter(Entity),
    HoverLeave(Entity),
    DragStart(Entity),
    Drag(Entity, Vec2),
    DragEnd(Entity),
    Focus(Entity),
    Blur(Entity),
    TextInput(Entity, String),
    ValueChanged(Entity, String),
    ValueChangedSlider(Entity, f32),
    Toggled(Entity, bool),
    Scroll(Entity, Vec2),
}
```

### UiEventReader

```rust
pub struct UiEventReader<E: Component>;

impl<E: Component> UiEventReader<E> {
    pub fn read(&mut self, world: &World) -> Vec<&E>;
}
```

### UiEventWriter

```rust
pub struct UiEventWriter<E: Component>;

impl<E: Component> UiEventWriter<E> {
    pub fn write(&mut self, entity: Entity, event: E);
}
```

### UiEventQueue

```rust
pub struct UiEventQueue;

impl UiEventQueue {
    pub fn enqueue(&mut self, event: impl Component);
    pub fn drain(&mut self) -> Vec<Box<dyn Component>>;
}
```

### hit_test_system

```rust
pub fn hit_test_system(world: &World, mouse_pos: Vec2) -> Option<Entity>;
```

### focus_system

```rust
pub fn focus_system(world: &mut World, event: &InputEvent);
```

### ui_event_dispatch_system

```rust
pub fn ui_event_dispatch_system(world: &mut World);
```

### ui_event_reader_system

```rust
pub fn ui_event_reader_system(world: &mut World);
```

---

## 渲染系统

### ui_render_system

```rust
pub fn ui_render_system(world: &World, ctx: &mut Context);
```

### 渲染批处理统计

```rust
pub struct UiBatchStats {
    pub batch_count: usize,
    pub vertex_count: usize,
}

pub fn ui_batch_stats(world: &World) -> UiBatchStats;
```

---

## Bundle 构造器

```rust
pub struct UiNodeBundle;
pub struct TextBundle;
pub struct ImageBundle;
pub struct ButtonBundle;
pub struct InputBundle;
pub struct SliderBundle;
pub struct PanelBundle;
pub struct GridBundle;
pub struct ListBundle;
pub struct ScrollBundle;
pub struct CanvasBundle;

pub struct UiCommands<'a> {
    world: &'a mut World,
}

impl<'a> UiCommands<'a> {
    pub fn new(world: &'a mut World) -> Self;
    pub fn spawn_button(&mut self, label: &str) -> Entity;
    pub fn spawn_text(&mut self, text: &str) -> Entity;
    // ... 其他 spawn 方法
}
```

### ui_node! 宏

```rust
ui_node! {
    Panel(style) {
        Text("hello")
        Button("click")
    }
}
```

---

## 调试功能

### UiDebugPlugin

```rust
pub struct UiDebugPlugin;

impl Plugin for UiDebugPlugin {
    fn build(&self, app: &mut App);
}
```

### LayoutDebugPlugin

```rust
pub struct LayoutDebugPlugin;

impl Plugin for LayoutDebugPlugin {
    fn build(&self, app: &mut App);
}
```

---

## API 覆盖要求

| 需求ID | 要求 |
|--------|------|
| 338 | 公开 API doc comment 覆盖率 100% |
| 339 | `unsafe` 块 <= 2 |
