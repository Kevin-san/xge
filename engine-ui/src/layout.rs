//! 布局模块
//!
//! 定义 UI 布局类型和属性，包含 Flex 布局引擎与 Anchor 锚点适配逻辑。

use engine_math::{Rect, Vec2};

/// 布局类型
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutType {
    /// 无布局
    None,
    /// 水平布局
    Horizontal,
    /// 垂直布局
    Vertical,
    /// Flex 布局（支持 grow/shrink/basis）
    Flex,
    /// Anchor 锚点布局
    Anchor,
}

/// 布局方向
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutDirection {
    /// 水平方向
    Horizontal,
    /// 垂直方向
    Vertical,
}

/// Flex 主轴方向
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum FlexDirection {
    /// 水平排列（左→右）
    #[default]
    Row,
    /// 水平反向排列（右→左）
    RowReverse,
    /// 垂直排列（上→下）
    Column,
    /// 垂直反向排列（下→上）
    ColumnReverse,
}

impl FlexDirection {
    /// 是否为水平方向
    pub fn is_row(self) -> bool {
        matches!(self, FlexDirection::Row | FlexDirection::RowReverse)
    }

    /// 是否为垂直方向
    pub fn is_column(self) -> bool {
        matches!(self, FlexDirection::Column | FlexDirection::ColumnReverse)
    }
}

/// Flex 主轴对齐方式（justify-content）
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum JustifyContent {
    /// 起始对齐
    #[default]
    FlexStart,
    /// 末尾对齐
    FlexEnd,
    /// 居中对齐
    Center,
    /// 等间距，首尾无间距
    SpaceBetween,
    /// 等间距，首尾为一半间距
    SpaceAround,
    /// 等间距，首尾为完整间距
    SpaceEvenly,
}

/// Flex 交叉轴对齐方式（align-items）
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum AlignItems {
    /// 起始对齐
    FlexStart,
    /// 末尾对齐
    FlexEnd,
    /// 居中对齐
    Center,
    /// 拉伸填充
    #[default]
    Stretch,
    /// 基线对齐
    Baseline,
}

/// Flex 单个子项的交叉轴对齐覆盖（align-self）
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum AlignSelf {
    /// 继承父级 align-items
    #[default]
    Auto,
    /// 起始对齐
    FlexStart,
    /// 末尾对齐
    FlexEnd,
    /// 居中对齐
    Center,
    /// 拉伸填充
    Stretch,
    /// 基线对齐
    Baseline,
}

/// Flex 包装模式（flex-wrap）
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum FlexWrap {
    /// 不换行
    #[default]
    NoWrap,
    /// 换行
    Wrap,
    /// 反向换行
    WrapReverse,
}

/// 尺寸值（用于 width/height/flex-basis）
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Dimension {
    /// 自动（由内容决定）
    #[default]
    Auto,
    /// 固定像素值
    Pixels(f32),
    /// 百分比（0.0~1.0，相对父容器）
    Percent(f32),
}

impl Dimension {
    /// 创建固定像素值
    pub const fn px(value: f32) -> Self {
        Dimension::Pixels(value)
    }

    /// 创建百分比值
    pub const fn percent(value: f32) -> Self {
        Dimension::Percent(value)
    }

    /// 解析为具体像素值
    pub fn resolve(self, parent_size: f32, content_size: f32) -> f32 {
        match self {
            Dimension::Auto => content_size,
            Dimension::Pixels(v) => v.max(0.0),
            Dimension::Percent(p) => (parent_size * p.clamp(0.0, 1.0)).max(0.0),
        }
    }

    /// 是否为 Auto
    pub fn is_auto(self) -> bool {
        matches!(self, Dimension::Auto)
    }

    /// 是否为固定像素值
    pub fn is_pixels(self) -> bool {
        matches!(self, Dimension::Pixels(_))
    }
}

/// Flex 子项属性
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FlexItem {
    /// flex-grow：分配剩余空间的比例
    pub grow: f32,
    /// flex-shrink：空间不足时的收缩比例
    pub shrink: f32,
    /// flex-basis：初始主轴尺寸
    pub basis: Dimension,
    /// 主轴尺寸（覆盖 basis 解析后的值）
    pub main_size: Dimension,
    /// 交叉轴尺寸
    pub cross_size: Dimension,
    /// 最小主轴尺寸
    pub min_main: f32,
    /// 最大主轴尺寸
    pub max_main: f32,
    /// align-self 覆盖
    pub align_self: AlignSelf,
    /// 外边距
    pub margin: Margin,
}

impl FlexItem {
    /// 创建默认 Flex 子项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 flex-grow
    pub fn with_grow(mut self, grow: f32) -> Self {
        self.grow = grow.max(0.0);
        self
    }

    /// 设置 flex-shrink
    pub fn with_shrink(mut self, shrink: f32) -> Self {
        self.shrink = shrink.max(0.0);
        self
    }

    /// 设置 flex-basis
    pub fn with_basis(mut self, basis: Dimension) -> Self {
        self.basis = basis;
        self
    }

    /// 设置主轴尺寸
    pub fn with_main_size(mut self, size: Dimension) -> Self {
        self.main_size = size;
        self
    }

