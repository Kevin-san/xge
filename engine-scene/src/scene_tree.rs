//! 场景树模块
//!
//! 提供场景树的完整实现，包括节点管理和遍历。

use slab::Slab;
use std::collections::HashMap;

use super::{Node, Node2D, NodeHandle};

/// 节点存储条目
struct NodeEntry {
    /// 节点数据
    node: Box<dyn Node>,
    /// 名称
    name: String,
}

/// 场景树
///
/// 管理所有场景节点，提供添加、移除、查找和遍历功能。
pub struct SceneTree {
    /// 节点存储
    nodes: Slab<NodeEntry>,
    /// 根节点句柄
    root: NodeHandle,
    /// 名称索引
    name_index: HashMap<String, Vec<NodeHandle>>,
}

impl SceneTree {
    /// 创建新的场景树
    pub fn new() -> Self {
        let mut nodes = Slab::new();
        let root_index = nodes.insert(NodeEntry {
            node: Box::new(Node2D::new("root")),
            name: "root".to_string(),
        });

        let mut name_index = HashMap::new();
        name_index.insert("root".to_string(), vec![NodeHandle::new(root_index as u32)]);

        Self {
            nodes,
            root: NodeHandle::new(root_index as u32),
            name_index,
        }
    }

    /// 获取根节点句柄
    pub fn root(&self) -> NodeHandle {
        self.root
    }

    /// 添加子节点到父节点
    pub fn add_child(&mut self, parent: NodeHandle, child: NodeHandle) {
        if parent == self.root || parent.index() as usize >= self.nodes.len() {
            return;
        }

        // 更新子节点的父节点
        if let Some(child_entry) = self.nodes.get_mut(parent.index() as usize) {
            child_entry.node.set_parent(Some(parent));

            // 更新父节点的子节点列表
            if let Some(parent_entry) = self.nodes.get_mut(parent.index() as usize) {
                parent_entry.node.add_child(child);
            }
        }
    }

    /// 从父节点移除子节点
    pub fn remove_child(&mut self, parent: NodeHandle, child: NodeHandle) {
        if parent.index() as usize >= self.nodes.len() || child.index() as usize >= self.nodes.len()
        {
            return;
        }

        // 更新子节点的父节点
        if let Some(child_entry) = self.nodes.get_mut(child.index() as usize) {
            child_entry.node.detach();

            // 从父节点的子节点列表中移除
            if let Some(parent_entry) = self.nodes.get_mut(parent.index() as usize) {
                parent_entry.node.remove_child(child);
            }
        }
    }

    /// 销毁节点及其子树
    pub fn destroy_node(&mut self, handle: NodeHandle) {
        if handle == self.root || handle.index() as usize >= self.nodes.len() {
            return;
        }

        // 递归销毁子节点
        let children: Vec<NodeHandle> = {
            if let Some(entry) = self.nodes.get(handle.index() as usize) {
                entry.node.children().to_vec()
            } else {
                return;
            }
        };

        for child in children {
            self.destroy_node(child);
        }

        // 获取节点名称并从名称索引中移除
        let name = {
            if let Some(entry) = self.nodes.get(handle.index() as usize) {
                entry.name.clone()
            } else {
                return;
            }
        };

        // 从名称索引中移除
        if let Some(handles) = self.name_index.get_mut(&name) {
            handles.retain(|&h| h != handle);
        }

        // 从父节点的子节点列表中移除
        if let Some(entry) = self.nodes.get(handle.index() as usize) {
            if let Some(parent_handle) = entry.node.parent() {
                if let Some(parent_entry) = self.nodes.get_mut(parent_handle.index() as usize) {
                    parent_entry.node.remove_child(handle);
                }
            }
        }

        // 从存储中移除
        self.nodes.remove(handle.index() as usize);
    }

    /// 获取节点引用
    pub fn get_node(&self, handle: NodeHandle) -> Option<&dyn Node> {
        self.nodes
            .get(handle.index() as usize)
            .map(|e| e.node.as_ref())
    }

    /// 获取可变节点引用
    pub fn get_node_mut(&mut self, handle: NodeHandle) -> Option<&mut Box<dyn Node>> {
        self.nodes
            .get_mut(handle.index() as usize)
            .map(|e| &mut e.node)
    }

