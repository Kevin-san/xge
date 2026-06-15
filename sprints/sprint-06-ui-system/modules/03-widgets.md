# 控件库需求

## 模块概述

控件库模块基于 UI 核心组件，提供完整的 UI 控件集合。本模块包含 Button、Label、Image、Input、Slider、Panel、Grid、List、ScrollView、Toggle、ProgressBar、Tooltip、Window、Canvas 等控件，每个控件都是基于 ECS 架构的组件封装。

---

## 需求清单

### 基础控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 5 | `UiText` 组件：文本内容 / 字体 / 字号 / 颜色 / 对齐 / 换行模式 | P0 |
| 6 | `UiImage` 组件：texture 句柄 / 九宫格切片 / UV | P0 |
| 7 | `UiButton` 组件：label / normal / hovered / pressed style / disabled | P0 |
| 8 | `UiInput` 组件：文本输入 / 光标位置 / 选中区 / placeholder | P0 |
| 9 | `UiSlider` 组件：min/max/value/step/方向 | P0 |
| 10 | `UiToggle` 组件：bool 值 + 样式 | P0 |
| 11 | `UiPanel` 组件：容器 + 布局模式 | P0 |
| 12 | `UiGrid` 组件：行/列/间距/对齐 | P0 |
| 13 | `UiList` 组件：垂直/水平列表 + 子项 | P0 |
| 14 | `UiScroll` 组件：内容区域 + 滚动条 + 鼠标滚轮支持 | P0 |
| 15 | `UiProgressBar`：progress 0~1 | P1 |
| 16 | `UiTooltip`：悬停显示气泡 | P2 |
| 17 | `UiWindow`：可拖动/可关闭窗口（用于编辑器后续） | P2 |
| 18 | `UiCanvas`：根组件 + DPI + screen_size | P0 |

### UiButton 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 191 | `UiButton::label(&self) -> &str` | P0 |
| 192 | `UiButton::state(&self) -> ButtonState` | P0 |
| 193 | `UiButton::set_state(&mut self, state)` | P0 |
| 194 | `UiButton::is_disabled(&self) -> bool` | P0 |
| 195 | `UiButton::set_disabled(&mut self, bool)` | P0 |
| 196 | `ButtonState::Normal / Hovered / Pressed / Disabled / Selected` | P0 |
| 45 | `UiButton` 状态：Normal / Hovered / Pressed / Disabled / Selected | P0 |

### UiInput 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 197 | `UiInput::text(&self) -> &str` | P0 |
| 198 | `UiInput::set_text(&mut self, text)` | P0 |
| 199 | `UiInput::cursor(&self) -> usize` | P0 |
| 200 | `UiInput::set_cursor(&mut self, pos)` | P0 |
| 201 | `UiInput::select_range(&self) -> Range<usize>` | P0 |
| 202 | `UiInput::insert_char(&mut self, ch)` | P0 |
| 203 | `UiInput::delete_backward(&mut self)` | P0 |
| 204 | `UiInput::delete_forward(&mut self)` | P0 |
| 205 | `UiInput::max_length(&self) -> Option<usize>` | P1 |
| 206 | `UiInput::set_max_length(&mut self, opt_len)` | P1 |
| 207 | `UiInput::is_password(&self) -> bool` | P1 |
| 208 | `UiInput::password_char(&self) -> char` | P1 |
| 209 | `UiInput::placeholder(&self) -> &str` | P1 |
| 210 | `UiInput::set_placeholder(&mut self, text)` | P1 |
| 211 | `UiInput::numeric_mode(&self) -> bool` | P1 |
| 212 | `UiInput::set_numeric_mode(&mut self, bool)` | P1 |
| 42 | `UiInput` 支持：光标移动 / 删除 / 选中 / 复制 / 粘贴 / 撤销 | P0 |
| 43 | `UiInput` 支持：限制长度 / 数字模式 / 密码模式（隐藏字符） | P0 |

### UiSlider 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 213 | `UiSlider::value(&self) -> f32` | P0 |
| 214 | `UiSlider::set_value(&mut self, v)` — clamp 到 [min, max] | P0 |
| 215 | `UiSlider::min(&self) / max(&self) / step(&self)` | P0 |
| 216 | `UiSlider::orientation(&self) -> H / V` | P0 |
| 44 | `UiSlider` 支持：可拖动 / 点击跳转 / 步长 | P0 |

### UiToggle 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 217 | `UiToggle::value(&self) -> bool` | P0 |
| 218 | `UiToggle::set_value(&mut self, bool)` | P0 |

### UiProgressBar 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 219 | `UiProgressBar::progress(&self) -> f32` | P1 |
| 220 | `UiProgressBar::set_progress(&mut self, v)` | P1 |

### UiPanel 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 221 | `UiPanel::layout_mode(&self) -> LayoutMode` | P0 |

