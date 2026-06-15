//! hello_engine - 最简单的引擎使用示例

use engine_core::{Engine, EngineConfig, BUILD_COMMIT_HASH, BUILD_TIMESTAMP, ENGINE_VERSION};

fn main() {
    // 1. 打印引擎版本
    println!("Engine Version: {}", ENGINE_VERSION);
    println!("Build: {} @ {}", BUILD_COMMIT_HASH, BUILD_TIMESTAMP);

    // 2. 创建引擎配置
    let config = EngineConfig::default();

    // 3. 创建引擎实例
    let mut engine = Engine::new(config);

    // 4. 运行引擎（会触发一次空帧然后退出）
    engine.request_quit(); // 立即请求退出，因为我们只想演示初始化
    engine.run();
    
    println!("Engine exited successfully");
}