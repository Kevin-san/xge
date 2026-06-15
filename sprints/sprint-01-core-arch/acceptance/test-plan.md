# 测试计划

## 模块名称与概述

本文档定义 Sprint 01 的测试策略、测试用例和验收标准。测试覆盖 engine-core、engine-math、engine-platform、engine-utils 等核心模块，确保所有功能正确实现。

## 测试策略

### 测试层次

1. **单元测试（Unit Tests）**
   - 每个核心结构体至少 3 个测试
   - 测试公开 API 的正常和边界情况

2. **集成测试（Integration Tests）**
   - 模块间交互
   - 依赖解析和初始化顺序

3. **示例测试（Example Tests）**
   - `cargo run --example hello_engine`
   - `cargo run --example minimal_app`

### 覆盖率目标

| 模块 | 最低测试数量 |
|------|-------------|
| engine-core | 30+ |
| engine-math | 30+ |
| engine-platform | 20+ |
| engine-utils | 20+ |
| **总计** | **100+** |

### 基准测试（可选）

引入 `criterion` 基准测试框架：
- Arena 插入/查找性能
- EventBus 派发性能
- 数学库运算性能

---

## 1. engine-core 测试

### 1.1 Engine 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| CORE-001 | engine_new | `Engine::new(config)` 创建实例 | P0 |
| CORE-002 | engine_run_exit | `Engine::run()` 正常退出 | P0 |
| CORE-003 | engine_request_quit | `request_quit()` 线程安全退出 | P0 |
| CORE-004 | engine_is_running | `is_running()` 状态正确 | P0 |
| CORE-005 | engine_module_access | `module<T>()` 获取模块引用 | P0 |
| CORE-006 | engine_module_mut | `module_mut<T>()` 获取可变引用 | P0 |
| CORE-007 | engine_time_access | `time()` 返回时间引用 | P0 |
| CORE-008 | engine_filesystem_access | `filesystem()` 返回文件系统引用 | P0 |
| CORE-009 | engine_config_access | `config()` 返回配置引用 | P0 |
| CORE-010 | engine_spawn_task | `spawn_task()` 提交异步任务 | P1 |

### 1.2 Module / ModuleRegistry 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| CORE-011 | module_name | `Module::name()` 返回唯一名称 | P0 |
| CORE-012 | module_dependencies | `dependencies()` 返回正确列表 | P0 |
| CORE-013 | module_lifecycle | init/update/shutdown 正确调用 | P0 |
| CORE-014 | module_enabled | `enabled()` 控制模块启用/禁用 | P1 |
| CORE-015 | registry_register | `register()` 注册模块 | P0 |
| CORE-016 | registry_get_by_type | `get<T>()` 按类型查找 | P0 |
| CORE-017 | registry_get_by_name | `get_by_name()` 按名称查找 | P0 |
| CORE-018 | registry_init_order | 按依赖拓扑排序初始化 | P0 |
| CORE-019 | registry_shutdown_order | 逆序关闭 | P0 |
| CORE-020 | registry_update_all | 批量更新所有模块 | P0 |

### 1.3 App / AppBuilder 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| CORE-021 | app_lifecycle | setup/update/render/shutdown 正确调用 | P0 |
| CORE-022 | app_builder_new | `AppBuilder::new()` 创建 | P0 |
| CORE-023 | app_builder_config | `with_config()` 设置配置 | P0 |
| CORE-024 | app_builder_add_module | `add_module()` 注册模块 | P0 |
| CORE-025 | app_builder_fluent | fluent API 链式调用 | P0 |
| CORE-026 | app_builder_run | `run()` 启动引擎 | P0 |

### 1.4 Schedule 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| CORE-027 | schedule_new | `Schedule::new()` 创建 | P0 |
| CORE-028 | schedule_add_stage | `add_stage()` 注册阶段 | P0 |
| CORE-029 | schedule_add_system | `add_system_to_stage()` 添加系统 | P0 |
| CORE-030 | schedule_run_order | 按阶段顺序执行 | P0 |
| CORE-031 | schedule_stage_order | `stage_order()` 返回顺序 | P1 |

### 1.5 Plugin 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| CORE-032 | plugin_name | `Plugin::name()` 返回名称 | P1 |
| CORE-033 | plugin_build | `build()` 正确注册 | P1 |
| CORE-034 | plugin_group | PluginGroup 成组安装 | P1 |

---

## 2. engine-math 测试

