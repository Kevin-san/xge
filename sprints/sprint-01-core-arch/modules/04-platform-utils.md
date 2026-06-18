# 平台抽象与工具需求

## 模块名称与概述

`engine-platform` 模块提供平台抽象 trait（Window/FileSystem/Time/ThreadPool）以及通用工具类型（Uuid、HashMap 别名、类型安全句柄）。同时定义 `Platform` 枚举和 `Feature` 结构体用于运行时特性检测。

## 需求编号

对应原文档需求编号：13-28, 40, 41, 42-48, 51, 52, 63-74, 75-83, 97-102, 106, 120, 265-267

## 功能描述

### 1. Time 时间模块

Time 模块提供引擎时间管理功能。

**核心功能：**
- `Time::new()` — 创建 Time 实例
- `Time::tick(&mut self)` — 每帧调用，更新 dt
- `Time::delta_seconds(&self) -> f32` — 上帧时长（秒）
- `Time::delta(&self) -> Duration` — 上帧时长（Duration）
- `Time::elapsed(&self) -> Duration` — 启动至今时长
- `Time::frame_count(&self) -> u64` — 帧计数
- `Time::fps(&self) -> f32` — 当前 FPS
- `Time::set_fixed_timestep(&mut self, dt: f32)` — 设置固定时间步
- `Time::fixed_timestep(&self) -> f32` — 获取固定时间步
- `FixedTimestepSteps` — 累积步数计数

**子模块：**
- `FixedTimestepSteps` — 固定步长累积器

**工具类型：**
- `Stopwatch` — 局部计时器
  - `Stopwatch::new()` / `start()` / `stop()` / `reset()` / `elapsed()`

### 2. FileSystem 文件系统抽象

FileSystem 提供跨平台文件系统操作抽象。

**核心功能：**
- `FileSystem::read(&self, path) -> Result<Vec<u8>>`
- `FileSystem::read_string(&self, path) -> Result<String>`
- `FileSystem::write(&self, path, bytes) -> Result<()>`
- `FileSystem::write_string(&self, path, s) -> Result<()>`
- `FileSystem::exists(&self, path) -> bool`
- `FileSystem::list_dir(&self, path) -> Result<Vec<PathBuf>>`
- `FileSystem::create_dir_all(&self, path) -> Result<()>`
- `FileSystem::remove_file(&self, path) -> Result<()>`
- `FileSystem::is_dir(&self, path) -> bool`
- `FileSystem::canonicalize(&self, path) -> Result<PathBuf>`

**平台实现：**
- Windows/macOS/Linux：默认使用原生 `std::fs`
- Web：使用 `fetch` / `IndexedDB` 接口（先留 trait）

**路径处理：**
- 路径规范化（统一 `/` 分隔符）
- 忽略大小写策略可配置

### 3. ThreadPool 任务调度

ThreadPool 提供多线程任务调度功能。

**核心功能：**
- `ThreadPool::new(num_threads: Option<usize>)` — 创建线程池
- `ThreadPool::spawn<F>(&self, f: F) -> JoinHandle<F::Output>` — 异步任务
- `ThreadPool::try_spawn<F>(&self, f: F) -> Result<JoinHandle<F::Output>>` — 非阻塞
- `ThreadPool::block_on<F>(&self, f: F)` — 阻塞等待
- `ThreadPool::shutdown(&self)` — 关闭线程池
- `ThreadPool::active_count(&self) -> usize` — 活跃线程数

**配置：**
- 线程数量默认 = CPU 逻辑核心数 - 1（可配置）
- 支持任务优先级队列
- 支持 future-aware（引入 `futures-lite`）

### 4. Platform 平台检测

Platform 枚举检测当前运行平台。

**枚举变体：**
```rust
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Android,
    Ios,
    Web,
    Unknown,
}
```

**功能：**
- `Platform::current() -> Platform` — 获取当前平台
- `Platform::is_windows()` / `is_macos()` / `is_linux()` / `is_web()`
- `Platform::name(&self) -> &str` — 平台名称

**宏：**
- `current_platform()` — 编译期/运行期均可用
- `target_os_cfg!` — 按平台分发代码

### 5. Feature 特性开关

Feature 结构体在运行时查询 feature 是否启用。

**功能：**
- `Feature::enabled(name) -> bool` — 查询特性是否启用
- `Feature::list() -> Vec<&str>` — 返回所有 feature 列表
- `Feature::render_backend() -> &'static str` — 获取渲染后端

