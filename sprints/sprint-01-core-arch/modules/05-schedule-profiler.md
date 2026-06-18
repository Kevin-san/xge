# 调度器与性能分析需求

## 模块名称与概述

本模块定义引擎的调度器系统（Schedule）和性能分析工具（Profiler）。调度器将引擎更新拆分为 Startup / Update / Render / Shutdown 四个阶段，Profiler 提供轻量级作用域计时功能。

## 需求编号

对应原文档需求编号：112-118, 235-289

## 功能描述

### 1. Schedule 调度器

Schedule 是引擎的调度器骨架，将更新分为多个阶段。

**阶段定义：**
- `Startup` — 启动阶段
- `Update` — 逻辑更新阶段
- `Render` — 渲染阶段
- `Shutdown` — 关闭阶段

**核心功能：**
- `Schedule::new()` — 创建调度器
- `Schedule::add_stage(name)` — 注册新阶段
- `Schedule::add_system_to_stage(stage, system)` — 添加系统到阶段
- `Schedule::run(&mut self, engine)` — 执行调度
- `Schedule::stage_order(&self) -> &[String]` — 获取阶段顺序
- `Schedule::set_run_criteria(...)` — 留接口（未来扩展）

**系统依赖声明：**
- 本 Sprint 仅支持线性顺序
- 后续扩展并行化

### 2. Profiler 性能分析器

Profiler 提供轻量级作用域计时功能。

**核心功能：**
- `Profiler::new()` — 创建 Profiler
- `Profiler::begin_scope(name)` — 开始计时作用域
- `Profiler::end_scope()` — 结束作用域计时
- `Profiler::scope(name)` — RAII 守卫，自动计时
- `Profiler::dump(&self)` — 输出作用域耗时汇总

**使用方式：**
```rust
profiler.scope!("physics_update", {
    // 物理更新代码
});
```

### 3. FrameStats 帧统计

FrameStats 提供每帧性能统计。

**字段：**
- `frame_number: u64` — 帧号
- `dt: f32` — 上帧时长（秒）
- `cpu_time_us: u64` — CPU 耗时（微秒）

### 4. EngineStats 引擎统计

EngineStats 提供全局运行时统计。

**字段：**
- `uptime_seconds: f64` — 运行时长（秒）
- `total_frames: u64` — 总帧数
- `avg_fps: f32` — 平均 FPS

### 5. Plugin / PluginGroup 插件系统

Plugin 是插件系统接口（Module 的简化版）。

**Trait 定义：**
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn build(&mut self, app: &mut AppBuilder);
}
```

**功能：**
- `PluginGroup` — 成组安装插件
- `DefaultPlugins` — 默认插件组

## API 签名

### Schedule
```rust
pub struct Schedule {
    stages: Vec<Stage>,
    stage_order: Vec<String>,
}

pub struct Stage {
    name: String,
    systems: Vec<Box<dyn System>>,
}

pub trait System: Send + Sync {
    fn name(&self) -> &str;
    fn run(&mut self, engine: &mut Engine);
}

impl Schedule {
    pub fn new() -> Self;
    pub fn add_stage(&mut self, name: impl Into<String>) -> &mut Self;
    pub fn add_system_to_stage<S: System + 'static>(
        &mut self,
        stage: impl Into<String>,
        system: S,
    ) -> &mut Self;
    pub fn run(&mut self, engine: &mut Engine);
    pub fn stage_order(&self) -> &[String];
    pub fn set_run_criteria(&mut self, criteria: RunCriteria);
}
```

### Profiler
```rust
pub struct Profiler {
    scopes: Vec<ScopeData>,
}

pub struct ScopeData {
    name: String,
    start: Instant,
    duration: Option<Duration>,
}

impl Profiler {
    pub fn new() -> Self;
    pub fn begin_scope(&mut self, name: &str);
    pub fn end_scope(&mut self);
    pub fn scope<F, R>(&mut self, name: &str, f: F) -> R;
    pub fn dump(&self);
    pub fn clear(&mut self);
}

pub struct ScopedTimer<'a> {
    profiler: &'a mut Profiler,
    name: String,
}

impl Drop for ScopedTimer<'_> {
    fn drop(&mut self) {
        self.profiler.end_scope();
    }
}
```

### FrameStats
```rust
#[derive(Default)]
pub struct FrameStats {
    pub frame_number: u64,
    pub dt: f32,
    pub cpu_time_us: u64,
}
```

### EngineStats
```rust
#[derive(Default)]
pub struct EngineStats {
    pub uptime_seconds: f64,
    pub total_frames: u64,
    pub avg_fps: f32,
}
```

### Plugin
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn build(&mut self, app: &mut AppBuilder);
}

pub struct PluginGroup {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginGroup {
    pub fn new() -> Self;
    pub fn add<P: Plugin + 'static>(mut self, plugin: P) -> Self;
}

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn name(&self) -> &str;
    fn build(&mut self, app: &mut AppBuilder);
}
```

## 输入/输出

### Schedule::run(engine)
- **输入：** &mut Engine
- **输出：** 无（按阶段顺序执行所有系统）

### Profiler::scope(name, f)
- **输入：** 作用域名称和闭包
- **输出：** 闭包的返回值
- **副作用：** 自动记录作用域耗时

### Profiler::dump()
- **输入：** 无
- **输出：** 打印所有作用域耗时汇总到日志

## 验收标准

- [ ] Schedule 支持 Startup/Update/Render/Shutdown 四阶段
- [ ] Schedule::run() 按 stage_order 顺序执行
- [ ] Schedule::add_system_to_stage() 正确添加系统
- [ ] Schedule 支持系统依赖声明（线性顺序）
- [ ] Profiler::scope() RAII 守卫正确计时
- [ ] Profiler::dump() 输出所有作用域耗时
- [ ] FrameStats 帧号、dt、cpu_time_us 正确统计
- [ ] EngineStats uptime_seconds、total_frames、avg_fps 正确
- [ ] Plugin::build() 正确注册到 AppBuilder
- [ ] DefaultPlugins 包含默认插件组

## 依赖关系

**依赖模块：**
- `engine-core` — 引擎核心和 AppBuilder
- `parking_lot` — 同步原语

**被依赖模块：**
- `engine-core` — 主循环使用 Schedule
- 所有需要被调度的模块

## 优先级

**P0（必须）：**
- Schedule 调度器骨架
- Startup/Update/Render/Shutdown 四阶段
- add_system_to_stage 功能

**P1（重要）：**
- Profiler 轻量实现
- FrameStats / EngineStats
- Plugin / PluginGroup

**P2（可选）：**
- RunCriteria 扩展
- 并行化支持
- 统计可视化
