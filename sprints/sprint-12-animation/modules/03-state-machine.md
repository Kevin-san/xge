# 状态机模块 (State Machine)

## 模块概述

状态机是动画控制器的核心，负责根据条件切换动画状态。本模块包含状态机定义、过渡条件、参数系统和动画控制器构建器。

## 需求编号与功能描述

### StateMachine 状态机

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 69 | 创建状态机 | `StateMachine::new(name) -> Self` | &str | Self |
| 70 | 添加状态 | `StateMachine::add_state(name, node) -> StateHandle` | &str, StateNode | StateHandle |
| 71 | 设置入口状态 | `StateMachine::set_entry_state(name)` | &str | - |
| 72 | 添加过渡 | `StateMachine::add_transition(from, to, condition)` | &str, &str, Condition | - |
| 73 | 添加任意状态过渡 | `StateMachine::add_any_state_transition(to, condition)` | &str, Condition | - |
| 99 | 状态机数据结构 | `StateMachine` | 状态/过渡/条件/参数 | - |
| 289 | 创建状态机 | `StateMachine::new() -> Self` | - | Self |
| 290 | 添加状态 | `StateMachine::add_state(&mut self, name, node) -> StateHandle` | &str, StateNode | StateHandle |
| 291 | 设置入口状态 | `StateMachine::set_entry_state(&mut self, name)` | &str | - |
| 292 | 添加过渡 | `StateMachine::add_transition(&mut self, from, to, duration, condition)` | &str, &str, f32, Condition | - |
| 293 | 添加任意状态过渡 | `StateMachine::add_any_state_transition(&mut self, to, condition)` | &str, Condition | - |
| 294 | 获取状态数组 | `StateMachine::states(&self) -> &[StateNode]` | - | &[StateNode] |
| 295 | 获取过渡数组 | `StateMachine::transitions(&self) -> &[Transition]` | - | &[Transition] |
| 296 | 获取参数映射 | `StateMachine::parameters(&self) -> &ParameterMap` | - | &ParameterMap |

### StateHandle 状态句柄

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 307 | 状态句柄类型 | `StateHandle = usize` | - | - |

### StateNode 状态节点

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 84 | 状态节点类型 | `StateNode::Clip(clip) / Blend1D(tree) / Blend2D(tree) / BlendTree(tree) / Layered(layered) / StateMachine(nested)` | - | 枚举 |
| 321 | Clip 节点 | `StateNode::Clip(clip_handle)` | Handle<AnimationClip> | - |
| 322 | Blend1D 节点 | `StateNode::Blend1D(tree)` | BlendNode1D | - |
| 323 | Blend2D 节点 | `StateNode::Blend2D(tree)` | BlendNode2D | - |
| 324 | BlendTree 节点 | `StateNode::BlendTree(node)` | BlendTree | - |
| 325 | Layered 节点 | `StateNode::Layered(layered)` | LayeredBlend | - |
| 326 | 嵌套状态机 | `StateNode::StateMachine(nested)` | StateMachine | - |
| 327 | 获取时长 | `StateNode::duration(&self) -> f32` | - | f32 |
| 328 | 是否循环 | `StateNode::is_looping(&self) -> bool` | - | bool |
| 329 | 获取 WrapMode | `StateNode::wrap_mode(&self) -> WrapMode` | - | WrapMode |
| 330 | 获取速度 | `StateNode::speed(&self) -> f32` | - | f32 |
| 331 | 获取时间点事件 | `StateNode::events(&self, time) -> Vec<AnimationEvent>` | f32 | Vec<AnimationEvent> |

### Transition 过渡

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 79 | 获取源状态 | `Transition::from_state(&self) -> &str` | - | &str |
| 80 | 获取目标状态 | `Transition::to_state(&self) -> &str` | - | &str |
| 81 | 获取过渡时长 | `Transition::duration(&self) -> f32` | - | f32 |
| 82 | 获取混合模式 | `Transition::blend_mode(&self) -> BlendMode` | - | BlendMode |
| 311 | 获取源状态 | `Transition::from(&self) -> &str` | - | &str |
| 312 | 获取目标状态 | `Transition::to(&self) -> &str` | - | &str |
| 313 | 获取过渡时长 | `Transition::duration(&self) -> f32` | - | f32 |
| 314 | 获取混合模式 | `Transition::blend_mode(&self) -> BlendMode` | - | BlendMode |
| 315 | 获取退出时间 | `Transition::exit_time(&self) -> f32` | - | f32 |
| 316 | 是否有退出时间 | `Transition::has_exit_time(&self) -> bool` | - | bool |

### Condition 条件

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 74 | 条件类型 | `Condition::Parameter(name, op, value)` | - | - |
| 75 | 复合条件 | `Condition::And(a, b) / Or(a, b) / Not(a)` | - | - |
| 76 | 基础条件 | `Condition::True / False` | - | - |
| 77 | 时间条件 | `Condition::TimeElapsed(seconds)` | - | - |
| 78 | 事件触发条件 | `Condition::EventTriggered(event_name)` | - | - |
| 308 | 条件类型扩展 | `Condition::True / False / Parameter(name, CompareOp, value) / And(a, b) / Or(a, b) / Not(a) / TimeElapsed(seconds) / EventTriggered(name)` | - | - |
| 309 | 比较操作符 | `CompareOp::Equal / NotEqual / Less / LessEqual / Greater / GreaterEqual` | - | 枚举 |

