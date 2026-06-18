# 3D 刚体模块

## 模块概述

`RigidBody3D` 模块提供 3D 物理模拟中的刚体抽象，支持五种刚体类型：Dynamic（动态）、Static（静态）、KinematicPositionBased（位置驱动运动学）、KinematicVelocityBased（速度驱动运动学）和 Fixed（固定）。该模块通过 `RigidBodyBuilder` 建造者模式提供流畅的刚体配置接口，支持设置质量、阻尼、重力系数、CCD、力/冲量应用等物理属性。

## 需求编号

对应原需求清单：**25-115, 268-358**

| 编号 | 功能描述 | 优先级 |
|------|----------|--------|
| 25 | `RigidBody3D::Dynamic / Static / KinematicPositionBased / KinematicVelocityBased / Fixed` | P0 |
| 26 | `RigidBodyBuilder::dynamic() -> Self` | P0 |
| 27 | `RigidBodyBuilder::static_() -> Self` | P0 |
| 28 | `RigidBodyBuilder::kinematic_position_based() -> Self` | P0 |
| 29 | `RigidBodyBuilder::kinematic_velocity_based() -> Self` | P0 |
| 30 | `RigidBodyBuilder::fixed() -> Self` | P0 |
| 31 | `RigidBodyBuilder::translation(v) -> Self` | P0 |
| 32 | `RigidBodyBuilder::rotation(q) -> Self` | P0 |
| 33 | `RigidBodyBuilder::linvel(v) -> Self` | P0 |
| 34 | `RigidBodyBuilder::angvel(v) -> Self` | P0 |
| 35 | `RigidBodyBuilder::mass(v) -> Self` | P0 |
| 36 | `RigidBodyBuilder::mass_properties(mp) -> Self` | P1 |
| 37 | `RigidBodyBuilder::center_of_mass(v) -> Self` | P1 |
| 38 | `RigidBodyBuilder::principal_inertia(v) -> Self` | P1 |
| 39 | `RigidBodyBuilder::linear_damping(v) -> Self` | P0 |
| 40 | `RigidBodyBuilder::angular_damping(v) -> Self` | P0 |
| 41 | `RigidBodyBuilder::gravity_scale(v) -> Self` | P0 |
| 42 | `RigidBodyBuilder::ccd_enabled(b) -> Self` | P1 |
| 43 | `RigidBodyBuilder::sleeping(b) -> Self` | P1 |
| 44 | `RigidBodyBuilder::dominance_group(i8) -> Self` | P2 |
| 45 | `RigidBodyBuilder::additional_mass(v) -> Self` | P2 |
| 46 | `RigidBodyBuilder::lock_translations() -> Self` | P0 |
| 47 | `RigidBodyBuilder::lock_rotations() -> Self` | P0 |
| 48 | `RigidBodyBuilder::restrict_rotations(x, y, z) -> Self` | P1 |
| 49 | `RigidBodyBuilder::build(&self) -> RigidBody3D` | P0 |
| 50 | `RigidBodyHandle`（类型句柄） | P0 |
| 51 | `PhysicsWorld3D::insert_body(&mut self, body) -> RigidBodyHandle` | P0 |
| 52 | `PhysicsWorld3D::remove_body(&mut self, handle)` | P0 |
| 53 | `PhysicsWorld3D::body(&self, handle) -> &RigidBody3D` | P0 |
| 54 | `PhysicsWorld3D::body_mut(&mut self, handle) -> &mut RigidBody3D` | P0 |
| 55 | `RigidBody3D::type(&self) -> RigidBodyType` | P0 |
| 56 | `RigidBody3D::position(&self) -> Vec3` | P0 |
| 57 | `RigidBody3D::rotation(&self) -> Quat` | P0 |
| 58 | `RigidBody3D::transform(&self) -> (Vec3, Quat)` | P0 |
| 59 | `RigidBody3D::set_translation(&mut self, v, wake)` | P0 |
| 60 | `RigidBody3D::set_rotation(&mut self, q, wake)` | P0 |
| 61 | `RigidBody3D::set_position(&mut self, v, q, wake)` | P0 |
| 62 | `RigidBody3D::linvel(&self) -> Vec3` | P0 |
| 63 | `RigidBody3D::angvel(&self) -> Vec3` | P0 |
| 64 | `RigidBody3D::set_linvel(&mut self, v, wake)` | P0 |
| 65 | `RigidBody3D::set_angvel(&mut self, v, wake)` | P0 |
| 66 | `RigidBody3D::mass(&self) -> f32` | P0 |
| 67 | `RigidBody3D::set_mass(&mut self, mass)` | P0 |
| 68 | `RigidBody3D::apply_force(&mut self, force, wake)` | P0 |
| 69 | `RigidBody3D::apply_force_at_point(&mut self, force, point, wake)` | P0 |
| 70 | `RigidBody3D::apply_torque(&mut self, torque, wake)` | P0 |
| 71 | `RigidBody3D::apply_impulse(&mut self, impulse, wake)` | P0 |
| 72 | `RigidBody3D::apply_impulse_at_point(&mut self, impulse, point, wake)` | P0 |
| 73 | `RigidBody3D::apply_torque_impulse(&mut self, torque_impulse, wake)` | P0 |
| 74 | `RigidBody3D::linear_damping(&self) -> f32` | P0 |
| 75 | `RigidBody3D::set_linear_damping(&mut self, v)` | P0 |
| 76 | `RigidBody3D::angular_damping(&self) -> f32` | P0 |
| 77 | `RigidBody3D::set_angular_damping(&mut self, v)` | P0 |
| 78 | `RigidBody3D::gravity_scale(&self) -> f32` | P0 |
| 79 | `RigidBody3D::set_gravity_scale(&mut self, v)` | P0 |
| 80 | `RigidBody3D::is_sleeping(&self) -> bool` | P1 |
| 81 | `RigidBody3D::wake_up(&mut self, strong)` | P1 |
| 82 | `RigidBody3D::sleep(&mut self)` | P1 |
| 83 | `RigidBody3D::ccd_enabled(&self) -> bool` | P1 |
| 84 | `RigidBody3D::enable_ccd(&mut self, b)` | P1 |
| 85 | `RigidBody3D::is_dynamic(&self) -> bool` | P0 |
| 86 | `RigidBody3D::is_static(&self) -> bool` | P0 |
| 87 | `RigidBody3D::is_kinematic(&self) -> bool` | P0 |
| 268 | `RigidBodyBuilder::dynamic()` | P0 |
| 269 | `RigidBodyBuilder::static_()` | P0 |
| 270 | `RigidBodyBuilder::kinematic_position_based()` | P0 |
| 271 | `RigidBodyBuilder::kinematic_velocity_based()` | P0 |
| 272 | `RigidBodyBuilder::fixed()` | P0 |
| 273 | `RigidBodyBuilder::translation(v) -> Self` | P0 |
| 274 | `RigidBodyBuilder::rotation(q) -> Self` | P0 |
| 275 | `RigidBodyBuilder::linvel(v) -> Self` | P0 |
| 276 | `RigidBodyBuilder::angvel(v) -> Self` | P0 |
| 277 | `RigidBodyBuilder::mass(v) -> Self` | P0 |
| 278 | `RigidBodyBuilder::mass_properties(mp) -> Self` | P1 |
| 279 | `RigidBodyBuilder::center_of_mass(v) -> Self` | P1 |
| 280 | `RigidBodyBuilder::principal_inertia(v) -> Self` | P1 |
| 281 | `RigidBodyBuilder::linear_damping(v) -> Self` | P0 |
| 282 | `RigidBodyBuilder::angular_damping(v) -> Self` | P0 |
| 283 | `RigidBodyBuilder::gravity_scale(v) -> Self` | P0 |
| 284 | `RigidBodyBuilder::ccd_enabled(b) -> Self` | P1 |
| 285 | `RigidBodyBuilder::sleeping(b) -> Self` | P1 |
| 286 | `RigidBodyBuilder::dominance_group(g) -> Self` | P2 |
| 287 | `RigidBodyBuilder::additional_mass(v) -> Self` | P2 |
| 288 | `RigidBodyBuilder::lock_translations() -> Self` | P0 |
| 289 | `RigidBodyBuilder::lock_rotations() -> Self` | P0 |
| 290 | `RigidBodyBuilder::restrict_rotations(x, y, z) -> Self` | P1 |
| 291 | `RigidBodyBuilder::build(&self) -> RigidBody3D` | P0 |
| 292 | `RigidBodyType::Dynamic / Static / KinematicPositionBased / KinematicVelocityBased / Fixed` | P0 |
| 293 | `RigidBody3D::type(&self) -> RigidBodyType` | P0 |
| 294 | `RigidBody3D::position(&self) -> Vec3` | P0 |
| 295 | `RigidBody3D::rotation(&self) -> Quat` | P0 |
| 296 | `RigidBody3D::set_translation(&mut self, v, wake)` | P0 |
| 297 | `RigidBody3D::set_rotation(&mut self, q, wake)` | P0 |
| 298 | `RigidBody3D::linvel(&self) -> Vec3` | P0 |
| 299 | `RigidBody3D::set_linvel(&mut self, v, wake)` | P0 |
| 300 | `RigidBody3D::angvel(&self) -> Vec3` | P0 |
| 301 | `RigidBody3D::set_angvel(&mut self, v, wake)` | P0 |
| 302 | `RigidBody3D::mass(&self) -> f32` | P0 |
| 303 | `RigidBody3D::apply_force(&mut self, f, wake)` | P0 |
| 304 | `RigidBody3D::apply_force_at_point(&mut self, f, p, wake)` | P0 |
| 305 | `RigidBody3D::apply_torque(&mut self, t, wake)` | P0 |
| 306 | `RigidBody3D::apply_impulse(&mut self, i, wake)` | P0 |
| 307 | `RigidBody3D::apply_impulse_at_point(&mut self, i, p, wake)` | P0 |
| 308 | `RigidBody3D::apply_torque_impulse(&mut self, t, wake)` | P0 |
| 309 | `RigidBody3D::linear_damping(&self) -> f32` | P0 |
| 310 | `RigidBody3D::set_linear_damping(&mut self, v)` | P0 |
| 311 | `RigidBody3D::angular_damping(&self) -> f32` | P0 |
| 312 | `RigidBody3D::set_angular_damping(&mut self, v)` | P0 |
| 313 | `RigidBody3D::gravity_scale(&self) -> f32` | P0 |
| 314 | `RigidBody3D::set_gravity_scale(&mut self, v)` | P0 |
| 315 | `RigidBody3D::is_sleeping(&self) -> bool` | P1 |
| 316 | `RigidBody3D::wake_up(&mut self, strong)` | P1 |
| 317 | `RigidBody3D::sleep(&mut self)` | P1 |
| 318 | `RigidBody3D::ccd_enabled(&self) -> bool` | P1 |
| 319 | `RigidBody3D::enable_ccd(&mut self, b)` | P1 |

