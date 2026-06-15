# Tween 与信号系统（Tween / Signal）模块需求

## 模块概述

Tween（补间动画）系统提供数值插值动画能力，支持多种缓动曲线（30+）。Signal（信号）系统提供节点间事件通信机制，支持信号连接、断开、派发。两个系统共同构成游戏逻辑的动画与交互基础设施。

---

## 需求清单

### 1. Tween 补间动画

| 编号 | 需求 | 描述 |
|------|------|------|
| 142 | `Tween`：补间动画系统（线性、ease_in, ease_out, bounce, elastic） | 核心补间结构 |
| 143 | `TweenValue::Float / Vec2 / Vec3 / Color / Angle` | 支持的值类型 |
| 144 | `Tween::new(start, end, duration, ease)` | 构造方法 |
| 170 | `Tween::update(&mut self, dt)` | 更新动画 |
| 171 | `Tween::is_finished(&self) -> bool` | 是否完成 |
| 319 | `Tween::with_repeat(times, mode)` | 重复次数配置 |
| 320 | `Tween::with_yoyo(bool)` | 往复动画配置 |
| 321 | `Tween::with_delay(delay)` | 延迟启动配置 |
| 322 | `Tween::on_complete(callback)` | 完成回调 |
| 323 | `Tween::value(&self) -> TweenValue` | 当前插值结果 |
| 324 | `Tween::progress(&self) -> f32` | 进度百分比 |
| 325 | `Tween::update(&mut self, dt)` | 更新动画 |
| 326 | `Tween::is_finished(&self) -> bool` | 是否完成 |
| 327 | `Tween::reset(&mut self)` | 重置动画 |
| 328 | `Ease::Linear / InQuad / OutQuad / InOutQuad / ...` | 30+ 缓动曲线 |

### 2. TweenManager 补间管理器

| 编号 | 需求 | 描述 |
|------|------|------|
| 147 | `TweenManager`：管理多个 tweens | 补间管理器 |
| 329 | `TweenManager::new()` | 构造方法 |
| 330 | `TweenManager::add(&mut self, tween) -> TweenHandle` | 添加补间 |
| 331 | `TweenManager::remove(&mut self, handle)` | 移除补间 |
| 332 | `TweenManager::update(&mut self, dt)` | 更新所有补间 |
| 333 | `TweenManager::clear(&mut self)` | 清空所有补间 |

### 3. Timer 定时器

| 编号 | 需求 | 描述 |
|------|------|------|
| 148 | `Timer`：通用定时器，interval、oneshot | 定时器组件 |
| 149 | `Timer::new(duration, mode)` | 构造方法 |
| 150 | `Timer::tick(&mut self, dt) -> bool` | 更新定时器，返回是否触发 |
| 151 | `Timer::finished(&self) -> bool` | 是否已完成 |
| 152 | `Timer::reset(&mut self)` | 重置定时器 |
| 153 | `Timer::remaining(&self) -> f32` | 剩余时间 |
| 154 | `Timer::elapsed(&self) -> f32` | 已过时间 |
| 155 | `TimerMode::Once / Repeat` | 定时器模式 |

### 4. NodeSignal / Signal 事件系统

| 编号 | 需求 | 描述 |
|------|------|------|
| 123 | `NodeSignal` / `emit("clicked")` — 事件派发 | 节点信号基础 |
| 124 | `Node::connect(signal, handler)` — 注册信号处理 | 信号连接 |
| 125 | `Node::emit(signal, args...)` — 派发信号 | 信号派发 |
| 335 | `Signal::new(name)` | 创建信号 |
| 336 | `Signal::connect(&mut self, handler)` | 连接处理函数 |
| 337 | `Signal::disconnect(&mut self, handler_id)` | 断开连接 |
| 338 | `Signal::emit(&self, args...)` | 派发信号 |
| 339 | `Node::get_signal(&self, name) -> &Signal` | 获取信号引用 |
| 340 | `Node::signal_mut(&mut self, name) -> &mut Signal` | 获取可变信号引用 |

---

## API 签名

### TweenValue & Ease

```rust
pub enum TweenValue {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Color(Color),
    Angle(f32),  // 弧度
}

pub enum Ease {
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InSine,
    OutSine,
    InOutSine,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InBack,
    OutBack,
    InOutBack,
    InElastic,
    OutElastic,
    InOutElastic,
    InBounce,
    OutBounce,
    InOutBounce,
}
```

### Tween

