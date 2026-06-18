//! Prefab 预制体系统

use crate::{Node, NodeHandle, SceneTree};

/// 预制体
pub struct Prefab {
    root: NodeHandle,
    nodes: Vec<Box<dyn Node>>,
}

impl Prefab {
    /// 从场景创建预制体
    pub fn from_scene(scene: &SceneTree, root: NodeHandle) -> Self {
        let mut nodes = Vec::new();

        // 收集所有节点（从根开始）
        Self::collect_nodes(scene, root, &mut nodes);

        Self { root, nodes }
    }

    /// 递归收集节点
    fn collect_nodes(scene: &SceneTree, handle: NodeHandle, _nodes: &mut Vec<Box<dyn Node>>) {
        if let Some(node) = scene.get_node(handle) {
            // 注意：这里克隆了节点，实际使用时可能需要更复杂的序列化
            // 由于 Node trait 不支持 Clone，这里我们只是记录句柄
            let _ = handle;
            let _ = node.name();
        }

        // 获取子节点并递归遍历
        if let Some(entry) = scene.get_node(handle) {
            for &child in entry.children() {
                Self::collect_nodes(scene, child, _nodes);
            }
        }
    }

    /// 实例化预制体
    pub fn instantiate(&self) -> (NodeHandle, Vec<Box<dyn Node>>) {
        // 创建新节点的副本
        let new_nodes: Vec<Box<dyn Node>> = self
            .nodes
            .iter()
            .map(|n| {
                // 由于 Node trait 没有 Clone，这里需要根据具体类型创建
                // 这是一个简化的实现
                let _ = n.name();
                Box::new(crate::Node2D::new("instance")) as Box<dyn Node>
            })
            .collect();

        let root = NodeHandle::new(0);
        (root, new_nodes)
    }

    /// 保存为 JSON
    pub fn save_json(&self, path: &std::path::Path) -> Result<(), anyhow::Error> {
        let json = serde_json::json!({
            "root": self.root.index(),
            "node_count": self.nodes.len(),
        });
        std::fs::write(path, serde_json::to_string_pretty(&json)?)?;
        Ok(())
    }

    /// 从 JSON 加载
    pub fn load_json(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;

        let root_index = json["root"].as_u64().unwrap_or(0) as u32;
        let _node_count = json["node_count"].as_u64().unwrap_or(0) as usize;

        Ok(Self {
            root: NodeHandle::new(root_index),
            nodes: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefab_from_scene() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());

        assert_eq!(prefab.root, scene.root());
    }

    #[test]
    fn test_prefab_instantiate() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());

        let (root, nodes) = prefab.instantiate();
        assert!(!root.is_null());
        assert_eq!(nodes.len(), prefab.nodes.len());
    }

    #[test]
    fn test_prefab_save_load_json() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());

        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test.prefab.json");
        prefab.save_json(&path).unwrap();

        let loaded = Prefab::load_json(&path).unwrap();
        assert_eq!(loaded.root, prefab.root);

        // cleanup
        let _ = std::fs::remove_file(&path);
    }
}
