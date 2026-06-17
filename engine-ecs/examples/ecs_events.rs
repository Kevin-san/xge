//! 事件系统演示
//!
//! 演示如何使用事件进行实体间通信。

use engine_ecs::{Component, Event, World};

/// 碰撞事件
#[derive(Debug, Clone)]
struct CollisionEvent {
    entity1: engine_ecs::Entity,
    entity2: engine_ecs::Entity,
    position: [f32; 2],
}

impl Event for CollisionEvent {}

/// 伤害事件
#[derive(Debug, Clone)]
struct DamageEvent {
    target: engine_ecs::Entity,
    amount: f32,
    source: engine_ecs::Entity,
}

impl Event for DamageEvent {}

/// 死亡事件
#[derive(Debug, Clone)]
struct DeathEvent {
    entity: engine_ecs::Entity,
    killer: Option<engine_ecs::Entity>,
}

impl Event for DeathEvent {}

/// 生命值组件
#[derive(Debug, Clone)]
struct Health(f32);

impl Component for Health {}

/// 位置组件
#[derive(Debug, Clone)]
struct Position([f32; 2]);

impl Component for Position {}

fn main() {
    println!("=== ECS Event Demo ===\n");

    let mut world = World::new();

    // 创建实体
    let player = world.spawn();
    world.insert(player, Position([0.0, 0.0]));
    world.insert(player, Health(100.0));

    let enemy1 = world.spawn();
    world.insert(enemy1, Position([10.0, 0.0]));
    world.insert(enemy1, Health(50.0));

    let enemy2 = world.spawn();
    world.insert(enemy2, Position([20.0, 0.0]));
    world.insert(enemy2, Health(30.0));

    println!(
        "Created entities: player={:?}, enemy1={:?}, enemy2={:?}",
        player, enemy1, enemy2
    );

    // 发送碰撞事件
    println!("\n--- Sending Events ---");
    world.send_event(CollisionEvent {
        entity1: player,
        entity2: enemy1,
        position: [5.0, 0.0],
    });
    println!("Sent CollisionEvent: player hit enemy1 at (5.0, 0.0)");

    world.send_event(CollisionEvent {
        entity1: player,
        entity2: enemy2,
        position: [15.0, 0.0],
    });
    println!("Sent CollisionEvent: player hit enemy2 at (15.0, 0.0)");

    // 发送伤害事件
    world.send_event(DamageEvent {
        target: enemy1,
        amount: 25.0,
        source: player,
    });
    println!("Sent DamageEvent: player dealt 25 damage to enemy1");

    world.send_event(DamageEvent {
        target: enemy2,
        amount: 30.0,
        source: player,
    });
    println!("Sent DamageEvent: player dealt 30 damage to enemy2");

    // 发送死亡事件
    world.send_event(DeathEvent {
        entity: enemy2,
        killer: Some(player),
    });
    println!("Sent DeathEvent: enemy2 killed by player");

    // 处理事件
    println!("\n--- Processing Events ---");

    // 处理碰撞事件
    let reader = world.events::<CollisionEvent>();
    let collisions: Vec<_> = reader.into_iter().collect();
    println!("Received {} collision events:", collisions.len());
    for event in &collisions {
        println!(
            "  {:?} collided with {:?} at ({:.1}, {:.1})",
            event.entity1, event.entity2, event.position[0], event.position[1]
        );
    }

    // 处理伤害事件
    let reader = world.events::<DamageEvent>();
    let damages: Vec<_> = reader.into_iter().collect();
    println!("\nReceived {} damage events:", damages.len());
    for event in &damages {
        // 应用伤害
        if let Some(health) = world.get_component_mut::<Health>(event.target) {
            let old_health = health.0;
            health.0 = (health.0 - event.amount).max(0.0);
            println!(
                "  Applied {:.1} damage to {:?}: {:.1} -> {:.1}",
                event.amount, event.target, old_health, health.0
            );
        }
    }

    // 处理死亡事件
    let reader = world.events::<DeathEvent>();
    let deaths: Vec<_> = reader.into_iter().collect();
    println!("\nReceived {} death events:", deaths.len());
    for event in &deaths {
        println!("  {:?} died", event.entity);
        if let Some(killer) = event.killer {
            println!("    Killed by {:?}", killer);
        }
    }

    // 检查最终状态
    println!("\n--- Final State ---");
    for entity in world.entities_iter() {
        if let Some(health) = world.get_component::<Health>(entity) {
            println!("  {:?} health: {:.1}", entity, health.0);
        }
    }
}
