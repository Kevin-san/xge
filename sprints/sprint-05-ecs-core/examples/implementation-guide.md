# 示例实现指南

## 模块概述

本文档提供 `engine-ecs` crate 中所有示例的详细实现指南，包括代码结构、核心模式和使用说明。

---

## 需求编号：151-165, 320-336

### 1. examples/ecs_hello — 最小 ECS

**需求ID**: 151, 261

**目标**: 演示最基本的 ECS 使用：spawn + query + update

**代码结构**:
```rust
use engine_ecs::{World, Component, Query, System};

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32 }

fn main() {
    let mut world = World::new();
    
    // Spawn 10 个实体
    for i in 0..10 {
        world.spawn_bundle((Position { x: i as f32, y: i as f32 }, Velocity { x: 1.0, y: 1.0 }));
    }
    
    // 创建系统
    fn update_positions(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut pos, vel) in query.iter_mut() {
            pos.x += vel.x;
            pos.y += vel.y;
            println!("Entity at ({}, {})", pos.x, pos.y);
        }
    }
    
    // 运行系统
    world.run_system(update_positions);
}
```

**验收标准**:
- 成功 spawn 10 个实体
- 每个实体的 Position 被 Velocity 更新
- 打印所有实体位置

---

### 2. examples/ecs_100k — 10 万粒子

**需求ID**: 124, 153, 262, 322, 332

**目标**: 10 万粒子移动 + 位置/速度更新 + 绘制，稳定 >= 60fps

**代码结构**:
```rust
use engine_ecs::{World, Component, Query, Bundle};
use rand::Rng;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32, }

#[derive(Component)]
struct Color { r: f32, g: f32, b: f32 }

#[derive(Bundle)]
struct Particle {
    pos: Position,
    vel: Velocity,
    color: Color,
}

fn main() {
    let mut world = World::new();
    
    // 批量 spawn 100k 粒子
    let particles: Vec<_> = (0..100_000)
        .map(|_| {
            let mut rng = rand::thread_rng();
            Particle {
                pos: Position { x: rng.gen(), y: rng.gen() },
                vel: Velocity { x: rng.gen::<f32>() * 2.0 - 1.0, y: rng.gen::<f32>() * 2.0 - 1.0 },
                color: Color { r: rng.gen(), g: rng.gen(), b: rng.gen() },
            }
        })
        .collect();
    
    world.spawn_batch(particles.into_iter());
    
    // 移动系统
    fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut pos, vel) in query.iter_mut() {
            pos.x = (pos.x + vel.x).rem_euclid(1.0);
            pos.y = (pos.y + vel.y).rem_euclid(1.0);
        }
    }
    
    // 绘制系统（并行）
    fn draw_system(query: Query<(&Position, &Color)>) {
        // 渲染到屏幕
    }
    
    // 运行
    loop {
        world.run_system(move_system);
        world.run_system(draw_system);
        // 控制帧率 60fps
    }
}
```

**验收标准**:
- `cargo run --example ecs_100k` 成功运行
- 10 万粒子稳定 >= 60fps
- 性能 benchmark 通过

**Benchmark 指标**:
- `ecs_query_iter_100k`: Query::iter 100k 实体
- `ecs_query_par_100k`: Query::par_for_each 100k 实体
- `ecs_spawn_100k`: spawn_batch 100k 实体
- `ecs_insert_bundle`: insert_bundle 性能

---

### 3. examples/ecs_events — 事件读写

**需求ID**: 125, 263

**目标**: 键盘事件触发得分变化

**代码结构**:
```rust
use engine_ecs::{World, Component, Event, EventReader, EventWriter, System};

#[derive(Component)]
struct Score { value: u32 }

#[derive(Event)]
struct KeyboardEvent { key: String, pressed: bool }

fn main() {
    let mut world = World::new();
    world.insert_resource(Score { value: 0 });
    
    // 事件写入系统
    fn handle_input(mut ev: EventWriter<KeyboardEvent>) {
        // 模拟键盘输入
        ev.send(KeyboardEvent { key: "Space".to_string(), pressed: true });
    }
    
    // 事件读取系统
    fn update_score(mut score: ResMut<Score>, mut ev: EventReader<KeyboardEvent>) {
        for event in ev.iter() {
            if event.key == "Space" && event.pressed {
                score.value += 10;
                println!("Score: {}", score.value);
            }
        }
    }
    
    world.run_system(handle_input);
    world.run_system(update_score);
}
```

