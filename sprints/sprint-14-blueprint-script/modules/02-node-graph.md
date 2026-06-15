# 模块二：节点图编辑器 UI（Node Graph Editor UI）

## 1. 模块概述

Node Graph Editor UI 是蓝图系统的可视化编辑界面，提供节点拖拽、连线编辑、缩放平移、搜索插入等交互功能。本模块是 `BlueprintEditorView` 的核心实现，与底层数据结构 `BlueprintGraph` 解耦，通过 `Arc<RwLock<BlueprintGraph>>` 连接。

核心组件：
- `BlueprintEditorView`：编辑器主视图
- `NodeDragDropController`：节点拖拽控制器
- `WireEditor`：连线编辑器
- `ZoomPanController`：缩放平移控制器
- `NodeSearch`：节点搜索
- `AutoLayout`：自动布局算法
- `CommentBox`：注释框
- `RerouteNode`：连线中继节点

---

## 2. 需求清单

### 2.1 BlueprintEditorView（需求 63, 251-252）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-001 | `BlueprintEditorView::new(graph_arc) -> Self` 创建编辑器视图 | P0 |
| REQ-NG-002 | `BlueprintEditorView::show(&mut self, ui)` 渲染节点、连线、工具栏 | P0 |

### 2.2 NodeWidget 与 PinWidget（需求 253-254）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-003 | `NodeWidget` 渲染单个节点（标题栏 + 输入引脚列 + 输出引脚列 + 主体） | P0 |
| REQ-NG-004 | `PinWidget` 圆形锚点 + 名称标签 + 颜色 | P0 |

### 2.3 NodeDragDropController（需求 64, 114, 255-259）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-005 | `NodeDragDropController::begin_drag(node_id, mouse)` 开始拖拽 | P0 |
| REQ-NG-006 | `NodeDragDropController::drag_to(&mut self, mouse)` 拖拽移动 | P0 |
| REQ-NG-007 | `NodeDragDropController::end_drag(&mut self)` 结束拖拽 | P0 |
| REQ-NG-008 | `NodeDragDropController::snap_to_grid(&self, x, y) -> (f32, f32)` 网格吸附（20 像素） | P1 |

### 2.4 WireEditor（需求 65, 115, 259-262）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-009 | `WireEditor::begin_wire(from_pin, mouse)` 开始连线 | P0 |
| REQ-NG-010 | `WireEditor::preview(&self, mouse)` 绘制贝塞尔预览线 | P0 |
| REQ-NG-011 | `WireEditor::end_wire(&mut self, target_pin)` 类型兼容则创建连线 | P0 |
| REQ-NG-012 | `WireEditor::cancel(&mut self)` 取消连线 | P0 |

### 2.5 ZoomPanController（需求 66, 263-267）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-013 | `ZoomPanController::zoom(&self) -> f32` 获取当前缩放比例 | P0 |
| REQ-NG-014 | `ZoomPanController::set_zoom(&mut self, z, anchor)` 以鼠标为中心缩放 | P0 |
| REQ-NG-015 | `ZoomPanController::pan(&mut self, delta)` 平移画布 | P0 |
| REQ-NG-016 | `ZoomPanController::screen_to_world(&self, p) -> (f32, f32)` 屏幕坐标转世界坐标 | P0 |
| REQ-NG-017 | `ZoomPanController::world_to_screen(&self, p) -> (f32, f32)` 世界坐标转屏幕坐标 | P0 |
| REQ-NG-018 | 缩放比例范围 `[0.2, 4.0]` | P0 |

### 2.6 NodeSearch（需求 67, 268-271）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-019 | `NodeSearch::open(&mut self)` 弹出搜索菜单 | P1 |
| REQ-NG-020 | `NodeSearch::filter(&mut self, query) -> Vec<NodeKind>` 模糊匹配节点 | P1 |
| REQ-NG-021 | `NodeSearch::insert_at_cursor(&mut self, kind, cursor)` 插入节点并自动连输入 | P1 |

### 2.7 AutoWire 与 AutoLayout（需求 68-69, 272-273）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-022 | `AutoWire::wire_compatible(graph, from_node, to_node)` 按名称+类型自动连线 | P2 |
| REQ-NG-023 | `AutoLayout::apply(graph, algo)` 对图进行自动布局 | P1 |
| REQ-NG-024 | `AutoLayout::force_directed(graph)` 力导向布局（第一阶段仅层次化） | P2 |

### 2.8 CommentBox（需求 70, 274-277）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-025 | `CommentBox::new(rect, text, color) -> Self` 创建注释框 | P2 |
| REQ-NG-026 | `CommentBox::contains(&self, node) -> bool` 检查节点是否在框内 | P2 |
| REQ-NG-027 | `CommentBox::render(&self, ui)` 渲染注释框 | P2 |

### 2.9 RerouteNode（需求 71, 277-278）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-028 | `RerouteNode` 作为连线路由点的特殊节点 | P2 |
| REQ-NG-029 | `RerouteNode::split_wire(&mut self, wire_id, point)` 把一根 wire 拆为两根 + reroute | P2 |

### 2.10 编辑器工具栏与交互（需求 279-282）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-NG-030 | 编辑器工具栏：保存 / 加载 / 自动布局 / 缩放 100% / 编译 IR | P1 |
| REQ-NG-031 | 编辑器右键菜单：删除节点 / 断开连线 / 插入 reroute / 添加注释框 | P1 |
| REQ-NG-032 | 编辑器快捷键：Ctrl+S 保存、Ctrl+Z 撤销、Ctrl+Y 重做、Delete 删除、Ctrl+F 搜索 | P1 |
| REQ-NG-033 | `UndoStack` 记录节点/连线的增删改操作（上限 200） | P1 |

