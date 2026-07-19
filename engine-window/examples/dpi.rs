//! DPI 缩放示例 — 打印 DPI 变化
use engine_window::{
    Event, EventLoop, WindowBuilder, WindowEvent,
};

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = WindowBuilder::new()
        .with_title("DPI Demo")
        .with_inner_size(800, 600)
        .build(&event_loop)
        .expect("Failed to create window");

    println!("=== DPI Demo ===");
    println!("Current scale factor: {:.2}", window.scale_factor());
    println!(
        "Move window between monitors to see DPI changes. Close the window to exit."
    );

    event_loop
        .run(move |event, elwt| match &event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    println!("Scale factor changed: {:.2}", scale_factor);
                }
                _ => {}
            },
            _ => {}
        })
        .expect("Event loop error");
}
