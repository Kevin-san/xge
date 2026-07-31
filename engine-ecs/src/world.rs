//! World：ECS 世界容器
//!
//! World 结合了 EntityTable（实体分配/代际跟踪）、
//! ArchetypeStorage（按组件集合分组的组件存储）、
//! Resources（全局资源）以及 Events（事件分发）。

use crate::archetype::ArchetypeStorage;
use crate::bundle::Bundle;
use crate::component::{Component, ComponentSet};
use crate::entity::{Entity, EntityLocation, EntityTable};
use crate::event::Event;
use crate::resource::{Resource, Resources};

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct ResourceError(pub String);

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "resource error: {}", self.0)
    }
}

impl std::error::Error for ResourceError {}

pub struct World {
    pub entities: EntityTable,
    pub archetypes: ArchetypeStorage,
    pub resources: Resources,
    // 事件类型擦除存储
    pub(crate) events: HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntityTable::new(),
            archetypes: ArchetypeStorage::new(),
            resources: Resources::new(),
            events: HashMap::new(),
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    pub fn entity_count(&self) -> usize {
        self.entities.alive_count()
    }

    /// 遍历所有存活实体（不保证顺序）。
    pub fn entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter_alive()
    }

    pub fn spawn(&mut self) -> Entity {
        let entity = self.entities.allocate();
        let row = self.archetypes.finalize_push_entity(0, entity);
        self.entities
            .set_location(entity.id(), EntityLocation { archetype: 0, row });
        entity
    }

    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        let entity = self.spawn();
        bundle.insert(self, entity);
        entity
    }

    pub fn spawn_batch<B: Bundle, I: IntoIterator<Item = B>>(&mut self, iter: I) {
        for bundle in iter {
            self.spawn_bundle(bundle);
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.is_alive(entity) {
            return false;
        }
        let loc = match self.entities.get_location(entity.id()) {
            Some(l) => l,
            None => return false,
        };
        // 从 archetype 中移除
        let swapped = self.archetypes.remove_row(loc.archetype, loc.row as usize);
        // 如果有其他 entity 被 swap 到 row 位置，更新它的 location
        if let Some(moved_entity) = swapped {
            if let Some(old_loc) = self.entities.get_location(moved_entity.id()) {
                self.entities.set_location(
                    moved_entity.id(),
                    EntityLocation {
                        archetype: old_loc.archetype,
                        row: loc.row,
                    },
                );
            }
        }
        self.entities.free(entity);
        true
    }

    pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        if !self.entities.is_alive(entity) {
            return None;
        }
        let loc = self.entities.get_location(entity.id())?;
        self.archetypes.get_component::<C>(loc.archetype, loc.row)
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        if !self.entities.is_alive(entity) {
            return None;
        }
        let loc = self.entities.get_location(entity.id())?;
        self.archetypes
            .get_component_mut::<C>(loc.archetype, loc.row)
    }

    pub fn has_component<C: Component>(&self, entity: Entity) -> bool {
        if !self.entities.is_alive(entity) {
            return false;
        }
        let loc = match self.entities.get_location(entity.id()) {
            Some(l) => l,
            None => return false,
        };
        match self.archetypes.get(loc.archetype) {
            Some(arch) => arch.has_component(TypeId::of::<C>()),
            None => false,
        }
    }

    pub fn contains_component<C: Component>(&self, entity: Entity) -> bool {
        self.has_component::<C>(entity)
    }

    /// 向 entity 插入组件 C。若 entity 已存在 C，则替换并返回旧值。
    /// 若 entity 不包含 C，则跨 archetype 迁移整行并在新 archetype 末尾插入新 C。
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        if !self.entities.is_alive(entity) {
            // 对传入的值在 drop 前不触发 on_add：我们不 insert 它，但这里
            // 调用者期望 Option<C>，按约定返回 None 表示未 insert 任何东西
            // 但为了保持类型一致，这里返回 None
            return None;
        }
        let loc = self.entities.get_location(entity.id())?;

        // 检查是否已有 C
        let has_c = {
            let arch = self.archetypes.get(loc.archetype)?;
            arch.has_component(TypeId::of::<C>())
        };

        if has_c {
            // 原地替换
            return self
                .archetypes
                .replace_component::<C>(loc.archetype, loc.row, component);
        }

        // 构造新的组件集合
        let old_components = self
            .archetypes
            .get(loc.archetype)
            .unwrap()
            .components
            .clone();
        let mut new_types = old_components.types().to_vec();
        new_types.push(TypeId::of::<C>());
        let new_components = ComponentSet::new(new_types);
        let new_arch_id = self.archetypes.get_or_create(new_components);

        // 从旧 archetype 迁移（除 C 外，其实旧 archetype 并没有 C，所以这里是全部列）
        // move_all_columns_row 同时删除 src arch 的 row（包括所有列和 entities 列表）
        // 并返回被 swap 到 row 位置的 entity
        let swapped =
            self.archetypes
                .move_all_columns_row(loc.archetype, loc.row as usize, new_arch_id);
        // 插入 C 到新 archetype
        self.archetypes
            .push_entity_with_component::<C>(new_arch_id, entity, component);
        // 把 entity 加到新 archetype 的 entities vec
        let new_row = self.archetypes.finalize_push_entity(new_arch_id, entity);

        // 如果有 entity 被 swap 到 row 位置，更新它的 location
        if let Some(moved_entity) = swapped {
            if let Some(old_loc) = self.entities.get_location(moved_entity.id()) {
                self.entities.set_location(
                    moved_entity.id(),
                    EntityLocation {
                        archetype: old_loc.archetype,
                        row: loc.row,
                    },
                );
            }
        }
        // 更新当前 entity 的 location
        self.entities.set_location(
            entity.id(),
            EntityLocation {
                archetype: new_arch_id,
                row: new_row,
            },
        );

        None
    }

    /// 从 entity 移除组件 C。若 entity 有 C，则跨 archetype 迁移并返回被移除的值。
    pub fn remove<C: Component>(&mut self, entity: Entity) -> Option<C> {
        if !self.entities.is_alive(entity) {
            return None;
        }
        let loc = self.entities.get_location(entity.id())?;

        // 检查是否有 C
        let has_c = {
            let arch = self.archetypes.get(loc.archetype)?;
            arch.has_component(TypeId::of::<C>())
        };
        if !has_c {
            return None;
        }

        // 新的组件集合（去掉 C）
        let old_components = self
            .archetypes
            .get(loc.archetype)
            .unwrap()
            .components
            .clone();
        let new_types: Vec<TypeId> = old_components
            .types()
            .iter()
            .copied()
            .filter(|t| *t != TypeId::of::<C>())
            .collect();
        let new_components = ComponentSet::new(new_types);
        let new_arch_id = self.archetypes.get_or_create(new_components);

        // 先从旧 archetype 取出 C
        let mut value = self
            .archetypes
            .take_component::<C>(loc.archetype, loc.row)
            .unwrap();

        // 把其他列（除 C）从旧 archetype 迁移到新 archetype
        // move_all_columns_row_except 现在也处理 entities 的 swap_remove
        let except = vec![TypeId::of::<C>()];
        let swapped = self.archetypes.move_all_columns_row_except(
            loc.archetype,
            loc.row as usize,
            new_arch_id,
            &except,
        );

        // 把 entity 加到新 archetype
        let new_row = self.archetypes.finalize_push_entity(new_arch_id, entity);

        // 处理被 swap 到 row 位置的 entity
        if let Some(moved_entity) = swapped {
            if let Some(old_loc) = self.entities.get_location(moved_entity.id()) {
                self.entities.set_location(
                    moved_entity.id(),
                    EntityLocation {
                        archetype: old_loc.archetype,
                        row: loc.row,
                    },
                );
            }
        }
        // 更新当前 entity 的 location
        self.entities.set_location(
            entity.id(),
            EntityLocation {
                archetype: new_arch_id,
                row: new_row,
            },
        );

        // 对被移除的组件 value 调用 on_remove
        value.on_remove();

        Some(value)
    }

    // ========== 查询 API ==========

    /// 迭代遍历（单组件）。收集所有拥有 C 的 entity 并对其调用 f。
    pub fn for_each<C: Component, F: FnMut(Entity, &C)>(&self, mut f: F) {
        let required = ComponentSet::new(vec![TypeId::of::<C>()]);
        for arch in self
            .archetypes
            .iter_matching(&required, &ComponentSet::empty())
        {
            if let Some(slice) = self.archetypes.column_slice::<C>(arch.id) {
                for (idx, comp) in slice.iter().enumerate() {
                    f(arch.entities[idx], comp);
                }
            }
        }
    }

    pub fn for_each_mut<C: Component, F: FnMut(Entity, &mut C)>(&mut self, mut f: F) {
        let required = ComponentSet::new(vec![TypeId::of::<C>()]);
        let arch_ids: Vec<u32> = self
            .archetypes
            .iter_matching(&required, &ComponentSet::empty())
            .map(|a| a.id)
            .collect();
        for arch_id in arch_ids {
            let entities: Vec<Entity> = self
                .archetypes
                .get(arch_id)
                .map(|a| a.entities.clone())
                .unwrap_or_default();
            if let Some(slice) = self.archetypes.column_slice_mut::<C>(arch_id) {
                for (idx, comp) in slice.iter_mut().enumerate() {
                    f(entities[idx], comp);
                }
            }
        }
    }

    pub fn query_collect<C: Component>(&self) -> Vec<(Entity, &C)> {
        let mut result = Vec::new();
        let required = ComponentSet::new(vec![TypeId::of::<C>()]);
        for arch in self
            .archetypes
            .iter_matching(&required, &ComponentSet::empty())
        {
            if let Some(slice) = self.archetypes.column_slice::<C>(arch.id) {
                for (idx, comp) in slice.iter().enumerate() {
                    result.push((arch.entities[idx], comp));
                }
            }
        }
        result
    }

    // ========== Resources ==========

    pub fn insert_resource<R: Resource>(&mut self, res: R) {
        self.resources.insert(res);
    }

    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    /// 检查资源是否存在
    pub fn contains_resource<R: Resource>(&self) -> bool {
        self.resources.contains::<R>()
    }

    /// 移除资源，返回被移除的值
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove::<R>()
    }

    // ========== Events ==========

    pub fn send_event<E: Event + Clone>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        self.events
            .entry(type_id)
            .or_insert_with(|| Box::new(Vec::<E>::new()) as Box<dyn std::any::Any + Send + Sync>);
        if let Some(box_vec) = self.events.get_mut(&type_id) {
            if let Some(vec) = box_vec.downcast_mut::<Vec<E>>() {
                vec.push(event);
            }
        }
    }

    pub fn read_events<E: Event>(&self) -> Vec<&E> {
        let type_id = TypeId::of::<E>();
        if let Some(box_vec) = self.events.get(&type_id) {
            if let Some(vec) = box_vec.downcast_ref::<Vec<E>>() {
                return vec.iter().collect();
            }
        }
        Vec::new()
    }

    pub fn update(&mut self) {
        // 每帧清空事件队列（简单实现）
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Position {
        x: f32,
    }
    impl Component for Position {}

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        dx: f32,
    }
    impl Component for Velocity {}

    #[test]
    fn test_world_spawn_and_count() {
        let mut world = World::new();
        assert_eq!(world.entity_count(), 0);
        let _e = world.spawn();
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_world_insert_and_get() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(
            e,
            Position {
                x: std::f32::consts::PI,
            },
        );
        let p = world.get_component::<Position>(e).unwrap();
        assert_eq!(p.x, std::f32::consts::PI);
    }

    #[test]
    fn test_world_insert_to_entity_with_components_migrates() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 42.0 });
        world.insert(e, Velocity { dx: 100.0 });

        let p = world.get_component::<Position>(e).unwrap();
        let v = world.get_component::<Velocity>(e).unwrap();
        assert_eq!(p.x, 42.0);
        assert_eq!(v.dx, 100.0);
    }

    #[test]
    fn test_world_remove_component() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0 });
        world.insert(e, Velocity { dx: 2.0 });

        let removed = world.remove::<Position>(e);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().x, 1.0);

        // Position 应该没有了
        assert!(world.get_component::<Position>(e).is_none());
        // 但 Velocity 仍然存在
        assert_eq!(world.get_component::<Velocity>(e).unwrap().dx, 2.0);
    }

    #[test]
    fn test_world_despawn() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        world.insert(e1, Position { x: 1.0 });
        world.insert(e2, Position { x: 2.0 });

        assert!(world.despawn(e1));
        assert!(!world.contains(e1));
        assert!(world.contains(e2));
        assert_eq!(world.entity_count(), 1);

        // 查询应找到 1 个 Position
        let pos: Vec<_> = world.query_collect::<Position>();
        assert_eq!(pos.len(), 1);
    }

    #[test]
    fn test_world_for_each_mut() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0 });
        world.for_each_mut::<Position, _>(|_, p| p.x += 1.0);
        assert_eq!(world.get_component::<Position>(e).unwrap().x, 2.0);
    }

    #[test]
    fn test_world_resources() {
        let mut world = World::new();
        world.insert_resource(100i32);
        assert_eq!(*world.get_resource::<i32>().unwrap(), 100);
        *world.get_resource_mut::<i32>().unwrap() = 200;
        assert_eq!(*world.get_resource::<i32>().unwrap(), 200);
    }

    #[test]
    fn test_world_insert_replaces_existing_component() {
        let mut world = World::new();
        let e = world.spawn();
        world.insert(e, Position { x: 1.0 });
        let old = world.insert(e, Position { x: 2.0 });
        assert!(old.is_some());
        assert_eq!(old.unwrap().x, 1.0);
        assert_eq!(world.get_component::<Position>(e).unwrap().x, 2.0);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ParentComp(Entity);
    impl Component for ParentComp {}

    #[derive(Debug, Clone)]
    struct ChildrenComp {
        entities: Vec<Entity>,
    }
    impl Component for ChildrenComp {}

    #[test]
    fn test_world_hierarchy_like() {
        let mut world = World::new();
        let root = world.spawn();
        let parent = world.spawn();
        let child = world.spawn();

        // 模拟 set_parent(parent, Some(root))
        world.insert(parent, ParentComp(root));
        if world.get_component::<ChildrenComp>(root).is_none() {
            world.insert(
                root,
                ChildrenComp {
                    entities: Vec::new(),
                },
            );
        }
        if let Some(children) = world.get_component_mut::<ChildrenComp>(root) {
            children.entities.push(parent);
        }

        // 模拟 set_parent(child, Some(parent))
        world.insert(child, ParentComp(parent));
        if world.get_component::<ChildrenComp>(parent).is_none() {
            world.insert(
                parent,
                ChildrenComp {
                    entities: Vec::new(),
                },
            );
        }
        if let Some(children) = world.get_component_mut::<ChildrenComp>(parent) {
            children.entities.push(child);
        }

        assert!(
            world.get_component::<ParentComp>(child).is_some(),
            "child should have ParentComp"
        );
        assert_eq!(world.get_component::<ParentComp>(child).unwrap().0, parent);
        assert!(
            world.get_component::<ParentComp>(parent).is_some(),
            "parent should have ParentComp"
        );
        assert!(
            world.get_component::<ChildrenComp>(root).is_some(),
            "root should have ChildrenComp"
        );
        assert!(
            world.get_component::<ChildrenComp>(parent).is_some(),
            "parent should have ChildrenComp"
        );

        let ancestors: Vec<Entity> = {
            let mut result = Vec::new();
            let mut current = world.get_component::<ParentComp>(child).map(|p| p.0);
            while let Some(p) = current {
                result.push(p);
                current = world.get_component::<ParentComp>(p).map(|x| x.0);
            }
            result
        };
        assert_eq!(
            ancestors.len(),
            2,
            "expected 2 ancestors, got {}",
            ancestors.len()
        );
    }
}
