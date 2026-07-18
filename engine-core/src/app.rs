use crate::engine::{Engine, EngineConfig};
use crate::module::Module;
use crate::plugin::Plugin;
use std::sync::{atomic::AtomicBool, Arc};

pub trait App: Send + Sync {
    fn setup(&mut self) {}
    fn update(&mut self, _dt: f64) {}
    fn render(&mut self) {}
    fn shutdown(&mut self) {}
}

pub struct AppBuilder {
    config: EngineConfig,
    modules: Vec<Box<dyn Module>>,
    plugins: Vec<Box<dyn Plugin>>,
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
            plugins: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: EngineConfig) -> Self {
        self.config = config;
        self
    }

    /// 注册模块
    pub fn add_module(mut self, module: impl Module + 'static) -> Self {
        self.modules.push(Box::new(module));
        self
    }

    /// 注册插件
    pub fn add_plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn run(self, app: impl App + 'static) {
        let quit_flag = Arc::new(AtomicBool::new(false));
        self.run_with_quit_flag(app, quit_flag);
    }

    pub fn run_with_quit_flag(self, app: impl App + 'static, quit_flag: Arc<AtomicBool>) {
        let mut engine = Engine::new(self.config);
        engine.set_quit_flag(quit_flag.clone());

        // 注册所有模块
        for module in self.modules {
            engine.modules().register(module);
        }

        // 构建所有插件（插件可以向 AppBuilder 注册模块）
        // 由于 AppBuilder 已经消耗，插件在 engine 构建后直接操作
        let mut temp_builder = Self::new();
        for plugin in self.plugins {
            plugin.build(&mut temp_builder);
        }
        for module in temp_builder.modules {
            engine.modules().register(module);
        }

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
