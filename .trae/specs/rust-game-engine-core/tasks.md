# Tasks - 游戏引擎核心架构开发

## 阶段 1: 项目初始化

- [x] Task 1.1: 创建 Cargo workspace 结构
  - [x] 创建 engine-core 核心库 crate
  - [x] 配置 Cargo.toml 依赖和元数据
  - [x] 设置构建信息（build.rs 生成 BUILD_COMMIT_HASH 和 BUILD_TIMESTAMP）

## 阶段 2: 引擎核心实现

- [x] Task 2.1: 实现 EngineConfig 配置结构
  - [x] 窗口配置（标题、尺寸）
  - [x] 日志等级配置
  - [x] 目标帧率配置

- [x] Task 2.2: 实现 Engine 主结构
  - [x] 初始化逻辑
  - [x] run() 主循环
  - [x] request_quit() 退出请求
  - [x] is_running() 状态检查
  - [x] 模块获取方法

- [x] Task 2.3: 实现 Time 时间管理
  - [x] 帧时间计算
  - [x] delta_time (dt) 提供

## 阶段 3: 模块系统

- [x] Task 3.1: 定义 Module trait
  - [x] name() 模块名称
  - [x] dependencies() 依赖声明
  - [x] on_init/on_update/on_render/on_shutdown 生命周期
  - [x] enabled() 启用状态

- [x] Task 3.2: 实现 ModuleRegistry
  - [x] register() 模块注册
  - [x] initialize_all() 拓扑排序初始化
  - [x] update_all() 批量更新
  - [x] shutdown_all() 逆序关闭

## 阶段 4: 应用系统

- [x] Task 4.1: 定义 App trait
  - [x] setup() 初始化
  - [x] update() 每帧更新
  - [x] render() 渲染
  - [x] shutdown() 退出

- [x] Task 4.2: 实现 AppBuilder
  - [x] with_config() 配置
  - [x] add_module() 添加模块
  - [x] run() 启动引擎

## 阶段 5: 事件总线

- [x] Task 5.1: 实现 EventBus<T>
  - [x] subscribe() 订阅
  - [x] unsubscribe() 取消订阅
  - [x] send() 派发事件
  - [x] drain() 批量消费

## 阶段 6: 调度器

- [x] Task 6.1: 实现 Schedule
  - [x] add_stage() 注册阶段
  - [x] run() 执行调度

## 阶段 7: 日志系统

- [x] Task 7.1: 实现 log 模块
  - [x] init() 初始化
  - [x] set_level() 设置等级
  - [x] 日志宏 (info!, warn!, error!)

## 阶段 8: 示例程序

- [x] Task 8.1: 实现 hello_engine 示例
  - [x] 打印版本信息
  - [x] 最小引擎运行

- [x] Task 8.2: 实现 minimal_app 示例
  - [x] 完整 App trait 实现
  - [x] 默认运行模式

- [x] Task 8.3: 实现 module_order 示例
  - [x] 模块依赖演示
  - [x] 初始化顺序验证

- [x] Task 8.4: 实现 event_bus_demo 示例
  - [x] 订阅/派发/取消订阅演示

## 阶段 9: 测试验证

- [x] Task 9.1: 添加单元测试
  - [x] EventBus 功能测试

- [x] Task 9.2: 验证所有示例编译运行

# Task Dependencies
- Task 2.1, 2.2, 2.3 依赖 Task 1.1
- Task 3.1, 3.2 依赖 Task 2.1
- Task 4.1, 4.2 依赖 Task 3.1
- Task 5.1 独立
- Task 6.1 依赖 Task 3.2
- Task 7.1 独立
- Task 8.x 依赖 Task 2.x, 3.x, 4.x, 5.x, 6.x, 7.1
- Task 9.x 依赖所有 Task 8.x
