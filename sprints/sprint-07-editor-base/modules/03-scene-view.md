# 模块三：场景视图需求

## 1. 模块概述

SceneView 面板是编辑器的核心交互区域，提供 2D/3D 场景渲染、实体选择、变换工具（移动/旋转/缩放）、相机控制、网格/参考线/gizmos 显示等功能。

## 2. 功能需求

### 2.1 SceneView 基础功能

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 16 | `SceneView` 面板：场景绘制窗口 | P0 |
| 17 | `SceneView`：显示网格（2D / 3D） | P0 |
| 18 | `SceneView`：显示参考线 | P1 |
| 19 | `SceneView`：显示 gizmos（位置箭头/旋转/缩放手柄） | P0 |
| 27 | `SceneView`：2D / 3D 模式切换 | P0 |
| 47 | `SceneView` 面板：场景绘制窗口 | P0 |
| 48 | `SceneView` 显示网格 | P0 |
| 49 | `SceneView` 显示参考线 | P1 |
| 50 | `SceneView` 显示 gizmos | P0 |
| 56 | `SceneView` 正交 / 透视切换 | P1 |
| 57 | `SceneView` 2D / 3D 模式切换 | P0 |

### 2.2 选择功能

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 20 | `SceneView`：鼠标选择实体（点击命中测试） | P0 |
| 21 | `SceneView`：框选（Lasso/Rectangle） | P0 |
| 22 | `SceneView`：多选（按住 Shift/Click 或 Ctrl/Click） | P0 |
| 51 | `SceneView` 鼠标选择实体 | P0 |
| 52 | `SceneView` 框选 | P0 |
| 53 | `SceneView` 多选 | P0 |

### 2.3 变换工具

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 23 | `SceneView`：移动/旋转/缩放工具（W/E/R 键切换） | P0 |
| 88 | `EditorTools`：Select / Move / Rotate / Scale 工具切换 | P0 |
| 89 | `Snap`：像素吸附 / 网格吸附 / 旋转吸附（15° 等） | P0 |
| 118 | `EditorTool::Select / Move / Rotate / Scale` | P0 |
| 119 | `EditorTool::switch(tool)` 快捷键 W/E/R | P0 |
| 120 | `GizmoSystem::draw_transform_gizmo(transform, selected, tool)` 绘制手柄 | P0 |
| 121 | `GizmoSystem::draw_gizmo_circle(pos, r, color)` | P0 |
| 122 | `GizmoSystem::draw_gizmo_rect(rect, color)` | P0 |
| 123 | `GizmoSystem::draw_gizmo_grid(spacing, size, color)` | P0 |
| 124 | `GizmoSystem::draw_gizmo_arrow(from, to, color)` | P0 |
| 125 | `GizmoSystem::draw_gizmo_text(text, pos, color)` | P0 |

### 2.4 相机控制

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 24 | `SceneView`：场景平移中键拖拽 / 右键旋转 / 滚轮缩放 | P0 |
| 54 | `SceneView` 场景平移中键拖拽 / 右键旋转 / 滚轮缩放 | P0 |

### 2.5 2D 特性

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 25 | `SceneView`：像素对齐（2D 场景） | P1 |
| 55 | `SceneView` 像素对齐 | P1 |

### 2.6 运行控制

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 28 | `SceneView`：Play 按钮（运行场景）/ Stop / Step | P0 |
| 29 | `SceneView`：运行时不可编辑（只读视图） | P0 |
| 58 | `SceneView` Play 按钮 / Stop / Step | P0 |
| 59 | `SceneView` 运行时不可编辑 | P0 |

### 2.7 场景视图 API

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 211 | `SceneView::ui(&mut self, editor, ui)` | P0 |
| 212 | `SceneView::render(&self, engine, renderer)` | P0 |
| 213 | `SceneView::handle_mouse(&mut self, editor, event)` | P0 |
| 214 | `SceneView::draw_gizmos(&self, gizmos)` | P0 |
| 215 | `SceneView::hit_test(&self, pos) -> Option<Entity>` | P0 |
| 216 | `SceneView::tool(&self) -> EditorTool` | P0 |
| 217 | `SceneView::set_tool(&mut self, tool)` | P0 |
| 218 | `SceneView::snap_enabled(&self) -> bool` | P0 |
| 219 | `SceneView::snap_value(&self) -> f32` | P0 |
| 220 | `SceneView::camera_pan(&mut self, delta)` | P0 |
| 221 | `SceneView::camera_zoom(&mut self, factor)` | P0 |
| 222 | `SceneView::camera_rotate(&mut self, delta)` | P0 |
| 223 | `SceneView::grid_visible(&self) -> bool` | P0 |
| 224 | `SceneView::gizmos_visible(&self) -> bool` | P0 |
| 225 | `SceneView::mode_2d(&self) -> bool` | P0 |
| 226 | `SceneView::toggle_2d(&mut self)` | P0 |

