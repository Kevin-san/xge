//! ECS Hello World - 最基础的 ECS 使用示例
//!
//! 演示如何创建实体、添加组件、查询和更新数据。

use engine_ecs::{Component, World};

/// 位置组件
#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {}

fn main() {
    println!("=== ECS Hello World ===");

    // 创建世界
    let mut world = World::new();

    // 创建实体
    let entity = world.spawn();
    println!("Created entity: {:?}", entity);

    // 添加组件
    world.insert(
        entity,
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );
    world.insert(
        entity,
        Velocity {
            x: 1.0,
            y: 2.0,
            z: 0.0,
        },
    );

    // 查询并更新
    for _ in 0..10 {
        // 遍历所有实体并更新位置
        let entities: Vec<_> = world.entities_iter().collect();
        for ent in entities {
            // 先获取 velocity（不可变借用）
            let vel = world.get_component::<Velocity>(ent).copied();
            // 借用在此释放
            // 然后获取 position（可变借用）
            if let Some(pos) = world.get_component_mut::<Position>(ent) {
                if let Some(v) = vel {
                    pos.x += v.x * 0.016;
                    pos.y += v.y * 0.016;
                    pos.z += v.z * 0.016;
                }
            }
        }
    }

    // 打印最终结果
    if let Some(pos) = world.get_component::<Position>(entity) {
        println!("Final position: ({:.2}, {:.2}, {:.2})", pos.x, pos.y, pos.z);
    }

    println!("\nTotal entities: {}", world.entity_count());
}
