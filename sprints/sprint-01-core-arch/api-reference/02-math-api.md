# 数学库 API 清单

## 模块名称与概述

本文档列出 `engine-math` 模块的所有公开 API。数学库提供游戏引擎常用的数学原语，包括向量、矩阵、四元数、变换和几何原语。库保证 `no_std + alloc` 支持。

## API 清单

### 1. Vec2

#### 常量
```rust
impl Vec2 {
    pub const ZERO: Self;
    pub const ONE: Self;
    pub const X: Self;
    pub const Y: Self;
}
```

#### 构造与运算
```rust
impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self;
}

impl Add for Vec2 { /* a + b */ }
impl Sub for Vec2 { /* a - b */ }
impl Mul<f32> for Vec2 { /* a * scalar */ }
impl Div<f32> for Vec2 { /* a / scalar */ }
impl Neg for Vec2 { /* -a */ }
```

#### 向量运算
```rust
impl Vec2 {
    pub fn dot(self, other: Self) -> f32;
    pub fn cross(self, other: Self) -> f32;
    pub fn length(self) -> f32;
    pub fn length_squared(self) -> f32;
    pub fn normalize(self) -> Self;
    pub fn normalize_or_zero(self) -> Self;
    pub fn lerp(self, other: Self, t: f32) -> Self;
}
```

---

### 2. Vec3

#### 常量
```rust
impl Vec3 {
    pub const ZERO: Self;
    pub const ONE: Self;
    pub const X: Self;
    pub const Y: Self;
    pub const Z: Self;
}
```

#### 构造与运算
```rust
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self;
}

impl Add for Vec3 { /* a + b */ }
impl Sub for Vec3 { /* a - b */ }
impl Mul<f32> for Vec3 { /* a * scalar */ }
impl Div<f32> for Vec3 { /* a / scalar */ }
impl Neg for Vec3 { /* -a */ }
```

#### 向量运算
```rust
impl Vec3 {
    pub fn dot(self, other: Self) -> f32;
    pub fn cross(self, other: Self) -> Vec3;
    pub fn length(self) -> f32;
    pub fn length_squared(self) -> f32;
    pub fn normalize(self) -> Self;
    pub fn normalize_or_zero(self) -> Self;
    pub fn lerp(self, other: Self, t: f32) -> Self;
}
```

---

### 3. Vec4

#### 常量
```rust
impl Vec4 {
    pub const ZERO: Self;
    pub const ONE: Self;
    pub const X: Self;
    pub const Y: Self;
    pub const Z: Self;
    pub const W: Self;
}
```

#### 构造与运算
```rust
impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self;
}

impl Add for Vec4 { /* a + b */ }
impl Sub for Vec4 { /* a - b */ }
impl Mul<f32> for Vec4 { /* a * scalar */ }
impl Div<f32> for Vec4 { /* a / scalar */ }
impl Neg for Vec4 { /* -a */ }
```

#### 向量运算
```rust
impl Vec4 {
    pub fn dot(self, other: Self) -> f32;
    pub fn length(self) -> f32;
    pub fn length_squared(self) -> f32;
    pub fn normalize(self) -> Self;
    pub fn normalize_or_zero(self) -> Self;
    pub fn lerp(self, other: Self, t: f32) -> Self;
}
```

---

### 4. Mat4

#### 常量
```rust
impl Mat4 {
    pub const IDENTITY: Self;
    pub const ZERO: Self;
}
```

#### 构造
```rust
impl Mat4 {
    pub const fn from_translation(v: Vec3) -> Self;
    pub const fn from_scale(v: Vec3) -> Self;
    pub const fn from_rotation_x(angle: f32) -> Self;
    pub const fn from_rotation_y(angle: f32) -> Self;
    pub const fn from_rotation_z(angle: f32) -> Self;
    pub const fn from_quat(q: Quat) -> Self;
    pub fn look_at_rh(eye: Vec3, target: Vec3, up: Vec3) -> Self;
    pub fn perspective_rh(fovy: f32, aspect: f32, near: f32, far: f32) -> Self;
    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self;
}
```

#### 矩阵运算
```rust
impl Mat4 {
    pub fn inverse(&self) -> Option<Self>;
    pub fn transpose(&self) -> Self;
    pub fn mul_vec4(&self, v: Vec4) -> Vec4;
    pub fn to_cols_array(&self) -> [f32; 16];
}
```

---

### 5. Quat

#### 常量
```rust
impl Quat {
    pub const IDENTITY: Self;
}
```

#### 构造
```rust
impl Quat {
    pub const fn from_rotation_x(angle: f32) -> Self;
    pub const fn from_rotation_y(angle: f32) -> Self;
    pub const fn from_rotation_z(angle: f32) -> Self;
    pub const fn from_euler(euler: Euler) -> Self;
}
```

#### 四元数运算
```rust
impl Quat {
    pub fn to_euler(&self) -> Euler;
    pub fn mul(self, other: Self) -> Self;
    pub fn inverse(&self) -> Self;
    pub fn normalize(&self) -> Self;
    pub fn slerp(self, other: Self, t: f32) -> Self;
    pub fn nlerp(self, other: Self, t: f32) -> Self;
}
```

---

### 6. Transform

#### 构造
```rust
impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self;
    pub fn from_translation(v: Vec3) -> Self;
}
```

#### 变换运算
```rust
impl Transform {
    pub fn matrix(&self) -> Mat4;
    pub fn inverse(&self) -> Self;
}
```

---

### 7. Euler

#### 构造
```rust
impl Euler {
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self;
    pub fn from_radians(pitch: f32, yaw: f32, roll: f32) -> Self;
    pub fn to_radians(&self) -> (f32, f32, f32);
}
```

---

### 8. Rect

#### 构造
```rust
impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self;
}
```

#### 操作
```rust
impl Rect {
    pub fn contains(&self, point: Vec2) -> bool;
    pub fn intersects(&self, other: &Self) -> bool;
}
```

---

### 9. AABB

#### 构造
```rust
impl AABB {
    pub fn new(center: Vec3, half_extents: Vec3) -> Self;
}
```

#### 操作
```rust
impl AABB {
    pub fn min(&self) -> Vec3;
    pub fn max(&self) -> Vec3;
    pub fn contains(&self, point: Vec3) -> bool;
}
```

---

## API 统计

| 类型 | 方法数 |
|------|--------|
| Vec2 | 12 |
| Vec3 | 13 |
| Vec4 | 12 |
| Mat4 | 11 |
| Quat | 10 |
| Transform | 4 |
| Euler | 3 |
| Rect | 3 |
| AABB | 4 |
| **总计** | **72** |

## 优先级

**P0（必须）：**
- Vec2/Vec3/Vec4 完整实现
- Mat4 完整实现
- Quat 完整实现
- Transform

**P1（重要）：**
- Rect / AABB
- Euler

**P2（可选）：**
- OBB
- Mat2/Mat3
