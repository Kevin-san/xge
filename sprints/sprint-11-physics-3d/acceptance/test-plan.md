# Sprint 11 测试计划

## 概述

本文档定义 Sprint 11 (3D 物理引擎集成) 的完整测试计划，涵盖单元测试、集成测试、性能测试和验收测试。

## 测试环境

- **Rust 版本**: stable (最新)
- **测试框架**: `#[test]` + `#[tokio::test]`
- **物理后端**: Rapier3D v0.19+
- **目标平台**: Linux / macOS / Windows

---

## 1. 单元测试

### 1.1 PhysicsWorld3D 测试

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| UT-001 | `test_world_creation` | 创建 PhysicsWorld3D 后状态正确 | 236 |
| UT-002 | `test_step_updates_bodies` | step(dt) 后动态刚体位置更新 | 237, 274 |
| UT-003 | `test_step_zero_no_crash` | step(0.0) 不崩溃 | 228 |
| UT-004 | `test_set_paused` | set_paused(true) 后 step 不更新 | 241 |
| UT-005 | `test_gravity` | 重力影响 linvel | 207 |
| UT-006 | `test_ccd_high_speed` | CCD 启用后高速物体命中薄物体 | 229 |
| UT-007 | `test_clear_world` | clear() 后所有计数为 0 | 267 |
| UT-008 | `test_substeps` | step_with_substeps 正确细分 | 238 |

### 1.2 RigidBody3D 测试

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| UT-101 | `test_dynamic_body` | Dynamic 刚体受重力影响 | 25, 268 |
| UT-102 | `test_static_body` | Static 刚体位置不变 | 26, 269 |
| UT-103 | `test_kinematic_position` | KinematicPositionBased 位置由 set_position 控制 | 27, 270 |
| UT-104 | `test_kinematic_velocity` | KinematicVelocityBased 速度驱动位置 | 28, 271 |
| UT-105 | `test_fixed_body` | Fixed 刚体不能移动 | 29, 272 |
| UT-106 | `test_apply_force` | apply_force 改变 linvel | 68, 303 |
| UT-107 | `test_apply_impulse` | apply_impulse 立即改变 linvel | 71, 306 |
| UT-108 | `test_apply_force_at_point` | apply_force_at_point 产生旋转 | 69, 304 |
| UT-109 | `test_apply_torque` | apply_torque 改变 angvel | 70, 305 |
| UT-110 | `test_lock_translations` | lock_translations 阻止移动 | 46, 288 |
| UT-111 | `test_lock_rotations` | lock_rotations 阻止旋转 | 47, 289 |
| UT-112 | `test_linear_damping` | linear_damping 减缓速度 | 74, 309 |
| UT-113 | `test_angular_damping` | angular_damping 减缓角速度 | 76, 311 |
| UT-114 | `test_gravity_scale` | gravity_scale 调整重力影响 | 78, 313 |
| UT-115 | `test_sleeping` | 静止刚体进入睡眠 | 80, 315 |
| UT-116 | `test_wake_up` | wake_up 唤醒睡眠刚体 | 81, 316 |
| UT-117 | `test_ccd_enabled` | ccd_enabled 防止穿透 | 83, 318 |
| UT-118 | `test_mass_properties` | mass_properties 设置惯性 | 36, 278 |
| UT-119 | `test_center_of_mass` | center_of_mass 偏移质心 | 37, 279 |
| UT-120 | `test_restrict_rotations` | restrict_rotations 限制轴 | 48, 290 |

### 1.3 Collider3D 测试

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| UT-201 | `test_ball_collider` | Ball 碰撞体形状正确 | 89, 320 |
| UT-202 | `test_cuboid_collider` | Cuboid 碰撞体形状正确 | 90, 321 |
| UT-203 | `test_capsule_collider` | Capsule 碰撞体形状正确 | 91, 322 |
| UT-204 | `test_cylinder_collider` | Cylinder 碰撞体形状正确 | 92, 323 |
| UT-205 | `test_cone_collider` | Cone 碰撞体形状正确 | 93, 324 |
| UT-206 | `test_convex_hull_valid` | convex_hull 对有效点返回 Some | 94, 325 |
| UT-207 | `test_convex_hull_invalid` | convex_hull 对无效点返回 None | 94, 325 |
| UT-208 | `test_trimesh_collider` | Trimesh 碰撞体正确索引 | 95, 326 |
| UT-209 | `test_heightfield_collider` | Heightfield 碰撞体正确生成 | 96, 327 |
| UT-210 | `test_segment_collider` | Segment 碰撞体形状正确 | 98, 328 |
| UT-211 | `test_triangle_collider` | Triangle 碰撞体形状正确 | 99, 329 |
| UT-212 | `test_halfspace_collider` | Halfspace 碰撞体正确 | 100, 330 |
| UT-213 | `test_collider_aabb` | aabb() 返回正确 AABB | 124, 340 |
| UT-214 | `test_collider_friction` | set_friction 改变摩擦力 | 127, 343 |
| UT-215 | `test_collider_restitution` | set_restitution 改变弹性 | 129, 345 |
| UT-216 | `test_sensor_collider` | sensor 触发 intersection_event | 112, 130, 239 |
| UT-217 | `test_collision_groups` | CollisionGroups 过滤正确 | 120, 211 |
| UT-218 | `test_density_mass` | density 计算质量 | 103, 333 |

