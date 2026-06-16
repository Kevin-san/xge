//! 3D 基本图元演示示例
//!
//! 展示所有 3D 渲染核心支持的基本图元：立方体、球体、平面、圆柱、圆锥、圆环、胶囊。
//!
//! 这是一个 headless 示例，演示如何创建各种图元并打印其顶点和三角面数量。

use engine_math::Vec3;
use engine_render_3d::{
    Camera3D, DirectionalLight, LightManager, Mesh3D, Node3D, PointLight, Scene3D,
    Transform3D,
};

fn main() {
    println!("=== engine-render-3d primitives demo ===\n");

    // 演示所有基本图元
    let primitives: Vec<(&str, Mesh3D)> = vec![
        ("Cube", Mesh3D::cube(1.0)),
        ("Sphere", Mesh3D::sphere(1.0, 16, 8)),
        ("Plane", Mesh3D::plane(1.0, 4)),
        ("Cylinder", Mesh3D::cylinder(0.5, 1.0, 16)),
        ("Cone", Mesh3D::cone(0.5, 1.0, 16)),
        ("Torus", Mesh3D::torus(0.7, 0.3, 16, 8)),
        ("Capsule", Mesh3D::capsule(0.3, 0.8, 8)),
    ];

    println!("Primitive statistics:");
    println!("{:-<50}", "");
    println!("{:<12} | {:>10} | {:>10} | {:>10}", "Name", "Vertices", "Indices", "Triangles");
    println!("{:-<50}", "");

    for (name, mesh) in &primitives {
        println!(
            "{:<12} | {:>10} | {:>10} | {:>10}",
            name,
            mesh.vertices(),
            mesh.indices(),
            mesh.triangles()
        );
    }

    println!();

    // 创建场景
    let mut scene = Scene3D::new();
    let mut light_manager = LightManager::new(2, 4, 4);

    // 添加方向光
    let dir_light = DirectionalLight::new(
        Vec3::new(-0.3, -1.0, -0.5).normalize(),
        engine_math::Vec4::new(1.0, 0.95, 0.9, 1.0),
        1.0,
    );
    light_manager.add_directional(dir_light);

    // 添加点光源
    let point_light = PointLight::new(
        Vec3::new(0.0, 3.0, 0.0),
        engine_math::Vec4::new(1.0, 0.8, 0.6, 1.0),
        5.0,
        20.0,
    );
    light_manager.add_point(point_light);

    scene.light_manager_mut().set_ambient(
        engine_render_3d::AmbientLight::new(
            engine_math::Vec4::new(0.1, 0.1, 0.15, 1.0),
            1.0,
        ),
    );

    // 创建节点并排放置所有图元
    let primitive_count = primitives.len() as f32;
    let spacing = 2.5;
    let total_width = (primitive_count - 1.0) * spacing;
    let start_x = -total_width / 2.0;

    for (i, (name, _)) in primitives.iter().enumerate() {
        let mut node = Node3D::with_name(*name);
        let x = start_x + i as f32 * spacing;
        let mut transform = Transform3D::new();
        transform.set_translation(Vec3::new(x, 0.0, 0.0));
        node.set_local_transform(transform);
        scene.add_node(node);
    }

    // 创建相机
    let mut camera = Camera3D::perspective(60.0, 16.0 / 9.0, 0.1, 100.0);
    camera.set_position(Vec3::new(0.0, 3.0, 8.0));
    let target = Vec3::ZERO;
    camera.look_at(target);

    // 更新世界变换
    scene.update_world_transforms();

    // 演示视锥裁剪
    let vp = camera.view_projection();
    let frustum = engine_render_3d::Frustum::from_view_projection(&vp);

    println!("Scene statistics:");
    let stats = scene.stats();
    println!("  Total nodes: {}", stats.nodes);
    println!("  Visible nodes: {}", stats.visible_nodes);
    println!();

    println!("Frustum culling:");
    for (i, (name, _)) in primitives.iter().enumerate() {
        let x = start_x + i as f32 * spacing;
        let node_pos = Vec3::new(x, 0.0, 0.0);
        let aabb = engine_render_3d::AABB::new(
            node_pos + Vec3::new(-0.5, -0.5, -0.5),
            node_pos + Vec3::new(0.5, 0.5, 0.5),
        );
        let visible = frustum.contains_aabb(&aabb);
        println!("  {}: {}", name, if visible { "visible" } else { "CULLED" });
    }

    println!();
    println!("Light configuration:");
    println!("  Directional lights: {}", light_manager.directional_count());
    println!("  Point lights: {}", light_manager.point_count());
    println!("  Spot lights: {}", light_manager.spot_count());
    println!("  Ambient: {:?}", light_manager.ambient().color());

    println!();
    println!("=== demo complete ===");
}
