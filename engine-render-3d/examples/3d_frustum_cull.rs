//! 视锥裁剪演示示例
//!
//! 演示如何用 Frustum 进行视锥裁剪，并打印裁剪统计。

use engine_math::Vec3;
use engine_render_3d::{AABB, Camera3D, Frustum, Mesh3D, Node3D, Scene3D, Transform3D};

fn main() {
    println!("=== 3D Frustum Culling Example ===\n");

    // 创建相机
    let mut camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 50.0);
    camera.set_position(Vec3::new(0.0, 2.0, 8.0));
    camera.look_at(Vec3::ZERO);

    // 计算视锥
    let vp = camera.view_projection();
    let frustum = Frustum::from_view_projection(&vp);

    println!("Camera configuration:");
    println!("  Position: {:?}", camera.position());
    println!("  Forward: {:?}", camera.forward());
    println!("  FOV: {}°", camera.fov());
    println!("  Aspect: {}", camera.aspect());
    println!("  Near: {}, Far: {}", camera.near(), camera.far());

    // 创建场景：放置一排立方体，一部分在视锥内，一部分在视锥外
    let mut scene = Scene3D::new();

    let positions = [
        // 近处视锥内
        ("near_visible_1", Vec3::new(-3.0, 0.0, 0.0)),
        ("near_visible_2", Vec3::new(0.0, 0.0, 0.0)),
        ("near_visible_3", Vec3::new(3.0, 0.0, 0.0)),
        // 视锥外左侧
        ("left_culled", Vec3::new(-20.0, 0.0, 0.0)),
        // 视锥外右侧
        ("right_culled", Vec3::new(20.0, 0.0, 0.0)),
        // 视锥外远处
        ("far_culled", Vec3::new(0.0, 0.0, 30.0)),
        // 视锥外近处
        ("near_culled", Vec3::new(0.0, 0.0, 0.5)),
        // 视锥外高处
        ("top_culled", Vec3::new(0.0, 15.0, 0.0)),
        // 视锥外低处
        ("bottom_culled", Vec3::new(0.0, -15.0, 0.0)),
    ];

    for (name, pos) in &positions {
        let mut node = Node3D::with_name(*name);
        let mut transform = Transform3D::new();
        transform.set_translation(*pos);
        node.set_local_transform(transform);
        scene.add_node(node);
    }

    scene.update_world_transforms();

    println!("\nScene: {} nodes", scene.nodes().len());

    // 执行视锥裁剪
    let mut visible_count = 0;
    let mut culled_count = 0;

    println!("\nCulling results:");
    println!("{:-<60}", "");
    println!("{:<20} | {:<10} | {:>10}", "Name", "Status", "Distance");
    println!("{:-<60}", "");

    for (i, (name, pos)) in positions.iter().enumerate() {
        // 构造 AABB
        let aabb = AABB::new(
            *pos + Vec3::new(-0.5, -0.5, -0.5),
            *pos + Vec3::new(0.5, 0.5, 0.5),
        );

        let visible = frustum.contains_aabb(&aabb);
        let distance = (*pos - camera.position()).length();

        let status = if visible { "VISIBLE" } else { "CULLED" };
        if visible {
            visible_count += 1;
        } else {
            culled_count += 1;
        }

        println!("{:<20} | {:<10} | {:>10.2}", name, status, distance);
        let _ = i;
    }

    println!("{:-<60}", "");
    println!("Total: {} visible, {} culled", visible_count, culled_count);

    // 演示多种几何类型的裁剪测试
    println!("\nGeometry-specific tests:");
    let sphere_radius = 1.0;
    let test_sphere = engine_render_3d::Sphere::new(Vec3::new(0.0, 0.0, 0.0), sphere_radius);
    let sphere_in_frustum = frustum.contains_sphere(&test_sphere);
    println!(
        "  Sphere at origin (r={}): {}",
        sphere_radius,
        if sphere_in_frustum { "visible" } else { "culled" }
    );

    let plane = engine_render_3d::Plane::from_normal_and_point(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let point_above = Vec3::new(0.0, 5.0, 0.0);
    let point_below = Vec3::new(0.0, -5.0, 0.0);
    println!(
        "  Plane distance above: {:.2}",
        plane.distance(point_above)
    );
    println!(
        "  Plane distance below: {:.2}",
        plane.distance(point_below)
    );

    // 演示射线与 AABB 求交
    println!("\nRay-AABB intersection test:");
    let ray = engine_render_3d::Ray3::new(Vec3::new(0.0, 0.0, 8.0), Vec3::new(0.0, 0.0, -1.0));
    let aabb = AABB::new(Vec3::new(-0.5, -0.5, -1.5), Vec3::new(0.5, 0.5, -0.5));
    if let Some(t) = ray.hit_aabb(&aabb) {
        let hit_point = ray.at(t);
        println!("  Ray hits AABB at t={:.3}, point={:?}", t, hit_point);
    } else {
        println!("  Ray does not hit AABB");
    }

    // 演示射线与球体求交
    println!("\nRay-Sphere intersection test:");
    let ray = engine_render_3d::Ray3::new(Vec3::new(5.0, 0.0, 8.0), Vec3::new(-1.0, 0.0, 0.0));
    let sphere = engine_render_3d::Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    if let Some(t) = ray.hit_sphere(&sphere) {
        let hit_point = ray.at(t);
        println!("  Ray hits sphere at t={:.3}, point={:?}", t, hit_point);
    } else {
        println!("  Ray does not hit sphere");
    }

    // 演示射线与三角形求交（Möller-Trumbore）
    println!("\nRay-Triangle intersection test:");
    let ray = engine_render_3d::Ray3::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
    let v0 = Vec3::new(-1.0, -1.0, 0.0);
    let v1 = Vec3::new(1.0, -1.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    if let Some(t) = ray.hit_triangle(v0, v1, v2) {
        let hit_point = ray.at(t);
        println!("  Ray hits triangle at t={:.3}, point={:?}", t, hit_point);
    } else {
        println!("  Ray does not hit triangle");
    }

    // 演示 Mesh3D 数据
    let _mesh = Mesh3D::cube(1.0);
    println!("\nMesh data:");
    println!("  Vertices: {}", _mesh.vertices());
    println!("  Triangles: {}", _mesh.triangles());
    println!("  AABB: min={:?}, max={:?}", _mesh.aabb().min(), _mesh.aabb().max());

    println!("\n=== example complete ===");
}
