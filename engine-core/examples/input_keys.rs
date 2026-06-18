use engine_core::{Engine, EngineConfig};

fn main() {
    let config = EngineConfig {
        window_title: "Input Keys Example".to_string(),
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(config);

    engine.run();
}