    /// 设置交叉轴尺寸
    pub fn with_cross_size(mut self, size: Dimension) -> Self {
        self.cross_size = size;
        self
    }

    /// 设置 align-self
    pub fn with_align_self(mut self, align: AlignSelf) -> Self {
        self.align_self = align;
        self
    }

    /// 设置外边距
    pub fn with_margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }
}

impl Default for FlexItem {
    fn default() -> Self {
        Self {
            grow: 0.0,
            shrink: 1.0,
            basis: Dimension::Auto,
            main_size: Dimension::Auto,
            cross_size: Dimension::Auto,
            min_main: 0.0,
            max_main: f32::INFINITY,
            align_self: AlignSelf::Auto,
            margin: Margin::zero(),
        }
    }
}

/// Flex 容器属性
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FlexContainer {
    /// 主轴方向
    pub direction: FlexDirection,
    /// 主轴对齐
    pub justify_content: JustifyContent,
    /// 交叉轴对齐
    pub align_items: AlignItems,
    /// 换行模式
    pub wrap: FlexWrap,
    /// 子项间距
    pub gap: f32,
    /// 内边距
    pub padding: Padding,
}

impl FlexContainer {
    /// 创建默认 Flex 容器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主轴方向
    pub fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// 设置主轴对齐
    pub fn with_justify(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    /// 设置交叉轴对齐
    pub fn with_align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    /// 设置换行模式
    pub fn with_wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// 设置子项间距
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// 设置内边距
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            wrap: FlexWrap::NoWrap,
            gap: 0.0,
            padding: Padding::zero(),
        }
    }
}

/// Flex 布局计算结果（单个子项）
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlexLayoutResult {
    /// 子项最终矩形（相对父容器原点）
    pub rect: Rect,
    /// 主轴尺寸
    pub main_size: f32,
    /// 交叉轴尺寸
    pub cross_size: f32,
}

/// Flex 布局引擎
///
/// 实现简化版 Flex 布局算法，支持：
/// - flex-grow / flex-shrink / flex-basis
/// - justify-content（FlexStart/FlexEnd/Center/SpaceBetween/SpaceAround/SpaceEvenly）
/// - align-items / align-self（FlexStart/FlexEnd/Center/Stretch）
/// - flex-wrap（NoWrap/Wrap）
/// - gap 间距
/// - padding 内边距
pub struct FlexLayoutEngine;

