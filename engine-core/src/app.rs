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

    pub fn add_module(self, _module: impl Module + 'static) -> Self {
        // Module registration is handled internally by Engine
        // This method exists for API compatibility
        self
    }

    pub fn add_plugin(self, _plugin: impl Plugin + 'static) -> Self {
        // Plugin registration is handled internally by Engine
        // This method exists for API compatibility
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestApp {
        setup_called: bool,
        update_called: bool,
        render_called: bool,
        shutdown_called: bool,
    }

    impl TestApp {
        fn new() -> Self {
            Self {
                setup_called: false,
                update_called: false,
                render_called: false,
                shutdown_called: false,
            }
        }
    }

    impl App for TestApp {
        fn setup(&mut self) {
            self.setup_called = true;
        }
        fn update(&mut self, _dt: f64) {
            self.update_called = true;
        }
        fn render(&mut self) {
            self.render_called = true;
        }
        fn shutdown(&mut self) {
            self.shutdown_called = true;
        }
    }

    #[test]
    fn test_app_builder_new() {
        let builder = AppBuilder::new();
        assert_eq!(builder.config.window_title, "Game Engine");
    }

    #[test]
    fn test_app_builder_with_config() {
        let config = EngineConfig {
            window_title: "Test App".to_string(),
            window_width: 800,
            window_height: 600,
            target_fps: 30,
            log_level: "debug".to_string(),
        };
        let builder = AppBuilder::new().with_config(config.clone());
        assert_eq!(builder.config.window_title, "Test App");
        assert_eq!(builder.config.window_width, 800);
    }

    #[test]
    fn test_app_builder_default() {
        let builder = AppBuilder::default();
        assert_eq!(builder.config.window_title, "Game Engine");
    }
}
