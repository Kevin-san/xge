use crate::module::ModuleRegistry;
use crate::time::Time;
use engine_window::{
    CursorGrabMode, CursorIcon, Event, InputModule, Window, WindowExt, WindowState,
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

impl EngineConfig {
    /// 从 TOML 字符串解析 EngineConfig（简易手动解析，不支持嵌套表）
    ///
    /// 支持的字段: window_title, window_width, window_height, target_fps, log_level
    pub fn from_toml(toml_str: &str) -> Result<Self, String> {
        let mut config = Self::default();
        for line in toml_str.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "window_title" => config.window_title = value.trim_matches('"').to_string(),
                    "window_width" => {
                        config.window_width = value.parse::<u32>().map_err(|e| e.to_string())?
                    }
                    "window_height" => {
                        config.window_height = value.parse::<u32>().map_err(|e| e.to_string())?
                    }
                    "target_fps" => {
                        config.target_fps = value.parse::<u32>().map_err(|e| e.to_string())?
                    }
                    "log_level" => config.log_level = value.trim_matches('"').to_string(),
                    _ => {} // 忽略未知字段
                }
            }
        }
        Ok(config)
    }

    /// 从 JSON 字符串解析 EngineConfig（简易手动解析，仅支持扁平对象）
    ///
    /// 支持的字段: window_title, window_width, window_height, target_fps, log_level
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        let mut config = Self::default();
        let trimmed = json_str.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err("JSON 必须以 { 开始并以 } 结束".to_string());
        }
        let inner = trimmed[1..trimmed.len() - 1].trim();
        if inner.is_empty() {
            return Ok(config);
        }
        for pair in inner.split(',') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }
            if let Some((key_part, value_part)) = pair.split_once(':') {
                let key = key_part.trim().trim_matches('"');
                let value = value_part.trim();
                match key {
                    "window_title" => config.window_title = value.trim_matches('"').to_string(),
                    "window_width" => {
                        config.window_width = value.parse::<u32>().map_err(|e| e.to_string())?
                    }
                    "window_height" => {
                        config.window_height = value.parse::<u32>().map_err(|e| e.to_string())?
                    }
                    "target_fps" => {
                        config.target_fps = value.parse::<u32>().map_err(|e| e.to_string())?
                    }
                    "log_level" => config.log_level = value.trim_matches('"').to_string(),
                    _ => {} // 忽略未知字段
                }
            }
        }
        Ok(config)
    }
}

