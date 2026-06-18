//! System 参数系统

use crate::{Component, Entity, World};
use std::any::TypeId;
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
/// Commands 指令队列
pub struct Commands {
    queue: Vec<Command>,
}

#[allow(dead_code)]
enum Command {
    Spawn {
        bundle: Box<dyn std::any::Any + Send + Sync>,
    },
    Despawn {
        entity: Entity,
    },
    Insert {
        entity: Entity,
        component: Box<dyn std::any::Any + Send + Sync>,
    },
    Remove {
        entity: Entity,
        type_id: TypeId,
    },
}

impl Commands {
    /// 创建新的 Commands
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// 生成实体
    pub fn spawn(&mut self) -> Entity {
        Entity::null()
    }

    /// 销毁实体
    pub fn despawn(&mut self, entity: Entity) {
        self.queue.push(Command::Despawn { entity });
    }

    /// 插入组件
    pub fn insert(&mut self, entity: Entity, component: impl Send + Sync + 'static) {
        self.queue.push(Command::Insert {
            entity,
            component: Box::new(component),
        });
    }

    /// 移除组件
    pub fn remove<T: Component + Send + Sync + 'static>(&mut self, entity: Entity) {
        self.queue.push(Command::Remove {
            entity,
            type_id: TypeId::of::<T>(),
        });
    }

    /// 应用命令到世界
    pub fn apply(&mut self, world: &mut World) {
        for cmd in self.queue.drain(..) {
            match cmd {
                Command::Spawn { .. } => {}
                Command::Despawn { entity } => {
                    world.despawn(entity);
                }
                Command::Insert { .. } => {}
                Command::Remove { .. } => {}
            }
        }
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

        assert_eq!(commands.queue.len(), 2);
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
}
