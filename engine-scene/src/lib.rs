//! engine-scene crate - 场景树与节点系统
//!
//! 提供游戏场景的层级结构管理，包括节点、变换、相机等功能。

#![warn(missing_docs)]

pub mod area2d;
pub mod body2d;
pub mod camera2d;
pub mod node;
pub mod node2d;
pub mod scene_tree;
pub mod sprite2d;

pub use area2d::Area2D;
pub use body2d::Body2DNode;
pub use camera2d::Camera2DNode;
pub use node::{Node, NodeHandle};
pub use node2d::Node2D;
pub use scene_tree::SceneTree;
pub use sprite2d::Sprite2D;
