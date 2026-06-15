# 示例实现指南

## 概述

本文档提供 `engine-animation` crate 的示例实现指南，帮助开发者快速上手使用动画系统。每个示例对应一个具体的使用场景，包含代码示例和解释。

---

## 示例清单

| 示例名称 | 功能描述 | 需求编号 |
|----------|----------|----------|
| `animation_basic` | 播放单个动画循环 | 464 |
| `animation_state_machine` | Idle/Walk/Run/Jump 状态机 | 465, 214, 502 |
| `animation_blend_1d` | 基于 speed 参数混合 Idle→Walk→Run | 466 |
| `animation_blend_2d` | 2D 混合（XZ 方向混合） | 467 |
| `animation_additive` | 叠加呼吸 idle + 上半身攻击 | 468 |
| `animation_ragdoll` | 物理 ragdoll | 469, 186 |
| `animation_ik` | 两 bone IK 瞄准目标 | 470 |
| `animation_look_at` | 头部看向鼠标 | 471 |
| `animation_retarget` | 同 clip 复用 | 472 |
| `animation_event` | 动画事件（脚步声触发音效） | 473 |

---

## 示例实现详解

### 1. animation_basic - 基础动画播放

**功能说明**：加载并循环播放单个动画剪辑。

**核心代码结构**：

```rust
// 加载动画和骨架
let skeleton = Skeleton::from_gltf("character.gltf")?;
let clips = AnimationClip::from_gltf("character.gltf")?;
let idle_clip = clips[0].clone();

// 创建 Animator 组件
let mut animator = Animator::new(skeleton_handle);
animator.play(idle_clip_handle);

// 游戏循环中更新
animator.update(dt);
let pose = animator.pose();

// 应用到蒙皮网格
let matrix_palette = skinned_mesh.compute_matrix_palette(pose);
skinned_mesh_renderer.draw(renderer, mesh, skeleton, pose, material, camera);
```

**关键点**：
- 使用 `AnimationClip::from_gltf` 加载动画
- `Animator` 组件管理播放状态
- 每帧调用 `update(dt)` 推进动画时间

---

### 2. animation_state_machine - 状态机控制

**功能说明**：基于输入切换 Idle/Walk/Run/Jump 状态。

**核心代码结构**：

```rust
// 使用构建器创建状态机
let controller = AnimationControllerBuilder::new()
    .with_state("Idle", StateNode::Clip(idle_clip))
    .with_state("Walk", StateNode::Clip(walk_clip))
    .with_state("Run", StateNode::Clip(run_clip))
    .with_state("Jump", StateNode::Clip(jump_clip))
    .with_entry("Idle")
    .with_transition("Idle", "Walk", 0.2, Condition::Parameter("speed", Greater, 0.1))
    .with_transition("Walk", "Idle", 0.2, Condition::Parameter("speed", LessEqual, 0.1))
    .with_transition("Walk", "Run", 0.3, Condition::Parameter("speed", Greater, 5.0))
    .with_transition("Run", "Walk", 0.3, Condition::Parameter("speed", LessEqual, 5.0))
    .with_transition("Idle", "Jump", 0.1, Condition::EventTriggered("jump"))
    .with_transition("Walk", "Jump", 0.1, Condition::EventTriggered("jump"))
    .with_transition("Run", "Jump", 0.1, Condition::EventTriggered("jump"))
    .with_transition("Jump", "Idle", 0.2, Condition::TimeElapsed(1.0))
    .build();

// 更新参数
controller.set_parameter_float("speed", player_velocity.length());
if input.jump_pressed {
    controller.trigger("jump");
}

// 更新控制器
controller.update(dt);
let pose = controller.pose();
```

**关键点**：
- 使用 `AnimationControllerBuilder` 流畅构建状态机
- 通过 `set_parameter_*` 设置参数
- 使用 `trigger` 触发一次性事件

---

### 3. animation_blend_1d - 一维混合

**功能说明**：基于 speed 参数平滑混合 Idle→Walk→Run 动画。

**核心代码结构**：

