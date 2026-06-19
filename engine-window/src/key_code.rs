//! 引擎自定义按键枚举与 winit 原生按键映射

/// 引擎级按键枚举（屏蔽 winit 依赖）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum KeyCode {
    // ===== 字母键 =====
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // ===== 数字键 =====
    Digit0, Digit1, Digit2, Digit3, Digit4,
    Digit5, Digit6, Digit7, Digit8, Digit9,

    // ===== 功能键 =====
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10,
    F11, F12, F13, F14, F15, F16, F17, F18, F19, F20,

    // ===== 控制键 =====
    Escape, Enter, Space, Backspace, Tab, CapsLock,
    ShiftLeft, ShiftRight, ControlLeft, ControlRight,
    AltLeft, AltRight, SuperLeft, SuperRight,
    Insert, Delete, Home, End, PageUp, PageDown,

    // ===== 方向键 =====
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,

    // ===== 小键盘 =====
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    NumpadDecimal, NumpadEnter, NumLock,

    // ===== 标点符号 =====
    Grave, Minus, Equal, BracketLeft, BracketRight, Backslash,
    Semicolon, Apostrophe, Comma, Period, Slash,

    // ===== 未知 =====
    Unknown,
}

impl KeyCode {
    pub fn is_alphabetic(&self) -> bool {
        matches!(self,
            KeyCode::A | KeyCode::B | KeyCode::C | KeyCode::D | KeyCode::E | KeyCode::F |
            KeyCode::G | KeyCode::H | KeyCode::I | KeyCode::J | KeyCode::K | KeyCode::L |
            KeyCode::M | KeyCode::N | KeyCode::O | KeyCode::P | KeyCode::Q | KeyCode::R |
            KeyCode::S | KeyCode::T | KeyCode::U | KeyCode::V | KeyCode::W | KeyCode::X |
            KeyCode::Y | KeyCode::Z
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self,
            KeyCode::Digit0 | KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 |
            KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 |
            KeyCode::Digit8 | KeyCode::Digit9
        )
    }

    pub fn is_function(&self) -> bool {
        matches!(self,
            KeyCode::F1 | KeyCode::F2 | KeyCode::F3 | KeyCode::F4 | KeyCode::F5 |
            KeyCode::F6 | KeyCode::F7 | KeyCode::F8 | KeyCode::F9 | KeyCode::F10 |
            KeyCode::F11 | KeyCode::F12 | KeyCode::F13 | KeyCode::F14 | KeyCode::F15 |
            KeyCode::F16 | KeyCode::F17 | KeyCode::F18 | KeyCode::F19 | KeyCode::F20
        )
    }

    pub fn is_arrow(&self) -> bool {
        matches!(self,
            KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::ArrowLeft | KeyCode::ArrowRight
        )
    }

    pub fn is_modifier(&self) -> bool {
        matches!(self,
            KeyCode::ShiftLeft | KeyCode::ShiftRight |
            KeyCode::ControlLeft | KeyCode::ControlRight |
            KeyCode::AltLeft | KeyCode::AltRight |
            KeyCode::SuperLeft | KeyCode::SuperRight
        )
    }
}

impl Default for KeyCode {
    fn default() -> Self {
        KeyCode::Unknown
    }
}

// ===== 鼠标按钮 =====

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    Other(u16),
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

// ===== 修饰键状态 =====

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifiersState {
    bits: u32,
}

impl ModifiersState {
    const SHIFT: u32 = 1 << 0;
    const CONTROL: u32 = 1 << 1;
    const ALT: u32 = 1 << 2;
    const SUPER: u32 = 1 << 3;

    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn shift(&self) -> bool {
        self.bits & Self::SHIFT != 0
    }

    pub fn control(&self) -> bool {
        self.bits & Self::CONTROL != 0
    }

    pub fn alt(&self) -> bool {
        self.bits & Self::ALT != 0
    }

    pub fn super_key(&self) -> bool {
        self.bits & Self::SUPER != 0
    }

    pub fn set_shift(&mut self, on: bool) {
        if on { self.bits |= Self::SHIFT } else { self.bits &= !Self::SHIFT }
    }

