//! engine-window - 窗口系统 / 事件循环 / 输入原语
//!
//! Window system / Event loop / Input primitives for the game engine.
//! 提供窗口创建、事件循环封装、键盘/鼠标/触摸输入状态跟踪。

use engine_math::Vec2;

pub use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position, Size};
pub use winit::event::{DeviceEvent, Event, Touch, TouchPhase, WindowEvent};
pub use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
pub use winit::keyboard::{KeyCode, ModifiersState, NamedKey};
pub use winit::monitor::MonitorHandle;
pub use winit::window::{
    CursorGrabMode, CursorIcon, Fullscreen, Icon, Theme, Window, WindowId, WindowLevel,
};

/// 触摸点数据。
///
/// Touch point data structure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    /// 触摸 ID。Touch identifier.
    pub id: u64,
    /// 触摸位置（像素坐标）。Position in pixels.
    pub position: Vec2,
    /// 按压力度（0.0 - 1.0，不支持则为 0）。Pressure 0.0 - 1.0.
    pub force: f32,
    /// 触摸阶段。Touch phase.
    pub phase: TouchPhase,
}

/// 元素状态
///
/// Element state for buttons/keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementState {
    /// 按下 Pressed.
    Pressed,
    /// 释放 Released.
    Released,
}

impl From<winit::event::ElementState> for ElementState {
    fn from(s: winit::event::ElementState) -> Self {
        match s {
            winit::event::ElementState::Pressed => ElementState::Pressed,
            winit::event::ElementState::Released => ElementState::Released,
        }
    }
}

/// 鼠标按钮枚举
///
/// Mouse button enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// 左键 Left button.
    Left,
    /// 中键 Middle button.
    Middle,
    /// 右键 Right button.
    Right,
    /// 其他按钮（平台相关编码）。Other button with platform code.
    Other(u16),
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(b: winit::event::MouseButton) -> Self {
        match b {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Other(c) => MouseButton::Other(c),
            _ => MouseButton::Other(0),
        }
    }
}

/// 鼠标滚轮增量
///
/// Mouse scroll delta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScrollDelta {
    /// 行/列增量（通常是滚轮单位）。Line delta in lines/columns.
    LineDelta(f32, f32),
    /// 像素增量。Pixel delta.
    PixelDelta(f32, f32),
}

/// 原始扫描码
///
/// Raw scancode type.
pub type ScanCode = u32;

/// 窗口构建器 - Fluent API
///
/// Window builder with a fluent API.
/// 配置标题、大小、可调整、全屏、装饰、透明度等。
pub struct WindowBuilder {
    builder: winit::window::WindowBuilder,
}

impl WindowBuilder {
    /// 创建一个新的窗口构建器。
    ///
    /// Create a new window builder.
    pub fn new() -> Self {
        Self {
            builder: winit::window::WindowBuilder::new(),
        }
    }

