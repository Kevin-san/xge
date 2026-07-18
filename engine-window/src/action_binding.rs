//! 动作绑定系统 — 将物理输入映射到逻辑动作

use crate::{Input, KeyCode, MouseButton};
use std::collections::HashMap;

/// 动作绑定错误
#[derive(Debug, Clone)]
pub enum BindingError {
    DuplicateBinding(String),
}

impl std::fmt::Display for BindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateBinding(action) => write!(f, "重复绑定: {}", action),
        }
    }
}

/// 输入源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSource {
    Key(KeyCode),
    MouseButton(MouseButton),
}

/// 动作绑定映射
pub struct ActionBindings {
    /// 动作名 → 输入源列表
    bindings: HashMap<String, Vec<InputSource>>,
}

impl Default for ActionBindings {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBindings {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// 绑定动作到输入源
    pub fn bind(&mut self, action: &str, source: InputSource) {
        self.bindings
            .entry(action.to_string())
            .or_default()
            .push(source);
    }

    /// 解绑动作的所有输入源
    pub fn unbind(&mut self, action: &str) {
        self.bindings.remove(action);
    }

    /// 清除所有绑定
    pub fn clear(&mut self) {
        self.bindings.clear();
    }

    /// 检查动作是否被触发（按下或刚按下）
    pub fn action_pressed(&self, action: &str, input: &Input) -> bool {
        self.bindings
            .get(action)
            .map(|sources| {
                sources.iter().any(|source| match source {
                    InputSource::Key(code) => input.key_pressed(*code),
                    InputSource::MouseButton(btn) => input.mouse_button_pressed(*btn),
                })
            })
            .unwrap_or(false)
    }

    /// 检查动作是否刚被触发（仅刚按下）
    pub fn action_just_pressed(&self, action: &str, input: &Input) -> bool {
        self.bindings
            .get(action)
            .map(|sources| {
                sources.iter().any(|source| match source {
                    InputSource::Key(code) => input.key_just_pressed(*code),
                    InputSource::MouseButton(btn) => input.mouse_button_just_pressed(*btn),
                })
            })
            .unwrap_or(false)
    }

    /// 检查动作是否刚被释放
    pub fn action_just_released(&self, action: &str, input: &Input) -> bool {
        self.bindings
            .get(action)
            .map(|sources| {
                sources.iter().any(|source| match source {
                    InputSource::Key(code) => input.key_just_released(*code),
                    InputSource::MouseButton(btn) => input.mouse_button_just_released(*btn),
                })
            })
            .unwrap_or(false)
    }

    /// 获取动作的所有绑定
    pub fn get_bindings(&self, action: &str) -> &[InputSource] {
        self.bindings
            .get(action)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// 获取所有已注册的动作名
    pub fn actions(&self) -> impl Iterator<Item = &str> {
        self.bindings.keys().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ElementState;

    #[test]
    fn test_bind_and_query() {
        let mut bindings = ActionBindings::new();
        bindings.bind("jump", InputSource::Key(KeyCode::Space));
        bindings.bind("shoot", InputSource::MouseButton(MouseButton::Left));

        let mut input = Input::new();
        input.update_key(KeyCode::Space, ElementState::Pressed);

        assert!(bindings.action_pressed("jump", &input));
        assert!(bindings.action_just_pressed("jump", &input));
        assert!(!bindings.action_pressed("shoot", &input));
    }

    #[test]
    fn test_multiple_bindings() {
        let mut bindings = ActionBindings::new();
        bindings.bind("jump", InputSource::Key(KeyCode::Space));
        bindings.bind("jump", InputSource::Key(KeyCode::W));

        let mut input = Input::new();
        input.update_key(KeyCode::W, ElementState::Pressed);

        assert!(bindings.action_pressed("jump", &input));
    }

    #[test]
    fn test_unbind() {
        let mut bindings = ActionBindings::new();
        bindings.bind("jump", InputSource::Key(KeyCode::Space));
        bindings.unbind("jump");

        let mut input = Input::new();
        input.update_key(KeyCode::Space, ElementState::Pressed);

        assert!(!bindings.action_pressed("jump", &input));
    }

    #[test]
    fn test_unknown_action() {
        let bindings = ActionBindings::new();
        let input = Input::new();
        assert!(!bindings.action_pressed("unknown", &input));
        assert!(bindings.get_bindings("unknown").is_empty());
    }

    #[test]
    fn test_actions_iterator() {
        let mut bindings = ActionBindings::new();
        bindings.bind("jump", InputSource::Key(KeyCode::Space));
        bindings.bind("shoot", InputSource::MouseButton(MouseButton::Left));

        let actions: Vec<_> = bindings.actions().collect();
        assert_eq!(actions.len(), 2);
        assert!(actions.contains(&"jump"));
        assert!(actions.contains(&"shoot"));
    }

    #[test]
    fn test_just_released() {
        let mut bindings = ActionBindings::new();
        bindings.bind("jump", InputSource::Key(KeyCode::Space));

        let mut input = Input::new();
        input.update_key(KeyCode::Space, ElementState::Pressed);
        assert!(bindings.action_just_pressed("jump", &input));

        input.update_key(KeyCode::Space, ElementState::Released);
        assert!(bindings.action_just_released("jump", &input));
    }
}
