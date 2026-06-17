//! 碰撞体模块
//!
//! 提供 3D 碰撞体实现，包括球体、立方体、胶囊、圆柱、锥体等形状。

use engine_math::{Quat, Vec3};
use std::f32::consts::PI;

use crate::constants::{DEFAULT_FRICTION, DEFAULT_RESTITUTION};
use crate::{Axis, CollisionGroups, CombineRule, MassProperties, AABB};

/// 碰撞体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColliderType {
    /// 碰撞体（产生碰撞响应）
    #[default]
    Solid,
    /// 传感器（只检测碰撞不产生响应）
    Sensor,
}

/// 碰撞体形状
#[derive(Debug, Clone, PartialEq)]
pub enum ColliderShape3D {
    /// 球体
    Ball {
        /// 半径
        radius: f32,
    },
    /// 立方体
    Cuboid {
        /// X半尺寸
        hx: f32,
        /// Y半尺寸
        hy: f32,
        /// Z半尺寸
        hz: f32,
    },
    /// 胶囊体
    Capsule {
        /// 半高度
        half_height: f32,
        /// 半径
        radius: f32,
        /// 轴方向
        axis: Axis,
    },
    /// 圆柱体
    Cylinder {
        /// 半高度
        half_height: f32,
        /// 半径
        radius: f32,
    },
    /// 圆锥体
    Cone {
        /// 半高度
        half_height: f32,
        /// 半径
        radius: f32,
    },
    /// 凸包
    ConvexHull {
        /// 顶点列表
        points: Vec<Vec3>,
    },
    /// 三角网格
    Trimesh {
        /// 顶点列表
        vertices: Vec<Vec3>,
        /// 三角形索引
        indices: Vec<[u32; 3]>,
    },
    /// 高度场
    Heightfield {
        /// 高度数据
        heights: Vec<f32>,
        /// 缩放
        scale: Vec3,
    },
    /// 线段
    Segment {
        /// 起点
        a: Vec3,
        /// 终点
        b: Vec3,
    },
    /// 三角形
    Triangle {
        /// 顶点A
        a: Vec3,
        /// 顶点B
        b: Vec3,
        /// 顶点C
        c: Vec3,
    },
    /// 半空间（平面）
    Halfspace {
        /// 外向法线
        outward_normal: Vec3,
    },
}

impl ColliderShape3D {
    /// 创建球体
    pub fn ball(radius: f32) -> Self {
        Self::Ball { radius }
    }

    /// 创建立方体
    pub fn cuboid(hx: f32, hy: f32, hz: f32) -> Self {
        Self::Cuboid { hx, hy, hz }
    }

    /// 创建胶囊体
    pub fn capsule(half_height: f32, radius: f32, axis: Axis) -> Self {
        Self::Capsule {
            half_height,
            radius,
            axis,
        }
    }

    /// 创建圆柱体
    pub fn cylinder(half_height: f32, radius: f32) -> Self {
        Self::Cylinder {
            half_height,
            radius,
        }
    }

    /// 创建圆锥体
    pub fn cone(half_height: f32, radius: f32) -> Self {
        Self::Cone {
            half_height,
            radius,
        }
    }

    /// 创建凸包
    pub fn convex_hull(points: Vec<Vec3>) -> Self {
        Self::ConvexHull { points }
    }

    /// 创建三角网格
    pub fn trimesh(vertices: Vec<Vec3>, indices: Vec<[u32; 3]>) -> Self {
        Self::Trimesh { vertices, indices }
    }

    /// 创建高度场
    pub fn heightfield(heights: Vec<f32>, scale: Vec3) -> Self {
        Self::Heightfield { heights, scale }
    }

    /// 创建线段
    pub fn segment(a: Vec3, b: Vec3) -> Self {
        Self::Segment { a, b }
    }

