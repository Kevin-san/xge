# ECS World 核心需求

## 模块概述

ECS World 是整个 ECS 架构的核心容器，负责管理实体、组件存储、资源和事件系统。本模块定义了 World 的创建、实体生命周期管理、组件插入/移除、资源管理以及基本的世界状态查询功能。

---

## 需求编号：1-30, 166-167, 171-181

### 1. World 创建与初始化

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 1 | 建立 `engine-ecs` crate | N/A | N/A | N/A | crate 成功创建，cargo build 通过 | 无 | P0 |
| 2 | `World::new()` 创建空世界 | `World::new() -> World` | 无 | 空 World 实例 | 返回有效 World 实例，entities/components 为空 | 1 | P0 |
| 166 | `World::dump_stats()` 打印 archetype 信息 | `World::dump_stats(&self)` | self | `()` | 打印当前 archetype 数量、实体数量、组件数量 | 全部 | P1 |
| 167 | `World::validate()` 基本一致性校验 | `World::validate(&self) -> Result<()>` | self | `Result<()>` | 无一致性问题时返回 `Ok(())`，发现问题返回 `Err` | 全部 | P1 |

### 2. 实体 Spawn（生成）

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 3 | `spawn()` 生成实体 | `World::spawn(&mut self) -> Entity` | `&mut self` | `Entity` | 返回唯一新 Entity，generation=0 | 2 | P0 |
| 4 | `spawn_bundle()` 批量组件插入 | `World::spawn_bundle(&mut self, bundle: B) -> Entity` | bundle: 实现 Bundle trait | `Entity` | 返回实体 ID，bundle 中所有组件被正确插入 | 2, 45 | P0 |
| 5 | `spawn_batch()` 批量生成 | `World::spawn_batch(&mut self, bundles: impl Iterator<Item = impl Bundle>)` | bundles 迭代器 | `()` | 性能符合 benchmark，100k 实体 < 100ms | 4 | P0 |
| 172 | `spawn()` 生成唯一 Entity | 同需求 3 | 同需求 3 | 同需求 3 | 每次调用返回不同 Entity | 2 | P0 |
| 173 | `spawn_bundle()` 一次调用插入多个组件 | 同需求 4 | 同需求 4 | 同需求 4 | 组件全部插入成功 | 2, 45 | P0 |
| 174 | `spawn_batch()` 批量 spawn 性能达标 | 同需求 5 | 同需求 5 | 同需求 5 | benchmark: 100k 实体 < 100ms | 4 | P0 |

### 3. 实体 Despawn（销毁）

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 6 | `despawn()` 销毁实体 | `World::despawn(&mut self, entity: Entity) -> bool` | entity | `bool` | 返回是否成功销毁，实体不存在返回 false | 3 | P0 |
| 7 | `clear_entities()` 清空所有实体 | `World::clear_entities(&mut self)` | `&mut self` | `()` | 所有实体被销毁，World 可继续使用 | 6 | P0 |
| 175 | `despawn()` 回收实体并清理组件 | 同需求 6 | 同需求 6 | 同需求 6 | 实体及所有组件被移除 | 6 | P0 |
| 176 | `despawn()` 在空实体上安全调用 | 同需求 6 | 同需求 6 | 同需求 6 | 对已销毁实体调用不 panic | 6 | P0 |

### 4. 实体存活状态查询

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 12 | `entity()` 获取 EntityRef | `World::entity(&self, entity: Entity) -> EntityRef` | entity | `EntityRef` | 返回实体引用，可查询组件 | 3 | P0 |
| 13 | `entity_mut()` 获取 EntityMut | `World::entity_mut(&mut self, entity: Entity) -> EntityMut` | entity | `EntityMut` | 返回可变实体引用 | 3 | P0 |
| 14 | `contains()` 检查实体存活 | `World::contains(&self, entity: Entity) -> bool` | entity | `bool` | 存活返回 true，否则 false | 3 | P0 |
| 177 | `contains()` 正确反映存活状态 | 同需求 14 | 同需求 14 | 同需求 14 | 与实际存活状态一致 | 14 | P0 |
| 178 | `clear_entities()` 清空后可继续工作 | 同需求 7 | 同需求 7 | 同需求 7 | 清空后仍可 spawn 新实体 | 7 | P0 |

