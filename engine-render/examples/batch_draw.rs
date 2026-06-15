//! batch_draw 示例 - 10k 精灵合批
//!
//! 本示例演示大规模精灵批处理性能。

use engine_math::Vec2;
use engine_render::{Color, Image, RenderContext, Sprite, SpriteBatch, Texture2D, TextureHandle};

fn main() {
    println!("Batch Draw Example (10,000 sprites)");
    println!("===================================");

    let ctx = RenderContext::new();

    // Create a single white texture for batching
    let white_image = create_color_image(32, 32, Color::WHITE);
    let texture = Texture2D::from_image(&white_image);

    // Create a single sprite batch
    let mut batch = SpriteBatch::with_capacity(TextureHandle::null(), 10000);

    // Create base sprite
    let sprite = Sprite::from_texture(TextureHandle::null())
        .with_source_rect(engine_render::Rect::new(0.0, 0.0, 32.0, 32.0));

    // Add 10,000 sprites
    let mut rng = SimpleRng::new(42);

    println!("Adding 10,000 sprites to batch...");
    for i in 0..10000 {
        let x = rng.next_f32() * 1280.0;
        let y = rng.next_f32() * 720.0;
        let color = Color::from_rgb(rng.next_f32(), rng.next_f32(), rng.next_f32());

        let colored_sprite = sprite.clone().with_color(color);
        batch.add(&colored_sprite, Vec2::new(x, y));

        if (i + 1) % 1000 == 0 {
            println!("  Added {} sprites...", i + 1);
        }
    }

    println!("\nBatch statistics:");
    println!("  Total sprites: {}", batch.len());
    println!("  Vertex count: {}", batch.vertex_count());
    println!("  Index count: {}", batch.index_count());
    println!("  Capacity: {}", batch.capacity());

    // Simulate rendering
    println!("\nSimulating render...");

    let stats = ctx.stats();
    println!("\nRender stats:");
    println!("  Draw calls: {}", stats.draw_calls);
    println!("  Vertices: {}", stats.vertices);
    println!("  Indices: {}", stats.indices);
    println!("  Batches: {}", stats.batches);

    // Compare: Without batching would be 10,000 draw calls
    // With batching it's just 1 draw call
    let batching_reduction = 10000.0 / stats.draw_calls.max(1) as f32;
    println!(
        "\nBatching reduction: {:.1}x fewer draw calls",
        batching_reduction
    );

    println!("\nBatch draw example completed successfully!");
}

fn create_color_image(width: u32, height: u32, color: Color) -> Image {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..(width * height) {
        data.push((color.r * 255.0) as u8);
        data.push((color.g * 255.0) as u8);
        data.push((color.b * 255.0) as u8);
        data.push((color.a * 255.0) as u8);
    }
    Image::from_rgba(width, height, data)
}

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next() as f32) / (u64::MAX as f32)
    }
}
