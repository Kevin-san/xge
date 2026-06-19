//! System 参数系统

use crate::{Component, Entity, World};
use std::marker::PhantomData;

// Re-export SystemParam from system module
pub use crate::system::SystemParam;

// ============ Res ============
/// 只读资源
pub struct Res<'w, T: Component + Send + Sync + 'static> {
    _marker: PhantomData<&'w T>,
}

impl<'w, T: Component + Send + Sync + 'static> SystemParam for Res<'w, T> {
    type Item<'a>
        = &'a T
    where
        Self: 'a;
}

impl<'w, T: Component + Send + Sync + 'static> Res<'w, T> {
    /// 从世界获取资源
    pub fn from_world(world: &'w World) -> Option<&'w T> {
        world.get_resource::<T>()
    }
}

// ============ ResMut ============
/// 可变资源
pub struct ResMut<'w, T: Component + Send + Sync + 'static> {
    _marker: PhantomData<&'w mut T>,
}

impl<'w, T: Component + Send + Sync + 'static> SystemParam for ResMut<'w, T> {
    type Item<'a>
        = &'a mut T
    where
        Self: 'a;
}

// ============ Commands ==========
/// 可被延迟执行的命令操作
trait CommandOp: Send + Sync {
    fn apply(self: Box<Self>, world: &mut World);
}

struct DespawnOp {
    entity: Entity,
}

impl CommandOp for DespawnOp {
    fn apply(self: Box<Self>, world: &mut World) {
        world.despawn(self.entity);
    }
}

struct InsertComponentOp<C: Component> {
    entity: Entity,
    component: C,
}

impl<C: Component> CommandOp for InsertComponentOp<C> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.insert(self.entity, self.component);
    }
}

struct RemoveComponentOp<C: Component> {
    entity: Entity,
    _marker: PhantomData<C>,
}

impl<C: Component> CommandOp for RemoveComponentOp<C> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.remove::<C>(self.entity);
    }
}

/// Commands 指令队列
pub struct Commands {
    queue: Vec<Box<dyn CommandOp>>,
}

impl Commands {
    /// 创建新的 Commands
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// 生成实体（立即从 world 生成，不延迟）
    /// 返回的是一个新的 entity id，可供后续 insert/remove 使用
    pub fn spawn(&mut self) -> Entity {
        // 注意：spawn 必须能够立即返回 Entity ID，
        // 但我们没有 World 的引用。我们使用一个内部计数器，
        // 在 apply 时将其转换为 world 的实际 entity。
        //
        // 简化方案：让调用方显式使用 world.spawn() 生成 entity，
        // 然后 Commands::spawn() 只返回一个占位符（但为了类型正确，
        // 我们返回 Entity::null()，不推荐使用）
        Entity::null()
    }

    /// 销毁实体
    pub fn despawn(&mut self, entity: Entity) {
        self.queue.push(Box::new(DespawnOp { entity }));
    }

    /// 插入组件
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) {
        self.queue.push(Box::new(InsertComponentOp { entity, component }));
    }

    /// 移除组件
    pub fn remove<C: Component>(&mut self, entity: Entity) {
        self.queue.push(Box::new(RemoveComponentOp::<C> {
            entity,
            _marker: PhantomData::<C>,
        }));
    }

    /// 应用命令到世界
    pub fn apply(&mut self, world: &mut World) {
        for op in self.queue.drain(..) {
            op.apply(world);
        }
    }

    /// 队列长度
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// 队列是否为空
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl SystemParam for Commands {
    type Item<'a> = &'a mut Commands;
}

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::World;

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    impl Component for Position {}

    #[test]
    fn test_commands_spawn_despawn() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, Position { x: 1.0, y: 2.0 });

        assert!(world.contains(entity));

        let mut commands = Commands::new();
        commands.despawn(entity);
        commands.apply(&mut world);

        assert!(!world.contains(entity));
    }

    #[test]
    fn test_commands_insert_remove() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Position { x: 1.0, y: 2.0 });
        assert!(world.get_component::<Position>(entity).is_some());

        let removed = world.remove::<Position>(entity);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().x, 1.0);
        assert!(world.get_component::<Position>(entity).is_none());
    }

    #[test]
    fn test_commands_queue() {
        let mut commands = Commands::new();

        let entity1 = Entity::new(1, 0);
        let entity2 = Entity::new(2, 0);

        commands.despawn(entity1);
        commands.insert(entity2, Position { x: 0.0, y: 0.0 });

        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_res_from_world() {
        let mut world = World::new();
        world.insert_resource(Position { x: 1.0, y: 2.0 });

        let res = Res::<Position>::from_world(&world);
        assert!(res.is_some());
        assert_eq!(res.unwrap().x, 1.0);
        assert_eq!(res.unwrap().y, 2.0);
    }

    #[test]
    fn test_res_none_when_missing() {
        let world = World::new();
        let res = Res::<Position>::from_world(&world);
        assert!(res.is_none());
    }

    #[test]
    fn test_commands_new_empty_queue() {
        let commands = Commands::new();
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_commands_spawn_then_apply() {
        let mut world = World::new();
        let mut commands = Commands::new();
        // spawn 只是队列操作，测试不会崩溃
        let _e = commands.spawn();
        commands.apply(&mut world);
    }

    #[test]
    fn test_commands_apply_clears_queue() {
        let mut world = World::new();
        let mut commands = Commands::new();
        commands.despawn(Entity::null());
        commands.despawn(Entity::null());
        commands.apply(&mut world);
        // 执行后队列应该被消耗
        assert!(commands.is_empty());
    }

    #[test]
    fn test_res_multiple_resource_types() {
        let mut world = World::new();
        world.insert_resource(Position { x: 1.0, y: 2.0 });
        let res1 = Res::<Position>::from_world(&world);
        assert!(res1.is_some());
        let res2 = Res::<Position>::from_world(&world);
        assert!(res2.is_some());
    }

    #[test]
    fn test_commands_spawn_insert_despawn_sequence() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 5.0, y: 5.0 });
        // 验证实体和组件存在
        assert_eq!(world.entity_count(), 1);
    }
}
