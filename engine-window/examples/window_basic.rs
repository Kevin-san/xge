//! 基础窗口示例 — 创建窗口并在 3 秒后自动退出
use engine_window::{
    Event, EventLoop, WindowBuilder, WindowEvent,
};
use std::time::{Duration, Instant};

fn main() {
    println!("=== Window Basic Example ===");

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let _window = WindowBuilder::new()
        .with_title("Basic Window")
        .with_inner_size(800, 600)
        .build(&event_loop)
        .expect("Failed to create window");

    println!("Window created! Will auto-close in 3 seconds...");
    let start = Instant::now();

    event_loop
        .run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = &event {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window close requested");
                    }
                    WindowEvent::Resized(size) => {
                        println!("Window resized: {}x{}", size.width, size.height);
                    }
                    _ => {}
                }
            }

            if start.elapsed() >= Duration::from_secs(3) {
                println!("3 seconds elapsed, exiting...");
                elwt.exit();
            }
        })
        .expect("Event loop error");
}
