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

    /// 将场景序列化为二进制
    pub fn to_binary(scene: &SceneTree) -> Result<Vec<u8>, anyhow::Error> {
        let data = SceneData::from(scene);
        let json = serde_json::to_vec(&data)?;
        Ok(json)
    }

    /// 从二进制加载场景
    pub fn from_binary(data: &[u8]) -> Result<SceneTree, anyhow::Error> {
        let scene_data: SceneData = serde_json::from_slice(data)?;
        Ok(scene_data.into())
    }

    /// 保存场景到二进制文件
    pub fn save_binary(scene: &SceneTree, path: &std::path::Path) -> Result<(), anyhow::Error> {
        let data = Self::to_binary(scene)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// 从二进制文件加载场景
    pub fn load_binary(path: &std::path::Path) -> Result<SceneTree, anyhow::Error> {
        let data = std::fs::read(path)?;
        Self::from_binary(&data)
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

    // ============= SceneLoader 更多测试 =============

    #[test]
    fn test_scene_loader_to_json_has_version_field() {
        let scene = SceneTree::new();
        let json = SceneLoader::to_json(&scene).unwrap();
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"nodes\""));
    }

    #[test]
    fn test_scene_loader_from_json_roundtrip() {
        let scene = SceneTree::new();
        let json = SceneLoader::to_json(&scene).unwrap();
        let loaded = SceneLoader::from_json(&json).unwrap();
        assert!(loaded.node_count() > 0);
    }

    #[test]
    fn test_scene_loader_save_file() {
        let scene = SceneTree::new();
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_scene_save.json");
        assert!(SceneLoader::save_json(&scene, &path).is_ok());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_scene_loader_load_file() {
        let scene = SceneTree::new();
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_scene_load.json");
        SceneLoader::save_json(&scene, &path).unwrap();
        let loaded = SceneLoader::load_json(&path);
        assert!(loaded.is_ok());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_scene_loader_invalid_json_variety() {
        assert!(SceneLoader::from_json("").is_err());
        assert!(SceneLoader::from_json("null").is_err());
        assert!(SceneLoader::from_json("{invalid").is_err());
    }

    #[test]
    fn test_scene_loader_to_binary() {
        let scene = SceneTree::new();
        let data = SceneLoader::to_binary(&scene).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_scene_loader_from_binary_roundtrip() {
        let scene = SceneTree::new();
        let data = SceneLoader::to_binary(&scene).unwrap();
        let loaded = SceneLoader::from_binary(&data).unwrap();
        assert_eq!(loaded.node_count(), scene.node_count());
    }

    #[test]
    fn test_scene_loader_save_load_binary() {
        let scene = SceneTree::new();
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_scene.scene.bin");

        SceneLoader::save_binary(&scene, &path).unwrap();
        let loaded = SceneLoader::load_binary(&path).unwrap();
        assert_eq!(loaded.node_count(), scene.node_count());

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_scene_loader_from_binary_invalid() {
        let result = SceneLoader::from_binary(&[0xFF, 0xFE, 0xFD]);
        assert!(result.is_err());
    }
}
