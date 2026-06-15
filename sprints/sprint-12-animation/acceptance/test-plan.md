# 测试计划

## 概述

本文档定义 `engine-animation` crate 的完整测试计划，包括单元测试、集成测试和验收测试。测试覆盖所有核心功能模块，确保动画系统的正确性和稳定性。

---

## 测试目标

1. 验证所有核心 API 的正确性
2. 确保数学计算的准确性（插值、混合、IK）
3. 验证状态机逻辑的正确性
4. 确保动画事件在正确时间触发
5. 验证 glTF 加载功能的完整性
6. 确保示例工程可正常运行

---

## 测试范围

| 模块 | 测试类型 | 优先级 |
|------|----------|--------|
| Keyframe / Curve | 单元测试 | P0 |
| Track / AnimationClip | 单元测试 | P0 |
| Pose | 单元测试 | P0 |
| Skeleton / Skin / SkinnedMesh | 单元测试 | P0 |
| Animator | 单元测试 | P0 |
| StateMachine | 单元测试 | P0 |
| Blend1D / Blend2D | 单元测试 | P0 |
| IK | 单元测试 | P0 |
| Ragdoll | 集成测试 | P1 |
| glTF 加载 | 集成测试 | P0 |
| 示例工程 | 验收测试 | P0 |

---

## 测试用例详细设计

### 1. Curve 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `curve_sample_linear_vec3` | 202 | 测试 Curve<Vec3> 线性插值 | 采样结果正确线性插值 |
| `curve_sample_slerp_quat` | 203 | 测试 Curve<Quat> slerp | 采样结果正确球面插值 |
| `curve_sample_linear_f32` | 204 | 测试 Curve<f32> 线性插值 | 采样结果正确线性插值 |
| `curve_sample_with_wrap_loop` | 167 | 测试循环模式采样 | 时间超过 duration 时正确回绕 |
| `curve_duration` | 7, 239 | 测试时长计算 | 返回正确的曲线时长 |
| `curve_optimize` | 10, 244 | 测试曲线优化 | 去除冗余关键帧，误差在阈值内 |
| `curve_cubic_spline` | 170, 427 | 测试三次样条插值 | CubicSpline 切线采样正确 |

### 2. Pose 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `pose_blend` | 27, 239, 476 | 测试姿态混合 | 输出中间姿态 |
| `pose_additive_blend` | 28, 241 | 测试加性混合 | 正确叠加姿态 |
| `pose_local_to_world` | 278, 488 | 测试世界空间计算 | 正确计算每个骨骼的世界矩阵 |
| `pose_identity` | 243 | 测试单位姿态 | 所有骨骼为单位变换 |

### 3. AnimationClip 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `animation_clip_sample` | 18, 260 | 测试动画采样 | 返回正确的 Pose |
| `animation_clip_wrap_mode_loop` | 167, 483 | 测试循环模式 | 时间回绕正确 |
| `animation_clip_events_trigger` | 169, 484 | 测试事件触发 | 在指定时间触发事件 |

### 4. Skeleton 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `skeleton_find_bone_by_name` | 258, 487 | 测试按名称查找骨骼 | 正确返回骨骼索引 |
| `skeleton_bind_pose` | 35, 256 | 测试绑定姿态 | 返回正确的绑定姿态 |
| `skeleton_inverse_bind_matrices` | 257 | 测试逆绑定矩阵 | 返回正确的矩阵数组 |

### 5. SkinnedMesh 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `skinned_mesh_compute_matrix_palette` | 271, 489 | 测试矩阵调色板计算 | 正确计算 skinning 矩阵 |

### 6. StateMachine 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `state_machine_transition_condition` | 165, 477 | 测试条件触发切换 | 条件满足时正确切换状态 |
| `state_machine_entry_state` | 71, 301 | 测试入口状态 | 正确进入初始状态 |
| `state_machine_any_state_transition` | 73, 303 | 测试任意状态过渡 | 可从任意状态过渡 |

### 7. Blend 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `blend_1d_boundary` | 166, 478 | 测试边界值插值 | 边界值时使用边界 clip |
| `blend_2d_bilinear` | 479 | 测试双线性插值 | 正确混合四个方向的 clip |
| `playback_crossfade` | 171, 485 | 测试交叉淡入 | 正确输出混合 pose |

