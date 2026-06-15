# 示例实现指南（Implementation Guide for Examples）

## 概述

本文档提供 Sprint 04 示例工程的实现指南，涵盖每个示例的核心代码结构、关键实现点、以及验证方法。所有示例均位于 `examples/` 目录下。

---

## 示例清单

| 编号 | 示例名称 | 描述 | 对应需求 |
|------|----------|------|----------|
| 158 | `pixel_platformer` | 横版像素平台游戏 | 158-168, 343-350 |
| 159 | `ball_pit` | 1000 个球掉落物理演示 | 159, 350, 404 |
| 160 | `dominoes` | 多米诺骨牌物理演示 | 160, 351, 405 |
| 161 | `ray_cast` | 射线检测演示 | 161, 352, 406 |
| 162 | `joints` | 关节连接演示 | 162, 353, 407 |
| 163 | `scene_tree` | 节点树基础演示 | 163, 354, 408 |
| 164 | `prefab_basic` | 预制体基础 | 164, 355, 409 |
| 165 | `scene_switch` | 多场景切换 | 165, 356, 410 |
| 166 | `signals` | 信号系统 | 166, 357, 411 |
| 167 | `tween` | 补间动画 | 167, 358, 412 |
| 168 | `hello_engine` | 升级版最小演示 | 168, 413 |
| - | `timer` | 定时器演示 | 413 |
| - | `physics_perf` | 1000 球性能测试 | 360, 414 |

---

## 1. pixel_platformer 像素平台游戏

### 核心功能
- 玩家移动（左右方向键）
- 跳跃（空格键）
- 地面碰撞检测
- 敌人 AI（简单巡逻）
- 金币收集与得分
- HUD 显示分数与生命

### 实现结构

```rust
// main.rs
fn main() {
    let mut engine = Engine::new();
    let mut scene = SceneLoader::load_json("res/scenes/game.json").unwrap();
    
    // 设置玩家输入
    scene.get_node_mut::<Player>(player_handle)
        .map(|p| p.set_input(Input::keyboard()));
    
    engine.run(scene);
}
```

### 关键实现点

1. **玩家控制器**：使用 `Body2DNode` + `RigidBody2DBuilder::dynamic()`
2. **跳跃逻辑**：`apply_impulse_at_center(Vec2::Y * jump_force)` 当且仅当在地面上
3. **地面检测**：通过 `Area2D` 或 `ContactEvent` 判断
4. **敌人 AI**：`on_update` 中左右移动 + 边界检测反转
5. **金币**：使用 `Area2D` 作为 sensor，`on_entered` 检测玩家碰撞

### 验证方法
```bash
cargo run --example pixel_platformer
# 操作：← → 移动，空格 跳跃
# 预期：玩家可跳跃、收集金币、得分增加
```

---

## 2. ball_pit 物理球演示

### 核心功能
- 1000 个彩色球随机生成
- 重力下落
- 球与球、球与边界碰撞

### 实现结构

```rust
fn main() {
    let mut world = World2D::new_default();
    
    // 创建边界（静态刚体）
    let ground = RigidBody2DBuilder::static_().build();
    let ground_handle = world.insert_body(ground);
    let ground_collider = ColliderBuilder::rect(WIDTH, 20).build();
    world.insert_collider(ground_collider, ground_handle);
    
    // 生成 1000 个球
    for i in 0..1000 {
        let ball = RigidBody2DBuilder::dynamic()
            .translation(random_position())
            .build();
        let handle = world.insert_body(ball);
        
        let color = Color::hsl((i as f32 * 0.36) % 360.0, 0.8, 0.6);
        let collider = ColliderBuilder::circle(5.0)
            .material(PhysicsMaterial { friction: 0.3, restitution: 0.5, density: 1.0 })
            .build();
        world.insert_collider(collider, handle);
    }
    
    // 渲染循环
    loop {
        world.step(dt);
        renderer.draw(&world);
    }
}
```

### 关键实现点
1. 使用 `random_position()` 随机分布初始位置
2. 不同颜色通过 HSL 色彩空间生成
3. 碰撞体使用较小半径（5px）提高性能
4. 可关闭 sleep 提升大量刚体性能

### 验证方法
```bash
cargo run --example ball_pit
# 预期：1000 个球稳定下落，1680x720 下 >= 30fps
```

---

## 3. dominoes 多米诺骨牌

### 核心功能
- 多米诺骨牌排列
- 首个骨牌受击倒下
- 级联效应

