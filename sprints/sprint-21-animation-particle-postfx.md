# Sprint 21 · 动画 + 粒子 + 后处理（Animation / Particle / PostFX / SSAO）

> 文档编号: `sprint-21-animation-particle-postfx.md / v1.0
> 周期: 4 周 (20 个工作日)
> 上游依赖: Sprint 17 (Math), Sprint 18 (ECS), Sprint 19 (Render), Sprint 20 (Physics)
> 下游交付: Sprint 22 (Asset / Editor)

---

## 1. 目标与范围

**目标：** 实现 **角色动画系统**（骨骼蒙皮 / 状态机 / 混合树 / IK）、**GPU 粒子系统**（Niagara 风格数据驱动）、**后处理效果**（Bloom / DoF / SSAO / SSR / 色彩分级）。

**范围：**
- ✅ 骨骼动画 / Skeleton / Skinned Mesh
- ✅ Dual Quat 蒙皮（Sprint 17 集成）
- ✅ AnimationClip 关键帧 / 曲线插值
- ✅ 状态机（State Machine）+ 混合树（Blend Tree）
- ✅ IK：FABRIK / CCD / Two-Bone
- ✅ GPU 粒子系统：发射器 / 寿命 / 颜色 / 大小 / 力场
- ✅ 屏幕空间效果：Bloom / DoF / SSAO / SSR
- ✅ 色彩分级：Lift/Gamma/Gain
- ⛔ 不含：毛发（Hair）/ 布料（Cloth）/ 流体 / Niagara 完整版

**核心参考：** O3DE EMotionFX / UE Animation Blueprint / Godot AnimationPlayer / Niagara。

---

## 2. 上游需求对接

| 来源 | 关联章节 | 承接 |
|------|---------|------|
| [NEXT_PHASE_REQUIREMENTS.md § 8.1](../NEXT_PHASE_REQUIREMENTS.md) | 动画系统 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 8.2](../NEXT_PHASE_REQUIREMENTS.md) | 粒子系统 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 8.3](../NEXT_PHASE_REQUIREMENTS.md) | 后处理 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 5 M5](../NEXT_PHASE_REQUIREMENTS.md) | Milestone M5 部分 | ✅ 本 sprint |

---

## 3. 子模块拆分

### 3.1 [01-animation-skeleton.md](modules/01-animation-skeleton.md) — 骨骼与 Skinned Mesh

**核心交付：**
- `engine-anim/src/skeleton.rs`
  - `Skeleton { bones: Vec<Bone>, inverse_bind_poses: Vec<Mat4> }`
  - `Bone { name, parent_index, local_transform }`
  - `local_to_world()` 累积父级变换
- `engine-anim/src/skinning/mod.rs`
  - `SkinningMode::Linear` (LBS)
  - `SkinningMode::DualQuaternion` (DLB) — **使用 sprint-17 DualQuat**
  - GPU 蒙皮 shader 集成
- `engine-anim/src/skinned_mesh.rs`
  - `SkinnedMesh { mesh, skeleton, vertex_weights }`
  - 4 骨骼权重 / 顶点（GPU 蒙皮）

**验收：**
- 100 骨骼蒙皮 < 0.5 ms GPU
- DLB vs LBS 视觉差异 < 1%（无 candy-wrapper 形变）
- 蒙皮精度测试：球套球动画

---

### 3.2 [02-animation-clip.md](modules/02-animation-clip.md) — AnimationClip / Curve

**核心交付：**
- `engine-anim/src/clip.rs`
  - `AnimationClip { tracks: Vec<Track>, duration, fps }`
  - `Track { bone_name, position_keys, rotation_keys, scale_keys }`
  - `Keyframe { time, value }` 时间 → 值
- `engine-anim/src/curve.rs`
  - `Curve<T>` 通用关键帧曲线
  - 插值器：`Linear`, `Bezier`（三次贝塞尔）, `Hermite`, `Step`
  - 平滑切线（Tangent）自动计算
- `engine-anim/src/sample.rs`
  - `clip.sample(time) -> Pose`
  - 时间 wrap / ping-pong / clamp

**验收：**
- 100 骨骼 60 FPS 关键帧采样 < 0.1 ms CPU
- 曲线插值：贝塞尔与 Unity 动画曲线对比一致
- 动画导入 GLTF / FBX

---

### 3.3 [03-state-machine-blend-tree.md](modules/03-state-machine-blend-tree.md) — 状态机 + 混合树

**核心交付：**
- `engine-anim/src/state_machine.rs`
  - `StateMachine { states: Vec<State>, transitions: Vec<Transition> }`
  - `State { clip, speed, loop_mode }`
  - `Transition { from, to, condition, blend_duration, blend_curve }`
  - `Condition { parameter, comparison, value }`
- `engine-anim/src/blend_tree.rs`
  - `BlendTree` 节点：ClipNode / BlendNode(2-way, 1D, 2D) / AdditiveNode / SlotNode
  - 1D Blend（speed 维度）
  - 2D Blend（locomotion：x = direction, y = speed）
- `engine-anim/src/parameters.rs`
  - `AnimationParameter { f32, bool, trigger }` 实时更新

**验收：**
- 状态机切换：100 状态拓扑 < 1 ms
- 混合树：5 clip blend 0.1 ms
- 状态转换视觉无跳变

---

### 3.4 [04-ik-system.md](modules/04-ik-system.md) — IK 求解器

**核心交付：**
- `engine-anim/src/ik/fabrik.rs`
  - **FABRIK**（前向-后向 IK）
  - 位置约束、关节限制
- `engine-anim/src/ik/ccd.rs`
  - **Cyclic Coordinate Descent**
- `engine-anim/src/ik/two_bone.rs`
  - **Two-Bone IK**（腿 / 臂）
  - 极坐标解析解
- `engine-anim/src/ik/look_at.rs`
  - Look-At 约束
- `engine-anim/src/ik/control.rs`
  - 外部目标驱动

**验收：**
- FABRIK 30 关节链 5 迭代 < 0.2 ms
- Two-bone IK 解析解 < 1 µs
- 关节限制 + IK 兼容

---

### 3.5 [05-particle-system.md](modules/05-particle-system.md) — 粒子系统

**核心交付：**
- `engine-particle/src/system/mod.rs`
  - `ParticleSystem { emitters: Vec<Emitter>, modules: Vec<Module> }`
  - GPU 模拟（Compute Shader）+ 顶点缓冲输出
- `engine-particle/src/emitter.rs`
  - `Emitter { shape, rate, duration, looping }`
  - 形状：Point / Sphere / Cone / Box / Mesh Surface
- `engine-particle/src/module/mod.rs`
  - `Module` trait
  - 实现：`Init`（位置/速度/颜色/大小）, `Update`（力场/碰撞/颜色/大小）, `Render`（sprite/mesh）
  - 模块可组合（Niagara 思路）
- `engine-particle/src/force.rs`
  - `Gravity`, `Wind`, `Vortex`, `Turbulence` 力场
- `engine-particle/src/render.rs`
  - Sprite / Mesh 粒子渲染
  - Soft Particle（深度融合）

**验收：**
- 10000 GPU 粒子 60 FPS
- 100 个发射器混合 < 0.5 ms CPU
- 4 种力场同时作用视觉正确

---

### 3.6 [06-post-processing.md](modules/06-post-processing.md) — 后处理效果

**核心交付：**
- `engine-postfx/src/bloom.rs`
  - **Bloom**：HDR → 阈值 → 高斯模糊 → 合成
  - 5 级 downsample
- `engine-postfx/src/dof.rs`
  - **Depth of Field**：CoC 计算 → Bokeh 散景
  - 近 / 远 / 焦距三段
- `engine-postfx/src/ssao.rs`
  - **SSAO**：深度 + 法线 → AO 采样 → 模糊
  - SSAO / HBAO+ / GTAO 选择
- `engine-postfx/src/ssr.rs`
  - **Screen Space Reflection**：Hi-Z 加速
  - Roughness 退化为模糊
- `engine-postfx/src/color_grading.rs`
  - **色彩分级**：Lift / Gamma / Gain / Saturation
  - LUT（Look Up Table）应用

**验收：**
- Bloom 5 级模糊 < 0.5 ms GPU @ 1080p
- SSAO 16 采样 < 0.5 ms GPU
- SSR 64 步 < 1 ms GPU
- 色彩分级 LUT 256³ 应用 < 0.1 ms GPU

---

## 4. 验收清单（acceptance/test-plan.md）

- [ ] 100 骨骼蒙皮 < 0.5 ms GPU
- [ ] 100 骨骼 60 FPS 关键帧采样 < 0.1 ms CPU
- [ ] 状态机 100 状态切换 < 1 ms
- [ ] FABRIK 30 关节链 < 0.2 ms
- [ ] 10000 GPU 粒子 60 FPS
- [ ] Bloom 5 级 < 0.5 ms
- [ ] SSAO 16 采样 < 0.5 ms
- [ ] SSR 64 步 < 1 ms
- [ ] `cargo test -p engine-anim -p engine-particle -p engine-postfx` 全通过
- [ ] `cargo bench` 基准存档
- [ ] 示例：`character_anim_demo`, `particle_niagara`, `ssr_demo`, `color_grading`

---

## 5. API 稳定承诺

```rust
// engine-anim
pub use skeleton::Skeleton;
pub use clip::AnimationClip;
pub use state_machine::AnimationStateMachine;
pub use blend_tree::BlendTree;
pub use ik::{FabrikSolver, CcdSolver, TwoBoneIk};
pub use skinning::SkinningMode;

