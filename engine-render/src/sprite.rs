//! Sprite 模块 - 精灵结构
//!
//! 提供 Sprite 类型，表示可绘制的 2D 精灵。

use super::{Color, DrawParams, TextureHandle};
use engine_math::Vec2;

/// 矩形区域
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Rect {
    /// X 坐标
    pub x: f32,
    /// Y 坐标
    pub y: f32,
    /// 宽度
    pub width: f32,
    /// 高度
    pub height: f32,
}

impl Rect {
    /// 创建新矩形
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// 从尺寸创建（以原点为中心）
    #[inline]
    pub const fn from_size(width: f32, height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    /// 获取左边界
    #[inline]
    pub fn left(&self) -> f32 {
        self.x
    }

    /// 获取右边界
    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// 获取上边界
    #[inline]
    pub fn top(&self) -> f32 {
        self.y
    }

    /// 获取下边界
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// 获取中心点
    #[inline]
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// 检查是否包含点
    #[inline]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }
}

/// 精灵
///
/// 表示一个可绘制的 2D 图像区域。
#[derive(Clone, Debug)]
pub struct Sprite {
    /// 纹理句柄
    texture: TextureHandle,
    /// 源区域（纹理坐标）
    source_rect: Option<Rect>,
    /// 颜色叠加
    color: Color,
    /// 水平翻转
    flip_x: bool,
    /// 垂直翻转
    flip_y: bool,
    /// 锚点（相对于源矩形大小，0-1 范围）
    anchor: Vec2,
    /// 大小（如果有覆盖）
    size_override: Option<Vec2>,
}

impl Sprite {
    // region: 构造方法

    /// 从纹理创建精灵
    pub fn new(texture: TextureHandle) -> Self {
        Self {
            texture,
            source_rect: None,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            anchor: Vec2::new(0.5, 0.5),
            size_override: None,
        }
    }

    /// 从纹理和区域创建精灵
    pub fn from_texture_rect(texture: TextureHandle, rect: Rect) -> Self {
        Self {
            texture,
            source_rect: Some(rect),
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            anchor: Vec2::new(0.5, 0.5),
            size_override: None,
        }
    }

    /// 从纹理创建（链式 API）
    pub fn from_texture(texture: TextureHandle) -> Self {
        Self::new(texture)
    }

    // endregion

    // region: 链式设置方法

    /// 设置源区域
    pub fn with_source_rect(mut self, rect: Rect) -> Self {
        self.source_rect = Some(rect);
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 设置水平翻转
    pub fn with_flip_x(mut self, flip: bool) -> Self {
        self.flip_x = flip;
        self
    }

    /// 设置垂直翻转
    pub fn with_flip_y(mut self, flip: bool) -> Self {
        self.flip_y = flip;
        self
    }

    /// 设置锚点
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = anchor;
        self
    }

    // endregion

    // region: Getter/Setter

    /// 获取纹理句柄
    #[inline]
    pub fn texture(&self) -> TextureHandle {
        self.texture.clone()
    }

    /// 获取源区域
    #[inline]
    pub fn source_rect(&self) -> Option<Rect> {
        self.source_rect
    }

    /// 设置源区域
    #[inline]
    pub fn set_source_rect(&mut self, rect: Option<Rect>) {
        self.source_rect = rect;
    }

    /// 获取颜色
    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    /// 设置颜色
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// 获取水平翻转
    #[inline]
    pub fn flip_x(&self) -> bool {
        self.flip_x
    }

    /// 设置水平翻转
    #[inline]
    pub fn set_flip_x(&mut self, flip: bool) {
        self.flip_x = flip;
    }

    /// 获取垂直翻转
    #[inline]
    pub fn flip_y(&self) -> bool {
        self.flip_y
    }

    /// 设置垂直翻转
    #[inline]
    pub fn set_flip_y(&mut self, flip: bool) {
        self.flip_y = flip;
    }

    /// 获取锚点
    #[inline]
    pub fn anchor(&self) -> Vec2 {
        self.anchor
    }

    /// 设置锚点
    #[inline]
    pub fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    /// 获取大小
    pub fn size(&self) -> Vec2 {
        self.size_override.unwrap_or_else(|| {
            self.source_rect
                .map(|r| Vec2::new(r.width, r.height))
                .unwrap_or(Vec2::ZERO)
        })
    }

    /// 设置大小覆盖
    pub fn set_size(&mut self, size: Vec2) {
        self.size_override = Some(size);
    }

    // endregion

