# 3D 物理示例实现指南

## 概述

本文档提供 `engine-physics-3d` crate 中各示例的实现指导，涵盖从基础物理模拟到高级角色控制器和关节系统的完整示例代码结构。

## 示例列表

| 示例名称 | 需求编号 | 描述 |
|----------|----------|------|
| `physics_3d_basic` | 197, 451 | 盒子从空中掉落 |
| `physics_3d_stack` | 198, 452 | 一堆盒子堆成塔 |
| `physics_3d_ragdoll` | 199, 453 | 铰接 ragdoll |
| `physics_3d_character` | 200, 454 | 角色控制器行走/跳跃/斜坡 |
| `physics_3d_joints` | 201, 455 | 钟摆 / 弹簧演示 |
| `physics_3d_ray_cast` | 202, 456 | 点击发射射线 |
| `physics_3d_trigger` | 203, 457 | 进入/离开传感器事件 |
| `physics_3d_heightfield` | 204, 458 | 高度场地形碰撞 |
| `physics_3d_compound` | 205, 459 | 复合碰撞体（子 collider） |
| `physics_3d_billiard` | 206, 460 | 撞球（可选） |

---

## 1. physics_3d_basic

### 需求

- 需求 197: `examples/physics_3d_basic`：盒子从空中掉落
- 需求 451: 盒子下落到地面

### 实现步骤

1. 创建 `PhysicsWorld3D`，设置重力 `(0.0, -9.81, 0.0)`
2. 创建静态地面刚体 (`RigidBodyBuilder::static_()`) 并添加盒碰撞体
3. 创建动态盒子刚体，初始位置在地面上方 `(0.0, 5.0, 0.0)`
4. 在游戏循环中调用 `world.step(dt)`
5. 每帧读取盒子刚体位置并更新场景变换

### 核心代码结构

```rust
fn main() {
    // 创建物理世界
    let mut world = PhysicsWorld3D::new(Vec3::new(0.0, -9.81, 0.0));
    
    // 创建地面
    let ground_handle = world.insert_body(
        RigidBodyBuilder::static_()
            .translation(Vec3::new(0.0, 0.0, 0.0))
            .build()
    );
    world.insert_collider(
        ColliderBuilder::cuboid(10.0, 0.1, 10.0).build(),
        ground_handle
    );
    
    // 创建掉落盒子
    let box_handle = world.insert_body(
        RigidBodyBuilder::dynamic()
            .translation(Vec3::new(0.0, 5.0, 0.0))
            .build()
    );
    world.insert_collider(
        ColliderBuilder::cuboid(0.5, 0.5, 0.5).build(),
        box_handle
    );
    
    // 游戏循环
    loop {
        world.step(dt);
        
        let pos = world.body(box_handle).position();
        // 更新场景变换...
    }
}
```

### 验收标准

- 盒子从 5 米高度落下，最终停在地面上
- `step_time` 稳定，CPU 占用正常

---

## 2. physics_3d_stack

### 需求

- 需求 198: `examples/physics_3d_stack`：一堆盒子堆成塔
- 需求 258: 100 个盒子稳定堆叠，CPU 稳定（>= 60fps）
- 需求 452: 堆叠盒子

### 实现步骤

1. 创建 `PhysicsWorld3D`
2. 创建静态地面
3. 使用循环创建 100 个盒子，位置从下往上堆叠
4. 每个盒子添加微小随机偏移以增加稳定性测试
5. 验证堆叠稳定性和性能

### 核心代码结构

```rust
fn create_stack(mut world: &mut PhysicsWorld3D, count: usize) {
    let ground = world.insert_body(
        RigidBodyBuilder::static_()
            .translation(Vec3::new(0.0, -0.5, 0.0))
            .build()
    );
    world.insert_collider(
        ColliderBuilder::cuboid(5.0, 0.5, 5.0).build(),
        ground
    );
    
    for i in 0..count {
        let y = i as f32 * 1.02; // 稍微分开一点
        let x = rand::random::<f32>() * 0.01;
        let z = rand::random::<f32>() * 0.01;
        
        let box_handle = world.insert_body(
            RigidBodyBuilder::dynamic()
                .translation(Vec3::new(x, y, z))
                .build()
        );
        world.insert_collider(
            ColliderBuilder::cuboid(0.5, 0.5, 0.5)
                .friction(0.7)
                .restitution(0.1)
                .build(),
            box_handle
        );
    }
}
```

