//! 场景管理器

use crate::SceneTree;
use std::collections::HashMap;

/// 场景过渡类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transition {
    /// 无过渡
    None,
    /// 淡入淡出
    Fade {
        /// 过渡持续时间（秒）
        duration: f32,
    },
}

/// 场景管理器
pub struct SceneManager {
    scenes: HashMap<String, SceneTree>,
    stack: Vec<String>,
    current: Option<String>,
    transition: Transition,
}

impl std::fmt::Debug for SceneManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SceneManager")
            .field("scenes.len()", &self.scenes.len())
            .field("stack", &self.stack)
            .field("current", &self.current)
            .field("transition", &self.transition)
            .finish()
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    /// 创建新的场景管理器
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            stack: Vec::new(),
            current: None,
            transition: Transition::None,
        }
    }

    /// 加载场景（不切换）
    pub fn load(&mut self, name: &str, scene: SceneTree) {
        self.scenes.insert(name.to_string(), scene);
    }

    /// 切换到指定场景
    pub fn switch_to(&mut self, name: &str) {
        if self.scenes.contains_key(name) {
            self.current = Some(name.to_string());
            self.stack.clear();
            self.stack.push(name.to_string());
        }
    }

    /// 压入新场景（保留当前场景）
    pub fn push(&mut self, name: &str) {
        if self.scenes.contains_key(name) {
            self.stack.push(name.to_string());
            self.current = Some(name.to_string());
        }
    }

    /// 弹出场景
    pub fn pop(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
            self.current = self.stack.last().cloned();
        }
    }

    /// 获取当前场景
    pub fn current(&self) -> Option<&SceneTree> {
        self.current.as_ref().and_then(|name| self.scenes.get(name))
    }

    /// 获取当前场景可变引用
    pub fn current_mut(&mut self) -> Option<&mut SceneTree> {
        if let Some(ref name) = self.current {
            self.scenes.get_mut(name)
        } else {
            None
        }
    }

    /// 设置过渡效果
    pub fn set_transition(&mut self, transition: Transition) {
        self.transition = transition;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_manager_load() {
        let mut manager = SceneManager::new();
        let scene = SceneTree::new();

        manager.load("test", scene);

        assert!(manager.scenes.contains_key("test"));
    }

    #[test]
    fn test_scene_manager_switch_to() {
        let mut manager = SceneManager::new();
        let scene = SceneTree::new();

        manager.load("test", scene);
        manager.switch_to("test");

        // Compare pointers since SceneTree doesn't implement Debug/PartialEq
        let current_ptr = manager.current().map(|s| s as *const _);
        let stored_ptr = manager.scenes.get("test").map(|s| s as *const _);
        assert_eq!(current_ptr, stored_ptr);
    }

    #[test]
    fn test_scene_manager_push_pop() {
        let mut manager = SceneManager::new();
        let scene1 = SceneTree::new();
        let scene2 = SceneTree::new();

        manager.load("scene1", scene1);
        manager.load("scene2", scene2);

        manager.switch_to("scene1");
        manager.push("scene2");

        let current_ptr = manager.current().map(|s| s as *const _);
        let scene2_ptr = manager.scenes.get("scene2").map(|s| s as *const _);
        assert_eq!(current_ptr, scene2_ptr);

        manager.pop();
        let current_ptr = manager.current().map(|s| s as *const _);
        let scene1_ptr = manager.scenes.get("scene1").map(|s| s as *const _);
        assert_eq!(current_ptr, scene1_ptr);
    }

    #[test]
    fn test_scene_manager_transition() {
        let mut manager = SceneManager::new();

        manager.set_transition(Transition::Fade { duration: 1.0 });

        if let Transition::Fade { duration } = manager.transition {
            assert!((duration - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_scene_manager_empty() {
        let manager = SceneManager::new();
        assert!(manager.current().is_none());
    }

    // ============= SceneManager 更多测试 =============

    #[test]
    fn test_scene_manager_new() {
        let manager = SceneManager::new();
        assert!(manager.current().is_none());
    }

    #[test]
    fn test_scene_manager_load_and_switch() {
        let mut manager = SceneManager::new();
        manager.load("scene1", SceneTree::new());
        manager.switch_to("scene1");
        assert!(manager.current().is_some());
    }

    #[test]
    fn test_scene_manager_push_pop_with_names() {
        let mut manager = SceneManager::new();
        manager.load("scene1", SceneTree::new());
        manager.load("scene2", SceneTree::new());
        manager.switch_to("scene1");
        manager.push("scene2");
        manager.pop();
        assert!(manager.current().is_some());
    }

    #[test]
    fn test_scene_manager_pop_empty_stack() {
        let mut manager = SceneManager::new();
        manager.pop();
    }

    #[test]
    fn test_scene_manager_set_transition() {
        let mut manager = SceneManager::new();
        manager.set_transition(Transition::None);
        match manager.transition {
            Transition::None => {}
            _ => panic!("Expected None transition"),
        }
    }

    #[test]
    fn test_scene_manager_switch_to_nonexistent() {
        let mut manager = SceneManager::new();
        manager.switch_to("nonexistent");
        assert!(manager.current().is_none());
    }

    #[test]
    fn test_scene_manager_multiple_switches() {
        let mut manager = SceneManager::new();
        manager.load("scene1", SceneTree::new());
        manager.switch_to("scene1");
        manager.load("scene2", SceneTree::new());
        manager.switch_to("scene2");
        assert!(manager.current().is_some());
    }
}
