//! 键盘输入示例 — 实时打印按键事件
use engine_window::{
    Event, EventLoop, InputModule, KeyCode, WindowBuilder, WindowEvent,
};

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let _window = WindowBuilder::new()
        .with_title("Input Keys Demo")
        .with_inner_size(800, 600)
        .build(&event_loop)
        .expect("Failed to create window");

    println!("=== Input Keys Demo ===");
    println!("Press any key to see its KeyCode. Press Escape to exit.");

    let mut input_module = InputModule::new();

    event_loop
        .run(move |event, elwt| {
            input_module.process_event(&event);

            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::KeyboardInput { .. } => {
                        for key in input_module.input().pressed_keys() {
                            println!("Key pressed: {:?}", key);
                        }
                        if input_module.input().key_pressed(KeyCode::Escape) {
                            println!("Escape pressed, exiting...");
                            elwt.exit();
                        }
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
