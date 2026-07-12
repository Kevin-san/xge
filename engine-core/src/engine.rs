use crate::module::{EngineContext, ModuleRegistry};
use engine_platform::{FileSystem, ThreadPool, Time};
use engine_window::{
    CursorGrabMode, CursorIcon, Event, InputModule, Window, WindowExt, WindowState,
};
use parking_lot::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn from_toml(content: &str) -> anyhow::Result<Self> {
        Ok(toml::from_str(content)?)
    }

    pub fn to_toml(&self) -> anyhow::Result<String> {
        Ok(toml::to_string(self)?)
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
    thread_pool: ThreadPool,
    filesystem: FileSystem,
    running: bool,
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
            thread_pool: ThreadPool::new(),
            filesystem: FileSystem::new(),
            running: false,
        }
    }

    pub fn set_quit_flag(&mut self, flag: Arc<std::sync::atomic::AtomicBool>) {
        self.quit_flag = Some(flag);
    }

    pub fn request_quit(&mut self) {
        if self.quit_flag.is_none() {
            self.quit_flag = Some(Arc::new(std::sync::atomic::AtomicBool::new(false)));
        }
        if let Some(flag) = &self.quit_flag {
            flag.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
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

    pub fn modules_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.modules
    }

    pub fn module<T: crate::module::Module + 'static>(&self) -> Option<&T> {
        self.modules.get::<T>()
    }

    pub fn module_mut<T: crate::module::Module + 'static>(&mut self) -> Option<&mut T> {
        self.modules.get_mut::<T>()
    }

    pub fn world(&self) -> Option<()> {
        None
    }

    pub fn world_mut(&mut self) -> Option<()> {
        None
    }

    pub fn filesystem(&self) -> &FileSystem {
        &self.filesystem
    }

    pub fn thread_pool(&self) -> &ThreadPool {
        &self.thread_pool
    }

    pub fn spawn_task<F>(&self, future: F)
    where
        F: futures_lite::Future<Output = ()> + Send + 'static,
    {
        self.thread_pool.spawn_future(future);
    }

    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn set_window(&mut self, window: Window) {
        self.window = Some(window);
    }

    pub fn input(&self) -> Option<MutexGuard<'_, InputModule>> {
        self.input_module.try_lock()
    }

    pub fn input_module(&self) -> Arc<Mutex<InputModule>> {
        self.input_module.clone()
    }

    pub fn process_window_event(&self, event: &Event<()>) {
        self.window_state.process_event(event);
    }

    pub fn is_focused(&self) -> bool {
        self.window_state.is_focused()
    }

    pub fn is_minimized(&self) -> bool {
        self.window_state.is_minimized()
    }

    pub fn is_maximized(&self) -> bool {
        self.window_state.is_maximized()
    }

    pub fn is_visible(&self) -> bool {
        self.window_state.is_visible()
    }

    pub fn show_cursor(&self, show: bool) {
        if let Some(window) = &self.window {
            window.set_cursor_visible(show);
        }
    }

    pub fn set_cursor_grab(&self, mode: CursorGrabMode) -> Result<(), String> {
        let window = match &self.window {
            Some(w) => w,
            None => return Err("未设置窗口句柄".to_string()),
        };
        window.set_engine_cursor_grab(mode)
    }

    pub fn set_cursor_grab_bool(&self, grab: bool) -> Result<(), String> {
        let mode = if grab {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        };
        self.set_cursor_grab(mode)
    }

    pub fn set_cursor_icon(&self, icon: CursorIcon) {
        if let Some(window) = &self.window {
            window.set_engine_cursor_icon(icon);
        }
    }

    pub fn set_cursor_position(&self, x: f64, y: f64) -> Result<(), String> {
        let window = match &self.window {
            Some(w) => w,
            None => return Err("未设置窗口句柄".to_string()),
        };
        window.set_engine_cursor_position(x, y)
    }

    pub fn set_ime_allowed(&self, allowed: bool) {
        if let Some(window) = &self.window {
            window.set_ime_allowed(allowed);
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        let context = EngineContext::new(
            &self.config,
            &self.time,
            &self.filesystem,
            &self.thread_pool,
        );

        if let Err(e) = self.modules.initialize_all(&context) {
            eprintln!("模块初始化失败: {}", e);
            self.running = false;
            return;
        }

        self.time.tick();

        loop {
            if let Some(flag) = &self.quit_flag {
                if flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }

            let steps = self.time.tick();
            let dt = self.time.delta_seconds();
            let fixed_timestep = self.time.fixed_timestep();

            let context = EngineContext::new(
                &self.config,
                &self.time,
                &self.filesystem,
                &self.thread_pool,
            );

            for _ in steps.iter() {
                self.modules.update_all(&context, fixed_timestep);
            }

            self.modules.update_all(&context, dt);
            self.modules.render_all(&context);

            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        let context = EngineContext::new(
            &self.config,
            &self.time,
            &self.filesystem,
            &self.thread_pool,
        );

        self.modules.shutdown_all(&context);
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
    fn test_engine_modules() {
        let engine = Engine::new(EngineConfig::default());
        assert!(engine.modules().is_empty());
    }

    #[test]
    fn test_engine_is_running() {
        let engine = Engine::new(EngineConfig::default());
        assert!(!engine.is_running());
    }
}