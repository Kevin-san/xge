//! 空间查询模块
//!
//! 提供 3D 射线投射、形状查询、点查询等空间查询功能。

use engine_math::{Quat, Vec3};

use crate::{ColliderShape3D, CollisionGroups, EntityHandle, AABB};

/// 射线
#[derive(Debug, Clone)]
pub struct Ray3 {
    /// 起点
    pub origin: Vec3,
    /// 方向（归一化）
    pub direction: Vec3,
}

impl Ray3 {
    /// 创建新的射线
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// 计算射线上某点的位置
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// 获取射线终点（给定最大距离）
    pub fn endpoint(&self, max_distance: f32) -> Vec3 {
        self.point_at(max_distance)
    }
}

/// 射线投射命中结果
#[derive(Debug, Clone)]
pub struct RayCastHit {
    /// 命中点
    pub point: Vec3,
    /// 法线
    pub normal: Vec3,
    /// 命中时间（距离）
    pub toi: f32,
    /// 命中的实体
    pub entity: EntityHandle,
    /// 命中的碰撞体索引
    pub collider_index: usize,
}

impl RayCastHit {
    /// 创建新的命中结果
    pub fn new(point: Vec3, normal: Vec3, toi: f32, entity: EntityHandle) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            toi,
            entity,
            collider_index: 0,
        }
    }

    /// 获取命中点
    pub fn point(&self) -> Vec3 {
        self.point
    }

    /// 获取法线
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// 获取命中时间
    pub fn toi(&self) -> f32 {
        self.toi
    }

    /// 获取实体
    pub fn entity(&self) -> EntityHandle {
        self.entity
    }
}

/// 形状投射命中结果
#[derive(Debug, Clone)]
pub struct ShapeCastHit {
    /// 命中时间
    pub toi: f32,
    /// 命中点（形状上的）
    pub point1: Vec3,
    /// 命中点（被碰撞物体上的）
    pub point2: Vec3,
    /// 法线
    pub normal: Vec3,
    /// 命中的实体
    pub entity: EntityHandle,
}

impl ShapeCastHit {
    /// 创建新的形状投射命中结果
    pub fn new(toi: f32, point1: Vec3, point2: Vec3, normal: Vec3, entity: EntityHandle) -> Self {
        Self {
            toi,
            point1,
            point2,
            normal: normal.normalize(),
            entity,
        }
    }

    /// 获取命中时间
    pub fn toi(&self) -> f32 {
        self.toi
    }

    /// 获取法线
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// 获取实体
    pub fn entity(&self) -> EntityHandle {
        self.entity
    }
}

/// 查询过滤器
#[derive(Debug, Clone)]
pub struct QueryFilter {
    /// 碰撞分组
    pub groups: CollisionGroups,
    /// 是否排除传感器
    pub exclude_sensors: bool,
    /// 是否排除动态刚体
    pub exclude_dynamic: bool,
    /// 是否排除静态刚体
    pub exclude_static: bool,
    /// 是否排除固定刚体
    pub exclude_fixed: bool,
    /// 是否排除运动刚体
    pub exclude_kinematic: bool,
    /// 排除的实体列表
    pub exclude_entities: Vec<EntityHandle>,
}

impl QueryFilter {
    /// 创建新的查询过滤器
    pub fn new() -> Self {
        Self {
            groups: CollisionGroups::ALL,
            exclude_sensors: false,
            exclude_dynamic: false,
            exclude_static: false,
            exclude_fixed: false,
            exclude_kinematic: false,
            exclude_entities: Vec::new(),
        }
    }

    /// 只查询动态刚体
    pub fn only_dynamic() -> Self {
        Self::new()
            .with_exclude_static(true)
            .with_exclude_fixed(true)
            .with_exclude_kinematic(true)
    }

    /// 排除传感器
    pub fn exclude_sensors() -> Self {
        Self::new().with_exclude_sensors(true)
    }

