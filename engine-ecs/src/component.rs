//! 组件模块
//!
//! 定义 Component trait 和组件存储。

use std::any::Any;
use std::collections::HashMap;

/// 组件 trait
///
/// 所有组件必须实现此 trait。
pub trait Component: Any + Send + Sync + 'static {
    /// 获取组件类型名称
    fn type_name() -> &'static str
    where
        Self: Sized,
    {
        std::any::type_name::<Self>()
    }
}

// 为基本类型实现 Component（允许它们作为组件使用）
impl Component for i32 {}
impl Component for u32 {}
impl Component for f32 {}
impl Component for String {}

/// 组件存储
///
/// 存储所有组件类型的组件数据。
pub struct ComponentStorage {
    /// 组件数据存储
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ComponentStorage {
    /// 创建新的组件存储
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// 插入组件
    pub fn insert<C: Component>(&mut self, component: C) {
        let type_id = TypeId::of::<C>();
        self.components.insert(type_id, Box::new(component));
    }

    /// 获取组件引用
    pub fn get<C: Component>(&self) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        self.components
            .get(&type_id)
            .and_then(|c| c.downcast_ref::<C>())
    }

    /// 获取组件可变引用
    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        self.components
            .get_mut(&type_id)
            .and_then(|c| c.downcast_mut::<C>())
    }

    /// 移除组件
    pub fn remove<C: Component>(&mut self) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.components
            .remove(&type_id)
            .and_then(|c| c.downcast::<C>().ok().map(|boxed| *boxed))
    }

    /// 清空所有组件
    pub fn clear(&mut self) {
        self.components.clear();
    }
}

impl Default for ComponentStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// 类型 ID
type TypeId = std::any::TypeId;

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
    fn test_component_storage_insert_get() {
        let mut storage = ComponentStorage::new();
        let pos = Position { x: 1.0, y: 2.0 };
        storage.insert(pos);

        let retrieved = storage.get::<Position>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().x, 1.0);
        assert_eq!(retrieved.unwrap().y, 2.0);
    }

    #[test]
    fn test_component_storage_remove() {
        let mut storage = ComponentStorage::new();
        let pos = Position { x: 1.0, y: 2.0 };
        storage.insert(pos);

        let removed = storage.remove::<Position>();
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().x, 1.0);

        let retrieved = storage.get::<Position>();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_component_storage_multiple_types() {
        let mut storage = ComponentStorage::new();
        storage.insert(Position { x: 1.0, y: 2.0 });
        storage.insert(Velocity { x: 0.5, y: 0.5 });

        assert!(storage.get::<Position>().is_some());
        assert!(storage.get::<Velocity>().is_some());
    }

    #[test]
    fn test_component_storage_new_empty() {
        let storage = ComponentStorage::new();
        assert!(storage.get::<Position>().is_none());
    }

    #[test]
    fn test_component_storage_overwrite_value() {
        let mut storage = ComponentStorage::new();
        storage.insert(Position { x: 1.0, y: 2.0 });
        storage.insert(Position { x: 9.0, y: 9.0 });
        let pos = storage.get::<Position>().unwrap();
        assert_eq!(pos.x, 9.0);
        assert_eq!(pos.y, 9.0);
    }

    #[test]
    fn test_component_storage_get_mut() {
        let mut storage = ComponentStorage::new();
        storage.insert(Position { x: 1.0, y: 2.0 });
        if let Some(p) = storage.get_mut::<Position>() {
            p.x = 99.0;
            p.y = 88.0;
        }
        let pos = storage.get::<Position>().unwrap();
        assert_eq!(pos.x, 99.0);
        assert_eq!(pos.y, 88.0);
    }
}
