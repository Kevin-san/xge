use engine_core::{Engine, EngineConfig};

fn main() {
    println!("Engine Version: {}", engine_core::ENGINE_VERSION);
    println!("Build: {} @ {}", engine_core::BUILD_COMMIT_HASH, engine_core::BUILD_TIMESTAMP);

    let config = EngineConfig {
        frame_limit: Some(1),
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(config);
    engine.run();
}
