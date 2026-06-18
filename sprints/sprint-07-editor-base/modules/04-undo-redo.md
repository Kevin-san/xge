# 模块四：Undo/Redo 命令系统需求

## 1. 模块概述

Undo/Redo 命令系统提供编辑器操作的撤销/重做能力，包括 EditorAction trait、具体 Action 实现（CreateNode/DeleteNode/SetProperty 等）、EditorActionStack 栈管理、EditorClipboard 剪贴板功能。

## 2. 功能需求

### 2.1 EditorAction Trait

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 109 | `EditorAction`：Action 系统（抽象操作：CreateEntity/Delete/SetProperty/...） | P0 |
| 237 | `EditorAction` trait：`apply(&mut self, editor)` / `undo(&mut self, editor)` / `mergeable(&self) -> bool` | P0 |
| 111 | `EditorAction` 可序列化，便于回放/录制 | P1 |

### 2.2 具体 Action 实现

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 238 | `CreateNodeAction`：新建节点 | P0 |
| 239 | `DeleteNodeAction`：删除节点 | P0 |
| 240 | `RenameNodeAction`：重命名 | P0 |
| 241 | `SetParentAction`：改变父子关系 | P0 |
| 242 | `SetPropertyAction`：设置单字段 | P0 |
| 243 | `AddComponentAction`：新增组件 | P0 |
| 244 | `RemoveComponentAction`：移除组件 | P0 |
| 245 | `MoveNodesAction`：位移 | P0 |
| 246 | `DuplicateAction`：复制（含 apply） | P0 |
| 247 | `PasteAction`：粘贴 | P0 |
| 248 | `BatchAction`：多 action 合并 | P1 |

### 2.3 EditorActionStack

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 110 | `EditorActionStack`：undo/redo 栈 | P0 |
| 249 | `EditorActionStack::push(&mut self, action)` | P0 |
| 250 | `EditorActionStack::undo(&mut self, editor)` | P0 |
| 251 | `EditorActionStack::redo(&mut self, editor)` | P0 |
| 252 | `EditorActionStack::clear(&mut self)` | P0 |
| 253 | `EditorActionStack::can_undo(&self) -> bool` | P0 |
| 254 | `EditorActionStack::can_redo(&self) -> bool` | P0 |
| 255 | `EditorActionStack::len(&self) -> usize` | P0 |
| 256 | `EditorActionStack::max_len(&self) -> usize`（默认 50） | P0 |
| 108 | `EditorUndo`：至少保存 50 步 undo | P0 |
| 109 | `EditorUndo`：保存/加载时清空 | P0 |

### 2.4 EditorClipboard

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 114 | `EditorClipboard`：复制/粘贴节点/组件/资源路径 | P0 |
| 306 | `EditorClipboard::copy_entities(&mut self, entities)` | P0 |
| 307 | `EditorClipboard::paste_entities(&self, editor)` | P0 |
| 308 | `EditorClipboard::copy_component(&mut self, component)` | P0 |
| 309 | `EditorClipboard::paste_component(&self, editor, entity)` | P0 |

### 2.5 EditorSelection

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 82 | `EditorSelection`：选择集（多选 entity / node） | P0 |
| 83 | `EditorSelection::clear()` / `select(entity)` / `toggle(entity)` / `contains(entity)` / `iter()` / `len()` | P0 |
| 84 | `EditorSelectionChanged` 事件 | P0 |
| 260 | `EditorSelection::new()` | P0 |
| 261 | `EditorSelection::clear(&mut self)` | P0 |
| 262 | `EditorSelection::select(&mut self, entity)` | P0 |
| 263 | `EditorSelection::toggle(&mut self, entity)` | P0 |
| 264 | `EditorSelection::add(&mut self, entity)` | P0 |
| 265 | `EditorSelection::remove(&mut self, entity)` | P0 |
| 266 | `EditorSelection::contains(&self, entity) -> bool` | P0 |
| 267 | `EditorSelection::is_empty(&self) -> bool` | P0 |
| 268 | `EditorSelection::len(&self) -> usize` | P0 |
| 269 | `EditorSelection::iter(&self) -> impl Iterator` | P0 |
| 270 | `EditorSelection::first(&self) -> Option<Entity>` | P0 |
| 271 | `EditorSelection::last(&self) -> Option<Entity>` | P0 |
| 272 | `EditorSelectionChanged` 事件 | P0 |
| 273 | `EditorSelectionChanged::old() / new()` | P0 |

