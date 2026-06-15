# Window API 清单

> 本文档是 `engine-window` crate 的公开 API 完整清单。API 数量控制在 30-40 个之间。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 1-170, 195-200
**公开 API 数量控制**: 30-40 个

---

## 公开 API 清单

### WindowBuilder

```rust
/// WindowBuilder 使用 Fluent API 构建 Window 对象。
/// 
/// # 示例
/// 
/// ```rust
/// let window = WindowBuilder::new()
///     .with_title("My Game")
///     .with_inner_size(LogicalSize::new(1280, 720))
///     .with_resizable(true)
///     .build()
///     .unwrap();
/// ```
pub struct WindowBuilder { /* ... */ }

impl WindowBuilder {
    /// 创建一个新的 WindowBuilder 实例。
    pub fn new() -> Self;
    
    /// 设置窗口标题。
    pub fn with_title(self, title: impl Into<String>) -> Self;
    
    /// 设置窗口内部尺寸（逻辑像素）。
    pub fn with_inner_size(self, size: LogicalSize<u32>) -> Self;
    
    /// 设置窗口最小内部尺寸。
    pub fn with_min_inner_size(self, size: LogicalSize<u32>) -> Self;
    
    /// 设置窗口最大内部尺寸。
    pub fn with_max_inner_size(self, size: LogicalSize<u32>) -> Self;
    
    /// 设置窗口是否可调整大小。
    pub fn with_resizable(self, resizable: bool) -> Self;
    
    /// 设置全屏模式。
    pub fn with_fullscreen(self, mode: Fullscreen) -> Self;
    
    /// 设置是否显示窗口装饰（标题栏、边框）。
    pub fn with_decorations(self, decorations: bool) -> Self;
    
    /// 设置窗口是否透明。
    pub fn with_transparent(self, transparent: bool) -> Self;
    
    /// 设置窗口是否永远置顶。
    pub fn with_always_on_top(self, always_on_top: bool) -> Self;
    
    /// 设置窗口初始可见性。
    pub fn with_visible(self, visible: bool) -> Self;
    
    /// 设置窗口初始最大化状态。
    pub fn with_maximized(self, maximized: bool) -> Self;
    
    /// 设置窗口初始最小化状态。
    pub fn with_minimized(self, minimized: bool) -> Self;
    
    /// 设置内容保护（防止截屏）。
    pub fn with_content_protected(self, protected: bool) -> Self;
    
    /// 设置窗口图标。
    pub fn with_window_icon(self, icon: Option<Icon>) -> Self;
    
    /// 设置是否阻止窗口失焦。
    pub fn with_prevent_defocus(self, prevent: bool) -> Self;
    
    /// 设置是否启用 IME。
    pub fn with_ime(self, ime_enabled: bool) -> Self;
    
    /// 设置是否启用光标命中测试。
    pub fn with_cursor_hittest(self, hittest: bool) -> Self;
    
    /// 构建 Window 对象。
    pub fn build(&self) -> Result<Window>;
}
```

### Window

```rust
/// Window 是对底层窗口的抽象。
/// 
/// 通过 Window 可以查询和修改窗口的各种属性。
pub trait Window {
    /// 获取窗口 ID。
    fn id(&self) -> WindowId;
    
    /// 获取窗口标题。
    fn title(&self) -> String;
    
    // ========== 尺寸 ==========
    
    /// 获取窗口内部尺寸（像素）。
    fn inner_size(&self) -> PhysicalSize<u32>;
    
    /// 获取窗口外部尺寸（像素）。
    fn outer_size(&self) -> PhysicalSize<u32>;
    
    /// 获取 DPI 缩放因子。
    fn scale_factor(&self) -> f64;
    
    // ========== 位置 ==========
    
    /// 获取窗口位置。
    fn position(&self) -> PhysicalPosition<i32>;
    
    /// 获取窗口内部位置。
    fn inner_position(&self) -> PhysicalPosition<i32>;
    
    /// 获取窗口外部位置。
    fn outer_position(&self) -> PhysicalPosition<i32>;
    
    // ========== 设置 ==========
    
    /// 设置窗口标题。
    fn set_title(&mut self, title: impl Into<String>);
    
    /// 设置窗口内部尺寸。
    fn set_inner_size(&mut self, size: PhysicalSize<u32>);
    
    /// 设置窗口最小尺寸。
    fn set_min_size(&mut self, size: Option<PhysicalSize<u32>>);
    
    /// 设置窗口最大尺寸。
    fn set_max_size(&mut self, size: Option<PhysicalSize<u32>>);
    
    /// 设置是否可调整大小。
    fn set_resizable(&mut self, resizable: bool);
    
    /// 设置最小化状态。
    fn set_minimized(&mut self, minimized: bool);
    
    /// 设置最大化状态。
    fn set_maximized(&mut self, maximized: bool);
    
    /// 设置可见性。
    fn set_visible(&mut self, visible: bool);
    
