//! hello_engine - 最简单的引擎使用示例
//!
//! 演示内容：
//! - 打印引擎版本 / 构建信息
//! - 创建引擎实例
//! - 触发一帧更新并打印 delta_time
//! - 使用 is_running / request_quit / spawn_task API

use engine_core::{Engine, EngineConfig, BUILD_COMMIT_HASH, BUILD_TIMESTAMP, ENGINE_VERSION};

fn main() {
    // 1. 打印引擎版本与构建信息
    println!("=== Game Engine ===");
    println!("Version : {}", ENGINE_VERSION);
    println!("Commit  : {}", BUILD_COMMIT_HASH);
    println!("Built   : {}", BUILD_TIMESTAMP);

    // 2. 创建引擎配置
    let config = EngineConfig {
        window_title: "Hello Engine".to_string(),
        window_width: 800,
        window_height: 600,
        target_fps: 60,
        log_level: "info".to_string(),
    };

    // 3. 创建引擎实例
    let mut engine = Engine::new(config);
    println!("\nEngine initialized successfully");
    println!(
        "Config: {}x{} @ {}fps",
        engine.config().window_width,
        engine.config().window_height,
        engine.config().target_fps
    );

    // 4. 检查引擎运行状态
    println!("Engine is_running: {}", engine.is_running());

    // 5. 手动触发一帧更新
    engine.time_mut().update();
    let dt = engine.time().delta_time();
    println!("Frame delta_time: {:.4}s ({:.2}ms)", dt, dt * 1000.0);

    // 6. 演示 spawn_task
    let handle = engine.spawn_task(|| {
        println!("[spawned task] Running in background thread");
    });
    handle.join().expect("spawned task panicked");

    // 7. 请求退出
    engine.request_quit();
    println!("After request_quit, is_running: {}", engine.is_running());
    engine.run(); // 主循环会立即检测到 quit_flag 并退出

    println!("\nEngine exited successfully");
}