```rust
pub enum TweenRepeatMode {
    Times(u32),
    Forever,
}

pub struct Tween {
    start: TweenValue,
    end: TweenValue,
    duration: f32,
    elapsed: f32,
    ease: Ease,
    repeat_mode: Option<(u32, bool)>,  // (times, yoyo)
    delay: f32,
    on_complete: Option<Box<dyn Fn()>>,
}

impl Tween {
    pub fn new(start: TweenValue, end: TweenValue, duration: f32, ease: Ease) -> Self;
    
    pub fn with_repeat(mut self, times: u32, mode: TweenRepeatMode) -> Self;
    pub fn with_yoyo(mut self, yoyo: bool) -> Self;
    pub fn with_delay(mut self, delay: f32) -> Self;
    pub fn on_complete(mut self, callback: impl Fn() + 'static) -> Self;
    
    pub fn value(&self) -> TweenValue;
    pub fn progress(&self) -> f32;
    pub fn update(&mut self, dt: f32);
    pub fn is_finished(&self) -> bool;
    pub fn reset(&mut self);
}
```

### TweenManager

```rust
pub struct TweenHandle(u64);

pub struct TweenManager {
    tweens: HashMap<TweenHandle, Tween>,
    next_handle: u64,
}

impl TweenManager {
    pub fn new() -> Self;
    pub fn add(&mut self, tween: Tween) -> TweenHandle;
    pub fn remove(&mut self, handle: TweenHandle);
    pub fn update(&mut self, dt: f32);
    pub fn clear(&mut self);
}
```

### Timer

```rust
pub enum TimerMode {
    Once,
    Repeat,
}

pub struct Timer {
    duration: f32,
    elapsed: f32,
    mode: TimerMode,
    finished: bool,
}

impl Timer {
    pub fn new(duration: f32, mode: TimerMode) -> Self;
    pub fn tick(&mut self, dt: f32) -> bool;
    pub fn finished(&self) -> bool;
    pub fn reset(&mut self);
    pub fn remaining(&self) -> f32;
    pub fn elapsed(&self) -> f32;
}
```

### Signal

```rust
pub struct HandlerId(u64);

pub struct Signal {
    name: String,
    handlers: Vec<(HandlerId, Box<dyn Fn(&[&dyn Any])>)>,
}

impl Signal {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn connect(&mut self, handler: impl Fn(&[&dyn Any]) + 'static) -> HandlerId;
    pub fn disconnect(&mut self, id: HandlerId);
    pub fn emit(&self, args: &[&dyn Any]);
}
```

---

## 输入/输出

### Tween
- **输入**：起始值、结束值、持续时间、缓动曲线
- **输出**：插值后的当前值

### Timer
- **输入**：时间步长 dt
- **输出**：是否触发（返回 bool）

### Signal
- **输入**：信号参数
- **输出**：所有已连接 handler 被调用

---

## 验收标准

1. ✅ `Tween::new(Float(0), Float(100), 1.0, Ease::Linear)` 在 1 秒内从 0 到 100
2. ✅ `Tween::with_repeat(3, Forever)` 无限重复
3. ✅ `Tween::with_yoyo(true)` 往复动画
4. ✅ `Ease::InOutCubic` 在 t=0/0.5/1 处输出正确值
5. ✅ `Tween::on_complete` 在动画完成时被调用
6. ✅ `TweenManager::update` 更新所有活跃补间
7. ✅ `TweenManager::remove` 移除指定补间
8. ✅ `Timer::new(1.0, Once).tick(0.5)` 返回 false
9. ✅ `Timer::new(1.0, Once).tick(1.0)` 返回 true
10. ✅ `Timer::Repeat` 模式循环触发
11. ✅ `Signal::connect` 返回 handler_id
12. ✅ `Signal::disconnect` 移除指定 handler
13. ✅ `Signal::emit` 调用所有已连接 handler
14. ✅ 单元测试：Tween ease_in_out 时间曲线
15. ✅ 单元测试：Ease::InOutCubic 在 t=0 / 0.5 / 1 处输出
16. ✅ 单元测试：Timer Once 模式的 finished 行为
17. ✅ 单元测试：Signal emit 被所有 handler 接收
18. ✅ 示例 `tween` 多种缓动演示正常
19. ✅ 示例 `timer` 定时器演示正常
20. ✅ 示例 `signals` 点击按钮派发信号正常

---

## 依赖关系

- 依赖 `math` crate（Vec2、Vec3、Color）
- 被 `SceneTree` / `Node` 集成
- 示例 `tween`、`timer`、`signals` 依赖本模块

---

## 优先级

| 优先级 | 含义 | 需求编号 |
|--------|------|----------|
| P0 | 核心功能 | 142-155, 170-171, 319-340 |
| P1 | 重要功能 | 328-333 |
| P2 | 增强功能 | 320-322 |
