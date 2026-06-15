# 游戏引擎核心架构开发规范

## Why

基于现有的16阶段技术文档体系，开发一个模块化的Rust游戏引擎核心框架，提供基础的引擎生命周期管理、模块系统、事件总线和时间管理等核心功能。

## What Changes

- 创建 `engine-core`  crate 作为引擎核心
- 实现 `Engine` 主结构和生命周期管理
- 实现 `Module` trait 和 `ModuleRegistry` 模块注册系统
- 实现 `App` trait 和 `AppBuilder` 应用构建器
- 实现 `EventBus` 事件总线系统
- 实现 `Schedule` 任务调度器
- 创建4个示例程序演示核心功能
- 添加日志系统和构建信息

## Impact

- Affected specs: 整个游戏引擎技术文档体系的第一个阶段
- Affected code: 新建 `engine-core` crate

## ADDED Requirements

### Requirement: Engine 核心

系统 SHALL 提供 `Engine` 结构体，具备以下功能：
- 使用 `EngineConfig` 配置创建引擎实例
- `run()` 方法启动主循环
- `request_quit()` 安全退出请求
- `is_running()` 检查运行状态
- 通过 `module<T>()` 和 `module_mut<T>()` 获取模块

#### Scenario: 最小引擎运行
- **WHEN** 用户创建 Engine 并调用 run()
- **THEN** 引擎初始化后进入主循环，用户调用 request_quit() 后安全退出

### Requirement: Module 系统

系统 SHALL 提供 `Module` trait，允许用户定义自定义模块：
- 模块具有唯一名称
- 模块声明依赖关系
- 实现 on_init/on_update/on_render/on_shutdown 生命周期回调
- `ModuleRegistry` 按拓扑排序初始化模块

#### Scenario: 模块依赖初始化
- **WHEN** ModuleB 依赖 ModuleA，用户注册 ModuleB 后再注册 ModuleA
- **THEN** ModuleA 先于 ModuleB 初始化，关闭时逆序执行

### Requirement: App 应用

系统 SHALL 提供 `App` trait 定义用户游戏代码入口：
- setup() 初始化回调
- update() 每帧更新
- render() 渲染钩子
- shutdown() 退出清理
- `AppBuilder` 链式构建应用

### Requirement: 事件总线

系统 SHALL 提供类型安全的事件总线：
- 泛型 `EventBus<T>` 支持任意事件类型
- subscribe() 订阅事件并返回 handle
- unsubscribe() 取消订阅
- send() 派发事件
- drain() 批量消费事件
- 支持 Clone（允许多订阅者）

### Requirement: 调度器

系统 SHALL 提供 `Schedule` 任务调度器：
- add_stage() 注册执行阶段
- run() 按阶段顺序执行

### Requirement: 日志系统

系统 SHALL 提供全局日志功能：
- init() 初始化日志系统
- set_level() 设置日志等级
- BUILD_COMMIT_HASH 和 BUILD_TIMESTAMP 全局常量

## REMOVED Requirements

无

## 技术约束

- Rust edition 2021
- MSRV (Minimum Supported Rust Version): 1.70
- 依赖项最小化，使用标准库优先
- 线程安全：Engine 内部状态使用 Arc<Mutex<>> 保护
- 错误处理：使用 `anyhow::Result`
