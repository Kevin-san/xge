# 示例实现指南

## 1. 概述

本指南提供 Sprint 03 各示例的实现参考，共计 11+ 个示例（需求 351-361, 381）。

---

## 2. 示例列表

| 示例 | 需求 | 描述 |
|------|------|------|
| sprite_draw | 351 | 绘制单个精灵 |
| multi_sprite | 352 | 1000 个随机精灵 + FPS |
| batch_draw | 353 | 10k 精灵合批 |
| atlas_animation | 354 | 图集帧动画 |
| camera_follow | 355 | 相机跟随 |
| shape_draw | 356 | 基本图形绘制 |
| debug_draw | 357 | 调试信息绘制 |
| blend_mode | 358 | BlendMode 演示 |
| scissor | 359 | 剪刀矩形演示 |
| transform_stack | 360 | 矩阵栈操作 |
| hot_shader | 361 | Shader 热重载 |

---

## 3. sprite_draw 示例

### 3.1 目标
绘制一个简单的精灵到窗口中央。

### 3.2 实现步骤

```rust
// 1. 创建窗口
let window = Window::new("Sprite Draw", 800, 600);

// 2. 创建渲染器
let mut renderer = GlRenderer::new(&window)?;

// 3. 加载纹理
let texture = Texture2D::from_file(renderer.context(), "assets/player.png")?;

// 4. 创建精灵
let sprite = Sprite::new(texture.handle())
    .with_anchor(Vec2::new(0.5, 0.5))
    .with_color(Color::WHITE);

// 5. 主循环
loop {
    renderer.begin_frame()?;
    renderer.clear(Color::SKY_BLUE);

    sprite.draw(renderer.context(), Vec2::new(400.0, 300.0));

    renderer.end_frame()?;
    window.poll_events();
}
```

### 3.3 验收标准
- 窗口显示蓝色背景
- 精灵居中显示
- 无控制台错误

---

## 4. multi_sprite 示例

### 4.1 目标
绘制 1000 个随机精灵，显示 FPS 统计。

### 4.2 实现步骤

```rust
// 1. 初始化
let mut rng = rand::thread_rng();
let mut sprites: Vec<Sprite> = (0..1000)
    .map(|_| {
        let x = rng.gen_range(0.0..800.0);
        let y = rng.gen_range(0.0..600.0);
        Sprite::new(texture.handle())
            .with_position(Vec2::new(x, y))
            .with_color(Color::from_u8(
                rng.gen(),
                rng.gen(),
                rng.gen(),
                255,
            ))
    })
    .collect();

// 2. FPS 计算
let mut frame_count = 0;
let mut last_time = Instant::now();

loop {
    // FPS 统计
    frame_count += 1;
    let elapsed = last_time.elapsed().as_secs_f32();
    if elapsed >= 1.0 {
        println!("FPS: {}", frame_count as f32 / elapsed);
        frame_count = 0;
        last_time = Instant::now();
    }

    renderer.begin_frame()?;
    renderer.clear(Color::BLACK)?;

    for sprite in &sprites {
        sprite.draw(renderer.context(), sprite.position());
    }

    renderer.end_frame()?;
}
```

### 4.3 验收标准
- 显示 1000 个精灵
- FPS >= 30
- 无明显卡顿

---

## 5. batch_draw 示例

### 5.1 目标
使用 SpriteBatch 绘制 10k 同纹理精灵。

### 5.2 实现步骤

```rust
// 1. 创建 SpriteBatch
let mut batch = SpriteBatch::new(texture.handle());

// 2. 添加 10k 精灵
batch.reserve(10000);
for i in 0..10000 {
    let x = (i % 100) as f32 * 8.0;
    let y = (i / 100) as f32 * 6.0;
    let sprite = Sprite::new(texture.handle());
    batch.add(&sprite, Vec2::new(x, y));
}

// 3. 批量绘制
loop {
    renderer.begin_frame()?;
    renderer.clear(Color::BLACK)?;

    batch.draw(renderer.context());

    renderer.end_frame()?;
}
```

### 5.3 验收标准
- draw_calls <= 5（合批效果好）
- FPS >= 30
- 无闪烁

---

## 6. atlas_animation 示例

### 6.1 目标
使用 TextureAtlas 播放帧动画。

