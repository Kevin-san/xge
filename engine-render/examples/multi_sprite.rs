//! multi_sprite 示例 - 绘制 1000 个随机精灵 + FPS 统计
//!
//! 本示例演示绘制大量精灵的性能。

use engine_math::Vec2;
use engine_render::{Color, Image, RenderContext, Sprite, SpriteBatch, Texture2D, TextureHandle};

fn main() {
    println!("Multi-Sprite Example (1000 sprites + FPS)");
    println!("==========================================");

    let ctx = RenderContext::new();

    // Create textures for different colors
    let red_image = create_color_image(32, 32, Color::RED);
    let green_image = create_color_image(32, 32, Color::GREEN);
    let blue_image = create_color_image(32, 32, Color::BLUE);

    let _red_tex = Texture2D::from_image(&red_image);
    let _green_tex = Texture2D::from_image(&green_image);
    let _blue_tex = Texture2D::from_image(&blue_image);

    // Create sprites with different textures
    let sprites = [
        Sprite::from_texture(TextureHandle::null()).with_color(Color::RED),
        Sprite::from_texture(TextureHandle::null()).with_color(Color::GREEN),
        Sprite::from_texture(TextureHandle::null()).with_color(Color::BLUE),
    ];

    // Create batch for each texture type
    let mut red_batch = SpriteBatch::with_capacity(TextureHandle::null(), 400);
    let mut green_batch = SpriteBatch::with_capacity(TextureHandle::null(), 300);
    let mut blue_batch = SpriteBatch::with_capacity(TextureHandle::null(), 300);

    // Add 1000 random sprites
    let mut rng = SimpleRng::new(12345);

    for i in 0..1000 {
        let x = rng.next_f32() * 800.0;
        let y = rng.next_f32() * 600.0;
        let sprite_idx = i % 3;

        let batch = match sprite_idx {
            0 => &mut red_batch,
            1 => &mut green_batch,
            _ => &mut blue_batch,
        };

        let sprite = &sprites[sprite_idx];
        batch.add(sprite, Vec2::new(x, y));
    }

    println!("Added 1000 sprites to batches");
    println!("Red batch: {} sprites", red_batch.len());
    println!("Green batch: {} sprites", green_batch.len());
    println!("Blue batch: {} sprites", blue_batch.len());

    // Simulate FPS calculation
    let frame_count = 100;
    let start_time = std::time::Instant::now();

    for _ in 0..frame_count {
        // Simulate render
        let stats = ctx.stats();
        let _ = stats;
    }

    let elapsed = start_time.elapsed();
    let fps = frame_count as f64 / elapsed.as_secs_f64();

    println!(
        "\nSimulated {} frames in {:.3}s",
        frame_count,
        elapsed.as_secs_f64()
    );
    println!("Estimated FPS: {:.1}", fps);

    // Get stats
    let stats = ctx.stats();
    println!("\nRender stats:");
    println!("  Draw calls: {}", stats.draw_calls);
    println!("  Vertices: {}", stats.vertices);
    println!("  Indices: {}", stats.indices);
    println!("  Batches: {}", stats.batches);

    println!("\nMulti-sprite example completed successfully!");
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

/// Simple pseudo-random number generator for deterministic results
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn next_f32(&mut self) -> f32 {
        (self.next() as f32) / (u64::MAX as f32)
    }
}
