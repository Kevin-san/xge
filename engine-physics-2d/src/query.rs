//! 空间查询模块
//!
//! 提供射线投射、形状查询、点查询等空间查询功能。

use crate::ColliderShape;
use engine_math::{Rect, Vec2};

/// 变换（位置 + 旋转）
pub type Transform2D = (Vec2, f32);

/// 射线投射
#[derive(Debug, Clone)]
pub struct RayCast2D {
    /// 起点
    pub origin: Vec2,
    /// 方向（归一化）
    pub direction: Vec2,
    /// 最大距离
    pub max_distance: f32,
    /// 碰撞组过滤
    pub collision_group: u32,
    /// 碰撞掩码
    pub collision_mask: u32,
}

impl RayCast2D {
    /// 创建新的射线投射
    pub fn new(origin: Vec2, direction: Vec2, max_distance: f32) -> Self {
        let direction = if direction.length() > 0.0 {
            direction.normalize()
        } else {
            Vec2::ZERO
        };
        Self {
            origin,
            direction,
            max_distance,
            collision_group: 0xFFFFFFFF,
            collision_mask: 0xFFFFFFFF,
        }
    }

    /// 设置碰撞组
    pub fn with_collision_group(mut self, group: u32) -> Self {
        self.collision_group = group;
        self
    }

    /// 设置碰撞掩码
    pub fn with_collision_mask(mut self, mask: u32) -> Self {
        self.collision_mask = mask;
        self
    }

    /// 计算射线上某点的位置
    pub fn point_at(&self, t: f32) -> Vec2 {
        self.origin + self.direction * t
    }

    /// 计算射线的终点
    pub fn endpoint(&self) -> Vec2 {
        self.point_at(self.max_distance)
    }
}

/// 射线投射命中结果
#[derive(Debug, Clone)]
pub struct RayCastHit2D {
    /// 命中点
    pub point: Vec2,
    /// 法线
    pub normal: Vec2,
    /// 命中间隔 [0, 1]
    pub time: f32,
    /// 命中的碰撞体
    pub collider: u64,
    /// 命中面的索引
    pub face_index: usize,
}

/// 形状投射
#[derive(Debug, Clone)]
pub struct ShapeCast2D {
    /// 形状
    pub shape: ColliderShape,
    /// 变换（位置，旋转弧度）
    pub transform: Transform2D,
    /// 平移
    pub translation: Vec2,
    /// 最大距离
    pub max_distance: f32,
    /// 碰撞组过滤
    pub collision_group: u32,
    /// 碰撞掩码
    pub collision_mask: u32,
}

impl ShapeCast2D {
    /// 创建新的形状投射
    pub fn new(
        shape: ColliderShape,
        transform: Transform2D,
        translation: Vec2,
        max_distance: f32,
    ) -> Self {
        Self {
            shape,
            transform,
            translation,
            max_distance,
            collision_group: 0xFFFFFFFF,
            collision_mask: 0xFFFFFFFF,
        }
    }

    /// 设置碰撞组
    pub fn with_collision_group(mut self, group: u32) -> Self {
        self.collision_group = group;
        self
    }

    /// 设置碰撞掩码
    pub fn with_collision_mask(mut self, mask: u32) -> Self {
        self.collision_mask = mask;
        self
    }
}

/// 点查询信息
#[derive(Debug, Clone)]
pub struct PointQuery {
    /// 查询点
    pub position: Vec2,
    /// 碰撞组
    pub collision_group: u32,
    /// 碰撞掩码
    pub collision_mask: u32,
}

impl PointQuery {
    /// 创建新的点查询
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            collision_group: 0xFFFFFFFF,
            collision_mask: 0xFFFFFFFF,
        }
    }

    /// 设置碰撞组
    pub fn with_collision_group(mut self, group: u32) -> Self {
        self.collision_group = group;
        self
    }

    /// 设置碰撞掩码
    pub fn with_collision_mask(mut self, mask: u32) -> Self {
        self.collision_mask = mask;
        self
    }
}

/// 点查询命中结果
#[derive(Debug, Clone)]
pub struct PointQueryHit {
    /// 命中点
    pub point: Vec2,
    /// 最近的点
    pub closest_point: Vec2,
    /// 距离
    pub distance: f32,
    /// 命中的碰撞体
    pub collider: u64,
}

