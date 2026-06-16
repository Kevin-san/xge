//! 实体模块
//!
//! 定义 Entity 结构体和相关类型。

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
        Self { id: u32::MAX, generation: 0 }
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
}
