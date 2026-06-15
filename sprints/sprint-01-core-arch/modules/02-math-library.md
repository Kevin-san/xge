# 数学库需求

## 模块名称与概述

`engine-math` 模块提供面向游戏引擎的数学原语库，包括向量（Vec2/Vec3/Vec4）、矩阵（Mat2/Mat3/Mat4）、四元数（Quat）、变换（Transform）以及基础几何原语（Rect/AABB/OBB）。数学库保证 `no_std + alloc` 支持，便于嵌入式主机移植。

## 需求编号

对应原文档需求编号：32-38, 56-61, 169-244

## 功能描述

### 1. 向量类型

#### Vec2
- `Vec2::new(x, y)` — 创建二维向量
- `Vec2::ZERO` / `Vec2::ONE` / `Vec2::X` / `Vec2::Y` 常量
- `Add / Sub / Mul / Div` 运算符重载
- `dot()` — 点积
- `cross()` — 叉积（返回标量）
- `length()` / `length_squared()` — 向量长度
- `normalize()` / `normalize_or_zero()` — 归一化
- `lerp(a, b, t)` — 线性插值

#### Vec3
- 同 Vec2 完整功能套件
- 三维向量特有操作

#### Vec4
- 同 Vec2 完整功能套件
- 四维向量特有操作（齐次坐标）

### 2. 矩阵类型

#### Mat4
- `Mat4::IDENTITY` / `Mat4::ZERO`
- `from_translation(v)` — 从平移创建
- `from_scale(v)` — 从缩放创建
- `from_rotation_x(angle)` — X 轴旋转
- `from_rotation_y(angle)` — Y 轴旋转
- `from_rotation_z(angle)` — Z 轴旋转
- `from_quat(q)` — 从四元数创建
- `look_at_rh(eye, target, up)` — 右手坐标系观察矩阵
- `perspective_rh(fovy, aspect, near, far)` — 右手坐标系透视投影
- `orthographic_rh(left, right, bottom, top, near, far)` — 右手坐标系正交投影
- `inverse(&self)` — 矩阵求逆
- `transpose(&self)` — 矩阵转置
- `mul_vec4(&self, v)` — 矩阵向量乘法
- `to_cols_array(&self)` — 转换为列数组

### 3. 四元数 Quat

- `Quat::IDENTITY` — 单位四元数
- `from_rotation_x(angle)` — X 轴旋转
- `from_rotation_y(angle)` — Y 轴旋转
- `from_rotation_z(angle)` — Z 轴旋转
- `from_euler(euler)` — 从欧拉角创建
- `to_euler(&self)` — 转换为欧拉角
- `mul(q1, q2)` — 四元数乘法
- `inverse(&self)` — 求逆
- `normalize(&self)` — 归一化
- `slerp(a, b, t)` — 球面线性插值
- `nlerp(a, b, t)` — 规范化线性插值

### 4. Transform 变换

- `Transform::new(pos, rot, scale)` — 从位置、旋转、缩放创建
- `Transform::from_translation(v)` — 仅平移变换
- `matrix(&self) -> Mat4` — 转换为矩阵
- `inverse(&self) -> Transform` — 求逆变换

### 5. 几何原语

#### Rect
- `Rect::new(x, y, w, h)` — 创建矩形
- `contains(point)` — 点包含检测
- `intersects(other)` — 矩形相交检测

#### AABB（轴对齐包围盒）
- `AABB::new(center, half_extents)` — 创建 AABB
- `min()` — 获取最小点
- `max()` — 获取最大点
- `contains(point)` — 点包含检测

#### OBB（有向包围盒）
- 定向包围盒支持

### 6. Euler 角

- `Euler` 与 `Quat` 互转
- 支持多种旋转顺序（XYZ, YXZ, ZYX 等）

### 7. 插值函数

- `lerp` — 线性插值
- `slerp` — 球面线性插值
- `nlerp` — 规范化线性插值

## API 签名

### Vec2
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0 };

    pub const fn new(x: f32, y: f32) -> Self;
    pub fn dot(self, other: Self) -> f32;
    pub fn cross(self, other: Self) -> f32;
    pub fn length(self) -> f32;
    pub fn length_squared(self) -> f32;
    pub fn normalize(self) -> Self;
    pub fn normalize_or_zero(self) -> Self;
    pub fn lerp(self, other: Self, t: f32) -> Self;
}
```

### Vec3
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
    // ... 同 Vec2 方法
}
```

