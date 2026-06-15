# 输入系统需求

## 模块概述

InputSystem 负责处理所有输入设备（键盘、鼠标、触摸、手柄）的事件，并提供状态查询接口。输入状态分为"按下"（持续）和"刚刚按下/释放"（瞬时）两种，通过每帧调用 `clear()` 重置瞬时状态。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 56-78, 95, 191-229, 241-245

---

## 需求详情

### 窗口事件

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 63 | DPI 变化事件 | `WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size }` | 输入: f64, Size → 输出: Event | P1 |
| 64 | 最小化事件 | `WindowEvent::Minimized` | 输出: Event | P0 |
| 64 | 最大化事件 | `WindowEvent::Maximized` | 输出: Event | P0 |
| 64 | 恢复事件 | `WindowEvent::Restored` | 输出: Event | P0 |
| 64 | 关闭事件 | `WindowEvent::CloseRequested` | 输出: Event | P0 |
| 65 | 获得焦点事件 | `WindowEvent::Focused(true)` | 输出: Event | P1 |
| 65 | 失去焦点事件 | `WindowEvent::Focused(false)` | 输出: Event | P1 |
| 66 | 悬停进入事件 | `WindowEvent::Hovered(true)` | 输出: Event | P1 |
| 66 | 悬停离开事件 | `WindowEvent::Hovered(false)` | 输出: Event | P1 |
| 67 | 重绘请求事件 | `WindowEvent::RedrawRequested` | 输出: Event | P0 |

### 键盘事件

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 68 | KeyboardInput 事件 | `KeyboardInput { keycode, scancode, state, repeat }` | 输入: KeyCode, ScanCode, ElementState, bool → 输出: Event | P0 |
| 69 | ModifiersState | `ModifiersState { shift, ctrl, alt, meta, logo }` | 输入: bool × 5 → 输出: ModifiersState | P0 |
| 70 | ReceivedCharacter | `ReceivedCharacter(char)` 用于文本输入 | 输入: char → 输出: Event | P1 |

### 鼠标事件

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 72 | MouseButton 枚举 | `MouseButton::Left / Middle / Right / Other(u16)` | 输出: 枚举 | P0 |
| 73 | MouseInput 事件 | `MouseInput { button, state, modifiers }` | 输入: MouseButton, ElementState, ModifiersState → 输出: Event | P0 |
| 74 | CursorMoved 事件 | `CursorMoved { position, delta }` | 输入: Position, Delta → 输出: Event | P0 |
| 75 | CursorEntered 事件 | `CursorEntered` | 输出: Event | P1 |
| 75 | CursorLeft 事件 | `CursorLeft` | 输出: Event | P1 |
| 76 | MouseWheel 事件 | `MouseWheel { delta, phase }` | 输入: MouseScrollDelta, TouchPhase → 输出: Event | P0 |
| 221 | MouseButton 枚举 | `MouseButton::Left / Middle / Right` + `Other(u16)` | 输出: 枚举 | P0 |
| 222 | MouseScrollDelta | `LineDelta(x, y) / PixelDelta(x, y)` | 输出: 枚举 | P0 |

### 触摸事件

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 77 | Touch 事件 | `Touch { phase, location, force, id }` | 输入: TouchPhase, Position, Option<Force>, u64 → 输出: Event | P1 |
| 78 | TouchPhase 枚举 | `Started / Moved / Ended / Cancelled` | 输出: 枚举 | P1 |
| 223 | TouchPhase 枚举 | `Started / Moved / Ended / Cancelled` | 输出: 枚举 | P1 |
| 224 | Touch 结构体 | `id, position, force, phase` | 输出: 结构体 | P1 |
| 57 | Touch 事件（phase + location + force + id） | 同上 | 输入: TouchPhase, Position, Force, u64 → 输出: Event | P1 |

### 手柄事件（预留）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 79 | GamepadEvent 留位 | 预留接口 | - | P2 |

### Input 状态快照

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 81 | Input 状态快照 | 键盘/鼠标/触摸状态 | 输入: 事件 → 输出: 更新状态 | P0 |
| 191 | Input 新建 | `Input::new()` | 输出: Self | P0 |
| 192 | Input 清除 | `Input::clear(&mut self)` | 输入: &mut Self → 输出: () | P0 |
| 193 | Input 重置 | `Input::reset(&mut self)` | 输入: &mut Self → 输出: () | P0 |
| 228 | clear 与 reset 区别 | clear 重置瞬时状态，reset 完全重置 | 见验收标准 | P0 |

