//! 布局模块
//!
//! 定义 UI 布局类型和属性。

/// 布局类型
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutType {
    /// 无布局
    None,
    /// 水平布局
    Horizontal,
    /// 垂直布局
    Vertical,
}

/// 布局方向
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutDirection {
    /// 水平方向
    Horizontal,
    /// 垂直方向
    Vertical,
}

/// 内边距
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
}

impl Default for Padding {
    fn default() -> Self {
        Self::zero()
    }
}

/// 外边距
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
}
