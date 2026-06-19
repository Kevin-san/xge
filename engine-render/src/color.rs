//! Color 模块 - RGBA 颜色表示
//!
//! 提供 f32 RGBA 颜色类型，支持 24+ 常用颜色常量、hex 解析、线性插值等操作。

use core::fmt;
use engine_math::Vec4;

/// RGBA 颜色，四个分量均为 f32 范围 [0.0, 1.0]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Color {
    /// 红色分量
    pub r: f32,
    /// 绿色分量
    pub g: f32,
    /// 蓝色分量
    pub b: f32,
    /// 阿尔法分量
    pub a: f32,
}

impl Color {
    // region: 构造方法

    /// 创建新颜色
    ///
    /// # Example
    /// ```
    /// use engine_render::Color;
    /// let c = Color::new(1.0, 0.0, 0.0, 1.0); // 红色
    /// ```
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// 从 RGB 创建颜色（alpha 默认为 1.0）
    #[inline]
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// 从 RGBA 创建颜色
    #[inline]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// 从 u8 值创建颜色（每个分量除以 255.0）
    #[inline]
    pub const fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// 从 hex 字符串解析颜色
    ///
    /// 支持格式: #RGB, #RGBA, #RRGGBB, #RRGGBBAA
    ///
    /// # Example
    /// ```
    /// use engine_render::Color;
    /// let c = Color::from_hex("#FF0000").unwrap(); // 红色
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self, ColorParseError> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            3 => {
                // RGB
                let r =
                    u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| ColorParseError)?;
                let g =
                    u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| ColorParseError)?;
                let b =
                    u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| ColorParseError)?;
                Ok(Self::from_u8(r, g, b, 255))
            }
            4 => {
                // RGBA
                let r =
                    u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| ColorParseError)?;
                let g =
                    u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| ColorParseError)?;
                let b =
                    u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| ColorParseError)?;
                let a =
                    u8::from_str_radix(&hex[3..4].repeat(2), 16).map_err(|_| ColorParseError)?;
                Ok(Self::from_u8(r, g, b, a))
            }
            6 => {
                // RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorParseError)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorParseError)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorParseError)?;
                Ok(Self::from_u8(r, g, b, 255))
            }
            8 => {
                // RRGGBBAA
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorParseError)?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorParseError)?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorParseError)?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| ColorParseError)?;
                Ok(Self::from_u8(r, g, b, a))
            }
            _ => Err(ColorParseError),
        }
    }

    // endregion

    // region: 转换方法

    /// 转换为 hex 字符串（#RRGGBBAA 格式）
    #[inline]
    pub fn to_hex(&self) -> String {
        let r = (self.r.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (self.g.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (self.b.clamp(0.0, 1.0) * 255.0) as u8;
        let a = (self.a.clamp(0.0, 1.0) * 255.0) as u8;
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }

    /// 转换为 Vec4
    #[inline]
    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// 转换为 [f32; 4] 数组
    #[inline]
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    // endregion

    // region: 颜色运算

    /// 线性插值
    ///
    /// # Example
    /// ```
    /// use engine_render::Color;
    /// let red = Color::RED;
    /// let blue = Color::BLUE;
    /// let purple = Color::lerp(red, blue, 0.5);
    /// ```
    #[inline]
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color::new(
            a.r + (b.r - a.r) * t,
            a.g + (b.g - a.g) * t,
            a.b + (b.b - a.b) * t,
            a.a + (b.a - a.a) * t,
        )
    }

    /// 颜色乘法（分量相乘）
    #[inline]
    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
            self.a * other.a,
        )
    }

    /// 颜色乘法（标量）
    #[inline]
    pub fn mul_scalar(self, scalar: f32) -> Self {
        Self::new(
            self.r * scalar,
            self.g * scalar,
            self.b * scalar,
            self.a * scalar,
        )
    }

    // endregion

    // region: 常用颜色常量

    /// 红色
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// 绿色
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    /// 蓝色
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    /// 白色
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    /// 黑色
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// 完全透明
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    /// 黄色
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    /// 青色
    pub const CYAN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    /// 洋红色
    pub const MAGENTA: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    /// 橙色
    pub const ORANGE: Self = Self {
        r: 1.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    /// 灰色
    pub const GRAY: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    /// 浅灰色
    pub const LIGHTGRAY: Self = Self {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };
    /// 深灰色
    pub const DARKGRAY: Self = Self {
        r: 0.25,
        g: 0.25,
        b: 0.25,
        a: 1.0,
    };
    /// 金色
    pub const GOLD: Self = Self {
        r: 1.0,
        g: 0.84,
        b: 0.0,
        a: 1.0,
    };
    /// 酸橙色
    pub const LIME: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    /// 粉色
    pub const PINK: Self = Self {
        r: 1.0,
        g: 0.75,
        b: 0.8,
        a: 1.0,
    };
    /// 紫色
    pub const PURPLE: Self = Self {
        r: 0.5,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
    /// 蓝绿色
    pub const TEAL: Self = Self {
        r: 0.0,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    /// 栗色
    pub const MAROON: Self = Self {
        r: 0.5,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// 海军蓝
    pub const NAVY: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
    /// 橄榄绿
    pub const OLIVE: Self = Self {
        r: 0.5,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    /// 棕色
    pub const BROWN: Self = Self {
        r: 0.6,
        g: 0.3,
        b: 0.0,
        a: 1.0,
    };
    /// 青色
    pub const AQUA: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    /// 珊瑚色
    pub const CORAL: Self = Self {
        r: 1.0,
        g: 0.5,
        b: 0.31,
        a: 1.0,
    };
    /// 番茄红
    pub const TOMATO: Self = Self {
        r: 1.0,
        g: 0.39,
        b: 0.28,
        a: 1.0,
    };
    /// 草绿色
    pub const SPRINGGREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.5,
        a: 1.0,
    };

    // endregion
}

/// 颜色解析错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorParseError;

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse color from hex string")
    }
}

impl core::error::Error for ColorParseError {}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

// region: From implementations

impl From<Vec4> for Color {
    fn from(v: Vec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }
}

impl From<Color> for Vec4 {
    fn from(c: Color) -> Self {
        Vec4::new(c.r, c.g, c.b, c.a)
    }
}

// endregion

// region: Operator overloads

impl core::ops::Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.mul(other)
    }
}

