# 动画 API 参考清单

## 概述

本文档列出 `engine-animation` crate 提供的所有公开 API，包括核心类型、方法和枚举。

---

## 核心类型

### Keyframe<T>

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Keyframe<T>::new(time, value)` | 创建关键帧 |
| `with_interpolation` | `Keyframe<T>::with_interpolation(interp)` | 设置插值模式 |
| `time` | `Keyframe<T>::time(&self) -> f32` | 获取时间 |
| `value` | `Keyframe<T>::value(&self) -> &T` | 获取值 |

### Curve<T>

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Curve<T>::new() -> Self` | 创建空曲线 |
| `with_interpolation` | `Curve<T>::with_interpolation(interp) -> Self` | 创建带插值模式的曲线 |
| `push` | `Curve<T>::push(&mut self, kf)` | 添加关键帧 |
| `insert_sorted` | `Curve<T>::insert_sorted(&mut self, kf)` | 有序插入关键帧 |
| `remove` | `Curve<T>::remove(&mut self, idx)` | 移除关键帧 |
| `len` | `Curve<T>::len(&self) -> usize` | 获取长度 |
| `is_empty` | `Curve<T>::is_empty(&self) -> bool` | 判空 |
| `keyframes` | `Curve<T>::keyframes(&self) -> &[Keyframe<T>]` | 获取关键帧数组 |
| `keyframes_mut` | `Curve<T>::keyframes_mut(&mut self) -> &mut [Keyframe<T>]` | 可变获取关键帧 |
| `sample` | `Curve<T>::sample(&self, time) -> T` | 采样曲线 |
| `sample_with_wrap` | `Curve<T>::sample_with_wrap(&self, time, wrap) -> T` | 带循环模式采样 |
| `duration` | `Curve<T>::duration(&self) -> f32` | 获取时长 |
| `optimize` | `Curve<T>::optimize(&mut self, max_error)` | 优化曲线 |
| `wrap_mode` | `Curve<T>::wrap_mode(&self) -> WrapMode` | 获取循环模式 |
| `set_wrap_mode` | `Curve<T>::set_wrap_mode(&mut self, mode)` | 设置循环模式 |

