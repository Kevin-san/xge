//! sprite2d_demo.rs - 精灵节点演示
//!
//! 本示例演示 Sprite2D 的设计用法。
//! 注意：这是一个 API 设计演示，Sprite2D 尚未在 engine-render 中可直接使用。

// 由于 engine_scene 不是 engine_render 的依赖，这些是设计 API 演示
// use engine_math::Vec2;
// use engine_scene::{Node2D, SceneTree, Sprite2D, Sprite as SpriteData, SpriteRegion};

fn main() {
    println!("=== Sprite2D Demo (API Design) ===");
    println!();

    // Sprite2D API 基于 sprint-04 文档设计

    // 1. Sprite 创建
    println!("1. Creating Sprite2D:");
    println!();
    println!("   // 创建简单的精灵");
    println!("   let sprite = Sprite::new(texture_id);");
    println!();
    println!("   // 创建带纹理的精灵");
    println!("   let sprite = Sprite::with_region(texture_id, x, y, width, height);");
    println!();

    // 2. Sprite2D 节点
    println!("2. Sprite2D node creation:");
    println!();
    println!("   let sprite = Sprite::new(1);");
    println!("   let node = Sprite2D::new(\"PlayerSprite\", sprite);");
    println!();

    // 3. 精灵属性
    println!("3. Sprite properties:");
    println!();
    println!("   sprite.texture_id  // u32 纹理 ID");
    println!("   sprite.region      // SpriteRegion {{x, y, width, height}}");
    println!("   sprite.flip_x      // bool 水平翻转");
    println!("   sprite.flip_y      // bool 垂直翻转");
    println!("   sprite.modulate    // Color 颜色调制");
    println!();

    // 4. 修改精灵属性
    println!("4. Modifying sprite properties:");
    println!();
    println!("   sprite.texture_id = 10;");
    println!("   sprite.region = SpriteRegion {{");
    println!("       x: 128.0,");
    println!("       y: 256.0,");
    println!("       width: 32.0,");
    println!("       height: 32.0,");
    println!("   }};");
    println!("   sprite.flip_x = true;");
    println!();

    // 5. Node2D 变换属性
    println!("5. Node2D transform properties:");
    println!();
    println!("   node.position()   // Vec2 位置");
    println!("   node.rotation()   // f32 旋转（弧度）");
    println!("   node.scale()      // Vec2 缩放");
    println!("   node.z_index()    // i32 Z 层级");
    println!("   node.visible()    // bool 可见性");
    println!();

    // 6. 设置变换
    println!("6. Setting transforms:");
    println!();
    println!("   node.set_position(Vec2::new(100.0, 200.0));");
    println!("   node.set_rotation(std::f32::consts::PI / 4.0);");
    println!("   node.set_scale(Vec2::new(2.0, 2.0));");
    println!();

    // 7. 变换操作
    println!("7. Transform operations:");
    println!();
    println!("   node.translate(Vec2::new(5.0, -10.0));");
    println!("   node.rotate(std::f32::consts::PI / 2.0);");
    println!("   node.set_scale(node.scale() * 2.0);");
    println!();

    // 8. 层级变换
    println!("8. Hierarchical transform:");
    println!();
    println!("   // 父节点缩放 2x，子节点会继承这个缩放");
    println!("   parent.set_scale(Vec2::new(2.0, 2.0));");
    println!("   // 子节点 world_position = parent.world_transform * local_position");
    println!();

    // 9. Z-Index 和层次
    println!("9. Z-Index and layers:");
    println!();
    println!("   node.z_index();           // 获取 Z 层级");
    println!("   node.set_z_index(100);    // 设置 Z 层级");
    println!("   // 更高 Z-index 的节点渲染在上层");
    println!();

    // 10. 可见性
    println!("10. Visibility:");
    println!();
    println!("    node.visible();        // 获取可见性");
    println!("    node.set_visible(false); // 隐藏节点");
    println!("    // 隐藏的节点不渲染，但仍然在场景树中");
    println!();

    // 11. SpriteRegion
    println!("11. SpriteRegion:");
    println!();
    println!("    struct SpriteRegion {{");
    println!("        x: f32,        // 左上角 X");
    println!("        y: f32,        // 左上角 Y");
    println!("        width: f32,    // 宽度");
    println!("        height: f32,  // 高度");
    println!("    }}");
    println!("    // 用于纹理图集（Atlas）中的精灵选取");
    println!();

    // 12. 在场景中使用
    println!("12. Using in SceneTree:");
    println!();
    println!("    let mut tree = SceneTree::new();");
    println!("    let sprite_node = tree.add_2d_node(tree.root(), \"Hero\");");
    println!();

    // 13. 纹理图集动画
    println!("13. Texture atlas animation:");
    println!();
    println!("    // 通过快速切换 region 实现动画");
    println!("    let frames = vec![");
    println!("        SpriteRegion {{ x: 0.0, y: 0.0, w: 32.0, h: 32.0 }},");
    println!("        SpriteRegion {{ x: 32.0, y: 0.0, w: 32.0, h: 32.0 }},");
    println!("        SpriteRegion {{ x: 64.0, y: 0.0, w: 32.0, h: 32.0 }},");
    println!("    ];");
    println!();

    // 14. 实际使用模式
    println!("14. Practical usage patterns:");
    println!();
    println!("    // 创建角色精灵");
    println!("    let player_sprite = Sprite::new(player_texture_id);");
    println!("    let player_node = Sprite2D::new(\"Player\", player_sprite);");
    println!("    player_node.node2d_mut().set_position(Vec2::new(100.0, 200.0));");
    println!();

    // 15. 翻转实现动画方向
    println!("15. Directional sprites:");
    println!();
    println!("    // 面向左");
    println!("    sprite.flip_x = true;");
    println!("    // 面向右");
    println!("    sprite.flip_x = false;");
    println!();

    // 16. 颜色调制
    println!("16. Color modulation:");
    println!();
    println!("    sprite.modulate = Color::new(1.0, 0.5, 0.5, 1.0); // 红色调");
    println!("    sprite.modulate = Color::new(0.5, 0.5, 1.0, 1.0); // 蓝色调");
    println!("    sprite.modulate = Color::new(1.0, 1.0, 1.0, 1.0); // 白色（无调制）");
    println!();

    // 17. 批量渲染优化
    println!("17. Batch rendering optimization:");
    println!();
    println!("    // 相同纹理的精灵可以批量渲染");
    println!("    // 使用 TextureAtlas 将多个小纹理合并为大纹理");
    println!();

    // 18. 性能考虑
    println!("18. Performance considerations:");
    println!();
    println!("    - 精灵切换纹理可能打断批处理");
    println!("    - 频繁修改 sprite 属性会有成本");
    println!("    - 建议使用 flip_x/flip_y 而非创建两个精灵");
    println!();

    println!("Sprite2D demo completed (API design demonstration)!");
}