**验收标准**:
- 事件正确发送和接收
- 多 reader 独立读取

---

### 4. examples/ecs_hierarchy — 父子层级与 Transform 传播

**需求ID**: 126, 153, 264, 290

**目标**: 父子 Transform 传播

**代码结构**:
```rust
use engine_ecs::{World, Component, Parent, Children, BuildChildren, Transform2D, GlobalTransform2D};

#[derive(Component)]
struct Transform2D { translation: Vec2, rotation: f32, scale: Vec2 }

#[derive(Component)]
struct GlobalTransform2D(Mat3);

fn main() {
    let mut world = World::new();
    
    // 创建父实体
    let parent = world.spawn_bundle((Transform2D { translation: Vec2::ZERO, rotation: 0.0, scale: Vec2::ONE },));
    
    // 创建子实体
    let child = world.spawn_bundle((Transform2D { translation: Vec2::new(10.0, 0.0), rotation: 0.0, scale: Vec2::ONE },));
    
    // 建立父子关系
    world.push_child(parent, child);
    
    // 变换传播系统（应在 PreUpdate 运行）
    fn transform_propagate_system(mut query: Query<(Entity, &Transform2D, &mut GlobalTransform2D)>) {
        // 计算 GlobalTransform
    }
    
    world.add_system_to_stage(PreUpdate, transform_propagate_system);
    world.schedule("default").run(&mut world);
}
```

**验收标准**:
- 子实体 GlobalTransform 正确包含父变换
- `cargo run --example ecs_hierarchy` 正常运行

---

### 5. examples/ecs_parallel — 并行系统对比单线程

**需求ID**: 127, 265, 323

**目标**: 并行系统比单线程快 >= 1.5x

**代码结构**:
```rust
use engine_ecs::{World, Component, Query, SystemStage};

#[derive(Component)]
struct Data { value: f32 }

fn main() {
    let mut world = World::new();
    
    // 插入大量实体
    for _ in 0..1_000_000 {
        world.spawn_bundle((Data { value: 0.0 },));
    }
    
    // 单线程阶段
    let single_threaded = SystemStage::single_threaded();
    
    // 并行阶段
    let parallel = SystemStage::parallel();
    
    // 对比运行时间
}
```

**验收标准**:
- 并行执行比单线程快 >= 1.5x
- 无 data race

---

### 6. examples/ecs_commands — Commands 延迟命令

**需求ID**: 128, 266

**目标**: Commands 延迟命令演示

**代码结构**:
```rust
use engine_ecs::{World, Component, Commands, Bundle};

#[derive(Component)]
struct Position { x: f32, y: f32 }

fn main() {
    let mut world = World::new();
    let mut commands = Commands::default();
    
    // 记录命令
    let entity = commands.spawn((Position { x: 0.0, y: 0.0 },));
    commands.insert_resource(42u32);
    commands.despawn(entity);
    
    // 应用到 world
    commands.apply(&mut world);
    
    // 此时 entity 已被销毁
}
```

**验收标准**:
- Commands 正确记录命令
- apply 后命令生效

---

### 7. examples/ecs_change_tracking — Changed/Added 过滤

**需求ID**: 129, 267

**目标**: Changed/Added 过滤演示

**代码结构**:
```rust
use engine_ecs::{World, Component, Query, Added, Changed};

#[derive(Component)]
struct Health { value: f32 }

fn main() {
    let mut world = World::new();
    let entity = world.spawn_bundle((Health { value: 100.0 },));
    
    // 系统 1: 添加 Health (Added)
    fn heal_system(mut query: Query<&mut Health, Added<Health>>) {
        for mut health in query.iter_mut() {
            println!("New entity with health: {}", health.value);
        }
    }
    
    // 系统 2: 检测 Health 变化 (Changed)
    fn damage_system(mut query: Query<&mut Health, Changed<Health>>) {
        for mut health in query.iter_mut() {
            if health.value < 50.0 {
                println!("Low health!");
            }
        }
    }
    
    world.run_system(heal_system);
    world.run_system(damage_system);
}
```

