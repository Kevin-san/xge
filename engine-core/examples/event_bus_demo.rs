use engine_core::{App, AppBuilder, EngineConfig, EventBus};
use std::sync::{atomic::AtomicUsize, Arc};

#[derive(Clone, Debug, PartialEq)]
struct GameEvent {
    event_type: EventType,
    value: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum EventType {
    PlayerMove,
    EnemySpawn,
    ScoreUpdate,
}

struct EventDemoApp {
    event_bus: EventBus<GameEvent>,
    quit_flag: Arc<std::sync::atomic::AtomicBool>,
    received_count: Arc<AtomicUsize>,
}

impl EventDemoApp {
    fn new(quit_flag: Arc<std::sync::atomic::AtomicBool>) -> Self {
        let event_bus = EventBus::new();
        let received_count = Arc::new(AtomicUsize::new(0));

        let rc = received_count.clone();
        event_bus.subscribe(move |event: &GameEvent| {
            let count = rc.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            println!(
                "[Subscriber 1] Received: {:?} (value={}) [count={}]",
                event.event_type, event.value, count
            );
        });

        let rc2 = received_count.clone();
        event_bus.subscribe(move |event: &GameEvent| {
            let count = rc2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if let EventType::ScoreUpdate = event.event_type {
                println!(
                    "[Subscriber 2] Score updated to {} [count={}]",
                    event.value, count
                );
            }
        });

        Self {
            event_bus,
            quit_flag,
            received_count,
        }
    }
}

impl App for EventDemoApp {
    fn setup(&mut self) {
        println!("[EventDemoApp] Setup complete");
        println!("[EventDemoApp] Subscribed 2 listeners to EventBus\n");
    }

    fn update(&mut self, _dt: f64) {
        println!("\n--- Sending events ---");
        self.event_bus.send(GameEvent {
            event_type: EventType::PlayerMove,
            value: 10,
        });
        self.event_bus.send(GameEvent {
            event_type: EventType::EnemySpawn,
            value: 5,
        });
        self.event_bus.send(GameEvent {
            event_type: EventType::ScoreUpdate,
            value: 100,
        });

        println!("\n--- Queue contains {} events ---", self.event_bus.len());
        let queued: Vec<_> = self.event_bus.iter().collect();
        println!("Queued events: {:?}", queued);
        self.event_bus.drain_queue();

        self.quit_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn shutdown(&mut self) {
        let total = self.received_count.load(std::sync::atomic::Ordering::SeqCst);
        println!("\n[EventDemoApp] Shutdown - Total events received: {}", total);
        assert_eq!(total, 6, "Expected 6 event receptions (3 events × 2 subscribers)");
        println!("EventBus demo completed successfully!");
    }
}

fn main() {
    println!("=== EventBus Demo ===");
    println!("Demonstrates event subscription, sending, and queue iteration\n");

    let quit_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = quit_flag.clone();

    AppBuilder::new()
        .with_config(EngineConfig::default())
        .run_with_quit_flag(EventDemoApp::new(quit_flag), flag);
}