    /// 创建三角形
    pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self::Triangle { a, b, c }
    }

    /// 创建半空间
    pub fn halfspace(outward_normal: Vec3) -> Self {
        Self::Halfspace {
            outward_normal: outward_normal.normalize(),
        }
    }

    /// 计算AABB
    pub fn compute_aabb(&self, position: Vec3, rotation: Quat) -> AABB {
        match self {
            Self::Ball { radius } => AABB::from_center_half_extents(position, Vec3::splat(*radius)),
            Self::Cuboid { hx, hy, hz } => {
                // 考虑旋转的AABB计算
                let half_extents = Vec3::new(*hx, *hy, *hz);
                let rotated_extents = Self::compute_rotated_half_extents(half_extents, rotation);
                AABB::from_center_half_extents(position, rotated_extents)
            }
            Self::Capsule {
                half_height,
                radius,
                axis,
            } => {
                let axis_vec = axis.vector();
                let half_height_vec = axis_vec * *half_height;
                let radius_vec = Vec3::splat(*radius);
                // 旋转胶囊的轴
                let rotated_axis = rotation * axis_vec;
                let rotated_half_height = rotated_axis * *half_height;
                AABB::from_center_half_extents(position, rotated_half_height.abs() + radius_vec)
            }
            Self::Cylinder {
                half_height,
                radius,
            } => {
                // 简化处理，假设Y轴为圆柱轴
                let half_extents = Vec3::new(*radius, *half_height, *radius);
                let rotated_extents = Self::compute_rotated_half_extents(half_extents, rotation);
                AABB::from_center_half_extents(position, rotated_extents)
            }
            Self::Cone {
                half_height,
                radius,
            } => {
                let half_extents = Vec3::new(*radius, *half_height, *radius);
                let rotated_extents = Self::compute_rotated_half_extents(half_extents, rotation);
                AABB::from_center_half_extents(position, rotated_extents)
            }
            Self::ConvexHull { points } => {
                Self::compute_convex_hull_aabb(points, position, rotation)
            }
            Self::Trimesh { vertices, .. } => {
                Self::compute_convex_hull_aabb(vertices, position, rotation)
            }
            Self::Heightfield { scale, .. } => {
                AABB::from_center_half_extents(position, *scale * 0.5)
            }
            Self::Segment { a, b } => {
                let min = position + Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
                let max = position + Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
                AABB::new(min, max)
            }
            Self::Triangle { a, b, c } => {
                let min = position
                    + Vec3::new(
                        a.x.min(b.x).min(c.x),
                        a.y.min(b.y).min(c.y),
                        a.z.min(b.z).min(c.z),
                    );
                let max = position
                    + Vec3::new(
                        a.x.max(b.x).max(c.x),
                        a.y.max(b.y).max(c.y),
                        a.z.max(b.z).max(c.z),
                    );
                AABB::new(min, max)
            }
            Self::Halfspace { .. } => {
                // 半空间是无限的，返回一个大的AABB
                AABB::new(Vec3::new(-1e6, -1e6, -1e6), Vec3::new(1e6, 1e6, 1e6))
            }
        }
    }

    /// 计算旋转后的半尺寸
    fn compute_rotated_half_extents(half_extents: Vec3, rotation: Quat) -> Vec3 {
        // 简化计算：取旋转后各轴的最大投影
        let abs_rot = Quat {
            x: rotation.x.abs(),
            y: rotation.y.abs(),
            z: rotation.z.abs(),
            w: rotation.w.abs(),
        };

        // 计算旋转矩阵的绝对值行
        let row0 = Vec3::new(
            abs_rot.w * abs_rot.w + abs_rot.x * abs_rot.x
                - abs_rot.y * abs_rot.y
                - abs_rot.z * abs_rot.z,
            2.0 * (abs_rot.x * abs_rot.y + abs_rot.w * abs_rot.z),
            2.0 * (abs_rot.x * abs_rot.z - abs_rot.w * abs_rot.y),
        );
        let row1 = Vec3::new(
            2.0 * (abs_rot.x * abs_rot.y - abs_rot.w * abs_rot.z),
            abs_rot.w * abs_rot.w - abs_rot.x * abs_rot.x + abs_rot.y * abs_rot.y
                - abs_rot.z * abs_rot.z,
            2.0 * (abs_rot.y * abs_rot.z + abs_rot.w * abs_rot.x),
        );
        let row2 = Vec3::new(
            2.0 * (abs_rot.x * abs_rot.z + abs_rot.w * abs_rot.y),
            2.0 * (abs_rot.y * abs_rot.z - abs_rot.w * abs_rot.x),
            abs_rot.w * abs_rot.w - abs_rot.x * abs_rot.x - abs_rot.y * abs_rot.y
                + abs_rot.z * abs_rot.z,
        );

        Vec3::new(
            (row0.x * half_extents.x + row0.y * half_extents.y + row0.z * half_extents.z).abs(),
            (row1.x * half_extents.x + row1.y * half_extents.y + row1.z * half_extents.z).abs(),
            (row2.x * half_extents.x + row2.y * half_extents.y + row2.z * half_extents.z).abs(),
        )
    }

    /// 计算凸包的AABB
    fn compute_convex_hull_aabb(points: &[Vec3], position: Vec3, rotation: Quat) -> AABB {
        if points.is_empty() {
            return AABB::from_center_half_extents(position, Vec3::ZERO);
        }

        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);

        for p in points {
            let rotated = rotation * *p;
            let world_pos = position + rotated;
            min.x = min.x.min(world_pos.x);
            min.y = min.y.min(world_pos.y);
            min.z = min.z.min(world_pos.z);
            max.x = max.x.max(world_pos.x);
            max.y = max.y.max(world_pos.y);
            max.z = max.z.max(world_pos.z);
        }

        AABB::new(min, max)
    }

    /// 计算质量
    pub fn compute_mass(&self, density: f32) -> f32 {
        match self {
            Self::Ball { radius } => (4.0 / 3.0) * PI * radius * radius * radius * density,
            Self::Cuboid { hx, hy, hz } => 8.0 * hx * hy * hz * density,
            Self::Capsule {
                half_height,
                radius,
                ..
            } => {
                // 胶囊 = 圆柱 + 两个半球
                let cylinder_volume = 2.0 * PI * radius * radius * half_height;
                let sphere_volume = (4.0 / 3.0) * PI * radius * radius * radius;
                (cylinder_volume + sphere_volume) * density
            }
            Self::Cylinder {
                half_height,
                radius,
            } => PI * radius * radius * 2.0 * half_height * density,
            Self::Cone {
                half_height,
                radius,
            } => (1.0 / 3.0) * PI * radius * radius * 2.0 * half_height * density,
            Self::ConvexHull { points } => Self::compute_convex_hull_volume(points) * density,
            Self::Trimesh { .. } | Self::Heightfield { .. } => {
                // 复杂形状，返回默认质量
                density
            }
            Self::Segment { .. } | Self::Triangle { .. } | Self::Halfspace { .. } => {
                // 无体积形状
                0.0
            }
        }
    }

    /// 计算凸包体积（简化实现）
    fn compute_convex_hull_volume(points: &[Vec3]) -> f32 {
        if points.len() < 4 {
            return 0.0;
        }
        // 简化：使用包围盒体积作为估计
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        let size = max - min;
        size.x * size.y * size.z
    }

    /// 计算质量属性
    pub fn compute_mass_properties(&self, density: f32) -> MassProperties {
        let mass = self.compute_mass(density);
        let center_of_mass = Vec3::ZERO;
        let principal_inertia = self.compute_inertia(mass);

        MassProperties {
            mass,
            center_of_mass,
            principal_inertia,
        }
    }

    /// 计算惯性
    fn compute_inertia(&self, mass: f32) -> Vec3 {
        match self {
            Self::Ball { radius } => {
                let i = 0.4 * mass * radius * radius;
                Vec3::splat(i)
            }
            Self::Cuboid { hx, hy, hz } => Vec3::new(
                mass * (hy * hy + hz * hz) / 3.0,
                mass * (hx * hx + hz * hz) / 3.0,
                mass * (hx * hx + hy * hy) / 3.0,
            ),
            Self::Capsule {
                half_height,
                radius,
                ..
            } => {
                let i_cylinder =
                    mass * (3.0 * radius * radius + 4.0 * half_height * half_height) / 12.0;
                Vec3::new(i_cylinder, i_cylinder, i_cylinder)
            }
            Self::Cylinder {
                half_height,
                radius,
            } => {
                let i_radial =
                    mass * (3.0 * radius * radius + 4.0 * half_height * half_height) / 12.0;
                let i_axial = 0.5 * mass * radius * radius;
                Vec3::new(i_radial, i_axial, i_radial)
            }
            Self::Cone {
                half_height,
                radius,
            } => {
                let i_radial =
                    mass * (3.0 * radius * radius + 4.0 * half_height * half_height) / 12.0;
                let i_axial = 0.3 * mass * radius * radius;
                Vec3::new(i_radial, i_axial, i_radial)
            }
            _ => Vec3::splat(mass),
        }
    }
}

