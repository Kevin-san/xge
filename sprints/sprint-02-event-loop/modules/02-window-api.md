# Window API 需求

## 模块概述

Window API 提供对窗口各项属性的查询与修改能力。包括窗口尺寸、位置、标题、光标、全屏模式、显示器信息等。Window 对象由 WindowBuilder 构建生成，提供只读查询和可变修改两种操作方式。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 31-56, 130-170, 195-200

---

## 需求详情

### 窗口尺寸

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 31 | 获取内部尺寸 | `Window::inner_size(&self) -> PhysicalSize<u32>` | 输入: &Self → 输出: PhysicalSize | P0 |
| 32 | 获取外部尺寸 | `Window::outer_size(&self) -> PhysicalSize<u32>` | 输入: &Self → 输出: PhysicalSize | P0 |
| 132 | 获取内部尺寸（精确） | `Window::inner_size(&self) -> PhysicalSize<u32>` | 输入: &Self → 输出: PhysicalSize<u32> | P0 |
| 133 | 获取外部尺寸（精确） | `Window::outer_size(&self) -> PhysicalSize<u32>` | 输入: &Self → 输出: PhysicalSize<u32> | P0 |
| 163 | 获取缩放因子 | `Window::scale_factor(&self) -> f64` | 输入: &Self → 输出: f64 | P1 |
| 34 | 设置内部尺寸 | `Window::set_inner_size(&self, size: PhysicalSize<u32>)` | 输入: &Self, Size → 输出: () | P0 |
| 35 | 设置最小尺寸 | `Window::set_min_size(&self, size: Option<PhysicalSize<u32>>)` | 输入: &Self, Option → 输出: () | P1 |
| 36 | 设置最大尺寸 | `Window::set_max_size(&self, size: Option<PhysicalSize<u32>>)` | 输入: &Self, Option → 输出: () | P1 |
| 139 | 设置尺寸 | `Window::set_dimensions(&self, size: PhysicalSize<u32>)` | 输入: &Self, Size → 输出: () | P0 |
| 140 | 设置最小尺寸 | `Window::set_min_dimensions(&self, size: Option<PhysicalSize<u32>>)` | 输入: &Self, Option → 输出: () | P1 |
| 141 | 设置最大尺寸 | `Window::set_max_dimensions(&self, size: Option<PhysicalSize<u32>>)` | 输入: &Self, Option → 输出: () | P1 |

### 窗口位置

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 134 | 获取位置 | `Window::position(&self) -> PhysicalPosition<i32>` | 输入: &Self → 输出: PhysicalPosition | P1 |
| 135 | 获取内部位置 | `Window::inner_position(&self) -> PhysicalPosition<i32>` | 输入: &Self → 输出: PhysicalPosition | P1 |
| 136 | 获取外部位置 | `Window::outer_position(&self) -> PhysicalPosition<i32>` | 输入: &Self → 输出: PhysicalPosition | P1 |