### 6.2 实现步骤

```rust
// 1. 构建图集
let images = vec![
    Image::from_file("assets/player/run1.png")?,
    Image::from_file("assets/player/run2.png")?,
    Image::from_file("assets/player/run3.png")?,
    // ... 更多帧
];
let (atlas, rects) = TextureAtlasBuilder::new(512)
    .with_padding(2)
    .build(renderer.context(), images)?;

// 2. 创建动画精灵
let mut anim = AnimatedSprite::new(
    atlas.handle(),
    12.0, // 12 FPS
    rects,
);

// 3. 主循环
let mut last_time = Instant::now();
loop {
    let dt = last_time.elapsed().as_secs_f32();
    last_time = Instant::now();

    renderer.begin_frame()?;
    renderer.clear(Color::BLACK)?;

    anim.update(dt);
    anim.draw(renderer.context(), Vec2::new(400.0, 300.0));

    renderer.end_frame()?;
}
```

### 6.3 验收标准
- 动画流畅播放
- 帧率符合设置

---

## 7. camera_follow 示例

### 7.1 目标
相机平滑跟随移动的目标。

### 7.2 实现步骤

```rust
// 1. 创建相机
let mut camera = Camera2D::from_window(window, 1.0);
camera.set_target(Vec2::ZERO);

// 2. 目标位置
let mut target_pos = Vec2::ZERO;
let mut target_vel = Vec2::new(100.0, 0.0);

// 3. 主循环
loop {
    let dt = last_frame_time();

    // 更新目标
    target_pos += target_vel * dt;
    if target_pos.x > 1000.0 || target_pos.x < 0.0 {
        target_vel.x *= -1.0;
    }
    camera.set_target(target_pos);
    camera.update(dt);

    renderer.begin_frame()?;
    renderer.set_camera(camera.clone());

    // 绘制背景（世界坐标）
    for i in 0..100 {
        let x = i as f32 * 100.0;
        renderer.draw_rectangle(x, 0.0, 50.0, 50.0, Color::GREEN);
    }

    renderer.end_frame()?;
}
```

### 7.3 验收标准
- 相机平滑跟随目标
- 无抖动

---

## 8. shape_draw 示例

### 8.1 目标
绘制基本几何图形。

### 8.2 实现步骤

```rust
loop {
    renderer.begin_frame()?;
    renderer.clear(Color::WHITE)?;

    // 矩形
    renderer.draw_rectangle(50.0, 50.0, 100.0, 60.0, Color::RED);
    renderer.draw_rectangle_lines(200.0, 50.0, 100.0, 60.0, 2.0, Color::BLUE);

    // 圆形
    renderer.draw_circle(100.0, 200.0, 40.0, Color::GREEN);
    renderer.draw_circle_lines(250.0, 200.0, 40.0, 2.0, Color::ORANGE);

    // 线条
    renderer.draw_line(50.0, 300.0, 150.0, 350.0, 3.0, Color::PURPLE);

    // 三角形
    renderer.draw_triangle(
        Vec2::new(100.0, 400.0),
        Vec2::new(150.0, 450.0),
        Vec2::new(50.0, 450.0),
        Color::CYAN,
    );

    // 多边形
    renderer.draw_poly(400.0, 300.0, 6, 50.0, 0.0, Color::YELLOW);

    renderer.end_frame()?;
}
```

---

## 9. debug_draw 示例

### 9.1 目标
使用 DebugRenderer 绘制调试信息。

### 9.2 实现步骤

```rust
let mut debug = DebugRenderer::new();

loop {
    renderer.begin_frame()?;
    renderer.clear(Color::BLACK)?;

    // 绘制玩家包围盒
    debug.rect(player.bounds(), Color::GREEN);
    debug.cross(player.position(), 10.0, Color::RED);

    // 绘制路径点
    for point in path {
        debug.circle(*point, 5.0, Color::YELLOW);
    }

    // 绘制坐标轴
    debug.line(Vec2::ZERO, Vec2::new(100.0, 0.0), Color::RED);
    debug.line(Vec2::ZERO, Vec2::new(0.0, 100.0), Color::GREEN);

    // 绘制网格
    debug.grid(Vec2::ZERO, 50.0, 16, 12, Color::GRAY);

    debug.flush(renderer.context());
    debug.clear();

    renderer.end_frame()?;
}
```

