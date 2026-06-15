# 实体与组件需求

## 模块概述

本模块定义了 ECS 架构中的核心数据类型：Entity（实体）、Component（组件）、ComponentStorage（组件存储）以及 Archetype（原型）。组件是 ECS 架构中承载数据的载体，实体是组件的容器，存储类型决定了组件数据的组织方式和访问性能。

---

## 需求编号：31-54, 63-74, 183-243

### 1. Component Trait 与存储

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 39 | `Component` trait 定义 | `trait Component { type Storage = SparseSet<C>; }` | 无 | trait 定义 | 包含关联类型 Storage | 无 | P0 |
| 40 | `#[derive(Component)]` 派生宏 | `#[derive(Component)]` | 结构体定义 | 派生实现 | 默认使用 SparseSet 存储 | 39 | P0 |
| 41 | `ComponentStorage<C>` 抽象 trait | `trait ComponentStorage<C: Component> { ... }` | 无 | trait 定义 | 定义 insert/get/remove/iter/len 等方法 | 39 | P0 |
| 184 | `#[derive(Component)]` 默认使用 SparseSet | 同需求 40 | 同需求 40 | 同需求 40 | 未指定 storage 时使用 SparseSet | 40 | P0 |
| 185 | `#[component(storage = "DenseVec")]` 切换存储 | `#[component(storage = "DenseVec")]` | 结构体定义 | 派生实现 | 可通过属性切换存储类型 | 40 | P0 |
| 225 | `Component` trait 提供 `type Storage` 关联类型 | 同需求 39 | 同需求 39 | 同需求 39 | 同需求 39 | 39 | P0 |

### 2. SparseSet 存储

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 42 | `SparseSet<T>`：稀疏索引 + 密集数组 | `struct SparseSet<T> { sparse: Vec<usize>, dense: Vec<T>, ... }` | 无 | 数据结构 | 稀疏索引支持 O(1) 查找 | 41 | P0 |
| 227 | `SparseSet::insert()` 插入 | `SparseSet::insert(&mut self, entity: Entity, value: T)` | entity, value | `()` | 组件被插入，返回旧值（如果有） | 42 | P0 |
| 228 | `SparseSet::get()` 获取 | `SparseSet::get(&self, entity: Entity) -> Option<&T>` | entity | `Option<&T>` | O(1) 查找，存在返回引用 | 42 | P0 |
| 229 | `SparseSet::remove()` 移除 | `SparseSet::remove(&mut self, entity: Entity) -> Option<T>` | entity | `Option<T>` | 组件被移除，返回值 | 42 | P0 |
| 230 | `SparseSet::iter()` 只读迭代 | `SparseSet::iter(&self) -> Iter<T>` | self | `Iter<T>` | 返回所有组件的迭代器 | 42 | P0 |
| 231 | `SparseSet::iter_mut()` 可变迭代 | `SparseSet::iter_mut(&mut self) -> IterMut<T>` | self | `IterMut<T>` | 返回所有组件的可变迭代器 | 42 | P0 |
| 232 | `SparseSet::contains()` 检查存在 | `SparseSet::contains(&self, entity: Entity) -> bool` | entity | `bool` | 存在返回 true | 42 | P0 |
| 233 | `SparseSet::len()` 长度 | `SparseSet::len(&self) -> usize` | self | `usize` | 返回组件数量 | 42 | P0 |

### 3. DenseVec 存储

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 43 | `DenseVec<T>`：索引即数组下标 | `struct DenseVec<T> { data: Vec<T>, ... }` | 无 | 数据结构 | 紧凑存储，适合频繁遍历 | 41 | P0 |
| 234 | DenseVec 插入/删除/查找/迭代 | 同 SparseSet 接口 | 同 SparseSet | 同 SparseSet | 与 SparseSet 接口一致 | 43 | P0 |

### 4. HashMapStorage 存储

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 44 | `HashMapStorage<T>`：散列存储 | `struct HashMapStorage<T> { data: HashMap<Entity, T> }` | 无 | 数据结构 | 使用 HashMap 组织组件 | 41 | P0 |
| 235 | HashMapStorage 插入/删除/查找/迭代 | 同 SparseSet 接口 | 同 SparseSet | 同 SparseSet | 与 SparseSet 接口一致 | 44 | P0 |

---

## 需求编号：45-48, 74（Bundle 相关）

