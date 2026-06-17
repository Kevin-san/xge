# Sprint 17 · 数学库重构 + SIMD 加速（Mat4 inverse 修复 / SO-A / squad / Dual Quat）

> 文档编号: `sprint-17-math-simd.md / v1.0
> 周期: 3 周 (15 个工作日)
> 上游依赖: 无
> 下游交付: Sprint 18 (ECS), Sprint 19 (Render), Sprint 20 (Physics) 全线使用

---

## 1. 目标与范围

**目标：** 将 `engine-math` 从基础线性代数库升级为 SIMD 友好、SOA 布局、数学模型最新的高性能数学库，并修复 Mat4 inverse 行列式符号错误。

**范围：**
- ✅ 修复 `engine-math/src/mat4.rs#inverse` 行列式符号/列主序错误
- ✅ 引入 SIMD 抽象层（`f32x4` / `f32x8`）封装 SSE2/AVX2/NEON
- ✅ 4×4 矩阵乘法向量化（4 个 f32x4 列并行）
- ✅ Quat 增加 `squad` 样条插值、Swing-Twist 分解
- ✅ Dual Quaternion（对偶四元数）线性蒙皮
- ✅ 视锥剔除 SIMD 8 物体批量测试
- ⛔ 不含：2D 渲染专用数学、UI 数学、向量图形数学（这些留在各自 crate）

**不目标（Out of Scope）：**
- 不替换 glam/nalgebra 第三方库（保持 crate 自治）
- 不实现 Tensor/Matrix 维度动态（保持固定 2/3/4 维）

---

## 2. 与上游需求对接

| 来源 | 关联章节 | 承接 |
|------|---------|------|
| [NEXT_PHASE_REQUIREMENTS.md § 3.2](../NEXT_PHASE_REQUIREMENTS.md) | Mat4 inverse 算法 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 3.3](../NEXT_PHASE_REQUIREMENTS.md) | Quat 优化 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 3.4](../NEXT_PHASE_REQUIREMENTS.md) | SIMD 抽象 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 3.5](../NEXT_PHASE_REQUIREMENTS.md) | 视锥剔除 SIMD | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 3.6](../NEXT_PHASE_REQUIREMENTS.md) | SOA 布局 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 5 M1](../NEXT_PHASE_REQUIREMENTS.md) | Milestone M1 | ✅ 本 sprint |

---

## 3. 子模块拆分

### 3.1 [01-simd-math.md](modules/01-simd-math.md) — SIMD 抽象层与基础类型

**核心交付：**
- `engine-math/src/simd.rs` — 平台抽象 SIMD 类型
  - `#[repr(simd)]` 或 `core::arch::x86_64::__m128`
  - `f32x4` (SSE2 128-bit), `f32x8` (AVX2 256-bit)
  - feature flag 切换：`simd-sse2` / `simd-avx2` / `simd-neon`
- `engine-math/src/soa.rs` — SOA 数据结构宏
  - `define_soa!` 宏，生成 `PositionSoA { x: Vec<f32>, y: Vec<f32>, z: Vec<f32> }`
  - 转换函数 `aos_to_soa` / `soa_to_aos`
- `engine-math/src/lib.rs` — re-export 新类型

**验收：**
- 单元测试覆盖 `f32x4` 加减乘点积 100% 路径
- `criterion` 基准：SIMD vs scalar 比较，4×4 matmul 提速 ≥ 2x
- 平台分发：x86_64 默认 SSE2，可选 AVX2；aarch64 自动 NEON

---

### 3.2 [02-mat4-inverse.md](modules/02-mat4-inverse.md) — 4×4 矩阵逆重构

**核心交付：**
- `engine-math/src/mat4.rs` 重写 `inverse()` 方法
  - **算法选择：** 列主序伴随矩阵法（无分支 LU 替代方案作为 future 优化）
  - 显式 `a00..a33` 命名，避免混淆
  - 使用 `f32x4` SIMD 加速 4×4 行列式
- 添加 `inverse_unchecked()` 用于内联热路径
- 添加 `inverse_transpose()` 联合操作（法线矩阵常用）

