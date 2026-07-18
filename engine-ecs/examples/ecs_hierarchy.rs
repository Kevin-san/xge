//! 父子层级系统演示
//!
//! 演示如何使用 Parent/Children 组件构建实体层级。

use engine_ecs::{hierarchy::WorldHierarchyExt, Component, World};

/// 变换组件（简化版）
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Transform {
    translation: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
}

impl Component for Transform {}

impl Transform {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    #[allow(dead_code)]
    fn with_translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: [x, y, z],
            ..Self::new()
        }
    }
}

fn main() {
    println!("=== ECS Hierarchy Demo ===\n");

    let mut world = World::new();

    // 创建场景层级：Root -> Child1 -> Grandchild
    //              Root -> Child2

    let root = world.spawn();
    let child1 = world.spawn();
    let child2 = world.spawn();
    let grandchild = world.spawn();

    println!("Created entities:");
    println!("  root: {:?}", root);
    println!("  child1: {:?}", child1);
    println!("  child2: {:?}", child2);
    println!("  grandchild: {:?}", grandchild);

    // 设置父子关系
    world.set_parent(child1, Some(root));
    world.set_parent(child2, Some(root));
    world.set_parent(grandchild, Some(child1));

    println!("\nSet up hierarchy: root -> child1 -> grandchild, root -> child2");

    // 查询父实体
    println!("\nParent queries:");
    println!("  Parent of child1: {:?}", world.get_parent(child1));
    println!("  Parent of grandchild: {:?}", world.get_parent(grandchild));
    println!("  Parent of root: {:?}", world.get_parent(root));

    // 查询子实体
    println!("\nChildren queries:");
    if let Some(children) = world.get_children(root) {
        println!("  Children of root: {:?}", children.entities);
    }
    if let Some(children) = world.get_children(child1) {
        println!("  Children of child1: {:?}", children.entities);
    }

    // 查询祖先
    println!(
        "\nAncestors of grandchild: {:?}",
        world.get_ancestors(grandchild)
    );

    // 查询后代
    println!("\nDescendants of root: {:?}", world.get_descendants(root));

    // 检查是否是根实体
    println!("\nIs root entity:");
    println!("  root is root: {}", world.is_root(root));
    println!("  child1 is root: {}", world.is_root(child1));
    println!("  grandchild is root: {}", world.is_root(grandchild));

    // 检查父子关系
    println!("\nParent-child checks:");
    println!(
        "  is root parent of child1: {}",
        world.is_parent_of(root, child1)
    );
    println!(
        "  is root parent of grandchild: {}",
        world.is_parent_of(root, grandchild)
    );

    // 断开连接
    println!("\nDetaching grandchild from child1...");
    world.detach_from_parent(grandchild);
    println!(
        "  Parent of grandchild after detach: {:?}",
        world.get_parent(grandchild)
    );
    println!(
        "  Descendants of root after detach: {:?}",
        world.get_descendants(root)
    );
    println!(
        "  Descendants of child1 after detach: {:?}",
        world.get_descendants(child1)
    );
}
