# Module 04 — SIMD 视锥剔除

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)
> 文件位置: `engine-math/src/frustum.rs`

---

## 1. 目标

**实现 SIMD 加速的视锥剔除：**
- `Frustum` 从视图投影矩阵提取（Gribb-Hartmann 法）
- `Plane` 平面表示
- 单 AABB 测试 < 5 ns
- 8 AABB 批量 SIMD 测试 < 25 ns

## 2. API

```rust
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    /// 平面归一化（normal 和 d 同时除以 normal 长度）
    pub fn normalize(self) -> Self;
    
    /// 点到平面的有符号距离
    pub fn signed_distance(self, p: Vec3) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrustumResult {
    Inside,
    Outside,
    Intersect,
}

#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    /// 从 view_projection 矩阵提取 6 个平面
    /// Gribb-Hartmann 法
    pub fn from_view_projection(vp: Mat4) -> Self;
    
    /// AABB 测试
    pub fn classify_aabb(&self, aabb: AABB) -> FrustumResult;
    
    /// SIMD 批量 AABB 测试（8 个 AABB 一次）
    pub fn classify_aabb_batch(&self, aabbs: &[AABB], results: &mut [FrustumResult]);
    
    /// 简单剔除（仅 Outside / Inside，无 Intersect）
    pub fn cull_aabb(&self, aabb: AABB) -> bool {
        matches!(self.classify_aabb(aabb), FrustumResult::Outside)
    }
}
```

## 3. Gribb-Hartmann 法

```rust
pub fn from_view_projection(vp: Mat4) -> Self {
    // vp 是行主序 mat[col][row] 的列主序（engine-math 约定）
    // Gribb-Hartmann 提取 6 平面：
    //   left   = row3 + row0
    //   right  = row3 - row0
    //   bottom = row3 + row1
    //   top    = row3 - row1
    //   near   = row3 + row2
    //   far    = row3 - row2
    // 然后归一化
    
    // engine-math Mat4::cols[col][row]，所以 row r 是 cols[0][r], cols[1][r], cols[2][r], cols[3][r]
    let r0 = [vp.cols[0][0], vp.cols[1][0], vp.cols[2][0], vp.cols[3][0]];
    let r1 = [vp.cols[0][1], vp.cols[1][1], vp.cols[2][1], vp.cols[3][1]];
    let r2 = [vp.cols[0][2], vp.cols[1][2], vp.cols[2][2], vp.cols[3][2]];
    let r3 = [vp.cols[0][3], vp.cols[1][3], vp.cols[2][3], vp.cols[3][3]];
    
    let planes = [
        Self::make_plane(r3[0] + r0[0], r3[1] + r0[1], r3[2] + r0[2], r3[3] + r0[3]),
        Self::make_plane(r3[0] - r0[0], r3[1] - r0[1], r3[2] - r0[2], r3[3] - r0[3]),
        Self::make_plane(r3[0] + r1[0], r3[1] + r1[1], r3[2] + r1[2], r3[3] + r1[3]),
        Self::make_plane(r3[0] - r1[0], r3[1] - r1[1], r3[2] - r1[2], r3[3] - r1[3]),
        Self::make_plane(r3[0] + r2[0], r3[1] + r2[1], r3[2] + r2[2], r3[3] + r2[3]),
        Self::make_plane(r3[0] - r2[0], r3[1] - r2[1], r3[2] - r2[2], r3[3] - r2[3]),
    ];
    
    Self { planes }
}

fn make_plane(a: f32, b: f32, c: f32, d: f32) -> Plane {
    let normal = Vec3::new(a, b, c);
    let len = normal.length();
    if len > 0.0 {
        Plane { normal: normal / len, d: d / len }
    } else {
        Plane { normal: Vec3::Y, d: 0.0 }
    }
}
```

## 4. AABB 测试

```rust
pub fn classify_aabb(&self, aabb: AABB) -> FrustumResult {
    let mut intersect = false;
    for plane in &self.planes {
        // AABB 正向顶点（平面法向最正方向）
        let p_vertex = Vec3::new(
            if plane.normal.x > 0.0 { aabb.max.x } else { aabb.min.x },
            if plane.normal.y > 0.0 { aabb.max.y } else { aabb.min.y },
            if plane.normal.z > 0.0 { aabb.max.z } else { aabb.min.z },
        );
        // AABB 负向顶点
        let n_vertex = Vec3::new(
            if plane.normal.x > 0.0 { aabb.min.x } else { aabb.max.x },
            if plane.normal.y > 0.0 { aabb.min.y } else { aabb.max.y },
            if plane.normal.z > 0.0 { aabb.min.z } else { aabb.max.z },
        );
        
        if plane.signed_distance(p_vertex) < 0.0 {
            return FrustumResult::Outside;  // 完全在外
        }
        if plane.signed_distance(n_vertex) < 0.0 {
            intersect = true;
        }
    }
    if intersect { FrustumResult::Intersect } else { FrustumResult::Inside }
}
```

## 5. SIMD 批量

```rust
pub fn classify_aabb_batch(&self, aabbs: &[AABB], results: &mut [FrustumResult]) {
    // 8 个一组 SIMD
    for chunk in aabbs.chunks(8) {
        for (i, aabb) in chunk.iter().enumerate() {
            results[i] = self.classify_aabb(*aabb);
        }
        // 未来：使用 f32x4 一次处理 4 个 plane × 8 个 aabb 的距离计算
        // 当前为简单实现，待 SIMD 抽象层（sprint-17 module 01）完成后优化
    }
}
```

## 6. 验收

- [ ] 单 AABB 分类 < 5 ns
- [ ] 8 AABB 批量 < 25 ns
- [ ] 1000 物体视锥剔除 < 25 µs
- [ ] Gribb-Hartmann 提取平面与 nalgebra 误差 < 1e-5
- [ ] 集成 `engine-render-3d/src/camera.rs`
- [ ] 示例：1000 cube 视锥剔除 60 FPS
