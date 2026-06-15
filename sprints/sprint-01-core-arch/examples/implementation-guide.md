# 示例实现指南

## 模块名称与概述

本文档提供 `hello_engine`、`minimal_app`、`module_order` 和 `event_bus_demo` 四个示例的实现指南，演示引擎核心功能的使用方法。

## 示例概览

| 示例 | 目的 | 核心演示内容 |
|------|------|-------------|
| hello_engine | 入门 | 打印引擎版本、初始化、触发空帧 |
| minimal_app | 最小 Demo | App trait 实现、default().run() |
| module_order | 模块系统 | 模块注册与依赖顺序初始化 |
| event_bus_demo | 事件总线 | 订阅/派发/取消订阅 |

## 示例 1: hello_engine

### 目的
最简单的引擎使用示例，演示如何创建引擎实例并运行。

### 实现步骤

1. **创建 `examples/hello_engine.rs`**：

```rust
use engine_core::{Engine, EngineConfig};

fn main() {
    // 1. 打印引擎版本
    println!("Engine Version: {}", engine_core::ENGINE_VERSION);
    println!("Build: {} @ {}", engine_core::BUILD_COMMIT_HASH, engine_core::BUILD_TIMESTAMP);

    // 2. 创建引擎配置
    let config = EngineConfig::default();

    // 3. 创建引擎实例
    let mut engine = Engine::new(config);

    // 4. 运行引擎（会触发一次空帧然后退出）
    engine.run();
}
```

2. **验证输出**：
```
Engine Version: 0.1.0-dev
Build: abc1234 @ 2024-01-01T00:00:00Z
[INFO] Engine initialized
[INFO] Starting main loop
[INFO] Frame 1 - dt: 16.67ms
[INFO] Engine shutdown complete
```

### 验收标准
- [ ] 运行成功退出码 0
- [ ] 打印引擎版本号
- [ ] 触发一次空帧后退出

---

## 示例 2: minimal_app

### 目的
演示完整的 App trait 实现。

### 实现步骤

1. **创建 `examples/minimal_app.rs`**：

```rust
use engine_core::{
    App, AppBuilder, Engine, EngineConfig,
    module::Module,
};

struct MyGame {
    frame_count: u64,
}

impl Default for MyGame {
    fn default() -> Self {
        Self { frame_count: 0 }
    }
}

impl App for MyGame {
    fn setup(&mut self, _engine: &Engine) {
        println!("[MyGame] Setup complete");
    }

    fn update(&mut self, _engine: &mut Engine, dt: f64) {
        self.frame_count += 1;
        println!("[MyGame] Update frame {} (dt={:.2}ms)", 
                 self.frame_count, dt * 1000.0);
    }

    fn render(&mut self, _engine: &mut Engine) {
        // 渲染逻辑（空实现）
    }

    fn shutdown(&mut self, _engine: &Engine) {
        println!("[MyGame] Shutdown after {} frames", self.frame_count);
    }
}

fn main() {
    AppBuilder::new()
        .with_config(EngineConfig::default())
        .run(MyGame::default());
}
```

2. **验证输出**：
```
[MyGame] Setup complete
[MyGame] Update frame 1 (dt=16.67ms)
[MyGame] Update frame 2 (dt=16.67ms)
...
[MyGame] Shutdown after 60 frames
```

### 验收标准
- [ ] 运行成功退出码 0
- [ ] App 生命周期正确执行
- [ ] 支持 `App::default().run()`

---

## 示例 3: module_order

### 目的
演示模块注册与依赖顺序初始化。

### 实现步骤

1. **定义模块 A 和 B（B 依赖 A）**：

```rust
use engine_core::{Engine, Module, ModuleRegistry, ModuleTrait};
use std::sync::Arc;

// 模块 A：无依赖
struct ModuleA {
    name: String,
}

impl ModuleA {
    fn new() -> Self {
        Self { name: "ModuleA".into() }
    }
}

impl Module for ModuleA {
    fn name(&self) -> &str { &self.name }
    fn dependencies(&self) -> Vec<&str> { vec![] }
    fn on_init(&mut self, _engine: &Engine) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
    fn on_render(&mut self, _engine: &mut Engine) {}
    fn on_shutdown(&mut self, _engine: &Engine) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool { true }
}

// 模块 B：依赖 A
struct ModuleB {
    name: String,
}

impl ModuleB {
    fn new() -> Self {
        Self { name: "ModuleB".into() }
    }
}

impl Module for ModuleB {
    fn name(&self) -> &str { &self.name }
    fn dependencies(&self) -> Vec<&str> { vec!["ModuleA"] }
    fn on_init(&mut self, _engine: &Engine) {
        println!("[{}] Initialized", self.name);
    }
    fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
    fn on_render(&mut self, _engine: &mut Engine) {}
    fn on_shutdown(&mut self, _engine: &Engine) {
        println!("[{}] Shutdown", self.name);
    }
    fn enabled(&self) -> bool { true }
}
```