### 2.8 选择区域

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 227 | `SelectionRect`：矩形框选区域 | P0 |
| 228 | `LassoSelect`：自由曲线选择（后续完善，留接口） | P2 |

### 2.9 GizmoSystem

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 117 | `EditorGizmos`：绘制辅助图形（箭头/网格/选择框/工具手柄） | P0 |
| 118 | `Gizmo2d`：线/圆/矩形/文本/箭头 | P0 |
| 119 | `Gizmo3d`：变换手柄、相机视锥体、光照 gizmo | P0 |

## 3. API 签名

### 3.1 SceneView

```rust
pub struct SceneView {
    camera: Camera,
    tool: EditorTool,
    grid_visible: bool,
    gizmos_visible: bool,
    mode_2d: bool,
    snap_enabled: bool,
    snap_value: f32,
    selection_rect: Option<SelectionRect>,
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

pub struct LassoSelect {
    pub points: Vec<Vec2>,
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

### 3.2 GizmoSystem

```rust
pub struct GizmoSystem;

impl GizmoSystem {
    pub fn draw_transform_gizmo(&mut self, transform: &Transform, selected: bool, tool: EditorTool);
    pub fn draw_gizmo_circle(&mut self, pos: Vec2, r: f32, color: Color);
    pub fn draw_gizmo_rect(&mut self, rect: Rect, color: Color);
    pub fn draw_gizmo_grid(&mut self, spacing: f32, size: f32, color: Color);
    pub fn draw_gizmo_arrow(&mut self, from: Vec2, to: Vec2, color: Color);
    pub fn draw_gizmo_text(&mut self, text: &str, pos: Vec2, color: Color);
}
```

### 3.3 Camera

```rust
pub enum CameraMode {
    Perspective,
    Orthographic,
}

pub struct Camera {
    mode: CameraMode,
    position: Vec3,
    rotation: Quat,
    zoom: f32,
}
```

## 4. 输入/输出

| 操作 | 输入 | 输出 |
|-----|-----|-----|
| 点击选择 | 屏幕坐标 | 选中的 Entity |
| 框选 | 起始/结束屏幕坐标 | 选中的 Entity 列表 |
| 移动/旋转/缩放 | 变换数据、工具类型 | 更新实体 Transform |
| 相机平移 | 鼠标中键拖拽 delta | 更新相机位置 |
| 相机缩放 | 滚轮 delta | 更新相机 zoom |
| 相机旋转 | 鼠标右键拖拽 delta | 更新相机旋转 |
| 命中测试 | 屏幕坐标 | Option<Entity> |

## 5. 验收标准

- [ ] SceneView 正确渲染场景内容
- [ ] 显示 2D/3D 网格
- [ ] 显示变换 gizmos（移动/旋转/缩放手柄）
- [ ] 点击场景中实体可选中
- [ ] 支持框选（Lasso/Rectangle）
- [ ] 支持多选（Shift/Ctrl + Click）
- [ ] W/E/R 快捷键切换 Move/Rotate/Scale 工具
- [ ] 中键拖拽平移相机
- [ ] 右键拖拽旋转相机
- [ ] 滚轮缩放
- [ ] 像素对齐功能正常（2D 模式）
- [ ] 正交/透视模式切换
- [ ] Play/Stop/Step 按钮可用
- [ ] 运行模式切换后视图只读
- [ ] Snap 吸附功能正常

## 6. 依赖关系

- 依赖 EditorApp 核心模块
- 依赖引擎渲染系统
- 依赖引擎 ECS 系统（实体查询）
- 依赖 EditorSelection 选择集模块
- 依赖 EditorAction 命令系统

## 7. 优先级

| 优先级 | 说明 |
|-------|------|
| P0 | 核心功能，必须完成 |
| P1 | 重要功能，应完成 |
| P2 | 增强功能，可后续完善 |
