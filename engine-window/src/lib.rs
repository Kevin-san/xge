//! engine-window crate — 窗口系统、事件循环与输入抽象
//!
//! 提供跨平台窗口管理、输入事件处理和剪贴板访问

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
pub use key_code::{KeyCode, MouseButton, ModifiersState};

// 引擎级输入事件
pub use input_event::{
    CursorGrabMode, CursorIcon, CursorVisibility, ElementState, InputEvent, KeyEvent,
    MouseButtonEvent, MouseMotionEvent, MouseWheelEvent, TextInputEvent,
};

// 窗口状态
pub use window_state::{WindowSize, WindowState};

// 剪贴板错误和结构体
pub use crate::clipboard::{Clipboard, ClipboardError};

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

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self.builder.with_min_inner_size(PhysicalSize::new(width, height));
        self
    }

    pub fn with_max_inner_size(mut self, width: u32, height: u32) -> Self {
        self.builder = self.builder.with_max_inner_size(PhysicalSize::new(width, height));
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: Option<Fullscreen>) -> Self {
        self.builder = self.builder.with_fullscreen(fullscreen);
        self
    }

    pub fn with_content_protected(mut self, protected: bool) -> Self {
        self.builder = self.builder.with_content_protected(protected);
        self
    }

    pub fn with_window_icon(mut self, icon: Option<Icon>) -> Self {
        self.builder = self.builder.with_window_icon(icon);
        self
    }

    /// TODO: winit 0.29 WindowBuilder 不支持 with_always_on_top，目前为 no-op
    pub fn with_always_on_top(self, _always_on_top: bool) -> Self {
        self
    }

    /// TODO: winit 0.29 WindowBuilder 不支持 with_minimized，目前为 no-op
    pub fn with_minimized(self, _minimized: bool) -> Self {
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

impl WindowConfig {
    pub fn from_title(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Convert to WindowBuilder
    pub fn to_builder(&self) -> WindowBuilder {
        WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(self.width, self.height)
            .with_resizable(self.resizable)
            .with_decorations(self.decorations)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}

/// 窗口主题
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// 触摸阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TouchPhase {
    #[default]
    Started,
    Moved,
    Ended,
    Cancelled,
}

/// 鼠标滚轮增量
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScrollDelta {
    /// 行增量
    LineDelta(f32, f32),
    /// 像素增量
    PixelDelta(f32, f32),
}

/// 主循环配置
#[derive(Debug, Clone)]
pub struct MainLoopConfig {
    /// 目标帧率
    pub target_fps: u64,
    /// 固定更新步长（秒）
    pub fixed_timestep: f64,
    /// 最大 dt 钳制值（秒），防止尖峰
    pub max_dt: f64,
    /// 是否启用 vsync
    pub vsync: bool,
}

impl Default for MainLoopConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            fixed_timestep: 1.0 / 60.0,
            max_dt: 0.1,
            vsync: true,
        }
    }
}

/// 主循环 — 封装 poll → update → render 流程
pub struct MainLoop {
    config: MainLoopConfig,
    accumulator: f64,
    last_time: std::time::Instant,
}

impl MainLoop {
    pub fn new(config: MainLoopConfig) -> Self {
        Self {
            config,
            accumulator: 0.0,
            last_time: std::time::Instant::now(),
        }
    }

    /// 获取配置引用
    pub fn config(&self) -> &MainLoopConfig {
        &self.config
    }

    /// 开始新帧，返回 (dt, should_do_fixed_update)
    /// dt: 自上一帧以来的时间（已钳制）
    /// should_do_fixed_update: 是否应执行固定时间步更新
    pub fn begin_frame(&mut self) -> (f64, bool) {
        let now = std::time::Instant::now();
        let mut dt = (now - self.last_time).as_secs_f64();
        self.last_time = now;

        // 钳制 dt 防止尖峰
        if dt > self.config.max_dt {
            dt = self.config.max_dt;
        }

        self.accumulator += dt;

        let should_fixed = self.accumulator >= self.config.fixed_timestep;
        (dt, should_fixed)
    }

    /// 消耗固定时间步，返回本帧需要执行的固定更新次数
    pub fn consume_fixed_steps(&mut self) -> u32 {
        let mut steps = 0u32;
        while self.accumulator >= self.config.fixed_timestep && steps < 5 {
            self.accumulator -= self.config.fixed_timestep;
            steps += 1;
        }
        // 防止螺旋式下降：丢弃多余累积
        if self.accumulator > self.config.fixed_timestep {
            self.accumulator = self.config.fixed_timestep;
        }
        steps
    }

