# 场景树与节点（SceneTree / Node）模块需求

## 模块概述

场景树（SceneTree）是游戏引擎的核心数据结构，采用节点层级结构管理所有场景对象。Node 是所有节点的基类 trait，Node2D 是 2D 节点的实现。场景树负责节点的创建、销毁、更新（先序遍历）、绘制（后序遍历），以及节点间关系的维护。

---

## 需求清单

### 1. Node Trait 定义

| 编号 | 需求 | 描述 |
|------|------|------|
| 91 | `Node` trait：`name() / on_update(dt) / on_ready() / on_destroy()` | 节点基本接口 |
| 269 | `Node::on_ready(&mut self)` — 首次创建后调用 | 就绪回调 |
| 270 | `Node::on_update(&mut self, dt)` | 每帧更新 |
| 271 | `Node::on_draw(&self, renderer)` | 绘制回调 |
| 272 | `Node::on_destroy(&mut self)` | 销毁回调 |
| 273 | `Node::name(&self) -> &str` | 获取节点名称 |
| 274 | `Node::parent(&self) -> Option<NodeHandle>` | 获取父节点 |
| 275 | `Node::children(&self) -> &[NodeHandle]` | 获取子节点列表 |
| 276 | `Node::add_child(&mut self, child)` | 添加子节点 |
| 277 | `Node::remove_child(&mut self, child)` | 移除子节点 |
| 278 | `Node::set_parent(&mut self, parent)` | 设置父节点 |
| 279 | `Node::detach(&mut self)` | 从父节点分离 |
| 280 | `Node::visible(&self) -> bool` | 获取可见性 |
| 281 | `Node::set_visible(&mut self, bool)` | 设置可见性 |
| 282 | `Node::paused(&self) -> bool` | 获取暂停状态 |
| 283 | `Node::set_paused(&mut self, bool)` | 设置暂停状态 |

### 2. Node2D 结构体

| 编号 | 需求 | 描述 |
|------|------|------|
| 92 | `Node2D` 结构体：position / rotation / scale / z_index | 2D 节点属性 |
| 93 | `Node2D::new(name)` | 构造方法 |
| 284 | `Node2D::position(&self) -> Vec2` | 获取位置 |
| 285 | `Node2D::set_position(&mut self, v)` | 设置位置 |
| 286 | `Node2D::translate(&mut self, delta)` | 位置增量 |
| 287 | `Node2D::rotation(&self) -> f32` | 获取旋转 |
| 288 | `Node2D::set_rotation(&mut self, rad)` | 设置旋转 |
| 289 | `Node2D::rotate(&mut self, delta)` | 旋转增量 |
| 290 | `Node2D::scale(&self) -> Vec2` | 获取缩放 |
| 291 | `Node2D::set_scale(&mut self, v)` | 设置缩放 |
| 292 | `Node2D::z_index(&self) -> i32` | 获取 Z 索引 |
| 293 | `Node2D::set_z_index(&mut self, v)` | 设置 Z 索引 |
| 294 | `Node2D::world_position(&self) -> Vec2` | 世界坐标位置 |
| 295 | `Node2D::world_rotation(&self) -> f32` | 世界坐标旋转 |
| 296 | `Node2D::world_scale(&self) -> Vec2` | 世界坐标缩放 |
| 297 | `Node2D::local_matrix(&self) -> Mat3` | 本地变换矩阵 |
| 298 | `Node2D::world_matrix(&self) -> Mat3` | 世界变换矩阵 |
| 299 | `Node2D::local_transform(&self) -> Transform` | 本地变换 |

### 3. Node2D 父级/子级关系

| 编号 | 需求 | 描述 |
|------|------|------|
| 99 | `Node2D::children(&self) -> &[NodeHandle]` | 获取子节点 |
| 100 | `Node2D::parent(&self) -> Option<NodeHandle>` | 获取父节点 |
| 101 | `Node2D::add_child(&mut self, child)` | 添加子节点 |
| 102 | `Node2D::remove_child(&mut self, child)` | 移除子节点 |
| 103 | `Node2D::detach(&mut self)` | 从父节点分离 |

### 4. SceneTree 场景树

