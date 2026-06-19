//! Bundle 模块 - 批量插入/删除组件

use crate::{Component, World};
use std::any::TypeId;
use std::vec::Vec;

/// Bundle trait - 批量组件操作
pub trait Bundle: Send + Sync {
    /// 批量插入组件到世界
    fn insert(self, world: &mut World, entity: crate::Entity)
    where
        Self: Sized;

    /// 批量移除组件从世界
    fn remove(self, world: &mut World, entity: crate::Entity)
    where
        Self: Sized;

    /// 获取 Bundle 中所有组件的类型 ID
    fn type_ids() -> Vec<TypeId>
    where
        Self: Sized;
}

/// Bundle for 单个组件
impl<C: Component + Send + Sync + 'static> Bundle for C {
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self);
    }

    fn remove(self, world: &mut World, entity: crate::Entity) {
        world.remove::<C>(entity);
    }

    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<C>()]
    }
}

/// Bundle for 两个组件
impl<C1: Component + Send + Sync + 'static, C2: Component + Send + Sync + 'static> Bundle
    for (C1, C2)
{
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
    }

    fn remove(self, world: &mut World, entity: crate::Entity) {
        world.remove::<C1>(entity);
        world.remove::<C2>(entity);
    }

    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<C1>(), TypeId::of::<C2>()]
    }
}

/// Bundle for 三个组件
impl<
        C1: Component + Send + Sync + 'static,
        C2: Component + Send + Sync + 'static,
        C3: Component + Send + Sync + 'static,
    > Bundle for (C1, C2, C3)
{
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
        world.insert(entity, self.2);
    }

    fn remove(self, world: &mut World, entity: crate::Entity) {
        world.remove::<C1>(entity);
        world.remove::<C2>(entity);
        world.remove::<C3>(entity);
    }

    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<C1>(), TypeId::of::<C2>(), TypeId::of::<C3>()]
    }
}

/// Bundle for 四个组件
impl<
        C1: Component + Send + Sync + 'static,
        C2: Component + Send + Sync + 'static,
        C3: Component + Send + Sync + 'static,
        C4: Component + Send + Sync + 'static,
    > Bundle for (C1, C2, C3, C4)
{
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
        world.insert(entity, self.2);
        world.insert(entity, self.3);
    }

    fn remove(self, world: &mut World, entity: crate::Entity) {
        world.remove::<C1>(entity);
        world.remove::<C2>(entity);
        world.remove::<C3>(entity);
        world.remove::<C4>(entity);
    }

    fn type_ids() -> Vec<TypeId> {
        vec![
            TypeId::of::<C1>(),
            TypeId::of::<C2>(),
            TypeId::of::<C3>(),
            TypeId::of::<C4>(),
        ]
    }
}

/// Bundle for 五个组件
impl<
        C1: Component + Send + Sync + 'static,
        C2: Component + Send + Sync + 'static,
        C3: Component + Send + Sync + 'static,
        C4: Component + Send + Sync + 'static,
        C5: Component + Send + Sync + 'static,
    > Bundle for (C1, C2, C3, C4, C5)
{
    fn insert(self, world: &mut World, entity: crate::Entity) {
        world.insert(entity, self.0);
        world.insert(entity, self.1);
        world.insert(entity, self.2);
        world.insert(entity, self.3);
        world.insert(entity, self.4);
    }

    fn remove(self, world: &mut World, entity: crate::Entity) {
        world.remove::<C1>(entity);
        world.remove::<C2>(entity);
        world.remove::<C3>(entity);
        world.remove::<C4>(entity);
        world.remove::<C5>(entity);
    }

    fn type_ids() -> Vec<TypeId> {
        vec![
            TypeId::of::<C1>(),
            TypeId::of::<C2>(),
            TypeId::of::<C3>(),
            TypeId::of::<C4>(),
            TypeId::of::<C5>(),
        ]
    }
}

/// Bundle 错误
#[derive(Debug, Clone)]
pub struct BundleError {
    /// 错误消息
    pub msg: String,
}

impl BundleError {
    /// 创建新的 BundleError
    pub fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

impl std::fmt::Display for BundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BundleError: {}", self.msg)
    }
}

