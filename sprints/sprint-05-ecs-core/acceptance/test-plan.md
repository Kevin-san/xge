# 测试计划

## 模块概述

本文档定义 `engine-ecs` crate 的完整测试策略，包括单元测试、集成测试、性能测试和验收测试。所有测试必须通过才能完成 Sprint 05。

---

## 需求编号：278-300, 340-363

### 1. 单元测试（Unit Tests）

#### 1.1 World 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-W01 | World spawn/despawn 不泄漏 | 176, 294 | 多次 spawn/despawn 后实体计数正确 | `cargo test -p engine-ecs world_spawn_despawn` |
| UT-W02 | World clear_entities | 7, 178 | clear 后 World 可继续使用 | `cargo test -p engine-ecs world_clear_entities` |
| UT-W03 | World validate | 167 | validate 返回正确结果 | `cargo test -p engine-ecs world_validate` |

#### 1.2 Entity 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-E01 | Entity Copy + Eq + Hash | 181 | Entity 可拷贝、相等、可哈希 | `cargo test -p engine-ecs entity_copy_eq_hash` |
| UT-E02 | Entity generation | 180 | despawn 后新实体 generation 增加 | `cargo test -p engine-ecs entity_generation` |
| UT-E03 | Entity id 唯一稳定 | 179 | 存活期间 id 不变 | `cargo test -p engine-ecs entity_id_stable` |

#### 1.3 Component/Storage 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-C01 | SparseSet 插入/删除/查找 | 289 | 基本操作正确 | `cargo test -p engine-ecs sparse_set_operations` |
| UT-C02 | SparseSet 迭代器 | 230-231 | iter/iter_mut 正确 | `cargo test -p engine-ecs sparse_set_iter` |
| UT-C03 | DenseVec 索引 | 290, 234 | 索引操作正确 | `cargo test -p engine-ecs dense_vec_indexing` |
| UT-C04 | HashMapStorage 操作 | 235, 44 | 基本操作正确 | `cargo test -p engine-ecs hashmap_storage_operations` |
| UT-C05 | Component 派生宏 | 40, 168 | #[derive(Component)] 正确生成 | `cargo test -p engine-ecs component_derive` |

#### 1.4 Bundle 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-B01 | Bundle 往返 | 281, 159 | Bundle -> components -> Bundle 正确 | `cargo test -p engine-ecs bundle_roundtrip` |
| UT-B02 | Bundle 派生宏 | 169, 45 | #[derive(Bundle)] 正确生成 | `cargo test -p engine-ecs bundle_derive` |
| UT-B03 | Bundle spawn_batch | 5, 174 | 批量 spawn 正确 | `cargo test -p engine-ecs bundle_spawn_batch` |

#### 1.5 Query 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-Q01 | Query With/Without 过滤 | 279, 206-207 | 过滤正确 | `cargo test -p engine-ecs query_with_without` |
| UT-Q02 | Query Changed/Added | 280, 208-209 | 变更过滤正确 | `cargo test -p engine-ecs query_changed_added` |
| UT-Q03 | Query::single panic | 215, 349 | 0 或 >1 时 panic | `cargo test -p engine-ecs query_single_panic` |
| UT-Q04 | Query::iter_mut 借用 | 287, 350 | 不与其他借用冲突 | `cargo test -p engine-ecs query_iter_mut_borrow` |
| UT-Q05 | Query get 返回 None | 213-214 | 未找到时返回 None | `cargo test -p engine-ecs query_get_none` |
| UT-Q06 | Query par_for_each 线程安全 | 218 | 无 data race | `cargo test -p engine-ecs query_par_for_each` |

#### 1.6 System 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-S01 | System 基本运行 | 67, 26 | run 方法正确执行 | `cargo test -p engine-ecs system_run` |
| UT-S02 | SystemParam Res/ResMut | 220-221 | 资源参数正确解析 | `cargo test -p engine-ecs system_param_resource` |
| UT-S03 | SystemParam Query | 219 | Query 参数正确解析 | `cargo test -p engine-ecs system_param_query` |
| UT-S04 | System 并行无竞争 | 288, 218 | 无 data race | `cargo test -p engine-ecs parallel_system` |

