use crate::Handle;
use alloc::vec::Vec;

/// 使用 free list 的对象池，提供 O(1) 平均复杂度的增删改查
pub struct Arena<T> {
    items: Vec<Option<T>>,
    generations: Vec<u32>,
    free_indices: Vec<u32>,
    len: usize,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            generations: Vec::new(),
            free_indices: Vec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            generations: Vec::with_capacity(capacity),
            free_indices: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    /// 插入对象，返回句柄
    pub fn insert(&mut self, value: T) -> Handle<T> {
        let (index, generation) = if let Some(free_index) = self.free_indices.pop() {
            let generation = self.generations[free_index as usize];
            self.items[free_index as usize] = Some(value);
            (free_index, generation)
        } else {
            let index = self.items.len() as u32;
            self.items.push(Some(value));
            self.generations.push(0);
            (index, 0)
        };

        self.len += 1;
        Handle::<T>::new(index, generation)
    }

    /// 移除对象
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        if handle.is_null() {
            return None;
        }

        let index = handle.index() as usize;
        if index >= self.items.len() {
            return None;
        }

        if self.generations[index] != handle.generation() {
            return None;
        }

        let value = self.items[index].take();
        if value.is_some() {
            // 检查 generation 溢出
            self.generations[index] = self.generations[index].wrapping_add(1);
            // 如果溢出回绕到 0，新插入的相同索引会匹配旧的 generation=0 的句柄
            // 这是一个已知的边界情况，但 40 亿次循环不太可能在正常使用时发生
            self.free_indices.push(handle.index());
            self.len -= 1;
        }

        value
    }

    /// 获取不可变引用
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        if handle.is_null() {
            return None;
        }

        let index = handle.index() as usize;
        if index >= self.items.len() {
            return None;
        }

        if self.generations[index] != handle.generation() {
            return None;
        }

        self.items[index].as_ref()
    }

    /// 获取可变引用
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        if handle.is_null() {
            return None;
        }

        let index = handle.index() as usize;
        if index >= self.items.len() {
            return None;
        }

        if self.generations[index] != handle.generation() {
            return None;
        }

        self.items[index].as_mut()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.generations.clear();
        self.free_indices.clear();
        self.len = 0;
    }

    pub fn iter(&self) -> ArenaIter<'_, T> {
        ArenaIter::<T> {
            arena: self,
            index: 0,
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(Handle<T>, &mut T) -> bool,
    {
        let mut i = 0;
        while i < self.items.len() {
            if let Some(ref mut item) = self.items[i] {
                let handle: Handle<T> = Handle::new(i as u32, self.generations[i]);
                if !f(handle.clone(), item) {
                    self.remove(handle);
                    continue;
                }
            }
            i += 1;
        }
    }
}

/// Arena 迭代器
pub struct ArenaIter<'a, T> {
    arena: &'a Arena<T>,
    index: usize,
}

impl<'a, T> IntoIterator for &'a Arena<T> {
    type Item = (Handle<T>, &'a T);
    type IntoIter = ArenaIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIter::<T>::new(self)
    }
}

impl<'a, T> ArenaIter<'a, T> {
    fn new(arena: &'a Arena<T>) -> Self {
        Self { arena, index: 0 }
    }
}

impl<'a, T> Iterator for ArenaIter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.arena.items.len() {
            let idx = self.index;
            self.index += 1;

            if let Some(ref item) = self.arena.items[idx] {
                let handle: Handle<T> = Handle::new(idx as u32, self.arena.generations[idx]);
                return Some((handle, item));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_get() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        assert_eq!(arena.get(h), Some(&42));
    }

    #[test]
    fn test_remove() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        assert_eq!(arena.remove(h), Some(42));
        assert_eq!(arena.get(h), None);
    }

    #[test]
    fn test_generation() {
        let mut arena = Arena::new();
        let h = arena.insert(1);
        arena.remove(h);
        let h2 = arena.insert(2);

        assert!(arena.get(h).is_none());
        assert_eq!(arena.get(h2), Some(&2));
    }

    #[test]
    fn test_free_list() {
        let mut arena = Arena::new();
        let _h1 = arena.insert(1);
        let h2 = arena.insert(2);
        let _h3 = arena.insert(3);

        arena.remove(h2);

        let h4 = arena.insert(4);
        assert_eq!(h4.index(), h2.index());
        assert!(arena.get(h4).is_some());
    }

    #[test]
    fn test_iter() {
        let mut arena = Arena::new();
        arena.insert(1);
        arena.insert(2);
        arena.insert(3);

        let sum: i32 = arena.iter().map(|(_, v)| *v).sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_len() {
        let mut arena = Arena::new();
        assert!(arena.is_empty());

        arena.insert(1);
        assert_eq!(arena.len(), 1);

        let h = arena.insert(2);
        arena.remove(h);
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn test_retain() {
        let mut arena = Arena::new();
        let h1 = arena.insert(1);
        arena.insert(2);
        arena.insert(3);

        arena.retain(|_, v| *v > 1);
        assert!(arena.get(h1).is_none());
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn test_get_mut() {
        let mut arena = Arena::new();
        let h = arena.insert(10);
        if let Some(val) = arena.get_mut(h) {
            *val = 20;
        }
        assert_eq!(arena.get(h), Some(&20));
    }

    #[test]
    fn test_clear() {
        let mut arena = Arena::new();
        arena.insert(1);
        arena.insert(2);
        assert_eq!(arena.len(), 2);

        arena.clear();
        assert_eq!(arena.len(), 0);
        assert!(arena.is_empty());
    }

    #[test]
    fn test_null_handle() {
        let mut arena = Arena::new();
        let h: Handle<i32> = Handle::null();
        assert!(arena.get(h).is_none());
        assert!(arena.get_mut(h).is_none());
        assert!(arena.remove(h).is_none());
    }

    #[test]
    fn test_out_of_bounds_handle() {
        let mut arena = Arena::new();
        arena.insert(1);
        let h = Handle::new(100, 0);
        assert!(arena.get(h).is_none());
        assert!(arena.get_mut(h).is_none());
        assert!(arena.remove(h).is_none());
    }

    #[test]
    fn test_with_capacity() {
        let arena = Arena::<i32>::with_capacity(100);
        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn test_multiple_free_and_reuse() {
        let mut arena = Arena::new();
        let h1 = arena.insert(1);
        let h2 = arena.insert(2);
        let h3 = arena.insert(3);

        arena.remove(h1);
        arena.remove(h3);

        let h4 = arena.insert(4);
        let h5 = arena.insert(5);

        assert_eq!(h4.index(), h3.index());
        assert_eq!(h5.index(), h1.index());
        assert_eq!(arena.get(h4), Some(&4));
        assert_eq!(arena.get(h5), Some(&5));
    }

    #[test]
    fn test_into_iter() {
        let mut arena = Arena::new();
        arena.insert(10);
        arena.insert(20);
        arena.insert(30);

        let mut values = Vec::new();
        for (_, val) in &arena {
            values.push(*val);
        }
        assert_eq!(values, vec![10, 20, 30]);
    }

    #[test]
    fn test_remove_preserves_others() {
        let mut arena = Arena::new();
        let h1 = arena.insert(1);
        let h2 = arena.insert(2);
        let h3 = arena.insert(3);

        arena.remove(h2);

        assert_eq!(arena.get(h1), Some(&1));
        assert_eq!(arena.get(h3), Some(&3));
    }
}
