# 动画剪辑模块 (Animation Clip)

## 模块概述

动画剪辑是动画系统的核心数据结构，负责存储关键帧数据并提供采样能力。本模块包含关键帧（Keyframe）、曲线（Curve）、轨道（Track）、动画剪辑（AnimationClip）和姿态（Pose）等核心类型。

## 需求编号与功能描述

### Keyframe 关键帧

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 2 | Keyframe 数据结构 | `Keyframe<T>` | time: f32, value: T, interpolation: Interpolation | - |
| 3 | 获取时间 | `Keyframe<T>::time(&self) -> f32` | - | f32 |
| 4 | 获取值 | `Keyframe<T>::value(&self) -> &T` | - | &T |
| 187 | 构造函数 | `Keyframe<T>::new(time, value)` | f32, T | Self |
| 188 | 带插值模式构造 | `Keyframe<T>::with_interpolation(interp)` | Interpolation | Self |
| 227 | 插值模式枚举 | `KeyframeInterpolation::Linear / Step / Bezier(c0, c1) / Hermite(tan_in, tan_out) / EaseIn / EaseOut / EaseInOut` | - | 枚举 |

### Curve 曲线

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 5 | Curve 数据结构 | `Curve<T>` | Vec<Keyframe<T>>, 插值模式 | - |
| 6 | 采样曲线 | `Curve<T>::sample(&self, t) -> T` | f32 时间 | T 值 |
| 7 | 获取时长 | `Curve<T>::duration(&self) -> f32` | - | f32 |
| 8 | 插入关键帧 | `Curve<T>::insert(&mut self, keyframe)` | Keyframe | - |
| 9 | 移除关键帧 | `Curve<T>::remove(&mut self, idx)` | usize 索引 | - |
| 10 | 优化曲线 | `Curve<T>::optimize(&mut self, error)` | f32 误差阈值 | - |
| 228 | 默认构造 | `Curve<T>::new() -> Self` | - | Self |
| 229 | 带插值模式构造 | `Curve<T>::with_interpolation(interp) -> Self` | Interpolation | Self |
| 230 | 添加关键帧 | `Curve<T>::push(&mut self, kf)` | Keyframe | - |
| 231 | 有序插入 | `Curve<T>::insert_sorted(&mut self, kf)` | Keyframe | - |
| 232 | 移除关键帧 | `Curve<T>::remove(&mut self, idx)` | usize | - |
| 233 | 获取长度 | `Curve<T>::len(&self) -> usize` | - | usize |
| 234 | 判空 | `Curve<T>::is_empty(&self) -> bool` | - | bool |
| 235 | 获取关键帧数组 | `Curve<T>::keyframes(&self) -> &[Keyframe<T>]` | - | &[Keyframe<T>] |
| 236 | 可变获取关键帧 | `Curve<T>::keyframes_mut(&mut self) -> &mut [Keyframe<T>]` | - | &mut [Keyframe<T>] |
| 237 | 采样 | `Curve<T>::sample(&self, time) -> T` | f32 | T |
| 238 | 带 WrapMode 采样 | `Curve<T>::sample_with_wrap(&self, time, wrap) -> T` | f32, WrapMode | T |
| 239 | 时长 | `Curve<T>::duration(&self) -> f32` | - | f32 |
| 244 | 优化关键帧 | `Curve<T>::optimize(&mut self, max_error)` | f32 | - |
| 245 | 获取 WrapMode | `Curve<T>::wrap_mode(&self) -> WrapMode` | - | WrapMode |
| 246 | 设置 WrapMode | `Curve<T>::set_wrap_mode(&mut self, mode)` | WrapMode | - |
| 248 | 时间包装函数 | `wrap_time(time, duration, mode) -> f32` | f32, f32, WrapMode | f32 |

**插值实现要求：**
- 202: `Curve<Vec3>` 使用线性插值
- 203: `Curve<Quat>` 使用 slerp
- 204: `Curve<f32>` 使用线性插值
- 205: `Curve<Color>` 使用线性插值

