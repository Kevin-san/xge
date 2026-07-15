//! SparseSet 存储演示
//!
//! 演示稀疏集存储的特性和使用场景。

use engine_ecs::{Component, World};

/// 位置组件
#[derive(Debug, Clone)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件（频繁添加/删除）
#[derive(Debug, Clone)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 标识组件
#[derive(Debug, Clone)]
struct Identifier(#[allow(dead_code)] u32);

impl Component for Identifier {}

fn main() {
    println!("=== ECS SparseSet Demo ===\n");

    let mut world = World::new();

    // 创建大量实体
    let count = 10000;
    println!("Creating {} entities...", count);

    for i in 0..count {
        let entity = world.spawn();
        world.insert(entity, Position([i as f32, i as f32]));
        world.insert(entity, Identifier(i));

        // 只有一半的实体有速度
        if i % 2 == 0 {
            world.insert(entity, Velocity([1.0, 1.0]));
        }
    }

    println!("Created {} entities", world.entity_count());

    // 计算有速度的实体数量
    let mut with_velocity = 0;
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in &entities {
        if world.get_component::<Velocity>(*entity).is_some() {
            with_velocity += 1;
        }
    }
    println!("Entities with Velocity: {}", with_velocity);

    // 演示稀疏集的高效随机访问
    println!("\n--- Sparse Access Demo ---");

    // 随机访问一些实体
    let test_indices = [0, 100, 1000, 5000, 9999];
    for idx in test_indices {
        // 找到第 idx 个实体
        let target_entity =
            entities
                .iter()
                .enumerate()
                .find_map(|(i, e)| if i == idx { Some(*e) } else { None });

        if let Some(entity) = target_entity {
            let pos = world.get_component::<Position>(entity);
            let vel = world.get_component::<Velocity>(entity);
            println!(
                "  Entity[{}] {:?}: pos={:?}, vel={:?}",
                idx,
                entity,
                pos.map(|p| (p.0[0], p.0[1])),
                vel.map(|v| (v.0[0], v.0[1]))
            );
        }
    }

    // 频繁添加/删除速度组件的场景
    println!("\n--- Dynamic Component Toggle ---");

    let test_entity = entities[0];
    println!("Test entity: {:?}", test_entity);

    // 检查初始状态
    let has_vel = world.get_component::<Velocity>(test_entity).is_some();
    println!("  Initial: has_velocity={}", has_vel);

    // 移除速度
    world.remove::<Velocity>(test_entity);
    let has_vel = world.get_component::<Velocity>(test_entity).is_some();
    println!("  After remove: has_velocity={}", has_vel);

    // 重新添加速度
    world.insert(test_entity, Velocity([5.0, 5.0]));
    let has_vel = world.get_component::<Velocity>(test_entity).is_some();
    let vel = world.get_component::<Velocity>(test_entity);
    println!(
        "  After re-add: has_velocity={}, velocity={:?}",
        has_vel,
        vel.map(|v| (v.0[0], v.0[1]))
    );

    // 性能测试
    println!("\n--- Performance Test ---");
    let start = std::time::Instant::now();

    // 1000 次随机查询
    for _ in 0..1000 {
        let idx = ((rand_simple() * count as f32) as usize) % (count as usize);
        let target_entity =
            entities
                .iter()
                .enumerate()
                .find_map(|(i, e)| if i == idx { Some(*e) } else { None });
        if let Some(entity) = target_entity {
            let _ = world.get_component::<Position>(entity);
            let _ = world.get_component::<Velocity>(entity);
        }
    }

    let elapsed = start.elapsed();
    println!(
        "  1000 random queries took: {:.3}ms",
        elapsed.as_secs_f32() * 1000.0
    );
}

// 简单的随机数生成器（不依赖外部库）
fn rand_simple() -> f32 {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    ((seed as f32 * 9301.0 + 49297.0) % 233280.0) / 233280.0
}
