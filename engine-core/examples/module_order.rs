use engine_core::{Engine, EngineConfig, EngineContext, Module, ModuleRegistry};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

struct ModuleA {
    name: String,
    quit_flag: Arc<AtomicBool>,
}

impl ModuleA {
    fn new(quit_flag: Arc<AtomicBool>) -> Self {
        Self {
            name: "ModuleA".into(),
            quit_flag,
        }
    }
}

impl Module for ModuleA {
    fn name(&self) -> &str {
        &self.name
    }
    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }
    fn on_init(&mut self, _context: &EngineContext<'_>) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _context: &EngineContext<'_>, _dt: f64) {
        if self.quit_flag.load(Ordering::SeqCst) {
            println!("[{}] Quit flag set, would request shutdown", self.name);
        }
    }
    fn on_render(&mut self, _context: &EngineContext<'_>) {}
    fn on_shutdown(&mut self, _context: &EngineContext<'_>) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
struct ModuleB {
    name: String,
    quit_flag: Arc<AtomicBool>,
}

impl ModuleB {
    fn new(quit_flag: Arc<AtomicBool>) -> Self {
        Self {
            name: "ModuleB".into(),
            quit_flag,
        }
    }
}

impl Module for ModuleB {
    fn name(&self) -> &str {
        &self.name
    }
    fn dependencies(&self) -> Vec<&str> {
        vec!["ModuleA"]
    }
    fn on_init(&mut self, _context: &EngineContext<'_>) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _context: &EngineContext<'_>, _dt: f64) {}
    fn on_render(&mut self, _context: &EngineContext<'_>) {}
    fn on_shutdown(&mut self, _context: &EngineContext<'_>) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool {
        true
    }
}

fn main() {
    let quit_flag = Arc::new(AtomicBool::new(false));

    let mut registry = ModuleRegistry::new();
    registry.register(Box::new(ModuleB::new(quit_flag.clone())));
    registry.register(Box::new(ModuleA::new(quit_flag.clone())));

    println!("Registered modules: {} modules", registry.len());

    let mut engine = Engine::new(EngineConfig::default());
    let context = EngineContext::new(
        &engine.config(),
        &engine.time(),
        &engine.filesystem(),
        engine.thread_pool(),
    );

    println!("\n--- Initializing (A should come before B) ---");
    if let Err(e) = registry.initialize_all(&context) {
        eprintln!("Initialization error: {}", e);
        return;
    }

    println!("\n--- Running for 1 frame ---");
    registry.update_all(&context, 0.016);

    println!("\n--- Shutting down (B should come before A) ---");
    registry.shutdown_all(&context);

    println!("\nTest completed successfully!");
}