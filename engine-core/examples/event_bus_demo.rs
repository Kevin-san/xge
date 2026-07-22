use engine_core::EventBus;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
struct PlayerEvent {
    player_id: u32,
    event_type: PlayerEventType,
}

#[derive(Debug, Clone, PartialEq)]
enum PlayerEventType {
    Spawn,
    Move { x: f32, y: f32 },
    Damage { amount: u32 },
    Despawn,
}

fn main() {
    println!("=== Event Bus Demo ===\n");

    let bus = EventBus::<PlayerEvent>::new();

    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received_events.clone();

    let _handle1 = bus.subscribe(move |event| {
        let mut events = received_clone.lock().unwrap();
        events.push(event.clone());
        println!("[Subscriber 1] Received: {:?}", event);
    });

    let received_clone2 = received_events.clone();
    let handle2 = bus.subscribe(move |event| {
        let mut events = received_clone2.lock().unwrap();
        events.push(event.clone());
        if let PlayerEventType::Damage { amount } = &event.event_type {
            println!("[Subscriber 2] Player {} took {} damage", event.player_id, amount);
        }
    });

    println!("Sending events...\n");
    bus.send(PlayerEvent {
        player_id: 1,
        event_type: PlayerEventType::Spawn,
    });

    bus.send(PlayerEvent {
        player_id: 1,
        event_type: PlayerEventType::Move { x: 100.0, y: 200.0 },
    });

    bus.send(PlayerEvent {
        player_id: 2,
        event_type: PlayerEventType::Damage { amount: 25 },
    });

    bus.send(PlayerEvent {
        player_id: 1,
        event_type: PlayerEventType::Despawn,
    });

    println!();
    println!("Queue contains {} events", bus.len());

    println!("\nIterating queue (read-only):");
    for event in bus.iter() {
        println!("  {:?}", event);
    }

    println!("\nUnsubscribing subscriber 2...");
    bus.unsubscribe(handle2);

    bus.send(PlayerEvent {
        player_id: 1,
        event_type: PlayerEventType::Spawn,
    });

    println!();
    println!("Subscriber count: {}", bus.subscriber_count());

    println!("\nDraining queue...");
    bus.drain_queue();
    assert!(bus.is_empty());

    println!("\nTotal events received by all subscribers: {}", 
             received_events.lock().unwrap().len());
    println!("\nEvent bus demo completed successfully!");
}