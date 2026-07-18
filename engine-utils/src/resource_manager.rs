use crate::AssetId;
use alloc::collections::BTreeMap;

/// 通用资源管理器
pub struct ResourceManager<T> {
    resources: BTreeMap<AssetId, T>,
}

impl<T> Default for ResourceManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ResourceManager<T> {
    pub fn new() -> Self {
        Self {
            resources: BTreeMap::new(),
        }
    }

    /// 加载资源
    pub fn load(&mut self, id: AssetId, value: T) -> Option<T> {
        self.resources.insert(id, value)
    }

    /// 获取资源
    pub fn get(&self, id: &AssetId) -> Option<&T> {
        self.resources.get(id)
    }

    /// 获取可变引用
    pub fn get_mut(&mut self, id: &AssetId) -> Option<&mut T> {
        self.resources.get_mut(id)
    }

    /// 卸载资源
    pub fn unload(&mut self, id: &AssetId) -> Option<T> {
        self.resources.remove(id)
    }

    /// 检查资源是否存在
    pub fn contains(&self, id: &AssetId) -> bool {
        self.resources.contains_key(id)
    }

    /// 重新加载资源（替换已有值，返回旧值）
    pub fn reload(&mut self, id: AssetId, value: T) -> Option<T> {
        self.resources.insert(id, value)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_get() {
        let mut rm = ResourceManager::new();
        let id = AssetId::null();
        rm.load(id, "test".to_string());
        assert_eq!(rm.get(&id), Some(&"test".to_string()));
    }

    #[test]
    fn test_unload() {
        let mut rm = ResourceManager::new();
        let id = AssetId::null();
        rm.load(id, 42);
        assert_eq!(rm.unload(&id), Some(42));
        assert!(rm.get(&id).is_none());
    }

    #[test]
    fn test_replace() {
        let mut rm = ResourceManager::new();
        let id = AssetId::null();
        rm.load(id, 1);
        let old = rm.load(id, 2);
        assert_eq!(old, Some(1));
        assert_eq!(rm.get(&id), Some(&2));
    }

    #[test]
    fn test_reload() {
        let mut rm = ResourceManager::new();
        let id = AssetId::new(42);
        rm.load(id, "v1");
        let old = rm.reload(id, "v2");
        assert_eq!(old, Some("v1"));
        assert_eq!(rm.get(&id), Some(&"v2"));
    }

    #[test]
    fn test_contains() {
        let mut rm = ResourceManager::new();
        let id = AssetId::new(1);
        assert!(!rm.contains(&id));
        rm.load(id, 100);
        assert!(rm.contains(&id));
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut rm = ResourceManager::new();
        assert!(rm.is_empty());
        assert_eq!(rm.len(), 0);

        rm.load(AssetId::new(1), "a");
        rm.load(AssetId::new(2), "b");
        assert_eq!(rm.len(), 2);
        assert!(!rm.is_empty());
    }
}
