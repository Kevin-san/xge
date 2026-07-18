//! Node2D 模块
//!
//! 提供 2D 节点实现，包含变换属性。

use super::{Node, NodeHandle};
use engine_math::{Mat3, Vec2, Vec3};

/// 2D 节点
///
/// 所有 2D 游戏对象的基础节点类型。
#[derive(Debug, Clone)]
pub struct Node2D {
    /// 节点名称
    name: String,
    /// 父节点句柄
    parent: Option<NodeHandle>,
    /// 子节点列表
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

    /// 计算局部变换矩阵（2D 仿射变换用 3x3 矩阵表示）
    ///
    /// 矩阵组合顺序：Scale -> Rotate -> Translate
    pub fn local_matrix(&self) -> Mat3 {
        let t = Mat3::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            self.position.x, self.position.y, 1.0,
        );
        let r = Mat3::from_rotation_z(self.rotation);
        let s = Mat3::from_scale(Vec3::new(self.scale.x, self.scale.y, 1.0));
        t * r * s
    }

    /// 设置世界位置（需要传入父节点的世界逆矩阵）
    ///
    /// 仅在父节点存在时有效
    pub fn set_world_position(&mut self, world_pos: Vec2, parent_world_matrix: &Mat3) {
        if let Some(inv) = parent_world_matrix.inverse() {
            let local = inv.mul_vec3(Vec3::new(world_pos.x, world_pos.y, 1.0));
            self.position = Vec2::new(local.x, local.y);
            self.transform_dirty = true;
        }
    }

    /// 获取世界位置（需要传入父节点的世界矩阵）
    pub fn world_position(&self, parent_world_matrix: &Mat3) -> Vec2 {
        let local = parent_world_matrix.mul_vec3(Vec3::new(self.position.x, self.position.y, 1.0));
        Vec2::new(local.x, local.y)
    }

    /// 获取世界旋转（需要传入父节点旋转）
    pub fn world_rotation(&self, parent_rotation: f32) -> f32 {
        parent_rotation + self.rotation
    }

    /// 获取世界缩放（需要传入父节点缩放）
    pub fn world_scale(&self, parent_scale: Vec2) -> Vec2 {
        Vec2::new(
            parent_scale.x * self.scale.x,
            parent_scale.y * self.scale.y,
        )
    }

    /// 计算世界变换矩阵（需要传入父节点的世界矩阵）
    pub fn world_matrix(&self, parent_world_matrix: &Mat3) -> Mat3 {
        *parent_world_matrix * self.local_matrix()
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

    // ============= Node2D / Node 更多变换和层级测试 =============

    #[test]
    fn test_node2d_default_position_rotation_scale() {
        let node = Node2D::new("test");
        assert_eq!(node.position(), Vec2::ZERO);
        assert_eq!(node.rotation(), 0.0);
        assert_eq!(node.scale(), Vec2::ONE);
    }

    #[test]
    fn test_node2d_set_name() {
        let mut node = Node2D::new("original");
        node.set_name("changed".to_string());
        assert_eq!(node.name(), "changed");
    }

    #[test]
    fn test_node2d_children_empty_initially() {
        let node = Node2D::new("n");
        assert!(node.children().is_empty());
    }

    #[test]
    fn test_node2d_detach_sets_parent_none() {
        let mut node = Node2D::new("n");
        node.set_parent(Some(NodeHandle::new(1)));
        assert!(node.parent().is_some());
        node.detach();
        assert!(node.parent().is_none());
    }

    #[test]
    fn test_node2d_on_ready_on_update_on_draw_no_panic() {
        let mut node = Node2D::new("n");
        node.on_ready();
        node.on_update(0.016);
        node.on_draw();
        node.on_destroy();
    }

    #[test]
    fn test_node2d_set_multiple_positions() {
        let mut node = Node2D::new("n");
        node.set_position(Vec2::new(1.0, 2.0));
        assert_eq!(node.position(), Vec2::new(1.0, 2.0));
        node.set_position(Vec2::new(5.0, -3.0));
        assert_eq!(node.position(), Vec2::new(5.0, -3.0));
    }

    #[test]
    fn test_node2d_rotation_update() {
        let mut node = Node2D::new("n");
        node.set_rotation(std::f32::consts::PI);
        assert!((node.rotation() - std::f32::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_node2d_scale_half() {
        let mut node = Node2D::new("n");
        node.set_scale(Vec2::new(0.5, 0.5));
        assert_eq!(node.scale(), Vec2::new(0.5, 0.5));
    }

    #[test]
    fn test_node2d_multiple_children() {
        let mut node = Node2D::new("p");
        for i in 0..5 {
            node.add_child(NodeHandle::new(i));
        }
        assert_eq!(node.children().len(), 5);
        node.remove_child(NodeHandle::new(2));
        assert_eq!(node.children().len(), 4);
    }

    #[test]
    fn test_node2d_remove_nonexistent_child_keeps_same() {
        let mut node = Node2D::new("p");
        node.add_child(NodeHandle::new(0));
        node.remove_child(NodeHandle::new(99));
        assert_eq!(node.children().len(), 1);
    }

    #[test]
    fn test_node2d_toggle_paused() {
        let mut node = Node2D::new("n");
        node.set_paused(true);
        assert!(node.paused());
        node.set_paused(false);
        assert!(!node.paused());
    }

    #[test]
    fn test_node2d_toggle_visible() {
        let mut node = Node2D::new("n");
        node.set_visible(false);
        assert!(!node.visible());
        node.set_visible(true);
        assert!(node.visible());
    }

    #[test]
    fn test_node2d_handle_new() {
        let h = NodeHandle::new(5);
        assert_eq!(h.index(), 5);
    }

    // ============= 世界变换和矩阵测试 =============

    #[test]
    fn test_node2d_local_matrix_identity() {
        let node = Node2D::new("n");
        let m = node.local_matrix();
        // 平移分量在 cols[2][0] 和 cols[2][1]
        assert!((m.cols[2][0] - 0.0).abs() < 1e-5);
        assert!((m.cols[2][1] - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_node2d_local_matrix_translation() {
        let mut node = Node2D::new("n");
        node.set_position(Vec2::new(5.0, 3.0));
        let m = node.local_matrix();
        assert!((m.cols[2][0] - 5.0).abs() < 1e-5);
        assert!((m.cols[2][1] - 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_node2d_local_matrix_scale() {
        let mut node = Node2D::new("n");
        node.set_scale(Vec2::new(2.0, 3.0));
        let m = node.local_matrix();
        assert!((m.cols[0][0] - 2.0).abs() < 1e-5);
        assert!((m.cols[1][1] - 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_node2d_world_position_no_parent() {
        let node = Node2D::new("n");
        let identity = Mat3::IDENTITY;
        let wp = node.world_position(&identity);
        assert_eq!(wp, node.position());
    }

    #[test]
    fn test_node2d_world_position_with_parent() {
        let mut node = Node2D::new("child");
        node.set_position(Vec2::new(1.0, 0.0));
        // 父节点在世界 (5,3)
        let parent_mat = Mat3::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            5.0, 3.0, 1.0,
        );
        let wp = node.world_position(&parent_mat);
        assert!((wp.x - 6.0).abs() < 1e-4);
        assert!((wp.y - 3.0).abs() < 1e-4);
    }

    #[test]
    fn test_node2d_world_rotation() {
        let mut node = Node2D::new("n");
        node.set_rotation(std::f32::consts::FRAC_PI_4);
        let wr = node.world_rotation(std::f32::consts::FRAC_PI_4);
        assert!((wr - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    }

    #[test]
    fn test_node2d_world_scale() {
        let mut node = Node2D::new("n");
        node.set_scale(Vec2::new(2.0, 3.0));
        let ws = node.world_scale(Vec2::new(2.0, 2.0));
        assert_eq!(ws, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_node2d_world_matrix_identity_parent() {
        let mut node = Node2D::new("n");
        node.set_position(Vec2::new(5.0, 3.0));
        let identity = Mat3::IDENTITY;
        let wm = node.world_matrix(&identity);
        assert!((wm.cols[2][0] - 5.0).abs() < 1e-4);
        assert!((wm.cols[2][1] - 3.0).abs() < 1e-4);
    }

    #[test]
    fn test_node2d_set_world_position() {
        let mut node = Node2D::new("n");
        let parent_mat = Mat3::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            5.0, 3.0, 1.0,
        );
        node.set_world_position(Vec2::new(6.0, 3.0), &parent_mat);
        assert!((node.position().x - 1.0).abs() < 1e-4);
        assert!((node.position().y - 0.0).abs() < 1e-4);
    }

    #[test]
    fn test_node2d_local_matrix_rotation() {
        let mut node = Node2D::new("n");
        node.set_rotation(std::f32::consts::FRAC_PI_2);
        let m = node.local_matrix();
        // cos(90°) ≈ 0, sin(90°) ≈ 1
        assert!(m.cols[0][0].abs() < 1e-5);
        assert!((m.cols[0][1] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_node2d_world_matrix_chained() {
        let mut parent_node = Node2D::new("parent");
        parent_node.set_position(Vec2::new(10.0, 5.0));
        let mut child_node = Node2D::new("child");
        child_node.set_position(Vec2::new(2.0, 3.0));
        let parent_mat = parent_node.local_matrix();
        let child_world = child_node.world_position(&parent_mat);
        assert!((child_world.x - 12.0).abs() < 1e-4);
        assert!((child_world.y - 8.0).abs() < 1e-4);
    }
}
