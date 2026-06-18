# Module 04 — Bundle 与 ChangeTick

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)
> 文件位置: `engine-ecs/src/bundle.rs`, `engine-ecs/src/change.rs`

---

## 1. Bundle

```rust
// engine-ecs/src/bundle.rs

pub trait Bundle: Send + Sync + 'static {
    fn component_types() -> Vec<TypeId>;
    fn insert(self, world: &mut World, entity: Entity);
}

/// 自动派生（procedural macro）
/// #[derive(Bundle)]
/// pub struct Player {
///     pub name: Name,
///     pub transform: Transform,
///     pub health: Health,
/// }
```

## 2. BundleInserter（高效批量插入）

```rust
pub struct BundleInserter<'w, B: Bundle> {
    world: &'w mut World,
    archetype: ArchetypeId,
    _phantom: PhantomData<B>,
}

impl<'w, B: Bundle> BundleInserter<'w, B> {
    pub fn new(world: &'w mut World) -> Self {
        let types = B::component_types();
        let arch = world.archetypes_mut().get_or_create(&types);
        Self { world, archetype: arch, _phantom: PhantomData }
    }
    
    /// 插入并返回实体
    pub fn insert(&mut self, bundle: B) -> Entity {
        let entity = self.world.spawn();
        bundle.insert(self.world, entity);
        entity
    }
    
    /// 不创建实体的纯插入（用于 entity 复用）
    pub fn insert_for(&mut self, entity: Entity, bundle: B) {
        bundle.insert(self.world, entity);
    }
}
```

## 3. ChangeTick

```rust
// engine-ecs/src/change.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick(pub u32);

#[derive(Debug, Clone, Copy, Default)]
pub struct ComponentTicks {
    pub added: Tick,
    pub changed: Tick,
}

pub struct World {
    // ...
    pub last_change_tick: Tick,
    pub change_tick: Tick,
}

impl World {
    pub fn increment_change_tick(&mut self) {
        self.last_change_tick = self.change_tick;
        self.change_tick = Tick(self.change_tick.0.wrapping_add(1));
    }
}
```

## 4. 组件 Tick 存储

```rust
/// 每列存储额外的 tick 信息
pub struct TypedColumnWithTicks<T: Component> {
    data: Vec<T>,
    ticks: Vec<ComponentTicks>,
    archetype_change_tick: Tick,
}

impl<T: Component + 'static> TypedColumnWithTicks<T> {
    pub fn push_with_tick(&mut self, value: T, tick: Tick) {
        self.data.push(value);
        self.ticks.push(ComponentTicks { added: tick, changed: tick });
    }
    
    pub fn get_ticks(&self, index: usize) -> ComponentTicks {
        self.ticks[index]
    }
}
```

## 5. Changed / Added Filter 实现

```rust
impl<C: Component> WorldFilter for Changed<C> {
    type State = CachedAccess<ComponentTicks>;
    
    fn init(world: &World) -> Self::State {
        CachedAccess { last_change_tick: world.last_change_tick, _phantom: PhantomData::<C> }
    }
    
    fn matches(arch: &Archetype, state: &Self::State, current_tick: Tick) -> bool {
        if let Some(col) = arch.get_column::<C>() {
            for ticks in &col.ticks {
                if ticks.changed > state.last_change_tick && ticks.changed <= current_tick {
                    return true;
                }
            }
        }
        false
    }
}
```

## 6. RemovedComponents

```rust
pub struct RemovedComponents<C: Component> {
    /// 跨 tick 累积，System 调用 `update` 后清理
    queue: VecDeque<Entity>,
    _phantom: PhantomData<C>,
}

impl<C: Component> RemovedComponents<C> {
    pub fn read(&mut self) -> impl Iterator<Item = Entity> + '_ {
        // 返回当前累积的实体
        self.queue.iter().copied().collect::<Vec<_>>().into_iter()
    }
    
    pub fn update(&mut self, world: &World) {
        // 从 World 的 removal table 拉取
        let removed = world.get_removed::<C>();
        for e in removed {
            self.queue.push_back(e);
        }
    }
}
```

## 7. 验收

- [ ] 1000 实体批量 spawn < 5 ms
- [ ] ChangeTick 推进开销 < 100 ns / 系统
- [ ] RemovedComponents 测试：add→remove→re-add 全部 tick 正确
- [ ] Changed filter 准确：100 帧动画只在前 1 帧触发

## 8. 性能

| 操作 | 目标 |
|------|------|
| `Bundle::insert` (5 组件) | < 200 ns |
| `spawn_batch` (1000 实体) | < 5 ms |
| ChangeTick 推进 | < 100 ns / 系统 |
