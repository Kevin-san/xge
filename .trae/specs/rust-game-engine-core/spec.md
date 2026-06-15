# 游戏引擎核心架构开发规范 (Sprint-01)

## Why

基于 `sprint-01-core-arch` 技术文档，开发游戏引擎核心框架的第一阶段，涵盖引擎核心、数学库、平台抽象、调度器和构建系统。

## What Changes

### 1. engine-core crate
- Engine 主结构和生命周期管理
- Module trait 和 ModuleRegistry 模块注册系统
- App trait 和 AppBuilder 应用构建器
- EventBus 事件总线系统
- Schedule 任务调度器

### 2. engine-math crate
- Vec2/Vec3/Vec4 向量类型
- Mat4 矩阵运算
- Quat 四元数
- Transform 变换
- Rect/AABB 几何原语

### 3. engine-platform crate
- Time 时间管理
- FileSystem 文件系统抽象
- ThreadPool 线程池
- Platform 平台检测
- Feature 特性开关

### 4. engine-utils crate
- Handle<T> 类型安全句柄
- Arena<T> 对象池
- ResourceManager<T> 资源管理
- AssetId 资源标识符

### 5. 构建系统
- Cargo workspace 配置
- rust-toolchain.toml
- CI 配置

## Impact
- Affected specs: 整个游戏引擎技术文档体系的基础
- Affected code: 新建多个 crate

## ADDED Requirements

### Requirement: Engine 核心
系统 SHALL 提供 `Engine` 结构体，具备以下功能：
- `Engine::new(config)` 创建引擎实例
- `Engine::run()` 启动主循环
- `Engine::request_quit()` 安全退出请求
- `Engine::module<T>()` 获取模块引用
- `Engine::world()` / `world_mut()` 返回 ECS World 引用（本 Sprint 返回占位）

### Requirement: Module 系统
系统 SHALL 提供 `Module` trait，允许用户定义自定义模块：
- `name()` 返回唯一名称
- `dependencies()` 返回依赖列表
- `on_init(engine)` 初始化回调
- `on_update(engine, dt)` 每帧更新
- `on_render(engine)` 渲染回调
- `on_shutdown(engine)` 关闭回调

### Requirement: App 应用
系统 SHALL 提供 `App` trait 定义用户游戏代码入口：
- `setup(engine)` 初始化回调
- `update(engine, dt)` 每帧更新
- `render(engine)` 渲染钩子
- `shutdown(engine)` 退出清理

### Requirement: 数学库
系统 SHALL 提供完整的数学库：
- Vec2/Vec3/Vec4 向量运算
- Mat4 矩阵运算（乘法、求逆、变换）
- Quat 四元数（与 Euler 角互转）
- Transform 变换（TRS 矩阵）
- Rect/AABB 几何检测

### Requirement: 平台抽象
系统 SHALL 提供平台抽象层：
- Time 时间管理（delta_time, fps, fixed_timestep）
- FileSystem trait 和实现
- ThreadPool 任务调度
- Platform 平台检测

### Requirement: 句柄与资源
系统 SHALL 提供类型安全的句柄和资源管理：
- Handle<T> 代际号句柄
- Arena<T> 对象池（O(1) 增删改查）
- ResourceManager<T> 资源管理
- AssetId 资源标识符

## 技术约束
- Rust edition 2021
- MSRV: 1.70
- 本 Sprint 公开 API <= 30 个
- 本 Sprint 单元测试 >= 30 条
- 支持 `no_std + alloc`（数学库）
