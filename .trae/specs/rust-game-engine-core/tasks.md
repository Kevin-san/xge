# Tasks - 游戏引擎核心架构开发 (Sprint-01)

## 阶段 1: 构建系统

- [ ] Task 1.1: 更新 Cargo workspace 配置
  - [ ] 添加 engine-math, engine-platform, engine-utils 成员
  - [ ] 配置统一 workspace.dependencies

- [ ] Task 1.2: 创建 rust-toolchain.toml
  - [ ] 固定 Rust 工具链版本

- [ ] Task 1.3: 配置格式化工具
  - [ ] rustfmt.toml
  - [ ] clippy.toml

- [ ] Task 1.4: 更新 build.rs 构建信息
  - [ ] ENGINE_VERSION, BUILD_COMMIT_HASH, BUILD_TIMESTAMP

## 阶段 2: engine-core 核心（已部分完成）

- [x] Task 2.1: Engine 主结构 ✓
- [x] Task 2.2: Module trait 和 ModuleRegistry ✓
- [x] Task 2.3: App trait 和 AppBuilder ✓
- [ ] Task 2.4: 添加 world() / world_mut() 占位实现
- [ ] Task 2.5: 添加 spawn_task() 异步任务支持

## 阶段 3: engine-math 数学库

- [ ] Task 3.1: 创建 engine-math crate
  - [ ] 配置 no_std + alloc 支持

- [ ] Task 3.2: 实现 Vec2 类型
  - [ ] 基本运算（加减乘除）
  - [ ] dot, cross, length
  - [ ] normalize, lerp

- [ ] Task 3.3: 实现 Vec3 类型
  - [ ] 同 Vec2 完整功能

- [ ] Task 3.4: 实现 Vec4 类型
  - [ ] 同 Vec2 完整功能

- [ ] Task 3.5: 实现 Mat4 矩阵
  - [ ] 乘法、求逆、转置
  - [ ] from_translation/scale/rotation
  - [ ] look_at, perspective, orthographic

- [ ] Task 3.6: 实现 Quat 四元数
  - [ ] 与 Euler 角互转
  - [ ] slerp, nlerp

- [ ] Task 3.7: 实现 Transform 变换
  - [ ] matrix() 输出 TRS 矩阵

- [ ] Task 3.8: 实现几何原语
  - [ ] Rect 点包含和相交检测
  - [ ] AABB 包围盒

## 阶段 4: engine-platform 平台抽象

- [ ] Task 4.1: 创建 engine-platform crate

- [ ] Task 4.2: 实现 Time 时间管理
  - [ ] tick(), delta_seconds(), fps()
  - [ ] FixedTimestepSteps 固定时间步
  - [ ] Stopwatch 计时器

- [ ] Task 4.3: 实现 FileSystem trait
  - [ ] read/write 文件操作
  - [ ] 路径规范化

- [ ] Task 4.4: 实现 ThreadPool 线程池
  - [ ] spawn(), try_spawn()
  - [ ] 线程数量计算

- [ ] Task 4.5: 实现 Platform 平台检测
  - [ ] current() 获取当前平台

- [ ] Task 4.6: 实现 Feature 特性开关

## 阶段 5: engine-utils 工具库

- [ ] Task 5.1: 创建 engine-utils crate

- [ ] Task 5.2: 实现 Handle<T> 句柄
  - [ ] index + generation 机制
  - [ ] Copy + Eq + Hash

- [ ] Task 5.3: 实现 Arena<T> 对象池
  - [ ] insert/get/remove O(1)
  - [ ] free list 复用
  - [ ] iter() 遍历存活对象

- [ ] Task 5.4: 实现 ResourceManager<T>
  - [ ] load/get/unload 资源管理

- [ ] Task 5.5: 实现 AssetId
  - [ ] new(), from_path(), null()

## 阶段 6: Schedule 调度器

- [ ] Task 6.1: 更新 Schedule 实现
  - [ ] Startup/Update/Render/Shutdown 四阶段
  - [ ] add_system_to_stage()

## 阶段 7: 示例程序

- [x] Task 7.1: hello_engine 示例 ✓
- [x] Task 7.2: minimal_app 示例 ✓
- [x] Task 7.3: module_order 示例 ✓
- [x] Task 7.4: event_bus_demo 示例 ✓
- [ ] Task 7.5: arena_bench 示例（可选）

## 阶段 8: 测试验证

- [ ] Task 8.1: 数学库单元测试 >= 10 条
- [ ] Task 8.2: 句柄/Arena 单元测试 >= 10 条
- [ ] Task 8.3: EventBus 单元测试 >= 10 条
- [ ] Task 8.4: Time 单元测试 >= 10 条
- [ ] Task 8.5: 验证所有示例编译运行

## Task Dependencies
- Task 1.x 依赖无
- Task 2.x 依赖 Task 1.1
- Task 3.x 依赖 Task 1.1
- Task 4.x 依赖 Task 1.1
- Task 5.x 依赖 Task 1.1
- Task 6.x 依赖 Task 2.1
- Task 7.x 依赖 Task 2.x, 3.x, 4.x, 5.x
- Task 8.x 依赖所有实现任务
