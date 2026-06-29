//! 引擎级输入事件类型

use engine_math::Vec2;

use crate::key_code::{KeyCode, ModifiersState, MouseButton};

/// 按键事件
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyEvent {
    /// 按键代码
    pub code: KeyCode,
    /// 当前按键状态（pressed/released）
    pub state: ElementState,
    /// 事件发生时的修饰键
    pub modifiers: ModifiersState,
}

/// 鼠标按钮事件
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseButtonEvent {
    /// 鼠标按钮
    pub button: MouseButton,
    /// 当前按钮状态
    pub state: ElementState,
    /// 事件发生时的修饰键
    pub modifiers: ModifiersState,
}

/// 鼠标移动事件
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotionEvent {
    /// 当前鼠标位置（窗口内坐标）
    pub position: Vec2,
    /// 相对上一帧的位移
    pub delta: Vec2,
}

/// 鼠标滚轮事件
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseWheelEvent {
    /// 滚轮位移
    pub delta: Vec2,
    /// 当前修饰键
    pub modifiers: ModifiersState,
}

/// 文本输入事件
#[derive(Debug, Clone, PartialEq)]
pub struct TextInputEvent {
    /// 输入的文本
    pub text: String,
}

/// 元素状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElementState {
    /// 按下
    #[default]
    Pressed,
    /// 松开
    Released,
}

/// 统一的引擎输入事件
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// 键盘按键事件
    Key(KeyEvent),
    /// 鼠标按钮事件
    MouseButton(MouseButtonEvent),
    /// 鼠标移动事件
    MouseMotion(MouseMotionEvent),
    /// 鼠标滚轮事件
    MouseWheel(MouseWheelEvent),
    /// 文本输入事件
    TextInput(TextInputEvent),
}

/// 光标可见性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorVisibility {
    #[default]
    Visible,
    Hidden,
}

/// 光标捕获模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorGrabMode {
    /// 无捕获
    #[default]
    None,
    /// 限制在窗口内
    Confined,
    /// 锁定光标位置（完全隐藏并捕获）
    Locked,
}

/// 光标图标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorIcon {
    #[default]
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_state_default() {
        assert_eq!(ElementState::default(), ElementState::Pressed);
    }

    #[test]
    fn test_cursor_visibility_default() {
        assert_eq!(CursorVisibility::default(), CursorVisibility::Visible);
    }

    #[test]
    fn test_cursor_grab_mode_default() {
        assert_eq!(CursorGrabMode::default(), CursorGrabMode::None);
    }

    #[test]
    fn test_cursor_icon_default() {
        assert_eq!(CursorIcon::default(), CursorIcon::Default);
    }

    #[test]
    fn test_key_event_creation() {
        let event = KeyEvent {
            code: KeyCode::A,
            state: ElementState::Pressed,
            modifiers: ModifiersState::empty(),
        };
        assert_eq!(event.code, KeyCode::A);
        assert_eq!(event.state, ElementState::Pressed);
    }

    #[test]
    fn test_mouse_button_event_creation() {
        let event = MouseButtonEvent {
            button: MouseButton::Left,
            state: ElementState::Pressed,
            modifiers: ModifiersState::empty(),
        };
        assert_eq!(event.button, MouseButton::Left);
    }

    #[test]
    fn test_input_event_variants() {
        let key = InputEvent::Key(KeyEvent {
            code: KeyCode::Escape,
            state: ElementState::Released,
            modifiers: ModifiersState::empty(),
        });

        let mouse = InputEvent::MouseButton(MouseButtonEvent {
            button: MouseButton::Right,
            state: ElementState::Pressed,
            modifiers: ModifiersState::empty(),
        });

        let motion = InputEvent::MouseMotion(MouseMotionEvent {
            position: Vec2::new(100.0, 200.0),
            delta: Vec2::ZERO,
        });

        match (&key, &mouse, &motion) {
            (InputEvent::Key(_), InputEvent::MouseButton(_), InputEvent::MouseMotion(_)) => {}
            _ => panic!("Unexpected input event variants"),
        }
    }
}
