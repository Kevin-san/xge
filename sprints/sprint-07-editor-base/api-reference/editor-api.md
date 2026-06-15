# 编辑器 API 清单

## 概述

本文档列出 `engine-editor` crate 的公开 API，按模块组织。

## 模块列表

- [EditorApp](#editorapp)
- [EditorState](#editorstate)
- [EditorMode](#editormode)
- [EditorSettings](#editorsettings)
- [EditorSelection](#editorselection)
- [EditorActionStack](#editoractionstack)
- [EditorClipboard](#editorclipboard)
- [EditorAction](#editoraction)
- [Panel](#panel)
- [SceneView](#sceneview)
- [GizmoSystem](#gizmosystem)
- [AssetPipeline](#assetpipeline)
- [AssetMeta](#assetmeta)
- [AssetDB](#assetdb)
- [SceneSaver/SceneLoader](#scenesaver-场景加载)
- [PrefabSaver/PrefabLoader](#prefabsaver-预制体加载)
- [EditorPlugin](#editorplugin)

---

## EditorApp

编辑器主结构体，持有引擎、场景、选择集、撤销栈等。

```rust
pub struct EditorApp { /* ... */ }

impl EditorApp {
    // 构造与运行
    pub fn new(window: Window, engine: Engine) -> Self;
    pub fn run(&mut self);
    pub fn update(&mut self, dt: f32);
    pub fn render(&mut self);
    pub fn handle_event(&mut self, event: Event);

    // 模式
    pub fn mode(&self) -> EditorMode;
    pub fn set_mode(&mut self, mode: EditorMode);

    // 选择集
    pub fn selection(&self) -> &EditorSelection;
    pub fn selection_mut(&mut self) -> &mut EditorSelection;

    // 撤销栈
    pub fn action_stack(&self) -> &EditorActionStack;
    pub fn action_stack_mut(&mut self) -> &mut EditorActionStack;

    // 场景
    pub fn scene(&self) -> &SceneTree;
    pub fn scene_mut(&mut self) -> &mut SceneTree;
    pub fn new_scene(&mut self);
    pub fn save_scene(&self, path: &Path) -> Result<()>;
    pub fn load_scene(&mut self, path: &Path) -> Result<()>;

    // 设置
    pub fn settings(&self) -> &EditorSettings;
    pub fn settings_mut(&mut self) -> &mut EditorSettings;
    pub fn save_settings(&self, path: &Path) -> Result<()>;
    pub fn load_settings(&mut self, path: &Path) -> Result<()>;

    // 面板控制
    pub fn open_menu(&mut self, menu_name: &str);
    pub fn show_panel(&mut self, panel_id: PanelId);
    pub fn hide_panel(&mut self, panel_id: PanelId);
    pub fn toggle_panel(&mut self, panel_id: PanelId);
    pub fn reset_layout(&mut self);

    // 插件
    pub fn register_plugin<P: EditorPlugin>(&mut self, plugin: P);
}
```

---

## EditorState

编辑器全局状态。

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

---

## EditorMode

编辑器模式枚举。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Edit,
    Play,
    Paused,
}
```

---

## EditorSettings

编辑器配置。

```rust
pub struct EditorSettings {
    pub theme: EditorTheme,
    pub key_bindings: KeyBindings,
    pub font_size: u32,
    pub external_tools: ExternalTools,
    pub default_create_path: PathBuf,
    pub auto_save: bool,
    pub auto_save_interval: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorTheme {
    Dark,
    Light,
}

pub struct KeyBindings {
    bindings: HashMap<EditorAction, Vec<Key>>,
}

pub struct ExternalTools {
    pub vscode_path: Option<PathBuf>,
    pub git_path: Option<PathBuf>,
    pub cargo_path: Option<PathBuf>,
}

impl EditorSettings {
    pub fn default() -> Self;
    pub fn theme(&self) -> EditorTheme;
    pub fn set_theme(&mut self, theme: EditorTheme);
    pub fn auto_save(&self) -> bool;
    pub fn auto_save_interval(&self) -> Duration;
    pub fn key_bindings(&self) -> &KeyBindings;
    pub fn set_key_binding(&mut self, action: EditorAction, keys: Vec<Key>);
}

impl KeyBindings {
    pub fn get(&self, action: EditorAction) -> &[Key];
}
```

---

## EditorSelection

选择集管理。

```rust
#[derive(Debug, Clone, Default)]
pub struct EditorSelection {
    entities: HashSet<Entity>,
}

pub struct EditorSelectionChanged {
    old_selection: HashSet<Entity>,
    new_selection: HashSet<Entity>,
}

impl EditorSelection {
    pub fn new() -> Self;
    pub fn clear(&mut self);
    pub fn select(&mut self, entity: Entity);
    pub fn toggle(&mut self, entity: Entity);
    pub fn add(&mut self, entity: Entity);
    pub fn remove(&mut self, entity: Entity);
    pub fn contains(&self, entity: Entity) -> bool;
    pub fn is_empty(&self) -> bool;
    pub fn len(&self) -> usize;
    pub fn iter(&self) -> impl Iterator<Item=&Entity>;
    pub fn first(&self) -> Option<Entity>;
    pub fn last(&self) -> Option<Entity>;
}

impl EditorSelectionChanged {
    pub fn old(&self) -> &HashSet<Entity>;
    pub fn new(&self) -> &HashSet<Entity>;
}
```

---

## EditorActionStack

撤销/重做栈。

```rust
pub struct EditorActionStack {
    undo_stack: Vec<Box<dyn EditorAction>>,
    redo_stack: Vec<Box<dyn EditorAction>>,
    max_len: usize,
}

impl EditorActionStack {
    pub fn new(max_len: usize) -> Self;
    pub fn push(&mut self, action: Box<dyn EditorAction>);
    pub fn undo(&mut self, editor: &mut EditorApp);
    pub fn redo(&mut self, editor: &mut EditorApp);
    pub fn clear(&mut self);
    pub fn can_undo(&self) -> bool;
    pub fn can_redo(&self) -> bool;
    pub fn len(&self) -> usize;
    pub fn max_len(&self) -> usize;
}
```

---

## EditorClipboard

剪贴板。

```rust
pub struct EditorClipboard {
    entities: Vec<Entity>,
    components: Vec<Box<dyn Component>>,
    asset_paths: Vec<PathBuf>,
}

impl EditorClipboard {
    pub fn copy_entities(&mut self, entities: &[Entity]);
    pub fn paste_entities(&self, editor: &mut EditorApp) -> Vec<Entity>;
    pub fn copy_component(&mut self, component: &dyn Component);
    pub fn paste_component(&self, editor: &mut EditorApp, entity: Entity);
    pub fn clear(&mut self);
}
```

---

## EditorAction

操作抽象 trait。

```rust
pub trait EditorAction: Any + Send + Sync {
    fn apply(&mut self, editor: &mut EditorApp);
    fn undo(&mut self, editor: &mut EditorApp);
    fn mergeable(&self) -> bool;
    fn name(&self) -> &str { std::any::type_name::<Self>() }
}

// 具体 Action 实现
pub struct CreateNodeAction { /* ... */ }
pub struct DeleteNodeAction { /* ... */ }
pub struct RenameNodeAction { /* ... */ }
pub struct SetParentAction { /* ... */ }
pub struct SetPropertyAction { /* ... */ }
pub struct AddComponentAction { /* ... */ }
pub struct RemoveComponentAction { /* ... */ }
pub struct MoveNodesAction { /* ... */ }
pub struct DuplicateAction { /* ... */ }
pub struct PasteAction { /* ... */ }
pub struct BatchAction { /* ... */ }
```

---

## Panel

面板 trait。

```rust
pub trait Panel: 'static {
    fn title(&self) -> &str;
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}

// 面板类型
pub struct HierarchyPanel { /* ... */ }
pub struct InspectorPanel { /* ... */ }
pub struct AssetPanel { /* ... */ }
pub struct ConsolePanel { /* ... */ }
pub struct AnimationPreviewPanel { /* ... */ }
pub struct DebugPanel { /* ... */ }
pub struct EditorSettingsPanel { /* ... */ }
pub struct SceneView { /* ... */ }
```

---

## SceneView

场景视图。

```rust
pub struct SceneView {
    camera: Camera,
    tool: EditorTool,
    grid_visible: bool,
    gizmos_visible: bool,
    mode_2d: bool,
    snap_enabled: bool,
    snap_value: f32,
}

pub enum EditorTool {
    Select,
    Move,
    Rotate,
    Scale,
}

pub struct SelectionRect {
    pub start: Vec2,
    pub end: Vec2,
}

impl SceneView {
    pub fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
    pub fn render(&self, engine: &Engine, renderer: &mut Renderer);
    pub fn handle_mouse(&mut self, editor: &mut EditorApp, event: &Event);
    pub fn draw_gizmos(&self, gizmos: &mut GizmoSystem);
    pub fn hit_test(&self, pos: Vec2) -> Option<Entity>;
    pub fn tool(&self) -> EditorTool;
    pub fn set_tool(&mut self, tool: EditorTool);
    pub fn snap_enabled(&self) -> bool;
    pub fn snap_value(&self) -> f32;
    pub fn camera_pan(&mut self, delta: Vec2);
    pub fn camera_zoom(&mut self, factor: f32);
    pub fn camera_rotate(&mut self, delta: Vec2);
    pub fn grid_visible(&self) -> bool;
    pub fn gizmos_visible(&self) -> bool;
    pub fn mode_2d(&self) -> bool;
    pub fn toggle_2d(&mut self);
}
```

---

## GizmoSystem

Gizmo 绘制系统。

```rust
pub struct GizmoSystem;

impl GizmoSystem {
    pub fn new() -> Self;
    pub fn draw_transform_gizmo(&mut self, transform: &Transform, selected: bool, tool: EditorTool);
    pub fn draw_gizmo_circle(&mut self, pos: Vec2, r: f32, color: Color);
    pub fn draw_gizmo_rect(&mut self, rect: Rect, color: Color);
    pub fn draw_gizmo_grid(&mut self, spacing: f32, size: f32, color: Color);
    pub fn draw_gizmo_arrow(&mut self, from: Vec2, to: Vec2, color: Color);
    pub fn draw_gizmo_text(&mut self, text: &str, pos: Vec2, color: Color);
}
```

---

## AssetPipeline

资源管线。

```rust
pub struct AssetPipeline {
    asset_dir: PathBuf,
    importers: Vec<Box<dyn AssetImporter>>,
    assets: Vec<AssetInfo>,
}

pub struct AssetInfo {
    pub path: PathBuf,
    pub meta: Option<AssetMeta>,
    pub size: u64,
    pub mtime: DateTime<Utc>,
}

pub trait AssetImporter: Send + Sync {
    fn can_import(&self, ext: &str) -> bool;
    fn import(&self, path: &Path) -> Result<Asset>;
    fn name(&self) -> &str;
}

impl AssetPipeline {
    pub fn new(asset_dir: PathBuf) -> Self;
    pub fn register_importer(&mut self, importer: Box<dyn AssetImporter>);
    pub fn scan(&mut self) -> Result<()>;
    pub fn import_all(&mut self) -> Result<()>;
    pub fn reimport_changed(&mut self) -> Result<()>;
    pub fn assets(&self) -> &[AssetInfo];
}
```

---

## AssetMeta

资源元数据。

```rust
pub struct AssetMeta {
    pub guid: Guid,
    pub path: PathBuf,
    pub importer_type: String,
    pub importer_settings: HashMap<String, Value>,
    pub imported_at: DateTime<Utc>,
}

impl AssetMeta {
    pub fn new(guid: Guid, path: PathBuf, importer_settings: HashMap<String, Value>) -> Self;
    pub fn save(&self, path: &Path) -> Result<()>;
    pub fn load(path: &Path) -> Result<Self>;
}
```

---

## AssetDB

资源数据库。

```rust
pub struct AssetDB {
    assets: HashMap<Guid, Asset>,
    path_to_guid: HashMap<PathBuf, Guid>,
}

impl AssetDB {
    pub fn instance() -> &'static mut Self;
    pub fn get(&self, guid: Guid) -> Option<&Asset>;
    pub fn get_by_path(&self, path: &Path) -> Option<Guid>;
    pub fn register(&mut self, guid: Guid, asset: Asset, path: PathBuf);
    pub fn load_asset(&mut self, path: &Path) -> Result<Guid>;
}
```

---

## SceneSaver 场景加载

```rust
pub struct SceneSaver;

impl SceneSaver {
    pub fn save_json(scene: &SceneTree, path: &Path) -> Result<()>;
    pub fn save_bin(scene: &SceneTree, path: &Path) -> Result<()>;
}

pub struct SceneLoader;

impl SceneLoader {
    pub fn load_json(path: &Path) -> Result<SceneTree>;
    pub fn load_bin(path: &Path) -> Result<SceneTree>;
}
```

---

## PrefabSaver 预制体加载

```rust
pub struct PrefabSaver;

impl PrefabSaver {
    pub fn save_json(prefab: &Prefab, path: &Path) -> Result<()>;
    pub fn save_bin(prefab: &Prefab, path: &Path) -> Result<()>;
}

pub struct PrefabLoader;

impl PrefabLoader {
    pub fn load_json(path: &Path) -> Result<Prefab>;
    pub fn load_bin(path: &Path) -> Result<Prefab>;
}
```

---

## EditorPlugin

插件扩展 trait。

```rust
pub trait EditorPlugin: Send + Sync {
    fn register(&mut self, editor: &mut EditorApp);
    fn update(&mut self, editor: &mut EditorApp, dt: f32);
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}

pub struct EditorPluginRegistry {
    plugins: Vec<Box<dyn EditorPlugin>>,
}

impl EditorPluginRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, plugin: Box<dyn EditorPlugin>);
    pub fn update_all(&mut self, editor: &mut EditorApp, dt: f32);
    pub fn ui_all(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}
```

---

## 类型别名

```rust
pub type PanelId = &'static str;
pub type Entity = u64;
pub type ComponentId = u64;
pub type Guid = String;
```
