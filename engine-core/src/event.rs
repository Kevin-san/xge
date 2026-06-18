// 事件总线模块
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type SubscriptionHandle = usize;

/// Callback type alias
type Callback<T> = Arc<dyn Fn(T) + Send + Sync>;

/// Subscriber map type alias
type SubscriberMap<T> = HashMap<SubscriptionHandle, Callback<T>>;

/// 事件总线
///
/// 提供类型安全的发布-订阅模式
///
/// # Example
/// ```ignore
/// use engine_core::EventBus;
///
/// let mut bus = EventBus::<String>::new();
///
/// let handle = bus.subscribe(|msg| {
///     println!("Received: {}", msg);
/// });
///
/// bus.send("Hello".to_string());
/// bus.unsubscribe(handle);
/// ```
pub struct EventBus<T: Clone + Send + Sync + 'static> {
    subscribers: Arc<Mutex<SubscriberMap<T>>>,
    next_handle: Arc<Mutex<usize>>,
}

impl<T: Clone + Send + Sync + 'static> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + Sync + 'static> EventBus<T> {
    /// 创建新的事件总线
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            next_handle: Arc::new(Mutex::new(0)),
        }
    }

    /// 订阅事件
    ///
    /// 返回一个 handle，可用于取消订阅
    pub fn subscribe<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let mut handle_counter = self.next_handle.lock().unwrap();
        let handle = *handle_counter;
        *handle_counter += 1;

        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.insert(handle, Arc::new(callback));

        handle
    }

    /// 取消订阅
    pub fn unsubscribe(&self, handle: SubscriptionHandle) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.remove(&handle);
    }

    /// 发送事件
    ///
    /// 所有订阅者都会收到事件
    pub fn send(&self, event: T) {
        // 复制订阅者列表，在锁外调用回调
        let callbacks: Vec<_> = {
            let subscribers = self.subscribers.lock().unwrap();
            subscribers.values().cloned().collect()
        };

        // 调用所有回调
        for callback in callbacks {
            callback(event.clone());
        }
    }

    /// 清空所有订阅
    pub fn drain(&self) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.clear();
        let mut handle_counter = self.next_handle.lock().unwrap();
        *handle_counter = 0;
    }

    /// 获取订阅者数量
    pub fn subscriber_count(&self) -> usize {
        let subscribers = self.subscribers.lock().unwrap();
        subscribers.len()
    }

    /// 克隆事件总线（共享订阅者）
    pub fn clone_shared(&self) -> Self {
        Self {
            subscribers: self.subscribers.clone(),
            next_handle: self.next_handle.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_subscribe_and_send() {
        let bus = EventBus::<String>::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let received_clone = received.clone();
        let handle = bus.subscribe(move |msg| {
            received_clone.lock().unwrap().push(msg);
        });

        bus.send("Hello".to_string());
        bus.send("World".to_string());

        assert_eq!(*received.lock().unwrap(), vec!["Hello", "World"]);
        bus.unsubscribe(handle);
    }

    #[test]
    fn test_unsubscribe() {
        let bus = EventBus::<String>::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let received_clone = received.clone();
        let handle = bus.subscribe(move |msg| {
            received_clone.lock().unwrap().push(msg);
        });

        bus.send("Before".to_string());
        bus.unsubscribe(handle);
        bus.send("After".to_string());

        assert_eq!(*received.lock().unwrap(), vec!["Before"]);
    }

    #[test]
    fn test_multiple_subscribers() {
        let bus = EventBus::<i32>::new();
        let count1 = Arc::new(Mutex::new(0));
        let count2 = Arc::new(Mutex::new(0));

        let c1 = count1.clone();
        bus.subscribe(move |_| {
            *c1.lock().unwrap() += 1;
        });

        let c2 = count2.clone();
        bus.subscribe(move |_| {
            *c2.lock().unwrap() += 1;
        });

        bus.send(42);

        assert_eq!(*count1.lock().unwrap(), 1);
        assert_eq!(*count2.lock().unwrap(), 1);
    }

    #[test]
    fn test_drain() {
        let bus = EventBus::<String>::new();

        bus.subscribe(|_| {});
        bus.subscribe(|_| {});

        assert_eq!(bus.subscriber_count(), 2);

        bus.drain();

        assert_eq!(bus.subscriber_count(), 0);
    }
}