### 实现结构

```rust
fn main() {
    let mut world = World2D::new_default();
    
    // 创建骨牌链
    for i in 0..20 {
        let x = 50.0 + i as f32 * 30.0;
        let domino = RigidBody2DBuilder::dynamic()
            .translation(Vec2::new(x, 50.0))
            .rotation(0.05)  // 稍微倾斜
            .build();
        let handle = world.insert_body(domino);
        
        let collider = ColliderBuilder::rect(5.0, 30.0).build();
        world.insert_collider(collider, handle);
    }
    
    // 推动第一个骨牌
    world.get_body_mut(first_handle)
        .apply_impulse_at_center(Vec2::X * 50.0);
}
```

---

## 4. ray_cast 射线检测

### 核心功能
- 鼠标射线投射
- 显示命中点
- 实时交互

### 实现结构

```rust
fn main() {
    let mut world = World2D::new_default();
    // ... 创建静态障碍物 ...
    
    let mut renderer = DebugRenderer::new();
    
    loop {
        let mouse_pos = input.mouse_position();
        
        // 从鼠标位置向下发射射线
        if let Some(hit) = world.ray_cast(
            mouse_pos,
            Vec2::NEG_Y,
            1000.0,
            QueryFilter { skip_bodies: vec![], include_sensors: true },
        ) {
            // 绘制命中点
            renderer.draw_circle(hit.point, 5.0, Color::RED);
            renderer.draw_line(hit.point, hit.point + hit.normal * 20.0, Color::GREEN);
        }
        
        world.step(dt);
        renderer.render();
    }
}
```

---

## 5. joints 关节演示

### 核心功能
- 钟摆结构（RevoluteJoint）
- 弹簧连接（SpringJoint）
- 铰链约束

### 实现结构

```rust
fn main() {
    let mut world = World2D::new_default();
    
    // 固定锚点
    let anchor = RigidBody2DBuilder::static_()
        .translation(Vec2::new(400.0, 100.0))
        .build();
    let anchor_handle = world.insert_body(anchor);
    
    // 摆锤
    let pendulum = RigidBody2DBuilder::dynamic()
        .translation(Vec2::new(400.0, 250.0))
        .build();
    let pendulum_handle = world.insert_body(pendulum);
    
    // 铰链关节
    let joint = RevoluteJointBuilder::new(anchor_handle, pendulum_handle, Vec2::new(400.0, 100.0))
        .build();
    world.insert_joint(joint);
    
    // 弹簧
    let spring = SpringJointBuilder::new(
        anchor_handle, pendulum_handle,
        Vec2::ZERO, Vec2::ZERO,
    )
    .stiffness(100.0)
    .damping(5.0)
    .rest_length(150.0)
    .build();
    world.insert_joint(spring);
}
```

---

## 6. scene_tree 节点树演示

### 核心功能
- 创建层级节点结构
- 展示父子关系
- 节点遍历

### 实现结构

```rust
fn main() {
    let mut scene = SceneTree::new();
    
    // 创建层级
    let root = scene.root();
    let child1 = scene.add_child(root, Node2D::new("Child1"));
    let child2 = scene.add_child(root, Node2D::new("Child2"));
    let grandchild = scene.add_child(child1, Node2D::new("GrandChild"));
    
    // 查找节点
    if let Some(handle) = scene.find_by_name("GrandChild") {
        let node = scene.get_node(handle);
        println!("Found: {}", node.name());
    }
    
    // 更新循环
    loop {
        scene.update(dt);
        scene.draw(&mut renderer);
    }
}
```

---

## 7. prefab_basic 预制体基础

### 核心功能
- 加载 Prefab
- 实例化多个副本
- 修改实例不影响模板

### 实现结构

```rust
fn main() {
    let prefab = Prefab::load_json("res/prefabs/enemy.json").unwrap();
    
    let mut scene = SceneTree::new();
    
    // 实例化多个敌人
    for i in 0..5 {
        let handle = prefab.instantiate_in(&mut scene);
        scene.get_node_mut(handle)
            .map(|n| n.set_position(Vec2::new(i as f32 * 100.0, 0.0)));
    }
}
```

---

## 8. scene_switch 多场景切换

### 核心功能
- 多个场景切换
- push/pop 保留历史
- 键盘切换（1/2/3）

### 实现结构