| 编号 | 需求 | 描述 |
|------|------|------|
| 104 | `SceneTree` 场景树：根节点 + 更新顺序 | 核心场景管理结构 |
| 105 | `SceneTree::new()` | 构造 |
| 106 | `SceneTree::root(&self) -> NodeHandle` | 获取根节点句柄 |
| 107 | `SceneTree::add_child(parent, child)` | 添加子节点 |
| 108 | `SceneTree::remove_child(parent, child)` | 移除子节点 |
| 109 | `SceneTree::destroy_node(handle)` | 销毁节点（含子树） |
| 110 | `SceneTree::get_node(&self, handle) -> &Node2D` | 获取节点引用 |
| 111 | `SceneTree::get_node_mut(&mut self, handle) -> &mut Node2D` | 获取可变引用 |
| 112 | `SceneTree::update(&mut self, dt)` — 先序遍历 on_update | 更新遍历 |
| 113 | `SceneTree::draw(&self, renderer)` — 后序遍历 draw | 绘制遍历 |
| 114 | `SceneTree::find_by_name(&self, name) -> Option<NodeHandle>` | 按名称查找 |
| 115 | `SceneTree::iter(&self) -> 迭代器` | 迭代所有节点 |

### 5. Sprite2D 精灵节点

| 编号 | 需求 | 描述 |
|------|------|------|
| 116 | `Sprite2D`：Node2D 子类，含精灵数据 | 2D 精灵组件 |
| 300 | `Sprite2D::sprite(&self) -> &Sprite` | 获取精灵引用 |
| 301 | `Sprite2D::set_sprite(&mut self, sprite)` | 设置精灵 |

### 6. Camera2DNode 相机节点

| 编号 | 需求 | 描述 |
|------|------|------|
| 117 | `Camera2DNode`：Node2D 子类，含相机 | 2D 相机组件 |
| 302 | `Camera2DNode::camera(&self) -> &Camera2D` | 获取相机引用 |
| 303 | `Camera2DNode::set_camera(&mut self, camera)` | 设置相机 |

### 7. Area2D 区域节点

| 编号 | 需求 | 描述 |
|------|------|------|
| 121 | `Area2D`：Node2D 子类，含 sensor collider | 2D 检测区域 |
| 304 | `Area2D::collider(&self) -> Collider2D` | 获取碰撞体引用 |
| 305 | `Area2D::on_entered(&self) -> &[BodyHandle]` | 进入区域的刚体列表 |

### 8. Body2DNode 物理刚体节点

| 编号 | 需求 | 描述 |
|------|------|------|
| 122 | `Body2DNode`：Node2D 子类，含 RigidBody2D + collider | 2D 物理刚体节点 |
| 306 | `Body2DNode::body(&self) -> BodyHandle` | 获取刚体句柄 |
| 307 | `Body2DNode::sync_from_world(&mut self, world)` | 从物理世界同步状态 |

### 9. 其他节点类型

| 编号 | 需求 | 描述 |
|------|------|------|
| 118 | `Audio2DNode`：Node2D 子类，含音源（下一阶段） | 音频节点（占位） |
| 119 | `AnimationPlayerNode`：Node2D 子类，含动画 | 动画播放器 |
| 120 | `TimerNode`：Node2D 子类，含倒计时 | 定时器节点 |

---

## API 签名

### Node Trait

```rust
pub trait Node {
    fn name(&self) -> &str;
    fn parent(&self) -> Option<NodeHandle>;
    fn children(&self) -> &[NodeHandle];
    
    fn on_ready(&mut self);
    fn on_update(&mut self, dt: f32);
    fn on_draw(&self, renderer: &mut dyn Renderer);
    fn on_destroy(&mut self);
    
    fn add_child(&mut self, child: NodeHandle);
    fn remove_child(&mut self, child: NodeHandle);
    fn set_parent(&mut self, parent: NodeHandle);
    fn detach(&mut self);
    
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn paused(&self) -> bool;
    fn set_paused(&mut self, paused: bool);
}
```

### Node2D

```rust
pub struct Node2D {
    name: String,
    parent: Option<NodeHandle>,
    children: Vec<NodeHandle>,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    z_index: i32,
    visible: bool,
    paused: bool,
}

impl Node2D {
    pub fn new(name: impl Into<String>) -> Self;
    
    pub fn position(&self) -> Vec2;
    pub fn set_position(&mut self, v: Vec2);
    pub fn translate(&mut self, delta: Vec2);
    
    pub fn rotation(&self) -> f32;
    pub fn set_rotation(&mut self, rad: f32);
    pub fn rotate(&mut self, delta: f32);
    
    pub fn scale(&self) -> Vec2;
    pub fn set_scale(&mut self, v: Vec2);
    
    pub fn z_index(&self) -> i32;
    pub fn set_z_index(&mut self, v: i32);
    
    pub fn world_position(&self) -> Vec2;
    pub fn world_rotation(&self) -> f32;
    pub fn world_scale(&self) -> Vec2;
    
    pub fn local_matrix(&self) -> Mat3;
    pub fn world_matrix(&self) -> Mat3;
    pub fn local_transform(&self) -> Transform;
}
```