    /// 设置永远置顶。
    fn set_always_on_top(&mut self, always_on_top: bool);
    
    /// 设置全屏模式。
    fn set_fullscreen(&mut self, mode: Fullscreen);
    
    /// 设置窗口装饰。
    fn set_decorations(&mut self, decorations: bool);
    
    /// 设置窗口层级。
    fn set_window_level(&mut self, level: WindowLevel);
    
    // ========== 光标 ==========
    
    /// 设置光标图标。
    fn set_cursor_icon(&mut self, cursor: CursorIcon);
    
    /// 设置光标位置。
    fn set_cursor_position(&mut self, pos: Position) -> Result<()>;
    
    /// 设置光标捕获模式。
    fn set_cursor_grab(&mut self, grab: CursorGrabMode) -> Result<()>;
    
    /// 设置光标可见性。
    fn set_cursor_visible(&mut self, visible: bool);
    
    // ========== IME ==========
    
    /// 设置是否允许 IME。
    fn set_ime_allowed(&mut self, allowed: bool);
    
    /// 设置 IME 位置。
    fn set_ime_position(&mut self, position: Position);
    
    // ========== 窗口操作 ==========
    
    /// 请求重绘。
    fn request_redraw(&self);
    
    /// 开始拖动窗口。
    fn drag_window(&self) -> Result<()>;
    
    /// 开始拖动调整窗口大小。
    fn drag_resize_window(&self, edge: ResizeEdge);
    
    /// 聚焦窗口。
    fn focus_window(&self);
    
    /// 显示窗口菜单。
    fn show_window_menu(&self, pos: Position);
    
    // ========== 底层句柄 ==========
    
    /// 获取底层显示句柄。
    fn raw_display_handle(&self) -> RawDisplayHandle;
    
    /// 获取底层窗口句柄。
    fn raw_window_handle(&self) -> RawWindowHandle;
    
    // ========== 显示器 ==========
    
    /// 获取当前显示器。
    fn current_monitor(&self) -> Option<MonitorHandle>;
    
    /// 获取所有可用显示器。
    fn available_monitors(&self) -> MonitorHandleIter;
    
    /// 获取主显示器。
    fn primary_monitor(&self) -> Option<MonitorHandle>;
    
    // ========== 主题 ==========
    
    /// 获取窗口主题（浅色/深色）。
    fn theme(&self) -> Theme;
    
    /// 获取 DPI 变化事件通知器。
    fn scale_factor_changed_event_notifier(&self) -> Notifier;
}
```

### EventLoop

```rust
/// EventLoop 是应用的主事件循环。
pub struct EventLoop<T: 'static> {
    // 私有字段
}

impl<T: 'static> EventLoop<T> {
    /// 创建新的 EventLoop。
    pub fn new() -> Self;
    
    /// 运行事件循环（阻塞）。
    /// 
    /// 此方法永不返回，直到收到退出信号。
    pub fn run(self, event_handler: impl FnMut(Event<T>, &mut ControlFlow));
    
    /// 运行事件循环一次（非阻塞）。
    pub fn run_return<F>(&mut self, event_handler: F) -> Result<(), EventLoopError>
    where
        F: FnMut(Event<T>, &mut ControlFlow);
    
    /// 设置控制流行为。
    pub fn set_control_flow(&mut self, control_flow: ControlFlow);
    
    /// 获取当前控制流设置。
    pub fn control_flow(&self) -> ControlFlow;
}

/// EventLoop 代理，用于跨线程通信。
pub struct EventLoopProxy<T: 'static> {
    // 私有字段
}

impl<T: 'static> EventLoopProxy<T> {
    /// 创建事件代理。
    pub fn create_proxy(&self) -> EventLoopProxy<T>;
    
    /// 发送用户事件。
    pub fn send_event(&self, event: T) -> Result<(), EventLoopError>;
    
    /// 唤醒事件循环。
    pub fn wake_up(&self);
}
```

### EventLoopBuilder

```rust
/// EventLoop 构建器。
pub struct EventLoopBuilder<T: 'static> {
    // 私有字段
}

impl<T: 'static> EventLoopBuilder<T> {
    /// 创建新的 EventLoopBuilder。
    pub fn new() -> Self;
    
    /// 添加用户事件类型。
    pub fn with_user_event<U: 'static>(self) -> EventLoopBuilder<U>;
    
    /// 构建 EventLoop。
    pub fn build(&mut self) -> EventLoop<T>;
}
```

### Event

```rust
/// 事件类型。
pub enum Event<T: 'static> {
    /// 窗口事件。
    WindowEvent { window_id: WindowId, event: WindowEvent },
    
    /// 设备事件（键盘、鼠标等）。
    DeviceEvent { device_id: DeviceId, event: DeviceEvent },
    
    /// 新事件开始。
    NewEvents(Cause),
    
    /// 即将进入等待状态。
    AboutToWait,
    
    /// 循环已销毁。
    LoopDestroyed,
    
    /// 用户自定义事件。
    UserEvent(T),
    
    /// 应用暂停。
    Suspended,
    
    /// 应用恢复。
    Resumed,
}