**验收标准**:
- Added 仅匹配本帧新增
- Changed 仅匹配本帧变化

---

### 8. examples/ecs_resources — 资源单件

**需求ID**: 130, 268

**目标**: 资源单件演示

**代码结构**:
```rust
use engine_ecs::{World, Resource, Res, ResMut};

#[derive(Resource)]
struct GameTime { elapsed: f32 }

#[derive(Resource)]
struct Config { difficulty: u32 }

fn main() {
    let mut world = World::new();
    world.insert_resource(GameTime { elapsed: 0.0 });
    world.insert_resource(Config { difficulty: 1 });
    
    fn update_time(mut time: ResMut<GameTime>) {
        time.elapsed += 1.0;
    }
    
    fn check_config(config: Res<Config>) {
        println!("Difficulty: {}", config.difficulty);
    }
    
    world.run_system(update_time);
    world.run_system(check_config);
}
```

**验收标准**:
- 资源正确插入和获取
- Res/ResMut 正常工作

---

### 9. examples/ecs_bundle — Bundle 简化 spawn

**需求ID**: 131, 269

**目标**: Bundle 简化 spawn

**代码结构**:
```rust
use engine_ecs::{World, Component, Bundle};

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32 }

#[derive(Bundle)]
struct MovingBundle {
    pos: Position,
    vel: Velocity,
}

fn main() {
    let mut world = World::new();
    
    // 使用 Bundle 一次 spawn
    let entity = world.spawn_bundle(MovingBundle {
        pos: Position { x: 0.0, y: 0.0 },
        vel: Velocity { x: 1.0, y: 1.0 },
    });
    
    // Bundle 也可以用于 insert
    world.insert_bundle(entity, MovingBundle {
        pos: Position { x: 10.0, y: 10.0 },
        vel: Velocity { x: 0.0, y: 0.0 },
    });
}
```

**验收标准**:
- Bundle 正确派生
- 批量 spawn 正常工作

---

### 10. examples/ecs_schedule — 多阶段 Schedule

**需求ID**: 132, 270, 285

**目标**: 多阶段 Schedule

**代码结构**:
```rust
use engine_ecs::{World, Schedule, SystemStage, StageLabel};

#[derive(StageLabel)]
enum MyStage {
    PreUpdate,
    Update,
    PostUpdate,
}

fn main() {
    let mut world = World::new();
    let mut schedule = Schedule::new();
    
    schedule.add_stage(MyStage::PreUpdate, SystemStage::single_threaded());
    schedule.add_stage(MyStage::Update, SystemStage::parallel());
    schedule.add_stage(MyStage::PostUpdate, SystemStage::single_threaded());
    
    // 添加系统到阶段
    world.add_system_to_stage(MyStage::PreUpdate, pre_system);
    world.add_system_to_stage(MyStage::Update, update_system);
    world.add_system_to_stage(MyStage::PostUpdate, post_system);
    
    // 运行
    schedule.run(&mut world);
}
```

**验收标准**:
- 阶段按顺序执行
- `cargo run --example ecs_schedule` 正常运行

---

### 11. examples/ecs_ray_cast — ECS + 物理 raycast

**需求ID**: 133, 271

**目标**: 射线检测 ECS

**代码结构**:
```rust
use engine_ecs::{World, Component, Query, Entity};

#[derive(Component)]
struct Transform2D { translation: Vec2, rotation: f32 }

#[derive(Component)]
struct Collider { radius: f32 }

fn ray_cast(origin: Vec2, direction: Vec2, world: &World) -> Option<Entity> {
    let query = world.query::<(Entity, &Transform2D, &Collider)>();
    
    for (entity, transform, collider) in query.iter() {
        // 简单的射线-圆碰撞检测
        if ray_circle_intersect(origin, direction, transform.translation, collider.radius) {
            return Some(entity);
        }
    }
    None
}
```

