# 关节系统（Joint2D）模块需求

## 模块概述

关节（Joint2D）用于约束两个刚体之间的相对运动，实现物理连接。本模块提供距离关节、铰链关节、焊接关节、滑动关节、弹簧关节、驱动关节六种关节类型。

---

## 需求清单

### 1. Joint 抽象接口

| 编号 | 需求 | 描述 |
|------|------|------|
| 81 | `Joint2D` 抽象 trait | 所有关节的公共接口 |
| 264 | `Joint::body_a(&self) -> BodyHandle` | 获取关节连接的刚体 A |
| 265 | `Joint::body_b(&self) -> BodyHandle` | 获取关节连接的刚体 B |

### 2. 距离关节（DistanceJoint）

| 编号 | 需求 | 描述 |
|------|------|------|
| 109 | `DistanceJoint` | 距离关节实现 |
| 251 | `DistanceJointBuilder::new(body_a, body_b, local_a, local_b)` | 构造器 |
| 252 | `DistanceJointBuilder::length(f)` | 设置静止长度 |
| 253 | `DistanceJointBuilder::stiffness(f)` | 刚度（硬度） |
| 254 | `DistanceJointBuilder::damping(f)` | 阻尼 |

### 3. 铰链关节（RevoluteJoint）

| 编号 | 需求 | 描述 |
|------|------|------|
| 110 | `RevoluteJoint`（铰链） | 铰链/旋转关节 |
| 255 | `RevoluteJointBuilder::new(body_a, body_b, anchor)` | 构造器 |
| 256 | `RevoluteJointBuilder::limits(min, max)` | 旋转角度限制 |
| 257 | `RevoluteJointBuilder::motor(velocity, max_torque)` | 马达配置 |

### 4. 滑动关节（PrismaticJoint）

| 编号 | 需求 | 描述 |
|------|------|------|
| 112 | `PrismaticJoint`（滑动） | 滑动/棱柱关节 |
| 258 | `PrismaticJointBuilder::new(body_a, body_b, anchor, axis)` | 构造器（锚点+轴） |
| 259 | `PrismaticJointBuilder::limits(min, max)` | 滑动距离限制 |
| 260 | `PrismaticJointBuilder::motor(velocity, max_force)` | 马达配置 |

### 5. 焊接关节（WeldJoint）

| 编号 | 需求 | 描述 |
|------|------|------|
| 111 | `WeldJoint`（焊接） | 焊接关节，固定两点 |
| 261 | `WeldJointBuilder::new(body_a, body_b, local_a, local_b)` | 构造器 |

### 6. 弹簧关节（SpringJoint）

| 编号 | 需求 | 描述 |
|------|------|------|
| 113 | `SpringJoint` | 弹簧关节 |
| 262 | `SpringJointBuilder::new(body_a, body_b, anchor_a, anchor_b)` | 构造器 |
| 263 | `SpringJointBuilder::stiffness(f) / damping(f) / rest_length(f)` | 弹簧参数 |

### 7. 驱动关节（MotorJoint）

| 编号 | 需求 | 描述 |
|------|------|------|
| 114 | `MotorJoint`（驱动） | 驱动关节，可控制相对运动 |
| （使用通用 Builder 模式） | 关节配置方法 | motor_velocity / motor_max_force |

### 8. World2D 关节管理

| 编号 | 需求 | 描述 |
|------|------|------|
| 266 | `World2D::insert_joint(joint) -> JointHandle` | 插入关节 |
| 267 | `World2D::remove_joint(handle)` | 移除关节 |
| 268 | `World2D::joints(&self) -> 迭代器` | 遍历关节 |

---

## API 签名

### Joint2D Trait

```rust
pub trait Joint2D {
    fn body_a(&self) -> BodyHandle;
    fn body_b(&self) -> BodyHandle;
    fn constraint(&self) -> &Constraint;  // 内部约束表达
}
```

### DistanceJoint

```rust
pub struct DistanceJoint {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    length: f32,
    stiffness: f32,
    damping: f32,
}

pub struct DistanceJointBuilder {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    length: f32,
    stiffness: f32,
    damping: f32,
}

impl DistanceJointBuilder {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, local_a: Vec2, local_b: Vec2) -> Self;
    pub fn length(mut self, f: f32) -> Self;
    pub fn stiffness(mut self, f: f32) -> Self;
    pub fn damping(mut self, f: f32) -> Self;
    pub fn build(self) -> DistanceJoint;
}
```

### RevoluteJoint

