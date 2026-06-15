# 模块二：面板系统需求

## 1. 模块概述

面板系统提供编辑器中各类面板的 UI 实现，包括 HierarchyPanel（层级面板）、InspectorPanel（检视面板）、AssetPanel（资源面板）、ConsolePanel（控制台面板）、AnimationPreviewPanel（动画预览面板）、DebugPanel（调试面板）、EditorSettingsPanel（设置面板）。

## 2. 功能需求

### 2.1 HierarchyPanel 层级面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 30 | `HierarchyPanel`：显示场景树（Node 或 Entity） | P0 |
| 31 | `HierarchyPanel`：可展开/折叠节点 | P0 |
| 32 | `HierarchyPanel`：点击选择节点；双击重命名 | P0 |
| 33 | `HierarchyPanel`：右键菜单（添加子节点/重命名/删除/复制/粘贴/保存为 Prefab） | P0 |
| 34 | `HierarchyPanel`：拖拽改变父子关系 | P0 |
| 35 | `HierarchyPanel`：支持搜索过滤（按名称/类型） | P1 |
| 62 | `HierarchyPanel` 显示场景树 | P0 |
| 63 | `HierarchyPanel` 节点显示图标（sprite/model/camera） | P1 |
| 64 | `HierarchyPanel` 节点可拖放 | P0 |
| 65 | `HierarchyPanel` 右键菜单 | P0 |
| 66 | `HierarchyPanel` 搜索框 | P1 |

### 2.2 InspectorPanel 检视面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 36 | `InspectorPanel`：显示当前选中节点的组件列表 | P0 |
| 37 | `InspectorPanel`：每个组件显示为可折叠 section | P0 |
| 38 | `InspectorPanel`：可编辑基础类型：i32/f32/f64/u32/bool/String/Vec2/Vec3/Vec4/Mat4/Color/Rect | P0 |
| 39 | `InspectorPanel`：可编辑资源引用（Texture/Material/Prefab/Scene/Font） | P0 |
| 40 | `InspectorPanel`：资源引用下拉选择/拖拽引用 | P0 |
| 41 | `InspectorPanel`：组件新增/删除菜单（按类型） | P0 |
| 42 | `InspectorPanel`：禁用/启用组件 | P0 |
| 43 | `InspectorPanel`：实时显示组件变化（ECS 变更检测） | P1 |
| 44 | `InspectorPanel`：批量编辑多选节点的公共属性 | P1 |
| 67 | `InspectorPanel` 显示 entity 的组件 | P0 |
| 68 | `InspectorPanel` 组件 section 折叠 | P0 |
| 69 | `InspectorPanel` 基础类型字段 UI | P0 |
| 70 | `InspectorPanel` 字符串字段：单行输入框 | P0 |
| 71 | `InspectorPanel` bool 字段：checkbox | P0 |
| 72 | `InspectorPanel` 资源引用：下拉 + 拖拽 | P0 |
| 73 | `InspectorPanel` 对象引用（entity ref）：选择 | P1 |
| 74 | `InspectorPanel` 组合字段：Vec2/3/4 显示 x/y/z/w | P0 |
| 75 | `InspectorPanel` 颜色：RGBA + 颜色选择器 | P0 |
| 76 | `InspectorPanel` 枚举：下拉选择 | P1 |
| 77 | `InspectorPanel` 数组：可展开列表 | P1 |
| 78 | `InspectorPanel` 结构体：递归 section | P1 |
| 79 | `InspectorPanel` Add Component 按钮 | P0 |
| 80 | `InspectorPanel` Remove Component 按钮 | P0 |
| 81 | `InspectorPanel` Enable/Disable Component 开关 | P0 |

### 2.3 AssetPanel 资源面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 45 | `AssetPanel`：左侧项目资源文件树（基于 `assets/` 目录） | P0 |
| 46 | `AssetPanel`：右侧资源网格视图（缩略图） | P0 |
| 47 | `AssetPanel`：支持拖拽资源到场景视图创建节点 | P0 |
| 48 | `AssetPanel`：支持双击打开（场景文件在编辑器中切换） | P0 |
| 49 | `AssetPanel`：右键菜单（新建文件夹/删除/重命名/导入/导出） | P1 |
| 50 | `AssetPanel`：支持常见扩展名过滤（png/jpg/ttf/otf/fbx/glb/json/bin） | P0 |
| 51 | `AssetPanel`：显示资源大小、修改时间 | P2 |
| 52 | `AssetPanel`：搜索框按名称过滤 | P1 |
| 53 | `AssetPanel`：支持创建资源元数据 `.meta` | P1 |
| 82 | `AssetPanel` 左侧目录树 | P0 |
| 83 | `AssetPanel` 右侧网格 | P0 |
| 84 | `AssetPanel` 文件过滤器 | P0 |
| 85 | `AssetPanel` 右键菜单 | P1 |
| 86 | `AssetPanel` 缩略图（初版占位） | P2 |
| 87 | `AssetPanel` 支持拖动到场景 | P0 |

### 2.4 ConsolePanel 控制台面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 54 | `ConsolePanel`：日志分级显示（Info/Warn/Error/Debug） | P0 |
| 55 | `ConsolePanel`：过滤级别切换 | P0 |
| 56 | `ConsolePanel`：过滤关键字 | P1 |
| 57 | `ConsolePanel`：清空按钮 | P0 |
| 58 | `ConsolePanel`：点击日志显示详情（堆栈、文件、行号） | P1 |
| 59 | `ConsolePanel`：可复制 | P1 |
| 60 | `ConsolePanel`：支持颜色标记（Info=绿/Warn=黄/Error=红） | P0 |
| 88 | `ConsolePanel` 日志显示行 | P0 |
| 89 | `ConsolePanel` 级别过滤 | P0 |
| 90 | `ConsolePanel` 搜索过滤 | P1 |
| 91 | `ConsolePanel` 行点击详情 | P1 |

