//! 3D 拾取示例
//!
//! 演示使用 Ray3 进行网格拾取和射线相交。

use engine_math::Vec3;
use engine_render_3d::{Camera3D, Mesh3D, Ray3};

fn main() {
    println!("=== 3D Picker Example ===\n");

    // 创建相机
    let mut camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 100.0);
    camera.set_position(Vec3::new(0.0, 3.0, 5.0));
    camera.look_at(Vec3::ZERO);

    println!("Camera: pos={:?}, looking at origin", camera.position());

    // 测试多个屏幕坐标的射线拾取
    let screen_size = engine_math::Vec2::new(1920.0, 1080.0);
    let test_points = [
        ("center", engine_math::Vec2::new(960.0, 540.0)),
        ("top-left", engine_math::Vec2::new(100.0, 100.0)),
        ("bottom-right", engine_math::Vec2::new(1820.0, 980.0)),
        ("cube-area", engine_math::Vec2::new(960.0, 500.0)),
    ];

    // 创建网格
    let cube = Mesh3D::cube(1.0);
    let cube_aabb = cube.aabb();

    let sphere = Mesh3D::sphere(0.8, 16, 8);
    let sphere_bounds = sphere.bounding_sphere();

    // 将物体摆放在不同位置
    struct PickableObject {
        name: String,
        mesh_kind: String,
        position: Vec3,
        aabb: engine_render_3d::AABB,
        sphere: engine_render_3d::Sphere,
    }

    let objects = vec![
        PickableObject {
            name: "cube_origin".to_string(),
            mesh_kind: "cube".to_string(),
            position: Vec3::ZERO,
            aabb: cube_aabb,
            sphere: engine_render_3d::Sphere::new(Vec3::ZERO, 1.0),
        },
        PickableObject {
            name: "sphere_left".to_string(),
            mesh_kind: "sphere".to_string(),
            position: Vec3::new(-3.0, 0.0, 0.0),
            aabb: engine_render_3d::AABB::new(
                Vec3::new(-3.8, -0.8, -0.8),
                Vec3::new(-2.2, 0.8, 0.8),
            ),
            sphere: engine_render_3d::Sphere::new(Vec3::new(-3.0, 0.0, 0.0), 0.8),
        },
        PickableObject {
            name: "sphere_right".to_string(),
            mesh_kind: "sphere".to_string(),
            position: Vec3::new(3.0, 0.0, 0.0),
            aabb: engine_render_3d::AABB::new(
                Vec3::new(2.2, -0.8, -0.8),
                Vec3::new(3.8, 0.8, 0.8),
            ),
            sphere: engine_render_3d::Sphere::new(Vec3::new(3.0, 0.0, 0.0), 0.8),
        },
    ];

    for (label, screen_pos) in &test_points {
        println!("\nScreen point: {} ({}, {})", label, screen_pos.x, screen_pos.y);

        let ray = camera.screen_to_world_ray(*screen_pos, screen_size);
        println!("  Ray origin: {:?}", ray.origin());
        println!("  Ray direction: {:?}", ray.direction());

        // 收集所有命中
        let mut hits: Vec<(String, f32, Vec3)> = Vec::new();

        for obj in &objects {
            // 尝试 AABB 拾取
            if let Some(t) = ray.hit_aabb(&obj.aabb) {
                let point = ray.at(t);
                hits.push((obj.name.clone(), t, point));
            }
        }

        // 按距离排序
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        if hits.is_empty() {
            println!("  No hits");
        } else {
            println!("  Hits (sorted by distance):");
            for (name, t, point) in &hits {
                println!("    {} at t={:.3}, point={:?}", name, t, point);
            }
        }
    }

    // 演示不同类型的相交
    println!("\n=== Ray intersection types ===");

    // 射线与 AABB
    let ray = Ray3::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
    let aabb = engine_render_3d::AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    if let Some(t) = ray.hit_aabb(&aabb) {
        println!("AABB hit at t={:.3}", t);
    }

    // 射线与球体
    let ray = Ray3::new(Vec3::new(-3.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let sphere = engine_render_3d::Sphere::new(Vec3::ZERO, 1.0);
    if let Some(t) = ray.hit_sphere(&sphere) {
        println!("Sphere hit at t={:.3}", t);
    }

    // 射线与三角形
    let ray = Ray3::new(Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, -1.0));
    let v0 = Vec3::new(-1.0, -1.0, 0.0);
    let v1 = Vec3::new(1.0, -1.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    if let Some(t) = ray.hit_triangle(v0, v1, v2) {
        let p = ray.at(t);
        println!("Triangle hit at t={:.3}, point={:?}", t, p);
    }

    // 射线与平面
    let plane = engine_render_3d::Plane::from_normal_and_point(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::ZERO,
    );
    let ray = Ray3::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
    if let Some(t) = ray.hit_plane(&plane) {
        println!("Plane hit at t={:.3}", t);
    }

    // 演示未命中场景
    println!("\n=== Miss scenarios ===");
    let ray = Ray3::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 1.0, 0.0)); // 朝上
    let aabb = engine_render_3d::AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    if ray.hit_aabb(&aabb).is_none() {
        println!("Ray pointing up missed AABB (expected)");
    }

    println!("\n=== example complete ===");
}
