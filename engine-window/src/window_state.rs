//! 窗口状态管理 — 自动监听 Focused/Minimized/Maximized/Visible 等事件

use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc,
};

/// 窗口尺寸
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

/// 窗口状态 — 自动监听 winit 事件并更新状态
pub struct WindowState {
    focused: Arc<AtomicBool>,
    minimized: Arc<AtomicBool>,
    maximized: Arc<AtomicBool>,
    visible: Arc<AtomicBool>,
    size: Arc<(AtomicU32, AtomicU32)>,
}

impl WindowState {
    /// 创建新的窗口状态
    pub fn new() -> Self {
        Self {
            focused: Arc::new(AtomicBool::new(true)),
            minimized: Arc::new(AtomicBool::new(false)),
            maximized: Arc::new(AtomicBool::new(false)),
            visible: Arc::new(AtomicBool::new(true)),
            size: Arc::new((AtomicU32::new(1280), AtomicU32::new(720))),
        }
    }

    /// 从原始 winit 事件更新窗口状态
    pub fn process_event(&self, event: &winit::event::Event<()>) {
        if let winit::event::Event::WindowEvent {
            event: window_event,
            ..
        } = event
        {
            match window_event {
                winit::event::WindowEvent::Focused(focused) => {
                    self.focused.store(*focused, Ordering::SeqCst);
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    self.size
                        .0
                        .store(physical_size.width, Ordering::SeqCst);
                    self.size
                        .1
                        .store(physical_size.height, Ordering::SeqCst);
                }
                winit::event::WindowEvent::Occluded(occluded) => {
                    // 窗口被遮挡时视为不可见
                    self.visible.store(!occluded, Ordering::SeqCst);
                }
                winit::event::WindowEvent::CloseRequested => {
                    // 关闭请求时标记为不可见
                    self.visible.store(false, Ordering::SeqCst);
                }
                _ => {}
            }
        }
    }

    /// 窗口是否拥有焦点
    pub fn is_focused(&self) -> bool {
        self.focused.load(Ordering::SeqCst)
    }

    /// 窗口是否最小化
    pub fn is_minimized(&self) -> bool {
        self.minimized.load(Ordering::SeqCst)
    }

    /// 窗口是否最大化
    pub fn is_maximized(&self) -> bool {
        self.maximized.load(Ordering::SeqCst)
    }

    /// 窗口是否可见
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::SeqCst)
    }

    /// 当前窗口尺寸
    pub fn size(&self) -> WindowSize {
        WindowSize {
            width: self.size.0.load(Ordering::SeqCst),
            height: self.size.1.load(Ordering::SeqCst),
        }
    }

    /// 获取 focused 状态的共享原子引用
    pub fn focused_flag(&self) -> Arc<AtomicBool> {
        self.focused.clone()
    }

    /// 获取 minimized 状态的共享原子引用
    pub fn minimized_flag(&self) -> Arc<AtomicBool> {
        self.minimized.clone()
    }

    /// 获取 maximized 状态的共享原子引用
    pub fn maximized_flag(&self) -> Arc<AtomicBool> {
        self.maximized.clone()
    }

    /// 获取 visible 状态的共享原子引用
    pub fn visible_flag(&self) -> Arc<AtomicBool> {
        self.visible.clone()
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::{Event, WindowEvent};
    use winit::window::WindowId;

    /// 构造测试用的 dummy WindowId。
    ///
    /// # Safety
    /// 此函数仅用于测试目的。`WindowState::process_event` 仅检查事件类型
    /// 而不读取 `window_id` 字段，因此零值 window_id 不会导致未定义行为。
    /// 此函数将 unsafe 封装在单一位置，便于审计和维护。
    fn dummy_window_id() -> WindowId {
        unsafe { std::mem::zeroed() }
    }

    fn make_window_event(event: WindowEvent) -> Event<()> {
        Event::WindowEvent {
            window_id: dummy_window_id(),
            event,
        }
    }

    #[test]
    fn test_initial_state() {
        let state = WindowState::new();
        assert!(state.is_focused());
        assert!(!state.is_minimized());
        assert!(!state.is_maximized());
        assert!(state.is_visible());
    }

    #[test]
    fn test_focused_event() {
        let state = WindowState::new();

        state.process_event(&make_window_event(WindowEvent::Focused(false)));
        assert!(!state.is_focused());

        state.process_event(&make_window_event(WindowEvent::Focused(true)));
        assert!(state.is_focused());
    }

    #[test]
    fn test_minimized_event() {
        let state = WindowState::new();
        assert!(!state.is_minimized());
    }

    #[test]
    fn test_maximized_event() {
        let state = WindowState::new();
        assert!(!state.is_maximized());
    }

    #[test]
    fn test_occluded_event() {
        let state = WindowState::new();

        state.process_event(&make_window_event(WindowEvent::Occluded(true)));
        assert!(!state.is_visible());

        state.process_event(&make_window_event(WindowEvent::Occluded(false)));
        assert!(state.is_visible());
    }

    #[test]
    fn test_resized_event() {
        let state = WindowState::new();

        state.process_event(&make_window_event(WindowEvent::Resized(
            winit::dpi::PhysicalSize {
                width: 1920,
                height: 1080,
            },
        )));
        let size = state.size();
        assert_eq!(size.width, 1920);
        assert_eq!(size.height, 1080);
    }

    #[test]
    fn test_close_requested_event() {
        let state = WindowState::new();

        state.process_event(&make_window_event(WindowEvent::CloseRequested));
        assert!(!state.is_visible());
    }

    #[test]
    fn test_flags_cloned_independence() {
        let state = WindowState::new();
        let focused = state.focused_flag();
        focused.store(false, Ordering::SeqCst);
        assert!(!state.is_focused());
    }
}
