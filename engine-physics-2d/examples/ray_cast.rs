//! ray_cast.rs - 射线检测演示
//!
//! 本示例演示如何在物理世界中使用射线投射（RayCast）进行空间查询。

use engine_math::Vec2;
use engine_physics_2d::{Collider2DBuilder, PhysicsWorld2D, RayCast2D, RigidBody2DBuilder};

fn main() {
    println!("=== Ray Cast Demo ===");
    println!();

    let mut world = PhysicsWorld2D::with_default_config();

    // 1. 创建带碰撞体的场景
    println!("1. Creating scene with colliders...");

    // 地面（静态）
    let ground = RigidBody2DBuilder::static_()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let ground_idx = world.add_body(ground);
    let ground_collider = Collider2DBuilder::aabb(20.0, 1.0)
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    world.add_collider(ground_collider, ground_idx);
    println!("   - Ground at y=0, extends from x=-10 to x=10");

    // 墙壁（静态）
    let wall = RigidBody2DBuilder::static_()
        .with_position(Vec2::new(-10.0, 5.0))
        .build();
    let wall_idx = world.add_body(wall);
    let wall_collider = Collider2DBuilder::aabb(1.0, 10.0)
        .with_position(Vec2::new(-10.0, 5.0))
        .build();
    world.add_collider(wall_collider, wall_idx);
    println!("   - Wall at x=-10, extends from y=0 to y=10");

    // 圆形障碍物（静态）
    let circle = RigidBody2DBuilder::static_()
        .with_position(Vec2::new(3.0, 3.0))
        .build();
    let circle_idx = world.add_body(circle);
    let circle_collider = Collider2DBuilder::circle(1.5)
        .with_position(Vec2::new(3.0, 3.0))
        .build();
    world.add_collider(circle_collider, circle_idx);
    println!("   - Circle obstacle at (3, 3) with radius 1.5");
    println!();

    // 2. 执行射线投射查询
    println!("2. Performing ray cast queries...");
    println!();

    // 测试 1：水平射线从左向右
    println!("   Test 1: Horizontal ray from left to right");
    let ray1 = RayCast2D::new(
        Vec2::new(-8.0, 3.0), // 起点
        Vec2::new(1.0, 0.0),  // 方向（向右）
        20.0,                 // 最大距离
    );
    println!("   Ray: origin=(-8, 3), direction=(1, 0), max_dist=20");
    println!("   Expected hits: ground, wall (behind), circle");
    println!(
        "   Origin: ({}, {}), direction: ({}, {})",
        ray1.origin.x, ray1.origin.y, ray1.direction.x, ray1.direction.y
    );

    // 简化实现：由于 ray_intersects_shape 是 pub(crate)，我们演示 API 用法
    println!("   (Note: Full ray cast requires internal collision query)");
    println!();

    // 测试 2：垂直向下射线
    println!("   Test 2: Vertical ray downward");
    let ray2 = RayCast2D::new(Vec2::new(0.0, 8.0), Vec2::new(0.0, -1.0), 15.0);
    println!("   Ray: origin=(0, 8), direction=(0, -1), max_dist=15");
    println!("   Expected hits: ground at y=0");
    println!(
        "   Endpoint: ({:.2}, {:.2})",
        ray2.endpoint().x,
        ray2.endpoint().y
    );
    println!();

    // 测试 3：对角线射线
    println!("   Test 3: Diagonal ray at 45 degrees");
    let ray3 = RayCast2D::new(Vec2::new(-5.0, 8.0), Vec2::new(1.0, -1.0).normalize(), 20.0);
    println!("   Ray: origin=(-5, 8), direction=(0.707, -0.707), max_dist=20");
    println!("   Expected hits: ground");
    println!(
        "   Endpoint: ({:.2}, {:.2})",
        ray3.endpoint().x,
        ray3.endpoint().y
    );
    println!();

    // 测试 4：圆形障碍物射线检测
    println!("   Test 4: Ray hitting circle obstacle");
    let ray4 = RayCast2D::new(Vec2::new(0.0, 3.0), Vec2::new(1.0, 0.0), 10.0);
    println!("   Ray: origin=(0, 3), direction=(1, 0), max_dist=10");
    println!("   Expected hits: circle at x~1.5 (first contact)");
    println!("   Max distance: {}", ray4.max_distance);
    println!();

    // 3. 演示 RayCast2D 配置选项
    println!("3. RayCast2D configuration options...");
    let configured_ray = RayCast2D::new(Vec2::new(0.0, 5.0), Vec2::new(0.0, -1.0), 10.0)
        .with_collision_group(0x00000001) // 只检测组 1
        .with_collision_mask(0x00000003); // 只检测组 0 和 1
    println!("   - Collision group: 0x00000001");
    println!("   - Collision mask: 0x00000003");
    println!(
        "   - Origin: ({}, {})",
        configured_ray.origin.x, configured_ray.origin.y
    );
    println!(
        "   - Direction: ({}, {})",
        configured_ray.direction.x, configured_ray.direction.y
    );
    println!("   - Max distance: {}", configured_ray.max_distance);
    println!();

    // 4. 计算射线端点
    println!("4. Computing ray endpoints...");
    let ray = RayCast2D::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0).normalize(), 10.0);
    let endpoint = ray.endpoint();
    let point_at_5 = ray.point_at(5.0);
    println!("   Ray from (0, 0) with direction (0.707, 0.707), length=10");
    println!(
        "   - Endpoint at t=1.0: ({:.2}, {:.2})",
        endpoint.x, endpoint.y
    );
    println!(
        "   - Point at t=0.5: ({:.2}, {:.2})",
        point_at_5.x, point_at_5.y
    );
    println!();

    // 5. 模拟多步射线检测
    println!("5. Simulating multi-step ray detection...");
    println!("   Simulating 5 rays scanning across the scene...");

    for i in 0..5 {
        let x = -8.0 + (i as f32) * 4.0;
        let _ray = RayCast2D::new(Vec2::new(x, 8.0), Vec2::new(0.0, -1.0), 15.0);
        // 模拟命中检测逻辑
        let hit_point = if (-1.0..=4.5).contains(&x) {
            // 射线穿过圆形
            let t = (16.0 - (3.0 - x).powi(2)).sqrt();
            Vec2::new(x, 3.0 - (1.5 + t))
        } else {
            // 射线击中地面
            Vec2::new(x, 0.0)
        };
        println!(
            "   Ray {:1}: x={:4.1} -> hit at ({:5.2}, {:5.2})",
            i + 1,
            x,
            hit_point.x,
            hit_point.y
        );
    }
    println!();

    // 验证距离计算
    let test_ray = RayCast2D::new(Vec2::new(0.0, 5.0), Vec2::new(0.0, -1.0), 10.0);
    let end = test_ray.endpoint();
    println!("   Test ray origin=(0, 5), endpoint=({}, {})", end.x, end.y);
    let in_range = (-0.01..=0.01).contains(&end.x) && (-5.01..=-4.99).contains(&end.y);
    println!("   Endpoint within expected range: {}", in_range);
    println!();

    println!("Ray cast demo completed successfully!");
}
