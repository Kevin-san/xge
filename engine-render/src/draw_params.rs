//! DrawParams 模块 - 绘制参数
//!
//! 提供 DrawParams 绘制参数和 BlendMode 混合模式。

use super::Color;

/// 混合模式枚举
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BlendMode {
    /// Alpha 混合（正常混合）
    Alpha,
    /// 加法混合（发光效果）
    Additive,
    /// 减法混合
    Subtract,
    /// 乘法混合（正片叠底）
    Multiply,
    /// 替换模式
    Replace,
    /// 反转模式
    Invert,
    /// 预乘 Alpha 混合
    PreMultiplied,
    #[default]
    None,
}

impl BlendMode {
    /// 转换为 OpenGL 混合枚举值
    ///
    /// 返回 (src_factor, dst_factor) 对
    #[inline]
    pub fn to_gl_enum(&self) -> (u32, u32) {
        match self {
            BlendMode::Alpha => (glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA),
            BlendMode::Additive => (glow::SRC_ALPHA, glow::ONE),
            BlendMode::Subtract => (glow::ONE_MINUS_SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA),
            BlendMode::Multiply => (glow::DST_COLOR, glow::ONE_MINUS_SRC_ALPHA),
            BlendMode::Replace => (glow::ONE, glow::ZERO),
            BlendMode::Invert => (glow::ONE_MINUS_DST_COLOR, glow::ZERO),
            BlendMode::PreMultiplied => (glow::ONE, glow::ONE_MINUS_SRC_ALPHA),
            BlendMode::None => (glow::ONE, glow::ZERO),
        }
    }

    /// 获取 GL blend equation
    #[inline]
    pub fn to_gl_equation(&self) -> u32 {
        match self {
            BlendMode::Subtract => glow::FUNC_REVERSE_SUBTRACT,
            _ => glow::FUNC_ADD,
        }
    }
}

/// 绘制参数
///
/// 用于指定绘制时的额外参数，如颜色叠加、混合模式、z-order 等。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawParams {
    /// 颜色叠加（会在纹理颜色上乘以此颜色）
    pub color: Color,
    /// 混合模式
    pub blend_mode: BlendMode,
    /// Z 顺序（数值越大越靠前）
    pub z_order: f32,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            blend_mode: BlendMode::Alpha,
            z_order: 0.0,
        }
    }
}

impl DrawParams {
    /// 创建新的绘制参数
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置颜色
    #[inline]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 设置混合模式
    #[inline]
    pub fn with_blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.blend_mode = blend_mode;
        self
    }

    /// 设置 Z 顺序
    #[inline]
    pub fn with_z_order(mut self, z_order: f32) -> Self {
        self.z_order = z_order;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_mode_gl_enum_alpha() {
        let (src, dst) = BlendMode::Alpha.to_gl_enum();
        assert_eq!(src, glow::SRC_ALPHA);
        assert_eq!(dst, glow::ONE_MINUS_SRC_ALPHA);
    }

    #[test]
    fn test_blend_mode_gl_enum_additive() {
        let (src, dst) = BlendMode::Additive.to_gl_enum();
        assert_eq!(src, glow::SRC_ALPHA);
        assert_eq!(dst, glow::ONE);
    }

    #[test]
    fn test_blend_mode_gl_enum_replace() {
        let (src, dst) = BlendMode::Replace.to_gl_enum();
        assert_eq!(src, glow::ONE);
        assert_eq!(dst, glow::ZERO);
    }

    #[test]
    fn test_blend_mode_gl_enum_multiply() {
        let (src, dst) = BlendMode::Multiply.to_gl_enum();
        assert_eq!(src, glow::DST_COLOR);
        assert_eq!(dst, glow::ONE_MINUS_SRC_ALPHA);
    }

    #[test]
    fn test_blend_mode_gl_enum_invert() {
        let (src, dst) = BlendMode::Invert.to_gl_enum();
        assert_eq!(src, glow::ONE_MINUS_DST_COLOR);
        assert_eq!(dst, glow::ZERO);
    }

    #[test]
    fn test_blend_mode_gl_enum_premultiplied() {
        let (src, dst) = BlendMode::PreMultiplied.to_gl_enum();
        assert_eq!(src, glow::ONE);
        assert_eq!(dst, glow::ONE_MINUS_SRC_ALPHA);
    }

    #[test]
    fn test_blend_mode_gl_enum_none() {
        let (src, dst) = BlendMode::None.to_gl_enum();
        assert_eq!(src, glow::ONE);
        assert_eq!(dst, glow::ZERO);
    }

    #[test]
    fn test_blend_mode_default_is_none() {
        let mode: BlendMode = Default::default();
        assert_eq!(mode, BlendMode::None);
    }

    #[test]
    fn test_draw_params_default() {
        let params = DrawParams::default();
        assert_eq!(params.color, Color::WHITE);
        assert_eq!(params.blend_mode, BlendMode::Alpha);
        assert_eq!(params.z_order, 0.0);
    }

    #[test]
    fn test_draw_params_new() {
        let params = DrawParams::new();
        assert_eq!(params.color, Color::WHITE);
        assert_eq!(params.blend_mode, BlendMode::Alpha);
        assert_eq!(params.z_order, 0.0);
    }

    #[test]
    fn test_draw_params_builder() {
        let params = DrawParams::new()
            .with_color(Color::RED)
            .with_blend_mode(BlendMode::Additive)
            .with_z_order(1.0);
        assert_eq!(params.color, Color::RED);
        assert_eq!(params.blend_mode, BlendMode::Additive);
        assert_eq!(params.z_order, 1.0);
    }

    #[test]
    fn test_draw_params_builder_chain_multiple() {
        let params = DrawParams::new()
            .with_color(Color::BLUE)
            .with_blend_mode(BlendMode::Multiply)
            .with_z_order(2.5)
            .with_z_order(5.0);
        assert_eq!(params.color, Color::BLUE);
        assert_eq!(params.blend_mode, BlendMode::Multiply);
        assert_eq!(params.z_order, 5.0);
    }

    #[test]
    fn test_draw_params_z_order_negative() {
        let params = DrawParams::new().with_z_order(-1.0);
        assert_eq!(params.z_order, -1.0);
    }
}
