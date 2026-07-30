//! 查询模块 - ECS 查询系统
//!
//! 提供类型安全的查询接口，支持组件过滤（With/Without/Changed/Added）
//! 和安全的组件访问模式。

use crate::{Component, Entity, World};
use std::marker::PhantomData;

// ============ QueryFilter ============
/// 查询过滤器 trait — 支持运行时检查实体是否匹配过滤条件
pub trait QueryFilter: Sized + 'static {
    /// 检查实体是否通过过滤器
    fn check(world: &World, entity: Entity) -> bool;
}

/// 无过滤器（匹配所有实体）
pub struct NoneFilter;

impl QueryFilter for NoneFilter {
    fn check(_world: &World, _entity: Entity) -> bool {
        true
    }
}

/// 过滤包含特定组件的实体
pub struct With<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for With<C> {
    fn check(world: &World, entity: Entity) -> bool {
        world.has_component::<C>(entity)
    }
}

/// 过滤不包含特定组件的实体
pub struct Without<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for Without<C> {
    fn check(world: &World, entity: Entity) -> bool {
        !world.has_component::<C>(entity)
    }
}

/// 过滤上一帧发生变化的组件（当前实现为 stub，返回 true）
pub struct Changed<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for Changed<C> {
    fn check(_world: &World, _entity: Entity) -> bool {
        true
    }
}

/// 过滤新增的组件（当前实现为 stub，返回 true）
pub struct Added<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for Added<C> {
    fn check(_world: &World, _entity: Entity) -> bool {
        true
    }
}

/// 过滤禁用的实体（当前实现为 stub，返回 true）
pub struct WithoutBanned;

impl QueryFilter for WithoutBanned {
    fn check(_world: &World, _entity: Entity) -> bool {
        true
    }
}

// ============ AccessMode ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Read,
    Write,
    ReadWrite,
}

// ============ QueryItem ============
/// 查询结果项（组件引用）
pub struct QueryItem<'w, C: Component> {
    pub entity_id: u32,
    pub component: &'w C,
}

impl<'w, C: Component> std::ops::Deref for QueryItem<'w, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.component
    }
}

/// Query 结果项（可变组件）
pub struct QueryItemMut<'a, C: Component> {
    pub entity_id: u32,
    pub component: &'a mut C,
}

impl<'a, C: Component> std::ops::Deref for QueryItemMut<'a, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.component
    }
}

impl<'a, C: Component> std::ops::DerefMut for QueryItemMut<'a, C> {
    fn deref_mut(&mut self) -> &mut C {
        self.component
    }
}

// ============ QueryState ============
/// Query 状态（可用于系统参数）
pub struct QueryState<C: Component, F: QueryFilter = NoneFilter> {
    _marker: PhantomData<(C, F)>,
}

impl<C: Component, F: QueryFilter> QueryState<C, F> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<C: Component> Default for QueryState<C, NoneFilter> {
    fn default() -> Self {
        Self::new()
    }
}

// ============ Query ============
/// 查询接口（只读）
pub struct Query<'w, C: Component, F: QueryFilter = NoneFilter> {
    world: &'w World,
    _marker: PhantomData<(C, F)>,
}

impl<'w, C: Component, F: QueryFilter> Query<'w, C, F> {
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> QueryIter<'w, C, F> {
        QueryIter::<C, F>::new(self.world)
    }
}

/// Query 只读迭代器 — 应用过滤器后遍历匹配实体
pub struct QueryIter<'w, C: Component, F: QueryFilter = NoneFilter> {
    world: &'w World,
    entities: Vec<Entity>,
    index: usize,
    _marker: PhantomData<(C, F)>,
}

impl<'w, C: Component, F: QueryFilter> QueryIter<'w, C, F> {
    fn new(world: &'w World) -> Self {
        let entities = world.entities.iter_alive().collect::<Vec<_>>();
        Self {
            world,
            entities,
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'w, C: Component, F: QueryFilter> Iterator for QueryIter<'w, C, F> {
    type Item = QueryItem<'w, C>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;

            if !self.world.contains(entity) {
                continue;
            }

            if !F::check(self.world, entity) {
                continue;
            }

            if let Some(component) = self.world.get_component::<C>(entity) {
                return Some(QueryItem {
                    entity_id: entity.id(),
                    component,
                });
            }
        }
        None
    }
}

// ============ ComponentAccess trait ============
/// 组件访问标记 trait
pub trait ComponentAccess<C: Component> {
    fn access_mode() -> AccessMode {
        AccessMode::Read
    }
}

impl<C: Component> ComponentAccess<C> for &C {
    fn access_mode() -> AccessMode {
        AccessMode::Read
    }
}

impl<C: Component> ComponentAccess<C> for &mut C {
    fn access_mode() -> AccessMode {
        AccessMode::Write
    }
}

impl<C: Component> ComponentAccess<C> for (&C, &mut C) {
    fn access_mode() -> AccessMode {
        AccessMode::ReadWrite
    }
}

#[cfg(test)]
mod tests {
    use crate::query::{NoneFilter, Query, With, Without};
    use crate::{Component, World};

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
        hp: f32,
    }

    impl Component for Health {}

