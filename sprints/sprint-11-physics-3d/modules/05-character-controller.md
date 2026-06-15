# 角色控制器模块

## 模块概述

`CharacterController3D` 模块提供 3D 游戏中的角色控制器实现，基于 KinematicCharacterController 概念。该模块允许程序化控制角色移动，处理与碰撞体的交互，支持斜坡爬升、斜坡滑动、跳跃、墙壁检测等常见角色控制功能。角色控制器通过 `move_shape` 方法执行移动，该方法会根据碰撞信息返回移动结果，包括实际位移、是否着地、是否碰到天花板、是否碰到墙壁等状态。

## 需求编号

对应原需求清单：**142-154, 170-188, 371-436**

| 编号 | 功能描述 | 优先级 |
|------|----------|--------|
| 142 | `CharacterController3D`：基于 `KinematicCharacterController` 概念 | P0 |
| 143 | `CharacterController3D::new(offset, up_dir, max_slope_climb_angle, max_slide_angle)` | P0 |
| 144 | `CharacterController3D::move_shape(&mut self, dt, desired_translation, body, collider, filter) -> CharacterMovement` | P0 |
| 145 | `CharacterController3D::set_apply_impulse_to_dynamic_bodies(&mut self, b)` | P1 |
| 146 | `CharacterController3D::set_slope_climb_angle(&mut self, angle)` | P0 |
| 147 | `CharacterController3D::set_slide_angle(&mut self, angle)` | P0 |
| 148 | `CharacterController3D::set_offset(&mut self, offset)` | P0 |
| 149 | `CharacterController3D::set_max_distance_to_ground(&mut self, d)` | P1 |
| 150 | `CharacterController3D::set_up(&mut self, v)` | P0 |
| 151 | `CharacterMovement::translation(&self) -> Vec3` | P0 |
| 152 | `CharacterMovement::grounded(&self) -> bool` | P0 |
| 153 | `CharacterMovement::hit_ceil(&self) -> bool` | P0 |
| 154 | `CharacterMovement::hit_wall(&self) -> bool` | P0 |
| 170 | 角色控制器支持可调节参数 | P0 |
| 171 | `CharacterControllerPlugin`：在 ECS 中提供 `character_controller_system` | P0 |
| 172 | 系统在 update 阶段运行，读取 input/velocity 并更新 transform | P0 |
| 173 | 系统自动在 grounded=true 时取消重力（可选配置） | P1 |
| 174 | 系统支持「跳跃」逻辑（基于 grounded 触发） | P0 |
| 175 | 支持可调步高（step_offset） | P1 |
| 176 | 支持动态物体在角色脚下，产生推挤 | P1 |
| 177 | 支持斜坡滑动（高于 slide_angle 时不爬升） | P0 |
| 371 | `CharacterController3D::new(offset, up, max_slope_climb_angle, max_slide_angle)` | P0 |
| 372 | `CharacterController3D::move_shape(dt, desired, body, collider, filter) -> CharacterMovement` | P0 |
| 373 | `CharacterController3D::set_apply_impulse_to_dynamic_bodies(&mut self, b)` | P1 |
| 374 | `CharacterController3D::set_slope_climb_angle(&mut self, angle)` | P0 |
| 375 | `CharacterController3D::set_slide_angle(&mut self, angle)` | P0 |
| 376 | `CharacterController3D::set_offset(&mut self, offset)` | P0 |
| 377 | `CharacterController3D::set_max_distance_to_ground(&mut self, d)` | P1 |
| 378 | `CharacterController3D::set_up(&mut self, v)` | P0 |
| 379 | `CharacterMovement::translation(&self) -> Vec3` | P0 |
| 380 | `CharacterMovement::grounded(&self) -> bool` | P0 |
| 381 | `CharacterMovement::hit_ceil(&self) -> bool` | P0 |
| 382 | `CharacterMovement::hit_wall(&self) -> bool` | P0 |
| 383 | `CharacterMovement::ground_normal(&self) -> Vec3` | P0 |
| 384 | `CharacterControllerPlugin`：在 ECS 中提供 `character_controller_system` | P0 |
| 385 | 系统在 update 阶段运行，读取 input/velocity 并更新 transform | P0 |
| 386 | 系统自动在 grounded=true 时取消重力（可选配置） | P1 |
| 387 | 系统支持「跳跃」逻辑（基于 grounded 触发） | P0 |
| 388 | 支持可调步高（step_offset） | P1 |
| 389 | 支持动态物体在角色脚下，产生推挤 | P1 |
| 390 | 支持斜坡滑动（高于 slide_angle 时不爬升） | P0 |

