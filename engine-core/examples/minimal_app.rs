use engine_core::{App, AppBuilder, Engine, EngineConfig};

struct MyGame {
    frame_count: u64,
}

impl Default for MyGame {
    fn default() -> Self {
        Self { frame_count: 0 }
    }
}

impl App for MyGame {
    fn setup(&mut self, _engine: &Engine) {
        println!("[MyGame] Setup complete");
    }

    fn update(&mut self, _engine: &mut Engine, dt: f64) {
        self.frame_count += 1;
        println!("[MyGame] Update frame {} (dt={:.2}ms)", self.frame_count, dt * 1000.0);
    }

    fn render(&mut self, _engine: &Engine) {}

    fn shutdown(&mut self, _engine: &Engine) {
        println!("[MyGame] Shutdown after {} frames", self.frame_count);
    }
}

fn main() {
    AppBuilder::new()
        .with_config(EngineConfig {
            frame_limit: Some(3),
            ..EngineConfig::default()
        })
        .run(MyGame::default());
}
