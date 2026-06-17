# Module 02 — 类型安全 Query 系统

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)
> 文件位置: `engine-ecs/src/query/mod.rs`

---

## 1. 目标

**修复 `engine-ecs/src/query.rs#L1-L22` 空实现，完整实现：**
- 类型安全 `Query<Q, F>` — 编译期检查组件类型
- 过滤器 `With<C>`, `Without<C>`, `Changed<C>`, `Added<C>`, `Or<(...)>`
- 高效迭代（matched_archetypes 缓存）

## 2. API

```rust
// engine-ecs/src/query/mod.rs

/// Query 数据（元组：&T, &mut T, Option<&T> 等）
pub trait WorldData {
    type Item<'w>;
}

/// 过滤器
pub trait WorldFilter {
    type State;
    fn init(world: &World) -> Self::State;
    fn matches(archetype: &Archetype, state: &Self::State, change_tick: u32) -> bool;
}

/// 主查询类型
pub struct Query<Q: WorldData, F: WorldFilter = ()> {
    state: QueryState<Q, F>,
}

impl<Q: WorldData, F: WorldFilter> Query<Q, F> {
    pub fn new(world: &World) -> Self {
        Self { state: QueryState::new(world) }
    }
    
    pub fn iter<'w>(&'w mut self, world: &'w World) -> QueryIter<'w, Q, F> {
        self.state.iter(world)
    }
    
    pub fn get(&mut self, world: &World, entity: Entity) -> Option<Q::Item<'_>> {
        self.state.get(world, entity)
    }
    
    pub fn single(&mut self, world: &World) -> Q::Item<'_> { ... }
}
```

## 3. Filter

```rust
// With<C>：实体必须包含 C
pub struct With<C>(PhantomData<C>);
impl<C: Component> WorldFilter for With<C> { ... }

// Without<C>：实体不能包含 C
pub struct Without<C>(PhantomData<C>);
impl<C: Component> WorldFilter for Without<C> { ... }

// Changed<C>：C 自上次访问以来有变更
pub struct Changed<C>(PhantomData<C>);
impl<C: Component> WorldFilter for Changed<C> { ... }

// Added<C>：C 自上次访问以来新增
pub struct Added<C>(PhantomData<C>);
impl<C: Component> WorldFilter for Added<C> { ... }

// Or<(F1, F2, ...)>：任一过滤器匹配
pub struct Or<T>(PhantomData<T>);
impl<F1: WorldFilter, F2: WorldFilter> WorldFilter for Or<(F1, F2)> { ... }
```

## 4. WorldData 实现

```rust
// &T
impl<'w, T: Component> WorldData for &T {
    type Item<'w> = &'w T;
}

// &mut T
impl<'w, T: Component> WorldData for &mut T {
    type Item<'w> = &'w mut T;
}

// Entity
impl WorldData for Entity {
    type Item<'w> = Entity;
}

// EntityRef
impl WorldData for EntityRef<'_> { ... }

// 元组
impl<A: WorldData, B: WorldData> WorldData for (A, B) {
    type Item<'w> = (A::Item<'w>, B::Item<'w>);
}

// Option<&T>：可能不存在的组件
impl<'w, T: Component> WorldData for Option<&'w T> {
    type Item<'w> = Option<&'w T>;
}
```

## 5. 查询优化

```rust
pub struct QueryState<Q, F> {
    /// 已匹配的 Archetype 列表（缓存）
    matched_archetypes: Vec<ArchetypeId>,
    /// 类型擦除的访问器
    fetchers: Vec<Box<dyn Fetch<'static>>>,
    _phantom: PhantomData<(Q, F)>,
}

impl<Q: WorldData, F: WorldFilter> QueryState<Q, F> {
    pub fn new(world: &World) -> Self {
        let mut state = Self { matched_archetypes: Vec::new(), fetchers: Vec::new(), _phantom: PhantomData };
        state.update_archetypes(world);
        state
    }
    
    fn update_archetypes(&mut self, world: &World) {
        // 遍历所有 Archetype，应用 F 过滤
        for &arch_id in world.archetypes().all_ids() {
            let arch = world.archetype(arch_id);
            if F::matches(arch, &F::State::init(world), world.change_tick()) {
                if !self.matched_archetypes.contains(&arch_id) {
                    self.matched_archetypes.push(arch_id);
                    // 注册 fetcher
                    self.fetchers.push(Q::init_fetch(arch));
                }
            }
        }
    }
}
```

## 6. 迭代

```rust
pub struct QueryIter<'w, Q, F> {
    archetypes: &'w [ArchetypeId],
    current_arch: usize,
    current_row: usize,
    fetcher: &'w mut dyn Fetch<'w>,
    _phantom: PhantomData<Q>,
}

impl<'w, Q: WorldData, F: WorldFilter> Iterator for QueryIter<'w, Q, F> {
    type Item = Q::Item<'w>;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_row < self.archetype_len() {
                let item = self.fetcher.fetch(self.current_row);
                self.current_row += 1;
                return Some(item);
            }
            // 切到下个 Archetype
            self.current_arch += 1;
            if self.current_arch >= self.archetypes.len() {
                return None;
            }
            self.current_row = 0;
        }
    }
}
```

## 7. 验收

- [ ] 单 archetype 10000 实体 Query 迭代 < 100 µs
- [ ] 编译期类型检查：访问不存在组件编译错误
  ```rust
  fn bad(world: &mut World) {
      let mut q = Query::<&NonExistentComponent>::new(world);
      // 编译错误：NonExistentComponent 未实现 Component
  }
  ```
- [ ] `Changed` / `Added` 时间戳准确
- [ ] 5 种过滤（With/Without/Or/Changed/Added）100% 路径
- [ ] 性能基准：cargo bench 10000 实体
