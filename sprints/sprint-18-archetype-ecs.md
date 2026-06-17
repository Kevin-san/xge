# Sprint 18 · ECS 重构 (Archetype + 并行调度 + SystemParam)

> 文档编号: `sprint-18-archetype-ecs.md / v1.0
> 周期: 4 周 (20 个工作日)
> 上游依赖: Sprint 17 (Math SIMD)
> 下游交付: Sprint 19 (Render) / 20 (Physics) / 21 (Animation) 全部使用 ECS

---

## 1. 目标与范围

**目标：** 将当前 `engine-ecs` 从 `HashMap<TypeId, Box<dyn Any>>` 的 **Type-Erased 异构存储** 升级为 **Archetype-Based ECS**（Bevy ECS 4 架构），实现类型安全查询、并行调度、零成本抽象、SystemParam 自动派生。

**范围：**
- ✅ Archetype 内存布局（同一组件组合的实体连续存储）
- ✅ 真正的 Query 系统（支持 `With/Without/Changed/Added` 过滤）
- ✅ System 并行调度（`Schedule` 拓扑排序 + 多线程 worker）
- ✅ SystemParam 类型安全（Query/Res/ResMut/EventReader/EventWriter/Commands）
- ✅ Bundle 批量插入 + ChangeTick
- ⛔ 不含：异步系统（async system）、GPU ECS、Distributed ECS

**核心参考：** Bevy ECS v0.14、EnTT、Flecs、Rust 编译期反射最小集。

---

## 2. 上游需求对接

| 来源 | 关联章节 | 承接 |
|------|---------|------|
| [NEXT_PHASE_REQUIREMENTS.md § 4.1](../NEXT_PHASE_REQUIREMENTS.md) | ECS 内存布局 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 4.2](../NEXT_PHASE_REQUIREMENTS.md) | Query 系统 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 4.3](../NEXT_PHASE_REQUIREMENTS.md) | System 调度 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 4.4](../NEXT_PHASE_REQUIREMENTS.md) | SystemParam | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 5 M2](../NEXT_PHASE_REQUIREMENTS.md) | Milestone M2 | ✅ 本 sprint |

---

## 3. 子模块拆分

### 3.1 [01-archetype-storage.md](modules/01-archetype-storage.md) — Archetype 内存布局

**核心交付：**
- `engine-ecs/src/storage/archetype.rs`
  - `Archetype { id: ArchetypeId, columns: HashMap<TypeId, Column> }`
  - 同一组件组合的实体共享 Archetype
  - `Table { components: Vec<Column> }` 实际内存，SoA 布局
- `engine-ecs/src/storage/column.rs`
  - `Column { data: Box<dyn AnyColumn> }` 长度对齐组件存储
  - `AnyColumn` trait：`get(index)`, `set(index, value)`, `push(value)`, `remove(swap_remove)`, `len()`
- `engine-ecs/src/world/archetypes.rs`
  - `Archetypes { by_components: HashMap<ArchetypeId, ArchetypeId> }`
  - 组件组合 → Archetype 映射

**Bug 修复对应：** `engine-ecs/src/world.rs#L124-L236` HashMap 存储 → Archetype

**验收：**
- 10000 实体 5 组件 spawn 时间 < 50 ms
- Archetype 切换（添加/移除组件） < 5 µs
- 内存布局：cache line 对齐，SoA 列内连续
- `cargo test` 100% 路径覆盖 archetype 移动

---

### 3.2 [02-query-system.md](modules/02-query-system.md) — 类型安全 Query

**核心交付：**
- `engine-ecs/src/query/mod.rs`
  - `Query<T: WorldData>` 主 trait
  - `QueryState<T>` 内部状态（matched_archetypes, fetch 状态）
  - `QueryFilter` trait：`With<C>`, `Without<C>`, `Changed<C>`, `Added<C>`, `Or<(...)>`
- `engine-ecs/src/query/fetch.rs`
  - `Fetch<'w, T>` 实际数据获取
  - 类型级别 `&T`, `&mut T`, `Option<&T>`, `Option<&mut T>`, `Entity`, `EntityRef`
- `engine-ecs/src/query/iter.rs`
  - `QueryIter<'w, Q, F>` 实现 `Iterator<Item = Q::Item>`

**Bug 修复对应：** `engine-ecs/src/query.rs#L1-L22` 空实现 → 完整 Query

**验收：**
- 单 archetype 10000 实体 Query 迭代 < 100 µs
- 编译期类型检查：访问不存在组件编译错误
- `Changed` / `Added` 时间戳准确（ChangeTick 系统）
- 测试覆盖：With/Without/Or/Changed/Added 5 种过滤 100% 路径

---

### 3.3 [03-system-schedule.md](modules/03-system-schedule.md) — 并行调度

**核心交付：**
- `engine-ecs/src/system/param.rs`
  - `SystemParam` trait（自动派生）
  - 实现：`Query`, `Res`, `ResMut`, `EventReader`, `EventWriter`, `Commands`, `Local`
  - 派生宏 `#[derive(SystemParam)]` 复合参数
- `engine-ecs/src/system/function_system.rs`
  - `FunctionSystem<Marker>` 将 `fn` 包装成 System
  - 静态访问分析（`SystemAccess` 集合，BorrowCheck）
