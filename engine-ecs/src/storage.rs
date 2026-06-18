//! 存储模块 - 提供组件存储类型

use crate::Component;
use std::collections::HashMap;

// ============ SparseSet ============
/// SparseSet 稀疏集存储
/// O(1) 插入/删除，通过稠密数组+稀疏数组实现
pub struct SparseSet<T> {
    dense: Vec<T>,
    indices: HashMap<u32, usize>, // sparse: entity_id -> dense index
    len: usize,
}

impl<T> SparseSet<T> {
    /// 创建新的空 SparseSet
    pub fn new() -> Self {
        Self {
            dense: Vec::new(),
            indices: HashMap::new(),
            len: 0,
        }
    }

    /// 创建具有指定容量的 SparseSet
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            dense: Vec::with_capacity(capacity),
            indices: HashMap::with_capacity(capacity),
            len: 0,
        }
    }

    /// 插入元素，返回被替换的旧值（如果存在）
    pub fn insert(&mut self, entity_id: u32, value: T) -> Option<T> {
        if let Some(&dense_index) = self.indices.get(&entity_id) {
            // 替换现有元素，不改变长度
            Some(std::mem::replace(&mut self.dense[dense_index], value))
        } else {
            let dense_index = self.dense.len();
            self.indices.insert(entity_id, dense_index);
            self.dense.push(value);
            self.len += 1;
            None
        }
    }

    /// 移除元素，返回被移除的值
    pub fn remove(&mut self, entity_id: u32) -> Option<T> {
        if let Some(dense_index) = self.indices.remove(&entity_id) {
            // 从 dense 数组中移除元素
            let removed = self.dense.remove(dense_index);

            // 更新所有受影响的后续索引
            for (_, index) in self.indices.iter_mut() {
                if *index > dense_index {
                    *index -= 1;
                }
            }

            self.len -= 1;
            Some(removed)
        } else {
            None
        }
    }

    /// 获取元素的引用
    pub fn get(&self, entity_id: u32) -> Option<&T> {
        self.indices
            .get(&entity_id)
            .and_then(|&dense_index| self.dense.get(dense_index))
    }

    /// 获取元素的可变引用
    pub fn get_mut(&mut self, entity_id: u32) -> Option<&mut T> {
        self.indices
            .get(&entity_id)
            .and_then(|&dense_index| self.dense.get_mut(dense_index))
    }

    /// 检查是否包含指定实体
    pub fn contains(&self, entity_id: u32) -> bool {
        self.indices.contains_key(&entity_id)
    }

    /// 返回元素数量
    pub fn len(&self) -> usize {
        self.len
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 清空所有元素
    pub fn clear(&mut self) {
        self.dense.clear();
        self.indices.clear();
        self.len = 0;
    }

    /// 返回所有值的迭代器
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.dense.iter()
    }

    /// 返回所有值可变迭代器
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.dense.iter_mut()
    }

    /// 返回所有实体 ID 的迭代器
    pub fn entities(&self) -> impl Iterator<Item = u32> + '_ {
        self.indices.keys().copied()
    }

    /// 预留容量
    pub fn reserve(&mut self, additional: usize) {
        self.dense.reserve(additional);
        self.indices.reserve(additional);
    }
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============ HashMapStorage ============
/// HashMap 存储（适用于大组件）
pub struct HashMapStorage<T: Send + Sync> {
    data: HashMap<u32, T>,
}

impl<T: Send + Sync> HashMapStorage<T> {
    /// 创建新的空 HashMapStorage
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// 创建具有指定容量的 HashMapStorage
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
        }
    }

    /// 插入元素，返回被替换的旧值（如果存在）
    pub fn insert(&mut self, entity_id: u32, value: T) -> Option<T> {
        self.data.insert(entity_id, value)
    }

    /// 移除元素，返回被移除的值
    pub fn remove(&mut self, entity_id: u32) -> Option<T> {
        self.data.remove(&entity_id)
    }

    /// 获取元素的引用
    pub fn get(&self, entity_id: u32) -> Option<&T> {
        self.data.get(&entity_id)
    }

    /// 获取元素的可变引用
    pub fn get_mut(&mut self, entity_id: u32) -> Option<&mut T> {
        self.data.get_mut(&entity_id)
    }

    /// 检查是否包含指定实体
    pub fn contains(&self, entity_id: u32) -> bool {
        self.data.contains_key(&entity_id)
    }

    /// 返回元素数量
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 清空所有元素
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 返回所有值的迭代器
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }
}

