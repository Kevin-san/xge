# 示例实现指南

## 概述

本文档提供 Sprint 09 中各示例的实现指导，包括目录结构、核心代码逻辑和实现要点。

---

## 1. 示例列表

| 示例名称 | 路径 | 描述 |
|----------|------|------|
| mesh_viewer | `examples/mesh_viewer/` | 3D 模型查看器 |
| primitives_demo | `examples/primitives_demo/` | 基本几何体展示 |
| 3d_scene_simple | `examples/3d_scene_simple/` | 简单场景 |
| 3d_lights | `examples/3d_lights/` | 多光源演示 |
| 3d_frustum_cull | `examples/3d_frustum_cull/` | 视锥裁剪演示 |
| 3d_picker | `examples/3d_picker/` | 鼠标拾取 |
| 3d_shader_hot_reload | `examples/3d_shader_hot_reload/` | 着色器热重载 |

---

## 2. mesh_viewer

### 2.1 功能需求

- [ ] 命令行参数加载 GLTF
- [ ] WASD 飞行相机
- [ ] 鼠标右键旋转
- [ ] 数字键 1-6 切换渲染模式

### 2.2 目录结构

```
examples/mesh_viewer/
├── Cargo.toml
└── src/
    └── main.rs
```

### 2.3 核心实现

```rust
// main.rs 伪代码结构

// 1. 初始化引擎和 3D 渲染器
let engine = Engine::new();
let renderer_3d = RenderPipeline3D::new(&engine.renderer, config)?;

// 2. 加载 GLTF 文件
let args: Vec<String> = env::args().collect();
if args.len() > 1 {
    let mesh = Mesh3D::from_file(&args[1])?;
    scene.add_node(Node3D::with_mesh(mesh));
}

// 3. 相机控制
struct FlyCamera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
}

impl FlyCamera {
    fn update(&mut self, dt: f32, input: &Input) {
        // WASD 前后左右移动
        // 鼠标右键拖拽旋转
        // 更新 Camera3D
    }
}

// 4. 渲染模式切换
enum RenderMode {
    Solid,
    Wireframe,
    Normal,
    AABB,
    Light,
    Combined,
}

fn handle_key_input(key: KeyCode) -> Option<RenderMode> {
    match key {
        KeyCode::Key1 => Some(RenderMode::Solid),
        KeyCode::Key2 => Some(RenderMode::Wireframe),
        KeyCode::Key3 => Some(RenderMode::Normal),
        KeyCode::Key4 => Some(RenderMode::AABB),
        KeyCode::Key5 => Some(RenderMode::Light),
        KeyCode::Key6 => Some(RenderMode::Combined),
        _ => None,
    }
}

// 5. 主循环
loop {
    // 处理输入
    fly_camera.update(delta_time, &input);
    
    // 更新场景
    scene.update_world_transforms();
    scene.cull(&frustum);
    
    // 渲染
    pipeline.begin_frame();
    pipeline.draw_scene(&scene, &camera, &lights);
    pipeline.end_frame();
    
    // 显示统计 (~ 键切换)
    if show_stats {
        println!("{}", pipeline.stats().to_string());
    }
}
```

### 2.4 实现要点

1. **GLTF 加载**：使用 `Mesh3D::from_file("model.gltf")`
2. **相机控制**：基于 Camera3D 的 `look_at()` 和矩阵计算
3. **渲染模式**：通过 `RenderPipeline3D::set_wireframe()` 切换
4. **统计显示**：使用 `RenderStats3D::to_string()`

---

## 3. primitives_demo

### 3.1 功能需求

- [ ] 展示所有基本几何体
- [ ] 旋转动画

### 3.2 核心实现

