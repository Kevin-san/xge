# Module 01 — Archetype 内存布局

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)
> 文件位置: `engine-ecs/src/storage/archetype.rs`

---

## 1. 目标

**核心思想：** 同一组件组合的实体共享一个 Archetype，其组件在内存中 SoA 布局，迭代时缓存友好。

**对比：**
- ❌ 当前 `HashMap<TypeId, DenseStorage<C>>`：跨类型访问需要多次 indirect
- ✅ Archetype：单次 indirect，所有组件同 row

## 2. 数据结构

```rust
// engine-ecs/src/storage/archetype.rs

/// Archetype 标识（组件组合的指纹）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArchetypeId(pub u32);

/// Archetype 元数据
pub struct Archetype {
    pub id: ArchetypeId,
    /// 此 Archetype 包含的组件类型 ID
    pub component_types: Vec<TypeId>,
    /// 列存储（每种组件一个）
    pub columns: HashMap<TypeId, Column>,
    /// 实体表（行 = 实体，列 = 组件）
    pub entities: Vec<Entity>,
}

/// 单列存储（SoA 中的一列）
pub struct Column {
    /// 真实组件数据（Box<dyn Any> 存储）
    data: Box<dyn AnyColumn>,
}

pub trait AnyColumn: Any + Send + Sync {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool { self.len() == 0 }
    /// 移动删除（swap_remove），O(1)
    fn swap_remove(&mut self, index: usize);
    /// 类型化 push
    fn push_untyped(&mut self, value: Box<dyn Any>);
    /// 取值
    fn get_untyped(&self, index: usize) -> &dyn Any;
    fn get_mut_untyped(&mut self, index: usize) -> &mut dyn Any;
}

pub struct TypedColumn<T: Component> {
    data: Vec<T>,
}

impl<T: Component + 'static> AnyColumn for TypedColumn<T> {
    fn len(&self) -> usize { self.data.len() }
    fn swap_remove(&mut self, index: usize) { self.data.swap_remove(index); }
    fn push_untyped(&mut self, value: Box<dyn Any>) {
        self.data.push(*value.downcast::<T>().expect("type mismatch"));
    }
    fn get_untyped(&self, index: usize) -> &dyn Any { &self.data[index] }
    fn get_mut_untyped(&mut self, index: usize) -> &mut dyn Any { &mut self.data[index] }
}
```

## 3. Archetype 表

```rust
pub struct ArchetypeTable {
    /// ID → Archetype
    by_id: HashMap<ArchetypeId, Archetype>,
    /// 组件组合 → Archetype ID
    by_components: HashMap<ArchetypeId, ArchetypeId>,  // 指纹
    next_id: u32,
}

impl ArchetypeTable {
    pub fn get_or_create(&mut self, components: &[TypeId]) -> ArchetypeId {
        let fingerprint = compute_fingerprint(components);
        if let Some(&id) = self.by_components.get(&fingerprint) {
            return id;
        }
        let id = ArchetypeId(self.next_id);
        self.next_id += 1;
        let mut columns = HashMap::new();
        for &c in components {
            columns.insert(c, Column::new_untyped::<()>());  // 占位
        }
        let arch = Archetype {
            id,
            component_types: components.to_vec(),
            columns,
            entities: Vec::new(),
        };
        self.by_id.insert(id, arch);
        self.by_components.insert(fingerprint, id);
        id
    }
}
```

## 4. Archetype 切换

实体添加/移除组件时，需要从旧 Archetype 移到新 Archetype：

```rust
pub fn move_entity(
    world: &mut World,
    entity: Entity,
    from: ArchetypeId,
    to: ArchetypeId,
) {
    // 1. 复制共同组件
    // 2. 从 from 移除实体
    // 3. 添加到 to（含新组件的默认值）
    // 4. 更新 Entity → Archetype 映射
}
```

## 5. Entity → Archetype 映射

```rust
// 在 World 中
pub struct EntityLocation {
    pub archetype: ArchetypeId,
    pub row: usize,  // 在 Archetype 表中的行号
}

pub struct World {
    // ...
    pub entities: Slab<EntityData>,
    pub locations: HashMap<Entity, EntityLocation>,
    // ...
}
```

## 6. 验收

- [ ] 10000 实体 5 组件 spawn < 50 ms
- [ ] Archetype 切换（添加/移除组件） < 5 µs
- [ ] SoA 列内连续：每列 `&[T]` 可直接传给 SIMD
- [ ] 测试：100% 路径覆盖
- [ ] 内存占用：与 HashMap 方案对比，差异 < 5%
