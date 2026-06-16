//! 3D Scene node tree

use alloc::vec::Vec;
use alloc::string::String;
use engine_math::{Mat4, Vec3};
use engine_utils::Handle;
use crate::transform::Transform3D;
use crate::geometry::AABB;
use crate::mesh::Mesh3D;
use crate::frustum::Frustum;

/// Node handle type
pub type NodeHandle = Handle<Node3D>;

/// 3D Scene node
#[derive(Debug)]
pub struct Node3D {
    name: String,
    parent: Option<NodeHandle>,
    children: Vec<NodeHandle>,
    local_transform: Transform3D,
    world_transform: Transform3D,
    mesh: Option<Handle<Mesh3D>>,
    material_index: Option<usize>,
    visible: bool,
    aabb: AABB,
}

impl Node3D {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            parent: None,
            children: Vec::new(),
            local_transform: Transform3D::IDENTITY,
            world_transform: Transform3D::IDENTITY,
            mesh: None,
            material_index: None,
            visible: true,
            aabb: AABB::EMPTY,
        }
    }

    pub fn with_name(name: String) -> Self {
        Self {
            name,
            parent: None,
            children: Vec::new(),
            local_transform: Transform3D::IDENTITY,
            world_transform: Transform3D::IDENTITY,
            mesh: None,
            material_index: None,
            visible: true,
            aabb: AABB::EMPTY,
        }
    }

    pub fn with_mesh(mesh: Handle<Mesh3D>) -> Self {
        Self {
            name: String::new(),
            parent: None,
            children: Vec::new(),
            local_transform: Transform3D::IDENTITY,
            world_transform: Transform3D::IDENTITY,
            mesh: Some(mesh),
            material_index: None,
            visible: true,
            aabb: AABB::EMPTY,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn parent(&self) -> Option<NodeHandle> {
        self.parent.clone()
    }

    #[inline]
    pub fn children(&self) -> &[NodeHandle] {
        &self.children
    }

    #[inline]
    pub fn local_transform(&self) -> &Transform3D {
        &self.local_transform
    }

    #[inline]
    pub fn world_transform(&self) -> &Transform3D {
        &self.world_transform
    }

    #[inline]
    pub fn aabb(&self) -> AABB {
        self.aabb
    }

    #[inline]
    pub fn visible(&self) -> bool {
        self.visible
    }

    #[inline]
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    #[inline]
    pub fn mesh(&self) -> Option<Handle<Mesh3D>> {
        self.mesh.clone()
    }

    #[inline]
    pub fn material(&self) -> Option<usize> {
        self.material_index
    }

    #[inline]
    pub fn set_local_transform(&mut self, transform: Transform3D) {
        self.local_transform = transform;
    }

    #[inline]
    pub fn set_mesh(&mut self, mesh: Handle<Mesh3D>) {
        self.mesh = Some(mesh);
    }

    #[inline]
    pub fn set_material(&mut self, index: usize) {
        self.material_index = Some(index);
    }

    pub fn add_child(&mut self, child: NodeHandle) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: NodeHandle) {
        self.children.retain(|h| h.index() != child.index());
    }
}

impl Default for Node3D {
    fn default() -> Self {
        Self::new()
    }
}

/// Render entity for drawing
#[derive(Clone, Debug)]
pub struct RenderEntity3D {
    pub node_handle: NodeHandle,
    pub mesh: Handle<Mesh3D>,
    pub material_index: Option<usize>,
    pub world_matrix: Mat4,
    pub aabb: AABB,
}

/// Scene statistics
#[derive(Clone, Debug, Default)]
pub struct SceneStats3D {
    pub total_nodes: usize,
    pub visible_nodes: usize,
    pub total_triangles: usize,
    pub culled_nodes: usize,
}

/// 3D Scene containing node tree
#[derive(Debug)]
pub struct Scene3D {
    nodes: Vec<Node3D>,
    root_nodes: Vec<NodeHandle>,
    visible_entities: Vec<RenderEntity3D>,
    main_camera_node: Option<NodeHandle>,
    stats: SceneStats3D,
}