impl std::error::Error for BundleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    impl Component for Position {}

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    impl Component for Velocity {}

    #[derive(Debug, Clone, PartialEq)]
    struct Health {
        value: u32,
    }

    impl Component for Health {}

    #[derive(Debug, Clone, PartialEq)]
    struct Name {
        value: String,
    }

    impl Component for Name {}

    #[derive(Debug, Clone, PartialEq)]
    #[allow(dead_code)]
    struct Active {
        flag: bool,
    }

    impl Component for Active {}

    #[test]
    fn test_bundle_single() {
        let mut world = World::new();
        let entity = world.spawn();

        let pos = Position { x: 1.0, y: 2.0 };
        pos.insert(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_some());
    }

    #[test]
    fn test_bundle_tuple_2() {
        let mut world = World::new();
        let entity = world.spawn();

        let bundle = (Position { x: 1.0, y: 2.0 }, Velocity { x: 0.5, y: 0.5 });
        bundle.insert(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
    }

    #[test]
    fn test_bundle_tuple_4() {
        let mut world = World::new();
        let entity = world.spawn();

        let bundle = (
            Position { x: 1.0, y: 2.0 },
            Velocity { x: 0.5, y: 0.5 },
            Health { value: 100 },
            Name {
                value: "test".to_string(),
            },
        );
        bundle.insert(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
        assert!(world.get_component::<Health>(entity).is_some());
        assert!(world.get_component::<Name>(entity).is_some());
        assert_eq!(world.get_component::<Name>(entity).unwrap().value, "test");
    }

    #[test]
    fn test_bundle_remove() {
        let mut world = World::new();
        let entity = world.spawn();

        // 插入组件
        let bundle = (
            Position { x: 1.0, y: 2.0 },
            Velocity { x: 0.5, y: 0.5 },
            Health { value: 100 },
        );
        bundle.insert(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
        assert!(world.get_component::<Health>(entity).is_some());

        // 移除组件
        let remove_bundle = (Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0, y: 0.0 });
        remove_bundle.remove(&mut world, entity);

        assert!(world.get_component::<Position>(entity).is_none());
        assert!(world.get_component::<Velocity>(entity).is_none());
        assert!(world.get_component::<Health>(entity).is_some()); // Health 没有被移除
    }

    #[test]
    fn test_bundle_type_ids() {
        let type_ids = <(Position, Velocity)>::type_ids();
        assert_eq!(type_ids.len(), 2);
    }

    #[test]
    fn test_bundle_error() {
        let err = BundleError::new("test error");
        assert_eq!(err.msg, "test error");
        assert_eq!(err.to_string(), "BundleError: test error");
    }

    #[test]
    fn test_spawn_bundle_single() {
        let mut world = World::new();
        let entity = world.spawn_bundle(Position { x: 5.0, y: 10.0 });

        assert!(world.contains(entity));
        assert!(world.get_component::<Position>(entity).is_some());
        assert_eq!(world.get_component::<Position>(entity).unwrap().x, 5.0);
    }

    #[test]
    fn test_spawn_bundle_tuple() {
        let mut world = World::new();
        let entity = world.spawn_bundle((Position { x: 1.0, y: 2.0 }, Velocity { x: 0.5, y: 0.5 }));

        assert!(world.contains(entity));
        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
    }

    #[test]
    fn test_bundle_replace_component() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Position { x: 1.0, y: 2.0 });

        // Inserting same component type should replace
        let new_pos = Position { x: 10.0, y: 20.0 };
        new_pos.insert(&mut world, entity);

        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    #[test]
    fn test_spawn_batch() {
        let mut world = World::new();

        let bundles = vec![
            Position { x: 1.0, y: 2.0 },
            Position { x: 3.0, y: 4.0 },
            Position { x: 5.0, y: 6.0 },
        ];

        world.spawn_batch(bundles.into_iter());

        assert_eq!(world.entity_count(), 3);
    }

    #[test]
    fn test_bundle_multiple_entities() {
        let mut world = World::new();
        let e1 = world.spawn_bundle((Position { x: 1.0, y: 2.0 }, Velocity { x: 0.1, y: 0.2 }));
        let e2 = world.spawn_bundle((Position { x: 3.0, y: 4.0 }, Velocity { x: 0.3, y: 0.4 }));

        assert_eq!(world.entity_count(), 2);

        let pos1 = world.get_component::<Position>(e1).unwrap();
        let pos2 = world.get_component::<Position>(e2).unwrap();

        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos2.x, 3.0);
    }

    #[test]
    fn test_bundle_insert_single_component() {
        let mut world = World::new();
        let e = world.spawn_bundle(Position { x: 5.0, y: 5.0 });
        assert!(world.contains_component::<Position>(e));
    }

    #[test]
    fn test_bundle_multiple_tuple_components() {
        let mut world = World::new();
        let e = world.spawn_bundle((Position { x: 1.0, y: 2.0 }, Velocity { x: 0.1, y: 0.2 }));
        assert!(world.contains_component::<Position>(e));
        assert!(world.contains_component::<Velocity>(e));
    }

    #[test]
    fn test_bundle_spawn_batch_iter() {
        let mut world = World::new();
        let bundles = (0..5).map(|i| Position { x: i as f32, y: i as f32 });
        world.spawn_batch(bundles);
        assert_eq!(world.entity_count(), 5);
    }
}
