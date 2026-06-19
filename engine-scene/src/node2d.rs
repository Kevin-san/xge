//! Node2D 模块
//!
//! 提供 2D 节点实现，包含变换属性。

use super::{Node, NodeHandle};
use engine_math::Vec2;
use std::any::Any;

/// 2D 节点
///
/// 所有 2D 游戏对象的基础节点类型。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Node2D {
    /// 节点名称
    name: String,
    /// 父节点句柄
    #[serde(skip)]
    parent: Option<NodeHandle>,
    /// 子节点列表
    #[serde(skip)]
    children: Vec<NodeHandle>,
    /// 局部位置
    position: Vec2,
    /// 局部旋转（弧度）
    rotation: f32,
    /// 局部缩放
    scale: Vec2,
    /// Z 索引（用于排序）
    z_index: i32,
    /// 是否可见
    visible: bool,
    /// 是否暂停
    paused: bool,
    /// 变换脏标记
    #[serde(skip)]
    transform_dirty: bool,
}

impl Node2D {
    /// 创建新的 2D 节点
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: None,
            children: Vec::new(),
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            z_index: 0,
            visible: true,
            paused: false,
            transform_dirty: true,
        }
    }

    /// 获取局部位置
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// 设置局部位置
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.transform_dirty = true;
    }

    /// 移动位置
    pub fn translate(&mut self, delta: Vec2) {
        self.position += delta;
        self.transform_dirty = true;
    }

    /// 获取局部旋转
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// 设置局部旋转
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.transform_dirty = true;
    }

    /// 旋转
    pub fn rotate(&mut self, delta: f32) {
        self.rotation += delta;
        self.transform_dirty = true;
    }

    /// 获取局部缩放
    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    /// 设置缩放
    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        self.transform_dirty = true;
    }

    /// 获取 Z 索引
    pub fn z_index(&self) -> i32 {
        self.z_index
    }

    /// 设置 Z 索引
    pub fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    /// 检查变换是否脏
    pub fn is_transform_dirty(&self) -> bool {
        self.transform_dirty
    }

    /// 清空脏标记
    pub fn clear_transform_dirty(&mut self) {
        self.transform_dirty = false;
    }

    /// 获取子节点列表引用
    pub fn children(&self) -> &[NodeHandle] {
        &self.children
    }

    /// 获取父节点句柄
    pub fn parent(&self) -> Option<NodeHandle> {
        self.parent
    }
}

impl Node for Node2D {
    fn name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> Option<NodeHandle> {
        self.parent
    }

    fn children(&self) -> &[NodeHandle] {
        &self.children
    }

    fn paused(&self) -> bool {
        self.paused
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn on_ready(&mut self) {
        // 默认实现为空
    }

    fn on_update(&mut self, _dt: f32) {
        // 默认实现为空
    }

    fn on_draw(&self) {
        // 默认实现为空
    }

    fn on_destroy(&mut self) {
        // 默认实现为空
    }

    fn add_child(&mut self, child: NodeHandle) {
        if !self.children.contains(&child) {
            self.children.push(child);
        }
    }

    fn remove_child(&mut self, child: NodeHandle) {
        self.children.retain(|&c| c != child);
    }

    fn set_parent(&mut self, parent: Option<NodeHandle>) {
        self.parent = parent;
    }

    fn detach(&mut self) {
        self.parent = None;
    }

    fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn node_type(&self) -> &'static str {
        "Node2D"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node2d_creation() {
        let node = Node2D::new("test");
        assert_eq!(node.name(), "test");
        assert_eq!(node.position(), Vec2::ZERO);
        assert_eq!(node.rotation(), 0.0);
        assert_eq!(node.scale(), Vec2::ONE);
    }

    #[test]
    fn test_node2d_transform() {
        let mut node = Node2D::new("test");
        node.set_position(Vec2::new(10.0, 20.0));
        node.set_rotation(std::f32::consts::PI / 2.0);
        node.set_scale(Vec2::new(2.0, 2.0));

        assert_eq!(node.position(), Vec2::new(10.0, 20.0));
        assert!((node.rotation() - std::f32::consts::PI / 2.0).abs() < 0.001);
        assert_eq!(node.scale(), Vec2::new(2.0, 2.0));
    }

    #[test]
    fn test_node2d_children() {
        let mut node = Node2D::new("parent");
        let child_handle = NodeHandle::new(0);
        node.add_child(child_handle);
        assert_eq!(node.children().len(), 1);
        node.remove_child(child_handle);
        assert_eq!(node.children().len(), 0);
    }

    #[test]
    fn test_node2d_paused() {
        let mut node = Node2D::new("test");
        assert!(!node.paused());
        node.set_paused(true);
        assert!(node.paused());
    }

    #[test]
    fn test_node2d_visible() {
        let mut node = Node2D::new("test");
        assert!(node.visible());
        node.set_visible(false);
        assert!(!node.visible());
    }
}
