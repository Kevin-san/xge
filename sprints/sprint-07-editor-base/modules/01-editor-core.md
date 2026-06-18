# 模块一：编辑器核心需求

## 1. 模块概述

编辑器核心模块建立 `engine-editor` crate，提供编辑器主窗口、Docking 布局、菜单系统、EditorApp 生命周期管理、EditorState 状态管理、EditorMode 模式切换、EditorSettings 配置管理等基础设施。

## 2. 功能需求

### 2.1 项目建立

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 1 | `engine-editor` crate 建立 | P0 |

### 2.2 EditorApp 主结构体

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 2 | `EditorApp` 结构体：持有引擎、当前场景、选择集 | P0 |
| 3 | `EditorApp::run()`：启动编辑器主循环 | P0 |
| 141 | `EditorApp::new(window, engine)` 构造 | P0 |
| 142 | `EditorApp::run(&mut self)` 启动 | P0 |
| 143 | `EditorApp::update(&mut self, dt)` | P0 |
| 144 | `EditorApp::render(&mut self)` | P0 |
| 145 | `EditorApp::handle_event(&mut self, event)` | P0 |
| 146 | `EditorApp::mode(&self) -> EditorMode` | P0 |
| 147 | `EditorApp::set_mode(&mut self, mode)` | P0 |
| 148 | `EditorApp::selection(&self) -> &EditorSelection` | P0 |
| 149 | `EditorApp::selection_mut(&mut self) -> &mut EditorSelection` | P0 |
| 150 | `EditorApp::action_stack(&self) -> &EditorActionStack` | P0 |
| 151 | `EditorApp::action_stack_mut(&mut self) -> &mut EditorActionStack` | P0 |
| 152 | `EditorApp::scene(&self) -> &SceneTree` | P0 |
| 153 | `EditorApp::scene_mut(&mut self) -> &mut SceneTree` | P0 |
| 154 | `EditorApp::new_scene(&mut self)` | P0 |
| 155 | `EditorApp::save_scene(&self, path)` | P0 |
| 156 | `EditorApp::load_scene(&mut self, path)` | P0 |
| 157 | `EditorApp::settings(&self) -> &EditorSettings` | P0 |
| 158 | `EditorApp::settings_mut(&mut self) -> &mut EditorSettings` | P0 |
| 159 | `EditorApp::save_settings(&self, path)` | P0 |
| 160 | `EditorApp::load_settings(&mut self, path)` | P0 |
| 161 | `EditorApp::open_menu(&mut self, menu_name)` | P1 |
| 162 | `EditorApp::show_panel(&mut self, panel_id)` | P1 |
| 163 | `EditorApp::hide_panel(&mut self, panel_id)` | P1 |
| 164 | `EditorApp::toggle_panel(&mut self, panel_id)` | P1 |
| 165 | `EditorApp::reset_layout(&mut self)` | P1 |
| 166 | `EditorApp::register_plugin<P: EditorPlugin>(&mut self, plugin)` | P1 |

### 2.3 EditorState 状态管理

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 208 | `EditorState` 结构体：场景、选择、撤销栈、剪贴板、设置、插件 | P0 |
| 209 | `EditorState::tick(&mut self, dt)` | P0 |

### 2.4 EditorMode 模式切换

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 95 | `EditorMode`：EditMode / PlayMode / PausedMode | P0 |
| 96 | `EditorMode` 切换按钮在工具栏 | P0 |
| 97 | PlayMode 下禁止编辑，但可以修改值观察效果，退出时还原 | P0 |
| 127 | `EditorMode::Edit / Play / Paused` | P0 |

### 2.5 Docking 布局系统

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 4 | 编辑器 Docking 布局：顶部菜单、左侧场景树/资源、中间场景视图、右侧属性、底部控制台 | P0 |
| 14 | Dock 布局可拖拽重新布局（简化版） | P1 |
| 15 | Dock 布局可重置为默认 | P1 |

### 2.6 菜单系统

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 5 | 顶部菜单：File / Edit / View / Project / Build / Tools / Help | P0 |
| 6 | File：New Scene / Open / Save / Save As / Import Asset / Export / Exit | P0 |
| 7 | Edit：Undo / Redo / Cut / Copy / Paste / Delete / Duplicate / Select All / Preferences | P0 |
| 8 | View：Toggle 各面板显示、重置布局、全屏 | P1 |
| 9 | Project：Project Settings / Build Settings / Plugins | P1 |
| 10 | Build：Run / Build Windows / Build macOS / Build Linux / Build Android / Build iOS / Build Web / Rebuild | P1 |
| 11 | Tools：Options 调试选项 / 性能分析器 / 插件管理器 | P1 |
| 12 | Help：About / Documentation / Report Bug | P2 |
| 43 | 键盘快捷键：Ctrl+S 保存 / Ctrl+Z 撤销 / Ctrl+Y 重做 / Ctrl+N 新场景 / Ctrl+O 打开 / F2 重命名 / Del 删除 | P0 |

