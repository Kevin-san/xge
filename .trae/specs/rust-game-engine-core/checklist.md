# Checklist - 游戏引擎核心架构开发 (Sprint-01)

## 构建系统
- [ ] Cargo workspace 配置正确
- [ ] rust-toolchain.toml 固定工具链
- [ ] rustfmt.toml 格式化规则
- [ ] clippy.toml 检查规则
- [ ] build.rs 构建信息正确

## engine-core 核心
- [x] Engine::new() 正确初始化
- [x] Engine::run() 主循环
- [x] request_quit() 线程安全
- [ ] world() / world_mut() 占位实现
- [ ] spawn_task() 异步任务

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
- [ ] Startup/Update/Render/Shutdown 四阶段
- [ ] add_system_to_stage() 功能

## engine-math 数学库
- [ ] Vec2 基本运算和插值
- [ ] Vec3 完整功能
- [ ] Vec4 完整功能
- [ ] Mat4 矩阵运算
- [ ] Quat 四元数
- [ ] Transform 变换
- [ ] Rect / AABB 几何

## engine-platform 平台抽象
- [ ] Time 时间管理
- [ ] FileSystem trait 和实现
- [ ] ThreadPool 线程池
- [ ] Platform 平台检测
- [ ] Feature 特性开关

## engine-utils 工具库
- [ ] Handle<T> 句柄
- [ ] Arena<T> 对象池
- [ ] ResourceManager<T> 资源管理
- [ ] AssetId 资源标识符

## 示例程序
- [x] hello_engine 编译运行
- [x] minimal_app 编译运行
- [x] module_order 正确顺序
- [x] event_bus_demo 事件收发
- [ ] arena_bench（可选）

## 代码质量
- [x] 无编译错误
- [ ] 单元测试 >= 30 条
- [ ] 文档注释完整
- [ ] API 数量 <= 30