## API 签名

### CharacterController3D

```rust
pub struct CharacterController3D {
    // 内部状态
}

impl CharacterController3D {
    pub fn new(
        offset: Vec3,
        up_dir: Vec3,
        max_slope_climb_angle: f32,
        max_slide_angle: f32,
    ) -> Self;
    
    pub fn move_shape(
        &mut self,
        dt: f32,
        desired_translation: Vec3,
        body: &mut RigidBody3D,
        collider: &Collider3D,
        filter: QueryFilter,
    ) -> CharacterMovement;
    
    pub fn set_apply_impulse_to_dynamic_bodies(&mut self, b: bool);
    pub fn set_slope_climb_angle(&mut self, angle: f32);
    pub fn set_slide_angle(&mut self, angle: f32);
    pub fn set_offset(&mut self, offset: Vec3);
    pub fn set_max_distance_to_ground(&mut self, d: f32);
    pub fn set_up(&mut self, v: Vec3);
}
```

### CharacterMovement

```rust
pub struct CharacterMovement {
    // 内部状态
}

impl CharacterMovement {
    pub fn translation(&self) -> Vec3;
    pub fn grounded(&self) -> bool;
    pub fn hit_ceil(&self) -> bool;
    pub fn hit_wall(&self) -> bool;
    pub fn ground_normal(&self) -> Vec3;
}
```

### CharacterControllerPlugin (ECS)

```rust
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App);
}
```

### 系统参数

```rust
pub struct CharacterControllerComponent {
    pub controller: CharacterController3D,
    pubcollider: ColliderHandle,
}

pub fn character_controller_system(world: &mut World);
```

## 输入/输出

| 方法 | 输入 | 输出 |
|------|------|------|
| `CharacterController3D::new(offset, up_dir, max_slope_climb_angle, max_slide_angle)` | `Vec3`, `Vec3`, `f32`, `f32` | `CharacterController3D` |
| `move_shape(dt, desired, body, collider, filter)` | `f32`, `Vec3`, `&mut RigidBody3D`, `&Collider3D`, `QueryFilter` | `CharacterMovement` |
| `CharacterMovement::translation()` | - | `Vec3` |
| `CharacterMovement::grounded()` | - | `bool` |
| `CharacterMovement::ground_normal()` | - | `Vec3` |

## 验收标准

1. 角色可在平地上正常行走
2. 角色可跳跃（基于 `grounded` 状态触发）
3. 角色在斜坡上站立时，若坡度小于 `max_slope_climb_angle` 可站立
4. 角色在斜坡上滑动时，若坡度大于 `max_slide_angle` 不爬升
5. `hit_ceil` 在头部碰到障碍物时返回 `true`
6. `hit_wall` 在侧面碰到障碍物时返回 `true`
7. `ground_normal` 返回地面法向量
8. `move_shape` 返回的实际 `translation` 考虑碰撞后的滑动
9. `set_up` 改变角色 "上" 方向（用于失重环境）
10. `CharacterController` 在斜坡上可站立（需求227）
11. `examples/physics_3d_character` 角色可在斜坡上站立、滑行（需求259）
12. 与 `RigidBody3D` 集成，修改 kinematic 刚体位置

## 依赖关系

- **内部依赖**: `RigidBody3D`, `Collider3D`, `QueryFilter`
- **外部依赖**: `nalgebra` (Vec3)
- **被依赖**: `PhysicsModule` (ECS 集成)

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 结束前完成
- **P1**: 重要功能，应在 Sprint 结束前完成
- **P2**: 增强功能，可延后到后续 Sprint