### Track

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Track::new(bone, translation_curve, rotation_curve, scale_curve) -> Self` | 创建轨道 |
| `bone` | `Track::bone(&self) -> usize` | 获取骨骼索引 |
| `translation` | `Track::translation(&self) -> &Curve<Vec3>` | 获取位移曲线 |
| `rotation` | `Track::rotation(&self) -> &Curve<Quat>` | 获取旋转曲线 |
| `scale` | `Track::scale(&self) -> &Curve<Vec3>` | 获取缩放曲线 |
| `custom_curves` | `Track::custom_curves(&self) -> &HashMap<String, Curve<f32>>` | 获取自定义曲线 |
| `sample_local_pose` | `Track::sample_local_pose(&self, time) -> (Vec3, Quat, Vec3)` | 采样局部姿态 |

### AnimationClip

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `AnimationClip::new(name, duration) -> Self` | 创建动画剪辑 |
| `with_warp_mode` | `AnimationClip::with_warp_mode(mode) -> Self` | 带循环模式创建 |
| `name` | `AnimationClip::name(&self) -> &str` | 获取名称 |
| `duration` | `AnimationClip::duration(&self) -> f32` | 获取时长 |
| `tracks` | `AnimationClip::tracks(&self) -> &[Track]` | 获取轨道数组 |
| `tracks_mut` | `AnimationClip::tracks_mut(&mut self) -> &mut Vec<Track>` | 可变获取轨道 |
| `add_track` | `AnimationClip::add_track(&mut self, track)` | 添加轨道 |
| `sample` | `AnimationClip::sample(&self, time) -> Pose` | 采样动画 |
| `sample_into` | `AnimationClip::sample_into(&self, time, pose)` | 采样到已有 Pose |
| `events` | `AnimationClip::events(&self) -> &[AnimationEvent]` | 获取事件数组 |
| `add_event` | `AnimationClip::add_event(&mut self, event)` | 添加事件 |
| `wrap_mode` | `AnimationClip::wrap_mode(&self) -> WrapMode` | 获取循环模式 |
| `set_wrap_mode` | `AnimationClip::set_wrap_mode(&mut self, mode)` | 设置循环模式 |
| `is_looping` | `AnimationClip::is_looping(&self) -> bool` | 是否循环 |
| `from_gltf` | `AnimationClip::from_gltf(path) -> Result<Vec<AnimationClip>>` | 从 glTF 加载 |
| `to_json` | `AnimationClip::to_json(&self) -> String` | 序列化为 JSON |
| `from_json` | `AnimationClip::from_json(json) -> Result<Self>` | 从 JSON 反序列化 |

### Pose

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Pose::new(num_bones) -> Self` | 创建姿态 |
| `with_default_bind` | `Pose::with_default_bind(skeleton) -> Self` | 从骨架创建绑定姿态 |
| `len` | `Pose::len(&self) -> usize` | 获取骨骼数量 |
| `bones` | `Pose::bones(&self) -> &[(Vec3, Quat, Vec3)]` | 获取骨骼数组 |
| `bones_mut` | `Pose::bones_mut(&mut self) -> &mut [(Vec3, Quat, Vec3)]` | 可变获取骨骼 |
| `set_bone` | `Pose::set_bone(&mut self, idx, pos, rot, scale)` | 设置骨骼变换 |
| `get_bone` | `Pose::get_bone(&self, idx) -> (Vec3, Quat, Vec3)` | 获取骨骼变换 |
| `blend` | `Pose::blend(a, b, alpha) -> Pose` | 混合两个姿态 |
| `blend_into` | `Pose::blend_into(&mut self, other, alpha)` | 混合到当前姿态 |
| `additive_blend` | `Pose::additive_blend(base, additive, alpha) -> Pose` | 加性混合 |
| `lerp` | `Pose::lerp(a, b, alpha) -> Pose` | 线性插值 |
| `identity` | `Pose::identity(num_bones) -> Pose` | 创建单位姿态 |
| `local_to_world` | `Pose::local_to_world(&self, skeleton) -> Vec<Mat4>` | 计算世界空间矩阵 |

### Bone

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Bone::new(name, parent, local_bind_pose, inverse_bind_pose) -> Self` | 创建骨骼 |
| `name` | `Bone::name(&self) -> &str` | 获取名称 |
| `parent` | `Bone::parent(&self) -> Option<usize>` | 获取父骨骼索引 |
| `local_bind_pose` | `Bone::local_bind_pose(&self) -> (Vec3, Quat, Vec3)` | 获取局部绑定姿态 |
| `inverse_bind_pose` | `Bone::inverse_bind_pose(&self) -> Mat4` | 获取逆绑定矩阵 |

### Skeleton

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Skeleton::new(bones) -> Self` | 创建骨架 |
| `bones` | `Skeleton::bones(&self) -> &[Bone]` | 获取骨骼数组 |
| `bone` | `Skeleton::bone(&self, idx) -> &Bone` | 获取单个骨骼 |
| `bone_count` | `Skeleton::bone_count(&self) -> usize` | 获取骨骼数量 |
| `root` | `Skeleton::root(&self) -> usize` | 获取根骨骼索引 |
| `children` | `Skeleton::children(&self, parent) -> Vec<usize>` | 获取子骨骼列表 |
| `bind_pose` | `Skeleton::bind_pose(&self) -> &Pose` | 获取绑定姿态 |
| `inverse_bind_matrices` | `Skeleton::inverse_bind_matrices(&self) -> &[Mat4]` | 获取逆绑定矩阵数组 |
| `find_bone_by_name` | `Skeleton::find_bone_by_name(&self, name) -> Option<usize>` | 按名称查找骨骼 |
| `from_gltf` | `Skeleton::from_gltf(path) -> Result<Self>` | 从 glTF 加载 |

