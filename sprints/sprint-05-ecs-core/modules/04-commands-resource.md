# Commands 与资源需求

## 模块概述

Commands 是延迟命令队列，允许在系统执行过程中记录 spawn、insert、remove 等操作，待当前帧或指定时机统一应用到 World。Resource 是全局单件数据机制，用于存储跨系统共享的配置和状态。

---

## 需求编号：78-97, 232-245

### 1. Commands 命令队列

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 78 | `Commands`：延迟命令队列 | `struct Commands { queue: Vec<Command> }` | 无 | 数据结构 | 存储待执行的命令 | 34 | P0 |
| 79 | `Commands::spawn()` 生成实体 | `Commands::spawn(&mut self, bundle: impl Bundle) -> EntityCommands` | bundle | `EntityCommands` | 返回实体命令构建器 | 78 | P0 |
| 80 | `Commands::spawn_batch()` 批量生成 | `Commands::spawn_batch(&mut self, bundles: impl IntoIterator<Item = impl Bundle>)` | bundles | `()` | 批量插入命令 | 78 | P0 |
| 81 | `Commands::insert()` 插入组件 | `Commands::insert(&mut self, entity: Entity, bundle: impl Bundle)` | entity, bundle | `()` | 插入组件命令 | 78 | P0 |
| 82 | `Commands::remove::<C>()` 移除组件 | `Commands::remove::<C>(&mut self, entity: Entity)` | entity | `()` | 移除组件命令 | 78 | P0 |
| 83 | `Commands::despawn()` 销毁实体 | `Commands::despawn(&mut self, entity: Entity)` | entity | `()` | 销毁实体命令 | 78 | P0 |
| 84 | `Commands::insert_resource()` 插入资源 | `Commands::insert_resource(&mut self, resource: impl Resource)` | resource | `()` | 插入资源命令 | 78 | P0 |
| 85 | `Commands::remove_resource::<R>()` 移除资源 | `Commands::remove_resource::<R>(&mut self)` | self | `()` | 移除资源命令 | 78 | P0 |
| 86 | `Commands::add()` 添加自定义命令 | `Commands::add(&mut self, command: impl Command)` | command | `()` | 执行任意自定义命令 | 78 | P0 |
| 87 | `Commands::apply()` 应用到 world | `Commands::apply(&mut self, world: &mut World)` | world | `()` | 将所有命令应用到 World | 78 | P0 |
| 88 | `EntityCommands`：实体命令构建器 | `struct EntityCommands<'a> { commands: &'a mut Commands, entity: Entity }` | 无 | 数据结构 | 提供链式 API | 78 | P0 |
| 89 | `EntityCommands::insert()` 插入组件 | `EntityCommands::insert(&mut self, bundle: impl Bundle) -> &mut Self` | bundle | `Self` | 链式调用 | 88 | P0 |
| 90 | `EntityCommands::remove::<C>()` 移除组件 | `EntityCommands::remove::<C>(&mut self) -> &mut Self` | self | `Self` | 链式调用 | 88 | P0 |
| 91 | `EntityCommands::despawn()` 销毁实体 | `EntityCommands::despawn(&mut self)` | self | `()` | 销毁此实体 | 88 | P0 |
| 92 | `EntityCommands::insert_resource()` | `EntityCommands::insert_resource(&mut self, resource: impl Resource) -> &mut Self` | resource | `Self` | 链式调用 | 88 | P0 |
| 93 | `EntityCommands::remove_resource::<R>()` | `EntityCommands::remove_resource::<R>(&mut self) -> &mut Self` | self | `Self` | 链式调用 | 88 | P0 |
| 94 | `EntityCommands::id()` 获取实体 ID | `EntityCommands::id(&self) -> Entity` | self | `Entity` | 返回实体 ID | 88 | P0 |
| 232 | `Commands::spawn` 在 apply 后才实际插入世界 | 同需求 79 | 同需求 79 | 同需求 79 | apply 前 World 中不存在 | 79, 87 | P0 |
| 233 | `Commands::insert` 在 apply 后才实际插入组件 | 同需求 81 | 同需求 81 | 同需求 81 | apply 前实体无新组件 | 81, 87 | P0 |
| 234 | `Commands::remove` 在 apply 后才实际移除组件 | 同需求 82 | 同需求 82 | 同需求 82 | apply 前实体仍有组件 | 82, 87 | P0 |
| 235 | `Commands::despawn` 在 apply 后才实际移除实体 | 同需求 83 | 同需求 83 | 同需求 83 | apply 前实体仍存活 | 83, 87 | P0 |
| 236 | `Commands::insert_resource` / `remove_resource` 延迟生效 | 同需求 84-85 | 同需求 84-85 | 同需求 84-85 | apply 后资源才变化 | 84, 85, 87 | P0 |

