# 句柄与事件总线需求

## 模块名称与概述

本模块提供游戏引擎的核心工具类型：类型安全句柄（Handle<T>）、对象池（Arena<T>）、事件总线（EventBus<T>）和资源管理器（ResourceManager<T>）。这些类型是构建高效 ECS 架构和事件驱动系统的基础。

## 需求编号

对应原文档需求编号：40, 42-74, 208-274

## 功能描述

### 1. Handle<T> 类型安全句柄

Handle<T> 是强类型句柄，使用索引 + 代际号（generation）机制避免悬挂引用。

**结构：**
```rust
pub struct Handle<T> {
    index: u32,
    generation: u32,
}
```

**核心功能：**
- `Handle::is_null(&self)` — 检查是否为 null 句柄
- 实现 `Copy + Eq + Hash` trait
- 索引用于 O(1) 查找
- 代际号用于检测对象是否已被释放

### 2. Arena<T> 对象池

Arena<T> 是以句柄为键的高效对象池。

**核心功能：**
- `Arena::new()` — 创建空 Arena
- `Arena::insert(value) -> Handle<T>` — 插入对象，返回句柄
- `Arena::remove(handle)` — 移除对象
- `Arena::get(handle) -> Option<&T>` — 获取不可变引用
- `Arena::get_mut(handle) -> Option<&mut T>` — 获取可变引用
- `Arena::len(&self) -> usize` — 返回对象数量
- `Arena::is_empty(&self) -> bool` — 是否为空
- `Arena::clear(&mut self)` — 清空所有对象
- `Arena::iter(&self)` — 遍历存活项（返回迭代器）
- 支持 O(1) 平均复杂度的增删改查

### 3. EventBus<T> 事件总线

EventBus<T> 是主题式事件总线，支持订阅/取消订阅和跨线程派发。

**核心功能：**
- `EventBus::new()` — 创建事件总线
- `EventBus::subscribe(&self, callback) -> SubscriptionHandle` — 订阅事件，返回订阅者句柄
- `EventBus::unsubscribe(&self, handle)` — 取消订阅
- `EventBus::send(&self, event)` — 同步派发事件
- `EventBus::drain(&mut self)` — 批量消费累积事件
- `EventBus::len(&self) -> usize` — 返回订阅者数量
- 线程安全：支持跨线程派发

### 4. ResourceManager<T> 资源管理器

ResourceManager<T> 是通用资源管理骨架。

**核心功能：**
- `ResourceManager::new()` — 创建资源管理器
- `ResourceManager::load(id, value)` — 加载资源
- `ResourceManager::get(id) -> Option<&T>` — 获取资源
- `ResourceManager::unload(id)` — 卸载资源
- `ResourceManager::contains(id) -> bool` — 检查资源是否存在

### 5. AssetId 资源标识符

AssetId 是资源唯一标识符，结合 Uuid 和路径哈希。

**核心功能：**
- `AssetId::new(uuid: Uuid)` — 从 UUID 创建
- `AssetId::from_path(path: &Path)` — 从路径创建
- `AssetId::null()` — 返回 null AssetId
- `AssetId::is_null(&self) -> bool` — 检查是否为 null

## API 签名

### Handle<T>
```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    index: u32,
    generation: u32,
}

impl<T> Handle<T> {
    pub const fn is_null(&self) -> bool;
    pub const fn index(&self) -> u32;
    pub const fn generation(&self) -> u32;
}
```

### Arena<T>
```rust
pub struct Arena<T> {
    items: Vec<T>,
    generations: Vec<u32>,
    free_indices: Vec<u32>,
}

impl<T> Arena<T> {
    pub fn new() -> Self;
    pub fn with_capacity(capacity: usize) -> Self;
    pub fn insert(&mut self, value: T) -> Handle<T>;
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T>;
    pub fn get(&self, handle: Handle<T>) -> Option<&T>;
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn clear(&mut self);
    pub fn iter(&self) -> ArenaIter<'_, T>;
    pub fn retain(&mut self, f: impl FnMut(Handle<T>, &T) -> bool);
}
```

### EventBus<T>
```rust
pub struct EventBus<T: Clone + Send + Sync> {
    subscribers: Vec<(SubscriptionHandle, Box<dyn Fn(T) + Send + Sync>)>,
}

impl<T: Clone + Send + Sync> EventBus<T> {
    pub fn new() -> Self;
    pub fn subscribe<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(T) + Send + Sync + 'static;
    pub fn unsubscribe(&self, handle: SubscriptionHandle);
    pub fn send(&self, event: T);
    pub fn drain(&mut self);
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

### ResourceManager<T>
```rust
pub struct ResourceManager<T> {
    resources: HashMap<AssetId, T>,
}

impl<T> ResourceManager<T> {
    pub fn new() -> Self;
    pub fn load(&mut self, id: AssetId, value: T) -> Option<T>;
    pub fn get(&self, id: &AssetId) -> Option<&T>;
    pub fn get_mut(&mut self, id: &AssetId) -> Option<&mut T>;
    pub fn unload(&mut self, id: &AssetId) -> Option<T>;
    pub fn contains(&self, id: &AssetId) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &T)>;
}
```

### AssetId
```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId {
    uuid: Uuid,
    path_hash: u64,
}

impl AssetId {
    pub fn new(uuid: Uuid) -> Self;
    pub fn from_path(path: &Path) -> Self;
    pub fn null() -> Self;
    pub fn is_null(&self) -> bool;
}
```

## 输入/输出

### Arena::insert(value)
- **输入：** T 类型的值
- **输出：** Handle<T> 句柄，可用于后续访问

### Arena::get(handle)
- **输入：** Handle<T> 句柄
- **输出：** Option<&T>（对象存在且未释放时返回 Some）

### EventBus::send(event)
- **输入：** T 类型的事件
- **输出：** 无（同步派发给所有订阅者）

### ResourceManager::load(id, value)
- **输入：** AssetId 和 T 类型的值
- **输出：** Option<T>（如果已存在则返回旧值）

## 验收标准

- [ ] Handle<T> 实现 Copy + Eq + Hash
- [ ] Handle<T>::is_null() 正确识别 null 句柄
- [ ] Arena<T> 插入/删除/查找操作 O(1) 平均复杂度
- [ ] Arena<T> 的 remove 操作不产生内存碎片（使用 free list）
- [ ] Arena<T>::iter() 只遍历存活对象
- [ ] EventBus<T> 订阅/取消订阅正常工作
- [ ] EventBus<T>::send() 线程安全
- [ ] EventBus<T>::drain() 批量消费累积事件
- [ ] ResourceManager<T> 支持资源加载/获取/卸载
- [ ] AssetId::from_path() 路径变化时产生不同的 AssetId
- [ ] Arena 单元测试 >= 10 条
- [ ] EventBus 单元测试 >= 10 条

## 依赖关系

**依赖模块：**
- `engine-utils` — Uuid、HashMap 别名
- `parking_lot` — 线程安全锁（用于 EventBus）
- `ahash` — 高性能哈希

**被依赖模块：**
- `engine-core` — 模块注册和生命周期
- `engine-ecs` — ECS World 管理
- `engine-asset` — 资源加载系统

## 优先级

**P0（必须）：**
- Handle<T> 完整实现
- Arena<T> 完整实现
- EventBus<T> 完整实现

**P1（重要）：**
- ResourceManager<T> 骨架
- AssetId 定义
