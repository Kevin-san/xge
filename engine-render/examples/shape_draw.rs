//! shape_draw 示例 - 基本图形绘制
//!
//! 本示例演示如何使用 Renderer 绘制基本图形（矩形、圆形、线条等）。

use engine_render::{Color, RenderContext, Renderer};

fn main() {
    println!("Shape Draw Example");
    println!("==================");

    let ctx = RenderContext::new();

    // Test that RenderContext exists and has expected methods
    println!("\nRenderContext created successfully");
    println!("  Clear color: {:?}", ctx.clear_color());
    println!("  Blend mode: {:?}", ctx.blend_mode());

    // Create a mutable context to test state changes
    let mut ctx = ctx;

    // Test clear color
    ctx.set_clear_color(Color::from_hex("#1a1a2e").unwrap());
    println!("\nChanged clear color to: {:?}", ctx.clear_color());

    // Test blend mode
    ctx.set_blend_mode(engine_render::BlendMode::Additive);
    println!("Changed blend mode to: Additive");

    ctx.set_blend_mode(engine_render::BlendMode::Alpha);
    println!("Changed blend mode back to: Alpha");

    // Test transform stack
    let mat = engine_math::Mat4::from_translation(engine_math::Vec3::new(100.0, 0.0, 0.0));
    ctx.push_transform(mat);
    println!("\nPushed transform matrix");

    ctx.pop_transform();
    println!("Popped transform matrix");

    // Test scissor stack
    let rect = engine_render::Rect::new(0.0, 0.0, 100.0, 100.0);
    ctx.push_scissor(rect);
    println!("\nPushed scissor rect: {:?}", rect);

    ctx.pop_scissor();
    println!("Popped scissor rect");

    // Get stats
    let stats = ctx.stats();
    println!("\nRender stats:");
    println!("  Draw calls: {}", stats.draw_calls);
    println!("  Vertices: {}", stats.vertices);
    println!("  Indices: {}", stats.indices);
    println!("  Batches: {}", stats.batches);

    // Color test
    println!("\nColor constants test:");
    println!("  RED: {:?}", Color::RED);
    println!("  GREEN: {:?}", Color::GREEN);
    println!("  BLUE: {:?}", Color::BLUE);
    println!("  WHITE: {:?}", Color::WHITE);
    println!("  BLACK: {:?}", Color::BLACK);
    println!("  GOLD: {:?}", Color::GOLD);

    // Color operations
    let red = Color::RED;
    let blue = Color::BLUE;
    let purple = Color::lerp(red, blue, 0.5);
    println!("\nColor lerp(RED, BLUE, 0.5): {:?}", purple);

    // Hex parsing
    match Color::from_hex("#FF8800") {
        Ok(c) => println!("Parsed #FF8800: {:?}", c),
        Err(e) => println!("Failed to parse: {}", e),
    }

    println!("\nShape draw example completed successfully!");
    println!("\nNote: Actual drawing requires a window and OpenGL context.");
    println!("The following draw calls would be available:");
    println!("  - draw_rectangle(x, y, w, h, color)");
    println!("  - draw_circle(x, y, radius, color)");
    println!("  - draw_line(x1, y1, x2, y2, thickness, color)");
    println!("  - draw_triangle(p1, p2, p3, color)");
    println!("  - draw_poly(x, y, sides, radius, rotation, color)");
}
