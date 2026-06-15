# Checklist - 游戏引擎核心架构开发

## 项目结构
- [x] Cargo workspace 正确配置
- [x] engine-core crate 结构完整
- [x] build.rs 生成构建信息

## Engine 核心
- [x] EngineConfig 提供合理的默认值
- [x] Engine::new() 正确初始化所有子系统
- [x] Engine::run() 实现主循环
- [x] request_quit() 线程安全
- [x] module<T>() 按类型获取模块
- [x] Time::delta_time() 计算正确

## Module 系统
- [x] Module trait 所有方法正确实现
- [x] ModuleRegistry 按依赖拓扑排序
- [x] initialize_all() 无循环依赖
- [x] shutdown_all() 逆序执行
- [x] 按名称查找模块功能正常

## App 应用
- [x] App trait 所有生命周期方法正确
- [x] AppBuilder 链式调用正常工作
- [x] run() 方法正确启动引擎

## EventBus
- [x] EventBus<T> 泛型支持
- [x] subscribe() 返回有效 handle
- [x] unsubscribe() 正确移除订阅者
- [x] send() 同步派发事件
- [x] drain() 消费所有事件
- [x] 事件类型 Clone 支持

## Schedule 调度器
- [x] add_stage() 正确注册阶段
- [x] run() 按顺序执行

## 日志系统
- [x] init() 正确初始化
- [x] 日志等级过滤正常
- [x] BUILD_COMMIT_HASH 可用
- [x] BUILD_TIMESTAMP 可用

## 示例程序
- [x] hello_engine 编译运行成功
- [x] minimal_app 编译运行成功
- [x] module_order 输出正确初始化顺序
- [x] event_bus_demo 事件收发正常
- [x] 所有示例退出码为 0

## 代码质量
- [x] 无编译警告 (event_bus_demo 有未使用变量警告，可忽略)
- [x] 文档注释完整
- [x] 错误处理完善
