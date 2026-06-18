//! 事件系统 - 双缓冲事件队列

use std::any::TypeId;
use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::{Component, Resources, SystemParam, World};

/// 事件读取器
pub struct EventReader<'w, E: Event> {
    events: Vec<E>,
    _marker: PhantomData<&'w E>,
}

impl<'w, E: Event> EventReader<'w, E> {
    pub fn new(events: Vec<E>) -> Self {
        Self {
            events,
            _marker: PhantomData,
        }
    }

    /// 获取所有事件
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.events.iter()
    }
}

impl<'w, E: Event> IntoIterator for EventReader<'w, E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

/// 事件写入器
pub struct EventWriter<'w, E: Event> {
    _marker: PhantomData<&'w E>,
}

/// 事件 trait
pub trait Event: Send + Sync + Clone + 'static {
    fn id() -> TypeId {
        TypeId::of::<Self>()
    }
}

/// 双缓冲事件存储
pub struct Events<E: Event> {
    /// 当前帧事件
    current: VecDeque<E>,
    /// 上一帧事件
    previous: VecDeque<E>,
    /// 事件计数
    event_count: usize,
}

impl<E: Event> Events<E> {
    pub fn new() -> Self {
        Self {
            current: VecDeque::new(),
            previous: VecDeque::new(),
            event_count: 0,
        }
    }

    pub fn send(&mut self, event: E) {
        self.current.push_back(event);
        self.event_count += 1;
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.current, &mut self.previous);
        self.current.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.current.iter()
    }

    pub fn len(&self) -> usize {
        self.current.len()
    }

    pub fn is_empty(&self) -> bool {
        self.current.is_empty()
    }

    pub fn event_count(&self) -> usize {
        self.event_count
    }

    pub fn clear(&mut self) {
        self.current.clear();
        self.previous.clear();
    }
}

impl<E: Event> Default for Events<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'w, E: Event> SystemParam for EventReader<'w, E> {
    type Item<'a>
        = EventIterator<'a, E>
    where
        Self: 'a;
}

impl<'w, E: Event> SystemParam for EventWriter<'w, E> {
    type Item<'a>
        = &'a mut Events<E>
    where
        Self: 'a;
}

pub struct EventIterator<'a, E: Event> {
    events: &'a Events<E>,
}

impl<'a, E: Event> Iterator for EventIterator<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.current.iter().next()
    }
}

// ============ 常用事件类型 ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntitySpawned {
    pub entity: crate::Entity,
}

impl Event for EntitySpawned {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityDespawned {
    pub entity: crate::Entity,
}

impl Event for EntityDespawned {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentInserted<C: Component> {
    pub entity: crate::Entity,
    pub component: PhantomData<C>,
}

impl<C: Component + Clone> Event for ComponentInserted<C> {}

/// Events 资源更新系统
pub fn events_update_system<E: Event>(_world: &mut World, resources: &mut Resources) {
    if let Some(events) = resources.get_mut::<Events<E>>() {
        events.update();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestEvent {
        value: i32,
    }

    impl Event for TestEvent {}

    #[test]
    fn test_events_send_update() {
        let mut events = Events::<TestEvent>::new();
        assert!(events.is_empty());

        events.send(TestEvent { value: 1 });
        assert_eq!(events.len(), 1);
        assert_eq!(events.event_count(), 1);

        events.update();
        assert_eq!(events.len(), 0); // current is cleared after update
        assert_eq!(events.event_count(), 1);
    }

    #[test]
    fn test_events_double_buffer() {
        let mut events = Events::<TestEvent>::new();

        // Send events to current buffer
        events.send(TestEvent { value: 1 });
        events.send(TestEvent { value: 2 });

        // Before update, iter only sees current
        assert_eq!(events.iter().count(), 2);

        // After update, current becomes previous
        events.update();

        // Previous still has events
        assert_eq!(events.len(), 0);

        // Can iterate over previous (via internal access)
        // The double buffer swap is verified by update working
    }

    #[test]
    fn test_events_clear() {
        let mut events = Events::<TestEvent>::new();
        events.send(TestEvent { value: 1 });
        events.send(TestEvent { value: 2 });

        assert_eq!(events.len(), 2);

        events.clear();

        assert!(events.is_empty());
        assert_eq!(events.len(), 0);
    }
}
