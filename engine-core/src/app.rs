use crate::engine::{Engine, EngineConfig};
use crate::module::Module;
use std::sync::{atomic::AtomicBool, Arc};

pub trait App: Send + Sync {
    fn setup(&mut self) {}
    fn update(&mut self, _dt: f64) {}
    fn render(&mut self) {}
    fn shutdown(&mut self) {}
}

pub struct AppBuilder {
    config: EngineConfig,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            config: EngineConfig::default(),
        }
    }

    pub fn with_config(mut self, config: EngineConfig) -> Self {
        self.config = config;
        self
    }

    pub fn run(self, app: impl App + 'static) {
        let quit_flag = Arc::new(AtomicBool::new(false));
        self.run_with_quit_flag(app, quit_flag);
    }

    pub fn run_with_quit_flag(self, app: impl App + 'static, quit_flag: Arc<AtomicBool>) {
        let mut engine = Engine::new(self.config);
        engine.set_quit_flag(quit_flag.clone());

        let app_module = AppModule::new(app, quit_flag);
        engine.modules().register(Box::new(app_module));

        engine.run();
    }
}

#[allow(dead_code)]
struct AppModule {
    app: Box<dyn App>,
    quit_flag: Arc<AtomicBool>,
}

impl AppModule {
    fn new(app: impl App + 'static, quit_flag: Arc<AtomicBool>) -> Self {
        Self {
            app: Box::new(app),
            quit_flag,
        }
    }
}

impl Module for AppModule {
    fn name(&self) -> &str {
        "AppModule"
    }

    fn on_init(&mut self) {
        self.app.setup();
    }

    fn on_update(&mut self, dt: f64) {
        self.app.update(dt);
    }

    fn on_render(&mut self) {
        self.app.render();
    }

    fn on_shutdown(&mut self) {
        self.app.shutdown();
    }
}
