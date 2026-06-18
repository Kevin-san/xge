# Query 与系统需求

## 模块概述

Query 是 ECS 架构中用于筛选和迭代实体的核心机制，System 是执行业务逻辑的可执行单元。本模块定义了 Query 的泛型结构、过滤器、迭代器以及 System 的定义、调度和并行执行机制。

---

## 需求编号：55-89, 202-277

### 1. Query 泛型查询

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 49 | `Query<'w, Q, F>`：泛型查询 | `struct Query<'w, Q, F = ()> { ... }` | 无 | 数据结构 | Q 是查询类型，F 是过滤器 | 39 | P0 |
| 50 | `Query::iter()` 只读迭代 | `Query::iter(&self) -> QueryItemIter<'_, Q>` | self | `Iter` | 返回查询结果的只读迭代器 | 49 | P0 |
| 51 | `Query::iter_mut()` 可变迭代 | `Query::iter_mut(&mut self) -> QueryItemIterMut<'_, Q>` | self | `IterMut` | 返回查询结果的可变迭代器 | 49 | P0 |
| 52 | `Query::get()` 单实体查询 | `Query::get(&self, entity: Entity) -> Option<QueryItem<'_, Q>>` | entity | `Option<Q>` | 未找到返回 None | 49 | P0 |
| 53 | `Query::get_mut()` 单实体可变查询 | `Query::get_mut(&mut self, entity: Entity) -> Option<QueryItemMut<'_, Q>>` | entity | `Option<Q>` | 未找到返回 None | 49 | P0 |
| 54 | `Query::single()` 期望单个 | `Query::single(&self) -> QueryItem<'_, Q>` | self | `Q` | 0 或 >1 个时 panic | 49 | P0 |
| 55 | `Query::single_mut()` 单个可变 | `Query::single_mut(&mut self) -> QueryItemMut<'_, Q>` | self | `Q` | 0 或 >1 个时 panic | 49 | P0 |
| 56 | `Query::is_empty()` 是否为空 | `Query::is_empty(&self) -> bool` | self | `bool` | 无结果返回 true | 49 | P0 |
| 57 | `Query::len()` 结果数量 | `Query::len(&self) -> usize` | self | `usize` | 返回匹配实体数量 | 49 | P0 |
| 58 | `Query::for_each()` 遍历执行 | `Query::for_each(&self, f: impl FnMut(QueryItem<'_, Q>))` | f | `()` | 对每个结果执行 f | 49 | P0 |
| 59 | `Query::par_for_each()` 并行遍历 | `Query::par_for_each(&self, batch_size: usize, f: impl FnMut(QueryItem<'_, Q>) + Send + Sync)` | batch_size, f | `()` | 分批并行执行 | 49 | P0 |
| 60 | `QueryState<Q, F>`：可缓存的查询状态 | `struct QueryState<Q, F = ()> { ... }` | 无 | 数据结构 | 缓存查询计划，加速连续调用 | 49 | P0 |
| 202 | `Query<(&A, &mut B)>` 合法 borrow | 同需求 49 | 同需求 49 | 同需求 49 | 编译通过，运行时正确 | 49 | P0 |
| 203 | `Query<(&mut A, &mut B)>` 合法 borrow | 同需求 49 | 同需求 49 | 同需求 49 | 编译通过，运行时正确 | 49 | P0 |
| 204 | `Query<(&A, &A)>` 非法 -> 编译期错误 | 同需求 49 | 同需求 49 | 同需求 49 | 编译失败，错误信息明确 | 49 | P0 |
| 205 | `Query<(&mut A, &mut A)>` 非法 -> 编译期错误 | 同需求 49 | 同需求 49 | 同需求 49 | 编译失败，错误信息明确 | 49 | P0 |
| 211 | `Query::iter` 返回正确个数 | 同需求 50 | 同需求 50 | 同需求 50 | 与实际匹配实体数一致 | 50 | P0 |
| 212 | `Query::iter_mut` 返回正确个数 | 同需求 51 | 同需求 51 | 同需求 51 | 与实际匹配实体数一致 | 51 | P0 |
| 213 | `Query::get` 未找到时返回 `None` | 同需求 52 | 同需求 52 | 同需求 52 | 不存在的实体返回 None | 52 | P0 |
| 214 | `Query::get_mut` 未找到时返回 `None` | 同需求 53 | 同需求 53 | 同需求 53 | 不存在的实体返回 None | 53 | P0 |
| 215 | `Query::single` 在 0 或 >1 时 panic | 同需求 54 | 同需求 54 | 同需求 54 | panic 信息包含 "single" | 54 | P0 |
| 216 | `QueryState` 缓存查询状态，连续调用快速 | 同需求 60 | 同需求 60 | 同需求 60 | 连续调用比首次快 >10% | 60 | P0 |
| 217 | `Query::par_for_each` 拆分任务到线程池 | 同需求 59 | 同需求 59 | 同需求 59 | 正确分配到可用线程 | 59 | P0 |
| 218 | `Query::par_for_each` 线程安全：无 data race | 同需求 59 | 同需求 59 | 同需求 59 | 无 data race，miri 测试通过 | 59 | P0 |

