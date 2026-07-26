use engine_core::{Engine, EngineConfig};

fn main() {
    let config = EngineConfig {
        window_title: "Window Basic Example".to_string(),
        ..EngineConfig::default()
    };

    let mut engine = Engine::new(config);

    println!("Window will close after 3 seconds...");

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));
    });

    engine.run();
}
