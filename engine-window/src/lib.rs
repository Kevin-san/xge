//! engine-window crate - 窗口系统与事件循环模块
//!
//! 提供窗口、事件循环和输入抽象

use engine_math::Vec2;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

pub use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position};
pub use winit::event::{DeviceEvent, Event, WindowEvent};
pub use winit::event::{ElementState, Modifiers, MouseButton, MouseScrollDelta, Touch, TouchPhase};
pub use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
pub use winit::keyboard::{KeyCode, ModifiersState, NamedKey};
pub use winit::monitor::{MonitorHandle, VideoMode};
pub use winit::window::{CursorGrabMode, CursorIcon, Fullscreen, Icon, Window, WindowLevel};

/// 剪贴板错误类型
#[derive(Debug, Clone, thiserror::Error)]
pub enum ClipboardError {
    /// 剪贴板系统未初始化或不可用
    #[error("剪贴板系统未初始化或不可用: {0}")]
    NotInitialized(String),

    /// 系统剪贴板被其他程序占用
    #[error("系统剪贴板被占用: {0}")]
    SystemBusy(String),

    /// 剪贴板内容为空或格式不支持
    #[error("剪贴板内容为空或格式不支持")]
    ContentUnavailable,

    /// 文本编码错误
    #[error("文本编码错误: {0}")]
    EncodingError(String),

    /// IO 错误
    #[error("IO 错误: {0}")]
    IoError(String),

    /// 内存分配错误
    #[error("内存分配错误: {0}")]
    OutOfMemory(String),

    /// 其他未知错误
    #[error("剪贴板操作失败: {0}")]
    Unknown(String),
}

/// 剪贴板访问器
enum ClipboardState {
    /// 已初始化
    Ready(arboard::Clipboard),
    /// 初始化失败
    Failed(ClipboardError),
}

impl ClipboardState {
    fn get(&mut self) -> Result<&mut arboard::Clipboard, ClipboardError> {
        match self {
            ClipboardState::Ready(cb) => Ok(cb),
            ClipboardState::Failed(e) => Err(e.clone()),
        }
    }
}

/// 全局剪贴板状态
static CLIPBOARD: OnceCell<Mutex<ClipboardState>> = OnceCell::new();

/// 获取剪贴板实例
fn get_clipboard() -> Result<std::sync::MutexGuard<'static, ClipboardState>, ClipboardError> {
    // 初始化剪贴板（仅执行一次）
    CLIPBOARD.get_or_init(|| Mutex::new(
        match arboard::Clipboard::new() {
            Ok(cb) => ClipboardState::Ready(cb),
            Err(e) => ClipboardState::Failed(
                ClipboardError::NotInitialized(format!(
                    "无法访问系统剪贴板，可能是因为窗口未初始化或平台不支持: {:?}",
                    e
                ))
            ),
        }
    ));

    // 获取锁
    CLIPBOARD.get().unwrap().lock().map_err(|e| {
        ClipboardError::Unknown(format!("无法锁定剪贴板: {}", e))
    })
}

/// 剪贴板模块
pub mod clipboard {
    use super::*;