/// 碰撞体句柄
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColliderHandle {
    /// 索引
    pub index: u32,
    /// 版本号
    pub generation: u32,
}

impl ColliderHandle {
    /// 创建新的句柄
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// 无效句柄
    pub const INVALID: Self = Self {
        index: u32::MAX,
        generation: u32::MAX,
    };

    /// 检查是否有效
    pub fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }
}

impl Default for ColliderHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

/// 碰撞体
///
/// 附加到刚体上的3D碰撞形状。
#[derive(Debug, Clone)]
pub struct Collider3D {
    /// 形状
    shape: ColliderShape3D,
    /// 碰撞体类型
    collider_type: ColliderType,
    /// 相对于刚体的位置
    local_position: Vec3,
    /// 相对于刚体的旋转
    local_rotation: Quat,
    /// 密度
    density: f32,
    /// 弹性系数
    restitution: f32,
    /// 摩擦系数
    friction: f32,
    /// 弹性组合规则
    restitution_combine_rule: CombineRule,
    /// 摩擦组合规则
    friction_combine_rule: CombineRule,
    /// 碰撞分组
    collision_groups: CollisionGroups,
    /// 解算器分组
    solver_groups: CollisionGroups,
    /// 接触力事件阈值
    contact_force_event_threshold: f32,
    /// 接触皮肤厚度
    contact_skin: f32,
    /// 是否启用
    enabled: bool,
    /// 所属刚体索引
    parent_body_index: Option<usize>,
}