#### 1.7 Schedule 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-SC01 | Schedule 阶段顺序 | 285, 228 | 阶段按顺序执行 | `cargo test -p engine-ecs schedule_order` |
| UT-SC02 | Schedule run 不崩溃 | 229 | 完整执行无 panic | `cargo test -p engine-ecs schedule_run` |
| UT-SC03 | SystemSet disable | 230, 75 | disabled 时跳过 | `cargo test -p engine-ecs system_set_disable` |

#### 1.8 Commands 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-CM01 | Commands 延迟生效 | 283, 232-236 | apply 前命令未生效 | `cargo test -p engine-ecs commands_apply` |
| UT-CM02 | Commands spawn | 79, 232 | spawn 延迟正确 | `cargo test -p engine-ecs commands_spawn` |
| UT-CM03 | Commands insert/remove | 81-82, 233-234 | insert/remove 延迟正确 | `cargo test -p engine-ecs commands_insert_remove` |
| UT-CM04 | EntityCommands 链式调用 | 88-94 | 链式 API 正确 | `cargo test -p engine-ecs entity_commands_chain` |

#### 1.9 Events 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-EV01 | Events 双缓冲清理 | 282, 239 | update 清理旧事件 | `cargo test -p engine-ecs events_double_buffer` |
| UT-EV02 | EventReader iter | 240-241 | 仅返回新事件，多 reader 独立 | `cargo test -p engine-ecs event_reader_iter` |
| UT-EV03 | EventWriter send | 242-243 | 发送立即可读 | `cargo test -p engine-ecs event_writer_send` |

#### 1.10 Hierarchy 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-H01 | Children 跟随 Parent | 284, 248 | push_child 后关系正确 | `cargo test -p engine-ecs hierarchy_children` |
| UT-H02 | remove_child | 249 | 解除关系正确 | `cargo test -p engine-ecs hierarchy_remove_child` |
| UT-H03 | despawn_recursive | 250, 83 | 递归销毁正确 | `cargo test -p engine-ecs despawn_recursive` |

#### 1.11 Archetype 测试

| 测试ID | 描述 | 需求ID | 测试内容 | 命令 |
|--------|------|--------|----------|------|
| UT-A01 | Archetype 迁移 | 291, 199-200 | insert/remove 后迁移正确 | `cargo test -p engine-ecs archetype_migration` |
| UT-A02 | Archetype SoA 对齐 | 239, 197 | 组件数组对齐正确 | `cargo test -p engine-ecs archetype_soa_alignment` |
| UT-A03 | Archetype 选择正确 | 198-199 | spawn 时选择正确 archetype | `cargo test -p engine-ecs archetype_selection` |

---

### 2. 集成测试（Integration Tests）

| 测试ID | 描述 | 需求ID | 测试内容 |
|--------|------|--------|----------|
| IT-01 | 10 万实体压力测试 | 153, 322 | spawn 100k 实体不崩溃 |
| IT-02 | 循环迭代 100k | 262, 332 | Query::iter 100k 实体 < 10ms |
| IT-03 | 并行迭代 100k | 265, 323 | Query::par_for_each 100k 实体 < 5ms |
| IT-04 | 事件多 reader | 241, 125 | 多 EventReader 独立读取 |
| IT-05 | 资源竞争 | 237 | 错误资源访问 panic |

---

### 3. 性能测试（Performance Tests）

#### 3.1 Criterion Benchmark

| 测试ID | 描述 | 需求ID | 指标 |
|--------|------|--------|------|
| BT-01 | `ecs_query_iter_100k` | 273, 333 | Query::iter 100k < 10ms |
| BT-02 | `ecs_query_par_100k` | 274, 334 | Query::par_for_each 100k < 5ms |
| BT-03 | `ecs_spawn_100k` | 275, 335 | spawn_batch 100k < 100ms |
| BT-04 | `ecs_insert_bundle` | 276, 336 | insert_bundle 性能稳定 |

#### 3.2 验收性能指标

| 场景 | 指标 | 需求ID |
|------|------|--------|
| 10 万实体移动 + 绘制 | >= 60fps | 200, 262 |
| 并行 vs 单线程 | >= 1.5x | 265, 323 |

---

### 4. 示例测试（Example Tests）

