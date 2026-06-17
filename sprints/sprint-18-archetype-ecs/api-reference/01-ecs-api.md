# Sprint 18 · API 参考

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)

---

## World

```rust
pub struct World { /* ... */ }

impl World {
    pub fn new() -> Self;
    
    // Entity
    pub fn spawn(&mut self) -> Entity;
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity;
    pub fn despawn(&mut self, entity: Entity) -> bool;
    pub fn contains(&self, entity: Entity) -> bool;
    pub fn entity(&self, entity: Entity) -> EntityRef<'_>;
    pub fn entity_mut(&mut self, entity: Entity) -> EntityMut<'_>;
    pub fn clear_entities(&mut self);
    
    // Component
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C>;
    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C>;
    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C>;
    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C>;
    
    // Resource
    pub fn insert_resource<R: Resource>(&mut self, resource: R);
    pub fn resource<R: Resource>(&self) -> &R;
    pub fn resource_mut<R: Resource>(&mut self) -> &mut R;
    pub fn get_resource<R: Resource>(&self) -> Option<&R>;
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R>;
    
    // Event
    pub fn send_event<E: Event>(&mut self, event: E);
    pub fn events<E: Event>(&self) -> EventReader<E>;
    
    // Tick
    pub fn change_tick(&self) -> Tick;
    pub fn last_change_tick(&self) -> Tick;
    pub fn increment_change_tick(&mut self);
    
    // 内部（高级用户）
    pub fn archetypes(&self) -> &ArchetypeTable;
    pub fn locations(&self) -> &HashMap<Entity, EntityLocation>;
}
```

## Component / Bundle

```rust
pub trait Component: Send + Sync + 'static { /* 默认无方法 */ }

pub trait Bundle: Send + Sync + 'static {
    fn component_types() -> Vec<TypeId>;
    fn insert(self, world: &mut World, entity: Entity);
}
```

## Query

```rust
pub struct Query<Q: WorldData, F: WorldFilter = ()> { /* ... */ }

impl<Q: WorldData, F: WorldFilter> Query<Q, F> {
    pub fn new(world: &World) -> Self;
    pub fn iter<'w>(&'w mut self, world: &'w World) -> QueryIter<'w, Q, F>;
    pub fn get(&mut self, world: &World, entity: Entity) -> Option<Q::Item<'_>>;
    pub fn single(&mut self, world: &World) -> Q::Item<'_>;
    pub fn is_empty(&mut self, world: &World) -> bool;
    pub fn len(&mut self, world: &World) -> usize;
}

// 过滤器
pub struct With<C>(PhantomData<C>);
pub struct Without<C>(PhantomData<C>);
pub struct Changed<C>(PhantomData<C>);
pub struct Added<C>(PhantomData<C>);
pub struct Or<T>(PhantomData<T>);

// WorldData
impl<T: Component> WorldData for &T { type Item<'w> = &'w T; }
impl<T: Component> WorldData for &mut T { type Item<'w> = &'w mut T; }
impl<T: Component> WorldData for Option<&T> { type Item<'w> = Option<&T>; }
impl<T: Component> WorldData for Option<&mut T> { type Item<'w> = Option<&mut T>; }
impl WorldData for Entity { type Item<'w> = Entity; }
impl<A, B> WorldData for (A, B) where A: WorldData, B: WorldData { ... }
```

## Schedule

```rust
pub struct Schedule { /* ... */ }

impl Schedule {
    pub fn new() -> Self;
    pub fn add_system<S: System>(&mut self, stage: StageLabel, system: S) -> &mut Self;
    pub fn run(&mut self, world: &mut World);
    pub fn set_executor<E: Executor>(&mut self, executor: E);
}

pub trait StageLabel: Send + Sync + 'static {
    fn as_str(&self) -> &str;
}

pub struct PreUpdate;
pub struct Update;
pub struct PostUpdate;
```

## Resource

```rust
pub trait Resource: Send + Sync + 'static { /* 默认无方法 */ }

// SystemParam
pub struct Res<'w, R: Resource> { /* ... */ }
pub struct ResMut<'w, R: Resource> { /* ... */ }
```

## Event

```rust
pub trait Event: Send + Sync + 'static { /* 默认无方法 */ }

pub struct Events<E: Event> { /* 双缓冲 */ }
pub struct EventReader<'w, E: Event> { /* ... */ }
pub struct EventWriter<'w, E: Event> { /* ... */ }
```

## Command

```rust
pub struct Commands<'w> { /* ... */ }
impl<'w> Commands<'w> {
    pub fn spawn(&mut self) -> Entity;
    pub fn despawn(&mut self, entity: Entity);
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C);
    pub fn remove<C: Component>(&mut self, entity: Entity);
    pub fn send_event<E: Event>(&mut self, event: E);
}
```

## SystemParam 宏

```rust
#[derive(SystemParam)]
pub struct MyParams<'w> {
    pub query: Query<'w, (&'w Position, &'w mut Velocity)>,
    pub time: Res<'w, Time>,
    pub mut events: EventWriter<'w, MyEvent>,
}
```