### Track 轨道

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 11 | Track 数据结构 | `Track` | entity + property 绑定曲线 | - |
| 12 | 轨道目标类型 | `TrackTarget::Translation / Rotation / Scale / Float(String)` | - | 枚举 |
| 211 | Track 构造 | `Track::new(bone, translation_curve, rotation_curve, scale_curve) -> Self` | usize, Curve<Vec3>, Curve<Quat>, Curve<Vec3> | Self |
| 212 | 获取骨骼索引 | `Track::bone(&self) -> usize` | - | usize |
| 213 | 获取位移曲线 | `Track::translation(&self) -> &Curve<Vec3>` | - | &Curve<Vec3> |
| 214 | 获取旋转曲线 | `Track::rotation(&self) -> &Curve<Quat>` | - | &Curve<Quat> |
| 215 | 获取缩放曲线 | `Track::scale(&self) -> &Curve<Vec3>` | - | &Curve<Vec3> |
| 216 | 获取自定义曲线 | `Track::custom_curves(&self) -> &HashMap<String, Curve<f32>>` | - | &HashMap |
| 217 | 采样局部姿态 | `Track::sample_local_pose(&self, time) -> (Vec3, Quat, Vec3)` | f32 | (Vec3, Quat, Vec3) |

### AnimationClip 动画剪辑

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 13 | 创建动画剪辑 | `AnimationClip::new(name, duration) -> Self` | &str, f32 | Self |
| 14 | 获取名称 | `AnimationClip::name(&self) -> &str` | - | &str |
| 15 | 获取时长 | `AnimationClip::duration(&self) -> f32` | - | f32 |
| 16 | 添加轨道 | `AnimationClip::add_track(&mut self, track)` | Track | - |
| 17 | 获取轨道数组 | `AnimationClip::tracks(&self) -> &[Track]` | - | &[Track] |
| 18 | 采样动画 | `AnimationClip::sample(&self, time) -> Pose` | f32 | Pose |
| 19 | 获取循环模式 | `AnimationClip::wrap_mode(&self) -> WrapMode` | - | WrapMode |
| 20 | 设置循环模式 | `AnimationClip::set_wrap_mode(&mut self, mode)` | WrapMode | - |
| 21 | WrapMode 枚举 | `WrapMode::Once / Loop / PingPong / Clamp` | - | 枚举 |
| 218 | 带循环模式创建 | `AnimationClip::with_warp_mode(mode) -> Self` | WrapMode | Self |
| 258 | 获取轨道（可变） | `AnimationClip::tracks_mut(&mut self) -> &mut Vec<Track>` | - | &mut Vec<Track> |
| 259 | 添加轨道 | `AnimationClip::add_track(&mut self, track)` | Track | - |
| 260 | 采样到 Pose | `AnimationClip::sample(&self, time) -> Pose` | f32 | Pose |
| 261 | 采样到已有 Pose | `AnimationClip::sample_into(&self, time, pose)` | f32, &mut Pose | - |
| 262 | 获取事件数组 | `AnimationClip::events(&self) -> &[AnimationEvent]` | - | &[AnimationEvent] |
| 263 | 添加事件 | `AnimationClip::add_event(&mut self, event)` | AnimationEvent | - |
| 264 | 获取循环模式 | `AnimationClip::wrap_mode(&self) -> WrapMode` | - | WrapMode |
| 265 | 设置循环模式 | `AnimationClip::set_wrap_mode(&mut self, mode)` | WrapMode | - |
| 266 | 是否循环 | `AnimationClip::is_looping(&self) -> bool` | - | bool |
| 405 | 从 glTF 加载 | `AnimationClip::from_gltf(path) -> Result<Vec<AnimationClip>>` | &str | Result<Vec<AnimationClip>> |
| 406 | 序列化为 JSON | `AnimationClip::to_json(&self) -> String` | - | String |
| 407 | 从 JSON 反序列化 | `AnimationClip::from_json(json) -> Result<Self>` | &str | Result<Self> |

**WrapMode 扩展（需求 247）：**
- Once: 播放一次后停留在最后一帧
- Loop: 循环播放
- PingPong: 来回播放
- Clamp: 夹取到边界值
- ClampForever: 超时后保持最后一帧

