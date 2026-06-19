//! Camera2D 模块
//!
//! 提供 2D 相机节点实现。

use super::{Node, Node2D, NodeHandle};
use engine_math::{Rect, Vec2};

/// 2D 相机
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// 是否是当前激活相机
    current: bool,
    /// 缩放
    zoom: f32,
    /// 是否使用固定缩放
    anchor_mode: CameraAnchorMode,
    /// 视野矩形
    view_rect: Rect,
    /// 是否跟随目标
    following: Option<Vec2>,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            current: false,
            zoom: 1.0,
            anchor_mode: CameraAnchorMode::FixedRect,
            view_rect: Rect::new(0.0, 0.0, 1920.0, 1080.0),
            following: None,
        }
    }
}

/// 相机锚点模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraAnchorMode {
    /// 固定矩形
    FixedRect,
    /// 固定中心
    FixedCenter,
    /// 跟随目标
    FollowTarget,
}

impl Camera2D {
    /// 创建新的 2D 相机
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置为当前激活相机
    pub fn set_current(&mut self, current: bool) {
        self.current = current;
    }

    /// 检查是否是当前激活相机
    pub fn is_current(&self) -> bool {
        self.current
    }

    /// 获取缩放
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// 设置缩放
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1);
    }

    /// 获取锚点模式
    pub fn anchor_mode(&self) -> CameraAnchorMode {
        self.anchor_mode
    }

    /// 设置锚点模式
    pub fn set_anchor_mode(&mut self, mode: CameraAnchorMode) {
        self.anchor_mode = mode;
    }

    /// 获取视野矩形
    pub fn view_rect(&self) -> Rect {
        self.view_rect
    }

    /// 设置视野矩形
    pub fn set_view_rect(&mut self, rect: Rect) {
        self.view_rect = rect;
    }

    /// 设置跟随目标
    pub fn set_follow(&mut self, target: Option<Vec2>) {
        self.following = target;
        if target.is_some() {
            self.anchor_mode = CameraAnchorMode::FollowTarget;
        }
    }

    /// 应用变换到相机
    pub fn apply_transform(&self, _node: &Node2D) {
        // 根据节点位置和相机设置计算视图矩阵
        // 实际实现会在渲染器中完成
    }
}

/// 2D 相机节点
#[derive(Debug, Clone)]
pub struct Camera2DNode {
    /// 基础 2D 节点
    node2d: Node2D,
    /// 相机数据
    camera: Camera2D,
}

impl Camera2DNode {
    /// 创建新的相机节点
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            node2d: Node2D::new(name),
            camera: Camera2D::new(),
        }
    }

    /// 创建带相机的节点
    pub fn with_camera(name: impl Into<String>, camera: Camera2D) -> Self {
        Self {
            node2d: Node2D::new(name),
            camera,
        }
    }

    /// 获取相机引用
    pub fn camera(&self) -> &Camera2D {
        &self.camera
    }

    /// 获取可变相机引用
    pub fn camera_mut(&mut self) -> &mut Camera2D {
        &mut self.camera
    }

    /// 设置相机
    pub fn set_camera(&mut self, camera: Camera2D) {
        self.camera = camera;
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

impl Node for Camera2DNode {
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
    fn test_camera2d_creation() {
        let camera = Camera2D::new();
        assert!(!camera.is_current());
        assert_eq!(camera.zoom(), 1.0);
    }

    #[test]
    fn test_camera2d_zoom() {
        let mut camera = Camera2D::new();
        camera.set_zoom(2.0);
        assert_eq!(camera.zoom(), 2.0);
        camera.set_zoom(-1.0); // 应该被限制
        assert!(camera.zoom() > 0.0);
    }

    #[test]
    fn test_camera2d_node() {
        let node = Camera2DNode::new("camera");
        assert_eq!(node.name(), "camera");
    }

    // ============= Camera2D / Camera2DNode 更多测试 =============

    #[test]
    fn test_camera2d_is_current_toggle() {
        let mut camera = Camera2D::new();
        camera.set_current(true);
        assert!(camera.is_current());
        camera.set_current(false);
        assert!(!camera.is_current());
    }

    #[test]
    fn test_camera2d_with_camera_node() {
        let mut camera = Camera2D::new();
        camera.set_zoom(3.0);
        let node = Camera2DNode::with_camera("cam", camera);
        assert_eq!(node.camera().zoom(), 3.0);
    }

    #[test]
    fn test_camera2d_node_set_camera() {
        let mut node = Camera2DNode::new("cam");
        let mut c = Camera2D::new();
        c.set_zoom(0.5);
        node.set_camera(c);
        assert_eq!(node.camera().zoom(), 0.5);
    }

    #[test]
    fn test_camera2d_node_camera_mut() {
        let mut node = Camera2DNode::new("cam");
        node.camera_mut().set_zoom(5.0);
        assert_eq!(node.camera().zoom(), 5.0);
    }

    #[test]
    fn test_camera2d_node_node2d_access() {
        let node = Camera2DNode::new("cam");
        assert_eq!(node.node2d().position(), Vec2::ZERO);
    }

    #[test]
    fn test_camera2d_apply_transform_no_panic() {
        let camera = Camera2D::new();
        let dummy = Node2D::new("dummy");
        camera.apply_transform(&dummy);
    }

    #[test]
    fn test_camera2d_node_on_update() {
        let mut node = Camera2DNode::new("cam");
        node.on_update(0.016);
    }

    #[test]
    fn test_camera2d_node_on_draw() {
        let node = Camera2DNode::new("cam");
        node.on_draw();
    }

    #[test]
    fn test_camera2d_node_add_remove_child() {
        let mut node = Camera2DNode::new("cam");
        node.add_child(NodeHandle::new(1));
        assert_eq!(node.children().len(), 1);
        node.remove_child(NodeHandle::new(1));
        assert_eq!(node.children().len(), 0);
    }

    #[test]
    fn test_camera2d_node_set_paused_visible() {
        let mut node = Camera2DNode::new("cam");
        node.set_paused(true);
        assert!(node.paused());
        node.set_visible(false);
        assert!(!node.visible());
    }

    #[test]
    fn test_camera2d_node_detach_parent() {
        let mut node = Camera2DNode::new("cam");
        node.set_parent(Some(NodeHandle::new(5)));
        assert!(node.parent().is_some());
        node.detach();
        assert!(node.parent().is_none());
    }
}
