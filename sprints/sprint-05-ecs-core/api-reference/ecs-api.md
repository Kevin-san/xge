# ECS API 清单

## 模块概述

本文档列出 `engine-ecs` crate 的所有公开 API，包括宏定义、结构体、trait 和函数。公开 API 数量应 <= 100，doc comment 覆盖率 100%。

---

## 需求编号：166-170

### 1. 宏定义（Macros）

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `#[derive(Component)]` | `#[derive(Component)]` | 40, 168 | 派生 Component trait，默认使用 SparseSet 存储 |
| `#[derive(Bundle)]` | `#[derive(Bundle)]` | 45, 169 | 派生 Bundle trait |
| `#[derive(Resource)]` | `#[derive(Resource)]` | 90, 170 | 派生 Resource trait |
| `#[derive(Event)]` | `#[derive(Event)]` | 92, 171 | 派生 Event trait |
| `#[derive(SystemLabel)]` | `#[derive(SystemLabel)]` | 121, 172 | 派生 SystemLabel trait |
| `#[derive(StageLabel)]` | `#[derive(StageLabel)]` | 77, 173 | 派生 StageLabel trait |
| `#[derive(SystemSet)]` | `#[derive(SystemSet)]` | 75, 174 | 派生 SystemSet trait |
| `#[derive(RunCriteriaLabel)]` | `#[derive(RunCriteriaLabel)]` | 76, 175 | 派生 RunCriteriaLabel trait |
| `#[component(storage = "...")]` | `#[component(storage = "DenseVec")]` | 185 | 指定组件存储类型 |
| `#[local(default = ...)]` | `#[local(default = ...)]` | 245 | 指定 Local 默认值 |

---

### 2. 核心类型（Core Types）

