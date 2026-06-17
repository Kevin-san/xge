# Module 05 — API 参考 / 公共 API 速查

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)

---

## 1. SIMD 基础

```rust
// 128-bit / 256-bit 向量
pub struct f32x4(/* 平台分发 */);
pub struct f32x8(/* 平台分发 */);

// 操作
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
```

## 2. SOA 布局

```rust
// 宏
#[macro_export]
macro_rules! define_soa { ... }

// 类型
pub struct PositionSoA {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
}
```

## 3. Mat4 inverse

```rust
impl Mat4 {
    pub fn inverse(&self) -> Option<Self>;        // 主逆
    pub fn inverse_unchecked(&self) -> Self;      // 热路径，跳过奇异检查
    pub fn inverse_transpose(&self) -> Option<Mat3>;  // 法线矩阵
}
```

## 4. Quat 高级

```rust
impl Quat {
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self;
    pub fn to_axis_angle(self) -> (Vec3, f32);
    pub fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Self;  // ZYX
    pub fn to_euler(self) -> (f32, f32, f32);
    pub fn squad(q0: Self, q1: Self, q2: Self, t: f32) -> Self;
    pub fn swing_twist(self, twist_axis: Vec3) -> (Quat, Quat);
    pub fn log(self) -> Vec3;
    pub fn exp(v: Vec3) -> Self;
}
```

## 5. Dual Quat

```rust
pub struct DualQuat {
    pub real: Quat,
    pub dual: Quat,
}

impl DualQuat {
    pub const IDENTITY: Self;
    pub fn from_translation(t: Vec3) -> Self;
    pub fn from_rotation_translation(rot: Quat, trans: Vec3) -> Self;
    pub fn to_mat4(self) -> Mat4;
    pub fn sclerp(self, other: Self, t: f32) -> Self;
    pub fn conjugate(self) -> Self;
    pub fn inverse(self) -> Self;
    pub fn transform_point(self, p: Vec3) -> Vec3;
}
```

## 6. 视锥剔除

```rust
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    pub fn normalize(self) -> Self;
    pub fn signed_distance(self, p: Vec3) -> f32;
}

pub enum FrustumResult {
    Inside,
    Outside,
    Intersect,
}

pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_view_projection(vp: Mat4) -> Self;
    pub fn classify_aabb(&self, aabb: AABB) -> FrustumResult;
    pub fn classify_aabb_batch(&self, aabbs: &[AABB], results: &mut [FrustumResult]);
    pub fn cull_aabb(&self, aabb: AABB) -> bool;
}
```
