//! Prefab 预制体系统

use crate::{Node, NodeHandle, SceneTree};

/// 预制体序列化数据
#[derive(serde::Serialize, serde::Deserialize)]
struct PrefabData {
    name: String,
    root_index: u32,
    node_count: usize,
}

/// 预制体
pub struct Prefab {
    name: String,
    root: NodeHandle,
    nodes: Vec<Box<dyn Node>>,
}

impl Prefab {
    /// 创建新的预制体
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: NodeHandle::new(0),
            nodes: Vec::new(),
        }
    }

    /// 获取预制体名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 从场景创建预制体
    pub fn from_scene(scene: &SceneTree, root: NodeHandle) -> Self {
        let mut nodes = Vec::new();

        // 收集所有节点（从根开始）
        Self::collect_nodes(scene, root, &mut nodes);

        let name = scene
            .get_node(root)
            .map(|n| n.name().to_string())
            .unwrap_or_default();

        Self {
            name,
            root,
            nodes,
        }
    }

    /// 递归收集节点
    fn collect_nodes(scene: &SceneTree, handle: NodeHandle, nodes: &mut Vec<Box<dyn Node>>) {
        if let Some(node) = scene.get_node(handle) {
            // 记录节点名称并创建Node2D副本
            let name = node.name().to_string();
            nodes.push(Box::new(crate::Node2D::new(name)) as Box<dyn Node>);
        }

        // 获取子节点并递归遍历
        if let Some(entry) = scene.get_node(handle) {
            for &child in entry.children() {
                Self::collect_nodes(scene, child, nodes);
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
            name: String::new(),
            root: NodeHandle::new(root_index),
            nodes: Vec::new(),
        })
    }

    /// 在指定场景树中实例化
    pub fn instantiate_in(&self, tree: &mut crate::SceneTree, parent: crate::NodeHandle) -> crate::NodeHandle {
        tree.add_2d_node(parent, &self.name)
    }

    /// 保存预制件到二进制数据
    pub fn save_bin(&self) -> Result<Vec<u8>, anyhow::Error> {
        let data = PrefabData {
            name: self.name.clone(),
            root_index: self.root.index(),
            node_count: self.nodes.len(),
        };
        let json = serde_json::to_vec(&data)?;
        Ok(json)
    }

    /// 从二进制数据加载预制件
    pub fn load_bin(data: &[u8]) -> Result<Self, anyhow::Error> {
        let prefab_data: PrefabData = serde_json::from_slice(data)?;
        Ok(Self {
            name: prefab_data.name,
            root: NodeHandle::new(prefab_data.root_index),
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

    // ============= Prefab 更多测试 =============

    #[test]
    fn test_prefab_instantiate_root_valid() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());
        let (root, _nodes) = prefab.instantiate();
        assert!(!root.is_null());
    }

    #[test]
    fn test_prefab_save_json() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_prefab_save.json");
        let result = prefab.save_json(&path);
        assert!(result.is_ok());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_prefab_load_json() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_prefab_load.json");
        prefab.save_json(&path).unwrap();
        let loaded = Prefab::load_json(&path);
        assert!(loaded.is_ok());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_prefab_instantiate_multiple() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());
        let (r1, _) = prefab.instantiate();
        let (r2, _) = prefab.instantiate();
        assert!(!r1.is_null());
        assert!(!r2.is_null());
    }

    #[test]
    fn test_prefab_instantiate_returns_nodes() {
        let scene = SceneTree::new();
        let prefab = Prefab::from_scene(&scene, scene.root());
        let (_root, nodes) = prefab.instantiate();
        // 至少验证返回的节点列表不为空
        assert!(!nodes.is_empty());
    }

    #[test]
    fn test_prefab_instantiate_in() {
        let mut tree = crate::SceneTree::new();
        let root = tree.root();
        let prefab = Prefab::new("test_prefab");
        let handle = prefab.instantiate_in(&mut tree, root);
        assert!(tree.get_node(handle).is_some());
    }

    #[test]
    fn test_prefab_save_bin_load_bin() {
        let prefab = Prefab::new("test_binary");
        let data = prefab.save_bin().unwrap();
        let loaded = Prefab::load_bin(&data).unwrap();
        assert_eq!(loaded.name(), "test_binary");
    }
}
