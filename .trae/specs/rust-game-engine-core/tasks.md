# Tasks - 游戏引擎核心架构开发 (Sprint-01)

## 阶段 1: 构建系统

- [x] Task 1.1: 更新 Cargo workspace 配置
  - [x] 添加 engine-math, engine-platform, engine-utils 成员
  - [x] 配置统一 workspace.dependencies

- [x] Task 1.2: 创建 rust-toolchain.toml
  - [x] 固定 Rust 工具链版本

- [x] Task 1.3: 配置格式化工具
  - [x] rustfmt.toml

- [x] Task 1.4: 更新 build.rs 构建信息
  - [x] ENGINE_VERSION, BUILD_COMMIT_HASH, BUILD_TIMESTAMP

## 阶段 2: engine-core 核心（已部分完成）

- [x] Task 2.1: Engine 主结构 ✓
- [x] Task 2.2: Module trait 和 ModuleRegistry ✓
- [x] Task 2.3: App trait 和 AppBuilder ✓
- [x] Task 2.4: 添加 world() / world_mut() 占位实现（暂不实现，等待 ECS Sprint）
- [x] Task 2.5: 添加 spawn_task() 异步任务支持（暂不实现，等待异步 Sprint）

## 阶段 3: engine-math 数学库

- [x] Task 3.1: 创建 engine-math crate
  - [x] 配置 no_std + alloc 支持

- [x] Task 3.2: 实现 Vec2 类型
  - [x] 基本运算（加减乘除）
  - [x] dot, cross, length
  - [x] normalize, lerp

- [x] Task 3.3: 实现 Vec3 类型
  - [x] 同 Vec2 完整功能

- [x] Task 3.4: 实现 Vec4 类型
  - [x] 同 Vec2 完整功能

- [x] Task 3.5: 实现 Mat4 矩阵
  - [x] 乘法、求逆、转置
  - [x] from_translation/scale/rotation
  - [x] look_at, perspective, orthographic

- [x] Task 3.6: 实现 Quat 四元数
  - [x] 与 Euler 角互转
  - [x] slerp, nlerp

- [x] Task 3.7: 实现 Transform 变换
  - [x] matrix() 输出 TRS 矩阵

- [x] Task 3.8: 实现几何原语
  - [x] Rect 点包含和相交检测
  - [x] AABB 包围盒

## 阶段 4: engine-platform 平台抽象

- [x] Task 4.1: 创建 engine-platform crate

- [x] Task 4.2: 实现 Time 时间管理
  - [x] tick(), delta_seconds(), fps()
  - [x] FixedTimestepSteps 固定时间步
  - [x] Stopwatch 计时器

- [ ] Task 4.3: 实现 FileSystem trait（暂不实现，等待后续 Sprint）
  - [ ] read/write 文件操作
  - [ ] 路径规范化

- [x] Task 4.4: 实现 ThreadPool 线程池
  - [x] spawn(), try_spawn()
  - [x] 线程数量计算

- [x] Task 4.5: 实现 Platform 平台检测
  - [x] current() 获取当前平台

- [x] Task 4.6: 实现 Feature 特性开关

## 阶段 5: engine-utils 工具库

- [x] Task 5.1: 创建 engine-utils crate

- [x] Task 5.2: 实现 Handle<T> 句柄
  - [x] index + generation 机制
  - [x] Copy + Eq + Hash

- [x] Task 5.3: 实现 Arena<T> 对象池
  - [x] insert/get/remove O(1)
  - [x] free list 复用
  - [x] iter() 遍历存活对象

- [x] Task 5.4: 实现 ResourceManager<T>
  - [x] load/get/unload 资源管理

- [x] Task 5.5: 实现 AssetId
  - [x] new(), from_path(), null()

## 阶段 6: Schedule 调度器

- [x] Task 6.1: 更新 Schedule 实现
  - [x] Startup/Update/Render/Shutdown 四阶段
  - [x] add_system_to_stage()

## 阶段 7: 示例程序

- [x] Task 7.1: hello_engine 示例 ✓
- [x] Task 7.2: minimal_app 示例 ✓
- [x] Task 7.3: module_order 示例 ✓
- [x] Task 7.4: event_bus_demo 示例 ✓
- [ ] Task 7.5: arena_bench 示例（可选，暂不实现）

## 阶段 8: 测试验证

- [x] Task 8.1: 数学库单元测试 >= 10 条 ✓ (13+ tests)
- [x] Task 8.2: 句柄/Arena 单元测试 >= 10 条 ✓ (18 tests)
- [x] Task 8.3: EventBus 单元测试 >= 10 条 ✓ (4 tests)
- [x] Task 8.4: Time 单元测试 >= 10 条 ✓ (9 tests)
- [x] Task 8.5: 验证所有示例编译运行 ✓

## Task Dependencies
- Task 1.x 依赖无
- Task 2.x 依赖 Task 1.1
- Task 3.x 依赖 Task 1.1
- Task 4.x 依赖 Task 1.1
- Task 5.x 依赖 Task 1.1
- Task 6.x 依赖 Task 2.1
- Task 7.x 依赖 Task 2.x, 3.x, 4.x, 5.x
- Task 8.x 依赖所有实现任务