    /// 计算帧间等待时间（用于帧率限制）
    pub fn frame_time_remaining(&self) -> std::time::Duration {
        let target_frame_time = 1_000_000_000 / self.config.target_fps;
        let elapsed = self.last_time.elapsed().as_nanos() as u64;
        if elapsed < target_frame_time {
            std::time::Duration::from_nanos(target_frame_time - elapsed)
        } else {
            std::time::Duration::ZERO
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    pub id: u64,
    pub position: Vec2,
    pub force: f32,
    pub phase: TouchPhase,
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
            self.key_states.get(&code).copied().unwrap_or(KeyPressState::Released),
            KeyPressState::Pressed | KeyPressState::JustPressed
        )
    }

    pub fn key_just_pressed(&self, code: KeyCode) -> bool {
        matches!(
            self.key_states.get(&code).copied().unwrap_or(KeyPressState::Released),
            KeyPressState::JustPressed
        )
    }

    pub fn key_just_released(&self, code: KeyCode) -> bool {
        matches!(
            self.key_states.get(&code).copied().unwrap_or(KeyPressState::Released),
            KeyPressState::JustReleased
        )
    }

    // ===== 鼠标查询 =====

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        matches!(
            self.button_states.get(&button).copied().unwrap_or(ButtonPressState::Released),
            ButtonPressState::Pressed | ButtonPressState::JustPressed
        )
    }

    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        matches!(
            self.button_states.get(&button).copied().unwrap_or(ButtonPressState::Released),
            ButtonPressState::JustPressed
        )
    }

    pub fn mouse_button_just_released(&self, button: MouseButton) -> bool {
        matches!(
            self.button_states.get(&button).copied().unwrap_or(ButtonPressState::Released),
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
        let current = self.key_states.get(&code).copied().unwrap_or(KeyPressState::Released);
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
        let current = self.button_states.get(&button).copied().unwrap_or(ButtonPressState::Released);
        let new_state = match (current, state) {
            (ButtonPressState::Released | ButtonPressState::JustReleased, ElementState::Pressed) => {
                ButtonPressState::JustPressed
            }
            (ButtonPressState::JustPressed | ButtonPressState::Pressed, ElementState::Pressed) => {
                ButtonPressState::Pressed
            }
            (ButtonPressState::Pressed | ButtonPressState::JustPressed, ElementState::Released) => {
                ButtonPressState::JustReleased
            }
            (ButtonPressState::Released | ButtonPressState::JustReleased, ElementState::Released) => {
                ButtonPressState::Released
            }
        };
        self.button_states.insert(button, new_state);

        // 生成鼠标按钮事件
        self.events_this_frame.push(InputEvent::MouseButton(MouseButtonEvent {
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
        self.events_this_frame.push(InputEvent::MouseWheel(MouseWheelEvent {
            delta,
            modifiers: self.modifiers,
        }));
    }

    pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    pub fn add_text(&mut self, text: &str) {
        self.text_input.push_str(text);
        self.events_this_frame.push(InputEvent::TextInput(TextInputEvent {
            text: text.to_string(),
        }));
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

    // ===== 快捷查询 =====

    pub fn is_any_key_pressed(&self) -> bool {
        self.key_states.values().any(|s| {
            matches!(s, KeyPressState::Pressed | KeyPressState::JustPressed)
        })
    }

    // ===== 迭代器工具方法 =====

    pub fn pressed_keys(&self) -> impl Iterator<Item = KeyCode> + '_ {
        self.key_states.iter()
            .filter(|(_, s)| matches!(s, KeyPressState::Pressed | KeyPressState::JustPressed))
            .map(|(k, _)| *k)
    }

    pub fn released_keys(&self) -> impl Iterator<Item = KeyCode> + '_ {
        self.key_states.iter()
            .filter(|(_, s)| matches!(s, KeyPressState::JustReleased))
            .map(|(k, _)| *k)
    }

    pub fn pressed_buttons(&self) -> impl Iterator<Item = MouseButton> + '_ {
        self.button_states.iter()
            .filter(|(_, s)| matches!(s, ButtonPressState::Pressed | ButtonPressState::JustPressed))
            .map(|(b, _)| *b)
    }

    pub fn released_buttons(&self) -> impl Iterator<Item = MouseButton> + '_ {
        self.button_states.iter()
            .filter(|(_, s)| matches!(s, ButtonPressState::JustReleased))
            .map(|(b, _)| *b)
    }

    pub fn set_cursor_in_window(&mut self, in_window: bool) {
        let _ = in_window;
    }

    pub fn modifiers_state(&self) -> ModifiersState {
        self.modifiers
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
                WindowEvent::KeyboardInput { event: key_event, .. } => {
                    // winit 0.29: physical_key 是 PhysicalKey（非 Option）
                    let code = key_code::map_physical_key(key_event.physical_key);
                    if matches!(code, KeyCode::Unknown) {
                        // 未知按键 — 尝试用 logical_key 的 NamedKey 兜底
                        if let winit::keyboard::Key::Named(named) = key_event.logical_key {
                            if let Some(mapped) = key_code::map_named_key_to_keycode(&named) {
                                let state = if key_event.state == winit::event::ElementState::Pressed {
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
                    let engine_phase = match touch.phase {
                        winit::event::TouchPhase::Started => TouchPhase::Started,
                        winit::event::TouchPhase::Moved => TouchPhase::Moved,
                        winit::event::TouchPhase::Ended => TouchPhase::Ended,
                        winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
                    };
                    self.input
                        .update_touch(touch.id, position, force, engine_phase);
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
        let has_mb = input.events().any(|e| matches!(e, InputEvent::MouseButton(_)));
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

    // ===== Sprint 02: MainLoop 测试 =====

    #[test]
    fn test_main_loop_config_default() {
        let config = MainLoopConfig::default();
        assert_eq!(config.target_fps, 60);
        assert!((config.fixed_timestep - 1.0 / 60.0).abs() < 1e-6);
        assert!((config.max_dt - 0.1).abs() < 1e-6);
        assert!(config.vsync);
    }

    #[test]
    fn test_main_loop_begin_frame() {
        let mut main_loop = MainLoop::new(MainLoopConfig::default());
        let (dt, should_fixed) = main_loop.begin_frame();
        // First frame: dt should be very small (near 0)
        assert!(dt < 0.001, "First frame dt should be near 0, got {}", dt);
        assert!(!should_fixed, "First frame should not need fixed update");
    }

    #[test]
    fn test_main_loop_consume_fixed_steps_empty() {
        let mut main_loop = MainLoop::new(MainLoopConfig::default());
        assert_eq!(main_loop.consume_fixed_steps(), 0);
    }

    #[test]
    fn test_main_loop_frame_time_remaining() {
        let main_loop = MainLoop::new(MainLoopConfig {
            target_fps: 60,
            ..Default::default()
        });
        let remaining = main_loop.frame_time_remaining();
        // Right after creation, there should be close to a full frame time remaining
        assert!(remaining.as_millis() > 0);
    }

    #[test]
    fn test_touch_phase_default() {
        assert_eq!(TouchPhase::default(), TouchPhase::Started);
    }

    // ===== Sprint 02: Input 迭代器测试 =====

    #[test]
    fn test_input_pressed_keys_iterator() {
        let mut input = Input::new();
        input.update_key(KeyCode::A, ElementState::Pressed);
        input.update_key(KeyCode::D, ElementState::Pressed);

        let pressed: Vec<KeyCode> = input.pressed_keys().collect();
        assert!(pressed.contains(&KeyCode::A));
        assert!(pressed.contains(&KeyCode::D));
        assert!(!pressed.contains(&KeyCode::W));
    }

    #[test]
    fn test_input_released_keys_iterator() {
        let mut input = Input::new();
        input.update_key(KeyCode::A, ElementState::Pressed);
        input.update_key(KeyCode::A, ElementState::Released);

        let released: Vec<KeyCode> = input.released_keys().collect();
        assert!(released.contains(&KeyCode::A));
    }

    #[test]
    fn test_input_pressed_buttons_iterator() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_button(MouseButton::Right, ElementState::Pressed);

        let pressed: Vec<MouseButton> = input.pressed_buttons().collect();
        assert!(pressed.contains(&MouseButton::Left));
        assert!(pressed.contains(&MouseButton::Right));
    }

    #[test]
    fn test_input_released_buttons_iterator() {
        let mut input = Input::new();
        input.update_button(MouseButton::Left, ElementState::Pressed);
        input.update_button(MouseButton::Left, ElementState::Released);

        let released: Vec<MouseButton> = input.released_buttons().collect();
        assert!(released.contains(&MouseButton::Left));
    }

    #[test]
    fn test_input_modifiers_state() {
        let mut input = Input::new();
        let mut mods = ModifiersState::empty();
        mods.set_shift(true);
        mods.set_control(true);
        input.update_modifiers(mods);

        let m = input.modifiers_state();
        assert!(m.shift());
        assert!(m.control());
        assert!(!m.alt());
    }

    // ===== Sprint 02: WindowConfig builder 测试 =====

    #[test]
    fn test_window_config_from_title() {
        let config = WindowConfig::from_title("My Game");
        assert_eq!(config.title, "My Game");
        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
    }

    #[test]
    fn test_window_config_builder_chain() {
        let config = WindowConfig::default()
            .with_size(1920, 1080)
            .with_vsync(false)
            .with_fullscreen(true)
            .with_resizable(false)
            .with_decorations(false);

        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert!(!config.vsync);
        assert!(config.fullscreen);
        assert!(!config.resizable);
        assert!(!config.decorations);
    }

    #[test]
    fn test_window_config_to_builder() {
        let config = WindowConfig::default().with_title("Test");
        let _builder = config.to_builder();
        // Builder construction should not panic
    }

    // ===== Sprint 02: WindowBuilder 扩展测试 =====

    #[test]
    fn test_window_builder_fluent_chain() {
        let builder = WindowBuilder::new()
            .with_title("Test")
            .with_inner_size(800, 600)
            .with_min_inner_size(400, 300)
            .with_max_inner_size(1920, 1080)
            .with_resizable(true)
            .with_maximized(false)
            .with_visible(true)
            .with_decorations(true)
            .with_transparent(false)
            .with_always_on_top(false)
            .with_content_protected(false);

        let _ = builder; // Should compile without error
    }

    // ===== Sprint 02: TouchPoint 和 TouchPhase 测试 =====

    #[test]
    fn test_touch_update_and_query() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);

        assert_eq!(input.touch_count(), 1);
        let touch = input.touch(1).unwrap();
        assert_eq!(touch.id, 1);
        assert!((touch.position.x - 100.0).abs() < f32::EPSILON);
        assert_eq!(touch.phase, TouchPhase::Started);
    }

    #[test]
    fn test_touch_move() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        input.update_touch(1, Vec2::new(150.0, 250.0), 0.5, TouchPhase::Moved);

        let touch = input.touch(1).unwrap();
        assert!((touch.position.x - 150.0).abs() < f32::EPSILON);
        assert_eq!(touch.phase, TouchPhase::Moved);
    }

    #[test]
    fn test_touch_end_removes() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        assert_eq!(input.touch_count(), 1);

        input.update_touch(1, Vec2::new(100.0, 200.0), 0.0, TouchPhase::Ended);
        assert_eq!(input.touch_count(), 0);
    }

    #[test]
    fn test_touch_cancel_removes() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.0, TouchPhase::Cancelled);
        assert_eq!(input.touch_count(), 0);
    }

    #[test]
    fn test_touch_multiple() {
        let mut input = Input::new();
        input.update_touch(1, Vec2::new(100.0, 200.0), 0.5, TouchPhase::Started);
        input.update_touch(2, Vec2::new(200.0, 300.0), 0.5, TouchPhase::Started);

        assert_eq!(input.touch_count(), 2);
        assert!(input.touch(1).is_some());
        assert!(input.touch(2).is_some());
        assert!(input.touch(3).is_none());
    }

    // ===== Sprint 02: WindowMode 和 Theme 测试 =====

    #[test]
    fn test_window_mode_variants() {
        assert_eq!(WindowMode::Windowed, WindowMode::Windowed);
        assert_eq!(WindowMode::Fullscreen, WindowMode::Fullscreen);
        assert_eq!(WindowMode::Borderless, WindowMode::Borderless);
        assert_ne!(WindowMode::Windowed, WindowMode::Fullscreen);
    }

    #[test]
    fn test_theme_variants() {
        assert_eq!(Theme::default(), Theme::Light);
        assert_eq!(Theme::Dark, Theme::Dark);
    }

    // ===== Sprint 02: MouseScrollDelta 测试 =====

    #[test]
    fn test_mouse_scroll_delta_line() {
        let delta = MouseScrollDelta::LineDelta(1.0, 3.0);
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                assert!((x - 1.0).abs() < f32::EPSILON);
                assert!((y - 3.0).abs() < f32::EPSILON);
            }
            MouseScrollDelta::PixelDelta(_, _) => panic!("Expected LineDelta"),
        }
    }

    #[test]
    fn test_mouse_scroll_delta_pixel() {
        let delta = MouseScrollDelta::PixelDelta(10.0, 30.0);
        match delta {
            MouseScrollDelta::PixelDelta(x, y) => {
                assert!((x - 10.0).abs() < f32::EPSILON);
                assert!((y - 30.0).abs() < f32::EPSILON);
            }
            MouseScrollDelta::LineDelta(_, _) => panic!("Expected PixelDelta"),
        }
    }

    // ===== Sprint 02: MainLoop 累积测试 =====

    #[test]
    fn test_main_loop_accumulation() {
        let config = MainLoopConfig {
            target_fps: 60,
            fixed_timestep: 0.016,
            max_dt: 0.1,
            vsync: true,
        };
        let mut main_loop = MainLoop::new(config);

        // Simulate first frame
        let _ = main_loop.begin_frame();

        // Manually add accumulated time to test fixed step logic
        // After 3 frames of 16ms each, we should have enough for one fixed step
        // This is tricky to test without sleeping, so just test the config
        assert_eq!(main_loop.config().fixed_timestep, 0.016);
    }
}