```rust
// 创建 1D 混合节点
let blend_1d = BlendNode1D::new("speed")
    .with_nodes(vec![
        (0.0, idle_clip),
        (2.0, walk_clip),
        (6.0, run_clip),
    ]);

// 创建状态机使用混合节点
let controller = AnimationControllerBuilder::new()
    .with_state("Movement", StateNode::Blend1D(blend_1d))
    .with_entry("Movement")
    .build();

// 更新速度参数
controller.set_parameter_float("speed", player_velocity.length());
controller.update(dt);
let pose = controller.pose();
```

**关键点**：
- `BlendNode1D` 根据参数值自动选择相邻 clip 进行插值
- 参数值超出范围时使用边界 clip

---

### 4. animation_blend_2d - 二维混合

**功能说明**：基于 XZ 方向速度进行双线性混合。

**核心代码结构**：

```rust
// 创建 2D 混合节点（前/后/左/右四个方向）
let blend_2d = BlendNode2D::new("x_speed", "z_speed")
    .push((-1.0, 0.0), walk_back_clip)
    .push((1.0, 0.0), walk_front_clip)
    .push((0.0, -1.0), walk_left_clip)
    .push((0.0, 1.0), walk_right_clip)
    .push((1.0, 1.0), walk_front_right_clip);

// 创建状态机
let controller = AnimationControllerBuilder::new()
    .with_state("Movement", StateNode::Blend2D(blend_2d))
    .with_entry("Movement")
    .build();

// 更新参数
let velocity = player_velocity.xz();
controller.set_parameter_float("x_speed", velocity.x);
controller.set_parameter_float("z_speed", velocity.y);
controller.update(dt);
let pose = controller.pose();
```

**关键点**：
- `BlendNode2D` 使用双线性插值混合四个相邻 clip
- 需要配置 X 和 Y 两个参数

---

### 5. animation_additive - 加性动画

**功能说明**：在基础动画上叠加额外动画（如呼吸 + 攻击）。

**核心代码结构**：

```rust
// 创建层级混合
let layered = LayeredBlend::new(
    StateNode::Clip(idle_clip),  // 基础层
    vec![
        Layer::new(StateNode::Clip(breathing_clip), 0.5, None),
        Layer::new(StateNode::Clip(attack_clip), 1.0, Some(attack_mask)),
    ]
);

// 创建状态机
let controller = AnimationControllerBuilder::new()
    .with_state("Idle", StateNode::Layered(layered))
    .with_entry("Idle")
    .build();

// 动态调整层权重
let layers = controller.state_mut("Idle").layers_mut();
layers[1].set_weight(attacking as f32);
```

**关键点**：
- `LayeredBlend` 支持多层叠加
- 使用 `AnimationMask` 限制层影响的骨骼
- 可动态调整每层权重

---

### 6. animation_ragdoll - 物理布娃娃

**功能说明**：角色死亡后切换为物理驱动的 ragdoll 效果。

**核心代码结构**：

```rust
// 构建 Ragdoll
let ragdoll = Ragdoll::new(&skeleton)
    .bone(0, Collider::capsule(0.5, 0.3), RagdollJointType::Fixed)  // 根骨骼
    .bone(1, Collider::capsule(0.4, 0.5), RagdollJointType::Ball)   // 脊柱
    .bone(2, Collider::capsule(0.3, 0.6), RagdollJointType::Ball)   // 左臂
    .bone(3, Collider::capsule(0.3, 0.6), RagdollJointType::Ball)   // 右臂
    // ... 其他骨骼
    .build(world);

// 角色死亡时激活 ragdoll
if character_dead && !ragdoll.is_active() {
    ragdoll.activate(world);
}

// 动画状态与物理状态同步
if ragdoll.is_active() {
    let pose = ragdoll.bake_pose(world);
    skinned_mesh_renderer.draw(renderer, mesh, skeleton, pose, material, camera);
} else {
    // 使用动画控制器的 pose
    controller.update(dt);
    let pose = controller.pose();
    skinned_mesh_renderer.draw(renderer, mesh, skeleton, pose, material, camera);
}
```

**关键点**：
- `RagdollBuilder` 配置每个骨骼的碰撞体和关节类型
- `activate` / `deactivate` 切换物理模拟状态
- `bake_pose` 从物理状态提取姿态

---

### 7. animation_ik - 反向运动学

**功能说明**：使用两骨 IK 让角色手臂瞄准目标点。

