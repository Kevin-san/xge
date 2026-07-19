//! 事件循环代理示例 — 跨线程发送事件
use engine_window::{
    Event, EventLoop, WindowBuilder, WindowEvent,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let proxy = event_loop.create_proxy();
    let _window = WindowBuilder::new()
        .with_title("Event Loop Proxy Demo")
        .with_inner_size(800, 600)
        .build(&event_loop)
        .expect("Failed to create window");

    println!("=== Event Loop Proxy Demo ===");
    println!("Background thread will wake up the event loop every second.");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    thread::spawn(move || {
        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            println!("Background thread: tick {} (waking event loop)", i);
            let _ = proxy.send_event(());
        }
        running_clone.store(false, Ordering::SeqCst);
    });

    event_loop
        .run(move |event, elwt| {
            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        running.store(false, Ordering::SeqCst);
                        elwt.exit();
                    }
                    _ => {}
                },
                Event::UserEvent(()) => {
                    println!("Event loop received wake-up from proxy!");
                }
                _ => {}
            }

            if !running.load(Ordering::SeqCst) {
                elwt.exit();
            }
        })
        .expect("Event loop error");
}
