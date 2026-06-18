# 示例实现指南

## 概述

本文档提供 `engine-editor` crate 中示例程序的实现指导，包括 `editor_app`、`editor_custom_panel`、`editor_plugin`、`editor_game` 四个示例。

## 示例列表

1. [editor_app](#1-editor_app) - 启动编辑器
2. [editor_custom_panel](#2-editor_custom_panel) - 自定义面板
3. [editor_plugin](#3-editor_plugin) - 自定义插件
4. [editor_game](#4-editor_game) - 简单游戏开发

---

## 1. editor_app

### 概述

最基本的编辑器启动示例，展示如何创建 EditorApp 并运行。

### 文件位置

```
examples/editor_app.rs
```

### 实现步骤

```rust
// 1. 创建 Window
let window = Window::new("Editor", 1280, 720);

// 2. 创建 Engine
let engine = Engine::new();

// 3. 创建 EditorApp
let mut editor = EditorApp::new(window, engine);

// 4. 配置面板（可选）
editor.show_panel("hierarchy");
editor.show_panel("inspector");
editor.show_panel("asset");
editor.show_panel("console");
editor.show_panel("debug");

// 5. 运行编辑器
editor.run();
```

### 预期效果

- 弹出编辑器窗口
- 显示默认 Docking 布局
- 顶部菜单可用
- 可创建节点、选择节点

### 验收标准

- [ ] `cargo run --example editor_app` 可启动编辑器
- [ ] 编辑器窗口正常显示
- [ ] 菜单系统正常工作
- [ ] 面板可显示/隐藏

---

## 2. editor_custom_panel

### 概述

展示如何创建自定义面板并注册到编辑器。

### 文件位置

```
examples/editor_custom_panel.rs
```

### 实现步骤

```rust
use engine_editor::{Panel, EditorApp, Ui};

// 1. 定义自定义面板
pub struct MyCustomPanel {
    title: String,
    message: String,
}

impl MyCustomPanel {
    pub fn new() -> Self {
        Self {
            title: "My Panel".to_string(),
            message: "Hello from custom panel!".to_string(),
        }
    }
}

impl Panel for MyCustomPanel {
    fn title(&self) -> &str {
        &self.title
    }

    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui) {
        ui.heading(&self.title);
        ui.label(&self.message);
        
        if ui.button("Click Me").clicked() {
            self.message = "Button clicked!".to_string();
        }
        
        // 访问编辑器状态
        let selection = editor.selection();
        ui.label(format!("Selected entities: {}", selection.len()));
    }
}

// 2. 在 main 中注册面板
fn main() {
    let window = Window::new("Custom Panel Example", 1280, 720);
    let engine = Engine::new();
    let mut editor = EditorApp::new(window, engine);
    
    // 注册自定义面板
    editor.add_panel(MyCustomPanel::new());
    
    editor.run();
}
```

### 预期效果

- 编辑器中显示自定义面板
- 面板显示标题和消息
- 点击按钮更新消息
- 显示当前选择集数量

### 验收标准

- [ ] 自定义面板正确显示
- [ ] 面板可与编辑器状态交互
- [ ] 面板支持 show/hide

---

## 3. editor_plugin

### 概述

展示如何创建 EditorPlugin 扩展。

### 文件位置

```
examples/editor_plugin.rs
```

### 实现步骤

```rust
use engine_editor::{EditorPlugin, EditorApp, Ui, EditorMode};

pub struct MyPlugin {
    enabled: bool,
    counter: i32,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            enabled: true,
            counter: 0,
        }
    }
}

impl EditorPlugin for MyPlugin {
    fn register(&mut self, editor: &mut EditorApp) {
        // 注册菜单项
        editor.add_menu_item("Tools", "My Plugin");
        println!("Plugin registered");
    }

    fn update(&mut self, editor: &mut EditorApp, dt: f32) {
        if self.enabled {
            self.counter += 1;
            if self.counter % 60 == 0 {
                println!("Plugin update: {:.2}s", dt * 60.0);
            }
        }
    }

    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui) {
        ui.checkbox(&mut self.enabled, "Enable Plugin");
        ui.label(format!("Counter: {}", self.counter));
        
        if ui.button("Reset").clicked() {
            self.counter = 0;
        }
        
        // 显示当前编辑器模式
        ui.label(format!("Mode: {:?}", editor.mode()));
    }
}

fn main() {
    let window = Window::new("Plugin Example", 1280, 720);
    let engine = Engine::new();
    let mut editor = EditorApp::new(window, engine);
    
    // 注册插件
    editor.register_plugin(MyPlugin::new());
    
    editor.run();
}
```

### 预期效果

- 插件正确注册
- 插件 update 定期执行
- 插件 UI 显示在编辑器中
- 插件可启用/禁用

### 验收标准

- [ ] 插件成功注册到编辑器
- [ ] update 方法定期调用
- [ ] 插件 UI 正常显示
- [ ] 插件可启用/禁用

---

## 4. editor_game

### 概述

展示如何使用编辑器开发简单 2D 游戏。

### 文件位置

```
examples/editor_game.rs
```

### 实现步骤

```rust
// 1. 定义游戏组件
#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Sprite {
    texture: String,
}

// 2. 创建游戏场景
fn create_game_scene() -> SceneTree {
    let mut scene = SceneTree::new();
    
    // 创建玩家节点
    let player = scene.create_entity();
    scene.set_name(player, "Player");
    scene.add_component(player, Player { speed: 100.0 });
    scene.add_component(player, Sprite { texture: "player.png".to_string() });
    scene.set_position(player, Vec2::new(400.0, 300.0));
    
    // 创建背景节点
    let background = scene.create_entity();
    scene.set_name(background, "Background");
    scene.add_component(background, Sprite { texture: "bg.png".to_string() });
    
    // 创建敌人
    for i in 0..5 {
        let enemy = scene.create_entity();
        scene.set_name(enemy, format!("Enemy_{}", i));
        scene.add_component(enemy, Sprite { texture: "enemy.png".to_string() });
        scene.set_position(enemy, Vec2::new(100.0 + i as f32 * 150.0, 100.0));
    }
    
    scene
}

// 3. 创建游戏插件
struct GamePlugin;

impl EditorPlugin for GamePlugin {
    fn register(&mut self, editor: &mut EditorApp) {
        // 添加游戏菜单
        editor.add_menu_item("Game", "Start Game");
        editor.add_menu_item("Game", "Pause Game");
        editor.add_menu_item("Game", "End Game");
    }
    
    fn update(&mut self, editor: &mut EditorApp, dt: f32) {
        if editor.mode() == EditorMode::Play {
            // 游戏逻辑更新
            let scene = editor.scene_mut();
            for entity in scene.query::<&Player>() {
                // 玩家移动逻辑
            }
        }
    }
    
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui) {
        if editor.mode() == EditorMode::Play {
            ui.label("Game Running...");
            
            // 显示 FPS
            ui.label("FPS: 60");
            
            // 游戏状态
            ui.label("Score: 0");
        }
    }
}

// 4. main 函数
fn main() {
    let window = Window::new("Game Editor", 1280, 720);
    let engine = Engine::new();
    let mut editor = EditorApp::new(window, engine);
    
    // 设置初始场景
    let scene = create_game_scene();
    editor.set_scene(scene);
    
    // 注册游戏插件
    editor.register_plugin(GamePlugin);
    
    // 运行编辑器
    editor.run();
}
```

### 预期效果

- 编辑器加载游戏场景
- 场景中显示玩家和敌人
- Play 模式下游戏运行
- 显示游戏 UI（FPS、分数）

### 验收标准

- [ ] 场景正确加载
- [ ] 实体正确显示在层级面板
- [ ] Play 模式可运行
- [ ] 游戏 UI 正常显示

---

## 通用验收标准

所有示例应满足：

- [ ] `cargo run --example <example_name>` 正常运行
- [ ] 窗口正常显示
- [ ] 无 panic 或 panic
- [ ] 资源正确加载（纹理、场景等）
- [ ] 可正常退出
