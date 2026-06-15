# 事件与 Bundle 需求

## 模块概述

Event 是 ECS 中的事件通信机制，支持双缓冲模式实现帧间事件传递。Bundle 提供批量组件操作能力，简化实体创建过程。本模块定义事件读写、资源单件以及 Bundle 的完整实现。

---

## 需求编号：98-105, 119-127, 281-295

### 1. Event 事件系统

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 92 | `Event<T>`：事件队列 | `struct Event<T> { ... }` | 无 | 数据结构 | 包装事件数据 | 无 | P0 |
| 93 | `#[derive(Event)]` 宏 | `#[derive(Event)]` | 结构体定义 | 派生实现 | 自动实现 Event trait | 92 | P0 |
| 122 | `EventReader<T>::iter()` 读取事件 | `EventReader<T>::iter(&self) -> EventIterator<T>` | self | `Iter<T>` | 返回新事件的迭代器 | 92 | P0 |
| 123 | `EventWriter<T>::send()` 发送事件 | `EventWriter<T>::send(&mut self, event: T)` | event | `()` | 事件进入队列 | 92 | P0 |
| 124 | `EventWriter<T>::send_batch()` 批量发送 | `EventWriter<T>::send_batch(&mut self, events: impl IntoIterator<Item = T>)` | events | `()` | 批量事件进入队列 | 92 | P0 |
| 125 | `Events<T>::update()` 旧事件清理 | `Events<T>::update(&mut self)` | self | `()` | 双缓冲切换，清理旧事件 | 92 | P0 |
| 239 | `Events<T>` 双缓冲：`update()` 丢弃上一帧之前的事件 | 同需求 125 | 同需求 125 | 同需求 125 | 上一帧事件被清除 | 125 | P0 |
| 240 | `EventReader` `iter` 仅返回新事件 | 同需求 122 | 同需求 122 | 同需求 122 | 不返回已读事件 | 122 | P0 |
| 241 | `EventReader` 支持多次读取同一条事件（多 reader 独立） | 同需求 122 | 同需求 122 | 同需求 122 | 每个 reader 独立位置 | 122 | P0 |
| 242 | `EventWriter` `send` 发送立即入队 | 同需求 123 | 同需求 123 | 同需求 123 | 立即可读 | 123 | P0 |
| 243 | `EventWriter` `send_batch` 批量 | 同需求 124 | 同需求 124 | 同需求 124 | 所有事件立即可读 | 124 | P0 |

### 2. Bundle 批量组件

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 45 | `Bundle` trait + `#[derive(Bundle)]` 宏 | `trait Bundle { ... }` | 无 | trait 定义 | 支持批量组件操作 | 39 | P0 |
| 46 | `Bundle::bundle_components()` 遍历组件 | `Bundle::bundle_components(&self, func: impl FnMut(ComponentId, &dyn Component))` | func | `()` | 对每个组件调用 func | 45 | P0 |
| 47 | `Bundle::from_components()` 从组件构造 | `Bundle::from_components(func: impl FnOnce(ComponentId) -> Box<dyn Component>) -> Self` | func | `Self` | 从组件构造 Bundle | 45 | P0 |
| 48 | `Bundle::bundle_id()` 获取 Bundle ID | `Bundle::bundle_id(&self) -> BundleId` | self | `BundleId` | 返回唯一标识 | 45 | P0 |
| 169 | `#[derive(Bundle)]` 宏 | 同需求 45 | 同需求 45 | 同需求 45 | 派生宏正确生成 Bundle 实现 | 45 | P0 |
| 159 | `examples/ecs_bundle` — Bundle 简化 spawn | `cargo run --example ecs_bundle` | 无 | 示例运行 | 正常运行 | 45 | P0 |
| 281 | 单元测试 `Bundle` 往返 | `cargo test -p engine-ecs bundle_roundtrip` | 无 | 测试结果 | Bundle -> components -> Bundle 正确 | 47 | P0 |

### 3. Local 系统本地状态

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 96 | `Local<T>`：系统本地状态 | `struct Local<T> { value: T }` | 无 | 数据结构 | 每个系统实例独有 | 66 | P0 |
| 244 | `Local<T>` 默认值 T: Default | 同需求 96 | 同需求 96 | 同需求 96 | 可用 `Local::default()` | 96 | P0 |
| 245 | `Local<T>::default` 支持 `#[local]` 自定义初始值 | `#[local(default = ...)]` | 属性 | 派生实现 | 支持自定义默认值 | 244 | P1 |

---

## 需求编号：119-127（额外类型）

### 4. 其他类型

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 98 | `Archetype`：原型 | 同 02-entity-component.md | - | - | 见 02-entity-component.md | 39 | P0 |
| 99 | `Archetype::id()` | 同 02-entity-component.md | - | - | - | 98 | P0 |
| 100 | `Archetype::entities()` | 同 02-entity-component.md | - | - | - | 98 | P0 |
| 101 | `Archetype::component_ids()` | 同 02-entity-component.md | - | - | - | 98 | P0 |
| 102 | `Archetype::get::<C>()` | 同 02-entity-component.md | - | - | - | 98 | P0 |
| 103 | `ArchetypeGraph` | 同 02-entity-component.md | - | - | - | 103 | P0 |
| 104 | `ChangeTrackers` | 同 02-entity-component.md | - | - | - | 104 | P0 |
| 105 | `Tick` | 同 02-entity-component.md | - | - | - | 105 | P0 |
| 106 | `Ticks<T>` | 同 02-entity-component.md | - | - | - | 106 | P0 |
| 107 | `Ref<T>` | 同 02-entity-component.md | - | - | - | 107 | P0 |
| 108 | `Reflect` | 同 02-entity-component.md | - | - | - | 108 | P2 |
| 109 | `Name` 组件 | `struct Name(String)` | 无 | 组件 | 便于调试的命名组件 | 39 | P1 |

---

## 依赖关系图

```
Event<T> (92)
    │
    ├── #[derive(Event)] (93)
    │
    ├── EventReader<T> (122)
    │       └── iter (122)
    │
    ├── EventWriter<T> (123)
    │       ├── send (123)
    │       └── send_batch (124)
    │
    └── Events<T> (125)
            └── update (125)

Bundle (45)
    ├── bundle_components (46)
    ├── from_components (47)
    └── bundle_id (48)

Local<T> (96)
    └── #[local] (245)
```

---

## 单元测试要求

| 需求ID | 描述 | 测试命令 |
|--------|------|----------|
| 281 | 单元测试 `Bundle` 往返 | `cargo test -p engine-ecs bundle_roundtrip` |
| 282 | 单元测试 `Events` 双缓冲清理 | `cargo test -p engine-ecs events_double_buffer` |
| 283 | 单元测试 `Commands` | `cargo test -p engine-ecs commands_apply` |
| 294 | 单元测试 `World::spawn / despawn` | `cargo test -p engine-ecs world_spawn_despawn` |

---

## 优先级说明

- **P0（关键）**：必须完成的核心功能，直接影响 Sprint 验收
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代