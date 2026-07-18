//! 游戏手柄输入支持 — Sprint 02 存根，后续 Sprint 完整实现

/// 游戏手柄按钮
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    South, // A / Cross
    East,  // B / Circle
    West,  // X / Square
    North, // Y / Triangle
    Start,
    Select,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

/// 游戏手柄轴
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

/// 游戏手柄事件
#[derive(Debug, Clone, PartialEq)]
pub enum GamepadEvent {
    ButtonPressed { button: GamepadButton },
    ButtonReleased { button: GamepadButton },
    AxisChanged { axis: GamepadAxis, value: f32 },
    Connected { id: usize },
    Disconnected { id: usize },
}

/// 游戏手柄状态（存根）
#[derive(Debug, Clone, Default)]
pub struct GamepadState {
    connected: bool,
}

impl GamepadState {
    pub fn new() -> Self {
        Self { connected: false }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// 处理游戏手柄事件（存根 — 后续 Sprint 基于 gilrs 实现）
    pub fn process_event(&mut self, _event: &GamepadEvent) {
        // TODO: Sprint 05 完整实现
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamepad_state_default() {
        let state = GamepadState::default();
        assert!(!state.is_connected());
    }

    #[test]
    fn test_gamepad_event_variants() {
        let events = vec![
            GamepadEvent::ButtonPressed {
                button: GamepadButton::South,
            },
            GamepadEvent::AxisChanged {
                axis: GamepadAxis::LeftStickX,
                value: 0.5,
            },
            GamepadEvent::Connected { id: 0 },
        ];
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_gamepad_button_equality() {
        assert_eq!(GamepadButton::South, GamepadButton::South);
        assert_ne!(GamepadButton::South, GamepadButton::East);
    }
}
