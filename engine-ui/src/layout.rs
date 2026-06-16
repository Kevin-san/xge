//! 布局模块
//!
//! 定义 UI 布局类型和属性。

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutType {
    None,
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn uniform(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

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

pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Margin {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn uniform(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

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

pub struct LayoutProperties {
    pub padding: Padding,
    pub margin: Margin,
    pub spacing: f32,
    pub align: Alignment,
    pub stretch: Stretch,
}

impl LayoutProperties {
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Stretch {
    None,
    Horizontal,
    Vertical,
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
}
