//! event_bus_demo - 事件总线订阅/派发/取消订阅示例
//!
//! 演示内容：
//! - 回调式：subscribe / send / unsubscribe
//! - 队列式：send / iter / drain_queue
//! - subscriber_count / len / snapshot

use engine_core::EventBus;

#[derive(Clone, Debug, PartialEq)]
enum GameEvent {
    PlayerJoined(String),
    ScoreChanged(i32),
    GameOver,
}

fn main() {
    println!("=== EventBus Demo ===\n");

    let bus = EventBus::<GameEvent>::new();

    // ===== 回调式 =====
    println!("--- Callback mode ---");
    let handle1 = bus.subscribe(|event| {
        println!("  [Callback1] {:?}", event);
    });

    let handle2 = bus.subscribe(|event| {
        println!("  [Callback2] {:?}", event);
    });

    println!("Subscribers: {}", bus.subscriber_count());

    // 派发事件（回调 + 队列同时生效）
    println!("\nSending events...");
    bus.send(GameEvent::PlayerJoined("Alice".to_string()));
    bus.send(GameEvent::ScoreChanged(100));
    println!("Queue length: {}", bus.len());

    // 取消订阅
    println!("\nUnsubscribing handle2...");
    bus.unsubscribe(handle2);
    println!("Subscribers: {}", bus.subscriber_count());

    // 再派发（只有 handle1 收到回调）
    println!("\nSending GameOver...");
    bus.send(GameEvent::GameOver);

    // ===== 队列式 =====
    println!("\n--- Queue mode ---");
    println!("Snapshot: {:?}", bus.snapshot());
    println!("Iter:");
    for event in bus.iter() {
        println!("  [Queue] {:?}", event);
    }

    // 清空队列
    bus.drain_queue();
    println!("\nAfter drain_queue, length: {}", bus.len());

    // ===== 批量发送 =====
    println!("\n--- Batch send ---");
    bus.send_batch(vec![
        GameEvent::PlayerJoined("Bob".to_string()),
        GameEvent::ScoreChanged(200),
    ]);
    println!("After send_batch, length: {}", bus.len());

    // ===== 完全清空 =====
    println!("\n--- Full drain ---");
    bus.drain(); // 清空订阅者 + 队列
    println!("After drain: {} subscribers, {} events", bus.subscriber_count(), bus.len());

    // 丢弃 handle1（已通过 drain 清空）
    let _ = handle1;

    println!("\nEventBus demo completed!");
}
