# Sprint 20 · 3D 物理引擎（Chaos 风格 + 连续碰撞检测 + 关节）

> 文档编号: `sprint-20-physics-3d.md / v1.0
> 周期: 4 周 (20 个工作日)
> 上游依赖: Sprint 17 (Math SIMD), Sprint 18 (ECS)
> 下游交付: Sprint 22 (Character Controller / 物理资产)

---

## 1. 目标与范围

**目标：** 全新 `engine-physics-3d` crate，实现 **Unreal Chaos** 风格的现代 3D 物理引擎：BVH 加速 Broad Phase、连续碰撞检测（CCD）、PGS 约束求解器、铰链/球窝/滑动关节、可破坏关节断点。

**范围：**
- ✅ BVH 加速 Broad Phase
- ✅ 连续碰撞检测（CCD）：Tobii / Conservative Advancement
- ✅ 多种形状：Box / Sphere / Capsule / Cylinder / Convex Hull / Trimesh
- ✅ 约束求解器：PGS（Projected Gauss-Seidel）
- ✅ 关节：Fixed / Hinge / Ball-Socket / Slider / Cone-Twist
- ✅ Trigger 触发器
- ✅ 力场 / 风力 / 流体密度
- ⛔ 不含：软体（Cloth）/ 破碎 / 流体（XPBD Fluids）/ Vehicle / Ragdoll 完整 IK

**核心参考：** PhysX 5 / Chaos / Rapier / Bullet3 / Box2D 三维化。

---

## 2. 上游需求对接

| 来源 | 关联章节 | 承接 |
|------|---------|------|
| [NEXT_PHASE_REQUIREMENTS.md § 7.1](../NEXT_PHASE_REQUIREMENTS.md) | 物理 3D 总设计 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 7.2](../NEXT_PHASE_REQUIREMENTS.md) | BVH Broad Phase | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 7.3](../NEXT_PHASE_REQUIREMENTS.md) | 连续碰撞检测 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 7.4](../NEXT_PHASE_REQUIREMENTS.md) | 约束求解器 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 5 M4](../NEXT_PHASE_REQUIREMENTS.md) | Milestone M4 | ✅ 本 sprint |

---

## 3. 子模块拆分

### 3.1 [01-physics-world3d.md](modules/01-physics-world3d.md) — 物理世界与时间步进

**核心交付：**
- `engine-physics-3d/src/world.rs`
  - `PhysicsWorld3D` 主容器
  - `PhysicsConfig { gravity, fixed_dt, max_substeps, solver_iterations }`
  - `step(real_dt: f32)` 固定步长 + 子步循环
  - Island 拆分（静态物体独立 island）
  - Sleeping Bodies：速度阈值 + 时间窗口
- `engine-physics-3d/src/island.rs`
  - `Island { bodies, joints }` 求解单元
  - 并行 island 求解（rayon）

**验收：**
- 1000 刚体 60 Hz 步进 < 5 ms
- Island 拆分：无约束物体 5x speedup
- Sleeping：闲置 0.1s 后进入 sleep，CPU 占用 < 1%

---

### 3.2 [02-bvh-broad-phase.md](modules/02-bvh-broad-phase.md) — BVH Broad Phase

**核心交付：**
- `engine-physics-3d/src/broadphase/bvh.rs`
  - `BvhNode { aabb: AABB, children: [u32; 2], body_index: Option<u32> }`
  - `insert_body`, `remove_body`, `update_body`
  - `find_pairs() -> Vec<(BodyHandle, BodyHandle)>`
  - SAH 重建（每 100 帧）
- `engine-physics-3d/src/broadphase/dynamic_aabb_tree.rs`
  - Erin Catto Dynamic AABB Tree（Box2D 风格）
  - 增量更新，无重建

**Bug 修复对应：** `engine-physics-2d/src/world.rs#L230-L248` O(n²) 扫描

**验收：**
- 10000 动态物体 Broad Phase < 1 ms
- BVH 增量更新 < 5 µs / 物体
- 与 O(n²) 对比：1000 物体 100x speedup

---

### 3.3 [03-shapes-colliders.md](modules/03-shapes-colliders.md) — 形状与碰撞体

**核心交付：**
- `engine-physics-3d/src/shape/mod.rs`
  - `Shape` trait
  - `BoxShape { half_extents: Vec3 }`
  - `SphereShape { radius: f32 }`
  - `CapsuleShape { half_height, radius }`
  - `CylinderShape { half_height, radius }`
  - `ConvexHullShape { vertices: Vec<Vec3>, faces: Vec<[u32;3]> }`
  - `TrimeshShape { vertices, indices }`
- `engine-physics-3d/src/collider.rs`
  - `Collider { shape, local_transform, material }`
  - 摩擦 / 弹性 / 密度

**验收：**
- 凸包 Convex Hull：QuickHull 算法 < 1 ms（256 点输入）
- Trimesh 仅支持 kinematic 物体
- 形状内存布局：SIMD 友好（8 字节对齐）

---

### 3.4 [04-narrow-phase-ccd.md](modules/04-narrow-phase-ccd.md) — 窄相碰撞 + CCD

**核心交付：**
- `engine-physics-3d/src/narrowphase/gjk.rs`
  - **GJK 算法** 凸体相交测试
  - 支持 Box/Sphere/Capsule/Cylinder/Convex
- `engine-physics-3d/src/narrowphase/epa.rs`
  - **EPA 算法** 穿透深度 + 法线
- `engine-physics-3d/src/narrowphase/sat.rs`
  - SAT 备用路径（OBB-OBB / OBB-Plane）