```rust
pub struct RevoluteJoint {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    limits: Option<(f32, f32)>,  // (min, max)
    motor: Option<(f32, f32)>,   // (velocity, max_torque)
}

pub struct RevoluteJointBuilder {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    limits: Option<(f32, f32)>,
    motor: Option<(f32, f32)>,
}

impl RevoluteJointBuilder {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor: Vec2) -> Self;
    pub fn limits(mut self, min: f32, max: f32) -> Self;
    pub fn motor(mut self, velocity: f32, max_torque: f32) -> Self;
    pub fn build(self) -> RevoluteJoint;
}
```

### PrismaticJoint

```rust
pub struct PrismaticJoint {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    axis: Vec2,
    limits: Option<(f32, f32)>,
    motor: Option<(f32, f32)>,
}

pub struct PrismaticJointBuilder {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    axis: Vec2,
    limits: Option<(f32, f32)>,
    motor: Option<(f32, f32)>,
}

impl PrismaticJointBuilder {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor: Vec2, axis: Vec2) -> Self;
    pub fn limits(mut self, min: f32, max: f32) -> Self;
    pub fn motor(mut self, velocity: f32, max_force: f32) -> Self;
    pub fn build(self) -> PrismaticJoint;
}
```

### WeldJoint

```rust
pub struct WeldJoint {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
}

pub struct WeldJointBuilder {
    body_a: BodyHandle,
    body_b: BodyHandle,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
}

impl WeldJointBuilder {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, local_a: Vec2, local_b: Vec2) -> Self;
    pub fn build(self) -> WeldJoint;
}
```

### SpringJoint

```rust
pub struct SpringJoint {
    body_a: BodyHandle,
    body_b: BodyHandle,
    anchor_a: Vec2,
    anchor_b: Vec2,
    stiffness: f32,
    damping: f32,
    rest_length: f32,
}

pub struct SpringJointBuilder {
    body_a: BodyHandle,
    body_b: BodyHandle,
    anchor_a: Vec2,
    anchor_b: Vec2,
    stiffness: f32,
    damping: f32,
    rest_length: f32,
}

impl SpringJointBuilder {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor_a: Vec2, anchor_b: Vec2) -> Self;
    pub fn stiffness(mut self, f: f32) -> Self;
    pub fn damping(mut self, f: f32) -> Self;
    pub fn rest_length(mut self, f: f32) -> Self;
    pub fn build(self) -> SpringJoint;
}
```

### MotorJoint

```rust
pub struct MotorJoint {
    body_a: BodyHandle,
    body_b: BodyHandle,
    offset: Vec2,
    motor_velocity: Vec2,
    motor_max_force: f32,
}

pub struct MotorJointBuilder {
    body_a: BodyHandle,
    body_b: BodyHandle,
    offset: Vec2,
    motor_velocity: Vec2,
    motor_max_force: f32,
}

impl MotorJointBuilder {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, offset: Vec2) -> Self;
    pub fn motor_velocity(mut self, v: Vec2) -> Self;
    pub fn motor_max_force(mut self, f: f32) -> Self;
    pub fn build(self) -> MotorJoint;
}
```

---

## 输入/输出

### 输入
- 连接的刚体 A 和 B 的句柄
- 关节本地锚点坐标
- 关节参数（刚度、阻尼、限制、马达配置）

### 输出
- 关节约束力
- 刚体相对运动的约束效果

---

## 验收标准

1. ✅ `DistanceJoint` 可约束两刚体间的距离
2. ✅ `RevoluteJoint` 允许两刚体绕锚点相对旋转
3. ✅ `RevoluteJoint::limits` 正确限制旋转角度
4. ✅ `PrismaticJoint` 允许两刚体沿轴线相对滑动
5. ✅ `WeldJoint` 固定两刚体间的相对位置和旋转
6. ✅ `SpringJoint` 提供弹性约束效果
7. ✅ `MotorJoint` 可驱动两刚体产生相对运动
8. ✅ `World2D::insert_joint` 后关节立即生效
9. ✅ `World2D::remove_joint` 后两刚体运动解耦
10. ✅ 示例 `joints` 展示钟摆/弹簧关节效果

---

## 依赖关系

- 依赖 `engine-physics-2d` crate（World2D、RigidBody2D）
- 示例 `joints` 依赖本模块

---

## 优先级

| 优先级 | 含义 | 需求编号 |
|--------|------|----------|
| P0 | 核心功能 | 81, 109-114, 251-268 |
| P1 | 重要功能 | 255-263 |
| P2 | 增强功能 | 264-265 |
