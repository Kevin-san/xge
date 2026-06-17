# Module 05 — 刚体动力学 + PGS 求解器

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/{rigidbody,solver}/`

## 1. RigidBody

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBodyType {
    Static,        // 无穷质量
    Kinematic,    // 用户控制，无物理
    Dynamic,      // 物理驱动
}

pub struct RigidBody {
    pub body_type: RigidBodyType,
    pub mass: f32,
    pub inv_mass: f32,           // 0 if Static/Kinematic
    pub inertia: Mat3,           // 局部空间惯性张量
    pub inv_inertia: Mat3,
    pub inv_inertia_world: Mat3, // 世界空间
    
    pub transform: Transform,
    pub prev_transform: Transform, // CCD 用
    
    pub linear_vel: Vec3,
    pub angular_vel: Vec3,
    pub force: Vec3,
    pub torque: Vec3,
    
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub gravity_scale: f32,
    
    pub sleep_state: SleepState,
}

pub enum SleepState {
    Awake,
    Sleepy { time: f32 },
    Sleeping,
}
```

## 2. 积分（半隐式欧拉）

```rust
pub fn integrate(body: &mut RigidBody, dt: f32) {
    if body.body_type != RigidBodyType::Dynamic || body.is_sleeping() {
        return;
    }
    
    // 应用力
    let acceleration = body.force * body.inv_mass * body.gravity_scale;
    body.linear_vel += acceleration * dt;
    
    let angular_accel = body.inv_inertia_world * body.torque;
    body.angular_vel += angular_accel * dt;
    
    // 阻尼
    body.linear_vel *= 1.0 / (1.0 + dt * body.linear_damping);
    body.angular_vel *= 1.0 / (1.0 + dt * body.angular_damping);
    
    // 积分位置
    body.transform.position += body.linear_vel * dt;
    
    // 旋转
    let ang_delta = body.angular_vel * dt;
    let dq = Quat::from_scaled_axis(ang_delta);
    body.transform.rotation = (dq * body.transform.rotation).normalize();
    
    // 重置累积力
    body.force = Vec3::ZERO;
    body.torque = Vec3::ZERO;
    
    // 更新世界空间惯性
    body.inv_inertia_world = body.transform.rotation * body.inv_inertia * body.transform.rotation.conjugate();
}
```

## 3. PGS 求解器

```rust
pub fn solve_islands(world: &mut PhysicsWorld3D, dt: f32) {
    for island in &mut world.islands {
        if island.is_sleeping() { continue; }
        
        // 准备约束
        let mut constraints = Vec::new();
        for manifold in world.contact_manifolds.values() {
            if manifold.belongs_to_island(island) {
                constraints.push(Constraint::Contact(manifold.clone()));
            }
        }
        for joint_handle in &island.joints {
            if let Some(joint) = world.joints.get(*joint_handle) {
                constraints.push(Constraint::Joint(joint.clone()));
            }
        }
        
        // 速度迭代
        for _ in 0..world.config.solver_iterations {
            for c in &mut constraints {
                c.solve_velocity(world, dt);
            }
        }
        
        // 位置迭代
        for _ in 0..world.config.position_iterations {
            for c in &mut constraints {
                c.solve_position(world, dt);
            }
        }
    }
}
```

## 4. 接触约束

```rust
pub fn solve_contact_velocity(
    body_a: &mut RigidBody,
    body_b: &mut RigidBody,
    contact: &mut ContactPoint,
    normal: Vec3,
    friction: f32,
) {
    // 1. 计算相对速度
    let v_a = body_a.linear_vel + body_a.angular_vel.cross(contact.world_position - body_a.transform.position);
    let v_b = body_b.linear_vel + body_b.angular_vel.cross(contact.world_position - body_b.transform.position);
    let v_rel = v_a - v_b;
    
    // 2. 法向相对速度
    let vn = v_rel.dot(normal);
    if vn > 0.0 { return; }  // 分离
    
    // 3. 法向冲量
    let inv_mass_sum = body_a.inv_mass + body_b.inv_mass;
    let r_a = contact.world_position - body_a.transform.position;
    let r_b = contact.world_position - body_b.transform.position;
    let rn_a = r_a.cross(normal);
    let rn_b = r_b.cross(normal);
    let k = inv_mass_sum + (body_a.inv_inertia_world * rn_a).cross(r_a).dot(normal)
                       + (body_b.inv_inertia_world * rn_b).cross(r_b).dot(normal);
    
    let restitution = (body_a.restitution + body_b.restitution) * 0.5;
    let bias = -restitution * vn;  // 弹跳修正
    
    let j = -(vn + bias) / k;
    let impulse = normal * j;
    
    body_a.linear_vel += impulse * body_a.inv_mass;
    body_a.angular_vel += body_a.inv_inertia_world * r_a.cross(impulse);
    body_b.linear_vel -= impulse * body_b.inv_mass;
    body_b.angular_vel -= body_b.inv_inertia_world * r_b.cross(impulse);
    
    contact.normal_impulse = j;
    
    // 4. 摩擦（2 个切向）
    for tangent in [tangent_x, tangent_y] {
        let vt = v_rel.dot(tangent);
        let rt_a = r_a.cross(tangent);
        let rt_b = r_b.cross(tangent);
        let kt = inv_mass_sum + (body_a.inv_inertia_world * rt_a).cross(r_a).dot(tangent)
                            + (body_b.inv_inertia_world * rt_b).cross(r_b).dot(tangent);
        let jt = -vt / kt;
        let max_friction = friction * contact.normal_impulse;
        let jt = jt.clamp(-max_friction, max_friction);
        // 应用切向冲量
        // ...
    }
}
```

## 5. 位置修正（Baumgarte）

```rust
pub fn position_correction(world: &mut PhysicsWorld3D) {
    let slop = 0.005;
    let baumgarte = 0.2;
    
    for manifold in world.contact_manifolds.values_mut() {
        for contact in &mut manifold.points {
            let correction_magnitude = (contact.penetration - slop).max(0.0) * baumgarte;
            let correction = manifold.normal * correction_magnitude;
            
            let total_inv_mass = manifold.body_a.inv_mass + manifold.body_b.inv_mass;
            if total_inv_mass == 0.0 { continue; }
            
            let move_a = -correction * (manifold.body_a.inv_mass / total_inv_mass);
            let move_b = correction * (manifold.body_b.inv_mass / total_inv_mass);
            
            // 移动位置
            // body_a.transform.position += move_a;
            // body_b.transform.position += move_b;
        }
    }
}
```

## 6. 验收

- [ ] 100 物体稳定堆叠 5 层无抖动
- [ ] 摩擦斜面：滑动 vs 静止 0.01 m/s 精度
- [ ] 求解器发散检测：穿透 > 1cm 时退化
- [ ] 100 物体 8 iter PGS < 5 ms
- [ ] 睡眠检测：闲置 0.1s 进入 sleep
- [ ] 测试：pile of 100 boxes 静态稳定 60s
