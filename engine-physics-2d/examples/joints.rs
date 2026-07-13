//! joints.rs - 关节连接演示
//!
//! 本示例演示如何在 PhysicsWorld2D 中创建和使用关节连接，
//! 包括 DistanceJoint、RevoluteJoint 和 PrismaticJoint。

use engine_math::Vec2;
use engine_physics_2d::{
    DistanceJoint, Joint2D, PhysicsWorld2D, PrismaticJoint, RevoluteJoint, RigidBody2DBuilder,
};

fn main() {
    println!("=== Joints Demo ===");
    println!();

    let mut world = PhysicsWorld2D::with_default_config();
    // 降低重力便于观察关节效果
    world.set_gravity(Vec2::new(0.0, -2.0));

    // 1. 创建两个动态刚体
    println!("1. Creating two dynamic bodies...");
    let body_a = RigidBody2DBuilder::dynamic()
        .with_mass(1.0)
        .with_position(Vec2::new(0.0, 5.0))
        .build();
    let body_a_index = world.add_body(body_a);
    println!(
        "   - Body A at index: {}, position: {:?}",
        body_a_index,
        Vec2::new(0.0, 5.0)
    );

    let body_b = RigidBody2DBuilder::dynamic()
        .with_mass(1.0)
        .with_position(Vec2::new(3.0, 5.0))
        .build();
    let body_b_index = world.add_body(body_b);
    println!(
        "   - Body B at index: {}, position: {:?}",
        body_b_index,
        Vec2::new(3.0, 5.0)
    );
    println!();

    // 2. 创建距离关节（DistanceJoint）
    println!("2. Creating DistanceJoint...");
    let anchor_a = Vec2::new(0.0, 5.0); // 关节在 body A 上的锚点
    let anchor_b = Vec2::new(3.0, 5.0); // 关节在 body B 上的锚点
    let distance_joint = DistanceJoint::new(body_a_index, body_b_index, anchor_a, anchor_b);
    let joint_index = world.add_joint(Joint2D::new(
        engine_physics_2d::JointType::Distance,
        body_a_index,
        body_b_index,
    ));
    println!("   - Joint index: {}", joint_index);
    println!("   - Joint type: DistanceJoint");
    println!("   - Distance: {:.2}", distance_joint.length());
    println!();

    // 3. 模拟观察距离约束
    println!("3. Simulating with DistanceJoint (watch distance constraint)...");
    println!(
        "   {:^6} | {:^12} | {:^12} | {:^12}",
        "Step", "Body A Pos", "Body B Pos", "Dist"
    );
    println!("   {:->6} | {:->12} | {:->12} | {:->12}", "", "", "", "");

    for step in 0..8 {
        world.step(1.0 / 60.0);

        if let Some(body_a_state) = world.get_body(body_a_index) {
            if let Some(body_b_state) = world.get_body(body_b_index) {
                let pos_a = body_a_state.position();
                let pos_b = body_b_state.position();
                let dist = pos_a.distance(pos_b);
                if step % 2 == 0 {
                    println!(
                        "   {:6} | ({:5.2}, {:5.2}) | ({:5.2}, {:5.2}) | {:6.2}",
                        step, pos_a.x, pos_a.y, pos_b.x, pos_b.y, dist
                    );
                }
            }
        }
    }
    println!();

    // 4. 创建旋转关节（RevoluteJoint）
    println!("4. Creating RevoluteJoint...");
    let mut world2 = PhysicsWorld2D::with_default_config();
    world2.set_gravity(Vec2::ZERO); // 零重力便于观察旋转

    let body1 = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let body1_idx = world2.add_body(body1);

    let body2 = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let body2_idx = world2.add_body(body2);

    let revolute_joint = RevoluteJoint::new(body1_idx, body2_idx, Vec2::new(2.0, 0.0));
    let revolute_idx = world2.add_joint(Joint2D::new(
        engine_physics_2d::JointType::Revolute,
        body1_idx,
        body2_idx,
    ));
    println!("   - Revolute joint index: {}", revolute_idx);
    println!("   - Joint angle: {:.2} rad", revolute_joint.angle());
    println!();

    // 5. 创建滑块关节（PrismaticJoint）
    println!("5. Creating PrismaticJoint...");
    let mut world3 = PhysicsWorld2D::with_default_config();
    world3.set_gravity(Vec2::ZERO);

    let p_body1 = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let p_body1_idx = world3.add_body(p_body1);

    let p_body2 = RigidBody2DBuilder::dynamic()
        .with_position(Vec2::new(0.0, 0.0))
        .build();
    let p_body2_idx = world3.add_body(p_body2);

    let prismatic_joint = PrismaticJoint::new(
        p_body1_idx,
        p_body2_idx,
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0), // 沿 X 轴滑动
    );
    let prismatic_idx = world3.add_joint(Joint2D::new(
        engine_physics_2d::JointType::Prismatic,
        p_body1_idx,
        p_body2_idx,
    ));
    println!("   - Prismatic joint index: {}", prismatic_idx);
    println!("   - Axis: X direction (1.0, 0.0)");
    println!(
        "   - Axis vector: ({:.2}, {:.2})",
        prismatic_joint.axis().x,
        prismatic_joint.axis().y
    );
    println!();

    // 6. 演示移除关节
    println!("6. Testing joint removal...");
    println!("   Joint count before removal: {}", world.joint_count());
    world.remove_joint(joint_index);
    println!("   Joint count after removal: {}", world.joint_count());
    println!();

    // 7. 使用新关节类型
    println!("7. Testing new joint types...");

    // 弹簧关节
    let spring_joint = engine_physics_2d::SpringJoint::new(
        body_a_index,
        body_b_index,
        Vec2::ZERO,
        Vec2::new(3.0, 0.0),
    );
    println!(
        "   - Spring joint stiffness: {:.2}",
        spring_joint.stiffness()
    );
    println!("   - Spring joint damping: {:.2}", spring_joint.damping());
    println!("   - Spring rest length: {:.2}", spring_joint.rest_length());

    // 焊接关节
    let weld_joint = engine_physics_2d::WeldJoint::new(
        body_a_index,
        body_b_index,
        Vec2::ZERO,
        Vec2::new(3.0, 0.0),
    );
    println!("   - Weld joint stiffness: {:.2}", weld_joint.stiffness());

    // 驱动关节
    let motor_joint = engine_physics_2d::MotorJoint::new(body_a_index, body_b_index);
    println!(
        "   - Motor joint max force: {:.2}",
        motor_joint.motor_max_force()
    );
    println!();

    println!("Joints demo completed successfully!");
}