### 验收标准

- 100 个盒子在 60fps 下稳定堆叠
- 堆叠不发生倒塌或穿透

---

## 3. physics_3d_ragdoll

### 需求

- 需求 199: `examples/physics_3d_ragdoll`：铰接 ragdoll
- 需求 227: `examples/physics_3d_ragdoll` ragdoll 崩溃稳定

### 实现步骤

1. 创建 ragdoll 各部位刚体：头部、躯干、上臂、下臂、手、大腿、小腿、脚
2. 使用 `FixedJoint` 连接相邻部位
3. 设置适当的 mass 和 inertia
4. 施加初始速度或力使 ragdoll 倒下

### 核心代码结构

```rust
struct RagdollParts {
    torso: RigidBodyHandle,
    head: RigidBodyHandle,
    upper_arm_l: RigidBodyHandle,
    lower_arm_l: RigidBodyHandle,
    upper_arm_r: RigidBodyHandle,
    lower_arm_r: RigidBodyHandle,
    upper_leg_l: RigidBodyHandle,
    lower_leg_l: RigidBodyHandle,
    upper_leg_r: RigidBodyHandle,
    lower_leg_r: RigidBodyHandle,
}

fn create_ragdoll(world: &mut PhysicsWorld3D) -> RagdollParts {
    // 创建躯干
    let torso = world.insert_body(
        RigidBodyBuilder::dynamic()
            .mass(10.0)
            .translation(Vec3::new(0.0, 2.0, 0.0))
            .build()
    );
    world.insert_collider(
        ColliderBuilder::cuboid(0.25, 0.4, 0.15).build(),
        torso
    );
    
    // 创建头部
    let head = world.insert_body(
        RigidBodyBuilder::dynamic()
            .mass(2.0)
            .translation(Vec3::new(0.0, 2.6, 0.0))
            .build()
    );
    world.insert_collider(
        ColliderBuilder::ball(0.15).build(),
        head
    );
    
    // 使用 FixedJoint 连接头和躯干
    world.insert_joint(torso, head, 
        FixedJointBuilder::new()
            .local_anchor1(Vec3::new(0.0, 0.4, 0.0))
            .local_anchor2(Vec3::new(0.0, -0.15, 0.0))
            .build()
    );
    
    // ... 其他部位和关节
    RagdollParts { /* ... */ }
}
```

### 验收标准

- Ragdoll 各部位通过关节正确连接
- 倒下后各关节保持约束
- 崩溃过程流畅，无穿模

---

## 4. physics_3d_character

### 需求

- 需求 200: `examples/physics_3d_character`：角色控制器行走/跳跃/斜坡
- 需求 259: 角色可在斜坡上站立、滑行

### 实现步骤

1. 创建 `CharacterController3D`
2. 创建 kinematic 刚体作为角色
3. 创建角色碰撞体（胶囊形状）
4. 处理输入：WASD 移动，空格跳跃
5. 每帧调用 `move_shape` 更新位置
6. 处理 `CharacterMovement` 结果

### 核心代码结构