---

## 3. API 签名

### 3.1 BlueprintEditorView

```rust
pub struct BlueprintEditorView {
    graph: Arc<RwLock<BlueprintGraph>>,
    drag_controller: NodeDragDropController,
    wire_editor: WireEditor,
    zoom_pan: ZoomPanController,
    node_search: NodeSearch,
    auto_layout: AutoLayout,
    undo_stack: UndoStack,
}

impl BlueprintEditorView {
    pub fn new(graph_arc: Arc<RwLock<BlueprintGraph>>) -> Self;
    pub fn show(&mut self, ui: &mut Ui);
}
```

### 3.2 NodeDragDropController

```rust
pub struct NodeDragDropController {
    dragging_node: Option<NodeId>,
    drag_start: (f32, f32),
    node_start: (f32, f32),
}

impl NodeDragDropController {
    pub fn begin_drag(&mut self, node_id: NodeId, mouse: (f32, f32));
    pub fn drag_to(&mut self, mouse: (f32, f32));
    pub fn end_drag(&mut self);
    pub fn snap_to_grid(&self, x: f32, y: f32) -> (f32, f32);
}
```

### 3.3 WireEditor

```rust
pub struct WireEditor {
    state: WireEditorState,
    preview_start: PinRef,
    preview_end: (f32, f32),
}

enum WireEditorState { Idle, Wiring(PinRef) }

impl WireEditor {
    pub fn begin_wire(&mut self, from_pin: PinRef, mouse: (f32, f32));
    pub fn preview(&self, mouse: (f32, f32));
    pub fn end_wire(&mut self, target_pin: PinRef) -> Option<WireId>;
    pub fn cancel(&mut self);
}
```

### 3.4 ZoomPanController

```rust
pub struct ZoomPanController {
    zoom: f32,
    pan: (f32, f32),
}

impl ZoomPanController {
    pub fn zoom(&self) -> f32;
    pub fn set_zoom(&mut self, z: f32, anchor: (f32, f32));
    pub fn pan(&mut self, delta: (f32, f32));
    pub fn screen_to_world(&self, p: (f32, f32)) -> (f32, f32);
    pub fn world_to_screen(&self, p: (f32, f32)) -> (f32, f32);
}
```

### 3.5 NodeSearch

```rust
pub struct NodeSearch {
    is_open: bool,
    query: String,
    results: Vec<NodeKind>,
}

impl NodeSearch {
    pub fn open(&mut self);
    pub fn filter(&mut self, query: &str) -> Vec<NodeKind>;
    pub fn insert_at_cursor(&mut self, kind: NodeKind, cursor: (f32, f32));
}
```

### 3.6 AutoLayout

```rust
pub enum LayoutAlgorithm { Hierarchical, ForceDirected }

pub struct AutoLayout;

impl AutoLayout {
    pub fn apply(graph: &mut BlueprintGraph, algo: LayoutAlgorithm);
    pub fn force_directed(graph: &mut BlueprintGraph);
}
```

### 3.7 CommentBox

```rust
pub struct CommentBox {
    rect: Rect,
    text: String,
    color: Color,
}

impl CommentBox {
    pub fn new(rect: Rect, text: &str, color: Color) -> Self;
    pub fn contains(&self, node: &BlueprintNode) -> bool;
    pub fn render(&self, ui: &mut Ui);
}
```

### 3.8 RerouteNode

```rust
pub struct RerouteNode {
    node_id: NodeId,
    position: (f32, f32),
}

impl RerouteNode {
    pub fn split_wire(graph: &mut BlueprintGraph, wire_id: WireId, point: (f32, f32)) -> (WireId, WireId);
}
```

---

## 4. 输入与输出

| 组件 | 输入 | 输出 |
|------|------|------|
| `NodeDragDropController::drag_to` | `(f32, f32)` 鼠标位置 | 更新节点 position |
| `WireEditor::end_wire` | `PinRef` 目标引脚 | `Option<WireId>` |
| `ZoomPanController::screen_to_world` | `(f32, f32)` 屏幕坐标 | `(f32, f32)` 世界坐标 |
| `NodeSearch::filter` | `&str` 查询字符串 | `Vec<NodeKind>` |
| `AutoLayout::apply` | `LayoutAlgorithm` | 修改 graph 节点位置 |

---

## 5. 验收标准

- [ ] 节点可拖拽移动，松开鼠标后位置更新到 `BlueprintGraph`
- [ ] 节点拖拽时进行 20 像素网格吸附
- [ ] 连线使用贝塞尔曲线预览
- [ ] 类型不兼容的引脚之间无法连线
- [ ] 缩放在 `[0.2, 4.0]` 范围内
- [ ] 以鼠标为中心进行缩放
- [ ] 右键菜单功能正常
- [ ] 快捷键 Ctrl+S/Z/Y/Delete/Ctrl+F 响应正常
- [ ] UndoStack 正确记录并回放操作（上限 200 条）
- [ ] `AutoLayout` 按拓扑序左到右排布节点
- [ ] `RerouteNode::split_wire` 正确拆分连线

---

## 6. 依赖关系

- 依赖 `01-blueprint-core` 模块的 `BlueprintGraph`、`BlueprintNode`、`Pin`、`BlueprintWire`
- 被 `BlueprintEditorView` 聚合使用
- 通过 `Arc<RwLock<BlueprintGraph>>` 与数据层解耦

---

## 7. 优先级汇总

| 优先级 | 需求数量 | 说明 |
|-------|---------|------|
| P0 | 17 | 核心交互功能，必须完成 |
| P1 | 13 | 重要功能，应完成 |
| P2 | 7 | 增强功能，可延后 |