```rust
fn main() {
    let mut scene_manager = SceneManager::new();
    
    scene_manager.load("res/scenes/title.json");
    scene_manager.load("res/scenes/game.json");
    scene_manager.load("res/scenes/gameover.json");
    
    scene_manager.switch_to("title");
    
    loop {
        if input.is_key_pressed(Key::Key1) {
            scene_manager.switch_to("title");
        } else if input.is_key_pressed(Key::Key2) {
            scene_manager.push("game");  // 保留 title
        } else if input.is_key_pressed(Key::Key3) {
            scene_manager.pop();  // 回到 title
        }
        
        scene_manager.current_mut().update(dt);
    }
}
```

---

## 9. signals 信号系统

### 核心功能
- 按钮点击
- 信号派发
- 处理器响应

### 实现结构

```rust
// 按钮节点
struct Button {
    node2d: Node2D,
}

impl Node for Button {
    fn on_update(&mut self, dt: f32) {
        if self.is_clicked() {
            self.emit("clicked");
        }
    }
}

// 处理器连接
let mut button = Button { /* ... */ };
button.connect("clicked", |args| {
    println!("Button clicked!");
});
```

---

## 10. tween 补间动画演示

### 核心功能
- 多种缓动曲线
- 位置/缩放/旋转动画
- 重复与往复

### 实现结构

```rust
fn main() {
    let mut tween_manager = TweenManager::new();
    
    let sprite_handle = scene.find_by_name("Cube").unwrap();
    
    // 位置动画
    let tween = Tween::new(
        Vec2::ZERO,
        Vec2::new(200.0, 100.0),
        2.0,
        Ease::InOutCubic,
    )
    .with_repeat(3, TweenRepeatMode::Times(3))
    .with_yoyo(true)
    .on_complete(|| println!("Animation done!"))
    .build();
    
    tween_manager.add(tween);
    
    loop {
        tween_manager.update(dt);
        
        // 应用到节点
        if let Some(tween) = tween_manager.get(handle) {
            scene.get_node_mut(sprite_handle)
                .set_position(tween.value().as_vec2());
        }
    }
}
```

---

## 11. hello_engine 升级版

### 核心功能
- 节点树结构
- 2D 精灵渲染
- 最小完整可运行

### 实现结构

```rust
fn main() {
    let mut scene = SceneTree::new();
    
    // 添加精灵
    let sprite = Sprite2D::new("Player", Sprite::from_file("res/sprites/player.png"));
    scene.add_child(scene.root(), sprite);
    
    // 添加相机
    let camera = Camera2DNode::new("MainCamera", Camera2D::new());
    scene.add_child(scene.root(), camera);
    
    Engine::new().run(scene);
}
```

---

## 12. timer 定时器演示

### 核心功能
- 倒计时显示
- Once / Repeat 模式
- 回调触发

### 实现结构

```rust
fn main() {
    let mut timer = Timer::new(1.0, TimerMode::Repeat);
    
    loop {
        if timer.tick(dt) {
            println!("Timer fired! Elapsed: {}", timer.elapsed());
        }
        timer.reset();
    }
}
```

---

## 13. physics_perf 性能测试

### 核心功能
- 1000 球体性能测试
- FPS 显示
- 性能指标记录

### 验证标准
- 100 个球在 1680x720 下稳定 60fps
- 1000 个球在 1680x720 下 >= 30fps

---

## Debug 功能

所有示例支持以下 Debug 快捷键：

| 按键 | 功能 |
|------|------|
| `` ` `` + B | 显示/隐藏碰撞体线框 |
| `` ` `` + P | 暂停/继续物理 |
| `` ` `` + F | 显示/隐藏 FPS / FrameTime |

---

## 验收检查清单

- [ ] `pixel_platformer` 可玩（跳跃、碰撞、得分）
- [ ] `ball_pit` 1000 个球稳定 60fps (100球) / 30fps (1000球)
- [ ] `dominoes` 多米诺骨牌倒下效果
- [ ] `ray_cast` 鼠标射线检测正常
- [ ] `joints` 钟摆/弹簧关节演示正常
- [ ] `scene_tree` 节点层级演示正常
- [ ] `prefab_basic` 实例化多个 Prefab 无崩溃
- [ ] `scene_switch` 按键切换场景正常
- [ ] `signals` 信号派发正常
- [ ] `tween` 多种缓动演示正常
- [ ] `timer` 定时器演示正常
- [ ] `physics_perf` 性能指标达标
- [ ] Debug 按键功能正常