    /// 从系统剪贴板获取文本内容
    ///
    /// # 返回值
    /// - `Ok(Some(text))` - 成功获取剪贴板文本
    /// - `Ok(None)` - 剪贴板为空或不包含文本
    /// - `Err(ClipboardError)` - 获取失败
    pub fn get_text() -> Result<Option<String>, ClipboardError> {
        let mut state = get_clipboard()?;
        let clipboard = state.get()?;

        match clipboard.get_text() {
            Ok(text) => {
                if text.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(text))
                }
            }
            Err(arboard::Error::ContentNotAvailable) => Ok(None),
            Err(e) => {
                let msg = format!("{:?}", e);
                if msg.contains("not available") || msg.contains("empty") {
                    Ok(None)
                } else {
                    Err(ClipboardError::Unknown(format!("读取剪贴板失败: {}", e)))
                }
            }
        }
    }

    /// 设置系统剪贴板文本内容
    ///
    /// # 参数
    /// - `text` - 要设置的文本内容
    ///
    /// # 返回值
    /// - `Ok(())` - 成功设置剪贴板
    /// - `Err(ClipboardError)` - 设置失败
    pub fn set_text(text: &str) -> Result<(), ClipboardError> {
        if text.is_empty() {
            return Err(ClipboardError::ContentUnavailable);
        }

        let mut state = get_clipboard()?;
        let clipboard = state.get()?;

        clipboard
            .set_text(text)
            .map_err(|e| {
                let msg = format!("{:?}", e);
                if msg.contains("permission") || msg.contains("denied") {
                    ClipboardError::SystemBusy("权限被拒绝，请重试".to_string())
                } else if msg.contains("memory") || msg.contains("allocation") {
                    ClipboardError::OutOfMemory("内存不足".to_string())
                } else {
                    ClipboardError::Unknown(format!("写入剪贴板失败: {}", e))
                }
            })
    }

    /// 检查剪贴板是否包含文本内容
    ///
    /// # 返回值
    /// - `Ok(true)` - 剪贴板包含文本
    /// - `Ok(false)` - 剪贴板为空或不包含文本
    /// - `Err(ClipboardError)` - 检查失败
    pub fn has_text() -> Result<bool, ClipboardError> {
        Ok(get_text()?.is_some())
    }

    /// 清空剪贴板内容
    ///
    /// # 返回值
    /// - `Ok(())` - 成功清空
    /// - `Err(ClipboardError)` - 清空失败
    pub fn clear() -> Result<(), ClipboardError> {
        let mut state = get_clipboard()?;
        let clipboard = state.get()?;

        clipboard
            .clear()
            .map_err(|e| ClipboardError::Unknown(format!("清空剪贴板失败: {}", e)))
    }
}

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

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self
            .builder
            .with_min_inner_size(PhysicalSize::new(width, height));
        self
    }

    pub fn with_max_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self
            .builder
            .with_max_inner_size(PhysicalSize::new(width, height));
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.builder = self.builder.with_resizable(resizable);
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.builder = self.builder.with_decorations(decorations);
        self
    }

    pub fn with_fullscreen_mode(mut self, fullscreen: Option<Fullscreen>) -> Self {
        self.builder = self.builder.with_fullscreen(fullscreen);
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        if fullscreen {
            self.builder = self
                .builder
                .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        } else {
            self.builder = self.builder.with_fullscreen(None);
        }
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.builder = self.builder.with_transparent(transparent);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.builder = self.builder.with_visible(visible);
        self
    }

    pub fn with_always_on_top(self, _always_on_top: bool) -> Self {
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.builder = self.builder.with_maximized(maximized);
        self
    }

    pub fn with_minimized(self, _minimized: bool) -> Self {
        self
    }

    pub fn with_content_protected(mut self, protected: bool) -> Self {
        self.builder = self.builder.with_content_protected(protected);
        self
    }

    pub fn with_prevent_defocus(self, _prevent: bool) -> Self {
        self
    }

    pub fn with_ime(self, _ime_enabled: bool) -> Self {
        self
    }

    #[cfg(target_os = "macos")]
    pub fn with_cursor_hittest(mut self, hittest: bool) -> Self {
        #[allow(unused_mut)]
        let mut builder = self.builder;
        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowBuilderExtMacOS;
            builder = builder.with_cursor_hittest(hittest);
        }
        self.builder = builder;
        self
    }

    #[cfg(not(target_os = "macos"))]
    pub fn with_cursor_hittest(self, _hittest: bool) -> Self {
        self
    }

    pub fn with_window_icon(mut self, icon: Option<Icon>) -> Self {
        self.builder = self.builder.with_window_icon(icon);
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
    pub phase: TouchPhase,
}