impl FlexLayoutEngine {
    /// 执行 Flex 布局计算
    ///
    /// `container_rect`：父容器矩形（绝对坐标）
    /// `container`：Flex 容器属性
    /// `items`：子项属性列表，元组为 (FlexItem, 内容主轴尺寸, 内容交叉轴尺寸)
    ///
    /// 返回每个子项的布局结果（与 items 顺序一致）
    pub fn layout(
        container_rect: Rect,
        container: &FlexContainer,
        items: &[(FlexItem, f32, f32)],
    ) -> Vec<FlexLayoutResult> {
        if items.is_empty() {
            return Vec::new();
        }

        let pad = container.padding;
        let inner_x = container_rect.x + pad.left;
        let inner_y = container_rect.y + pad.top;
        let inner_w = (container_rect.w - pad.left - pad.right).max(0.0);
        let inner_h = (container_rect.h - pad.top - pad.bottom).max(0.0);

        let is_row = container.direction.is_row();
        let (container_main, container_cross) = if is_row {
            (inner_w, inner_h)
        } else {
            (inner_h, inner_w)
        };

        // 1. 计算每个子项的 base main size（含 margin）
        let n = items.len();
        let mut base_main = vec![0.0f32; n];
        let mut hypothetical_main = vec![0.0f32; n];
        let mut margins_main = vec![0.0f32; n];
        let mut margins_cross = vec![0.0f32; n];

        for i in 0..n {
            let (item, content_main, _content_cross) = &items[i];
            margins_main[i] = if is_row {
                item.margin.left + item.margin.right
            } else {
                item.margin.top + item.margin.bottom
            };
            margins_cross[i] = if is_row {
                item.margin.top + item.margin.bottom
            } else {
                item.margin.left + item.margin.right
            };

            // flex-basis 优先于 main_size
            let basis_value = if !item.basis.is_auto() {
                item.basis.resolve(container_main, *content_main)
            } else if !item.main_size.is_auto() {
                item.main_size.resolve(container_main, *content_main)
            } else {
                *content_main
            };

            base_main[i] = basis_value;
            // hypothetical = base + margin，并 clamp 到 min/max
            let hypo = (base_main[i] + margins_main[i]).clamp(item.min_main, item.max_main);
            hypothetical_main[i] = (hypo - margins_main[i]).max(0.0);
        }

        // 2. 计算总主轴需求
        let total_gap = if n > 1 {
            container.gap * (n - 1) as f32
        } else {
            0.0
        };
        let total_main_used: f32 =
            hypothetical_main.iter().sum::<f32>() + margins_main.iter().sum::<f32>() + total_gap;

        // 3. 处理 grow/shrink
        let mut final_main = hypothetical_main.clone();
        let free_space = container_main - total_main_used;

        if free_space > 0.0 {
            // 分配剩余空间给 grow
            let total_grow: f32 = items.iter().map(|(it, _, _)| it.grow).sum();
            if total_grow > 0.0 {
                for i in 0..n {
                    let grow = items[i].0.grow;
                    if grow > 0.0 {
                        let add = free_space * (grow / total_grow);
                        final_main[i] = (final_main[i] + add).clamp(
                            items[i].0.min_main - margins_main[i],
                            items[i].0.max_main - margins_main[i],
                        );
                        final_main[i] = final_main[i].max(0.0);
                    }
                }
            }
        } else if free_space < 0.0 {
            // 收缩
            let total_shrink_weight: f32 = items
                .iter()
                .zip(hypothetical_main.iter())
                .map(|((it, _, _), &hm)| it.shrink * hm)
                .sum();
            if total_shrink_weight > 0.0 {
                let deficit = -free_space;
                for i in 0..n {
                    let shrink = items[i].0.shrink;
                    if shrink > 0.0 && hypothetical_main[i] > 0.0 {
                        let weight = shrink * hypothetical_main[i];
                        let sub = deficit * (weight / total_shrink_weight);
                        final_main[i] = (final_main[i] - sub).clamp(
                            items[i].0.min_main - margins_main[i],
                            items[i].0.max_main - margins_main[i],
                        );
                        final_main[i] = final_main[i].max(0.0);
                    }
                }
            }
        }

        // 4. 计算交叉轴尺寸
        let mut final_cross = vec![0.0f32; n];
        let mut max_line_cross = 0.0f32;

        for i in 0..n {
            let (item, _content_main, content_cross) = &items[i];
            let cross = if !item.cross_size.is_auto() {
                item.cross_size.resolve(container_cross, *content_cross)
            } else {
                *content_cross
            };
            final_cross[i] = cross;
            let total = cross + margins_cross[i];
            if total > max_line_cross {
                max_line_cross = total;
            }
        }

        // 5. 计算主轴起点偏移（justify-content）
        let used_main: f32 =
            final_main.iter().sum::<f32>() + margins_main.iter().sum::<f32>() + total_gap;
        let free = (container_main - used_main).max(0.0);
        let (main_start_offset, between_gap) = match container.justify_content {
            JustifyContent::FlexStart => (0.0, container.gap),
            JustifyContent::FlexEnd => (free, container.gap),
            JustifyContent::Center => (free * 0.5, container.gap),
            JustifyContent::SpaceBetween => {
                let extra = if n > 1 { free / (n - 1) as f32 } else { 0.0 };
                (0.0, container.gap + extra)
            }
            JustifyContent::SpaceAround => {
                let extra = if n > 0 { free / n as f32 } else { 0.0 };
                (extra * 0.5, container.gap + extra)
            }
            JustifyContent::SpaceEvenly => {
                let extra = if n > 0 { free / (n + 1) as f32 } else { 0.0 };
                (extra, container.gap + extra)
            }
        };

        // 6. 处理反向排列
        let reverse_main = matches!(
            container.direction,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        );

        // 7. 计算每个子项的最终矩形
        let mut results = Vec::with_capacity(n);
        let mut cursor = main_start_offset;

        for i in 0..n {
            let (item, _content_main, _content_cross) = &items[i];
            let main_size = final_main[i];
            let cross_size = final_cross[i];

            // 主轴位置（考虑 margin）
            let main_pos = cursor
                + if is_row {
                    item.margin.left
                } else {
                    item.margin.top
                };

            // 交叉轴对齐
            let align = match item.align_self {
                AlignSelf::Auto => container.align_items,
                other => match other {
                    AlignSelf::FlexStart => AlignItems::FlexStart,
                    AlignSelf::FlexEnd => AlignItems::FlexEnd,
                    AlignSelf::Center => AlignItems::Center,
                    AlignSelf::Stretch => AlignItems::Stretch,
                    AlignSelf::Baseline => AlignItems::Baseline,
                    AlignSelf::Auto => AlignItems::Stretch,
                },
            };

            let cross_with_margin = cross_size + margins_cross[i];
            // 对齐使用容器交叉轴尺寸（而非行内最大值），以支持 center/stretch/flex-end
            let cross_free = (container_cross - cross_with_margin).max(0.0);
            let (cross_pos, final_cross_size) = match align {
                AlignItems::FlexStart => (0.0, cross_size),
                AlignItems::FlexEnd => (cross_free, cross_size),
                AlignItems::Center => (cross_free * 0.5, cross_size),
                AlignItems::Stretch => (0.0, cross_size + cross_free),
                AlignItems::Baseline => (0.0, cross_size), // 简化：等同 FlexStart
            };

            // 加上交叉轴 margin
            let cross_pos_with_margin = cross_pos
                + if is_row {
                    item.margin.top
                } else {
                    item.margin.left
                };

            // 转换回 x/y
            let (x, y, w, h) = if is_row {
                (
                    inner_x + main_pos,
                    inner_y + cross_pos_with_margin,
                    main_size,
                    final_cross_size,
                )
            } else {
                (
                    inner_x + cross_pos_with_margin,
                    inner_y + main_pos,
                    final_cross_size,
                    main_size,
                )
            };

            // 处理反向：在容器内翻转主轴位置
            let (final_x, final_y) = if reverse_main {
                if is_row {
                    let right = inner_x + container_main - main_pos - main_size;
                    (right, y)
                } else {
                    let bottom = inner_y + container_main - main_pos - main_size;
                    (x, bottom)
                }
            } else {
                (x, y)
            };

            results.push(FlexLayoutResult {
                rect: Rect::new(final_x, final_y, w, h),
                main_size,
                cross_size: final_cross_size,
            });

            cursor += main_size + margins_main[i] + between_gap;
        }

        results
    }
}

