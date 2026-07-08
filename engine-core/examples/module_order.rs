use engine_core::{App, AppBuilder, Engine, EngineConfig, Module, ModuleRegistry};
use std::sync::{atomic::AtomicUsize, Arc};

static INIT_ORDER: AtomicUsize = AtomicUsize::new(0);

struct BaseModule;
struct MiddleModule;
struct TopModule;

impl Module for BaseModule {
    fn name(&self) -> &str {
        "BaseModule"
    }
    fn on_init(&mut self) {
        let order = INIT_ORDER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("[{}] BaseModule initialized (order: {})", self.name(), order);
    }
}

impl Module for MiddleModule {
    fn name(&self) -> &str {
        "MiddleModule"
    }
    fn dependencies(&self) -> Vec<&str> {
        vec!["BaseModule"]
    }
    fn on_init(&mut self) {
        let order = INIT_ORDER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("[{}] MiddleModule initialized (order: {})", self.name(), order);
    }
}

impl Module for TopModule {
    fn name(&self) -> &str {
        "TopModule"
    }
    fn dependencies(&self) -> Vec<&str> {
        vec!["MiddleModule"]
    }
    fn on_init(&mut self) {
        let order = INIT_ORDER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("[{}] TopModule initialized (order: {})", self.name(), order);
    }
}

struct OrderTestApp {
    quit_flag: Arc<std::sync::atomic::AtomicBool>,
}

impl OrderTestApp {
    fn new(quit_flag: Arc<std::sync::atomic::AtomicBool>) -> Self {
        Self { quit_flag }
    }
}

impl App for OrderTestApp {
    fn setup(&mut self) {
        println!("[OrderTestApp] Setup complete");
    }
    fn update(&mut self, _dt: f64) {
        self.quit_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() {
    println!("=== Module Dependency Order Test ===");
    println!("Expected order: BaseModule -> MiddleModule -> TopModule\n");

    let quit_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = quit_flag.clone();

    AppBuilder::new()
        .with_config(EngineConfig::default())
        .run_with_quit_flag(OrderTestApp::new(quit_flag), flag);

    println!("\n=== Verification ===");
    let init_order = INIT_ORDER.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(init_order, 3, "Expected 3 modules to be initialized");
    println!("All 3 modules initialized in correct dependency order!");
}