### 2. QueryFilter 过滤器

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 61 | `QueryFilter`：`With / Without / Added / Changed / Mutated` | `enum QueryFilter { With<T>, Without<T>, Added<T>, Changed<T>, Mutated<T> }` | 无 | enum 定义 | 支持组合过滤 | 49 | P0 |
| 62 | `Or<(A, B, C)>` / `And<(A, B, C)>` 组合过滤 | `struct Or<T>(pub T); struct And<T>(pub T)` | 无 | 结构体定义 | 支持逻辑组合 | 61 | P0 |
| 63 | `Changed<T>` 过滤组件变化 | `struct Changed<T>(pub T)` | 无 | 结构体 | 仅匹配本帧变化的组件 | 61, 257 | P0 |
| 64 | `Added<T>` 过滤组件被添加 | `struct Added<T>(pub T)` | 无 | 结构体 | 仅匹配本帧新增的组件 | 61, 258 | P0 |
| 65 | `RemovedComponents<T>` 迭代被移除的组件 | `struct RemovedComponents<'w, T> { ... }` | 无 | 结构体 | 返回之前有现在被移除的实体 | 259 | P0 |
| 114 | `WorldQuery` trait | `trait WorldQuery { type Item; }` | 无 | trait 定义 | `&T / &mut T / Option<&T> / Changed<T> / Added<T> / Without<T> / With<T>` | 39 | P0 |
| 115 | `WorldQuery::ReadOnly` 标记 | `trait WorldQuery { type ReadOnly: bool; }` | 无 | trait 定义 | 标识只读查询，可并行 | 114 | P0 |
| 116 | `QueryBorrowState<Q>`：查询借用状态跟踪 | `struct QueryBorrowState<Q> { ... }` | 无 | 数据结构 | 防止非法借用组合 | 49 | P0 |
| 117 | `BorrowError`：多次可变借用错误 | `struct BorrowError; impl Debug for BorrowError` | 无 | 错误类型 | 显示有用的借用冲突信息 | 116 | P0 |
| 206 | `Query` 支持 `With<T>` 过滤 | 同需求 61 | 同需求 61 | 同需求 61 | 仅返回包含 T 的实体 | 61 | P0 |
| 207 | `Query` 支持 `Without<T>` 过滤 | 同需求 61 | 同需求 61 | 同需求 61 | 仅返回不包含 T 的实体 | 61 | P0 |
| 208 | `Query` 支持 `Added<T>` 过滤 | 同需求 64 | 同需求 64 | 同需求 64 | 仅返回本帧新增 T 的实体 | 64 | P0 |
| 209 | `Query` 支持 `Changed<T>` 过滤 | 同需求 63 | 同需求 63 | 同需求 63 | 仅返回本帧变化 T 的实体 | 63 | P0 |
| 210 | `Query` 支持 `Or<(A, B)>` 组合过滤 | 同需求 62 | 同需求 62 | 同需求 62 | 匹配 A 或 B 的实体 | 62 | P0 |

---

## 需求编号：66-77, 219-231（System 系统）