### 5. 组件插入与移除

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 8 | `insert()` 插入单个组件 | `World::insert(&mut self, entity: Entity, component: C) -> Option<C>` | entity, component | `Option<C>` | 返回被替换的旧组件（如果有） | 3, 39 | P0 |
| 9 | `insert_bundle()` 插入组件包 | `World::insert_bundle(&mut self, entity: Entity, bundle: B) -> bool` | entity, bundle | `bool` | bundle 所有组件被插入 | 3, 45 | P0 |
| 10 | `remove::<C>()` 移除单个组件 | `World::remove::<C>(&mut self, entity: Entity) -> Option<C>` | entity | `Option<C>` | 返回被移除的组件 | 3, 39 | P0 |
| 11 | `remove_bundle::<Bundle>()` 移除组件包 | `World::remove_bundle::<B>(&mut self, entity: Entity) -> bool` | entity | `bool` | bundle 所有组件被移除 | 3, 45 | P0 |

### 6. 组件获取

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 15 | `get_component::<C>()` 只读获取 | `World::get_component::<C>(&self, entity: Entity) -> Option<&C>` | entity | `Option<&C>` | 存在返回 Some(&C)，否则 None | 3, 39 | P0 |
| 16 | `get_component_mut::<C>()` 可变获取 | `World::get_component_mut::<C>(&mut self, entity: Entity) -> Option<&mut C>` | entity | `Option<&mut C>` | 存在返回 Some(&mut C)，否则 None | 3, 39 | P0 |

### 7. World 内部状态访问

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 17 | `entities()` 获取 Entities 引用 | `World::entities(&self) -> &Entities` | self | `&Entities` | 返回实体表引用 | 2 | P0 |
| 18 | `components()` 获取 Components 引用 | `World::components(&self) -> &Components` | self | `&Components` | 返回组件存储引用 | 2 | P0 |

---

## 需求编号：31-54（资源与事件部分）

### 8. 资源管理

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 19 | `resource::<R>()` 获取资源只读引用 | `World::resource::<R>(&self) -> &R` | self | `&R` | 资源不存在时 panic | 2, 90 | P0 |
| 20 | `resource_mut::<R>()` 获取资源可变引用 | `World::resource_mut::<R>(&mut self) -> &mut R` | self | `&mut R` | 资源不存在时 panic | 2, 90 | P0 |
| 21 | `insert_resource()` 插入资源 | `World::insert_resource(&mut self, resource: R)` | resource | `()` | 资源被插入，覆盖旧值 | 2, 90 | P0 |
| 22 | `remove_resource::<R>()` 移除资源 | `World::remove_resource::<R>(&mut self) -> Option<R>` | self | `Option<R>` | 返回被移除的资源 | 2, 90 | P0 |
| 23 | `contains_resource::<R>()` 检查资源存在 | `World::contains_resource::<R>(&self) -> bool` | self | `bool` | 存在返回 true | 2, 90 | P0 |
| 50 | `insert_resource()` 插入资源（重复） | 同需求 21 | 同需求 21 | 同需求 21 | 同需求 21 | 21 | P0 |
| 237 | `resource::<Time>()` 不可用资源时 panic | 同需求 19 | 同需求 19 | 同需求 19 | panic 并显示有用信息 | 19 | P0 |
| 238 | `get_resource::<R>()` 返回 Option | `World::get_resource::<R>(&self) -> Option<&R>` | self | `Option<&R>` | 存在返回 Some(&R)，否则 None | 19 | P0 |

### 9. 事件系统

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 24 | `send_event::<E>()` 发送事件 | `World::send_event::<E>(&mut self, event: E)` | event | `()` | 事件进入 EventWriter 队列 | 2, 92 | P0 |
| 25 | `events::<E>()` 获取事件读取器 | `World::events::<E>(&self) -> EventReader<E>` | self | `EventReader<E>` | 返回可迭代的事件读取器 | 2, 92 | P0 |

---

## 需求编号：55-62（系统调度部分）