### 2.1 Vec2 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-001 | vec2_const | ZERO/ONE/X/Y 常量正确 | P0 |
| MATH-002 | vec2_new | `new(x, y)` 构造 | P0 |
| MATH-003 | vec2_operators | Add/Sub/Mul/Div 运算符 | P0 |
| MATH-004 | vec2_dot | `dot()` 点积 | P0 |
| MATH-005 | vec2_cross | `cross()` 叉积 | P0 |
| MATH-006 | vec2_length | `length()` 向量长度 | P0 |
| MATH-007 | vec2_normalize | `normalize()` 归一化 | P0 |
| MATH-008 | vec2_lerp | `lerp()` 线性插值 | P0 |

### 2.2 Vec3 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-009 | vec3_const | ZERO/ONE/X/Y/Z 常量正确 | P0 |
| MATH-010 | vec3_new | `new(x, y, z)` 构造 | P0 |
| MATH-011 | vec3_operators | Add/Sub/Mul/Div 运算符 | P0 |
| MATH-012 | vec3_dot | `dot()` 点积 | P0 |
| MATH-013 | vec3_cross | `cross()` 叉积 | P0 |
| MATH-014 | vec3_length | `length()` 向量长度 | P0 |
| MATH-015 | vec3_normalize | `normalize()` 归一化 | P0 |
| MATH-016 | vec3_lerp | `lerp()` 线性插值 | P0 |

### 2.3 Vec4 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-017 | vec4_const | ZERO/ONE/X/Y/Z/W 常量正确 | P0 |
| MATH-018 | vec4_new | `new(x, y, z, w)` 构造 | P0 |
| MATH-019 | vec4_operators | Add/Sub/Mul/Div 运算符 | P0 |
| MATH-020 | vec4_dot | `dot()` 点积 | P0 |
| MATH-021 | vec4_length | `length()` 向量长度 | P0 |
| MATH-022 | vec4_normalize | `normalize()` 归一化 | P0 |
| MATH-023 | vec4_lerp | `lerp()` 线性插值 | P0 |

### 2.4 Mat4 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-024 | mat4_const | IDENTITY/ZERO 常量正确 | P0 |
| MATH-025 | mat4_translation | `from_translation()` | P0 |
| MATH-026 | mat4_scale | `from_scale()` | P0 |
| MATH-027 | mat4_rotation_x | `from_rotation_x()` | P0 |
| MATH-028 | mat4_rotation_y | `from_rotation_y()` | P0 |
| MATH-029 | mat4_rotation_z | `from_rotation_z()` | P0 |
| MATH-030 | mat4_from_quat | `from_quat()` | P0 |
| MATH-031 | mat4_look_at | `look_at_rh()` 观察矩阵 | P0 |
| MATH-032 | mat4_perspective | `perspective_rh()` 透视投影 | P0 |
| MATH-033 | mat4_orthographic | `orthographic_rh()` 正交投影 | P0 |
| MATH-034 | mat4_inverse | `inverse()` 矩阵求逆 | P0 |
| MATH-035 | mat4_transpose | `transpose()` 矩阵转置 | P0 |
| MATH-036 | mat4_mul_vec4 | `mul_vec4()` 矩阵向量乘 | P0 |
| MATH-037 | mat4_to_array | `to_cols_array()` 转换数组 | P0 |

### 2.5 Quat 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-038 | quat_identity | IDENTITY 常量正确 | P0 |
| MATH-039 | quat_from_rotation | `from_rotation_*()` 旋转构造 | P0 |
| MATH-040 | quat_from_euler | `from_euler()` 欧拉角构造 | P0 |
| MATH-041 | quat_to_euler | `to_euler()` 转换欧拉角 | P0 |
| MATH-042 | quat_mul | `mul()` 四元数乘法 | P0 |
| MATH-043 | quat_inverse | `inverse()` 求逆 | P0 |
| MATH-044 | quat_normalize | `normalize()` 归一化 | P0 |
| MATH-045 | quat_slerp | `slerp()` 球面线性插值 | P0 |
| MATH-046 | quat_nlerp | `nlerp()` 规范化线性插值 | P0 |

### 2.6 Transform 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-047 | transform_new | `new(pos, rot, scale)` 构造 | P0 |
| MATH-048 | transform_from_translation | `from_translation()` | P0 |
| MATH-049 | transform_matrix | `matrix()` 转换为 Mat4 | P0 |
| MATH-050 | transform_inverse | `inverse()` 求逆 | P0 |