- `engine-ecs/src/schedule/mod.rs`
  - `Schedule { stages: Vec<Stage>, label: ScheduleLabel }`
  - 阶段（Stage）：逻辑分组，如 `PreUpdate`, `Update`, `PostUpdate`
  - 拓扑排序：自动按资源访问冲突分组并行层
  - `Schedule::run(&mut world)` 自动多线程执行
- `engine-ecs/src/schedule/executor.rs`
  - `Executor`：`SingleThreaded` / `MultiThreaded` / `Simple`
  - 跨步并行：同层 system 并行（rayon-like work stealing）
- `engine-ecs/src/world/access.rs`
  - `WorldAccess` 跟踪每个 system 的读/写访问

**验收：**
- 100 个 system 自动拓扑排序 < 10 ms
- 8 核 CPU 上 64 个并行 system 提速 ≥ 4x
- 资源冲突检测：编译期 + 运行期 borrow check
- 测试覆盖：竞态检测（rayon + arc_mutex 模拟）

---

### 3.4 [04-bundle-change-detection.md](modules/04-bundle-change-detection.md) — Bundle / ChangeTick

**核心交付：**
- `engine-ecs/src/bundle.rs` — 增强
  - `Bundle` trait（已经存在，扩展 `ComponentBundle` 标记）
  - `BundleInserter` 高效批量插入
  - `spawn_batch` 利用 Archetype 直接填充
- `engine-ecs/src/change.rs`
  - `ChangeTick { tick: u32 }` 全局 tick 计数器
  - `ComponentTicks { added: u32, changed: u32 }` 组件级别 tick
  - `world.change_tick()` 推进 tick
- `engine-ecs/src/removal_detection.rs`
  - `RemovedComponents<C>` 跟踪被移除组件的实体

**验收：**
- 1000 实体批量 spawn < 5 ms
- ChangeTick 推进开销 < 100 ns / 系统
- RemovedComponents 测试：add→remove→re-add 全部 tick 正确

---

### 3.5 [05-events-commands.md](modules/05-events-commands.md) — 事件与命令缓冲

**核心交付：**
- `engine-ecs/src/event.rs` — 重构
  - `Events<E>` 双缓冲队列（避免 System 期间 borrow 冲突）
  - `EventReader<E>` / `EventWriter<E>` SystemParam
  - `Event::update` 在 stage 之间刷新
- `engine-ecs/src/command.rs`
  - `Commands` 延迟命令缓冲
  - `EntityCommands` 实体级别命令
  - 命令应用：stage 结束前批量应用

**验收：**
- 10000 事件 / 帧 < 1 ms
- `EventReader` 重复读幂等
- `Commands::spawn` 立即可用（应用延迟 1 stage）

---

## 4. 验收清单（acceptance/test-plan.md）

- [ ] 10000 实体 5 组件 spawn < 50 ms
- [ ] Archetype 切换 < 5 µs
- [ ] 10000 实体 Query 迭代 < 100 µs
- [ ] 64 system 多线程并行加速 ≥ 4x (8 核)
- [ ] 编译期类型检查 Query 错误 → 编译失败
- [ ] ChangeTick 准确：ComponentTicks 测试通过
- [ ] Events 双缓冲测试通过
- [ ] RemovedComponents 跨 stage 正确
- [ ] `cargo test -p engine-ecs` 全通过
- [ ] `cargo bench -p engine-ecs` 基准记录存档
- [ ] `cargo doc` 文档生成，链接到子模块

---

## 5. API 稳定承诺

**对外主要 API：**

```rust
pub use world::World;
pub use entity::Entity;
pub use component::Component;
pub use query::{Query, With, Without, Changed, Added, Or};
pub use system::{System, IntoSystem, SystemParam};
pub use schedule::{Schedule, Stage};
pub use resource::{Resource, Res, ResMut};
pub use event::{Event, EventReader, EventWriter, Events};
pub use bundle::Bundle;
pub use commands::Commands;
```

**兼容策略：** 旧的 `engine_ecs::query::Query` 空实现将迁移到 `engine_ecs::query::QueryState`，提供 shim 适配层。

---

## 6. 与 Sprint 17 依赖

| 依赖点 | 来自 | 用途 |
|--------|------|------|
| `f32x4` SIMD | sprint-17 | 组件 AOS → SOA 转换 |
| `DualQuat` 蒙皮 | sprint-17 | `Transform3D` 组件内部使用 |
| `Frustum` SIMD | sprint-17 | 可视化调试（archetype 级 culling） |

---

## 7. 风险与缓解

| 风险 | 等级 | 缓解 |
|------|------|------|
| Archetype 切换成本（添加/移除组件） | 中 | 引入 "Tick Buffer" 延迟切换 |
| SystemParam 派生宏复杂 | 中 | 限制派生范围，先支持基本类型 |
| 多线程数据竞争 | 高 | 严格 borrow check + arc mutex 兜底 |
| 编译时间膨胀 | 中 | feature flag 拆分 archetype-only / full |
| 与旧 API 不兼容 | 高 | shim 适配层 + 2 sprint deprecation |
