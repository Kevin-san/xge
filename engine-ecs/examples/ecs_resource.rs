//! 资源系统演示
//!
//! 演示如何使用 Resources 存储全局数据。

use engine_ecs::World;

/// 游戏配置
#[derive(Debug, Clone)]
struct GameConfig {
    #[allow(dead_code)]
    gravity: f32,
    #[allow(dead_code)]
    time_scale: f32,
    #[allow(dead_code)]
    max_entities: u32,
}

// GameConfig 已通过 blanket impl 实现 Resource

/// 游戏时间
#[derive(Debug, Clone)]
struct GameTime {
    delta_seconds: f32,
    total_seconds: f32,
}

// GameTime 已通过 blanket impl 实现 Resource

/// 分数
#[derive(Debug, Clone, Default)]
struct Score {
    player1: u32,
    player2: u32,
}

// Score 已通过 blanket impl 实现 Resource: impl<R: Any + Send + Sync + 'static> Resource for R {}

fn main() {
    println!("=== ECS Resource Demo ===\n");

    let mut world = World::new();

    // 插入资源
    world.insert_resource(GameConfig {
        gravity: 9.81,
        time_scale: 1.0,
        max_entities: 10000,
    });

    world.insert_resource(GameTime {
        delta_seconds: 0.016,
        total_seconds: 0.0,
    });

    world.insert_resource(Score::default());

    // 读取资源
    let config = world.resource::<GameConfig>();
    println!("Game config: {:?}", config);

    // 可变访问资源
    {
        let time = world.resource_mut::<GameTime>();
        time.total_seconds += time.delta_seconds;
        println!("Game time updated: {:.3}s", time.total_seconds);
    }

    // 使用资源
    {
        let score = world.resource_mut::<Score>();
        score.player1 += 100;
        score.player2 += 50;
    }

    let score = world.resource::<Score>();
    println!(
        "Score - Player1: {}, Player2: {}",
        score.player1, score.player2
    );

    // 检查资源是否存在
    println!("\nResource checks:");
    println!(
        "  Has GameConfig: {}",
        world.contains_resource::<GameConfig>()
    );
    println!("  Has GameTime: {}", world.contains_resource::<GameTime>());
    println!("  Has Score: {}", world.contains_resource::<Score>());

    // 移除资源
    world.remove_resource::<Score>();
    println!("\nAfter removing Score:");
    println!("  Has Score: {}", world.contains_resource::<Score>());
}
