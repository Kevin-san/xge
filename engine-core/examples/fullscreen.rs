use engine_core::{Engine, EngineConfig};
use engine_window::{Fullscreen, KeyCode};

fn main() {
    let config = EngineConfig {
        window_title: "Fullscreen Example".to_string(),
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(config);

    engine.run();
}
