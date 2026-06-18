//! 性能测试演示
//!
//! 大批量实体操作的性能基准测试。

use engine_ecs::{Component, World};

/// 位置组件
#[derive(Debug, Clone, Copy)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone, Copy)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 质量组件
#[derive(Debug, Clone)]
struct Mass(#[allow(dead_code)] f32);

impl Component for Mass {}

/// 标识组件
#[derive(Debug, Clone)]
struct Id(#[allow(dead_code)] u32);

impl Component for Id {}

fn main() {
    println!("=== ECS Performance Test ===\n");

    // 测试1：创建性能
    println!("--- Test 1: Entity Creation ---");
    let creation_counts = [100, 1000, 5000, 10000];

    for count in creation_counts {
        let start = std::time::Instant::now();
        let mut world = World::new();

        for i in 0..count {
            let entity = world.spawn();
            world.insert(entity, Position([i as f32, i as f32]));
            world.insert(entity, Velocity([1.0, 1.0]));
            world.insert(entity, Mass(1.0));
            world.insert(entity, Id(i));
        }

        let elapsed = start.elapsed();
        println!(
            "  Created {} entities in {:.3}ms ({:.0} entities/sec)",
            count,
            elapsed.as_secs_f32() * 1000.0,
            count as f32 / elapsed.as_secs_f32()
        );
    }

    // 测试2：组件查询性能
    println!("\n--- Test 2: Component Query ---");

    let mut world = World::new();
    let entity_count = 10000;

    for i in 0..entity_count {
        let entity = world.spawn();
        world.insert(entity, Position([i as f32, i as f32]));
        world.insert(entity, Velocity([1.0, 1.0]));
        world.insert(entity, Mass(1.0));
    }

    let start = std::time::Instant::now();
    let iterations = 100;

    for _ in 0..iterations {
        let entities: Vec<_> = world.entities_iter().collect();
        for entity in &entities {
            let _ = world.get_component::<Position>(*entity);
            let _ = world.get_component::<Velocity>(*entity);
        }
    }

    let elapsed = start.elapsed();
    let total_ops = entity_count * iterations * 2; // 每个实体2个组件查询
    println!(
        "  {} iterations x {} entities: {:.3}ms",
        iterations,
        entity_count,
        elapsed.as_secs_f32() * 1000.0
    );
    println!("  Total component accesses: {}", total_ops);
    println!(
        "  Accesses per second: {:.0}",
        total_ops as f32 / elapsed.as_secs_f32()
    );

    // 测试3：组件更新性能
    println!("\n--- Test 3: Component Update ---");

    let start = std::time::Instant::now();
    let iterations = 100;

    for _ in 0..iterations {
        let entities: Vec<_> = world.entities_iter().collect();
        for entity in entities {
            let vel = world.get_component::<Velocity>(entity).copied();
            if let Some(pos) = world.get_component_mut::<Position>(entity) {
                if let Some(v) = vel {
                    pos.0[0] += v.0[0] * 0.016;
                    pos.0[1] += v.0[1] * 0.016;
                }
            }
        }
    }

    let elapsed = start.elapsed();
    let total_updates = entity_count * iterations;
    println!(
        "  {} iterations x {} entities: {:.3}ms",
        iterations,
        entity_count,
        elapsed.as_secs_f32() * 1000.0
    );
    println!(
        "  Updates per second: {:.0}",
        total_updates as f32 / elapsed.as_secs_f32()
    );

    // 测试4：条件查询性能
    println!("\n--- Test 4: Conditional Query ---");

    let start = std::time::Instant::now();
    let iterations = 100;

    for _ in 0..iterations {
        let entities: Vec<_> = world.entities_iter().collect();
        for entity in entities {
            let has_pos = world.get_component::<Position>(entity).is_some();
            let has_vel = world.get_component::<Velocity>(entity).is_some();
            if has_pos && has_vel {
                // 满足条件的实体
            }
        }
    }

    let elapsed = start.elapsed();
    let total_checks = entity_count * iterations;
    println!(
        "  {} iterations x {} entities: {:.3}ms",
        iterations,
        entity_count,
        elapsed.as_secs_f32() * 1000.0
    );
    println!(
        "  Checks per second: {:.0}",
        total_checks as f32 / elapsed.as_secs_f32()
    );

    // 测试5：批量操作
    println!("\n--- Test 5: Batch Operations ---");

    let mut world = World::new();

    let start = std::time::Instant::now();
    let batch_count = 1000;
    let batch_size = 100;

    for _ in 0..batch_count {
        for i in 0..batch_size {
            let entity = world.spawn();
            world.insert(entity, Position([i as f32, 0.0]));
            world.insert(entity, Velocity([0.0, 1.0]));
        }
    }

    let total_created = batch_count * batch_size;
    let elapsed = start.elapsed();
    println!(
        "  Created {} entities in {:.3}ms",
        total_created,
        elapsed.as_secs_f32() * 1000.0
    );
    println!(
        "  Creation rate: {:.0} entities/sec",
        total_created as f32 / elapsed.as_secs_f32()
    );

    println!("\n=== Performance Test Complete ===");
}
