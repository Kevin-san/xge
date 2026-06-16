//! 场景模块
//!
//! 提供 Scene3D 场景图和 Node3D 节点类型。

use engine_math::Vec3;
use engine_utils::Handle;
use crate::camera::Camera3D;
use crate::geometry::AABB;
use crate::light::LightManager;
use crate::mesh::Mesh3D;
use crate::transform::Transform3D;

/// 节点句柄类型
pub type NodeHandle = Handle<Node3D>;

/// 3D 场景节点
#[derive(Debug, Clone)]
pub struct Node3D {
    /// 节点名称
    name: String,
    /// 父节点句柄
    parent: Option<NodeHandle>,
    /// 子节点句柄列表
    children: Vec<NodeHandle>,
    /// 本地变换
    local_transform: Transform3D,
    /// 世界变换（缓存）
    world_transform: Transform3D,
    /// 世界变换是否需要更新
    dirty: bool,
    /// 挂载的网格句柄
    mesh: Option<Handle<Mesh3D>>,
    /// 是否可见
    visible: bool,
}

impl Node3D {
    /// 创建新节点
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建带名称的节点
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// 创建带网格的节点
    pub fn with_mesh(mesh: Handle<Mesh3D>) -> Self {
        Self {
            mesh: Some(mesh),
            ..Default::default()
        }
    }

    /// 获取名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 设置名称
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// 获取父节点
    pub fn parent(&self) -> Option<&NodeHandle> {
        self.parent.as_ref()
    }

    /// 设置父节点
    pub fn set_parent(&mut self, parent: Option<NodeHandle>) {
        self.parent = parent;
    }

    /// 获取子节点列表
    pub fn children(&self) -> &[NodeHandle] {
        &self.children
    }

    /// 添加子节点
    pub fn add_child(&mut self, child: NodeHandle) {
        self.children.push(child);
    }

    /// 移除子节点
    pub fn remove_child(&mut self, child: NodeHandle) {
        self.children.retain(|c| c.index() != child.index());
    }

    /// 获取本地变换
    pub fn local_transform(&self) -> &Transform3D {
        &self.local_transform
    }

    /// 获取世界变换
    pub fn world_transform(&self) -> &Transform3D {
        &self.world_transform
    }

    /// 设置本地变换
    pub fn set_local_transform(&mut self, transform: Transform3D) {
        self.local_transform = transform;
        self.dirty = true;
    }

    /// 获取网格句柄
    pub fn mesh(&self) -> Option<&Handle<Mesh3D>> {
        self.mesh.as_ref()
    }

    /// 设置网格
    pub fn set_mesh(&mut self, mesh: Option<Handle<Mesh3D>>) {
        self.mesh = mesh;
    }

    /// 是否可见
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// 设置可见性
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// 获取包围盒（本地空间）
    pub fn aabb(&self) -> Option<AABB> {
        // 这里需要通过 mesh handle 获取实际的网格包围盒
        // 简化实现返回 None
        None
    }

    /// 标记为需要更新
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// 检查是否需要更新
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 更新世界变换
    pub fn update_world_transform(&mut self, parent_world: Option<&Transform3D>) {
        if self.dirty {
            if let Some(parent_transform) = parent_world {
                self.world_transform = Transform3D::lerp(
                    *parent_transform,
                    self.local_transform,
                    1.0,
                );
            } else {
                self.world_transform = self.local_transform;
            }
            self.dirty = false;
        }
    }
}

impl Default for Node3D {
    fn default() -> Self {
        Self {
            name: String::new(),
            parent: None,
            children: Vec::new(),
            local_transform: Transform3D::IDENTITY,
            world_transform: Transform3D::IDENTITY,
            dirty: true,
            mesh: None,
            visible: true,
        }
    }
}

/// 渲染实体
#[derive(Debug, Clone)]
pub struct RenderEntity3D {
    /// 节点句柄
    pub node: NodeHandle,
    /// 网格句柄
    pub mesh: Handle<Mesh3D>,
    /// 世界矩阵
    pub world_matrix: engine_math::Mat4,
}

/// 场景统计信息
#[derive(Debug, Clone, Default)]
pub struct SceneStats3D {
    /// 总节点数
    pub nodes: usize,
    /// 可见节点数
    pub visible_nodes: usize,
    /// 总三角面数
    pub total_triangles: usize,
}

/// 3D 场景
#[derive(Debug, Clone)]
pub struct Scene3D {
    /// 节点列表
    nodes: Vec<Node3D>,
    /// 根节点句柄列表
    root_nodes: Vec<NodeHandle>,
    /// 主相机句柄
    main_camera: Option<NodeHandle>,
    /// 光源管理器
    light_manager: LightManager,
}

