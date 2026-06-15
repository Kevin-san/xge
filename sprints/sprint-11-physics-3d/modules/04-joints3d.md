# 3D 关节模块

## 模块概述

`Joint3D` 模块提供 3D 物理模拟中的关节约束抽象，支持多种关节类型：Fixed（固定关节）、Revolute（旋转关节，单轴旋转）、Prismatic（棱柱关节，单轴滑动）、Ball（球窝关节，万向）、Distance（距离约束）、Rope（软绳，距离上限）、Spherical（球形关节，带锥限制）。每个关节类型通过对应的 Builder 建造者模式进行配置，支持设置局部锚点、轴向、限制角度、马达等参数。

## 需求编号

对应原需求清单：**132-169, 350-413**

| 编号 | 功能描述 | 优先级 |
|------|----------|--------|
| 132 | `Joint3D`：关节抽象 | P0 |
| 133 | `FixedJointBuilder` | P0 |
| 134 | `RevoluteJointBuilder`：单轴旋转 | P0 |
| 135 | `PrismaticJointBuilder`：单轴滑动 | P0 |
| 136 | `BallJointBuilder`：万向 | P0 |
| 137 | `DistanceJointBuilder`：两点距离约束 | P0 |
| 138 | `RopeJointBuilder`：距离上限（软绳） | P0 |
| 139 | `SphericalJointBuilder`：带锥限制 | P0 |
| 140 | `PhysicsWorld3D::insert_joint(&mut self, body1, body2, joint) -> JointHandle` | P0 |
| 141 | `PhysicsWorld3D::remove_joint(&mut self, handle)` | P0 |
| 350 | `FixedJointBuilder::new()` | P0 |
| 351 | `FixedJointBuilder::local_anchor1(v) -> Self` | P0 |
| 352 | `FixedJointBuilder::local_anchor2(v) -> Self` | P0 |
| 353 | `FixedJointBuilder::local_basis1(q) -> Self` | P0 |
| 354 | `FixedJointBuilder::local_basis2(q) -> Self` | P0 |
| 355 | `FixedJointBuilder::build(&self) -> Joint3D` | P0 |
| 356 | `RevoluteJointBuilder::new(axis)` | P0 |
| 357 | `RevoluteJointBuilder::local_anchor1(v) -> Self` | P0 |
| 358 | `RevoluteJointBuilder::local_anchor2(v) -> Self` | P0 |
| 359 | `RevoluteJointBuilder::motor_model(model) -> Self` | P1 |
| 360 | `RevoluteJointBuilder::limits(min, max) -> Self` | P0 |
| 361 | `RevoluteJointBuilder::motor_velocity(vel, factor) -> Self` | P1 |
| 362 | `RevoluteJointBuilder::motor_position(pos, stiffness, damping) -> Self` | P1 |
| 363 | `PrismaticJointBuilder::new(axis)` | P0 |
| 364 | `PrismaticJointBuilder::limits(min, max) -> Self` | P0 |
| 365 | `BallJointBuilder::new(anchor1, anchor2)` | P0 |
| 366 | `BallJointBuilder::limits(max_angle) -> Self` | P0 |
| 367 | `DistanceJointBuilder::new(anchor1, anchor2)` | P0 |
| 368 | `DistanceJointBuilder::length(l) -> Self` | P0 |
| 369 | `RopeJointBuilder::new(anchor1, anchor2, max_length)` | P0 |
| 370 | `SphericalJointBuilder::with_cone_limit(axis, angle)` | P0 |

## API 签名

### JointHandle 类型

```rust
pub struct JointHandle(/* 内部表示 */);
```

### Joint3D 类型

```rust
pub enum Joint3D {
    Fixed(FixedJoint),
    Revolute(RevoluteJoint),
    Prismatic(PrismaticJoint),
    Ball(BallJoint),
    Distance(DistanceJoint),
    Rope(RopeJoint),
    Spherical(SphericalJoint),
}
```

### FixedJointBuilder