## API 签名

### RigidBodyType 枚举

```rust
pub enum RigidBodyType {
    Dynamic,
    Static,
    KinematicPositionBased,
    KinematicVelocityBased,
    Fixed,
}
```

### RigidBodyHandle 类型

```rust
pub struct RigidBodyHandle(/* 内部表示 */);
```

### RigidBodyBuilder 建造者

```rust
impl RigidBodyBuilder {
    pub fn dynamic() -> Self;
    pub fn static_() -> Self;
    pub fn kinematic_position_based() -> Self;
    pub fn kinematic_velocity_based() -> Self;
    pub fn fixed() -> Self;
    
    pub fn translation(mut self, v: Vec3) -> Self;
    pub fn rotation(mut self, q: Quat) -> Self;
    pub fn linvel(mut self, v: Vec3) -> Self;
    pub fn angvel(mut self, v: Vec3) -> Self;
    pub fn mass(mut self, m: f32) -> Self;
    pub fn mass_properties(mut self, mp: MassProperties) -> Self;
    pub fn center_of_mass(mut self, v: Vec3) -> Self;
    pub fn principal_inertia(mut self, v: Vec3) -> Self;
    pub fn linear_damping(mut self, v: f32) -> Self;
    pub fn angular_damping(mut self, v: f32) -> Self;
    pub fn gravity_scale(mut self, v: f32) -> Self;
    pub fn ccd_enabled(mut self, b: bool) -> Self;
    pub fn sleeping(mut self, b: bool) -> Self;
    pub fn dominance_group(mut self, g: i8) -> Self;
    pub fn additional_mass(mut self, v: f32) -> Self;
    pub fn lock_translations(mut self) -> Self;
    pub fn lock_rotations(mut self) -> Self;
    pub fn restrict_rotations(mut self, x: bool, y: bool, z: bool) -> Self;
    
    pub fn build(&self) -> RigidBody3D;
}
```

