//! 场景加载器
//!
//! 提供场景树的序列化和反序列化功能，支持 JSON 格式存储。

use crate::{
    Area2DNode, Body2DNode, Camera2DNode, Node, Node2D, NodeHandle, SceneTree, Sprite2D,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// 场景格式版本
    version: String,
    /// 根节点在 nodes 数组中的索引
    root_index: u32,
    /// 所有节点数据
    nodes: Vec<NodeData>,
}

/// 节点数据（用于序列化）
#[derive(Debug, Serialize, Deserialize)]
struct NodeData {
    /// 节点名称
    name: String,
    /// 父节点在 nodes 数组中的索引（根节点为 None）
    parent: Option<u32>,
    /// 子节点在 nodes 数组中的索引列表
    children: Vec<u32>,
    /// 节点类型标识
    #[serde(rename = "type")]
    node_type: String,
    /// 节点类型特定数据（JSON 值）
    data: serde_json::Value,
}

impl From<&SceneTree> for SceneData {
    fn from(scene: &SceneTree) -> Self {
        // 收集所有节点句柄并建立 handle -> 顺序索引 映射
        let handles: Vec<NodeHandle> = scene.all_handles();
        let handle_to_index: HashMap<u32, u32> = handles
            .iter()
            .enumerate()
            .map(|(idx, &h)| (h.index(), idx as u32))
            .collect();

        let root_seq_index = handle_to_index
            .get(&scene.root().index())
            .copied()
            .unwrap_or(0);

        let mut nodes = Vec::with_capacity(handles.len());

        for &handle in &handles {
            if let Some(node) = scene.get_node(handle) {
                let name = node.name().to_string();
                let parent = node
                    .parent()
                    .and_then(|p| handle_to_index.get(&p.index()).copied());
                let children: Vec<u32> = node
                    .children()
                    .iter()
                    .filter_map(|&c| handle_to_index.get(&c.index()).copied())
                    .collect();
                let node_type = node.node_type().to_string();
                let data = serialize_node_data(node);

                nodes.push(NodeData {
                    name,
                    parent,
                    children,
                    node_type,
                    data,
                });
            }
        }

        Self {
            version: "1.0".to_string(),
            root_index: root_seq_index,
            nodes,
        }
    }
}

impl From<SceneData> for SceneTree {
    fn from(data: SceneData) -> Self {
        let mut scene = SceneTree::new();

        // 第一遍：创建所有节点，建立 顺序索引 -> 新 NodeHandle 映射
        let mut index_to_handle: HashMap<u32, NodeHandle> = HashMap::new();

        for (seq_idx, node_data) in data.nodes.iter().enumerate() {
            let seq_idx_u32 = seq_idx as u32;
            let node = match deserialize_node(&node_data.node_type, &node_data.data) {
                Some(n) => n,
                None => continue,
            };

            if seq_idx_u32 == data.root_index {
                // 替换根节点
                scene.replace_root(node_data.name.clone(), node);
                index_to_handle.insert(seq_idx_u32, scene.root());
            } else {
                // 添加新节点
                let handle = scene.add_node_boxed(node_data.name.clone(), node);
                index_to_handle.insert(seq_idx_u32, handle);
            }
        }

        // 第二遍：修复父子关系
        for (seq_idx, node_data) in data.nodes.iter().enumerate() {
            let seq_idx_u32 = seq_idx as u32;
            let Some(&handle) = index_to_handle.get(&seq_idx_u32) else {
                continue;
            };

            // 设置父节点
            if let Some(parent_seq) = node_data.parent {
                if let Some(&parent_handle) = index_to_handle.get(&parent_seq) {
                    if let Some(node_mut) = scene.get_node_mut(handle) {
                        node_mut.set_parent(Some(parent_handle));
                    }
                }
            } else {
                if let Some(node_mut) = scene.get_node_mut(handle) {
                    node_mut.set_parent(None);
                }
            }

            // 设置子节点
            if let Some(node_mut) = scene.get_node_mut(handle) {
                // 先清空现有子节点（反序列化后的默认状态）
                // 注意：由于 #[serde(skip)]，children 字段在反序列化后是空的
                // 所以直接添加即可
                for &child_seq in &node_data.children {
                    if let Some(&child_handle) = index_to_handle.get(&child_seq) {
                        node_mut.add_child(child_handle);
                    }
                }
            }
        }

        scene
    }
}

