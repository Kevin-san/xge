use crate::module::{Module, ModuleRegistry, CycleError};
use crate::schedule::Schedule;
use engine_platform::{Time, FileSystem, NativeFileSystem, ThreadPool, JoinHandle};
use parking_lot::RwLock;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct EngineConfig {
    pub frame_limit: Option<u64>,
    pub tick_rate: f32,
}

pub struct Engine {
    config: EngineConfig,
    module_registry: ModuleRegistry,
    schedule: Schedule,
    time: Time,
    filesystem: Box<dyn FileSystem>,
    thread_pool: ThreadPool,
    running: Arc<AtomicBool>,
    world: RwLock<World>,
}

pub struct World;

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            module_registry: ModuleRegistry::new(),
            schedule: Schedule::new(),
            time: Time::new(),
            filesystem: Box::new(NativeFileSystem),
            thread_pool: ThreadPool::new(None).unwrap(),
            running: Arc::new(AtomicBool::new(false)),
            world: RwLock::new(World),
        }
    }

    pub fn run(&mut self) {
        self.running.store(true, Ordering::SeqCst);
        
        match self.module_registry.initialize_all(self) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to initialize modules: {}", e);
                return;
            }
        }

        while self.running.load(Ordering::SeqCst) {
            self.time.tick();
            
            let dt = self.time.delta_seconds() as f64;
            
            self.schedule.run(self);
            self.module_registry.update_all(self, dt);

            if let Some(limit) = self.config.frame_limit {
                if self.time.frame_count() >= limit {
                    self.request_quit();
                }
            }
        }

        self.module_registry.shutdown_all(self);
    }

    pub fn request_quit(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn module<T: Module>(&self) -> Option<&T> {
        self.module_registry.get::<T>()
    }

    pub fn module_mut<T: Module>(&mut self) -> Option<&mut T> {
        self.module_registry.get_mut::<T>()
    }

    pub fn world(&self) -> &World {
        &*self.world.read()
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut *self.world.write()
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn time_mut(&mut self) -> &mut Time {
        &mut self.time
    }

    pub fn filesystem(&self) -> &dyn FileSystem {
        self.filesystem.as_ref()
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn spawn_task<F>(&self, f: F) -> JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.thread_pool.spawn(move || futures_lite::future::block_on(f))
    }

    pub fn schedule(&mut self) -> &mut Schedule {
        &mut self.schedule
    }

    pub(crate) fn register_module<M: Module + 'static>(&mut self, module: M) {
        self.module_registry.register(module);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModule {
        initialized: bool,
        updated: bool,
        shutdown: bool,
    }

    impl TestModule {
        fn new() -> Self {
            Self {
                initialized: false,
                updated: false,
                shutdown: false,
            }
        }
    }

    impl Module for TestModule {
        fn name(&self) -> &str { "TestModule" }
        fn dependencies(&self) -> Vec<&str> { Vec::new() }
        fn on_init(&mut self, _engine: &Engine) { self.initialized = true; }
        fn on_update(&mut self, _engine: &mut Engine, _dt: f64) { self.updated = true; }
        fn on_render(&mut self, _engine: &mut Engine) {}
        fn on_shutdown(&mut self, _engine: &Engine) { self.shutdown = true; }
        fn enabled(&self) -> bool { true }
    }

    #[test]
    fn engine_new() {
        let config = EngineConfig::default();
        let engine = Engine::new(config);
        assert!(!engine.is_running());
    }

    #[test]
    fn engine_is_running() {
        let config = EngineConfig { frame_limit: Some(1), ..Default::default() };
        let mut engine = Engine::new(config);
        assert!(!engine.is_running());
        engine.run();
        assert!(!engine.is_running());
    }

    #[test]
    fn engine_time_access() {
        let engine = Engine::new(EngineConfig::default());
        let _time = engine.time();
    }

    #[test]
    fn engine_filesystem_access() {
        let engine = Engine::new(EngineConfig::default());
        let _fs = engine.filesystem();
    }

    #[test]
    fn engine_config_access() {
        let config = EngineConfig::default();
        let engine = Engine::new(config);
        assert_eq!(engine.config().frame_limit, None);
    }

    #[test]
    fn engine_module_access() {
        let config = EngineConfig { frame_limit: Some(1), ..Default::default() };
        let mut engine = Engine::new(config);
        engine.register_module(TestModule::new());
        
        let module = engine.module::<TestModule>();
        assert!(module.is_some());
    }
}
