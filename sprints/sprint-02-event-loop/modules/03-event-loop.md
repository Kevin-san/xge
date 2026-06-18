# 事件循环需求

## 模块概述

EventLoop 是游戏引擎的核心驱动，负责窗口事件分发、主循环控制（poll/update/render）、固定时间步支持、帧率限制等。EventLoop 运行后阻塞直到窗口关闭或收到退出信号。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 47-50, 102-110, 171-189, 204-223

---

## 需求详情

### EventLoop 构造与运行

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 47 | EventLoop 包装 | `EventLoop::new()` | 输出: Self | P0 |
| 204 | EventLoop 新建 | `EventLoop::new()` | 输出: Self | P0 |
| 25 | 启动并阻塞 | `EventLoop::run(handler)` | 输入: EventHandler → 输出: !（never returns） | P0 |
| 174 | 运行（非阻塞） | `EventLoop::run_return(handler)` | 输入: EventHandler → 输出: () | P1 |
| 175 | 创建代理 | `EventLoopProxy::create_proxy(&self)` | 输入: &Self → 输出: EventLoopProxy | P1 |
| 176 | 发送用户事件 | `EventLoopProxy::send_event(user_event)` | 输入: user_event → 输出: Result | P1 |
| 177 | 唤醒事件循环 | `EventLoopProxy::wake_up()` | 输入: &Self → 输出: () | P1 |
| 189 | 获取代理 | `Engine::event_loop_proxy(&self) -> EventLoopProxy` | 输入: &Self → 输出: EventLoopProxy | P1 |

### EventLoopBuilder

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 205 | EventLoopBuilder 新建 | `EventLoopBuilder::new()` | 输出: Self | P0 |
| 206 | 添加用户事件类型 | `EventLoopBuilder::with_user_event::<T>()` | 输入: UserEvent 类型 → 输出: Self | P1 |
| 50 | 支持自定义事件 | `EventLoopBuilder::with_user_event()` 支持自定义事件 | 输入: 事件类型 → 输出: Self | P1 |

### 控制流

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 187 | 控制流枚举 | `EventLoopControlFlow` 枚举：Poll / Wait / WaitUntil / Exit | 输出: 枚举 | P0 |
| 188 | 设置控制流 | `EventLoop::set_control_flow(control_flow)` | 输入: ControlFlow → 输出: () | P0 |
| 189 | 获取控制流 | `EventLoop::control_flow(&self)` | 输入: &Self → 输出: ControlFlow | P0 |

### 事件类型

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 179 | 窗口事件 | `Event::WindowEvent { window_id, event }` | 输入: WindowId, WindowEvent → 输出: Event | P0 |
| 180 | 设备事件 | `Event::DeviceEvent { device_id, event }` | 输入: DeviceId, DeviceEvent → 输出: Event | P0 |
| 181 | 新事件开始 | `Event::NewEvents(cause)` | 输入: Cause → 输出: Event | P0 |
| 182 | 即将等待 | `Event::AboutToWait` | 输出: Event | P0 |
| 183 | 循环已销毁 | `Event::LoopDestroyed` | 输出: Event | P0 |
| 184 | 用户事件 | `Event::UserEvent(T)` | 输入: T → 输出: Event | P1 |
| 185 | 暂停事件 | `Event::Suspended` | 输出: Event | P1 |
| 186 | 恢复事件 | `Event::Resumed` | 输出: Event | P1 |
| 212 | WindowEvent 变体 | `Event::WindowEvent { window_id, event }` | 输入: WindowId, WindowEvent → 输出: Event | P0 |
| 213 | DeviceEvent 变体 | `Event::DeviceEvent { device_id, event }` | 输入: DeviceId, DeviceEvent → 输出: Event | P0 |
| 214 | NewEvents 变体 | `Event::NewEvents(cause)` | 输入: cause → 输出: Event | P0 |
| 215 | AboutToWait 变体 | `Event::AboutToWait` | 输出: Event | P0 |
| 216 | LoopDestroyed 变体 | `Event::LoopDestroyed` | 输出: Event | P0 |
| 217 | UserEvent 变体 | `Event::UserEvent(T)` | 输入: T → 输出: Event | P1 |

