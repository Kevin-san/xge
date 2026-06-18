# Engine/App/Module 核心架构需求

## 模块名称与概述

本模块定义了游戏引擎的核心抽象层，包括 `Engine`（引擎主结构）、`App`（用户应用入口）和 `Module`（模块化组件系统）。这是整个引擎的骨架，提供了生命周期管理、模块注册、依赖解析和应用构建的核心功能。

## 需求编号

对应原文档需求编号：1, 6, 7, 8, 9, 10, 11, 12, 29-41, 109-172

## 功能描述

### 1. Engine 结构体

Engine 是引擎的核心结构体，负责管理整个引擎的生命周期和资源访问。

**核心功能：**
- `Engine::new(config)` — 使用配置结构体构造引擎实例
- `Engine::run()` — 启动主循环：`poll events → update → render → present`
- `Engine::request_quit()` — 请求安全退出（线程安全）
- `Engine::is_running()` — 返回引擎是否正在运行
- `Engine::module<T: Module>()` — 获取模块引用（按类型）
- `Engine::module_mut<T: Module>()` — 获取模块可变引用
- `Engine::world()` / `world_mut()` — 返回 ECS World 引用（本 Sprint 返回占位）
- `Engine::time()` — 返回时间引用
- `Engine::filesystem()` — 返回文件系统引用
- `Engine::config()` — 返回配置引用
- `Engine::spawn_task(future)` — 向线程池提交异步任务

### 2. Module Trait

Module 是模块化组件的抽象接口，所有引擎模块都实现此 trait。

**Trait 定义：**
```rust
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn dependencies(&self) -> Vec<&str>;
    fn on_init(&mut self, engine: &Engine);
    fn on_update(&mut self, engine: &mut Engine, dt: f64);
    fn on_render(&mut self, engine: &mut Engine);
    fn on_shutdown(&mut self, engine: &mut Engine);
    fn enabled(&self) -> bool;
}
```

**功能：**
- `name()` — 返回唯一模块名称
- `dependencies()` — 返回依赖模块名列表，用于拓扑排序
- `on_init()` — 初始化阶段调用
- `on_update()` — 每帧更新调用
- `on_render()` — 渲染前调用
- `on_shutdown()` — 关闭时调用
- `enabled()` — 返回模块是否启用

### 3. ModuleRegistry

ModuleRegistry 负责模块的注册、初始化顺序管理和生命周期调度。

**核心功能：**
- `ModuleRegistry::register<T>()` — 注册模块
- `ModuleRegistry::get<T>()` — 按类型查找模块
- `ModuleRegistry::get_by_name(name)` — 按名称动态查找
- `ModuleRegistry::initialize_all(engine)` — 按依赖拓扑排序后依序初始化
- `ModuleRegistry::update_all(engine, dt)` — 批量更新所有模块
- `ModuleRegistry::shutdown_all(engine)` — 逆序关闭所有模块

**初始化顺序保证：**
- 使用拓扑排序确保依赖模块先于依赖它的模块初始化
- 关闭时逆序执行

### 4. App Trait

App 是开发者入口抽象，游戏项目实现 App 即可接入引擎。

**Trait 定义：**
```rust
pub trait App: Send + Sync {
    fn setup(&mut self, engine: &Engine);
    fn update(&mut self, engine: &mut Engine, dt: f64);
    fn render(&mut self, engine: &mut Engine);
    fn shutdown(&mut self, engine: &Engine);
}
```

### 5. AppBuilder

AppBuilder 提供 Fluent API 用于注册模块和配置。

**核心功能：**
- `AppBuilder::new()` — 创建构建器
- `AppBuilder::with_config(config)` — 设置引擎配置
- `AppBuilder::add_module<T>()` — 注册模块
- `AppBuilder::add_plugin<T>()` — 注册插件
- `AppBuilder::run(app)` — 启动引擎并运行应用

### 6. 主循环功能

- 支持可变时间步（默认）与固定时间步（可选）
- 支持 `pause()` / `resume()` 控制
- 支持 `request_quit()` 请求安全退出
- 实现 `FrameStats` — 每帧统计：帧号、dt、CPU 耗时

## API 签名

