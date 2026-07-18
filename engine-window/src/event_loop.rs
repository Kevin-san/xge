//! 引擎级事件循环抽象 — 封装 winit 事件循环

use crate::{Input, InputModule, WindowState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// 事件循环配置
#[derive(Debug, Clone, Default)]
pub struct EventLoopConfig {
    pub poll: bool,
}

/// 引擎级事件循环
pub struct EngineEventLoop {
    input_module: InputModule,
    window_state: WindowState,
    running: Arc<AtomicBool>,
}

impl EngineEventLoop {
    pub fn new() -> Self {
        Self {
            input_module: InputModule::new(),
            window_state: WindowState::new(),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    /// 获取输入模块引用
    pub fn input(&self) -> &Input {
        self.input_module.input()
    }

    /// 获取输入模块可变引用
    pub fn input_mut(&mut self) -> &mut InputModule {
        &mut self.input_module
    }

    /// 获取窗口状态引用
    pub fn window_state(&self) -> &WindowState {
        &self.window_state
    }

    /// 处理 winit 事件，更新输入状态和窗口状态
    pub fn process_event(&mut self, event: &crate::Event<()>) {
        self.input_module.process_event(event);
        self.window_state.process_event(event);
    }

    /// 帧结束清理
    pub fn end_frame(&mut self) {
        self.input_module.clear();
    }

    /// 请求退出事件循环
    pub fn request_exit(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// 检查是否仍在运行
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// 获取运行标志的共享引用
    pub fn running_flag(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }
}

impl Default for EngineEventLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let el = EngineEventLoop::new();
        assert!(el.is_running());
    }

    #[test]
    fn test_request_exit() {
        let el = EngineEventLoop::new();
        el.request_exit();
        assert!(!el.is_running());
    }

    #[test]
    fn test_end_frame() {
        let mut el = EngineEventLoop::new();
        el.input_mut()
            .input_mut()
            .update_key(crate::KeyCode::A, crate::ElementState::Pressed);
        assert!(el.input().key_pressed(crate::KeyCode::A));
        el.end_frame();
        // After clear, JustPressed -> Pressed
        assert!(el.input().key_pressed(crate::KeyCode::A));
    }

    #[test]
    fn test_running_flag() {
        let el = EngineEventLoop::new();
        let flag = el.running_flag();
        assert!(flag.load(Ordering::SeqCst));
        flag.store(false, Ordering::SeqCst);
        assert!(!el.is_running());
    }
}
