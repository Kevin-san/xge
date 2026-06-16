use engine_core::{Engine, EngineConfig};
use engine_window::KeyCode;

fn main() {
    let config = EngineConfig {
        window_title: "Input Keys Example".to_string(),
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(config);

    engine.run();
}
