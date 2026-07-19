use crate::module::ModuleRegistry;
use crate::time::Time;
use engine_window::{
    CursorGrabMode, CursorIcon, Event, EventLoopProxy, Fullscreen, InputModule, PhysicalPosition,
    PhysicalSize, Window, WindowExt, WindowMode, WindowState,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub target_fps: u32,
    pub log_level: String,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Game Engine".to_string(),
            window_width: 1280,
            window_height: 720,
            target_fps: 60,
            log_level: "info".to_string(),
        }
    }
}

pub struct Engine {
    config: EngineConfig,
    modules: ModuleRegistry,
    time: Time,
    window: Option<Window>,
    input_module: Arc<Mutex<InputModule>>,
    window_state: Option<WindowState>,
    event_loop_proxy: Option<EventLoopProxy<()>>,
    quit_flag: std::sync::OnceLock<Arc<std::sync::atomic::AtomicBool>>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            modules: ModuleRegistry::new(),
            time: Time::new(),
            window: None,
            input_module: Arc::new(Mutex::new(InputModule::new())),
            window_state: None,
            event_loop_proxy: None,
            quit_flag: std::sync::OnceLock::new(),
        }
    }

    /// 设置退出标志（供 AppBuilder 使用）
    pub fn set_quit_flag(&self, flag: Arc<std::sync::atomic::AtomicBool>) {
        let _ = self.quit_flag.set(flag);
    }

    /// 请求引擎在当前帧结束后退出（设置内部 quit_flag）
    pub fn request_quit(&self) {
        let flag = self.quit_flag.get_or_init(|| Arc::new(std::sync::atomic::AtomicBool::new(false)));
        flag.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// 检查引擎是否正在运行
    ///
    /// 如果 `quit_flag` 未设置，视为正在运行；如果已设置且值为 true，则已退出。
    pub fn is_running(&self) -> bool {
        self.quit_flag
            .get()
            .map_or(true, |f| !f.load(std::sync::atomic::Ordering::SeqCst))
    }

    /// 暂停引擎
    ///
    /// 预留接口，后续 Sprint 引入独立暂停标志
    pub fn pause(&mut self) {
        // 预留接口，后续引入独立暂停标志
    }

    /// 恢复引擎运行
    ///
    /// 预留接口，与 `pause` 配对
    pub fn resume(&mut self) {
        // 预留接口
    }

    /// 向线程池提交异步任务
    ///
    /// 当前使用 `std::thread::spawn` 作为简易实现，后续接入完整线程池
    pub fn spawn_task<F>(&self, f: F) -> std::thread::JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        std::thread::spawn(f)
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn time_mut(&mut self) -> &mut Time {
        &mut self.time
    }

    pub fn modules(&self) -> &ModuleRegistry {
        &self.modules
    }

    // ===== 窗口访问 =====

    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    /// 设置窗口句柄（在外部创建窗口后调用）
    pub fn set_window(&mut self, window: Window) {
        self.window = Some(window);
    }

    // ===== 输入访问 =====

    pub fn input(&self) -> Option<std::sync::MutexGuard<'_, InputModule>> {
        self.input_module.lock().ok()
    }

    pub fn input_module(&self) -> Arc<Mutex<InputModule>> {
        self.input_module.clone()
    }

    // ===== 窗口状态（自动事件监听）=====

    /// 处理来自事件循环的窗口事件并更新内部状态
    ///
    /// 该方法自动更新窗口焦点、可见性、尺寸等状态，无需上层代码手动维护。
    ///
    /// # 参数
    /// - `event`: 来自事件循环的通用事件（引擎级，已屏蔽 winit 底层类型）
    pub fn process_window_event(&self, event: &Event<()>) {
        if let Some(ws) = &self.window_state {
            ws.process_event(event);
        }
    }

    /// 窗口是否拥有键盘焦点
    pub fn is_focused(&self) -> bool {
        self.window_state.as_ref().is_some_and(|ws| ws.is_focused())
    }

    /// 窗口是否被最小化
    pub fn is_minimized(&self) -> bool {
        self.window_state.as_ref().is_some_and(|ws| ws.is_minimized())
    }

    /// 窗口是否被最大化
    pub fn is_maximized(&self) -> bool {
        self.window_state.as_ref().is_some_and(|ws| ws.is_maximized())
    }

    /// 窗口是否可见
    pub fn is_visible(&self) -> bool {
        self.window_state.as_ref().map_or(true, |ws| ws.is_visible())
    }

    // ===== 光标控制（屏蔽 winit 依赖）=====

    /// 设置光标可见性
    pub fn show_cursor(&self, visible: bool) {
        if let Some(window) = &self.window {
            window.set_cursor_visible(visible);
        }
    }

    /// 设置光标捕获模式（None/Confined/Locked）
    ///
    /// - `CursorGrabMode::None` — 不捕获
    /// - `CursorGrabMode::Confined` — 限制在窗口内
    /// - `CursorGrabMode::Locked` — 完全锁定（鼠标坐标固定）
    pub fn set_cursor_grab(&self, mode: CursorGrabMode) -> Result<(), String> {
        if let Some(window) = &self.window {
            window.set_engine_cursor_grab(mode)
        } else {
            Err("未设置窗口句柄".to_string())
        }
    }

    /// 便捷方法：按布尔值切换 Confined/None
    pub fn set_cursor_grab_bool(&self, grab: bool) -> Result<(), String> {
        let mode = if grab {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        };
        self.set_cursor_grab(mode)
    }

    /// 设置光标图标
    pub fn set_cursor_icon(&self, icon: CursorIcon) {
        if let Some(window) = &self.window {
            window.set_engine_cursor_icon(icon);
        }
    }

    /// 设置光标位置（相对窗口左上角）
    pub fn set_cursor_position(&self, x: f64, y: f64) -> Result<(), String> {
        if let Some(window) = &self.window {
            window.set_engine_cursor_position(x, y)
        } else {
            Err("未设置窗口句柄".to_string())
        }
    }

    /// 设置 IME（输入法）是否启用
    pub fn set_ime_allowed(&self, allowed: bool) {
        if let Some(window) = &self.window {
            window.set_ime_allowed(allowed);
        }
    }

    /// 请求关闭窗口/退出
    pub fn request_close(&self) {
        self.request_quit();
    }

    /// 设置 IME 位置
    pub fn set_ime_position(&self, position: PhysicalPosition<i32>) {
        if let Some(window) = &self.window {
            window.set_ime_cursor_area(position, PhysicalSize::new(0, 0));
        }
    }

    /// 设置窗口模式
    pub fn set_window_mode(&self, mode: WindowMode) {
        if let Some(window) = &self.window {
            match mode {
                WindowMode::Windowed => {
                    window.set_fullscreen(None);
                }
                WindowMode::Fullscreen => {
                    if let Some(monitor) = window.current_monitor() {
                        window.set_fullscreen(Some(Fullscreen::Exclusive(
                            monitor.video_modes().next().unwrap_or_else(|| {
                                panic!("No video mode available")
                            })
                        )));
                    }
                }
                WindowMode::Borderless => {
                    window.set_fullscreen(Some(Fullscreen::Borderless(
                        window.current_monitor()
                    )));
                }
            }
        }
    }

    /// 获取事件循环代理
    pub fn event_loop_proxy(&self) -> Option<&EventLoopProxy<()>> {
        self.event_loop_proxy.as_ref()
    }

    // ===== 窗口状态与事件循环设置 =====

    /// 设置窗口状态
    pub fn set_window_state(&mut self, state: WindowState) {
        self.window_state = Some(state);
    }

    /// 设置事件循环代理
    pub fn set_event_loop_proxy(&mut self, proxy: EventLoopProxy<()>) {
        self.event_loop_proxy = Some(proxy);
    }

    /// 启动主循环（阻塞运行直到 quit_flag 被设置为 true）
    pub fn run(&mut self) {
        // 初始化所有模块
        if let Err(e) = self.modules.initialize_all() {
            eprintln!("模块初始化失败: {}", e);
            return;
        }

        // 初始化时间
        self.time.update();

        // 主循环
        loop {
            // 检查退出标志
            if let Some(flag) = self.quit_flag.get() {
                if flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }

            // 更新时间
            self.time.update();
            let dt = self.time.delta_time();

            // 更新所有模块
            self.modules.update_all(dt);

            // 渲染所有模块
            self.modules.render_all();

            // 节流（避免 CPU 空转）
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // 清理所有模块
        self.modules.shutdown_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_new() {
        let engine = Engine::new(EngineConfig::default());
        assert_eq!(engine.config().window_title, "Game Engine");
        assert_eq!(engine.config().window_width, 1280);
        assert_eq!(engine.config().window_height, 720);
    }

    #[test]
    fn test_engine_default() {
        let engine = Engine::default();
        assert!(!engine.is_focused()); // window_state 未设置，默认为 false
        assert!(!engine.is_minimized());
        assert!(engine.is_visible()); // window_state 未设置，默认为 true
    }

    #[test]
    fn test_engine_default_is_new() {
        let a = Engine::new(EngineConfig::default());
        let b = Engine::default();
        assert_eq!(a.config().window_title, b.config().window_title);
        assert_eq!(a.config().window_width, b.config().window_width);
    }

    #[test]
    fn test_engine_has_input_module() {
        let engine = Engine::default();
        let guard = engine.input();
        assert!(guard.is_some());
    }

    #[test]
    fn test_engine_input_module_clone() {
        let engine = Engine::default();
        let arc = engine.input_module();
        assert!(arc.lock().is_ok());
    }

    #[test]
    fn test_set_cursor_grab_no_window_returns_error() {
        let engine = Engine::default();
        let result = engine.set_cursor_grab_bool(true);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_cursor_grab_none_no_window_returns_error() {
        let engine = Engine::default();
        let result = engine.set_cursor_grab(CursorGrabMode::None);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_cursor_position_no_window_returns_error() {
        let engine = Engine::default();
        let result = engine.set_cursor_position(100.0, 200.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_show_cursor_no_window_does_not_panic() {
        let engine = Engine::default();
        engine.show_cursor(false); // 无窗口时应安全返回
        engine.show_cursor(true);
    }

    #[test]
    fn test_set_cursor_icon_no_window_does_not_panic() {
        let engine = Engine::default();
        engine.set_cursor_icon(CursorIcon::Hand); // 无窗口时应安全返回
    }

    #[test]
    fn test_set_ime_allowed_no_window_does_not_panic() {
        let engine = Engine::default();
        engine.set_ime_allowed(true);
        engine.set_ime_allowed(false);
    }

    #[test]
    fn test_engine_config_clone() {
        let config = EngineConfig::default();
        let cloned = config.clone();
        assert_eq!(config.window_title, cloned.window_title);
        assert_eq!(config.window_width, cloned.window_width);
        assert_eq!(config.target_fps, cloned.target_fps);
        assert_eq!(config.log_level, cloned.log_level);
    }

    #[test]
    fn test_process_window_event_no_panic() {
        let mut engine = Engine::default();
        engine.set_window_state(WindowState::new());
        // 构造一个空的 Event 来测试
        let event = Event::WindowEvent {
            window_id: unsafe { std::mem::zeroed() },
            event: engine_window::WindowEvent::Focused(true),
        };
        engine.process_window_event(&event);
        // 应仍处于默认状态（Focused(true) 事件不改变已为 true 的焦点状态）
        assert!(engine.is_focused());
        assert!(!engine.is_minimized());
    }

    #[test]
    fn test_engine_is_running_after_new() {
        let engine = Engine::new(EngineConfig::default());
        assert!(engine.is_running());
    }

    #[test]
    fn test_engine_is_not_running_after_quit() {
        let engine = Engine::new(EngineConfig::default());
        engine.request_quit();
        assert!(!engine.is_running());
    }

    #[test]
    fn test_engine_spawn_task() {
        let engine = Engine::new(EngineConfig::default());
        let result = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
        let r = result.clone();
        let handle = engine.spawn_task(move || {
            r.store(42, std::sync::atomic::Ordering::SeqCst);
        });
        handle.join().unwrap();
        assert_eq!(result.load(std::sync::atomic::Ordering::SeqCst), 42);
    }

    // ===== Sprint 02: Engine 窗口 API 测试 =====

    #[test]
    fn test_engine_is_focused_default() {
        let engine = Engine::new(EngineConfig::default());
        // No window state set, default should be false
        assert!(!engine.is_focused());
    }

    #[test]
    fn test_engine_is_visible_default() {
        let engine = Engine::new(EngineConfig::default());
        // No window state, default should be true
        assert!(engine.is_visible());
    }

    #[test]
    fn test_engine_is_minimized_default() {
        let engine = Engine::new(EngineConfig::default());
        assert!(!engine.is_minimized());
    }

    #[test]
    fn test_engine_is_maximized_default() {
        let engine = Engine::new(EngineConfig::default());
        assert!(!engine.is_maximized());
    }

    #[test]
    fn test_engine_event_loop_proxy_default() {
        let engine = Engine::new(EngineConfig::default());
        assert!(engine.event_loop_proxy().is_none());
    }
}
