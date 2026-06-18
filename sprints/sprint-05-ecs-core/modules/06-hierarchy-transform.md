# 层级与变换需求

## 模块概述

Hierarchy 层级系统通过 Parent/Children 组件建立实体间的父子关系，Transform 变换系统管理 2D 空间中的位置、旋转和缩放。父子变换通过 GlobalTransform 自动传播计算。

---

## 需求编号：106-113, 246-260, 299-308

### 1. Hierarchy 层级系统

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 110 | `Hierarchy`：`Parent / Children` 组件 | `struct Hierarchy { parent: Option<Entity>, children: Vec<Entity> }` | 无 | 数据结构 | 存储父子关系 | 34 | P0 |
| 111 | `Parent(entity)` 组件 | `struct Parent(pub Entity)` | 无 | 组件 | 指向父实体 | 110 | P0 |
| 112 | `Children(Vec<Entity>)` 组件 | `struct Children(pub Vec<Entity>)` | 无 | 组件 | 反向索引子实体 | 110 | P0 |
| 113 | `BuildChildren` trait | `trait BuildChildren { fn push_children(&mut self, children: &[Entity]); ... }` | 无 | trait 定义 | `push_children / insert_children / remove_children` | 110 | P0 |
| 246 | `Parent` 组件指向父实体 | 同需求 111 | 同需求 111 | 同需求 111 | Parent.0 正确指向父实体 | 111 | P0 |
| 247 | `Children(Vec<Entity>)` 反向索引 | 同需求 112 | 同需求 112 | 同需求 112 | Children.0 包含所有子实体 | 112 | P0 |
| 248 | `push_child(parent, child)` 建立双向关系 | `fn push_child(parent: Entity, child: Entity, world: &mut World)` | parent, child, world | `()` | parent 的 Children 包含 child，child 的 Parent 指向 parent | 111, 112 | P0 |
| 249 | `remove_child(parent, child)` 解除双向关系 | `fn remove_child(parent: Entity, child: Entity, world: &mut World)` | parent, child, world | `()` | parent 的 Children 不包含 child，child 的 Parent 为 None | 111, 112 | P0 |
| 250 | `despawn_recursive(entity)` 递归销毁子树 | `fn despawn_recursive(entity: Entity, world: &mut World)` | entity, world | `()` | 实体及其所有后代被销毁 | 83, 248 | P0 |

### 2. Transform 2D 变换系统

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 251 | `Transform2D`（位置/旋转/缩放）组件 | `struct Transform2D { translation: Vec2, rotation: f32, scale: Vec2 }` | 无 | 组件 | 存储局部变换 | 39 | P0 |
| 252 | `GlobalTransform2D` 组件 | `struct GlobalTransform2D(Mat3)` | 无 | 组件 | 存储世界变换 | 39 | P0 |
| 253 | `transform_propagate_system` 计算 GlobalTransform2D | `fn transform_propagate_system(world: &mut World)` | world | `()` | 遍历层级计算 GlobalTransform2D | 251, 252 | P0 |
| 254 | Transform 传播在 PreUpdate 阶段执行 | `Schedule::add_system_to_stage(PreUpdate, transform_propagate_system)` | stage | `()` | 在 PreUpdate 阶段执行 | 253 | P0 |
| 263 | `examples/ecs_hierarchy` 父子 Transform 传播 | `cargo run --example ecs_hierarchy` | 无 | 示例运行 | 父子 Transform 正确传播 | 253 | P0 |

---

## 需求编号：106-113（ChangeTrackers 相关）

### 3. ChangeTrackers 变更检测（ Hierarchy 相关）

| 需求ID | 描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|------|----------|------|------|----------|------|--------|
| 104 | `ChangeTrackers`：检测组件变更 | 同 02-entity-component.md | - | - | 见 02-entity-component.md | 98 | P0 |
| 105 | `Tick`：帧计数代际 | 同 02-entity-component.md | - | - | - | 104 | P0 |
| 106 | `Ticks<T>`：`added / changed / last_changed` | 同 02-entity-component.md | - | - | - | 104 | P0 |
| 107 | `Ref<T>`：组件引用（含变更检测） | 同 02-entity-component.md | - | - | - | 106 | P0 |
| 255 | `Tick` 每帧递增 | 同 02-entity-component.md | - | - | - | 105 | P0 |
| 256 | `Ref<T>` 暴露 `is_added()` / `is_changed()` | 同 02-entity-component.md | - | - | - | 107 | P0 |
| 257 | `Changed<T>` 过滤仅匹配本帧变化的 | 同 02-entity-component.md | - | - | - | 63 | P0 |
| 258 | `Added<T>` 过滤仅匹配本帧新增的 | 同 02-entity-component.md | - | - | - | 64 | P0 |
| 259 | `RemovedComponents<T>` 迭代被移除的实体 | 同 02-entity-component.md | - | - | - | 65 | P0 |
| 260 | `World::clear_trackers()` 手动重置 | 同 02-entity-component.md | - | - | - | 104 | P0 |

---

## 依赖关系图

```
Transform2D (251)
    ├── translation: Vec2
    ├── rotation: f32
    └── scale: Vec2

GlobalTransform2D (252)
    └── matrix: Mat3

Parent (111)
    └── Entity

Children (112)
    └── Vec<Entity>

Hierarchy (110)
    ├── Parent component
    └── Children component

BuildChildren (113)
    ├── push_children (248)
    ├── insert_children
    └── remove_children (249)

transform_propagate_system (253)
    │
    └── 在 PreUpdate 阶段执行 (254)
```

---

## 单元测试要求

| 需求ID | 描述 | 测试命令 |
|--------|------|----------|
| 284 | 单元测试 `Hierarchy` Children 跟随 Parent | `cargo test -p engine-ecs hierarchy_children` |
| 285 | 单元测试 `Schedule` 阶段顺序正确 | `cargo test -p engine-ecs schedule_order` |
| 287 | 单元测试 `Changed/Added` 正确 | `cargo test -p engine-ecs query_changed_added` |
| 290 | `examples/ecs_hierarchy` 父子 Transform 传播 | `cargo run --example ecs_hierarchy` |

---

## 优先级说明

- **P0（关键）**：必须完成的核心功能，直接影响 Sprint 验收
- **P1（重要）**：对功能完整性有重要影响
- **P2（期望）**：增强功能，可后续迭代