| 测试ID | 描述 | 需求ID | 命令 |
|--------|------|--------|------|
| ET-01 | ecs_hello | 151, 261 | `cargo run --example ecs_hello` |
| ET-02 | ecs_100k | 153, 262, 322 | `cargo run --example ecs_100k` |
| ET-03 | ecs_events | 125, 263 | `cargo run --example ecs_events` |
| ET-04 | ecs_hierarchy | 126, 264 | `cargo run --example ecs_hierarchy` |
| ET-05 | ecs_parallel | 127, 265, 323 | `cargo run --example ecs_parallel` |
| ET-06 | ecs_commands | 128, 266 | `cargo run --example ecs_commands` |
| ET-07 | ecs_change_tracking | 129, 267 | `cargo run --example ecs_change_tracking` |
| ET-08 | ecs_resources | 130, 268 | `cargo run --example ecs_resources` |
| ET-09 | ecs_bundle | 131, 269 | `cargo run --example ecs_bundle` |
| ET-10 | ecs_schedule | 132, 270 | `cargo run --example ecs_schedule` |
| ET-11 | ecs_ray_cast | 133, 271 | `cargo run --example ecs_ray_cast` |
| ET-12 | ecs_pong | 134, 272, 370 | `cargo run --example ecs_pong` |

---

### 5. 代码质量检查

| 检查项 | 描述 | 需求ID | 命令 |
|--------|------|--------|------|
| fmt | 代码格式 | 293, 356 | `cargo fmt --check --workspace` |
| clippy | 代码lint | 294, 357 | `cargo clippy --workspace -- -D warnings` |
| doc | 文档生成 | 295, 358 | `cargo doc --workspace --no-deps` |
| doc coverage | Doc comment 覆盖率 | 198, 362 | >= 100% |
| unsafe count | unsafe 块数量 | 199, 363 | <= 3 |

---

### 6. CI/CD 测试

| 测试项 | 描述 | 需求ID | 平台 |
|--------|------|--------|------|
| Linux CI | Linux 平台测试 | 296, 374 | Linux |
| macOS CI | macOS 平台测试 | 296, 374 | macOS |
| Windows CI | Windows 平台测试 | 296, 374 | Windows |

---

### 7. 文档与发布

| 检查项 | 描述 | 需求ID |
|--------|------|--------|
| CHANGELOG | 记录版本 0.5.0 | 297, 375 |
| README | 加入「ECS 系统」章节 | 298, 376 |
| API 数量 | 公开 API <= 100 | 196 |
| Doc 覆盖率 | 公开 API doc comment 100% | 197, 362 |

---

## 测试执行顺序

### 开发阶段

1. **TDD 开发**：先写单元测试，再实现功能
2. **每日构建**：每个 PR 必须通过所有测试

### Sprint 结束前

1. 运行所有单元测试
2. 运行所有集成测试
3. 运行所有示例
4. 运行 Criterion benchmarks
5. 运行代码质量检查
6. 验证 CI 三平台 green

---

## 测试覆盖率目标

| 模块 | 覆盖率目标 |
|------|------------|
| World | >= 90% |
| Entity | >= 95% |
| Component/Storage | >= 85% |
| Query | >= 90% |
| System | >= 85% |
| Commands | >= 90% |
| Events | >= 85% |
| Hierarchy | >= 85% |
| **总计** | **>= 85%** |

---

## 验收标准汇总

| 类别 | 标准 | 需求ID |
|------|------|--------|
| 单元测试 | `cargo test -p engine-ecs` 全部通过 | 292, 354 |
| 集成测试 | 所有集成测试通过 | IT-01 ~ IT-05 |
| 性能测试 | 所有 benchmark 达标 | BT-01 ~ BT-04, 200, 262 |
| 示例测试 | 所有示例正常运行 | ET-01 ~ ET-12 |
| 代码质量 | fmt/clippy/doc 通过 | 293-295, 356-358 |
| CI | 三平台 green | 296, 359, 374 |
| 文档 | CHANGELOG/READM 完整 | 297-298, 375-376 |
| API | <= 100 个公开 API | 196 |
| Doc | 100% doc comment 覆盖 | 197, 362 |
| Unsafe | <= 3 个 unsafe 块 | 199, 363 |

---

## 优先级说明

- **P0（关键）**：必须全部通过，直接影响 Sprint 验收
- **P1（重要）**：对质量有重要影响
- **P2（期望）**：增强测试，可后续迭代