### Engine
```rust
impl Engine {
    pub fn new(config: EngineConfig) -> Self;
    pub fn run(&mut self);
    pub fn request_quit(&self);
    pub fn is_running(&self) -> bool;
    pub fn module<T: Module>(&self) -> Option<&T>;
    pub fn module_mut<T: Module>(&mut self) -> Option<&mut T>;
    pub fn world(&self) -> &World;
    pub fn world_mut(&mut self) -> &mut World;
    pub fn time(&self) -> &Time;
    pub fn filesystem(&self) -> &FileSystem;
    pub fn config(&self) -> &EngineConfig;
    pub fn spawn_task<F>(&self, f: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static;
}
```

### Module
```rust
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn dependencies(&self) -> Vec<&str>;
    fn on_init(&mut self, engine: &Engine);
    fn on_update(&mut self, engine: &mut Engine, dt: f64);
    fn on_render(&mut self, engine: &mut Engine);
    fn on_shutdown(&mut self, engine: &mut Engine);
    fn enabled(&self) -> bool;
}
```

### ModuleRegistry
```rust
impl ModuleRegistry {
    pub fn new() -> Self;
    pub fn register<M: Module + 'static>(&mut self, module: M);
    pub fn get<M: Module + 'static>(&self) -> Option<&M>;
    pub fn get_by_name(&self, name: &str) -> Option<&dyn Module>;
    pub fn initialize_all(&mut self, engine: &Engine) -> Result<()>;
    pub fn update_all(&mut self, engine: &mut Engine, dt: f64);
    pub fn shutdown_all(&mut self, engine: &Engine);
}
```

### App
```rust
pub trait App: Send + Sync {
    fn setup(&mut self, engine: &Engine);
    fn update(&mut self, engine: &mut Engine, dt: f64);
    fn render(&mut self, engine: &mut Engine);
    fn shutdown(&mut self, engine: &Engine);
}
```

### AppBuilder
```rust
impl AppBuilder {
    pub fn new() -> Self;
    pub fn with_config(mut self, config: EngineConfig) -> Self;
    pub fn add_module<M: Module + 'static>(mut self, module: M) -> Self;
    pub fn add_plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self;
    pub fn run(self, app: impl App);
}
```

## 输入/输出

### Engine::new(config)
- **输入：** `EngineConfig` 结构体
- **输出：** `Engine` 实例

### Engine::run()
- **输入：** 无
- **输出：** 启动主循环，直到 `request_quit()` 被调用

### Module::on_init(engine)
- **输入：** `&Engine` 引用
- **输出：** 无（初始化副作用）

### App::setup(engine)
- **输入：** `&Engine` 引用
- **输出：** 无（用户初始化逻辑）

## 验收标准

- [ ] `Engine::new(config)` 成功创建引擎实例
- [ ] `Engine::run()` 能正常启动主循环
- [ ] `Engine::request_quit()` 能安全请求退出
- [ ] `Engine::is_running()` 正确反映运行状态
- [ ] `Engine::module<T>()` 能正确获取模块引用
- [ ] `Module::name()` 返回唯一名称
- [ ] `Module::dependencies()` 返回正确的依赖列表
- [ ] `ModuleRegistry::initialize_all()` 按依赖拓扑排序初始化
- [ ] `ModuleRegistry::shutdown_all()` 逆序关闭模块
- [ ] `AppBuilder` 支持 fluent API 链式调用
- [ ] `App::setup/update/render/shutdown` 生命周期正确执行
- [ ] 主循环支持 pause/resume
- [ ] 主循环支持 request_quit 安全退出

## 依赖关系

**依赖模块：**
- `engine-config` — 引擎配置
- `engine-time` — 时间管理
- `engine-filesystem` — 文件系统抽象
- `engine-platform` — 平台检测
- `engine-log` — 日志系统

**被依赖模块：**
- 所有上层模块（渲染、物理、UI 等）都依赖本模块

## 优先级

**P0（必须）：**
- Engine/App/Module 核心 trait 定义
- ModuleRegistry 依赖解析与排序
- 主循环骨架实现

**P1（重要）：**
- AppBuilder fluent API
- 线程安全退出机制

**P2（可选）：**
- World 占位结构（后续 ECS 集成）