impl Scene3D {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_nodes: Vec::new(),
            visible_entities: Vec::new(),
            main_camera_node: None,
            stats: SceneStats3D::default(),
        }
    }

    /// Add a node to the scene
    pub fn add_node(&mut self, node: Node3D) -> NodeHandle {
        let index = self.nodes.len() as u32;
        self.nodes.push(node);
        NodeHandle::new(index, 0)
    }

    /// Add a root node (no parent)
    pub fn add_root_node(&mut self, node: Node3D) -> NodeHandle {
        let handle = self.add_node(node);
        self.root_nodes.push(handle.clone());
        handle
    }

    /// Remove a node from the scene
    pub fn remove_node(&mut self, handle: NodeHandle) {
        let index = handle.index() as usize;
        if index < self.nodes.len() {
            // Remove from parent's children
            if let Some(parent_handle) = self.nodes[index].parent.clone() {
                if let Some(parent) = self.node_mut(parent_handle) {
                    parent.remove_child(handle.clone());
                }
            }
            // Remove from root nodes
            let handle_index = handle.index();
            self.root_nodes.retain(|h| h.index() != handle_index);
        }
    }

    /// Get node by handle
    #[inline]
    pub fn node(&self, handle: NodeHandle) -> Option<&Node3D> {
        self.nodes.get(handle.index() as usize)
    }

    /// Get node by handle (mutable)
    #[inline]
    pub fn node_mut(&mut self, handle: NodeHandle) -> Option<&mut Node3D> {
        self.nodes.get_mut(handle.index() as usize)
    }

    /// Get all nodes
    #[inline]
    pub fn nodes(&self) -> &[Node3D] {
        &self.nodes
    }

    /// Get root node handles
    #[inline]
    pub fn root_nodes(&self) -> Vec<NodeHandle> {
        self.root_nodes.clone()
    }

    /// Set main camera node
    #[inline]
    pub fn set_main_camera(&mut self, handle: NodeHandle) {
        self.main_camera_node = Some(handle);
    }

    /// Get main camera node handle
    #[inline]
    pub fn main_camera(&self) -> Option<NodeHandle> {
        self.main_camera_node.clone()
    }

    /// Update world transforms from parent to children
    pub fn update_world_transforms(&mut self) {
        // Start with root nodes
        let root_nodes = self.root_nodes.clone();
        for root_handle in root_nodes {
            self.update_node_world_transform(root_handle, Transform3D::IDENTITY);
        }
    }

    fn update_node_world_transform(&mut self, handle: NodeHandle, parent_world: Transform3D) {
        if let Some(node) = self.node_mut(handle) {
            // Compute world transform = parent_world * local_transform
            let local_mat = node.local_transform.matrix();
            let parent_mat = parent_world.matrix();
            let world_mat = parent_mat * local_mat;

            // Update world transform (simplified - just store matrix)
            node.world_transform = Transform3D::from_translation(
                Vec3::new(world_mat.cols[3][0], world_mat.cols[3][1], world_mat.cols[3][2])
            );

            // Update children
            let children = node.children.clone();
            let world = node.world_transform;
            for child_handle in children {
                self.update_node_world_transform(child_handle, world);
            }
        }
    }

    /// Perform frustum culling
    pub fn cull(&mut self, frustum: &Frustum) {
        self.visible_entities.clear();
        self.stats.culled_nodes = 0;
        self.stats.visible_nodes = 0;
        self.stats.total_nodes = self.nodes.len();
        self.stats.total_triangles = 0;

        for (index, node) in self.nodes.iter().enumerate() {
            if !node.visible {
                continue;
            }

            if frustum.contains_aabb(node.aabb) {
                self.stats.visible_nodes += 1;

                if let Some(mesh_handle) = node.mesh.clone() {
                    self.visible_entities.push(RenderEntity3D {
                        node_handle: NodeHandle::new(index as u32, 0),
                        mesh: mesh_handle,
                        material_index: node.material_index,
                        world_matrix: node.world_transform.matrix(),
                        aabb: node.aabb,
                    });
                }
            } else {
                self.stats.culled_nodes += 1;
            }
        }
    }

    /// Get visible entities after culling
    #[inline]
    pub fn visible_entities(&self) -> &[RenderEntity3D] {
        &self.visible_entities
    }

    /// Get scene statistics
    #[inline]
    pub fn stats(&self) -> &SceneStats3D {
        &self.stats
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
    fn test_node_creation() {
        let node = Node3D::with_name(String::from("test"));
        assert_eq!(node.name(), "test");
        assert!(node.visible());
    }

    #[test]
    fn test_scene_add_node() {
        let mut scene = Scene3D::new();
        let node = Node3D::new();
        let handle = scene.add_node(node);
        assert_eq!(scene.nodes().len(), 1);
        assert!(scene.node(handle).is_some());
    }

    #[test]
    fn test_scene_hierarchy() {
        let mut scene = Scene3D::new();

        let parent_handle = scene.add_root_node(Node3D::with_name(String::from("parent")));
        let child_handle = scene.add_node(Node3D::with_name(String::from("child")));

        // Set parent-child relationship
        if let Some(child) = scene.node_mut(child_handle.clone()) {
            child.parent = Some(parent_handle.clone());
        }
        if let Some(parent) = scene.node_mut(parent_handle.clone()) {
            parent.add_child(child_handle.clone());
        }

        assert_eq!(scene.node(parent_handle).unwrap().children().len(), 1);
    }

    #[test]
    fn test_scene_culling() {
        let mut scene = Scene3D::new();

        // Add a node with mesh
        let node = Node3D::new();
        let _handle = scene.add_root_node(node);

        // Create a simple frustum (identity VP)
        let frustum = Frustum::from_view_projection(Mat4::IDENTITY);

        scene.cull(&frustum);
        // Should have processed nodes
        assert!(scene.stats().total_nodes > 0);
    }
}