```rust
// 创建一个网格管理器展示所有图元
let primitives = vec![
    ("Cube", Mesh3D::cube(1.0)),
    ("Sphere", Mesh3D::sphere(0.5, 32, 16)),
    ("Plane", Mesh3D::plane(vec2(1.0, 1.0), 1)),
    ("Cylinder", Mesh3D::cylinder(0.5, 1.0, 32)),
    ("Cone", Mesh3D::cone(0.5, 1.0, 32)),
    ("Torus", Mesh3D::torus(0.7, 0.3, 32, 16)),
    ("Capsule", Mesh3D::capsule(0.3, 1.0, 16)),
];

// 布局摆放（3D 网格排列）
for (i, (_, mesh)) in primitives.iter().enumerate() {
    let row = i / 3;
    let col = i % 3;
    let x = (col as f32 - 1.0) * 3.0;
    let z = (row as f32 - 1.0) * 3.0;
    let mut node = Node3D::with_mesh(mesh.clone());
    node.set_translation(vec3(x, 0.0, z));
    scene.add_node(node);
}

// 动画：每帧旋转
loop {
    for node in scene.nodes_mut() {
        node.rotate(Quat::from_euler(glam::EulerRot::XYZ, 0.0, delta_time, 0.0));
    }
    // 渲染...
}
```

---

## 4. 3d_scene_simple

### 4.1 功能需求

- [ ] 立方体 + 平面 + 方向光
- [ ] 正确渲染光照效果

### 4.2 核心实现

```rust
// 创建场景
let mut scene = Scene3D::new();

// 添加平面（地面）
let ground = Mesh3D::plane(vec2(10.0, 10.0), 1);
let mut ground_node = Node3D::with_mesh(ground);
ground_node.set_translation(vec3(0.0, -1.0, 0.0));
scene.add_node(ground_node);

// 添加立方体
let cube = Mesh3D::cube(1.0);
let cube_node = Node3D::with_mesh(cube);
scene.add_node(cube_node);

// 添加方向光
let mut lights = LightManager::new();
lights.add_directional(DirectionalLight::new(
    vec3(-0.5, -1.0, -0.5).normalize(),
    Color::rgb(1.0, 1.0, 1.0),
    1.0,
));

// 设置材质
let material = Material3D::from_color(Color::rgb(0.8, 0.2, 0.2));
// 挂接到节点...

// 渲染
pipeline.draw_scene(&scene, &camera, &lights);
```

---

## 5. 3d_lights

### 5.1 功能需求

- [ ] 点光源演示
- [ ] 方向光演示
- [ ] 聚光灯演示

### 5.2 核心实现

```rust
let mut lights = LightManager::new();

// 方向光
lights.add_directional(DirectionalLight::new(
    vec3(-1.0, -1.0, -1.0).normalize(),
    Color::rgb(1.0, 1.0, 0.9),
    0.8,
));

// 点光源
lights.add_point(PointLight::new(
    vec3(2.0, 2.0, 0.0),
    Color::rgb(1.0, 0.2, 0.2),
    1.5,
    5.0,  // radius
));

// 聚光灯
lights.add_spot(SpotLight::new(
    vec3(0.0, 3.0, 0.0),
    vec3(0.0, -1.0, 0.0),
    30.0_f32.to_radians(),  // inner_angle
    45.0_f32.to_radians(), // outer_angle
    Color::rgb(0.2, 0.2, 1.0),
    2.0,
));

// 环境光
lights.set_ambient(AmbientLight::new(
    Color::rgb(0.1, 0.1, 0.1),
    0.3,
));

// 场景中放置多个物体展示不同光源效果
```

---

## 6. 3d_frustum_cull

### 6.1 功能需求

- [ ] 视锥裁剪演示
- [ ] 显示 Cull Stats

### 6.2 核心实现

```rust
// 创建大量随机分布的物体
for i in 0..100 {
    let mesh = Mesh3D::cube(0.5);
    let mut node = Node3D::with_mesh(mesh);
    node.set_translation(vec3(
        rand::random::<f32>() * 20.0 - 10.0,
        rand::random::<f32>() * 10.0 - 5.0,
        rand::random::<f32>() * 20.0 - 10.0,
    ));
    scene.add_node(node);
}

// 从相机生成视锥
let camera = scene.main_camera().unwrap();
let view_projection = camera.view_projection();
let frustum = Frustum::from_view_projection(view_projection);

// 执行裁剪
scene.cull(&frustum);

// 显示统计
let stats = scene.stats();
println!("Total nodes: {}", stats.nodes);
println!("Visible nodes: {}", stats.visible_nodes);
println!("Culled nodes: {}", stats.nodes - stats.visible_nodes);
println!("Total triangles: {}", stats.total_triangles);

// 可选：可视化视锥
debug_renderer.frustum(camera, Color::rgb(0.0, 1.0, 0.0));
```

---

## 7. 3d_picker

