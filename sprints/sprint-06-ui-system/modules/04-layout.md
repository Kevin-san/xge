# 布局系统需求

## 模块概述

布局系统模块负责 UI 控件的尺寸计算和位置排列。该系统基于 ECS 架构，使用 `layout_system` 每帧重算布局。布局系统支持锚点布局（Anchor）、Flexbox 简化版、百分比尺寸、内容自适应以及 SafeArea（刘海屏/异形屏）支持。

---

## 需求清单

### 布局模式

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 19 | `UiLayout`：垂直/水平/网格/绝对 | P0 |
| 43 | 布局系统：先递归计算 desired_size，再 apply 实际 rect | P0 |
| 50 | 布局系统：支持 `flex_grow / flex_shrink / flex_basis`（Flexbox 简化版） | P0 |
| 51 | 布局系统：支持 `auto_calculate_width / auto_calculate_height` | P0 |
| 52 | 布局系统：DPI 缩放感知 | P0 |
| 53 | 布局系统：SafeArea（刘海屏/异形屏） | P1 |
| 243 | `LayoutMode::None / Vertical / Horizontal / Grid / Flex` | P0 |
| 251 | `layout_system(world)` PreUpdate 执行 | P0 |
| 252 | `layout_system` O(n) 复杂度 | P1 |
| 253 | `layout_system` DPI 缩放感知 | P0 |
| 254 | `layout_system` 支持 SafeArea | P1 |
| 255 | `layout_system` 支持自动尺寸（根据内容） | P0 |
| 256 | `layout_system` 支持百分比尺寸 | P0 |
| 257 | `layout_system` 支持 anchor 相对父节点 | P0 |
| 258 | `layout_system` 支持 margin/padding | P0 |
| 259 | `layout_system` 支持 z_index 绘制顺序 | P0 |
| 260 | `layout_system` 支持裁剪 scissor | P1 |
| 261 | `LayoutDebugPlugin` 可视化布局边框 | P2 |

### Flex 布局

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 244 | `LayoutDirection::LeftToRight / RightToLeft / TopToBottom / BottomToTop` | P0 |
| 245 | `LayoutAlignment::Start / Center / End / Fill` | P0 |
| 246 | `LayoutConstraints::min_size / max_size / available` | P0 |
| 247 | `FlexLayout::gap / flex_direction / justify / align_items / align_self` | P0 |
| 248 | `Justify::Start / Center / End / SpaceBetween / SpaceAround / SpaceEvenly` | P0 |
| 249 | `AlignItems::Start / Center / End / Stretch / Baseline` | P0 |
| 250 | `UiNode::flex_grow / flex_shrink / flex_basis` | P0 |

### 布局系统执行

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 49 | `UiLayout` 使用：`layout_system` 每帧重算 | P0 |
| 74 | `ui_node!` 宏（声明式）：`ui_node! { Panel(style) { Text("hello") Button("click") } }` | P2 |

### Bundle 支持

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 55 | `TextBundle`：spawn 一个文本节点 | P1 |
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

---

## API 签名

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
#[derive(Component)]
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

### layout_system

```rust
pub fn layout_system(world: &mut World);
```

---

## 输入/输出

| 布局计算 | 输入 | 输出 |
|----------|------|------|
| layout_system | 父节点 rect、子节点约束 | 子节点 final_rect |
| FlexLayout | flex_grow/shrink/basis、justify、align | 分配的尺寸和位置 |
| LayoutConstraints | min/max size、available space | 约束后的尺寸 |

---

## 布局算法

### 递归计算流程

```
1. 遍历所有 UI 节点（深度优先或广度优先）
2. 对于每个节点：
   a. 收集子节点列表
   b. 根据布局模式计算每个子节点的 desired_size
   c. 应用 flex_grow/flex_shrink/flex_basis
   d. 根据 justify 和 align_items 分配实际位置
   e. 考虑 margin/padding
   f. 调用子节点的 final_rect 计算
3. 返回根节点的最终尺寸
```

### Anchor 计算

```
final_position = parent_position + anchor_offset * parent_size + margin
final_size = size_mode match
    Pixels(v) => v
    Percent(p) => parent_size * p
    Auto => desired_size
```

### Flex 布局

```
main_axis = flex_direction.is_horizontal ? width : height
cross_axis = flex_direction.is_horizontal ? height : width

1. 计算所有子项的 flex_basis
2. 按 flex_grow 分配剩余空间
3. 按 flex_shrink 收缩超出空间
4. 对齐（justify_content）
5. 换行处理（如果支持）
6. 交叉轴对齐（align_items）
```

---

## 验收标准

- [ ] `layout_system` 在 PreUpdate 阶段执行
- [ ] 垂直布局正确排列子节点从上到下
- [ ] 水平布局正确排列子节点从左到右
- [ ] 网格布局正确计算行列
- [ ] Flex 布局正确处理 flex_grow/shrink/basis
- [ ] 百分比尺寸基于父节点正确计算
- [ ] 自动尺寸根据子节点内容正确计算
- [ ] DPI 缩放正确应用到布局
- [ ] SafeArea 正确预留异形屏区域
- [ ] margin/padding 正确影响布局
- [ ] z_index 正确影响绘制顺序
- [ ] scissor 裁剪正确应用到子节点
- [ ] 布局算法复杂度为 O(n)

---

## 依赖关系

- 依赖 `UiNode`、`UiRect` 核心组件
- 依赖 `LayoutMode`、`FlexLayout` 等布局组件
- 被渲染系统依赖（获取最终 rect）

---

## 优先级说明

- **P0**：核心布局算法缺失会导致 UI 无法正常显示
- **P1**：重要增强功能，影响复杂布局支持
- **P2**：辅助功能，可后续补充