impl Collider3D {
    /// 创建新的碰撞体
    pub fn new(shape: ColliderShape3D) -> Self {
        Self {
            shape,
            collider_type: ColliderType::Solid,
            local_position: Vec3::ZERO,
            local_rotation: Quat::IDENTITY,
            density: 1.0,
            restitution: DEFAULT_RESTITUTION,
            friction: DEFAULT_FRICTION,
            restitution_combine_rule: CombineRule::default(),
            friction_combine_rule: CombineRule::default(),
            collision_groups: CollisionGroups::default(),
            solver_groups: CollisionGroups::default(),
            contact_force_event_threshold: 0.0,
            contact_skin: 0.0,
            enabled: true,
            parent_body_index: None,
        }
    }

    /// 获取形状
    pub fn shape(&self) -> &ColliderShape3D {
        &self.shape
    }

    /// 获取碰撞体类型
    pub fn collider_type(&self) -> ColliderType {
        self.collider_type
    }

    /// 获取局部位置
    pub fn local_position(&self) -> Vec3 {
        self.local_position
    }

    /// 设置局部位置
    pub fn set_local_position(&mut self, position: Vec3) {
        self.local_position = position;
    }

    /// 获取局部旋转
    pub fn local_rotation(&self) -> Quat {
        self.local_rotation
    }

    /// 设置局部旋转
    pub fn set_local_rotation(&mut self, rotation: Quat) {
        self.local_rotation = rotation.normalize();
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
        self.restitution = restitution.clamp(0.0, 1.0);
    }

    /// 获取摩擦系数
    pub fn friction(&self) -> f32 {
        self.friction
    }

    /// 设置摩擦系数
    pub fn set_friction(&mut self, friction: f32) {
        self.friction = friction.clamp(0.0, 1.0);
    }

    /// 获取弹性组合规则
    pub fn restitution_combine_rule(&self) -> CombineRule {
        self.restitution_combine_rule
    }

    /// 设置弹性组合规则
    pub fn set_restitution_combine_rule(&mut self, rule: CombineRule) {
        self.restitution_combine_rule = rule;
    }

    /// 获取摩擦组合规则
    pub fn friction_combine_rule(&self) -> CombineRule {
        self.friction_combine_rule
    }

    /// 设置摩擦组合规则
    pub fn set_friction_combine_rule(&mut self, rule: CombineRule) {
        self.friction_combine_rule = rule;
    }

    /// 获取碰撞分组
    pub fn collision_groups(&self) -> CollisionGroups {
        self.collision_groups
    }

    /// 设置碰撞分组
    pub fn set_collision_groups(&mut self, groups: CollisionGroups) {
        self.collision_groups = groups;
    }

    /// 获取解算器分组
    pub fn solver_groups(&self) -> CollisionGroups {
        self.solver_groups
    }

    /// 设置解算器分组
    pub fn set_solver_groups(&mut self, groups: CollisionGroups) {
        self.solver_groups = groups;
    }

