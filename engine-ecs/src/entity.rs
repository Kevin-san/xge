//! 实体模块
//!
//! 定义 Entity 结构体、EntityTable 实体表以及相关引用类型。

use std::fmt;

use crate::{Component, World};

/// 实体 ID
///
/// 用于唯一标识一个实体。包含索引和代际，用于避免 ABA 问题。
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Entity {
    /// 实体索引
    id: u32,
    /// 实体代际
    generation: u32,
}

impl Entity {
    /// 创建新实体
    pub fn new(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }

    /// 创建空实体（无效）
    pub fn null() -> Self {
        Self {
            id: u32::MAX,
            generation: 0,
        }
    }

    /// 获取实体索引
    pub fn id(&self) -> u32 {
        self.id
    }

    /// 获取实体代际
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// 检查是否是空实体
    pub fn is_null(&self) -> bool {
        self.id == u32::MAX
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({}, {})", self.id, self.generation)
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({}, {})", self.id, self.generation)
    }
}

/// 实体引用（只读访问）
#[derive(Copy, Clone)]
pub struct EntityRef<'a> {
    /// 世界引用
    pub world: &'a World,
    /// 实体 ID
    pub entity: Entity,
}

impl<'a> EntityRef<'a> {
    /// 获取实体 ID
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// 检查实体是否存活
    pub fn is_alive(&self) -> bool {
        self.world.contains(self.entity)
    }
}

/// 实体可变引用（读写访问）
pub struct EntityMut<'a> {
    /// 世界可变引用
    pub world: &'a mut World,
    /// 实体 ID
    pub entity: Entity,
}

impl<'a> EntityMut<'a> {
    /// 获取实体 ID
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// 插入组件
    pub fn insert<C: Component>(&mut self, component: C) -> Option<C> {
        self.world.insert(self.entity, component)
    }

    /// 移除组件
    pub fn remove<C: Component>(&mut self) -> Option<C> {
        self.world.remove::<C>(self.entity)
    }

    /// 获取组件可变引用
    pub fn get_component_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.world.get_component_mut::<C>(self.entity)
    }
}

/// 实体在 Archetype 中的位置
#[derive(Debug, Clone, Copy)]
pub struct EntityLocation {
    pub archetype: u32,
    pub row: u32,
}

/// 实体表：分配和跟踪实体的生命周期与位置
pub struct EntityTable {
    generations: Vec<u32>,
    alive: Vec<bool>,
    locations: Vec<Option<EntityLocation>>,
    free_list: Vec<u32>,
    next_id: u32,
}

impl EntityTable {
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            alive: Vec::new(),
            locations: Vec::new(),
            free_list: Vec::new(),
            next_id: 0,
        }
    }

    pub fn allocate(&mut self) -> Entity {
        if let Some(id) = self.free_list.pop() {
            // 重用已释放的 id，generation +1
            let gen = &mut self.generations[id as usize];
            *gen = gen.wrapping_add(1);
            self.alive[id as usize] = true;
            self.locations[id as usize] = None;
            return Entity::new(id, self.generations[id as usize]);
        }
        // 分配新 id
        let id = self.next_id;
        self.next_id += 1;
        self.generations.push(1);
        self.alive.push(true);
        self.locations.push(None);
        Entity::new(id, 1)
    }

    pub fn free(&mut self, entity: Entity) -> bool {
        let id = entity.id() as usize;
        if id >= self.alive.len() {
            return false;
        }
        if !self.alive[id] {
            return false;
        }
        // 检查代际
        if self.generations[id] != entity.generation() {
            return false;
        }
        self.alive[id] = false;
        self.locations[id] = None;
        self.free_list.push(entity.id());
        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let id = entity.id() as usize;
        if id >= self.alive.len() {
            return false;
        }
        if !self.alive[id] {
            return false;
        }
        self.generations[id] == entity.generation()
    }

    pub fn get_location(&self, entity_id: u32) -> Option<EntityLocation> {
        let id = entity_id as usize;
        if id >= self.alive.len() || !self.alive[id] {
            return None;
        }
        self.locations[id]
    }

    pub fn set_location(&mut self, entity_id: u32, location: EntityLocation) {
        let id = entity_id as usize;
        if id >= self.locations.len() {
            return;
        }
        self.locations[id] = Some(location);
    }

    /// 遍历所有活着的 entity（不保证顺序）
    pub fn iter_alive(&self) -> impl Iterator<Item = Entity> + '_ {
        self.alive
            .iter()
            .enumerate()
            .filter_map(move |(i, &alive)| {
                if alive {
                    Some(Entity::new(i as u32, self.generations[i]))
                } else {
                    None
                }
            })
    }

    pub fn alive_count(&self) -> usize {
        self.alive.iter().filter(|&&a| a).count()
    }
}

impl Default for EntityTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(0, 0);
        assert_eq!(entity.id(), 0);
        assert_eq!(entity.generation(), 0);
        assert!(!entity.is_null());
    }

    #[test]
    fn test_entity_null() {
        let entity = Entity::null();
        assert_eq!(entity.id(), u32::MAX);
        assert_eq!(entity.generation(), 0);
        assert!(entity.is_null());
    }

    #[test]
    fn test_entity_copy() {
        let entity = Entity::new(1, 2);
        let entity2 = entity;
        assert_eq!(entity, entity2);
    }

    #[test]
    fn test_entity_new_id_generation() {
        let entity = Entity::new(42, 3);
        assert_eq!(entity.id(), 42);
        assert_eq!(entity.generation(), 3);
    }

    #[test]
    fn test_entity_null_is_null() {
        let entity = Entity::null();
        assert!(entity.is_null());
    }

    #[test]
    fn test_entity_non_null_is_not_null() {
        let entity = Entity::new(1, 1);
        assert!(!entity.is_null());
    }

    #[test]
    fn test_entity_equality() {
        let e1 = Entity::new(10, 20);
        let e2 = Entity::new(10, 20);
        let e3 = Entity::new(10, 30);
        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }

    #[test]
    fn test_entity_different_ids() {
        let e1 = Entity::new(1, 0);
        let e2 = Entity::new(2, 0);
        assert_ne!(e1.id(), e2.id());
    }
}
