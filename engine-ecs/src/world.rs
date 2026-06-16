//! World 模块
//!
//! 定义 ECS World，是整个 ECS 架构的核心容器。

use slab::Slab;
use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::bundle::Bundle;
use super::{Component, Entity, Event, EventReader, Resource, Resources};

/// 实体表
struct EntityTable {
    /// 实体数据
    entities: Slab<EntityData>,
    /// 空闲实体索引
    free_list: Vec<u32>,
    /// 下一个可用索引
    next_index: u32,
}

/// 实体数据
#[derive(Clone)]
struct EntityData {
    #[allow(dead_code)]
    id: u32,
    generation: u32,
    alive: bool,
    component_types: Vec<TypeId>,
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

    fn len(&self) -> usize {
        self.entities.len()
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
}

/// 组件存储
struct ComponentStorages {
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
    entities: EntityTable,
    /// 组件存储
    components: ComponentStorages,
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
        // 移除所有组件
        // 这里需要更好的实现
        self.entities.despawn(entity)
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
    pub fn resource<R: Resource>(&self) -> &R {
        self.resources.get::<R>().expect("Resource not found")
    }

    /// 获取资源可变引用
    pub fn resource_mut<R: Resource>(&mut self) -> &mut R {
        self.resources.get_mut::<R>().expect("Resource not found")
    }

    /// 获取资源（可选）
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    /// 移除资源
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove::<R>()
    }

    /// 检查资源是否存在
    pub fn contains_resource<R: Resource>(&self) -> bool {
        self.resources.contains::<R>()
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
    pub fn events<E: Event>(&self) -> EventReader<E> {
        let type_id = TypeId::of::<E>();
        if let Some(events) = self.events.get(&type_id) {
            if let Some(events) = events.downcast_ref::<Vec<E>>() {
                return EventReader::new(events.to_vec());
            }
        }
        EventReader::new(Vec::new())
    }

    /// 获取实体数量
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// 运行系统
    pub fn run_system<F: FnOnce(&mut Self)>(&mut self, system_fn: F) {
        system_fn(self);
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
}
