use engine_core::{Engine, EngineConfig, Module, ModuleRegistry};

struct ModuleA {
    name: String,
}

impl ModuleA {
    fn new() -> Self {
        Self { name: "ModuleA".into() }
    }
}

impl Module for ModuleA {
    fn name(&self) -> &str { &self.name }
    fn dependencies(&self) -> Vec<&str> { vec![] }
    fn on_init(&mut self, _engine: &Engine) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
    fn on_render(&mut self, _engine: &Engine) {}
    fn on_shutdown(&mut self, _engine: &Engine) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool { true }
}

struct ModuleB {
    name: String,
}

impl ModuleB {
    fn new() -> Self {
        Self { name: "ModuleB".into() }
    }
}

impl Module for ModuleB {
    fn name(&self) -> &str { &self.name }
    fn dependencies(&self) -> Vec<&str> { vec!["ModuleA"] }
    fn on_init(&mut self, _engine: &Engine) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
    fn on_render(&mut self, _engine: &Engine) {}
    fn on_shutdown(&mut self, _engine: &Engine) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool { true }
}

fn main() {
    let mut registry = ModuleRegistry::new();
    registry.register(ModuleB::new());
    registry.register(ModuleA::new());

    let engine = Engine::new(EngineConfig::default());
    registry.initialize_all(&engine).unwrap();

    registry.shutdown_all(&engine);
}
