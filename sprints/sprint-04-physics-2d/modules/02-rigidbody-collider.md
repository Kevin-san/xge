# 刚体与碰撞体（RigidBody2D / Collider2D）模块需求

## 模块概述

刚体（RigidBody2D）是物理世界中的基本运动物体，具有质量、位置、旋转、速度等物理属性。碰撞体（Collider2D）定义刚体的几何形状，用于碰撞检测。本模块提供 Dynamic / Static / Kinematic 三种刚体类型，以及圆形、矩形、多边形、胶囊、三角形五种碰撞体形状。

---

## 需求清单

### 1. 刚体类型与 Builder

| 编号 | 需求 | 描述 |
|------|------|------|
| 21 | `RigidBody2D::Dynamic / Static / Kinematic` 三种类型 | 刚体类型枚举 |
| 49 | `RigidBody2DBuilder::new(BodyType)` | 创建 Builder |
| 219 | `BodyType::Dynamic / Static / Kinematic / Sensor` | 四种类型（含 Sensor） |
| 220 | `RigidBody2DBuilder::dynamic() / static() / kinematic() / sensor()` | 类型便捷构造 |
| 230 | `RigidBody2DBuilder::build(&self) -> RigidBody2D` | 构建刚体 |

### 2. 刚体变换属性

| 编号 | 需求 | 描述 |
|------|------|------|
| 50 | `RigidBody2D::translation(&self) -> Vec2` | 获取位置 |
| 51 | `RigidBody2D::set_translation(&mut self, v)` | 设置位置 |
| 52 | `RigidBody2D::rotation(&self) -> f32` | 获取旋转（弧度） |
| 53 | `RigidBody2D::set_rotation(&mut self, rad)` | 设置旋转 |
| 54 | `RigidBody2D::linvel(&self) -> Vec2` | 获取线速度 |
| 55 | `RigidBody2D::set_linvel(&mut self, v)` | 设置线速度 |
| 56 | `RigidBody2D::angvel(&self) -> f32` | 获取角速度 |
| 57 | `RigidBody2D::set_angvel(&mut self, v)` | 设置角速度 |

### 3. 刚体受力 API

| 编号 | 需求 | 描述 |
|------|------|------|
| 58 | `RigidBody2D::apply_force(&mut self, force, point)` | 在指定点施加力 |
| 59 | `RigidBody2D::apply_force_at_center(&mut self, force)` | 在质心施加力 |
| 60 | `RigidBody2D::apply_torque(&mut self, torque)` | 施加扭矩 |
| 61 | `RigidBody2D::apply_impulse(&mut self, impulse, point)` | 在指定点施加冲量 |
| 62 | `RigidBody2D::apply_impulse_at_center(&mut self, impulse)` | 在质心施加冲量 |

### 4. 刚体质量属性

| 编号 | 需求 | 描述 |
|------|------|------|
| 63 | `RigidBody2D::mass(&self) -> f32` | 获取质量 |
| 64 | `RigidBody2D::inertia(&self) -> f32` | 获取转动惯量 |
| 65 | `RigidBody2D::set_mass(&mut self, mass)` | 设置质量 |
| 66 | `RigidBody2D::local_center_of_mass(&self) -> Vec2` | 本地质心位置 |
| 235 | `RigidBody2D::mass_properties(&self) -> MassProperties2D` | 获取质量属性结构 |
| 236 | `MassProperties2D::mass / local_center / inertia` | 质量属性分量 |
| 237 | `Collider2D::mass_properties(&self, density) -> MassProperties2D` | 计算碰撞体质质量属性 |

### 5. 刚体物理参数

| 编号 | 需求 | 描述 |
|------|------|------|
| 67 | `RigidBody2D::set_gravity_scale(&mut self, scale)` | 设置重力缩放 |
| 68 | `RigidBody2D::gravity_scale(&self) -> f32` | 获取重力缩放 |
| 69 | `RigidBody2D::linear_damping(&self) -> f32` | 获取线性阻尼 |
| 70 | `RigidBody2D::set_linear_damping(&mut self, v)` | 设置线性阻尼 |
| 71 | `RigidBody2D::angular_damping(&self) -> f32` | 获取角阻尼 |
| 72 | `RigidBody2D::set_angular_damping(&mut self, v)` | 设置角阻尼 |

### 6. 刚体休眠

| 编号 | 需求 | 描述 |
|------|------|------|
| 73 | `RigidBody2D::can_sleep(&self) -> bool` | 是否允许休眠 |
| 74 | `RigidBody2D::set_can_sleep(&mut self, bool)` | 设置是否允许休眠 |
| 75 | `RigidBody2D::sleeping(&self) -> bool` | 是否处于休眠状态 |
| 76 | `RigidBody2D::wake_up(&mut self)` | 唤醒刚体 |