**验收标准**:
- `cargo run --example ecs_ray_cast` 正常运行
- 射线检测正确

---

### 12. examples/ecs_pong — ECS 简化 pong

**需求ID**: 134, 272, 370

**目标**: 简化 pong 游戏

**代码结构**:
```rust
use engine_ecs::{World, Component, Bundle};

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32 }

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component, Clone)]
struct Player { score: u32 }

fn main() {
    let mut world = World::new();
    
    // Spawn 球和挡板
    world.spawn_bundle((Ball, Position { x: 0.5, y: 0.5 }, Velocity { x: 0.01, y: 0.01 }));
    world.spawn_bundle((Paddle, Player { score: 0 }, Position { x: 0.1, y: 0.5 }));
    world.spawn_bundle((Paddle, Player { score: 0 }, Position { x: 0.9, y: 0.5 }));
    
    // 游戏系统
    fn ball_movement_system(mut query: Query<(&mut Position, &Velocity), With<Ball>>) { ... }
    fn paddle_system(query: Query<&Position, With<Paddle>>) { ... }
    fn collision_system(world: &mut World) { ... }
    
    world.add_system_to_stage(Update, ball_movement_system);
    world.add_system_to_stage(Update, paddle_system);
    world.add_system_to_stage(Update, collision_system);
    
    // 运行游戏循环
    loop {
        world.schedule("default").run(&mut world);
        // 渲染
    }
}
```

**验收标准**:
- `cargo run --example ecs_pong` 可玩
- 球和挡板正常移动
- 碰撞检测工作

---

## Criterion Benchmark 指南

**需求ID**: 135, 273-277, 336

### 注册 Benchmark

```rust
use criterion::{black_box, criterion_group, Criterion};

fn ecs_query_iter_100k(c: &mut Criterion) {
    let mut world = World::new();
    // spawn 100k 实体
    
    c.bench_function("ecs_query_iter_100k", |b| {
        b.iter(|| {
            let mut query = world.query::<(&Position, &Velocity)>();
            black_box(query.iter().count());
        });
    });
}

fn ecs_query_par_100k(c: &mut Criterion) {
    // ... 类似
}

fn ecs_spawn_100k(c: &mut Criterion) {
    c.bench_function("ecs_spawn_100k", |b| {
        b.iter(|| {
            let mut world = World::new();
            world.spawn_batch((0..100_000).map(|_| Bundle { ... }));
        });
    });
}

fn ecs_insert_bundle(c: &mut Criterion) {
    // ... 类似
}

criterion_group! {
    benches,
    ecs_query_iter_100k,
    ecs_query_par_100k,
    ecs_spawn_100k,
    ecs_insert_bundle
}
```

### 验收标准

| Benchmark | 指标 |
|-----------|------|
| `ecs_query_iter_100k` | Query::iter 100k 实体 < 10ms |
| `ecs_query_par_100k` | Query::par_for_each 100k 实体 < 5ms |
| `ecs_spawn_100k` | spawn_batch 100k 实体 < 100ms |
| `ecs_insert_bundle` | insert_bundle 性能稳定 |

---

## 依赖关系图

```
examples/
    ├── ecs_hello (最小 ECS) ──── World, Component, Query
    ├── ecs_100k (10 万粒子) ──── World, Bundle, Query::par_for_each
    ├── ecs_events (事件) ──── EventWriter, EventReader
    ├── ecs_hierarchy (层级) ──── Parent, Children, Transform2D
    ├── ecs_parallel (并行) ──── SystemStage::parallel
    ├── ecs_commands (命令) ──── Commands, apply
    ├── ecs_change_tracking (变更) ──── Added, Changed
    ├── ecs_resources (资源) ──── Resource, Res, ResMut
    ├── ecs_bundle (Bundle) ──── Bundle trait
    ├── ecs_schedule (调度) ──── Schedule, Stage
    ├── ecs_ray_cast (射线) ──── Query, Collider
    └── ecs_pong (游戏) ──── 综合示例
```

---

## 优先级说明

- **P0（关键）**：必须完成的示例
- **P1（重要）**：对功能展示有帮助
- **P2（期望）**：增强示例，可后续迭代