### 2.7 主题与 UI

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 110 | 编辑器 UI 主题：默认 Dark 主题 | P1 |
| 111 | 编辑器 UI 主题：Light 主题 | P1 |
| 112 | 编辑器 UI 可访问性：Tab 顺序 / 键盘导航 | P2 |
| 113 | 编辑器 UI 性能：每帧增量刷新，不重建全部 UI | P1 |
| 114 | 编辑器 UI 国际化：支持多语言（初版中文/英文） | P2 |
| 139 | 编辑器至少支持 10 个基础组件属性编辑 | P1 |
| 140 | 编辑器至少支持 2 种资源类型（PNG 与场景） | P1 |
| 141 | 编辑器 2D 场景下基本可用 | P1 |

### 2.8 帮助与关于

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 115 | 编辑器帮助菜单：文档链接 | P2 |
| 116 | 编辑器关于对话框：引擎版本、构建信息、依赖列表 | P2 |

## 3. API 签名

### 3.1 EditorApp

```rust
pub struct EditorApp {
    engine: Engine,
    state: EditorState,
    settings: EditorSettings,
    mode: EditorMode,
    // ...
}

impl EditorApp {
    pub fn new(window: Window, engine: Engine) -> Self;
    pub fn run(&mut self);
    pub fn update(&mut self, dt: f32);
    pub fn render(&mut self);
    pub fn handle_event(&mut self, event: Event);
    
    pub fn mode(&self) -> EditorMode;
    pub fn set_mode(&mut self, mode: EditorMode);
    
    pub fn selection(&self) -> &EditorSelection;
    pub fn selection_mut(&mut self) -> &mut EditorSelection;
    
    pub fn action_stack(&self) -> &EditorActionStack;
    pub fn action_stack_mut(&mut self) -> &mut EditorActionStack;
    
    pub fn scene(&self) -> &SceneTree;
    pub fn scene_mut(&mut self) -> &mut SceneTree;
    
    pub fn new_scene(&mut self);
    pub fn save_scene(&self, path: &Path);
    pub fn load_scene(&mut self, path: &Path);
    
    pub fn settings(&self) -> &EditorSettings;
    pub fn settings_mut(&mut self) -> &mut EditorSettings;
    pub fn save_settings(&self, path: &Path);
    pub fn load_settings(&mut self, path: &Path);
    
    pub fn open_menu(&mut self, menu_name: &str);
    pub fn show_panel(&mut self, panel_id: PanelId);
    pub fn hide_panel(&mut self, panel_id: PanelId);
    pub fn toggle_panel(&mut self, panel_id: PanelId);
    pub fn reset_layout(&mut self);
    
    pub fn register_plugin<P: EditorPlugin>(&mut self, plugin: P);
}
```

### 3.2 EditorState

```rust
pub struct EditorState {
    pub scene: SceneTree,
    pub selection: EditorSelection,
    pub action_stack: EditorActionStack,
    pub clipboard: EditorClipboard,
    pub settings: EditorSettings,
    pub plugins: EditorPluginRegistry,
}

impl EditorState {
    pub fn tick(&mut self, dt: f32);
}
```

### 3.3 EditorMode

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Edit,
    Play,
    Paused,
}
```

## 4. 输入/输出

| 操作 | 输入 | 输出 |
|-----|-----|-----|
| 创建 EditorApp | Window, Engine | EditorApp 实例 |
| 场景保存 | scene, path | Result<()> |
| 场景加载 | path | Result<SceneTree> |
| 模式切换 | EditorMode | 更新编辑器状态 |
| 面板控制 | PanelId | 显示/隐藏面板 |

## 5. 验收标准

- [ ] `EditorApp::new(window, engine)` 可正常构造
- [ ] `EditorApp::run()` 启动编辑器主循环
- [ ] Docking 布局正确显示（顶部菜单、左侧面板、中间场景视图、右侧属性、底部控制台）
- [ ] 顶部菜单 File/Edit/View/Project/Build/Tools/Help 可点击
- [ ] File 菜单：New Scene/Open/Save/Save As/Import Asset/Export/Exit 可用
- [ ] Edit 菜单：Undo/Redo/Cut/Copy/Paste/Delete/Duplicate/Select All/Preferences 可用
- [ ] 键盘快捷键正常工作
- [ ] EditorMode 可在 Edit/Play/Paused 之间切换
- [ ] PlayMode 下场景运行，退出后还原
- [ ] Dark/Light 主题可切换
- [ ] 布局可拖拽重排
- [ ] 布局可重置为默认

## 6. 依赖关系

- 依赖 `engine-core` crate（引擎基础）
- 依赖 `engine-ecs` crate（ECS 系统）
- 依赖 `engine-renderer` crate（渲染系统）
- 依赖 UI 框架（egui 或自定义实现）

## 7. 优先级

| 优先级 | 说明 |
|-------|------|
| P0 | 核心功能，必须完成 |
| P1 | 重要功能，应完成 |
| P2 | 增强功能，可后续完善 |