### 7. 刚体类型查询

| 编号 | 需求 | 描述 |
|------|------|------|
| 77 | `RigidBody2D::type(&self) -> BodyType` | 获取类型 |
| 231 | `RigidBody2D::is_dynamic(&self) -> bool` | 是否为动态 |
| 232 | `RigidBody2D::is_static(&self) -> bool` | 是否为静态 |
| 233 | `RigidBody2D::is_kinematic(&self) -> bool` | 是否为运动学 |
| 234 | `RigidBody2D::ccd_enabled(&self) -> bool` | 是否启用连续碰撞检测 |
| 269 | `RigidBody2D::ccd_enabled(bool)` | Builder 中 CCD 启用配置 |
| 271 | `RigidBody2D::handle(&self) -> BodyHandle` | 获取句柄引用 |

### 8. Builder 配置项

| 编号 | 需求 | 描述 |
|------|------|------|
| 221 | `RigidBody2DBuilder::translation(v)` | 初始位置 |
| 222 | `RigidBody2DBuilder::rotation(rad)` | 初始旋转 |
| 223 | `RigidBody2DBuilder::linvel(v)` | 初始线速度 |
| 224 | `RigidBody2DBuilder::angvel(v)` | 初始角速度 |
| 225 | `RigidBody2DBuilder::gravity_scale(f)` | 重力缩放 |
| 226 | `RigidBody2DBuilder::linear_damping(f)` | 线性阻尼 |
| 227 | `RigidBody2DBuilder::angular_damping(f)` | 角阻尼 |
| 228 | `RigidBody2DBuilder::can_sleep(bool)` | 允许休眠 |
| 229 | `RigidBody2DBuilder::ccd_enabled(bool)` | 启用 CCD |

### 9. 碰撞体类型

| 编号 | 需求 | 描述 |
|------|------|------|
| 51 | `Collider2D` 圆形 | Circle shape |
| 52 | `Collider2DBuilder::circle(radius)` | 圆形构造 |
| 53 | `Collider2DBuilder::rect(w, h)` | 矩形构造 |
| 54 | `Collider2DBuilder::polygon(points)` — 顶点需逆时针 | 多边形构造 |
| 55 | `Collider2DBuilder::capsule(half_h, radius)` | 胶囊构造 |
| 56 | `Collider2DBuilder::triangle(a, b, c)` | 三角形构造 |
| 281 | `Collider2D::is_sensor(&self) -> bool` | 是否为传感器 |

### 10. 碰撞体变换与材质

| 编号 | 需求 | 描述 |
|------|------|------|
| 57 | `Collider2DBuilder::translation(v)` | 本地偏移 |
| 58 | `Collider2DBuilder::rotation(rad)` | 本地旋转 |
| 87 | `Collider2DBuilder::material(PhysicsMaterial)` | 物理材质 |
| 88 | `Collider2DBuilder::density(density)` | 密度 |
| 89 | `Collider2DBuilder::friction(friction)` | 摩擦系数 |
| 90 | `Collider2DBuilder::restitution(restitution)` | 弹性系数 |

### 11. 碰撞分组

| 编号 | 需求 | 描述 |
|------|------|------|
| 91 | `Collider2DBuilder::collision_group(group)` | 碰撞分组 |
| 92 | `Collider2DBuilder::solver_groups(groups)` | 求解器分组 |
| 72 | `CollisionGroup`：bitmask 分组 + 掩码 | 位掩码分组机制 |
| 73 | `CollisionGroup::new(group_bits, mask_bits)` | 构造函数 |
| 74 | `CollisionGroup::with_all()` | 与所有交互 |
| 75 | `CollisionGroup::with_none()` | 不与任何交互 |
| 76 | `CollisionGroup::memberships(&self)` | 获取成员位 |
| 77 | `CollisionGroup::filters(&self)` | 获取过滤位 |
| 78 | `CollisionGroup::can_interact_with(a, b) -> bool` | 判断两分组是否可交互 |
| 282 | `Collider2D::collision_groups(&self) -> CollisionGroup` | 获取碰撞分组 |
| 283 | `Collider2D::solver_groups(&self) -> CollisionGroup` | 获取求解器分组 |

### 12. 碰撞体属性

| 编号 | 需求 | 描述 |
|------|------|------|
| 66 | `Collider2DBuilder::build(&self) -> Collider2D` | 构建碰撞体 |
| 94 | `PhysicsMaterial::default()` | 默认材质 |
| 95 | `PhysicsMaterial::friction / restitution / density` | 材质属性 |
| 238 | `Collider2D::aabb(&self) -> AABB` | 获取轴对齐包围盒 |
| 239 | `Collider2D::mass_properties(&self, density) -> MassProperties2D` | 质量属性 |
| 240 | `Collider2D::handle(&self) -> ColliderHandle` | 获取句柄 |
| 241 | `Collider2D::body(&self) -> Option<BodyHandle>` | 获取所属刚体 |
| 242 | `Collider2D::is_sensor(&self) -> bool` | 是否传感器 |
| 243 | `Collider2D::material(&self) -> &PhysicsMaterial` | 获取材质 |

