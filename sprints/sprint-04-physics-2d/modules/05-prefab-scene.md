# Prefab 与场景加载（Prefab / SceneLoader）模块需求

## 模块概述

Prefab（预制体）是节点树的模板，可重复实例化。SceneLoader 负责场景文件的序列化与反序列化，支持 JSON 和二进制两种格式。SceneManager 管理多场景切换、异步加载、预加载，并提供场景过渡动画。

---

## 需求清单

### 1. Prefab 预制体

| 编号 | 需求 | 描述 |
|------|------|------|
| 126 | `Prefab`：节点树模板，可实例化 | 预制体核心结构 |
| 127 | `Prefab::from_scene(scene)` | 从场景创建 Prefab |
| 128 | `Prefab::instantiate(&self) -> NodeHandle` | 实例化 Prefab |
| 129 | `Prefab::save(&self, path)` | 保存 Prefab（自动选择格式） |
| 130 | `Prefab::load(path) -> Result<Self>` | 加载 Prefab（自动选择格式） |
| 154 | `Prefab::save_json(&self, path)` | JSON 格式保存 |
| 155 | `Prefab::load_json(path) -> Result<Self>` | JSON 格式加载 |
| 156 | `Prefab::save_bin(&self, path)` | 二进制格式保存 |
| 157 | `Prefab::load_bin(path) -> Result<Self>` | 二进制格式加载 |
| 310 | `Prefab::instantiate_in(&self, scene) -> NodeHandle` | 在指定场景中实例化 |

### 2. SceneLoader 场景加载器

| 编号 | 需求 | 描述 |
|------|------|------|
| 131 | `SceneLoader::from_json(json) -> SceneTree` | 从 JSON 反序列化 |
| 132 | `SceneLoader::to_json(scene) -> String` | 序列化为 JSON |
| 133 | `SceneLoader::from_binary(bytes) -> SceneTree` | 从二进制反序列化 |
| 134 | `SceneLoader::to_binary(scene) -> Vec<u8>` | 序列化为二进制 |
| 311 | `SceneLoader::save_json(scene, path)` | 保存场景为 JSON 文件 |
| 312 | `SceneLoader::load_json(path) -> Result<SceneTree>` | 从 JSON 文件加载 |
| 313 | `SceneLoader::save_bin(scene, path)` | 保存场景为二进制文件 |
| 314 | `SceneLoader::load_bin(path) -> Result<SceneTree>` | 从二进制文件加载 |

### 3. SceneFile 场景文件格式

| 编号 | 需求 | 描述 |
|------|------|------|
| 315 | `SceneFile::version` 字段 | 场景文件版本号 |
| 316 | `SceneFile::nodes` 数组 | 节点数组 |
| 317 | `SceneFile::resources` 引用表 | 资源引用表 |
| 318 | `SceneFile::signals` 信号连接表 | 信号连接信息 |

### 4. SceneManager 场景管理器

| 编号 | 需求 | 描述 |
|------|------|------|
| 135 | `SceneManager`：管理多场景切换、异步加载、预加载 | 场景管理器核心 |
| 136 | `SceneManager::load(path)` | 加载场景 |
| 137 | `SceneManager::switch_to(name)` | 切换到指定场景（替换） |
| 138 | `SceneManager::push(name)` — 保留旧场景 | 压栈场景 |
| 139 | `SceneManager::pop()` — 恢复旧场景 | 弹出场景 |
| 140 | `SceneManager::current(&self) -> Option<&SceneTree>` | 获取当前场景 |

### 5. Transition 场景过渡

| 编号 | 需求 | 描述 |
|------|------|------|
| 141 | `Transition`：场景切换动画（淡入淡出 / 滑动 / 十字擦除） | 过渡效果类型 |

---

## API 签名

### Prefab

```rust
pub struct Prefab {
    root: NodeHandle,
    nodes: Slab<Box<dyn Node>>,
}

impl Prefab {
    pub fn from_scene(scene: &SceneTree) -> Self;
    pub fn instantiate(&self) -> NodeHandle;
    pub fn instantiate_in(&self, scene: &mut SceneTree) -> NodeHandle;
    
    pub fn save(&self, path: &Path) -> Result<()>;
    pub fn load(path: &Path) -> Result<Self>;
    
    pub fn save_json(&self, path: &Path) -> Result<()>;
    pub fn load_json(path: &Path) -> Result<Self>;
    
    pub fn save_bin(&self, path: &Path) -> Result<()>;
    pub fn load_bin(path: &Path) -> Result<Self>;
}
```

