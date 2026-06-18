//! 碰撞体模块
//!
//! 提供 2D 碰撞体实现，包括圆形、矩形、多边形等形状。

use engine_math::{Rect, Vec2};

/// 碰撞体形状
#[derive(Debug, Clone, PartialEq)]
pub enum ColliderShape {
    /// 圆形
    Circle {
        /// 半径
        radius: f32,
    },
    /// 轴对齐矩形
    Aabb {
        /// 半尺寸
        half_extents: Vec2,
    },
    /// 矩形（有旋转）
    Rectangle {
        /// 半尺寸
        half_extents: Vec2,
    },
    /// 凸多边形
    Polygon {
        /// 顶点列表（相对于碰撞体中心）
        vertices: Vec<Vec2>,
    },
    /// 胶囊形状
    Capsule {
        /// 上端点
        top: Vec2,
        /// 下端点
        bottom: Vec2,
        /// 半径
        radius: f32,
    },
}

impl ColliderShape {
    /// 创建圆形
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }

    /// 创建矩形
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self::Rectangle {
            half_extents: Vec2::new(width / 2.0, height / 2.0),
        }
    }

    /// 创建 AABB
    pub fn aabb(width: f32, height: f32) -> Self {
        Self::Aabb {
            half_extents: Vec2::new(width / 2.0, height / 2.0),
        }
    }

    /// 从矩形创建凸多边形
    pub fn convex_hull(vertices: Vec<Vec2>) -> Self {
        Self::Polygon { vertices }
    }

    /// 计算 AABB
    pub fn compute_aabb(&self, position: Vec2, rotation: f32) -> Rect {
        match self {
            Self::Circle { radius } => Rect::new(
                position.x - radius,
                position.y - radius,
                radius * 2.0,
                radius * 2.0,
            ),
            Self::Aabb { half_extents } | Self::Rectangle { half_extents } => Rect::new(
                position.x - half_extents.x,
                position.y - half_extents.y,
                half_extents.x * 2.0,
                half_extents.y * 2.0,
            ),
            Self::Polygon { vertices } => {
                Self::compute_aabb_from_vertices(vertices, position, rotation)
            }
            Self::Capsule {
                top,
                bottom,
                radius,
            } => {
                let min_x = (top.x.min(bottom.x) - radius).min(top.x);
                let max_x = (top.x.max(bottom.x) + radius).max(top.x);
                let min_y = (top.y.min(bottom.y) - radius).min(top.y);
                let max_y = (top.y.max(bottom.y) + radius).max(top.y);
                Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
            }
        }
    }

    /// 从顶点计算 AABB
    fn compute_aabb_from_vertices(vertices: &[Vec2], position: Vec2, rotation: f32) -> Rect {
        if vertices.is_empty() {
            return Rect::new(position.x, position.y, 0.0, 0.0);
        }

        let cos = rotation.cos();
        let sin = rotation.sin();

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for v in vertices {
            let rx = v.x * cos - v.y * sin + position.x;
            let ry = v.x * sin + v.y * cos + position.y;
            min_x = min_x.min(rx);
            min_y = min_y.min(ry);
            max_x = max_x.max(rx);
            max_y = max_y.max(ry);
        }

        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// 计算质量
    pub fn compute_mass(&self, density: f32) -> f32 {
        match self {
            Self::Circle { radius } => std::f32::consts::PI * radius * radius * density,
            Self::Aabb { half_extents } | Self::Rectangle { half_extents } => {
                4.0 * half_extents.x * half_extents.y * density
            }
            Self::Polygon { vertices } => Self::compute_polygon_area(vertices) * density,
            Self::Capsule {
                top,
                bottom,
                radius,
            } => {
                let length = (*top - *bottom).length();
                let cylinder_area = 2.0 * radius * length;
                let caps_area = std::f32::consts::PI * radius * radius;
                (cylinder_area + caps_area) * density
            }
        }
    }

    /// 计算转动惯量
    pub fn compute_inertia(&self, mass: f32) -> f32 {
        match self {
            Self::Circle { radius } => 0.5 * mass * radius * radius,
            Self::Aabb { half_extents } | Self::Rectangle { half_extents } => {
                mass * (half_extents.x * half_extents.x + half_extents.y * half_extents.y) / 3.0
            }
            Self::Polygon { vertices } => Self::compute_polygon_inertia(vertices, mass),
            Self::Capsule {
                top,
                bottom,
                radius,
            } => {
                let length = (*top - *bottom).length();
                let cylinder_inertia = mass * (0.25 * radius * radius + length * length / 12.0);
                let cap_inertia = 0.5 * mass * radius * radius;
                cylinder_inertia + cap_inertia
            }
        }
    }

    /// 计算多边形面积
    fn compute_polygon_area(vertices: &[Vec2]) -> f32 {
        if vertices.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            area += vertices[i].x * vertices[j].y;
            area -= vertices[j].x * vertices[i].y;
        }
        area.abs() / 2.0
    }

    /// 计算多边形转动惯量
    fn compute_polygon_inertia(vertices: &[Vec2], mass: f32) -> f32 {
        let area = Self::compute_polygon_area(vertices);
        if area <= 0.0 {
            return mass;
        }

        let mut inertia = 0.0;
        let mut signed_area = 0.0;

        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let cross = vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y;
            signed_area += cross;
            inertia += cross
                * (vertices[i].x * vertices[i].x
                    + vertices[i].x * vertices[j].x
                    + vertices[j].x * vertices[j].x
                    + vertices[i].y * vertices[i].y
                    + vertices[i].y * vertices[j].y
                    + vertices[j].y * vertices[j].y);
        }

        signed_area /= 2.0;
        inertia = inertia.abs() / 12.0;

        let density = mass / signed_area;
        inertia * density
    }
}