### 13. Shape Trait

| 编号 | 需求 | 描述 |
|------|------|------|
| 285 | `Shape` trait：`aabb(&self, transform) -> AABB` | 形状接口 |
| 286 | `Circle::aabb(transform)` | 圆形的 AABB |
| 287 | `Rect::aabb(transform)` | 矩形的 AABB |
| 288 | `Polygon::aabb(transform)` | 多边形的 AABB |
| 289 | `Capsule::aabb(transform)` | 胶囊的 AABB |

---

## API 签名

### RigidBody2D & BodyType

```rust
pub enum BodyType {
    Dynamic,
    Static,
    Kinematic,
    Sensor,
}

pub struct RigidBody2D { /* ... */ }

impl RigidBody2D {
    // 变换
    pub fn translation(&self) -> Vec2;
    pub fn set_translation(&mut self, v: Vec2);
    pub fn rotation(&self) -> f32;
    pub fn set_rotation(&mut self, rad: f32);
    pub fn linvel(&self) -> Vec2;
    pub fn set_linvel(&mut self, v: Vec2);
    pub fn angvel(&self) -> f32;
    pub fn set_angvel(&mut self, v: f32);
    
    // 力与冲量
    pub fn apply_force(&mut self, force: Vec2, point: Vec2);
    pub fn apply_force_at_center(&mut self, force: Vec2);
    pub fn apply_torque(&mut self, torque: f32);
    pub fn apply_impulse(&mut self, impulse: Vec2, point: Vec2);
    pub fn apply_impulse_at_center(&mut self, impulse: Vec2);
    
    // 质量
    pub fn mass(&self) -> f32;
    pub fn inertia(&self) -> f32;
    pub fn set_mass(&mut self, mass: f32);
    pub fn local_center_of_mass(&self) -> Vec2;
    pub fn mass_properties(&self) -> MassProperties2D;
    
    // 物理参数
    pub fn gravity_scale(&self) -> f32;
    pub fn set_gravity_scale(&mut self, scale: f32);
    pub fn linear_damping(&self) -> f32;
    pub fn set_linear_damping(&mut self, v: f32);
    pub fn angular_damping(&self) -> f32;
    pub fn set_angular_damping(&mut self, v: f32);
    
    // 休眠
    pub fn can_sleep(&self) -> bool;
    pub fn set_can_sleep(&mut self, can_sleep: bool);
    pub fn sleeping(&self) -> bool;
    pub fn wake_up(&mut self);
    
    // 类型
    pub fn type_(&self) -> BodyType;
    pub fn is_dynamic(&self) -> bool;
    pub fn is_static(&self) -> bool;
    pub fn is_kinematic(&self) -> bool;
    pub fn ccd_enabled(&self) -> bool;
    pub fn handle(&self) -> BodyHandle;
}

pub struct MassProperties2D {
    pub mass: f32,
    pub local_center: Vec2,
    pub inertia: f32,
}
```

### RigidBody2DBuilder

```rust
pub struct RigidBody2DBuilder {
    body_type: BodyType,
    translation: Vec2,
    rotation: f32,
    linvel: Vec2,
    angvel: f32,
    gravity_scale: f32,
    linear_damping: f32,
    angular_damping: f32,
    can_sleep: bool,
    ccd_enabled: bool,
}

impl RigidBody2DBuilder {
    pub fn new(body_type: BodyType) -> Self;
    pub fn dynamic() -> Self;
    pub fn static_() -> Self;
    pub fn kinematic() -> Self;
    pub fn sensor() -> Self;
    
    pub fn translation(mut self, v: Vec2) -> Self;
    pub fn rotation(mut self, rad: f32) -> Self;
    pub fn linvel(mut self, v: Vec2) -> Self;
    pub fn angvel(mut self, v: f32) -> Self;
    pub fn gravity_scale(mut self, f: f32) -> Self;
    pub fn linear_damping(mut self, f: f32) -> Self;
    pub fn angular_damping(mut self, f: f32) -> Self;
    pub fn can_sleep(mut self, bool: bool) -> Self;
    pub fn ccd_enabled(mut self, bool: bool) -> Self;
    
    pub fn build(self) -> RigidBody2D;
}
```

### Collider2D & ColliderBuilder