### 8. IK 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `ik_two_bone_ik` | 167, 480 | 测试两骨 IK | 数值计算正确 |
| `ik_ccd_ik` | 481 | 测试 CCD IK | 迭代收敛到目标 |
| `ik_fabrik` | 482 | 测试 FABRIK | 正确求解骨骼位置 |

### 9. AnimationMask 测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `animation_mask_union` | 172, 486 | 测试并集运算 | 正确计算两个遮罩的并集 |
| `animation_mask_intersection` | 135, 358 | 测试交集运算 | 正确计算两个遮罩的交集 |

### 10. glTF 加载测试

| 测试用例 | 需求编号 | 测试描述 | 预期结果 |
|----------|----------|----------|----------|
| `gltf_load_animation_clip` | 405 | 测试加载动画剪辑 | 正确解析并导入为 AnimationClip |
| `gltf_load_skeleton` | 406 | 测试加载骨架 | 正确解析 skeleton |
| `gltf_load_skinned_mesh` | 407 | 测试加载蒙皮网格 | 正确解析 skin 和 weights |

---

## 测试执行计划

### 测试环境

- Rust 版本：最新稳定版
- 操作系统：Windows 10, macOS 12+, Ubuntu 20.04
- 测试框架：`cargo test`

### 测试命令

| 命令 | 用途 |
|------|------|
| `cargo test -p engine-animation` | 运行所有单元测试 |
| `cargo test -p engine-animation -- --test-threads=1` | 单线程运行测试 |
| `cargo test -p engine-animation -- --show-output` | 显示测试输出 |
| `cargo test -p engine-animation -- test_name` | 运行特定测试 |

### 测试流程

1. **代码提交前**：开发者运行 `cargo test -p engine-animation` 确保所有测试通过
2. **CI 集成**：每次代码推送自动运行测试
3. **发布前**：运行完整测试套件（包括 clippy 和 fmt）

---

## 验收标准

### 代码质量

| 检查项 | 标准 | 需求编号 |
|--------|------|----------|
| 单元测试 | `cargo test -p engine-animation` 全部通过 | 490 |
| 静态分析 | `cargo clippy --workspace -- -D warnings` 通过 | 491 |
| 代码格式化 | `cargo fmt --check --workspace` 通过 | 492 |
| 文档生成 | `cargo doc --workspace --no-deps` 成功 | 493 |
| CI 状态 | 三平台 CI green | 494 |

### 示例工程验收

| 示例 | 验收标准 | 需求编号 |
|------|----------|----------|
| `animation_basic` | 循环播放动画 | 464 |
| `animation_state_machine` | 根据输入切换 Idle/Walk/Run/Jump | 465, 214, 502 |
| `animation_blend_1d` | 线性平滑过渡 | 466 |
| `animation_blend_2d` | 2D 混合正确 | 467 |
| `animation_additive` | 叠加动画正确 | 468 |
| `animation_ragdoll` | 角色死亡后切换 ragdoll | 469, 186 |
| `animation_ik` | 两骨 IK 瞄准目标 | 470 |
| `animation_look_at` | 头部看向鼠标 | 471 |
| `animation_retarget` | 同 clip 复用 | 472 |
| `animation_event` | 动画事件触发音效 | 473 |

### 技术指标

| 指标 | 标准 | 需求编号 |
|------|------|----------|
| API 文档覆盖率 | 100% | 499 |
| unsafe 块数量 | <= 3 | 500 |
| 新增示例工程数 | >= 10 | 501 |

---

## 测试工具与框架

- **测试框架**：Rust 内置测试框架（`#[test]` 宏）
- **断言库**：`assert_eq!`, `assert!`, `assert_approx_eq!`
- **模拟工具**：`mockall`（如需要）
- **代码覆盖率**：`cargo-tarpaulin`（可选）

---

## 测试报告

测试完成后生成测试报告，包含：

1. 测试用例总数
2. 通过/失败/跳过数量
3. 测试覆盖率（如启用）
4. 失败测试的详细信息
5. 性能指标（可选）