    /// 添加新的 2D 节点
    pub fn add_2d_node(&mut self, parent: NodeHandle, name: impl Into<String>) -> NodeHandle {
        let name = name.into();
        let index = self.nodes.insert(NodeEntry {
            node: Box::new(Node2D::new(&name)),
            name: name.clone(),
        });
        let handle = NodeHandle::new(index as u32);

        // 更新名称索引
        self.name_index
            .entry(name.clone())
            .or_default()
            .push(handle);

        // 设置父子关系
        self.add_child(parent, handle);

        handle
    }

    /// 按名称查找节点（返回最后一个添加的）
    pub fn find_by_name(&self, name: &str) -> Option<NodeHandle> {
        self.name_index
            .get(name)
            .and_then(|handles| handles.last().copied())
    }

    /// 按名称查找所有匹配节点
    pub fn find_all_by_name(&self, name: &str) -> Vec<NodeHandle> {
        self.name_index.get(name).cloned().unwrap_or_default()
    }

    /// 更新场景树（先序遍历）
    pub fn update(&mut self, dt: f32) {
        self.update_node(self.root, dt);
    }

    /// 递归更新节点
    fn update_node(&mut self, handle: NodeHandle, dt: f32) {
        if handle.index() as usize >= self.nodes.len() {
            return;
        }

        let children: Vec<NodeHandle> = {
            if let Some(entry) = self.nodes.get_mut(handle.index() as usize) {
                if entry.node.paused() {
                    return;
                }
                entry.node.on_update(dt);
                entry.node.children().to_vec()
            } else {
                return;
            }
        };

        // 先序遍历子节点
        for child in children {
            self.update_node(child, dt);
        }
    }

    /// 绘制场景树（后序遍历）
    pub fn draw(&self) {
        self.draw_node(self.root);
    }

    /// 递归绘制节点
    fn draw_node(&self, handle: NodeHandle) {
        if handle.index() as usize >= self.nodes.len() {
            return;
        }

        let children: Vec<NodeHandle> = {
            if let Some(entry) = self.nodes.get(handle.index() as usize) {
                if !entry.node.visible() {
                    return;
                }
                entry.node.on_draw();
                entry.node.children().to_vec()
            } else {
                return;
            }
        };

        // 后序遍历子节点
        for child in children {
            self.draw_node(child);
        }
    }

    /// 获取节点数量
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 获取根节点引用
    pub fn get_root_node(&self) -> &dyn Node {
        self.nodes
            .get(self.root.index() as usize)
            .map(|e| e.node.as_ref())
            .unwrap()
    }

    /// 获取根节点可变引用
    pub fn get_root_node_mut(&mut self) -> &mut dyn Node {
        self.nodes
            .get_mut(self.root.index() as usize)
            .map(|e| e.node.as_mut())
            .unwrap()
    }
}

impl Default for SceneTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_tree_creation() {
        let tree = SceneTree::new();
        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.root().index(), 0);
    }

    #[test]
    fn test_add_child() {
        let mut tree = SceneTree::new();
        let child = tree.add_2d_node(tree.root(), "child");
        assert_eq!(tree.node_count(), 2);
        assert_eq!(tree.find_by_name("child"), Some(child));
    }

    #[test]
    fn test_remove_child() {
        let mut tree = SceneTree::new();
        let child = tree.add_2d_node(tree.root(), "child");
        tree.remove_child(tree.root(), child);
        assert_eq!(tree.node_count(), 2); // 节点仍在存储中
    }

    #[test]
    fn test_destroy_node() {
        let mut tree = SceneTree::new();
        let child = tree.add_2d_node(tree.root(), "child");
        let _grandchild = tree.add_2d_node(child, "grandchild");
        tree.destroy_node(child);
        assert_eq!(tree.node_count(), 1); // 只有根节点
    }

    #[test]
    fn test_find_by_name() {
        let mut tree = SceneTree::new();
        let _child1 = tree.add_2d_node(tree.root(), "test");
        let child2 = tree.add_2d_node(tree.root(), "test");
        let found = tree.find_by_name("test");
        assert_eq!(found, Some(child2)); // 返回最后一个添加的
    }

    #[test]
    fn test_paused_node_not_updated() {
        let mut tree = SceneTree::new();
        let node = tree.add_2d_node(tree.root(), "test");
        if let Some(node_mut) = tree.get_node_mut(node) {
            node_mut.set_paused(true);
        }
        tree.update(1.0); // 不应该崩溃
    }
}
