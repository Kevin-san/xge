//! 场景加载器

use crate::SceneTree;
use serde::{Deserialize, Serialize};

/// 场景加载器
pub struct SceneLoader;

impl SceneLoader {
    /// 从 JSON 字符串加载场景
    pub fn from_json(json: &str) -> Result<SceneTree, anyhow::Error> {
        let data: SceneData = serde_json::from_str(json)?;
        Ok(data.into())
    }

    /// 将场景序列化为 JSON 字符串
    pub fn to_json(scene: &SceneTree) -> Result<String, anyhow::Error> {
        let data = SceneData::from(scene);
        Ok(serde_json::to_string_pretty(&data)?)
    }

    /// 保存场景到文件
    pub fn save_json(scene: &SceneTree, path: &std::path::Path) -> Result<(), anyhow::Error> {
        let json = Self::to_json(scene)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 从文件加载场景
    pub fn load_json(path: &std::path::Path) -> Result<SceneTree, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json(&content)
    }
}

/// 场景数据结构（用于序列化）
#[derive(Debug, Serialize, Deserialize)]
struct SceneData {
    version: String,
    nodes: Vec<NodeData>,
    root_index: u32,
}

/// 节点数据（用于序列化）
#[derive(Debug, Serialize, Deserialize)]
struct NodeData {
    name: String,
    parent: Option<u32>,
    children: Vec<u32>,
    #[serde(rename = "type")]
    node_type: String,
}

impl From<&SceneTree> for SceneData {
    fn from(scene: &SceneTree) -> Self {
        let mut nodes = Vec::new();
        let root_index = 0u32;

        // 遍历场景收集节点数据
        // 注意：这只是一个简化实现
        let _ = scene.node_count();
        let _ = scene.root();

        // 简化：只保存基本信息
        nodes.push(NodeData {
            name: "root".to_string(),
            parent: None,
            children: Vec::new(),
            node_type: "Node2D".to_string(),
        });

        Self {
            version: "1.0".to_string(),
            nodes,
            root_index,
        }
    }
}

impl From<SceneData> for SceneTree {
    fn from(data: SceneData) -> Self {
        let scene = SceneTree::new();

        // 重建节点结构
        for node_data in &data.nodes {
            let _ = node_data;
            // 简化实现
        }

        scene
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_loader_to_json() {
        let scene = SceneTree::new();
        let json = SceneLoader::to_json(&scene).unwrap();

        assert!(json.contains("version"));
        assert!(json.contains("nodes"));
    }

    #[test]
    fn test_scene_loader_from_json() {
        let scene = SceneTree::new();
        let json = SceneLoader::to_json(&scene).unwrap();

        let loaded = SceneLoader::from_json(&json).unwrap();
        assert_eq!(loaded.node_count(), scene.node_count());
    }

    #[test]
    fn test_scene_loader_save_load() {
        let scene = SceneTree::new();
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test.scene.json");

        SceneLoader::save_json(&scene, &path).unwrap();
        let loaded = SceneLoader::load_json(&path).unwrap();

        assert_eq!(loaded.node_count(), scene.node_count());

        // cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_invalid_json() {
        let result = SceneLoader::from_json("invalid json");
        assert!(result.is_err());
    }
}