/// Anchor 锚点定义（相对父容器的锚点比例）
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Anchor {
    /// 最小 X 锚点（0.0=左, 1.0=右）
    pub min_x: f32,
    /// 最大 X 锚点（0.0=左, 1.0=右）
    pub max_x: f32,
    /// 最小 Y 锚点（0.0=上, 1.0=下）
    pub min_y: f32,
    /// 最大 Y 锚点（0.0=上, 1.0=下）
    pub max_y: f32,
}

impl Anchor {
    /// 创建锚点
    pub const fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// 全部居中（0.5, 0.5）
    pub const fn center() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5)
    }

    /// 左上角（0, 0）
    pub const fn top_left() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// 右上角（1, 0）
    pub const fn top_right() -> Self {
        Self::new(1.0, 1.0, 0.0, 0.0)
    }

    /// 左下角（0, 1）
    pub const fn bottom_left() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// 右下角（1, 1）
    pub const fn bottom_right() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    /// 水平拉伸（0, 1, y, y）
    pub const fn stretch_horizontal(y: f32) -> Self {
        Self::new(0.0, 1.0, y, y)
    }

    /// 垂直拉伸（x, x, 0, 1）
    pub const fn stretch_vertical(x: f32) -> Self {
        Self::new(x, x, 0.0, 1.0)
    }

    /// 全拉伸（0, 1, 0, 1）
    pub const fn stretch() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::top_left()
    }
}

/// Anchor 偏移量（相对锚点位置的像素偏移）
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AnchorOffset {
    /// 左边距
    pub left: f32,
    /// 右边距
    pub right: f32,
    /// 上边距
    pub top: f32,
    /// 下边距
    pub bottom: f32,
}

impl AnchorOffset {
    /// 创建锚点偏移
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// 零偏移
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// 各边相同偏移
    pub const fn uniform(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
}

impl Default for AnchorOffset {
    fn default() -> Self {
        AnchorOffset::zero()
    }
}

/// Pivot 支点（0.0~1.0，相对自身矩形）
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pivot {
    /// X 支点（0.0=左, 0.5=中, 1.0=右）
    pub x: f32,
    /// Y 支点（0.0=上, 0.5=中, 1.0=下）
    pub y: f32,
}

impl Pivot {
    /// 创建支点
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// 左上
    pub const fn top_left() -> Self {
        Self::new(0.0, 0.0)
    }

    /// 居中
    pub const fn center() -> Self {
        Self::new(0.5, 0.5)
    }

    /// 右下
    pub const fn bottom_right() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl Default for Pivot {
    fn default() -> Self {
        Pivot::top_left()
    }
}

/// Anchor 布局引擎
///
/// 基于 Unity UGUI 风格的锚点布局：
/// - 锚点定义子项相对父容器的相对位置（0.0~1.0）
/// - 偏移量定义子项相对锚点的像素偏移
/// - 支持拉伸锚点（min != max）
pub struct AnchorLayoutEngine;

impl AnchorLayoutEngine {
    /// 计算 Anchor 布局后的矩形
    ///
    /// `parent_rect`：父容器矩形
    /// `anchor`：锚点定义
    /// `offset`：偏移量
    /// `pivot`：支点（相对自身）
    /// `custom_size`：自定义尺寸（None 表示由锚点拉伸决定）
    pub fn compute(
        parent_rect: Rect,
        anchor: Anchor,
        offset: AnchorOffset,
        pivot: Pivot,
        custom_size: Option<Vec2>,
    ) -> Rect {
        // 锚点对应的父容器中的位置
        let anchor_min_x = parent_rect.x + parent_rect.w * anchor.min_x;
        let anchor_max_x = parent_rect.x + parent_rect.w * anchor.max_x;
        let anchor_min_y = parent_rect.y + parent_rect.h * anchor.min_y;
        let anchor_max_y = parent_rect.y + parent_rect.h * anchor.max_y;

        // 加上偏移得到矩形边界
        let min_x = anchor_min_x + offset.left;
        let max_x = anchor_max_x - offset.right;
        let min_y = anchor_min_y + offset.top;
        let max_y = anchor_max_y - offset.bottom;

        let (x, y, w, h) = if let Some(size) = custom_size {
            // 非拉伸模式：使用自定义尺寸，按锚点和支点定位
            // 锚点中心 = (anchor_min + anchor_max) / 2
            let anchor_center_x = (anchor_min_x + anchor_max_x) * 0.5;
            let anchor_center_y = (anchor_min_y + anchor_max_y) * 0.5;
            // 偏移：left 正向、right 反向；top 正向、bottom 反向
            let offset_x = offset.left - offset.right;
            let offset_y = offset.top - offset.bottom;
            let x = anchor_center_x + offset_x - size.x * pivot.x;
            let y = anchor_center_y + offset_y - size.y * pivot.y;
            (x, y, size.x, size.y)
        } else {
            // 拉伸模式：由 min/max 决定
            (
                (min_x).min(max_x),
                (min_y).min(max_y),
                (max_x - min_x).abs(),
                (max_y - min_y).abs(),
            )
        };

        Rect::new(x, y, w.max(0.0), h.max(0.0))
    }