pub struct Engine {
    config: EngineConfig,
    modules: ModuleRegistry,
    time: Time,
    window: Option<Window>,
    input_module: Arc<Mutex<InputModule>>,
    window_state: WindowState,
    quit_flag: Option<Arc<std::sync::atomic::AtomicBool>>,
    running: bool,
    paused: bool,
    filesystem: Option<Box<dyn crate::filesystem::FileSystem>>,
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
            window_state: WindowState::new(),
            quit_flag: None,
            running: false,
            paused: false,
            filesystem: None,
        }
    }

    /// 设置退出标志（供 AppBuilder 使用）
    pub fn set_quit_flag(&mut self, flag: Arc<std::sync::atomic::AtomicBool>) {
        self.quit_flag = Some(flag);
    }

    /// 请求引擎在当前帧结束后退出（设置内部 quit_flag）
    pub fn request_quit(&mut self) {
        if self.quit_flag.is_none() {
            self.quit_flag = Some(Arc::new(std::sync::atomic::AtomicBool::new(false)));
        }
        if let Some(flag) = &self.quit_flag {
            flag.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn modules(&self) -> &ModuleRegistry {
        &self.modules
    }

    // ===== 运行状态 =====

    /// 引擎是否正在运行
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// 暂停引擎（暂停后主循环跳过 update/render）
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// 恢复引擎
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// 引擎是否处于暂停状态
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    // ===== 文件系统 =====

    /// 获取文件系统引用
    pub fn filesystem(&self) -> Option<&dyn crate::filesystem::FileSystem> {
        self.filesystem.as_deref()
    }

    /// 设置文件系统
    pub fn set_filesystem(&mut self, fs: Box<dyn crate::filesystem::FileSystem>) {
        self.filesystem = Some(fs);
    }

    // ===== 任务派发 =====

    /// 在独立线程中派发任务
    pub fn spawn_task<F: FnOnce() + Send + 'static>(&self, f: F) {
        std::thread::spawn(f);
    }

    // ===== 模块类型查找 =====

    // TODO: 实现 module<T>() 和 module_mut<T>() — 需要 ModuleRegistry 支持类型查找
    // pub fn module<T: 'static>(&self) -> Option<&T> { ... }
    // pub fn module_mut<T: 'static>(&mut self) -> Option<&mut T> { ... }

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
        self.window_state.process_event(event);
    }

    /// 窗口是否拥有键盘焦点
    pub fn is_focused(&self) -> bool {
        self.window_state.is_focused()
    }

    /// 窗口是否被最小化
    pub fn is_minimized(&self) -> bool {
        self.window_state.is_minimized()
    }

    /// 窗口是否被最大化
    pub fn is_maximized(&self) -> bool {
        self.window_state.is_maximized()
    }

    /// 窗口是否可见
    pub fn is_visible(&self) -> bool {
        self.window_state.is_visible()
    }

    // ===== 光标控制（屏蔽 winit 依赖）=====

    /// 设置光标可见性
    pub fn show_cursor(&self, show: bool) {
        if let Some(window) = &self.window {
            window.set_cursor_visible(show);
        }
    }

    /// 设置光标捕获模式（None/Confined/Locked）
    ///
    /// - `CursorGrabMode::None` — 不捕获
    /// - `CursorGrabMode::Confined` — 限制在窗口内
    /// - `CursorGrabMode::Locked` — 完全锁定（鼠标坐标固定）
    pub fn set_cursor_grab(&self, mode: CursorGrabMode) -> Result<(), String> {
        let window = match &self.window {
            Some(w) => w,
            None => return Err("未设置窗口句柄".to_string()),
        };
        // 使用 WindowExt trait 的引擎级方法（内部自动映射到 winit）
        window.set_engine_cursor_grab(mode)
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
        let window = match &self.window {
            Some(w) => w,
            None => return Err("未设置窗口句柄".to_string()),
        };
        window.set_engine_cursor_position(x, y)
    }

    /// 设置 IME（输入法）是否启用
    pub fn set_ime_allowed(&self, allowed: bool) {
        if let Some(window) = &self.window {
            window.set_ime_allowed(allowed);
        }
    }

    /// 启动主循环（阻塞运行直到 quit_flag 被设置为 true）
    pub fn run(&mut self) {
        self.running = true;

        // 初始化所有模块
        if let Err(e) = self.modules.initialize_all() {
            eprintln!("模块初始化失败: {}", e);
            self.running = false;
            return;
        }

        // 初始化时间
        self.time.update();

        // 主循环
        loop {
            // 检查退出标志
            if let Some(flag) = &self.quit_flag {
                if flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }

            // 暂停时跳过 update/render
            if self.paused {
                std::thread::sleep(std::time::Duration::from_millis(1));
                continue;
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
        self.running = false;
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
        assert!(engine.is_focused()); // 默认状态为已聚焦
        assert!(!engine.is_minimized());
        assert!(engine.is_visible());
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
        let engine = Engine::default();
        // 构造一个空的 Event 来测试
        let event = Event::WindowEvent {
            window_id: unsafe { std::mem::zeroed() },
            event: engine_window::WindowEvent::Focused(true),
        };
        engine.process_window_event(&event);
        // 应仍处于默认状态
        assert!(engine.is_focused());
        assert!(!engine.is_minimized());
    }

    #[test]
    fn test_is_running() {
        let mut engine = Engine::default();
        assert!(!engine.is_running());
        // run() 会设置 running = true，但因为会阻塞所以我们通过直接测试字段逻辑
        // 验证初始状态为 false
        engine.running = true;
        assert!(engine.is_running());
        engine.running = false;
        assert!(!engine.is_running());
    }

    #[test]
    fn test_pause_resume() {
        let mut engine = Engine::default();
        assert!(!engine.is_paused());
        engine.pause();
        assert!(engine.is_paused());
        engine.resume();
        assert!(!engine.is_paused());
        // 多次调用应保持幂等
        engine.resume();
        assert!(!engine.is_paused());
        engine.pause();
        engine.pause();
        assert!(engine.is_paused());
    }

    #[test]
    fn test_filesystem_accessor() {
        use crate::filesystem::StdFileSystem;
        let mut engine = Engine::default();
        assert!(engine.filesystem().is_none());
        engine.set_filesystem(Box::new(StdFileSystem));
        assert!(engine.filesystem().is_some());
    }

    #[test]
    fn test_engine_config_from_json() {
        let json = r#"{"window_title":"My Game","window_width":1920,"window_height":1080,"target_fps":120,"log_level":"debug"}"#;
        let config = EngineConfig::from_json(json).unwrap();
        assert_eq!(config.window_title, "My Game");
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert_eq!(config.target_fps, 120);
        assert_eq!(config.log_level, "debug");

        // 部分字段
        let json_partial = r#"{"window_title":"Partial"}"#;
        let config_partial = EngineConfig::from_json(json_partial).unwrap();
        assert_eq!(config_partial.window_title, "Partial");
        assert_eq!(config_partial.window_width, 1280); // 默认值

        // 空对象
        let json_empty = "{}";
        let config_empty = EngineConfig::from_json(json_empty).unwrap();
        assert_eq!(config_empty.window_title, "Game Engine");

        // 非法格式
        let json_bad = "not json";
        assert!(EngineConfig::from_json(json_bad).is_err());
    }

    #[test]
    fn test_engine_config_from_toml() {
        let toml = r#"
window_title = "My Game"
window_width = 1920
window_height = 1080
target_fps = 120
log_level = "debug"
"#;
        let config = EngineConfig::from_toml(toml).unwrap();
        assert_eq!(config.window_title, "My Game");
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert_eq!(config.target_fps, 120);
        assert_eq!(config.log_level, "debug");

        // 部分字段
        let toml_partial = r#"window_title = "Partial""#;
        let config_partial = EngineConfig::from_toml(toml_partial).unwrap();
        assert_eq!(config_partial.window_title, "Partial");
        assert_eq!(config_partial.window_width, 1280); // 默认值

        // 注释和表头应被忽略
        let toml_with_comments = r#"
# This is a comment
[section]
window_title = "Commented"
"#;
        let config_comments = EngineConfig::from_toml(toml_with_comments).unwrap();
        assert_eq!(config_comments.window_title, "Commented");

        // 非法数字
        let toml_bad = r#"window_width = "not_a_number""#;
        assert!(EngineConfig::from_toml(toml_bad).is_err());
    }
}