/// AABB 查询
#[derive(Debug, Clone)]
pub struct AabbQuery {
    /// AABB 范围
    pub bounds: Rect,
    /// 碰撞组
    pub collision_group: u32,
    /// 碰撞掩码
    pub collision_mask: u32,
}

impl AabbQuery {
    /// 创建新的 AABB 查询
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            collision_group: 0xFFFFFFFF,
            collision_mask: 0xFFFFFFFF,
        }
    }

    /// 设置碰撞组
    pub fn with_collision_group(mut self, group: u32) -> Self {
        self.collision_group = group;
        self
    }

    /// 设置碰撞掩码
    pub fn with_collision_mask(mut self, mask: u32) -> Self {
        self.collision_mask = mask;
        self
    }
}

#[allow(dead_code)]
pub(crate) fn ray_intersects_shape(
    ray: &RayCast2D,
    position: Vec2,
    rotation: f32,
    shape: &ColliderShape,
) -> Option<RayCastHit2D> {
    match shape {
        ColliderShape::Circle { radius } => ray_intersects_circle(ray, position, *radius),
        ColliderShape::Aabb { half_extents } => ray_intersects_aabb(ray, position, *half_extents),
        ColliderShape::Rectangle { half_extents } => {
            ray_intersects_rectangle(ray, position, rotation, *half_extents)
        }
        ColliderShape::Polygon { vertices } => {
            ray_intersects_polygon(ray, position, rotation, vertices)
        }
        ColliderShape::Capsule {
            top,
            bottom,
            radius,
        } => ray_intersects_capsule(ray, position, rotation, top, bottom, *radius),
    }
}

/// 射线与圆形求交
#[allow(dead_code)]
fn ray_intersects_circle(ray: &RayCast2D, center: Vec2, radius: f32) -> Option<RayCastHit2D> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t0 = (-b - sqrt_d) / (2.0 * a);
    let t1 = (-b + sqrt_d) / (2.0 * a);

    let t = if t0 >= 0.0 && t0 <= ray.max_distance {
        t0
    } else if t1 >= 0.0 && t1 <= ray.max_distance {
        t1
    } else {
        return None;
    };

    let point = ray.point_at(t);
    let normal = (point - center).normalize();

    Some(RayCastHit2D {
        point,
        normal,
        time: t / ray.max_distance,
        collider: 0,
        face_index: 0,
    })
}

#[allow(dead_code)]
fn ray_intersects_aabb(ray: &RayCast2D, center: Vec2, half_extents: Vec2) -> Option<RayCastHit2D> {
    let min = center - half_extents;
    let max = center + half_extents;

    let mut tmin = 0.0f32;
    let mut tmax = ray.max_distance;
    let mut normal = Vec2::ZERO;

    // X 轴
    if ray.direction.x.abs() > f32::EPSILON {
        let t1 = (min.x - ray.origin.x) / ray.direction.x;
        let t2 = (max.x - ray.origin.x) / ray.direction.x;
        let (enter_t, exit_t, enter_normal) = if t1 < t2 {
            (t1, t2, Vec2::new(-1.0, 0.0))
        } else {
            (t2, t1, Vec2::new(1.0, 0.0))
        };
        if enter_t > tmin {
            tmin = enter_t;
            normal = enter_normal;
        }
        tmax = tmax.min(exit_t);
    } else if ray.origin.x < min.x || ray.origin.x > max.x {
        return None;
    }

    // Y 轴
    if ray.direction.y.abs() > f32::EPSILON {
        let t1 = (min.y - ray.origin.y) / ray.direction.y;
        let t2 = (max.y - ray.origin.y) / ray.direction.y;
        let (enter_t, exit_t, enter_normal) = if t1 < t2 {
            (t1, t2, Vec2::new(0.0, -1.0))
        } else {
            (t2, t1, Vec2::new(0.0, 1.0))
        };
        if enter_t > tmin {
            tmin = enter_t;
            normal = enter_normal;
        }
        tmax = tmax.min(exit_t);
    } else if ray.origin.y < min.y || ray.origin.y > max.y {
        return None;
    }

    if tmin > tmax || tmax < 0.0 {
        return None;
    }

    let t = if tmin >= 0.0 { tmin } else { tmax };
    if t > ray.max_distance {
        return None;
    }

    let point = ray.point_at(t);

    Some(RayCastHit2D {
        point,
        normal,
        time: t / ray.max_distance,
        collider: 0,
        face_index: 0,
    })
}

