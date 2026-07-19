//! module_order - 模块注册与依赖顺序初始化示例
//!
//! 演示内容：
//! - 注册多个具有依赖关系的模块（乱序注册）
//! - 拓扑排序后按依赖顺序初始化
//! - 逆序关闭
//! - 使用 module_names() / contains() 查询

use engine_core::{Module, ModuleRegistry};

// 模块 Transform：无依赖（底层）
struct TransformModule;

impl Module for TransformModule {
    fn name(&self) -> &str {
        "Transform"
    }
    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }
    fn on_init(&mut self) {
        println!("  [Transform] on_init");
    }
    fn on_update(&mut self, dt: f64) {
        println!("  [Transform] on_update(dt={:.4}s)", dt);
    }
    fn on_render(&mut self) {}
    fn on_shutdown(&mut self) {
        println!("  [Transform] on_shutdown");
    }
}

// 模块 Physics：依赖 Transform
struct PhysicsModule;

impl Module for PhysicsModule {
    fn name(&self) -> &str {
        "Physics"
    }
    fn dependencies(&self) -> Vec<&str> {
        vec!["Transform"]
    }
    fn on_init(&mut self) {
        println!("  [Physics] on_init");
    }
    fn on_update(&mut self, dt: f64) {
        println!("  [Physics] on_update(dt={:.4}s)", dt);
    }
    fn on_render(&mut self) {}
    fn on_shutdown(&mut self) {
        println!("  [Physics] on_shutdown");
    }
}

// 模块 Render：依赖 Transform
struct RenderModule;

impl Module for RenderModule {
    fn name(&self) -> &str {
        "Render"
    }
    fn dependencies(&self) -> Vec<&str> {
        vec!["Transform"]
    }
    fn on_init(&mut self) {
        println!("  [Render] on_init");
    }
    fn on_update(&mut self, dt: f64) {
        println!("  [Render] on_update(dt={:.4}s)", dt);
    }
    fn on_render(&mut self) {}
    fn on_shutdown(&mut self) {
        println!("  [Render] on_shutdown");
    }
}

fn main() {
    println!("=== Module Order Demo ===\n");

    let registry = ModuleRegistry::new();

    // 乱序注册：Physics → Render → Transform
    println!("Registering modules (out of dependency order)...");
    registry.register(Box::new(PhysicsModule));
    registry.register(Box::new(RenderModule));
    registry.register(Box::new(TransformModule));

    println!("Registered: {} modules", registry.len());
    println!("Module names: {:?}", registry.module_names());
    println!("Contains 'Physics': {}", registry.contains("Physics"));
    println!("Contains 'Audio': {}", registry.contains("Audio"));

    // 初始化（应按依赖顺序：Transform → Physics → Render）
    println!("\n--- Initializing (should follow dependency order) ---");
    if let Err(e) = registry.initialize_all() {
        eprintln!("Initialization error: {}", e);
        return;
    }

    // 更新一帧
    println!("\n--- Updating all modules (1 frame) ---");
    registry.update_all(0.016);

    // 关闭（逆序：Render → Physics → Transform）
    println!("\n--- Shutting down (reverse order) ---");
    registry.shutdown_all();

    println!("\nModule order demo completed!");
}