### 键盘状态查询

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 58 | 键是否按下 | `Input::key_pressed(KeyCode) -> bool` | 输入: KeyCode → 输出: bool | P0 |
| 59 | 键刚刚按下 | `Input::key_just_pressed(KeyCode) -> bool` | 输入: KeyCode → 输出: bool | P0 |
| 60 | 键刚刚释放 | `Input::key_just_released(KeyCode) -> bool` | 输入: KeyCode → 输出: bool | P0 |
| 194 | 键按下查询 | `Input::key_pressed(&self, keycode: KeyCode)` | 输入: &Self, KeyCode → 输出: bool | P0 |
| 195 | 键刚刚按下查询 | `Input::key_just_pressed(&self, keycode: KeyCode)` | 输入: &Self, KeyCode → 输出: bool | P0 |
| 196 | 键刚刚释放查询 | `Input::key_just_released(&self, keycode: KeyCode)` | 输入: &Self, KeyCode → 输出: bool | P0 |
| 207 | 按下的键列表 | `Input::pressed_keys(&self)` | 输入: &Self → 输出: Vec<KeyCode> | P0 |
| 208 | 释放的键列表 | `Input::released_keys(&self)` | 输入: &Self → 输出: Vec<KeyCode> | P1 |

### 鼠标状态查询

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 62 | 鼠标按钮按下 | `Input::mouse_button_pressed(button) -> bool` | 输入: MouseButton → 输出: bool | P0 |
| 63 | 鼠标按钮刚刚按下 | `Input::mouse_button_just_pressed(button) -> bool` | 输入: MouseButton → 输出: bool | P0 |
| 64 | 鼠标按钮刚刚释放 | `Input::mouse_button_just_released(button) -> bool` | 输入: MouseButton → 输出: bool | P0 |
| 65 | 鼠标位置 | `Input::mouse_position(&self) -> Vec2` | 输入: &Self → 输出: Vec2 | P0 |
| 66 | 鼠标移动增量 | `Input::mouse_delta(&self) -> Vec2` | 输入: &Self → 输出: Vec2 | P0 |
| 90 | 滚轮增量 | `Input::wheel_delta(&self) -> Vec2` | 输入: &Self → 输出: Vec2 | P0 |
| 197 | 鼠标按钮按下 | `Input::mouse_pressed(&self, button: MouseButton)` | 输入: &Self, MouseButton → 输出: bool | P0 |
| 198 | 鼠标按钮刚刚按下 | `Input::mouse_just_pressed(&self, button: MouseButton)` | 输入: &Self, MouseButton → 输出: bool | P0 |
| 199 | 鼠标按钮刚刚释放 | `Input::mouse_just_released(&self, button: MouseButton)` | 输入: &Self, MouseButton → 输出: bool | P0 |
| 200 | 鼠标位置 | `Input::mouse_position(&self) -> Vec2` | 输入: &Self → 输出: Vec2 | P0 |
| 201 | 鼠标移动增量 | `Input::mouse_delta(&self) -> Vec2` | 输入: &Self → 输出: Vec2 | P0 |
| 202 | 滚轮增量 | `Input::wheel_delta(&self) -> Vec2` | 输入: &Self → 输出: Vec2 | P0 |
| 209 | 按下的按钮列表 | `Input::pressed_buttons(&self)` | 输入: &Self → 输出: Vec<MouseButton> | P0 |
| 210 | 释放的按钮列表 | `Input::released_buttons(&self)` | 输入: &Self → 输出: Vec<MouseButton> | P1 |

### 修饰键状态

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 69 | ModifiersState | `Input::modifiers(&self) -> ModifiersState` | 输入: &Self → 输出: ModifiersState | P0 |
| 203 | 修饰键状态 | `Input::modifiers(&self)` | 输入: &Self → 输出: ModifiersState | P0 |
| 225 | ModifiersState bitflags | SHIFT / CTRL / ALT / SUPER 组合操作 | 输出: ModifiersState | P0 |

### 光标状态

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 204 | 光标是否在窗口内 | `Input::set_cursor_in_window(&self, bool)` | 输入: bool → 输出: () | P1 |

### 文本输入

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 205 | 文本输入 | `Input::text(&self) -> &str` | 输入: &Self → 输出: &str | P1 |

### 触摸状态查询

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 91 | 触摸点数 | `Input::touch_count(&self) -> usize` | 输入: &Self → 输出: usize | P1 |
| 92 | 触摸迭代器 | `Input::touches(&self) -> impl Iterator` | 输入: &Self → 输出: Iterator<Item=Touch> | P1 |
| 211 | 触摸列表 | `Input::touches(&self)` | 输入: &Self → 输出: Vec<Touch> | P1 |
| 212 | 单点触摸 | `Input::touch(&self, id: u64)` | 输入: &Self, id → 输出: Option<Touch> | P1 |

### InputModule

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 96 | InputModule 实现 | `on_event` 订阅窗口事件并更新 `Input` | 输入: Event → 输出: () | P0 |
| 226 | InputModule 新建 | `InputModule::new()` | 输出: Self | P0 |
| 227 | 处理事件 | `InputModule::process_event(&mut self, event: Event)` | 输入: &mut Self, Event → 输出: () | P0 |
| 228 | 获取 Input 引用 | `InputModule::input(&self) -> &Input` | 输入: &Self → 输出: &Input | P0 |
| 229 | 获取 Input 可变引用 | `InputModule::input_mut(&mut self) -> &mut Input` | 输入: &mut Self → 输出: &mut Input | P0 |

