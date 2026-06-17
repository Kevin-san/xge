# Module 01 — 物理世界 3D 与时间步进

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/world.rs`

## 1. 目标

建立 `PhysicsWorld3D` 主容器：
- 固定步长 + 子步循环
- Island 拆分（独立求解单元）
- Sleeping Body（CPU 节省）

## 2. PhysicsConfig

```rust
pub struct PhysicsConfig {
    pub gravity: Vec3,
    pub fixed_dt: f32,           // 默认 1/60
    pub max_substeps: usize,     // 默认 4
    pub solver_iterations: usize, // 默认 8
    pub position_iterations: usize, // 默认 3
    pub sleep_threshold: f32,    // 速度阈值
    pub sleep_time: f32,         // 触发睡眠时间
    pub enable_ccd: bool,        // 全局 CCD
}
```

## 3. PhysicsWorld3D

```rust
pub struct PhysicsWorld3D {
    pub config: PhysicsConfig,
    pub bodies: Vec<RigidBody>,
    pub colliders: Vec<Collider>,
    pub joints: Vec<Joint>,
    pub islands: Vec<Island>,
    pub contact_manifolds: HashMap<(BodyHandle, BodyHandle), ContactManifold>,
    pub bvh: Bvh,
    pub simulation_time: f32,
    pub accumulator: f32,
    pub broad_phase_pairs: Vec<(BodyHandle, BodyHandle)>,
}

impl PhysicsWorld3D {
    pub fn new(config: PhysicsConfig) -> Self;
    pub fn with_default_config() -> Self;
    
    pub fn add_body(&mut self, body: RigidBody) -> BodyHandle;
    pub fn remove_body(&mut self, handle: BodyHandle);
    pub fn add_collider(&mut self, collider: Collider, body: BodyHandle) -> ColliderHandle;
    pub fn add_joint(&mut self, joint: Joint) -> JointHandle;
    
    /// 主步进：real_dt 自适应子步
    pub fn step(&mut self, real_dt: f32);
    
    pub fn ray_cast(&self, ray: Ray3) -> Option<RayCastHit>;
    pub fn overlap_test(&self, shape: &dyn Shape, pos: Vec3, rot: Quat) -> Vec<BodyHandle>;
    pub fn sweep_test(&self, shape: &dyn Shape, from: Vec3, to: Vec3) -> Vec<BodyHandle>;
}
```

## 4. Step 流程

```rust
pub fn step(&mut self, real_dt: f32) {
    self.accumulator += real_dt;
    let max_time = self.config.fixed_dt * self.config.max_substeps as f32;
    if self.accumulator > max_time {
        self.accumulator = max_time;  // 防止 spiral of death
    }
    
    while self.accumulator >= self.config.fixed_dt {
        let dt = self.config.fixed_dt;
        self.sub_step(dt);
        self.accumulator -= dt;
        self.simulation_time += dt;
    }
}

fn sub_step(&mut self, dt: f32) {
    // 1. 唤醒 sleeping body
    self.update_sleep_state(dt);
    
    // 2. 应用外力（重力、用户力）
    self.apply_forces();
    
    // 3. Broad Phase: BVH 找碰撞对
    self.broad_phase();
    
    // 4. Narrow Phase: GJK/EPA
    self.narrow_phase();
    
    // 5. 拆分 Island
    self.build_islands();
    
    // 6. 求解器（PGS）
    self.solve_islands(dt);
    
    // 7. 积分
    self.integrate(dt);
    
    // 8. 位置修正
    self.position_correction();
}
```

## 5. Island

```rust
pub struct Island {
    pub bodies: SmallVec<[BodyHandle; 8]>,
    pub joints: SmallVec<[JointHandle; 4]>,
    pub sleep_time: f32,
}

impl Island {
    pub fn is_sleeping(&self) -> bool;
    pub fn can_sleep(&self) -> bool;
}
```

## 6. Sleeping

```rust
fn update_sleep_state(&mut self, dt: f32) {
    for body in &mut self.bodies {
        if !body.is_dynamic() { continue; }
        let speed_sq = body.linear_vel.length_squared() + body.angular_vel.length_squared();
        if speed_sq < self.config.sleep_threshold.powi(2) {
            body.sleep_timer += dt;
            if body.sleep_timer > self.config.sleep_time {
                body.sleep();
            }
        } else {
            body.wake();
            body.sleep_timer = 0.0;
        }
    }
}
```

## 7. 验收

- [ ] 1000 刚体 60 Hz 步进 < 5 ms
- [ ] Island 拆分无约束物体 5x speedup
- [ ] Sleeping 节省 CPU < 1% 占用
- [ ] 测试：pile of boxes 静态稳定
- [ ] 测试：旋转陀螺睡眠/唤醒
