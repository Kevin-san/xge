# 键码定义需求

## 模块概述

KeyCode 枚举覆盖所有常用键位，包括字母键、数字键、功能键、方向键、控制键、修饰键、小键盘和符号键。ScanCode 是原始扫描码类型，与 KeyCode 分离以支持不同键盘布局。ElementState 表示按键的按下/释放状态。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 74-78, 96-99, 213-225

---

## 需求详情

### KeyCode 字母键（A-Z）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 213 | KeyCode::KeyA - KeyCode::KeyZ | 26 个字母键 | 输出: KeyCode 枚举 | P0 |

```rust
pub enum KeyCode {
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM,
    KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
}
```

### KeyCode 数字键（0-9）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 214 | KeyCode::Digit0 - KeyCode::Digit9 | 10 个数字键 | 输出: KeyCode 枚举 | P0 |

```rust
pub enum KeyCode {
    Digit0, Digit1, Digit2, Digit3, Digit4,
    Digit5, Digit6, Digit7, Digit8, Digit9,
}
```

### KeyCode 功能键（F1-F12）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 215 | KeyCode::F1 - KeyCode::F12 | 12 个功能键 | 输出: KeyCode 枚举 | P0 |

```rust
pub enum KeyCode {
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}
```

### KeyCode 方向键

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 216 | KeyCode::ArrowUp/Down/Left/Right | 4 个方向键 | 输出: KeyCode 枚举 | P0 |

```rust
pub enum KeyCode {
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
}
```

### KeyCode 控制键

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 217 | Escape / Tab / Space / Enter / Backspace / Delete / Insert / Home / End / PageUp / PageDown | 11 个控制键 | 输出: KeyCode 枚举 | P0 |

```rust
pub enum KeyCode {
    Escape,
    Tab,
    Space,
    Enter,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
}
```

### KeyCode 修饰键

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 218 | ShiftLeft / ShiftRight / ControlLeft / ControlRight / AltLeft / AltRight / SuperLeft / SuperRight | 8 个修饰键 | 输出: KeyCode 枚举 | P0 |

```rust
pub enum KeyCode {
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,
}
```

### KeyCode 小键盘

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 219 | Numpad0-9 + Add + Subtract + Multiply + Divide + Decimal + Enter | 15 个小键盘键 | 输出: KeyCode 枚举 | P1 |

```rust
pub enum KeyCode {
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,
    NumpadEnter,
}
```

### KeyCode 符号键

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 220 | Plus / Minus / Equal / BracketLeft / BracketRight / Semicolon / Apostrophe / Comma / Period / Slash / Backslash / Grave | 12 个符号键 | 输出: KeyCode 枚举 | P1 |

```rust
pub enum KeyCode {
    Plus,           // +
    Minus,          // -
    Equal,          // =
    BracketLeft,    // [
    BracketRight,   // ]
    Semicolon,      // ;
    Apostrophe,     // '
    Comma,          // ,
    Period,         // .
    Slash,          // /
    Backslash,      // \
    Grave,          // `
}
```

### ScanCode 原始扫描码

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 75 | ScanCode 原始扫描码类型 | `ScanCode(u32)` | 输入: u32 → 输出: ScanCode | P0 |

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScanCode(pub u32);
```

### ElementState 按键状态

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 76 | ElementState::Pressed / Released | 按下/释放状态枚举 | 输出: ElementState | P0 |

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementState {
    Pressed,
    Released,
}
```

### MouseButton 鼠标按钮

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 72 | MouseButton::Left / Middle / Right / Other(u16) | 鼠标按钮枚举 | 输出: MouseButton | P0 |
| 221 | MouseButton::Left / Middle / Right + Other(u16) | 同上 | 输出: MouseButton | P0 |

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}
```

