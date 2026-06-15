//! sprite_draw 示例 - 绘制单个精灵
//!
//! 本示例演示如何使用 engine-render 绘制一个简单的精灵。

use engine_render::{Color, Image, RenderContext, Sprite, Texture2D, TextureHandle};

fn main() {
    println!("Sprite Draw Example");
    println!("==================");

    // Create a render context
    let ctx = RenderContext::new();

    // Create a simple 64x64 texture with a red square
    let image_data = create_test_image(64, 64, Color::RED);
    let image = Image::from_rgba(64, 64, image_data);

    // Create a texture from the image
    let texture = Texture2D::from_image(&image);

    // Create a sprite from the texture
    let sprite = Sprite::from_texture(TextureHandle::null()).with_color(Color::WHITE);

    println!("Created sprite: {:?}", sprite);

    // In a full implementation, we would:
    // 1. Initialize a window with winit
    // 2. Create an OpenGL context
    // 3. Initialize the renderer
    // 4. Draw the sprite each frame

    println!("Sprite draw example completed successfully!");
}

/// Create a simple test image with a solid color
fn create_test_image(width: u32, height: u32, color: Color) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..(width * height) {
        data.push((color.r * 255.0) as u8);
        data.push((color.g * 255.0) as u8);
        data.push((color.b * 255.0) as u8);
        data.push((color.a * 255.0) as u8);
    }
    data
}