## 3. API 签名

### 3.1 EditorAction Trait

```rust
pub trait EditorAction: Any + Send + Sync {
    fn apply(&mut self, editor: &mut EditorApp);
    fn undo(&mut self, editor: &mut EditorApp);
    fn mergeable(&self) -> bool;
    
    fn name(&self) -> &str { std::any::type_name::<Self>() }
}
```

### 3.2 具体 Action 实现

```rust
pub struct CreateNodeAction {
    entity: Entity,
    parent: Option<Entity>,
    name: String,
}

pub struct DeleteNodeAction {
    entity: Entity,
    parent: Option<Entity>,
    components: Vec<Box<dyn Component>>,
    children: Vec<Entity>,
}

pub struct RenameNodeAction {
    entity: Entity,
    old_name: String,
    new_name: String,
}

pub struct SetParentAction {
    entity: Entity,
    old_parent: Option<Entity>,
    new_parent: Option<Entity>,
}

pub struct SetPropertyAction {
    entity: Entity,
    component_id: ComponentId,
    field_name: String,
    old_value: Value,
    new_value: Value,
}

pub struct AddComponentAction {
    entity: Entity,
    component: Box<dyn Component>,
}

pub struct RemoveComponentAction {
    entity: Entity,
    component: Box<dyn Component>,
}

pub struct MoveNodesAction {
    entities: Vec<Entity>,
    old_positions: Vec<Vec2>,
    new_positions: Vec<Vec2>,
}

pub struct DuplicateAction {
    entities: Vec<Entity>,
    new_entities: Vec<Entity>,
}

pub struct PasteAction {
    entities: Vec<Entity>,
    parent: Option<Entity>,
}

pub struct BatchAction {
    actions: Vec<Box<dyn EditorAction>>,
}
```

### 3.3 EditorActionStack

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

### 3.4 EditorClipboard

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

### 3.5 EditorSelection

```rust
#[derive(Debug, Clone)]
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
```

## 4. 输入/输出

| 操作 | 输入 | 输出 |
|-----|-----|-----|
| CreateNodeAction | parent, name | 创建实体 |
| DeleteNodeAction | entity | 删除实体及子节点 |
| SetPropertyAction | entity, component, field, value | 更新组件属性 |
| MoveNodesAction | entities, delta | 更新 Transform |
| EditorActionStack::undo | - | 执行栈顶 undo |
| EditorActionStack::redo | - | 执行 redo 栈顶 |
| EditorClipboard::copy | entities | 复制到剪贴板 |
| EditorClipboard::paste | - | 从剪贴板创建实体 |

## 5. 验收标准

- [ ] EditorActionStack 至少保存 50 步 undo
- [ ] Ctrl+Z 触发 undo
- [ ] Ctrl+Y 触发 redo
- [ ] CreateNodeAction 可撤销
- [ ] DeleteNodeAction 可撤销
- [ ] SetPropertyAction 可撤销
- [ ] MoveNodesAction 可撤销
- [ ] 批量操作合并为一个 undo 步骤
- [ ] 场景保存/加载时清空 undo 栈
- [ ] EditorClipboard 复制粘贴节点正常
- [ ] EditorClipboard 复制粘贴组件正常
- [ ] EditorSelection::select/toggle/add/remove/contains 正确
- [ ] 选择变更触发 EditorSelectionChanged 事件

## 6. 依赖关系

- 依赖 EditorApp 核心模块
- 依赖引擎 ECS 系统
- 依赖场景树 SceneTree

## 7. 优先级

| 优先级 | 说明 |
|-------|------|
| P0 | 核心功能，必须完成 |
| P1 | 重要功能，应完成 |
| P2 | 增强功能，可后续完善 |
