use crate::log::{self, Level};
use crate::module::ModuleRegistry;
use crate::schedule::Schedule;
use crate::time::Time;
use engine_window::{CursorGrabMode, CursorIcon, InputModule, PhysicalPosition, Window};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub target_fps: u32,
    pub log_level: Level,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Game Engine".to_string(),
            window_width: 1280,
            window_height: 720,
            target_fps: 60,
            log_level: Level::Info,
        }
    }
}

pub struct Engine {
    config: EngineConfig,
    modules: ModuleRegistry,
    schedule: Schedule,
    time: Time,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    running: Arc<AtomicBool>,
    quit_requested: Arc<AtomicBool>,
    external_quit: Option<Arc<AtomicBool>>,
    window: Option<Window>,
    input_module: Arc<Mutex<InputModule>>,
    focused: Arc<AtomicBool>,
    minimized: Arc<AtomicBool>,
    maximized: Arc<AtomicBool>,
    visible: Arc<AtomicBool>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        log::init(config.log_level);
        log::info("engine", "Engine initialized");

        Self {
            config,
            modules: ModuleRegistry::new(),
            schedule: Schedule::new(),
            time: Time::new(),
            resources: HashMap::new(),
            running: Arc::new(AtomicBool::new(false)),
            quit_requested: Arc::new(AtomicBool::new(false)),
            external_quit: None,
            window: None,
            input_module: Arc::new(Mutex::new(InputModule::new())),
            focused: Arc::new(AtomicBool::new(true)),
            minimized: Arc::new(AtomicBool::new(false)),
            maximized: Arc::new(AtomicBool::new(false)),
            visible: Arc::new(AtomicBool::new(true)),
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

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn request_quit(&self) {
        self.quit_requested.store(true, Ordering::SeqCst);
    }

    pub fn should_quit(&self) -> bool {
        self.quit_requested.load(Ordering::SeqCst)
    }

    pub fn set_quit_flag(&mut self, flag: Arc<AtomicBool>) {
        self.external_quit = Some(flag);
    }

    fn external_quit_raised(&self) -> bool {
        self.external_quit
            .as_ref()
            .map(|f| f.load(Ordering::SeqCst))
            .unwrap_or(false)
    }

    pub fn insert_resource<R: Send + Sync + 'static>(&mut self, resource: R) {
        let tid = TypeId::of::<R>();
        self.resources.insert(tid, Box::new(resource));
    }

    pub fn get_resource<R: Send + Sync + 'static>(&self) -> Option<&R> {
        let tid = TypeId::of::<R>();
        self.resources.get(&tid).and_then(|b| b.downcast_ref::<R>())
    }

    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn input(&self) -> Option<std::sync::MutexGuard<'_, InputModule>> {
        self.input_module.lock().ok()
    }

    pub fn is_focused(&self) -> bool {
        self.focused.load(Ordering::SeqCst)
    }

    pub fn is_minimized(&self) -> bool {
        self.minimized.load(Ordering::SeqCst)
    }

    pub fn is_maximized(&self) -> bool {
        self.maximized.load(Ordering::SeqCst)
    }

    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::SeqCst)
    }

    pub fn show_cursor(&self, show: bool) {
        if let Some(window) = &self.window {
            window.set_cursor_visible(show);
        }
    }

    pub fn set_cursor_grab(&self, grab: bool) {
        if let Some(window) = &self.window {
            let mode = if grab {
                CursorGrabMode::Confined
            } else {
                CursorGrabMode::None
            };
            let _ = window.set_cursor_grab(mode);
        }
    }

    pub fn set_cursor_icon(&self, icon: CursorIcon) {
        if let Some(window) = &self.window {
            window.set_cursor_icon(icon);
        }
    }

    pub fn set_cursor_position(&self, x: f64, y: f64) {
        if let Some(window) = &self.window {
            let _ = window.set_cursor_position(PhysicalPosition { x, y });
        }
    }

    pub fn set_ime_allowed(&self, allowed: bool) {
        if let Some(window) = &self.window {
            window.set_ime_allowed(allowed);
        }
    }

    pub fn set_ime_position(&self, _x: f64, _y: f64) {
        // IME position setting not supported in current winit version
    }

    pub fn run(&mut self) {
        log::info("engine", "Starting main loop");
        self.running.store(true, Ordering::SeqCst);

        if let Err(e) = self.modules.initialize_all() {
            log::error("engine", &format!("Failed to initialize modules: {}", e));
            return;
        }

        while self.is_running() && !self.should_quit() && !self.external_quit_raised() {
            self.time.update();
            let dt = self.time.delta_time();

            self.modules.update_all(dt);
            self.schedule.run();
            self.modules.render_all();

            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        self.modules.shutdown_all();
        log::info("engine", "Engine shutdown complete");
        self.running.store(false, Ordering::SeqCst);
    }
}