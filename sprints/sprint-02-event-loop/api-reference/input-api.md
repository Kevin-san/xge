# Input API 清单

> 本文档是 `engine-window` crate 中输入相关公开 API 的完整清单。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 56-99, 191-229

---

## 公开 API 清单

### Input

```rust
/// Input 结构体保存所有输入设备的状态快照。
///
/// 状态分为"持续"状态（如 key_pressed）和"瞬时"状态（如 key_just_pressed）。
/// 每帧调用 `clear()` 重置瞬时状态，调用 `reset()` 完全重置所有状态。
///
/// # 状态说明
///
/// - `key_pressed`: 按住键时持续返回 true
/// - `key_just_pressed`: 键刚按下时返回一次 true
/// - `key_just_released`: 键刚释放时返回一次 true
pub struct Input {
    // 私有字段
}

impl Input {
    /// 创建新的 Input 实例。
    pub fn new() -> Self;
    
    // ========== 键盘状态 ==========
    
    /// 检查指定键是否处于按下状态。
    ///
    /// # 参数
    ///
    /// * `keycode` - 要检查的键码
    ///
    /// # 返回
    ///
    /// 按住返回 `true`，未按住返回 `false`
    ///
    /// # 示例
    ///
    /// ```rust
    /// if input.key_pressed(KeyCode::KeyW) {
    ///     // W 键正在被按住
    /// }
    /// ```
    pub fn key_pressed(&self, keycode: KeyCode) -> bool;
    
    /// 检查键是否刚刚被按下。
    ///
    /// 此方法在键刚按下的第一帧返回 `true`，之后立即返回 `false`。
    pub fn key_just_pressed(&self, keycode: KeyCode) -> bool;
    
    /// 检查键是否刚刚被释放。
    ///
    /// 此方法在键刚释放的第一帧返回 `true`，之后立即返回 `false`。
    pub fn key_just_released(&self, keycode: KeyCode) -> bool;
    
    /// 获取所有当前按下的键列表。
    pub fn pressed_keys(&self) -> Vec<KeyCode>;
    
    /// 获取所有刚刚释放的键列表。
    pub fn released_keys(&self) -> Vec<KeyCode>;
    
    // ========== 鼠标状态 ==========
    
    /// 检查鼠标按钮是否处于按下状态。
    pub fn mouse_pressed(&self, button: MouseButton) -> bool;
    
    /// 检查鼠标按钮是否刚刚被按下。
    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool;
    
    /// 检查鼠标按钮是否刚刚被释放。
    pub fn mouse_just_released(&self, button: MouseButton) -> bool;
    
    /// 获取鼠标在窗口中的位置。
    ///
    /// # 返回
    ///
    /// 返回 `Vec2`，x 和 y 分别表示鼠标的水平和垂直位置
    pub fn mouse_position(&self) -> Vec2;
    
    /// 获取鼠标移动增量（与上一帧的位置差）。
    ///
    /// # 返回
    ///
    /// 返回 `Vec2`，表示从上一帧到当前位置的偏移量
    pub fn mouse_delta(&self) -> Vec2;
    
    /// 获取鼠标滚轮增量。
    ///
    /// # 返回
    ///
    /// 返回 `Vec2`，x 是水平滚动，y 是垂直滚动
    pub fn wheel_delta(&self) -> Vec2;
    
    /// 获取所有当前按下的鼠标按钮列表。
    pub fn pressed_buttons(&self) -> Vec<MouseButton>;
    
    /// 获取所有刚刚释放的鼠标按钮列表。
    pub fn released_buttons(&self) -> Vec<MouseButton>;
    
    // ========== 修饰键 ==========
    
    /// 获取当前修饰键状态。
    ///
    /// # 返回
    ///
    /// 返回 `ModifiersState`，包含 Shift/Ctrl/Alt/Super 的按下状态
    ///
    /// # 示例
    ///
    /// ```rust
    /// let modifiers = input.modifiers();
    /// if modifiers.contains(ModifiersState::CTRL) {
    ///     // Ctrl 键正在被按住
    /// }
    /// ```
    pub fn modifiers(&self) -> ModifiersState;
    
    // ========== 触摸状态 ==========
    
    /// 获取当前触摸点数量。
    pub fn touch_count(&self) -> usize;
    
    /// 获取触摸点迭代器。
    pub fn touches(&self) -> impl Iterator<Item=&Touch>;
    
    /// 根据 ID 获取特定触摸点。
    ///
    /// # 参数
    ///
    /// * `id` - 触摸点的唯一标识符
    ///
    /// # 返回
    ///
    /// 找到返回 `Some(Touch)`，未找到返回 `None`
    pub fn touch(&self, id: u64) -> Option<&Touch>;
    
    // ========== 文本输入 ==========
    
    /// 获取累积的文本输入。
    ///
    /// 每次收到 `ReceivedCharacter` 事件时累加到此处。
    /// 调用 `clear()` 后清空。
    pub fn text(&self) -> &str;
    
    // ========== 事件 ==========
    
    /// 获取输入事件的迭代器。
    ///
    /// 迭代器按顺序返回自上一次 `clear()` 以来的所有输入事件。
    pub fn events(&self) -> impl Iterator<Item=&InputEvent>;
    
    // ========== 状态管理 ==========
    
    /// 清除瞬时状态。
    ///
    /// 此方法清除：
    /// - `key_just_pressed` 状态
    /// - `key_just_released` 状态
    /// - `mouse_just_pressed` 状态
    /// - `mouse_just_released` 状态
    /// - `mouse_delta`
    /// - `wheel_delta`
    /// - `text`
    ///
    /// 保留"持续"状态如 `key_pressed` 和 `mouse_pressed`。
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 在每帧结束时调用
    /// input.clear();
    /// ```
    pub fn clear(&mut self);
    
    /// 完全重置输入状态。
    ///
    /// 此方法清除所有状态，包括：
    /// - 所有"持续"状态（key_pressed, mouse_pressed 等）
    /// - 所有"瞬时"状态
    /// - 位置和增量
    /// - 触摸点
    /// - 文本输入
    ///
    /// 通常在窗口失焦或应用暂停时调用。
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 当窗口失去焦点时重置
    /// if !engine.is_focused() {
    ///     input.reset();
    /// }
    /// ```
    pub fn reset(&mut self);
}
```

### InputModule

```rust
/// InputModule 是 Engine 的模块，负责处理输入事件。
///
/// 它订阅窗口事件并更新内部 Input 状态。
pub struct InputModule {
    // 私有字段
}