```rust
fn main() {
    let mut world = PhysicsWorld3D::new(Vec3::new(0.0, -9.81, 0.0));
    
    // 创建地面（包含斜坡）
    let ground = world.insert_body(
        RigidBodyBuilder::static_()
            .translation(Vec3::new(0.0, 0.0, 0.0))
            .build()
    );
    world.insert_collider(
        ColliderBuilder::cuboid(10.0, 0.1, 10.0)
            .friction(0.5)
            .build(),
        ground
    );
    
    // 创建角色
    let character_body = world.insert_body(
        RigidBodyBuilder::kinematic_position_based()
            .translation(Vec3::new(0.0, 1.0, 0.0))
            .build()
    );
    let character_collider = world.insert_collider(
        ColliderBuilder::capsule(0.5, 0.3, Axis::Y)
            .friction(0.5)
            .build(),
        character_body
    );
    
    let mut controller = CharacterController3D::new(
        Vec3::new(0.0, 0.0, 0.0),      // offset
        Vec3::new(0.0, 1.0, 0.0),      // up_dir
        45.0_f32.to_radians(),        // max_slope_climb_angle
        30.0_f32.to_radians(),        // max_slide_angle
    );
    
    // 输入状态
    let mut input_dir = Vec3::ZERO;
    let mut wants_jump = false;
    
    loop {
        // 处理输入
        if is_key_pressed('W') { input_dir.z -= 1.0; }
        if is_key_pressed('S') { input_dir.z += 1.0; }
        if is_key_pressed('A') { input_dir.x -= 1.0; }
        if is_key_pressed('D') { input_dir.x += 1.0; }
        if is_key_pressed(' ') { wants_jump = true; }
        
        // 计算期望移动
        let desired = input_dir.normalize() * MOVE_SPEED * dt;
        if wants_jump && movement.grounded() {
            // 应用跳跃
            world.body_mut(character_body)
                .set_linvel(Vec3::new(0.0, JUMP_VELOCITY, 0.0), true);
            wants_jump = false;
        }
        
        // 移动角色
        let movement = controller.move_shape(
            dt,
            desired,
            world.body_mut(character_body),
            world.collider(character_collider),
            QueryFilter::new(),
        );
        
        // 更新位置
        let new_pos = world.body(character_body).position();
        // ... 更新场景变换
    }
}
```

### 验收标准

- 角色可在平地上行走
- 角色可跳跃
- 角色在斜坡上站立不滑动（小角度）
- 角色在陡坡上滑动（大角度）

---

## 5. physics_3d_joints

### 需求

- 需求 201: `examples/physics_3d_joints`：钟摆 / 弹簧演示

### 实现步骤

1. 创建固定点（Static 刚体）
2. 使用 `RevoluteJoint` 创建钟摆
3. 使用 `DistanceJoint` 或弹簧关节创建弹簧系统
4. 施加初始力或角度，观察振动

### 核心代码结构

```rust
fn create_pendulum(world: &mut PhysicsWorld3D) {
    // 创建固定锚点
    let anchor = world.insert_body(
        RigidBodyBuilder::static_()
            .translation(Vec3::new(0.0, 3.0, 0.0))
            .build()
    );
    
    // 创建摆锤
    let pendulum = world.insert_body(
        RigidBodyBuilder::dynamic()
            .translation(Vec3::new(0.0, 2.0, 0.0))
            .angvel(Vec3::new(0.0, 0.0, 2.0)) // 初始角速度
            .build()
    );
    world.insert_collider(
        ColliderBuilder::ball(0.3).build(),
        pendulum
    );
    
    // 使用 RevoluteJoint 连接
    world.insert_joint(anchor, pendulum,
        RevoluteJointBuilder::new(Vec3::new(0.0, 0.0, 1.0)) // 绕 Z 轴旋转
            .local_anchor1(Vec3::new(0.0, -1.0, 0.0))
            .local_anchor2(Vec3::new(0.0, 1.0, 0.0))
            .build()
    );
}
```

### 验收标准

- 钟摆绕固定点往复摆动
- 弹簧系统正常伸缩

---

## 6. physics_3d_ray_cast

### 需求

- 需求 202: `examples/physics_3d_ray_cast`：点击发射射线
- 需求 456: 点击发射射线，选中物体

### 实现步骤

1. 创建可点击的场景物体
2. 监听鼠标点击事件
3. 从摄像机发射射线
4. 使用 `Query3D::cast_ray` 检测命中
5. 高亮显示被选中的物体

### 核心代码结构

```rust
fn on_mouse_click(world: &PhysicsWorld3D, query: &Query3D, camera: &Camera, mouse_pos: Vec2) {
    // 从摄像机发射射线
    let ray = camera.screen_to_ray(mouse_pos);
    
    // 射线检测
    if let Some(hit) = query.cast_ray_and_get_normal(
        world,
        &ray,
        100.0,
        true,
        QueryFilter::new().exclude_fixed(),
    ) {
        // 高亮被选中的物体
        highlight_entity(hit.entity());
    }
}

fn highlight_entity(entity: Entity) {
    // 设置材质或显示高亮边框
}
```

### 验收标准

- 点击物体时射线正确检测
- 被选中的物体正确高亮

---

## 7. physics_3d_trigger

### 需求

