//! scissor 示例 - 演示剪刀矩形裁剪
//!
//! 本示例演示如何使用 push_scissor_rect 和 pop_scissor_rect 来限制渲染区域。

use engine_core::{Engine, EngineConfig};
use engine_math::Vec2;
use engine_render::{Rect, RenderContext};

fn main() {
    println!("Scissor Example");
    println!("===============");

    let config = EngineConfig {
        window_title: "Scissor Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    let _engine = Engine::new(config);
    let mut ctx = RenderContext::new();

    // Push scissor rect to limit rendering area
    let scissor_area = Rect::new(100.0, 100.0, 400.0, 300.0);
    ctx.push_scissor(scissor_area);
    println!("Pushed scissor: {:?}", scissor_area);

    // Draw content within scissor area
    // (In actual implementation, draws would be clipped to the scissor rect)

    // Nested scissor for smaller area
    let inner_scissor = Rect::new(150.0, 150.0, 200.0, 150.0);
    ctx.push_scissor(inner_scissor);
    println!("Pushed inner scissor: {:?}", inner_scissor);

    // Pop the inner scissor
    ctx.pop_scissor();
    println!("Popped inner scissor");

    // Pop the outer scissor
    ctx.pop_scissor();
    println!("Popped outer scissor");

    println!("\nScissor stack depth: {}", ctx.scissor_stack_len());

    println!("\nExample demonstrates:");
    println!("  - push_scissor() to limit rendering to a rectangular area");
    println!("  - pop_scissor() to restore previous scissor rect");
    println!("  - Nested scissors for complex clipping regions");
    println!("  - Common use cases: UI clipping, minimap rendering, split-screen");

    println!("\nNote: Scissor affects OpenGL rendering when enabled.");
    println!("The scissors stack allows nested clipping regions.");
}
