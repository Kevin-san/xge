use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
    _marker: core::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    pub const fn null() -> Self {
        Self {
            index: u32::MAX,
            generation: 0,
            _marker: core::marker::PhantomData,
        }
    }

    pub const fn is_null(&self) -> bool {
        self.index == u32::MAX
    }

    pub const fn index(&self) -> u32 {
        self.index
    }

    pub const fn generation(&self) -> u32 {
        self.generation
    }

    pub(crate) const fn from_raw(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _marker: core::marker::PhantomData,
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
        write!(f, "Handle(index={}, generation={})", self.index, self.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_is_null() {
        let h: Handle<i32> = Handle::null();
        assert!(h.is_null());
        
        let h2: Handle<i32> = Handle::from_raw(0, 0);
        assert!(!h2.is_null());
    }

    #[test]
    fn handle_copy() {
        let h1: Handle<i32> = Handle::from_raw(42, 1);
        let h2 = h1;
        assert_eq!(h1, h2);
    }

    #[test]
    fn handle_eq() {
        let h1: Handle<i32> = Handle::from_raw(10, 2);
        let h2: Handle<i32> = Handle::from_raw(10, 2);
        let h3: Handle<i32> = Handle::from_raw(11, 2);
        let h4: Handle<i32> = Handle::from_raw(10, 3);
        
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert_ne!(h1, h4);
    }

    #[test]
    fn handle_hash() {
        use core::hash::{Hash, Hasher};
        
        let h1: Handle<i32> = Handle::from_raw(5, 2);
        let h2: Handle<i32> = Handle::from_raw(5, 2);
        
        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        
        h1.hash(&mut hasher1);
        h2.hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