    pub fn set_control(&mut self, on: bool) {
        if on { self.bits |= Self::CONTROL } else { self.bits &= !Self::CONTROL }
    }

    pub fn set_alt(&mut self, on: bool) {
        if on { self.bits |= Self::ALT } else { self.bits &= !Self::ALT }
    }

    pub fn set_super(&mut self, on: bool) {
        if on { self.bits |= Self::SUPER } else { self.bits &= !Self::SUPER }
    }

    pub fn or(&self, other: Self) -> Self {
        Self { bits: self.bits | other.bits }
    }
}

// ===== winit -> 引擎 映射函数 =====

/// 从 winit 的 NamedKey 映射到引擎 KeyCode
pub fn map_named_key_to_keycode(named: &winit::keyboard::NamedKey) -> Option<KeyCode> {
    use winit::keyboard::NamedKey as N;
    Some(match named {
        N::Enter => KeyCode::Enter,
        N::Escape => KeyCode::Escape,
        N::Space => KeyCode::Space,
        N::Tab => KeyCode::Tab,
        N::ArrowDown => KeyCode::ArrowDown,
        N::ArrowLeft => KeyCode::ArrowLeft,
        N::ArrowRight => KeyCode::ArrowRight,
        N::ArrowUp => KeyCode::ArrowUp,
        N::End => KeyCode::End,
        N::Home => KeyCode::Home,
        N::PageDown => KeyCode::PageDown,
        N::PageUp => KeyCode::PageUp,
        N::Backspace => KeyCode::Backspace,
        N::Delete => KeyCode::Delete,
        N::Insert => KeyCode::Insert,
        N::F1 => KeyCode::F1, N::F2 => KeyCode::F2,
        N::F3 => KeyCode::F3, N::F4 => KeyCode::F4,
        N::F5 => KeyCode::F5, N::F6 => KeyCode::F6,
        N::F7 => KeyCode::F7, N::F8 => KeyCode::F8,
        N::F9 => KeyCode::F9, N::F10 => KeyCode::F10,
        N::F11 => KeyCode::F11, N::F12 => KeyCode::F12,
        N::F13 => KeyCode::F13, N::F14 => KeyCode::F14,
        N::F15 => KeyCode::F15, N::F16 => KeyCode::F16,
        N::F17 => KeyCode::F17, N::F18 => KeyCode::F18,
        N::F19 => KeyCode::F19, N::F20 => KeyCode::F20,
        N::CapsLock => KeyCode::CapsLock,
        N::Shift => KeyCode::ShiftLeft,
        N::Control => KeyCode::ControlLeft,
        N::Alt => KeyCode::AltLeft,
        N::Super => KeyCode::SuperLeft,
        _ => return None,
    })
}

