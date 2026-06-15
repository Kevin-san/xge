//! atlas_animation 示例 - 图集帧动画
//!
//! 本示例演示如何使用 TextureAtlas 创建帧动画。

use engine_math::Vec2;
use engine_render::{
    AnimatedSprite, Color, Image, LoopMode, PackAlgorithm, Rect, RenderContext, TextureAtlas,
    TextureAtlasBuilder,
};

fn main() {
    println!("Atlas Animation Example");
    println!("=======================");

    let ctx = RenderContext::new();

    // Create a simple sprite sheet (4 frames in a row)
    let frame_width = 64;
    let frame_height = 64;
    let num_frames = 4;
    let sheet_width = frame_width * num_frames;

    println!("Creating sprite sheet: {}x{}", sheet_width, frame_height);

    // Create sprite sheet image with different colored frames
    let mut sheet_image = create_sprite_sheet(frame_width, frame_height, num_frames);

    // Save the sprite sheet (for debugging)
    if let Err(e) = sheet_image.save("/tmp/sprite_sheet.png") {
        println!(
            "Note: Could not save sprite sheet (expected in headless): {}",
            e
        );
    }

    // Build texture atlas
    let mut builder = TextureAtlasBuilder::new(256)
        .with_padding(2)
        .with_algorithm(PackAlgorithm::Guillotine);

    builder.add(sheet_image);

    println!("Building atlas...");
    // Note: build() requires a RenderContext in full implementation
    // For demonstration, we'll show the concept

    // Create animation frames manually
    let frames = vec![
        Rect::new(0.0, 0.0, 64.0, 64.0),
        Rect::new(64.0, 0.0, 64.0, 64.0),
        Rect::new(128.0, 0.0, 64.0, 64.0),
        Rect::new(192.0, 0.0, 64.0, 64.0),
    ];

    // Create animated sprite
    let mut anim = AnimatedSprite::new(
        engine_utils::Handle::null(),
        8.0, // 8 FPS
        frames,
    );

    println!("\nAnimation properties:");
    println!("  Total frames: {}", anim.total_frames());
    println!("  FPS: {}", anim.fps());
    println!("  Loop mode: {:?}", anim.loop_mode());
    println!("  Is playing: {}", anim.is_playing());

    // Test animation update
    anim.play();
    println!("\nPlaying animation...");

    for frame in 0..8 {
        anim.update(1.0 / 8.0); // Simulate frame time
        println!("  Frame {}: current = {}", frame, anim.current_frame());
    }

    // Test different loop modes
    anim.set_loop(LoopMode::PingPong);
    println!("\nChanged to PingPong mode: {:?}", anim.loop_mode());

    anim.set_loop(LoopMode::Once);
    anim.set_frame(0);
    println!("Changed to Once mode");

    // Simulate once mode playing to end
    anim.play();
    for _ in 0..10 {
        anim.update(1.0);
    }
    println!(
        "After 10 updates in Once mode: playing = {}",
        anim.is_playing()
    );

    println!("\nAtlas animation example completed successfully!");
}

fn create_sprite_sheet(frame_width: u32, frame_height: u32, num_frames: u32) -> Image {
    let width = frame_width * num_frames;
    let height = frame_height;

    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];

    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for frame in 0..num_frames {
        let color = colors[frame as usize % colors.len()];
        for _ in 0..(frame_width * frame_height) {
            data.push((color.r * 255.0) as u8);
            data.push((color.g * 255.0) as u8);
            data.push((color.b * 255.0) as u8);
            data.push((color.a * 255.0) as u8);
        }
    }

    Image::from_rgba(width, height, data)
}