**验收：**
- ✅ 单元测试 100 个随机矩阵 vs nalgebra 对比误差 < 1e-4
- ✅ `screen_to_world_ray`（3D 相机）可去掉手写 workaround
- ✅ 性能基准：4×4 矩阵求逆 < 50 ns (AVX2)

**Bug 修复对应：** `engine-math/src/mat4.rs#L175-L288`

---

### 3.3 [03-quat-squad-dual.md](modules/03-quat-squad-dual.md) — 四元数高级插值

**核心交付：**
- `engine-math/src/quat.rs` 扩展
  - `from_axis_angle(axis: Vec3, angle: f32) -> Quat`
  - `from_euler(yaw, pitch, roll) -> Quat` (ZYX 顺序)
  - `to_euler() -> (f32, f32, f32)`
  - `squad(q0, q1, q2, t) -> Quat` 球面四边形插值
  - `swing_twist_decompose(twist_axis: Vec3) -> (Quat, Quat)` 用于角色骨骼
  - `log()/exp()` 用于四元数微分
- `engine-math/src/dual_quat.rs` — 新增
  - `DualQuat { real: Quat, dual: Quat }`
  - 加法、乘法、共轭、取反
  - `to_mat4()` 转 4×4 矩阵
  - `sclerp()` 最短路径线性插值（双四元数线性混合蒙皮 DLB）

**验收：**
- 测试 `squad` 100 帧动画无跳变
- 测试 `swing_twist_decompose` 分解后 `swing * twist` 还原为原 quat
- DLB 蒙皮示例：4 关节链，误差 < 0.5% vs LBS（线性蒙皮）

---

### 3.4 [04-frustum-culling-simd.md](modules/04-frustum-culling-simd.md) — SIMD 视锥剔除

**核心交付：**
- `engine-math/src/frustum.rs` — 新增
  - `Frustum { planes: [Plane; 6] }` 6 平面
  - `Plane { normal: Vec3, d: f32 }`
  - `extract_from_view_proj(vp: Mat4) -> Frustum` Gribb-Hartmann 法
  - `classify_aabb(aabb: AABB) -> FrustumResult { Inside, Outside, Intersect }`
  - `classify_aabb_batch(aabbs: &[AABB], results: &mut [FrustumResult])` SIMD 批处理
- 与 `engine-render-3d/src/camera.rs` Frustum 集成

**验收：**
- 单 AABB 剔除 < 5 ns
- 8 AABB 批量 SIMD 测试 < 25 ns（理论上 4× speedup）
- 1000 物体大场景：cull pass < 25 µs

---

## 4. 验收清单（acceptance/test-plan.md）

- [ ] 100 个随机 Mat4 矩阵 inverse vs nalgebra 误差 < 1e-4
- [ ] 1000 个矩阵矩阵乘法基准，AVX2 提升 ≥ 2x vs scalar
- [ ] squad 100 帧动画视觉无跳变
- [ ] Dual Quat DLB 蒙皮 vs LBS 误差 < 1%
- [ ] 8 AABB 批量剔除 < 25 ns
- [ ] 全部平台编译通过：x86_64 (SSE2/AVX2) / aarch64 (NEON) / wasm32 (simd128)
- [ ] `cargo test -p engine-math` 全通过
- [ ] `cargo bench -p engine-math` 基准记录存档

---

## 5. API 稳定承诺

```rust
// engine-math/src/lib.rs 公共 API
pub use simd::{f32x4, f32x8};
pub use soa::SoaVec3;
pub use mat4::Mat4;
pub use quat::Quat;
pub use dual_quat::DualQuat;
pub use frustum::{Frustum, Plane, FrustumResult};
```

**禁止破坏性变更：** 已发布的 `Vec2/3/4`、`Mat4::from_*`、`Quat::from_rotation_*` API 必须保持兼容。

---

## 6. 风险与缓解

| 风险 | 等级 | 缓解 |
|------|------|------|
| SIMD 在小 crate 编译时间 | 中 | 默认仅启用 SSE2，AVX2 走 feature flag |
| `#[repr(simd)]` nightly 依赖 | 高 | 使用 `core::arch::*` 平台分发 + `#[target_feature]` |
| 列主序混淆导致回归 | 中 | 100 个随机矩阵交叉验证 |
| Dual Quat 蒙皮精度 | 低 | 限定用于骨骼权重，保留 LBS 兜底 |
