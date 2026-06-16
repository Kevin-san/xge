//! 事件模块
//!
//! 定义 Event trait 和事件读写器。

use std::any::Any;
use std::collections::VecDeque;

/// Event trait
///
/// 所有事件类型必须实现此 trait。
pub trait Event: Any + Send + Sync + Clone + 'static {
    /// 获取事件类型名称
    fn event_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// 事件读取器
pub struct EventReader<E: Event> {
    events: Vec<E>,
    index: usize,
}

impl<E: Event> EventReader<E> {
    /// 创建新的事件读取器
    pub fn new(events: Vec<E>) -> Self {
        Self { events, index: 0 }
    }

    /// 获取下一个事件
    pub fn read(&mut self) -> Option<&E> {
        if self.index < self.events.len() {
            let event = &self.events[self.index];
            self.index += 1;
            Some(event)
        } else {
            None
        }
    }

    /// 获取所有剩余事件
    pub fn read_all(&mut self) -> &[E] {
        let remaining = &self.events[self.index..];
        self.index = self.events.len();
        remaining
    }

    /// 检查是否还有事件
    pub fn is_empty(&self) -> bool {
        self.index >= self.events.len()
    }

    /// 获取已读事件数量
    pub fn len(&self) -> usize {
        self.events.len() - self.index
    }
}

/// 事件写入器
pub struct EventWriter<E: Event> {
    events: VecDeque<E>,
}

impl<E: Event> EventWriter<E> {
    /// 创建新的事件写入器
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    /// 发送事件
    pub fn send(&mut self, event: E) {
        self.events.push_back(event);
    }

    /// 获取事件数量
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// 获取所有事件并清空
    pub fn drain(&mut self) -> Vec<E> {
        self.events.drain(..).collect()
    }
}

impl<E: Event> Default for EventWriter<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// 事件系统
pub struct Events<E: Event> {
    writers: Vec<EventWriter<E>>,
}

impl<E: Event> Events<E> {
    /// 创建新的事件系统
    pub fn new() -> Self {
        Self {
            writers: Vec::new(),
        }
    }

    /// 添加写入器
    pub fn add_writer(&mut self) -> &mut EventWriter<E> {
        self.writers.push(EventWriter::new());
        self.writers.last_mut().unwrap()
    }

    /// 获取所有事件
    pub fn get_events(&self) -> Vec<E> {
        self.writers
            .iter()
            .flat_map(|w| w.events.iter().cloned())
            .collect()
    }

    /// 清空所有事件
    pub fn clear(&mut self) {
        for writer in &mut self.writers {
            writer.events.clear();
        }
    }
}

