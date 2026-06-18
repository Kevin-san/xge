//! transform_stack 示例 - 演示矩阵变换栈
//!
//! 本示例演示如何使用 push_transform 和 pop_transform 管理矩阵栈，实现嵌套变换。

use engine_core::{Engine, EngineConfig};
use engine_math::{Mat4, Vec3};
use engine_render::RenderContext;

fn main() {
    println!("Transform Stack Example");
    println!("======================");

    let config = EngineConfig {
        window_title: "Transform Stack Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    let _engine = Engine::new(config);
    let mut ctx = RenderContext::new();

    // Create some transformation matrices
    let translation = Mat4::from_translation(Vec3::new(100.0, 100.0, 0.0));
    let rotation = Mat4::from_rotation_z(std::f32::consts::PI / 4.0); // 45 degrees
    let scale = Mat4::from_scale(Vec3::new(2.0, 2.0, 1.0));

    // Push transformations onto the stack
    ctx.push_transform(translation);
    println!("Pushed translation matrix (100, 100, 0)");
    println!("Stack depth: {}", ctx.transform_stack_len());

    // Nested transform: rotate within translated space
    ctx.push_transform(rotation);
    println!("Pushed rotation matrix (45 degrees)");
    println!("Stack depth: {}", ctx.transform_stack_len());

    // Nested transform: scale within rotated space
    ctx.push_transform(scale);
    println!("Pushed scale matrix (2x, 2y, 1z)");
    println!("Stack depth: {}", ctx.transform_stack_len());

    // Draw something with combined transform
    // The final transform = scale * rotation * translation
    let _current_transform = ctx.current_transform();
    println!("\nCurrent transform computed (combined):");

    // Pop transforms
    ctx.pop_transform();
    println!("\nPopped scale");
    println!("Stack depth: {}", ctx.transform_stack_len());

    ctx.pop_transform();
    println!("Popped rotation");
    println!("Stack depth: {}", ctx.transform_stack_len());

    ctx.pop_transform();
    println!("Popped translation");
    println!("Stack depth: {}", ctx.transform_stack_len());

    // Verify stack is empty
    assert_eq!(
        ctx.transform_stack_len(),
        0,
        "Transform stack should be empty"
    );

    println!("\nExample demonstrates:");
    println!("  - push_transform() to save current transform and apply new one");
    println!("  - pop_transform() to restore previous transform");
    println!("  - Nested transforms combine via matrix multiplication");
    println!("  - Useful for hierarchical object transformations");
    println!("  - Common use: 2D UI systems, sprite hierarchies");

    println!("\nTransform order: new_transform * current_transform");
    println!("  This means transformations are applied in reverse order");
    println!("  (last pushed is applied first)");
}
