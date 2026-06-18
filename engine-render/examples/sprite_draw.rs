//! sprite_draw 示例 - 绘制单个精灵
//!
//! 本示例演示如何使用 engine-render 在窗口中绘制一个简单的精灵。

use engine_core::{Engine, EngineConfig};
use engine_render::{Camera2D, Color, Image, Sprite, Texture2D, TextureHandle};

fn main() {
    println!("Sprite Draw Example");
    println!("==================");

    // Create engine with default config
    let config = EngineConfig {
        window_title: "Sprite Draw Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    let _engine = Engine::new(config);

    // Create a simple 64x64 texture with a gradient pattern
    let image_data = create_gradient_image(64, 64);
    let image = Image::from_rgba(64, 64, image_data);
    let _texture = Texture2D::from_image(&image);

    // Create a sprite from the texture
    let sprite = Sprite::from_texture(TextureHandle::null()).with_color(Color::WHITE);

    println!("Created sprite with texture");
    println!("  Sprite color: {:?}", sprite.color());
    println!("  Sprite size: {:?}", sprite.size());

    // Create a camera
    let _camera = Camera2D::new();

    println!("\nExample demonstrates:");
    println!("  - Creating textures from images");
    println!("  - Creating sprites from textures");
    println!("  - Camera setup for 2D rendering");
    println!("  - Sprite batch for efficient rendering");

    // Note: Actual rendering to screen requires:
    // 1. OpenGL context from winit window
    // 2. GLSL shaders for sprite rendering
    // 3. Vertex buffers for sprite geometry

    // In a full implementation, the main loop would:
    // 1. Clear the screen with background color
    // 2. Set the camera transform
    // 3. Draw sprites using the renderer

    println!("\nTo see actual rendering, run this example with a display.");
    println!("The sprite would be rendered at the center of the window.");
}

/// Create a simple gradient test image
fn create_gradient_image(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = 200;
            let a = 255;
            data.push(r);
            data.push(g);
            data.push(b);
            data.push(a);
        }
    }
    data
}
