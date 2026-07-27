//! SIMD 抽象层：为游戏数学内核提供平台自适应的 `f32x4` / `f32x8` 类型。
//!
//! # 设计
//! - `f32x4` 在所有平台上均提供相同的公开标量方法，保证零成本抽象在
//!   非 SIMD 平台上仍然可用。
//! - 在 `x86_64` / `aarch64` 平台上额外提供
//!   `simd::x86::*` / `simd::aarch64_backend::*` 模块，通过 `#[target_feature]`
//!   启用原生 SIMD。
//! - `f32x8` 在所有平台上均以标量实现（AVX2 属可选优化，保持最简）。

use core::ops::{Add, Div, Mul, Sub};

/// 128-bit SIMD 向量，包含 4 个 `f32`。
///
/// 内部使用 `[f32; 4]` 存储，保证 `#[repr(C, align(16))]` 对齐，
/// 便于直接映射到 SSE2 / NEON / wasm32 的 128-bit SIMD 寄存器。
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct f32x4(pub(crate) [f32; 4]);

/// 256-bit SIMD 向量，包含 8 个 `f32`。
///
/// 在所有平台上使用标量实现，便于跨平台移植。
#[derive(Clone, Copy)]
#[repr(C, align(32))]
pub struct f32x8(pub(crate) [f32; 8]);

impl f32x4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self([v; 4])
    }

    /// 从内存中加载 4 个连续 `f32`。
    ///
    /// # Safety
    /// `ptr` 必须指向至少包含 4 个 `f32` 的有效对齐内存。
    #[inline]
    pub unsafe fn load(ptr: *const f32) -> Self {
        Self([*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3)])
    }

    /// 把 4 个分量写回到连续内存。
    ///
    /// # Safety
    /// `ptr` 必须指向至少 4 个 `f32` 可写的有效对齐内存。
    #[inline]
    pub unsafe fn store(self, ptr: *mut f32) {
        *ptr = self.0[0];
        *ptr.add(1) = self.0[1];
        *ptr.add(2) = self.0[2];
        *ptr.add(3) = self.0[3];
    }

    #[inline]
    pub const fn from_array(arr: [f32; 4]) -> Self {
        Self(arr)
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 4] {
        self.0
    }

    #[inline]
    pub const fn extract(self, i: usize) -> f32 {
        self.0[i]
    }

    #[inline]
    pub fn add_vec(self, other: Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
    }

    #[inline]
    pub fn sub_vec(self, other: Self) -> Self {
        Self([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
            self.0[3] - other.0[3],
        ])
    }

    #[inline]
    pub fn mul_vec(self, other: Self) -> Self {
        Self([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }

    #[inline]
    pub fn div_vec(self, other: Self) -> Self {
        Self([
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
            self.0[3] / other.0[3],
        ])
    }

    #[inline]
    pub fn mul_scalar(self, s: f32) -> Self {
        Self([
            self.0[0] * s,
            self.0[1] * s,
            self.0[2] * s,
            self.0[3] * s,
        ])
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self([
            self.0[0].abs(),
            self.0[1].abs(),
            self.0[2].abs(),
            self.0[3].abs(),
        ])
    }

    #[inline]
    pub fn sum(self) -> f32 {
        self.0[0] + self.0[1] + self.0[2] + self.0[3]
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self([
            self.0[0].max(other.0[0]),
            self.0[1].max(other.0[1]),
            self.0[2].max(other.0[2]),
            self.0[3].max(other.0[3]),
        ])
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self([
            self.0[0].min(other.0[0]),
            self.0[1].min(other.0[1]),
            self.0[2].min(other.0[2]),
            self.0[3].min(other.0[3]),
        ])
    }

    /// 返回每个分量是否小于 0 的 mask（小于 0 则为 -1.0，否则为 0.0）。
    #[inline]
    pub fn lt_zero_mask(self) -> Self {
        Self([
            if self.0[0] < 0.0 { -1.0 } else { 0.0 },
            if self.0[1] < 0.0 { -1.0 } else { 0.0 },
            if self.0[2] < 0.0 { -1.0 } else { 0.0 },
            if self.0[3] < 0.0 { -1.0 } else { 0.0 },
        ])
    }

    /// 三元混合：mask 中为真的位置取 `a`，否则取 `b`。
    #[inline]
    pub fn blendv(mask: Self, a: Self, b: Self) -> Self {
        Self([
            if mask.0[0] < 0.0 { a.0[0] } else { b.0[0] },
            if mask.0[1] < 0.0 { a.0[1] } else { b.0[1] },
            if mask.0[2] < 0.0 { a.0[2] } else { b.0[2] },
            if mask.0[3] < 0.0 { a.0[3] } else { b.0[3] },
        ])
    }
}

impl Add for f32x4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output { self.add_vec(rhs) }
}