### 窗口属性

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 33 | 设置标题 | `Window::set_title(&self, title: impl Into<String>)` | 输入: &Self, String → 输出: () | P0 |
| 37 | 设置可调整 | `Window::set_resizable(&self, resizable: bool)` | 输入: &Self, bool → 输出: () | P0 |
| 38 | 设置全屏模式 | `Window::set_fullscreen(&self, mode: Fullscreen)` | 输入: &Self, Fullscreen → 输出: () | P1 |
| 39 | 设置装饰 | `Window::set_decorations(&self, decorations: bool)` | 输入: &Self, bool → 输出: () | P1 |
| 40 | 设置置顶 | `Window::set_always_on_top(&self, always_on_top: bool)` | 输入: &Self, bool → 输出: () | P2 |
| 42 | 设置可见性 | `Window::set_visible(&self, visible: bool)` | 输入: &Self, bool → 输出: () | P0 |
| 130 | 获取窗口 ID | `Window::id(&self) -> WindowId` | 输入: &Self → 输出: WindowId | P0 |
| 131 | 获取标题 | `Window::title(&self) -> String` | 输入: &Self → 输出: String | P0 |
| 142 | 设置可调整 | `Window::set_resizable(&self, resizable: bool)` | 输入: &Self, bool → 输出: () | P0 |
| 143 | 设置最小化 | `Window::set_minimized(&self, minimized: bool)` | 输入: &Self, bool → 输出: () | P1 |
| 144 | 设置最大化 | `Window::set_maximized(&self, maximized: bool)` | 输入: &Self, bool → 输出: () | P1 |
| 145 | 设置可见性 | `Window::set_visible(&self, visible: bool)` | 输入: &Self, bool → 输出: () | P0 |
| 146 | 设置置顶 | `Window::set_always_on_top(&self, always_on_top: bool)` | 输入: &Self, bool → 输出: () | P2 |
| 147 | 设置全屏 | `Window::set_fullscreen(&self, mode: Fullscreen)` | 输入: &Self, Fullscreen → 输出: () | P1 |
| 148 | 设置装饰 | `Window::set_decorations(&self, decorations: bool)` | 输入: &Self, bool → 输出: () | P1 |
| 149 | 设置窗口层级 | `Window::set_window_level(&self, level: WindowLevel)` | 输入: &Self, WindowLevel → 输出: () | P2 |
| 172 | 设置最小化 | `Window::set_minimized(&self, minimized: bool)` | 输入: &Self, bool → 输出: () | P1 |
| 173 | 设置最大化 | `Window::set_maximized(&self, maximized: bool)` | 输入: &Self, bool → 输出: () | P1 |

### 光标控制

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 54 | 设置光标图标 | `Window::set_cursor_icon(&self, cursor: CursorIcon)` | 输入: &Self, CursorIcon → 输出: () | P1 |
| 55 | 设置光标位置 | `Window::set_cursor_position(&self, pos: Position) -> Result<()>` | 输入: &Self, Position → 输出: Result | P1 |
| 56 | 捕获光标 | `Window::set_cursor_grab(&self, grab: bool) -> Result<()>` | 输入: &Self, bool → 输出: Result | P1 |
| 174 | 设置光标图标 | `Window::set_cursor_icon(&self, cursor: CursorIcon)` | 输入: &Self, CursorIcon → 输出: () | P1 |
| 175 | 设置光标位置 | `Window::set_cursor_position(&self, pos: Position) -> Result<()>` | 输入: &Self, Position → 输出: Result | P1 |
| 176 | 捕获光标 | `Window::set_cursor_grab(&self, mode: CursorGrabMode)` | 输入: &Self, mode → 输出: Result | P1 |
| 177 | 设置光标命中测试 | `Window::set_cursor_hittest(&self, enabled: bool)` | 输入: &Self, bool → 输出: () | P2 |
| 178 | 设置光标可见 | `Window::set_cursor_visible(&self, visible: bool)` | 输入: &Self, bool → 输出: () | P1 |
| 155 | 设置光标图标 | `Window::set_cursor_icon(&self, cursor: CursorIcon)` | 输入: &Self, CursorIcon → 输出: () | P1 |
| 185 | 设置光标位置 | `Window::set_cursor_position(&self, pos: Position) -> Result<()>` | 输入: &Self, Position → 输出: Result | P1 |
| 186 | 捕获光标 | `Window::set_cursor_grab(&self, mode: CursorGrabMode)` | 输入: &Self, mode → 输出: Result | P1 |

### IME 输入法

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 57 | 设置 IME 允许 | `Window::set_ime_allowed(&self, allowed: bool)` | 输入: &Self, bool → 输出: () | P2 |
| 58 | 设置 IME 位置 | `Window::set_ime_position(&self, position: Position)` | 输入: &Self, Position → 输出: () | P2 |
| 181 | 设置 IME 允许 | `Window::set_ime_allowed(&self, allowed: bool)` | 输入: &Self, bool → 输出: () | P2 |
| 182 | 设置 IME 光标区域 | `Window::set_ime_cursor_area(&self, pos: Position, size: Size)` | 输入: &Self, pos, size → 输出: () | P2 |
| 183 | 设置 IME 目的 | `Window::set_ime_purpose(&self, purpose: ImePurpose)` | 输入: &Self, purpose → 输出: () | P2 |