### BlendMode 混合模式

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 83 | 混合模式枚举 | `BlendMode::Linear / Additive / Crossfade` | - | 枚举 |
| 320 | 混合模式枚举 | `BlendMode::Linear / Additive / Crossfade` | - | 枚举 |

### Parameter 参数系统

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 95 | 参数值类型 | `ParameterValue::Bool / Float / Int / Vec2 / Vec3 / Trigger(一次性事件)` | - | 枚举 |
| 310 | 参数值类型扩展 | `ParameterValue::Bool / Float / Int / Vec2 / Vec3 / Trigger` | - | 枚举 |
| 311 | 参数映射获取 | `ParameterMap::get(&self, name) -> Option<ParameterValue>` | &str | Option<ParameterValue> |
| 312 | 参数映射设置 | `ParameterMap::set(&mut self, name, value)` | &str, ParameterValue | - |
| 313 | 参数映射触发 | `ParameterMap::trigger(&mut self, name)` | &str | - |

### AnimationController 动画控制器

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 61 | 动画控制器 | `AnimationController` | 基于状态机切换动画 | - |
| 62 | 创建控制器 | `AnimationController::new(machine) -> Self` | StateMachine | Self |
| 63 | 设置浮点参数 | `AnimationController::set_parameter(name, value)` | &str, f32 | - |
| 64 | 获取参数值 | `AnimationController::parameter(&self, name) -> Option<ParameterValue>` | &str | Option<ParameterValue> |
| 65 | 获取当前状态 | `AnimationController::current_state(&self) -> &str` | - | &str |
| 66 | 更新控制器 | `AnimationController::update(&mut self, dt)` | f32 | - |
| 67 | 获取混合空间 | `AnimationController::blend_space(&self) -> &[f32]` | - | &[f32] |
| 68 | 获取当前 Pose | `AnimationController::pose(&self) -> &Pose` | - | &Pose |
| 289 | 创建控制器 | `AnimationController::new(state_machine) -> Self` | StateMachine | Self |
| 290 | 获取状态机 | `AnimationController::machine(&self) -> &StateMachine` | - | &StateMachine |
| 291 | 设置浮点参数 | `AnimationController::set_parameter_float(&mut self, name, value)` | &str, f32 | - |
| 292 | 设置布尔参数 | `AnimationController::set_parameter_bool(&mut self, name, value)` | &str, bool | - |
| 293 | 设置整型参数 | `AnimationController::set_parameter_int(&mut self, name, value)` | &str, i32 | - |
| 294 | 触发参数 | `AnimationController::trigger(&mut self, name)` | &str | - |
| 295 | 获取当前状态 | `AnimationController::current_state(&self) -> &str` | - | &str |
| 296 | 获取当前时间 | `AnimationController::current_time(&self) -> f32` | - | f32 |
| 297 | 更新控制器 | `AnimationController::update(&mut self, dt)` | f32 | - |
| 298 | 获取 Pose | `AnimationController::pose(&self) -> &Pose` | - | &Pose |

### AnimationControllerBuilder 构建器

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 96 | 构建器 | `AnimationControllerBuilder` | 流畅构造状态机 | - |
| 373 | 创建构建器 | `AnimationControllerBuilder::new() -> Self` | - | Self |
| 374 | 添加状态 | `AnimationControllerBuilder::with_state(name, node) -> Self` | &str, StateNode | Self |
| 375 | 设置入口 | `AnimationControllerBuilder::with_entry(name) -> Self` | &str | Self |
| 376 | 添加过渡 | `AnimationControllerBuilder::with_transition(from, to, duration, condition) -> Self` | &str, &str, f32, Condition | Self |
| 377 | 构建控制器 | `AnimationControllerBuilder::build(&self) -> AnimationController` | - | AnimationController |

### AnimationEventSystem 事件系统

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 98 | 事件系统 | `AnimationEventSystem` | 每帧查询触发事件 | - |
| 383 | 获取事件名称 | `AnimationEventSystem::pop(&self) -> String` | - | String |

## 验收标准

- [ ] StateMachine 正确维护状态和过渡关系
- [ ] 条件判断正确触发状态切换
- [ ] Parameter 系统正确设置/获取参数值
- [ ] 单测 StateMachine transition 通过（需求 477）
- [ ] `examples/animation_state_machine` 可切换 Idle/Walk/Run/Jump 状态（需求 214, 502）

## 依赖关系

- 依赖 `AnimationClip` / `Pose`（01-animation-clip.md）
- 依赖 `StateNode` / `Transition` / `Condition`
- 被 Animator 组件使用

## 优先级

**P0（核心）：**
- StateMachine 基本结构
- 状态切换逻辑
- Condition 条件判断
- Parameter 系统

**P1（重要）：**
- AnimationControllerBuilder 构建器
- 过渡时长和混合
- 任意状态过渡

**P2（增强）：**
- 嵌套状态机
- 状态机 JSON 序列化
- 编辑器可视化