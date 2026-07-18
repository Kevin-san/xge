//! engine-window crate — 窗口系统、事件循环与输入抽象
//!
//! 提供跨平台窗口管理、输入事件处理和剪贴板访问

pub mod action_binding;
pub mod clipboard;
pub mod input_event;
pub mod key_code;
pub mod window_state;

use engine_math::Vec2;
use std::collections::HashMap;

// ===== 公开类型重导出 =====

// 窗口相关：保留 winit 原始类型以便底层访问
pub use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position};
pub use winit::event::{DeviceEvent, Event, WindowEvent};
pub use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
pub use winit::monitor::{MonitorHandle, VideoMode};
pub use winit::window::{Fullscreen, Icon, Window, WindowLevel};

// 引擎级按键枚举（屏蔽 winit 依赖）
pub use key_code::{KeyCode, ModifiersState, MouseButton};

// 引擎级输入事件
pub use input_event::{
    CursorGrabMode, CursorIcon, CursorVisibility, ElementState, InputEvent, KeyEvent,
    MouseButtonEvent, MouseMotionEvent, MouseWheelEvent, TextInputEvent, TouchInputEvent,
    TouchPhase,
};

// 窗口状态
pub use window_state::{WindowSize, WindowState};

// 剪贴板错误
pub use crate::clipboard::ClipboardError;

// 动作绑定
pub use action_binding::{ActionBindings, InputSource};

// ===== 引擎级窗口 API（屏蔽 winit 依赖）=====

/// 引擎级光标捕获模式 → winit 映射
pub fn map_cursor_grab_mode(mode: CursorGrabMode) -> winit::window::CursorGrabMode {
    match mode {
        CursorGrabMode::None => winit::window::CursorGrabMode::None,
        CursorGrabMode::Confined => winit::window::CursorGrabMode::Confined,
        CursorGrabMode::Locked => winit::window::CursorGrabMode::Locked,
    }
}

/// 引擎级光标图标 → winit 映射
pub fn map_cursor_icon(icon: CursorIcon) -> winit::window::CursorIcon {
    match icon {
        CursorIcon::Default => winit::window::CursorIcon::Default,
        CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
        CursorIcon::Hand => winit::window::CursorIcon::Pointer,
        CursorIcon::Arrow => winit::window::CursorIcon::Default,
        CursorIcon::Move => winit::window::CursorIcon::Move,
        CursorIcon::Text => winit::window::CursorIcon::Text,
        CursorIcon::Wait => winit::window::CursorIcon::Wait,
        CursorIcon::Help => winit::window::CursorIcon::Help,
        CursorIcon::Progress => winit::window::CursorIcon::Progress,
        CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
        CursorIcon::ContextMenu => winit::window::CursorIcon::ContextMenu,
        CursorIcon::Cell => winit::window::CursorIcon::Cell,
        CursorIcon::VerticalText => winit::window::CursorIcon::VerticalText,
        CursorIcon::Alias => winit::window::CursorIcon::Alias,
        CursorIcon::Copy => winit::window::CursorIcon::Copy,
        CursorIcon::NoDrop => winit::window::CursorIcon::NoDrop,
        CursorIcon::Grab => winit::window::CursorIcon::Grab,
        CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
        CursorIcon::AllScroll => winit::window::CursorIcon::AllScroll,
        CursorIcon::ZoomIn => winit::window::CursorIcon::ZoomIn,
        CursorIcon::ZoomOut => winit::window::CursorIcon::ZoomOut,
        CursorIcon::EResize => winit::window::CursorIcon::EResize,
        CursorIcon::NResize => winit::window::CursorIcon::NResize,
        CursorIcon::NeResize => winit::window::CursorIcon::NeResize,
        CursorIcon::NwResize => winit::window::CursorIcon::NwResize,
        CursorIcon::SResize => winit::window::CursorIcon::SResize,
        CursorIcon::SeResize => winit::window::CursorIcon::SeResize,
        CursorIcon::SwResize => winit::window::CursorIcon::SwResize,
        CursorIcon::WResize => winit::window::CursorIcon::WResize,
        CursorIcon::EwResize => winit::window::CursorIcon::EwResize,
        CursorIcon::NsResize => winit::window::CursorIcon::NsResize,
        CursorIcon::NeswResize => winit::window::CursorIcon::NeswResize,
        CursorIcon::NwseResize => winit::window::CursorIcon::NwseResize,
        CursorIcon::ColResize => winit::window::CursorIcon::ColResize,
        CursorIcon::RowResize => winit::window::CursorIcon::RowResize,
    }
}