    /// 设置碰撞分组
    pub fn with_groups(mut self, groups: CollisionGroups) -> Self {
        self.groups = groups;
        self
    }

    /// 设置排除传感器
    pub fn with_exclude_sensors(mut self, exclude: bool) -> Self {
        self.exclude_sensors = exclude;
        self
    }

    /// 设置排除动态刚体
    pub fn with_exclude_dynamic(mut self, exclude: bool) -> Self {
        self.exclude_dynamic = exclude;
        self
    }

    /// 设置排除静态刚体
    pub fn with_exclude_static(mut self, exclude: bool) -> Self {
        self.exclude_static = exclude;
        self
    }

    /// 设置排除固定刚体
    pub fn with_exclude_fixed(mut self, exclude: bool) -> Self {
        self.exclude_fixed = exclude;
        self
    }

    /// 设置排除运动刚体
    pub fn with_exclude_kinematic(mut self, exclude: bool) -> Self {
        self.exclude_kinematic = exclude;
        self
    }

    /// 添加排除实体
    pub fn with_exclude(mut self, entity: EntityHandle) -> Self {
        self.exclude_entities.push(entity);
        self
    }

    /// 检查是否应该排除实体
    pub fn should_exclude(&self, entity: EntityHandle) -> bool {
        self.exclude_entities.contains(&entity)
    }
}

impl Default for QueryFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D 空间查询
#[derive(Debug, Clone)]
pub struct Query3D {
    /// 查询管道（用于加速查询）
    query_pipeline: QueryPipeline,
}

impl Query3D {
    /// 创建新的查询对象
    pub fn new() -> Self {
        Self {
            query_pipeline: QueryPipeline::new(),
        }
    }

    /// 射线投射
    ///
    /// 从射线起点沿方向投射，返回命中的实体、距离和法线。
    pub fn cast_ray(
        &self,
        ray: &Ray3,
        max_toi: f32,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<(EntityHandle, f32, Vec3)> {
        // 简化实现
        // 实际实现需要遍历所有碰撞体进行检测
        self.query_pipeline.cast_ray(ray, max_toi, solid, filter)
    }

    /// 射线投射并获取详细信息
    ///
    /// 返回完整的命中信息，包括命中点、法线等。
    pub fn cast_ray_and_get_normal(
        &self,
        ray: &Ray3,
        max_toi: f32,
        solid: bool,
        filter: QueryFilter,
    ) -> Option<RayCastHit> {
        self.query_pipeline
            .cast_ray_and_get_normal(ray, max_toi, solid, filter)
    }

    /// 形状投射
    ///
    /// 将形状沿方向移动，检测碰撞。
    pub fn cast_shape(
        &self,
        shape: &ColliderShape3D,
        position: Vec3,
        rotation: Quat,
        direction: Vec3,
        max_toi: f32,
        filter: QueryFilter,
    ) -> Option<ShapeCastHit> {
        self.query_pipeline
            .cast_shape(shape, position, rotation, direction, max_toi, filter)
    }

    /// 形状相交检测
    ///
    /// 检测形状与场景中其他物体的相交。
    pub fn intersection_with_shape(
        &self,
        shape: &ColliderShape3D,
        position: Vec3,
        rotation: Quat,
        filter: QueryFilter,
    ) -> Vec<EntityHandle> {
        self.query_pipeline
            .intersection_with_shape(shape, position, rotation, filter)
    }

    /// 点相交检测
    ///
    /// 检测点与场景中物体的相交。
    pub fn point_intersections(&self, point: Vec3, filter: QueryFilter) -> Vec<EntityHandle> {
        self.query_pipeline.point_intersections(point, filter)
    }

    /// AABB相交检测
    ///
    /// 检测AABB与场景中物体的相交。
    pub fn intersections_with_aabb(&self, aabb: AABB, filter: QueryFilter) -> Vec<EntityHandle> {
        self.query_pipeline.intersections_with_aabb(aabb, filter)
    }

    /// 更新查询管道
    pub fn update(&mut self) {
        self.query_pipeline.update();
    }
}

impl Default for Query3D {
    fn default() -> Self {
        Self::new()
    }
}

/// 查询管道（内部实现）
#[derive(Debug, Clone)]
pub struct QueryPipeline {
    /// 是否已更新
    updated: bool,
}

impl QueryPipeline {
    /// 创建新的查询管道
    pub fn new() -> Self {
        Self { updated: false }
    }