#### World 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `World` | `struct World` | 2 | ECS 世界容器 |
| `World::new()` | `fn new() -> World` | 2 | 创建空世界 |
| `World::spawn()` | `fn spawn(&mut self) -> Entity` | 3, 172 | 生成实体 |
| `World::spawn_bundle()` | `fn spawn_bundle(&mut self, bundle: impl Bundle) -> Entity` | 4, 173 | 生成实体并插入 bundle |
| `World::spawn_batch()` | `fn spawn_batch(&mut self, bundles: impl Iterator<Item = impl Bundle>)` | 5, 174 | 批量生成 |
| `World::despawn()` | `fn despawn(&mut self, entity: Entity) -> bool` | 6, 175 | 销毁实体 |
| `World::clear_entities()` | `fn clear_entities(&mut self)` | 7, 178 | 清空所有实体 |
| `World::insert()` | `fn insert(&mut self, entity: Entity, component: C) -> Option<C>` | 8 | 插入组件 |
| `World::insert_bundle()` | `fn insert_bundle(&mut self, entity: Entity, bundle: impl Bundle) -> bool` | 9 | 插入 bundle |
| `World::remove::<C>()` | `fn remove::<C>(&mut self, entity: Entity) -> Option<C>` | 10 | 移除组件 |
| `World::remove_bundle::<B>()` | `fn remove_bundle::<B>(&mut self, entity: Entity) -> bool` | 11 | 移除 bundle |
| `World::entity()` | `fn entity(&self, entity: Entity) -> EntityRef` | 12 | 获取 EntityRef |
| `World::entity_mut()` | `fn entity_mut(&mut self, entity: Entity) -> EntityMut` | 13 | 获取 EntityMut |
| `World::contains()` | `fn contains(&self, entity: Entity) -> bool` | 14, 177 | 检查实体存在 |
| `World::get_component::<C>()` | `fn get_component::<C>(&self, entity: Entity) -> Option<&C>` | 15 | 获取组件只读引用 |
| `World::get_component_mut::<C>()` | `fn get_component_mut::<C>(&mut self, entity: Entity) -> Option<&mut C>` | 16 | 获取组件可变引用 |
| `World::entities()` | `fn entities(&self) -> &Entities` | 17 | 获取 Entities 引用 |
| `World::components()` | `fn components(&self) -> &Components` | 18 | 获取 Components 引用 |
| `World::resource::<R>()` | `fn resource::<R>(&self) -> &R` | 19, 237 | 获取资源只读引用 |
| `World::resource_mut::<R>()` | `fn resource_mut::<R>(&mut self) -> &mut R` | 20 | 获取资源可变引用 |
| `World::get_resource::<R>()` | `fn get_resource::<R>(&self) -> Option<&R>` | 238 | 获取资源 Option 引用 |
| `World::insert_resource()` | `fn insert_resource(&mut self, resource: R)` | 21, 50 | 插入资源 |
| `World::remove_resource::<R>()` | `fn remove_resource::<R>(&mut self) -> Option<R>` | 22 | 移除资源 |
| `World::contains_resource::<R>()` | `fn contains_resource::<R>(&self) -> bool` | 23 | 检查资源存在 |
| `World::send_event::<E>()` | `fn send_event::<E>(&mut self, event: E)` | 24 | 发送事件 |
| `World::events::<E>()` | `fn events::<E>(&self) -> EventReader<E>` | 25 | 获取事件读取器 |
| `World::run_system()` | `fn run_system(&mut self, system_fn: impl FnOnce(&mut World))` | 26 | 运行系统 |
| `World::run_system_catched()` | `fn run_system_catched(&mut self, system_fn: impl FnOnce(&mut World)) -> Result<()>` | 27 | 捕获错误运行 |
| `World::schedule()` | `fn schedule(&mut self, name: &str) -> &mut Schedule` | 28 | 获取调度器 |
| `World::add_system_to_stage()` | `fn add_system_to_stage(&mut self, stage: impl StageLabel, system: impl IntoSystem)` | 29 | 添加系统到阶段 |
| `World::add_system_set()` | `fn add_system_set(&mut self, system_set: impl SystemSet)` | 30 | 添加系统集 |
| `World::add_stage_after()` | `fn add_stage_after(&mut self, existing: impl StageLabel, name: &str, stage: impl Stage)` | 32 | 在阶段后添加 |
| `World::add_stage_before()` | `fn add_stage_before(&mut self, existing: impl StageLabel, name: &str, stage: impl Stage)` | 33 | 在阶段前添加 |
| `World::dump_stats()` | `fn dump_stats(&self)` | 166 | 打印 archetype 信息 |
| `World::validate()` | `fn validate(&self) -> Result<()>` | 167 | 基本一致性校验 |
| `World::clear_trackers()` | `fn clear_trackers(&mut self)` | 260 | 手动重置 trackers |

#### Entity 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Entity` | `struct Entity { id: u32, generation: u32 }` | 34 | 实体标识符 |
| `Entity::id()` | `fn id(&self) -> u32` | 35, 179 | 获取实体索引 |
| `Entity::generation()` | `fn generation(&self) -> u32` | 36, 180 | 获取实体代际 |
| `Entity::null()` | `fn null() -> Entity` | 37 | 创建空实体 |
| `Entity::is_null()` | `fn is_null(&self) -> bool` | 38 | 检查空实体 |

#### Component 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Component` | `trait Component { type Storage = SparseSet<C>; }` | 39, 225 | 组件 trait |
| `ComponentStorage` | `trait ComponentStorage<C: Component> { ... }` | 41 | 存储抽象 trait |
| `SparseSet<T>` | `struct SparseSet<T> { sparse, dense, ... }` | 42 | 稀疏集存储 |
| `DenseVec<T>` | `struct DenseVec<T>` | 43 | 密集向量存储 |
| `HashMapStorage<T>` | `struct HashMapStorage<T>` | 44 | 哈希映射存储 |

#### Bundle 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Bundle` | `trait Bundle { ... }` | 45 | Bundle trait |
| `Bundle::bundle_components()` | `fn bundle_components(&self, func: impl FnMut(ComponentId, &dyn Component))` | 46 | 遍历组件 |
| `Bundle::from_components()` | `fn from_components(func: impl FnOnce(ComponentId) -> Box<dyn Component>) -> Self` | 47 | 从组件构造 |
| `Bundle::bundle_id()` | `fn bundle_id(&self) -> BundleId` | 48 | 获取 Bundle ID |

