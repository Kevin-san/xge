//! debug_draw 示例 - 绘制调试信息
//!
//! 本示例演示如何使用 DebugRenderer 绘制线条、矩形、圆形等调试图形。

use engine_core::{Engine, EngineConfig};
use engine_math::Vec2;
use engine_render::{Camera2D, Color, DebugRenderer, Rect};

fn main() {
    println!("Debug Draw Example");
    println!("=================");

    let config = EngineConfig {
        window_title: "Debug Draw Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    let _engine = Engine::new(config);

    let mut debug_renderer = DebugRenderer::new();
    let _camera = Camera2D::new();

    // Draw some lines
    debug_renderer.line(Vec2::new(100.0, 100.0), Vec2::new(400.0, 100.0), Color::RED);
    debug_renderer.line(
        Vec2::new(100.0, 100.0),
        Vec2::new(100.0, 400.0),
        Color::GREEN,
    );
    debug_renderer.line(
        Vec2::new(100.0, 400.0),
        Vec2::new(400.0, 400.0),
        Color::BLUE,
    );
    debug_renderer.line(
        Vec2::new(400.0, 100.0),
        Vec2::new(400.0, 400.0),
        Color::YELLOW,
    );

    // Draw rectangles
    let rect1 = Rect::new(500.0, 100.0, 150.0, 100.0);
    debug_renderer.rect(rect1, Color::from_rgba(1.0, 0.0, 0.0, 0.3));

    let rect2 = Rect::new(500.0, 220.0, 150.0, 100.0);
    debug_renderer.rect_lines(rect2, Color::CYAN);

    // Draw circles
    debug_renderer.circle(Vec2::new(900.0, 200.0), 80.0, Color::ORANGE);
    debug_renderer.circle_lines(Vec2::new(900.0, 400.0), 60.0, Color::MAGENTA);

    // Draw cross at origin
    debug_renderer.cross(Vec2::ZERO, 40.0, Color::WHITE);

    // Draw grid
    debug_renderer.grid(Vec2::new(50.0, 500.0), 50.0, 5, 3, Color::GRAY);

    println!("\nDebug renderer statistics:");
    println!("  Lines: {}", debug_renderer.line_count());
    println!("  Rects: {}", debug_renderer.rect_count());
    println!("  Circles: {}", debug_renderer.circle_count());
    println!("  Enabled: {}", debug_renderer.is_enabled());

    println!("\nExample demonstrates:");
    println!("  - DebugRenderer line() for drawing lines");
    println!("  - DebugRenderer rect() and rect_lines() for rectangles");
    println!("  - DebugRenderer circle() and circle_lines() for circles");
    println!("  - DebugRenderer cross() for crosshair markers");
    println!("  - DebugRenderer grid() for drawing grids");
    println!("  - DebugRenderer flush() to render to screen");
    println!("  - DebugRenderer clear() to reset all debug shapes");
}
