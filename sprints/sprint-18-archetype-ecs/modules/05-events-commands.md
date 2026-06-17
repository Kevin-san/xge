# Module 05 — Events 与 Commands

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)
> 文件位置: `engine-ecs/src/event.rs`, `engine-ecs/src/command.rs`

---

## 1. Events（事件双缓冲）

```rust
// engine-ecs/src/event.rs

pub trait Event: Send + Sync + 'static {}

pub struct Events<E: Event> {
    /// 双缓冲：A、B
    buffer_a: Vec<E>,
    buffer_b: Vec<E>,
    a_is_active: bool,
}

impl<E: Event> Events<E> {
    pub fn new() -> Self { Self { buffer_a: Vec::new(), buffer_b: Vec::new(), a_is_active: true } }
    
    pub fn send(&mut self, event: E) {
        if self.a_is_active { self.buffer_a.push(event); }
        else { self.buffer_b.push(event); }
    }
    
    pub fn update(&mut self) {
        // 切换活动缓冲
        self.a_is_active = !self.a_is_active;
        if self.a_is_active { self.buffer_a.clear(); }
        else { self.buffer_b.clear(); }
    }
    
    pub fn reader(&self) -> EventReader<E> {
        let events = if self.a_is_active { &self.buffer_b } else { &self.buffer_a };
        EventReader::new(events)
    }
}

pub struct EventReader<'w, E: Event> {
    events: &'w [E],
}

impl<'w, E: Event> EventReader<'w, E> {
    pub fn iter(&self) -> impl Iterator<Item = &E> { self.events.iter() }
    pub fn len(&self) -> usize { self.events.len() }
    pub fn is_empty(&self) -> bool { self.events.is_empty() }
}
```

## 2. SystemParam 实现

```rust
impl<'w, E: Event> SystemParam for EventReader<'w, E> {
    type State = ();
    type Item<'w> = EventReader<'w, E>;
    
    fn init_state(_: &World) -> Self::State { }
    fn get_param<'w>(_: &'w Self::State, world: &'w World) -> Self::Item<'w> {
        world.get_resource::<Events<E>>().expect("Events<E> not found").reader()
    }
}

impl<'w, E: Event> SystemParam for EventWriter<'w, E> {
    type State = ();
    type Item<'w> = EventWriter<'w, E>;
    
    fn get_param<'w>(_: &'w Self::State, world: &'w World) -> Self::Item<'w> {
        // world 借出，EventWriter 持有 &mut 引用
        todo!()  // 需要 World 提供分时借用
    }
}
```

## 3. Commands（延迟命令缓冲）

```rust
// engine-ecs/src/command.rs

pub enum Command {
    Spawn(Entity),
    Despawn(Entity),
    Insert(Entity, Box<dyn AnyComponent>),
    Remove(Entity, TypeId),
    SendEvent(Box<dyn AnyEvent>),
    InsertResource(Box<dyn AnyResource>),
}

pub struct Commands<'w> {
    queue: &'w mut Vec<Command>,
}

impl<'w> Commands<'w> {
    pub fn spawn(&mut self) -> Entity {
        let e = Entity::new(rand(), 0);  // 占位
        self.queue.push(Command::Spawn(e));
        e
    }
    
    pub fn despawn(&mut self, entity: Entity) {
        self.queue.push(Command::Despawn(entity));
    }
    
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) {
        self.queue.push(Command::Insert(entity, Box::new(component)));
    }
    
    pub fn send_event<E: Event>(&mut self, event: E) {
        self.queue.push(Command::SendEvent(Box::new(event)));
    }
}

pub fn apply_commands(world: &mut World, commands: Vec<Command>) {
    for cmd in commands {
        match cmd {
            Command::Spawn(e) => { world.force_spawn(e); }
            Command::Despawn(e) => { world.despawn(e); }
            Command::Insert(e, c) => { world.insert_boxed(e, c); }
            Command::Remove(e, t) => { world.remove_boxed(e, t); }
            Command::SendEvent(ev) => { world.send_event_boxed(ev); }
            Command::InsertResource(r) => { world.insert_resource_boxed(r); }
        }
    }
}
```

## 4. SystemParam 集成

```rust
fn update_system(
    mut commands: Commands,
    mut events: EventReader<MyEvent>,
    mut query: Query<&mut Health>,
) {
    for event in events.iter() {
        // ...
        commands.spawn(/* ... */);
    }
}

// Schedule 自动调用
let mut commands_queue: Vec<Command> = Vec::new();
schedule.run_with_commands(&mut world, &mut commands_queue);
apply_commands(&mut world, commands_queue.drain(..).collect());
```

## 5. 验收

- [ ] 10000 事件 / 帧 < 1 ms
- [ ] `EventReader` 重复读幂等
- [ ] `Commands::spawn` 立即可用（应用延迟 1 stage）
- [ ] 双缓冲测试：A→B 切换后 A 清空

## 6. 性能

| 操作 | 目标 |
|------|------|
| `Events::send` | < 50 ns |
| `Events::update` (1000 事件) | < 100 µs |
| `Commands::spawn` | < 100 ns |
| `apply_commands` (1000) | < 1 ms |