**核心代码结构**：

```rust
// 创建 IK 链（肩膀 -> 肘部 -> 手腕）
let mut ik_chain = IKChain::new(vec![shoulder_idx, elbow_idx, wrist_idx]);

// 获取当前骨骼世界位置
let shoulder_pos = pose.local_to_world(&skeleton)[shoulder_idx].translation();
let elbow_pos = pose.local_to_world(&skeleton)[elbow_idx].translation();
let wrist_pos = pose.local_to_world(&skeleton)[wrist_idx].translation();

// 应用两骨 IK
let target_pos = mouse_world_position;
let elbow_dir = Vec3::new(0.0, -1.0, 0.0);  // 肘部向下
let (shoulder_rot, elbow_rot) = IK::two_bone_ik(
    shoulder_pos, elbow_pos, wrist_pos, target_pos, elbow_dir
);

// 将旋转应用到 pose
pose.set_bone(shoulder_idx, shoulder_pos, shoulder_rot, Vec3::ONE);
pose.set_bone(elbow_idx, elbow_pos, elbow_rot, Vec3::ONE);
```

**关键点**：
- `IKChain` 定义骨骼链
- `IK::two_bone_ik` 计算关节旋转
- 结果需要转换回局部坐标

---

### 8. animation_look_at - 注视目标

**功能说明**：让角色头部看向鼠标位置。

**核心代码结构**：

```rust
// 创建 LookAtIK
let look_at = LookAtIK::new(&skeleton, head_bone_idx, Vec3::Y);

// 更新头部朝向
let target_pos = mouse_world_position;
let pose = look_at.apply(&mut base_pose, target_pos);
```

**关键点**：
- `LookAtIK` 自动计算头部旋转
- 支持设置向上向量

---

### 9. animation_retarget - 动画重定向

**功能说明**：将同一动画 clip 应用到不同骨架。

**核心代码结构**：

```rust
// 加载源动画和目标骨架
let source_clip = AnimationClip::from_gltf("generic_walk.gltf")?;
let target_skeleton = Skeleton::from_gltf("character_skeleton.gltf")?;

// 创建骨骼映射（名称匹配）
let bone_mapping: Vec<Option<usize>> = source_skeleton.bones()
    .iter()
    .map(|bone| target_skeleton.find_bone_by_name(bone.name()))
    .collect();

// 重定向动画
let retargeted_clip = source_clip.retarget(&bone_mapping, &target_skeleton);

// 播放重定向后的动画
animator.play(retargeted_clip_handle);
```

**关键点**：
- 通过骨骼名称匹配进行重定向
- 支持不同骨架结构的动画复用

---

### 10. animation_event - 动画事件

**功能说明**：在动画特定时间触发事件（如脚步声）。

**核心代码结构**：

```rust
// 创建带有事件的动画剪辑
let mut walk_clip = AnimationClip::new("walk", 1.0);
walk_clip.add_event(AnimationEvent::new("footstep_left", 0.0));
walk_clip.add_event(AnimationEvent::new("footstep_right", 0.5));

// 更新动画并获取触发的事件
animator.play(walk_clip_handle);
animator.update(dt);

for event in animator.events_triggered() {
    match event.name() {
        "footstep_left" => play_sound("footstep_left.wav"),
        "footstep_right" => play_sound("footstep_right.wav"),
        _ => {}
    }
}
```

**关键点**：
- 使用 `add_event` 添加时间点事件
- 通过 `events_triggered` 获取当前帧触发的事件
- 支持自定义 payload

---

## 最佳实践

### 性能优化

1. **动画压缩**：使用 `Curve::optimize` 去除冗余关键帧
2. **实例化渲染**：对于相同动画的多个实例，共享 `AnimationClip`
3. **LOD 动画**：远距离时使用简化的动画 clip
4. **GPU Skinning**：使用 `SkinnedMeshRenderer` 进行硬件加速

### 内存管理

1. 使用 `Handle` 而非直接持有数据
2. 及时卸载不再使用的动画资源
3. 复用 `Pose` 对象避免频繁分配

### 调试技巧

1. 使用 `AnimationDebugRenderer` 绘制骨骼和权重
2. 在编辑器中预览动画时间轴
3. 打印状态机参数和当前状态进行调试