### Skin

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Skin::new(bone_names, inverse_bind_matrices) -> Self` | 创建蒙皮 |
| `bone_count` | `Skin::bone_count(&self) -> usize` | 获取骨骼数量 |
| `bone_names` | `Skin::bone_names(&self) -> &[String]` | 获取骨骼名称 |
| `inverse_bind_matrices` | `Skin::inverse_bind_matrices(&self) -> &[Mat4]` | 获取逆绑定矩阵 |

### SkinnedMesh

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `SkinnedMesh::new(mesh, skin) -> Self` | 创建蒙皮网格 |
| `mesh` | `SkinnedMesh::mesh(&self) -> Handle<Mesh3D>` | 获取网格句柄 |
| `skin` | `SkinnedMesh::skin(&self) -> &Skin` | 获取蒙皮数据 |
| `vertex_weights` | `SkinnedMesh::vertex_weights(&self) -> &[Vec<VertexWeight>]` | 获取顶点权重 |
| `compute_matrix_palette` | `SkinnedMesh::compute_matrix_palette(&self, pose) -> Vec<Mat4>` | 计算矩阵调色板 |
| `from_gltf` | `SkinnedMesh::from_gltf(path) -> Result<Self>` | 从 glTF 加载 |

### Animator

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `Animator::new(skeleton_handle) -> Self` | 创建动画播放器 |
| `play` | `Animator::play(&mut self, clip)` | 播放动画 |
| `play_with_speed` | `Animator::play_with_speed(&mut self, clip, speed)` | 带速度播放 |
| `stop` | `Animator::stop(&mut self)` | 停止播放 |
| `is_playing` | `Animator::is_playing(&self) -> bool` | 是否播放中 |
| `time` | `Animator::time(&self) -> f32` | 当前时间 |
| `set_time` | `Animator::set_time(&mut self, t)` | 设置时间 |
| `speed` | `Animator::speed(&self) -> f32` | 获取速度 |
| `set_speed` | `Animator::set_speed(&mut self, speed)` | 设置速度 |
| `wrap_mode` | `Animator::wrap_mode(&self) -> WrapMode` | 获取循环模式 |
| `set_wrap_mode` | `Animator::set_wrap_mode(&mut self, mode)` | 设置循环模式 |
| `current_clip` | `Animator::current_clip(&self) -> Option<Handle<AnimationClip>>` | 当前剪辑 |
| `pose` | `Animator::pose(&self) -> &Pose` | 获取姿态 |
| `update` | `Animator::update(&mut self, dt)` | 更新动画 |
| `events_triggered` | `Animator::events_triggered(&self) -> &[AnimationEvent]` | 触发的事件 |

### StateMachine

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `StateMachine::new() -> Self` | 创建状态机 |
| `add_state` | `StateMachine::add_state(&mut self, name, node) -> StateHandle` | 添加状态 |
| `set_entry_state` | `StateMachine::set_entry_state(&mut self, name)` | 设置入口状态 |
| `add_transition` | `StateMachine::add_transition(&mut self, from, to, duration, condition)` | 添加过渡 |
| `add_any_state_transition` | `StateMachine::add_any_state_transition(&mut self, to, condition)` | 添加任意状态过渡 |
| `states` | `StateMachine::states(&self) -> &[StateNode]` | 获取状态数组 |
| `transitions` | `StateMachine::transitions(&self) -> &[Transition]` | 获取过渡数组 |
| `parameters` | `StateMachine::parameters(&self) -> &ParameterMap` | 获取参数映射 |

### AnimationController

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `AnimationController::new(state_machine) -> Self` | 创建控制器 |
| `machine` | `AnimationController::machine(&self) -> &StateMachine` | 获取状态机 |
| `set_parameter_float` | `AnimationController::set_parameter_float(&mut self, name, value)` | 设置浮点参数 |
| `set_parameter_bool` | `AnimationController::set_parameter_bool(&mut self, name, value)` | 设置布尔参数 |
| `set_parameter_int` | `AnimationController::set_parameter_int(&mut self, name, value)` | 设置整型参数 |
| `trigger` | `AnimationController::trigger(&mut self, name)` | 触发参数 |
| `current_state` | `AnimationController::current_state(&self) -> &str` | 当前状态 |
| `current_time` | `AnimationController::current_time(&self) -> f32` | 当前时间 |
| `update` | `AnimationController::update(&mut self, dt)` | 更新控制器 |
| `pose` | `AnimationController::pose(&self) -> &Pose` | 获取姿态 |

### BlendNode1D

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `BlendNode1D::new(param_name) -> Self` | 创建一维混合节点 |
| `with_nodes` | `BlendNode1D::with_nodes(nodes) -> Self` | 带节点创建 |
| `push` | `BlendNode1D::push(&mut self, value, clip)` | 添加节点 |
| `parameter` | `BlendNode1D::parameter(&self) -> &str` | 获取参数名 |
| `interpolate` | `BlendNode1D::interpolate(&self, param_value, clips) -> Pose` | 插值计算 |

### BlendNode2D

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `BlendNode2D::new(x_param, y_param) -> Self` | 创建二维混合节点 |
| `push` | `BlendNode2D::push(&mut self, (x, y), clip)` | 添加节点 |
| `interpolate` | `BlendNode2D::interpolate(&self, x, y) -> Pose` | 双线性插值 |

### IK（模块）

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `two_bone_ik` | `IK::two_bone_ik(shoulder, elbow, wrist, target_pos, elbow_dir) -> (Quat, Quat)` | 两骨 IK |
| `ccd_ik` | `IK::ccd_ik(chain, target, tolerance, max_iter) -> Vec<Quat>` | CCD IK |
| `fabrik` | `IK::fabrik(chain, target, tolerance, max_iter) -> Vec<Vec3>` | FABRIK |

### IKChain

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `IKChain::new(bones) -> Self` | 创建 IK 链 |
| `push` | `IKChain::push(&mut self, bone_idx)` | 添加骨骼 |
| `bones` | `IKChain::bones(&self) -> &[usize]` | 获取骨骼数组 |
| `root` | `IKChain::root(&self) -> usize` | 获取根骨骼 |
| `apply` | `IKChain::apply(&mut self, pose) -> Pose` | 应用 IK |

### AimIK

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `AimIK::new(skeleton, head_bone, aim_axis)` | 创建瞄准 IK |
| `apply` | `AimIK::apply(&self, pose, target_pos) -> Pose` | 应用瞄准 IK |

### LookAtIK

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `LookAtIK::new(skeleton, head_bone, up)` | 创建注视 IK |
| `apply` | `LookAtIK::apply(&self, pose, target_pos) -> Pose` | 应用注视 IK |

### FootIK

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `FootIK::new(skeleton, left_foot_bone, right_foot_bone)` | 创建脚部 IK |
| `apply` | `FootIK::apply(&self, world, pose, ground_height_fn) -> Pose` | 应用脚部 IK |

### Ragdoll

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `activate` | `Ragdoll::activate(&mut self, world)` | 激活 ragdoll |
| `deactivate` | `Ragdoll::deactivate(&mut self, world)` | 停用 ragdoll |
| `bake_pose` | `Ragdoll::bake_pose(&self, world) -> Pose` | 烘焙姿态 |
| `is_active` | `Ragdoll::is_active(&self) -> bool` | 是否激活 |

### RagdollBuilder

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `bone` | `RagdollBuilder::bone(&mut self, idx, collider, joint_type)` | 配置骨骼物理 |
| `build` | `RagdollBuilder::build(&self, world) -> Ragdoll` | 构建 Ragdoll |

### AnimationDebugRenderer

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `AnimationDebugRenderer::new() -> Self` | 创建调试渲染器 |
| `draw_skeleton` | `AnimationDebugRenderer::draw_skeleton(&mut self, skeleton, pose, transform, color)` | 绘制骨骼 |
| `draw_joints` | `AnimationDebugRenderer::draw_joints(&mut self, skeleton, pose, color)` | 绘制关节 |
| `draw_bone_weights` | `AnimationDebugRenderer::draw_bone_weights(&mut self, mesh, weights, color)` | 绘制权重 |
| `flush` | `AnimationDebugRenderer::flush(&self, renderer)` | 刷新渲染 |
| `clear` | `AnimationDebugRenderer::clear(&mut self)` | 清除缓存 |

### PlayBack

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `PlayBack::new() -> Self` | 创建播放队列 |
| `play` | `PlayBack::play(&mut self, clip)` | 播放 |
| `stop` | `PlayBack::stop(&mut self)` | 停止 |
| `queue` | `PlayBack::queue(&mut self, clip)` | 加入队列 |
| `crossfade` | `PlayBack::crossfade(&mut self, clip, duration)` | 交叉淡入 |
| `jump_to` | `PlayBack::jump_to(&mut self, clip, time)` | 跳转 |
| `set_time` | `PlayBack::set_time(&mut self, time)` | 设置时间 |
| `set_speed` | `PlayBack::set_speed(&mut self, speed)` | 设置速度 |
| `is_playing` | `PlayBack::is_playing(&self) -> bool` | 是否播放中 |
| `time` | `PlayBack::time(&self) -> f32` | 当前时间 |
| `update` | `PlayBack::update(&mut self, dt)` | 更新 |
| `pose` | `PlayBack::pose(&self) -> &Pose` | 获取姿态 |
| `events` | `PlayBack::events(&self) -> &[AnimationEvent]` | 获取事件 |

---

## 枚举类型

### WrapMode

| 枚举值 | 描述 |
|--------|------|
| `Once` | 播放一次后停留在最后一帧 |
| `Loop` | 循环播放 |
| `PingPong` | 来回播放 |
| `Clamp` | 夹取到边界值 |
| `ClampForever` | 超时后保持最后一帧 |

### KeyframeInterpolation

| 枚举值 | 描述 |
|--------|------|
| `Linear` | 线性插值 |
| `Step` | 阶梯插值 |
| `Bezier(c0, c1)` | 贝塞尔曲线插值 |
| `Hermite(tan_in, tan_out)` | 埃尔米特插值 |
| `EaseIn` | 缓入 |
| `EaseOut` | 缓出 |
| `EaseInOut` | 缓入缓出 |

### Interpolation (glTF)

| 枚举值 | 描述 |
|--------|------|
| `Linear` | 线性插值 |
| `Step` | 阶梯插值 |
| `CubicSpline` | 三次样条插值 |

### TrackTarget

| 枚举值 | 描述 |
|--------|------|
| `Translation` | 位移 |
| `Rotation` | 旋转 |
| `Scale` | 缩放 |
| `Float(String)` | 自定义浮点属性 |

### ParameterValue

| 枚举值 | 描述 |
|--------|------|
| `Bool` | 布尔值 |
| `Float` | 浮点值 |
| `Int` | 整数值 |
| `Vec2` | 二维向量 |
| `Vec3` | 三维向量 |
| `Trigger` | 一次性触发事件 |

### CompareOp

| 枚举值 | 描述 |
|--------|------|
| `Equal` | 等于 |
| `NotEqual` | 不等于 |
| `Less` | 小于 |
| `LessEqual` | 小于等于 |
| `Greater` | 大于 |
| `GreaterEqual` | 大于等于 |

### Condition

| 枚举值 | 描述 |
|--------|------|
| `True` | 始终为真 |
| `False` | 始终为假 |
| `Parameter(name, op, value)` | 参数条件 |
| `And(a, b)` | 与条件 |
| `Or(a, b)` | 或条件 |
| `Not(a)` | 非条件 |
| `TimeElapsed(seconds)` | 时间流逝条件 |
| `EventTriggered(name)` | 事件触发条件 |

### BlendMode

| 枚举值 | 描述 |
|--------|------|
| `Linear` | 线性混合 |
| `Additive` | 加性混合 |
| `Crossfade` | 交叉淡入淡出 |

### StateNode

| 枚举值 | 描述 |
|--------|------|
| `Clip(clip_handle)` | 动画剪辑节点 |
| `Blend1D(tree)` | 一维混合节点 |
| `Blend2D(tree)` | 二维混合节点 |
| `BlendTree(node)` | 混合树节点 |
| `Layered(layered)` | 层级混合节点 |
| `StateMachine(nested)` | 嵌套状态机 |

### RagdollJointType

| 枚举值 | 描述 |
|--------|------|
| `Ball` | 球关节 |
| `Revolute` | 旋转关节 |
| `Fixed` | 固定关节 |

---

## 资源加载器

### AnimationAssetLoader

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `AnimationAssetLoader::new()` | 创建加载器 |
| `load` | `AnimationAssetLoader::load(path) -> Handle<AnimationClip>` | 加载动画 |
| `get` | `AnimationAssetLoader::get(handle) -> &AnimationClip` | 获取动画 |
| `contains` | `AnimationAssetLoader::contains(handle) -> bool` | 是否包含 |
| `unload` | `AnimationAssetLoader::unload(handle)` | 卸载动画 |

### SkeletonAssetLoader

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `SkeletonAssetLoader::new()` | 创建加载器 |
| `load` | `SkeletonAssetLoader::load(path) -> Handle<Skeleton>` | 加载骨架 |
| `get` | `SkeletonAssetLoader::get(handle) -> &Skeleton` | 获取骨架 |

### SkinAssetLoader

| 方法 | 签名 | 功能描述 |
|------|------|----------|
| `new` | `SkinAssetLoader::new()` | 创建加载器 |
| `load` | `SkinAssetLoader::load(path) -> Handle<Skin>` | 加载蒙皮 |
| `get` | `SkinAssetLoader::get(handle) -> &Skin` | 获取蒙皮 |

---

## 系统函数

| 函数 | 签名 | 功能描述 |
|------|------|----------|
| `animation_clip_sample_system` | `animation_clip_sample_system(world)` | 动画剪辑采样系统 |
| `animation_controller_update_system` | `animation_controller_update_system(world, dt)` | 控制器更新系统 |
| `animation_event_system` | `animation_event_system(world)` | 动画事件系统 |
| `animation_pose_apply_system` | `animation_pose_apply_system(world)` | 姿态应用系统 |
| `animation_skinning_system` | `animation_skinning_system(world)` | 蒙皮计算系统 |
| `animation_skinning_render_system` | `animation_skinning_render_system(world, renderer, camera)` | 蒙皮渲染系统 |
| `animation_debug_draw_system` | `animation_debug_draw_system(world, renderer)` | 调试绘制系统 |
| `animation_ragdoll_system` | `animation_ragdoll_system(world)` | Ragdoll 系统 |
| `animation_ik_system` | `animation_ik_system(world)` | IK 系统 |
| `animation_look_at_system` | `animation_look_at_system(world)` | 注视 IK 系统 |
| `animation_additive_blend_system` | `animation_additive_blend_system(world)` | 加性混合系统 |
| `animation_crossfade_system` | `animation_crossfade_system(world)` | 交叉淡入系统 |
| `animation_event_dispatch_system` | `animation_event_dispatch_system(world)` | 事件分发系统 |

---

## 资源类型

| 类型 | 描述 |
|------|------|
| `AnimationClipSet` | AnimationClip Handle 集合 |
| `AnimationClipMapping` | clip 名称到 handle 的映射 |
| `SkinMatrixPalette` | skinning 矩阵数组 |
| `SkinMatrixBuffer` | GPU 端 buffer |
| `SkinUniform` | shader 中 uniform |