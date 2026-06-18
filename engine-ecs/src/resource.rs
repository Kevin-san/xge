//! 资源模块
//!
//! 定义 Resource trait 和资源管理。

use std::any::Any;
use std::collections::HashMap;

use crate::SystemParam;

/// 资源 trait
///
/// 资源是不属于实体的全局数据。
pub trait Resource: Any + Send + Sync + 'static {}

/// 资源存储
pub struct Resources {
    /// 资源数据
    resources: HashMap<std::any::TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    /// 创建新的资源存储
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// 插入资源
    pub fn insert<R: Resource>(&mut self, resource: R) {
        let type_id = std::any::TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
    }

    /// 获取资源引用
    pub fn get<R: Resource>(&self) -> Option<&R> {
        let type_id = std::any::TypeId::of::<R>();
        self.resources
            .get(&type_id)
            .and_then(|r| r.downcast_ref::<R>())
    }

    /// 获取资源可变引用
    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        let type_id = std::any::TypeId::of::<R>();
        self.resources
            .get_mut(&type_id)
            .and_then(|r| r.downcast_mut::<R>())
    }

    /// 移除资源
    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        let type_id = std::any::TypeId::of::<R>();
        self.resources
            .remove(&type_id)
            .and_then(|r| r.downcast::<R>().ok().map(|boxed| *boxed))
    }

    /// 检查资源是否存在
    pub fn contains<R: Resource>(&self) -> bool {
        let type_id = std::any::TypeId::of::<R>();
        self.resources.contains_key(&type_id)
    }

    /// 清空所有资源
    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

// 为所有满足条件的类型实现 Resource
impl<R: Any + Send + Sync + 'static> Resource for R {}

impl SystemParam for Resources {
    type Item<'a> = &'a Resources;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Timer {
        elapsed: f32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Config {
        name: String,
    }

    #[test]
    fn test_resources_insert_get() {
        let mut resources = Resources::new();
        resources.insert(Timer { elapsed: 0.0 });

        let timer = resources.get::<Timer>();
        assert!(timer.is_some());
        assert_eq!(timer.unwrap().elapsed, 0.0);
    }

    #[test]
    fn test_resources_remove() {
        let mut resources = Resources::new();
        resources.insert(Timer { elapsed: 5.0 });

        let removed = resources.remove::<Timer>();
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().elapsed, 5.0);

        assert!(resources.get::<Timer>().is_none());
    }

    #[test]
    fn test_resources_contains() {
        let mut resources = Resources::new();
        resources.insert(Timer { elapsed: 0.0 });

        assert!(resources.contains::<Timer>());
        assert!(!resources.contains::<Config>());
    }

    #[test]
    fn test_resources_overwrite() {
        let mut resources = Resources::new();
        resources.insert(Timer { elapsed: 0.0 });
        resources.insert(Timer { elapsed: 10.0 });

        let timer = resources.get::<Timer>();
        assert!(timer.is_some());
        assert_eq!(timer.unwrap().elapsed, 10.0);
    }
}