```rust
impl FixedJointBuilder {
    pub fn new() -> Self;
    pub fn local_anchor1(mut self, v: Vec3) -> Self;
    pub fn local_anchor2(mut self, v: Vec3) -> Self;
    pub fn local_basis1(mut self, q: Quat) -> Self;
    pub fn local_basis2(mut self, q: Quat) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### RevoluteJointBuilder

```rust
impl RevoluteJointBuilder {
    pub fn new(axis: Vec3) -> Self;
    pub fn local_anchor1(mut self, v: Vec3) -> Self;
    pub fn local_anchor2(mut self, v: Vec3) -> Self;
    pub fn motor_model(mut self, model: MotorModel) -> Self;
    pub fn limits(mut self, min: f32, max: f32) -> Self;
    pub fn motor_velocity(mut self, target_vel: f32, factor: f32) -> Self;
    pub fn motor_position(mut self, target_pos: f32, stiffness: f32, damping: f32) -> Self;
    pub fn build(&self) -> Joint3D;
}
```

### PrismaticJointBuilder

```rust
impl PrismaticJointBuilder {
    pub fn new(axis: Vec3) -> Self;
    pub fn limits(mut self, min: f32, max: f32) -> Self;
    // 继承 local_anchor1, local_anchor2
}
```

### BallJointBuilder

```rust
impl BallJointBuilder {
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self;
    pub fn limits(mut self, max_angle: f32) -> Self;
}
```

### DistanceJointBuilder

```rust
impl DistanceJointBuilder {
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self;
    pub fn length(mut self, l: f32) -> Self;
}
```

### RopeJointBuilder

```rust
impl RopeJointBuilder {
    pub fn new(anchor1: Vec3, anchor2: Vec3, max_length: f32) -> Self;
}
```

### SphericalJointBuilder

```rust
impl SphericalJointBuilder {
    pub fn with_cone_limit(mut self, axis: Vec3, angle: f32) -> Self;
}
```

### PhysicsWorld3D 关节管理

```rust
pub fn insert_joint(&mut self, body1: RigidBodyHandle, body2: RigidBodyHandle, joint: Joint3D) -> JointHandle;
pub fn remove_joint(&mut self, handle: JointHandle);
```

## 输入/输出

| 方法 | 输入 | 输出 |
|------|------|------|
| `FixedJointBuilder::new()` | - | `FixedJointBuilder` |
| `FixedJointBuilder::build()` | - | `Joint3D` |
| `RevoluteJointBuilder::new(axis)` | `Vec3` | `RevoluteJointBuilder` |
| `RevoluteJointBuilder::limits(min, max)` | `f32`, `f32` | `Self` |
| `insert_joint(body1, body2, joint)` | `RigidBodyHandle`, `RigidBodyHandle`, `Joint3D` | `JointHandle` |
| `remove_joint(handle)` | `JointHandle` | `()` |

## 验收标准

1. `FixedJoint` 连接两个刚体后，它们之间的相对变换保持固定
2. `RevoluteJoint` 限制两个刚体只能绕指定轴旋转
3. `RevoluteJoint::limits` 设置旋转角度限制
4. `PrismaticJoint` 限制两个刚体只能沿指定轴滑动
5. `BallJoint` 允许两个刚体在锚点处万向旋转
6. `BallJoint::limits` 可设置锥角限制
7. `DistanceJoint` 约束两个锚点之间的距离为固定值
8. `DistanceJoint::length` 可设置约束距离
9. `RopeJoint` 限制两个锚点之间的距离不超过最大值
10. `SphericalJoint::with_cone_limit` 设置锥形限制
11. `Joint` 两体被限制距离（需求240）
12. `insert_joint` 正确关联关节与两个刚体
13. `remove_joint` 可移除已存在的关节

## 依赖关系

- **内部依赖**: `RigidBodyHandle`, `JointHandle`
- **外部依赖**: `nalgebra` (Vec3, Quat)
- **被依赖**: `PhysicsWorld3D`

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 结束前完成
- **P1**: 重要功能，应在 Sprint 结束前完成
- **P2**: 增强功能，可延后到后续 Sprint