### 主循环

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 84 | 主循环 poll -> update -> render | 主循环结构 | 输入: DeltaTime → 输出: () | P0 |
| 85 | 固定时间步支持 | `update(dt)` 多次调用以补偿 | 输入: dt → 输出: 多次调用 | P0 |
| 86 | 大 dt 钳制 | max_dt 防止尖峰 | 输入: dt → 输出: clamp(dt) | P0 |
| 108 | vsync 开启/关闭可配置 | vsync 配置项 | 输入: bool → 输出: () | P1 |
| 109 | 帧率限制可配置 | max_fps 配置项 | 输入: u32 → 输出: () | P1 |
| 110 | WindowConfig 结构体 | 从配置文件加载 | 输入: 文件路径 → 输出: Config | P1 |

### Engine 事件接口

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 82 | EngineEvent 与 WindowEvent 分离 | EngineEvent 作为用户自定义事件 | 输入: T → 输出: Event | P1 |
| 102 | Event trait | `Event` trait — 泛型事件类型 | 输入: T → 输出: bool | P1 |
| 81 | 事件派发使用 EventReader/EventWriter | 模式 | 输入: &EventWriter → 输出: () | P0 |
| 83 | 获取事件读取器 | `Engine::events<T>() -> EventReader<T>` | 输入: &Self → 输出: EventReader<T> | P0 |
| 104 | 发送事件 | `Engine::send_event<T>(event: T)` | 输入: event → 输出: () | P0 |

### Engine 窗口管理

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 51 | 获取主窗口引用 | `Engine::window(&self) -> &Window` | 输入: &Self → 输出: &Window | P0 |
| 52 | 切换窗口模式 | `Engine::set_window_mode(mode: WindowMode)` | 输入: mode → 输出: () | P1 |
| 53 | 请求退出 | `Engine::request_close(&self)` | 输入: &Self → 输出: () | P0 |
| 100 | WindowMode 枚举 | 窗口模式枚举 | 输出: WindowMode | P1 |
| 207 | 窗口模式切换 | `Engine::set_window_mode(...)` | 输入: WindowMode → 输出: () | P1 |
| 208 | 请求关闭 | `Engine::request_close(&self)` | 输入: &Self → 输出: () | P0 |

### Engine 光标控制

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 54 | 显示/隐藏光标 | `Engine::show_cursor(&self, bool)` | 输入: bool → 输出: () | P1 |
| 55 | 捕获/释放光标 | `Engine::set_cursor_grab(&self, bool)` | 输入: bool → 输出: Result | P1 |
| 56 | 设置光标图标 | `Engine::set_cursor_icon(&self, icon: CursorIcon)` | 输入: icon → 输出: () | P1 |

### Engine IME

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 57 | 设置 IME 允许 | `Engine::set_ime_allowed(&self, bool)` | 输入: bool → 输出: () | P2 |
| 58 | 设置 IME 位置 | `Engine::set_ime_position(&self, position)` | 输入: position → 输出: () | P2 |

### Engine 剪贴板（预留）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 107 | Clipboard 支持（预留接口） | `Clipboard::get_text / set_text` | 输入: &str → 输出: Option<String> | P2 |
| 125 | 获取剪贴板文本 | `Clipboard::get_text(&self) -> Option<String>` | 输入: &Self → 输出: Option<String> | P2 |
| 126 | 设置剪贴板文本 | `Clipboard::set_text(&self, text) -> Result<()>` | 输入: &Self, text → 输出: Result | P2 |

---

## API 签名汇总

### EventLoop

```rust
pub struct EventLoop<T: 'static> {
    // 内部状态
}

impl<T> EventLoop<T> {
    pub fn new() -> Self;
    pub fn run(self, event_handler: impl FnMut(Event<T>, &mut ControlFlow));
    pub fn run_return<F>(&mut self, event_handler: F) -> Result<(), EventLoopError>
    where F: FnMut(Event<T>, &mut ControlFlow);
}

pub struct EventLoopProxy<T: 'static> {
    // 内部状态
}

impl<T> EventLoopProxy<T> {
    pub fn create_proxy(&self) -> EventLoopProxy<T>;
    pub fn send_event(&self, event: T) -> Result<(), EventLoopError>;
    pub fn wake_up(&self);
}

pub enum ControlFlow {
    Poll,
    Wait,
    WaitUntil(Instant),
    Exit,
}

pub enum Event<T: 'static> {
    WindowEvent { window_id: WindowId, event: WindowEvent },
    DeviceEvent { device_id: DeviceId, event: DeviceEvent },
    NewEvents(Cause),
    AboutToWait,
    LoopDestroyed,
    UserEvent(T),
    Suspended,
    Resumed,
}
```

