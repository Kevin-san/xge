use crate::log::{self, Level};
use crate::module::ModuleRegistry;
use crate::schedule::Schedule;
use crate::time::Time;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
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