    /// 设置窗口标题。
    ///
    /// Set window title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.builder = self.builder.with_title(title);
        self
    }

    /// 设置窗口内部逻辑大小（宽 x 高，以物理像素为单位）。
    ///
    /// Set inner size in physical pixels.
    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self
            .builder
            .with_inner_size(PhysicalSize::new(width, height));
        self
    }

    /// 设置窗口内部逻辑大小（使用 LogicalSize / PhysicalSize / Size）。
    ///
    /// Set inner size from a winit `Size`.
    pub fn with_inner_size_p<S: Into<Size>>(mut self, size: S) -> Self {
        self.builder = self.builder.with_inner_size(size.into());
        self
    }

    /// 设置窗口最小尺寸。
    ///
    /// Set minimum inner size.
    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self
            .builder
            .with_min_inner_size(PhysicalSize::new(width, height));
        self
    }

    /// 设置窗口最大尺寸。
    ///
    /// Set maximum inner size.
    pub fn with_max_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self
            .builder
            .with_max_inner_size(PhysicalSize::new(width, height));
        self
    }

    /// 设置窗口是否可调整大小。
    ///
    /// Set whether window is resizable.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.builder = self.builder.with_resizable(resizable);
        self
    }

    /// 设置窗口是否有标题栏/装饰。
    ///
    /// Set window decorations (title bar).
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.builder = self.builder.with_decorations(decorations);
        self
    }

    /// 设置窗口透明度（背景是否透明）。
    ///
    /// Set transparent (transparent background).
    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.builder = self.builder.with_transparent(transparent);
        self
    }

    /// 设置窗口是否置顶。
    ///
    /// Set always-on-top.
    pub fn with_always_on_top(mut self, always_on_top: bool) -> Self {
        self.builder = self.builder.with_window_level(if always_on_top {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        });
        self
    }

    /// 设置初始窗口可见性。
    ///
    /// Set initial visibility.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.builder = self.builder.with_visible(visible);
        self
    }

    /// 设置是否启动时最大化。
    ///
    /// Start maximized.
    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.builder = self.builder.with_maximized(maximized);
        self
    }

    /// 设置是否启动时最小化。
    ///
    /// Start minimized (fallback via `with_visible(false)`).
    pub fn with_minimized(mut self, minimized: bool) -> Self {
        if minimized {
            self.builder = self.builder.with_visible(false);
        }
        self
    }

    /// 设置窗口内容受保护（不允许截屏）。
    ///
    /// Enable content protection (screenshot blocking).
    pub fn with_content_protected(mut self, protected: bool) -> Self {
        self.builder = self.builder.with_content_protected(protected);
        self
    }

    /// 设置窗口图标。
    ///
    /// Set window icon.
    pub fn with_window_icon(mut self, icon: Option<Icon>) -> Self {
        self.builder = self.builder.with_window_icon(icon);
        self
    }

    /// 切换全屏模式（boolean 版本，使用 Borderless）。
    ///
    /// Toggle fullscreen (boolean shortcut for Borderless mode).
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        if fullscreen {
            self.builder = self
                .builder
                .with_fullscreen(Some(Fullscreen::Borderless(None)));
        } else {
            self.builder = self.builder.with_fullscreen(None);
        }
        self
    }

    /// 设置完整的全屏模式（可选择 Borderless 或 Exclusive）。
    ///
    /// Set fullscreen mode.
    pub fn with_fullscreen_mode(mut self, fullscreen: Option<Fullscreen>) -> Self {
        self.builder = self.builder.with_fullscreen(fullscreen);
        self
    }

    /// 请求输入法（IME）开启。
    ///
    /// Enable IME (stub in current winit version).
    pub fn with_ime(self, _ime_enabled: bool) -> Self {
        self
    }

    /// macOS: 允许禁用鼠标光标命中测试（用于穿透点击）。
    ///
    /// macOS: cursor hit test toggle.
    #[cfg(target_os = "macos")]
    pub fn with_cursor_hittest(mut self, hittest: bool) -> Self {
        use winit::platform::macos::WindowBuilderExtMacOS;
        self.builder = self.builder.with_cursor_hittest(hittest);
        self
    }

    /// 非 macOS: 忽略（空操作）。
    ///
    /// Non-macOS: no-op.
    #[cfg(not(target_os = "macos"))]
    pub fn with_cursor_hittest(self, _hittest: bool) -> Self {
        self
    }

    /// 构建窗口。
    ///
    /// Build the window using the given event loop.
    pub fn build<T>(
        self,
        event_loop: &EventLoop<T>,
    ) -> Result<Window, Box<dyn std::error::Error>> {
        self.builder.build(event_loop).map_err(|e| e.into())
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 剪贴板支持（留位接口）。
///
/// Clipboard support (placeholder API).
pub mod clipboard {
    /// 读取剪贴板文本（当前实现始终返回 None）。
    ///
    /// Read text from clipboard (always returns None in current impl).
    pub fn get_text() -> Option<String> {
        None
    }

    /// 设置剪贴板文本（当前实现始终失败）。
    ///
    /// Set clipboard text (always fails in current impl).
    #[allow(clippy::result_unit_err)]
    pub fn set_text(_text: &str) -> Result<(), ()> {
        Err(())
    }
}

// —— 独立子模块 ——

/// 输入相关实现：Input 状态快照 + InputModule 事件处理。
///
/// Input-related implementation: Input state snapshot + InputModule event processor.
mod input {
    use super::{
        ElementState, MouseButton, MouseScrollDelta, TouchPhase, TouchPoint, Vec2,
    };
    use std::collections::HashMap;
    use winit::event::{Event, WindowEvent};
    use winit::keyboard::ModifiersState;
    use winit::keyboard::NamedKey;

    /// 输入状态快照。
    ///
    /// Input state snapshot. Tracks pressed/just-pressed/just-released for
    /// keys and mouse buttons, mouse position/delta, wheel delta, and active
    /// touches. Call `clear()` every frame to reset "just" semantics.
    pub struct Input {
        pressed_keys: Vec<NamedKey>,
        previous_keys: Vec<NamedKey>,
        pressed_buttons: Vec<MouseButton>,
        previous_buttons: Vec<MouseButton>,
        mouse_position: Vec2,
        mouse_delta: Vec2,
        wheel_delta: Vec2,
        modifiers: ModifiersState,
        text_input: String,
        touches: HashMap<u64, TouchPoint>,
        cursor_in_window: bool,
    }

    impl Input {
        /// 创建一个新的输入状态。
        ///
        /// Create new empty input state.
        pub fn new() -> Self {
            Self {
                pressed_keys: Vec::new(),
                previous_keys: Vec::new(),
                pressed_buttons: Vec::new(),
                previous_buttons: Vec::new(),
                mouse_position: Vec2::ZERO,
                mouse_delta: Vec2::ZERO,
                wheel_delta: Vec2::ZERO,
                modifiers: ModifiersState::empty(),
                text_input: String::new(),
                touches: HashMap::new(),
                cursor_in_window: false,
            }
        }

        /// 每帧重置瞬时状态：保留 pressed 集合；清除 "just" 集合。
        ///
        /// Per-frame reset: preserves pressed state; clears "just" state.
        pub fn clear(&mut self) {
            self.previous_keys.clear();
            self.previous_keys.extend(self.pressed_keys.iter().copied());
            self.previous_buttons.clear();
            self.previous_buttons
                .extend(self.pressed_buttons.iter().copied());
            self.mouse_delta = Vec2::ZERO;
            self.wheel_delta = Vec2::ZERO;
            self.text_input.clear();
        }

        /// 完全重置所有状态。
        ///
        /// Complete reset: clears every field.
        pub fn reset(&mut self) {
            self.pressed_keys.clear();
            self.previous_keys.clear();
            self.pressed_buttons.clear();
            self.previous_buttons.clear();
            self.mouse_position = Vec2::ZERO;
            self.mouse_delta = Vec2::ZERO;
            self.wheel_delta = Vec2::ZERO;
            self.modifiers = ModifiersState::empty();
            self.text_input.clear();
            self.touches.clear();
            self.cursor_in_window = false;
        }

        /// 查询指定键是否被按下（保持状态）。
        ///
        /// Query if key is currently pressed.
        pub fn key_pressed(&self, keycode: NamedKey) -> bool {
            self.pressed_keys.contains(&keycode)
        }

        /// 查询指定键是否在本帧刚被按下。
        ///
        /// Query if key was just pressed this frame.
        pub fn key_just_pressed(&self, keycode: NamedKey) -> bool {
            self.pressed_keys.contains(&keycode) && !self.previous_keys.contains(&keycode)
        }

        /// 查询指定键是否在本帧刚被释放。
        ///
        /// Query if key was just released this frame.
        pub fn key_just_released(&self, keycode: NamedKey) -> bool {
            !self.pressed_keys.contains(&keycode) && self.previous_keys.contains(&keycode)
        }

        /// 当前所有按下键的切片。
        ///
        /// Slice of currently pressed keys.
        pub fn pressed_keys(&self) -> &[NamedKey] {
            &self.pressed_keys
        }

        /// 当前本帧刚释放键的迭代器（previous_keys \ pressed_keys）。
        ///
        /// Iterator of keys released this frame.
        pub fn released_keys(&self) -> impl Iterator<Item = &NamedKey> {
            self.previous_keys
                .iter()
                .filter(move |k| !self.pressed_keys.contains(k))
        }

        /// 查询指定鼠标按钮是否被按下。
        ///
        /// Query if mouse button is pressed.
        pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
            self.pressed_buttons.contains(&button)
        }

        /// 查询指定鼠标按钮是否本帧刚按下。
        ///
        /// Query if mouse button was just pressed.
        pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
            self.pressed_buttons.contains(&button) && !self.previous_buttons.contains(&button)
        }

        /// 查询指定鼠标按钮是否本帧刚释放。
        ///
        /// Query if mouse button was just released.
        pub fn mouse_button_just_released(&self, button: MouseButton) -> bool {
            !self.pressed_buttons.contains(&button) && self.previous_buttons.contains(&button)
        }

        /// 当前所有按下鼠标按钮的切片。
        ///
        /// Slice of currently pressed buttons.
        pub fn pressed_buttons(&self) -> &[MouseButton] {
            &self.pressed_buttons
        }

        /// 本帧刚释放的鼠标按钮迭代器。
        ///
        /// Iterator of buttons released this frame.
        pub fn released_buttons(&self) -> impl Iterator<Item = &MouseButton> {
            self.previous_buttons
                .iter()
                .filter(move |b| !self.pressed_buttons.contains(b))
        }

        /// 返回当前鼠标坐标（像素坐标）。
        ///
        /// Current mouse position in pixels.
        pub fn mouse_position(&self) -> Vec2 {
            self.mouse_position
        }

        /// 返回本帧鼠标移动增量（像素）。
        ///
        /// Mouse movement delta in pixels for this frame.
        pub fn mouse_delta(&self) -> Vec2 {
            self.mouse_delta
        }

        /// 返回本帧滚轮增量（像素或行单位）。
        ///
        /// Wheel delta this frame.
        pub fn wheel_delta(&self) -> Vec2 {
            self.wheel_delta
        }

        /// 当前修饰键状态。
        ///
        /// Current modifier keys state.
        pub fn modifiers(&self) -> ModifiersState {
            self.modifiers
        }

        /// 本帧由 IME 录入的文本。
        ///
        /// Text input gathered this frame.
        pub fn text(&self) -> &str {
            &self.text_input
        }

        /// 光标是否在窗口内。
        ///
        /// Whether cursor is inside window.
        pub fn cursor_in_window(&self) -> bool {
            self.cursor_in_window
        }

        /// 设置光标是否在窗口内。
        ///
        /// Set whether cursor is inside window.
        pub fn set_cursor_in_window(&mut self, inside: bool) {
            self.cursor_in_window = inside;
        }

        /// 当前活动触摸点总数。
        ///
        /// Number of active touch points.
        pub fn touch_count(&self) -> usize {
            self.touches.len()
        }

        /// 活动触摸点迭代器。
        ///
        /// Iterator of active touch points.
        pub fn touches(&self) -> impl Iterator<Item = &TouchPoint> {
            self.touches.values()
        }

        /// 按 ID 查询触摸点。
        ///
        /// Lookup touch point by id.
        pub fn touch(&self, id: u64) -> Option<&TouchPoint> {
            self.touches.get(&id)
        }

        // —— 内部更新接口 ——

        pub(crate) fn update_mouse_position(&mut self, x: f64, y: f64) {
            let x = x as f32;
            let y = y as f32;
            self.mouse_delta = Vec2::new(x - self.mouse_position.x, y - self.mouse_position.y);
            self.mouse_position = Vec2::new(x, y);
        }

        pub(crate) fn update_wheel(&mut self, delta: MouseScrollDelta) {
            match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    self.wheel_delta += Vec2::new(x, y);
                }
                MouseScrollDelta::PixelDelta(x, y) => {
                    self.wheel_delta += Vec2::new(x, y);
                }
            }
        }

        pub(crate) fn update_key(&mut self, keycode: NamedKey, state: ElementState) {
            match state {
                ElementState::Pressed => {
                    if !self.pressed_keys.contains(&keycode) {
                        self.pressed_keys.push(keycode);
                    }
                }
                ElementState::Released => {
                    self.pressed_keys.retain(|&k| k != keycode);
                }
            }
        }

        pub(crate) fn update_button(&mut self, button: MouseButton, state: ElementState) {
            match state {
                ElementState::Pressed => {
                    if !self.pressed_buttons.contains(&button) {
                        self.pressed_buttons.push(button);
                    }
                }
                ElementState::Released => {
                    self.pressed_buttons.retain(|&b| b != button);
                }
            }
        }

        pub(crate) fn update_modifiers(&mut self, modifiers: ModifiersState) {
            self.modifiers = modifiers;
        }

        pub(crate) fn add_text(&mut self, text: &str) {
            self.text_input.push_str(text);
        }

        pub(crate) fn update_touch(
            &mut self,
            id: u64,
            position: Vec2,
            force: f32,
            phase: TouchPhase,
        ) {
            match phase {
                TouchPhase::Ended | TouchPhase::Cancelled => {
                    self.touches.remove(&id);
                }
                _ => {
                    self.touches.insert(id, TouchPoint {
                        id,
                        position,
                        force,
                        phase,
                    });
                }
            }
        }
    }

    impl Default for Input {
        fn default() -> Self {
            Self::new()
        }
    }

    /// 输入模块：订阅窗口事件并更新 Input 状态。
    ///
    /// Input module: subscribes to window events and updates Input state.
    pub struct InputModule {
        input: Input,
    }

    impl InputModule {
        /// 创建一个新的输入模块。
        ///
        /// Create a new input module.
        pub fn new() -> Self {
            Self {
                input: Input::new(),
            }
        }

        /// 处理一个 winit 事件并更新内部状态。
        ///
        /// Process a winit event and update internal state.
        pub fn process_event<T>(&mut self, event: &Event<T>) {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::KeyboardInput { event: key_event, .. } => {
                        if let winit::keyboard::Key::Named(keycode) = key_event.logical_key {
                            self.input.update_key(keycode, key_event.state.into());
                        }
                        if let winit::event::ElementState::Pressed = key_event.state {
                            if let winit::keyboard::Key::Character(ch) = &key_event.logical_key {
                                self.input.add_text(ch);
                            }
                        }
                    }
                    WindowEvent::ModifiersChanged(mods) => {
                        self.input.update_modifiers(mods.state());
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        self.input.update_button((*button).into(), (*state).into());
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.input.update_mouse_position(position.x, position.y);
                    }
                    WindowEvent::CursorEntered { .. } => {
                        self.input.set_cursor_in_window(true);
                    }
                    WindowEvent::CursorLeft { .. } => {
                        self.input.set_cursor_in_window(false);
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let d = match delta {
                            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                MouseScrollDelta::LineDelta(*x, *y)
                            }
                            winit::event::MouseScrollDelta::PixelDelta(pos) => {
                                MouseScrollDelta::PixelDelta(pos.x as f32, pos.y as f32)
                            }
                        };
                        self.input.update_wheel(d);
                    }
                    WindowEvent::Touch(touch) => {
                        let position =
                            Vec2::new(touch.location.x as f32, touch.location.y as f32);
                        let force = match touch.force {
                            Some(winit::event::Force::Normalized(f)) => f as f32,
                            Some(winit::event::Force::Calibrated { force: f, .. }) => f as f32,
                            _ => 0.0,
                        };
                        self.input.update_touch(touch.id, position, force, touch.phase);
                    }
                    _ => {}
                }
            }
        }

        /// 读取输入状态快照引用。
        ///
        /// Borrow input snapshot.
        pub fn input(&self) -> &Input {
            &self.input
        }

        /// 获取输入状态的可变引用。
        ///
        /// Borrow input snapshot mutably.
        pub fn input_mut(&mut self) -> &mut Input {
            &mut self.input
        }

        /// 每帧调用：清除瞬时状态。
        ///
        /// Call each frame: resets "just" semantics.
        pub fn clear(&mut self) {
            self.input.clear();
        }

        /// 完全重置所有状态。
        ///
        /// Reset all state.
        pub fn reset(&mut self) {
            self.input.reset();
        }
    }

    impl Default for InputModule {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// 窗口配置。
///
/// Window configuration.
mod window_config {
    /// 窗口配置结构体。默认 1280x720，可调整，无全屏，有标题栏。
    ///
    /// Window configuration. Defaults to 1280x720, resizable,
    /// non-fullscreen, decorated, vsync on.
    #[derive(Debug, Clone)]
    pub struct WindowConfig {
        /// 窗口标题。Window title.
        pub title: String,
        /// 宽度（像素）。Width in pixels.
        pub width: u32,
        /// 高度（像素）。Height in pixels.
        pub height: u32,
        /// 是否可调整大小。Resizable.
        pub resizable: bool,
        /// 是否开启垂直同步。Vsync.
        pub vsync: bool,
        /// 是否全屏。Fullscreen.
        pub fullscreen: bool,
        /// 是否有标题栏。Decorations.
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

    /// 窗口模式：窗口化 / 全屏 / 无边框。
    ///
    /// Window mode: windowed / fullscreen / borderless.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WindowMode {
        /// 窗口化 Windowed.
        Windowed,
        /// 全屏（独占） Fullscreen (exclusive).
        Fullscreen,
        /// 无边框 Borderless.
        Borderless,
    }
}

pub use input::{Input, InputModule};
pub use window_config::{WindowConfig, WindowMode};

// —— 测试 ——

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_new() {
        let input = Input::new();
        assert!(input.pressed_keys().is_empty());
        assert!(input.pressed_buttons().is_empty());
        assert_eq!(input.mouse_position(), Vec2::ZERO);
        assert_eq!(input.mouse_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_input_key_pressed() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        assert!(input.key_pressed(NamedKey::Space));
        assert!(!input.key_pressed(NamedKey::Enter));
        input.update_key(NamedKey::Space, ElementState::Released);
        assert!(!input.key_pressed(NamedKey::Space));
    }

    #[test]
    fn test_input_key_just_pressed() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        assert!(input.key_just_pressed(NamedKey::Space));
        input.clear();
        assert!(!input.key_just_pressed(NamedKey::Space));
        assert!(input.key_pressed(NamedKey::Space));
    }

    #[test]
    fn test_input_key_just_released() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        input.clear();
        input.update_key(NamedKey::Space, ElementState::Released);
        assert!(input.key_just_released(NamedKey::Space));
    }

    #[test]
    fn test_input_mouse_button_pressed() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.mouse_button_pressed(MouseButton::Left));
        assert!(!input.mouse_button_pressed(MouseButton::Right));
    }

    #[test]
    fn test_input_mouse_button_just_pressed() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.mouse_button_just_pressed(MouseButton::Left));
        input.clear();
        assert!(!input.mouse_button_just_pressed(MouseButton::Left));
        assert!(input.mouse_button_pressed(MouseButton::Left));
    }

    #[test]
    fn test_input_mouse_position() {
        let mut input = Input::new();
        input.update_mouse_position(100.0, 200.0);
        assert_eq!(input.mouse_position(), Vec2::new(100.0, 200.0));
        assert_eq!(input.mouse_delta(), Vec2::new(100.0, 200.0));
        input.update_mouse_position(150.0, 250.0);
        assert_eq!(input.mouse_position(), Vec2::new(150.0, 250.0));
        assert_eq!(input.mouse_delta(), Vec2::new(50.0, 50.0));
    }

    #[test]
    fn test_input_clear() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_mouse_position(100.0, 200.0);
        input.clear();
        assert!(input.key_pressed(NamedKey::Space));
        assert!(input.mouse_button_pressed(MouseButton::Left));
        assert_eq!(input.mouse_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_input_reset() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_mouse_position(100.0, 200.0);
        input.reset();
        assert!(!input.key_pressed(NamedKey::Space));
        assert!(!input.mouse_button_pressed(MouseButton::Left));
        assert_eq!(input.mouse_position(), Vec2::ZERO);
    }

    #[test]
    fn test_input_wheel() {
        let mut input = Input::new();
        input.update_wheel(MouseScrollDelta::LineDelta(1.0, 2.0));
        assert_eq!(input.wheel_delta(), Vec2::new(1.0, 2.0));
        input.update_wheel(MouseScrollDelta::PixelDelta(0.5, 0.5));
        assert_eq!(input.wheel_delta(), Vec2::new(1.5, 2.5));
        input.clear();
        assert_eq!(input.wheel_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_input_modifiers() {
        let mut input = Input::new();
        let mods = ModifiersState::SHIFT | ModifiersState::CONTROL;
        input.update_modifiers(mods);
        assert_eq!(input.modifiers(), mods);
    }

    #[test]
    fn test_input_text() {
        let mut input = Input::new();
        input.add_text("Hello");
        assert_eq!(input.text(), "Hello");
        input.add_text(" World");
        assert_eq!(input.text(), "Hello World");
        input.clear();
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_touch_point() {
        let point = TouchPoint {
            id: 1,
            position: Vec2::new(100.0, 200.0),
            force: 0.5,
            phase: TouchPhase::Started,
        };
        assert_eq!(point.id, 1);
        assert_eq!(point.position, Vec2::new(100.0, 200.0));
        assert_eq!(point.force, 0.5);
    }

    #[test]
    fn test_input_module_creation() {
        let module = InputModule::new();
        let input = module.input();
        assert!(input.pressed_keys().is_empty());
        assert_eq!(input.touch_count(), 0);
    }

    #[test]
    fn test_window_config_default() {
        let cfg = WindowConfig::default();
        assert_eq!(cfg.width, 1280);
        assert_eq!(cfg.height, 720);
        assert!(cfg.resizable);
        assert!(cfg.vsync);
        assert!(!cfg.fullscreen);
        assert!(cfg.decorations);
    }

    #[test]
    fn test_window_builder_fluent() {
        // WindowBuilder build() requires an event loop, so we only check the
        // fluent-API side-effect-free chain works.
        let _ = WindowBuilder::new()
            .with_title("Test")
            .with_inner_size(800, 600)
            .with_min_inner_size(320, 240)
            .with_max_inner_size(1920, 1080)
            .with_resizable(false)
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(true)
            .with_visible(false)
            .with_maximized(true)
            .with_minimized(false)
            .with_content_protected(true)
            .with_fullscreen(false)
            .with_ime(true)
            .with_cursor_hittest(true);
    }

    #[test]
    fn test_window_mode() {
        let m1 = WindowMode::Windowed;
        let m2 = WindowMode::Fullscreen;
        let m3 = WindowMode::Borderless;
        assert_ne!(m1, m2);
        assert_ne!(m2, m3);
    }

    #[test]
    fn test_element_state_from() {
        let p: ElementState = winit::event::ElementState::Pressed.into();
        let r: ElementState = winit::event::ElementState::Released.into();
        assert_eq!(p, ElementState::Pressed);
        assert_eq!(r, ElementState::Released);
    }

    #[test]
    fn test_mouse_button_from() {
        let l: MouseButton = winit::event::MouseButton::Left.into();
        let m: MouseButton = winit::event::MouseButton::Middle.into();
        let r: MouseButton = winit::event::MouseButton::Right.into();
        let o: MouseButton = winit::event::MouseButton::Other(42).into();
        assert_eq!(l, MouseButton::Left);
        assert_eq!(m, MouseButton::Middle);
        assert_eq!(r, MouseButton::Right);
        assert_eq!(o, MouseButton::Other(42));
    }
}
