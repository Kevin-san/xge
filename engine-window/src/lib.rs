//! engine-window crate - 窗口系统与事件循环模块
//!
//! 提供窗口、事件循环和输入抽象

use engine_math::Vec2;
use std::collections::HashMap;

pub use winit::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position};
pub use winit::event::{DeviceEvent, Event, WindowEvent};
pub use winit::event::{ElementState, MouseButton, MouseScrollDelta, Touch, TouchPhase, Modifiers};
pub use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
pub use winit::keyboard::{KeyCode, ModifiersState, NamedKey};
pub use winit::monitor::{MonitorHandle, VideoMode};
pub use winit::window::{CursorGrabMode, CursorIcon, Fullscreen, Icon, Window, WindowLevel};

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

pub mod clipboard {
    pub fn get_text() -> Option<String> {
        None
    }
    #[allow(clippy::result_unit_err)]
    pub fn set_text(_text: &str) -> Result<(), ()> {
        Err(())
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
        self.previous_buttons.extend(self.pressed_buttons.iter().copied());
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
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            self.input.update_wheel(Vec2::new(*x, *y));
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            self.input.update_wheel(Vec2::new(pos.x as f32, pos.y as f32));
                        }
                    }
                }
                WindowEvent::Touch(touch) => {
                    let position = Vec2::new(touch.location.x as f32, touch.location.y as f32);
                    let force = match touch.force {
                        Some(winit::event::Force::Normalized(f)) => f as f32,
                        _ => 0.0,
                    };
                    self.input.update_touch(
                        touch.id,
                        position,
                        force,
                        touch.phase,
                    );
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
    use winit::keyboard::NamedKey;
    use winit::event::{ElementState, MouseButton};

    

    #[test]
    fn test_input_new() {
        let input = Input::new();
        assert!(input.pressed_keys.is_empty());
        assert!(input.pressed_buttons.is_empty());
        assert_eq!(input.mouse_position, Vec2::ZERO);
        assert_eq!(input.mouse_delta, Vec2::ZERO);
    }

    #[test]
    fn test_input_key_pressed() {
        let mut input = Input::new();
        let space_key = NamedKey::Space;
        let enter_key = NamedKey::Enter;
        input.update_key(space_key, ElementState::Pressed);
        assert!(input.key_pressed(&space_key));
        assert!(!input.key_pressed(&enter_key));
    }

    #[test]
    fn test_input_key_just_pressed() {
        let mut input = Input::new();
        let space_key = NamedKey::Space;
        let enter_key = NamedKey::Enter;
        input.update_key(space_key, ElementState::Pressed);
        assert!(input.key_just_pressed(&space_key));
        assert!(!input.key_just_pressed(&enter_key));
        input.clear();
        assert!(!input.key_just_pressed(&space_key));
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
    fn test_input_reset() {
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

    #[test]
    fn test_input_wheel() {
        let mut input = Input::new();
        input.update_wheel(Vec2::new(1.0, 2.0));
        assert_eq!(input.wheel_delta(), Vec2::new(1.0, 2.0));
        
        input.update_wheel(Vec2::new(0.5, 0.5));
        assert_eq!(input.wheel_delta(), Vec2::new(1.5, 2.5));
        
        input.clear();
        assert_eq!(input.wheel_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_input_modifiers() {
        let mut input = Input::new();
        let modifiers = Modifiers::default();
        input.update_modifiers(modifiers);
        assert_eq!(input.modifiers(), modifiers);
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
}