impl InputModule {
    /// 创建新的 InputModule 实例。
    pub fn new() -> Self;
    
    /// 处理单个事件。
    ///
    /// 由 EventLoop 调用，传入窗口事件以更新 Input 状态。
    pub fn process_event(&mut self, event: &Event);
    
    /// 获取 Input 的不可变引用。
    pub fn input(&self) -> &Input;
    
    /// 获取 Input 的可变引用。
    pub fn input_mut(&mut self) -> &mut Input;
}

impl Module for InputModule {
    fn on_event(&mut self, event: &Event) {
        self.process_event(event);
    }
}
```

---

## 枚举和结构体

### KeyCode

```rust
/// 键码枚举，覆盖所有常用键位。
///
/// # 分类
///
/// - 字母键：A-Z (KeyA-KeyZ)
/// - 数字键：0-9 (Digit0-Digit9)
/// - 功能键：F1-F12
/// - 方向键：ArrowUp/Down/Left/Right
/// - 控制键：Escape, Tab, Space, Enter, Backspace, Delete, Insert, Home, End, PageUp, PageDown
/// - 修饰键：ShiftLeft/Right, ControlLeft/Right, AltLeft/Right, SuperLeft/Right
/// - 小键盘：Numpad0-9, NumpadAdd/Subtract/Multiply/Divide/Decimal/Enter
/// - 符号键：Plus, Minus, Equal, BracketLeft/Right, Semicolon, Apostrophe, Comma, Period, Slash, Backslash, Grave
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

### ScanCode

```rust
/// 原始键盘扫描码。
///
/// 扫描码是与键盘布局无关的硬件码。
/// 某些情况下需要使用 ScanCode 而非 KeyCode（例如游戏需要一致的按键映射）。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScanCode(pub u32);
```

### ElementState

```rust
/// 元素状态（按下或释放）。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementState {
    /// 元素被按下
    Pressed,
    /// 元素被释放
    Released,
}
```

### MouseButton

```rust
/// 鼠标按钮。
///
/// # 变体
///
/// - `Left`: 左键
/// - `Middle`: 中键（滚轮）
/// - `Right`: 右键
/// - `Other(u16)`: 其他按钮（侧键等）
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}
```

### MouseScrollDelta

```rust
/// 鼠标滚轮增量类型。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MouseScrollDelta {
    /// 行增量（典型鼠标滚轮）
    LineDelta(f32, f32),
    /// 像素增量（高精度滚轮或触控板）
    PixelDelta(LogicalPosition<f32>),
}
```

### TouchPhase

```rust
/// 触摸阶段。
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    /// 触摸开始
    Started,
    /// 触摸移动
    Moved,
    /// 触摸结束
    Ended,
    /// 触摸取消
    Cancelled,
}
```

### Touch

```rust
/// 触摸点信息。
pub struct Touch {
    /// 触摸点唯一 ID
    pub id: u64,
    /// 触摸位置
    pub position: Vec2,
    /// 触摸压力（如果有）
    pub force: Option<f64>,
    /// 触摸阶段
    pub phase: TouchPhase,
}
```

### ModifiersState

```rust
/// 修饰键状态（使用 bitflags）。
///
/// # 标志位
///
/// - `SHIFT`: Shift 键
/// - `CTRL`: Control 键
/// - `ALT`: Alt 键
/// - `SUPER`: Super/Windows/Command 键
bitflags! {
    pub struct ModifiersState: u32 {
        const SHIFT = 1 << 0;
        const CTRL = 1 << 1;
        const ALT = 1 << 2;
        const SUPER = 1 << 3;
    }
}
```

### InputEvent

```rust
/// 输入事件枚举。
///
/// 用于 `Input::events()` 迭代器返回。
pub enum InputEvent {
    /// 键盘输入
    Keyboard(KeyCode, ElementState),
    /// 鼠标按钮输入
    MouseButton(MouseButton, ElementState),
    /// 鼠标移动
    MouseMotion { x: f64, y: f64 },
    /// 鼠标滚轮
    MouseWheel { delta: MouseScrollDelta },
    /// 触摸输入
    Touch(Touch),
    /// 文本输入
    Text(char),
}
```

---

## 中文注释要求

- 所有公开 API 必须有中文 doc comment
- 解释参数含义和返回值
- 包含使用示例

---

## 英文注释要求

- 中文注释与英文注释并存
- 英文注释用于国际化和代码可读性
