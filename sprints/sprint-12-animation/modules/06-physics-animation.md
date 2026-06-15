# 物理动画模块 (Physics Animation / Ragdoll)

## 模块概述

物理动画模块负责将骨骼系统与物理引擎集成，实现 ragdoll（布娃娃）效果。本模块支持将 skeleton 映射为物理关节，实现物理驱动的动画，并支持动画与物理状态的同步。

## 需求编号与功能描述

### Ragdoll 布娃娃系统

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 107 | Ragdoll 结构 | `Ragdoll` | 把 skeleton 映射为物理 joint | - |
| 108 | 激活 Ragdoll | `Ragdoll::activate(&self, world, entities) -> RagdollHandle` | &mut World, &[Entity] | RagdollHandle |
| 109 | 烘焙姿态 | `Ragdoll::bake(&self, world, pose) -> Pose` | &World, &Pose | Pose |
| 110 | 同步到动画 | `Ragdoll::sync_ragdoll_to_animation(&self, world, entities) -> Pose` | &World, &[Entity] | Pose |
| 111 | RagdollBuilder | 设置每段骨骼的物理 collider/joint | - | - |
| 397 | 创建 RagdollBuilder | `Ragdoll::new(skeleton) -> RagdollBuilder` | &Skeleton | RagdollBuilder |
| 398 | 配置骨骼物理 | `RagdollBuilder::bone(&mut self, idx, collider, joint_type)` | usize, Collider, RagdollJointType | - |
| 399 | 构建 Ragdoll | `RagdollBuilder::build(&self, world) -> Ragdoll` | &mut World | Ragdoll |
| 400 | 激活 | `Ragdoll::activate(&mut self, world)` | &mut World | - |
| 401 | 停用 | `Ragdoll::deactivate(&mut self, world)` | &mut World | - |
| 402 | 烘焙姿态 | `Ragdoll::bake_pose(&self, world) -> Pose` | &World | Pose |
| 403 | 是否激活 | `Ragdoll::is_active(&self) -> bool` | - | bool |

### RagdollJointType 关节类型

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 404 | 关节类型枚举 | `RagdollJointType::Ball / Revolute / Fixed` | - | 枚举 |

## 验收标准

- [ ] `examples/animation_ragdoll` 角色死亡后切换 ragdoll（需求 186, 469）
- [ ] Ragdoll 正确映射骨骼到物理关节
- [ ] Ragdoll::bake_pose 正确从物理状态烘焙为 Pose
- [ ] Ragdoll 激活/停用状态切换正确

## 依赖关系

- 依赖 `Skeleton`（02-skeleton.md）
- 依赖 `Pose`（01-animation-clip.md）
- 依赖物理引擎（如 Rapier 或 PhysX）
- 被动画系统使用

## 优先级

**P0（核心）：**
- RagdollBuilder 构建器
- Ragdoll 激活/停用
- 物理状态烘焙为 Pose

**P1（重要）：**
- 多种关节类型支持
- 动画与物理状态同步
- Ragdoll 事件触发

**P2（增强）：**
- 高级物理约束
- 碰撞层配置
- 性能优化