### MouseScrollDelta 滚轮增量

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 76 | MouseWheel 事件 | `LineDelta(x, y) / PixelDelta(x, y)` | 输入: f32, f32 → 输出: MouseScrollDelta | P0 |
| 222 | MouseScrollDelta | `LineDelta(x, y) / PixelDelta(x, y)` | 输出: 枚举 | P0 |

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(LogicalPosition<f32>),
}
```

### TouchPhase 触摸阶段

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 78 | TouchPhase::Started / Moved / Ended / Cancelled | 触摸阶段枚举 | 输出: TouchPhase | P1 |
| 223 | TouchPhase::Started / Moved / Ended / Cancelled | 同上 | 输出: TouchPhase | P1 |

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}
```

### CursorIcon 光标图标

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 100 | CursorIcon 枚举 | Default / Crosshair / Hand / Move / Text / Wait / Help / Progress / NResize / EResize / ... | 输出: CursorIcon | P1 |

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
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
    NEResize,
    NWResize,
    SEResize,
    SWResize,
    ColResize,
    RowResize,
}
```

### WindowMode 窗口模式

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 100 | WindowMode 枚举 | Windowed / Fullscreen / Borderless | 输出: WindowMode | P1 |

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
    Borderless,
}
```

### ModifiersState 修饰键状态

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 优先级 |
|---------|---------|---------|----------|--------|
| 69 | ModifiersState | shift/ctrl/alt/meta/win/cmd 组合 | 输入: bool × 5 → 输出: ModifiersState | P0 |
| 225 | ModifiersState bitflags | SHIFT / CTRL / ALT / SUPER 组合操作 | 输出: bitflags | P0 |

```rust
bitflags! {
    pub struct ModifiersState: u32 {
        const SHIFT = 1 << 0;
        const CTRL = 1 << 1;
        const ALT = 1 << 2;
        const SUPER = 1 << 3;
        const LOGO = Self::SUPER.bits; // 同一标志
    }
}
```

---

## API 签名汇总

### KeyCode 完整枚举

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // 字母键 A-Z
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM,
    KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    
    // 数字键 0-9
    Digit0, Digit1, Digit2, Digit3, Digit4,
    Digit5, Digit6, Digit7, Digit8, Digit9,
    
    // 功能键 F1-F12
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // 方向键
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    
    // 控制键
    Escape, Tab, Space, Enter, Backspace, Delete, Insert,
    Home, End, PageUp, PageDown,
    
    // 修饰键
    ShiftLeft, ShiftRight,
    ControlLeft, ControlRight,
    AltLeft, AltRight,
    SuperLeft, SuperRight,
    
    // 小键盘
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply,
    NumpadDivide, NumpadDecimal, NumpadEnter,
    
    // 符号键
    Plus, Minus, Equal,
    BracketLeft, BracketRight,
    Semicolon, Apostrophe,
    Comma, Period, Slash, Backslash, Grave,
}
```

### 辅助类型

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScanCode(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementState {
    Pressed,
    Released,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(LogicalPosition<f32>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

### KeyCode 查询
- **KeyCode** → 用于 `Input::key_pressed(keycode)` 等方法

### ScanCode
- **u32** → 原始键盘扫描码，与布局无关

### ElementState
- **Pressed** → 按下状态
- **Released** → 释放状态

---

## 验收标准

| 验收项 | 标准 |
|-------|------|
| A-Z 键 | `KeyCode::KeyA` 到 `KeyCode::KeyZ` 共 26 个 |
| 0-9 键 | `KeyCode::Digit0` 到 `KeyCode::Digit9` 共 10 个 |
| F1-F12 | `KeyCode::F1` 到 `KeyCode::F12` 共 12 个 |
| 方向键 | ArrowUp/Down/Left/Right 共 4 个 |
| 修饰键 | Shift/Control/Alt/Super 各有左右，共 8 个 |
| 小键盘 | Numpad0-9 + 运算键共 15 个 |
| ScanCode | `ScanCode(u32)` 可存储任意扫描码 |
| ElementState | Pressed/Released 可比较 |

---

## 依赖关系

- **内部依赖**: Input 模块
- **外部依赖**: 无
- **被依赖模块**: Input、InputModule

---

## 优先级定义

- **P0**: 核心功能，必须在 Sprint 02 完成
- **P1**: 重要功能，应在 Sprint 02 完成  
- **P2**: 增强功能，可延后到后续 Sprint