#[allow(dead_code)]
fn ray_intersects_rectangle(
    ray: &RayCast2D,
    position: Vec2,
    rotation: f32,
    half_extents: Vec2,
) -> Option<RayCastHit2D> {
    let cos = rotation.cos();
    let sin = rotation.sin();

    let local_origin = Vec2::new(
        (ray.origin.x - position.x) * cos + (ray.origin.y - position.y) * sin,
        -(ray.origin.x - position.x) * sin + (ray.origin.y - position.y) * cos,
    );
    let local_direction = Vec2::new(
        ray.direction.x * cos + ray.direction.y * sin,
        -ray.direction.x * sin + ray.direction.y * cos,
    );

    let local_ray = RayCast2D::new(local_origin, local_direction, ray.max_distance);

    let hit = ray_intersects_aabb(&local_ray, Vec2::ZERO, half_extents)?;

    let world_normal = Vec2::new(
        hit.normal.x * cos - hit.normal.y * sin,
        hit.normal.x * sin + hit.normal.y * cos,
    );

    Some(RayCastHit2D {
        point: hit.point,
        normal: world_normal,
        time: hit.time,
        collider: hit.collider,
        face_index: hit.face_index,
    })
}

#[allow(dead_code)]
fn ray_intersects_polygon(
    ray: &RayCast2D,
    position: Vec2,
    rotation: f32,
    vertices: &[Vec2],
) -> Option<RayCastHit2D> {
    if vertices.len() < 3 {
        return None;
    }

    let cos = rotation.cos();
    let sin = rotation.sin();

    // 转换到局部空间
    let local_origin = Vec2::new(
        (ray.origin.x - position.x) * cos + (ray.origin.y - position.y) * sin,
        -(ray.origin.x - position.x) * sin + (ray.origin.y - position.y) * cos,
    );
    let local_direction = Vec2::new(
        ray.direction.x * cos + ray.direction.y * sin,
        -ray.direction.x * sin + ray.direction.y * cos,
    );

    let mut closest_t = f32::MAX;
    let mut closest_normal = Vec2::ZERO;
    let mut closest_edge_index = 0;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        let edge = vertices[j] - vertices[i];
        let normal = Vec2::new(-edge.y, edge.x).normalize();

        let denom = local_direction.dot(normal);
        if denom.abs() < f32::EPSILON {
            continue;
        }

        let t = -(local_origin - vertices[i]).dot(normal) / denom;
        if t < 0.0 || t > ray.max_distance || t > closest_t {
            continue;
        }

        let point = local_origin + local_direction * t;

        // 检查点是否在边内
        let edge_vec = vertices[j] - vertices[i];
        let edge_length_sq = edge_vec.length_squared();
        let to_point = point - vertices[i];
        let proj = to_point.dot(edge_vec);
        if proj >= 0.0 && proj <= edge_length_sq {
            closest_t = t;
            closest_normal = normal;
            closest_edge_index = i;
        }
    }

    if closest_t == f32::MAX || closest_t > ray.max_distance {
        return None;
    }

    // 转换回世界空间
    let world_normal = Vec2::new(
        closest_normal.x * cos - closest_normal.y * sin,
        closest_normal.x * sin + closest_normal.y * cos,
    );

    Some(RayCastHit2D {
        point: ray.point_at(closest_t),
        normal: world_normal,
        time: closest_t / ray.max_distance,
        collider: 0,
        face_index: closest_edge_index,
    })
}