    #[test]
    fn test_query_basic_iteration() {
        let mut world = World::new();

        let entity1 = world.spawn();
        world.insert(entity1, Position { x: 1.0, y: 2.0 });

        let entity2 = world.spawn();
        world.insert(entity2, Position { x: 3.0, y: 4.0 });

        let _entity3 = world.spawn();

        let query = Query::<Position, NoneFilter>::new(&world);
        let mut count = 0;

        for item in query.iter() {
            count += 1;
            assert!(item.component.x >= 1.0 && item.component.x <= 3.0);
            assert!(item.component.y >= 2.0 && item.component.y <= 4.0);
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn test_query_empty_world() {
        let world = World::new();
        let query = Query::<Position, NoneFilter>::new(&world);
        let count = query.iter().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_query_with_filter_filters_correctly() {
        let mut world = World::new();

        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 2.0 });
        world.insert(e1, Velocity { x: 0.5, y: 0.5 });

        let e2 = world.spawn();
        world.insert(e2, Position { x: 3.0, y: 4.0 });

        // With<Velocity> should only find e1 (has both Position and Velocity)
        let query = Query::<Position, With<Velocity>>::new(&world);
        let count = query.iter().count();
        assert_eq!(
            count, 1,
            "With<Velocity> 应找到 1 个同时拥有 Position 和 Velocity 的实体"
        );
    }

    #[test]
    fn test_query_with_filter_no_match() {
        let mut world = World::new();

        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 2.0 });

        // With<Health> should find nothing (no entity has Health)
        let query = Query::<Position, With<Health>>::new(&world);
        let count = query.iter().count();
        assert_eq!(count, 0, "With<Health> 应找到 0 个实体");
    }

    #[test]
    fn test_query_without_filter() {
        let mut world = World::new();

        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 2.0 });
        world.insert(e1, Velocity { x: 0.5, y: 0.5 });

        let e2 = world.spawn();
        world.insert(e2, Position { x: 3.0, y: 4.0 });

        // Without<Velocity> should only find e2 (has Position but NOT Velocity)
        let query = Query::<Position, Without<Velocity>>::new(&world);
        let count = query.iter().count();
        assert_eq!(
            count, 1,
            "Without<Velocity> 应找到 1 个没有 Velocity 的实体"
        );
    }

    #[test]
    fn test_query_without_filter_all_match() {
        let mut world = World::new();

        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 2.0 });

        // Without<Health> should find e1 (it doesn't have Health)
        let query = Query::<Position, Without<Health>>::new(&world);
        let count = query.iter().count();
        assert_eq!(
            count, 1,
            "Without<Health> 应找到 1 个实体（没有 Health 组件）"
        );
    }

    #[test]
    fn test_query_despawned_entity() {
        let mut world = World::new();

        let entity1 = world.spawn();
        world.insert(entity1, Position { x: 1.0, y: 2.0 });

        let entity2 = world.spawn();
        world.insert(entity2, Position { x: 3.0, y: 4.0 });

        world.despawn(entity1);

        let query = Query::<Position, NoneFilter>::new(&world);
        let count = query.iter().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_query_multiple_components() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 2.0 });
        world.insert(e1, Velocity { x: 0.5, y: 0.5 });
        let e2 = world.spawn();
        world.insert(e2, Position { x: 3.0, y: 4.0 });
        let query = Query::<Position, NoneFilter>::new(&world);
        assert_eq!(query.iter().count(), 2);
    }

    #[test]
    fn test_query_component_values_read() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert(e1, Position { x: 100.0, y: 200.0 });
        let query = Query::<Position, NoneFilter>::new(&world);
        for item in query.iter() {
            assert_eq!(item.component.x, 100.0);
            assert_eq!(item.component.y, 200.0);
        }
    }

    #[test]
    fn test_query_no_components() {
        let mut world = World::new();
        world.spawn();
        world.spawn();
        let query = Query::<Position, NoneFilter>::new(&world);
        assert_eq!(query.iter().count(), 0);
    }

    #[test]
    fn test_query_10_entities() {
        let mut world = World::new();
        for i in 0..10 {
            let e = world.spawn();
            world.insert(
                e,
                Position {
                    x: i as f32,
                    y: i as f32,
                },
            );
        }
        let query = Query::<Position, NoneFilter>::new(&world);
        assert_eq!(query.iter().count(), 10);
    }

    #[test]
    fn test_query_with_complex_filter_scenario() {
        let mut world = World::new();

        // e1: Position + Velocity + Health
        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 1.0 });
        world.insert(e1, Velocity { x: 1.0, y: 1.0 });
        world.insert(e1, Health { hp: 100.0 });

        // e2: Position + Velocity
        let e2 = world.spawn();
        world.insert(e2, Position { x: 2.0, y: 2.0 });
        world.insert(e2, Velocity { x: 2.0, y: 2.0 });

        // e3: Position + Health
        let e3 = world.spawn();
        world.insert(e3, Position { x: 3.0, y: 3.0 });
        world.insert(e3, Health { hp: 50.0 });

        // e4: Position only
        let e4 = world.spawn();
        world.insert(e4, Position { x: 4.0, y: 4.0 });

        // With<Velocity>: e1, e2
        let q1 = Query::<Position, With<Velocity>>::new(&world);
        assert_eq!(q1.iter().count(), 2);

        // With<Health>: e1, e3
        let q2 = Query::<Position, With<Health>>::new(&world);
        assert_eq!(q2.iter().count(), 2);

        // Without<Velocity>: e3, e4
        let q3 = Query::<Position, Without<Velocity>>::new(&world);
        assert_eq!(q3.iter().count(), 2);

        // Without<Health>: e2, e4
        let q4 = Query::<Position, Without<Health>>::new(&world);
        assert_eq!(q4.iter().count(), 2);
    }
}
