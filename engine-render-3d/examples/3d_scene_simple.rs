//! 简单 3D 场景示例
//!
//! 演示一个包含地面、立方体、球体、点光源的简单 3D 场景。

use engine_math::{Vec3, Vec4};
use engine_render_3d::{
    AmbientLight, Camera3D, DirectionalLight, LightManager, Mesh3D, Node3D, PointLight,
    RenderPipeline3D, Scene3D, Transform3D,
};

fn build_scene() -> Scene3D {
    let mut scene = Scene3D::new();

    // 配置光源
    let mut light_manager = LightManager::new(2, 8, 4);

    // 主方向光（太阳光）
    let sun = DirectionalLight::new(
        Vec3::new(-0.5, -1.0, -0.3).normalize(),
        Vec4::new(1.0, 0.95, 0.85, 1.0),
        1.5,
    );
    light_manager.add_directional(sun);

    // 蓝色点光源
    let blue_light = PointLight::new(
        Vec3::new(3.0, 2.0, 0.0),
        Vec4::new(0.4, 0.6, 1.0, 1.0),
        8.0,
        15.0,
    );
    light_manager.add_point(blue_light);

    // 暖色点光源
    let warm_light = PointLight::new(
        Vec3::new(-3.0, 2.0, 2.0),
        Vec4::new(1.0, 0.7, 0.4, 1.0),
        6.0,
        12.0,
    );
    light_manager.add_point(warm_light);

    // 环境光
    light_manager.set_ambient(AmbientLight::new(
        Vec4::new(0.1, 0.12, 0.18, 1.0),
        1.0,
    ));

    *scene.light_manager_mut() = light_manager;

    // 创建地面
    let _ground = Mesh3D::plane(20.0, 16);

    // 创建中心立方体
    let _cube = Mesh3D::cube(1.5);

    // 创建几个球体
    let _sphere1 = Mesh3D::sphere(0.8, 16, 8);
    let _sphere2 = Mesh3D::sphere(0.6, 16, 8);

    // 场景节点

    // 地面
    let mut ground_node = Node3D::with_name("ground");
    let mut ground_transform = Transform3D::new();
    ground_transform.set_translation(Vec3::new(0.0, -1.0, 0.0));
    ground_node.set_local_transform(ground_transform);
    scene.add_node(ground_node);

    // 中心立方体
    let mut cube_node = Node3D::with_name("cube");
    let mut cube_transform = Transform3D::new();
    cube_transform.set_translation(Vec3::new(0.0, 0.0, 0.0));
    cube_node.set_local_transform(cube_transform);
    scene.add_node(cube_node);

    // 球体 1
    let mut sphere1_node = Node3D::with_name("sphere_blue");
    let mut sphere1_transform = Transform3D::new();
    sphere1_transform.set_translation(Vec3::new(3.0, 0.0, 0.0));
    sphere1_node.set_local_transform(sphere1_transform);
    scene.add_node(sphere1_node);

    // 球体 2
    let mut sphere2_node = Node3D::with_name("sphere_warm");
    let mut sphere2_transform = Transform3D::new();
    sphere2_transform.set_translation(Vec3::new(-3.0, 0.0, 2.0));
    sphere2_node.set_local_transform(sphere2_transform);
    scene.add_node(sphere2_node);

    scene
}

fn main() {
    println!("=== 3D Scene Simple Example ===\n");

    let mut scene = build_scene();

    // 创建相机
    let mut camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 100.0);
    camera.set_position(Vec3::new(5.0, 4.0, 8.0));
    camera.look_at(Vec3::ZERO);

    // 更新世界变换
    scene.update_world_transforms();

    // 创建渲染管线
    let mut pipeline = RenderPipeline3D::new();
    pipeline.init();
    pipeline.set_clear_color(Vec4::new(0.05, 0.07, 0.12, 1.0));

    // 模拟一帧渲染
    pipeline.begin_frame();
    pipeline.prepare_scene(&scene, &camera, scene.light_manager());

    let stats = pipeline.stats();
    println!("Frame statistics:");
    println!("  Entities rendered: {}", stats.entities_rendered);
    println!("  Entities culled: {}", stats.entities_culled);
    println!("  Draw calls: {}", stats.draw_calls);
    println!("  Triangles: {}", stats.triangles);
    println!("  Vertices: {}", stats.vertices);

    pipeline.end_frame();

    // 演示矩阵操作
    println!("\nCamera matrices:");
    let view = camera.view_matrix();
    let proj = camera.projection_matrix();
    let vp = camera.view_projection();

    println!("  View matrix: cols[0] = {:?}", view.cols[0]);
    println!("  Projection matrix: cols[0] = {:?}", proj.cols[0]);
    println!("  VP matrix: cols[0] = {:?}", vp.cols[0]);

    // 演示射线拾取
    println!("\nRay picking demo:");
    let screen_pos = engine_math::Vec2::new(640.0, 360.0);
    let screen_size = engine_math::Vec2::new(1280.0, 720.0);
    let ray = camera.screen_to_world_ray(screen_pos, screen_size);

    println!("  Screen: ({}, {})", screen_pos.x, screen_pos.y);
    println!("  Ray origin: {:?}", ray.origin());
    println!("  Ray direction: {:?}", ray.direction());

    // 模拟射线与中心立方体相交
    let cube_pos = Vec3::ZERO;
    let cube_aabb = engine_render_3d::AABB::new(
        cube_pos + Vec3::new(-0.75, -0.75, -0.75),
        cube_pos + Vec3::new(0.75, 0.75, 0.75),
    );

    if let Some(t) = ray.hit_aabb(&cube_aabb) {
        let hit_point = ray.at(t);
        println!("  Ray hits cube at t={:.3}", t);
        println!("  Hit point: {:?}", hit_point);
    } else {
        println!("  Ray does not hit cube");
    }

    println!("\n=== example complete ===");
}
