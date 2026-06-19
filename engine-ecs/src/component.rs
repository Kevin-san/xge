//! 组件模块
//!
//! 定义 Component trait 和组件存储。

use std::any::{self, Any, TypeId};
use std::collections::{HashMap, HashSet};

/// 组件存储类型
///
/// 用于控制组件在存储层的存储策略，影响访问性能与缓存友好度。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageType {
    /// 表存储（默认）— 按 Archetype 分组的密集数组
    ///
    /// 最适合高频迭代的组件，提供最佳缓存命中。
    Table,
    /// 稀疏集存储 — 适合实体稀疏拥有的组件
    ///
    /// 如偶发标记/状态组件。按实体 ID 直接索引。
    SparseSet,
}

impl Default for StorageType {
    fn default() -> Self {
        StorageType::Table
    }
}

/// 组件唯一标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(pub u64);

impl ComponentId {
    /// 从类型推导稳定的 ComponentId
    pub fn of<C: Component>() -> Self {
        // 使用 TypeId 的哈希值作为稳定 ID
        // 注：在同一编译单元内 TypeId 是稳定的
        let type_id = TypeId::of::<C>();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        type_id.hash(&mut hasher);
        ComponentId(hasher.finish())
    }
}

/// 组件 trait
///
/// 所有组件必须实现此 trait。
pub trait Component: Any + Send + Sync + 'static {
    /// 组件的存储类型（默认 Table）
    fn storage_type() -> StorageType
    where
        Self: Sized,
    {
        StorageType::Table
    }

    /// 获取组件类型名称
    fn type_name() -> &'static str
    where
        Self: Sized,
    {
        any::type_name::<Self>()
    }

    /// 获取组件唯一 ID
    fn component_id() -> ComponentId
    where
        Self: Sized,
    {
        ComponentId::of::<Self>()
    }

    /// 组件生命周期：当被附加到实体时调用（可选实现）
    fn on_add(&mut self) {}

    /// 组件生命周期：当被从实体移除时调用（可选实现）
    fn on_remove(&mut self) {}
}

// 为基本类型实现 Component（允许它们作为组件使用）
impl Component for i32 {}
impl Component for u32 {}
impl Component for f32 {}
impl Component for String {}

/// 组件类型集合（排序后的 TypeId，用于描述一个 Archetype）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ComponentSet {
    types: Vec<TypeId>,
}

impl ComponentSet {
    /// 空集合
    pub fn empty() -> Self {
        Self { types: Vec::new() }
    }

    /// 创建一个新的组件集合（自动去重+排序）
    pub fn new(mut types: Vec<TypeId>) -> Self {
        types.sort_unstable();
        types.dedup();
        Self { types }
    }

    /// 包含的类型数量
    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    pub fn types(&self) -> &[TypeId] {
        &self.types
    }

    /// 集合中是否包含某类型
    pub fn contains(&self, type_id: TypeId) -> bool {
        self.types.binary_search(&type_id).is_ok()
    }

    /// 集合是否包含另一个集合的全部元素（子集判断）
    pub fn contains_all(&self, other: &ComponentSet) -> bool {
        other.types.iter().all(|t| self.contains(*t))
    }

    /// 是否与另一个集合无交集
    pub fn is_disjoint(&self, other: &ComponentSet) -> bool {
        for t in &other.types {
            if self.contains(*t) {
                return false;
            }
        }
        true
    }

    /// 添加一个类型（保持排序与去重）
    pub fn insert(&mut self, type_id: TypeId) {
        if let Err(idx) = self.types.binary_search(&type_id) {
            self.types.insert(idx, type_id);
        }
    }

    /// 移除一个类型
    pub fn remove(&mut self, type_id: TypeId) {
        if let Ok(idx) = self.types.binary_search(&type_id) {
            self.types.remove(idx);
        }
    }

    /// 转换为 HashSet 便于集合运算
    pub fn to_set(&self) -> HashSet<TypeId> {
        self.types.iter().copied().collect()
    }
}

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

    // ========= 新增：Component trait 增强测试 =========

    #[test]
    fn test_storage_type_default() {
        assert_eq!(<Position as Component>::storage_type(), StorageType::Table);
        assert_eq!(<Velocity as Component>::storage_type(), StorageType::Table);
    }

    #[test]
    fn test_component_type_name() {
        let name = <Position as Component>::type_name();
        assert!(name.contains("Position"));
    }

    #[test]
    fn test_component_id_unique() {
        let id_pos = <Position as Component>::component_id();
        let id_vel = <Velocity as Component>::component_id();
        assert_ne!(id_pos, id_vel);
    }

    #[test]
    fn test_component_id_stable() {
        let id1 = <Position as Component>::component_id();
        let id2 = <Position as Component>::component_id();
        assert_eq!(id1, id2);
    }

    // ========= ComponentSet 测试 =========

    #[test]
    fn test_component_set_basic() {
        let mut set = ComponentSet::new(vec![
            TypeId::of::<Position>(),
            TypeId::of::<Velocity>(),
        ]);
        assert_eq!(set.len(), 2);
        assert!(set.contains(TypeId::of::<Position>()));
        assert!(set.contains(TypeId::of::<Velocity>()));
        assert!(!set.contains(TypeId::of::<String>()));
    }

    #[test]
    fn test_component_set_dedup_and_sort() {
        // 乱序 + 重复
        let set = ComponentSet::new(vec![
            TypeId::of::<f32>(),
            TypeId::of::<i32>(),
            TypeId::of::<f32>(),
            TypeId::of::<u32>(),
            TypeId::of::<i32>(),
        ]);
        assert_eq!(set.len(), 3);
        // types 内部应是排序的
        let types = set.types();
        assert!(types.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_component_set_contains_all() {
        let big = ComponentSet::new(vec![
            TypeId::of::<Position>(),
            TypeId::of::<Velocity>(),
            TypeId::of::<String>(),
        ]);
        let small = ComponentSet::new(vec![TypeId::of::<Position>(), TypeId::of::<Velocity>()]);
        assert!(big.contains_all(&small));
        assert!(!small.contains_all(&big));
    }

    #[test]
    fn test_component_set_insert_remove() {
        let mut set = ComponentSet::new(vec![TypeId::of::<Position>()]);
        set.insert(TypeId::of::<Velocity>());
        assert_eq!(set.len(), 2);

        // 重复 insert 不增长
        set.insert(TypeId::of::<Velocity>());
        assert_eq!(set.len(), 2);

        set.remove(TypeId::of::<Position>());
        assert_eq!(set.len(), 1);
        assert!(!set.contains(TypeId::of::<Position>()));

        // 移除不存在的类型不崩溃
        set.remove(TypeId::of::<i32>());
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_component_set_is_disjoint() {
        let a = ComponentSet::new(vec![TypeId::of::<Position>()]);
        let b = ComponentSet::new(vec![TypeId::of::<Velocity>()]);
        let c = ComponentSet::new(vec![TypeId::of::<Position>(), TypeId::of::<f32>()]);

        assert!(a.is_disjoint(&b));
        assert!(!a.is_disjoint(&c));
    }

    #[test]
    fn test_component_set_empty() {
        let set = ComponentSet::empty();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    // ========= 自定义 StorageType =========

    #[derive(Debug, Clone, PartialEq)]
    struct Tag;
    impl Component for Tag {
        fn storage_type() -> StorageType {
            StorageType::SparseSet
        }
    }

    #[test]
    fn test_custom_storage_type() {
        assert_eq!(<Tag as Component>::storage_type(), StorageType::SparseSet);
        assert_eq!(<Position as Component>::storage_type(), StorageType::Table);
    }
}