- 需求 203: `examples/physics_3d_trigger`：进入/离开传感器事件
- 需求 457: sensor 进入/离开事件

### 实现步骤

1. 创建传感器碰撞体（`sensor(true)`）
2. 创建可进入的动态物体
3. 监听 `IntersectionEvent`
4. 在事件触发时执行相应逻辑

### 核心代码结构

```rust
fn create_trigger(world: &mut PhysicsWorld3D) {
    // 创建触发器
    let trigger_body = world.insert_body(
        RigidBodyBuilder::static_().build()
    );
    world.insert_collider(
        ColliderBuilder::cuboid(1.0, 1.0, 1.0)
            .sensor(true)
            .build(),
        trigger_body
    );
}

fn check_events(world: &PhysicsWorld3D) {
    for event in world.intersection_events() {
        match event {
            IntersectionEvent::Started(a, b) => {
                println!("物体进入触发器!");
            }
            IntersectionEvent::Stopped(a, b) => {
                println!("物体离开触发器!");
            }
        }
    }
}
```

### 验收标准

- 物体进入传感器时触发 Started 事件
- 物体离开传感器时触发 Stopped 事件

---

## 8. physics_3d_heightfield

### 需求

- 需求 204: `examples/physics_3d_heightfield`：高度场地形碰撞

### 实现步骤

1. 生成高度数据（程序或从纹理读取）
2. 使用 `ColliderBuilder::heightfield(heights, scale)` 创建碰撞体
3. 创建静态刚体并关联碰撞体
4. 可选：生成可视化网格

### 核心代码结构

```rust
fn create_terrain(world: &mut PhysicsWorld3D, heights: Vec<f32>, scale: Vec3) {
    let terrain = world.insert_body(
        RigidBodyBuilder::static_().build()
    );
    world.insert_collider(
        ColliderBuilder::heightfield(heights, scale).build(),
        terrain
    );
}
```

### 验收标准

- 高度场碰撞体正确生成
- 物体可在高度场上行走

---

## 9. physics_3d_compound

### 需求

- 需求 205: `examples/physics_3d_compound`：复合碰撞体（子 collider）
- 需求 459: 复合碰撞体

### 实现步骤

1. 创建一个刚体
2. 向同一刚体添加多个碰撞体（子碰撞体）
3. 子碰撞体相对于父刚体有各自的局部位置和旋转
4. 这形成复合碰撞体

### 核心代码结构

```rust
fn create_compound(world: &mut PhysicsWorld3D) {
    let compound = world.insert_body(
        RigidBodyBuilder::dynamic()
            .translation(Vec3::new(0.0, 5.0, 0.0))
            .build()
    );
    
    // 添加第一个碰撞体（盒子）
    world.insert_collider(
        ColliderBuilder::cuboid(0.5, 0.5, 0.5)
            .translation(Vec3::new(0.0, 0.0, 0.0))
            .build(),
        compound
    );
    
    // 添加第二个碰撞体（球，偏移到一边）
    world.insert_collider(
        ColliderBuilder::ball(0.3)
            .translation(Vec3::new(0.6, 0.0, 0.0))
            .build(),
        compound
    );
}
```

### 验收标准

- 复合碰撞体的多个子碰撞体正确关联到同一刚体
- 碰撞检测正确（多形状联合）

---

## 10. physics_3d_billiard (可选)

### 需求

- 需求 206: `examples/physics_3d_billiard`：撞球（可选）

### 实现步骤

1. 创建台球桌（静态碰撞体作为边界）
2. 创建多个球（动态刚体）
3. 使用射线检测或鼠标拖拽施加冲量
4. 实现球与球之间的弹性碰撞

---

## 验收总览

| 示例 | 验收条件 |
|------|----------|
| `physics_3d_basic` | 盒子正常下落 |
| `physics_3d_stack` | 100 盒子 >= 60fps |
| `physics_3d_ragdoll` | 关节保持约束 |
| `physics_3d_character` | 行走/跳跃/斜坡 |
| `physics_3d_joints` | 振动/摆动正常 |
| `physics_3d_ray_cast` | 点击选中物体 |
| `physics_3d_trigger` | 事件正确触发 |
| `physics_3d_heightfield` | 地形碰撞正常 |
| `physics_3d_compound` | 多形状联合正常 |
