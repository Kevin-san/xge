# 核心 API 清单

## 模块名称与概述

本文档列出 `engine-core` 公开的所有 API，数量控制在 20-30 个公开函数以内（符合原需求 #126）。这些 API 构成了引擎的公开接口，所有其他模块都应围绕这些核心类型构建。

## API 清单

### 1. Engine 主结构

#### `Engine::new(config: EngineConfig) -> Self`
创建引擎实例。

```rust
pub fn new(config: EngineConfig) -> Self
```

**参数：**
- `config: EngineConfig` — 引擎配置

**返回：**
- `Engine` 实例

**示例：**
```rust
let config = EngineConfig::default();
let engine = Engine::new(config);
```

---

#### `Engine::run(&mut self)`
启动引擎主循环。

```rust
pub fn run(&mut self)
```

**行为：**
- 进入主循环：`poll events → update → render → present`
- 直到 `request_quit()` 被调用才退出

---

#### `Engine::request_quit(&self)`
请求安全退出引擎。

```rust
pub fn request_quit(&self)
```

**线程安全：** 可从任何线程调用

---

#### `Engine::is_running(&self) -> bool`
检查引擎是否正在运行。

```rust
pub fn is_running(&self) -> bool
```

**返回：**
- `true` — 引擎正在运行
- `false` — 引擎已停止

---

#### `Engine::time(&self) -> &Time`
获取时间管理器引用。

```rust
pub fn time(&self) -> &Time
```

---

#### `Engine::filesystem(&self) -> &FileSystem`
获取文件系统引用。

```rust
pub fn filesystem(&self) -> &FileSystem
```

---

#### `Engine::config(&self) -> &EngineConfig`
获取配置引用。

```rust
pub fn config(&self) -> &EngineConfig
```

---

#### `Engine::module<T: Module>(&self) -> Option<&T>`
按类型获取模块引用。

```rust
pub fn module<T: Module>(&self) -> Option<&T>
```

---

#### `Engine::module_mut<T: Module>(&mut self) -> Option<&mut T>`
按类型获取模块可变引用。

```rust
pub fn module_mut<T: Module>(&mut self) -> Option<&mut T>
```

---

#### `Engine::spawn_task<F>(&self, f: F) -> JoinHandle<F::Output>`
向线程池提交异步任务。

```rust
pub fn spawn_task<F>(&self, f: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
```

---

### 2. Module Trait

#### `Module::name(&self) -> &str`
获取模块唯一名称。

```rust
fn name(&self) -> &str
```

---

#### `Module::dependencies(&self) -> Vec<&str>`
获取依赖模块名列表。

```rust
fn dependencies(&self) -> Vec<&str>
```

---

#### `Module::on_init(&mut self, engine: &Engine)`
模块初始化回调。

```rust
fn on_init(&mut self, engine: &Engine)
```

---

#### `Module::on_update(&mut self, engine: &mut Engine, dt: f64)`
模块每帧更新回调。

```rust
fn on_update(&mut self, engine: &mut Engine, dt: f64)
```

---

#### `Module::on_render(&mut self, engine: &mut Engine)`
模块渲染前回调。

```rust
fn on_render(&mut self, engine: &mut Engine)
```

---

#### `Module::on_shutdown(&mut self, engine: &mut Engine)`
模块关闭回调。

```rust
fn on_shutdown(&mut self, engine: &mut Engine)
```

---

#### `Module::enabled(&self) -> bool`
检查模块是否启用。

```rust
fn enabled(&self) -> bool
```

---

### 3. ModuleRegistry

#### `ModuleRegistry::new() -> Self`
创建模块注册表。

```rust
pub fn new() -> Self
```

---

#### `ModuleRegistry::register<M: Module + 'static>(&mut self, module: M)`
注册模块。

```rust
pub fn register<M: Module + 'static>(&mut self, module: M)
```

---

#### `ModuleRegistry::initialize_all(&mut self, engine: &Engine) -> Result<()>`
按依赖拓扑排序初始化所有模块。

```rust
pub fn initialize_all(&mut self, engine: &Engine) -> Result<()>
```

---

#### `ModuleRegistry::update_all(&mut self, engine: &mut Engine, dt: f64)`
批量更新所有模块。

```rust
pub fn update_all(&mut self, engine: &mut Engine, dt: f64)
```

---

#### `ModuleRegistry::shutdown_all(&mut self, engine: &Engine)`
逆序关闭所有模块。

```rust
pub fn shutdown_all(&mut self, engine: &Engine)
```

---

### 4. App Trait

#### `App::setup(&mut self, engine: &Engine)`
用户游戏代码入口。

```rust
fn setup(&mut self, engine: &Engine)
```

---

#### `App::update(&mut self, engine: &mut Engine, dt: f64)`
用户帧更新。

```rust
fn update(&mut self, engine: &mut Engine, dt: f64)
```

---

#### `App::render(&mut self, engine: &mut Engine)`
用户渲染钩子。

```rust
fn render(&mut self, engine: &mut Engine)
```

---

#### `App::shutdown(&mut self, engine: &Engine)`
用户退出钩子。

```rust
fn shutdown(&mut self, engine: &Engine)
```

---

### 5. AppBuilder

#### `AppBuilder::new() -> Self`
创建应用构建器。

```rust
pub fn new() -> Self
```

---

#### `AppBuilder::with_config(mut self, config: EngineConfig) -> Self`
设置引擎配置。

```rust
pub fn with_config(mut self, config: EngineConfig) -> Self
```

---

#### `AppBuilder::add_module<M: Module + 'static>(mut self, module: M) -> Self`
注册模块。

```rust
pub fn add_module<M: Module + 'static>(mut self, module: M) -> Self
```

---

#### `AppBuilder::run(self, app: impl App)`
启动引擎并运行应用。

```rust
pub fn run(self, app: impl App)
```

---

### 6. Schedule

#### `Schedule::new() -> Self`
创建调度器。

```rust
pub fn new() -> Self
```

---

#### `Schedule::add_stage(&mut self, name: impl Into<String>) -> &mut Self`
注册新阶段。

```rust
pub fn add_stage(&mut self, name: impl Into<String>) -> &mut Self
```

---

#### `Schedule::run(&mut self, engine: &mut Engine)`
执行调度。

```rust
pub fn run(&mut self, engine: &mut Engine)
```

---

### 7. 全局函数

#### `log::init(level: Level)`
初始化日志系统。

```rust
pub fn init(level: Level)
```

---

#### `log::set_level(level: Level)`
设置日志等级。

```rust
pub fn set_level(level: Level)
```

---

#### `Platform::current() -> Platform`
获取当前平台。

```rust
pub fn current() -> Platform
```

---

#### `Feature::enabled(name: &str) -> bool`
查询特性是否启用。

```rust
pub fn enabled(name: &str) -> bool
```

---

## 公开 API 统计

| 类别 | 数量 |
|------|------|
| Engine | 10 |
| Module Trait | 7 |
| ModuleRegistry | 5 |
| App Trait | 4 |
| AppBuilder | 4 |
| Schedule | 3 |
| 全局函数 | 4 |
| **总计** | **37** |

> 注：实际实现时需控制在 20-30 个，可通过 trait method 数量精简或合并实现。

## 优先级

**P0（必须）：**
- Engine 核心方法
- Module trait
- AppBuilder

**P1（重要）：**
- Schedule
- App trait

**P2（可选）：**
- 辅助函数
