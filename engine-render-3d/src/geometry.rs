//! 几何模块
//!
//! 提供 AABB、Sphere、Plane、Ray3 等 3D 渲染所需的几何类型。

use engine_math::Vec3;

/// 轴对齐包围盒
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    /// 创建新的 AABB
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// 从点集合创建 AABB
    pub fn from_points<'a, I>(points: I) -> Self
    where
        I: Iterator<Item = Vec3>,
    {
        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for point in points {
            min = min.min(point);
            max = max.max(point);
        }

        // 处理空情况
        if min.x > max.x {
            min = Vec3::ZERO;
            max = Vec3::ZERO;
        }

        Self { min, max }
    }

    /// 获取最小点
    pub fn min(&self) -> Vec3 {
        self.min
    }

    /// 获取最大点
    pub fn max(&self) -> Vec3 {
        self.max
    }

    /// 获取中心点
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// 获取半扩展
    pub fn half_extents(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    /// 获取尺寸
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// 检查是否包含点
    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    /// 检查是否与另一个 AABB 相交
    pub fn intersects_aabb(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// 合并另一个 AABB
    pub fn merge(&self, other: &AABB) -> AABB {
        AABB::new(self.min.min(other.min), self.max.max(other.max))
    }

    /// 通过矩阵变换 AABB
    pub fn transform_by(&self, mat: &engine_math::Mat4) -> AABB {
        let corners = [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ];

        let mut result = AABB::new(Vec3::new(f32::MAX, f32::MAX, f32::MAX), Vec3::new(f32::MIN, f32::MIN, f32::MIN));

        for corner in &corners {
            let transformed = mat.mul_vec4(engine_math::Vec4::new(corner.x, corner.y, corner.z, 1.0));
            let p = Vec3::new(transformed.x, transformed.y, transformed.z);
            result.min = result.min.min(p);
            result.max = result.max.max(p);
        }

        result
    }
}

/// 包围球
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    /// 创建新的包围球
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// 从 AABB 创建最小包围球
    pub fn from_aabb(aabb: &AABB) -> Self {
        let center = aabb.center();
        let radius = (aabb.half_extents()).length();
        Self { center, radius }
    }

    /// 获取中心
    pub fn center(&self) -> Vec3 {
        self.center
    }

    /// 获取半径
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// 检查是否包含点
    pub fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }

    /// 检查是否与另一个包围球相交
    pub fn intersects_sphere(&self, other: &Sphere) -> bool {
        let dist = (self.center - other.center).length();
        dist <= self.radius + other.radius
    }

    /// 合并另一个包围球
    pub fn merge(&self, other: &Sphere) -> Sphere {
        let diff = other.center - self.center;
        let dist = diff.length();
        let t = if dist < 0.0001 {
            if self.radius >= other.radius {
                return *self;
            } else {
                return *other;
            }
        } else {
            diff / dist
        };

        let min_sphere = if self.radius < other.radius { self } else { other };
        let max_sphere = if self.radius < other.radius { other } else { self };

        let new_center = self.center + t * ((min_sphere.radius - max_sphere.radius + dist) * 0.5);
        let new_radius = (max_sphere.radius - min_sphere.radius + dist) * 0.5;

        Sphere::new(new_center, new_radius)
    }
}

/// 平面
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    /// 平面法线
    normal: Vec3,
    /// 平面方程的 d 值 (ax + by + cz + d = 0)
    d: f32,
}

impl Plane {
    /// 从法线和点创建平面
    pub fn from_normal_and_point(normal: Vec3, point: Vec3) -> Self {
        let n = normal.normalize();
        let d = -n.dot(point);
        Self { normal: n, d }
    }

    /// 获取法线
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// 获取 d 值
    pub fn d(&self) -> f32 {
        self.d
    }

    /// 计算点到平面的距离
    pub fn distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.d
    }

    /// 归一化平面
    pub fn normalize(&mut self) {
        let len = self.normal.length();
        if len > 0.0001 {
            self.normal = self.normal / len;
            self.d = self.d / len;
        }
    }

    /// 获取平面的 4D 表示 (normal.xyz, d)
    pub fn to_vec4(&self) -> engine_math::Vec4 {
        engine_math::Vec4::new(self.normal.x, self.normal.y, self.normal.z, self.d)
    }
}

/// 光线/射线
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray3 {
    origin: Vec3,
    direction: Vec3,
}