### 1.4 Joint3D 测试

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| UT-301 | `test_fixed_joint` | FixedJoint 保持相对位置 | 133, 350-355 |
| UT-302 | `test_revolute_joint` | RevoluteJoint 单轴旋转 | 134, 356-362 |
| UT-303 | `test_revolute_limits` | RevoluteJoint limits 限制角度 | 360 |
| UT-304 | `test_prismatic_joint` | PrismaticJoint 单轴滑动 | 135, 363-364 |
| UT-305 | `test_prismatic_limits` | PrismaticJoint limits 限制距离 | 364 |
| UT-306 | `test_ball_joint` | BallJoint 万向旋转 | 136, 365-366 |
| UT-307 | `test_ball_joint_limits` | BallJoint limits 锥角限制 | 366 |
| UT-308 | `test_distance_joint` | DistanceJoint 约束距离 | 137, 367-368 |
| UT-309 | `test_rope_joint` | RopeJoint 限制最大距离 | 138, 369 |
| UT-310 | `test_spherical_joint` | SphericalJoint 锥限制 | 139, 370 |
| UT-311 | `test_joint_distance_constraint` | Joint 两体距离被限制 | 240 |

### 1.5 CharacterController3D 测试

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| UT-401 | `test_character_flat_ground` | 角色在平地上站立 | 371, 372 |
| UT-402 | `test_character_slope_stand` | 角色在小坡度斜坡站立 | 374, 427 |
| UT-403 | `test_character_slope_slide` | 角色在大坡度斜坡滑动 | 375, 390 |
| UT-404 | `test_character_jump` | 角色可跳跃 | 387 |
| UT-405 | `test_character_grounded` | grounded 状态正确 | 379, 380 |
| UT-406 | `test_character_hit_ceil` | hit_ceil 检测天花板 | 381 |
| UT-407 | `test_character_hit_wall` | hit_wall 检测墙壁 | 382 |
| UT-408 | `test_character_ground_normal` | ground_normal 返回正确法线 | 383 |
| UT-409 | `test_character_up_direction` | set_up 改变上方向 | 378 |

### 1.6 Query3D 测试

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| UT-501 | `test_ray_hit_static` | 射线命中静态物体返回 toi > 0 | 155, 209 |
| UT-502 | `test_ray_miss` | 射线错过物体返回 None | 156, 210 |
| UT-503 | `test_ray_parallel_no_crash` | 射线平行于面不崩溃 | 245 |
| UT-504 | `test_ray_filter_groups` | CollisionGroups 过滤命中 | 157, 211 |
| UT-505 | `test_cast_shape` | cast_shape 检测形状碰撞 | 158, 395 |
| UT-506 | `test_intersection_with_shape` | intersection_with_shape 返回实体 | 159, 396 |
| UT-507 | `test_point_intersections` | point_intersections 回调触发 | 160, 397 |
| UT-508 | `test_intersections_with_aabb` | intersections_with_aabb 返回实体 | 161, 398 |
| UT-509 | `test_query_filter_exclude_sensors` | exclude_sensors 排除传感器 | 161, 401 |
| UT-510 | `test_query_filter_exclude_body` | exclude 排除指定实体 | 161, 404 |
| UT-511 | `test_ray3_point_at` | Ray3::point_at 计算正确 | 163, 392 |

---

## 2. 集成测试

### 2.1 物理模拟集成

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| IT-001 | `test_sphere_falls_to_plane` | 球体落到静态平面 | 206 |
| IT-002 | `test_gravity_affects_linvel` | 重力改变 linvel | 207, 461 |
| IT-003 | `test_apply_impulse_changes_linvel` | apply_impulse 改变 linvel | 208, 463 |
| IT-004 | `test_stack_stability` | 100 盒子堆叠稳定 | 258, 484 |
| IT-005 | `test_ccd_thin_object` | 高速小物体命中薄物体 | 229, 469 |