impl core::ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        self.mul_scalar(scalar)
    }
}

impl core::ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, color: Color) -> Color {
        color.mul_scalar(self)
    }
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let c = Color::new(1.0, 0.5, 0.25, 1.0);
        assert!((c.r - 1.0).abs() < 0.001);
        assert!((c.g - 0.5).abs() < 0.001);
        assert!((c.b - 0.25).abs() < 0.001);
        assert!((c.a - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_color_from_rgb() {
        let c = Color::from_rgb(0.5, 0.5, 0.5);
        assert!((c.r - 0.5).abs() < 0.001);
        assert!((c.g - 0.5).abs() < 0.001);
        assert!((c.b - 0.5).abs() < 0.001);
        assert!((c.a - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_color_from_rgba() {
        let c = Color::from_rgba(0.1, 0.2, 0.3, 0.5);
        assert!((c.r - 0.1).abs() < 0.001);
        assert!((c.g - 0.2).abs() < 0.001);
        assert!((c.b - 0.3).abs() < 0.001);
        assert!((c.a - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_from_hex_rgb() {
        let c = Color::from_hex("#FF0000").unwrap();
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_hex_green() {
        let c = Color::from_hex("#00FF00").unwrap();
        assert!((c.r - 0.0).abs() < 0.01);
        assert!((c.g - 1.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_hex_blue() {
        let c = Color::from_hex("#0000FF").unwrap();
        assert!((c.r - 0.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
        assert!((c.b - 1.0).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_hex_rgba() {
        let c = Color::from_hex("#FF000080").unwrap();
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.a - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_from_hex_6_digit() {
        let c = Color::from_hex("00FF00").unwrap();
        assert!((c.r - 0.0).abs() < 0.01);
        assert!((c.g - 1.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_from_hex_short_format() {
        let c = Color::from_hex("#F00").unwrap();
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_hex_short_with_alpha() {
        let c = Color::from_hex("#F008").unwrap();
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
        // alpha should be 0x88 / 255.0
        assert!(c.a > 0.4 && c.a < 0.6);
    }

    #[test]
    fn test_from_hex_invalid_length() {
        // 5 characters not supported
        assert!(Color::from_hex("#12345").is_err());
        assert!(Color::from_hex("#12").is_err());
        assert!(Color::from_hex("").is_err());
    }

    #[test]
    fn test_to_hex() {
        let c = Color::from_hex("#FF0000").unwrap();
        assert_eq!(c.to_hex(), "#FF0000FF");
    }

    #[test]
    fn test_to_hex_2() {
        let c = Color::from_hex("#00FF0080").unwrap();
        let hex = c.to_hex();
        // Should start with #
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 9);
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = Color::from_hex("#FF8040C0").unwrap();
        let hex = original.to_hex();
        let parsed = Color::from_hex(&hex).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_hex_roundtrip_black() {
        let original = Color::from_hex("#00000000").unwrap();
        let hex = original.to_hex();
        let parsed = Color::from_hex(&hex).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_hex_roundtrip_white() {
        let original = Color::from_hex("#FFFFFFFF").unwrap();
        let hex = original.to_hex();
        let parsed = Color::from_hex(&hex).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_lerp_identity_red_to_blue() {
        let red = Color::RED;
        let blue = Color::BLUE;
        let r = Color::lerp(red, blue, 0.0);
        assert!((r.r - 1.0).abs() < 0.01);
        assert!((r.g - 0.0).abs() < 0.01);
        assert!((r.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_lerp_half_red_to_blue() {
        let red = Color::RED;
        let blue = Color::BLUE;
        let purple = Color::lerp(red, blue, 0.5);
        assert!((purple.r - 0.5).abs() < 0.01);
        assert!((purple.g - 0.0).abs() < 0.01);
        assert!((purple.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_lerp_full_red_to_blue() {
        let red = Color::RED;
        let blue = Color::BLUE;
        let b = Color::lerp(red, blue, 1.0);
        assert!((b.r - 0.0).abs() < 0.01);
        assert!((b.b - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_lerp_white_black_to_gray() {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let gray = Color::lerp(white, black, 0.5);
        assert!((gray.r - 0.5).abs() < 0.01);
        assert!((gray.g - 0.5).abs() < 0.01);
        assert!((gray.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_color_constants_red() {
        assert_eq!(Color::RED, Color::from_hex("#FF0000").unwrap());
    }

    #[test]
    fn test_color_constants_green() {
        assert_eq!(Color::GREEN, Color::from_hex("#00FF00").unwrap());
    }

    #[test]
    fn test_color_constants_blue() {
        assert_eq!(Color::BLUE, Color::from_hex("#0000FF").unwrap());
    }

    #[test]
    fn test_color_constants_white() {
        assert_eq!(Color::WHITE, Color::from_hex("#FFFFFF").unwrap());
    }

    #[test]
    fn test_color_constants_black() {
        assert_eq!(Color::BLACK, Color::from_hex("#000000").unwrap());
    }

    #[test]
    fn test_color_constants_transparent() {
        assert_eq!(Color::TRANSPARENT, Color::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_color_constants_yellow() {
        assert_eq!(Color::YELLOW, Color::new(1.0, 1.0, 0.0, 1.0));
    }

    #[test]
    fn test_color_constants_cyan() {
        assert_eq!(Color::CYAN, Color::new(0.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_from_u8_min() {
        let c = Color::from_u8(0, 0, 0, 0);
        assert!((c.r - 0.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
        assert!((c.a - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_from_u8_max() {
        let c = Color::from_u8(255, 255, 255, 255);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 1.0).abs() < 0.01);
        assert!((c.b - 1.0).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_u8_mixed() {
        let c = Color::from_u8(128, 64, 32, 255);
        assert!(c.r > 0.45 && c.r < 0.55);
        assert!(c.g > 0.2 && c.g < 0.3);
        assert!(c.b > 0.1 && c.b < 0.15);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_mul() {
        let c = Color::new(0.5, 0.5, 0.5, 1.0);
        let result = c * c;
        assert!((result.r - 0.25).abs() < 0.01);
        assert!((result.g - 0.25).abs() < 0.01);
        assert!((result.b - 0.25).abs() < 0.01);
        assert!((result.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_mul_via_method() {
        let c = Color::new(0.5, 0.5, 0.5, 1.0);
        let result = c.mul(c);
        assert!((result.r - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_mul_scalar_via_operator() {
        let c = Color::new(0.5, 0.5, 0.5, 1.0);
        let result = c * 2.0;
        assert!((result.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_mul_scalar_reverse() {
        let c = Color::new(0.5, 0.5, 0.5, 1.0);
        let result = 2.0 * c;
        assert!((result.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_from_vec4() {
        let v = Vec4::new(1.0, 0.5, 0.5, 1.0);
        let c = Color::from(v);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.5).abs() < 0.01);
        assert!((c.b - 0.5).abs() < 0.01);
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_to_vec4() {
        let c = Color::RED;
        let v: Vec4 = c.into();
        assert!((v.x - 1.0).abs() < 0.01);
        assert!((v.y - 0.0).abs() < 0.01);
        assert!((v.z - 0.0).abs() < 0.01);
        assert!((v.w - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_to_vec4_via_method() {
        let c = Color::GREEN;
        let v = c.to_vec4();
        assert!((v.x - 0.0).abs() < 0.01);
        assert!((v.y - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_to_array() {
        let c = Color::new(0.1, 0.2, 0.3, 0.4);
        let arr = c.to_array();
        assert!((arr[0] - 0.1).abs() < 0.001);
        assert!((arr[1] - 0.2).abs() < 0.001);
        assert!((arr[2] - 0.3).abs() < 0.001);
        assert!((arr[3] - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_color_parse_error_display() {
        let err = ColorParseError;
        let s = format!("{}", err);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_color_display() {
        let c = Color::new(1.0, 0.5, 0.25, 1.0);
        let s = format!("{}", c);
        assert!(s.contains("Color"));
    }

    #[test]
    fn test_color_derive_copy() {
        // Ensure Copy works
        let a = Color::RED;
        let b = a;
        let c = a;
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn test_color_magenta() {
        assert_eq!(Color::MAGENTA, Color::new(1.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn test_color_orange() {
        assert_eq!(Color::ORANGE, Color::new(1.0, 0.5, 0.0, 1.0));
    }

    #[test]
    fn test_color_gray() {
        assert_eq!(Color::GRAY, Color::new(0.5, 0.5, 0.5, 1.0));
    }

    #[test]
    fn test_color_light_gray() {
        assert_eq!(Color::LIGHTGRAY, Color::new(0.75, 0.75, 0.75, 1.0));
    }

    #[test]
    fn test_color_dark_gray() {
        assert_eq!(Color::DARKGRAY, Color::new(0.25, 0.25, 0.25, 1.0));
    }
}
