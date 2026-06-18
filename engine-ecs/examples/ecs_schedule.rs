//! 调度器演示
//!
//! 演示如何使用调度器组织系统的执行顺序。

use engine_ecs::{Component, Resources, System, World};

/// 位置组件
#[derive(Debug, Clone, Copy)]
struct Position([f32; 2]);

impl Component for Position {}

/// 速度组件
#[derive(Debug, Clone, Copy)]
struct Velocity([f32; 2]);

impl Component for Velocity {}

/// 玩家输入组件
#[derive(Debug, Clone, Copy)]
struct PlayerInput {
    move_x: f32,
    move_y: f32,
}

impl Component for PlayerInput {}

/// 输入处理系统
struct InputSystem;

impl System for InputSystem {
    fn name(&self) -> &str {
        "InputSystem"
    }

    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        println!("[{}] Processing input...", self.name());
        let entities: Vec<_> = world.entities_iter().collect();
        for entity in entities {
            let input = world.get_component::<PlayerInput>(entity).copied();
            if let Some(inp) = input {
                if let Some(vel) = world.get_component_mut::<Velocity>(entity) {
                    vel.0[0] = inp.move_x;
                    vel.0[1] = inp.move_y;
                    println!(
                        "  Set velocity for {:?}: ({:.1}, {:.1})",
                        entity, vel.0[0], vel.0[1]
                    );
                }
            }
        }
    }

    fn read(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
    fn write(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
}

/// 物理更新系统
struct PhysicsSystem;

impl System for PhysicsSystem {
    fn name(&self) -> &str {
        "PhysicsSystem"
    }

    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        println!("[{}] Updating physics...", self.name());
        let entities: Vec<_> = world.entities_iter().collect();
        for entity in entities {
            let vel = world.get_component::<Velocity>(entity).copied();
            if let Some(v) = vel {
                if let Some(pos) = world.get_component_mut::<Position>(entity) {
                    let old_x = pos.0[0];
                    let old_y = pos.0[1];
                    pos.0[0] += v.0[0] * 0.016;
                    pos.0[1] += v.0[1] * 0.016;
                    if v.0[0] != 0.0 || v.0[1] != 0.0 {
                        println!(
                            "  Moved {:?}: ({:.2}, {:.2}) -> ({:.2}, {:.2})",
                            entity, old_x, old_y, pos.0[0], pos.0[1]
                        );
                    }
                }
            }
        }
    }

    fn read(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
    fn write(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
}

/// 渲染系统
struct RenderSystem;

impl System for RenderSystem {
    fn name(&self) -> &str {
        "RenderSystem"
    }

    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        println!("[{}] Rendering...", self.name());
        let entities: Vec<_> = world.entities_iter().collect();
        for entity in entities {
            if let Some(pos) = world.get_component::<Position>(entity) {
                println!(
                    "  Render {:?} at ({:.2}, {:.2})",
                    entity, pos.0[0], pos.0[1]
                );
            }
        }
    }

    fn read(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
    fn write(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
}

/// 调试系统
struct DebugSystem;

impl System for DebugSystem {
    fn name(&self) -> &str {
        "DebugSystem"
    }

    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        println!("[{}] Debug info:", self.name());
        println!("  Total entities: {}", world.entity_count());
    }

    fn read(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
    fn write(&self) -> Vec<std::any::TypeId> {
        Vec::new()
    }
}

fn main() {
    println!("=== ECS Schedule Demo ===\n");

    let mut world = World::new();
    let mut resources = Resources::new();

    // 创建玩家实体
    let player = world.spawn();
    world.insert(player, Position([0.0, 0.0]));
    world.insert(player, Velocity([0.0, 0.0]));
    world.insert(
        player,
        PlayerInput {
            move_x: 5.0,
            move_y: 3.0,
        },
    );

    // 创建静态实体
    let _static_entity = world.spawn();
    world.insert(_static_entity, Position([100.0, 100.0]));

    println!("Created {} entities\n", world.entity_count());

    // 创建调度器并添加阶段
    let mut schedule = engine_ecs::Schedule::new();
    schedule
        .add_stage_to_schedule("input")
        .add_system(InputSystem);
    schedule
        .add_stage_to_schedule("physics")
        .add_system(PhysicsSystem);
    schedule
        .add_stage_to_schedule("render")
        .add_system(RenderSystem);
    schedule
        .add_stage_to_schedule("debug")
        .add_system(DebugSystem);

    // 运行多帧模拟
    for frame in 0..3 {
        println!("\n========== Frame {} ==========", frame);
        schedule.run(&mut world, &mut resources);
    }

    println!("\n========== Simulation Complete ==========");
    println!("Final positions:");
    let entities: Vec<_> = world.entities_iter().collect();
    for entity in entities {
        if let Some(pos) = world.get_component::<Position>(entity) {
            println!("  {:?}: ({:.2}, {:.2})", entity, pos.0[0], pos.0[1]);
        }
    }
}
