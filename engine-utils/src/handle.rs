use core::fmt;
use core::marker::PhantomData;

/// 类型安全句柄，使用索引 + 代际号机制避免悬挂引用
#[derive(Copy)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            generation: self.generation,
            _phantom: PhantomData,
        }
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

impl<T> Eq for Handle<T> {}

impl<T> core::hash::Hash for Handle<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.generation.hash(state);
    }
}

impl<T> Handle<T> {
    #[inline]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.index == u32::MAX
    }

    #[inline]
    pub const fn index(&self) -> u32 {
        self.index
    }

    #[inline]
    pub const fn generation(&self) -> u32 {
        self.generation
    }

    /// 返回 null 句柄
    pub fn null() -> Self {
        Self {
            index: u32::MAX,
            generation: u32::MAX,
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for Handle<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Handle {{ index: {}, generation: {} }}",
            self.index, self.generation
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h = Handle::<i32>::new(42, 1);
        assert_eq!(h.index(), 42);
        assert_eq!(h.generation(), 1);
        assert!(!h.is_null());
    }

    #[test]
    fn test_null() {
        let h: Handle<i32> = Handle::null();
        assert!(h.is_null());
        assert_eq!(h.index(), u32::MAX);
    }

    #[test]
    fn test_copy() {
        let h = Handle::<i32>::new(1, 2);
        let h2 = h;
        assert_eq!(h, h2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let h1 = Handle::<i32>::new(1, 1);
        let h2 = Handle::<i32>::new(1, 1);
        let h3 = Handle::<i32>::new(2, 1);

        let mut set = HashSet::new();
        set.insert(h1);

        assert!(set.contains(&h2));
        assert!(!set.contains(&h3));
    }

    #[test]
    fn test_eq_different_index() {
        let h1 = Handle::<i32>::new(1, 1);
        let h2 = Handle::<i32>::new(2, 1);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_eq_different_generation() {
        let h1 = Handle::<i32>::new(1, 1);
        let h2 = Handle::<i32>::new(1, 2);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_eq_same() {
        let h1 = Handle::<i32>::new(10, 5);
        let h2 = Handle::<i32>::new(10, 5);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_default_is_null() {
        let h: Handle<i32> = Handle::default();
        assert!(h.is_null());
    }

    #[test]
    fn test_clone() {
        let h1 = Handle::<i32>::new(42, 100);
        let h2 = h1.clone();
        assert_eq!(h1, h2);
        assert_eq!(h1.index(), h2.index());
        assert_eq!(h1.generation(), h2.generation());
    }

    #[test]
    fn test_hash_map() {
        use std::collections::HashMap;

        let h1 = Handle::<String>::new(1, 1);
        let h2 = Handle::<String>::new(1, 1);
        let h3 = Handle::<String>::new(2, 1);

        let mut map = HashMap::new();
        map.insert(h1, "value1");

        assert_eq!(map.get(&h2), Some(&"value1"));
        assert!(map.get(&h3).is_none());
    }

    #[test]
    fn test_debug_format() {
        let h = Handle::<i32>::new(42, 7);
        let s = format!("{:?}", h);
        assert!(s.contains("42"));
        assert!(s.contains("7"));
    }

    #[test]
    fn test_const_new() {
        const H: Handle<i32> = Handle::new(10, 5);
        assert_eq!(H.index(), 10);
        assert_eq!(H.generation(), 5);
        assert!(!H.is_null());
    }
}
