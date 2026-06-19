//! Prefab 预制体系统
//!
//! 提供场景子树的序列化、反序列化和实例化功能。

use crate::{
    Area2DNode, Body2DNode, Camera2DNode, Node, Node2D, NodeHandle, SceneTree, Sprite2D,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 预制体
///
/// 表示一个可复用的场景子树，可以序列化为 JSON 并实例化到场景中。
pub struct Prefab {
    /// 预制体数据
    data: PrefabData,
}

/// 预制体数据（用于序列化）
#[derive(Debug, Serialize, Deserialize)]
struct PrefabData {
    /// 格式版本
    version: String,
    /// 根节点在 nodes 数组中的索引
    root_index: u32,
    /// 所有节点数据
    nodes: Vec<PrefabNodeData>,
}

/// 预制体节点数据
#[derive(Debug, Serialize, Deserialize)]
struct PrefabNodeData {
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

impl Prefab {
    /// 从场景创建预制体
    ///
    /// 收集以 `root` 为根的子树中的所有节点。
    pub fn from_scene(scene: &SceneTree, root: NodeHandle) -> Self {
        // 收集子树中所有节点的句柄
        let mut handles = Vec::new();
        Self::collect_subtree_handles(scene, root, &mut handles);

        // 建立 handle -> 顺序索引 映射
        let handle_to_index: HashMap<u32, u32> = handles
            .iter()
            .enumerate()
            .map(|(idx, &h)| (h.index(), idx as u32))
            .collect();

        let root_seq_index = handle_to_index
            .get(&root.index())
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

                nodes.push(PrefabNodeData {
                    name,
                    parent,
                    children,
                    node_type,
                    data,
                });
            }
        }

        Self {
            data: PrefabData {
                version: "1.0".to_string(),
                root_index: root_seq_index,
                nodes,
            },
        }
    }

    /// 递归收集子树中所有节点的句柄
    fn collect_subtree_handles(
        scene: &SceneTree,
        handle: NodeHandle,
        handles: &mut Vec<NodeHandle>,
    ) {
        handles.push(handle);
        if let Some(node) = scene.get_node(handle) {
            let children: Vec<NodeHandle> = node.children().to_vec();
            for child in children {
                Self::collect_subtree_handles(scene, child, handles);
            }
        }
    }

    /// 实例化预制体
    ///
    /// 创建预制体中所有节点的副本，返回 (根句柄, 节点列表)。
    /// 节点列表中的索引对应预制体数据中的顺序。
    pub fn instantiate(&self) -> (NodeHandle, Vec<Box<dyn Node>>) {
        let mut nodes: Vec<Option<Box<dyn Node>>> = Vec::with_capacity(self.data.nodes.len());
        let mut index_to_pos: HashMap<u32, usize> = HashMap::new();

        // 创建所有节点
        for (idx, node_data) in self.data.nodes.iter().enumerate() {
            let node = deserialize_node(&node_data.node_type, &node_data.data);
            index_to_pos.insert(idx as u32, idx);
            nodes.push(node);
        }

        // 修复父子关系
        for (idx, node_data) in self.data.nodes.iter().enumerate() {
            // 设置父节点
            if let Some(ref mut node) = nodes[idx] {
                if let Some(parent_seq) = node_data.parent {
                    // 父节点句柄使用顺序索引（在实例化后的列表中）
                    node.set_parent(Some(NodeHandle::new(parent_seq)));
                } else {
                    node.set_parent(None);
                }

                // 设置子节点
                for &child_seq in &node_data.children {
                    node.add_child(NodeHandle::new(child_seq));
                }
            }
        }

        // 收集非空节点
        let result: Vec<Box<dyn Node>> = nodes.into_iter().flatten().collect();
        let root = NodeHandle::new(self.data.root_index);

        (root, result)
    }

    /// 实例化预制体到场景树
    ///
    /// 将预制体中的所有节点添加到场景树中，返回新根节点的句柄。
    pub fn instantiate_into(&self, scene: &mut SceneTree, parent: NodeHandle) -> NodeHandle {
        let mut handle_map: HashMap<u32, NodeHandle> = HashMap::new();
        let mut new_root = None;

        // 第一遍：创建所有节点
        for (idx, node_data) in self.data.nodes.iter().enumerate() {
            let seq_idx = idx as u32;
            let Some(node) = deserialize_node(&node_data.node_type, &node_data.data) else {
                continue;
            };

            if seq_idx == self.data.root_index {
                // 根节点添加到指定父节点下
                let handle = scene.add_node_boxed(node_data.name.clone(), node);
                scene.add_child(parent, handle);
                handle_map.insert(seq_idx, handle);
                new_root = Some(handle);
            } else {
                let handle = scene.add_node_boxed(node_data.name.clone(), node);
                handle_map.insert(seq_idx, handle);
            }
        }

        // 第二遍：修复父子关系
        for (idx, node_data) in self.data.nodes.iter().enumerate() {
            let seq_idx = idx as u32;
            let Some(&handle) = handle_map.get(&seq_idx) else {
                continue;
            };

            // 设置父节点（根节点的父节点已经设置为 parent）
            if seq_idx != self.data.root_index {
                if let Some(parent_seq) = node_data.parent {
                    if let Some(&parent_handle) = handle_map.get(&parent_seq) {
                        if let Some(node_mut) = scene.get_node_mut(handle) {
                            node_mut.set_parent(Some(parent_handle));
                        }
                    }
                }
            }

            // 设置子节点
            if let Some(node_mut) = scene.get_node_mut(handle) {
                for &child_seq in &node_data.children {
                    if let Some(&child_handle) = handle_map.get(&child_seq) {
                        node_mut.add_child(child_handle);
                    }
                }
            }
        }

        new_root.unwrap_or(parent)
    }

    /// 保存为 JSON 文件
    pub fn save_json(&self, path: &std::path::Path) -> Result<(), anyhow::Error> {
        let json = serde_json::to_string_pretty(&self.data)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 从 JSON 文件加载
    pub fn load_json(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let data: PrefabData = serde_json::from_str(&content)?;
        Ok(Self { data })
    }

    /// 序列化为 JSON 字符串
    pub fn to_json_string(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(&self.data)?)
    }

    /// 从 JSON 字符串加载
    pub fn from_json_string(json: &str) -> Result<Self, anyhow::Error> {
        let data: PrefabData = serde_json::from_str(json)?;
        Ok(Self { data })
    }

    /// 获取节点数量
    pub fn node_count(&self) -> usize {
        self.data.nodes.len()
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
    fn test_prefab_from_scene() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");
        let _grandchild = scene.add_2d_node(child, "grandchild");

        let prefab = Prefab::from_scene(&scene, scene.root());
        assert_eq!(prefab.node_count(), 3);
    }

    #[test]
    fn test_prefab_from_subtree() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");
        let _grandchild = scene.add_2d_node(child, "grandchild");
        let _other = scene.add_2d_node(scene.root(), "other");

        // 只收集 child 子树
        let prefab = Prefab::from_scene(&scene, child);
        assert_eq!(prefab.node_count(), 2); // child + grandchild
    }

    #[test]
    fn test_prefab_instantiate() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");
        let _grandchild = scene.add_2d_node(child, "grandchild");

        let prefab = Prefab::from_scene(&scene, scene.root());
        let (root, nodes) = prefab.instantiate();

        assert!(!root.is_null());
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_prefab_instantiate_into() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");
        let _grandchild = scene.add_2d_node(child, "grandchild");

        let prefab = Prefab::from_scene(&scene, child);

        // 创建另一个场景并实例化
        let mut target_scene = SceneTree::new();
        let target_parent = target_scene.root();
        let original_count = target_scene.node_count();

        let new_root = prefab.instantiate_into(&mut target_scene, target_parent);

        assert!(!new_root.is_null());
        assert_eq!(target_scene.node_count(), original_count + 2);

        // 验证新节点存在
        let node = target_scene.get_node(new_root).unwrap();
        assert_eq!(node.name(), "child");
    }

    #[test]
    fn test_prefab_save_load_json() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");
        let _grandchild = scene.add_2d_node(child, "grandchild");

        let prefab = Prefab::from_scene(&scene, scene.root());

        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_prefab_save_load.json");
        prefab.save_json(&path).unwrap();

        let loaded = Prefab::load_json(&path).unwrap();
        assert_eq!(loaded.node_count(), prefab.node_count());

        // cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_prefab_to_from_json_string() {
        let mut scene = SceneTree::new();
        let child = scene.add_2d_node(scene.root(), "child");

        let prefab = Prefab::from_scene(&scene, scene.root());
        let json = prefab.to_json_string().unwrap();

        let loaded = Prefab::from_json_string(&json).unwrap();
        assert_eq!(loaded.node_count(), prefab.node_count());
    }

    #[test]
    fn test_prefab_preserves_sprite_data() {
        let mut scene = SceneTree::new();
        let sprite = Sprite::new(99);
        let sprite_node = Sprite2D::new("test_sprite", sprite);
        let handle = scene.add_node_boxed("test_sprite", Box::new(sprite_node));

        // 设置位置
        if let Some(node) = scene.get_node_mut(handle) {
            if let Some(s) = node.as_any_mut().downcast_mut::<Sprite2D>() {
                s.node2d_mut().set_position(Vec2::new(3.0, 4.0));
            }
        }

        let prefab = Prefab::from_scene(&scene, handle);
        let json = prefab.to_json_string().unwrap();
        let loaded = Prefab::from_json_string(&json).unwrap();

        let (root, nodes) = loaded.instantiate();
        assert_eq!(nodes.len(), 1);

        // 验证 sprite 数据
        let node = &nodes[0];
        assert_eq!(node.name(), "test_sprite");
        assert_eq!(node.node_type(), "Sprite2D");

        let sprite_node = node.as_any().downcast_ref::<Sprite2D>().unwrap();
        assert_eq!(sprite_node.sprite().texture_id, 99);
        assert_eq!(sprite_node.node2d().position(), Vec2::new(3.0, 4.0));

        let _ = root;
    }
}
