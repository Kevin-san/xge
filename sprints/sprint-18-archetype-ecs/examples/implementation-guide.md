# Sprint 18 · 示例程序

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)

---

## 示例列表

### 01-archetype-bench
**功能：** Archetype vs HashMap 性能对比
**输出：** 控制台表格

```rust
use engine_ecs::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32 }

fn main() {
    let mut world = World::new();
    let start = Instant::now();
    for _ in 0..10_000 {
        world.spawn_bundle((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }));
    }
    let dt = start.elapsed();
    println!("10k spawn: {:?}", dt);
    
    // Query 性能
    let mut q = Query::<(&Position, &Velocity)>::new(&world);
    let start = Instant::now();
    let count = q.iter(&world).count();
    let dt = start.elapsed();
    println!("Query 10k: {:?} ({} items)", dt, count);
}
```

### 02-query-iteration
**功能：** 5 种过滤器演示
**输出：** 实体列表

### 03-system-schedule
**功能：** 6 个 system 跨 2 stage 并行执行
**输出：** 调度顺序 + 时间

### 04-change-detection
**功能：** Changed / Added / Removed 演示
**输出：** 触发日志

### 05-bundle-spawn
**功能：** Bundle 派生 + 批量 spawn
**输出：** 性能数据