    /// 检查是否是传感器
    pub fn is_sensor(&self) -> bool {
        self.collider_type == ColliderType::Sensor
    }

    /// 设置是否为传感器
    pub fn set_sensor(&mut self, sensor: bool) {
        self.collider_type = if sensor {
            ColliderType::Sensor
        } else {
            ColliderType::Solid
        };
    }

    /// 获取接触力事件阈值
    pub fn contact_force_event_threshold(&self) -> f32 {
        self.contact_force_event_threshold
    }

    /// 设置接触力事件阈值
    pub fn set_contact_force_event_threshold(&mut self, threshold: f32) {
        self.contact_force_event_threshold = threshold;
    }

    /// 获取接触皮肤厚度
    pub fn contact_skin(&self) -> f32 {
        self.contact_skin
    }

    /// 设置接触皮肤厚度
    pub fn set_contact_skin(&mut self, skin: f32) {
        self.contact_skin = skin;
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 获取所属刚体索引
    pub fn parent_body_index(&self) -> Option<usize> {
        self.parent_body_index
    }

    /// 设置所属刚体索引
    pub fn set_parent_body_index(&mut self, index: Option<usize>) {
        self.parent_body_index = index;
    }

    /// 计算世界坐标位置
    pub fn world_position(&self, body_position: Vec3, body_rotation: Quat) -> Vec3 {
        body_rotation * self.local_position + body_position
    }

    /// 计算世界旋转
    pub fn world_rotation(&self, body_rotation: Quat) -> Quat {
        body_rotation * self.local_rotation
    }

    /// 计算世界AABB
    pub fn aabb(&self, body_position: Vec3, body_rotation: Quat) -> AABB {
        let world_pos = self.world_position(body_position, body_rotation);
        let world_rot = self.world_rotation(body_rotation);
        self.shape.compute_aabb(world_pos, world_rot)
    }

    /// 获取质量
    pub fn mass(&self) -> f32 {
        self.shape.compute_mass(self.density)
    }

    /// 获取质量属性
    pub fn mass_properties(&self) -> MassProperties {
        self.shape.compute_mass_properties(self.density)
    }
}

/// 碰撞体构建器
pub struct Collider3DBuilder {
    collider: Collider3D,
}

impl Collider3DBuilder {
    /// 创建球体碰撞体
    pub fn ball(radius: f32) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::ball(radius)),
        }
    }

    /// 创建立方体碰撞体
    pub fn cuboid(hx: f32, hy: f32, hz: f32) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::cuboid(hx, hy, hz)),
        }
    }

    /// 创建胶囊体碰撞体
    pub fn capsule(half_height: f32, radius: f32, axis: Axis) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::capsule(half_height, radius, axis)),
        }
    }

    /// 创建圆柱体碰撞体
    pub fn cylinder(half_height: f32, radius: f32) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::cylinder(half_height, radius)),
        }
    }

    /// 创建圆锥体碰撞体
    pub fn cone(half_height: f32, radius: f32) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::cone(half_height, radius)),
        }
    }

    /// 创建凸包碰撞体
    pub fn convex_hull(points: Vec<Vec3>) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::convex_hull(points)),
        }
    }

    /// 创建三角网格碰撞体
    pub fn trimesh(vertices: Vec<Vec3>, indices: Vec<[u32; 3]>) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::trimesh(vertices, indices)),
        }
    }

    /// 创建高度场碰撞体
    pub fn heightfield(heights: Vec<f32>, scale: Vec3) -> Self {
        Self {
            collider: Collider3D::new(ColliderShape3D::heightfield(heights, scale)),
        }
    }

    /// 设置相对位置
    pub fn translation(mut self, position: Vec3) -> Self {
        self.collider.set_local_position(position);
        self
    }

    /// 设置相对旋转
    pub fn rotation(mut self, rotation: Quat) -> Self {
        self.collider.set_local_rotation(rotation);
        self
    }

    /// 设置密度
    pub fn density(mut self, density: f32) -> Self {
        self.collider.set_density(density);
        self
    }

    /// 设置质量
    pub fn mass(mut self, mass: f32) -> Self {
        // 根据质量计算密度
        let shape_mass = self.collider.shape.compute_mass(1.0);
        if shape_mass > 0.0 {
            self.collider.set_density(mass / shape_mass);
        }
        self
    }

    /// 设置质量属性
    pub fn mass_properties(mut self, mp: MassProperties) -> Self {
        self.collider
            .set_density(mp.mass / self.collider.shape.compute_mass(1.0).max(0.001));
        self
    }

    /// 设置弹性系数
    pub fn restitution(mut self, restitution: f32) -> Self {
        self.collider.set_restitution(restitution);
        self
    }

    /// 设置摩擦系数
    pub fn friction(mut self, friction: f32) -> Self {
        self.collider.set_friction(friction);
        self
    }

    /// 设置弹性组合规则
    pub fn restitution_combine_rule(mut self, rule: CombineRule) -> Self {
        self.collider.set_restitution_combine_rule(rule);
        self
    }

    /// 设置摩擦组合规则
    pub fn friction_combine_rule(mut self, rule: CombineRule) -> Self {
        self.collider.set_friction_combine_rule(rule);
        self
    }

    /// 设置碰撞分组
    pub fn collision_groups(mut self, groups: CollisionGroups) -> Self {
        self.collider.set_collision_groups(groups);
        self
    }

    /// 设置解算器分组
    pub fn solver_groups(mut self, groups: CollisionGroups) -> Self {
        self.collider.set_solver_groups(groups);
        self
    }

    /// 设置为传感器
    pub fn sensor(mut self, sensor: bool) -> Self {
        self.collider.set_sensor(sensor);
        self
    }

    /// 设置接触力事件阈值
    pub fn contact_force_event_threshold(mut self, threshold: f32) -> Self {
        self.collider.set_contact_force_event_threshold(threshold);
        self
    }

    /// 设置接触皮肤厚度
    pub fn contact_skin(mut self, skin: f32) -> Self {
        self.collider.set_contact_skin(skin);
        self
    }

    /// 构建碰撞体
    pub fn build(self) -> Collider3D {
        self.collider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_collider() {
        let collider = Collider3DBuilder::ball(1.0).build();
        assert!(matches!(collider.shape(), ColliderShape3D::Ball { .. }));
    }

    #[test]
    fn test_cuboid_collider() {
        let collider = Collider3DBuilder::cuboid(1.0, 2.0, 3.0).build();
        assert!(matches!(collider.shape(), ColliderShape3D::Cuboid { .. }));
    }

    #[test]
    fn test_collider_mass() {
        let collider = Collider3DBuilder::ball(1.0).density(2.0).build();
        let expected_mass = (4.0 / 3.0) * PI * 1.0 * 1.0 * 1.0 * 2.0;
        assert!((collider.mass() - expected_mass).abs() < 0.001);
    }

    #[test]
    fn test_collider_builder() {
        let collider = Collider3DBuilder::ball(0.5)
            .translation(Vec3::new(1.0, 2.0, 3.0))
            .density(3.0)
            .restitution(0.8)
            .friction(0.4)
            .sensor(true)
            .build();

        assert_eq!(collider.local_position(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(collider.density(), 3.0);
        assert_eq!(collider.restitution(), 0.8);
        assert_eq!(collider.friction(), 0.4);
        assert!(collider.is_sensor());
    }

    #[test]
    fn test_shape_aabb() {
        let shape = ColliderShape3D::cuboid(1.0, 2.0, 3.0);
        let aabb = shape.compute_aabb(Vec3::ZERO, Quat::IDENTITY);
        assert_eq!(aabb.center(), Vec3::ZERO);
        assert_eq!(aabb.half_extents(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_shape_ball_aabb() {
        let shape = ColliderShape3D::ball(5.0);
        let aabb = shape.compute_aabb(Vec3::new(10.0, 10.0, 10.0), Quat::IDENTITY);
        assert_eq!(aabb.center(), Vec3::new(10.0, 10.0, 10.0));
        assert_eq!(aabb.half_extents(), Vec3::splat(5.0));
    }

    #[test]
    fn test_collision_groups() {
        let collider = Collider3DBuilder::ball(1.0)
            .collision_groups(CollisionGroups::new(0x0001, 0xFFFF))
            .build();
        assert_eq!(collider.collision_groups().memberships, 0x0001);
    }

    #[test]
    fn test_capsule_collider() {
        let collider = Collider3DBuilder::capsule(1.0, 0.5, Axis::Y).build();
        assert!(matches!(collider.shape(), ColliderShape3D::Capsule { .. }));
    }
}