/// 控制流设置。
pub enum ControlFlow {
    /// 持续轮询，不阻塞。
    Poll,
    
    /// 等待事件。
    Wait,
    
    /// 等待直到指定时间。
    WaitUntil(Instant),
    
    /// 退出事件循环。
    Exit,
}
```

### WindowEvent

```rust
/// 窗口事件类型。
pub enum WindowEvent {
    Resized(PhysicalSize<u32>),
    Moved(PhysicalPosition<i32>),
    CloseRequested,
    Destroyed,
    DroppedFile(PathBuf),
    HoveredFile(PathBuf),
    HoveredFileCancelled,
    ReceivedCharacter(char),
    Focused(bool),
    Captured,
    Uncaptured,
    MouseInput { button: MouseButton, state: ElementState, modifiers: ModifiersState },
    MouseWheel { delta: MouseScrollDelta, phase: TouchPhase },
    CursorMoved { position: Position, delta: (f64, f64) },
    CursorEntered,
    CursorLeft,
    DpiChanged { scale_factor: f64, new_inner_size: PhysicalSize<u32> },
    ThemeChanged(Theme),
}
```

### DeviceEvent

```rust
/// 设备事件类型。
pub enum DeviceEvent {
    Keyboard(KeyboardInput),
    MouseMotion { delta: (f64, f64) },
    MouseWheel { delta: MouseScrollDelta },
    Touch(Touch),
}
```

---

## 枚举类型

```rust
/// 全屏模式。
pub enum Fullscreen {
    /// 无边框全屏。
    Borderless(Option<MonitorHandle>),
    /// 独占全屏（特定分辨率）。
    Exclusive(VideoMode),
    /// 窗口模式。
    None,
}

/// 窗口层级。
pub enum WindowLevel {
    Normal,
    AlwaysOnTop,
    AlwaysOnTopBottom,
}

/// 光标图标。
pub enum CursorIcon {
    Default, Crosshair, Hand, Move, Text, Wait, Help, Progress,
    NResize, SResize, EResize, WResize,
    NEResize, NWResize, SEResize, SWResize,
}

/// 光标捕获模式。
pub enum CursorGrabMode {
    None,
    Confined,
    Locked,
}

/// IME 用途。
pub enum ImePurpose {
    Default,
    Password,
    Email,
    URL,
    Text,
    Number,
}

/// 窗口主题。
pub enum Theme {
    Light,
    Dark,
}

/// 调整窗口大小的边。
pub enum ResizeEdge {
    Top, Bottom, Left, Right,
    TopLeft, TopRight, BottomLeft, BottomRight,
}

/// 视频模式。
pub struct VideoMode {
    pub size: PhysicalSize<u32>,
    pub refresh_rate: u32,
    pub monitor: MonitorHandle,
}
```

---

## 尺寸类型

```rust
/// 物理尺寸（像素）。
pub struct PhysicalSize<T> {
    pub width: T,
    pub height: T,
}

/// 逻辑尺寸（与 DPI 无关）。
pub struct LogicalSize<T> {
    pub width: T,
    pub height: T,
}

/// 物理位置。
pub struct PhysicalPosition<T> {
    pub x: T,
    pub y: T,
}

/// 逻辑位置。
pub struct LogicalPosition<T> {
    pub x: T,
    pub y: T,
}
```

---

## 辅助类型

```rust
/// 窗口 ID。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u64);

/// 设备 ID。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(u64);

/// 显示器信息。
pub struct MonitorHandle { /* ... */ }

impl MonitorHandle {
    pub fn name(&self) -> String;
    pub fn size(&self) -> PhysicalSize<u32>;
    pub fn refresh_rate(&self) -> u32;
    pub fn scale_factor(&self) -> f64;
}

/// 图标。
pub struct Icon { /* ... */ }

impl Icon {
    pub fn from_rgba(rgba: &[u8], width: u32, height: u32) -> Result<Self>;
}
```

---

## 公开 API 统计

| 模块 | API 数量 |
|------|---------|
| WindowBuilder | 20 |
| Window (trait) | 32 |
| EventLoop | 5 |
| EventLoopProxy | 3 |
| EventLoopBuilder | 3 |
| Event/WindowEvent/DeviceEvent | 6+ |
| 枚举类型 | 10+ |
| 尺寸类型 | 8 |
| 辅助类型 | 5+ |
| **总计** | **~35-40** |

---

## 中文注释要求

- 所有公开 API 必须有中文 doc comment
- 示例代码应包含中文注释
- 复杂类型应有使用说明

---

## 英文注释要求

- 中文注释与英文注释并存
- 英文注释用于国际化和代码可读性
