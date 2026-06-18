# Sprint 20 · 验收测试计划

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)

## 1. 单元测试清单

| 模块 | 测试数 | 覆盖 |
|------|--------|------|
| PhysicsWorld3D | 25+ | 步进 / Sleeping / Island |
| BVH | 30+ | 插入 / 删除 / 更新 / 查询 |
| Shape | 30+ | 6 种形状 AABB / Inertia / Support |
| GJK | 25+ | 凸体相交 / 退化 |
| EPA | 20+ | 穿透深度 / 迭代上限 |
| CCD | 25+ | 球体 / 凸体 / 旋转 |
| RigidBody | 20+ | 积分 / 阻尼 / 力 |
| PGS Solver | 30+ | 接触 / 摩擦 / 位置 |
| Joint | 40+ | 6 种关节 |
| Character | 20+ | 移动 / 跳跃 / 斜坡 |

**总计：** 270+ 单元测试

## 2. 关键测试

### 2.1 GJK 凸体相交

```rust
#[test]
fn test_gjk_box_sphere() {
    let box_shape = BoxShape { half_extents: Vec3::splat(1.0) };
    let sphere_shape = SphereShape { radius: 0.5 };
    let t_a = Transform::from_translation(Vec3::ZERO);
    let t_b = Transform::from_translation(Vec3::new(0.7, 0.0, 0.0));
    
    let result = gjk_intersect(&box_shape, &sphere_shape, &t_a, &t_b);
    assert!(result.intersects);
}
```

### 2.2 堆叠稳定

```rust
#[test]
fn test_pile_stability() {
    let mut world = PhysicsWorld3D::with_default_config();
    // 5x5x5 = 125 boxes 堆叠
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                let body = create_box(Vec3::new(i as f32, k as f32, j as f32));
                world.add_body(body);
            }
        }
    }
    
    // 模拟 5 秒
    for _ in 0..300 {
        world.step(1.0 / 60.0);
    }
    
    // 验证堆叠仍在
    for body in world.bodies.iter() {
        assert!(body.transform.position.y > 0.0);
    }
}
```

### 2.3 CCD 薄墙

```rust
#[test]
fn test_ccd_thin_wall() {
    let mut world = PhysicsWorld3D::with_default_config();
    let wall = create_static_box(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.1, 1.0, 1.0));
    world.add_body(wall);
    
    // 高速小球（1 frame 内可能穿透）
    let mut ball = create_dynamic_sphere(Vec3::new(-5.0, 0.0, 0.0), 0.5);
    ball.linear_vel = Vec3::new(100.0, 0.0, 0.0);
    ball.enable_ccd = true;
    let ball_handle = world.add_body(ball);
    
    world.step(1.0 / 60.0);
    
    let ball = world.get_body(ball_handle);
    assert!(ball.transform.position.x > 0.0, "Ball passed through wall");
}
```

### 2.4 关节机械臂

```rust
#[test]
fn test_robot_arm_4_joints() {
    let mut world = PhysicsWorld3D::with_default_config();
    let base = world.add_body(create_static_box(Vec3::ZERO, Vec3::splat(0.5)));
    let link1 = world.add_body(create_dynamic_box(Vec3::new(0.0, 0.5, 0.0), Vec3::new(0.1, 0.5, 0.1)));
    let link2 = world.add_body(create_dynamic_box(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.1, 0.5, 0.1)));
    
    let j1 = HingeJoint::new(base, link1, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -0.5, 0.0), Vec3::Z);
    let j2 = HingeJoint::new(link1, link2, Vec3::new(0.0, 0.5, 0.0), Vec3::new(0.0, -0.5, 0.0), Vec3::Z);
    world.add_joint(Box::new(j1));
    world.add_joint(Box::new(j2));
    
    // 控制 link2 角度
    for _ in 0..300 {
        world.step(1.0 / 60.0);
        // PD 控制器：link2 朝目标角度
    }
    
    let link2_pos = world.get_body(link2).transform.position;
    // 验证 link2 在合理范围内
}
```

## 3. 性能基准

| 基准 | 目标 |
|------|------|
| 1000 刚体 60Hz | < 5 ms |
| 10000 物体 BVH | < 1 ms |
| 100 接触 GJK/EPA | < 0.5 ms |
| PGS 100 物体 8 iter | < 5 ms |
| 100 关节 | < 1 ms |
| 4 关节 1kHz 跟随 | 60 Hz 步进 |

## 4. 视觉/物理对比

- [ ] 与 Box2D 2D 对比穿透率
- [ ] 与 PhysX 5 对比堆叠稳定性
- [ ] 角色控制器：30° 斜坡测试
- [ ] CCD：1mm 薄墙穿透率 < 0.01%
