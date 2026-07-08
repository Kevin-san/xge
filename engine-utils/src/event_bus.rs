use crate::Handle;
use std::collections::HashMap;
use core::sync::atomic::{AtomicU32, Ordering};
use parking_lot::RwLock;

type Callback<T> = Box<dyn Fn(&T) + Send + Sync + 'static>;

/// 主题式事件总线，支持跨线程派发
pub struct EventBus<T: Send + Sync + Clone> {
    subscriptions: RwLock<HashMap<Handle<Callback<T>>, Callback<T>>>,
    events: RwLock<Vec<T>>,
    next_id: AtomicU32,
}

impl<T: Send + Sync + Clone> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + Sync + Clone> EventBus<T> {
    pub fn new() -> Self {
        Self {
            subscriptions: RwLock::new(HashMap::new()),
            events: RwLock::new(Vec::new()),
            next_id: AtomicU32::new(0),
        }
    }

    pub fn subscribe<F>(&self, callback: F) -> Handle<Callback<T>>
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let handle = Handle::new(id, 0);
        self.subscriptions.write().insert(handle.clone(), Box::new(callback));
        handle
    }

    pub fn unsubscribe(&self, handle: Handle<Callback<T>>) {
        self.subscriptions.write().remove(&handle);
    }

    pub fn send(&self, event: T) {
        self.events.write().push(event);
    }

    pub fn drain(&mut self) -> Vec<T> {
        core::mem::take(&mut *self.events.write())
    }

    pub fn dispatch(&self) {
        let events = core::mem::take(&mut *self.events.write());
        let subscriptions = self.subscriptions.read();

        for event in events {
            for callback in subscriptions.values() {
                callback(&event);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.subscriptions.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.subscriptions.read().is_empty()
    }

    pub fn pending_events(&self) -> usize {
        self.events.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_subscribe_unsubscribe() {
        let bus = EventBus::<i32>::new();
        assert_eq!(bus.len(), 0);

        let handle = bus.subscribe(|_| {});
        assert_eq!(bus.len(), 1);

        bus.unsubscribe(handle);
        assert_eq!(bus.len(), 0);
    }

    #[test]
    fn test_send_dispatch() {
        let bus = EventBus::<i32>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        bus.subscribe(move |val| {
            c.fetch_add(*val as usize, Ordering::SeqCst);
        });

        bus.send(1);
        bus.send(2);
        bus.send(3);

        assert_eq!(bus.pending_events(), 3);
        bus.dispatch();
        assert_eq!(bus.pending_events(), 0);
        assert_eq!(counter.load(Ordering::SeqCst), 6);
    }

    #[test]
    fn test_multiple_subscribers() {
        let bus = EventBus::<i32>::new();
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));
        let c1 = counter1.clone();
        let c2 = counter2.clone();

        bus.subscribe(move |val| {
            c1.fetch_add(*val as usize, Ordering::SeqCst);
        });
        bus.subscribe(move |val| {
            c2.fetch_add(*val as usize * 2, Ordering::SeqCst);
        });

        bus.send(5);
        bus.dispatch();

        assert_eq!(counter1.load(Ordering::SeqCst), 5);
        assert_eq!(counter2.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_unsubscribe_after_send() {
        let bus = EventBus::<i32>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        let handle = bus.subscribe(move |val| {
            c.fetch_add(*val as usize, Ordering::SeqCst);
        });

        bus.send(1);
        bus.unsubscribe(handle);
        bus.send(2);
        bus.dispatch();

        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_drain() {
        let mut bus = EventBus::<i32>::new();
        bus.send(1);
        bus.send(2);
        bus.send(3);

        let events = bus.drain();
        assert_eq!(events, vec![1, 2, 3]);
        assert_eq!(bus.pending_events(), 0);
    }

    #[test]
    fn test_is_empty() {
        let bus = EventBus::<i32>::new();
        assert!(bus.is_empty());

        let _handle = bus.subscribe(|_| {});
        assert!(!bus.is_empty());
    }

    #[test]
    fn test_clone_event() {
        #[derive(Clone, Debug, PartialEq)]
        struct TestEvent {
            value: i32,
        }

        let bus = EventBus::<TestEvent>::new();
        let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        let r = received.clone();
        bus.subscribe(move |event| {
            r.lock().unwrap().push(event.clone());
        });

        bus.send(TestEvent { value: 42 });
        bus.dispatch();

        let events = received.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].value, 42);
    }

    #[test]
    fn test_thread_safety() {
        let bus = std::sync::Arc::new(EventBus::<i32>::new());
        let counter = std::sync::Arc::new(AtomicUsize::new(0));

        let c = counter.clone();
        bus.subscribe(move |val| {
            c.fetch_add(*val as usize, Ordering::SeqCst);
        });

        let mut handles = Vec::new();
        for i in 1..=10 {
            let b = bus.clone();
            handles.push(std::thread::spawn(move || {
                b.send(i);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        bus.dispatch();
        assert_eq!(counter.load(Ordering::SeqCst), 55);
    }
}