//! prefab_demo.rs - 预制体演示
//!
//! 本示例演示 Prefab（预制体）系统的设计用法。
//! 注意：这是一个 API 设计演示，Prefab 系统尚未实现。

// use engine_scene::{Node2D, SceneTree};

fn main() {
    println!("=== Prefab Demo (API Design) ===");
    println!();

    // Prefab API 基于 sprint-04 文档设计

    // 1. Prefab 创建
    println!("1. Creating Prefab (API design):");
    println!("   // 从场景创建预制体");
    println!("   let scene = SceneTree::new();");
    println!("   // ... 添加节点到场景 ...");
    println!();
    println!("   let prefab = Prefab::from_scene(&scene);");
    println!("   // 或从文件加载");
    println!("   let prefab = Prefab::load_json(\"res/player.prefab\").unwrap();");
    println!();

    // 2. 实例化 Prefab
    println!("2. Instantiating Prefab:");
    println!("   // 创建预制体的独立副本");
    println!("   let instance = prefab.instantiate();");
    println!("   // 返回 NodeHandle，可以添加到场景树");
    println!();

    // 3. 实例化到指定场景
    println!("3. Instantiate into specific scene:");
    println!("   let mut scene = SceneTree::new();");
    println!("   let handle = prefab.instantiate_in(&mut scene);");
    println!();

    // 4. 独立实例概念
    println!("4. Independent instances concept:");
    println!("   // 修改实例不影响原始 Prefab");
    println!("   instance.set_position(Vec2::new(100.0, 200.0));");
    println!("   // Prefab 模板保持不变");
    println!();

    // 5. 保存/加载
    println!("5. Save and Load:");
    println!("   // JSON 格式");
    println!("   prefab.save_json(\"res/player.prefab\").unwrap();");
    println!("   let loaded = Prefab::load_json(\"res/player.prefab\").unwrap();");
    println!();
    println!("   // 二进制格式");
    println!("   prefab.save_bin(\"res/player.bin\").unwrap();");
    println!("   let loaded = Prefab::load_bin(\"res/player.bin\").unwrap();");
    println!();

    // 6. 典型使用场景
    println!("6. Typical usage scenarios:");
    println!();
    println!("   // 游戏中的敌人预制体");
    println!("   let enemy_prefab = Prefab::load_json(\"res/enemy.prefab\").unwrap();");
    println!();
    println!("   // 游戏中需要生成敌人时:");
    println!("   fn spawn_enemy(position: Vec2) {{");
    println!("       let enemy = enemy_prefab.instantiate_in(&mut current_scene);");
    println!("       enemy.set_position(position);");
    println!("   }}");
    println!();

    // 7. 预制体结构示例
    println!("7. Prefab structure example:");
    println!("   // player.prefab 结构:");
    println!("   // - Player (root)");
    println!("   //   - Sprite");
    println!("   //   - Hitbox (collider)");
    println!("   //   - HealthBar");
    println!();

    // 8. 多次实例化
    println!("8. Multiple instantiation:");
    println!();
    println!("   let positions = [");
    println!("       Vec2::new(0.0, 0.0),");
    println!("       Vec2::new(100.0, 0.0),");
    println!("       Vec2::new(200.0, 0.0),");
    println!("   ];");
    println!();
    println!("   for pos in positions {{");
    println!("       let enemy = enemy_prefab.instantiate();");
    println!("       enemy.set_position(pos);");
    println!("   }}");
    println!();

    // 9. 预制体修改
    println!("9. Modifying instances (not template):");
    println!();
    println!("   let soldier1 = soldier_prefab.instantiate();");
    println!("   let soldier2 = soldier_prefab.instantiate();");
    println!();
    println!("   // 每个实例可以独立修改");
    println!("   soldier1.set_name(\"Alpha\");");
    println!("   soldier2.set_name(\"Beta\");");
    println!();
    println!("   // 原始预制体保持不变");
    println!();

    // 10. 预制体嵌套
    println!("10. Nested prefabs:");
    println!("    // 房间预制体包含桌子、椅子等");
    println!("    let room_prefab = Prefab::load_json(\"res/room.prefab\").unwrap();");
    println!();
    println!("    // 房子由多个房间组成");
    println!("    let house_prefab = Prefab::load_json(\"res/house.prefab\").unwrap();");
    println!();
    println!("    // 每个实例都是独立的对象");
    println!("    let my_house = house_prefab.instantiate();");
    println!();

    // 11. 性能优势
    println!("11. Performance benefits:");
    println!("    - 预制体模板只需解析一次");
    println!("    - 实例化是轻量级克隆");
    println!("    - 适合大量相似对象（如子弹、粒子）");
    println!();

    // 12. 设计注意事项
    println!("12. Design considerations:");
    println!("    - 预制体应该是完整的自包含单元");
    println!("    - 避免在预制体中硬编码绝对位置");
    println!("    - 使用相对坐标或通过实例化参数设置位置");
    println!();

    println!("Prefab demo completed (API design demonstration)!");
}