/// 序列化节点数据为 JSON Value
fn serialize_node_data(node: &dyn Node) -> serde_json::Value {
    use std::any::Any;

    let any: &dyn Any = node.as_any();
    let node_type = node.node_type();

    match node_type {
        "Node2D" => {
            if let Some(n) = any.downcast_ref::<Node2D>() {
                serde_json::to_value(n).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        "Sprite2D" => {
            if let Some(n) = any.downcast_ref::<Sprite2D>() {
                serde_json::to_value(n).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        "Camera2D" => {
            if let Some(n) = any.downcast_ref::<Camera2DNode>() {
                serde_json::to_value(n).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        "Area2D" => {
            if let Some(n) = any.downcast_ref::<Area2DNode>() {
                serde_json::to_value(n).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        "Body2D" => {
            if let Some(n) = any.downcast_ref::<Body2DNode>() {
                serde_json::to_value(n).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        _ => serde_json::Value::Null,
    }
}

/// 从类型标识和 JSON 数据反序列化节点
fn deserialize_node(node_type: &str, data: &serde_json::Value) -> Option<Box<dyn Node>> {
    match node_type {
        "Node2D" => {
            let node: Node2D = serde_json::from_value(data.clone()).ok()?;
            Some(Box::new(node))
        }
        "Sprite2D" => {
            let node: Sprite2D = serde_json::from_value(data.clone()).ok()?;
            Some(Box::new(node))
        }
        "Camera2D" => {
            let node: Camera2DNode = serde_json::from_value(data.clone()).ok()?;
            Some(Box::new(node))
        }
        "Area2D" => {
            let node: Area2DNode = serde_json::from_value(data.clone()).ok()?;
            Some(Box::new(node))
        }
        "Body2D" => {
            let node: Body2DNode = serde_json::from_value(data.clone()).ok()?;
            Some(Box::new(node))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprite2d::Sprite;
    use engine_math::Vec2;

    #[test]
    fn test_scene_loader_to_json() {
        let scene = SceneTree::new();
        let json = SceneLoader::to_json(&scene).unwrap();

        assert!(json.contains("version"));
        assert!(json.contains("nodes"));
        assert!(json.contains("root_index"));
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
        let path = temp_dir.join("test_scene_loader_save_load.json");

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

    #[test]
    fn test_scene_with_children() {
        let mut scene = SceneTree::new();
        let child1 = scene.add_2d_node(scene.root(), "child1");
        let child2 = scene.add_2d_node(scene.root(), "child2");
        let _grandchild = scene.add_2d_node(child1, "grandchild");

        // 设置一些属性
        if let Some(node) = scene.get_node_mut(child1) {
            if let Some(n) = node.as_any_mut().downcast_mut::<Node2D>() {
                n.set_position(Vec2::new(10.0, 20.0));
                n.set_rotation(1.5);
            }
        }

        let json = SceneLoader::to_json(&scene).unwrap();
        let loaded = SceneLoader::from_json(&json).unwrap();

        assert_eq!(loaded.node_count(), 4);

        // 验证子节点结构
        let loaded_child1 = loaded.find_by_name("child1").expect("child1 should exist");
        let loaded_child2 = loaded.find_by_name("child2").expect("child2 should exist");
        let loaded_grand =
            loaded.find_by_name("grandchild").expect("grandchild should exist");

        // 验证根节点有两个子节点
        let root_node = loaded.get_node(loaded.root()).unwrap();
        assert_eq!(root_node.children().len(), 2);

        // 验证 child1 有一个子节点
        let child1_node = loaded.get_node(loaded_child1).unwrap();
        assert_eq!(child1_node.children().len(), 1);
        assert_eq!(child1_node.children()[0], loaded_grand);

        // 验证 child2 没有子节点
        let child2_node = loaded.get_node(loaded_child2).unwrap();
        assert_eq!(child2_node.children().len(), 0);

        // 验证位置和旋转被保留
        let child1_node = loaded.get_node(loaded_child1).unwrap();
        let pos = child1_node
            .as_any()
            .downcast_ref::<Node2D>()
            .unwrap()
            .position();
        assert_eq!(pos, Vec2::new(10.0, 20.0));
    }

    #[test]
    fn test_scene_with_sprite() {
        let mut scene = SceneTree::new();
        let sprite = Sprite::new(42);
        let sprite_node = Sprite2D::new("test_sprite", sprite);
        let handle = scene.add_node_boxed("test_sprite", Box::new(sprite_node));

        // 设置位置
        if let Some(node) = scene.get_node_mut(handle) {
            if let Some(sprite) = node
                .as_any_mut()
                .downcast_mut::<Sprite2D>()
            {
                sprite.node2d_mut().set_position(Vec2::new(5.0, 5.0));
            }
        }

        let json = SceneLoader::to_json(&scene).unwrap();
        let loaded = SceneLoader::from_json(&json).unwrap();

        assert_eq!(loaded.node_count(), 2);

        let loaded_handle = loaded.find_by_name("test_sprite").expect("sprite should exist");
        let node = loaded.get_node(loaded_handle).unwrap();
        assert_eq!(node.node_type(), "Sprite2D");

        let sprite_node = node.as_any().downcast_ref::<Sprite2D>().unwrap();
        assert_eq!(sprite_node.sprite().texture_id, 42);
        assert_eq!(sprite_node.node2d().position(), Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_scene_with_camera() {
        let mut scene = SceneTree::new();
        let camera_node = Camera2DNode::new("main_camera");
        let handle = scene.add_node_boxed("main_camera", Box::new(camera_node));

        // 设置缩放
        if let Some(node) = scene.get_node_mut(handle) {
            if let Some(cam) = node.as_any_mut().downcast_mut::<Camera2DNode>() {
                cam.camera_mut().set_zoom(2.5);
            }
        }

        let json = SceneLoader::to_json(&scene).unwrap();
        let loaded = SceneLoader::from_json(&json).unwrap();

        let loaded_handle = loaded.find_by_name("main_camera").expect("camera should exist");
        let node = loaded.get_node(loaded_handle).unwrap();
        assert_eq!(node.node_type(), "Camera2D");

        let cam_node = node.as_any().downcast_ref::<Camera2DNode>().unwrap();
        assert!((cam_node.camera().zoom() - 2.5).abs() < 1e-6);
    }

    #[test]
    fn test_scene_preserves_visibility_paused() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");

        if let Some(node) = scene.get_node_mut(child) {
            node.set_visible(false);
            node.set_paused(true);
        }

        let json = SceneLoader::to_json(&scene).unwrap();
        let loaded = SceneLoader::from_json(&json).unwrap();

        let loaded_child = loaded.find_by_name("child").expect("child should exist");
        let node = loaded.get_node(loaded_child).unwrap();
        assert!(!node.visible());
        assert!(node.paused());
    }
}