/// 碰撞体
///
/// 附加到刚体上的碰撞形状。
#[derive(Debug, Clone)]
pub struct Collider2D {
    /// 形状
    shape: ColliderShape,
    /// 相对于刚体的位置
    local_position: Vec2,
    /// 相对于刚体的旋转
    local_rotation: f32,
    /// 密度
    density: f32,
    /// 弹性系数
    restitution: f32,
    /// 摩擦系数
    friction: f32,
    /// 静摩擦系数
    static_friction: f32,
    /// 是否是传感器（只检测碰撞不产生响应）
    is_sensor: bool,
    /// 碰撞分组
    collision_group: u32,
    /// 碰撞掩码
    collision_mask: u32,
    /// 是否启用
    enabled: bool,
}

impl Collider2D {
    /// 创建新的碰撞体
    pub fn new(shape: ColliderShape) -> Self {
        Self {
            shape,
            local_position: Vec2::ZERO,
            local_rotation: 0.0,
            density: 1.0,
            restitution: 0.3,
            friction: 0.5,
            static_friction: 0.6,
            is_sensor: false,
            collision_group: 0xFFFFFFFF,
            collision_mask: 0xFFFFFFFF,
            enabled: true,
        }
    }

    /// 获取形状
    pub fn shape(&self) -> &ColliderShape {
        &self.shape
    }

    /// 获取局部位置
    pub fn local_position(&self) -> Vec2 {
        self.local_position
    }

    /// 设置局部位置
    pub fn set_local_position(&mut self, position: Vec2) {
        self.local_position = position;
    }

    /// 获取局部旋转
    pub fn local_rotation(&self) -> f32 {
        self.local_rotation
    }

    /// 设置局部旋转
    pub fn set_local_rotation(&mut self, rotation: f32) {
        self.local_rotation = rotation;
    }

    /// 获取密度
    pub fn density(&self) -> f32 {
        self.density
    }

    /// 设置密度
    pub fn set_density(&mut self, density: f32) {
        self.density = density;
    }

    /// 获取弹性系数
    pub fn restitution(&self) -> f32 {
        self.restitution
    }

    /// 设置弹性系数
    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    /// 获取摩擦系数
    pub fn friction(&self) -> f32 {
        self.friction
    }

    /// 设置摩擦系数
    pub fn set_friction(&mut self, friction: f32) {
        self.friction = friction;
    }

    /// 获取静摩擦系数
    pub fn static_friction(&self) -> f32 {
        self.static_friction
    }

    /// 设置静摩擦系数
    pub fn set_static_friction(&mut self, friction: f32) {
        self.static_friction = friction;
    }

    /// 检查是否是传感器
    pub fn is_sensor(&self) -> bool {
        self.is_sensor
    }

    /// 设置是否为传感器
    pub fn set_sensor(&mut self, sensor: bool) {
        self.is_sensor = sensor;
    }

    /// 获取碰撞分组
    pub fn collision_group(&self) -> u32 {
        self.collision_group
    }

    /// 设置碰撞分组
    pub fn set_collision_group(&mut self, group: u32) {
        self.collision_group = group;
    }

    /// 获取碰撞掩码
    pub fn collision_mask(&self) -> u32 {
        self.collision_mask
    }

