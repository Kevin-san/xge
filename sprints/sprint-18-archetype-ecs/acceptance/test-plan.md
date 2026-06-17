# Sprint 18 · 验收测试计划

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)

---

## 1. 单元测试清单

| 模块 | 测试数 | 覆盖 |
|------|--------|------|
| Archetype | 40+ | 插入 / 移除 / 切换 / 多列 |
| Query | 80+ | With/Without/Or/Changed/Added + 5 种 WorldData |
| System | 30+ | 拓扑排序 / 资源冲突 |
| Schedule | 30+ | 并行 / 单线程 / 多 stage |
| Bundle | 20+ | 派生 / 批量 |
| ChangeTick | 15+ | 推进 / 过滤 / RemovedComponents |
| Event | 20+ | 双缓冲 / 跨 stage |
| Command | 20+ | 批量应用 / 冲突 |

**总计：** 250+ 单元测试

## 2. 关键测试用例

### 2.1 Archetype 切换

```rust
#[test]
fn test_archetype_migration() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Position { x: 0.0, y: 0.0 });
    world.insert(e, Velocity { x: 1.0, y: 0.0 });
    world.insert(e, Health { value: 100 });
    
    // 移除 Velocity，应触发 archetype 切换
    world.remove::<Velocity>(e);
    
    let pos = world.get_component::<Position>(e).unwrap();
    let health = world.get_component::<Health>(e).unwrap();
    assert_eq!(pos.x, 0.0);
    assert_eq!(health.value, 100);
}
```

### 2.2 Query 编译期检查

```rust
#[test]
fn test_query_compile_error() {
    // 编译期错误：NonExistent 未实现 Component
    // fn bad(mut world: World) {
    //     let _q = Query::<&NonExistent>::new(&mut world);
    // }
}
```

### 2.3 Changed Filter 准确

```rust
#[test]
fn test_changed_filter() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Position::default());
    world.increment_change_tick();
    
    // 第一次 Query 包含（刚刚 inserted）
    let mut q = Query::<&Position, Changed<Position>>::new(&world);
    assert_eq!(q.iter(&world).count(), 1);
    
    world.increment_change_tick();
    
    // 不变则不包含
    let mut q = Query::<&Position, Changed<Position>>::new(&world);
    assert_eq!(q.iter(&world).count(), 0);
}
```

### 2.4 Schedule 并行

```rust
#[test]
fn test_schedule_parallel() {
    let mut world = World::new();
    world.insert_resource(Counter(AtomicUsize::new(0)));
    world.insert_resource(OtherResource(0));
    
    let mut schedule = Schedule::new();
    schedule.add_system(Update, system_a);  // 写 Counter
    schedule.add_system(Update, system_b);  // 写 Counter
    schedule.add_system(Update, system_c);  // 读 OtherResource
    
    // 应在 2 个并行层执行：{a, b} 与 {c}
    let start = Instant::now();
    schedule.run(&mut world);
    let dt = start.elapsed();
    assert!(dt < Duration::from_millis(10));
}
```

### 2.5 资源冲突检测

```rust
#[test]
fn test_resource_conflict() {
    // 编译期检测：两个 system 都写 Counter
    // 应自动序列化，不在同层
    let mut world = World::new();
    let mut schedule = Schedule::new();
    schedule.add_system(Update, write_counter_a);
    schedule.add_system(Update, write_counter_b);
    
    // Schedule 自动拓扑排序为 a → b（同一资源写）
    schedule.run(&mut world);
    // 不应有竞争
}
```

## 3. 性能基准

| 基准 | 目标 |
|------|------|
| spawn 10000 实体 5 组件 | < 50 ms |
| archetype 切换 | < 5 µs |
| Query 10000 实体 | < 100 µs |
| 100 system 拓扑排序 | < 10 ms |
| 64 system 并行执行 | < 5 ms (8 核) |
| ChangeTick 推进 | < 100 ns / 系统 |

## 4. 跨平台编译

- [ ] x86_64 linux/windows/macos
- [ ] aarch64 (mac M1+)
- [ ] wasm32 (basic)

## 5. 兼容性

- [ ] 旧 API shim 适配层（2 sprint 兼容期）
- [ ] `cargo test --workspace` 全部通过
- [ ] 旧代码示例能继续工作
