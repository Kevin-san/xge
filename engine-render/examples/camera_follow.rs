//! camera_follow 示例 - 相机跟随
//!
//! 本示例演示如何使用 Camera2D 的目标跟随功能。

use engine_math::Vec2;
use engine_render::{Camera2D, OrthographicCamera};

fn main() {
    println!("Camera Follow Example");
    println!("=====================");

    // Create a 2D camera
    let mut camera = Camera2D::from_window(1280, 720, 1.0);

    println!("\nCamera initial state:");
    println!("  Position: {:?}", camera.position());
    println!("  Zoom: {}", camera.zoom());
    println!("  Target: {:?}", camera.target());

    // Set a target for the camera to follow
    let target_pos = Vec2::new(500.0, 300.0);
    camera.set_target(Some(target_pos));
    println!("\nSet target to: {:?}", target_pos);

    // Simulate entity movement (target moves)
    let mut target_position;
    println!("\nSimulating camera follow...");
    println!("Frame | Target Pos    | Camera Pos    | Delta");
    println!("------|---------------|---------------|-------");

    for frame in 0..10 {
        // Move target in a circle
        let angle = frame as f32 * 0.5;
        target_position = Vec2::new(500.0 + 200.0 * angle.cos(), 300.0 + 100.0 * angle.sin());
        camera.set_target(Some(target_position));

        // Update camera (with smoothing)
        camera.update(0.016); // ~60fps timestep

        let delta = target_position - camera.position();

        if frame % 3 == 0 {
            println!(
                "  {:4} | ({:7.1}, {:7.1}) | ({:7.1}, {:7.1}) | ({:6.2}, {:6.2})",
                frame,
                target_position.x,
                target_position.y,
                camera.position().x,
                camera.position().y,
                delta.x,
                delta.y
            );
        }
    }

    // Test OrthographicCamera as well
    println!("\n--- OrthographicCamera Test ---");
    let mut ortho = OrthographicCamera::from_window(1280, 720, 1.0);

    println!("OrthographicCamera initial:");
    println!("  Position: {:?}", ortho.position());

    // Move camera
    ortho.move_by(Vec2::new(100.0, 50.0));
    println!("After move_by(100, 50): {:?}", ortho.position());

    // Zoom
    ortho.zoom(2.0);
    println!("After zoom(2.0):");

    // Test coordinate conversion
    let screen_pos = Vec2::new(640.0, 360.0);
    let world_pos = ortho.screen_to_world(screen_pos);
    let back_to_screen = ortho.world_to_screen(world_pos);

    println!("\nCoordinate conversion test:");
    println!("  Screen: ({}, {})", screen_pos.x, screen_pos.y);
    println!("  World:  ({}, {})", world_pos.x, world_pos.y);
    println!("  Back:   ({}, {})", back_to_screen.x, back_to_screen.y);

    println!("\nCamera follow example completed successfully!");
}