### EventLoopBuilder

```rust
pub struct EventLoopBuilder<T: 'static> {
    // 内部状态
}

impl<T: 'static> EventLoopBuilder<T> {
    pub fn new() -> Self;
    pub fn with_user_event<U: 'static>(self) -> EventLoopBuilder<U>;
    pub fn build(&mut self) -> EventLoop<T>;
}
```

### Engine

```rust
pub trait Engine {
    fn window(&self) -> &Window;
    fn set_window_mode(&mut self, mode: WindowMode);
    fn request_close(&self);
    fn show_cursor(&self, visible: bool);
    fn set_cursor_grab(&self, grab: bool) -> Result<()>;
    fn set_cursor_icon(&self, icon: CursorIcon);
    fn set_ime_allowed(&self, allowed: bool);
    fn set_ime_position(&self, position: Position);
    fn is_focused(&self) -> bool;
    fn is_minimized(&self) -> bool;
    fn is_maximized(&self) -> bool;
    fn is_visible(&self) -> bool;
    fn event_loop_proxy(&self) -> EventLoopProxy;
    fn events<T: 'static>(&self) -> EventReader<T>;
    fn send_event<T: 'static>(&self, event: T);
}
```

### WindowConfig

```rust
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub vsync: bool,
    pub fullscreen: Fullscreen,
    pub decorations: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "engine".to_string(),
            width: 1280,
            height: 720,
            resizable: true,
            vsync: true,
            fullscreen: Fullscreen::None,
            decorations: true,
        }
    }
}
```

---

## 输入/输出

### EventLoop::run
- **输入**: `event_handler: FnMut(Event<T>, &mut ControlFlow)`
- **输出**: `!` (never returns，程序终止)

### EventHandler 签名
```rust
fn event_handler(event: Event<UserEvent>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent { window_id, event } => { /* 处理 */ }
        Event::AboutToWait => { /* update 和 render */ }
        _ => {}
    }
}
```

---

## 主循环流程

```
loop {
    // 1. NewEvents
    let cause = if first_event { StartCause::Init } else { StartCause::Poll };
    event_handler(Event::NewEvents(cause), &mut control_flow);
    
    // 2. 处理窗口事件
    for event in pending_events {
        event_handler(event, &mut control_flow);
    }
    
    // 3. AboutToWait - 准备渲染
    event_handler(Event::AboutToWait, &mut control_flow);
    
    // 4. Poll 模式：立即返回；Wait 模式：等待事件
    if matches!(control_flow, ControlFlow::Wait) {
        wait_for_events();
    }
}
```

### 固定时间步

```rust
const FIXED_DT: f64 = 1.0 / 60.0;
const MAX_DT: f64 = 0.25;

fn update_loop(mut accumulator: f64, actual_dt: f64) {
    accumulator += actual_dt.min(MAX_DT);
    
    while accumulator >= FIXED_DT {
        update(FIXED_DT);
        accumulator -= FIXED_DT;
    }
}
```

---

## 验收标准

| 验收项 | 标准 |
|-------|------|
| EventLoop 创建 | `EventLoop::new()` 成功创建 |
| 阻塞运行 | `event_loop.run(handler)` 阻塞直到收到退出 |
| 非阻塞运行 | `event_loop.run_return()` 执行一次事件处理后返回 |
| 用户事件 | `proxy.send_event()` 可从其他线程发送事件 |
| ControlFlow::Poll | CPU 占用 100%，持续轮询 |
| ControlFlow::Wait | 空闲时阻塞等待事件 |
| ControlFlow::WaitUntil | 阻塞到指定时间 |
| 主循环固定步长 | 60 FPS 下 update 每帧调用约 1 次 |

---

## 依赖关系

- **内部依赖**: Window、InputModule
- **外部依赖**: `winit` crate
- **被依赖模块**: InputModule、RenderModule（后续）

---

## 优先级定义

- **P0**: 核心功能，必须在 Sprint 02 完成
- **P1**: 重要功能，应在 Sprint 02 完成  
- **P2**: 增强功能，可延后到后续 Sprint