### SceneLoader

```rust
pub struct SceneFile {
    pub version: u32,
    pub nodes: Vec<SceneNodeData>,
    pub resources: HashMap<String, ResourceRef>,
    pub signals: Vec<SignalConnection>,
}

pub struct SceneNodeData {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub type_name: String,
    pub properties: HashMap<String, serde_json::Value>,
}

pub struct ResourceRef {
    pub path: String,
    pub type_name: String,
}

pub struct SignalConnection {
    pub node_id: u64,
    pub signal_name: String,
    pub target_node_id: u64,
    pub method_name: String,
}

pub struct SceneLoader;

impl SceneLoader {
    pub fn from_json(json: &str) -> SceneTree;
    pub fn to_json(scene: &SceneTree) -> String;
    
    pub fn from_binary(bytes: &[u8]) -> SceneTree;
    pub fn to_binary(scene: &SceneTree) -> Vec<u8>;
    
    pub fn save_json(scene: &SceneTree, path: &Path) -> Result<()>;
    pub fn load_json(path: &Path) -> Result<SceneTree>;
    
    pub fn save_bin(scene: &SceneTree, path: &Path) -> Result<()>;
    pub fn load_bin(path: &Path) -> Result<SceneTree>;
}
```

### SceneManager

```rust
pub enum Transition {
    None,
    Fade { duration: f32, color: Color },
    Slide { duration: f32, direction: Direction },
    CircleWipe { duration: f32 },
}

pub enum Direction {
    Left, Right, Up, Down,
}

pub struct SceneManager {
    scenes: HashMap<String, SceneTree>,
    stack: Vec<String>,
    current: Option<String>,
    transition: Option<Transition>,
}

impl SceneManager {
    pub fn new() -> Self;
    
    pub fn load(&mut self, path: &Path) -> Result<()>;
    pub fn switch_to(&mut self, name: &str);
    pub fn push(&mut self, name: &str);
    pub fn pop(&mut self);
    
    pub fn current(&self) -> Option<&SceneTree>;
    pub fn current_mut(&mut self) -> Option<&mut SceneTree>;
    
    pub fn set_transition(&mut self, transition: Transition);
}
```

---

## 输入/输出

### 输入
- 场景/预制体文件路径
- JSON 或二进制格式数据
- 场景名称标识

### 输出
- 反序列化后的 SceneTree
- 实例化后的节点句柄
- 序列化后的文件/字节流

---

## 验收标准

1. ✅ `Prefab::instantiate()` 返回独立节点树
2. ✅ 实例化后的节点修改不影响原始 Prefab
3. ✅ `Prefab::save_json` / `load_json` 往返数据一致
4. ✅ `Prefab::save_bin` / `load_bin` 往返数据一致
5. ✅ `SceneLoader::to_json` / `from_json` 往返数据一致
6. ✅ `SceneLoader::to_binary` / `from_binary` 往返数据一致
7. ✅ `SceneManager::push` 保留当前场景
8. ✅ `SceneManager::pop` 正确恢复上一场景
9. ✅ `SceneManager::switch_to` 替换当前场景
10. ✅ `SceneFile::version` 支持版本升级
11. ✅ 单元测试：Prefab 实例化不修改模板
12. ✅ 单元测试：SceneLoader JSON 往返
13. ✅ 示例 `prefab_basic` 实例化多个 Prefab 正常
14. ✅ 示例 `scene_switch` 按键切换场景正常

---

## 依赖关系

- 依赖 `engine-scene` crate（SceneTree、Node）
- 依赖 `serde` / `serde_json`（JSON 序列化）
- 示例 `prefab_basic`、`scene_switch` 依赖本模块

---

## 优先级

| 优先级 | 含义 | 需求编号 |
|--------|------|----------|
| P0 | 核心功能 | 126-130, 131-134, 136-140, 308-318 |
| P1 | 重要功能 | 141, 310-314 |
| P2 | 增强功能 | 315-318 |