### 窗口操作

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 41 | 请求重绘 | `Window::request_redraw(&self)` | 输入: &Self → 输出: () | P0 |
| 59 | 拖动窗口 | `Window::drag_window(&self) -> Result<()>` | 输入: &Self → 输出: Result | P2 |
| 60 | 拖动调整窗口 | `Window::drag_resize_window(&self, edge: ResizeEdge)` | 输入: &Self, edge → 输出: Result | P2 |
| 61 | 聚焦窗口 | `Window::focus_window(&self)` | 输入: &Self → 输出: () | P2 |
| 62 | 显示窗口菜单 | `Window::show_window_menu(&self, pos: Position)` | 输入: &Self, pos → 输出: () | P2 |
| 167 | 请求重绘 | `Window::request_redraw(&self)` | 输入: &Self → 输出: () | P0 |
| 179 | 拖动窗口 | `Window::drag_window(&self) -> Result<()>` | 输入: &Self → 输出: Result | P2 |
| 180 | 拖动调整窗口 | `Window::drag_resize_window(&self, edge: ResizeEdge)` | 输入: &Self, edge → 输出: Result | P2 |
| 181 | 聚焦窗口 | `Window::focus_window(&self)` | 输入: &Self → 输出: () | P2 |
| 182 | 显示窗口菜单 | `Window::show_window_menu(&self, pos: Position)` | 输入: &Self, pos → 输出: () | P2 |

### 底层句柄

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 45 | 获取原始显示句柄 | `Window::raw_display_handle(&self) -> RawDisplayHandle` | 输入: &Self → 输出: RawDisplayHandle | P0 |
| 46 | 获取原始窗口句柄 | `Window::raw_window_handle(&self) -> RawWindowHandle` | 输入: &Self → 输出: RawWindowHandle | P0 |
| 194 | 获取原始显示句柄 | `Window::raw_display_handle(&self) -> RawDisplayHandle` | 输入: &Self → 输出: RawDisplayHandle | P0 |
| 195 | 获取原始窗口句柄 | `Window::raw_window_handle(&self) -> RawWindowHandle` | 输入: &Self → 输出: RawWindowHandle | P0 |

### 显示器信息

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 44 | 获取当前显示器 | `Window::current_monitor(&self) -> Option<MonitorHandle>` | 输入: &Self → 输出: Option | P1 |
| 166 | 获取当前显示器 | `Window::current_monitor(&self) -> Option<MonitorHandle>` | 输入: &Self → 输出: Option | P1 |
| 196 | 获取可用显示器列表 | `Window::available_monitors(&self) -> MonitorHandleIter` | 输入: &Self → 输出: Iter | P2 |
| 197 | 获取主显示器 | `Window::primary_monitor(&self) -> Option<MonitorHandle>` | 输入: &Self → 输出: Option | P2 |

### 主题与 DPI

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 199 | 获取窗口主题 | `Window::theme(&self) -> Theme` | 输入: &Self → 输出: Theme | P2 |
| 200 | DPI 变化事件通知 | `Window::scale_factor_changed_event_notifier(&self)` | 输入: &Self → 输出: () | P1 |

### 窗口状态查询（Engine 级别）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 103 | 查询焦点状态 | `Engine::is_focused(&self) -> bool` | 输入: &Self → 输出: bool | P1 |
| 104 | 查询最小化状态 | `Engine::is_minimized(&self) -> bool` | 输入: &Self → 输出: bool | P1 |
| 105 | 查询最大化状态 | `Engine::is_maximized(&self) -> bool` | 输入: &Self → 输出: bool | P1 |
| 106 | 查询可见状态 | `Engine::is_visible(&self) -> bool` | 输入: &Self → 输出: bool | P1 |

---

## API 签名汇总

### Window Trait

