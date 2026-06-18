//! module_order - 模块注册与依赖顺序初始化示例

use engine_core::{Engine, EngineConfig, Module, ModuleRegistry};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// 模块 A：无依赖
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
    fn on_init(&mut self) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _dt: f64) {
        // 检查外部 quit flag
        if self.quit_flag.load(Ordering::SeqCst) {
            println!("[{}] Quit flag set, would request shutdown", self.name);
        }
    }
    fn on_render(&mut self) {}
    fn on_shutdown(&mut self) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool {
        true
    }
}

// 模块 B：依赖 A
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
    fn on_init(&mut self) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _dt: f64) {}
    fn on_render(&mut self) {}
    fn on_shutdown(&mut self) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool {
        true
    }
}

fn main() {
    let quit_flag = Arc::new(AtomicBool::new(false));

    let registry = ModuleRegistry::new();
    registry.register(Box::new(ModuleB::new(quit_flag.clone()))); // B 先注册
    registry.register(Box::new(ModuleA::new(quit_flag.clone()))); // A 后注册

    println!("Registered modules: {} modules", registry.len());

    // 创建引擎
    let _engine = Engine::new(EngineConfig::default());

    println!("\n--- Initializing (A should come before B) ---");
    if let Err(e) = registry.initialize_all() {
        eprintln!("Initialization error: {}", e);
        return;
    }

    println!("\n--- Running for 1 frame ---");
    registry.update_all(0.016);

    println!("\n--- Shutting down (B should come before A) ---");
    registry.shutdown_all();

    println!("\nTest completed successfully!");
}