### 事件迭代器

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 206 | 事件迭代器 | `Input::events(&self) -> impl Iterator` | 输入: &Self → 输出: Iterator | P1 |

---

## API 签名汇总

### Input 结构体

```rust
pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
    touches: Vec<Touch>,
    modifiers: ModifiersState,
}

impl Input {
    pub fn new() -> Self;
    
    // 键盘
    pub fn key_pressed(&self, keycode: KeyCode) -> bool;
    pub fn key_just_pressed(&self, keycode: KeyCode) -> bool;
    pub fn key_just_released(&self, keycode: KeyCode) -> bool;
    pub fn pressed_keys(&self) -> Vec<KeyCode>;
    pub fn released_keys(&self) -> Vec<KeyCode>;
    
    // 鼠标
    pub fn mouse_pressed(&self, button: MouseButton) -> bool;
    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool;
    pub fn mouse_just_released(&self, button: MouseButton) -> bool;
    pub fn mouse_position(&self) -> Vec2;
    pub fn mouse_delta(&self) -> Vec2;
    pub fn wheel_delta(&self) -> Vec2;
    pub fn pressed_buttons(&self) -> Vec<MouseButton>;
    
    // 修饰键
    pub fn modifiers(&self) -> ModifiersState;
    
    // 触摸
    pub fn touch_count(&self) -> usize;
    pub fn touches(&self) -> impl Iterator<Item=&Touch>;
    pub fn touch(&self, id: u64) -> Option<&Touch>;
    
    // 文本
    pub fn text(&self) -> &str;
    
    // 事件
    pub fn events(&self) -> impl Iterator<Item=&InputEvent>;
    
    // 状态管理
    pub fn clear(&mut self);  // 重置瞬时状态（just_pressed, just_released）
    pub fn reset(&mut self);  // 完全重置所有状态
}
```

### InputModule

```rust
pub struct InputModule {
    input: Input,
}

impl InputModule {
    pub fn new() -> Self;
    pub fn process_event(&mut self, event: &Event);
    pub fn input(&self) -> &Input;
    pub fn input_mut(&mut self) -> &mut Input;
}

impl Module for InputModule {
    fn on_event(&mut self, event: &Event) {
        self.process_event(event);
    }
}
```

### 事件枚举

```rust
pub enum InputEvent {
    Keyboard(KeyCode, ElementState),
    MouseButton(MouseButton, ElementState),
    MouseMotion { x: f64, y: f64 },
    MouseWheel { delta: MouseScrollDelta },
    Touch(Touch),
    Text(char),
}

pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(LogicalPosition<f32>),
}

pub struct Touch {
    pub id: u64,
    pub position: Vec2,
    pub force: Option<f64>,
    pub phase: TouchPhase,
}

pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

bitflags! {
    pub struct ModifiersState: u32 {
        const SHIFT = 1 << 0;
        const CTRL = 1 << 1;
        const ALT = 1 << 2;
        const SUPER = 1 << 3;
    }
}
```

---

## 输入/输出

### 键盘查询
- **输入**: `KeyCode` 枚举
- **输出**: `bool`（是否按下/刚刚按下/刚刚释放）

### 鼠标查询
- **输入**: `MouseButton` 枚举
- **输出**: `bool`（是否按下/刚刚按下/刚刚释放）

### 位置/增量查询
- **输出**: `Vec2`（二维向量）

---

## 验收标准

| 验收项 | 标准 |
|-------|------|
| key_pressed | 按住 A 键时返回 true，多帧持续 |
| key_just_pressed | A 键刚按下时返回 true，仅一帧 |
| key_just_released | A 键刚释放时返回 true，仅一帧 |
| clear() vs reset() | clear() 只清除 just_pressed/just_released；reset() 清除所有状态 |
| mouse_position | 返回当前鼠标位置（屏幕坐标或窗口坐标） |
| mouse_delta | 返回与上一帧的鼠标位置差值 |
| wheel_delta | 返回滚轮滚动增量 |
| touch_count | 多点触摸时返回触摸点数量 |
| 修饰键组合 | Ctrl+Shift+A 同时按下时 modifiers() 返回对应标志位 |

---

## 依赖关系

- **内部依赖**: KeyCode、MouseButton、ModifiersState 等枚举
- **外部依赖**: `winit` crate（事件映射）
- **被依赖模块**: EventLoop、Engine

---

## 优先级定义

- **P0**: 核心功能，必须在 Sprint 02 完成
- **P1**: 重要功能，应在 Sprint 02 完成  
- **P2**: 增强功能，可延后到后续 Sprint