impl<T: Send + Sync> Default for HashMapStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============ StorageType 枚举 ============
/// 存储类型标签
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    /// 密集存储（默认）
    Dense,
    /// 稀疏集存储
    Sparse,
    /// HashMap 存储（适用于大组件）
    HashMap,
}

/// StorageLabel trait - 让组件选择存储类型
pub trait StorageLabel: Sized + 'static {
    /// 返回该组件使用的存储类型
    fn storage_type() -> StorageType;
}

impl<T> StorageLabel for T
where
    T: Component,
    T: 'static,
    T: Send + Sync,
{
    fn storage_type() -> StorageType {
        StorageType::Dense
    }
}

// 为大组件实现 HashMapStorage
#[allow(unused_macros)]
macro_rules! impl_hashmap_storage {
    ($($t:ty),*) => {
        $(
            impl StorageLabel for $t {
                fn storage_type() -> StorageType {
                    StorageType::HashMap
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparseset_insert_remove() {
        let mut set = SparseSet::new();

        // 插入
        assert!(set.insert(1, "a").is_none());
        assert!(set.insert(2, "b").is_none());
        assert!(set.insert(3, "c").is_none());

        assert_eq!(set.len(), 3);
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(set.contains(3));

        // 替换
        assert_eq!(set.insert(1, "a'").unwrap(), "a");

        // 移除
        assert_eq!(set.remove(2).unwrap(), "b");
        assert!(!set.contains(2));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_sparseset_get() {
        let mut set = SparseSet::new();

        set.insert(1, 100);
        set.insert(2, 200);

        assert_eq!(set.get(1), Some(&100));
        assert_eq!(set.get(2), Some(&200));
        assert_eq!(set.get(3), None);

        // 可变访问
        if let Some(val) = set.get_mut(1) {
            *val = 150;
        }
        assert_eq!(set.get(1), Some(&150));
    }

    #[test]
    fn test_sparseset_clear() {
        let mut set = SparseSet::new();

        set.insert(1, "a");
        set.insert(2, "b");
        set.insert(3, "c");

        assert_eq!(set.len(), 3);

        set.clear();

        assert!(set.is_empty());
        assert!(!set.contains(1));
        assert!(!set.contains(2));
        assert!(!set.contains(3));
    }

    #[test]
    fn test_sparseset_iterators() {
        let mut set = SparseSet::new();

        set.insert(1, "a");
        set.insert(2, "b");
        set.insert(3, "c");

        let values: Vec<_> = set.values().collect();
        assert_eq!(values, vec![&"a", &"b", &"c"]);

        let entities: Vec<_> = set.entities().collect();
        assert!(entities.contains(&1));
        assert!(entities.contains(&2));
        assert!(entities.contains(&3));
    }

    #[test]
    fn test_hashmap_storage() {
        let mut storage = HashMapStorage::new();

        // 插入
        assert!(storage.insert(1, "hello").is_none());
        assert!(storage.insert(2, "world").is_none());

        assert_eq!(storage.len(), 2);
        assert!(storage.contains(1));
        assert!(storage.contains(2));

        // 获取
        assert_eq!(storage.get(1), Some(&"hello"));
        assert_eq!(storage.get(2), Some(&"world"));
        assert_eq!(storage.get(3), None);

        // 替换
        assert_eq!(storage.insert(1, "hi").unwrap(), "hello");
        assert_eq!(storage.get(1), Some(&"hi"));

        // 移除
        assert_eq!(storage.remove(2).unwrap(), "world");
        assert!(!storage.contains(2));
        assert_eq!(storage.len(), 1);

        // 清空
        storage.clear();
        assert!(storage.is_empty());
    }

    #[test]
    fn test_hashmap_storage_iter() {
        let mut storage = HashMapStorage::new();

        storage.insert(1, 10);
        storage.insert(2, 20);
        storage.insert(3, 30);

        let values: Vec<_> = storage.values().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&&10));
        assert!(values.contains(&&20));
        assert!(values.contains(&&30));
    }
}