### 5. Bundle Trait 与宏

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 45 | `Bundle` trait + `#[derive(Bundle)]` 宏 | `trait Bundle { ... }` | 无 | trait 定义 | 支持批量组件操作 | 39 | P0 |
| 46 | `Bundle::bundle_components()` 遍历组件 | `Bundle::bundle_components(&self, func: impl FnMut(ComponentId, &dyn Component))` | func | `()` | 对每个组件调用 func | 45 | P0 |
| 47 | `Bundle::from_components()` 从组件构造 | `Bundle::from_components(func: impl FnOnce(ComponentId) -> Box<dyn Component>) -> Self` | func | `Self` | 从组件构造 Bundle | 45 | P0 |
| 48 | `Bundle::bundle_id()` 获取 Bundle ID | `Bundle::bundle_id(&self) -> BundleId` | self | `BundleId` | 返回唯一标识 | 45 | P0 |
| 169 | `#[derive(Bundle)]` 宏 | 同需求 45 | 同需求 45 | 同需求 45 | 派生宏正确生成 Bundle 实现 | 45 | P0 |

---

## 需求编号：98-105（Archetype 相关）

### 6. Archetype 原型系统

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 98 | `Archetype`：相同组件集合的实体分组 | `struct Archetype { ... }` | 无 | 数据结构 | 按组件集合分组实体 | 39 | P0 |
| 99 | `Archetype::id()` 获取原型 ID | `Archetype::id(&self) -> ArchetypeId` | self | `ArchetypeId` | 返回唯一标识 | 98 | P0 |
| 100 | `Archetype::entities()` 获取实体列表 | `Archetype::entities(&self) -> &[Entity]` | self | `&[Entity]` | 返回属于此原型的所有实体 | 98 | P0 |
| 101 | `Archetype::component_ids()` 获取组件 ID 列表 | `Archetype::component_ids(&self) -> &[ComponentId]` | self | `&[ComponentId]` | 返回此原型包含的组件 ID | 98 | P0 |
| 102 | `Archetype::get::<C>()` 获取组件数组 | `Archetype::get::<C>(&self) -> Option<&[C]>` | self | `Option<&[C]>` | 返回组件数据的只读切片 | 98 | P0 |
| 103 | `ArchetypeGraph`：archetype 迁移边 | `struct ArchetypeGraph { edges: HashMap<ArchetypeId, Vec<ArchetypeId>> }` | 无 | 数据结构 | 缓存 archetype 迁移路径 | 98 | P0 |
| 195 | `Archetype` 按组件集合自动创建 | 同需求 98 | 同需求 98 | 同需求 98 | spawn 时根据组件集合选择/创建 archetype | 98 | P0 |
| 196 | `Archetype` 组件按 ComponentId 排序 | 同需求 101 | 同需求 101 | 同需求 101 | component_ids 返回排序后的列表 | 101 | P0 |
| 197 | `Archetype` 内部组件数组 `SoA` 对齐 | 同需求 98 | 同需求 98 | 同需求 98 | 组件数据按类型紧凑存储 (Structure of Arrays) | 98 | P0 |
| 198 | 实体 spawn 时选择正确 archetype | 同需求 3 | 同需求 3 | 同需求 3 | 根据组件集合选择正确的 archetype | 3, 98 | P0 |
| 199 | insert 新组件时迁移到新 archetype | 同需求 8 | 同需求 8 | 同需求 8 | 组件插入后实体位于正确的 archetype | 8, 98 | P0 |
| 200 | remove 组件时迁移到新 archetype | 同需求 10 | 同需求 10 | 同需求 10 | 组件移除后实体位于正确的 archetype | 10, 98 | P0 |
| 201 | `ArchetypeGraph` 缓存迁移路径 | 同需求 103 | 同需求 103 | 同需求 103 | 迁移查找高效 | 103 | P0 |
| 239 | 单元测试 `Archetype` 组件数组对齐正确 | `cargo test archetype_soa_alignment` | 无 | 测试结果 | 测试通过 | 197 | P0 |

---

## 需求编号：63-73（Entity 基础结构）

### 7. Entity 结构与操作

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 34 | `Entity` 结构体 | `struct Entity { id: u32, generation: u32 }` | 无 | 数据结构 | id + generation | 无 | P0 |
| 35 | `Entity::id()` | 同 01-ecs-world.md | - | - | - | 34 | P0 |
| 36 | `Entity::generation()` | 同 01-ecs-world.md | - | - | - | 34 | P0 |
| 37 | `Entity::null()` | 同 01-ecs-world.md | - | - | - | 34 | P0 |
| 38 | `Entity::is_null()` | 同 01-ecs-world.md | - | - | - | 34 | P0 |
| 179-182 | Entity 验收标准 | 同 01-ecs-world.md | - | - | - | 34 | P0 |