- `engine-physics-3d/src/narrowphase/ccd.rs`
  - **连续碰撞检测**
  - **Tobii Raycast CCD** (球体)
  - **Conservative Advancement** (凸体)
  - 角速度 CCD
- `engine-physics-3d/src/contact.rs`
  - `ContactManifold { points: SmallVec<[ContactPoint; 4]> }` 4 点流形
  - 摩擦锥（4 方向）
  - Warmstarting 缓存

**Bug 修复对应：** `engine-physics-2d/src/world.rs#L250-L268` generate_contact 空实现

**验收：**
- 100 接触对 GJK/EPA < 0.5 ms
- CCD：1 mm 薄墙穿透率 < 0.01%
- 接触点稳定：4 frame 持久率 > 95%

---

### 3.5 [05-rigidbody-solver.md](modules/05-rigidbody-solver.md) — 刚体动力学 + PGS 求解器

**核心交付：**
- `engine-physics-3d/src/rigidbody.rs`
  - `RigidBody { mass, inv_inertia, linear_vel, angular_vel, transform, force, torque }`
  - 状态：Dynamic / Kinematic / Static
  - Sleeping 标志
- `engine-physics-3d/src/solver/pgs.rs`
  - **Projected Gauss-Seidel** 顺序脉冲约束求解
  - 8 速度迭代 + 3 位置迭代
  - 顺序：接触 → 摩擦 → 关节
- `engine-physics-3d/src/solver/contact_constraint.rs`
  - 非穿透约束（法向冲量）
  - 摩擦约束（库仑摩擦）
- `engine-physics-3d/src/solver/baumgarte.rs`
  - 位置投影稳定化
- `engine-physics-3d/src/integrator.rs`
  - 半隐式欧拉积分
  - 子步时间步

**验收：**
- 100 物体稳定堆叠 5 层无抖动
- 摩擦斜面：滑动 vs 静止 0.01 m/s 精度
- 求解器发散检测：穿透 > 1cm 时退化

---

### 3.6 [06-joints-constraints.md](modules/06-joints-constraints.md) — 关节与约束

**核心交付：**
- `engine-physics-3d/src/joint/mod.rs`
  - `Joint` trait
- `engine-physics-3d/src/joint/fixed.rs` — Fixed Joint
- `engine-physics-3d/src/joint/hinge.rs` — Hinge (1 DOF 旋转)
- `engine-physics-3d/src/joint/ball.rs` — Ball-Socket (3 DOF 旋转)
- `engine-physics-3d/src/joint/slider.rs` — Slider (1 DOF 平移)
- `engine-physics-3d/src/joint/cone_twist.rs` — Cone-Twist（角色关节）
- `engine-physics-3d/src/joint/breakable.rs` — 可断关节
  - 力度 / 力矩阈值
  - 断裂回调

**验收：**
- 4 关节机械臂：1kHz 控制 60 Hz 物理
- 关节断裂：阈值 ±5% 误差
- 关节约束：100 关节求解 < 1 ms

---

### 3.7 [07-character-controller.md](modules/07-character-controller.md) — 角色控制器

**核心交付：**
- `engine-physics-3d/src/character.rs`
  - `CharacterController { capsule_shape, step_offset, slope_limit }`
  - `move(desired: Vec3, dt: f32) -> CollisionInfo`
  - 楼梯 / 斜坡检测
  - 地面粘连

**验收：**
- 30° 斜坡自由行走
- 0.3m 高度台阶自动越过
- 贴墙滑行

---

## 4. 验收清单（acceptance/test-plan.md）

- [ ] 1000 刚体 60Hz 步进 < 5 ms
- [ ] 10000 物体 BVH Broad Phase < 1 ms
- [ ] GJK/EPA 100 接触 < 0.5 ms
- [ ] CCD 1mm 薄墙穿透率 < 0.01%
- [ ] PGS 求解器 100 物体堆叠稳定
- [ ] 4 关节机械臂 1kHz 跟随
- [ ] 角色控制器 30° 斜坡 / 0.3m 台阶
- [ ] 关节断裂阈值 ±5%
- [ ] `cargo test -p engine-physics-3d` 全通过
- [ ] `cargo bench` 基准存档
- [ ] 示例：`physics_3d_showcase`, `ragdoll_demo`, `joint_robot`, `ccd_thin_wall`

---

## 5. API 稳定承诺

```rust
pub use world::PhysicsWorld3D;
pub use rigidbody::{RigidBody, RigidBodyType};
pub use collider::Collider;
pub use shape::{BoxShape, SphereShape, CapsuleShape, CylinderShape, ConvexHullShape, TrimeshShape};
pub use joint::{Joint, FixedJoint, HingeJoint, BallSocketJoint, SliderJoint, ConeTwistJoint};
pub use contact::{ContactManifold, ContactPoint};
pub use character::CharacterController;
```

---

## 6. 与上下游依赖

| 依赖 | 来自 | 用途 |
|------|------|------|
| `f32x4` SIMD | sprint-17 | GJK support point |
| `Frustum` SIMD | sprint-17 | 调试可视化 |
| `World`, `Query<(&RigidBody, &Collider)>` | sprint-18 | 系统集成 |
| `PbrMaterial` 关联 | sprint-19 | 物理材质 → 渲染材质映射 |

---

## 7. 风险与缓解

| 风险 | 等级 | 缓解 |
|------|------|------|
| CCD 性能开销 | 高 | 仅在 `ccd_enabled` 物体启用 |
| GJK 数值稳定性 | 中 | 100 迭代上限 + 退化检测 |
| 关节求解器发散 | 中 | 顺序 + 阻尼 |
| 大场景 island 数量 | 中 | 多线程 island 求解 |