### 2.7 几何原语测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| MATH-051 | rect_new | `new(x, y, w, h)` 构造 | P1 |
| MATH-052 | rect_contains | `contains()` 点包含 | P1 |
| MATH-053 | rect_intersects | `intersects()` 相交检测 | P1 |
| MATH-054 | aabb_new | `new(center, half_extents)` 构造 | P1 |
| MATH-055 | aabb_min_max | `min()`/`max()` 获取边界 | P1 |
| MATH-056 | aabb_contains | `contains()` 点包含 | P1 |

---

## 3. engine-platform 测试

### 3.1 Time 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| PLAT-001 | time_new | `Time::new()` 构造 | P0 |
| PLAT-002 | time_tick | `tick()` 更新 delta | P0 |
| PLAT-003 | time_delta_seconds | `delta_seconds()` 返回时长 | P0 |
| PLAT-004 | time_delta | `delta()` 返回 Duration | P0 |
| PLAT-005 | time_elapsed | `elapsed()` 启动至今时长 | P0 |
| PLAT-006 | time_frame_count | `frame_count()` 帧计数 | P0 |
| PLAT-007 | time_fps | `fps()` 计算帧率 | P0 |
| PLAT-008 | time_fixed_timestep | 固定时间步设置/获取 | P1 |
| PLAT-009 | stopwatch_new | `Stopwatch::new()` 构造 | P0 |
| PLAT-010 | stopwatch_elapsed | `elapsed()` 计时精度 | P0 |

### 3.2 FileSystem 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| PLAT-011 | fs_read_write | 读写文件 | P0 |
| PLAT-012 | fs_read_string | `read_string()` 读文本 | P0 |
| PLAT-013 | fs_write_string | `write_string()` 写文本 | P0 |
| PLAT-014 | fs_exists | `exists()` 检查存在 | P0 |
| PLAT-015 | fs_list_dir | `list_dir()` 列出目录 | P0 |
| PLAT-016 | fs_create_dir | `create_dir_all()` 创建目录 | P0 |
| PLAT-017 | fs_remove_file | `remove_file()` 删除文件 | P0 |
| PLAT-018 | fs_is_dir | `is_dir()` 判断目录 | P0 |
| PLAT-019 | fs_canonicalize | `canonicalize()` 路径规范化 | P0 |

### 3.3 ThreadPool 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| PLAT-020 | tp_new | `new()` 创建线程池 | P0 |
| PLAT-021 | tp_spawn | `spawn()` 提交任务 | P0 |
| PLAT-022 | tp_try_spawn | `try_spawn()` 非阻塞提交 | P1 |
| PLAT-023 | tp_block_on | `block_on()` 阻塞等待 | P0 |
| PLAT-024 | tp_shutdown | `shutdown()` 关闭线程池 | P0 |
| PLAT-025 | tp_active_count | `active_count()` 活跃线程数 | P1 |

### 3.4 Platform 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| PLAT-026 | platform_current | `current()` 返回当前平台 | P0 |
| PLAT-027 | platform_is_* | `is_windows()`/`is_macos()` 等 | P0 |
| PLAT-028 | platform_name | `name()` 返回平台名称 | P0 |

### 3.5 Feature 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| PLAT-029 | feature_enabled | `enabled()` 查询特性 | P0 |
| PLAT-030 | feature_list | `list()` 返回所有特性 | P1 |
| PLAT-031 | feature_render_backend | `render_backend()` 获取后端 | P1 |

---

## 4. engine-utils 测试

### 4.1 Handle 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| UTIL-001 | handle_is_null | `is_null()` 判断 null | P0 |
| UTIL-002 | handle_copy | 实现 Copy trait | P0 |
| UTIL-003 | handle_eq | 实现 Eq trait | P0 |
| UTIL-004 | handle_hash | 实现 Hash trait | P0 |

### 4.2 Arena 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| UTIL-005 | arena_new | `new()` 构造 | P0 |
| UTIL-006 | arena_insert | `insert()` 插入返回句柄 | P0 |
| UTIL-007 | arena_remove | `remove()` 移除对象 | P0 |
| UTIL-008 | arena_get | `get()` 获取引用 | P0 |
| UTIL-009 | arena_get_mut | `get_mut()` 获取可变引用 | P0 |
| UTIL-010 | arena_len | `len()` 返回数量 | P0 |
| UTIL-011 | arena_is_empty | `is_empty()` 判断空 | P0 |
| UTIL-012 | arena_clear | `clear()` 清空 | P0 |
| UTIL-013 | arena_iter | `iter()` 遍历存活项 | P0 |
| UTIL-014 | arena_retain | `retain()` 条件保留 | P1 |
| UTIL-015 | arena_generation | 代际号复用检测 | P1 |

