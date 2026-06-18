# Sprint 17 · 验收测试计划

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)

---

## 1. 单元测试清单

| 模块 | 测试数 | 路径 |
|------|--------|------|
| SIMD f32x4 | 30+ | `simd.rs::tests` |
| SIMD f32x8 | 30+ | `simd.rs::tests` |
| SOA 宏 | 10+ | `soa.rs::tests` |
| Mat4 inverse | 50+ | `mat4.rs::tests` |
| Quat 高级 | 40+ | `quat.rs::tests` |
| Dual Quat | 30+ | `dual_quat.rs::tests` |
| Frustum | 25+ | `frustum.rs::tests` |

## 2. 关键测试用例

### 2.1 Mat4 inverse 跨平台一致性

```rust
#[test]
fn test_inverse_consistency() {
    for _ in 0..1000 {
        let m = random_mat4();
        let inv = m.inverse().expect("non-singular");
        let product = m * inv;
        let diff: f32 = (0..16).map(|i| {
            let a = product.cols[i / 4][i % 4];
            let b = if i % 5 == 0 { 1.0 } else { 0.0 };
            (a - b).abs()
        }).sum();
        assert!(diff < 1e-3, "Mat4 inverse not identity: {}", diff);
    }
}
```

### 2.2 Quat squad 平滑性

```rust
#[test]
fn test_squad_smooth() {
    let q0 = Quat::IDENTITY;
    let q1 = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 2.0);
    let q2 = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI);
    
    // 100 帧插值，相邻帧差 < 阈值
    let mut prev = q0;
    for i in 0..100 {
        let t = i as f32 / 99.0;
        let curr = Quat::squad(q0, q1, q2, t);
        let diff = (curr.dot(prev) - 1.0).abs();
        assert!(diff < 0.05, "squad frame {} jump: {}", i, diff);
        prev = curr;
    }
}
```

### 2.3 Dual Quat DLB 蒙皮

```rust
#[test]
fn test_dual_quat_skinning() {
    // 4 关节链：shoulder → upper_arm → lower_arm → hand
    let transforms = vec![/* 4 个 DQ */];
    let weights = vec![/* 4 权重 / 顶点 */];
    
    let point = Vec3::new(0.0, 0.0, 1.0);
    let dlb_result = dlb_skin(point, &transforms, &weights);
    let lbs_result = lbs_skin(point, &transforms, &weights);
    
    let diff = (dlb_result - lbs_result).length();
    assert!(diff < 0.01, "DLB vs LBS diff: {}", diff);
}
```

### 2.4 Frustum 平面提取

```rust
#[test]
fn test_frustum_extraction() {
    let camera = Camera3D::default();
    let vp = camera.view_projection();
    let frustum = Frustum::from_view_projection(vp);
    
    // 8 个角的 cube 测试
    let cube = AABB::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5));
    
    // 中心点应该在视锥内
    let result = frustum.classify_aabb(cube);
    assert_ne!(result, FrustumResult::Outside);
}
```

## 3. 性能基准（cargo bench）

| 基准 | 目标 (AVX2) |
|------|------------|
| `f32x4_dot_1000` | < 1 µs |
| `f32x8_dot_1000` | < 500 ns |
| `mat4_inverse` | < 50 ns |
| `mat4_mul` | < 30 ns |
| `quat_slerp` | < 15 ns |
| `quat_squad` | < 30 ns |
| `dual_quat_sclerp` | < 50 ns |
| `frustum_classify_single` | < 5 ns |
| `frustum_classify_batch_8` | < 25 ns |
| `frustum_classify_batch_1000` | < 2.5 µs |

## 4. 跨平台编译矩阵

| 平台 | SIMD 后端 | 测试 |
|------|----------|------|
| x86_64 (SSE2) | f32x4 = `__m128` | ✅ |
| x86_64 (AVX2) | f32x8 = `__m256` | ✅ |
| aarch64 (NEON) | f32x4 = `float32x4_t` | ✅ |
| wasm32 (simd128) | f32x4 = `v128` | ✅ |
| 纯 scalar | 数组回退 | ✅ |

## 5. 集成验证

- [ ] `engine-render-3d` 的 `screen_to_world_ray` 移除 workaround，引用 `Mat4::inverse()`
- [ ] `engine-render-3d` 的 `Frustum` 替换为 `engine-math::Frustum`
- [ ] `engine-physics-2d` AABB 剔除改用 sprint-17 `Frustum`

## 6. 文档

- [ ] 公共 API 文档生成（`cargo doc`）
- [ ] SIMD 平台分发说明
- [ ] 4 种 Mat4 inverse 算法对比（伴随 / LU / Bareiss / SIMD）
- [ ] 动画曲线插值图示