impl Scene3D {
    /// 创建新场景
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_nodes: Vec::new(),
            main_camera: None,
            light_manager: LightManager::default(),
        }
    }

    /// 添加节点
    pub fn add_node(&mut self, mut node: Node3D) -> NodeHandle {
        let handle = Handle::new(self.nodes.len() as u32, 0);
        node.mark_dirty();
        self.nodes.push(node);
        self.root_nodes.push(handle.clone());
        handle
    }

    /// 移除节点
    pub fn remove_node(&mut self, handle: NodeHandle) {
        let index = handle.index() as usize;
        if index >= self.nodes.len() {
            return;
        }

        // 递归移除子节点（克隆子节点列表避免借用冲突）
        let children: Vec<NodeHandle> = self.nodes[index].children.clone();
        for child in children {
            self.remove_node(child);
        }

        // 从根节点列表移除
        self.root_nodes.retain(|h| h.index() != handle.index());

        // 移除节点
        self.nodes.remove(index);

        // 注意：移除后节点的索引会改变，子节点的父引用需要更新
        // 这是一个简化实现，真正的引擎需要处理索引映射
    }

    /// 获取节点引用
    pub fn node(&self, handle: &NodeHandle) -> Option<&Node3D> {
        self.nodes.get(handle.index() as usize)
    }

    /// 获取节点可变引用
    pub fn node_mut(&mut self, handle: &NodeHandle) -> Option<&mut Node3D> {
        self.nodes.get_mut(handle.index() as usize)
    }

    /// 获取所有节点
    pub fn nodes(&self) -> &[Node3D] {
        &self.nodes
    }

    /// 获取根节点列表
    pub fn root_nodes(&self) -> &[NodeHandle] {
        &self.root_nodes
    }

    /// 获取主相机
    pub fn main_camera(&self) -> Option<&Camera3D> {
        // 这里需要从节点中获取相机组件
        None
    }

    /// 设置主相机
    pub fn set_main_camera(&mut self, _handle: NodeHandle) {
        self.main_camera = Some(_handle);
    }

    /// 获取光源管理器
    pub fn light_manager(&self) -> &LightManager {
        &self.light_manager
    }

    /// 获取光源管理器可变引用
    pub fn light_manager_mut(&mut self) -> &mut LightManager {
        &mut self.light_manager
    }

    /// 更新所有世界变换（从父到子传播）
    pub fn update_world_transforms(&mut self) {
        // 从根节点开始更新
        let roots = self.root_nodes.clone();
        for root in roots {
            self.update_transform_recursive(root, None);
        }
    }

    /// 递归更新变换
    fn update_transform_recursive(&mut self, handle: NodeHandle, parent_world: Option<&Transform3D>) {
        // 获取节点的子节点句柄副本，避免借用冲突
        let children_handles: Vec<NodeHandle> = {
            if let Some(node) = self.node(&handle) {
                node.children().to_vec()
            } else {
                return;
            }
        };

        // 更新当前节点
        {
            if let Some(node) = self.node_mut(&handle) {
                node.update_world_transform(parent_world);
            }
        }

        // 获取当前节点的世界变换
        let world = if let Some(node) = self.node(&handle) {
            *node.world_transform()
        } else {
            return;
        };

        // 递归更新子节点
        for child in children_handles {
            self.update_transform_recursive(child, Some(&world));
        }
    }

    /// 视锥裁剪
    pub fn cull(&mut self, _frustum: &crate::camera::Frustum) {
        // 简化实现：后续需要遍历所有节点进行裁剪测试
    }

    /// 获取可见渲染实体列表
    pub fn visible_entities(&self) -> Vec<RenderEntity3D> {
        let mut entities = Vec::new();
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.visible() && node.mesh().is_some() {
                entities.push(RenderEntity3D {
                    node: Handle::new(idx as u32, 0),
                    mesh: node.mesh().unwrap().clone(),
                    world_matrix: node.world_transform().matrix(),
                });
            }
        }
        entities
    }

    /// 获取场景统计
    pub fn stats(&self) -> SceneStats3D {
        let mut visible_count = 0;
        let mut triangle_count = 0;

        for node in &self.nodes {
            if node.visible() {
                visible_count += 1;
                // 后续需要通过 mesh handle 获取三角面数
            }
        }

        SceneStats3D {
            nodes: self.nodes.len(),
            visible_nodes: visible_count,
            total_triangles: triangle_count,
        }
    }

    /// 查找节点
    pub fn find_node(&self, name: &str) -> Option<NodeHandle> {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.name() == name {
                return Some(Handle::new(idx as u32, 0));
            }
        }
        None
    }

    /// 获取节点的世界位置
    pub fn node_world_position(&self, handle: NodeHandle) -> Option<Vec3> {
        self.node(&handle).map(|n| n.world_transform().translation())
    }
}

impl Default for Scene3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_add_node() {
        let mut scene = Scene3D::new();
        let node = Node3D::with_name("test");
        let handle = scene.add_node(node);
        assert_eq!(scene.nodes().len(), 1);
        assert_eq!(scene.node(&handle).unwrap().name(), "test");
    }

    #[test]
    fn test_scene_remove_node() {
        let mut scene = Scene3D::new();
        let node = Node3D::with_name("test");
        let handle = scene.add_node(node);
        scene.remove_node(handle);
        assert!(scene.node(&handle).is_none());
    }

    #[test]
    fn test_scene_stats() {
        let mut scene = Scene3D::new();
        scene.add_node(Node3D::with_name("node1"));
        scene.add_node(Node3D::with_name("node2"));
        let stats = scene.stats();
        assert_eq!(stats.nodes, 2);
    }
}
