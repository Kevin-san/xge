//! Body2D 模块
//!
//! 提供 2D 物理刚体节点实现。

use super::{Node, Node2D, NodeHandle};
use engine_math::Vec2;
use std::any::Any;

/// 刚体句柄
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default, serde::Serialize, serde::Deserialize)]
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

/// 2D 物理刚体节点
///
/// 关联物理引擎中的刚体，实现节点与物理世界的同步。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Body2DNode {
    /// 基础 2D 节点
    node2d: Node2D,
    /// 刚体句柄
    body_handle: BodyHandle,
    /// 质量
    mass: f32,
    /// 物理同步启用
    physics_sync: bool,
    /// 位置同步启用
    position_sync: bool,
}

impl Body2DNode {
    /// 创建新的刚体节点
    pub fn new(name: impl Into<String>, body_handle: BodyHandle) -> Self {
        Self {
            node2d: Node2D::new(name),
            body_handle,
            mass: 1.0,
            physics_sync: true,
            position_sync: true,
        }
    }

    /// 获取刚体句柄
    pub fn body(&self) -> BodyHandle {
        self.body_handle
    }

    /// 获取质量
    pub fn mass(&self) -> f32 {
        self.mass
    }

    /// 设置质量
    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    /// 是否启用物理同步
    pub fn physics_sync_enabled(&self) -> bool {
        self.physics_sync
    }

    /// 设置物理同步启用状态
    pub fn set_physics_sync(&mut self, enabled: bool) {
        self.physics_sync = enabled;
    }

    /// 是否启用位置同步
    pub fn position_sync_enabled(&self) -> bool {
        self.position_sync
    }

    /// 设置位置同步启用状态
    pub fn set_position_sync(&mut self, enabled: bool) {
        self.position_sync = enabled;
    }

    /// 从物理世界同步状态到节点
    pub fn sync_from_world(&mut self, _world_position: Vec2, _world_rotation: f32) {
        // 实际实现需要与物理世界交互
    }

    /// 同步节点状态到物理世界
    pub fn sync_to_world(&self) -> (Vec2, f32) {
        (self.node2d.position(), self.node2d.rotation())
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

impl Node for Body2DNode {
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

    fn node_type(&self) -> &'static str {
        "Body2D"
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
    fn test_body2d_node_creation() {
        let handle = BodyHandle::new(0);
        let node = Body2DNode::new("body", handle);
        assert_eq!(node.name(), "body");
        assert_eq!(node.body(), handle);
    }

    #[test]
    fn test_body2d_node_mass() {
        let handle = BodyHandle::new(0);
        let mut node = Body2DNode::new("body", handle);
        assert_eq!(node.mass(), 1.0);
        node.set_mass(2.0);
        assert_eq!(node.mass(), 2.0);
    }

    #[test]
    fn test_body2d_node_sync() {
        let handle = BodyHandle::new(0);
        let node = Body2DNode::new("body", handle);
        assert!(node.physics_sync_enabled());
        assert!(node.position_sync_enabled());
    }
}
