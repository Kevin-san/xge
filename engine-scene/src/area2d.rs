//! Area2D 模块
//!
//! 提供 2D 检测区域节点实现。

use super::{Node, Node2D, NodeHandle};
use engine_math::Vec2;

/// 刚体句柄
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BodyHandle(u32);

impl BodyHandle {
    /// 创建新的句柄
    pub fn new(index: u32) -> Self {
        Self(index)
    }

    /// 获取索引
    pub fn index(&self) -> u32 {
        self.0
    }

    /// 空句柄
    pub fn null() -> Self {
        Self(u32::MAX)
    }

    /// 是否是空句柄
    pub fn is_null(&self) -> bool {
        self.0 == u32::MAX
    }
}

/// 碰撞形状
#[derive(Debug, Clone)]
pub enum ColliderShape {
    /// 圆形
    Circle {
        /// 半径
        radius: f32,
    },
    /// 矩形
    Rectangle {
        /// 宽度
        width: f32,
        /// 高度
        height: f32,
    },
}

/// 2D 检测区域
///
/// 用于检测物理刚体进入和离开区域。
pub struct Area2D {
    /// 碰撞形状
    pub shape: ColliderShape,
    /// 位置偏移
    pub position_offset: Vec2,
    /// 是否是传感器
    pub is_sensor: bool,
    /// 进入区域的刚体列表
    entered_bodies: Vec<BodyHandle>,
}

impl Area2D {
    /// 创建新的区域
    pub fn new(_name: impl Into<String>) -> Self {
        Self {
            shape: ColliderShape::Rectangle {
                width: 64.0,
                height: 64.0,
            },
            position_offset: Vec2::ZERO,
            is_sensor: true,
            entered_bodies: Vec::new(),
        }
    }

    /// 创建带碰撞体的区域
    pub fn with_shape(shape: ColliderShape) -> Self {
        Self {
            shape,
            position_offset: Vec2::ZERO,
            is_sensor: true,
            entered_bodies: Vec::new(),
        }
    }

    /// 获取进入区域的刚体列表
    pub fn entered_bodies(&self) -> &[BodyHandle] {
        &self.entered_bodies
    }

    /// 添加进入的刚体
    pub fn add_entered_body(&mut self, body: BodyHandle) {
        if !self.entered_bodies.contains(&body) {
            self.entered_bodies.push(body);
        }
    }

    /// 移除离开的刚体
    pub fn remove_entered_body(&mut self, body: &BodyHandle) {
        self.entered_bodies.retain(|b| b != body);
    }
}

/// 2D 区域节点
pub struct Area2DNode {
    /// 基础 2D 节点
    node2d: Node2D,
    /// 区域数据
    area: Area2D,
}

impl Area2DNode {
    /// 创建新的区域节点
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            node2d: Node2D::new(name),
            area: Area2D::new(""),
        }
    }

    /// 创建带形状的区域节点
    pub fn with_shape(name: impl Into<String>, shape: ColliderShape) -> Self {
        Self {
            node2d: Node2D::new(name),
            area: Area2D::with_shape(shape),
        }
    }

    /// 获取区域引用
    pub fn area(&self) -> &Area2D {
        &self.area
    }

    /// 获取可变区域引用
    pub fn area_mut(&mut self) -> &mut Area2D {
        &mut self.area
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

impl Node for Area2DNode {
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
    fn test_area2d_creation() {
        let area = Area2D::new("test");
        assert!(area.entered_bodies().is_empty());
    }

    #[test]
    fn test_area2d_node() {
        let node = Area2DNode::new("area");
        assert_eq!(node.name(), "area");
    }

    // ============= Area2D 更多测试 =============

    #[test]
    fn test_area2d_with_shape_rectangle() {
        let area = Area2D::with_shape(ColliderShape::Rectangle { width: 10.0, height: 10.0 });
        assert!(area.is_sensor);
    }

    #[test]
    fn test_area2d_with_shape_circle() {
        let area = Area2D::with_shape(ColliderShape::Circle { radius: 5.0 });
        assert!(area.is_sensor);
    }

    #[test]
    fn test_area2d_entered_bodies_add_remove() {
        let mut area = Area2D::new("area");
        let handle = BodyHandle::new(1);
        area.add_entered_body(handle);
        area.add_entered_body(handle);
        assert_eq!(area.entered_bodies().len(), 1);
        area.remove_entered_body(&handle);
        assert_eq!(area.entered_bodies().len(), 0);
    }

    #[test]
    fn test_area2d_position_offset() {
        let mut area = Area2D::new("area");
        area.position_offset = Vec2::new(5.0, 5.0);
        assert_eq!(area.position_offset, Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_area2d_node_area_access() {
        let node = Area2DNode::new("area");
        assert!(node.area().entered_bodies().is_empty());
    }

    #[test]
    fn test_area2d_node_area_mut() {
        let mut node = Area2DNode::new("area");
        node.area_mut().add_entered_body(BodyHandle::new(1));
        assert_eq!(node.area().entered_bodies().len(), 1);
    }

    #[test]
    fn test_area2d_node_on_update_draw() {
        let mut node = Area2DNode::new("area");
        node.on_update(0.016);
        node.on_draw();
    }

    #[test]
    fn test_area2d_node_add_remove_child() {
        let mut node = Area2DNode::new("area");
        node.add_child(NodeHandle::new(1));
        assert_eq!(node.children().len(), 1);
        node.remove_child(NodeHandle::new(1));
        assert_eq!(node.children().len(), 0);
    }

    #[test]
    fn test_area2d_node_paused_visible() {
        let mut node = Area2DNode::new("area");
        node.set_paused(true);
        assert!(node.paused());
        node.set_visible(false);
        assert!(!node.visible());
    }

    #[test]
    fn test_area2d_node_detach() {
        let mut node = Area2DNode::new("area");
        node.set_parent(Some(NodeHandle::new(1)));
        assert!(node.parent().is_some());
        node.detach();
        assert!(node.parent().is_none());
    }

    #[test]
    fn test_body_handle_index() {
        let h = BodyHandle::new(42);
        assert_eq!(h.index(), 42);
    }

    #[test]
    fn test_body_handle_null() {
        let h = BodyHandle::null();
        assert!(h.is_null());
    }

    #[test]
    fn test_area2d_with_shape_sensor_flag() {
        let area = Area2D::with_shape(ColliderShape::Rectangle { width: 10.0, height: 10.0 });
        assert!(area.is_sensor);
    }
}
