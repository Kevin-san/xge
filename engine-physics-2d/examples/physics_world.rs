//! physics_world.rs - 物理世界基础演示
//!
//! 本示例演示 PhysicsWorld2D 的基本用法，包括创建世界、添加刚体、模拟重力下落。

use engine_math::Vec2;
use engine_physics_2d::world::PhysicsWorldConfig;
use engine_physics_2d::{PhysicsWorld2D, RigidBody2DBuilder};

fn main() {
    println!("=== Physics World Demo ===");
    println!();

    // 1. 创建物理世界（使用默认配置）
    println!("1. Creating physics world with default config...");
    let world = PhysicsWorld2D::with_default_config();
    println!("   - Gravity: {:?}", world.gravity());
    println!("   - Body count: {}", world.body_count());
    println!();

    // 2. 创建自定义配置的物理世界
    println!("2. Creating physics world with custom config...");
    let config = PhysicsWorldConfig {
        gravity: Vec2::new(0.0, -20.0), // 更强的重力
        timestep: 1.0 / 120.0,          // 更精细的时间步长
        max_substeps: 8,
        velocity_iterations: 16,
        position_iterations: 4,
        default_restitution: 0.5,
        default_friction: 0.3,
    };
    let mut world = PhysicsWorld2D::new(config);
    println!("   - Gravity: {:?}", world.gravity());
    println!("   - Timestep: {:?}", 1.0 / 120.0);
    println!();

    // 3. 创建动态刚体
    println!("3. Creating dynamic rigid bodies...");
    let body1 = RigidBody2DBuilder::dynamic()
        .with_mass(1.0)
        .with_position(Vec2::new(0.0, 10.0))
        .with_velocity(Vec2::new(0.0, 0.0))
        .build();
    let body1_index = world.add_body(body1);
    println!("   - Added dynamic body at index: {}", body1_index);

    let body2 = RigidBody2DBuilder::dynamic()
        .with_mass(2.0)
        .with_position(Vec2::new(1.0, 15.0))
        .build();
    let body2_index = world.add_body(body2);
    println!("   - Added dynamic body at index: {}", body2_index);
    println!("   - Total bodies: {}", world.body_count());
    println!();

    // 4. 创建静态刚体（地面）
    println!("4. Creating static rigid body (ground)...");
    let ground = RigidBody2DBuilder::static_()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let ground_index = world.add_body(ground);
    println!("   - Added static body at index: {}", ground_index);
    println!("   - Total bodies: {}", world.body_count());
    println!();

    // 5. 模拟重力下落
    println!("5. Simulating gravity (stepping world)...");
    println!(
        "   {:^6} | {:^15} | {:^15} | {:^15}",
        "Step", "Body1 Pos", "Body2 Pos", "Sim Time"
    );
    println!("   {:->6} | {:->15} | {:->15} | {:->15}", "", "", "", "");

    for step in 0..10 {
        world.step(1.0 / 60.0);

        if let Some(body1_state) = world.get_body(body1_index) {
            let pos1 = body1_state.position();
            if step % 2 == 0 {
                println!("   {:6} | ({:6.2}, {:6.2}) |", step, pos1.x, pos1.y);
            }
        }

        if let Some(body2_state) = world.get_body(body2_index) {
            let pos2 = body2_state.position();
            if step % 2 == 0 {
                println!(
                    "{:6}   |           | ({:6.2}, {:6.2}) |",
                    "", pos2.x, pos2.y
                );
            }
        }
    }
    println!("   Simulation time: {:.4}s", world.simulation_time());
    println!();

    // 6. 测试重力修改
    println!("6. Testing gravity modification...");
    world.set_gravity(Vec2::new(0.0, -5.0));
    println!("   Changed gravity to: {:?}", world.gravity());
    println!();

    // 7. 测试启用/禁用仿真
    println!("7. Testing simulation enable/disable...");
    world.set_simulation(false);
    println!("   Simulation disabled");
    let time_before = world.simulation_time();
    world.step(1.0);
    let time_after = world.simulation_time();
    println!(
        "   Time before: {}, Time after: {} (should be same)",
        time_before, time_after
    );

    world.set_simulation(true);
    println!("   Simulation enabled");
    println!();

    // 8. 清理世界
    println!("8. Clearing world...");
    world.clear();
    println!("   - Body count after clear: {}", world.body_count());
    println!(
        "   - Collider count after clear: {}",
        world.collider_count()
    );
    println!();

    println!("Physics world demo completed successfully!");
}
