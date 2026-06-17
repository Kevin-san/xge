//! tween_demo.rs - 补间动画演示
//!
//! 本示例演示 Tween（补间动画）系统的设计用法。
//! 注意：这是一个 API 设计演示，Tween 系统尚未实现。

use engine_math::Vec2;

fn main() {
    println!("=== Tween Demo (API Design) ===");
    println!();

    // Tween API 基于 sprint-04 文档设计
    // https://github.com/.../sprint-04-physics-2d/modules/06-tween-signal.md

    // 1. TweenValue 枚举
    println!("1. TweenValue types (design):");
    println!("   - TweenValue::Float(f32)");
    println!("   - TweenValue::Vec2(Vec2)");
    println!("   - TweenValue::Vec3(Vec3)");
    println!("   - TweenValue::Color(Color)");
    println!("   - TweenValue::Angle(f32) // radians");
    println!();

    // 2. Ease 缓动曲线
    println!("2. Ease curves (30+ variants):");
    println!("   - Ease::Linear");
    println!("   - Ease::InQuad, OutQuad, InOutQuad");
    println!("   - Ease::InCubic, OutCubic, InOutCubic");
    println!("   - Ease::InQuart, OutQuart, InOutQuart");
    println!("   - Ease::InSine, OutSine, InOutSine");
    println!("   - Ease::InExpo, OutExpo, InOutExpo");
    println!("   - Ease::InCirc, OutCirc, InOutCirc");
    println!("   - Ease::InBack, OutBack, InOutBack");
    println!("   - Ease::InElastic, OutElastic, InOutElastic");
    println!("   - Ease::InBounce, OutBounce, InOutBounce");
    println!();

    // 3. Tween 创建和使用
    println!("3. Tween creation (API design):");
    println!("   // 创建从 0.0 到 100.0 的 1 秒补间");
    println!("   let tween = Tween::new(");
    println!("       TweenValue::Float(0.0),");
    println!("       TweenValue::Float(100.0),");
    println!("       1.0,              // duration");
    println!("       Ease::Linear");
    println!("   );");
    println!();

    // 4. Tween 链式配置
    println!("4. Tween chain configuration:");
    println!("   let tween = Tween::new(start, end, duration, ease)");
    println!("       .with_repeat(3, TweenRepeatMode::Times)  // 重复 3 次");
    println!("       .with_yoyo(true)                           // 往复动画");
    println!("       .with_delay(0.5)                          // 延迟 0.5 秒启动");
    println!("       .on_complete(|| println!(\"Done!\"))        // 完成回调");
    println!();

    // 5. Tween 更新
    println!("5. Tween update (per-frame):");
    println!("   // 在游戏循环中:");
    println!("   tween.update(dt);");
    println!("   let value = tween.value();          // 获取当前插值");
    println!("   let progress = tween.progress();    // 获取进度 0.0-1.0");
    println!("   let finished = tween.is_finished(); // 检查是否完成");
    println!();

    // 6. 模拟 Ease 计算
    println!("6. Simulated ease calculations:");

    // 模拟 Ease::InOutCubic
    println!("   Ease::InOutCubic at various t:");
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let eased = ease_in_out_cubic(t);
        println!("      t={:.2} -> {:.4}", t, eased);
    }
    println!();

    // 7. TweenManager
    println!("7. TweenManager (API design):");
    println!("   let mut manager = TweenManager::new();");
    println!("   let handle = manager.add(tween);");
    println!("   // 每帧:");
    println!("   manager.update(dt);");
    println!("   // 移除:");
    println!("   manager.remove(handle);");
    println!();

    // 8. Vec2 Tween 演示
    println!("8. Vec2 Tween example (design):");
    println!("   // 位置从 (0,0) 移动到 (100, 200)");
    println!("   let move_tween = Tween::new(");
    println!("       TweenValue::Vec2(Vec2::new(0.0, 0.0)),");
    println!("       TweenValue::Vec2(Vec2::new(100.0, 200.0)),");
    println!("       2.0,");
    println!("       Ease::OutQuad");
    println!("   );");

    // 模拟插值
    let start = Vec2::new(0.0, 0.0);
    let end = Vec2::new(100.0, 200.0);
    println!();
    println!("   Simulated movement:");
    for i in 0..=4 {
        let t = i as f32 / 4.0;
        let eased = ease_out_quad(t);
        let x = start.x + (end.x - start.x) * eased;
        let y = start.y + (end.y - start.y) * eased;
        println!(
            "      t={:.2}: eased={:.2} -> pos=({:.1}, {:.1})",
            t, eased, x, y
        );
    }
    println!();

    // 9. 重复模式
    println!("9. TweenRepeatMode:");
    println!("   - TweenRepeatMode::Times(u32)  // 重复指定次数");
    println!("   - TweenRepeatMode::Forever      // 无限重复");
    println!();

    // 10. 实际使用示例
    println!("10. Practical usage pattern:");
    println!("    // UI 弹出动画");
    println!("    let scale_tween = Tween::new(");
    println!("        TweenValue::Vec2(Vec2::ZERO),");
    println!("        TweenValue::Vec2(Vec2::ONE),");
    println!("        0.3,");
    println!("        Ease::OutBack");
    println!("    ).on_complete(|| {{");
    println!("        // 动画完成后显示内容");
    println!("    }});");
    println!();

    println!("Tween demo completed (API design demonstration)!");
}

// 模拟 Ease::OutQuad
fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

// 模拟 Ease::InOutCubic
fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}
