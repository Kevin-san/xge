use core::fmt;
use core::marker::PhantomData;

/// 类型安全句柄，使用索引 + 代际号机制避免悬挂引用
#[derive(Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            index: self.index,
            generation: self.generation,
            _phantom: PhantomData,
        }
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
}