#### Query 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Query<'w, Q, F>` | `struct Query<'w, Q, F = ()> { ... }` | 49 | 泛型查询 |
| `Query::iter()` | `fn iter(&self) -> QueryItemIter<'_, Q>` | 50, 211 | 只读迭代 |
| `Query::iter_mut()` | `fn iter_mut(&mut self) -> QueryItemIterMut<'_, Q>` | 51, 212 | 可变迭代 |
| `Query::get()` | `fn get(&self, entity: Entity) -> Option<QueryItem<'_, Q>>` | 52, 213 | 单实体查询 |
| `Query::get_mut()` | `fn get_mut(&mut self, entity: Entity) -> Option<QueryItemMut<'_, Q>>` | 53, 214 | 单实体可变查询 |
| `Query::single()` | `fn single(&self) -> QueryItem<'_, Q>` | 54, 215 | 期望单个 |
| `Query::single_mut()` | `fn single_mut(&mut self) -> QueryItemMut<'_, Q>` | 55 | 单个可变 |
| `Query::is_empty()` | `fn is_empty(&self) -> bool` | 56 | 是否为空 |
| `Query::len()` | `fn len(&self) -> usize` | 57 | 结果数量 |
| `Query::for_each()` | `fn for_each(&self, f: impl FnMut(QueryItem<'_, Q>))` | 58 | 遍历执行 |
| `Query::par_for_each()` | `fn par_for_each(&self, batch_size: usize, f: impl FnMut(...) + Send + Sync)` | 59, 217-218 | 并行遍历 |
| `QueryState<Q, F>` | `struct QueryState<Q, F = ()>` | 60, 216 | 可缓存的查询状态 |

#### QueryFilter 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `With<T>` | `struct With<T>(pub T)` | 61, 206 | 包含过滤 |
| `Without<T>` | `struct Without<T>(pub T)` | 61, 207 | 不包含过滤 |
| `Added<T>` | `struct Added<T>(pub T)` | 64, 208 | 新增过滤 |
| `Changed<T>` | `struct Changed<T>(pub T)` | 63, 209 | 变化过滤 |
| `Mutated<T>` | `struct Mutated<T>(pub T)` | 61 | 变异过滤 |
| `Or<T>` | `struct Or<T>(pub T)` | 62, 210 | 或组合过滤 |
| `And<T>` | `struct And<T>(pub T)` | 62 | 与组合过滤 |
| `RemovedComponents<'w, T>` | `struct RemovedComponents<'w, T>` | 65, 259 | 已移除组件 |

#### System 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `SystemParam` | `trait SystemParam { ... }` | 66 | 系统参数 trait |
| `System` | `trait System { fn run(&mut self, world: &mut World); }` | 67 | 系统 trait |
| `System::name()` | `fn name(&self) -> &str` | 68 | 获取系统名称 |
| `System::is_exclusive()` | `fn is_exclusive(&self) -> bool` | 69 | 是否独占 |
| `IntoSystem` | `trait IntoSystem { type System; fn into_system(self) -> Self::System; }` | 70 | 转换为系统 |
| `ExclusiveSystem` | `trait ExclusiveSystem { fn run(&mut self, world: &mut World); }` | 118 | 独占系统 trait |
| `SystemStage` | `enum SystemStage { ... }` | 71-72 | 系统阶段 |
| `SystemStage::single_threaded()` | `fn single_threaded() -> SystemStage` | 71 | 单线程调度 |
| `SystemStage::parallel()` | `fn parallel() -> SystemStage` | 72, 227 | 多线程调度 |
| `Schedule` | `struct Schedule { ... }` | 73 | 调度器 |
| `Schedule::run()` | `fn run(&mut self, world: &mut World)` | 229 | 运行调度器 |
| `Stage` | `trait Stage { fn run(&mut self, world: &mut World); }` | 74 | 阶段 trait |
| `SystemSet` | `struct SystemSet { ... }` | 75, 230 | 系统集合 |
| `RunCriteria` | `trait RunCriteria { fn should_run(&mut self) -> bool; }` | 76, 231 | 运行条件 |
| `Label` | `trait Label { fn label(&self) -> &str; }` | 77 | 标签 trait |
| `SystemLabel` | `trait SystemLabel: Label` | 121 | 系统标签 |
| `StageLabel` | `trait StageLabel: Label` | 77 | 阶段标签 |
| `RunCriteriaLabel` | `trait RunCriteriaLabel: Label` | 121 | 运行条件标签 |