```rust
pub enum ColliderShape {
    Circle { radius: f32 },
    Rect { half_w: f32, half_h: f32 },
    Polygon { points: Vec<Vec2> },  // 逆时针
    Capsule { half_height: f32, radius: f32 },
    Triangle { a: Vec2, b: Vec2, c: Vec2 },
}

pub struct Collider2D { /* ... */ }

impl Collider2D {
    pub fn aabb(&self) -> AABB;
    pub fn mass_properties(&self, density: f32) -> MassProperties2D;
    pub fn handle(&self) -> ColliderHandle;
    pub fn body(&self) -> Option<BodyHandle>;
    pub fn is_sensor(&self) -> bool;
    pub fn material(&self) -> &PhysicsMaterial;
    pub fn collision_groups(&self) -> CollisionGroup;
    pub fn solver_groups(&self) -> CollisionGroup;
}

pub struct ColliderBuilder {
    shape: ColliderShape,
    translation: Vec2,
    rotation: f32,
    sensor: bool,
    material: PhysicsMaterial,
    density: f32,
    collision_group: CollisionGroup,
    solver_group: CollisionGroup,
}

impl ColliderBuilder {
    pub fn circle(radius: f32) -> Self;
    pub fn rect(w: f32, h: f32) -> Self;
    pub fn polygon(points: Vec<Vec2>) -> Self;  // 需逆时针
    pub fn capsule(half_h: f32, radius: f32) -> Self;
    pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> Self;
    
    pub fn translation(mut self, v: Vec2) -> Self;
    pub fn rotation(mut self, rad: f32) -> Self;
    pub fn sensor(mut self, bool: bool) -> Self;
    pub fn material(mut self, m: PhysicsMaterial) -> Self;
    pub fn density(mut self, d: f32) -> Self;
    pub fn friction(mut self, f: f32) -> Self;
    pub fn restitution(mut self, r: f32) -> Self;
    pub fn collision_group(mut self, g: CollisionGroup) -> Self;
    pub fn solver_groups(mut self, g: CollisionGroup) -> Self;
    
    pub fn build(self) -> Collider2D;
}
```

### PhysicsMaterial & CollisionGroup

```rust
pub struct PhysicsMaterial {
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
}

impl PhysicsMaterial {
    pub fn default() -> Self;
}

pub struct CollisionGroup {
    memberships: u32,
    filters: u32,
}

impl CollisionGroup {
    pub fn new(memberships: u32, filters: u32) -> Self;
    pub fn with_all() -> Self;
    pub fn with_none() -> Self;
    pub fn memberships(&self) -> u32;
    pub fn filters(&self) -> u32;
    pub fn can_interact_with(a: CollisionGroup, b: CollisionGroup) -> bool;
}
```

---

## 输入/输出

### 输入
- 刚体类型（Dynamic/Static/Kinematic/Sensor）
- 变换参数（位置、旋转）
- 速度参数（线速度、角速度）
- 质量参数（质量、密度、重力缩放）
- 阻尼参数（线性阻尼、角阻尼）
- 碰撞体几何形状与材质

### 输出
- 仿真后的位置、旋转更新
- 速度变化
- 碰撞检测结果

---

## 验收标准

1. ✅ `RigidBody2DBuilder::dynamic().build()` 可创建动态刚体
2. ✅ 动态刚体在重力 (0, -9.81) 作用下正确下落
3. ✅ `apply_impulse_at_center` 正确改变刚体速度
4. ✅ `apply_force` 与 `apply_force_at_center` 在不同作用点产生不同旋转效果
5. ✅ `set_mass` 后 `mass()` 返回正确值
6. ✅ `linear_damping` 影响刚体速度衰减
7. ✅ 静态刚体不受重力影响，位置固定
8. ✅ `circle(radius).build()` 生成的碰撞体 `aabb()` 正确
9. ✅ `polygon` 顶点顺序不影响 AABB 计算
10. ✅ `sensor(true)` 的碰撞体不产生物理响应
11. ✅ `CollisionGroup::can_interact_with` 正确过滤碰撞
12. ✅ 单元测试：RigidBody2DBuilder 构建正常
13. ✅ 单元测试：重力下球体下落符合物理预期
14. ✅ 单元测试：两个圆碰撞反弹速度守恒
15. ✅ 单元测试：Circle vs AABB 碰撞点正确

---

## 依赖关系

- 依赖 `engine-physics-2d` crate（World2D）
- 依赖 `math` crate（Vec2、Mat3、AABB）
- 被 `Body2DNode` 封装使用
- 所有物理示例依赖本模块

---

## 优先级

| 优先级 | 含义 | 需求编号 |
|--------|------|----------|
| P0 | 核心功能 | 21, 49-77, 94-95, 219-234, 238-243, 269-271 |
| P1 | 重要功能 | 58-62, 67-72, 78, 88-93, 235-237, 281-289 |
| P2 | 增强功能 | 73-76, 229, 266-268 |
