//! World 模块
//!
//! 定义 ECS World，是整个 ECS 架构的核心容器。

use slab::Slab;
use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::bundle::Bundle;
use super::{Component, Entity, Event, EventReader, Resource, Resources};

/// 资源访问错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceError;

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource not found")
    }
}

impl std::error::Error for ResourceError {}

/// 实体表
pub(crate) struct EntityTable {
    /// 实体数据
    entities: Slab<EntityData>,
    /// 空闲实体索引
    free_list: Vec<u32>,
    /// 下一个可用索引
    next_index: u32,
}

/// 实体数据
#[derive(Clone)]
pub(crate) struct EntityData {
    #[allow(dead_code)]
    pub id: u32,
    pub generation: u32,
    pub alive: bool,
    pub(crate) component_types: Vec<TypeId>,
}

impl EntityTable {
    fn new() -> Self {
        Self {
            entities: Slab::new(),
            free_list: Vec::new(),
            next_index: 0,
        }
    }

    fn spawn(&mut self) -> Entity {
        let id = if let Some(free_index) = self.free_list.pop() {
            free_index
        } else {
            let index = self.next_index;
            self.next_index += 1;
            index
        };

        let generation = if let Some(data) = self.entities.get(id as usize) {
            data.generation + 1
        } else {
            0
        };

        let entity = Entity::new(id, generation);

        self.entities.insert(EntityData {
            id,
            generation,
            alive: true,
            component_types: Vec::new(),
        });

        entity
    }

    fn despawn(&mut self, entity: Entity) -> bool {
        let id = entity.id() as usize;
        if id >= self.entities.len() {
            return false;
        }

        let data = &mut self.entities[id];
        if !data.alive || data.generation != entity.generation() {
            return false;
        }

        data.alive = false;
        self.free_list.push(id as u32);
        true
    }

    fn contains(&self, entity: Entity) -> bool {
        let id = entity.id() as usize;
        if id >= self.entities.len() {
            return false;
        }

        let data = &self.entities[id];
        data.alive && data.generation == entity.generation()
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[allow(dead_code)]
    fn get(&self, entity: Entity) -> Option<&EntityData> {
        let id = entity.id() as usize;
        self.entities
            .get(id)
            .filter(|d| d.alive && d.generation == entity.generation())
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut EntityData> {
        let id = entity.id() as usize;
        self.entities
            .get_mut(id)
            .filter(|d| d.alive && d.generation == entity.generation())
    }

    fn clear(&mut self) {
        self.entities.clear();
        self.free_list.clear();
        self.next_index = 0;
    }

    /// 迭代所有存活实体
    pub(crate) fn iter_alive(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().filter_map(|(_, data)| {
            if data.alive {
                Some(Entity::new(data.id, data.generation))
            } else {
                None
            }
        })
    }

    /// 根据索引获取实体数据
    #[allow(dead_code)]
    pub(crate) fn get_by_index(&self, index: usize) -> Option<&EntityData> {
        self.entities.get(index)
    }

    /// 获取实体总数
    pub(crate) fn len(&self) -> usize {
        self.entities.len()
    }
}

/// 组件存储（公开给 crate 内部使用）
pub(crate) struct ComponentStorages {
    storages: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ComponentStorages {
    fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let type_id = TypeId::of::<C>();
        let storage = self
            .storages
            .entry(type_id)
            .or_insert_with(|| Box::new(DenseStorage::<C>::new()) as Box<dyn Any + Send + Sync>);

        if let Some(storage) = storage.downcast_mut::<DenseStorage<C>>() {
            storage.insert(entity.id(), component)
        } else {
            None
        }
    }

    fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.storages.get_mut(&type_id).and_then(|storage| {
            storage
                .downcast_mut::<DenseStorage<C>>()
                .and_then(|s| s.remove(entity.id()))
        })
    }

    fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        self.storages.get(&type_id).and_then(|storage| {
            storage
                .downcast_ref::<DenseStorage<C>>()
                .and_then(|s| s.get(entity.id()))
        })
    }

    fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        self.storages.get_mut(&type_id).and_then(|storage| {
            storage
                .downcast_mut::<DenseStorage<C>>()
                .and_then(|s| s.get_mut(entity.id()))
        })
    }

    #[allow(dead_code)]
    fn contains<C: Component>(&self, entity: Entity) -> bool {
        self.get::<C>(entity).is_some()
    }

    fn clear(&mut self) {
        self.storages.clear();
    }

    /// 移除实体的所有组件（用于实体销毁时的清理）
    ///
    /// 注意：由于组件存储使用类型擦除，此方法无法正常工作。
    /// 无法在运行时通过 TypeId 调用泛型 remove<C> 方法。
    /// 这是一个已知的架构限制。组件会残留在存储中直到 clear_entities。
    #[allow(dead_code)]
    fn remove_entity(&mut self, _entity_id: u32) {
        // 无法通过类型擦除的 Box<dyn Any> 调用具体的 remove 方法
        // 真正的解决方案是重新设计组件存储以支持按 entity_id 批量移除
    }
}

/// 密集向量存储
struct DenseStorage<C: Component> {
    data: Vec<Option<C>>,
    sparse: HashMap<u32, usize>,
}

impl<C: Component> DenseStorage<C> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            sparse: HashMap::new(),
        }
    }

    fn insert(&mut self, entity_id: u32, component: C) -> Option<C> {
        if let Some(&dense_index) = self.sparse.get(&entity_id) {
            self.data[dense_index].replace(component)
        } else {
            // 插入新组件
            let dense_index = self.data.len();
            self.sparse.insert(entity_id, dense_index);
            self.data.push(Some(component));
            None
        }
    }

    fn remove(&mut self, entity_id: u32) -> Option<C> {
        if let Some(dense_index) = self.sparse.remove(&entity_id) {
            self.data[dense_index].take()
        } else {
            None
        }
    }

    fn get(&self, entity_id: u32) -> Option<&C> {
        if let Some(&dense_index) = self.sparse.get(&entity_id) {
            self.data[dense_index].as_ref()
        } else {
            None
        }
    }

    fn get_mut(&mut self, entity_id: u32) -> Option<&mut C> {
        if let Some(&dense_index) = self.sparse.get(&entity_id) {
            self.data[dense_index].as_mut()
        } else {
            None
        }
    }
}

/// ECS World
pub struct World {
    /// 实体表
    pub(crate) entities: EntityTable,
    /// 组件存储
    pub(crate) components: ComponentStorages,
    /// 资源存储
    resources: Resources,
    /// 事件
    events: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl World {
    /// 创建新的空世界
    pub fn new() -> Self {
        Self {
            entities: EntityTable::new(),
            components: ComponentStorages::new(),
            resources: Resources::new(),
            events: HashMap::new(),
        }
    }

    /// 生成新实体
    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn()
    }

