# Module 07 — 角色控制器

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/character.rs`

## 1. 目标

实现可控制角色（Capsule 形状）：
- 楼梯自动越过
- 斜坡自动处理
- 贴墙滑行
- 地面粘连

## 2. CharacterController

```rust
pub struct CharacterController {
    pub body: BodyHandle,
    pub capsule: CapsuleShape,
    pub step_offset: f32,    // 台阶高度，默认 0.3
    pub slope_limit: f32,    // 斜坡角度（弧度），默认 45°
    pub contact_offset: f32, // 接触 skin，默认 0.01
    pub max_speed: f32,
    
    pub grounded: bool,
    pub ground_normal: Vec3,
    pub ground_body: Option<BodyHandle>,
}

pub struct CharacterMovement {
    pub desired_velocity: Vec3,  // 用户输入
    pub jump: bool,
}

pub struct CharacterCollisionInfo {
    pub collisions: Vec<CharacterContact>,
    pub grounded: bool,
}

pub struct CharacterContact {
    pub body: BodyHandle,
    pub normal: Vec3,
    pub penetration: f32,
}
```

## 3. Move 算法

```rust
pub fn move_character(
    world: &mut PhysicsWorld3D,
    controller: &mut CharacterController,
    movement: CharacterMovement,
    dt: f32,
) -> CharacterCollisionInfo {
    // 1. 沿 desired 速度尝试移动
    let mut current_pos = world.get_body(controller.body).transform.position;
    let velocity = movement.desired_velocity;
    
    // 2. 多步子移动（避免穿透）
    let sub_steps = 4;
    let sub_dt = dt / sub_steps as f32;
    let mut collisions = Vec::new();
    let mut grounded = false;
    
    for _ in 0..sub_steps {
        let step = velocity * sub_dt;
        let result = world.sweep_test_capsule(
            &controller.capsule,
            current_pos,
            current_pos + step,
        );
        
        if let Some(hit) = result {
            // 3. 沿地面 / 墙面分解运动
            let normal = hit.normal;
            let angle = normal.angle_between(Vec3::Y);
            
            if angle < controller.slope_limit {
                // 地面
                grounded = true;
                // 沿地面滑动
                let projected = velocity - velocity.project_on(normal);
                current_pos += projected * sub_dt;
            } else {
                // 墙面：贴墙滑行
                let dot = velocity.dot(normal);
                if dot < 0.0 {
                    current_pos += (step - normal * dot) * sub_dt;
                }
            }
            
            collisions.push(CharacterContact { /* ... */ });
        } else {
            current_pos += step;
        }
        
        // 4. 尝试上台阶
        if grounded && step.y > 0.0 {
            let step_check = world.sweep_test_capsule(
                &controller.capsule,
                current_pos,
                current_pos + Vec3::new(0.0, controller.step_offset, 0.0),
            );
            if step_check.is_none() {
                current_pos.y += controller.step_offset;
            }
        }
    }
    
    // 5. 更新位置
    let body = world.get_body_mut(controller.body);
    body.transform.position = current_pos;
    body.linear_vel = velocity;
    
    CharacterCollisionInfo { collisions, grounded }
}
```

## 4. 跳跃

```rust
pub fn jump(controller: &mut CharacterController, world: &mut PhysicsWorld3D, jump_speed: f32) {
    if controller.grounded {
        let body = world.get_body_mut(controller.body);
        body.linear_vel.y = jump_speed;
    }
}
```

## 5. 地面粘连

```rust
pub fn stick_to_ground(controller: &mut CharacterController, world: &mut PhysicsWorld3D) {
    if !controller.grounded { return; }
    
    // 向下扫一次
    let down = world.sweep_test_capsule(
        &controller.capsule,
        controller.position,
        controller.position - Vec3::new(0.0, 0.1, 0.0),
    );
    
    if let Some(hit) = down {
        // 移动到地面
        let body = world.get_body_mut(controller.body);
        body.transform.position.y -= hit.penetration;
    }
}
```

## 6. 验收

- [ ] 30° 斜坡自由行走
- [ ] 0.3m 高度台阶自动越过
- [ ] 贴墙滑行
- [ ] 跳跃：垂直跳起 + 落地
- [ ] 上下斜坡 60 FPS
- [ ] 与胶囊体半径检测一致