---

## 10. blend_mode 示例

### 10.1 目标
演示不同 BlendMode 效果。

### 10.2 实现步骤

```rust
loop {
    renderer.begin_frame()?;
    renderer.clear(Color::WHITE)?;

    // Alpha 混合（默认）
    renderer.set_blend_mode(BlendMode::Alpha);
    renderer.draw_circle(100.0, 100.0, 50.0, Color::RED.with_alpha(0.5));

    // 加法混合
    renderer.set_blend_mode(BlendMode::Additive);
    renderer.draw_circle(130.0, 100.0, 50.0, Color::GREEN.with_alpha(0.5));

    // 乘法混合
    renderer.set_blend_mode(BlendMode::Multiply);
    renderer.draw_circle(160.0, 100.0, 50.0, Color::BLUE.with_alpha(0.5));

    renderer.reset_blend_mode();
    renderer.end_frame()?;
}
```

---

## 11. scissor 示例

### 11.1 目标
演示 scissor rectangle 裁剪效果。

### 11.2 实现步骤

```rust
loop {
    renderer.begin_frame()?;
    renderer.clear(Color::WHITE)?;

    // 设置裁剪区域
    renderer.push_scissor_rect(Rect::new(100.0, 100.0, 200.0, 150.0));

    // 绘制到裁剪区域
    for i in 0..10 {
        renderer.draw_rectangle(
            0.0,
            i as f32 * 30.0,
            400.0,
            20.0,
            Color::BLUE,
        );
    }

    renderer.pop_scissor_rect();

    // 裁剪区域外不受影响
    renderer.draw_circle(50.0, 50.0, 30.0, Color::RED);

    renderer.end_frame()?;
}
```

---

## 12. transform_stack 示例

### 12.1 目标
演示矩阵变换栈。

### 12.2 实现步骤

```rust
loop {
    renderer.begin_frame()?;
    renderer.clear(Color::WHITE)?;

    // 原始位置绘制红色矩形
    renderer.draw_rectangle(0.0, 0.0, 50.0, 50.0, Color::RED);

    // 平移
    renderer.push_transform(Mat4::from_translation(Vec3::new(100.0, 0.0, 0.0)));
    renderer.draw_rectangle(0.0, 0.0, 50.0, 50.0, Color::GREEN);

    // 旋转 + 缩放
    renderer.push_transform(
        Mat4::from_rotation_z(std::f32::consts::PI / 4.0) *
        Mat4::from_scale(Vec3::new(1.5, 1.5, 1.0))
    );
    renderer.draw_rectangle(0.0, 0.0, 50.0, 50.0, Color::BLUE);

    // 弹出变换
    renderer.pop_transform();
    renderer.pop_transform();

    renderer.end_frame()?;
}
```

---

## 13. hot_shader 示例（Debug 模式）

### 13.1 目标
演示 shader 热重载。

### 13.2 实现步骤

```rust
// Debug 模式下启用热重载
let shader = ShaderModule::from_file(ShaderStage::Vertex, "shaders/sprite.vert")?;
let mut pipeline = PipelineBuilder::new(shader)
    .with_vertex_layout(vertex_layout)
    .build(renderer.context())?;

loop {
    #[cfg(debug_assertions)]
    shader.hot_reload(); // 检测文件变化自动重编译

    renderer.begin_frame()?;
    // ... 绘制
    renderer.end_frame()?;
}
```

---

## 14. 通用验收检查清单

- [ ] `cargo run --example sprite_draw` 正常显示
- [ ] `cargo run --example batch_draw` 10k 精灵流畅
- [ ] `cargo run --example atlas_animation` 动画播放正常
- [ ] `cargo run --example camera_follow` 跟随平滑
- [ ] `cargo run --example debug_draw` 调试信息正确
- [ ] `cargo run --example blend_mode` 混合模式效果正确
- [ ] `cargo run --example scissor` 裁剪正确
- [ ] `cargo run --example transform_stack` 变换正确
- [ ] `cargo run --example hot_shader` 热重载生效（debug）
- [ ] 所有示例无 panic
- [ ] 所有示例无内存泄漏