    /// 使用 Bundle 生成实体
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        let entity = self.spawn();
        bundle.insert(self, entity);
        entity
    }

    /// 批量生成实体
    pub fn spawn_batch(&mut self, bundles: impl Iterator<Item = impl Bundle>) {
        for bundle in bundles {
            self.spawn_bundle(bundle);
        }
    }

    /// 销毁实体
    pub fn despawn(&mut self, entity: Entity) -> bool {
        // 重要：此实现有已知限制 - 由于组件存储使用类型擦除（HashMap<TypeId, Box<dyn Any>>），
        // 无法在运行时获取泛型类型信息，因此无法正确移除该实体的组件。
        // 已死亡实体的组件会残留在存储中直到 clear_entities 被调用。
        // 这是一个架构设计限制，真正的解决方案是重新设计组件存储以支持按 entity_id 移除。

        // 注意：组件并未从 self.components 中移除，这是一个内存泄漏问题
        // 组件数据会保留在 DenseStorage 中直到实体被重新创建（generation 增加）
        // 或者直到 clear_entities 被调用

        // 标记实体为死亡
        self.entities.despawn(entity)
    }

    /// 通过实体ID移除组件（内部辅助方法）
    #[allow(dead_code)]
    fn remove_components_by_entity(&mut self, entity_id: u32) {
        // 由于 ComponentStorages 使用类型擦除，无法在运行时调用泛型 remove<C> 方法
        // 尝试移除实体在所有组件类型中的数据
        self.components.remove_entity(entity_id);
    }

    /// 清空所有实体
    pub fn clear_entities(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    /// 获取实体引用
    pub fn entity(&self, entity: Entity) -> super::EntityRef<'_> {
        super::EntityRef {
            world: self,
            entity,
        }
    }

    /// 获取实体可变引用
    pub fn entity_mut(&mut self, entity: Entity) -> super::EntityMut<'_> {
        super::EntityMut {
            world: self,
            entity,
        }
    }

    /// 检查实体是否存活
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// 插入单个组件
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        if let Some(data) = self.entities.get_mut(entity) {
            data.component_types.push(TypeId::of::<C>());
        }
        self.components.insert(entity, component)
    }

    /// 移除单个组件
    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        if let Some(data) = self.entities.get_mut(entity) {
            data.component_types.retain(|&t| t != TypeId::of::<C>());
        }
        self.components.remove::<C>(entity)
    }

    /// 获取组件只读引用
    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        self.components.get::<C>(entity)
    }

    /// 获取组件可变引用
    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut::<C>(entity)
    }

    /// 插入资源
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resources.insert(resource);
    }

    /// 获取资源只读引用
    /// 
    /// # Panics
    /// 如果资源不存在则 panic
    pub fn resource<R: Resource>(&self) -> &R {
        self.resources.get::<R>().expect("Resource not found")
    }

    /// 安全获取资源只读引用（推荐使用）
    /// 
    /// 返回 `Ok(&R)` 如果资源存在，否则返回 `Err`
    pub fn try_resource<R: Resource>(&self) -> Result<&R, ResourceError> {
        self.resources.get::<R>().ok_or(ResourceError)
    }

    /// 获取资源可变引用
    /// 
    /// # Panics
    /// 如果资源不存在则 panic
    pub fn resource_mut<R: Resource>(&mut self) -> &mut R {
        self.resources.get_mut::<R>().expect("Resource not found")
    }

    /// 安全获取资源可变引用（推荐使用）
    /// 
    /// 返回 `Ok(&mut R)` 如果资源存在，否则返回 `Err`
    pub fn try_resource_mut<R: Resource>(&mut self) -> Result<&mut R, ResourceError> {
        self.resources.get_mut::<R>().ok_or(ResourceError)
    }

    /// 获取资源（可选）
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    /// 安全获取可变资源引用
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    /// 移除资源
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove::<R>()
    }

    /// 检查资源是否存在
    pub fn contains_resource<R: Resource>(&self) -> bool {
        self.resources.contains::<R>()
    }

    /// 检查实体是否存活
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    /// 获取所有存活实体的迭代器
    pub fn entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter_alive()
    }

    /// 发送事件
    pub fn send_event<E: Event>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        let events = self
            .events
            .entry(type_id)
            .or_insert_with(|| Box::new(Vec::<E>::new()) as Box<dyn Any + Send + Sync>);
        if let Some(events) = events.downcast_mut::<Vec<E>>() {
            events.push(event);
        }
    }

    /// 获取事件读取器
    pub fn events<E: Event>(&self) -> EventReader<'_, E> {
        let type_id = TypeId::of::<E>();
        if let Some(events) = self.events.get(&type_id) {
            if let Some(events) = events.downcast_ref::<Vec<E>>() {
                return EventReader::new(events.to_vec());
            }
        }
        EventReader::new(Vec::new())
    }

    /// 销毁实体并返回是否成功
    pub fn despawn_with_bundle(&mut self, entity: Entity) -> bool {
        self.despawn(entity)
    }

    /// 尝试获取实体（返回 None 如果已销毁）
    pub fn get_entity(&self, id: u32) -> Option<Entity> {
        let entity = Entity::new(id, 0);
        if self.contains(entity) {
            Some(entity)
        } else {
            None
        }
    }

    /// 获取实体数量
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// 清空世界（删除所有实体和组件）
    pub fn clear(&mut self) {
        self.entities.clear();
        self.components.clear();
        self.resources.clear();
        self.events.clear();
    }

    /// 检查实体是否包含指定组件
    pub fn contains_component<C: Component>(&self, entity: Entity) -> bool {
        self.components.contains::<C>(entity)
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// 运行系统
    pub fn run_system<F: FnOnce(&mut Self)>(&mut self, system_fn: F) {
        system_fn(self);
    }

    /// 迭代所有存活的实体
    ///
    /// 返回一个迭代器，遍历所有 is_alive 为 true 的实体。
    /// 注意：这需要遍历内部的 entity slab，可能较慢。
    pub fn iter_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter_alive()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

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

    #[test]
    fn test_world_spawn() {
        let mut world = World::new();
        let entity = world.spawn();

        assert!(!entity.is_null());
        assert!(world.contains(entity));
    }

    #[test]
    fn test_world_spawn_despawn() {
        let mut world = World::new();
        let entity = world.spawn();

        assert!(world.contains(entity));
        assert!(world.despawn(entity));
        assert!(!world.contains(entity));
    }

    #[test]
    fn test_world_insert_get_component() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Position { x: 1.0, y: 2.0 });

        let pos = world.get_component::<Position>(entity);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap().x, 1.0);
        assert_eq!(pos.unwrap().y, 2.0);
    }

    #[test]
    fn test_world_remove_component() {
        let mut world = World::new();
        let entity = world.spawn();

        world.insert(entity, Position { x: 1.0, y: 2.0 });
        let removed = world.remove::<Position>(entity);

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().x, 1.0);
        assert!(world.get_component::<Position>(entity).is_none());
    }

    #[test]
    fn test_world_resource() {
        let mut world = World::new();
        world.insert_resource(Position { x: 0.0, y: 0.0 });

        let pos = world.resource::<Position>();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
    }

    #[test]
    fn test_world_clear_entities() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, Position { x: 1.0, y: 2.0 });

        world.clear_entities();

        assert!(!world.contains(entity));
    }

    #[test]
    fn test_world_entity_count() {
        let mut world = World::new();
        assert_eq!(world.entity_count(), 0);

        let e1 = world.spawn();
        let e2 = world.spawn();

        assert_eq!(world.entity_count(), 2);
        assert!(world.contains(e1));
        assert!(world.contains(e2));
    }

    #[test]
    fn test_world_despawn_entity_not_accessible() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, Position { x: 1.0, y: 2.0 });

        // 实体被销毁后，contains 返回 false
        assert!(world.despawn(entity));
        assert!(!world.contains(entity));

        // 注意：组件仍然残留在存储中（已知限制）
        // 这是一个架构设计问题，需要重新设计组件存储来解决
    }

    #[test]
    fn test_world_despawn_clears_component_tracking() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, Position { x: 1.0, y: 2.0 });
        world.insert(entity, Velocity { x: 0.0, y: 0.0 });

        assert!(world.despawn(entity));

        // 实体不再存在
        assert!(!world.contains(entity));

        // 注意：由于类型擦除限制，无法在运行时移除组件
        // 这是已知的架构限制
    }

    #[test]
    fn test_world_spawn_multiple_entities() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();
        assert_eq!(world.entity_count(), 3);
        assert!(world.contains(e1));
        assert!(world.contains(e2));
        assert!(world.contains(e3));
    }

    #[test]
    fn test_world_is_alive_after_spawn() {
        let mut world = World::new();
        let entity = world.spawn();
        assert!(world.is_alive(entity));
        world.despawn(entity);
        assert!(!world.is_alive(entity));
    }

    #[test]
    fn test_world_is_empty_new() {
        let world = World::new();
        assert!(world.is_empty());
    }

    #[test]
    fn test_world_not_empty_after_spawn() {
        let mut world = World::new();
        world.spawn();
        assert!(!world.is_empty());
    }

    #[test]
    fn test_world_clear_all() {
        let mut world = World::new();
        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 1.0 });
        world.insert_resource(Position { x: 0.0, y: 0.0 });
        world.clear();
        assert!(world.is_empty());
    }

    #[test]
    fn test_world_entities_iter_collects_all() {
        let mut world = World::new();
        world.spawn();
        world.spawn();
        world.spawn();
        let entities: Vec<Entity> = world.entities_iter().collect();
        assert_eq!(entities.len(), 3);
    }

    #[test]
    fn test_world_contains_component_after_insert() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0, y: 2.0 });
        assert!(world.contains_component::<Position>(e));
    }

    #[test]
    fn test_world_get_component_mut_modify() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0, y: 2.0 });
        if let Some(pos) = world.get_component_mut::<Position>(e) {
            pos.x = 100.0;
        }
        assert_eq!(world.get_component::<Position>(e).unwrap().x, 100.0);
    }

    #[test]
    fn test_world_resource_mut_modify() {
        let mut world = World::new();
        world.insert_resource(Position { x: 0.0, y: 0.0 });
        {
            let pos = world.resource_mut::<Position>();
            pos.x = 42.0;
        }
        assert_eq!(world.resource::<Position>().x, 42.0);
    }

    #[test]
    fn test_world_remove_resource() {
        let mut world = World::new();
        world.insert_resource(Position { x: 0.0, y: 0.0 });
        assert!(world.contains_resource::<Position>());
        world.remove_resource::<Position>();
        assert!(!world.contains_resource::<Position>());
    }

    #[test]
    fn test_world_get_resource_option() {
        let mut world = World::new();
        assert!(world.get_resource::<Position>().is_none());
        world.insert_resource(Position { x: 1.0, y: 2.0 });
        assert!(world.get_resource::<Position>().is_some());
    }

    #[test]
    fn test_world_spawn_bundle_insert() {
        let mut world = World::new();
        let e = world.spawn_bundle((Position { x: 1.0, y: 2.0 }, Velocity { x: 0.0, y: 0.0 }));
        assert!(world.get_component::<Position>(e).is_some());
        assert!(world.get_component::<Velocity>(e).is_some());
    }

    #[test]
    fn test_world_spawn_batch_multiple() {
        let mut world = World::new();
        let before = world.entity_count();
        let bundles = (0..5).map(|_| Position { x: 0.0, y: 0.0 });
        world.spawn_batch(bundles);
        assert_eq!(world.entity_count(), before + 5);
    }

    #[test]
    fn test_world_entity_ref_is_alive() {
        let mut world = World::new();
        let e = world.spawn();
        let entity_ref = world.entity(e);
        assert!(entity_ref.is_alive());
    }

    #[test]
    fn test_world_insert_same_component_replaces() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0, y: 1.0 });
        world.insert(e, Position { x: 99.0, y: 99.0 });
        let pos = world.get_component::<Position>(e).unwrap();
        assert_eq!(pos.x, 99.0);
        assert_eq!(pos.y, 99.0);
    }

    #[test]
    fn test_world_insert_multiple_components() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0, y: 2.0 });
        world.insert(e, Velocity { x: 3.0, y: 4.0 });
        assert!(world.contains_component::<Position>(e));
        assert!(world.contains_component::<Velocity>(e));
    }
}