### 10. 系统运行

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 26 | `run_system()` 运行系统 | `World::run_system(&mut self, system_fn: impl FnOnce(&mut World))` | system_fn | `()` | 系统函数被执行 | 2 | P0 |
| 27 | `run_system_catched()` 捕获错误运行 | `World::run_system_catched(&mut self, system_fn: impl FnOnce(&mut World)) -> Result<()>` | system_fn | `Result<()>` | 捕获系统 panic 返回 Err | 2 | P0 |
| 28 | `schedule()` 获取调度器 | `World::schedule(&mut self, name: &str) -> &mut Schedule` | name | `&mut Schedule` | 返回指定名称的调度器 | 2, 73 | P0 |
| 29 | `add_system_to_stage()` 添加系统到阶段 | `World::add_system_to_stage(&mut self, stage: impl StageLabel, system: impl IntoSystem)` | stage, system | `()` | 系统被添加到指定阶段 | 2, 72 | P0 |
| 30 | `add_system_set()` 添加系统集 | `World::add_system_set(&mut self, system_set: impl SystemSet)` | system_set | `()` | 系统集被添加 | 2, 75 | P0 |
| 31 | `insert_resource()` 插入资源（重复） | 同需求 21 | 同需求 21 | 同需求 21 | 同需求 21 | 21 | P0 |
| 32 | `add_stage_after()` 在现有阶段后添加 | `World::add_stage_after(&mut self, existing: impl StageLabel, name: &str, stage: impl Stage)` | existing, name, stage | `()` | 新阶段被添加到现有阶段之后 | 2, 74 | P1 |
| 33 | `add_stage_before()` 在现有阶段前添加 | `World::add_stage_before(&mut self, existing: impl StageLabel, name: &str, stage: impl Stage)` | existing, name, stage | `()` | 新阶段被添加到现有阶段之前 | 2, 74 | P1 |

---

## 需求编号：171-181（Entity 核心验收）

### 11. Entity 结构与实现

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 34 | `Entity` 结构体定义 | `struct Entity { id: u32, generation: u32 }` | N/A | N/A | id 和 generation 字段存在 | 3 | P0 |
| 35 | `Entity::id()` 获取索引 | `Entity::id(&self) -> u32` | self | `u32` | 返回实体索引 | 34 | P0 |
| 36 | `Entity::generation()` 获取代际 | `Entity::generation(&self) -> u32` | self | `u32` | 返回实体代际 | 34 | P0 |
| 37 | `Entity::null()` 创建空实体 | `Entity::null() -> Entity` | 无 | `Entity` | 返回 id=u32::MAX, generation=0 的实体 | 34 | P0 |
| 38 | `Entity::is_null()` 检查空实体 | `Entity::is_null(&self) -> bool` | self | `bool` | 空实体返回 true | 34 | P0 |
| 179 | `Entity::id()` 唯一且稳定 | 同需求 35 | 同需求 35 | 同需求 35 | 实体存活期间 id 不变 | 35 | P0 |
| 180 | `Entity::generation()` 防止 ABA 问题 | 同需求 36 | 同需求 36 | 同需求 36 | despawn 后相同 id 的新实体 generation 增加 | 36 | P0 |
| 181 | `Entity` 实现 `Copy + Eq + Hash + Send + Sync` | N/A | N/A | N/A | Entity 可以拷贝、相等比较、可哈希、可跨线程 | 34 | P0 |
| 182 | `Entity` 可作为 HashMap 键 | N/A | N/A | N/A | 可用 Entity 作为 HashMap<K,V> 的键 | 181 | P0 |

---

## 依赖关系图

```
engine-ecs crate (1)
    │
    ├── World::new() (2)
    │       │
    │       ├── Entity 实体 (34-38, 179-182)
    │       │       │
    │       │       └── Component trait (39)
    │       │               │
    │       │               ├── #[derive(Component)] (40)
    │       │               └── ComponentStorage (41-44)
    │       │
    │       ├── Bundle trait (45-48)
    │       │
    │       ├── SparseSet Storage (42)
    │       ├── DenseVec Storage (43)
    │       └── HashMapStorage (44)
    │
    ├── Resources (19-23, 50, 237-238)
    │       │
    │       └── Resource trait (90)
    │               │
    │               └── #[derive(Resource)] (91)
    │
    ├── Events (24-25)
    │       │
    │       └── Event trait (92)
    │               │
    │               └── #[derive(Event)] (93)
    │
    └── Schedule (28-33)
            │
            ├── System (67-69)
            ├── SystemStage (72-73)
            └── Stage (74)
```

---

## 优先级说明

- **P0（关键）**：必须完成的核心功能，直接影响 Sprint 验收
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代