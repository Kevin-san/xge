//! 鼠标输入示例 — 实时打印鼠标位置和按钮
use engine_window::{
    Event, EventLoop, InputModule, WindowBuilder, WindowEvent,
};

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let _window = WindowBuilder::new()
        .with_title("Input Mouse Demo")
        .with_inner_size(800, 600)
        .build(&event_loop)
        .expect("Failed to create window");

    println!("=== Input Mouse Demo ===");
    println!("Move mouse and click buttons. Press Escape to exit.");

    let mut input_module = InputModule::new();

    event_loop
        .run(move |event, elwt| {
            input_module.process_event(&event);

            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::CursorMoved { .. } => {
                        let pos = input_module.input().mouse_position();
                        let delta = input_module.input().mouse_delta();
                        if delta.x != 0.0 || delta.y != 0.0 {
                            println!(
                                "Mouse pos: ({:.1}, {:.1}), delta: ({:.1}, {:.1})",
                                pos.x, pos.y, delta.x, delta.y
                            );
                        }
                    }
                    WindowEvent::MouseInput { .. } => {
                        for btn in input_module.input().pressed_buttons() {
                            println!("Mouse button pressed: {:?}", btn);
                        }
                    }
                    WindowEvent::MouseWheel { .. } => {
                        let wheel = input_module.input().wheel_delta();
                        println!("Mouse wheel: ({:.1}, {:.1})", wheel.x, wheel.y);
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    input_module.clear();
                }
                _ => {}
            }
        })
        .expect("Event loop error");
}
