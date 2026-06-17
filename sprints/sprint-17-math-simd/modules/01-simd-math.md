# Module 01 — SIMD 抽象层与基础类型

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)
> 文件位置: `engine-math/src/simd.rs`, `engine-math/src/soa.rs`

---

## 1. 目标

建立平台分发（feature flag）的 SIMD 抽象层，提供：
- `f32x4` (128-bit) / `f32x8` (256-bit) 基础类型
- SOA (Structure of Arrays) 数据布局宏
- 自动选择最优后端：x86_64 SSE2 → AVX2、aarch64 NEON、wasm32 simd128

## 2. API 设计

```rust
// engine-math/src/simd.rs

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub struct f32x8(__m256);

#[cfg(all(target_arch = "x86_64", not(target_feature = "avx2")))]
pub struct f32x8([f32; 8]);

#[cfg(target_arch = "aarch64")]
pub struct f32x4(float32x4_t);

#[cfg(target_arch = "wasm32")]
pub struct f32x4(v128);

pub struct f32x4([f32; 4]); // fallback

impl f32x4 {
    pub fn splat(v: f32) -> Self;
    pub fn from_array(a: [f32; 4]) -> Self;
    pub fn to_array(self) -> [f32; 4];
    pub fn add(self, rhs: Self) -> Self;
    pub fn sub(self, rhs: Self) -> Self;
    pub fn mul(self, rhs: Self) -> Self;
    pub fn div(self, rhs: Self) -> Self;
    pub fn dot(self, rhs: Self) -> f32;
    pub fn length(self) -> f32;
    pub fn normalize(self) -> Self;
    pub fn min(self, rhs: Self) -> Self;
    pub fn max(self, rhs: Self) -> Self;
    pub fn lerp(self, b: Self, t: Self) -> Self;
}

impl f32x8 {
    // 同上
}
```

## 3. SOA 宏

```rust
// engine-math/src/soa.rs

#[macro_export]
macro_rules! define_soa {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            $(pub $field: Vec<$ty>),*
        }

        impl $name {
            pub fn with_capacity(cap: usize) -> Self {
                Self {
                    $($field: Vec::with_capacity(cap)),*
                }
            }

            pub fn push(&mut self, $(ref $field: $ty),*) {
                $(self.$field.push($field));*;
            }

            pub fn len(&self) -> usize {
                let lens = [$(self.$field.len()),*];
                *lens.iter().max().unwrap()
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }
    };
}

// 用法
define_soa!(PositionSoA {
    x: f32,
    y: f32,
    z: f32,
});
```

## 4. 平台分发策略

**Cargo.toml feature：**
```toml
[features]
default = ["std", "simd-sse2"]
simd-sse2 = []
simd-avx2 = ["simd-sse2"]
simd-neon = []
no_std = []
```

**运行时检测：**
```rust
#[inline]
pub fn best_f32x4() -> impl Fn(f32, f32, f32, f32) -> f32x4 {
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    { |x, y, z, w| unsafe { f32x4(_mm256_set_ps(w, z, y, x, w, z, y, x)) } } // 占位
    #[cfg(not(target_feature = "avx2"))]
    { |x, y, z, w| f32x4([x, y, z, w]) }
}
```

## 5. 验收标准

- [ ] 单元测试：f32x4 加减乘点积 100% 路径
- [ ] 平台编译矩阵：x86_64 (SSE2/AVX2) / aarch64 (NEON) / wasm32 (simd128) / 纯 scalar fallback
- [ ] `cargo bench` 基准：f32x4 批量 dot 1000 元素对比 scalar 加速比 ≥ 4x (AVX2)
- [ ] 文档：每个平台后端使用说明

## 6. 风险

| 风险 | 缓解 |
|------|------|
| `#[repr(simd)]` 不稳定 | 使用 `core::arch::*` 平台分发 |
| 不同 SIMD 指令集差异 | 测试矩阵覆盖 |
| Debug 模式性能 | 提供 `#[inline(never)]` 调试路径 |