### RigidBody3D 实例方法

```rust
// 类型查询
pub fn type(&self) -> RigidBodyType;
pub fn is_dynamic(&self) -> bool;
pub fn is_static(&self) -> bool;
pub fn is_kinematic(&self) -> bool;

// 变换
pub fn position(&self) -> Vec3;
pub fn rotation(&self) -> Quat;
pub fn transform(&self) -> (Vec3, Quat);
pub fn set_translation(&mut self, v: Vec3, wake: bool);
pub fn set_rotation(&mut self, q: Quat, wake: bool);
pub fn set_position(&mut self, v: Vec3, q: Quat, wake: bool);

// 速度
pub fn linvel(&self) -> Vec3;
pub fn angvel(&self) -> Vec3;
pub fn set_linvel(&mut self, v: Vec3, wake: bool);
pub fn set_angvel(&mut self, v: Vec3, wake: bool);

// 质量
pub fn mass(&self) -> f32;
pub fn set_mass(&mut self, mass: f32);

// 力与冲量
pub fn apply_force(&mut self, force: Vec3, wake: bool);
pub fn apply_force_at_point(&mut self, force: Vec3, point: Vec3, wake: bool);
pub fn apply_torque(&mut self, torque: Vec3, wake: bool);
pub fn apply_impulse(&mut self, impulse: Vec3, wake: bool);
pub fn apply_impulse_at_point(&mut self, impulse: Vec3, point: Vec3, wake: bool);
pub fn apply_torque_impulse(&mut self, torque_impulse: Vec3, wake: bool);

// 阻尼
pub fn linear_damping(&self) -> f32;
pub fn set_linear_damping(&mut self, v: f32);
pub fn angular_damping(&self) -> f32;
pub fn set_angular_damping(&mut self, v: f32);

// 重力
pub fn gravity_scale(&self) -> f32;
pub fn set_gravity_scale(&mut self, v: f32);

// 睡眠状态
pub fn is_sleeping(&self) -> bool;
pub fn wake_up(&mut self, strong: bool);
pub fn sleep(&mut self);

// CCD
pub fn ccd_enabled(&self) -> bool;
pub fn enable_ccd(&mut self, b: bool);
```