    /// 简化版：仅使用锚点中心和偏移定位（非拉伸）
    pub fn compute_simple(
        parent_rect: Rect,
        anchor_point: Vec2,
        offset: Vec2,
        pivot: Pivot,
        size: Vec2,
    ) -> Rect {
        let anchor_x = parent_rect.x + parent_rect.w * anchor_point.x;
        let anchor_y = parent_rect.y + parent_rect.h * anchor_point.y;
        let x = anchor_x + offset.x - size.x * pivot.x;
        let y = anchor_y + offset.y - size.y * pivot.y;
        Rect::new(x, y, size.x, size.y)
    }
}

/// 内边距
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Padding {
    /// 左边距
    pub left: f32,
    /// 右边距
    pub right: f32,
    /// 上边距
    pub top: f32,
    /// 下边距
    pub bottom: f32,
}

impl Padding {
    /// 创建新的内边距
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// 创建各边相同内边距
    pub fn uniform(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// 创建零内边距
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// 水平总内边距
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// 垂直总内边距
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::zero()
    }
}

/// 外边距
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Margin {
    /// 左边距
    pub left: f32,
    /// 右边距
    pub right: f32,
    /// 上边距
    pub top: f32,
    /// 下边距
    pub bottom: f32,
}

impl Margin {
    /// 创建新的外边距
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// 创建各边相同外边距
    pub fn uniform(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    /// 创建零外边距
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }

