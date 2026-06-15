use super::asset_id::AssetId;
use std::collections::HashMap;

pub struct ResourceManager<T> {
    resources: HashMap<AssetId, T>,
}

impl<T> ResourceManager<T> {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn load(&mut self, id: AssetId, value: T) -> Option<T> {
        self.resources.insert(id, value)
    }

    pub fn get(&self, id: &AssetId) -> Option<&T> {
        self.resources.get(id)
    }

    pub fn get_mut(&mut self, id: &AssetId) -> Option<&mut T> {
        self.resources.get_mut(id)
    }

    pub fn unload(&mut self, id: &AssetId) -> Option<T> {
        self.resources.remove(id)
    }

    pub fn contains(&self, id: &AssetId) -> bool {
        self.resources.contains_key(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &T)> {
        self.resources.iter()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

impl<T> Default for ResourceManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rm_new() {
        let rm: ResourceManager<String> = ResourceManager::new();
        assert!(rm.is_empty());
        assert_eq!(rm.len(), 0);
    }

    #[test]
    fn rm_load() {
        let mut rm = ResourceManager::new();
        let id = AssetId::from_path("test.txt");
        let old = rm.load(id, "hello".to_string());
        
        assert!(old.is_none());
        assert_eq!(rm.len(), 1);
        assert!(rm.contains(&id));
    }

    #[test]
    fn rm_load_replace() {
        let mut rm = ResourceManager::new();
        let id = AssetId::from_path("test.txt");
        
        rm.load(id, "first".to_string());
        let old = rm.load(id, "second".to_string());
        
        assert_eq!(old, Some("first".to_string()));
        assert_eq!(rm.get(&id), Some(&"second".to_string()));
    }

    #[test]
    fn rm_get() {
        let mut rm = ResourceManager::new();
        let id = AssetId::from_path("test.txt");
        
        rm.load(id, 42);
        assert_eq!(rm.get(&id), Some(&42));
        assert_eq!(rm.get(&AssetId::null()), None);
    }

    #[test]
    fn rm_get_mut() {
        let mut rm = ResourceManager::new();
        let id = AssetId::from_path("test.txt");
        
        rm.load(id, 42);
        *rm.get_mut(&id).unwrap() = 100;
        assert_eq!(rm.get(&id), Some(&100));
    }

    #[test]
    fn rm_unload() {
        let mut rm = ResourceManager::new();
        let id = AssetId::from_path("test.txt");
        
        rm.load(id, 42);
        let value = rm.unload(&id);
        
        assert_eq!(value, Some(42));
        assert!(!rm.contains(&id));
    }

    #[test]
    fn rm_iter() {
        let mut rm = ResourceManager::new();
        let id1 = AssetId::from_path("a.txt");
        let id2 = AssetId::from_path("b.txt");
        
        rm.load(id1, 1);
        rm.load(id2, 2);
        
        let values: Vec<i32> = rm.iter().map(|(_, v)| *v).collect();
        assert!(values.contains(&1));
        assert!(values.contains(&2));
    }
}