impl Sub for f32x4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { self.sub_vec(rhs) }
}

impl Mul for f32x4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output { self.mul_vec(rhs) }
}

impl Div for f32x4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output { self.div_vec(rhs) }
}

impl f32x8 {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(v0: f32, v1: f32, v2: f32, v3: f32, v4: f32, v5: f32, v6: f32, v7: f32) -> Self {
        Self([v0, v1, v2, v3, v4, v5, v6, v7])
    }

    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self([v; 8])
    }

    /// 从内存中加载 8 个连续 `f32`。
    ///
    /// # Safety
    /// `ptr` 必须指向至少包含 8 个 `f32` 的有效对齐内存。
    #[inline]
    pub unsafe fn load(ptr: *const f32) -> Self {
        Self([
            *ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3),
            *ptr.add(4), *ptr.add(5), *ptr.add(6), *ptr.add(7),
        ])
    }

    /// 把 8 个分量写回到连续内存。
    ///
    /// # Safety
    /// `ptr` 必须指向至少 8 个 `f32` 可写的有效对齐内存。
    #[inline]
    pub unsafe fn store(self, ptr: *mut f32) {
        let mut i = 0;
        while i < 8 {
            *ptr.add(i) = self.0[i];
            i += 1;
        }
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 8] {
        self.0
    }

    #[inline]
    pub fn add_vec(self, other: Self) -> Self {
        let mut r = [0.0f32; 8];
        let mut i = 0;
        while i < 8 {
            r[i] = self.0[i] + other.0[i];
            i += 1;
        }
        Self(r)
    }

    #[inline]
    pub fn sub_vec(self, other: Self) -> Self {
        let mut r = [0.0f32; 8];
        let mut i = 0;
        while i < 8 {
            r[i] = self.0[i] - other.0[i];
            i += 1;
        }
        Self(r)
    }

    #[inline]
    pub fn mul_vec(self, other: Self) -> Self {
        let mut r = [0.0f32; 8];
        let mut i = 0;
        while i < 8 {
            r[i] = self.0[i] * other.0[i];
            i += 1;
        }
        Self(r)
    }

    #[inline]
    pub fn mul_scalar(self, s: f32) -> Self {
        let mut r = [0.0f32; 8];
        let mut i = 0;
        while i < 8 {
            r[i] = self.0[i] * s;
            i += 1;
        }
        Self(r)
    }

    #[inline]
    pub fn lt_zero_mask(self) -> Self {
        let mut r = [0.0f32; 8];
        let mut i = 0;
        while i < 8 {
            r[i] = if self.0[i] < 0.0 { -1.0 } else { 0.0 };
            i += 1;
        }
        Self(r)
    }

    #[inline]
    pub fn blendv(mask: Self, a: Self, b: Self) -> Self {
        let mut r = [0.0f32; 8];
        let mut i = 0;
        while i < 8 {
            r[i] = if mask.0[i] < 0.0 { a.0[i] } else { b.0[i] };
            i += 1;
        }
        Self(r)
    }
}

impl Add for f32x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output { self.add_vec(rhs) }
}

impl Sub for f32x8 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { self.sub_vec(rhs) }
}

impl Mul for f32x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output { self.mul_vec(rhs) }
}

