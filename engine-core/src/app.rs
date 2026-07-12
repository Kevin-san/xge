use crate::engine::{Engine, EngineConfig};
use crate::module::Module;
use std::sync::{atomic::AtomicBool, Arc};

pub trait App: Send + Sync + 'static {
    fn setup(&mut self, _engine: &crate::module::EngineContext<'_>) {}
    fn update(&mut self, _engine: &crate::module::EngineContext<'_>, _dt: f64) {}
    fn render(&mut self, _engine: &crate::module::EngineContext<'_>) {}
    fn shutdown(&mut self, _engine: &crate::module::EngineContext<'_>) {}
}

pub struct AppBuilder {
    config: EngineConfig,
    modules: Vec<Box<dyn Module>>,
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
            modules: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: EngineConfig) -> Self {
        self.config = config;
        self
    }

    pub fn add_module<T: Module + 'static>(mut self, module: T) -> Self {
        self.modules.push(Box::new(module));
        self
    }

    pub fn add_plugin<T: Plugin + 'static>(mut self, plugin: T) -> Self {
        plugin.build(&mut self);
        self
    }

    pub fn run(self, app: impl App) {
        let quit_flag = Arc::new(AtomicBool::new(false));
        self.run_with_quit_flag(app, quit_flag);
    }

    pub fn run_with_quit_flag(self, app: impl App, quit_flag: Arc<AtomicBool>) {
        let mut engine = Engine::new(self.config);
        engine.set_quit_flag(quit_flag.clone());

        for module in self.modules {
            engine.modules_mut().register(module);
        }

        let app_module = AppModule::new(app, quit_flag);
        engine.modules_mut().register(Box::new(app_module));

        engine.run();
    }
}

pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn build(&self, app: &mut AppBuilder);
}

pub struct PluginGroup {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginGroup {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn add<T: Plugin + 'static>(&mut self, plugin: T) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn install(&self, app: &mut AppBuilder) {
        for plugin in &self.plugins {
            plugin.build(app);
        }
    }
}

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn name(&self) -> &str {
        "DefaultPlugins"
    }

    fn build(&self, _app: &mut AppBuilder) {
    }
}

#[allow(dead_code)]
struct AppModule {
    app: Box<dyn App>,
    quit_flag: Arc<AtomicBool>,
}

impl AppModule {
    fn new(app: impl App, quit_flag: Arc<AtomicBool>) -> Self {
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

    fn on_init(&mut self, _engine: &crate::module::EngineContext<'_>) {}

    fn on_update(&mut self, _engine: &crate::module::EngineContext<'_>, _dt: f64) {}

    fn on_render(&mut self, _engine: &crate::module::EngineContext<'_>) {}

    fn on_shutdown(&mut self, _engine: &crate::module::EngineContext<'_>) {}
}