### 3. SystemParam Trait

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 66 | `SystemParam` trait | `trait SystemParam { ... }` | 无 | trait 定义 | `Res / ResMut / Query / EventReader / EventWriter / Local / Commands` | 39 | P0 |
| 219 | `fn system(query: Query<(&mut Pos, &Vel)>)` 合法定义 | `fn system(query: Query<(&mut Pos, &Vel)>)` | Query 参数 | 系统函数 | 编译通过，Query 正常工作 | 66 | P0 |
| 220 | `fn system(res: Res<Time>)` 合法定义 | `fn system(res: Res<Time>)` | Res 参数 | 系统函数 | 编译通过，Res 正常工作 | 66 | P0 |
| 221 | `fn system(mut res: ResMut<Score>)` 合法定义 | `fn system(mut res: ResMut<Score>)` | ResMut 参数 | 系统函数 | 编译通过，ResMut 正常工作 | 66 | P0 |
| 222 | `fn system(ev: EventReader<E>)` 合法定义 | `fn system(ev: EventReader<E>)` | EventReader 参数 | 系统函数 | 编译通过，EventReader 正常工作 | 66 | P0 |
| 223 | `fn system(mut ev: EventWriter<E>)` 合法定义 | `fn system(mut ev: EventWriter<E>)` | EventWriter 参数 | 系统函数 | 编译通过，EventWriter 正常工作 | 66 | P0 |
| 224 | `fn system(mut cmds: Commands)` 合法定义 | `fn system(mut cmds: Commands)` | Commands 参数 | 系统函数 | 编译通过，Commands 正常工作 | 66 | P0 |
| 225 | `fn system(local: Local<u32>)` 合法定义 | `fn system(local: Local<u32>)` | Local 参数 | 系统函数 | 编译通过，Local 正常工作 | 66 | P0 |
| 97 | `ParamSet`：同一系统中同时存在多个冲突 Query | `struct ParamSet<P1, P2> { ... }` | 无 | 数据结构 | 解决同一系统内多个可变 Query 冲突 | 66 | P0 |

### 4. System Trait 与实现

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 67 | `System` trait | `trait System { fn run(&mut self, world: &mut World); }` | world | `()` | 定义系统执行接口 | 66 | P0 |
| 68 | `System::name()` 获取系统名称 | `System::name(&self) -> &str` | self | `&str` | 返回系统名称 | 67 | P0 |
| 69 | `System::is_exclusive()` 是否独占 | `System::is_exclusive(&self) -> bool` | self | `bool` | 独占系统返回 true | 67 | P0 |
| 70 | `IntoSystem` 转换 | `trait IntoSystem { type System; fn into_system(self) -> Self::System; }` | self | `Self::System` | 函数转换为系统 | 67 | P0 |
| 118 | `ExclusiveSystem`：直接获取 `&mut World` | `trait ExclusiveSystem { fn run(&mut self, world: &mut World); }` | world | `()` | 可直接访问 World | 67 | P0 |
| 148 | `NonSend`：不在线程间发送的资源标记 | `struct NonSend<T>(T)` | 无 | 包装类型 | 标记资源不可跨线程 | 90 | P1 |
| 149 | `NonSendMut` | `struct NonSendMut<T>(T)` | 无 | 包装类型 | 标记可变资源不可跨线程 | 148 | P1 |
| 150 | `SystemLabel / SystemSet / RunCriteriaLabel` 标记 | `trait SystemLabel { fn label(&self) -> &str; }` | 无 | trait 定义 | 用于系统组织和控制 | 67 | P1 |