### 2.2 ECS 集成

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| IT-101 | `test_sync_transform_to_physics` | Scene transform 同步到物理引擎 | 184, 263 |
| IT-102 | `test_sync_physics_to_transform` | 物理引擎结果同步到 Scene transform | 185, 263 |
| IT-103 | `test_physics_query_system_param` | PhysicsQuery system param 查询正常 | 186 |

### 2.3 事件系统集成

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| IT-201 | `test_contact_event_started` | ContactEvent::Started 正确触发 | 165, 409 |
| IT-202 | `test_contact_event_stopped` | ContactEvent::Stopped 正确触发 | 165, 409 |
| IT-203 | `test_intersection_event_started` | IntersectionEvent::Started 正确触发 | 168, 410 |
| IT-204 | `test_intersection_event_stopped` | IntersectionEvent::Stopped 正确触发 | 168, 410 |
| IT-205 | `test_sensor_trigger` | Sensor collider 触发 intersection_event | 216, 525 |

### 2.4 调试可视化集成

| 测试编号 | 测试名称 | 验证内容 | 对应需求 |
|----------|----------|----------|----------|
| IT-301 | `test_debug_draw_colliders` | 绘制碰撞体线条 | 418, 419 |
| IT-302 | `test_debug_draw_contacts` | 绘制接触点 | 418, 422 |
| IT-303 | `test_debug_draw_joints` | 绘制关节锚点 | 418, 424 |
| IT-304 | `test_debug_draw_aabb` | 绘制 AABB | 418, 423 |

---

## 3. 示例验证测试

每个示例需验证：

| 示例 | 验证命令 | 验收条件 |
|------|----------|----------|
| `physics_3d_basic` | `cargo run --example physics_3d_basic` | 盒子正常下落 |
| `physics_3d_stack` | `cargo run --example physics_3d_stack` | 100 盒子稳定 |
| `physics_3d_ragdoll` | `cargo run --example physics_3d_ragdoll` | ragdoll 正常 |
| `physics_3d_character` | `cargo run --example physics_3d_character` | 角色可移动/跳跃 |
| `physics_3d_joints` | `cargo run --example physics_3d_joints` | 钟摆/弹簧正常 |
| `physics_3d_ray_cast` | `cargo run --example physics_3d_ray_cast` | 点击命中高亮 |
| `physics_3d_trigger` | `cargo run --example physics_3d_trigger` | 触发事件正确 |
| `physics_3d_heightfield` | `cargo run --example physics_3d_heightfield` | 地形碰撞正常 |
| `physics_3d_compound` | `cargo run --example physics_3d_compound` | 复合碰撞体正常 |

---

## 4. 性能测试

### 4.1 帧率测试

| 测试编号 | 测试场景 | 目标 | 对应需求 |
|----------|----------|------|----------|
| PT-001 | 100 盒子堆叠 | >= 60 fps | 258, 484 |
| PT-002 | 复杂 ragdoll 场景 | >= 60 fps | 260 |
| PT-003 | 大量射线查询 | < 1ms per query | - |

### 4.2 内存测试

| 测试编号 | 测试场景 | 目标 |
|----------|----------|------|
| MT-001 | 1000 刚体场景 | 内存 < 100MB |
| MT-002 | 长时间运行 | 无内存泄漏 |

---

## 5. 代码质量检查

| 检查项 | 命令 | 验收条件 |
|--------|------|----------|
| 单元测试 | `cargo test -p engine-physics-3d` | 全部通过 |
| Clippy 检查 | `cargo clippy --workspace -- -D warnings` | 无警告 |
| 代码格式 | `cargo fmt --check --workspace` | 通过 |
| 文档生成 | `cargo doc --workspace --no-deps` | 成功 |
| 公开 API 文档 | - | 100% 覆盖 |

---

## 6. CI 测试矩阵

| 平台 | 工具链 | 验收条件 |
|------|--------|----------|
| Linux | stable | green |
| macOS | stable | green |
| Windows | stable | green |

---

## 7. 测试执行计划

### 阶段 1: 单元测试（开发期）
- 每日运行 `cargo test -p engine-physics-3d`
- TDD 模式：先写测试，再实现

### 阶段 2: 集成测试（Sprint 中期）
- 功能完成后运行集成测试
- 修复失败的集成测试

### 阶段 3: 示例验证（Sprint 后期）
- 运行所有示例验证
- 性能测试

### 阶段 4: 最终验收（Sprint 结束）
- 完整测试套件
- CI 三平台 green
- 代码质量检查全部通过

---

## 8. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| Rapier3D 后端 bug | 高 | 使用 NullBackend 进行核心逻辑测试 |
| 性能不达标 | 中 | 预留 1 周进行性能优化 |
| ECS 集成复杂 | 中 | 提前进行 ECS 集成原型验证 |
| 跨平台差异 | 低 | CI 三平台覆盖 |
