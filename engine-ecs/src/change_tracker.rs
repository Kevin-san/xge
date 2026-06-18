//! 变更追踪器 - 追踪组件变更

use crate::Component;
use std::any::TypeId;

/// Tick（帧标记）- 用于检测组件变更
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Tick(pub u32);

impl Tick {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn tick(&mut self) {
        self.0 += 1;
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// 组件变更记录
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentChange {
    pub tick: u32,
}

/// 变更追踪器
pub struct ChangeTrackers {
    trackers: std::collections::HashMap<TypeId, ComponentChange>,
}

impl ChangeTrackers {
    pub fn new() -> Self {
        Self {
            trackers: std::collections::HashMap::new(),
        }
    }

    pub fn record(&mut self, type_id: TypeId, tick: u32) {
        self.trackers.insert(type_id, ComponentChange { tick });
    }

    pub fn get(&self, type_id: TypeId) -> Option<u32> {
        self.trackers.get(&type_id).map(|c| c.tick)
    }

    pub fn has_changed(&self, type_id: TypeId, current_tick: u32) -> bool {
        self.get(type_id).map(|t| t != current_tick).unwrap_or(true)
    }
}

impl Default for ChangeTrackers {
    fn default() -> Self {
        Self::new()
    }
}

/// Ref - 组件引用（带变更检测）
pub struct Ref<'a, T: Component> {
    value: &'a T,
    tick: u32,
    changed_this_tick: bool,
}

impl<'a, T: Component> Ref<'a, T> {
    pub fn new(value: &'a T, tick: u32) -> Self {
        Self {
            value,
            tick,
            changed_this_tick: false,
        }
    }

    pub fn is_changed(&self) -> bool {
        self.changed_this_tick
    }

    pub fn changed_tick(&self) -> u32 {
        self.tick
    }
}

impl<'a, T: Component> std::ops::Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_increment() {
        let mut tick = Tick::new();
        assert_eq!(tick.get(), 0);

        tick.tick();
        assert_eq!(tick.get(), 1);

        tick.tick();
        assert_eq!(tick.get(), 2);
    }

    #[test]
    fn test_change_trackers_record() {
        let mut trackers = ChangeTrackers::new();
        let type_id = TypeId::of::<i32>();

        trackers.record(type_id, 1);
        assert_eq!(trackers.get(type_id), Some(1));

        trackers.record(type_id, 2);
        assert_eq!(trackers.get(type_id), Some(2));
    }

    #[test]
    fn test_change_trackers_has_changed() {
        let mut trackers = ChangeTrackers::new();
        let type_id = TypeId::of::<i32>();

        // First record
        trackers.record(type_id, 1);

        // Same tick - not changed
        assert!(!trackers.has_changed(type_id, 1));

        // Different tick - has changed
        assert!(trackers.has_changed(type_id, 2));

        // Unknown type - always changed
        let unknown_type = TypeId::of::<u32>();
        assert!(trackers.has_changed(unknown_type, 1));
    }
}
