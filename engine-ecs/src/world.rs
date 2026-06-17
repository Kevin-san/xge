//! World 模块
//!
//! 定义 ECS World，是整个 ECS 架构的核心容器。

use slab::Slab;
use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::bundle::Bundle;
use super::{Component, Entity, Event, EventReader, Resource, Resources};

trait ComponentStorage: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove_entity(&mut self, entity_id: u32);
    fn clear(&mut self);
}

impl<T: Any + Send + Sync + for<'a> ComponentStorageImpl<'a>> ComponentStorage for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn remove_entity(&mut self, entity_id: u32) {
        <Self as ComponentStorageImpl>::remove_entity(self, entity_id);
    }
    fn clear(&mut self) {
        <Self as ComponentStorageImpl>::clear(self);
    }
}

trait ComponentStorageImpl<'a> {
    fn remove_entity(&mut self, entity_id: u32);
    fn clear(&mut self);
}

struct EntityTable {
    entities: Slab<EntityData>,
    free_list: Vec<u32>,
    next_index: u32,
}

#[derive(Clone)]
struct EntityData {
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

    fn despawn(&mut self, entity: Entity) -> Option<&EntityData> {
        let id = entity.id() as usize;
        if id >= self.entities.len() {
            return None;
        }

        let data = &mut self.entities[id];
        if !data.alive || data.generation != entity.generation() {
            return None;
        }

        data.alive = false;
        self.free_list.push(id as u32);
        Some(data)
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

struct ComponentStorages {
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
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
            .or_insert_with(|| Box::new(DenseStorage::<C>::new()));

        if let Some(storage) = storage.as_any_mut().downcast_mut::<DenseStorage<C>>() {
            storage.insert(entity.id(), component)
        } else {
            None
        }
    }

    fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.storages.get_mut(&type_id).and_then(|storage| {
            storage
                .as_any_mut()
                .downcast_mut::<DenseStorage<C>>()
                .and_then(|s| s.remove(entity.id()))
        })
    }

    fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        self.storages.get(&type_id).and_then(|storage| {
            storage
                .as_any()
                .downcast_ref::<DenseStorage<C>>()
                .and_then(|s| s.get(entity.id()))
        })
    }

    fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        self.storages.get_mut(&type_id).and_then(|storage| {
            storage
                .as_any_mut()
                .downcast_mut::<DenseStorage<C>>()
                .and_then(|s| s.get_mut(entity.id()))
        })
    }

    fn contains<C: Component>(&self, entity: Entity) -> bool {
        self.get::<C>(entity).is_some()
    }

    fn clear(&mut self) {
        self.storages.clear();
    }

    fn remove_entity(&mut self, entity_id: u32) {
        for storage in self.storages.values_mut() {
            storage.remove_entity(entity_id);
        }
    }

    fn remove_entity_with_types(&mut self, entity_id: u32, types: &[TypeId]) {
        for &type_id in types {
            if let Some(storage) = self.storages.get_mut(&type_id) {
                storage.remove_entity(entity_id);
            }
        }
    }
}

struct DenseStorage<C: Component> {
    data: Vec<Option<C>>,
    sparse: HashMap<u32, usize>,
    reverse: HashMap<usize, u32>,
}

impl<C: Component> DenseStorage<C> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            sparse: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    fn insert(&mut self, entity_id: u32, component: C) -> Option<C> {
        if let Some(&dense_index) = self.sparse.get(&entity_id) {
            self.data[dense_index].replace(component)
        } else {
            let dense_index = self.data.len();
            self.sparse.insert(entity_id, dense_index);
            self.reverse.insert(dense_index, entity_id);
            self.data.push(Some(component));
            None
        }
    }

    fn remove(&mut self, entity_id: u32) -> Option<C> {
        if let Some(dense_index) = self.sparse.remove(&entity_id) {
            self.reverse.remove(&dense_index);

            let last_index = self.data.len() - 1;
            if dense_index != last_index {
                if let Some(last_entity_id) = self.reverse.remove(&last_index) {
                    self.sparse.insert(last_entity_id, dense_index);
                    self.reverse.insert(dense_index, last_entity_id);
                    self.data.swap(dense_index, last_index);
                }
            }

            self.data.pop().flatten()
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

    fn len(&self) -> usize {
        self.sparse.len()
    }

    fn is_empty(&self) -> bool {
        self.sparse.is_empty()
    }
}

impl<C: Component> ComponentStorageImpl<'_> for DenseStorage<C> {
    fn remove_entity(&mut self, entity_id: u32) {
        let _ = self.remove(entity_id);
    }

    fn clear(&mut self) {
        self.data.clear();
        self.sparse.clear();
        self.reverse.clear();
    }
}

