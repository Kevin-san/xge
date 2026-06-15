//! engine-window crate - 窗口系统与事件循环模块
//!
//! 提供窗口、事件循环和输入抽象

// Re-exports for convenience
pub use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position};
pub use winit::event::{DeviceEvent, Event, WindowEvent};
pub use winit::event::{ElementState, MouseButton, MouseScrollDelta, Touch, TouchPhase};
pub use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
pub use winit::keyboard::{KeyCode, ModifiersState};
pub use winit::monitor::{MonitorHandle, VideoMode};
pub use winit::window::{CursorGrabMode, CursorIcon, Fullscreen, Icon, Window, WindowLevel};

/// WindowBuilder 窗口构建器
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

    pub fn build(self, event_loop: &EventLoop<()>) -> Result<Window, Box<dyn std::error::Error>> {
        self.builder.build(event_loop).map_err(|e| e.into())
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Clipboard placeholder
pub mod clipboard {
    pub fn get_text() -> Option<String> {
        None
    }
    #[allow(clippy::result_unit_err)]
    pub fn set_text(_text: &str) -> Result<(), ()> {
        Err(())
    }
}

/// WindowConfig 窗口配置
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

/// WindowMode 窗口模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}

/// Input 输入状态管理
pub struct Input {
    pressed_keys: Vec<KeyCode>,
    previous_keys: Vec<KeyCode>,
    pressed_buttons: Vec<MouseButton>,
    previous_buttons: Vec<MouseButton>,
    mouse_position: (f64, f64),
    mouse_delta: (f64, f64),
    modifiers: ModifiersState,
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed_keys: Vec::new(),
            previous_keys: Vec::new(),
            pressed_buttons: Vec::new(),
            previous_buttons: Vec::new(),
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            modifiers: ModifiersState::empty(),
        }
    }

    pub fn clear(&mut self) {
        self.previous_keys.clear();
        self.previous_keys.extend(self.pressed_keys.iter().copied());
        self.previous_buttons.clear();
        self.previous_buttons
            .extend(self.pressed_buttons.iter().copied());
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn reset(&mut self) {
        self.pressed_keys.clear();
        self.previous_keys.clear();
        self.pressed_buttons.clear();
        self.previous_buttons.clear();
        self.mouse_position = (0.0, 0.0);
        self.mouse_delta = (0.0, 0.0);
        self.modifiers = ModifiersState::empty();
    }

    pub fn key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn key_just_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode) && !self.previous_keys.contains(&keycode)
    }

    pub fn key_just_released(&self, keycode: KeyCode) -> bool {
        !self.pressed_keys.contains(&keycode) && self.previous_keys.contains(&keycode)
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

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn modifiers(&self) -> ModifiersState {
        self.modifiers
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        self.mouse_delta = (x - self.mouse_position.0, y - self.mouse_position.1);
        self.mouse_position = (x, y);
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