    /// 水平总外边距
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// 垂直总外边距
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::zero()
    }
}

/// 布局属性
pub struct LayoutProperties {
    /// 内边距
    pub padding: Padding,
    /// 外边距
    pub margin: Margin,
    /// 间距
    pub spacing: f32,
    /// 对齐方式
    pub align: Alignment,
    /// 拉伸方式
    pub stretch: Stretch,
}

impl LayoutProperties {
    /// 创建新的布局属性
    pub fn new() -> Self {
        Self {
            padding: Padding::zero(),
            margin: Margin::zero(),
            spacing: 0.0,
            align: Alignment::Center,
            stretch: Stretch::None,
        }
    }
}

impl Default for LayoutProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// 对齐方式
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Alignment {
    /// 起始对齐
    Start,
    /// 居中对齐
    Center,
    /// 末尾对齐
    End,
}

/// 拉伸方式
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Stretch {
    /// 无拉伸
    None,
    /// 水平拉伸
    Horizontal,
    /// 垂直拉伸
    Vertical,
    /// 双向拉伸
    Both,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_uniform() {
        let padding = Padding::uniform(10.0);
        assert_eq!(padding.left, 10.0);
        assert_eq!(padding.right, 10.0);
        assert_eq!(padding.top, 10.0);
        assert_eq!(padding.bottom, 10.0);
    }

    #[test]
    fn test_margin_new() {
        let margin = Margin::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(margin.left, 1.0);
        assert_eq!(margin.right, 2.0);
        assert_eq!(margin.top, 3.0);
        assert_eq!(margin.bottom, 4.0);
    }

    #[test]
    fn test_layout_properties_default() {
        let props = LayoutProperties::default();
        assert_eq!(props.spacing, 0.0);
        assert_eq!(props.align, Alignment::Center);
        assert_eq!(props.stretch, Stretch::None);
    }

    #[test]
    fn test_padding_zero() {
        let p = Padding::zero();
        assert_eq!(p.left, 0.0);
        assert_eq!(p.right, 0.0);
        assert_eq!(p.top, 0.0);
        assert_eq!(p.bottom, 0.0);
    }

    #[test]
    fn test_padding_new() {
        let p = Padding::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(p.left, 1.0);
        assert_eq!(p.right, 2.0);
        assert_eq!(p.top, 3.0);
        assert_eq!(p.bottom, 4.0);
    }

    #[test]
    fn test_margin_uniform() {
        let m = Margin::uniform(8.0);
        assert_eq!(m.left, 8.0);
        assert_eq!(m.right, 8.0);
        assert_eq!(m.top, 8.0);
        assert_eq!(m.bottom, 8.0);
    }

    #[test]
    fn test_margin_zero() {
        let m = Margin::zero();
        assert_eq!(m.left, 0.0);
        assert_eq!(m.right, 0.0);
        assert_eq!(m.top, 0.0);
        assert_eq!(m.bottom, 0.0);
    }

    #[test]
    fn test_layout_type_variants() {
        let _n = LayoutType::None;
        let _h = LayoutType::Horizontal;
        let _v = LayoutType::Vertical;
        let _f = LayoutType::Flex;
        let _a = LayoutType::Anchor;
    }

    #[test]
    fn test_layout_direction_variants() {
        let _h = LayoutDirection::Horizontal;
        let _v = LayoutDirection::Vertical;
    }

    #[test]
    fn test_alignment_variants() {
        let _s = Alignment::Start;
        let _c = Alignment::Center;
        let _e = Alignment::End;
    }

    #[test]
    fn test_stretch_variants() {
        let _n = Stretch::None;
        let _h = Stretch::Horizontal;
        let _v = Stretch::Vertical;
        let _b = Stretch::Both;
    }

    #[test]
    fn test_layout_properties_spacing_field() {
        let mut props = LayoutProperties::new();
        props.spacing = 12.0;
        assert_eq!(props.spacing, 12.0);
    }

    #[test]
    fn test_layout_properties_align_field() {
        let mut props = LayoutProperties::new();
        props.align = Alignment::Start;
        assert_eq!(props.align, Alignment::Start);
        props.align = Alignment::End;
        assert_eq!(props.align, Alignment::End);
    }

    #[test]
    fn test_layout_properties_stretch_field() {
        let mut props = LayoutProperties::new();
        props.stretch = Stretch::Both;
        assert_eq!(props.stretch, Stretch::Both);
    }

    #[test]
    fn test_layout_properties_padding_mut() {
        let mut props = LayoutProperties::new();
        props.padding = Padding::uniform(5.0);
        assert_eq!(props.padding.left, 5.0);
        assert_eq!(props.padding.top, 5.0);
    }

    #[test]
    fn test_layout_properties_margin_mut() {
        let mut props = LayoutProperties::new();
        props.margin = Margin::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(props.margin.left, 1.0);
        assert_eq!(props.margin.bottom, 4.0);
    }

    // ===== Flex 布局引擎测试 =====

    #[test]
    fn test_flex_direction_is_row_or_column() {
        assert!(FlexDirection::Row.is_row());
        assert!(FlexDirection::RowReverse.is_row());
        assert!(FlexDirection::Column.is_column());
        assert!(FlexDirection::ColumnReverse.is_column());
        assert!(!FlexDirection::Row.is_column());
        assert!(!FlexDirection::Column.is_row());
    }

    #[test]
    fn test_dimension_resolve() {
        assert_eq!(Dimension::px(100.0).resolve(200.0, 0.0), 100.0);
        assert_eq!(Dimension::percent(0.5).resolve(200.0, 0.0), 100.0);
        assert_eq!(Dimension::Auto.resolve(200.0, 50.0), 50.0);
        assert!(Dimension::px(-10.0).resolve(200.0, 0.0) >= 0.0);
    }

    #[test]
    fn test_dimension_helpers() {
        assert!(Dimension::Auto.is_auto());
        assert!(Dimension::px(10.0).is_pixels());
        assert!(!Dimension::Auto.is_pixels());
    }

    #[test]
    fn test_flex_item_default() {
        let item = FlexItem::new();
        assert_eq!(item.grow, 0.0);
        assert_eq!(item.shrink, 1.0);
        assert!(item.basis.is_auto());
    }

    #[test]
    fn test_flex_item_builder() {
        let item = FlexItem::new()
            .with_grow(2.0)
            .with_shrink(0.0)
            .with_basis(Dimension::px(100.0))
            .with_main_size(Dimension::percent(0.5))
            .with_align_self(AlignSelf::Center);
        assert_eq!(item.grow, 2.0);
        assert_eq!(item.shrink, 0.0);
        assert_eq!(item.basis, Dimension::px(100.0));
        assert_eq!(item.align_self, AlignSelf::Center);
    }

    #[test]
    fn test_flex_container_default() {
        let c = FlexContainer::new();
        assert_eq!(c.direction, FlexDirection::Row);
        assert_eq!(c.justify_content, JustifyContent::FlexStart);
        assert_eq!(c.align_items, AlignItems::Stretch);
        assert_eq!(c.wrap, FlexWrap::NoWrap);
        assert_eq!(c.gap, 0.0);
    }

    #[test]
    fn test_flex_layout_empty_items() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 600.0);
        let container = FlexContainer::new();
        let items: Vec<(FlexItem, f32, f32)> = vec![];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        assert!(results.is_empty());
    }

    #[test]
    fn test_flex_layout_row_flex_start() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new().with_direction(FlexDirection::Row);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(200.0)),
                200.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        assert_eq!(results.len(), 2);
        // 第一个子项应在 x=0
        assert_eq!(results[0].rect.x, 0.0);
        assert_eq!(results[0].rect.w, 100.0);
        // 第二个子项应在 x=100
        assert_eq!(results[1].rect.x, 100.0);
        assert_eq!(results[1].rect.w, 200.0);
    }

    #[test]
    fn test_flex_layout_row_with_grow() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new().with_direction(FlexDirection::Row);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new()
                    .with_grow(1.0)
                    .with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new()
                    .with_grow(1.0)
                    .with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 总固定 300，剩余 500，两个 grow=1 各分 250
        assert_eq!(results[0].rect.w, 100.0);
        assert!(
            (results[1].rect.w - 350.0).abs() < 0.1,
            "got {}",
            results[1].rect.w
        );
        assert!(
            (results[2].rect.w - 350.0).abs() < 0.1,
            "got {}",
            results[2].rect.w
        );
    }

    #[test]
    fn test_flex_layout_row_with_shrink() {
        let container_rect = Rect::new(0.0, 0.0, 200.0, 100.0);
        let container = FlexContainer::new().with_direction(FlexDirection::Row);
        let items = vec![
            (
                FlexItem::new()
                    .with_main_size(Dimension::px(150.0))
                    .with_shrink(1.0),
                150.0,
                50.0,
            ),
            (
                FlexItem::new()
                    .with_main_size(Dimension::px(150.0))
                    .with_shrink(1.0),
                150.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 总需求 300，容器 200，缺 100，各 shrink 50
        assert!(
            (results[0].rect.w - 100.0).abs() < 0.1,
            "got {}",
            results[0].rect.w
        );
        assert!(
            (results[1].rect.w - 100.0).abs() < 0.1,
            "got {}",
            results[1].rect.w
        );
    }

    #[test]
    fn test_flex_layout_justify_center() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Row)
            .with_justify(JustifyContent::Center);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 总用 200，剩余 600，居中起点 300
        assert!(
            (results[0].rect.x - 300.0).abs() < 0.1,
            "got {}",
            results[0].rect.x
        );
        assert!(
            (results[1].rect.x - 400.0).abs() < 0.1,
            "got {}",
            results[1].rect.x
        );
    }

    #[test]
    fn test_flex_layout_justify_space_between() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Row)
            .with_justify(JustifyContent::SpaceBetween);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 总用 300，剩余 500，2 个间隙各 250
        assert!((results[0].rect.x - 0.0).abs() < 0.1);
        assert!(
            (results[1].rect.x - 350.0).abs() < 0.1,
            "got {}",
            results[1].rect.x
        );
        assert!(
            (results[2].rect.x - 700.0).abs() < 0.1,
            "got {}",
            results[2].rect.x
        );
    }

    #[test]
    fn test_flex_layout_column_direction() {
        let container_rect = Rect::new(0.0, 0.0, 100.0, 800.0);
        let container = FlexContainer::new().with_direction(FlexDirection::Column);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(200.0)),
                200.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        assert_eq!(results[0].rect.y, 0.0);
        assert_eq!(results[0].rect.h, 100.0);
        assert_eq!(results[1].rect.y, 100.0);
        assert_eq!(results[1].rect.h, 200.0);
    }

    #[test]
    fn test_flex_layout_row_reverse() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new().with_direction(FlexDirection::RowReverse);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 反向：第一个子项应在最右侧
        assert!(
            (results[0].rect.x - 700.0).abs() < 0.1,
            "got {}",
            results[0].rect.x
        );
        assert!(
            (results[1].rect.x - 600.0).abs() < 0.1,
            "got {}",
            results[1].rect.x
        );
    }

    #[test]
    fn test_flex_layout_align_items_center() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Row)
            .with_align_items(AlignItems::Center);
        let items = vec![(
            FlexItem::new()
                .with_main_size(Dimension::px(100.0))
                .with_cross_size(Dimension::px(20.0)),
            100.0,
            20.0,
        )];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 容器高 100，子项高 20，居中 y=40
        assert!(
            (results[0].rect.y - 40.0).abs() < 0.1,
            "got {}",
            results[0].rect.y
        );
        assert_eq!(results[0].rect.h, 20.0);
    }

    #[test]
    fn test_flex_layout_align_items_stretch() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Row)
            .with_align_items(AlignItems::Stretch);
        let items = vec![(
            FlexItem::new()
                .with_main_size(Dimension::px(100.0))
                .with_cross_size(Dimension::Auto),
            100.0,
            20.0,
        )];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 拉伸：子项高应等于容器高
        assert!(
            (results[0].rect.h - 100.0).abs() < 0.1,
            "got {}",
            results[0].rect.h
        );
    }

    #[test]
    fn test_flex_layout_with_gap() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Row)
            .with_gap(10.0);
        let items = vec![
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        assert_eq!(results[0].rect.x, 0.0);
        assert!(
            (results[1].rect.x - 110.0).abs() < 0.1,
            "got {}",
            results[1].rect.x
        );
        assert!(
            (results[2].rect.x - 220.0).abs() < 0.1,
            "got {}",
            results[2].rect.x
        );
    }

    #[test]
    fn test_flex_layout_with_padding() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 600.0);
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Row)
            .with_padding(Padding::uniform(20.0));
        let items = vec![(
            FlexItem::new().with_main_size(Dimension::px(100.0)),
            100.0,
            50.0,
        )];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 内边距 20，子项起点 x=20, y=20
        assert_eq!(results[0].rect.x, 20.0);
        assert_eq!(results[0].rect.y, 20.0);
    }

    #[test]
    fn test_flex_layout_with_margin() {
        let container_rect = Rect::new(0.0, 0.0, 800.0, 100.0);
        let container = FlexContainer::new().with_direction(FlexDirection::Row);
        let items = vec![
            (
                FlexItem::new()
                    .with_main_size(Dimension::px(100.0))
                    .with_margin(Margin::new(10.0, 10.0, 0.0, 0.0)),
                100.0,
                50.0,
            ),
            (
                FlexItem::new().with_main_size(Dimension::px(100.0)),
                100.0,
                50.0,
            ),
        ];
        let results = FlexLayoutEngine::layout(container_rect, &container, &items);
        // 第一个子项有左 margin 10，所以 x=10
        assert_eq!(results[0].rect.x, 10.0);
        // 第二个子项应在第一个之后 + margin right 10
        assert!(
            (results[1].rect.x - 120.0).abs() < 0.1,
            "got {}",
            results[1].rect.x
        );
    }

    // ===== Anchor 布局测试 =====

    #[test]
    fn test_anchor_presets() {
        let c = Anchor::center();
        assert_eq!(c.min_x, 0.5);
        assert_eq!(c.max_x, 0.5);
        assert_eq!(c.min_y, 0.5);
        assert_eq!(c.max_y, 0.5);

        let tl = Anchor::top_left();
        assert_eq!(tl.min_x, 0.0);
        assert_eq!(tl.max_x, 0.0);

        let s = Anchor::stretch();
        assert_eq!(s.min_x, 0.0);
        assert_eq!(s.max_x, 1.0);
        assert_eq!(s.min_y, 0.0);
        assert_eq!(s.max_y, 1.0);
    }

    #[test]
    fn test_anchor_layout_stretch() {
        let parent = Rect::new(0.0, 0.0, 800.0, 600.0);
        let anchor = Anchor::stretch();
        let offset = AnchorOffset::uniform(10.0);
        let pivot = Pivot::top_left();
        let rect = AnchorLayoutEngine::compute(parent, anchor, offset, pivot, None);
        // 拉伸：x=10, y=10, w=780, h=580
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 10.0);
        assert!((rect.w - 780.0).abs() < 0.1);
        assert!((rect.h - 580.0).abs() < 0.1);
    }

    #[test]
    fn test_anchor_layout_center_with_size() {
        let parent = Rect::new(0.0, 0.0, 800.0, 600.0);
        let anchor = Anchor::center();
        let offset = AnchorOffset::zero();
        let pivot = Pivot::center();
        let size = Vec2::new(100.0, 50.0);
        let rect = AnchorLayoutEngine::compute(parent, anchor, offset, pivot, Some(size));
        // 居中：x=350, y=275
        assert!((rect.x - 350.0).abs() < 0.1, "got {}", rect.x);
        assert!((rect.y - 275.0).abs() < 0.1, "got {}", rect.y);
        assert_eq!(rect.w, 100.0);
        assert_eq!(rect.h, 50.0);
    }

    #[test]
    fn test_anchor_layout_top_left() {
        let parent = Rect::new(0.0, 0.0, 800.0, 600.0);
        let anchor = Anchor::top_left();
        let offset = AnchorOffset::new(10.0, 0.0, 20.0, 0.0);
        let pivot = Pivot::top_left();
        let size = Vec2::new(100.0, 50.0);
        let rect = AnchorLayoutEngine::compute(parent, anchor, offset, pivot, Some(size));
        // 锚点 (0,0)，偏移 (10, 20)
        assert!((rect.x - 10.0).abs() < 0.1, "got {}", rect.x);
        assert!((rect.y - 20.0).abs() < 0.1, "got {}", rect.y);
    }

    #[test]
    fn test_anchor_layout_bottom_right_with_pivot() {
        let parent = Rect::new(0.0, 0.0, 800.0, 600.0);
        let anchor = Anchor::bottom_right();
        let offset = AnchorOffset::new(0.0, 10.0, 0.0, 10.0);
        let pivot = Pivot::new(1.0, 1.0);
        let size = Vec2::new(100.0, 50.0);
        let rect = AnchorLayoutEngine::compute(parent, anchor, offset, pivot, Some(size));
        // 锚点 (800, 600)，偏移 right=10 → anchor_max_x - offset.right = 800 - 10 = 790
        // pivot (1,1) → x = 790 - 100*1 = 690, y = 590 - 50*1 = 540
        assert!((rect.x - 690.0).abs() < 0.1, "got {}", rect.x);
        assert!((rect.y - 540.0).abs() < 0.1, "got {}", rect.y);
    }

    #[test]
    fn test_anchor_layout_simple() {
        let parent = Rect::new(0.0, 0.0, 800.0, 600.0);
        let rect = AnchorLayoutEngine::compute_simple(
            parent,
            Vec2::new(0.5, 0.5),
            Vec2::new(0.0, 0.0),
            Pivot::center(),
            Vec2::new(100.0, 50.0),
        );
        assert!((rect.x - 350.0).abs() < 0.1);
        assert!((rect.y - 275.0).abs() < 0.1);
    }

    #[test]
    fn test_pivot_presets() {
        assert_eq!(Pivot::top_left(), Pivot::new(0.0, 0.0));
        assert_eq!(Pivot::center(), Pivot::new(0.5, 0.5));
        assert_eq!(Pivot::bottom_right(), Pivot::new(1.0, 1.0));
    }

    #[test]
    fn test_padding_horizontal_vertical() {
        let p = Padding::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(p.horizontal(), 30.0);
        assert_eq!(p.vertical(), 70.0);
    }

    #[test]
    fn test_margin_horizontal_vertical() {
        let m = Margin::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(m.horizontal(), 30.0);
        assert_eq!(m.vertical(), 70.0);
    }
}