### Pose 姿态

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 22 | Pose 数据结构 | `Pose` | 所有骨骼的局部变换数组 | - |
| 23 | 创建 Pose | `Pose::new(num_bones) -> Self` | usize | Self |
| 24 | 获取骨骼数组 | `Pose::bones(&self) -> &[(Vec3, Quat, Vec3)]` | - | &[(Vec3, Quat, Vec3)] |
| 25 | 设置骨骼变换 | `Pose::set_bone(&mut self, idx, pos, rot, scale)` | usize, Vec3, Quat, Vec3 | - |
| 26 | 获取骨骼变换 | `Pose::get_bone(&self, idx) -> (Vec3, Quat, Vec3)` | usize | (Vec3, Quat, Vec3) |
| 27 | 混合两个姿态 | `Pose::blend(a, b, alpha) -> Pose` | &Pose, &Pose, f32 | Pose |
| 28 | 加性混合 | `Pose::additive_blend(base, additive, alpha) -> Pose` | &Pose, &Pose, f32 | Pose |
| 29 | 克隆到目标 | `Pose::clone_into(&self, other) -> ()` | &mut Pose | - |
| 267 | 创建带默认绑定姿态 | `Pose::with_default_bind(skeleton) -> Self` | &Skeleton | Self |
| 268 | 获取骨骼数量 | `Pose::len(&self) -> usize` | - | usize |
| 269 | 获取骨骼数组 | `Pose::bones(&self) -> &[(Vec3, Quat, Vec3)]` | - | &[(Vec3, Quat, Vec3)] |
| 270 | 可变获取骨骼 | `Pose::bones_mut(&mut self) -> &mut [(Vec3, Quat, Vec3)]` | - | &mut [(Vec3, Quat, Vec3)] |
| 271 | 设置骨骼变换 | `Pose::set_bone(&mut self, idx, pos, rot, scale)` | usize, Vec3, Quat, Vec3 | - |
| 272 | 获取骨骼变换 | `Pose::get_bone(&self, idx) -> (Vec3, Quat, Vec3)` | usize | (Vec3, Quat, Vec3) |
| 273 | 混合姿态 | `Pose::blend(a, b, alpha) -> Pose` | &Pose, &Pose, f32 | Pose |
| 274 | 混合到目标 | `Pose::blend_into(&mut self, other, alpha)` | &Pose, f32 | - |
| 275 | 加性混合 | `Pose::additive_blend(base, additive, alpha) -> Pose` | &Pose, &Pose, f32 | Pose |
| 276 | 线性插值混合 | `Pose::lerp(a, b, alpha) -> Pose` | &Pose, &Pose, f32 | Pose |
| 277 | 创建单位姿态 | `Pose::identity(num_bones) -> Pose` | usize | Pose |
| 278 | 计算世界空间矩阵 | `Pose::local_to_world(&self, skeleton) -> Vec<Mat4>` | &Skeleton | Vec<Mat4> |

### AnimationEvent 动画事件

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 97 | 动画事件结构 | `AnimationEvent` | name + time + payload | - |
| 126 | 创建事件 | `AnimationEvent::new(name, time) -> Self` | &str, f32 | Self |
| 127 | 带载荷创建 | `AnimationEvent::with_payload(name, time, payload) -> Self` | &str, f32, &str | Self |
| 128 | 获取名称 | `AnimationEvent::name(&self) -> &str` | - | &str |
| 129 | 获取时间 | `AnimationEvent::time(&self) -> f32` | - | f32 |
| 130 | 获取载荷 | `AnimationEvent::payload(&self) -> Option<&str>` | - | Option<&str> |
| 98 | 事件系统 | `AnimationEventSystem` | 每帧查询触发的事件 | - |

### AnimationSampler (glTF)

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 143 | Sampler 抽象 | `AnimationSampler` | glTF sampler | - |
| 144 | 获取插值模式 | `AnimationSampler::interpolation(&self) -> Interpolation` | - | Interpolation |
| 145 | Interpolation 枚举 | `Interpolation::Linear / Step / CubicSpline` | - | 枚举 |
| 146 | 获取时间输入 | `AnimationSampler::input(&self) -> &[f32]` | - | &[f32] |
| 147 | 获取值输出 | `AnimationSampler::output(&self) -> &[T]` | - | &[T] |
| 419 | 从 glTF 创建 | `AnimationSampler::from_gltf(sampler) -> Self` | gltf::animation::Sampler | Self |
| 420 | 获取插值 | `AnimationSampler::interpolation(&self) -> Interpolation` | - | Interpolation |
| 421 | 获取输入时间 | `AnimationSampler::input(&self) -> &[f32]` | - | &[f32] |
| 422 | 输出 Vec3 | `AnimationSampler::output_vec3(&self) -> &[Vec3]` | - | &[Vec3] |
| 423 | 输出 Quat | `AnimationSampler::output_quat(&self) -> &[Quat]` | - | &[Quat] |
| 424 | 采样 Vec3 | `AnimationSampler::sample_vec3(&self, t) -> Vec3` | f32 | Vec3 |
| 425 | 采样 Quat | `AnimationSampler::sample_quat(&self, t) -> Quat` | f32 | Quat |
| 427 | CubicSpline 实现 | CubicSpline 切线采样实现正确 | - | - |

