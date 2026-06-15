# Window 构建器需求

## 模块概述

WindowBuilder 是窗口系统的 Fluent API 构造器，提供链式调用方式配置窗口各项属性（标题、尺寸、全屏模式等）。采用 Builder 模式简化 Window 对象的创建过程。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 2-10, 26-30, 111-159

---

## 需求详情

### 基础配置

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 2 | Fluent API 设置标题 | `WindowBuilder::with_title(title: impl Into<String>)` | 输入: String → 输出: Self | P0 |
| 3 | 设置窗口尺寸 | `WindowBuilder::with_inner_size(size: LogicalSize<u32>)` | 输入: Size → 输出: Self | P0 |
| 26 | 设置最小尺寸 | `WindowBuilder::with_min_inner_size(size: LogicalSize<u32>)` | 输入: Size → 输出: Self | P1 |
| 27 | 设置最大尺寸 | `WindowBuilder::with_max_inner_size(size: LogicalSize<u32>)` | 输入: Size → 输出: Self | P1 |
| 6 | 设置可调整大小 | `WindowBuilder::with_resizable(bool)` | 输入: bool → 输出: Self | P0 |
| 8 | 设置 vsync | `WindowBuilder::with_vsync(bool)` | 输入: bool → 输出: Self | P1 |
| 9 | 设置全屏模式 | `WindowBuilder::with_fullscreen(mode: Fullscreen)` | 输入: Fullscreen → 输出: Self | P1 |

### 外观配置

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 4 | 设置最小/最大尺寸 | `WindowBuilder::with_min_inner_size / with_max_inner_size` | 输入: Size → 输出: Self | P1 |
| 5 | 设置图标（可选） | `WindowBuilder::with_window_icon(icon: Option<Icon>)` | 输入: Option<Icon> → 输出: Self | P2 |
| 28 | 设置是否装饰 | `WindowBuilder::with_decorations(bool)` | 输入: bool → 输出: Self | P1 |
| 29 | 设置透明度 | `WindowBuilder::with_transparent(bool)` | 输入: bool → 输出: Self | P2 |
| 150 | 设置永远置顶 | `WindowBuilder::with_always_on_top(bool)` | 输入: bool → 输出: Self | P2 |
| 151 | 设置可见性 | `WindowBuilder::with_visible(bool)` | 输入: bool → 输出: Self | P0 |
| 152 | 设置最大化 | `WindowBuilder::with_maximized(bool)` | 输入: bool → 输出: Self | P1 |
| 153 | 设置最小化 | `WindowBuilder::with_minimized(bool)` | 输入: bool → 输出: Self | P1 |
| 154 | 设置内容保护 | `WindowBuilder::with_content_protected(bool)` | 输入: bool → 输出: Self | P2 |
| 156 | 防止失焦 | `WindowBuilder::with_prevent_defocus(bool)` | 输入: bool → 输出: Self | P2 |
| 157 | 设置 IME | `WindowBuilder::with_ime(bool)` | 输入: bool → 输出: Self | P2 |
| 158 | 设置光标命中测试 | `WindowBuilder::with_cursor_hittest(bool)` | 输入: bool → 输出: Self | P2 |

### 多显示器支持

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 30 | 设置显示器 | `WindowBuilder::with_monitor(monitor: MonitorHandle)` | 输入: MonitorHandle → 输出: Self | P2 |

### 构建与实例化

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 1 | 创建 engine-window crate | `WindowBuilder::new()` | 输出: Self | P0 |
| 129 | 构建 Window 对象 | `WindowBuilder::build(&self) -> Result<Window>` | 输入: &Self → 输出: Result<Window> | P0 |

---

## API 签名汇总

### WindowBuilder

```rust
pub struct WindowBuilder {
    // 内部状态
}

impl WindowBuilder {
    pub fn new() -> Self;
    
    // 基础配置
    pub fn with_title(self, title: impl Into<String>) -> Self;
    pub fn with_inner_size(self, size: LogicalSize<u32>) -> Self;
    pub fn with_min_inner_size(self, size: LogicalSize<u32>) -> Self;
    pub fn with_max_inner_size(self, size: LogicalSize<u32>) -> Self;
    pub fn with_resizable(self, resizable: bool) -> Self;
    pub fn with_vsync(self, vsync: bool) -> Self;
    pub fn with_fullscreen(self, mode: Fullscreen) -> Self;
    
    // 外观配置
    pub fn with_decorations(self, decorations: bool) -> Self;
    pub fn with_transparent(self, transparent: bool) -> Self;
    pub fn with_window_icon(self, icon: Option<Icon>) -> Self;
    pub fn with_always_on_top(self, always_on_top: bool) -> Self;
    pub fn with_visible(self, visible: bool) -> Self;
    pub fn with_maximized(self, maximized: bool) -> Self;
    pub fn with_minimized(self, minimized: bool) -> Self;
    pub fn with_content_protected(self, protected: bool) -> Self;
    pub fn with_prevent_defocus(self, prevent: bool) -> Self;
    pub fn with_ime(self, ime_enabled: bool) -> Self;
    pub fn with_cursor_hittest(self, hittest: bool) -> Self;
    
    // 多显示器
    pub fn with_monitor(self, monitor: MonitorHandle) -> Self;
    
    // 构建
    pub fn build(&self) -> Result<Window>;
}
```

### 辅助类型

```rust
// 全屏模式枚举
pub enum Fullscreen {
    Borderless(Option<MonitorHandle>),
    Exclusive(VideoMode),
    None,
}

// 视频模式
pub struct VideoMode {
    pub size: PhysicalSize<u32>,
    pub refresh_rate: u32,
    pub monitor: MonitorHandle,
}

// 显示器句柄
pub struct MonitorHandle { /* ... */ }

// 图标
pub struct Icon { /* ... */ }

// 尺寸类型
pub struct LogicalSize<T> { /* ... */ }
pub struct PhysicalSize<T> { /* ... */ }
```

---

## 输入/输出

### 构造输入
- `title`: String - 窗口标题
- `size`: LogicalSize<u32> - 窗口尺寸（逻辑像素）
- `resizable`: bool - 是否可调整大小
- `fullscreen`: Fullscreen - 全屏模式
- `decorations`: bool - 是否显示装饰（标题栏）
- `transparent`: bool - 是否透明
- `icon`: Option<Icon> - 窗口图标
- `monitor`: MonitorHandle - 目标显示器

### 构建输出
- `Result<Window>` - 成功返回 Window 实例，失败返回错误

---

## 验收标准

| 验收项 | 标准 |
|-------|------|
| 构建成功 | `WindowBuilder::new().with_title("Test").build()` 返回 Ok |
| 尺寸设置 | 设置后 `window.inner_size()` 返回设置值 |
| 装饰隐藏 | `with_decorations(false)` 隐藏标题栏和边框 |
| 透明度 | `with_transparent(true)` 支持透明背景 |
| 最小/最大尺寸 | 窗口无法resize到限制外 |
| 全屏模式 | `with_fullscreen(Borderless(None))` 进入无边框全屏 |

---

## 依赖关系

- **内部依赖**: 无
- **外部依赖**: 
  - `winit` crate（窗口管理）
  - `raw-window-handle 0.5`（底层句柄）
- **被依赖模块**: Window API、EventLoop

---

## 优先级定义

- **P0**: 核心功能，必须在 Sprint 02 完成
- **P1**: 重要功能，应在 Sprint 02 完成  
- **P2**: 增强功能，可延后到后续 Sprint
