# Module 04 — 窄相碰撞 + CCD

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/narrowphase/`

## 1. 目标

**修复 `engine-physics-2d/src/world.rs#L250-L268` generate_contact 空实现。**

实现 GJK + EPA + SAT + CCD 完整窄相。

## 2. GJK（Gilbert-Johnson-Keerthi）

```rust
pub struct GjkResult {
    pub intersects: bool,
    pub simplex: Simplex,
}

pub fn gjk_intersect(
    a: &dyn Shape,
    b: &dyn Shape,
    transform_a: &Transform,
    transform_b: &Transform,
) -> GjkResult {
    // 1. 计算 Minkowski 差支持函数
    let support = |dir: Vec3| -> Vec3 {
        let pa = a.support_point(transform_a.inverse_transform_direction(dir));
        let pb = b.support_point(transform_b.inverse_transform_direction(-dir));
        pa - pb
    };
    
    // 2. 初始化 simplex
    let mut simplex = Simplex::new();
    let d = support(Vec3::X);  // 初始方向
    simplex.push(d);
    
    // 3. 迭代
    for _ in 0..100 {
        let d = simplex.closest_point_to_origin();
        if d.length() < 1e-6 { return GjkResult { intersects: true, simplex }; }
        let p = support(d);
        if p.dot(d) <= 0.0 { return GjkResult { intersects: false, simplex }; }
        simplex.push(p);
        if simplex.do_simplex() { return GjkResult { intersects: true, simplex }; }
    }
    
    GjkResult { intersects: true, simplex }
}
```

## 3. Simplex（2D / 3D）

```rust
pub struct Simplex {
    pub points: SmallVec<[Vec3; 4]>,  // 最多 4 个点
}

impl Simplex {
    pub fn do_simplex(&mut self) -> bool {
        match self.points.len() {
            2 => self.do_line(),
            3 => self.do_triangle(),
            4 => self.do_tetrahedron(),
            _ => false,
        }
    }
    
    // 包含原点的判定
    fn do_line(&mut self) -> bool { /* ... */ }
    fn do_triangle(&mut self) -> bool { /* ... */ }
    fn do_tetrahedron(&mut self) -> bool { /* ... */ }
    
    pub fn closest_point_to_origin(&self) -> Vec3;
}
```

## 4. EPA（Expanding Polytope Algorithm）

```rust
pub struct EpaResult {
    pub penetration: f32,
    pub normal: Vec3,
    pub contact: Vec3,
}

pub fn epa(
    simplex: Simplex,
    a: &dyn Shape,
    b: &dyn Shape,
    transform_a: &Transform,
    transform_b: &Transform,
) -> EpaResult {
    // 1. 初始化 polytope
    let mut polytope: Vec<Face> = simplex.to_faces();
    
    // 2. 迭代扩展
    for _ in 0..100 {
        // 找最近面
        let closest = polytope.iter().min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap()).unwrap();
        let support_dir = closest.normal;
        let support_point = a.support_point(/* ... */) - b.support_point(/* ... */);
        let d = support_point.dot(support_dir);
        
        if (d - closest.distance).abs() < 1e-6 {
            return EpaResult {
                penetration: d,
                normal: closest.normal,
                contact: support_point,
            };
        }
        
        // 扩展 polytope
        polytope.expand(support_point);
    }
    
    panic!("EPA didn't converge");
}
```

## 5. SAT（OBB-OBB 备用）

```rust
pub fn obb_vs_obb(a: &Transform, ha: Vec3, b: &Transform, hb: Vec3) -> Option<(Vec3, f32)> {
    // 15 个分离轴
    let mut min_overlap = f32::MAX;
    let mut min_axis = Vec3::ZERO;
    for axis in 0..15 {
        let sep_axis = compute_separating_axis(a, b, axis);
        let (overlap, _) = project_obb(sep_axis, a, ha, b, hb);
        if overlap < 0.0 { return None; }  // 分离
        if overlap < min_overlap {
            min_overlap = overlap;
            min_axis = sep_axis;
        }
    }
    Some((min_axis, min_overlap))
}
```

## 6. CCD（连续碰撞检测）

### 6.1 球体：保守前进（Conservative Advancement）

```rust
pub fn ccd_sphere_sphere(
    pos_a: Vec3, prev_pos_a: Vec3, radius_a: f32,
    pos_b: Vec3, prev_pos_b: Vec3, radius_b: f32,
) -> Option<f32> {
    let motion_a = pos_a - prev_pos_a;
    let motion_b = pos_b - prev_pos_b;
    let rel_motion = motion_a - motion_b;
    
    // 球心距离 + 半径 = 接触距离
    let to_b = pos_b - pos_a;
    let target_dist_sq = (radius_a + radius_b).powi(2);
    
    // 投影相对运动到连线
    let motion_along = rel_motion.dot(to_b.normalize());
    if motion_along >= 0.0 { return None; }  // 远离
    
    // 找到时间 t 使球心距离 = radius_a + radius_b
    // |to_b + t * rel_motion|² = target_dist²
    // 二次方程
    let a = rel_motion.length_squared();
    let b = 2.0 * to_b.dot(rel_motion);
    let c = to_b.length_squared() - target_dist;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 { return None; }
    
    let t = (-b - disc.sqrt()) / (2.0 * a);
    if t < 0.0 || t > 1.0 { return None; }
    Some(t)
}
```

### 6.2 凸体：Tobii Raycast

```rust
pub fn ccd_convex_convex(
    shape_a: &dyn Shape,
    transform_a: &Transform,
    prev_transform_a: &Transform,
    shape_b: &dyn Shape,
    transform_b: &Transform,
    prev_transform_b: &Transform,
) -> Option<f32> {
    // 迭代：保守前进到新位置
    let mut t = 0.0;
    let mut current_a = *prev_transform_a;
    let mut current_b = *prev_transform_b;
    let delta_a = transform_a.compose(&prev_transform_a.inverse());
    let delta_b = transform_b.compose(&prev_transform_b.inverse());
    
    for _ in 0..20 {
        // 测试当前姿态是否接触
        if gjk_intersect(shape_a, shape_b, &current_a, &current_b).intersects {
            return Some(t);
        }
        
        // 计算最近点
        let closest = epa_closest(/* ... */);
        // 估计运动量
        let t_step = closest.distance / 1.0;  // 简化
        t = (t + t_step).min(1.0);
        current_a = prev_transform_a.interpolate(transform_a, t);
        current_b = prev_transform_b.interpolate(transform_b, t);
    }
    None
}
```

## 7. Contact Manifold

```rust
pub struct ContactManifold {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub normal: Vec3,           // a → b
    pub points: SmallVec<[ContactPoint; 4]>,  // 最多 4 个接触点
    pub friction: f32,
    pub restitution: f32,
}

pub struct ContactPoint {
    pub local_a: Vec3,
    pub local_b: Vec3,
    pub world_position: Vec3,
    pub penetration: f32,
    pub normal_impulse: f32,
    pub tangent_impulse: [f32; 2],
    pub warm_start: [f32; 3],  // (normal, tangent_x, tangent_y)
}
```

## 8. 验收

- [ ] 100 接触对 GJK/EPA < 0.5 ms
- [ ] CCD 1mm 薄墙穿透率 < 0.01%
- [ ] 接触点稳定：4 frame 持久率 > 95%
- [ ] GJK 100 迭代上限 + 退化检测
- [ ] EPA 100 迭代上限
- [ ] Tobii CCD 凸体角速度测试
