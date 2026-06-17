# Module 06 — 关节与约束

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/joint/`

## 1. Joint Trait

```rust
pub trait Joint: Send + Sync {
    fn body_a(&self) -> BodyHandle;
    fn body_b(&self) -> BodyHandle;
    fn local_anchor_a(&self) -> Vec3;
    fn local_anchor_b(&self) -> Vec3;
    
    /// 添加约束到求解器
    fn add_constraints(&self, body_a: &RigidBody, body_b: &RigidBody, constraints: &mut Vec<Constraint>);
    
    /// 求解
    fn solve_velocity(&self, body_a: &mut RigidBody, body_b: &mut RigidBody, dt: f32);
    fn solve_position(&self, body_a: &mut RigidBody, body_b: &mut RigidBody, dt: f32) -> bool;
}
```

## 2. Fixed Joint

```rust
pub struct FixedJoint {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub local_anchor_a: Vec3,
    pub local_anchor_b: Vec3,
    pub local_rotation_a: Quat,
    pub local_rotation_b: Quat,
    /// 限制（None 表示自由）
    pub linear_limits: Option<LinearLimits>,
    pub angular_limits: Option<AngularLimits>,
}

impl FixedJoint {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle, anchor_a: Vec3, anchor_b: Vec3) -> Self;
}
```

## 3. Hinge Joint（铰链，1 DOF 旋转）

```rust
pub struct HingeJoint {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub local_anchor_a: Vec3,
    pub local_anchor_b: Vec3,
    pub local_axis_a: Vec3,    // 旋转轴
    pub local_axis_b: Vec3,
    pub limits: Option<HingeLimits>,
    pub motor: Option<HingeMotor>,
}

pub struct HingeLimits {
    pub min: f32,   // 弧度
    pub max: f32,
}

pub struct HingeMotor {
    pub target_velocity: f32,
    pub max_impulse: f32,
}
```

## 4. Ball-Socket Joint（球窝，3 DOF 旋转）

```rust
pub struct BallSocketJoint {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub local_anchor_a: Vec3,
    pub local_anchor_b: Vec3,
    pub cone_limits: Option<ConeLimit>,
}

pub struct ConeLimit {
    pub axis_a: Vec3,
    pub angle: f32,  // 最大半角
}
```

## 5. Slider Joint（滑动，1 DOF 平移）

```rust
pub struct SliderJoint {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub local_anchor_a: Vec3,
    pub local_anchor_b: Vec3,
    pub local_axis_a: Vec3,    // 滑动方向
    pub local_axis_b: Vec3,
    pub limits: Option<SliderLimits>,
}
```

## 6. Cone-Twist Joint（角色关节）

```rust
pub struct ConeTwistJoint {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub local_anchor_a: Vec3,
    pub local_anchor_b: Vec3,
    pub twist_axis_a: Vec3,
    pub swing_limit: f32,   // 弧度
    pub twist_limit: f32,   // 弧度
}
```

## 7. 可断关节

```rust
pub struct BreakableJoint {
    pub inner: Box<dyn Joint>,
    pub break_force: f32,    // 力阈值
    pub break_torque: f32,   // 力矩阈值
    pub broken: bool,
    pub on_break: Option<Box<dyn FnMut()>>,
}

impl BreakableJoint {
    pub fn check_break(&mut self, joint: &Constraint) {
        let force = joint.last_applied_force();
        let torque = joint.last_applied_torque();
        if force.length() > self.break_force || torque.length() > self.break_torque {
            self.broken = true;
            if let Some(cb) = &mut self.on_break {
                cb();
            }
        }
    }
}
```

## 8. 验收

- [ ] 4 关节机械臂 1kHz 跟随
- [ ] 关节断裂阈值 ±5% 误差
- [ ] 100 关节求解 < 1 ms
- [ ] 测试：链条垂落自然
- [ ] 测试：hinge joint 角度限制生效
