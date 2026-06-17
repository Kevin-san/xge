//! 批量创建实体演示
//!
//! 演示如何高效地批量创建实体。

use engine_ecs::{Component, World};

/// 位置组件
#[derive(Debug, Clone, Copy)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone, Copy)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 标识组件
#[derive(Debug, Clone)]
struct PlayerId(u32);

impl Component for PlayerId {}

fn main() {
    println!("=== ECS Spawn Batch Demo ===\n");

    let mut world = World::new();

    // 批量创建 1000 个玩家实体
    let player_count = 1000;
    println!("Creating {} player entities...", player_count);

    for i in 0..player_count {
        let entity = world.spawn();
        world.insert(entity, Position([i as f32, 0.0]));
        world.insert(entity, Velocity([0.0, 1.0]));
        world.insert(entity, PlayerId(i));
    }

    println!("Created {} entities", world.entity_count());

    // 验证创建
    let mut count = 0;
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        if world.get_component::<PlayerId>(entity).is_some() {
            count += 1;
        }
    }
    println!("Entities with PlayerId: {}", count);

    // 简单的位置更新
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let vel = world.get_component::<Velocity>(entity).copied();
        if let Some(pos) = world.get_component_mut::<Position>(entity) {
            if let Some(v) = vel {
                pos.0[1] += v.0[1] * 0.016;
            }
        }
    }
    println!("Updated positions for all entities");

    // 统计移动过的实体
    let mut moved = 0;
    for entity in world.entities_iter() {
        if let Some(pos) = world.get_component::<Position>(entity) {
            if pos.0[1] > 0.0 {
                moved += 1;
            }
        }
    }
    println!("Entities that moved: {}", moved);
}