### Animator 动画播放器组件

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 52 | Animator 组件 | `Animator` | 当前 clip + 速度 + 时间 | - |
| 53 | 播放动画 | `Animator::play(clip_handle)` | Handle<AnimationClip> | - |
| 54 | 停止播放 | `Animator::stop(&mut self)` | - | - |
| 55 | 设置时间 | `Animator::set_time(&mut self, t)` | f32 | - |
| 56 | 获取时间 | `Animator::time(&self) -> f32` | - | f32 |
| 57 | 设置速度 | `Animator::set_speed(&mut self, speed)` | f32 | - |
| 58 | 获取速度 | `Animator::speed(&self) -> f32` | - | f32 |
| 59 | 是否正在播放 | `Animator::is_playing(&self) -> bool` | - | bool |
| 60 | 获取触发事件 | `Animator::events(&self) -> &[AnimationEvent]` | - | &[AnimationEvent] |
| 321 | 创建 Animator | `Animator::new(skeleton_handle) -> Self` | Handle<Skeleton> | Self |
| 322 | 播放 | `Animator::play(&mut self, clip)` | Handle<AnimationClip> | - |
| 323 | 带速度播放 | `Animator::play_with_speed(&mut self, clip, speed)` | Handle<AnimationClip>, f32 | - |
| 324 | 停止 | `Animator::stop(&mut self)` | - | - |
| 325 | 是否播放中 | `Animator::is_playing(&self) -> bool` | - | bool |
| 326 | 当前时间 | `Animator::time(&self) -> f32` | - | f32 |
| 327 | 设置时间 | `Animator::set_time(&mut self, t)` | f32 | - |
| 328 | 获取速度 | `Animator::speed(&self) -> f32` | - | f32 |
| 329 | 设置速度 | `Animator::set_speed(&mut self, speed)` | f32 | - |
| 330 | 获取 WrapMode | `Animator::wrap_mode(&self) -> WrapMode` | - | WrapMode |
| 331 | 设置 WrapMode | `Animator::set_wrap_mode(&mut self, mode)` | WrapMode | - |
| 332 | 当前 clip | `Animator::current_clip(&self) -> Option<Handle<AnimationClip>>` | - | Option<Handle<AnimationClip>> |
| 333 | 获取 Pose | `Animator::pose(&self) -> &Pose` | - | &Pose |
| 334 | 更新 | `Animator::update(&mut self, dt)` | f32 | - |
| 335 | 触发的事件 | `Animator::events_triggered(&self) -> &[AnimationEvent]` | - | &[AnimationEvent] |

## 验收标准

- [ ] `Keyframe<T>` 支持所有插值模式
- [ ] `Curve<Vec3>` 线性插值正确（需求 202）
- [ ] `Curve<Quat>` slerp 正确（需求 203）
- [ ] `Curve<f32>` 线性插值正确（需求 204）
- [ ] `AnimationClip::sample` 返回正确 Pose
- [ ] `AnimationClip::wrap_mode_loop` 时间回绕正确（需求 167）
- [ ] `AnimationEvent` 在指定时间触发（需求 169）
- [ ] `AnimationSampler::CubicSpline` 采样正确（需求 170）
- [ ] 公开 API doc comment 覆盖率 100%（需求 211）

## 依赖关系

- 依赖 `nalgebra` 进行数学计算
- 依赖 `gltf`  crate 进行 glTF 加载
- 被 Skeleton、StateMachine、IK 等模块使用

## 优先级

**P0（核心）：**
- Keyframe、Curve、Track、AnimationClip、Pose 基础结构
- 插值实现（Linear、Step、Slerp）
- WrapMode 处理
- sample 方法

**P1（重要）：**
- AnimationEvent 事件系统
- AnimationSampler glTF 解析
- CubicSpline 插值
- Pose::local_to_world 计算

**P2（增强）：**
- Curve 优化算法
- JSON 序列化/反序列化
- Curve 压缩