use crate::{Engine, EngineConfig, Module};

pub trait App: Send + Sync + 'static {
    fn setup(&mut self, engine: &Engine);
    fn update(&mut self, engine: &mut Engine, dt: f64);
    fn render(&mut self, engine: &mut Engine);
    fn shutdown(&mut self, engine: &Engine);
}

pub struct AppBuilder {
    config: EngineConfig,
    modules: Vec<Box<dyn Module>>,
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

    pub fn add_module<M: Module + 'static>(mut self, module: M) -> Self {
        self.modules.push(Box::new(module));
        self
    }

    pub fn run(self, mut app: impl App) {
        let mut engine = Engine::new(self.config);
        
        for module in self.modules {
            engine.register_module(module);
        }

        app.setup(&engine);
        engine.run();
        app.shutdown(&engine);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestApp {
        setup_called: bool,
        update_called: bool,
        shutdown_called: bool,
    }

    impl TestApp {
        fn new() -> Self {
            Self {
                setup_called: false,
                update_called: false,
                shutdown_called: false,
            }
        }
    }

    impl App for TestApp {
        fn setup(&mut self, _engine: &Engine) {
            self.setup_called = true;
        }

        fn update(&mut self, _engine: &mut Engine, _dt: f64) {
            self.update_called = true;
        }

        fn render(&mut self, _engine: &Engine) {}

        fn shutdown(&mut self, _engine: &Engine) {
            self.shutdown_called = true;
        }
    }

    #[test]
    fn app_builder_new() {
        let builder = AppBuilder::new();
        assert!(builder.modules.is_empty());
    }

    #[test]
    fn app_builder_with_config() {
        let config = EngineConfig { frame_limit: Some(1), ..Default::default() };
        let builder = AppBuilder::new().with_config(config);
        assert_eq!(builder.config.frame_limit, Some(1));
    }

    #[test]
    fn app_builder_add_module() {
        struct DummyModule;
        impl Module for DummyModule {
            fn name(&self) -> &str { "DummyModule" }
            fn dependencies(&self) -> Vec<&str> { Vec::new() }
            fn on_init(&mut self, _engine: &Engine) {}
            fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
            fn on_render(&mut self, _engine: &Engine) {}
            fn on_shutdown(&mut self, _engine: &Engine) {}
            fn enabled(&self) -> bool { true }
        }

        let builder = AppBuilder::new().add_module(DummyModule);
        assert_eq!(builder.modules.len(), 1);
    }

    #[test]
    fn app_builder_run() {
        let config = EngineConfig { frame_limit: Some(1), ..Default::default() };
        let mut app = TestApp::new();
        
        AppBuilder::new()
            .with_config(config)
            .run(&mut app);
        
        assert!(app.setup_called);
        assert!(app.update_called);
        assert!(app.shutdown_called);
    }
}
