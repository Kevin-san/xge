# Module 03 — Quat 高级插值（Squad / Swing-Twist / Dual Quat）

> 上游 sprint: [Sprint 17](../sprint-17-math-simd.md)
> 文件位置: `engine-math/src/quat.rs`, `engine-math/src/dual_quat.rs`

---

## 1. 目标

扩展 `Quat` 与新增 `DualQuat`，覆盖：
- 欧拉角 / 轴角 双向转换
- squad 球面四边形插值
- swing-twist 分解（角色骨骼）
- Dual Quat 线性蒙皮（DLB）

## 2. Quat 扩展 API

```rust
impl Quat {
    // 轴角
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self;
    pub fn to_axis_angle(self) -> (Vec3, f32);
    
    // 欧拉角
    pub fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Self;  // ZYX
    pub fn to_euler(self) -> (f32, f32, f32);
    
    // squad
    pub fn squad(q0: Self, q1: Self, q2: Self, t: f32) -> Self;
    
    // swing-twist 分解
    pub fn swing_twist(self, twist_axis: Vec3) -> (Quat /*swing*/, Quat /*twist*/);
    
    // 微分
    pub fn log(self) -> Vec3;  // 四元数对数（虚部 = 轴角向量）
    pub fn exp(v: Vec3) -> Self;  // 四元数指数
}
```

## 3. Squad 算法

```rust
/// 球面四边形插值（squad）
/// q0, q1, q2 是三个连续关键帧；t ∈ [0, 1]
pub fn squad(q0: Self, q1: Self, q2: Self, t: f32) -> Self {
    // 中间控制点（slerp(q0, q2)）
    let ctrl = Self::slerp(q0, q2, 0.5);
    // 二次 slerp
    let a = Self::slerp(q0, q1, t);
    let b = Self::slerp(q1, q2, t);
    Self::slerp(a, b, 2.0 * t * (1.0 - t))
}
```

## 4. Swing-Twist 分解

```rust
/// 绕 twist_axis 分解
/// 返回 (swing, twist) 使得 q = swing * twist
pub fn swing_twist(self, twist_axis: Vec3) -> (Self, Self) {
    // twist 分量 = twist_axis 上的投影
    let proj = self.x * twist_axis.x + self.y * twist_axis.y + self.z * twist_axis.z;
    let twist = Self {
        x: twist_axis.x * proj,
        y: twist_axis.y * proj,
        z: twist_axis.z * proj,
        w: self.w,
    }.normalize();
    
    // swing = q * twist^-1
    let swing = self * twist.inverse();
    (swing, twist)
}
```

## 5. Dual Quat

```rust
// engine-math/src/dual_quat.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DualQuat {
    pub real: Quat,
    pub dual: Quat,
}

impl DualQuat {
    pub const IDENTITY: Self = Self {
        real: Quat::IDENTITY,
        dual: Quat::ZERO,
    };
    
    pub fn from_translation(t: Vec3) -> Self {
        // dual = 0.5 * (0, tx, ty, tz) * real
        let real = Quat::IDENTITY;
        let dual = Quat {
            x: 0.5 * t.x,
            y: 0.5 * t.y,
            z: 0.5 * t.z,
            w: 0.0,
        } * real;
        Self { real, dual }
    }
    
    pub fn from_rotation_translation(rot: Quat, trans: Vec3) -> Self {
        let real = rot;
        let t_quat = Quat {
            x: trans.x,
            y: trans.y,
            z: trans.z,
            w: 0.0,
        };
        let dual = (t_quat * rot) * 0.5;
        Self { real, dual }
    }
    
    pub fn to_mat4(self) -> Mat4 {
        // 4x4 矩阵展开（与普通 quat 转换相似）
        let r = self.real;
        let d = self.dual;
        // ... 复杂展开
    }
    
    /// 最短路径线性混合插值（Dual Quat Linear Blending, DLB）
    pub fn sclerp(self, other: Self, t: f32) -> Self {
        // dot < 0 时翻转避免长路径
        let dot = self.real.dot(other.real);
        let (r0, r1) = if dot < 0.0 {
            (-self.real, -other.real)
        } else {
            (self.real, other.real)
        };
        let real = r0.slerp(r1, t);
        let dual = self.dual.lerp(other.dual, t);
        // 归一化
        let norm = real.dot(real).sqrt();
        Self { real: real / norm, dual: dual / norm }
    }
}
```

## 6. 验收

- [ ] `squad` 与 Unity 动画曲线插值误差 < 0.01°
- [ ] `swing_twist_decompose` 还原测试：`swing * twist` == 原 quat
- [ ] `DualQuat::to_mat4` 与 LBS 4x4 矩阵差异 < 1%
- [ ] 单元测试 100% 路径覆盖
- [ ] 文档：每个函数说明含图示

## 7. 性能预算

| 操作 | 目标 |
|------|------|
| `from_axis_angle` | < 5 ns |
| `squad` | < 20 ns |
| `swing_twist` | < 10 ns |
| `DualQuat::sclerp` | < 30 ns |
| `DualQuat::to_mat4` | < 50 ns |
