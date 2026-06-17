# Module 02 — BVH Broad Phase

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/broadphase/bvh.rs`

## 1. 目标

**修复 `engine-physics-2d/src/world.rs#L230-L248` O(n²) Broad Phase，实现 BVH 加速。**

## 2. Dynamic AABB Tree（Erin Catto 风格）

```rust
pub struct DynamicAabbTree {
    nodes: Vec<Node>,
    root: NodeId,
    free_list: VecDeque<NodeId>,
    insertion_count: u32,
}

pub struct Node {
    pub aabb: AABB,
    pub parent: NodeId,
    pub children: [NodeId; 2],  // [0] = left, [1] = right
    pub height: i32,
    pub body: Option<BodyHandle>,
}

pub type NodeId = u32;
pub const NULL: NodeId = u32::MAX;
```

## 3. 核心操作

```rust
impl DynamicAabbTree {
    /// 插入 body（O(log n)）
    pub fn insert(&mut self, body: BodyHandle, aabb: AABB) -> NodeId;
    
    /// 删除（O(log n)）
    pub fn remove(&mut self, node: NodeId);
    
    /// 更新 body 位置（O(log n) 增量）
    pub fn update(&mut self, node: NodeId, new_aabb: AABB) -> bool;
    
    /// 找碰撞对
    pub fn find_pairs<F: FnMut(BodyHandle, BodyHandle)>(&self, mut callback: F);
}
```

## 4. SAH 重建

```rust
pub struct SahConfig {
    pub rebuild_threshold: u32,  // 默认 100
    pub leaf_size: usize,        // 1
}

impl DynamicAabbTree {
    pub fn rebuild_if_needed(&mut self) {
        if self.insertion_count > self.sah.rebuild_threshold {
            self.rebuild();
            self.insertion_count = 0;
        }
    }
    
    pub fn rebuild(&mut self) {
        // 1. 收集所有 leaf
        let leaves: Vec<_> = self.collect_leaves();
        // 2. 排序 / 空间分割
        let sorted = self.sort_leaves(leaves);
        // 3. 自底向上建树
        self.build_from_leaves(sorted);
    }
}
```

## 5. 查询

```rust
pub fn query<F: FnMut(NodeId)>(&self, aabb: &AABB, mut callback: F);
pub fn ray_cast<F: FnMut(&RayCastHit)>(&self, ray: &Ray3, max_t: f32, callback: F);
pub fn overlap_test(&self, shape: &dyn Shape, pos: Vec3, rot: Quat) -> Vec<BodyHandle>;
```

## 6. 性能对比

| 物体数 | O(n²) | Dynamic AABB Tree |
|--------|-------|-------------------|
| 100    | 10000 | ~500 |
| 1000   | 1M    | ~5000 |
| 10000  | 100M  | ~50000 |

## 7. 验收

- [ ] 10000 动态物体 Broad Phase < 1 ms
- [ ] BVH 增量更新 < 5 µs / 物体
- [ ] 与 O(n²) 对比 1000 物体 100x speedup
- [ ] 100 帧随机移动测试：无 AABB 错漏
- [ ] 重建阈值调优：每 100 帧