### UiGrid 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 222 | `UiGrid::rows(&self) / cols(&self) / gap(&self)` | P0 |
| 223 | `UiGrid::set_rows(&mut self, rows)` | P0 |
| 224 | `UiGrid::set_cols(&mut self, cols)` | P0 |
| 225 | `UiGrid::set_gap(&mut self, gap)` | P0 |
| 47 | `UiGrid`：列自动布局 | P0 |

### UiList 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 226 | `UiList::items(&self) -> &[Entity]` | P0 |
| 227 | `UiList::add_item(&mut self, item)` | P0 |
| 228 | `UiList::remove_item(&mut self, index)` | P0 |
| 229 | `UiList::direction(&self) -> V / H` | P0 |
| 48 | `UiList`：支持虚拟化（仅渲染可见部分） | P1 |

### UiScroll 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 230 | `UiScroll::content(&self) -> Entity` | P0 |
| 231 | `UiScroll::scroll_offset(&self) -> Vec2` | P0 |
| 232 | `UiScroll::set_scroll_offset(&mut self, v)` | P0 |
| 233 | `UiScroll::scrollbar_visible(&self) -> bool` | P1 |
| 46 | `UiScroll`：内容区域 + 滑块 + 步长 + 页跳 | P0 |

### UiTooltip 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 234 | `UiTooltip::text(&self) -> &str` | P2 |
| 235 | `UiTooltip::delay(&self) -> f32` | P2 |

### UiWindow 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 236 | `UiWindow::title(&self) -> &str` | P2 |
| 237 | `UiWindow::draggable(&self) -> bool` | P2 |
| 238 | `UiWindow::resizable(&self) -> bool` | P2 |
| 239 | `UiWindow::closable(&self) -> bool` | P2 |

### UiCanvas 控件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 240 | `UiCanvas::size(&self) -> Vec2` | P0 |
| 241 | `UiCanvas::dpi(&self) -> f64` | P0 |
| 242 | `UiCanvas::safe_area(&self) -> Rect` | P1 |

### Bundle 便捷构造

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 56 | `TextBundle`：spawn 一个文本节点 | P1 |
| 57 | `ImageBundle`：spawn 一个图片节点 | P1 |
| 58 | `ButtonBundle`：spawn 一个按钮节点 | P1 |
| 59 | `InputBundle`：spawn 一个输入框 | P1 |
| 60 | `SliderBundle`：spawn 一个滑块 | P1 |
| 61 | `PanelBundle`：spawn 一个面板容器 | P1 |
| 62 | `GridBundle`：spawn 一个网格容器 | P1 |
| 63 | `ListBundle`：spawn 一个列表容器 | P1 |
| 64 | `ScrollBundle`：spawn 一个滚动容器 | P1 |
| 65 | `CanvasBundle`：spawn UI Canvas | P1 |
| 66 | `UiCommands`：便捷 API 创建 UI | P1 |

---

## API 签名

### UiButton

```rust
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
pub struct UiPanel {
    pub layout_mode: LayoutMode,
}

impl UiPanel {
    pub fn layout_mode(&self) -> LayoutMode;
}
```

### UiGrid

```rust
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
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
#[derive(Component)]
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

### UiCanvas

```rust
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

---

## 输入/输出

| 控件 | 输入 | 输出 |
|------|------|------|
| UiButton | label, state | 按钮渲染状态 |
| UiInput | text, cursor, selection | 输入框内容 |
| UiSlider | value, min, max, step | 滑块值 |
| UiToggle | value | 开关状态 |
| UiProgressBar | progress | 进度条显示 |
| UiPanel | layout_mode | 子控件布局 |
| UiGrid | rows, cols, gap | 网格布局 |
| UiList | items, direction | 列表显示 |
| UiScroll | content, scroll_offset | 滚动视图 |
| UiTooltip | text, delay | 提示显示 |
| UiWindow | title, draggable, resizable, closable | 窗口控制 |
| UiCanvas | size, dpi | 画布配置 |

---

## 验收标准

- [ ] `UiButton` 支持 Normal/Hovered/Pressed/Disabled/Selected 五种状态
- [ ] `UiInput` 支持文本输入、光标移动、删除、选中操作
- [ ] `UiInput` 支持密码模式（显示遮罩字符）
- [ ] `UiInput` 支持数字模式（仅允许数字输入）
- [ ] `UiSlider` 值被正确 clamp 到 [min, max]
- [ ] `UiToggle` 正确切换 bool 值
- [ ] `UiProgressBar` progress 值在 0~1 范围内
- [ ] `UiGrid` 支持动态设置行列数和间距
- [ ] `UiList` 支持添加/删除子项
- [ ] `UiScroll` 支持鼠标滚轮滚动
- [ ] `UiCanvas` 正确处理 DPI 缩放
- [ ] 所有 Bundle 可正确 spawn UI 节点

---

## 依赖关系

- 依赖 `UiNode`、`UiRect`、`UiStyle` 核心组件
- 依赖 `Font`、`TextLayout` 文本系统
- 被布局系统、事件系统依赖

---

## 优先级说明

- **P0**：核心控件缺失会导致 UI 无法使用
- **P1**：重要增强功能，提升用户体验
- **P2**：辅助功能，可后续补充
