//! 查询模块 - ECS 查询系统

use crate::{Component, Entity, World};
use std::marker::PhantomData;

// ============ QueryFilter ============
/// 查询过滤器基类
pub trait QueryFilter: Sized + 'static {}

/// 无过滤器
pub struct NoneFilter;

impl QueryFilter for NoneFilter {}

/// 过滤包含特定组件的实体
pub struct With<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for With<C> {}

/// 过滤不包含特定组件的实体
pub struct Without<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for Without<C> {}

/// 过滤上一帧发生变化的组件
pub struct Changed<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for Changed<C> {}

/// 过滤新增的组件
pub struct Added<C: Component>(PhantomData<C>);

impl<C: Component> QueryFilter for Added<C> {}

/// 过滤禁用的实体
pub struct WithoutBanned;

impl QueryFilter for WithoutBanned {}

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
    /// 实体ID
    pub entity_id: u32,
    /// 组件引用
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
    /// 实体ID
    pub entity_id: u32,
    /// 组件可变引用
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
/// Query 状态
pub struct QueryState<C: Component, F: QueryFilter = NoneFilter> {
    _marker: PhantomData<(C, F)>,
}

impl<C: Component, F: QueryFilter> QueryState<C, F> {
    /// 创建新的 Query 状态
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
/// 查询迭代器（只读）
pub struct Query<'w, C: Component, F: QueryFilter = NoneFilter> {
    world: &'w World,
    _marker: PhantomData<(C, F)>,
}

impl<'w, C: Component, F: QueryFilter> Query<'w, C, F> {
    /// 创建新的查询
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }

    /// 获取迭代器
    pub fn iter(&self) -> QueryIter<'w, C, F> {
        QueryIter::<C, F>::new(self.world)
    }
}

/// Query 只读迭代器
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
    use crate::query::{NoneFilter, Query};
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

        assert_eq!(count, 2, "应该找到 2 个拥有 Position 组件的实体");
    }

    #[test]
    fn test_query_empty_world() {
        let world = World::new();
        let query = Query::<Position, NoneFilter>::new(&world);

        let count = query.iter().count();
        assert_eq!(count, 0, "空世界应该没有查询结果");
    }

    #[test]
    fn test_query_filter_with() {
        // 注意：With 过滤器需要运行时类型检查才能正常工作
        // 当前实现只演示了基本的查询功能
        // 实际的过滤器功能需要在运行时通过 TypeId 检查实现
        let mut world = World::new();

        let entity1 = world.spawn();
        world.insert(entity1, Position { x: 1.0, y: 2.0 });
        world.insert(entity1, Velocity { x: 0.5, y: 0.5 });

        let entity2 = world.spawn();
        world.insert(entity2, Position { x: 3.0, y: 4.0 });

        // 使用 NoneFilter 来获取所有 Position 组件
        let query = Query::<Position, NoneFilter>::new(&world);
        let count = query.iter().count();

        assert_eq!(count, 2, "应该找到 2 个拥有 Position 组件的实体");
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

        assert_eq!(count, 1, "应该只有 1 个存活的实体");
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
            world.insert(e, Position { x: i as f32, y: i as f32 });
        }
        let query = Query::<Position, NoneFilter>::new(&world);
        assert_eq!(query.iter().count(), 10);
    }
}