```rust
pub trait Window {
    // 尺寸
    fn inner_size(&self) -> PhysicalSize<u32>;
    fn outer_size(&self) -> PhysicalSize<u32>;
    fn scale_factor(&self) -> f64;
    
    // 位置
    fn position(&self) -> PhysicalPosition<i32>;
    fn inner_position(&self) -> PhysicalPosition<i32>;
    fn outer_position(&self) -> PhysicalPosition<i32>;
    
    // 属性
    fn id(&self) -> WindowId;
    fn title(&self) -> String;
    
    // 设置
    fn set_title(&mut self, title: impl Into<String>);
    fn set_inner_size(&mut self, size: PhysicalSize<u32>);
    fn set_min_size(&mut self, size: Option<PhysicalSize<u32>>);
    fn set_max_size(&mut self, size: Option<PhysicalSize<u32>>);
    fn set_resizable(&mut self, resizable: bool);
    fn set_minimized(&mut self, minimized: bool);
    fn set_maximized(&mut self, maximized: bool);
    fn set_visible(&mut self, visible: bool);
    fn set_always_on_top(&mut self, always_on_top: bool);
    fn set_fullscreen(&mut self, mode: Fullscreen);
    fn set_decorations(&mut self, decorations: bool);
    fn set_window_level(&mut self, level: WindowLevel);
    
    // 光标
    fn set_cursor_icon(&mut self, cursor: CursorIcon);
    fn set_cursor_position(&mut self, pos: Position) -> Result<()>;
    fn set_cursor_grab(&mut self, grab: bool) -> Result<()>;
    fn set_cursor_visible(&mut self, visible: bool);
    
    // IME
    fn set_ime_allowed(&mut self, allowed: bool);
    fn set_ime_position(&mut self, position: Position);
    
    // 窗口操作
    fn request_redraw(&self);
    fn drag_window(&self) -> Result<()>;
    fn drag_resize_window(&self, edge: ResizeEdge);
    fn focus_window(&self);
    
    // 底层句柄
    fn raw_display_handle(&self) -> RawDisplayHandle;
    fn raw_window_handle(&self) -> RawWindowHandle;
    fn current_monitor(&self) -> Option<MonitorHandle>;
}
```

### 辅助枚举

```rust
pub enum Fullscreen {
    Borderless(Option<MonitorHandle>),
    Exclusive(VideoMode),
    None,
}

pub enum WindowLevel {
    Normal,
    AlwaysOnTop,
    AlwaysOnTopBottom,
}

pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NResize,
    SResize,
    EResize,
    WResize,
    // ... 12+ 方向
}

pub enum CursorGrabMode {
    None,
    Confined,
    Locked,
}

pub enum Theme {
    Light,
    Dark,
}

pub enum ImePurpose {
    Default,
    Password,
    Email,
    URL,
    Text,
    Number,
}
```

---

## 输入/输出

### 查询操作（只读）
- `&self` → 返回对应属性值
- 无副作用

### 修改操作（可变）
- `&mut self` + 新值 → 修改对应属性
- 返回 `()` 或 `Result<()>`（光标操作可能失败）

### 底层句柄
- 用于接入图形 API（Vulkan、OpenGL 等）

---

## 验收标准

| 验收项 | 标准 |
|-------|------|
| 尺寸获取 | `window.inner_size()` 返回物理像素尺寸 |
| 尺寸设置 | `window.set_inner_size(800x600)` 后 `inner_size()` 返回新尺寸 |
| 标题设置 | `window.set_title("New Title")` 后 `title()` 返回 "New Title" |
| 全屏切换 | `set_fullscreen(Borderless(None))` 进入无边框全屏 |
| 光标隐藏 | `set_cursor_visible(false)` 隐藏光标 |
| 光标捕获 | `set_cursor_grab(true)` 锁定光标在窗口内 |
| 底层句柄 | `raw_window_handle()` 返回可用于图形 API 的句柄 |

---

## 依赖关系

- **内部依赖**: WindowBuilder、dpi 模块
- **外部依赖**: 
  - `winit` crate
  - `raw-window-handle 0.5`
- **被依赖模块**: EventLoop、InputModule

---

## 优先级定义

- **P0**: 核心功能，必须在 Sprint 02 完成
- **P1**: 重要功能，应在 Sprint 02 完成  
- **P2**: 增强功能，可延后到后续 Sprint