#[allow(dead_code)]
fn ray_intersects_capsule(
    ray: &RayCast2D,
    position: Vec2,
    rotation: f32,
    top: &Vec2,
    bottom: &Vec2,
    radius: f32,
) -> Option<RayCastHit2D> {
    let cos = rotation.cos();
    let sin = rotation.sin();

    // 变换胶囊端点到世界坐标
    let world_top = Vec2::new(
        top.x * cos - top.y * sin + position.x,
        top.x * sin + top.y * cos + position.y,
    );
    let world_bottom = Vec2::new(
        bottom.x * cos - bottom.y * sin + position.x,
        bottom.x * sin + bottom.y * cos + position.y,
    );

    let direction = world_top - world_bottom;
    let length = direction.length();
    if length < f32::EPSILON {
        return ray_intersects_circle(ray, world_bottom, radius);
    }
    let normalized = direction / length;

    // 端点圆
    if let Some(hit) = ray_intersects_circle(ray, world_top, radius) {
        if hit.time * ray.max_distance <= ray.max_distance {
            return Some(hit);
        }
    }

    if let Some(hit) = ray_intersects_circle(ray, world_bottom, radius) {
        if hit.time * ray.max_distance <= ray.max_distance {
            return Some(hit);
        }
    }

    // 胶囊柱体部分
    let oc = ray.origin - (world_top + world_bottom) / 2.0;
    let line_dir = normalized;
    let _line_half_length = length / 2.0;

    let a = ray.direction.dot(ray.direction) - ray.direction.dot(line_dir).powi(2);
    let b = 2.0 * (oc.dot(ray.direction) - oc.dot(line_dir) * ray.direction.dot(line_dir));
    let c = oc.dot(oc) - oc.dot(line_dir).powi(2) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t0 = (-b - sqrt_d) / (2.0 * a);
    let t1 = (-b + sqrt_d) / (2.0 * a);

    for t in [t0, t1] {
        if t < 0.0 || t > ray.max_distance {
            continue;
        }

        let point = ray.point_at(t);
        let axis_point = (point.dot(line_dir) / length) * direction + world_bottom;

        if axis_point.distance(world_bottom) <= length {
            let normal = (point - axis_point).normalize();
            return Some(RayCastHit2D {
                point,
                normal,
                time: t / ray.max_distance,
                collider: 0,
                face_index: 0,
            });
        }
    }

    None
}

/// 点是否在形状内（使用位置和旋转）
pub fn point_in_shape(position: Vec2, shape: &ColliderShape, transform: Transform2D) -> bool {
    let (transform_position, transform_rotation) = transform;
    point_in_shape_with_pos_rot(position, shape, transform_position, transform_rotation)
}

/// 点是否在形状内（使用显式位置和旋转）
pub fn point_in_shape_with_pos_rot(
    position: Vec2,
    shape: &ColliderShape,
    transform_position: Vec2,
    transform_rotation: f32,
) -> bool {
    match shape {
        ColliderShape::Circle { radius } => (position - transform_position).length() <= *radius,
        ColliderShape::Aabb { half_extents } => {
            let local = position - transform_position;
            local.x.abs() <= half_extents.x && local.y.abs() <= half_extents.y
        }
        ColliderShape::Rectangle { half_extents } => {
            let cos = transform_rotation.cos();
            let sin = transform_rotation.sin();
            let local = Vec2::new(
                (position.x - transform_position.x) * cos
                    + (position.y - transform_position.y) * sin,
                -(position.x - transform_position.x) * sin
                    + (position.y - transform_position.y) * cos,
            );
            local.x.abs() <= half_extents.x && local.y.abs() <= half_extents.y
        }
        ColliderShape::Polygon { vertices } => {
            if vertices.len() < 3 {
                return false;
            }
            let cos = transform_rotation.cos();
            let sin = transform_rotation.sin();
            let local = Vec2::new(
                (position.x - transform_position.x) * cos
                    + (position.y - transform_position.y) * sin,
                -(position.x - transform_position.x) * sin
                    + (position.y - transform_position.y) * cos,
            );
            for i in 0..vertices.len() {
                let j = (i + 1) % vertices.len();
                let edge = vertices[j] - vertices[i];
                let to_point = local - vertices[i];
                if edge.x * to_point.y - edge.y * to_point.x < 0.0 {
                    return false;
                }
            }
            true
        }
        ColliderShape::Capsule {
            top,
            bottom,
            radius,
        } => {
            let cos = transform_rotation.cos();
            let sin = transform_rotation.sin();
            let world_top = Vec2::new(
                top.x * cos - top.y * sin + transform_position.x,
                top.x * sin + top.y * cos + transform_position.y,
            );
            let world_bottom = Vec2::new(
                bottom.x * cos - bottom.y * sin + transform_position.x,
                bottom.x * sin + bottom.y * cos + transform_position.y,
            );
            let direction = world_top - world_bottom;
            let length = direction.length();
            if length < f32::EPSILON {
                return (position - world_top).length() <= *radius;
            }
            let normalized = direction / length;
            let proj = (position - world_bottom).dot(normalized);
            if proj < 0.0 {
                return (position - world_bottom).length() <= *radius;
            }
            if proj > length {
                return (position - world_top).length() <= *radius;
            }
            let closest = world_bottom + normalized * proj;
            (position - closest).length() <= *radius
        }
    }
}