pub struct World {
    entities: EntityTable,
    components: ComponentStorages,
    resources: Resources,
    events: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntityTable::new(),
            components: ComponentStorages::new(),
            resources: Resources::new(),
            events: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn()
    }

    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        let entity = self.spawn();
        bundle.insert(self, entity);
        entity
    }

    pub fn spawn_batch(&mut self, bundles: impl Iterator<Item = impl Bundle>) {
        for bundle in bundles {
            self.spawn_bundle(bundle);
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if let Some(entity_data) = self.entities.despawn(entity) {
            let component_types = entity_data.component_types.clone();
            self.components.remove_entity_with_types(entity.id(), &component_types);
            true
        } else {
            false
        }
    }

    pub fn clear_entities(&mut self) {
        self.entities.clear();
        self.components.clear();
    }

    pub fn entity(&self, entity: Entity) -> super::EntityRef<'_> {
        super::EntityRef {
            world: self,
            entity,
        }
    }

    pub fn entity_mut(&mut self, entity: Entity) -> super::EntityMut<'_> {
        super::EntityMut {
            world: self,
            entity,
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        if let Some(data) = self.entities.get_mut(entity) {
            let type_id = TypeId::of::<C>();
            if !data.component_types.contains(&type_id) {
                data.component_types.push(type_id);
            }
        }
        self.components.insert(entity, component)
    }

    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        if let Some(data) = self.entities.get_mut(entity) {
            data.component_types.retain(|&t| t != TypeId::of::<C>());
        }
        self.components.remove::<C>(entity)
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        self.components.get::<C>(entity)
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        self.components.get_mut::<C>(entity)
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resources.insert(resource);
    }

    pub fn resource<R: Resource>(&self) -> &R {
        self.resources.get::<R>().expect("Resource not found")
    }

    pub fn resource_mut<R: Resource>(&mut self) -> &mut R {
        self.resources.get_mut::<R>().expect("Resource not found")
    }

    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove::<R>()
    }

    pub fn contains_resource<R: Resource>(&self) -> bool {
        self.resources.contains::<R>()
    }

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

    pub fn events<E: Event>(&self) -> EventReader<E> {
        let type_id = TypeId::of::<E>();
        if let Some(events) = self.events.get(&type_id) {
            if let Some(events) = events.downcast_ref::<Vec<E>>() {
                return EventReader::new(events.to_vec());
            }
        }
        EventReader::new(Vec::new())
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

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

    #[derive(Debug, Clone, PartialEq)]
    struct Health {
        value: i32,
    }

    impl Component for Health {}

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
    fn test_despawn_cleans_up_components() {
        let mut world = World::new();
        let entity = world.spawn();
        
        world.insert(entity, Position { x: 1.0, y: 2.0 });
        world.insert(entity, Velocity { x: 3.0, y: 4.0 });
        world.insert(entity, Health { value: 100 });

        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
        assert!(world.get_component::<Health>(entity).is_some());

        assert!(world.despawn(entity));

        assert!(!world.contains(entity));
        assert!(world.get_component::<Position>(entity).is_none());
        assert!(world.get_component::<Velocity>(entity).is_none());
        assert!(world.get_component::<Health>(entity).is_none());
    }

    #[test]
    fn test_despawn_multiple_entities() {
        let mut world = World::new();
        
        let e1 = world.spawn();
        world.insert(e1, Position { x: 1.0, y: 2.0 });
        
        let e2 = world.spawn();
        world.insert(e2, Position { x: 3.0, y: 4.0 });

        assert!(world.get_component::<Position>(e1).is_some());
        assert!(world.get_component::<Position>(e2).is_some());

        assert!(world.despawn(e1));

        assert!(!world.contains(e1));
        assert!(world.contains(e2));
        assert!(world.get_component::<Position>(e1).is_none());
        assert!(world.get_component::<Position>(e2).is_some());

        assert!(world.despawn(e2));
        assert!(world.get_component::<Position>(e2).is_none());
    }

    #[test]
    fn test_storage_compaction() {
        let mut world = World::new();
        
        let e1 = world.spawn();
        let e2 = world.spawn();
        let e3 = world.spawn();
        
        world.insert(e1, Position { x: 1.0, y: 1.0 });
        world.insert(e2, Position { x: 2.0, y: 2.0 });
        world.insert(e3, Position { x: 3.0, y: 3.0 });

        assert_eq!(world.get_component::<Position>(e1).unwrap().x, 1.0);
        assert_eq!(world.get_component::<Position>(e2).unwrap().x, 2.0);
        assert_eq!(world.get_component::<Position>(e3).unwrap().x, 3.0);

        assert!(world.despawn(e2));

        assert!(world.get_component::<Position>(e1).is_some());
        assert!(world.get_component::<Position>(e2).is_none());
        assert!(world.get_component::<Position>(e3).is_some());

        assert_eq!(world.get_component::<Position>(e1).unwrap().x, 1.0);
        assert_eq!(world.get_component::<Position>(e3).unwrap().x, 3.0);
    }

    #[test]
    fn test_despawn_nonexistent_entity() {
        let mut world = World::new();
        let entity = Entity::new(999, 0);

        assert!(!world.despawn(entity));
    }

    #[test]
    fn test_despawn_twice() {
        let mut world = World::new();
        let entity = world.spawn();

        assert!(world.despawn(entity));
        assert!(!world.despawn(entity));
    }
}