**Feature Flags 设计：**
- `render-vulkan` / `render-gl` / `render-webgpu` / `audio` / `network` / `editor`
- 默认打开 `render-gl + audio`，其余关闭
- WebAssembly 下自动禁用 host-only feature

### 6. 工具类型

#### SpinLock<T>
- 轻量无栈锁，用于高频短持有场景

#### thread_local! 包装器
- 保证单例对象安全访问

#### parking_lot::Once
- 一次性初始化

## API 签名

### Time
```rust
pub struct Time {
    delta: Duration,
    elapsed: Duration,
    frame_count: u64,
    fixed_timestep: f32,
    fps: f32,
}

impl Time {
    pub fn new() -> Self;
    pub fn tick(&mut self);
    pub fn delta_seconds(&self) -> f32;
    pub fn delta(&self) -> Duration;
    pub fn elapsed(&self) -> Duration;
    pub fn frame_count(&self) -> u64;
    pub fn fps(&self) -> f32;
    pub fn set_fixed_timestep(&mut self, dt: f32);
    pub fn fixed_timestep(&self) -> f32;
}

pub struct FixedTimestepSteps {
    accumulator: f64,
    steps: u32,
}

impl Stopwatch {
    pub fn new() -> Self;
    pub fn start(&mut self);
    pub fn stop(&mut self);
    pub fn reset(&mut self);
    pub fn elapsed(&self) -> Duration;
}
```

### FileSystem
```rust
pub trait FileSystem: Send + Sync {
    fn read(&self, path: &Path) -> Result<Vec<u8>>;
    fn read_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, bytes: &[u8]) -> Result<()>;
    fn write_string(&self, path: &Path, s: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn remove_file(&self, path: &Path) -> Result<()>;
    fn is_dir(&self, path: &Path) -> bool;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
}
```

### ThreadPool
```rust
pub struct ThreadPool {
    // 内部实现
}

impl ThreadPool {
    pub fn new(num_threads: Option<usize>) -> Result<Self>;
    pub fn spawn<F>(&self, f: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static;
    pub fn try_spawn<F>(&self, f: F) -> Result<JoinHandle<F::Output>>
    where
        F: Future + Send + 'static;
    pub fn block_on<F>(&self, f: F);
    pub fn shutdown(&mut self);
    pub fn active_count(&self) -> usize;
}
```

### Platform
```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Android,
    Ios,
    Web,
    Unknown,
}

impl Platform {
    pub fn current() -> Self;
    pub fn is_windows(&self) -> bool;
    pub fn is_macos(&self) -> bool;
    pub fn is_linux(&self) -> bool;
    pub fn is_web(&self) -> bool;
    pub fn name(&self) -> &'static str;
}
```

### Feature
```rust
pub struct Feature;

impl Feature {
    pub fn enabled(name: &str) -> bool;
    pub fn list() -> Vec<&'static str>;
    pub fn render_backend() -> &'static str;
}
```

## 输入/输出

### Time::tick(&mut self)
- **输入：** 无
- **输出：** 无（更新内部 delta 和 elapsed）

### FileSystem::read(path)
- **输入：** &Path 文件路径
- **输出：** Result<Vec<u8>> 文件内容

### ThreadPool::spawn(f)
- **输入：** 异步任务 F: Future + Send + 'static
- **输出：** JoinHandle<F::Output>

## 验收标准

- [ ] Time::delta() 精度为毫秒级（f64）
- [ ] Time::fps() 正确计算当前帧率
- [ ] FixedTimestepSteps 正确累积步数
- [ ] Stopwatch 计时精度满足微秒级
- [ ] FileSystem 在所有平台正确读写文件
- [ ] FileSystem 路径规范化正确处理 `/` 分隔符
- [ ] ThreadPool 线程数量正确计算
- [ ] ThreadPool::try_spawn() 非阻塞失败时返回错误
- [ ] Platform::current() 正确检测当前平台
- [ ] Feature::enabled() 正确反映编译期 feature
- [ ] Time 单元测试 >= 10 条

## 依赖关系

**依赖模块：**
- `parking_lot` — 同步原语
- `futures-lite` — 异步支持
- `ahash` — 高性能哈希

**被依赖模块：**
- `engine-core` — 引擎核心使用时间和线程池
- 所有上层模块

## 优先级

**P0（必须）：**
- Time 模块完整实现
- FileSystem trait 和原生实现
- ThreadPool 实现
- Platform 检测

**P1（重要）：**
- Feature 特性开关
- Stopwatch 计时器
- Web FileSystem trait（预留）

**P2（可选）：**
- SpinLock
- 优先级队列
