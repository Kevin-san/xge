//! 自定义系统演示
//!
//! 演示如何创建和运行自定义系统。

use engine_ecs::{Component, FnSystem, Resources, System, World};

/// 位置组件
#[derive(Debug, Clone, Copy)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone, Copy)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 玩家控制组件
#[derive(Debug, Clone)]
struct PlayerControlled;

impl Component for PlayerControlled {}

/// 敌人控制组件
#[derive(Debug, Clone)]
struct EnemyControlled;

impl Component for EnemyControlled {}

fn player_x() -> f32 {
    100.0
}

fn player_y() -> f32 {
    100.0
}

fn main() {
    println!("=== ECS System Demo ===\n");

    let mut world = World::new();

    // 创建玩家实体
    let player = world.spawn();
    world.insert(player, Position([0.0, 0.0]));
    world.insert(player, Velocity([5.0, 3.0]));
    world.insert(player, PlayerControlled);

    // 创建敌人实体
    for i in 0..3 {
        let enemy = world.spawn();
        world.insert(enemy, Position([i as f32 * 50.0, i as f32 * 30.0]));
        world.insert(enemy, Velocity([0.0, 0.0]));
        world.insert(enemy, EnemyControlled);
    }

    println!("Created {} entities\n", world.entity_count());

    // 创建并运行移动系统
    let mut movement_system = FnSystem::new(
        "MovementSystem",
        |world: &mut World, _resources: &mut Resources| {
            let entities: Vec<_> = world.entities_iter().collect();
            for entity in entities {
                let pos = world.get_component::<Position>(entity);
                let vel = world.get_component::<Velocity>(entity);

                if let (Some(pos), Some(vel)) = (pos, vel) {
                    println!(
                        "Entity {:?}: pos=({}, {}), vel=({}, {})",
                        entity, pos.0[0], pos.0[1], vel.0[0], vel.0[1]
                    );
                }
            }
        },
    );

    // 创建并运行玩家移动系统
    let mut player_movement_system = FnSystem::new(
        "PlayerMovementSystem",
        |world: &mut World, _resources: &mut Resources| {
            println!("\n--- Player Movement ---");
            let entities: Vec<_> = world.entities_iter().collect();
            for entity in entities {
                let is_player = world.get_component::<PlayerControlled>(entity).is_some();
                if is_player {
                    let vel = world.get_component::<Velocity>(entity).copied();
                    if let Some(v) = vel {
                        if let Some(pos) = world.get_component_mut::<Position>(entity) {
                            pos.0[0] += v.0[0] * 0.016;
                            pos.0[1] += v.0[1] * 0.016;
                            println!(
                                "Player {:?} moved to: ({:.2}, {:.2})",
                                entity, pos.0[0], pos.0[1]
                            );
                        }
                    }
                }
            }
        },
    );

    // 创建并运行敌人 AI 系统
    let mut enemy_ai_system = FnSystem::new(
        "EnemyAISystem",
        |world: &mut World, _resources: &mut Resources| {
            println!("\n--- Enemy AI ---");
            let entities: Vec<_> = world.entities_iter().collect();
            for entity in entities {
                let is_enemy = world.get_component::<EnemyControlled>(entity).is_some();
                if is_enemy {
                    if let Some(vel) = world.get_component_mut::<Velocity>(entity) {
                        vel.0[0] = (player_x() - entity.id() as f32) * 0.1;
                        vel.0[1] = (player_y() - entity.id() as f32) * 0.1;
                        println!(
                            "Enemy {:?} AI updated velocity: ({:.2}, {:.2})",
                            entity, vel.0[0], vel.0[1]
                        );
                    }
                }
            }
        },
    );

    // 运行所有系统
    let mut resources = Resources::new();
    movement_system.run(&mut world, &mut resources);
    player_movement_system.run(&mut world, &mut resources);
    enemy_ai_system.run(&mut world, &mut resources);

    println!("\n--- Simulation Step Complete ---");
}