---

## 需求编号：106-113（变更检测与 Tick）

### 8. ChangeTrackers 变更检测

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 104 | `ChangeTrackers`：检测组件变更 | `struct ChangeTrackers { added: Tick, changed: Tick, last_changed: Tick }` | 无 | 数据结构 | 跟踪组件添加和变更时间 | 98 | P0 |
| 105 | `Tick`：帧计数代际 | `struct Tick { value: u32 }` | 无 | 数据结构 | 每帧递增，支持比较 | 104 | P0 |
| 106 | `Ticks<T>`：`added / changed / last_changed` | `struct Ticks<T> { added: Tick, changed: Tick, last_changed: Tick }` | 无 | 数据结构 | 存储组件的时间戳信息 | 104 | P0 |
| 107 | `Ref<T>`：组件引用（含变更检测） | `struct Ref<'a, T> { value: &'a T, ticks: Ticks<T> }` | 无 | 数据结构 | 访问数据时可检测变更 | 106 | P0 |
| 108 | `Reflect`：动态反射 trait | `trait Reflect { ... }` | 无 | trait 定义 | 定义（后续实现） | 无 | P2 |
| 255 | `Tick` 每帧递增 | `Tick::tick(&mut self)` | self | `()` | 每次调用 value++，溢出回绕 | 105 | P0 |
| 256 | `Ref<T>` 暴露 `is_added()` / `is_changed()` | `Ref<T>::is_added(&self) -> bool` | self | `bool` | 本帧新增返回 true | 107 | P0 |
| 257 | `Changed<T>` 过滤仅匹配本帧变化的 | 同需求 63 | 同需求 63 | 同需求 63 | 仅当 changed tick == current tick 时匹配 | 63, 255 | P0 |
| 258 | `Added<T>` 过滤仅匹配本帧新增的 | 同需求 64 | 同需求 64 | 同需求 64 | 仅当 added tick == current tick 时匹配 | 64, 255 | P0 |
| 259 | `RemovedComponents<T>` 迭代被移除的实体 | `struct RemovedComponents<'w, T> { ... }` | 无 | 数据结构 | 返回被移除组件的实体 | 104 | P0 |
| 260 | `World::clear_trackers()` 手动重置 | `World::clear_trackers(&mut self)` | self | `()` | 所有 tracker 被重置 | 104 | P0 |

---

## 依赖关系图

```
Component trait (39)
    │
    ├── #[derive(Component)] (40)
    │       │
    │       └── ComponentStorage<C> (41)
    │               │
    │               ├── SparseSet<T> (42)
    │               │       ├── insert (227)
    │               │       ├── get (228)
    │               │       ├── remove (229)
    │               │       ├── iter/iter_mut (230-231)
    │               │       ├── contains (232)
    │               │       └── len (233)
    │               │
    │               ├── DenseVec<T> (43)
    │               │       └── 同 SparseSet 接口 (234)
    │               │
    │               └── HashMapStorage<T> (44)
    │                       └── 同 SparseSet 接口 (235)
    │
    └── Bundle trait (45)
            ├── bundle_components (46)
            ├── from_components (47)
            └── bundle_id (48)

Archetype (98)
    ├── id (99)
    ├── entities (100)
    ├── component_ids (101)
    ├── get::<C> (102)
    └── ArchetypeGraph (103)

ChangeTrackers (104)
    └── Tick (105)
            └── Ticks<T> (106)
                    └── Ref<T> (107)
```

---

## 单元测试要求

| 需求ID | 描述 | 测试命令 |
|--------|------|----------|
| 239 | `Archetype` 组件数组 SoA 对齐正确 | `cargo test -p engine-ecs archetype_soa_alignment` |
| 289 | `SparseSet` 插入/删除/查找 | `cargo test -p engine-ecs sparse_set_operations` |
| 290 | `DenseVec` 索引 | `cargo test -p engine-ecs dense_vec_indexing` |
| 291 | `Archetype` 迁移 | `cargo test -p engine-ecs archetype_migration` |

---

## 优先级说明

- **P0（关键）**：必须完成的核心功能
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代