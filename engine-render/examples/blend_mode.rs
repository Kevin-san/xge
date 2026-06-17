//! blend_mode 示例 - 演示不同的混合模式
//!
//! 本示例演示 BlendMode 的各种效果：Alpha、Additive、Subtract、Multiply、Replace、Invert、PreMultiplied。

use engine_core::{Engine, EngineConfig};
use engine_render::{BlendMode, RenderContext};

fn main() {
    println!("Blend Mode Example");
    println!("=================");

    let config = EngineConfig {
        window_title: "Blend Mode Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    let _engine = Engine::new(config);
    let mut ctx = RenderContext::new();

    // Demonstrate all blend modes
    let blend_modes = [
        ("Alpha", BlendMode::Alpha),
        ("Additive", BlendMode::Additive),
        ("Subtract", BlendMode::Subtract),
        ("Multiply", BlendMode::Multiply),
        ("Replace", BlendMode::Replace),
        ("Invert", BlendMode::Invert),
        ("PreMultiplied", BlendMode::PreMultiplied),
    ];

    println!("\nAvailable blend modes:");
    for (name, mode) in &blend_modes {
        ctx.set_blend_mode(*mode);
        println!("  - {}: {:?}", name, mode);
    }

    // Reset to default
    ctx.set_blend_mode(BlendMode::Alpha);

    println!("\nBlend mode descriptions:");
    println!("  Alpha: Standard alpha blending - src*a + dst*(1-a)");
    println!("  Additive: Light/glow effect - src*a + dst*1");
    println!("  Subtract: Subtract blending - dst*(1-a) - src*a");
    println!("  Multiply: Multiply blend - dst*src");
    println!("  Replace: Direct replacement - src only");
    println!("  Invert: Inverts destination - 1-dst");
    println!("  PreMultiplied: For pre-multiplied alpha textures");

    println!("\nExample demonstrates:");
    println!("  - RenderContext.set_blend_mode() to change blending");
    println!("  - Each blend mode affects how overlapping sprites are rendered");
    println!("  - Useful for special effects, particles, UI transitions");

    println!("\nNote: Visual effect requires actual OpenGL context.");
}