    // region: 绘制方法

    /// 绘制精灵（简单版本）
    ///
    /// 默认在 (0, 0) 位置绘制
    pub fn draw(&self, _ctx: &super::RenderContext, _position: Vec2) {
        // Implementation depends on RenderContext
        // This is a placeholder that will be implemented in the GL backend
    }

    /// 绘制精灵（扩展版本）
    pub fn draw_ex(&self, _ctx: &super::RenderContext, _position: Vec2, _params: DrawParams) {
        // Implementation depends on RenderContext
    }

    // endregion
}

/// Sprite 构建器
pub struct SpriteBuilder {
    texture: Option<TextureHandle>,
    source_rect: Option<Rect>,
    color: Color,
    flip_x: bool,
    flip_y: bool,
    anchor: Vec2,
    size_override: Option<Vec2>,
}

impl SpriteBuilder {
    /// 创建新的 Sprite 构建器
    pub fn new() -> Self {
        Self {
            texture: None,
            source_rect: None,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            anchor: Vec2::new(0.5, 0.5),
            size_override: None,
        }
    }

    /// 设置纹理
    pub fn with_texture(mut self, texture: TextureHandle) -> Self {
        self.texture = Some(texture);
        self
    }

    /// 设置源区域
    pub fn with_source_rect(mut self, rect: Rect) -> Self {
        self.source_rect = Some(rect);
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 设置翻转
    pub fn with_flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }

    /// 设置锚点
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = anchor;
        self
    }

    /// 设置大小
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size_override = Some(size);
        self
    }

    /// 构建 Sprite
    pub fn build(self) -> Option<Sprite> {
        let texture = self.texture?;
        let mut sprite = Sprite::new(texture);
        sprite.source_rect = self.source_rect;
        sprite.color = self.color;
        sprite.flip_x = self.flip_x;
        sprite.flip_y = self.flip_y;
        sprite.anchor = self.anchor;
        sprite.size_override = self.size_override;
        Some(sprite)
    }
}