### 7.1 功能需求

- [ ] 鼠标点击选中 mesh
- [ ] ray-mesh intersection

### 7.2 核心实现

```rust
// 鼠标点击处理
fn on_mouse_click(screen_pos: Vec2, screen_size: Vec2, scene: &Scene3D, camera: &Camera3D) {
    // 1. 从屏幕位置生成世界射线
    let ray = camera.screen_to_world_ray(screen_pos, screen_size);
    
    // 2. 遍历场景节点进行射线检测
    let mut closest_hit: Option<HitResult> = None;
    let mut hit_node: Option<NodeHandle> = None;
    
    for (handle, node) in scene.nodes().iter().enumerate() {
        if let Some(mesh_handle) = node.mesh() {
            if let Some(mesh) = mesh_manager.get(mesh_handle) {
                if let Some(hit) = ray.hit_mesh(mesh, node.world_transform()) {
                    if closest_hit.is_none() || hit.t < closest_hit.unwrap().t {
                        closest_hit = Some(hit);
                        hit_node = Some(handle);
                    }
                }
            }
        }
    }
    
    // 3. 高亮选中的物体
    if let Some(node_handle) = hit_node {
        highlight_node(scene, node_handle);
    }
}

// 高亮效果（改变材质或显示边框）
fn highlight_node(scene: &mut Scene3D, handle: NodeHandle) {
    let node = scene.node_mut(handle);
    // 保存原材质，设置为高亮材质
}
```

---

## 8. 3d_shader_hot_reload

### 8.1 功能需求

- [ ] 监视着色器文件变化
- [ ] 热重载重新编译

### 8.2 核心实现

```rust
// 监视着色器目录
let shader_dir = "shaders/3d/";
let mut file_watcher = FileWatcher::new(shader_dir);

loop {
    // 检查文件变化
    if let Some(changed_file) = file_watcher.check() {
        println!("Shader changed: {}", changed_file);
        
        // 重新编译
        pipeline.recompile_shaders();
        
        // 输出编译结果
        if let Err(e) = result {
            eprintln!("Shader compile error: {}", e);
        }
    }
    
    // 正常渲染...
}
```

---

## 9. 通用实现模板

### 9.1 Cargo.toml 模板

```toml
[package]
name = "example_xxx"
version = "0.1.0"
edition = "2021"

[dependencies]
engine-render = { path = "../../engine-render" }
engine-render-3d = { path = "../../engine-render-3d" }
engine-math = { path = "../../engine-math" }
glam = "0.25"
```

### 9.2 基础 main.rs 结构

```rust
use engine_math::{Vec3, Quat, Color};
use engine_render::{Engine, Renderer};
use engine_render_3d::{
    Scene3D, Node3D, Camera3D, Mesh3D, Material3D,
    RenderPipeline3D, LightManager, DirectionalLight,
};

fn main() {
    // 1. 初始化
    let engine = Engine::new();
    let renderer = engine.renderer();
    
    // 2. 创建 3D 渲染管线
    let pipeline = RenderPipeline3D::new(renderer, config).unwrap();
    
    // 3. 创建场景
    let mut scene = Scene3D::new();
    
    // 4. 创建相机
    let camera = Camera3D::perspective(60.0_f32.to_radians(), 16.0/9.0, 0.1, 100.0);
    
    // 5. 创建光源
    let mut lights = LightManager::new();
    lights.add_directional(DirectionalLight::new(
        vec3(-1.0, -1.0, -1.0).normalize(),
        Color::WHITE,
        1.0,
    ));
    
    // 6. 添加场景内容...
    
    // 7. 主循环
    loop {
        // 更新
        scene.update_world_transforms();
        
        // 渲染
        pipeline.begin_frame();
        pipeline.draw_scene(&scene, &camera, &lights);
        pipeline.end_frame();
    }
}
```

---

## 10. 实现检查清单

每个示例实现前检查：

- [ ] 创建独立的 Cargo.toml
- [ ] 添加必要的 dependencies
- [ ] 实现 main.rs 基本结构
- [ ] 处理输入（如果需要）
- [ ] 创建场景内容
- [ ] 实现渲染循环
- [ ] 添加键盘/鼠标交互（如果需要）
- [ ] 运行 `cargo check` 无错误
- [ ] 运行示例验证功能正确