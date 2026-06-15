use super::handle::Handle;
use core::marker::PhantomData;

pub struct Arena<T> {
    items: Vec<T>,
    generations: Vec<u32>,
    free_indices: Vec<u32>,
    _marker: PhantomData<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            generations: Vec::new(),
            free_indices: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            generations: Vec::with_capacity(capacity),
            free_indices: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        if let Some(index) = self.free_indices.pop() {
            let generation = self.generations[index as usize];
            self.items[index as usize] = value;
            Handle::from_raw(index, generation)
        } else {
            let index = self.items.len() as u32;
            self.items.push(value);
            self.generations.push(1);
            Handle::from_raw(index, 1)
        }
    }

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

        self.generations[index] += 1;
        self.free_indices.push(index as u32);
        Some(core::mem::take(&mut self.items[index]))
    }

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

        Some(&self.items[index])
    }

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

        Some(&mut self.items[index])
    }

    pub fn len(&self) -> usize {
        self.items.len() - self.free_indices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.generations.clear();
        self.free_indices.clear();
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(Handle<T>, &T) -> bool,
    {
        let mut i = 0;
        while i < self.items.len() {
            let handle = Handle::from_raw(i as u32, self.generations[i]);
            if !f(handle, &self.items[i]) {
                self.remove(handle);
            } else {
                i += 1;
            }
        }
    }

    pub fn iter(&self) -> ArenaIter<'_, T> {
        ArenaIter {
            arena: self,
            index: 0,
        }
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ArenaIter<'a, T> {
    arena: &'a Arena<T>,
    index: usize,
}

impl<'a, T> Iterator for ArenaIter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.arena.items.len() {
            let idx = self.index;
            self.index += 1;
            
            if !self.arena.free_indices.contains(&(idx as u32)) {
                let handle = Handle::from_raw(idx as u32, self.arena.generations[idx]);
                return Some((handle, &self.arena.items[idx]));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_new() {
        let arena: Arena<i32> = Arena::new();
        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn arena_insert() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        assert!(!h.is_null());
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn arena_get() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        assert_eq!(arena.get(h), Some(&42));
    }

    #[test]
    fn arena_get_mut() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        *arena.get_mut(h).unwrap() = 100;
        assert_eq!(arena.get(h), Some(&100));
    }

    #[test]
    fn arena_remove() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        
        assert_eq!(arena.remove(h), Some(42));
        assert_eq!(arena.get(h), None);
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn arena_remove_twice() {
        let mut arena = Arena::new();
        let h = arena.insert(42);
        
        arena.remove(h);
        assert_eq!(arena.remove(h), None);
    }

    #[test]
    fn arena_reuse_index() {
        let mut arena = Arena::new();
        let h1 = arena.insert(1);
        arena.remove(h1);
        let h2 = arena.insert(2);
        
        assert_eq!(h1.index(), h2.index());
        assert_ne!(h1.generation(), h2.generation());
    }

    #[test]
    fn arena_iter() {
        let mut arena = Arena::new();
        let h1 = arena.insert(1);
        let h2 = arena.insert(2);
        let h3 = arena.insert(3);
        
        let mut items: Vec<(Handle<i32>, &i32)> = arena.iter().collect();
        items.sort_by_key(|(h, _)| h.index());
        
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].1, &1);
        assert_eq!(items[1].1, &2);
        assert_eq!(items[2].1, &3);
    }

    #[test]
    fn arena_retain() {
        let mut arena = Arena::new();
        arena.insert(1);
        arena.insert(2);
        arena.insert(3);
        arena.insert(4);
        
        arena.retain(|_, &val| val % 2 == 0);
        
        assert_eq!(arena.len(), 2);
        let values: Vec<i32> = arena.iter().map(|(_, v)| *v).collect();
        assert!(values.contains(&2));
        assert!(values.contains(&4));
    }
}