### 2.5 AnimationPreviewPanel 动画预览面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 61 | `AnimationPreviewPanel`：时间轴显示（初版） | P1 |
| 62 | `AnimationPreviewPanel`：播放/暂停/停止/循环 | P1 |
| 63 | `AnimationPreviewPanel`：关键帧列表显示 | P1 |
| 64 | `AnimationPreviewPanel`：缩放时间轴 | P2 |
| 92 | `AnimationPreviewPanel` 时间轴 UI | P1 |
| 93 | `AnimationPreviewPanel` 播放控件 | P1 |
| 94 | `AnimationPreviewPanel` 缩放时间轴 | P2 |

### 2.6 DebugPanel 调试面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 65 | `DebugPanel`：FPS/FrameTime/DrawCalls/Vertices/Entities/Components/Memory 统计 | P1 |
| 66 | `DebugPanel`：性能图表（折线图） | P2 |
| 67 | `DebugPanel`：ECS archetype 统计（与 Sprint 05 的 `World::dump_stats` 联动） | P1 |
| 68 | `DebugPanel`：内存使用（系统资源/GPU 资源） | P1 |
| 95 | `DebugPanel` FPS / FrameTime 折线图（简易实现） | P1 |
| 96 | `DebugPanel` 性能统计 | P1 |
| 97 | `DebugPanel` ECS 统计 | P1 |
| 98 | `DebugPanel` GPU 内存（估算） | P1 |

### 2.7 EditorSettingsPanel 设置面板

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 251 | `EditorSettingsPanel` 主题切换 UI | P1 |
| 252 | `EditorSettingsPanel` 键位配置 UI | P1 |
| 253 | `EditorSettingsPanel` 自动保存设置 | P1 |

## 3. API 签名

### 3.1 Panel Trait

```rust
pub trait Panel {
    fn title(&self) -> &str;
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}
```

### 3.2 HierarchyPanel

```rust
pub struct HierarchyPanel {
    search_query: String,
    expanded_nodes: HashSet<Entity>,
}

impl Panel for HierarchyPanel {
    fn title(&self) -> &str { "Hierarchy" }
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}
```

### 3.3 InspectorPanel

```rust
pub struct InspectorPanel;

impl Panel for InspectorPanel {
    fn title(&self) -> &str { "Inspector" }
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}
```

### 3.4 AssetPanel

```rust
pub struct AssetPanel {
    selected_path: Option<PathBuf>,
    search_query: String,
}

impl Panel for AssetPanel {
    fn title(&self) -> &str { "Assets" }
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}
```

### 3.5 ConsolePanel

```rust
pub struct ConsolePanel {
    entries: Vec<LogEntry>,
    filter_level: LogLevel,
    filter_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct LogEntry {
    level: LogLevel,
    message: String,
    file: Option<PathBuf>,
    line: Option<u32>,
    timestamp: DateTime<Utc>,
}
```

### 3.6 DebugPanel

```rust
pub struct DebugPanel {
    fps_history: Vec<f32>,
    frame_time_history: Vec<f32>,
}

impl Panel for DebugPanel {
    fn title(&self) -> &str { "Debug" }
    fn ui(&mut self, editor: &mut EditorApp, ui: &mut Ui);
}
```

## 4. 输入/输出

| 面板 | 输入 | 输出 |
|-----|-----|-----|
| HierarchyPanel | 场景树数据、用户点击/拖拽事件 | 选中节点、更新父子关系 |
| InspectorPanel | 选中实体、ECS 组件数据 | 属性修改、组件增删 |
| AssetPanel | 资源目录、用户选择 | 资源引用、拖拽数据 |
| ConsolePanel | 日志消息、过滤条件 | 过滤后的日志列表 |
| AnimationPreviewPanel | 动画数据、播放控制 | 预览画面 |
| DebugPanel | 引擎统计数据 | 统计图表 |

## 5. 验收标准

- [ ] HierarchyPanel 显示场景树，可展开/折叠节点
- [ ] HierarchyPanel 点击选择节点，双击重命名
- [ ] HierarchyPanel 右键菜单可用（添加/删除/复制/粘贴）
- [ ] HierarchyPanel 拖拽改变父子关系
- [ ] InspectorPanel 显示选中节点的组件列表
- [ ] InspectorPanel 可编辑基础类型（i32/f32/String/Vec2/Vec3/Color 等）
- [ ] InspectorPanel 支持资源引用下拉选择
- [ ] InspectorPanel 可新增/删除/禁用组件
- [ ] AssetPanel 显示 assets/ 目录文件树
- [ ] AssetPanel 右侧网格视图显示资源缩略图
- [ ] AssetPanel 拖拽资源到场景视图可创建节点
- [ ] ConsolePanel 显示 Info/Warn/Error/Debug 日志
- [ ] ConsolePanel 可按级别过滤
- [ ] ConsolePanel 日志颜色标记正确
- [ ] DebugPanel 显示 FPS/帧时间统计
- [ ] EditorSettingsPanel 可切换 Dark/Light 主题
- [ ] EditorSettingsPanel 可配置自动保存

## 6. 依赖关系

- 依赖 EditorApp 核心模块
- 依赖引擎 ECS 系统（组件数据访问）
- 依赖引擎资源系统（资源加载）
- 依赖 UI 框架

## 7. 优先级

| 优先级 | 说明 |
|-------|------|
| P0 | 核心功能，必须完成 |
| P1 | 重要功能，应完成 |
| P2 | 增强功能，可后续完善 |