/// Window 扩展 trait：以引擎级类型操作窗口（屏蔽 winit 依赖）
pub trait WindowExt {
    fn set_engine_cursor_grab(&self, mode: CursorGrabMode) -> Result<(), String>;
    fn set_engine_cursor_icon(&self, icon: CursorIcon);
    fn set_engine_cursor_position(&self, x: f64, y: f64) -> Result<(), String>;
}

impl WindowExt for Window {
    fn set_engine_cursor_grab(&self, mode: CursorGrabMode) -> Result<(), String> {
        self.set_cursor_grab(map_cursor_grab_mode(mode))
            .map_err(|e| format!("设置光标捕获失败: {}", e))
    }

    fn set_engine_cursor_icon(&self, icon: CursorIcon) {
        self.set_cursor_icon(map_cursor_icon(icon));
    }

    fn set_engine_cursor_position(&self, x: f64, y: f64) -> Result<(), String> {
        self.set_cursor_position(PhysicalPosition { x, y })
            .map_err(|e| format!("设置光标位置失败: {}", e))
    }
}

// ===== 窗口配置 =====

pub struct WindowBuilder {
    builder: winit::window::WindowBuilder,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            builder: winit::window::WindowBuilder::new(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.builder = self.builder.with_title(title);
        self
    }

    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self
            .builder
            .with_inner_size(PhysicalSize::new(width, height));
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.builder = self.builder.with_resizable(resizable);
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.builder = self.builder.with_maximized(maximized);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.builder = self.builder.with_visible(visible);
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.builder = self.builder.with_decorations(decorations);
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.builder = self.builder.with_transparent(transparent);
        self
    }

    pub fn build(self, event_loop: &EventLoop<()>) -> Result<Window, Box<dyn std::error::Error>> {
        self.builder.build(event_loop).map_err(|e| e.into())
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub vsync: bool,
    pub fullscreen: bool,
    pub decorations: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Game Engine".to_string(),
            width: 1280,
            height: 720,
            resizable: true,
            vsync: true,
            fullscreen: false,
            decorations: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    pub id: u64,
    pub position: Vec2,
    pub force: f32,
    pub phase: winit::event::TouchPhase,
}

// ===== 输入状态 =====

/// 按键状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum KeyPressState {
    /// 上一帧松开，当前帧按下 — 刚按下
    JustPressed,
    /// 持续按下
    Pressed,
    /// 上一帧按下，当前帧松开 — 刚松开
    JustReleased,
    /// 已松开
    #[default]
    Released,
}

/// 鼠标按钮状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ButtonPressState {
    JustPressed,
    Pressed,
    JustReleased,
    #[default]
    Released,
}