impl<E: Event> Default for Events<E> {
    fn default() -> Self {
        Self::new()
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
    fn test_event_writer_send() {
        let mut writer = EventWriter::new();
        writer.send(TestEvent { value: 1 });
        writer.send(TestEvent { value: 2 });

        assert_eq!(writer.len(), 2);
    }

    #[test]
    fn test_event_reader_read() {
        let events = vec![TestEvent { value: 1 }, TestEvent { value: 2 }];
        let mut reader = EventReader::new(events);

        assert_eq!(reader.read().unwrap().value, 1);
        assert_eq!(reader.read().unwrap().value, 2);
        assert!(reader.read().is_none());
    }

    #[test]
    fn test_event_reader_read_all() {
        let events = vec![
            TestEvent { value: 1 },
            TestEvent { value: 2 },
            TestEvent { value: 3 },
        ];
        let mut reader = EventReader::new(events);

        let remaining = reader.read_all();
        assert_eq!(remaining.len(), 3);
        assert!(reader.read().is_none());
    }

    #[test]
    fn test_event_reader_is_empty() {
        let events = vec![TestEvent { value: 1 }];
        let mut reader = EventReader::new(events);
        
        assert!(!reader.is_empty());
        reader.read();
        assert!(reader.is_empty());
    }

    #[test]
    fn test_event_reader_len() {
        let events = vec![TestEvent { value: 1 }, TestEvent { value: 2 }];
        let mut reader = EventReader::new(events);
        
        assert_eq!(reader.len(), 2);
        reader.read();
        assert_eq!(reader.len(), 1);
        reader.read();
        assert_eq!(reader.len(), 0);
    }

    #[test]
    fn test_event_reader_empty() {
        let events: Vec<TestEvent> = vec![];
        let mut reader = EventReader::new(events);
        
        assert!(reader.is_empty());
        assert!(reader.read().is_none());
        assert_eq!(reader.len(), 0);
    }

    #[test]
    fn test_event_writer_is_empty() {
        let mut writer = EventWriter::new();
        assert!(writer.is_empty());
        
        writer.send(TestEvent { value: 1 });
        assert!(!writer.is_empty());
    }

    #[test]
    fn test_event_writer_drain() {
        let mut writer = EventWriter::new();
        writer.send(TestEvent { value: 1 });
        writer.send(TestEvent { value: 2 });
        
        let drained = writer.drain();
        assert_eq!(drained.len(), 2);
        assert!(writer.is_empty());
    }

    #[test]
    fn test_event_writer_default() {
        let writer = EventWriter::<TestEvent>::default();
        assert!(writer.is_empty());
    }

    #[test]
    fn test_events_new() {
        let events = Events::<TestEvent>::new();
        assert!(events.get_events().is_empty());
    }

    #[test]
    fn test_events_default() {
        let events = Events::<TestEvent>::default();
        assert!(events.get_events().is_empty());
    }

    #[test]
    fn test_events_add_writer() {
        let mut events = Events::<TestEvent>::new();
        let writer = events.add_writer();
        writer.send(TestEvent { value: 1 });
        
        let all_events = events.get_events();
        assert_eq!(all_events.len(), 1);
    }

    #[test]
    fn test_events_multiple_writers() {
        let mut events = Events::<TestEvent>::new();
        
        let writer1 = events.add_writer();
        writer1.send(TestEvent { value: 1 });
        writer1.send(TestEvent { value: 2 });
        
        let writer2 = events.add_writer();
        writer2.send(TestEvent { value: 3 });
        
        let all_events = events.get_events();
        assert_eq!(all_events.len(), 3);
    }

    #[test]
    fn test_events_clear() {
        let mut events = Events::<TestEvent>::new();
        let writer = events.add_writer();
        writer.send(TestEvent { value: 1 });
        writer.send(TestEvent { value: 2 });
        
        events.clear();
        assert!(events.get_events().is_empty());
    }

    #[test]
    fn test_event_name() {
        assert!(TestEvent::event_name().contains("TestEvent"));
    }

    #[test]
    fn test_reader_partial_read() {
        let events = vec![
            TestEvent { value: 1 },
            TestEvent { value: 2 },
            TestEvent { value: 3 },
            TestEvent { value: 4 },
        ];
        let mut reader = EventReader::new(events);
        
        // Read first two
        reader.read();
        reader.read();
        
        // Read remaining
        let remaining = reader.read_all();
        assert_eq!(remaining.len(), 2);
        assert_eq!(remaining[0].value, 3);
        assert_eq!(remaining[1].value, 4);
    }

    #[test]
    fn test_writer_multiple_drains() {
        let mut writer = EventWriter::new();
        
        writer.send(TestEvent { value: 1 });
        let first_drain = writer.drain();
        assert_eq!(first_drain.len(), 1);
        
        writer.send(TestEvent { value: 2 });
        writer.send(TestEvent { value: 3 });
        let second_drain = writer.drain();
        assert_eq!(second_drain.len(), 2);
        
        assert!(writer.is_empty());
    }

    #[test]
    fn test_events_order_preserved() {
        let mut events = Events::<TestEvent>::new();
        let writer = events.add_writer();
        
        writer.send(TestEvent { value: 1 });
        writer.send(TestEvent { value: 2 });
        writer.send(TestEvent { value: 3 });
        
        let all_events = events.get_events();
        assert_eq!(all_events[0].value, 1);
        assert_eq!(all_events[1].value, 2);
        assert_eq!(all_events[2].value, 3);
    }
}
