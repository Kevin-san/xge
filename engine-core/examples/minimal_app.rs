//! minimal_app - 完整 App trait 实现示例

use engine_core::{App, AppBuilder, EngineConfig};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

struct MyGame {
    frame_count: u64,
    max_frames: u64,
    quit_flag: Arc<AtomicBool>,
}

impl MyGame {
    fn new(quit_flag: Arc<AtomicBool>) -> Self {
        Self {
            frame_count: 0,
            max_frames: 5,
            quit_flag,
        }
    }
}

impl App for MyGame {
    fn setup(&mut self) {
        println!("[MyGame] Setup complete");
    }

    fn update(&mut self, dt: f64) {
        self.frame_count += 1;
        println!("[MyGame] Update frame {} (dt={:.2}ms)",
                 self.frame_count, dt * 1000.0);

        // 达到最大帧数后请求退出
        if self.frame_count >= self.max_frames {
            self.quit_flag.store(true, Ordering::SeqCst);
        }
    }

    fn render(&mut self) {
        // 渲染逻辑（空实现）
    }

    fn shutdown(&mut self) {
        println!("[MyGame] Shutdown after {} frames", self.frame_count);
    }
}

fn main() {
    let quit_flag = Arc::new(AtomicBool::new(false));

    AppBuilder::new()
        .with_config(EngineConfig::default())
        .run_with_quit_flag(MyGame::new(quit_flag.clone()), quit_flag);
}