    /// 更新查询管道
    pub fn update(&mut self) {
        self.updated = true;
    }

    /// 射线投射
    pub fn cast_ray(
        &self,
        _ray: &Ray3,
        _max_toi: f32,
        _solid: bool,
        _filter: QueryFilter,
    ) -> Option<(EntityHandle, f32, Vec3)> {
        // 简化实现，返回None
        // 实际实现需要遍历碰撞体进行检测
        None
    }

    /// 射线投射并获取详细信息
    pub fn cast_ray_and_get_normal(
        &self,
        ray: &Ray3,
        max_toi: f32,
        _solid: bool,
        _filter: QueryFilter,
    ) -> Option<RayCastHit> {
        // 简化实现：射线与球体相交测试示例
        // 实际实现需要遍历所有碰撞体
        self.ray_intersects_ball(ray, Vec3::new(5.0, 0.0, 0.0), 1.0, max_toi)
    }

    /// 形状投射
    pub fn cast_shape(
        &self,
        _shape: &ColliderShape3D,
        _position: Vec3,
        _rotation: Quat,
        _direction: Vec3,
        _max_toi: f32,
        _filter: QueryFilter,
    ) -> Option<ShapeCastHit> {
        None
    }

    /// 形状相交检测
    pub fn intersection_with_shape(
        &self,
        _shape: &ColliderShape3D,
        _position: Vec3,
        _rotation: Quat,
        _filter: QueryFilter,
    ) -> Vec<EntityHandle> {
        Vec::new()
    }

    /// 点相交检测
    pub fn point_intersections(&self, _point: Vec3, _filter: QueryFilter) -> Vec<EntityHandle> {
        Vec::new()
    }

    /// AABB相交检测
    pub fn intersections_with_aabb(&self, _aabb: AABB, _filter: QueryFilter) -> Vec<EntityHandle> {
        Vec::new()
    }