/// 输入状态 — 基于引擎级 KeyCode 枚举
pub struct Input {
    // 按键状态
    key_states: HashMap<KeyCode, KeyPressState>,
    // 鼠标按钮状态
    button_states: HashMap<MouseButton, ButtonPressState>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    wheel_delta: Vec2,
    modifiers: ModifiersState,
    text_input: String,
    touches: HashMap<u64, TouchPoint>,
    // DPI 缩放因子
    scale_factor: f64,
    // 本帧生成的输入事件（队列）
    events_this_frame: Vec<InputEvent>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
            button_states: HashMap::new(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            wheel_delta: Vec2::ZERO,
            modifiers: ModifiersState::empty(),
            text_input: String::new(),
            touches: HashMap::new(),
            scale_factor: 1.0,
            events_this_frame: Vec::new(),
        }
    }

    /// 清空帧间临时状态（mouse_delta, wheel_delta, text）并翻转按键状态
    pub fn clear(&mut self) {
        // 翻转按键状态：JustPressed -> Pressed, JustReleased -> Released
        for state in self.key_states.values_mut() {
            *state = match *state {
                KeyPressState::JustPressed => KeyPressState::Pressed,
                KeyPressState::Pressed => KeyPressState::Pressed,
                KeyPressState::JustReleased => KeyPressState::Released,
                KeyPressState::Released => KeyPressState::Released,
            };
        }
        // 翻转鼠标按钮状态
        for state in self.button_states.values_mut() {
            *state = match *state {
                ButtonPressState::JustPressed => ButtonPressState::Pressed,
                ButtonPressState::Pressed => ButtonPressState::Pressed,
                ButtonPressState::JustReleased => ButtonPressState::Released,
                ButtonPressState::Released => ButtonPressState::Released,
            };
        }
        self.mouse_delta = Vec2::ZERO;
        self.wheel_delta = Vec2::ZERO;
        self.text_input.clear();
        self.events_this_frame.clear();
    }

    pub fn reset(&mut self) {
        self.key_states.clear();
        self.button_states.clear();
        self.mouse_position = Vec2::ZERO;
        self.mouse_delta = Vec2::ZERO;
        self.wheel_delta = Vec2::ZERO;
        self.modifiers = ModifiersState::empty();
        self.text_input.clear();
        self.touches.clear();
        self.events_this_frame.clear();
    }

    // ===== 按键查询 =====

    pub fn key_pressed(&self, code: KeyCode) -> bool {
        matches!(
            self.key_states
                .get(&code)
                .copied()
                .unwrap_or(KeyPressState::Released),
            KeyPressState::Pressed | KeyPressState::JustPressed
        )
    }

    pub fn key_just_pressed(&self, code: KeyCode) -> bool {
        matches!(
            self.key_states
                .get(&code)
                .copied()
                .unwrap_or(KeyPressState::Released),
            KeyPressState::JustPressed
        )
    }

    pub fn key_just_released(&self, code: KeyCode) -> bool {
        matches!(
            self.key_states
                .get(&code)
                .copied()
                .unwrap_or(KeyPressState::Released),
            KeyPressState::JustReleased
        )
    }

    // ===== 鼠标查询 =====

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        matches!(
            self.button_states
                .get(&button)
                .copied()
                .unwrap_or(ButtonPressState::Released),
            ButtonPressState::Pressed | ButtonPressState::JustPressed
        )
    }

    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        matches!(
            self.button_states
                .get(&button)
                .copied()
                .unwrap_or(ButtonPressState::Released),
            ButtonPressState::JustPressed
        )
    }

    pub fn mouse_button_just_released(&self, button: MouseButton) -> bool {
        matches!(
            self.button_states
                .get(&button)
                .copied()
                .unwrap_or(ButtonPressState::Released),
            ButtonPressState::JustReleased
        )
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn wheel_delta(&self) -> Vec2 {
        self.wheel_delta
    }

    pub fn modifiers(&self) -> ModifiersState {
        self.modifiers
    }

    pub fn text(&self) -> &str {
        &self.text_input
    }

    pub fn touches(&self) -> impl Iterator<Item = &TouchPoint> {
        self.touches.values()
    }

    pub fn touch(&self, id: u64) -> Option<&TouchPoint> {
        self.touches.get(&id)
    }

    pub fn touch_count(&self) -> usize {
        self.touches.len()
    }

    // ===== 帧事件迭代器 =====

    pub fn events(&self) -> impl Iterator<Item = &InputEvent> {
        self.events_this_frame.iter()
    }

    pub fn events_len(&self) -> usize {
        self.events_this_frame.len()
    }

    // ===== 状态更新（来自 winit 事件） =====

    pub fn update_key(&mut self, code: KeyCode, state: ElementState) {
        let current = self
            .key_states
            .get(&code)
            .copied()
            .unwrap_or(KeyPressState::Released);
        let new_state = match (current, state) {
            (KeyPressState::Released | KeyPressState::JustReleased, ElementState::Pressed) => {
                KeyPressState::JustPressed
            }
            (KeyPressState::JustPressed | KeyPressState::Pressed, ElementState::Pressed) => {
                KeyPressState::Pressed
            }
            (KeyPressState::Pressed | KeyPressState::JustPressed, ElementState::Released) => {
                KeyPressState::JustReleased
            }
            (KeyPressState::Released | KeyPressState::JustReleased, ElementState::Released) => {
                KeyPressState::Released
            }
        };
        self.key_states.insert(code, new_state);

        // 生成按键事件
        self.events_this_frame.push(InputEvent::Key(KeyEvent {
            code,
            state,
            modifiers: self.modifiers,
        }));
    }

    pub fn update_button(&mut self, button: MouseButton, state: ElementState) {
        let current = self
            .button_states
            .get(&button)
            .copied()
            .unwrap_or(ButtonPressState::Released);
        let new_state = match (current, state) {
            (
                ButtonPressState::Released | ButtonPressState::JustReleased,
                ElementState::Pressed,
            ) => ButtonPressState::JustPressed,
            (ButtonPressState::JustPressed | ButtonPressState::Pressed, ElementState::Pressed) => {
                ButtonPressState::Pressed
            }
            (ButtonPressState::Pressed | ButtonPressState::JustPressed, ElementState::Released) => {
                ButtonPressState::JustReleased
            }
            (
                ButtonPressState::Released | ButtonPressState::JustReleased,
                ElementState::Released,
            ) => ButtonPressState::Released,
        };
        self.button_states.insert(button, new_state);

        // 生成鼠标按钮事件
        self.events_this_frame
            .push(InputEvent::MouseButton(MouseButtonEvent {
                button,
                state,
                modifiers: self.modifiers,
            }));
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        let new_x = x as f32;
        let new_y = y as f32;
        let delta = Vec2::new(new_x - self.mouse_position.x, new_y - self.mouse_position.y);
        self.mouse_delta = delta;
        self.mouse_position = Vec2::new(new_x, new_y);

        if delta.x != 0.0 || delta.y != 0.0 {
            self.events_this_frame
                .push(InputEvent::MouseMotion(MouseMotionEvent {
                    position: self.mouse_position,
                    delta,
                }));
        }
    }

    pub fn update_wheel(&mut self, delta: Vec2) {
        self.wheel_delta += delta;
        self.events_this_frame
            .push(InputEvent::MouseWheel(MouseWheelEvent {
                delta,
                modifiers: self.modifiers,
            }));
    }

    pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    pub fn add_text(&mut self, text: &str) {
        self.text_input.push_str(text);
        self.events_this_frame
            .push(InputEvent::TextInput(TextInputEvent {
                text: text.to_string(),
            }));
    }

    pub fn update_touch(
        &mut self,
        id: u64,
        position: Vec2,
        force: f32,
        phase: winit::event::TouchPhase,
    ) {
        match phase {
            winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                self.touches.remove(&id);
            }
            _ => {
                self.touches.insert(
                    id,
                    TouchPoint {
                        id,
                        position,
                        force,
                        phase,
                    },
                );
            }
        }

        // Generate engine-level touch event
        let engine_phase = match phase {
            winit::event::TouchPhase::Started => crate::input_event::TouchPhase::Started,
            winit::event::TouchPhase::Moved => crate::input_event::TouchPhase::Moved,
            winit::event::TouchPhase::Ended => crate::input_event::TouchPhase::Ended,
            winit::event::TouchPhase::Cancelled => crate::input_event::TouchPhase::Cancelled,
        };
        self.events_this_frame.push(InputEvent::TouchInput(
            crate::input_event::TouchInputEvent {
                id,
                position,
                force,
                phase: engine_phase,
            },
        ));
    }

    // ===== 快捷查询 =====

    pub fn is_any_key_pressed(&self) -> bool {
        self.key_states
            .values()
            .any(|s| matches!(s, KeyPressState::Pressed | KeyPressState::JustPressed))
    }

    // ===== 按下键/按钮迭代器 =====

    /// 返回当前所有按下的键的迭代器
    pub fn pressed_keys(&self) -> impl Iterator<Item = KeyCode> + '_ {
        self.key_states
            .iter()
            .filter(|(_, &s)| matches!(s, KeyPressState::Pressed | KeyPressState::JustPressed))
            .map(|(&code, _)| code)
    }

    /// 返回当前所有按下的鼠标按钮的迭代器
    pub fn pressed_buttons(&self) -> impl Iterator<Item = MouseButton> + '_ {
        self.button_states
            .iter()
            .filter(|(_, &s)| {
                matches!(s, ButtonPressState::Pressed | ButtonPressState::JustPressed)
            })
            .map(|(&btn, _)| btn)
    }

    /// 返回已按下的键数量
    pub fn pressed_key_count(&self) -> usize {
        self.key_states
            .values()
            .filter(|&&s| matches!(s, KeyPressState::Pressed | KeyPressState::JustPressed))
            .count()
    }

    /// 返回已按下的鼠标按钮数量
    pub fn pressed_button_count(&self) -> usize {
        self.button_states
            .values()
            .filter(|&&s| matches!(s, ButtonPressState::Pressed | ButtonPressState::JustPressed))
            .count()
    }

    // ===== DPI 缩放 =====

    /// 获取 DPI 缩放因子
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// 更新 DPI 缩放因子
    pub fn set_scale_factor(&mut self, factor: f64) {
        self.scale_factor = factor;
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

// ===== 输入模块：统一处理 winit 事件并转换为引擎级 Input =====

pub struct InputModule {
    input: Input,
}

impl Default for InputModule {
    fn default() -> Self {
        Self::new()
    }
}

impl InputModule {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }

    /// 处理单个 winit 事件并更新状态
    pub fn process_event(&mut self, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } => {
                    // winit 0.29: physical_key 是 PhysicalKey（非 Option）
                    let code = key_code::map_physical_key(key_event.physical_key);
                    if matches!(code, KeyCode::Unknown) {
                        // 未知按键 — 尝试用 logical_key 的 NamedKey 兜底
                        if let winit::keyboard::Key::Named(named) = key_event.logical_key {
                            if let Some(mapped) = key_code::map_named_key_to_keycode(&named) {
                                let state =
                                    if key_event.state == winit::event::ElementState::Pressed {
                                        ElementState::Pressed
                                    } else {
                                        ElementState::Released
                                    };
                                self.input.update_key(mapped, state);
                            }
                        }
                    } else {
                        let state = if key_event.state == winit::event::ElementState::Pressed {
                            ElementState::Pressed
                        } else {
                            ElementState::Released
                        };
                        self.input.update_key(code, state);
                    }
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    let engine_modifiers = key_code::map_modifiers(*modifiers);
                    self.input.update_modifiers(engine_modifiers);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    let engine_button = key_code::map_mouse_button(*button);
                    let engine_state = if *state == winit::event::ElementState::Pressed {
                        ElementState::Pressed
                    } else {
                        ElementState::Released
                    };
                    self.input.update_button(engine_button, engine_state);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.input.update_mouse_position(position.x, position.y);
                }
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        self.input.update_wheel(Vec2::new(*x, *y));
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        self.input
                            .update_wheel(Vec2::new(pos.x as f32, pos.y as f32));
                    }
                },
                WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                    self.input.add_text(text);
                }
                WindowEvent::Touch(touch) => {
                    let position = Vec2::new(touch.location.x as f32, touch.location.y as f32);
                    let force = match touch.force {
                        Some(winit::event::Force::Normalized(f)) => f as f32,
                        _ => 0.0,
                    };
                    self.input
                        .update_touch(touch.id, position, force, touch.phase);
                }
                _ => {}
            }
        }
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Input 基础测试 =====

    #[test]
    fn test_input_new() {
        let input = Input::new();
        assert_eq!(input.mouse_position(), Vec2::ZERO);
        assert_eq!(input.text(), "");
        assert_eq!(input.touch_count(), 0);
        assert!(input.events_len() == 0);
    }

    #[test]
    fn test_input_key_press_and_release() {
        let mut input = Input::new();
        input.update_key(KeyCode::A, ElementState::Pressed);
        assert!(input.key_pressed(KeyCode::A));
        assert!(input.key_just_pressed(KeyCode::A));
        assert!(!input.key_just_released(KeyCode::A));

        // 第二次 update 同键（保持按下）— 应从 JustPressed 转为 Pressed
        input.update_key(KeyCode::A, ElementState::Pressed);
        assert!(input.key_pressed(KeyCode::A));
        assert!(!input.key_just_pressed(KeyCode::A));

        // 松开
        input.update_key(KeyCode::A, ElementState::Released);
        assert!(!input.key_pressed(KeyCode::A));
        assert!(input.key_just_released(KeyCode::A));
    }

    #[test]
    fn test_input_clear_transitions_states() {
        let mut input = Input::new();
        input.update_key(KeyCode::Space, ElementState::Pressed);
        assert!(input.key_just_pressed(KeyCode::Space));

        // clear 后 JustPressed -> Pressed
        input.clear();
        assert!(input.key_pressed(KeyCode::Space));
        assert!(!input.key_just_pressed(KeyCode::Space));

        input.update_key(KeyCode::Space, ElementState::Released);
        assert!(input.key_just_released(KeyCode::Space));

        input.clear();
        assert!(!input.key_pressed(KeyCode::Space));
        assert!(!input.key_just_released(KeyCode::Space));
    }

    #[test]
    fn test_input_mouse_buttons() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.mouse_button_pressed(MouseButton::Left));
        assert!(input.mouse_button_just_pressed(MouseButton::Left));

        input.update_button(MouseButton::Left, ElementState::Released);
        assert!(!input.mouse_button_pressed(MouseButton::Left));
        assert!(input.mouse_button_just_released(MouseButton::Left));
    }

    #[test]
    fn test_input_mouse_position_delta() {
        let mut input = Input::new();
        input.update_mouse_position(100.0, 200.0);
        let pos = input.mouse_position();
        assert!((pos.x - 100.0).abs() < f32::EPSILON);
        assert!((pos.y - 200.0).abs() < f32::EPSILON);

        input.update_mouse_position(150.0, 200.0);
        let delta = input.mouse_delta();
        assert!((delta.x - 50.0).abs() < f32::EPSILON);
        assert!(delta.y.abs() < f32::EPSILON);
    }

    #[test]
    fn test_input_wheel() {
        let mut input = Input::new();
        input.update_wheel(Vec2::new(0.0, 3.0));
        let d = input.wheel_delta();
        assert!((d.y - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_input_text_input() {
        let mut input = Input::new();
        input.add_text("hello");
        assert_eq!(input.text(), "hello");
    }

    #[test]
    fn test_input_reset_clears_everything() {
        let mut input = Input::new();
        input.update_key(KeyCode::A, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.add_text("test");
        input.update_mouse_position(100.0, 100.0);
        input.update_wheel(Vec2::new(1.0, 1.0));

        input.reset();

        assert!(!input.key_pressed(KeyCode::A));
        assert!(!input.mouse_button_pressed(MouseButton::Left));
        assert_eq!(input.text(), "");
        assert_eq!(input.mouse_position(), Vec2::ZERO);
        assert_eq!(input.wheel_delta(), Vec2::ZERO);
        assert_eq!(input.events_len(), 0);
    }

    #[test]
    fn test_input_events_generation() {
        let mut input = Input::new();
        input.update_key(KeyCode::Enter, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.events_len() >= 2);
        let has_key = input.events().any(|e| matches!(e, InputEvent::Key(_)));
        let has_mb = input
            .events()
            .any(|e| matches!(e, InputEvent::MouseButton(_)));
        assert!(has_key);
        assert!(has_mb);
    }

    #[test]
    fn test_input_events_cleared_after_clear() {
        let mut input = Input::new();
        input.update_key(KeyCode::A, ElementState::Pressed);
        assert!(input.events_len() > 0);
        input.clear();
        assert_eq!(input.events_len(), 0);
    }

    #[test]
    fn test_is_any_key_pressed() {
        let mut input = Input::new();
        assert!(!input.is_any_key_pressed());
        input.update_key(KeyCode::Escape, ElementState::Pressed);
        assert!(input.is_any_key_pressed());
    }

    // ===== InputModule 测试 =====

    #[test]
    fn test_input_module_new() {
        let module = InputModule::new();
        assert_eq!(module.input().events_len(), 0);
    }

    #[test]
    fn test_input_module_clear() {
        let mut module = InputModule::new();
        {
            let input = module.input_mut();
            input.update_key(KeyCode::Space, ElementState::Pressed);
            input.add_text("hello");
        }
        module.clear();
        assert_eq!(module.input().mouse_delta(), Vec2::ZERO);
        assert_eq!(module.input().text(), "");
        assert_eq!(module.input().events_len(), 0);
    }

    // ===== 剪贴板错误 Display 测试 =====

    #[test]
    fn test_clipboard_error_display() {
        let error = ClipboardError::NotInitialized("test error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("test error"));
    }

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.title, "Game Engine");
        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
        assert!(config.resizable);
    }

    #[test]
    fn test_window_builder_new() {
        let builder = WindowBuilder::new();
        let _ = builder.with_title("Test").with_inner_size(800, 600);
    }

    #[test]
    fn test_pressed_keys_iterator() {
        let mut input = Input::new();
        input.update_key(KeyCode::A, ElementState::Pressed);
        input.update_key(KeyCode::D, ElementState::Pressed);
        let pressed: Vec<_> = input.pressed_keys().collect();
        assert_eq!(pressed.len(), 2);
        assert!(pressed.contains(&KeyCode::A));
        assert!(pressed.contains(&KeyCode::D));
    }

    #[test]
    fn test_pressed_buttons_iterator() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_button(MouseButton::Right, ElementState::Pressed);
        let pressed: Vec<_> = input.pressed_buttons().collect();
        assert_eq!(pressed.len(), 2);
    }

    #[test]
    fn test_scale_factor() {
        let mut input = Input::new();
        assert_eq!(input.scale_factor(), 1.0);
        input.set_scale_factor(2.0);
        assert_eq!(input.scale_factor(), 2.0);
    }
}