### SceneTree

```rust
pub struct SceneTree {
    root: NodeHandle,
    nodes: Slab<Box<dyn Node>>,
    name_index: HashMap<String, Vec<NodeHandle>>,
}

impl SceneTree {
    pub fn new() -> Self;
    pub fn root(&self) -> NodeHandle;
    
    pub fn add_child(&mut self, parent: NodeHandle, child: NodeHandle);
    pub fn remove_child(&mut self, parent: NodeHandle, child: NodeHandle);
    pub fn destroy_node(&mut self, handle: NodeHandle);
    
    pub fn get_node(&self, handle: NodeHandle) -> &dyn Node;
    pub fn get_node_mut(&mut self, handle: NodeHandle) -> &mut dyn Node;
    
    pub fn update(&mut self, dt: f32);
    pub fn draw(&self, renderer: &mut dyn Renderer);
    
    pub fn find_by_name(&self, name: &str) -> Option<NodeHandle>;
    pub fn iter(&self) -> impl Iterator<Item = &dyn Node>;
}
```

### Sprite2D

```rust
pub struct Sprite2D {
    node2d: Node2D,
    sprite: Sprite,
}

impl Sprite2D {
    pub fn new(name: impl Into<String>, sprite: Sprite) -> Self;
    pub fn sprite(&self) -> &Sprite;
    pub fn set_sprite(&mut self, sprite: Sprite);
}
```

### Camera2DNode

```rust
pub struct Camera2DNode {
    node2d: Node2D,
    camera: Camera2D,
}

impl Camera2DNode {
    pub fn new(name: impl Into<String>, camera: Camera2D) -> Self;
    pub fn camera(&self) -> &Camera2D;
    pub fn set_camera(&mut self, camera: Camera2D);
}
```

### Area2D

```rust
pub struct Area2D {
    node2d: Node2D,
    collider: Collider2D,
    entered_bodies: Vec<BodyHandle>,
}

impl Area2D {
    pub fn new(name: impl Into<String>, collider: Collider2D) -> Self;
    pub fn collider(&self) -> &Collider2D;
    pub fn on_entered(&self) -> &[BodyHandle];
}
```

### Body2DNode

```rust
pub struct Body2DNode {
    node2d: Node2D,
    body_handle: BodyHandle,
    collider: Collider2D,
}

impl Body2DNode {
    pub fn new(name: impl Into<String>, body: RigidBody2D, collider: Collider2D) -> Self;
    pub fn body(&self) -> BodyHandle;
    pub fn sync_from_world(&mut self, world: &World2D);
}
```

---

## 输入/输出

### 输入
- 节点名称
- 变换参数（位置、旋转、缩放）
- 父子关系配置

### 输出
- 世界矩阵计算结果
- 层级遍历顺序
- 节点查找结果

---

## 验收标准

1. ✅ `SceneTree::new()` 创建包含根节点的场景树
2. ✅ `add_child` 正确建立父子关系
3. ✅ `remove_child` 正确解除父子关系
4. ✅ `destroy_node` 递归销毁子树
5. ✅ `update` 按先序遍历执行所有 `on_update`
6. ✅ `draw` 按后序遍历执行所有 `on_draw`
7. ✅ `find_by_name` 返回首个匹配节点
8. ✅ `world_position` 正确计算全局变换后的位置
9. ✅ `world_matrix` 正确计算矩阵乘法
10. ✅ 暂停节点不执行 `on_update`
11. ✅ 不可见节点不执行 `on_draw`
12. ✅ 单元测试：SceneTree 遍历顺序正确
13. ✅ 节点层级关系示例 `scene_tree` 正常运行

---

## 依赖关系

- 依赖 `math` crate（Vec2、Mat3、Transform）
- 依赖 `engine-renderer-2d` crate（Renderer）
- 被 `engine-scene` crate 封装
- 示例 `scene_tree` 依赖本模块
- `Body2DNode` 依赖 `engine-physics-2d`

---

## 优先级

| 优先级 | 含义 | 需求编号 |
|--------|------|----------|
| P0 | 核心功能 | 91, 92-103, 104-115, 269-307 |
| P1 | 重要功能 | 116-122, 280-299 |
| P2 | 增强功能 | 118-120 |