/// 从 winit 的 KeyCode 映射到引擎 KeyCode（处理字母/数字键）
pub fn map_keycode_to_keycode(winit_code: winit::keyboard::KeyCode) -> KeyCode {
    use winit::keyboard::KeyCode as W;
    match winit_code {
        // 字母
        W::KeyA => KeyCode::A, W::KeyB => KeyCode::B, W::KeyC => KeyCode::C,
        W::KeyD => KeyCode::D, W::KeyE => KeyCode::E, W::KeyF => KeyCode::F,
        W::KeyG => KeyCode::G, W::KeyH => KeyCode::H, W::KeyI => KeyCode::I,
        W::KeyJ => KeyCode::J, W::KeyK => KeyCode::K, W::KeyL => KeyCode::L,
        W::KeyM => KeyCode::M, W::KeyN => KeyCode::N, W::KeyO => KeyCode::O,
        W::KeyP => KeyCode::P, W::KeyQ => KeyCode::Q, W::KeyR => KeyCode::R,
        W::KeyS => KeyCode::S, W::KeyT => KeyCode::T, W::KeyU => KeyCode::U,
        W::KeyV => KeyCode::V, W::KeyW => KeyCode::W, W::KeyX => KeyCode::X,
        W::KeyY => KeyCode::Y, W::KeyZ => KeyCode::Z,

        // 数字
        W::Digit0 => KeyCode::Digit0, W::Digit1 => KeyCode::Digit1,
        W::Digit2 => KeyCode::Digit2, W::Digit3 => KeyCode::Digit3,
        W::Digit4 => KeyCode::Digit4, W::Digit5 => KeyCode::Digit5,
        W::Digit6 => KeyCode::Digit6, W::Digit7 => KeyCode::Digit7,
        W::Digit8 => KeyCode::Digit8, W::Digit9 => KeyCode::Digit9,

        // 控制
        W::Escape => KeyCode::Escape, W::Enter => KeyCode::Enter,
        W::Space => KeyCode::Space, W::Backspace => KeyCode::Backspace,
        W::Tab => KeyCode::Tab, W::CapsLock => KeyCode::CapsLock,
        W::ShiftLeft => KeyCode::ShiftLeft, W::ShiftRight => KeyCode::ShiftRight,
        W::ControlLeft => KeyCode::ControlLeft, W::ControlRight => KeyCode::ControlRight,
        W::AltLeft => KeyCode::AltLeft, W::AltRight => KeyCode::AltRight,
        W::SuperLeft => KeyCode::SuperLeft, W::SuperRight => KeyCode::SuperRight,
        W::Insert => KeyCode::Insert, W::Delete => KeyCode::Delete,
        W::Home => KeyCode::Home, W::End => KeyCode::End,
        W::PageUp => KeyCode::PageUp, W::PageDown => KeyCode::PageDown,

        // 方向
        W::ArrowUp => KeyCode::ArrowUp, W::ArrowDown => KeyCode::ArrowDown,
        W::ArrowLeft => KeyCode::ArrowLeft, W::ArrowRight => KeyCode::ArrowRight,

        // 小键盘
        W::Numpad0 => KeyCode::Numpad0, W::Numpad1 => KeyCode::Numpad1,
        W::Numpad2 => KeyCode::Numpad2, W::Numpad3 => KeyCode::Numpad3,
        W::Numpad4 => KeyCode::Numpad4, W::Numpad5 => KeyCode::Numpad5,
        W::Numpad6 => KeyCode::Numpad6, W::Numpad7 => KeyCode::Numpad7,
        W::Numpad8 => KeyCode::Numpad8, W::Numpad9 => KeyCode::Numpad9,
        W::NumpadAdd => KeyCode::NumpadAdd, W::NumpadSubtract => KeyCode::NumpadSubtract,
        W::NumpadMultiply => KeyCode::NumpadMultiply, W::NumpadDivide => KeyCode::NumpadDivide,
        W::NumpadDecimal => KeyCode::NumpadDecimal, W::NumpadEnter => KeyCode::NumpadEnter,
        W::NumLock => KeyCode::NumLock,

        // 标点
        W::Backquote => KeyCode::Grave, W::Minus => KeyCode::Minus,
        W::Equal => KeyCode::Equal, W::BracketLeft => KeyCode::BracketLeft,
        W::BracketRight => KeyCode::BracketRight, W::Backslash => KeyCode::Backslash,
        W::Semicolon => KeyCode::Semicolon, W::Quote => KeyCode::Apostrophe,
        W::Comma => KeyCode::Comma, W::Period => KeyCode::Period,
        W::Slash => KeyCode::Slash,

        // 其他
        W::F1 => KeyCode::F1, W::F2 => KeyCode::F2, W::F3 => KeyCode::F3,
        W::F4 => KeyCode::F4, W::F5 => KeyCode::F5, W::F6 => KeyCode::F6,
        W::F7 => KeyCode::F7, W::F8 => KeyCode::F8, W::F9 => KeyCode::F9,
        W::F10 => KeyCode::F10, W::F11 => KeyCode::F11, W::F12 => KeyCode::F12,
        W::F13 => KeyCode::F13, W::F14 => KeyCode::F14, W::F15 => KeyCode::F15,
        W::F16 => KeyCode::F16, W::F17 => KeyCode::F17, W::F18 => KeyCode::F18,
        W::F19 => KeyCode::F19, W::F20 => KeyCode::F20,

        _ => KeyCode::Unknown,
    }
}

