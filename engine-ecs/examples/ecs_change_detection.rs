//! 变更检测演示
//!
//! 演示如何检测组件的变更。

use engine_ecs::{Component, World};

/// 位置组件
#[derive(Debug, Clone, Copy)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone, Copy)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 上次位置（用于比较）
#[derive(Debug, Clone, Copy)]
struct LastPosition([f32; 2]);

impl Component for LastPosition {}

fn main() {
    println!("=== ECS Change Detection Demo ===\n");

    let mut world = World::new();

    let entity1 = world.spawn();
    world.insert(entity1, Position([0.0, 0.0]));
    world.insert(entity1, LastPosition([0.0, 0.0]));
    world.insert(entity1, Velocity([1.0, 1.0]));

    let entity2 = world.spawn();
    world.insert(entity2, Position([10.0, 10.0]));
    world.insert(entity2, LastPosition([10.0, 10.0]));
    world.insert(entity2, Velocity([0.0, 0.0])); // 静止

    println!("Initial state:");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let pos = world.get_component::<Position>(entity).unwrap();
        println!("  {:?}: pos=({:.1}, {:.1})", entity, pos.0[0], pos.0[1]);
    }

    // 模拟一帧
    println!("\n--- Frame Update ---");

    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let pos = world.get_component::<Position>(entity);
        let vel = world.get_component::<Velocity>(entity).copied();
        let last_pos = world.get_component::<LastPosition>(entity);

        if let (Some(pos), Some(v), Some(last_pos)) = (pos, vel, last_pos) {
            // 检测位置是否改变
            if (pos.0[0] - last_pos.0[0]).abs() > 0.001 || (pos.0[1] - last_pos.0[1]).abs() > 0.001
            {
                println!("  {:?}: POSITION CHANGED", entity);
            } else {
                println!("  {:?}: position unchanged", entity);
            }

            // 更新位置（如果速度非零）
            if v.0[0] != 0.0 || v.0[1] != 0.0 {
                if let Some(pos_mut) = world.get_component_mut::<Position>(entity) {
                    pos_mut.0[0] += v.0[0] * 0.016;
                    pos_mut.0[1] += v.0[1] * 0.016;
                }
            }
        }
    }

    // 更新 LastPosition
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let pos = world.get_component::<Position>(entity);
        if let Some(p) = pos {
            world.insert(entity, LastPosition(p.0));
        }
    }

    println!("\nAfter update:");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let pos = world.get_component::<Position>(entity).unwrap();
        let last = world.get_component::<LastPosition>(entity).unwrap();
        println!(
            "  {:?}: pos=({:.3}, {:.3}), last=({:.3}, {:.3})",
            entity, pos.0[0], pos.0[1], last.0[0], last.0[1]
        );
    }

    // 检测哪些实体移动了
    println!("\n--- Change Detection ---");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        let pos = world.get_component::<Position>(entity);
        let last = world.get_component::<LastPosition>(entity);
        let vel = world.get_component::<Velocity>(entity);

        if let (Some(pos), Some(last), Some(vel)) = (pos, last, vel) {
            let moved =
                (pos.0[0] - last.0[0]).abs() > 0.001 || (pos.0[1] - last.0[1]).abs() > 0.001;
            println!(
                "  {:?}: moved={}, velocity=({:.1}, {:.1})",
                entity, moved, vel.0[0], vel.0[1]
            );
        }
    }
}
