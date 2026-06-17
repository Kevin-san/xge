//! collision_groups.rs - 碰撞分组演示
//!
//! 本示例演示如何使用碰撞分组（CollisionGroup）来过滤物理碰撞检测。

use engine_math::Vec2;
use engine_physics_2d::{Collider2DBuilder, CollisionGroup, PhysicsWorld2D, RigidBody2DBuilder};

fn main() {
    println!("=== Collision Groups Demo ===");
    println!();

    // 1. 碰撞分组基础
    println!("1. Collision Group Basics...");

    let group_none = CollisionGroup::with_none();
    let group_all = CollisionGroup::with_all();
    // CollisionGroup::new(memberships, filters)
    let group_player = CollisionGroup::new(0b0001, 0b0110); // Bit 0, 过滤 Enemy 和 Wall
    let group_enemy = CollisionGroup::new(0b0010, 0b0101); // Bit 1, 过滤 Player 和 Pickup
    let group_wall = CollisionGroup::new(0b0100, 0b0001); // Bit 2, 过滤 Player
    let group_pickup = CollisionGroup::new(0b1000, 0b0010); // Bit 3, 过滤 Enemy

    println!(
        "   - None group memberships: {:016b}, filters: {:016b}",
        group_none.memberships(),
        group_none.filters()
    );
    println!(
        "   - All group memberships:  {:016b}, filters: {:016b}",
        group_all.memberships(),
        group_all.filters()
    );
    println!(
        "   - Player group memberships: {:016b}, filters: {:016b}",
        group_player.memberships(),
        group_player.filters()
    );
    println!(
        "   - Enemy group memberships:  {:016b}, filters: {:016b}",
        group_enemy.memberships(),
        group_enemy.filters()
    );
    println!(
        "   - Wall group memberships:   {:016b}, filters: {:016b}",
        group_wall.memberships(),
        group_wall.filters()
    );
    println!(
        "   - Pickup group memberships: {:016b}, filters: {:016b}",
        group_pickup.memberships(),
        group_pickup.filters()
    );
    println!();

    // 2. 分组位操作
    println!("2. Group Bit Operations...");

    let group_with_bit = group_player.add_group(5);
    println!(
        "   - Player + group 5 memberships: {:016b}",
        group_with_bit.memberships()
    );
    println!("   - Player has group 0: {}", group_player.contains(0));
    println!("   - Player has group 1: {}", group_player.contains(1));
    println!("   - All has group 0: {}", group_all.contains(0));
    println!();

    // 3. 分组交互检测
    println!("3. Group Interaction Tests...");

    let player_and_enemy = CollisionGroup::new(0b0011, 0b0011); // memberships=3, filters=3
    let walls = CollisionGroup::new(0b1100, 0b1100); // memberships=12, filters=12

    println!(
        "   - Player can interact with Enemy: {}",
        group_player.can_interact_with(group_enemy)
    );
    println!(
        "   - Player can interact with Wall: {}",
        group_player.can_interact_with(group_wall)
    );
    println!(
        "   - All can interact with Player: {}",
        group_all.can_interact_with(group_player)
    );
    println!();

    // 4. 创建物理世界并演示分组
    println!("4. Creating Physics World with Collision Groups...");

    let mut world = PhysicsWorld2D::with_default_config();

    // Player (Dynamic, 碰撞分组 0b0001)
    let player = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(0.0, 5.0))
        .build();
    let player_idx = world.add_body(player);
    let mut player_collider = Collider2DBuilder::circle(0.5)
        .with_position(Vec2::new(0.0, 5.0))
        .build();
    player_collider.set_collision_group(0b0001);
    player_collider.set_collision_mask(0b0110); // 只与 Enemy 和 Wall 碰撞
    world.add_collider(player_collider, player_idx);
    println!("   - Player: group=0b0001, mask=0b0110");

    // Enemy (Dynamic, 碰撞分组 0b0010)
    let enemy = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(2.0, 5.0))
        .build();
    let enemy_idx = world.add_body(enemy);
    let mut enemy_collider = Collider2DBuilder::circle(0.5)
        .with_position(Vec2::new(2.0, 5.0))
        .build();
    enemy_collider.set_collision_group(0b0010);
    enemy_collider.set_collision_mask(0b0101); // 只与 Player 和 Pickup 碰撞
    world.add_collider(enemy_collider, enemy_idx);
    println!("   - Enemy: group=0b0010, mask=0b0101");

    // Pickup (Dynamic, 碰撞分组 0b1000)
    let pickup = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(1.0, 3.0))
        .build();
    let pickup_idx = world.add_body(pickup);
    let mut pickup_collider = Collider2DBuilder::circle(0.3)
        .with_position(Vec2::new(1.0, 3.0))
        .build();
    pickup_collider.set_collision_group(0b1000);
    pickup_collider.set_collision_mask(0b0010); // 只与 Enemy 碰撞
    world.add_collider(pickup_collider, pickup_idx);
    println!("   - Pickup: group=0b1000, mask=0b0010");

    // Wall (Static, 碰撞分组 0b0100)
    let wall = RigidBody2DBuilder::static_()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let wall_idx = world.add_body(wall);
    let mut wall_collider = Collider2DBuilder::aabb(10.0, 1.0)
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    wall_collider.set_collision_group(0b0100);
    wall_collider.set_collision_mask(0b0001); // 只与 Player 碰撞
    world.add_collider(wall_collider, wall_idx);
    println!("   - Wall: group=0b0100, mask=0b0001");

    println!("   Total bodies: {}", world.body_count());
    println!("   Total colliders: {}", world.collider_count());
    println!();

    // 5. 模拟并检测碰撞事件
    println!("5. Simulating Physics with Group-Based Collision Filtering...");
    println!(
        "   {:^6} | {:^18} | {:^18} | {:^18}",
        "Step", "Player Pos", "Enemy Pos", "Pickup Pos"
    );
    println!("   {:->6} | {:->18} | {:->18} | {:->18}", "", "", "", "");

    // 禁用重力以更好观察分组效果
    world.set_gravity(Vec2::ZERO);

    for step in 0..6 {
        world.step(1.0 / 60.0);

        let player_pos = world
            .get_body(player_idx)
            .map(|b| b.position())
            .unwrap_or(Vec2::ZERO);
        let enemy_pos = world
            .get_body(enemy_idx)
            .map(|b| b.position())
            .unwrap_or(Vec2::ZERO);
        let pickup_pos = world
            .get_body(pickup_idx)
            .map(|b| b.position())
            .unwrap_or(Vec2::ZERO);

        println!(
            "   {:6} | ({:7.2}, {:7.2}) | ({:7.2}, {:7.2}) | ({:7.2}, {:7.2})",
            step, player_pos.x, player_pos.y, enemy_pos.x, enemy_pos.y, pickup_pos.x, pickup_pos.y
        );
    }
    println!();

    // 6. 演示掩码过滤逻辑
    println!("6. Collision Mask Filtering Logic...");
    println!("   For collision to occur between A and B:");
    println!("   - (A.memberships & B.filters) != 0 AND (B.memberships & A.filters) != 0");
    println!();

    // Player vs Enemy:
    // Player memberships=1, filters=6 (0b0110)
    // Enemy memberships=2, filters=5 (0b0101)
    // Player memberships & Enemy filters = 1 & 5 = 1 != 0 ✓
    // Enemy memberships & Player filters = 2 & 6 = 2 != 0 ✓
    println!("   Player vs Enemy: YES (both conditions met)");

    // Player vs Pickup:
    // Pickup memberships=8, filters=2 (0b0010)
    // Player memberships & Pickup filters = 1 & 2 = 0 ✗
    println!("   Player vs Pickup: NO (Player memberships doesn't intersect Pickup filters)");

    // Enemy vs Pickup:
    // Enemy memberships & Pickup filters = 2 & 2 = 2 != 0 ✓
    // Pickup memberships & Enemy filters = 8 & 5 = 0 ✗
    println!("   Enemy vs Pickup: NO (Pickup memberships doesn't intersect Enemy filters)");

    // Enemy vs Wall:
    // Enemy memberships & Wall filters = 2 & 1 = 0 ✗
    println!("   Enemy vs Wall: NO (Enemy memberships doesn't intersect Wall filters)");
    println!();

    // 7. 修改碰撞掩码
    println!("7. Modifying Collision Masks at Runtime...");
    println!("   Before: Player mask = 0b0110 (Enemy | Wall)");

    if let Some(collider) = world.get_collider(0) {
        println!(
            "   (Player collider collision_group: {:04b})",
            collider.collision_group()
        );
        println!(
            "   (Player collider collision_mask: {:04b})",
            collider.collision_mask()
        );
    }

    // 注意：修改需要通过 get_collider_mut，但由于是 pub(crate) 只能演示
    println!("   (Runtime mask modification requires mutable access)");
    println!();

    println!("Collision groups demo completed successfully!");
}