    /// 设置碰撞掩码
    pub fn set_collision_mask(&mut self, mask: u32) {
        self.collision_mask = mask;
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 计算世界坐标
    pub fn world_position(&self, body_position: Vec2, body_rotation: f32) -> Vec2 {
        let cos = body_rotation.cos();
        let sin = body_rotation.sin();
        Vec2::new(
            self.local_position.x * cos - self.local_position.y * sin + body_position.x,
            self.local_position.x * sin + self.local_position.y * cos + body_position.y,
        )
    }

    /// 计算世界旋转
    pub fn world_rotation(&self, body_rotation: f32) -> f32 {
        body_rotation + self.local_rotation
    }

    /// 获取质量
    pub fn mass(&self) -> f32 {
        self.shape.compute_mass(self.density)
    }

    /// 获取转动惯量
    pub fn inertia(&self) -> f32 {
        self.shape.compute_inertia(self.mass())
    }
}

/// 碰撞体构建器
pub struct Collider2DBuilder {
    collider: Collider2D,
}

impl Collider2DBuilder {
    /// 创建圆形碰撞体
    pub fn circle(radius: f32) -> Self {
        Self {
            collider: Collider2D::new(ColliderShape::circle(radius)),
        }
    }

    /// 创建矩形碰撞体
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self {
            collider: Collider2D::new(ColliderShape::rectangle(width, height)),
        }
    }

    /// 创建 AABB 碰撞体
    pub fn aabb(width: f32, height: f32) -> Self {
        Self {
            collider: Collider2D::new(ColliderShape::aabb(width, height)),
        }
    }

    /// 创建多边形碰撞体
    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        Self {
            collider: Collider2D::new(ColliderShape::convex_hull(vertices)),
        }
    }

    /// 设置相对位置
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.collider.set_local_position(position);
        self
    }

    /// 设置相对旋转
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.collider.set_local_rotation(rotation);
        self
    }

    /// 设置密度
    pub fn with_density(mut self, density: f32) -> Self {
        self.collider.set_density(density);
        self
    }

    /// 设置弹性系数
    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.collider.set_restitution(restitution);
        self
    }

    /// 设置摩擦系数
    pub fn with_friction(mut self, friction: f32) -> Self {
        self.collider.set_friction(friction);
        self
    }

    /// 设置为传感器
    pub fn as_sensor(mut self) -> Self {
        self.collider.set_sensor(true);
        self
    }

    /// 设置碰撞分组
    pub fn with_group(mut self, group: u32) -> Self {
        self.collider.set_collision_group(group);
        self
    }

    /// 设置碰撞掩码
    pub fn with_mask(mut self, mask: u32) -> Self {
        self.collider.set_collision_mask(mask);
        self
    }

    /// 构建碰撞体
    pub fn build(self) -> Collider2D {
        self.collider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_collider() {
        let collider = Collider2DBuilder::circle(1.0).build();
        assert!(matches!(collider.shape(), ColliderShape::Circle { .. }));
    }

    #[test]
    fn test_rectangle_collider() {
        let collider = Collider2DBuilder::rectangle(2.0, 1.0).build();
        assert!(matches!(collider.shape(), ColliderShape::Rectangle { .. }));
    }

    #[test]
    fn test_collider_mass() {
        let collider = Collider2DBuilder::circle(1.0).with_density(2.0).build();
        let expected_mass = std::f32::consts::PI * 1.0 * 1.0 * 2.0;
        assert!((collider.mass() - expected_mass).abs() < 0.001);
    }

    #[test]
    fn test_collider_builder() {
        let collider = Collider2DBuilder::circle(0.5)
            .with_position(Vec2::new(1.0, 2.0))
            .with_density(3.0)
            .with_restitution(0.8)
            .with_friction(0.4)
            .as_sensor()
            .build();

        assert_eq!(collider.local_position(), Vec2::new(1.0, 2.0));
        assert_eq!(collider.density(), 3.0);
        assert_eq!(collider.restitution(), 0.8);
        assert_eq!(collider.friction(), 0.4);
        assert!(collider.is_sensor());
    }

    #[test]
    fn test_shape_aabb() {
        let shape = ColliderShape::aabb(10.0, 20.0);
        let aabb = shape.compute_aabb(Vec2::new(0.0, 0.0), 0.0);
        assert_eq!(aabb.x, -5.0);
        assert_eq!(aabb.y, -10.0);
        assert_eq!(aabb.w, 10.0);
        assert_eq!(aabb.h, 20.0);
    }

    #[test]
    fn test_shape_circle_aabb() {
        let shape = ColliderShape::circle(5.0);
        let aabb = shape.compute_aabb(Vec2::new(10.0, 10.0), 0.0);
        assert_eq!(aabb.x, 5.0);
        assert_eq!(aabb.y, 5.0);
        assert_eq!(aabb.w, 10.0);
        assert_eq!(aabb.h, 10.0);
    }
}
