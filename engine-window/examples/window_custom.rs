//! 自定义窗口示例 — 使用 WindowConfig 创建 1280x720 窗口
use engine_window::{
    Event, EventLoop, WindowConfig, WindowEvent,
};

fn main() {
    let config = WindowConfig::from_title("Custom Window")
        .with_size(1280, 720)
        .with_resizable(true)
        .with_decorations(true);

    println!("=== Custom Window Example ===");
    println!(
        "Config: {}x{} resizable={} decorations={}",
        config.width, config.height, config.resizable, config.decorations
    );

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let _window = config
        .to_builder()
        .build(&event_loop)
        .expect("Failed to create window");

    println!("Window created. Close the window or press Ctrl+C to exit.");

    event_loop
        .run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = &event {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    _ => {}
                }
            }
        })
        .expect("Event loop error");
}
