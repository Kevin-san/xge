# Checklist - 游戏引擎核心架构开发 (Sprint-01)

## 构建系统
- [x] Cargo workspace 配置正确
- [x] rust-toolchain.toml 固定工具链
- [x] rustfmt.toml 格式化规则
- [x] build.rs 构建信息正确

## engine-core 核心
- [x] Engine::new() 正确初始化
- [x] Engine::run() 主循环
- [x] request_quit() 线程安全
- [x] world() / world_mut() 占位实现（暂不实现）
- [x] spawn_task() 异步任务（暂不实现）

## Module 系统
- [x] Module trait 所有方法正确
- [x] ModuleRegistry 拓扑排序
- [x] initialize_all() / shutdown_all()
- [x] 按名称查找模块

## App 应用
- [x] App trait 生命周期正确
- [x] AppBuilder 链式调用

## EventBus
- [x] subscribe() / unsubscribe()
- [x] send() 线程安全
- [x] drain() 批量消费

## Schedule 调度器
- [x] Startup/Update/Render/Shutdown 四阶段
- [x] add_system_to_stage() 功能

## engine-math 数学库
- [x] Vec2 基本运算和插值
- [x] Vec3 完整功能
- [x] Vec4 完整功能
- [x] Mat4 矩阵运算
- [x] Quat 四元数
- [x] Transform 变换
- [x] Rect / AABB 几何

## engine-platform 平台抽象
- [x] Time 时间管理
- [ ] FileSystem trait 和实现（暂不实现）
- [x] ThreadPool 线程池
- [x] Platform 平台检测
- [x] Feature 特性开关

## engine-utils 工具库
- [x] Handle<T> 句柄
- [x] Arena<T> 对象池
- [x] ResourceManager<T> 资源管理
- [x] AssetId 资源标识符

## 示例程序
- [x] hello_engine 编译运行
- [x] minimal_app 编译运行
- [x] module_order 正确顺序
- [x] event_bus_demo 事件收发
- [ ] arena_bench（可选，暂不实现）

## 代码质量
- [x] 无编译错误
- [x] 单元测试 >= 30 条 (总计 50+ tests)
- [x] 文档注释完整
- [x] API 数量 <= 30