impl Default for SpriteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Texture2D;
    use engine_utils::Handle;

    #[test]
    fn test_sprite_new() {
        let handle = Handle::<Texture2D>::null();
        let sprite = Sprite::new(handle);
        assert!(sprite.source_rect().is_none());
        assert_eq!(sprite.color(), Color::WHITE);
        assert!(!sprite.flip_x());
        assert!(!sprite.flip_y());
    }

    #[test]
    fn test_sprite_with_chain() {
        let handle = Handle::<Texture2D>::null();
        let rect = Rect::new(0.0, 0.0, 32.0, 32.0);
        let sprite = Sprite::from_texture(handle)
            .with_source_rect(rect)
            .with_color(Color::RED)
            .with_flip_x(true)
            .with_anchor(Vec2::ZERO);

        assert_eq!(sprite.source_rect(), Some(rect));
        assert_eq!(sprite.color(), Color::RED);
        assert!(sprite.flip_x());
        assert_eq!(sprite.anchor(), Vec2::ZERO);
    }

    #[test]
    fn test_sprite_with_flip_y() {
        let handle = Handle::<Texture2D>::null();
        let sprite = Sprite::from_texture(handle).with_flip_y(true);
        assert!(sprite.flip_y());
    }

    #[test]
    fn test_sprite_from_texture_rect() {
        let handle = Handle::<Texture2D>::null();
        let rect = Rect::new(10.0, 20.0, 64.0, 64.0);
        let sprite = Sprite::from_texture_rect(handle, rect);
        assert_eq!(sprite.source_rect(), Some(rect));
    }

    #[test]
    fn test_sprite_set_source_rect() {
        let handle = Handle::<Texture2D>::null();
        let mut sprite = Sprite::from_texture(handle);
        let rect = Rect::new(0.0, 0.0, 32.0, 32.0);
        sprite.set_source_rect(Some(rect));
        assert_eq!(sprite.source_rect(), Some(rect));
        sprite.set_source_rect(None);
        assert!(sprite.source_rect().is_none());
    }

    #[test]
    fn test_sprite_set_color() {
        let handle = Handle::<Texture2D>::null();
        let mut sprite = Sprite::from_texture(handle);
        sprite.set_color(Color::BLUE);
        assert_eq!(sprite.color(), Color::BLUE);
    }

    #[test]
    fn test_sprite_set_flip_x() {
        let handle = Handle::<Texture2D>::null();
        let mut sprite = Sprite::from_texture(handle);
        sprite.set_flip_x(true);
        assert!(sprite.flip_x());
        sprite.set_flip_x(false);
        assert!(!sprite.flip_x());
    }

    #[test]
    fn test_sprite_set_flip_y() {
        let handle = Handle::<Texture2D>::null();
        let mut sprite = Sprite::from_texture(handle);
        sprite.set_flip_y(true);
        assert!(sprite.flip_y());
    }

    #[test]
    fn test_sprite_set_anchor() {
        let handle = Handle::<Texture2D>::null();
        let mut sprite = Sprite::from_texture(handle);
        sprite.set_anchor(Vec2::new(0.25, 0.75));
        assert_eq!(sprite.anchor(), Vec2::new(0.25, 0.75));
    }

    #[test]
    fn test_sprite_size_from_rect() {
        let handle = Handle::<Texture2D>::null();
        let rect = Rect::new(0.0, 0.0, 64.0, 32.0);
        let sprite = Sprite::from_texture_rect(handle, rect);
        let size = sprite.size();
        assert_eq!(size.x, 64.0);
        assert_eq!(size.y, 32.0);
    }

    #[test]
    fn test_sprite_default_anchor_0_5() {
        let handle = Handle::<Texture2D>::null();
        let sprite = Sprite::from_texture(handle);
        assert_eq!(sprite.anchor(), Vec2::new(0.5, 0.5));
    }

    #[test]
    fn test_sprite_builder() {
        let handle = Handle::<Texture2D>::null();
        let rect = Rect::new(0.0, 0.0, 64.0, 64.0);
        let sprite = SpriteBuilder::new()
            .with_texture(handle)
            .with_source_rect(rect)
            .with_color(Color::BLUE)
            .build();

        assert!(sprite.is_some());
        let sprite = sprite.unwrap();
        assert_eq!(sprite.source_rect(), Some(rect));
        assert_eq!(sprite.color(), Color::BLUE);
    }

    #[test]
    fn test_sprite_builder_missing_texture() {
        let sprite = SpriteBuilder::new().with_color(Color::RED).build();
        assert!(sprite.is_none());
    }

    #[test]
    fn test_sprite_builder_with_flip() {
        let handle = Handle::<Texture2D>::null();
        let sprite = SpriteBuilder::new()
            .with_texture(handle)
            .with_flip(true, false)
            .build()
            .unwrap();
        assert!(sprite.flip_x());
        assert!(!sprite.flip_y());
    }

    #[test]
    fn test_sprite_builder_with_anchor() {
        let handle = Handle::<Texture2D>::null();
        let sprite = SpriteBuilder::new()
            .with_texture(handle)
            .with_anchor(Vec2::new(0.25, 0.75))
            .build()
            .unwrap();
        assert_eq!(sprite.anchor(), Vec2::new(0.25, 0.75));
    }

    #[test]
    fn test_sprite_builder_with_size() {
        let handle = Handle::<Texture2D>::null();
        let sprite = SpriteBuilder::new()
            .with_texture(handle)
            .with_size(Vec2::new(256.0, 128.0))
            .build()
            .unwrap();
        let size = sprite.size();
        assert_eq!(size.x, 256.0);
        assert_eq!(size.y, 128.0);
    }

    #[test]
    fn test_rect() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.bottom(), 70.0);
        assert_eq!(rect.center(), Vec2::new(60.0, 45.0));
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(rect.contains(Vec2::new(50.0, 50.0)));
        assert!(!rect.contains(Vec2::new(150.0, 50.0)));
        assert!(!rect.contains(Vec2::new(50.0, 150.0)));
        assert!(!rect.contains(Vec2::new(-10.0, -10.0)));
    }

    #[test]
    fn test_rect_contains_edges() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        // Origin point (top-left corner)
        assert!(rect.contains(Vec2::new(0.0, 0.0)));
        // Bottom-right corner
        assert!(rect.contains(Vec2::new(100.0, 100.0)));
    }

    #[test]
    fn test_rect_from_size() {
        let rect = Rect::from_size(50.0, 100.0);
        assert_eq!(rect.width, 50.0);
        assert_eq!(rect.height, 100.0);
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
    }

    #[test]
    fn test_rect_default() {
        let rect: Rect = Default::default();
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
    }

    #[test]
    fn test_rect_center_of_unit_rect() {
        let rect = Rect::new(0.0, 0.0, 2.0, 2.0);
        assert_eq!(rect.center(), Vec2::new(1.0, 1.0));
    }
}
