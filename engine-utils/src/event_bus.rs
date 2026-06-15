use parking_lot::{RwLock, Mutex};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionHandle {
    id: u64,
}

impl SubscriptionHandle {
    pub const fn null() -> Self {
        Self { id: u64::MAX }
    }

    pub const fn is_null(&self) -> bool {
        self.id == u64::MAX
    }
}

pub struct EventBus<T: Clone + Send + Sync + 'static> {
    subscribers: Arc<RwLock<Vec<(SubscriptionHandle, Box<dyn Fn(T) + Send + Sync + 'static>)>>,
    next_id: Arc<Mutex<u64>>,
    queued_events: Arc<Mutex<Vec<T>>>,
}

impl<T: Clone + Send + Sync + 'static> EventBus<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(Mutex::new(0)),
            queued_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let id = {
            let mut next_id = self.next_id.lock();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let handle = SubscriptionHandle { id };
        self.subscribers.write().push((handle, Box::new(callback)));
        handle
    }

    pub fn unsubscribe(&self, handle: SubscriptionHandle) {
        let mut subscribers = self.subscribers.write();
        subscribers.retain(|(h, _)| h.id != handle.id);
    }

    pub fn send(&self, event: T) {
        let subscribers = self.subscribers.read();
        for (_, callback) in subscribers.iter() {
            callback(event.clone());
        }
    }

    pub fn queue(&self, event: T) {
        self.queued_events.lock().push(event);
    }

    pub fn drain(&mut self) {
        let mut queued = self.queued_events.lock();
        let events = std::mem::take(&mut *queued);
        
        for event in events {
            self.send(event);
        }
    }

    pub fn len(&self) -> usize {
        self.subscribers.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Clone + Send + Sync + 'static> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone, Debug, PartialEq)]
    struct TestEvent {
        value: i32,
    }

    impl TestEvent {
        fn new(value: i32) -> Self {
            Self { value }
        }
    }

    #[test]
    fn eventbus_new() {
        let bus = EventBus::<TestEvent>::new();
        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);
    }

    #[test]
    fn eventbus_subscribe() {
        let bus = EventBus::<TestEvent>::new();
        let handle = bus.subscribe(|_| {});
        assert!(!handle.is_null());
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn eventbus_unsubscribe() {
        let bus = EventBus::<TestEvent>::new();
        let handle = bus.subscribe(|_| {});
        assert_eq!(bus.len(), 1);
        
        bus.unsubscribe(handle);
        assert_eq!(bus.len(), 0);
    }

    #[test]
    fn eventbus_send() {
        let bus = EventBus::<TestEvent>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        bus.subscribe(move |event| {
            assert_eq!(event.value, 42);
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.send(TestEvent::new(42));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn eventbus_multiple_subscribers() {
        let bus = EventBus::<TestEvent>::new();
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));
        
        bus.subscribe({
            let c = counter1.clone();
            move |_| c.fetch_add(1, Ordering::SeqCst)
        });
        
        bus.subscribe({
            let c = counter2.clone();
            move |_| c.fetch_add(1, Ordering::SeqCst)
        });

        bus.send(TestEvent::new(10));
        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn eventbus_unsubscribe_after_send() {
        let bus = EventBus::<TestEvent>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let handle = bus.subscribe(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.send(TestEvent::new(1));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        
        bus.unsubscribe(handle);
        
        bus.send(TestEvent::new(2));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn eventbus_drain() {
        let mut bus = EventBus::<TestEvent>::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        bus.subscribe(move |event| {
            assert!(event.value == 1 || event.value == 2);
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.queue(TestEvent::new(1));
        bus.queue(TestEvent::new(2));
        
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        bus.drain();
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
}