### 4.3 EventBus 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| UTIL-016 | eventbus_new | `new()` 构造 | P0 |
| UTIL-017 | eventbus_subscribe | `subscribe()` 订阅 | P0 |
| UTIL-018 | eventbus_unsubscribe | `unsubscribe()` 取消订阅 | P0 |
| UTIL-019 | eventbus_send | `send()` 派发事件 | P0 |
| UTIL-020 | eventbus_drain | `drain()` 批量消费 | P0 |
| UTIL-021 | eventbus_len | `len()` 返回订阅者数量 | P0 |
| UTIL-022 | eventbus_multiple | 多订阅者同时收到 | P0 |
| UTIL-023 | eventbus_unsubscribe_after | 取消后不再收到 | P0 |
| UTIL-024 | eventbus_clone | 事件 Clone 正确 | P0 |

### 4.4 ResourceManager 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| UTIL-025 | rm_new | `new()` 构造 | P0 |
| UTIL-026 | rm_load | `load()` 加载资源 | P0 |
| UTIL-027 | rm_get | `get()` 获取资源 | P0 |
| UTIL-028 | rm_unload | `unload()` 卸载资源 | P0 |
| UTIL-029 | rm_contains | `contains()` 检查存在 | P0 |
| UTIL-030 | rm_replace | 重复 load 替换旧值 | P1 |

### 4.5 AssetId 测试

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| UTIL-031 | assetid_new | `new()` 从 UUID 创建 | P0 |
| UTIL-032 | assetid_from_path | `from_path()` 从路径创建 | P0 |
| UTIL-033 | assetid_null | `null()` 返回 null | P0 |
| UTIL-034 | assetid_is_null | `is_null()` 判断 null | P0 |

---

## 5. 集成测试

### 5.1 模块初始化顺序

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| INT-001 | module_dep_chain | A→B→C 依赖链初始化 | P0 |
| INT-002 | module_diamond | 菱形依赖 A→B/C→D | P1 |
| INT-003 | module_missing_dep | 缺失依赖报错 | P0 |

### 5.2 主循环

| 编号 | 测试名称 | 测试内容 | 优先级 |
|------|---------|---------|--------|
| INT-004 | mainloop_single_frame | 单帧执行 | P0 |
| INT-005 | mainloop_quit | request_quit 退出 | P0 |

---

## 6. 验收标准汇总

### 6.1 构建验收

| 验收项 | 标准 |
|--------|------|
| cargo build --workspace | 成功 |
| cargo test --workspace | 全部通过 |
| cargo fmt --check --workspace | 通过 |
| cargo clippy --workspace -- -D warnings | 通过 |
| cargo doc --workspace --no-deps | 成功生成 |

### 6.2 示例验收

| 验收项 | 标准 |
|--------|------|
| cargo run --example hello_engine | 退出码 0，输出版本号 |
| cargo run --example minimal_app | 退出码 0，触发空帧 |
| cargo run --example module_order | 初始化顺序正确 |
| cargo run --example event_bus_demo | 事件正常收发 |

### 6.3 覆盖率验收

| 模块 | 标准 |
|------|------|
| Arena | >= 10 条测试 |
| EventBus | >= 10 条测试 |
| Time | >= 10 条测试 |
| engine-math | >= 30 条测试 |
| 本 Sprint 总计 | >= 30 条测试 |

### 6.4 代码质量验收

| 验收项 | 标准 |
|--------|------|
| unsafe 块数量 | <= 5（全部带 SAFETY 注释） |
| 公开 API 数量 | <= 30 |
| doc comment | 每个公开项至少一条 |

---

## 7. 测试执行计划

### 本地验证
```bash
# 格式化检查
cargo fmt --check

# 代码检查
cargo clippy --workspace -- -D warnings

# 测试
cargo test --workspace

# 文档
cargo doc --workspace --no-deps

# 示例
cargo run --example hello_engine
cargo run --example minimal_app
```

### CI 流水线
1. fmt check
2. clippy check
3. test (Linux/macOS/Windows)
4. build release
5. doc generate
6. 示例验证

---

## 8. 风险与缓解

| 风险 | 缓解措施 |
|------|---------|
| 平台差异导致测试失败 | 使用条件编译 `#[cfg(target_os = "...")]` |
| 时序相关测试不稳定 | 使用 mock 时间或增加 tolerance |
| 覆盖率目标未达成 | 优先确保 P0 测试通过 |