### 5. SystemStage 与调度

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 71 | `SystemStage::single_threaded()` 单线程调度 | `SystemStage::single_threaded() -> SystemStage` | 无 | `SystemStage` | 系统按添加顺序执行 | 67 | P0 |
| 72 | `SystemStage::parallel()` 多线程调度 | `SystemStage::parallel() -> SystemStage` | 无 | `SystemStage` | 基于依赖关系图并行执行 | 67 | P0 |
| 73 | `Schedule::new() / add_stage / run(world)` | `struct Schedule { ... }` | 无 | 数据结构 | 按 stage 顺序执行系统 | 74 | P0 |
| 74 | `Stage` trait | `trait Stage { fn run(&mut self, world: &mut World); }` | world | `()` | 阶段执行接口 | 67 | P0 |
| 75 | `SystemSet`：一组系统集合 | `struct SystemSet { systems: Vec<Box<dyn System>>, ... }` | 无 | 数据结构 | 可整体启用/禁用 | 67 | P0 |
| 76 | `RunCriteria`：基于条件决定是否执行阶段 | `trait RunCriteria { fn should_run(&mut self) -> bool; }` | 无 | trait 定义 | 返回 true 才执行 | 67 | P1 |
| 77 | `Label`：阶段/系统标签 | `trait Label { fn label(&self) -> &str; }` | 无 | trait 定义 | 用于引用和排序 | 74 | P1 |
| 226 | `fn exclusive(world: &mut World)` 合法定义 | `fn exclusive(world: &mut World)` | World 参数 | 函数 | 独占系统编译通过 | 118 | P0 |
| 227 | `SystemStage::parallel()` 按资源访问图并行化 | 同需求 72 | 同需求 72 | 同需求 72 | 无资源冲突的系统并行执行 | 72 | P0 |
| 228 | `Schedule` 按 stage 顺序执行 | 同需求 73 | 同需求 73 | 同需求 73 | 按添加顺序执行各 stage | 73 | P0 |
| 229 | `Schedule::run(world)` 不崩溃 | `Schedule::run(&mut self, world: &mut World)` | world | `()` | 完整执行无 panic | 73 | P0 |
| 230 | `SystemSet` 可整体 disable | 同需求 75 | 同需求 75 | 同需求 75 | disabled 时跳过执行 | 75 | P1 |
| 231 | `RunCriteria::Paused` 可暂停阶段 | 同需求 76 | 同需求 76 | 同需求 76 | Paused 时阶段不执行 | 76 | P1 |

---

## 依赖关系图

```
Query (49)
    │
    ├── iter (50) / iter_mut (51)
    ├── get (52) / get_mut (53)
    ├── single (54) / single_mut (55)
    ├── is_empty (56) / len (57)
    ├── for_each (58)
    ├── par_for_each (59)
    └── QueryState (60)

QueryFilter (61)
    ├── With<T> (206)
    ├── Without<T> (207)
    ├── Added<T> (208)
    ├── Changed<T> (209)
    └── Or<(A, B)> (210)

SystemParam (66)
    ├── Res<T>
    ├── ResMut<T>
    ├── Query<Q, F>
    ├── EventReader<T>
    ├── EventWriter<T>
    ├── Commands
    ├── Local<T>
    └── ParamSet (97)

System (67)
    ├── run (67)
    ├── name (68)
    └── is_exclusive (69)

IntoSystem (70)
    │
    └── ExclusiveSystem (118)

SystemStage (71-72)
    ├── single_threaded (71)
    └── parallel (72)

Schedule (73)
    ├── add_stage
    └── run (229)

Stage (74)
    └── run

SystemSet (75)
    └── disable (230)

RunCriteria (76)
    └── Paused (231)
```

---

## 单元测试要求

| 需求ID | 描述 | 测试命令 |
|--------|------|----------|
| 279 | 单元测试 `Query` With/Without | `cargo test -p engine-ecs query_with_without` |
| 280 | 单元测试 `Changed/Added` | `cargo test -p engine-ecs query_changed_added` |
| 286 | 单元测试 `Query::single` | `cargo test -p engine-ecs query_single` |
| 287 | 单元测试 `Query::iter_mut` | `cargo test -p engine-ecs query_iter_mut_borrow` |
| 288 | 单元测试 `Parallel System` | `cargo test -p engine-ecs parallel_system` |
| 349 | `Query::single` 在有多个实体时 panic | `cargo test -p engine-ecs query_single_panic` |
| 350 | `Query::iter_mut` 不与其他借用冲突 | `cargo test -p engine-ecs query_iter_mut_no_conflict` |

---

## 优先级说明

- **P0（关键）**：必须完成的核心功能，直接影响 Sprint 验收
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代