//! Bundle 批量插入演示
//!
//! 演示如何使用 Bundle 一次性添加多个组件。

use engine_ecs::{Bundle, Component, World};

/// 位置组件
#[derive(Debug, Clone)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 质量组件
#[derive(Debug, Clone)]
struct Mass(f32);

impl Component for Mass {}

/// 旋转组件
#[derive(Debug, Clone)]
struct Rotation(f32);

impl Component for Rotation {}

fn main() {
    println!("=== ECS Bundle Demo ===\n");

    let mut world = World::new();

    // 使用 spawn_bundle 一次性创建并插入（使用元组 Bundle）
    let entity1 = world.spawn_bundle((Position([0.0, 0.0]), Velocity([1.0, 2.0]), Mass(1.0)));
    println!("Created physics entity: {:?}", entity1);

    // 使用 spawn_bundle 创建完整实体
    let entity2 = world.spawn_bundle((
        Position([10.0, 20.0]),
        Velocity([0.5, 1.0]),
        Mass(2.0),
        Rotation(45.0),
    ));
    println!("Created full entity: {:?}", entity2);

    // 批量创建
    println!("\nSpawning batch...");
    world.spawn_batch((0..5).map(|i| {
        (
            Position([i as f32 * 10.0, 0.0]),
            Velocity([0.0, i as f32]),
            Mass(1.0),
        )
    }));
    println!("Total entities: {}", world.entity_count());

    // 验证组件
    println!("\nVerifying components:");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        if let Some(pos) = world.get_component::<Position>(entity) {
            print!("  pos=({:.1}, {:.1})", pos.0[0], pos.0[1]);
        } else {
            print!("  pos=false");
        }
        if let Some(vel) = world.get_component::<Velocity>(entity) {
            print!("  vel=({:.1}, {:.1})", vel.0[0], vel.0[1]);
        } else {
            print!("  vel=false");
        }
        if let Some(mass) = world.get_component::<Mass>(entity) {
            print!("  mass={:.1}", mass.0);
        } else {
            print!("  mass=false");
        }
        if let Some(rot) = world.get_component::<Rotation>(entity) {
            print!("  rot={:.1}", rot.0);
        } else {
            print!("  rot=false");
        }
        println!();
    }

    // 测试移除 Bundle
    println!("\n--- Testing Bundle Remove ---");
    let test_entity = world.spawn_bundle((Position([5.0, 5.0]), Velocity([1.0, 1.0]), Mass(3.0)));
    println!("Created test entity: {:?}", test_entity);

    // 使用 tuple bundle 移除组件
    let remove_bundle = (Position([0.0, 0.0]), Velocity([0.0, 0.0]));
    remove_bundle.remove(&mut world, test_entity);

    let has_pos = world.get_component::<Position>(test_entity).is_some();
    let has_vel = world.get_component::<Velocity>(test_entity).is_some();
    let has_mass = world.get_component::<Mass>(test_entity).is_some();
    println!(
        "After removing Position and Velocity: pos={}, vel={}, mass={}",
        has_pos, has_vel, has_mass
    );
}