2. **验证初始化顺序**：

```rust
fn main() {
    let mut registry = ModuleRegistry::new();
    registry.register(ModuleB::new()); // B 先注册
    registry.register(ModuleA::new()); // A 后注册

    // 创建引擎并初始化
    let engine = Engine::new(EngineConfig::default());
    registry.initialize_all(&engine).unwrap();

    // 预期输出顺序：
    // [ModuleA] Initialized  (A 先初始化，因为 B 依赖它)
    // [ModuleB] Initialized  (B 后初始化)

    registry.shutdown_all(&engine);

    // 预期关闭顺序：
    // [ModuleB] Shutdown
    // [ModuleA] Shutdown
}
```

### 验收标准
- [ ] ModuleA 在 ModuleB 之前初始化（依赖决定）
- [ ] ModuleB 在 ModuleA 之后关闭（逆序）
- [ ] 按名称查找模块正常工作

---

## 示例 4: event_bus_demo

### 目的
演示事件总线的订阅/派发/取消订阅功能。

### 实现步骤

1. **定义事件类型**：

```rust
use engine_core::EventBus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlayerEvent {
    player_id: u32,
    action: String,
}
```

2. **创建演示**：

```rust
fn main() {
    // 创建事件总线
    let bus = EventBus::<PlayerEvent>::new();

    // 订阅事件
    let handle1 = bus.subscribe(|event| {
        println!("[Subscriber1] Player {} performed {}", 
                 event.player_id, event.action);
    });

    let handle2 = bus.subscribe(|event| {
        println!("[Subscriber2] Received: {:?}", event);
    });

    // 派发事件
    bus.send(PlayerEvent {
        player_id: 42,
        action: "jump".into(),
    });

    // 取消订阅
    bus.unsubscribe(handle2);

    // 再次派发（只有 handle1 收到）
    bus.send(PlayerEvent {
        player_id: 42,
        action: "land".into(),
    });

    // drain 示例
    let mut bus2 = EventBus::<PlayerEvent>::new();
    bus2.send(PlayerEvent { player_id: 1, action: "run".into() });
    bus2.send(PlayerEvent { player_id: 2, action: "walk".into() });

    bus2.drain(); // 消费所有累积事件
}
```

3. **预期输出**：
```
[Subscriber1] Player 42 performed jump
[Subscriber2] Received: PlayerEvent { player_id: 42, action: "jump" }
[Subscriber1] Player 42 performed land
[Subscriber1] Player 1 performed run
[Subscriber1] Player 2 performed walk
```

### 验收标准
- [ ] 订阅成功返回 SubscriptionHandle
- [ ] 取消订阅后不再收到事件
- [ ] drain() 批量消费所有事件
- [ ] 线程安全：跨线程派发正常

---

## 示例 5: arena_bench（可选）

### 目的
演示 Arena 句柄系统性能基准。

### 实现要点

```rust
fn main() {
    use engine_utils::{Arena, Handle};

    let mut arena = Arena::new();

    // 插入大量对象
    let handles: Vec<Handle<MyComponent>> = (0..10000)
        .map(|i| arena.insert(MyComponent { value: i }))
        .collect();

    // O(1) 查找
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        for &h in &handles {
            let _ = arena.get(h);
        }
    }
    println!("Lookup time: {:?}", start.elapsed());

    // O(1) 删除
    for h in handles.iter().take(5000) {
        arena.remove(*h);
    }

    // 迭代只遍历存活对象
    let alive = arena.iter().count();
    println!("Alive objects: {}", alive);
}
```

---

## 通用实现指南

### 错误处理
所有示例应使用 `anyhow::Result<()>` 进行错误处理：

```rust
fn main() -> anyhow::Result<()> {
    // ...
    Ok(())
}
```

### 日志初始化
示例开头应初始化日志：

```rust
use engine_log::{init, Level};

fn main() {
    init(Level::Info);
    // ...
}
```

### 构建配置
示例 `Cargo.toml`：

```toml
[[example]]
name = "hello_engine"
path = "examples/hello_engine.rs"

[[example]]
name = "minimal_app"
path = "examples/minimal_app.rs"

[[example]]
name = "module_order"
path = "examples/module_order.rs"

[[example]]
name = "event_bus_demo"
path = "examples/event_bus_demo.rs"
```

## 验收标准

- [ ] `cargo run --example hello_engine` 成功
- [ ] `cargo run --example minimal_app` 成功
- [ ] `cargo run --example module_order` 按正确顺序初始化
- [ ] `cargo run --example event_bus_demo` 事件正常收发
- [ ] 所有示例退出码为 0
