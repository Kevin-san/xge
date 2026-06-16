//! Sprite2D 模块
//!
//! 提供 2D 精灵节点实现。

use engine_math::Vec2;
use super::{Node, Node2D, NodeHandle};

/// 精灵数据
#[derive(Debug, Clone)]
pub struct Sprite {
    /// 纹理 ID
    pub texture_id: u32,
    /// 纹理区域
    pub region: SpriteRegion,
    /// 翻转
    pub flip_x: bool,
    /// 翻转
    pub flip_y: bool,
    /// 调制颜色
    pub modulate: Vec2,
}

/// 精灵纹理区域
#[derive(Debug, Clone, Default)]
pub struct SpriteRegion {
    /// 左上角 X
    pub x: f32,
    /// 左上角 Y
    pub y: f32,
    /// 宽度
    pub width: f32,
    /// 高度
    pub height: f32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture_id: 0,
            region: SpriteRegion::default(),
            flip_x: false,
            flip_y: false,
            modulate: Vec2::new(1.0, 1.0),
        }
    }
}

impl Sprite {
    /// 创建新的精灵
    pub fn new(texture_id: u32) -> Self {
        Self {
            texture_id,
            ..Default::default()
        }
    }

    /// 创建带纹理的精灵
    pub fn with_region(texture_id: u32, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            texture_id,
            region: SpriteRegion { x, y, width, height },
            ..Default::default()
        }
    }
}

/// 2D 精灵节点
#[derive(Debug, Clone)]
pub struct Sprite2D {
    /// 基础 2D 节点
    node2d: Node2D,
    /// 精灵数据
    sprite: Sprite,
}

impl Sprite2D {
    /// 创建新的精灵节点
    pub fn new(name: impl Into<String>, sprite: Sprite) -> Self {
        Self {
            node2d: Node2D::new(name),
            sprite,
        }
    }

    /// 获取精灵引用
    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    /// 获取可变精灵引用
    pub fn sprite_mut(&mut self) -> &mut Sprite {
        &mut self.sprite
    }

    /// 设置精灵
    pub fn set_sprite(&mut self, sprite: Sprite) {
        self.sprite = sprite;
    }

    /// 获取基础 2D 节点引用
    pub fn node2d(&self) -> &Node2D {
        &self.node2d
    }

    /// 获取可变基础 2D 节点引用
    pub fn node2d_mut(&mut self) -> &mut Node2D {
        &mut self.node2d
    }
}

impl Node for Sprite2D {
    fn name(&self) -> &str {
        self.node2d.name()
    }

    fn parent(&self) -> Option<NodeHandle> {
        self.node2d.parent()
    }

    fn children(&self) -> &[NodeHandle] {
        self.node2d.children()
    }

    fn paused(&self) -> bool {
        self.node2d.paused()
    }

    fn visible(&self) -> bool {
        self.node2d.visible()
    }

    fn on_ready(&mut self) {
        self.node2d.on_ready();
    }

    fn on_update(&mut self, dt: f32) {
        self.node2d.on_update(dt);
    }

    fn on_draw(&self) {
        self.node2d.on_draw();
    }

    fn on_destroy(&mut self) {
        self.node2d.on_destroy();
    }

    fn add_child(&mut self, child: NodeHandle) {
        self.node2d.add_child(child);
    }

    fn remove_child(&mut self, child: NodeHandle) {
        self.node2d.remove_child(child);
    }

    fn set_parent(&mut self, parent: Option<NodeHandle>) {
        self.node2d.set_parent(parent);
    }

    fn detach(&mut self) {
        self.node2d.detach();
    }

    fn set_paused(&mut self, paused: bool) {
        self.node2d.set_paused(paused);
    }

    fn set_visible(&mut self, visible: bool) {
        self.node2d.set_visible(visible);
    }

    fn set_name(&mut self, name: String) {
        self.node2d.set_name(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite2d_creation() {
        let sprite = Sprite::new(1);
        let node = Sprite2D::new("test", sprite);
        assert_eq!(node.name(), "test");
    }

    #[test]
    fn test_sprite2d_sprite() {
        let sprite = Sprite::new(1);
        let mut node = Sprite2D::new("test", sprite);
        assert_eq!(node.sprite().texture_id, 1);
        node.set_sprite(Sprite::new(2));
        assert_eq!(node.sprite().texture_id, 2);
    }
}
