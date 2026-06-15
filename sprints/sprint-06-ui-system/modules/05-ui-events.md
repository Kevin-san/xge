# UI 事件与交互需求

## 模块概述

UI 事件与交互模块负责处理用户与 UI 控件的交互操作。该模块基于 ECS 架构，通过事件系统派发和读取 UI 事件。核心功能包括：点击检测、焦点管理、键盘导航、事件派发与消费。

---

## 需求清单

### UI 事件枚举

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 26 | `UiEvent` 枚举：Click/Press/Release/HoverEnter/HoverLeave/DragStart/Drag/DragEnd/Focus/Blur/TextInput/ValueChanged | P0 |
| 262 | `UiEvent::Click(entity)` | P0 |
| 263 | `UiEvent::Press(entity)` | P0 |
| 264 | `UiEvent::Release(entity)` | P0 |
| 265 | `UiEvent::HoverEnter(entity)` | P0 |
| 266 | `UiEvent::HoverLeave(entity)` | P0 |
| 267 | `UiEvent::DragStart(entity)` | P0 |
| 268 | `UiEvent::Drag(entity, delta)` | P0 |
| 269 | `UiEvent::DragEnd(entity)` | P0 |
| 270 | `UiEvent::Focus(entity)` | P0 |
| 271 | `UiEvent::Blur(entity)` | P0 |
| 272 | `UiEvent::TextInput(entity, text)` | P0 |
| 273 | `UiEvent::ValueChanged(entity, value)` | P0 |
| 274 | `UiEvent::ValueChangedSlider(entity, value)` | P0 |
| 275 | `UiEvent::Toggled(entity, bool)` | P0 |
| 276 | `UiEvent::Scroll(entity, offset)` | P0 |

### 事件读写器

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 27 | `UiEventReader<E>` | P0 |
| 28 | `UiEventWriter<E>` | P0 |
| 284 | `ui_event_reader_system` 读取并消费事件 | P0 |
| 285 | `UiEventQueue` 双缓冲 | P1 |

### 点击检测

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 68 | `UiEvent::Click(entity)` 由点击事件派发 | P0 |
| 69 | `UiEvent::HoverEnter(entity)` 由鼠标进入派发 | P0 |
| 70 | `UiEvent::HoverLeave(entity)` 由鼠标离开派发 | P0 |
| 71 | `UiEvent::DragStart(entity)` 由按下 + 移动派发 | P0 |
| 72 | `UiEvent::ValueChanged(entity, new_value)` | P0 |
| 73 | `UiEvent::Focus(entity)` / `Blur(entity)` | P0 |
| 74 | `UiEvent::TextInput(entity, text)` | P0 |
| 75 | UI 点击检测：AABB 命中测试 + 子节点排序 | P0 |
| 76 | UI 点击检测：透明区域穿透检测 | P1 |
| 277 | `hit_test_system` 将鼠标事件转换为 UI 事件 | P0 |
| 278 | `hit_test_system` 支持嵌套：返回最上层命中节点 | P0 |
| 279 | `hit_test_system` 支持透明穿透 | P1 |
| 323 | `hit_test_system` 将鼠标事件转换为 UI 事件 | P0 |
| 324 | `hit_test_system` 支持嵌套：返回最上层命中节点 | P0 |
| 325 | `hit_test_system` 支持透明穿透 | P1 |

### 焦点管理

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 77 | UI 输入焦点：Tab 切换 | P0 |
| 78 | UI 输入焦点：鼠标点击切换 | P0 |
| 79 | UI 键盘导航：方向键 | P1 |
| 280 | `focus_system` 管理 Tab 顺序 | P0 |
| 281 | `focus_system` 管理鼠标点击切换焦点 | P0 |
| 282 | `focus_system` 方向键切换焦点 | P1 |

### 事件派发

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 283 | `ui_event_dispatch_system` 派发事件 | P0 |

### 动画系统

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 101 | 动画系统基础：按钮 hover 缩放过渡 | P1 |
| 102 | 动画：按钮 press 下沉动画 | P1 |
| 103 | 动画：渐显/渐隐 | P1 |
| 104 | 动画：滑入/滑出 | P1 |

