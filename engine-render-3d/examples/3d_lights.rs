//! 多光源演示示例
//!
//! 展示方向光、点光源、聚光灯的组合使用。

use engine_math::{Vec3, Vec4};
use engine_render_3d::{
    AmbientLight, DirectionalLight, HemisphereLight, LightManager, LightType, Mesh3D,
    Node3D, PointLight, SpotLight, Transform3D,
};

fn main() {
    println!("=== 3D Lights Example ===\n");

    // 创建一个完整的光照场景
    let mut light_manager = LightManager::new(2, 4, 4);

    // 1. 主方向光（暖白色阳光）
    let sun = DirectionalLight::new(
        Vec3::new(-0.3, -1.0, -0.5).normalize(),
        Vec4::new(1.0, 0.95, 0.85, 1.0),
        1.2,
    );
    let sun_idx = light_manager.add_directional(sun).unwrap();
    println!("Added directional light at index {}", sun_idx);

    // 2. 蓝色点光源（冷色对比）
    let blue_light = PointLight::new(
        Vec3::new(3.0, 2.0, 0.0),
        Vec4::new(0.3, 0.5, 1.0, 1.0),
        10.0,
        20.0,
    );
    let blue_idx = light_manager.add_point(blue_light).unwrap();
    println!("Added point light at index {}", blue_idx);

    // 3. 红色聚光灯（戏剧效果）
    let red_spot = SpotLight::new(
        Vec3::new(0.0, 4.0, 4.0),
        Vec3::new(0.0, -1.0, -0.3).normalize(),
        15.0_f32.to_radians(),
        30.0_f32.to_radians(),
        Vec4::new(1.0, 0.3, 0.2, 1.0),
        20.0,
        30.0,
    );
    let red_idx = light_manager.add_spot(red_spot).unwrap();
    println!("Added spot light at index {}", red_idx);

    // 4. 半球光（天/地色）
    let _hemi = HemisphereLight::new(
        Vec4::new(0.5, 0.6, 0.9, 1.0), // 天空
        Vec4::new(0.4, 0.3, 0.2, 1.0), // 地面
        0.3,
    );

    // 5. 环境光
    light_manager.set_ambient(AmbientLight::new(
        Vec4::new(0.05, 0.05, 0.1, 1.0),
        0.5,
    ));

    println!("\nLight manager configuration:");
    println!("  Directional: {}", light_manager.directional_count());
    println!("  Point: {}", light_manager.point_count());
    println!("  Spot: {}", light_manager.spot_count());
    println!(
        "  Ambient color: {:?}",
        light_manager.ambient().color()
    );

    // 演示点光源的距离衰减
    println!("\nPoint light attenuation:");
    let point = &light_manager.point_lights()[0];
    for distance in &[1.0, 5.0, 10.0, 15.0, 25.0] {
        let atten = point.attenuation(*distance);
        println!("  distance={:>5.1}: attenuation={:.3}", distance, atten);
    }

    // 演示聚光灯的圆锥衰减
    println!("\nSpot light cone attenuation:");
    let spot = &light_manager.spot_lights()[0];
    let angles: &[f32] = &[0.0, 10.0, 20.0, 25.0, 35.0, 45.0, 60.0];
    for &angle_deg in angles {
        let angle_rad = angle_deg.to_radians();
        let dir_to_point = Vec3::new(
            angle_rad.sin(),
            -angle_rad.cos(),
            -0.3 * angle_rad.sin(),
        )
        .normalize();
        let atten = spot.cone_attenuation(dir_to_point);
        println!("  angle={:>5.1}°: cone_attenuation={:.3}", angle_deg, atten);
    }

    // 演示 LightType 枚举
    println!("\nLightType usage:");
    let types = [LightType::Directional, LightType::Point, LightType::Spot];
    for t in &types {
        println!("  {:?}", t);
    }

    // 创建一个简单的场景验证
    println!("\nBuilding scene with lights...");
    let mut scene_nodes = vec![];

    // 地板
    let mut ground = Node3D::with_name("ground");
    let mut ground_t = Transform3D::new();
    ground_t.set_translation(Vec3::new(0.0, -1.0, 0.0));
    ground.set_local_transform(ground_t);
    scene_nodes.push(ground);

    // 中心立方体
    let mut cube = Node3D::with_name("cube");
    scene_nodes.push(cube);

    // 测试点光源附近的物体
    let mut light_marker = Node3D::with_name("blue_light_zone");
    let mut light_t = Transform3D::new();
    light_t.set_translation(Vec3::new(3.0, 2.0, 0.0));
    light_marker.set_local_transform(light_t);
    scene_nodes.push(light_marker);

    println!("  Created {} scene nodes", scene_nodes.len());

    // 验证 Mesh3D 可用
    let _test = Mesh3D::cube(1.0);
    println!("  Cube created: {} vertices", _test.vertices());

    println!("\n=== example complete ===");
}
