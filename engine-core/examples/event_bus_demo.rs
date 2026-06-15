//! event_bus_demo - 事件总线订阅/派发/取消订阅示例

use engine_core::EventBus;

#[derive(Clone, Debug)]
struct PlayerEvent {
    player_id: u32,
    action: String,
}

fn main() {
    println!("=== EventBus Demo ===\n");
    
    // 创建事件总线
    let bus = EventBus::<PlayerEvent>::new();
    
    println!("Subscribing handlers...");
    
    // 订阅事件
    let handle1 = bus.subscribe(|event| {
        println!("[Subscriber1] Player {} performed {}", 
                 event.player_id, event.action);
    });

    let handle2 = bus.subscribe(|event| {
        println!("[Subscriber2] Received: {:?}", event);
    });
    
    println!("Active subscribers: {}\n", bus.subscriber_count());
    
    // 派发事件
    println!("--- Sending 'jump' event ---");
    bus.send(PlayerEvent {
        player_id: 42,
        action: "jump".into(),
    });
    
    // 取消订阅
    println!("\n--- Unsubscribing handle2 ---");
    bus.unsubscribe(handle2);
    println!("Active subscribers: {}\n", bus.subscriber_count());
    
    // 再次派发（只有 handle1 收到）
    println!("--- Sending 'land' event ---");
    bus.send(PlayerEvent {
        player_id: 42,
        action: "land".into(),
    });
    
    // drain 示例
    println!("\n--- Drain demo ---");
    let bus2 = EventBus::<PlayerEvent>::new();
    bus2.send(PlayerEvent { player_id: 1, action: "run".into() });
    bus2.send(PlayerEvent { player_id: 2, action: "walk".into() });
    
    println!("Before drain: {} subscribers", bus2.subscriber_count());
    bus2.drain();
    println!("After drain: {} subscribers", bus2.subscriber_count());
    
    println!("\nEventBus demo completed!");
}
