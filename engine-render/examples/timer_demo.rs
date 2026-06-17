//! timer_demo.rs - 定时器演示
//!
//! 本示例演示 Timer（定时器）系统的设计用法。
//! 注意：这是一个 API 设计演示，Timer 系统尚未实现。

fn main() {
    println!("=== Timer Demo (API Design) ===");
    println!();

    // Timer API 基于 sprint-04 文档设计

    // 1. TimerMode 枚举
    println!("1. TimerMode:");
    println!("   - TimerMode::Once   // 单次触发后停止");
    println!("   - TimerMode::Repeat // 循环触发");
    println!();

    // 2. Timer 创建
    println!("2. Timer creation (API design):");
    println!("   let timer = Timer::new(1.0, TimerMode::Once);");
    println!("   // 创建一个 1 秒后触发一次的定时器");
    println!();

    println!("   let repeat_timer = Timer::new(0.5, TimerMode::Repeat);");
    println!("   // 创建一个每 0.5 秒触发一次的循环定时器");
    println!();

    // 3. Timer 方法
    println!("3. Timer methods:");
    println!("   - timer.tick(dt) -> bool   // 更新并返回是否触发");
    println!("   - timer.finished() -> bool // 是否已完成（Once 模式下触发后）");
    println!("   - timer.reset()            // 重置定时器");
    println!("   - timer.remaining() -> f32 // 剩余时间");
    println!("   - timer.elapsed() -> f32   // 已过时间");
    println!();

    // 4. 模拟 Once 模式
    println!("4. Simulating Once mode timer:");
    println!("   Timer: duration=1.0s, mode=Once");
    println!(
        "   {:^8} | {:^10} | {:^10} | {:^8}",
        "Step", "Elapsed", "Remaining", "Triggered"
    );
    println!("   {:->8} | {:->10} | {:->10} | {:->8}", "", "", "", "");

    let duration = 1.0;
    let mut elapsed = 0.0;
    let dt = 0.2;
    let mut triggered_count = 0;

    while elapsed <= 2.0 {
        let remaining: f32 = f32::max(duration - elapsed, 0.0);
        let triggered = if elapsed >= duration && elapsed - dt < duration {
            triggered_count += 1;
            true
        } else {
            false
        };

        println!(
            "   {:8.2} | {:10.2} | {:10.2} | {:^8}",
            elapsed,
            elapsed,
            remaining,
            if triggered { "YES" } else { "no" }
        );

        elapsed += dt;
    }
    println!();

    // 5. 模拟 Repeat 模式
    println!("5. Simulating Repeat mode timer:");
    println!("   Timer: duration=0.3s, mode=Repeat");
    println!("   {:^8} | {:^10} | {:^10}", "Step", "Elapsed", "Triggered");
    println!("   {:->8} | {:->10} | {:->10}", "", "", "");

    let repeat_duration = 0.3;
    let mut repeat_elapsed = 0.0;
    let repeat_dt = 0.1;
    let mut repeat_count = 0;

    for step in 0..8 {
        let triggered = if repeat_elapsed >= repeat_duration {
            repeat_elapsed -= repeat_duration;
            repeat_count += 1;
            true
        } else {
            false
        };

        let triggered_str = if triggered {
            format!("YES (#{})", repeat_count)
        } else {
            "no".to_string()
        };
        println!(
            "   {:8.2} | {:10.2} | {:^10}",
            repeat_elapsed, repeat_elapsed, triggered_str
        );

        repeat_elapsed += repeat_dt;
    }
    println!();

    // 6. reset() 方法演示
    println!("6. reset() method:");
    println!("   // 在 Once 模式触发后:");
    println!("   timer.reset();");
    println!("   // 定时器重新开始计时");
    println!();

    // 7. 实际使用模式
    println!("7. Practical usage patterns:");
    println!();
    println!("   // 射击游戏冷却定时器");
    println!("   let mut shoot_cooldown = Timer::new(0.5, TimerMode::Once);");
    println!();
    println!("   // 游戏循环中:");
    println!("   if can_shoot && shoot_cooldown.tick(dt) {{");
    println!("       shoot();");
    println!("       shoot_cooldown.reset();");
    println!("   }}");
    println!();

    println!("   // 敌人生成定时器（循环）");
    println!("   let mut spawn_timer = Timer::new(3.0, TimerMode::Repeat);");
    println!();
    println!("   // 游戏循环中:");
    println!("   if spawn_timer.tick(dt) {{");
    println!("       spawn_enemy();");
    println!("   }}");
    println!();

    println!("   // 倒计时定时器");
    println!("   let mut countdown = Timer::new(10.0, TimerMode::Once);");
    println!();
    println!("   // 显示剩余时间:");
    println!("   let remaining = countdown.remaining();");
    println!("   println!(\"Time: {{:.1}}\", remaining);");
    println!();

    // 8. 时间精度
    println!("8. Time precision:");
    println!("   - Timer 使用 f32 秒为单位");
    println!("   - 典型 dt 值: 1/60 ≈ 0.0167s (60 FPS)");
    println!("   - 精度足够用于大多数游戏需求");
    println!();

    // 9. 暂停与恢复
    println!("9. Pause and resume (design):");
    println!("   // 注意: 基础 Timer 可能不直接支持暂停");
    println!("   // 实现方式之一:");
    println!("   let mut timer = Timer::new(1.0, TimerMode::Once);");
    println!("   let mut paused_time = 0.0;");
    println!();
    println!("   fn tick_paused(timer: &mut Timer, dt: f32, paused: &mut f32) {{");
    println!("       if *paused > 0.0 {{");
    println!("           *paused -= dt;");
    println!("           return false;");
    println!("       }}");
    println!("       timer.tick(dt)");
    println!("   }}");
    println!();

    println!("Timer demo completed (API design demonstration)!");
}
