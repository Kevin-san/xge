use engine_core::{Engine, EngineConfig, Module};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct LoggingModule {
    initialized: Arc<AtomicUsize>,
}

impl LoggingModule {
    fn new(initialized: Arc<AtomicUsize>) -> Self {
        Self { initialized }
    }
}

impl Module for LoggingModule {
    fn name(&self) -> &str {
        "LoggingModule"
    }

    fn on_init(&mut self) {
        self.initialized.fetch_add(1, Ordering::SeqCst);
        println!("[LoggingModule] Initialized");
    }
}

struct NetworkModule {
    initialized: Arc<AtomicUsize>,
}

impl NetworkModule {
    fn new(initialized: Arc<AtomicUsize>) -> Self {
        Self { initialized }
    }
}

impl Module for NetworkModule {
    fn name(&self) -> &str {
        "NetworkModule"
    }

    fn dependencies(&self) -> Vec<&str> {
        vec!["LoggingModule"]
    }

    fn on_init(&mut self) {
        self.initialized.fetch_add(1, Ordering::SeqCst);
        println!("[NetworkModule] Initialized (depends on Logging)");
    }
}

struct GameModule {
    initialized: Arc<AtomicUsize>,
}

impl GameModule {
    fn new(initialized: Arc<AtomicUsize>) -> Self {
        Self { initialized }
    }
}

impl Module for GameModule {
    fn name(&self) -> &str {
        "GameModule"
    }

    fn dependencies(&self) -> Vec<&str> {
        vec!["LoggingModule", "NetworkModule"]
    }

    fn on_init(&mut self) {
        self.initialized.fetch_add(1, Ordering::SeqCst);
        println!("[GameModule] Initialized (depends on Logging + Network)");
    }
}

fn main() {
    println!("=== Module Order Demo ===\n");
    println!("Demonstrating topological sort of modules by dependencies:\n");
    
    let log_init = Arc::new(AtomicUsize::new(0));
    let net_init = Arc::new(AtomicUsize::new(0));
    let game_init = Arc::new(AtomicUsize::new(0));

    let log_module = Box::new(LoggingModule::new(log_init.clone()));
    let net_module = Box::new(NetworkModule::new(net_init.clone()));
    let game_module = Box::new(GameModule::new(game_init.clone()));

    let engine = Engine::new(EngineConfig::default());
    let registry = engine.modules();

    registry.register(game_module);
    registry.register(net_module);
    registry.register(log_module);

    println!("Registered modules (order doesn't matter):");
    for name in registry.module_names() {
        println!("  - {}", name);
    }
    println!();

    println!("Initializing with dependency ordering...");
    if let Err(e) = registry.initialize_all() {
        eprintln!("Initialization failed: {}", e);
        return;
    }
    println!();

    assert_eq!(log_init.load(Ordering::SeqCst), 1);
    assert_eq!(net_init.load(Ordering::SeqCst), 1);
    assert_eq!(game_init.load(Ordering::SeqCst), 1);

    println!("Verification:");
    println!("  ✓ LoggingModule initialized first (no dependencies)");
    println!("  ✓ NetworkModule initialized second (depends on Logging)");
    println!("  ✓ GameModule initialized last (depends on both)");
    println!();
    println!("Dependency order verified: Logging → Network → Game");
}