//! 信号系统 - 节点间事件通信

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 信号处理器 ID
pub type HandlerId = usize;

/// 信号处理器类型
type SignalHandler = Box<dyn Fn(&[&dyn Any])>;

/// 信号
#[derive(Clone)]
pub struct Signal {
    name: String,
    handlers: Vec<(HandlerId, Rc<RefCell<SignalHandler>>)>,
    next_id: usize,
}

impl std::fmt::Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("name", &self.name)
            .field("handlers.len()", &self.handlers.len())
            .field("next_id", &self.next_id)
            .finish()
    }
}

impl Signal {
    /// 创建新的信号
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            handlers: Vec::new(),
            next_id: 0,
        }
    }

    /// 连接处理器
    pub fn connect(&mut self, handler: SignalHandler) -> HandlerId {
        let id = self.next_id;
        self.next_id += 1;
        self.handlers.push((id, Rc::new(RefCell::new(handler))));
        id
    }

    /// 断开连接
    pub fn disconnect(&mut self, id: HandlerId) {
        self.handlers.retain(|(h_id, _)| *h_id != id);
    }

    /// 发送信号
    pub fn emit(&self, args: &[&dyn Any]) {
        for (_, handler) in &self.handlers {
            let handler = handler.borrow();
            handler(args);
        }
    }
}

/// 信号总线
#[derive(Debug, Default)]
pub struct SignalBus {
    signals: HashMap<String, Signal>,
}

impl SignalBus {
    /// 创建新的信号总线
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    /// 获取或创建信号
    pub fn get_or_create(&mut self, name: &str) -> &mut Signal {
        self.signals
            .entry(name.to_string())
            .or_insert_with(|| Signal::new(name))
    }

    /// 发送信号
    pub fn emit(&mut self, name: &str, args: &[&dyn Any]) {
        if let Some(signal) = self.signals.get_mut(name) {
            signal.emit(args);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_connect_emit() {
        let mut signal = Signal::new("test");
        let received = Rc::new(RefCell::new(Vec::new()));

        let received_clone = received.clone();
        let id = signal.connect(Box::new(move |args| {
            if let Some(v) = args.first() {
                if let Some(i) = v.downcast_ref::<i32>() {
                    received_clone.borrow_mut().push(*i);
                }
            }
        }));

        signal.emit(&[&5i32]);
        assert_eq!(*received.borrow(), vec![5]);

        let _ = id;
    }

    #[test]
    fn test_signal_disconnect() {
        let mut signal = Signal::new("test");
        let called = Rc::new(RefCell::new(false));

        let called_clone = called.clone();
        let id = signal.connect(Box::new(move |_| {
            *called_clone.borrow_mut() = true;
        }));

        signal.disconnect(id);
        signal.emit(&[]);

        assert!(!*called.borrow());
    }

    #[test]
    fn test_signal_multiple_handlers() {
        let mut signal = Signal::new("test");
        let count = Rc::new(RefCell::new(0));

        for _ in 0..3 {
            let count_clone = count.clone();
            signal.connect(Box::new(move |_| {
                *count_clone.borrow_mut() += 1;
            }));
        }

        signal.emit(&[]);
        assert_eq!(*count.borrow(), 3);
    }

    #[test]
    fn test_signal_bus() {
        let mut bus = SignalBus::new();
        let value = Rc::new(RefCell::new(0));

        let value_clone = value.clone();
        bus.get_or_create("increment").connect(Box::new(move |_| {
            *value_clone.borrow_mut() += 1;
        }));

        bus.emit("increment", &[]);
        bus.emit("increment", &[]);
        bus.emit("increment", &[]);

        assert_eq!(*value.borrow(), 3);
    }

    #[test]
    fn test_signal_bus_get_or_create() {
        let mut bus = SignalBus::new();

        // 使用嵌套作用域来避免同时借用
        let signal1_ptr: *const Signal;
        {
            let signal1 = bus.get_or_create("test");
            signal1_ptr = signal1 as *const Signal;
        }
        {
            let signal2 = bus.get_or_create("test");
            // 应该是同一个信号
            assert!(std::ptr::eq(signal1_ptr, signal2 as *const Signal));
        }
    }

    // ============= Signal / SignalBus 更多测试 =============

    #[test]
    fn test_signal_new_with_name() {
        let signal = Signal::new("test_signal");
        // 验证信号创建后可触发（无 panic）
        let _ = signal;
    }

    #[test]
    fn test_signal_connect_disconnect() {
        let mut signal = Signal::new("sig");
        let id = signal.connect(Box::new(|_| {}));
        signal.disconnect(id);
    }

    #[test]
    fn test_signal_disconnect_nonexistent_id() {
        let mut signal = Signal::new("sig");
        signal.disconnect(999);
    }

    #[test]
    fn test_signal_emit_multiple_handlers() {
        use std::cell::Cell;
        let count = Rc::new(Cell::new(0));
        let mut signal = Signal::new("sig");

        let c1 = count.clone();
        signal.connect(Box::new(move |_| c1.set(c1.get() + 1)));
        let c2 = count.clone();
        signal.connect(Box::new(move |_| c2.set(c2.get() + 1)));

        signal.emit(&[]);
        signal.emit(&[]);

        assert_eq!(count.get(), 4);
    }

    #[test]
    fn test_signal_bus_new() {
        let _bus = SignalBus::new();
    }

    #[test]
    fn test_signal_bus_emit() {
        use std::cell::Cell;
        let count = Rc::new(Cell::new(0));

        let mut bus = SignalBus::new();
        let c = count.clone();
        bus.get_or_create("my_signal").connect(Box::new(move |_| c.set(c.get() + 1)));
        bus.emit("my_signal", &[]);
        bus.emit("my_signal", &[]);

        assert_eq!(count.get(), 2);
    }

    #[test]
    fn test_signal_bus_emit_nonexistent_no_panic() {
        let mut bus = SignalBus::new();
        bus.emit("nonexistent", &[]);
    }

    #[test]
    fn test_signal_connect_then_disconnect() {
        use std::cell::Cell;
        let count = Rc::new(Cell::new(0));
        let mut signal = Signal::new("sig");

        let c = count.clone();
        let id = signal.connect(Box::new(move |_| c.set(c.get() + 1)));
        signal.emit(&[]);
        signal.disconnect(id);
        signal.emit(&[]);

        assert_eq!(count.get(), 1);
    }
}