### Vec4
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
// ... 同 Vec2/Vec3 方法
```

### Mat4
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mat4([[f32; 4]; 4]);

impl Mat4 {
    pub const IDENTITY: Self = /* ... */;
    pub const ZERO: Self = /* ... */;

    pub const fn from_translation(v: Vec3) -> Self;
    pub const fn from_scale(v: Vec3) -> Self;
    pub const fn from_rotation_x(angle: f32) -> Self;
    pub const fn from_rotation_y(angle: f32) -> Self;
    pub const fn from_rotation_z(angle: f32) -> Self;
    pub const fn from_quat(q: Quat) -> Self;
    pub fn look_at_rh(eye: Vec3, target: Vec3, up: Vec3) -> Self;
    pub fn perspective_rh(fovy: f32, aspect: f32, near: f32, far: f32) -> Self;
    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self;
    pub fn inverse(&self) -> Option<Self>;
    pub fn transpose(&self) -> Self;
    pub fn mul_vec4(&self, v: Vec4) -> Vec4;
    pub fn to_cols_array(&self) -> [f32; 16];
}
```

### Quat
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub const fn from_rotation_x(angle: f32) -> Self;
    pub const fn from_rotation_y(angle: f32) -> Self;
    pub const fn from_rotation_z(angle: f32) -> Self;
    pub const fn from_euler(euler: Euler) -> Self;
    pub fn to_euler(&self) -> Euler;
    pub fn mul(self, other: Self) -> Self;
    pub fn inverse(&self) -> Self;
    pub fn normalize(&self) -> Self;
    pub fn slerp(self, other: Self, t: f32) -> Self;
    pub fn nlerp(self, other: Self, t: f32) -> Self;
}
```

### Transform
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self;
    pub fn from_translation(v: Vec3) -> Self;
    pub fn matrix(&self) -> Mat4;
    pub fn inverse(&self) -> Self;
}
```

### Rect
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self;
    pub fn contains(&self, point: Vec2) -> bool;
    pub fn intersects(&self, other: &Self) -> bool;
}
```

### AABB
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AABB {
    pub center: Vec3,
    pub half_extents: Vec3,
}

impl AABB {
    pub fn new(center: Vec3, half_extents: Vec3) -> Self;
    pub fn min(&self) -> Vec3;
    pub fn max(&self) -> Vec3;
    pub fn contains(&self, point: Vec3) -> bool;
}
```

## 输入/输出

### Vec2::new(x, y)
- **输入：** x: f32, y: f32
- **输出：** Vec2 实例

### Mat4::inverse(&self)
- **输入：** &Mat4 引用
- **输出：** Option<Mat4>（矩阵可逆时返回 Some）

### Quat::slerp(a, b, t)
- **输入：** a: Quat, b: Quat, t: f32
- **输出：** Quat（球面插值结果）

## 验收标准

- [ ] Vec2/Vec3/Vec4 所有运算符和方法的单元测试通过
- [ ] Mat4 矩阵运算（乘法、求逆、转置）结果正确
- [ ] Quat 与 Euler 互转精度在 1e-6 范围内
- [ ] Transform.matrix() 输出正确的 TRS 矩阵
- [ ] Rect::intersects() 正确检测矩形碰撞
- [ ] AABB::contains() 正确检测点包含
- [ ] lerp/slerp/nlerp 插值函数正确
- [ ] 数学库支持 `no_std + alloc`
- [ ] 所有公开 API 都有 doc comment
- [ ] 单元测试覆盖率 >= 30 条

## 依赖关系

**无外部依赖** — 数学库应保持最小依赖，便于 no_std 环境使用。

**被依赖模块：**
- `engine-core` — 引擎核心使用数学库
- `engine-renderer` — 渲染模块使用矩阵和向量
- `engine-physics` — 物理模块使用向量和变换

## 优先级

**P0（必须）：**
- Vec2/Vec3/Vec4 完整实现
- Mat4 完整实现
- Quat 完整实现
- Transform 完整实现

**P1（重要）：**
- Rect / AABB 几何原语
- lerp/slerp/nlerp 插值

**P2（可选）：**
- OBB 有向包围盒
- Mat2/Mat3 矩阵
