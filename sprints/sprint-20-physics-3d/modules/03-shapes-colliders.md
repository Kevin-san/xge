# Module 03 — 形状与碰撞体

> 上游 sprint: [Sprint 20](../sprint-20-physics-3d.md)
> 文件位置: `engine-physics-3d/src/shape/mod.rs`

## 1. Shape Trait

```rust
pub trait Shape: Send + Sync {
    /// 计算 AABB（世界空间）
    fn compute_aabb(&self, transform: &Transform) -> AABB;
    
    /// 局部空间体积
    fn volume(&self) -> f32;
    
    /// 局部空间质心
    fn center_of_mass(&self) -> Vec3;
    
    /// 局部空间惯性张量
    fn inertia_tensor(&self, mass: f32) -> Mat3;
    
    /// GJK support function
    fn support_point(&self, direction: Vec3) -> Vec3;
    
    /// 类型 ID
    fn shape_type(&self) -> ShapeType;
}

pub enum ShapeType {
    Box,
    Sphere,
    Capsule,
    Cylinder,
    ConvexHull,
    Trimesh,
}
```

## 2. 基础形状

```rust
#[derive(Debug, Clone, Copy)]
pub struct BoxShape { pub half_extents: Vec3 }

impl Shape for BoxShape {
    fn support_point(&self, dir: Vec3) -> Vec3 {
        Vec3::new(
            if dir.x > 0.0 { self.half_extents.x } else { -self.half_extents.x },
            if dir.y > 0.0 { self.half_extents.y } else { -self.half_extents.y },
            if dir.z > 0.0 { self.half_extents.z } else { -self.half_extents.z },
        )
    }
    
    fn inertia_tensor(&self, mass: f32) -> Mat3 {
        let h = self.half_extents;
        let k = mass / 3.0;
        Mat3::from_diagonal(Vec3::new(
            k * (h.y * h.y + h.z * h.z),
            k * (h.x * h.x + h.z * h.z),
            k * (h.x * h.x + h.y * h.y),
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereShape { pub radius: f32 }

#[derive(Debug, Clone, Copy)]
pub struct CapsuleShape { pub half_height: f32, pub radius: f32 }

#[derive(Debug, Clone, Copy)]
pub struct CylinderShape { pub half_height: f32, pub radius: f32 }
```

## 3. Convex Hull

```rust
#[derive(Debug, Clone)]
pub struct ConvexHullShape {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<[u32; 3]>,  // 三角面
    pub face_normals: Vec<Vec3>,
}

impl ConvexHullShape {
    /// QuickHull 算法
    pub fn from_points(points: &[Vec3]) -> Self;
    
    /// 验证凸性
    pub fn is_convex(&self) -> bool;
}

impl Shape for ConvexHullShape {
    fn support_point(&self, dir: Vec3) -> Vec3 {
        // 找点积最大顶点
        let mut best = self.vertices[0];
        let mut best_dot = best.dot(dir);
        for &v in &self.vertices[1..] {
            let d = v.dot(dir);
            if d > best_dot {
                best = v;
                best_dot = d;
            }
        }
        best
    }
}
```

## 4. Trimesh（仅 kinematic）

```rust
#[derive(Debug, Clone)]
pub struct TrimeshShape {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub bvh: TriangleBvh,  // 三角形 BVH
}

impl TrimeshShape {
    pub fn from_mesh(mesh: &Mesh3D) -> Self;
}

impl Shape for TrimeshShape {
    fn support_point(&self, dir: Vec3) -> Vec3 {
        // Trimesh 是非凸的，support point 无意义
        // 但可返回 AABB
        self.aabb.farthest_in_direction(dir)
    }
}
```

## 5. Collider

```rust
pub struct Collider {
    pub shape: Box<dyn Shape>,
    pub local_transform: Transform,
    pub material: PhysicsMaterial,
    pub is_trigger: bool,
    pub collision_group: CollisionGroup,
}

pub struct PhysicsMaterial {
    pub friction: f32,        // 0..1
    pub restitution: f32,     // 0..1
    pub density: f32,         // kg/m³
}
```

## 6. 验收

- [ ] 凸包 QuickHull：256 点输入 < 1 ms
- [ ] Trimesh 仅 kinematic 物体使用
- [ ] 形状内存布局：SIMD 友好
- [ ] Inertia tensor 物理精度 < 1%
- [ ] 100 形状 AABB 重算 < 100 µs
