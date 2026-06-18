# 混合树模块 (Blend Tree)

## 模块概述

混合树模块负责实现基于参数的动画混合，支持1D线性混合、2D双线性混合、层级混合和加性动画。本模块是状态机的重要组成部分，用于实现流畅的动画过渡和状态融合。

## 需求编号与功能描述

### BlendNode1D 一维混合节点

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 85 | 创建一维混合节点 | `BlendNode1D::new(param, nodes)` | &str, Vec<(f32, Handle<AnimationClip>)> | Self |
| 86 | 添加混合节点 | `BlendNode1D::push(&mut self, node, value)` | Handle<AnimationClip>, f32 | - |
| 87 | 获取参数名 | `BlendNode1D::parameter(&self) -> &str` | - | &str |
| 91 | 线性插值 | `BlendSpace1D::linear(a, b, alpha)` | Pose, Pose, f32 | Pose |
| 332 | 创建混合节点 | `BlendNode1D::new(param_name) -> Self` | &str | Self |
| 333 | 带节点创建 | `BlendNode1D::with_nodes(nodes: Vec<(f32, Handle<AnimationClip>)>) -> Self` | Vec<(f32, Handle<AnimationClip>)> | Self |
| 334 | 添加节点 | `BlendNode1D::push(&mut self, value, clip)` | f32, Handle<AnimationClip> | - |
| 335 | 获取参数名 | `BlendNode1D::parameter(&self) -> &str` | - | &str |
| 336 | 插值计算 | `BlendNode1D::interpolate(&self, param_value, clips) -> Pose` | f32, &[Handle<AnimationClip>] | Pose |

### BlendNode2D 二维混合节点

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 88 | 创建二维混合节点 | `BlendNode2D::new(x_param, y_param, nodes)` | &str, &str, Vec<((f32, f32), Handle<AnimationClip>)> | Self |
| 89 | 添加混合节点 | `BlendNode2D::push(&mut self, (x, y, clip))` | f32, f32, Handle<AnimationClip> | - |
| 90 | 插值计算 | `BlendNode2D::interpolate(&self, x, y) -> Pose` | f32, f32 | Pose |
| 92 | 双线性插值 | `BlendSpace2D::bilinear` | - | - |
| 337 | 创建二维混合节点 | `BlendNode2D::new(x_param, y_param) -> Self` | &str, &str | Self |
| 338 | 添加节点 | `BlendNode2D::push(&mut self, (x, y), clip)` | (f32, f32), Handle<AnimationClip> | - |
| 339 | 双线性插值 | `BlendNode2D::interpolate(&self, x, y) -> Pose` | f32, f32 | Pose |

### BlendTree 混合树

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 340 | 混合树结构 | `BlendTree` | 复合节点（自定义树状） | - |
| 341 | 创建混合树 | `BlendTree::new(root) -> Self` | BlendTreeNode | Self |
| 342 | 获取根节点 | `BlendTree::node(&self) -> &BlendTreeNode` | - | &BlendTreeNode |
| 343 | 叶子节点 | `BlendTreeNode::Leaf(clip)` | Handle<AnimationClip> | - |
| 344 | 混合节点类型 | `BlendTreeNode::Blend1D(n, nodes) / Blend2D(n, nodes) / Additive(a, b, alpha) / Layered(layers) / Masked(layer, mask)` | - | - |

### LayeredBlend 层级混合

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 93 | 层级混合结构 | `LayeredBlend::base_layer + additive_layer` | - | - |
| 345 | 创建层级混合 | `LayeredBlend::new(base, layers) -> Self` | StateNode, Vec<Layer> | Self |
| 346 | 获取基础层 | `LayeredBlend::base(&self) -> &StateNode` | - | &StateNode |
| 347 | 获取层数组 | `LayeredBlend::layers(&self) -> &[Layer]` | - | &[Layer] |

### Layer 层

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 348 | 创建层 | `Layer::new(node, weight, mask)` | StateNode, f32, Option<AnimationMask> | Self |
| 349 | 获取节点 | `Layer::node(&self) -> &StateNode` | - | &StateNode |
| 350 | 获取权重 | `Layer::weight(&self) -> f32` | - | f32 |
| 351 | 获取遮罩 | `Layer::mask(&self) -> Option<&AnimationMask>` | - | Option<&AnimationMask> |
| 352 | 设置权重 | `Layer::set_weight(&mut self, w)` | f32 | - |