### 2. Resource 资源单件

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 90 | `Resource<T>`：全局资源（单件） | `struct Resource<T>(T)` | 无 | 数据结构 | 包装单件数据 | 无 | P0 |
| 91 | `Resource` trait（可用 `#[derive(Resource)]`） | `trait Resource { }` | 无 | trait 定义 | 默认实现 | 90 | P0 |
| 170 | `#[derive(Resource)]` 宏 | `#[derive(Resource)]` | 结构体定义 | 派生实现 | 派生 Resource trait | 91 | P0 |
| 237 | `World::resource::<Time>()` 不可用资源时 panic | 同 01-ecs-world 需求 19 | 同需求 19 | 同需求 19 | panic 并显示资源名称 | 19 | P0 |
| 238 | `World::get_resource::<R>()` 返回 Option | 同 01-ecs-world 需求 20 | 同需求 20 | 同需求 20 | 不存在返回 None | 20 | P0 |
| 240 | `Local<T>`：系统本地状态 | `struct Local<T>(pub T)` | 无 | 数据结构 | 每个系统实例独有 | 96 | P0 |
| 241 | `Local<T>` 默认值 T: Default | 同需求 240 | 同需求 240 | 同需求 240 | 可用 `Local::default()` | 240 | P0 |
| 242 | `Local<T>::default` 支持 `#[local]` 自定义初始值 | `#[local(default = ...)]` | 属性 | 派生实现 | 支持自定义默认值 | 240 | P1 |
| 243 | 单元测试 `Commands` 应用正确 | `cargo test -p engine-ecs commands_apply` | 无 | 测试结果 | 测试通过 | 87 | P0 |

---

## 需求编号：119-127（额外类型）

### 3. 额外类型定义

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 119 | `NonSend`：不在线程间发送的资源标记 | `struct NonSend<T>(T)` | 无 | 标记类型 | 阻止资源跨线程 | 90 | P1 |
| 120 | `NonSendMut` | `struct NonSendMut<T>(T)` | 无 | 标记类型 | 阻止可变资源跨线程 | 119 | P1 |
| 121 | `SystemLabel / SystemSet / RunCriteriaLabel` 标记 | `trait SystemLabel { fn as_str(&self) -> &str; }` | 无 | trait 定义 | 用于系统组织和控制 | 67 | P1 |
| 126 | `Command` trait | `trait Command { fn apply(self, world: &mut World); }` | world | `()` | 命令执行接口 | 78 | P0 |

### 4. Commands 单元测试

| 需求ID | 描述 | 测试内容 | 验收标准 |
|--------|------|----------|----------|
| 243 | `Commands` 应用正确 | Commands 队列正确应用到 World | 延迟命令最终正确执行 |
| 283 | 单元测试 `Commands` | 命令队列的基本操作 | 测试通过 |

---

## 依赖关系图

```
Commands (78)
    │
    ├── spawn (79) -> EntityCommands (88)
    ├── spawn_batch (80)
    ├── insert (81)
    ├── remove (82)
    ├── despawn (83)
    ├── insert_resource (84)
    ├── remove_resource (85)
    ├── add (86)
    └── apply (87)
            │
            └── apply_to_world

EntityCommands (88)
    ├── insert (89)
    ├── remove (90)
    ├── despawn (91)
    ├── insert_resource (92)
    ├── remove_resource (93)
    └── id (94)

Command trait (126)
    └── apply

Resource<T> (90)
    └── #[derive(Resource)] (170, 91)

Local<T> (240)
    └── #[local] (242)
```

---

## 优先级说明

- **P0（关键）**：必须完成的核心功能，直接影响 Sprint 验收
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代