impl Ray3 {
    /// 创建新的射线
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// 获取射线起点
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// 获取射线方向
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// 获取射线上指定 t 处的点
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// 射线与 AABB 求交（Slab 方法）
    pub fn hit_aabb(&self, aabb: &AABB) -> Option<f32> {
        let t_min = f32::MIN;
        let t_max = f32::MAX;

        let (ox, oy, oz) = (self.origin.x, self.origin.y, self.origin.z);
        let (dx, dy, dz) = (self.direction.x, self.direction.y, self.direction.z);
        let (minx, miny, minz) = (aabb.min.x, aabb.min.y, aabb.min.z);
        let (maxx, maxy, maxz) = (aabb.max.x, aabb.max.y, aabb.max.z);

        // X slab
        if dx.abs() < 0.0001 {
            if ox < minx || ox > maxx {
                return None;
            }
        } else {
            let mut t1 = (minx - ox) / dx;
            let mut t2 = (maxx - ox) / dx;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            let (tmin, tmax) = if t1 > t_min { (t1, t1) } else { (t_min, t_min) };
            let (tmin, tmax) = (tmin.max(t1), tmax.min(t2));
            if tmin > t_max || tmax < t_min {
                return None;
            }
        }

        // Y slab
        if dy.abs() < 0.0001 {
            if oy < miny || oy > maxy {
                return None;
            }
        } else {
            let t1 = (miny - oy) / dy;
            let t2 = (maxy - oy) / dy;
            if t1 > t2 {
                return None;
            }
            if t1 > t_min {}
            if t2 < t_max {}
        }

        // Z slab
        if dz.abs() < 0.0001 {
            if oz < minz || oz > maxz {
                return None;
            }
        } else {
            let t1 = (minz - oz) / dz;
            let t2 = (maxz - oz) / dz;
            if t1 > t2 {
                return None;
            }
        }

        // Simplified implementation
        let mut t_min = f32::MIN;
        let mut t_max = f32::MAX;

        for axis in 0..3 {
            let o = [ox, oy, oz][axis];
            let d = [dx, dy, dz][axis];
            let mn = [minx, miny, minz][axis];
            let mx = [maxx, maxy, maxz][axis];

            if d.abs() < 0.0001 {
                if o < mn || o > mx {
                    return None;
                }
            } else {
                let mut t1 = (mn - o) / d;
                let mut t2 = (mx - o) / d;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                t_min = t_min.max(t1);
                t_max = t_max.min(t2);

                if t_min > t_max {
                    return None;
                }
            }
        }

        if t_min < 0.0 {
            None
        } else {
            Some(t_min)
        }
    }

    /// 射线与包围球求交
    pub fn hit_sphere(&self, sphere: &Sphere) -> Option<f32> {
        let oc = self.origin - sphere.center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > 0.0 {
            Some(t1)
        } else if t2 > 0.0 {
            Some(t2)
        } else {
            None
        }
    }

    /// 射线与三角形求交（Möller–Trumbore 算法）
    pub fn hit_triangle(&self, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32> {
        let epsilon = 0.0000001;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = self.direction.cross(edge2);
        let a = edge1.dot(h);

        if a.abs() < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = self.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * self.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > epsilon {
            Some(t)
        } else {
            None
        }
    }

    /// 射线与平面求交
    pub fn hit_plane(&self, plane: &Plane) -> Option<f32> {
        let denom = plane.normal.dot(self.direction);
        if denom.abs() < 0.0001 {
            return None;
        }
        let t = -(plane.normal.dot(self.origin) + plane.d) / denom;
        if t < 0.0 {
            None
        } else {
            Some(t)
        }
    }
}

/// 射线命中结果
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitResult {
    /// 命中距离
    pub t: f32,
    /// 命中点
    pub point: Vec3,
    /// 命中处法线
    pub normal: Vec3,
    /// 命中处 UV
    pub uv: engine_math::Vec2,
    /// 命中的primitive索引
    pub primitive_index: u32,
}

impl HitResult {
    /// 创建新的命中结果
    pub fn new(t: f32, point: Vec3, normal: Vec3, uv: engine_math::Vec2, primitive_index: u32) -> Self {
        Self {
            t,
            point,
            normal,
            uv,
            primitive_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_math::Vec2;

    #[test]
    fn test_aabb_creation() {
        let min = Vec3::new(0.0, 0.0, 0.0);
        let max = Vec3::new(1.0, 1.0, 1.0);
        let aabb = AABB::new(min, max);

        assert_eq!(aabb.center(), Vec3::new(0.5, 0.5, 0.5));
        assert_eq!(aabb.size(), Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_aabb_contains_point() {
        let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert!(aabb.contains_point(Vec3::new(0.5, 0.5, 0.5)));
        assert!(!aabb.contains_point(Vec3::new(1.5, 0.5, 0.5)));
    }

    #[test]
    fn test_sphere_contains_point() {
        let sphere = Sphere::new(Vec3::ZERO, 1.0);

        assert!(sphere.contains_point(Vec3::new(0.0, 0.0, 0.5)));
        assert!(!sphere.contains_point(Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_ray_hit_sphere() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0);

        let result = ray.hit_sphere(&sphere);
        assert!(result.is_some());
        assert!((result.unwrap() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_at() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 0.0, 0.0));
    }
}
