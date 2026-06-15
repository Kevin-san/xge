# UI 核心组件需求

## 模块概述

UI 核心组件模块定义了 `engine-ui` crate 的基础组件体系，基于 ECS 架构（Entity=控件，Component=UI属性）。本模块涵盖 UiNode、UiRect、UiStyle、UiTheme 等核心组件，为整个 UI 系统提供基础数据结构支撑。

---

## 需求清单

### 基础组件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 1 | `engine-ui` crate 建立 | P0 |
| 2 | `UiNode` 组件：entity 上的 UI 节点标记 | P0 |
| 26 | `UiEvent` 枚举：Click/Press/Release/HoverEnter/HoverLeave/DragStart/Drag/DragEnd/Focus/Blur/TextInput/ValueChanged | P0 |
| 27 | `UiEventReader<E>` | P0 |
| 28 | `UiEventWriter<E>` | P0 |
| 53 | `UiStyleSheet`：统一样式（类似 CSS subset） | P1 |
| 54 | `UiTheme`：亮色/暗色/自定义主题 | P1 |
| 79 | `UiNodeBundle`：spawn 一个 UI 节点 | P1 |
| 90 | `ui_node!` 宏（声明式）：`ui_node! { Panel(style) { Text("hello") Button("click") } }` | P2 |

### UiRect 组件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 141 | `UiNode` 组件 + Bundle：默认值、公开字段 | P0 |
| 142 | `UiRect::position`：相对父节点的偏移 | P0 |
| 143 | `UiRect::size`：宽高 | P0 |
| 144 | `UiRect::anchor`：锚点 | P0 |
| 145 | `UiRect::margin`：外边距 | P0 |
| 146 | `UiRect::padding`：内边距 | P0 |
| 147 | `UiRect::z_index`：绘制顺序 | P0 |
| 148 | `UiRect::desired_size(&self) -> Vec2` | P0 |
| 149 | `UiRect::final_rect(&self, parent_rect) -> Rect` | P0 |
| 150 | `UiRect::visible(&self) -> bool` | P0 |
| 3 | `UiRect` 组件：position / size / anchor / margin / padding | P0 |

### UiStyle 组件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 151 | `UiStyle::background_color` | P0 |
| 152 | `UiStyle::border_color` | P0 |
| 153 | `UiStyle::border_radius` | P0 |
| 154 | `UiStyle::border_width` | P0 |
| 155 | `UiStyle::box_shadow`（偏移 + 模糊 + 颜色） | P1 |
| 156 | `UiStyle::opacity`（0~1） | P1 |
| 157 | `UiStyle::merge(&self, other)` 主题合并 | P1 |
| 4 | `UiStyle` 组件：背景色 / 边框 / 圆角 / 阴影 | P0 |

### UiTheme 组件

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 158 | `UiTheme::default_light() / default_dark()` | P1 |
| 159 | `UiTheme::apply(&self, ui_node)` 应用样式 | P1 |
| 160 | `UiTheme::get_style(&self, class) -> &UiStyle` | P1 |

### 枚举类型

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 20 | `Anchor`：TopLeft / TopCenter / TopRight / CenterLeft / Center / CenterRight / BottomLeft / BottomCenter / BottomRight / Custom(Vec2) | P0 |
| 21 | `UiSize`：像素绝对 / 百分比 / 内容自动 | P0 |
| 22 | `UiMargin`：left/right/top/bottom | P0 |
| 23 | `UiVisibility`：Visible / Hidden / Collapsed | P0 |
| 24 | `UiFocus`：当前焦点 entity + Tab 顺序 | P0 |
| 25 | `UiZIndex`：z 排序 | P0 |

---

## API 签名

### UiNode

```rust
#[derive(Component, Default)]
pub struct UiNode {
    pub entity: Entity,
}
```

### UiRect

```rust
#[derive(Component)]
pub struct UiRect {
    pub position: Vec2,
    pub size: Vec2,
    pub anchor: Anchor,
    pub margin: UiMargin,
    pub padding: UiPadding,
    pub z_index: i32,
}

impl UiRect {
    pub fn desired_size(&self) -> Vec2;
    pub fn final_rect(&self, parent_rect: Rect) -> Rect;
    pub fn visible(&self) -> bool;
}
```

### UiStyle

```rust
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
    pub fn merge(&self, other: &UiStyle) -> UiStyle;
}
```

### UiTheme

```rust
pub struct UiTheme {
    pub name: String,
    pub styles: HashMap<String, UiStyle>,
}

impl UiTheme {
    pub fn default_light() -> Self;
    pub fn default_dark() -> Self;
    pub fn apply(&self, ui_node: Entity);
    pub fn get_style(&self, class: &str) -> &UiStyle;
}
```

### Anchor

```rust
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
```

### UiSize

```rust
pub enum UiSize {
    Pixels(f32),
    Percent(f32),
    Auto,
}
```

### UiMargin

```rust
pub struct UiMargin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
```

### UiVisibility

```rust
pub enum UiVisibility {
    Visible,
    Hidden,
    Collapsed,
}
```

---

## 输入/输出

| 组件 | 输入 | 输出 |
|------|------|------|
| UiNode | ECS Entity | UI 节点标记 |
| UiRect | 父节点 rect、自身属性 | 计算后的最终 rect |
| UiStyle | 主题、样式属性 | 渲染样式 |
| UiTheme | 样式定义 | 应用到节点的样式 |

---

## 验收标准

- [ ] `UiNode` 可作为 ECS Component 正常工作
- [ ] `UiRect::desired_size` 返回正确的期望尺寸
- [ ] `UiRect::final_rect` 基于父 rect 和锚点计算正确
- [ ] `UiStyle::merge` 正确合并两个样式
- [ ] `UiTheme::default_light/dark` 返回预设主题
- [ ] `Anchor` 所有变体可正确计算偏移
- [ ] `UiVisibility::visible` 正确反映节点可见性

---

## 依赖关系

- 依赖 `engine-ecs` crate
- 依赖 `engine-render` crate（用于 Color 类型）
- 被所有 UI 控件组件依赖

---

## 优先级说明

- **P0**：核心数据结构缺失会导致系统无法运行
- **P1**：重要功能但有变通方案
- **P2**：辅助功能，可后续补充