## 输入/输出

| 方法 | 输入 | 输出 |
|------|------|------|
| `RigidBodyBuilder::dynamic()` | - | `RigidBodyBuilder` |
| `RigidBodyBuilder::build()` | - | `RigidBody3D` |
| `position()` | - | `Vec3` |
| `rotation()` | - | `Quat` |
| `set_translation(v, wake)` | `Vec3`, `bool` | `()` |
| `apply_force(force, wake)` | `Vec3`, `bool` | `()` |
| `apply_impulse(impulse, wake)` | `Vec3`, `bool` | `()` |

## 验收标准

1. 创建 Dynamic 刚体并 `step` 后，受重力影响下落
2. 创建 Static 刚体后，位置不受 `step` 影响
3. `apply_force` 会改变 `linvel`（需求303）
4. `apply_impulse` 会立即改变 `linvel`（需求306）
5. `set_translation` 可直接设置刚体位置
6. `lock_translations()` 阻止刚体在指定轴上移动
7. `lock_rotations()` 阻止刚体在指定轴上旋转
8. `is_sleeping()` 在刚体静止一段时间后返回 `true`
9. `wake_up(true)` 可唤醒睡眠中的刚体
10. `ccd_enabled` 的刚体可命中高速小物体（需求229）
11. 重力（step 后 linvel.z == g * dt）（需求208）

## 依赖关系

- **内部依赖**: 无
- **外部依赖**: `nalgebra` (Vec3, Quat)
- **被依赖**: `PhysicsWorld3D`, `Collider3D`, `CharacterController3D`

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 结束前完成
- **P1**: 重要功能，应在 Sprint 结束前完成
- **P2**: 增强功能，可延后到后续 Sprint