    /// 射线与球体相交测试
    fn ray_intersects_ball(
        &self,
        ray: &Ray3,
        center: Vec3,
        radius: f32,
        max_toi: f32,
    ) -> Option<RayCastHit> {
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

        let t = if t0 >= 0.0 && t0 <= max_toi {
            t0
        } else if t1 >= 0.0 && t1 <= max_toi {
            t1
        } else {
            return None;
        };

        let point = ray.point_at(t);
        let normal = (point - center).normalize();

        Some(RayCastHit::new(point, normal, t, EntityHandle::new(0, 0)))
    }
}

impl Default for QueryPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// 射线与形状相交检测
pub fn ray_intersects_shape(
    ray: &Ray3,
    shape: &ColliderShape3D,
    position: Vec3,
    rotation: Quat,
    max_toi: f32,
) -> Option<RayCastHit> {
    match shape {
        ColliderShape3D::Ball { radius } => ray_intersects_ball(ray, position, *radius, max_toi),
        ColliderShape3D::Cuboid { hx, hy, hz } => {
            ray_intersects_cuboid(ray, position, rotation, *hx, *hy, *hz, max_toi)
        }
        ColliderShape3D::Capsule {
            half_height,
            radius,
            axis,
        } => ray_intersects_capsule(
            ray,
            position,
            rotation,
            *half_height,
            *radius,
            *axis,
            max_toi,
        ),
        ColliderShape3D::Cylinder {
            half_height,
            radius,
        } => ray_intersects_cylinder(ray, position, rotation, *half_height, *radius, max_toi),
        ColliderShape3D::Cone {
            half_height,
            radius,
        } => ray_intersects_cone(ray, position, rotation, *half_height, *radius, max_toi),
        ColliderShape3D::Triangle { a, b, c } => {
            ray_intersects_triangle(ray, position, rotation, *a, *b, *c, max_toi)
        }
        ColliderShape3D::Segment { a, b } => ray_intersects_segment(ray, position, *a, *b, max_toi),
        ColliderShape3D::Halfspace { outward_normal } => {
            ray_intersects_halfspace(ray, position, *outward_normal, max_toi)
        }
        _ => None, // 复杂形状需要更复杂的实现
    }
}

/// 射线与球体相交检测
fn ray_intersects_ball(ray: &Ray3, center: Vec3, radius: f32, max_toi: f32) -> Option<RayCastHit> {
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

    let t = if t0 >= 0.0 && t0 <= max_toi {
        t0
    } else if t1 >= 0.0 && t1 <= max_toi {
        t1
    } else {
        return None;
    };

    let point = ray.point_at(t);
    let normal = (point - center).normalize();

    Some(RayCastHit::new(point, normal, t, EntityHandle::new(0, 0)))
}

/// 射线与立方体相交检测
fn ray_intersects_cuboid(
    ray: &Ray3,
    position: Vec3,
    rotation: Quat,
    hx: f32,
    hy: f32,
    hz: f32,
    max_toi: f32,
) -> Option<RayCastHit> {
    // 将射线转换到立方体的局部空间
    let inv_rotation = rotation.inverse();
    let local_origin = inv_rotation * (ray.origin - position);
    let local_direction = inv_rotation * ray.direction;

    // AABB相交检测
    let mut tmin = 0.0f32;
    let mut tmax = max_toi;
    let mut normal = Vec3::ZERO;

    // X轴
    if local_direction.x.abs() > f32::EPSILON {
        let t1 = (-hx - local_origin.x) / local_direction.x;
        let t2 = (hx - local_origin.x) / local_direction.x;
        let (enter_t, exit_t, enter_normal) = if t1 < t2 {
            (t1, t2, Vec3::new(-1.0, 0.0, 0.0))
        } else {
            (t2, t1, Vec3::new(1.0, 0.0, 0.0))
        };
        if enter_t > tmin {
            tmin = enter_t;
            normal = enter_normal;
        }
        tmax = tmax.min(exit_t);
    } else if local_origin.x.abs() > hx {
        return None;
    }

    // Y轴
    if local_direction.y.abs() > f32::EPSILON {
        let t1 = (-hy - local_origin.y) / local_direction.y;
        let t2 = (hy - local_origin.y) / local_direction.y;
        let (enter_t, exit_t, enter_normal) = if t1 < t2 {
            (t1, t2, Vec3::new(0.0, -1.0, 0.0))
        } else {
            (t2, t1, Vec3::new(0.0, 1.0, 0.0))
        };
        if enter_t > tmin {
            tmin = enter_t;
            normal = enter_normal;
        }
        tmax = tmax.min(exit_t);
    } else if local_origin.y.abs() > hy {
        return None;
    }

    // Z轴
    if local_direction.z.abs() > f32::EPSILON {
        let t1 = (-hz - local_origin.z) / local_direction.z;
        let t2 = (hz - local_origin.z) / local_direction.z;
        let (enter_t, exit_t, enter_normal) = if t1 < t2 {
            (t1, t2, Vec3::new(0.0, 0.0, -1.0))
        } else {
            (t2, t1, Vec3::new(0.0, 0.0, 1.0))
        };
        if enter_t > tmin {
            tmin = enter_t;
            normal = enter_normal;
        }
        tmax = tmax.min(exit_t);
    } else if local_origin.z.abs() > hz {
        return None;
    }

    if tmin > tmax || tmax < 0.0 {
        return None;
    }

    let t = if tmin >= 0.0 { tmin } else { tmax };
    if t > max_toi {
        return None;
    }

    let point = ray.point_at(t);
    let world_normal = rotation * normal;

    Some(RayCastHit::new(
        point,
        world_normal,
        t,
        EntityHandle::new(0, 0),
    ))
}

/// 射线与胶囊体相交检测
fn ray_intersects_capsule(
    ray: &Ray3,
    position: Vec3,
    rotation: Quat,
    half_height: f32,
    radius: f32,
    axis: crate::Axis,
    max_toi: f32,
) -> Option<RayCastHit> {
    // 简化实现：将胶囊视为圆柱 + 两端球体
    let axis_vec = axis.vector();
    let rotated_axis = rotation * axis_vec;

    // 检测两端球体
    let top = position + rotated_axis * half_height;
    let bottom = position - rotated_axis * half_height;

    if let Some(hit) = ray_intersects_ball(ray, top, radius, max_toi) {
        return Some(hit);
    }

    if let Some(hit) = ray_intersects_ball(ray, bottom, radius, max_toi) {
        return Some(hit);
    }

    // 检测圆柱部分（简化）
    None
}

/// 射线与圆柱体相交检测
fn ray_intersects_cylinder(
    ray: &Ray3,
    position: Vec3,
    rotation: Quat,
    half_height: f32,
    radius: f32,
    max_toi: f32,
) -> Option<RayCastHit> {
    // 简化实现
    let inv_rotation = rotation.inverse();
    let local_origin = inv_rotation * (ray.origin - position);
    let local_direction = inv_rotation * ray.direction;

    // 圆柱在局部空间沿Y轴
    // 检测无限圆柱
    let a = local_direction.x * local_direction.x + local_direction.z * local_direction.z;
    let b = 2.0 * (local_origin.x * local_direction.x + local_origin.z * local_direction.z);
    let c = local_origin.x * local_origin.x + local_origin.z * local_origin.z - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t0 = (-b - sqrt_d) / (2.0 * a);
    let t1 = (-b + sqrt_d) / (2.0 * a);

    for t in [t0, t1] {
        if t < 0.0 || t > max_toi {
            continue;
        }

        let y = local_origin.y + t * local_direction.y;
        if y.abs() <= half_height {
            let point = ray.point_at(t);
            let local_normal = Vec3::new(
                local_origin.x + t * local_direction.x,
                0.0,
                local_origin.z + t * local_direction.z,
            )
            .normalize();
            let world_normal = rotation * local_normal;
            return Some(RayCastHit::new(
                point,
                world_normal,
                t,
                EntityHandle::new(0, 0),
            ));
        }
    }

    None
}

/// 射线与圆锥体相交检测
fn ray_intersects_cone(
    _ray: &Ray3,
    _position: Vec3,
    _rotation: Quat,
    _half_height: f32,
    _radius: f32,
    _max_toi: f32,
) -> Option<RayCastHit> {
    // 简化实现，暂不支持
    None
}

/// 射线与三角形相交检测
fn ray_intersects_triangle(
    ray: &Ray3,
    position: Vec3,
    rotation: Quat,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    max_toi: f32,
) -> Option<RayCastHit> {
    let inv_rotation = rotation.inverse();
    let local_origin = inv_rotation * (ray.origin - position);
    let local_direction = inv_rotation * ray.direction;

    // Möller-Trumbore算法
    let edge1 = b - a;
    let edge2 = c - a;
    let h = local_direction.cross(edge2);
    let a_det = edge1.dot(h);

    if a_det.abs() < f32::EPSILON {
        return None;
    }

    let f = 1.0 / a_det;
    let s = local_origin - a;
    let u = f * s.dot(h);

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(edge1);
    let v = f * local_direction.dot(q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(q);

    if t > f32::EPSILON && t <= max_toi {
        let point = ray.point_at(t);
        let normal = edge1.cross(edge2).normalize();
        let world_normal = rotation * normal;
        return Some(RayCastHit::new(
            point,
            world_normal,
            t,
            EntityHandle::new(0, 0),
        ));
    }

    None
}

/// 射线与线段相交检测
fn ray_intersects_segment(
    _ray: &Ray3,
    _position: Vec3,
    _a: Vec3,
    _b: Vec3,
    _max_toi: f32,
) -> Option<RayCastHit> {
    None
}

/// 射线与半空间相交检测
fn ray_intersects_halfspace(
    ray: &Ray3,
    position: Vec3,
    outward_normal: Vec3,
    max_toi: f32,
) -> Option<RayCastHit> {
    let denom = ray.direction.dot(outward_normal);

    if denom.abs() < f32::EPSILON {
        return None;
    }

    let t = -(ray.origin - position).dot(outward_normal) / denom;

    if t >= 0.0 && t <= max_toi {
        let point = ray.point_at(t);
        return Some(RayCastHit::new(
            point,
            outward_normal,
            t,
            EntityHandle::new(0, 0),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_creation() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.origin, Vec3::ZERO);
        assert_eq!(ray.direction, Vec3::X);
    }

    #[test]
    fn test_ray_point_at() {
        let ray = Ray3::new(Vec3::ZERO, Vec3::X);
        assert_eq!(ray.point_at(5.0), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_endpoint() {
        let ray = Ray3::new(Vec3::ZERO, Vec3::X);
        assert_eq!(ray.endpoint(10.0), Vec3::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_cast_hit() {
        let hit = RayCastHit::new(
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            5.0,
            EntityHandle::new(0, 0),
        );
        assert_eq!(hit.point(), Vec3::new(5.0, 0.0, 0.0));
        assert_eq!(hit.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(hit.toi(), 5.0);
    }

    #[test]
    fn test_query_filter() {
        let filter = QueryFilter::new();
        assert_eq!(filter.groups, CollisionGroups::ALL);
        assert!(!filter.exclude_sensors);
    }

    #[test]
    fn test_query_filter_only_dynamic() {
        let filter = QueryFilter::only_dynamic();
        assert!(filter.exclude_static);
        assert!(filter.exclude_fixed);
        assert!(filter.exclude_kinematic);
    }

    #[test]
    fn test_query_filter_exclude_sensors() {
        let filter = QueryFilter::exclude_sensors();
        assert!(filter.exclude_sensors);
    }

    #[test]
    fn test_ray_intersects_ball() {
        let ray = Ray3::new(Vec3::ZERO, Vec3::X);
        let hit = ray_intersects_ball(&ray, Vec3::new(5.0, 0.0, 0.0), 1.0, 10.0);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.point.x - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_misses_ball() {
        let ray = Ray3::new(Vec3::ZERO, Vec3::X);
        let hit = ray_intersects_ball(&ray, Vec3::new(5.0, 3.0, 0.0), 1.0, 10.0);
        assert!(hit.is_none());
    }

    #[test]
    fn test_ray_intersects_cuboid() {
        let ray = Ray3::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::X);
        let hit = ray_intersects_cuboid(&ray, Vec3::ZERO, Quat::IDENTITY, 1.0, 1.0, 1.0, 10.0);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.point.x - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_query3d_creation() {
        let query = Query3D::new();
        assert!(!query.query_pipeline.updated);
        // 更新后应该为true
        let mut query = Query3D::new();
        query.query_pipeline.update();
        assert!(query.query_pipeline.updated);
    }

    #[test]
    fn test_shape_cast_hit() {
        let hit = ShapeCastHit::new(
            5.0,
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            EntityHandle::new(0, 0),
        );
        assert_eq!(hit.toi(), 5.0);
    }

    #[test]
    fn test_ray_intersects_triangle() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
        let a = Vec3::new(-1.0, -1.0, 0.0);
        let b = Vec3::new(1.0, -1.0, 0.0);
        let c = Vec3::new(0.0, 1.0, 0.0);
        let hit = ray_intersects_triangle(&ray, Vec3::ZERO, Quat::IDENTITY, a, b, c, 10.0);
        assert!(hit.is_some());
    }
}