#### Commands 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Commands` | `struct Commands { ... }` | 78 | 延迟命令队列 |
| `Commands::spawn()` | `fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands` | 79, 232 | 生成实体 |
| `Commands::spawn_batch()` | `fn spawn_batch(&mut self, bundles: impl IntoIterator<Item = impl Bundle>)` | 80 | 批量生成 |
| `Commands::insert()` | `fn insert(&mut self, entity: Entity, bundle: impl Bundle)` | 81, 233 | 插入组件 |
| `Commands::remove::<C>()` | `fn remove::<C>(&mut self, entity: Entity)` | 82, 234 | 移除组件 |
| `Commands::despawn()` | `fn despawn(&mut self, entity: Entity)` | 83, 235 | 销毁实体 |
| `Commands::insert_resource()` | `fn insert_resource(&mut self, resource: impl Resource)` | 84, 236 | 插入资源 |
| `Commands::remove_resource::<R>()` | `fn remove_resource::<R>(&mut self)` | 85, 236 | 移除资源 |
| `Commands::add()` | `fn add(&mut self, command: impl Command)` | 86 | 添加自定义命令 |
| `Commands::apply()` | `fn apply(&mut self, world: &mut World)` | 87, 243 | 应用到世界 |
| `EntityCommands` | `struct EntityCommands<'a>` | 88 | 实体命令构建器 |
| `EntityCommands::insert()` | `fn insert(&mut self, bundle: impl Bundle) -> &mut Self` | 89 | 插入组件 |
| `EntityCommands::remove::<C>()` | `fn remove::<C>(&mut self) -> &mut Self` | 90 | 移除组件 |
| `EntityCommands::despawn()` | `fn despawn(&mut self)` | 91 | 销毁实体 |
| `EntityCommands::id()` | `fn id(&self) -> Entity` | 94 | 获取实体 ID |
| `Command` | `trait Command { fn apply(self, world: &mut World); }` | 126 | 命令 trait |

#### Resource 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Resource<T>` | `struct Resource<T>(T)` | 90 | 资源包装 |
| `Res<T>` | `struct Res<T>(T)` | 66, 220 | 只读资源参数 |
| `ResMut<T>` | `struct ResMut<T>(T)` | 66, 221 | 可变资源参数 |
| `Local<T>` | `struct Local<T> { value: T }` | 96, 240-242 | 系统本地状态 |

#### Event 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Event<T>` | `struct Event<T> { ... }` | 92 | 事件 trait 包装 |
| `Events<T>` | `struct Events<T> { ... }` | 125 | 双缓冲事件队列 |
| `Events::update()` | `fn update(&mut self)` | 125, 239 | 更新事件缓冲 |
| `EventReader<T>` | `struct EventReader<T>` | 122, 240-241 | 事件读取器 |
| `EventReader::iter()` | `fn iter(&self) -> EventIterator<T>` | 122 | 读取事件迭代器 |
| `EventWriter<T>` | `struct EventWriter<T>` | 123 | 事件写入器 |
| `EventWriter::send()` | `fn send(&mut self, event: T)` | 123, 242 | 发送事件 |
| `EventWriter::send_batch()` | `fn send_batch(&mut self, events: impl IntoIterator<Item = T>)` | 124, 243 | 批量发送 |

#### Archetype 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Archetype` | `struct Archetype { ... }` | 98, 195-197 | 原型 |
| `Archetype::id()` | `fn id(&self) -> ArchetypeId` | 99 | 获取原型 ID |
| `Archetype::entities()` | `fn entities(&self) -> &[Entity]` | 100 | 获取实体列表 |
| `Archetype::component_ids()` | `fn component_ids(&self) -> &[ComponentId]` | 101 | 获取组件 ID 列表 |
| `Archetype::get::<C>()` | `fn get::<C>(&self) -> Option<&[C]>` | 102 | 获取组件数组 |
| `ArchetypeGraph` | `struct ArchetypeGraph { ... }` | 103, 201 | 原型迁移图 |