/// 从 winit 的 MouseButton 映射到引擎 MouseButton
pub fn map_mouse_button(button: winit::event::MouseButton) -> MouseButton {
    match button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
        winit::event::MouseButton::Other(n) => MouseButton::Other(n),
    }
}

/// 从 winit 的 Modifiers 映射到引擎 ModifiersState
pub fn map_modifiers(modifiers: winit::event::Modifiers) -> ModifiersState {
    let mut state = ModifiersState::empty();
    let ms = modifiers.state();
    let raw: u32 = ms.bits() as u32;
    if raw & 0b0001 != 0 { state.bits |= ModifiersState::SHIFT; }
    if raw & 0b0010 != 0 { state.bits |= ModifiersState::CONTROL; }
    if raw & 0b0100 != 0 { state.bits |= ModifiersState::ALT; }
    if raw & 0b1000 != 0 { state.bits |= ModifiersState::SUPER; }
    state
}

/// 从 winit 的 PhysicalKey 提取引擎 KeyCode
pub fn map_physical_key(physical: winit::keyboard::PhysicalKey) -> KeyCode {
    match physical {
        winit::keyboard::PhysicalKey::Code(code) => map_keycode_to_keycode(code),
        winit::keyboard::PhysicalKey::Unidentified(_) => KeyCode::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(KeyCode::default(), KeyCode::Unknown);
        assert_eq!(MouseButton::default(), MouseButton::Left);
        assert!(!ModifiersState::empty().shift());
    }

    #[test]
    fn test_modifiers_set_and_check() {
        let mut m = ModifiersState::empty();
        m.set_shift(true);
        m.set_control(false);
        assert!(m.shift());
        assert!(!m.control());
        m.set_super(true);
        assert!(m.super_key());
    }

    #[test]
    fn test_is_alphabetic() {
        assert!(KeyCode::A.is_alphabetic());
        assert!(KeyCode::Z.is_alphabetic());
        assert!(!KeyCode::Digit1.is_alphabetic());
        assert!(!KeyCode::F1.is_alphabetic());
    }

    #[test]
    fn test_is_numeric_and_function_and_arrow() {
        assert!(KeyCode::Digit0.is_numeric());
        assert!(KeyCode::F1.is_function());
        assert!(KeyCode::ArrowUp.is_arrow());
        assert!(KeyCode::ShiftLeft.is_modifier());
    }

    #[test]
    fn test_map_keycode_letters() {
        assert_eq!(map_keycode_to_keycode(winit::keyboard::KeyCode::KeyA), KeyCode::A);
        assert_eq!(map_keycode_to_keycode(winit::keyboard::KeyCode::KeyZ), KeyCode::Z);
    }

    #[test]
    fn test_map_keycode_digits_and_arrows() {
        assert_eq!(map_keycode_to_keycode(winit::keyboard::KeyCode::Digit0), KeyCode::Digit0);
        assert_eq!(map_keycode_to_keycode(winit::keyboard::KeyCode::ArrowUp), KeyCode::ArrowUp);
    }

    #[test]
    fn test_map_mouse_button() {
        assert_eq!(map_mouse_button(winit::event::MouseButton::Left), MouseButton::Left);
        assert_eq!(map_mouse_button(winit::event::MouseButton::Right), MouseButton::Right);
        assert_eq!(map_mouse_button(winit::event::MouseButton::Middle), MouseButton::Middle);
    }

    #[test]
    fn test_map_physical_key() {
        let physical = winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA);
        assert_eq!(map_physical_key(physical), KeyCode::A);
    }

    #[test]
    fn test_map_named_key_basic() {
        // NamedKey 可能匹配不上，测试通过 None 路径即可
        let result = map_named_key_to_keycode(&winit::keyboard::NamedKey::Escape);
        assert_eq!(result, Some(KeyCode::Escape));
    }

    #[test]
    fn test_map_modifiers_default_empty() {
        let m = map_modifiers(Default::default());
        assert!(!m.shift());
        assert!(!m.control());
    }
}