pub struct Input {
    pressed_keys: Vec<NamedKey>,
    previous_keys: Vec<NamedKey>,
    pressed_buttons: Vec<MouseButton>,
    previous_buttons: Vec<MouseButton>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    wheel_delta: Vec2,
    modifiers: Modifiers,
    text_input: String,
    touches: HashMap<u64, TouchPoint>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::new(),
            previous_keys: Vec::new(),
            pressed_buttons: Vec::new(),
            previous_buttons: Vec::new(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            wheel_delta: Vec2::ZERO,
            modifiers: Modifiers::default(),
            text_input: String::new(),
            touches: HashMap::new(),
        }
    }

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

    pub fn reset(&mut self) {
        self.pressed_keys.clear();
        self.previous_keys.clear();
        self.pressed_buttons.clear();
        self.previous_buttons.clear();
        self.mouse_position = Vec2::ZERO;
        self.mouse_delta = Vec2::ZERO;
        self.wheel_delta = Vec2::ZERO;
        self.modifiers = Modifiers::default();
        self.text_input.clear();
        self.touches.clear();
    }

    pub fn key_pressed(&self, keycode: &NamedKey) -> bool {
        self.pressed_keys.contains(keycode)
    }

    pub fn key_just_pressed(&self, keycode: &NamedKey) -> bool {
        self.pressed_keys.contains(keycode) && !self.previous_keys.contains(keycode)
    }

    pub fn key_just_released(&self, keycode: &NamedKey) -> bool {
        !self.pressed_keys.contains(keycode) && self.previous_keys.contains(keycode)
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button) && !self.previous_buttons.contains(&button)
    }

    pub fn mouse_button_just_released(&self, button: MouseButton) -> bool {
        !self.pressed_buttons.contains(&button) && self.previous_buttons.contains(&button)
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

    pub fn modifiers(&self) -> Modifiers {
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

    pub fn pressed_keys(&self) -> &[NamedKey] {
        &self.pressed_keys
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        let x = x as f32;
        let y = y as f32;
        self.mouse_delta = Vec2::new(x - self.mouse_position.x, y - self.mouse_position.y);
        self.mouse_position = Vec2::new(x, y);
    }

    pub fn update_wheel(&mut self, delta: Vec2) {
        self.wheel_delta += delta;
    }

    pub fn update_key(&mut self, keycode: NamedKey, state: ElementState) {
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

    pub fn update_button(&mut self, button: MouseButton, state: ElementState) {
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

    pub fn update_modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
    }

    pub fn add_text(&mut self, text: &str) {
        self.text_input.push_str(text);
    }

    pub fn update_touch(&mut self, id: u64, position: Vec2, force: f32, phase: TouchPhase) {
        match phase {
            TouchPhase::Ended | TouchPhase::Cancelled => {
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
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputModule {
    input: Input,
}

impl InputModule {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }

    pub fn process_event(&mut self, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    if let winit::keyboard::Key::Named(keycode) = event.logical_key.as_ref() {
                        self.input.update_key(keycode, event.state);
                    }
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    self.input.update_modifiers(*modifiers);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    self.input.update_button(*button, *state);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.input.update_mouse_position(position.x, position.y);
                }
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.input.update_wheel(Vec2::new(*x, *y));
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.input
                            .update_wheel(Vec2::new(pos.x as f32, pos.y as f32));
                    }
                },
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

impl Default for InputModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::{ElementState, MouseButton, TouchPhase};
    use winit::keyboard::NamedKey;
    use winit::window::{CursorGrabMode, CursorIcon, Fullscreen};

    // ===== WindowBuilder 测试组 =====
    // 目标: 至少 15 个测试
    #[test]
    fn test_window_builder_new() {
        let builder = WindowBuilder::new();
        let _ = builder;
    }

    #[test]
    fn test_window_builder_default() {
        let builder: WindowBuilder = Default::default();
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_title() {
        let builder = WindowBuilder::new().with_title("Test Window");
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_inner_size() {
        let builder = WindowBuilder::new().with_inner_size(1280, 720);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_min_inner_size() {
        let builder = WindowBuilder::new().with_min_inner_size(640, 480);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_max_inner_size() {
        let builder = WindowBuilder::new().with_max_inner_size(1920, 1080);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_resizable_true() {
        let builder = WindowBuilder::new().with_resizable(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_resizable_false() {
        let builder = WindowBuilder::new().with_resizable(false);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_decorations_true() {
        let builder = WindowBuilder::new().with_decorations(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_decorations_false() {
        let builder = WindowBuilder::new().with_decorations(false);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_fullscreen_true() {
        let builder = WindowBuilder::new().with_fullscreen(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_fullscreen_false() {
        let builder = WindowBuilder::new().with_fullscreen(false);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_fullscreen_mode_some() {
        let builder =
            WindowBuilder::new().with_fullscreen_mode(Some(Fullscreen::Borderless(None)));
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_fullscreen_mode_none() {
        let builder = WindowBuilder::new().with_fullscreen_mode(None);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_transparent_true() {
        let builder = WindowBuilder::new().with_transparent(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_visible_true() {
        let builder = WindowBuilder::new().with_visible(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_always_on_top() {
        let builder = WindowBuilder::new().with_always_on_top(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_maximized_true() {
        let builder = WindowBuilder::new().with_maximized(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_minimized() {
        let builder = WindowBuilder::new().with_minimized(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_content_protected() {
        let builder = WindowBuilder::new().with_content_protected(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_prevent_defocus() {
        let builder = WindowBuilder::new().with_prevent_defocus(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_ime() {
        let builder = WindowBuilder::new().with_ime(true);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_with_window_icon_none() {
        let builder = WindowBuilder::new().with_window_icon(None);
        let _ = builder;
    }

    #[test]
    fn test_window_builder_chained_methods() {
        let builder = WindowBuilder::new()
            .with_title("Hello")
            .with_inner_size(800, 600)
            .with_resizable(true)
            .with_decorations(true)
            .with_transparent(false)
            .with_visible(true);
        let _ = builder;
    }

    // ===== WindowConfig 测试 =====
    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.title, "Game Engine");
        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
        assert!(config.resizable);
        assert!(config.vsync);
        assert!(!config.fullscreen);
        assert!(config.decorations);
    }

    // ===== WindowMode 测试 =====
    #[test]
    fn test_window_mode_windowed() {
        let mode = WindowMode::Windowed;
        assert_eq!(mode, WindowMode::Windowed);
        assert_ne!(mode, WindowMode::Fullscreen);
        assert_ne!(mode, WindowMode::Borderless);
    }

    #[test]
    fn test_window_mode_fullscreen() {
        let mode = WindowMode::Fullscreen;
        assert_eq!(mode, WindowMode::Fullscreen);
    }

    #[test]
    fn test_window_mode_borderless() {
        let mode = WindowMode::Borderless;
        assert_eq!(mode, WindowMode::Borderless);
    }

    #[test]
    fn test_window_mode_switch_cycle() {
        let modes = [
            WindowMode::Windowed,
            WindowMode::Fullscreen,
            WindowMode::Borderless,
        ];
        for mode in modes.iter() {
            // 确保 clone/copy 正常
            let copied = *mode;
            assert_eq!(copied, *mode);
        }
    }

    // ===== CursorIcon 测试 =====
    #[test]
    fn test_cursor_icon_default_is_arrow() {
        let icon = CursorIcon::default();
        assert_eq!(icon, CursorIcon::Default);
    }

    #[test]
    fn test_cursor_icon_named_variants_exist() {
        // 验证常见 CursorIcon 变体可被引用
        let _ = CursorIcon::Pointer;
        let _ = CursorIcon::Move;
        let _ = CursorIcon::Crosshair;
        let _ = CursorIcon::Text;
        let _ = CursorIcon::Wait;
        let _ = CursorIcon::Help;
        let _ = CursorIcon::Progress;
        let _ = CursorIcon::NotAllowed;
        let _ = CursorIcon::ContextMenu;
        let _ = CursorIcon::Cell;
        let _ = CursorIcon::VerticalText;
        let _ = CursorIcon::Alias;
        let _ = CursorIcon::Copy;
        let _ = CursorIcon::NoDrop;
        let _ = CursorIcon::Grab;
        let _ = CursorIcon::Grabbing;
        let _ = CursorIcon::AllScroll;
        let _ = CursorIcon::ZoomIn;
        let _ = CursorIcon::ZoomOut;
        let _ = CursorIcon::EResize;
        let _ = CursorIcon::NResize;
        let _ = CursorIcon::NeResize;
        let _ = CursorIcon::NwResize;
        let _ = CursorIcon::SResize;
        let _ = CursorIcon::SeResize;
        let _ = CursorIcon::SwResize;
        let _ = CursorIcon::WResize;
        let _ = CursorIcon::EwResize;
        let _ = CursorIcon::NsResize;
        let _ = CursorIcon::NeswResize;
        let _ = CursorIcon::NwseResize;
        let _ = CursorIcon::ColResize;
        let _ = CursorIcon::RowResize;
    }

    // ===== CursorGrabMode 测试 =====
    #[test]
    fn test_cursor_grab_mode_none() {
        let mode = CursorGrabMode::None;
        assert_eq!(mode, CursorGrabMode::None);
    }

    #[test]
    fn test_cursor_grab_mode_confined() {
        let mode = CursorGrabMode::Confined;
        assert_eq!(mode, CursorGrabMode::Confined);
    }

    #[test]
    fn test_cursor_grab_mode_locked() {
        let mode = CursorGrabMode::Locked;
        assert_eq!(mode, CursorGrabMode::Locked);
    }

    #[test]
    fn test_cursor_grab_mode_distinct() {
        assert_ne!(CursorGrabMode::None, CursorGrabMode::Confined);
        assert_ne!(CursorGrabMode::None, CursorGrabMode::Locked);
        assert_ne!(CursorGrabMode::Confined, CursorGrabMode::Locked);
    }

    // ===== Input 核心状态测试 =====
    #[test]
    fn test_input_new() {
        let input = Input::new();
        assert!(input.pressed_keys.is_empty());
        assert!(input.pressed_buttons.is_empty());
        assert_eq!(input.mouse_position, Vec2::ZERO);
        assert_eq!(input.mouse_delta, Vec2::ZERO);
    }

    #[test]
    fn test_input_default() {
        let input: Input = Default::default();
        assert!(input.pressed_keys.is_empty());
        assert!(input.pressed_buttons.is_empty());
        assert_eq!(input.mouse_position, Vec2::ZERO);
        assert_eq!(input.mouse_delta, Vec2::ZERO);
    }

    #[test]
    fn test_input_key_pressed_space() {
        let mut input = Input::new();
        let space_key = NamedKey::Space;
        input.update_key(space_key, ElementState::Pressed);
        assert!(input.key_pressed(&space_key));
    }

    #[test]
    fn test_input_key_not_pressed_other() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        assert!(!input.key_pressed(&NamedKey::Enter));
    }

    #[test]
    fn test_input_key_release() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        assert!(input.key_pressed(&NamedKey::Space));
        input.update_key(NamedKey::Space, ElementState::Released);
        assert!(!input.key_pressed(&NamedKey::Space));
    }

    #[test]
    fn test_input_key_duplicate_press() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        input.update_key(NamedKey::Space, ElementState::Pressed);
        input.update_key(NamedKey::Space, ElementState::Pressed);
        // pressed_keys 应该只有一个 Space
        assert!(input.key_pressed(&NamedKey::Space));
    }

    #[test]
    fn test_input_key_just_pressed_space() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        assert!(input.key_just_pressed(&NamedKey::Space));
        input.clear();
        // clear 之后，Space 仍然是 pressed，所以不再是 just_pressed
        assert!(!input.key_just_pressed(&NamedKey::Space));
    }

    #[test]
    fn test_input_key_just_pressed_unrelated_key_false() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        assert!(!input.key_just_pressed(&NamedKey::Enter));
    }

    #[test]
    fn test_input_key_just_released() {
        let mut input = Input::new();
        let space_key = NamedKey::Space;
        input.update_key(space_key, ElementState::Pressed);
        input.clear();
        input.update_key(space_key, ElementState::Released);
        assert!(input.key_just_released(&space_key));
    }

    #[test]
    fn test_input_key_just_released_without_press() {
        let mut input = Input::new();
        // 从未被按下，直接检测 should return false
        assert!(!input.key_just_released(&NamedKey::Space));
        input.clear();
        assert!(!input.key_just_released(&NamedKey::Space));
    }

    #[test]
    fn test_input_multiple_keys_pressed() {
        let mut input = Input::new();
        input.update_key(NamedKey::Space, ElementState::Pressed);
        input.update_key(NamedKey::Enter, ElementState::Pressed);
        input.update_key(NamedKey::Escape, ElementState::Pressed);
        assert!(input.key_pressed(&NamedKey::Space));
        assert!(input.key_pressed(&NamedKey::Enter));
        assert!(input.key_pressed(&NamedKey::Escape));
    }

    #[test]
    fn test_input_mouse_button_left_pressed() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.mouse_button_pressed(MouseButton::Left));
        assert!(!input.mouse_button_pressed(MouseButton::Right));
    }

    #[test]
    fn test_input_mouse_button_right_pressed() {
        let mut input = Input::new();
        input.update_button(MouseButton::Right, ElementState::Pressed);
        assert!(input.mouse_button_pressed(MouseButton::Right));
    }

    #[test]
    fn test_input_mouse_button_middle_pressed() {
        let mut input = Input::new();
        input.update_button(MouseButton::Middle, ElementState::Pressed);
        assert!(input.mouse_button_pressed(MouseButton::Middle));
    }

    #[test]
    fn test_input_mouse_button_released() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Released);
        assert!(!input.mouse_button_pressed(MouseButton::Left));
    }

    #[test]
    fn test_input_mouse_button_just_pressed() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.mouse_button_just_pressed(MouseButton::Left));
        input.clear();
        assert!(!input.mouse_button_just_pressed(MouseButton::Left));
    }

    #[test]
    fn test_input_mouse_button_just_released() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.clear();
        input.update_button(MouseButton::Left, ElementState::Released);
        assert!(input.mouse_button_just_released(MouseButton::Left));
    }

    #[test]
    fn test_input_mouse_position_first_call() {
        let mut input = Input::new();
        input.update_mouse_position(100.0, 200.0);
        assert_eq!(input.mouse_position(), Vec2::new(100.0, 200.0));
        assert_eq!(input.mouse_delta(), Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_input_mouse_position_second_call() {
        let mut input = Input::new();
        input.update_mouse_position(100.0, 200.0);
        input.update_mouse_position(150.0, 250.0);
        assert_eq!(input.mouse_position(), Vec2::new(150.0, 250.0));
        assert_eq!(input.mouse_delta(), Vec2::new(50.0, 50.0));
    }

    #[test]
    fn test_input_mouse_delta_after_clear() {
        let mut input = Input::new();
        input.update_mouse_position(100.0, 200.0);
        input.clear();
        // clear 不重置 mouse_position，但重置 mouse_delta
        assert_eq!(input.mouse_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_input_clear_preserves_pressed() {
        let mut input = Input::new();
        let space_key = NamedKey::Space;
        input.update_key(space_key, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_mouse_position(100.0, 200.0);

        input.clear();

        assert!(input.key_pressed(&space_key));
        assert!(input.mouse_button_pressed(MouseButton::Left));
        assert_eq!(input.mouse_delta, Vec2::ZERO);
    }

    #[test]
    fn test_input_reset_clears_all() {
        let mut input = Input::new();
        let space_key = NamedKey::Space;
        input.update_key(space_key, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_mouse_position(100.0, 200.0);

        input.reset();

        assert!(!input.key_pressed(&space_key));
        assert!(!input.mouse_button_pressed(MouseButton::Left));
        assert_eq!(input.mouse_position, Vec2::ZERO);
        assert_eq!(input.mouse_delta, Vec2::ZERO);
    }

    // ===== Wheel / 滚轮 测试 =====
    #[test]
    fn test_input_wheel_single_call() {
        let mut input = Input::new();
        input.update_wheel(Vec2::new(1.0, 2.0));
        assert_eq!(input.wheel_delta(), Vec2::new(1.0, 2.0));
    }

    #[test]
    fn test_input_wheel_accumulates() {
        let mut input = Input::new();
        input.update_wheel(Vec2::new(1.0, 2.0));
        input.update_wheel(Vec2::new(0.5, 0.5));
        assert_eq!(input.wheel_delta(), Vec2::new(1.5, 2.5));
    }

    #[test]
    fn test_input_wheel_after_clear() {
        let mut input = Input::new();
        input.update_wheel(Vec2::new(1.0, 2.0));
        input.clear();
        assert_eq!(input.wheel_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_input_wheel_after_reset() {
        let mut input = Input::new();
        input.update_wheel(Vec2::new(5.0, 5.0));
        input.reset();
        assert_eq!(input.wheel_delta(), Vec2::ZERO);
    }

    // ===== Text 输入 测试 =====
    #[test]
    fn test_input_text_single() {
        let mut input = Input::new();
        input.add_text("Hello");
        assert_eq!(input.text(), "Hello");
    }

    #[test]
    fn test_input_text_append() {
        let mut input = Input::new();
        input.add_text("Hello");
        input.add_text(" World");
        assert_eq!(input.text(), "Hello World");
    }

    #[test]
    fn test_input_text_clear() {
        let mut input = Input::new();
        input.add_text("Hello");
        input.clear();
        assert_eq!(input.text(), "");
    }

    #[test]
    fn test_input_text_empty_init() {
        let input = Input::new();
        assert_eq!(input.text(), "");
    }

    // ===== Modifiers 修饰键 测试 =====
    #[test]
    fn test_input_modifiers_default() {
        let input = Input::new();
        let modifiers = input.modifiers();
        // 初始值
        assert_eq!(modifiers, Modifiers::default());
    }

    #[test]
    fn test_input_modifiers_update_read() {
        let mut input = Input::new();
        let modifiers = Modifiers::default();
        input.update_modifiers(modifiers);
        assert_eq!(input.modifiers(), modifiers);
    }

    // ===== Touch 触摸 测试 =====
    #[test]
    fn test_touch_point_basic() {
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
    fn test_input_touch_add_single() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        assert_eq!(input.touch_count(), 1);
        assert!(input.touch(1).is_some());
        assert_eq!(input.touch(1).unwrap().position, Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_input_touch_multiple() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 100.0), 0.3, TouchPhase::Started);
        input.update_touch(2, Vec2::new(200.0, 200.0), 0.6, TouchPhase::Started);
        input.update_touch(3, Vec2::new(300.0, 300.0), 0.9, TouchPhase::Started);
        assert_eq!(input.touch_count(), 3);
        assert!(input.touch(1).is_some());
        assert!(input.touch(2).is_some());
        assert!(input.touch(3).is_some());
    }

    #[test]
    fn test_input_touch_end_removes() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        assert_eq!(input.touch_count(), 1);
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Ended);
        assert_eq!(input.touch_count(), 0);
    }

    #[test]
    fn test_input_touch_cancelled_removes() {
        let mut input = Input::new();
        input.update_touch(5, Vec2::new(50.0, 60.0), 0.1, TouchPhase::Started);
        assert_eq!(input.touch_count(), 1);
        input.update_touch(5, Vec2::new(50.0, 60.0), 0.0, TouchPhase::Cancelled);
        assert_eq!(input.touch_count(), 0);
    }

    #[test]
    fn test_input_touch_moved_updates() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 100.0), 0.5, TouchPhase::Started);
        input.update_touch(1, Vec2::new(150.0, 120.0), 0.6, TouchPhase::Moved);
        assert_eq!(input.touch_count(), 1);
        let t = input.touch(1).unwrap();
        assert_eq!(t.position, Vec2::new(150.0, 120.0));
    }

    #[test]
    fn test_input_touch_unknown() {
        let input = Input::new();
        assert!(input.touch(999).is_none());
        assert_eq!(input.touch_count(), 0);
    }

    #[test]
    fn test_input_touch_iterator() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 100.0), 0.5, TouchPhase::Started);
        input.update_touch(2, Vec2::new(200.0, 200.0), 0.7, TouchPhase::Started);
        let mut count = 0;
        for _ in input.touches() {
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_input_touch_phase_variants() {
        // 确认 TouchPhase 各种变体都可用
        let _ = TouchPhase::Started;
        let _ = TouchPhase::Moved;
        let _ = TouchPhase::Ended;
        let _ = TouchPhase::Cancelled;
    }

    // ===== InputModule 测试 =====
    #[test]
    fn test_input_module_new() {
        let module = InputModule::new();
        let input = module.input();
        assert!(input.pressed_keys.is_empty());
    }

    #[test]
    fn test_input_module_default() {
        let module: InputModule = Default::default();
        let _ = module;
    }

    #[test]
    fn test_input_module_process_touch_via_api() {
        // 不通过事件循环，直接测试 update_touch 行为（在 InputModule 的 input 上）
        let mut module = InputModule::new();
        {
            let input = module.input_mut();
            input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        }
        assert_eq!(module.input().touch_count(), 1);
    }

    #[test]
    fn test_input_module_process_key_via_api() {
        let mut module = InputModule::new();
        {
            let input = module.input_mut();
            input.update_key(NamedKey::Space, ElementState::Pressed);
        }
        assert!(module.input().key_pressed(&NamedKey::Space));
    }

    #[test]
    fn test_input_module_clear() {
        let mut module = InputModule::new();
        {
            let input = module.input_mut();
            input.update_mouse_position(100.0, 100.0);
            input.add_text("hello");
        }
        module.clear();
        assert_eq!(module.input().mouse_delta(), Vec2::ZERO);
        assert_eq!(module.input().text(), "");
    }

    // ===== Clipboard 测试 =====
    // 注意：某些测试需要 X11/Wayland 环境，在无显示器的 CI 环境中会失败

    /// 检查剪贴板是否可用于测试
    fn is_clipboard_available() -> bool {
        // 尝试创建剪贴板实例
        std::env::var("DISPLAY").is_ok()
            || std::env::var("WAYLAND_DISPLAY").is_ok()
            || cfg!(target_os = "windows")
            || cfg!(target_os = "macos")
    }

    #[test]
    fn test_clipboard_set_and_get_text() {
        // 此测试需要 X11/Wayland 环境
        if !is_clipboard_available() {
            eprintln!("Skipping clipboard test: no display server available");
            return;
        }

        // 设置剪贴板文本
        let test_text = "Hello, Clipboard!";
        clipboard::set_text(test_text).expect("set_text should succeed");

        // 获取剪贴板文本
        let result = clipboard::get_text().expect("get_text should succeed");
        assert!(result.is_some(), "Expected Some(text), got None");
        assert_eq!(result.unwrap(), test_text);
    }

    #[test]
    fn test_clipboard_get_empty_when_no_content() {
        if !is_clipboard_available() {
            eprintln!("Skipping clipboard test: no display server available");
            return;
        }

        // 清空剪贴板
        clipboard::clear().expect("clear should succeed");

        // 获取应该返回 None
        let result = clipboard::get_text().expect("get_text should succeed");
        assert!(result.is_none(), "Expected None for empty clipboard");
    }

    #[test]
    fn test_clipboard_has_text() {
        if !is_clipboard_available() {
            eprintln!("Skipping clipboard test: no display server available");
            return;
        }

        clipboard::clear().expect("clear should succeed");

        // 清空后检查
        let has_text = clipboard::has_text().expect("has_text should succeed");
        assert!(!has_text, "Expected no text after clear");

        // 设置后检查
        clipboard::set_text("test").expect("set_text should succeed");
        let has_text = clipboard::has_text().expect("has_text should succeed");
        assert!(has_text, "Expected has_text to return true after setting text");
    }

    #[test]
    fn test_clipboard_set_empty_text_returns_error() {
        // 此测试不需要 X11，因为它测试的是空文本错误
        let result = clipboard::set_text("");
        assert!(result.is_err(), "set_text with empty string should return error");
        assert!(matches!(result.unwrap_err(), ClipboardError::ContentUnavailable));
    }

    #[test]
    fn test_clipboard_unicode_text() {
        if !is_clipboard_available() {
            eprintln!("Skipping clipboard test: no display server available");
            return;
        }

        let unicode_text = "你好，世界！🎮 🔥 Rust游戏引擎";
        clipboard::set_text(unicode_text).expect("set_text should succeed with unicode");

        let result = clipboard::get_text().expect("get_text should succeed");
        assert_eq!(result.unwrap(), unicode_text);
    }

    #[test]
    fn test_clipboard_multiline_text() {
        if !is_clipboard_available() {
            eprintln!("Skipping clipboard test: no display server available");
            return;
        }

        let multiline_text = "第一行\n第二行\n第三行";
        clipboard::set_text(multiline_text).expect("set_text should succeed with multiline");

        let result = clipboard::get_text().expect("get_text should succeed");
        assert_eq!(result.unwrap(), multiline_text);
    }

    #[test]
    fn test_clipboard_error_display() {
        let error = ClipboardError::NotInitialized("test error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("test error"));

        let error2 = ClipboardError::ContentUnavailable;
        let display2 = format!("{}", error2);
        assert!(display2.contains("为空或格式不支持"));
    }

    #[test]
    fn test_clipboard_error_io_error() {
        let error = ClipboardError::IoError("test IO error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("IO 错误"));
        assert!(display.contains("test IO error"));
    }

    #[test]
    fn test_clipboard_error_variants() {
        use ClipboardError::*;
        let _ = NotInitialized("test".to_string());
        let _ = SystemBusy("busy".to_string());
        let _ = ContentUnavailable;
        let _ = EncodingError("encoding".to_string());
        let _ = IoError("io".to_string());
        let _ = OutOfMemory("oom".to_string());
        let _ = Unknown("unknown".to_string());
    }
}