// ---- x86_64 原生 SIMD 后端 ----
#[cfg(target_arch = "x86_64")]
#[allow(clippy::missing_safety_doc)]
pub mod x86 {
    use super::*;
    use core::arch::x86_64;

    #[inline]
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_new(x: f32, y: f32, z: f32, w: f32) -> f32x4 {
        let v = x86_64::_mm_setr_ps(x, y, z, w);
        let mut out = [0.0f32; 4];
        x86_64::_mm_storeu_ps(out.as_mut_ptr(), v);
        f32x4(out)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_add(a: f32x4, b: f32x4) -> f32x4 {
        let va = x86_64::_mm_loadu_ps(a.0.as_ptr());
        let vb = x86_64::_mm_loadu_ps(b.0.as_ptr());
        let r = x86_64::_mm_add_ps(va, vb);
        let mut out = [0.0f32; 4];
        x86_64::_mm_storeu_ps(out.as_mut_ptr(), r);
        f32x4(out)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_sub(a: f32x4, b: f32x4) -> f32x4 {
        let va = x86_64::_mm_loadu_ps(a.0.as_ptr());
        let vb = x86_64::_mm_loadu_ps(b.0.as_ptr());
        let r = x86_64::_mm_sub_ps(va, vb);
        let mut out = [0.0f32; 4];
        x86_64::_mm_storeu_ps(out.as_mut_ptr(), r);
        f32x4(out)
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_mul(a: f32x4, b: f32x4) -> f32x4 {
        let va = x86_64::_mm_loadu_ps(a.0.as_ptr());
        let vb = x86_64::_mm_loadu_ps(b.0.as_ptr());
        let r = x86_64::_mm_mul_ps(va, vb);
        let mut out = [0.0f32; 4];
        x86_64::_mm_storeu_ps(out.as_mut_ptr(), r);
        f32x4(out)
    }
}

// ---- aarch64 NEON 后端 ----
#[cfg(target_arch = "aarch64")]
#[allow(clippy::missing_safety_doc)]
pub mod aarch64_backend {
    use super::*;
    use core::arch::aarch64;

    #[inline]
    #[target_feature(enable = "neon")]
    pub unsafe fn neon_add(a: f32x4, b: f32x4) -> f32x4 {
        let va = aarch64::vld1q_f32(a.0.as_ptr());
        let vb = aarch64::vld1q_f32(b.0.as_ptr());
        let r = aarch64::vaddq_f32(va, vb);
        let mut out = [0.0f32; 4];
        aarch64::vst1q_f32(out.as_mut_ptr(), r);
        f32x4(out)
    }

    #[inline]
    #[target_feature(enable = "neon")]
    pub unsafe fn neon_mul(a: f32x4, b: f32x4) -> f32x4 {
        let va = aarch64::vld1q_f32(a.0.as_ptr());
        let vb = aarch64::vld1q_f32(b.0.as_ptr());
        let r = aarch64::vmulq_f32(va, vb);
        let mut out = [0.0f32; 4];
        aarch64::vst1q_f32(out.as_mut_ptr(), r);
        f32x4(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32x4_new_access() {
        let v = f32x4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.extract(0), 1.0);
        assert_eq!(v.extract(1), 2.0);
        assert_eq!(v.extract(2), 3.0);
        assert_eq!(v.extract(3), 4.0);
        assert_eq!(v.to_array(), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_f32x4_splat() {
        let v = f32x4::splat(7.0);
        assert_eq!(v.to_array(), [7.0, 7.0, 7.0, 7.0]);
    }

    #[test]
    fn test_f32x4_arithmetic() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!((a + b).to_array(), [6.0, 8.0, 10.0, 12.0]);
        assert_eq!((b - a).to_array(), [4.0, 4.0, 4.0, 4.0]);
        assert_eq!((a * b).to_array(), [5.0, 12.0, 21.0, 32.0]);
        assert_eq!((a / f32x4::splat(2.0)).to_array(), [0.5, 1.0, 1.5, 2.0]);
        assert_eq!(a.mul_scalar(3.0).to_array(), [3.0, 6.0, 9.0, 12.0]);
    }

    #[test]
    fn test_f32x4_abs_sum() {
        let v = f32x4::new(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(v.abs().to_array(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v.sum(), 2.0);
    }

    #[test]
    fn test_f32x4_min_max() {
        let a = f32x4::new(1.0, 5.0, 3.0, 7.0);
        let b = f32x4::new(4.0, 2.0, 6.0, 0.0);
        assert_eq!(a.min(b).to_array(), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(a.max(b).to_array(), [4.0, 5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_f32x4_blendv() {
        let mask = f32x4::new(-1.0, 0.5, -3.0, 4.0).lt_zero_mask();
        let a = f32x4::new(10.0, 20.0, 30.0, 40.0);
        let b = f32x4::new(100.0, 200.0, 300.0, 400.0);
        let r = f32x4::blendv(mask, a, b);
        assert_eq!(r.to_array(), [10.0, 200.0, 30.0, 400.0]);
    }

    #[test]
    fn test_f32x4_load_store() {
        let src = [1.0f32, 2.0, 3.0, 4.0];
        let v = unsafe { f32x4::load(src.as_ptr()) };
        let mut dst = [0.0f32; 4];
        unsafe { v.store(dst.as_mut_ptr()) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_f32x8_basic() {
        let a = f32x8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = f32x8::splat(1.0);
        assert_eq!((a + b).to_array(), [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        assert_eq!((a * b).to_array(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_f32x8_blendv() {
        let mask = f32x8::new(-1.0, 0.5, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0).lt_zero_mask();
        let a = f32x8::splat(1.0);
        let b = f32x8::splat(0.0);
        let r = f32x8::blendv(mask, a, b);
        assert_eq!(r.to_array(), [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_f32x8_mul_scalar() {
        let a = f32x8::splat(2.0);
        let r = a.mul_scalar(5.0);
        assert_eq!(r.to_array(), [10.0; 8]);
    }

    #[test]
    fn test_f32x4_zero_vector() {
        let v = f32x4::splat(0.0);
        assert_eq!(v.to_array(), [0.0; 4]);
        assert_eq!(v.sum(), 0.0);
        assert_eq!(v.abs().to_array(), [0.0; 4]);
    }

    #[test]
    fn test_f32x4_negative_values() {
        let a = f32x4::new(-1.0, -2.0, -3.0, -4.0);
        let b = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let sum = a + b;
        assert_eq!(sum.to_array(), [0.0; 4]);
        assert_eq!(a.abs().to_array(), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_f32x4_div_by_zero() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::splat(0.0);
        let result = a / b;
        assert!(result.extract(0).is_infinite() || result.extract(0).is_nan());
    }

    #[test]
    fn test_f32x4_clone_copy() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = a;
        assert_eq!(a.to_array(), b.to_array());
        let c = a.clone();
        assert_eq!(a.to_array(), c.to_array());
    }

    #[test]
    fn test_f32x4_order_independent() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let ab = a + b;
        let ba = b + a;
        assert_eq!(ab.to_array(), ba.to_array());
    }

    #[test]
    fn test_f32x8_zero_vector() {
        let v = f32x8::splat(0.0);
        assert_eq!(v.to_array(), [0.0; 8]);
    }

    #[test]
    fn test_f32x8_sub() {
        let a = f32x8::new(5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0);
        let b = f32x8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let r = a - b;
        assert_eq!(r.to_array(), [4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0]);
    }

    #[test]
    fn test_f32x8_clone_copy() {
        let a = f32x8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = a;
        assert_eq!(a.to_array(), b.to_array());
        let c = a.clone();
        assert_eq!(a.to_array(), c.to_array());
    }

    #[test]
    fn test_f32x4_scalar_mul_commutative() {
        let v = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let s = 3.0;
        assert_eq!(v.mul_scalar(s).to_array(), (v * f32x4::splat(s)).to_array());
    }
}