#### Change Tracking 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `ChangeTrackers` | `struct ChangeTrackers { ... }` | 104 | 变更追踪器 |
| `Tick` | `struct Tick { value: u32 }` | 105, 255 | 帧计数 |
| `Tick::tick()` | `fn tick(&mut self)` | 255 | 递增 tick |
| `Ticks<T>` | `struct Ticks<T> { added: Tick, changed: Tick, last_changed: Tick }` | 106 | 组件时间戳 |
| `Ref<'a, T>` | `struct Ref<'a, T> { value: &'a T, ticks: Ticks<T> }` | 107, 256 | 组件引用 |
| `Ref::is_added()` | `fn is_added(&self) -> bool` | 256 | 是否新增 |
| `Ref::is_changed()` | `fn is_changed(&self) -> bool` | 256 | 是否变化 |

#### Hierarchy 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Parent` | `struct Parent(pub Entity)` | 111, 246 | 父实体组件 |
| `Children` | `struct Children(pub Vec<Entity>)` | 112, 247 | 子实体列表组件 |
| `BuildChildren` | `trait BuildChildren { ... }` | 113 | 构建子实体 trait |
| `push_child()` | `fn push_child(parent: Entity, child: Entity, world: &mut World)` | 248 | 添加子实体 |
| `remove_child()` | `fn remove_child(parent: Entity, child: Entity, world: &mut World)` | 249 | 移除子实体 |
| `despawn_recursive()` | `fn despawn_recursive(entity: Entity, world: &mut World)` | 250 | 递归销毁 |

#### Transform 相关

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Transform2D` | `struct Transform2D { translation: Vec2, rotation: f32, scale: Vec2 }` | 251 | 2D 变换组件 |
| `GlobalTransform2D` | `struct GlobalTransform2D(Mat3)` | 252 | 世界变换组件 |
| `transform_propagate_system` | `fn transform_propagate_system(world: &mut World)` | 253 | 变换传播系统 |

#### 其他类型

| API 名称 | 签名 | 需求ID | 说明 |
|----------|------|--------|------|
| `Name` | `struct Name(String)` | 109 | 名称组件 |
| `NonSend<T>` | `struct NonSend<T>(T)` | 119, 148 | 非 Send 标记 |
| `NonSendMut<T>` | `struct NonSendMut<T>(T)` | 120, 149 | 非 Send 可变标记 |
| `ParamSet<P1, P2>` | `struct ParamSet<P1, P2>` | 97 | 参数集 |
| `WorldQuery` | `trait WorldQuery { ... }` | 114-115 | 世界查询 trait |
| `QueryBorrowState<Q>` | `struct QueryBorrowState<Q>` | 116 | 查询借用状态 |
| `BorrowError` | `struct BorrowError` | 117 | 借用错误 |
| `Reflect` | `trait Reflect { ... }` | 108 | 反射 trait（后续实现） |

---

## API 数量统计

| 类别 | 数量 |
|------|------|
| 宏定义 | 10 |
| World 相关 | 40 |
| Entity 相关 | 4 |
| Component 相关 | 5 |
| Bundle 相关 | 4 |
| Query 相关 | 14 |
| QueryFilter 相关 | 9 |
| System 相关 | 20 |
| Commands 相关 | 16 |
| Resource 相关 | 4 |
| Event 相关 | 9 |
| Archetype 相关 | 5 |
| Change Tracking 相关 | 7 |
| Hierarchy 相关 | 6 |
| Transform 相关 | 3 |
| 其他类型 | 9 |
| **总计** | **≤ 100** |

---

## Doc Comment 要求

所有公开 API 必须包含 doc comment，说明：
- 功能描述
- 参数说明
- 返回值说明
- 示例（可选）

验收标准：doc comment 覆盖率 100%

---

## 优先级说明

- **P0（关键）**：必须完成的核心 API
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代