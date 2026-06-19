//! 事件总线 — 同时支持：
//! - **队列式**（send → iter）：适合系统读取、事件缓冲、批处理
//! - **回调式**（subscribe → callback）：适合全局钩子、日志、遥测

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type SubscriptionHandle = usize;

/// 回调类型别名
type Callback<T> = Arc<dyn Fn(&T) + Send + Sync>;

/// 订阅者映射
type SubscriberMap<T> = HashMap<SubscriptionHandle, Callback<T>>;

/// 事件队列
type EventQueue<T> = Vec<T>;

/// 全局事件总线
///
/// # 两种读取模式
///
/// **队列式**（推荐用于游戏系统）：
/// ```ignore
/// bus.send(MyEvent::Spawn(42));
/// for event in bus.iter() {
///     println!("{:?}", event);
/// }
/// bus.drain_queue(); // 清空已处理事件
/// ```
///
/// **回调式**（推荐用于全局钩子）：
/// ```ignore
/// let handle = bus.subscribe(|event| {
///     log_event(event);
/// });
/// bus.send(MyEvent::Tick);
/// bus.unsubscribe(handle);
/// ```
pub struct EventBus<T: Clone + Send + Sync + 'static> {
    subscribers: Arc<Mutex<SubscriberMap<T>>>,
    next_handle: Arc<Mutex<SubscriptionHandle>>,
    queue: Arc<Mutex<EventQueue<T>>>,
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
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // ========== 订阅 / 取消订阅（回调式） ==========

    /// 订阅事件（回调式）
    pub fn subscribe<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(&T) + Send + Sync + 'static,
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

    /// 订阅者数量
    pub fn subscriber_count(&self) -> usize {
        let subscribers = self.subscribers.lock().unwrap();
        subscribers.len()
    }

    // ========== 发送 / 读取（队列式） ==========

    /// 发送事件
    ///
    /// 事件会同时：
    /// 1. 追加到内部事件队列，供后续 `iter()` / `drain()` 读取
    /// 2. 立即广播给所有已订阅的回调
    pub fn send(&self, event: T) {
        // 先入队列，再回调（避免回调内部 panic 导致丢失事件）
        {
            let mut queue = self.queue.lock().unwrap();
            queue.push(event.clone());
        }

        // 调用所有回调
        let callbacks: Vec<_> = {
            let subscribers = self.subscribers.lock().unwrap();
            subscribers.values().cloned().collect()
        };
        for callback in callbacks {
            callback(&event);
        }
    }

    /// 批量发送事件
    pub fn send_batch<I>(&self, events: I)
    where
        I: IntoIterator<Item = T>,
    {
        for event in events {
            self.send(event);
        }
    }

    /// 获取当前队列中的事件快照
    ///
    /// 返回一个副本，不会清空队列
    pub fn snapshot(&self) -> Vec<T> {
        let queue = self.queue.lock().unwrap();
        queue.clone()
    }

    /// 迭代当前队列中的事件（只读，不移除）
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.snapshot().into_iter()
    }

    /// 当前队列长度
    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    /// 队列是否为空
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }

    // ========== 清空 ==========

    /// 清空事件队列
    pub fn drain_queue(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.clear();
    }

    /// 清空订阅者（保留事件队列）
    pub fn clear_subscribers(&self) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.clear();
        let mut counter = self.next_handle.lock().unwrap();
        *counter = 0;
    }

    /// 完全清空：移除所有订阅者 + 清空所有事件
    pub fn drain(&self) {
        self.drain_queue();
        self.clear_subscribers();
    }

    // ========== 克隆 / 共享 ==========

    /// 克隆事件总线（共享内部状态，多线程可共用）
    pub fn clone_shared(&self) -> Self {
        Self {
            subscribers: self.subscribers.clone(),
            next_handle: self.next_handle.clone(),
            queue: self.queue.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEvent {
        value: i32,
    }

    // ===== 回调式测试 =====

    #[test]
    fn test_subscribe_and_send_callback() {
        let bus = EventBus::<TestEvent>::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let received_clone = received.clone();
        let handle = bus.subscribe(move |msg| {
            received_clone.lock().unwrap().push(msg.value);
        });

        bus.send(TestEvent { value: 10 });
        bus.send(TestEvent { value: 20 });

        assert_eq!(*received.lock().unwrap(), vec![10, 20]);
        bus.unsubscribe(handle);
    }

    #[test]
    fn test_unsubscribe_stops_callback() {
        let bus = EventBus::<TestEvent>::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let received_clone = received.clone();
        let handle = bus.subscribe(move |msg| {
            received_clone.lock().unwrap().push(msg.value);
        });

        bus.send(TestEvent { value: 1 });
        bus.unsubscribe(handle);
        bus.send(TestEvent { value: 2 });

        assert_eq!(*received.lock().unwrap(), vec![1]);
    }

    #[test]
    fn test_multiple_subscribers_callbacks() {
        let bus = EventBus::<TestEvent>::new();
        let count1 = Arc::new(Mutex::new(0));
        let count2 = Arc::new(Mutex::new(0));

        let c1 = count1.clone();
        bus.subscribe(move |_| {
            let mut guard = c1.lock().unwrap();
            *guard += 1;
        });

        let c2 = count2.clone();
        bus.subscribe(move |_| {
            let mut guard = c2.lock().unwrap();
            *guard += 1;
        });

        bus.send(TestEvent { value: 42 });

        assert_eq!(*count1.lock().unwrap(), 1);
        assert_eq!(*count2.lock().unwrap(), 1);
    }

    // ===== 队列式测试 =====

    #[test]
    fn test_send_and_iter_queue() {
        let bus = EventBus::<TestEvent>::new();
        bus.send(TestEvent { value: 1 });
        bus.send(TestEvent { value: 2 });
        bus.send(TestEvent { value: 3 });

        assert_eq!(bus.len(), 3);
        let values: Vec<_> = bus.iter().map(|e| e.value).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_queue_snapshot_is_clone() {
        let bus = EventBus::<TestEvent>::new();
        bus.send(TestEvent { value: 1 });
        let snap1 = bus.snapshot();
        bus.send(TestEvent { value: 2 });
        let snap2 = bus.snapshot();

        assert_eq!(snap1.len(), 1);
        assert_eq!(snap2.len(), 2);
    }

    #[test]
    fn test_is_empty_and_len() {
        let bus = EventBus::<TestEvent>::new();
        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);

        bus.send(TestEvent { value: 1 });
        assert!(!bus.is_empty());
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn test_drain_queue_preserves_subscribers() {
        let bus = EventBus::<TestEvent>::new();
        let count = Arc::new(Mutex::new(0));

        let c = count.clone();
        bus.subscribe(move |_| {
            let mut guard = c.lock().unwrap();
            *guard += 1;
        });

        bus.send(TestEvent { value: 1 });
        bus.send(TestEvent { value: 2 });
        bus.drain_queue();

        assert_eq!(bus.len(), 0);
        assert_eq!(bus.subscriber_count(), 1);
        // 再发送一次，回调仍然工作
        bus.send(TestEvent { value: 3 });
        assert_eq!(*count.lock().unwrap(), 3);
    }

    // ===== 双模式并行 =====

    #[test]
    fn test_both_modes_work_side_by_side() {
        let bus = EventBus::<TestEvent>::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let received_clone = received.clone();
        bus.subscribe(move |e| {
            received_clone.lock().unwrap().push(e.value);
        });

        bus.send(TestEvent { value: 100 });
        bus.send(TestEvent { value: 200 });

        // 队列可用
        assert_eq!(bus.len(), 2);
        let queued: Vec<_> = bus.iter().map(|e| e.value).collect();
        assert_eq!(queued, vec![100, 200]);

        // 回调也可用
        assert_eq!(*received.lock().unwrap(), vec![100, 200]);
    }

    #[test]
    fn test_send_batch() {
        let bus = EventBus::<TestEvent>::new();
        let events = vec![
            TestEvent { value: 1 },
            TestEvent { value: 2 },
            TestEvent { value: 3 },
        ];
        bus.send_batch(events);
        assert_eq!(bus.len(), 3);
    }

    #[test]
    fn test_drain_all() {
        let bus = EventBus::<TestEvent>::new();
        bus.subscribe(|_| {});
        bus.send(TestEvent { value: 1 });

        bus.drain();
        assert_eq!(bus.len(), 0);
        assert_eq!(bus.subscriber_count(), 0);
    }

    #[test]
    fn test_clone_shared() {
        let bus = EventBus::<TestEvent>::new();
        bus.send(TestEvent { value: 1 });

        let shared = bus.clone_shared();
        shared.send(TestEvent { value: 2 });

        // 两者共享同一个队列和订阅者表
        assert_eq!(bus.len(), 2);
        assert_eq!(shared.len(), 2);
    }

    #[test]
    fn test_default_constructs() {
        let bus = EventBus::<String>::default();
        bus.send("hello".to_string());
        assert_eq!(bus.len(), 1);
    }
}
