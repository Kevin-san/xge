//! 全屏切换示例 — 按 F 键切换全屏
use engine_window::{
    Event, EventLoop, Fullscreen, InputModule, KeyCode, WindowBuilder, WindowEvent,
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = Rc::new(RefCell::new(
        WindowBuilder::new()
            .with_title("Fullscreen Demo")
            .with_inner_size(800, 600)
            .build(&event_loop)
            .expect("Failed to create window"),
    ));

    println!("=== Fullscreen Demo ===");
    println!("Press F to toggle fullscreen. Press Escape to exit.");

    let mut input_module = InputModule::new();
    let mut is_fullscreen = false;

    event_loop
        .run(move |event, elwt| {
            input_module.process_event(&event);

            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::KeyboardInput { .. } => {
                        if input_module.input().key_just_pressed(KeyCode::F) {
                            is_fullscreen = !is_fullscreen;
                            let w = window.borrow();
                            if is_fullscreen {
                                w.set_fullscreen(Some(Fullscreen::Borderless(w.current_monitor())));
                                println!("Entered fullscreen");
                            } else {
                                w.set_fullscreen(None);
                                println!("Exited fullscreen");
                            }
                        }
                        if input_module.input().key_pressed(KeyCode::Escape) {
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