// engine-particle
pub use system::ParticleSystem;
pub use emitter::EmitterShape;
pub use module::ParticleModule;
pub use force::ForceField;

// engine-postfx
pub use bloom::BloomPass;
pub use dof::DofPass;
pub use ssao::SsaoPass;
pub use ssr::SsrPass;
pub use color_grading::ColorGradingLut;
```

---

## 6. 与上下游依赖

| 依赖 | 来自 | 用途 |
|------|------|------|
| `DualQuat` | sprint-17 | DLB 蒙皮 |
| `World`, `Query` | sprint-18 | 实体驱动 |
| `HdrTarget` | sprint-19 | 后处理输入 |
| `GpuDevice`, `CommandList` | sprint-19 | GPU 调度 |
| `PbrMaterial` | sprint-19 | 粒子材质 |
| `RigidBody` | sprint-20 | 物理驱动布娃娃 |

---

## 7. 风险与缓解

| 风险 | 等级 | 缓解 |
|------|------|------|
| 蒙皮 GPU 蒙皮 LOD | 中 | 远距离降级到 GPU 简化蒙皮 |
| 状态机可视化调试 | 中 | Editor 时支持（sprint-22） |
| GPU 粒子 Web 后端 | 高 | 优先桌面 GL/Vulkan |
| SSAO 闪烁 | 中 | 抖动 + TAA 缓解 |
| SSR 边缘噪声 | 中 | 边缘 fallback 到环境反射 |