---

## API 签名

### UiEvent

```rust
pub enum UiEvent {
    Click(Entity),
    Press(Entity),
    Release(Entity),
    HoverEnter(Entity),
    HoverLeave(Entity),
    DragStart(Entity),
    Drag(Entity, Vec2),
    DragEnd(Entity),
    Focus(Entity),
    Blur(Entity),
    TextInput(Entity, String),
    ValueChanged(Entity, String),
    ValueChangedSlider(Entity, f32),
    Toggled(Entity, bool),
    Scroll(Entity, Vec2),
}
```

### UiEventReader

```rust
pub struct UiEventReader<E: Component> {
    // 内部数据
}

impl<E: Component> UiEventReader<E> {
    pub fn read(&mut self, world: &World) -> Vec<&E>;
}
```

### UiEventWriter

```rust
pub struct UiEventWriter<E: Component> {
    // 内部数据
}

impl<E: Component> UiEventWriter<E> {
    pub fn write(&mut self, entity: Entity, event: E);
}
```

### UiEventQueue

```rust
pub struct UiEventQueue {
    // 双缓冲
}

impl UiEventQueue {
    pub fn enqueue(&mut self, event: impl Component);
    pub fn drain(&mut self) -> Vec<Box<dyn Component>>;
}
```

### hit_test_system

```rust
pub fn hit_test_system(world: &World, mouse_pos: Vec2) -> Option<Entity>;
```

### focus_system

```rust
pub fn focus_system(world: &mut World, event: &InputEvent);
```

### ui_event_dispatch_system

```rust
pub fn ui_event_dispatch_system(world: &mut World);
```

### ui_event_reader_system

```rust
pub fn ui_event_reader_system(world: &mut World);
```

---

## 输入/输出

| 系统 | 输入 | 输出 |
|------|------|------|
| hit_test_system | 鼠标位置 Vec2 | 命中的 Entity |
| focus_system | InputEvent（Tab/Click/方向键） | 焦点切换 |
| ui_event_dispatch_system | 鼠标/键盘事件 | 派发的 UiEvent |
| ui_event_reader_system | UiEventQueue | 消费的事件 |

---

## 事件流程

### 点击事件流程

```
1. 鼠标按下
   ↓
2. hit_test_system 检测命中节点
   ↓
3. 派发 Press 事件
   ↓
4. 鼠标释放
   ↓
5. hit_test_system 再次检测（同一节点？）
   ↓
6. 派发 Click 事件
   ↓
7. 派发 Release 事件
```

### 拖拽事件流程

```
1. 鼠标按下 + 移动
   ↓
2. 派发 DragStart 事件
   ↓
3. 每帧移动
   ↓
4. 派发 Drag 事件（带 delta）
   ↓
5. 鼠标释放
   ↓
6. 派发 DragEnd 事件
```

### 焦点事件流程

```
Tab 按下 / 点击节点:
   ↓
focus_system 处理
   ↓
旧焦点节点派发 Blur
   ↓
新焦点节点派发 Focus
```

---

## 验收标准

- [ ] 点击检测正确返回最上层命中的非透明节点
- [ ] 透明穿透检测正确跳过透明区域
- [ ] Tab 键正确切换焦点顺序
- [ ] 点击正确切换焦点到被点击节点
- [ ] 方向键正确切换相邻节点焦点
- [ ] 所有 UiEvent 变体可正确派发
- [ ] UiEventReader 可正确读取事件
- [ ] UiEventQueue 双缓冲正常工作
- [ ] 按钮 hover 动画正常播放
- [ ] 按钮 press 动画正常播放
- [ ] 渐显/渐隐动画正常播放
- [ ] 滑入/滑出动画正常播放

---

## 依赖关系

- 依赖 `UiNode`、`UiRect` 核心组件
- 依赖 `UiFocus` 焦点组件
- 依赖 `engine-input` crate（输入事件）
- 被渲染系统依赖（获取交互状态）

---

## 优先级说明

- **P0**：核心交互缺失会导致 UI 无法响应用户操作
- **P1**：重要增强功能，影响用户体验
- **P2**：辅助功能，可后续补充