### AnimationMask 动画遮罩

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 129 | 遮罩结构 | `AnimationMask` | 布尔数组，标记骨骼是否受影响 | - |
| 130 | 创建遮罩 | `AnimationMask::new(num_bones) -> Self` | usize | Self |
| 131 | 设置骨骼遮罩 | `AnimationMask::set(&mut self, idx, b)` | usize, bool | - |
| 132 | 获取骨骼遮罩 | `AnimationMask::get(&self, idx) -> bool` | usize | bool |
| 133 | 按骨骼名称创建 | `AnimationMask::with_bone_name(skeleton, name) -> Self` | &Skeleton, &str | Self |
| 134 | 并集运算 | `AnimationMask::union(&self, other) -> Self` | &AnimationMask | Self |
| 135 | 交集运算 | `AnimationMask::intersection(&self, other) -> Self` | &AnimationMask | Self |
| 353 | 创建遮罩 | `AnimationMask::new(num_bones) -> Self` | usize | Self |
| 354 | 设置遮罩 | `AnimationMask::set(&mut self, idx, b)` | usize, bool | - |
| 355 | 获取遮罩 | `AnimationMask::get(&self, idx) -> bool` | usize | bool |
| 356 | 反转遮罩 | `AnimationMask::invert(&mut self)` | - | - |
| 357 | 并集 | `AnimationMask::union(&self, other) -> Self` | &AnimationMask | Self |
| 358 | 交集 | `AnimationMask::intersection(&self, other) -> Self` | &AnimationMask | Self |
| 359 | 按骨骼名称设置 | `AnimationMask::with_bone_name(skeleton, name, b) -> Self` | &Skeleton, &str, bool | Self |

### AdditiveClip 加性动画

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 94 | 创建加性动画 | `AdditiveClip::new(base, additive, alpha)` | Handle<AnimationClip>, Handle<AnimationClip>, f32 | Self |

### PlayBack 播放队列

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 136 | 播放队列系统 | `PlayBack` | delay/queue/jump_to/crossfade | - |
| 137 | 加入队列 | `PlayBack::queue(clip)` | Handle<AnimationClip> | - |
| 138 | 交叉淡入 | `PlayBack::crossfade(clip, duration)` | Handle<AnimationClip>, f32 | - |
| 139 | 跳转到时间 | `PlayBack::jump_to(clip, time)` | Handle<AnimationClip>, f32 | - |
| 140 | 设置时间 | `PlayBack::set_time(time)` | f32 | - |
| 141 | 更新 | `PlayBack::update(&mut self, dt)` | f32 | - |
| 360 | 创建播放队列 | `PlayBack::new() -> Self` | - | Self |
| 361 | 播放 | `PlayBack::play(&mut self, clip)` | Handle<AnimationClip> | - |
| 362 | 停止 | `PlayBack::stop(&mut self)` | - | - |
| 363 | 加入队列 | `PlayBack::queue(&mut self, clip)` | Handle<AnimationClip> | - |
| 364 | 交叉淡入 | `PlayBack::crossfade(&mut self, clip, duration)` | Handle<AnimationClip>, f32 | - |
| 365 | 跳转 | `PlayBack::jump_to(&mut self, clip, time)` | Handle<AnimationClip>, f32 | - |
| 366 | 设置时间 | `PlayBack::set_time(&mut self, time)` | f32 | - |
| 367 | 设置速度 | `PlayBack::set_speed(&mut self, speed)` | f32 | - |
| 368 | 是否播放中 | `PlayBack::is_playing(&self) -> bool` | - | bool |
| 369 | 当前时间 | `PlayBack::time(&self) -> f32` | - | f32 |
| 370 | 更新 | `PlayBack::update(&mut self, dt)` | f32 | - |
| 371 | 获取 Pose | `PlayBack::pose(&self) -> &Pose` | - | &Pose |
| 372 | 获取事件 | `PlayBack::events(&self) -> &[AnimationEvent]` | - | &[AnimationEvent] |

## 验收标准

- [ ] `Blend1D` 在边界值正确插值（需求 166）
- [ ] 单测 `Blend1D` 通过（需求 478）
- [ ] 单测 `Blend2D` 通过（需求 479）
- [ ] 单测 `PlayBack::crossfade` 通过（需求 485）
- [ ] 单测 `AnimationMask` 通过（需求 486）
- [ ] `examples/animation_blend_1d` 线性平滑过渡（需求 466）
- [ ] `examples/animation_blend_2d` 2D 混合正确（需求 467）
- [ ] `examples/animation_additive` 叠加动画正确（需求 468）

## 依赖关系

- 依赖 `AnimationClip` / `Pose`（01-animation-clip.md）
- 依赖 `StateNode`（03-state-machine.md）
- 依赖 `Skeleton`（02-skeleton.md）
- 被 `AnimationController` 使用

## 优先级

**P0（核心）：**
- BlendNode1D 线性混合
- BlendNode2D 双线性混合
- PlayBack 基础播放控制

**P1（重要）：**
- LayeredBlend 层级混合
- AnimationMask 遮罩系统
- PlayBack::crossfade 交叉淡入

**P2（增强）：**
- BlendTree 复杂树结构
- 自定义混合曲线
- 遮罩组合运算