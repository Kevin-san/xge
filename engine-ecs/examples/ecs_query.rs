//! 查询系统演示
//!
//! 演示如何使用 Query 进行条件查询。

use engine_ecs::{Component, World};

/// 位置组件
#[derive(Debug, Clone)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 静态标记组件
#[derive(Debug, Clone)]
struct Static;

impl Component for Static {}

/// 动态标记组件
#[derive(Debug, Clone)]
struct Dynamic;

impl Component for Dynamic {}

fn main() {
    println!("=== ECS Query Demo ===\n");

    let mut world = World::new();

    // 创建静态实体（只有位置）
    for i in 0..3 {
        let entity = world.spawn();
        world.insert(entity, Position([i as f32 * 10.0, 0.0]));
        world.insert(entity, Static);
    }

    // 创建动态实体（有位置和速度）
    for i in 0..5 {
        let entity = world.spawn();
        world.insert(entity, Position([i as f32 * 20.0, 100.0]));
        world.insert(entity, Velocity([1.0, 0.5]));
        world.insert(entity, Dynamic);
    }

    // 创建混合实体（有位置，但速度可能被移除）
    let hybrid = world.spawn();
    world.insert(hybrid, Position([50.0, 50.0]));
    world.insert(hybrid, Velocity([2.0, 1.0]));
    world.insert(hybrid, Dynamic);

    println!("Total entities: {}\n", world.entity_count());

    // 查询所有有 Position 的实体
    println!("--- All entities with Position ---");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let pos = world.get_component::<Position>(entity);
        if pos.is_some() {
            let is_static = world.get_component::<Static>(entity).is_some();
            let is_dynamic = world.get_component::<Dynamic>(entity).is_some();
            let has_vel = world.get_component::<Velocity>(entity).is_some();
            println!(
                "  {:?}: static={}, dynamic={}, has_velocity={}",
                entity, is_static, is_dynamic, has_vel
            );
        }
    }

    // 查询有 Position 和 Velocity 的实体
    println!("\n--- Entities with Position AND Velocity ---");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let has_pos = world.get_component::<Position>(entity).is_some();
        let has_vel = world.get_component::<Velocity>(entity).is_some();
        if has_pos && has_vel {
            let pos = world.get_component::<Position>(entity).unwrap();
            let vel = world.get_component::<Velocity>(entity).unwrap();
            println!(
                "  {:?}: pos=({:.1}, {:.1}), vel=({:.1}, {:.1})",
                entity, pos.0[0], pos.0[1], vel.0[0], vel.0[1]
            );
        }
    }

    // 查询有 Position 但没有 Velocity 的实体
    println!("\n--- Entities with Position but NO Velocity ---");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let has_pos = world.get_component::<Position>(entity).is_some();
        let has_vel = world.get_component::<Velocity>(entity).is_some();
        if has_pos && !has_vel {
            let pos = world.get_component::<Position>(entity).unwrap();
            println!("  {:?}: pos=({:.1}, {:.1})", entity, pos.0[0], pos.0[1]);
        }
    }

    // 统计信息
    let entities: Vec<_> = world.entities_iter().collect();
    let total = entities.len();
    let with_pos = entities
        .iter()
        .filter(|e| world.get_component::<Position>(**e).is_some())
        .count();
    let with_vel = entities
        .iter()
        .filter(|e| world.get_component::<Velocity>(**e).is_some())
        .count();
    let with_both = entities
        .iter()
        .filter(|e| {
            world.get_component::<Position>(**e).is_some()
                && world.get_component::<Velocity>(**e).is_some()
        })
        .count();

    println!("\n--- Statistics ---");
    println!("  Total entities: {}", total);
    println!("  With Position: {}", with_pos);
    println!("  With Velocity: {}", with_vel);
    println!("  With both: {}", with_both);
}