/// 变换2D辅助 trait
pub trait Transform2DExt {
    /// 创建新的变换
    fn new(position: Vec2, rotation: f32) -> Self;
    /// 变换点（世界坐标）
    fn transform_point(&self, point: Vec2) -> Vec2;
    /// 逆变换点
    fn inverse_transform_point(&self, point: Vec2) -> Vec2;
}

impl Transform2DExt for Transform2D {
    /// 创建新的变换
    fn new(position: Vec2, rotation: f32) -> Self {
        (position, rotation)
    }

    /// 变换点（世界坐标）
    fn transform_point(&self, point: Vec2) -> Vec2 {
        let (position, rotation) = *self;
        let cos = rotation.cos();
        let sin = rotation.sin();
        // 先转换到局部空间，再旋转
        let local = point - position;
        Vec2::new(
            local.x * cos - local.y * sin + position.x,
            local.x * sin + local.y * cos + position.y,
        )
    }

    /// 逆变换点
    fn inverse_transform_point(&self, point: Vec2) -> Vec2 {
        let (position, rotation) = *self;
        let cos = rotation.cos();
        let sin = rotation.sin();
        // 先减去位置，再逆旋转
        let shifted = point - position;
        Vec2::new(
            shifted.x * cos + shifted.y * sin,
            -shifted.x * sin + shifted.y * cos,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_cast_circle() {
        let ray = RayCast2D::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), 10.0);
        let hit = ray_intersects_shape(&ray, Vec2::new(5.0, 0.0), 0.0, &ColliderShape::circle(1.0));
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.point.x - 4.0).abs() < 0.001);
        assert!((hit.time - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_ray_cast_miss() {
        let ray = RayCast2D::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), 10.0);
        let hit = ray_intersects_shape(&ray, Vec2::new(5.0, 3.0), 0.0, &ColliderShape::circle(1.0));
        assert!(hit.is_none());
    }

    #[test]
    fn test_ray_cast_aabb() {
        let ray = RayCast2D::new(Vec2::new(-5.0, 0.0), Vec2::new(1.0, 0.0), 10.0);
        let hit = ray_intersects_shape(
            &ray,
            Vec2::new(0.0, 0.0),
            0.0,
            &ColliderShape::aabb(2.0, 2.0),
        );
        assert!(hit.is_some());
    }

    #[test]
    fn test_point_in_circle() {
        let shape = ColliderShape::circle(1.0);
        let transform: Transform2D = Transform2DExt::new(Vec2::new(0.0, 0.0), 0.0);
        assert!(point_in_shape(Vec2::new(0.5, 0.0), &shape, transform));
        assert!(!point_in_shape(Vec2::new(2.0, 0.0), &shape, transform));
    }

    #[test]
    fn test_ray_endpoint() {
        let ray = RayCast2D::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), 10.0);
        let endpoint = ray.endpoint();
        let expected = Vec2::new(10.0 / 2.0_f32.sqrt(), 10.0 / 2.0_f32.sqrt());
        assert!((endpoint.x - expected.x).abs() < 0.001);
        assert!((endpoint.y - expected.y).abs() < 0.001);
    }

    #[test]
    fn test_ray_cast_capsule() {
        let ray = RayCast2D::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), 10.0);
        let hit = ray_intersects_shape(
            &ray,
            Vec2::new(5.0, 0.0),
            0.0,
            &ColliderShape::Capsule {
                top: Vec2::new(0.0, 1.0),
                bottom: Vec2::new(0.0, -1.0),
                radius: 0.5,
            },
        );
        assert!(hit.is_some());
    }

    #[test]
    fn test_transform2d() {
        let transform: Transform2D =
            Transform2DExt::new(Vec2::new(1.0, 2.0), std::f32::consts::FRAC_PI_2);
        let point = Vec2::new(0.0, 1.0);
        let transformed = Transform2DExt::transform_point(&transform, point);
        // 绕 (1, 2) 旋转 90 度
        // 点 (0, 1) 相对中心是 (-1, -1)
        // 旋转后: (-1, -1) -> (1, -1)
        // 加回中心: (2, 1)
        assert!((transformed.x - 2.0).abs() < 0.001);
        assert!((transformed.y - 1.0).abs() < 0.001);
    }
}
