//! engine-physics-3d crate - 3D 物理引擎
//!
//! 提供 3D 物理仿真，包括刚体、碰撞检测、关节、角色控制器等。

#![warn(missing_docs)]

pub mod character;
pub mod collider;
pub mod collision;
pub mod joint;
pub mod query;
pub mod rigidbody;
pub mod world;

pub use character::{CharacterController3D, CharacterMovement};
pub use collider::{Collider3D, Collider3DBuilder, ColliderHandle, ColliderShape3D, ColliderType};
pub use collision::{
    ContactEvent, ContactForceEvent, ContactPair, ContactPoint3D, IntersectionEvent, Manifold3D,
};
pub use joint::{
    BallJointBuilder, DistanceJointBuilder, FixedJointBuilder, Joint3D, JointHandle, JointType3D,
    PrismaticJointBuilder, RevoluteJointBuilder, RopeJointBuilder, SphericalJointBuilder,
};
pub use query::{Query3D, QueryFilter, Ray3, RayCastHit, ShapeCastHit};
pub use rigidbody::{
    RigidBody3D, RigidBody3DBuilder, RigidBodyHandle, RigidBodyState3D, RigidBodyType3D,
};
pub use world::{PhysicsWorld3D, PhysicsWorldConfig3D};

/// 碰撞分组
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionGroups {
    /// 分组成员身份
    pub memberships: u32,
    /// 碰撞过滤掩码
    pub filter: u32,
}

impl CollisionGroups {
    /// 创建新的碰撞分组
    pub fn new(memberships: u32, filter: u32) -> Self {
        Self {
            memberships,
            filter,
        }
    }

    /// 所有分组
    pub const ALL: Self = Self {
        memberships: 0xFFFFFFFF,
        filter: 0xFFFFFFFF,
    };

    /// 无分组
    pub const NONE: Self = Self {
        memberships: 0,
        filter: 0,
    };

    /// 默认分组
    pub const DEFAULT: Self = Self {
        memberships: 0x0001,
        filter: 0xFFFF,
    };

    /// 检查两个分组是否可以碰撞
    pub fn can_collide_with(&self, other: &CollisionGroups) -> bool {
        (self.filter & other.memberships) != 0 && (other.filter & self.memberships) != 0
    }
}

impl Default for CollisionGroups {
    fn default() -> Self {
        Self::ALL
    }
}

/// 材质组合规则
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombineRule {
    /// 平均值
    #[default]
    Average,
    /// 取最小值
    Min,
    /// 取最大值
    Max,
    /// 乘法
    Multiply,
}

/// 轴类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    /// X轴
    #[default]
    X,
    /// Y轴
    Y,
    /// Z轴
    Z,
}

impl Axis {
    /// 获取轴向量
    pub fn vector(&self) -> engine_math::Vec3 {
        match self {
            Axis::X => engine_math::Vec3::X,
            Axis::Y => engine_math::Vec3::Y,
            Axis::Z => engine_math::Vec3::Z,
        }
    }
}

/// 质量属性
#[derive(Debug, Clone, Copy)]
pub struct MassProperties {
    /// 质量
    pub mass: f32,
    /// 质心
    pub center_of_mass: engine_math::Vec3,
    /// 主惯性轴
    pub principal_inertia: engine_math::Vec3,
}

impl Default for MassProperties {
    fn default() -> Self {
        Self {
            mass: 1.0,
            center_of_mass: engine_math::Vec3::ZERO,
            principal_inertia: engine_math::Vec3::splat(1.0),
        }
    }
}

/// AABB包围盒
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    /// 最小点
    pub min: engine_math::Vec3,
    /// 最大点
    pub max: engine_math::Vec3,
}

impl AABB {
    /// 创建新的AABB
    pub fn new(min: engine_math::Vec3, max: engine_math::Vec3) -> Self {
        Self { min, max }
    }

    /// 从中心和半尺寸创建
    pub fn from_center_half_extents(
        center: engine_math::Vec3,
        half_extents: engine_math::Vec3,
    ) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
        }
    }

    /// 从中心和半径创建（用于球体）
    pub fn from_center_radius(center: engine_math::Vec3, radius: f32) -> Self {
        Self::from_center_half_extents(center, engine_math::Vec3::splat(radius))
    }

    /// 获取中心
    pub fn center(&self) -> engine_math::Vec3 {
        (self.min + self.max) * 0.5
    }

    /// 获取半尺寸
    pub fn half_extents(&self) -> engine_math::Vec3 {
        (self.max - self.min) * 0.5
    }

    /// 获取尺寸
    pub fn extents(&self) -> engine_math::Vec3 {
        self.max - self.min
    }

    /// 检查是否包含点
    pub fn contains(&self, point: engine_math::Vec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    /// 检查是否与另一个AABB相交
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// 合并两个AABB
    pub fn merge(&self, other: &AABB) -> Self {
        Self {
            min: engine_math::Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: engine_math::Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    /// 扩展AABB
    pub fn expand(&self, amount: engine_math::Vec3) -> Self {
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    /// 计算体积
    pub fn volume(&self) -> f32 {
        let size = self.extents();
        size.x * size.y * size.z
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self::new(engine_math::Vec3::ZERO, engine_math::Vec3::ONE)
    }
}

/// 物理实体句柄（用于标识刚体、碰撞体、关节等）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityHandle {
    /// 索引
    pub index: u32,
    /// 版本号（用于检测无效句柄）
    pub generation: u32,
}

impl EntityHandle {
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

impl Default for EntityHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

/// 物理调试渲染器trait
pub trait DebugRenderer {
    /// 绘制线
    fn draw_line(&mut self, start: engine_math::Vec3, end: engine_math::Vec3, color: [f32; 4]);
    /// 绘制三角形
    fn draw_triangle(
        &mut self,
        a: engine_math::Vec3,
        b: engine_math::Vec3,
        c: engine_math::Vec3,
        color: [f32; 4],
    );
}

/// 物理常量
pub mod constants {
    /// 默认重力加速度 (m/s²)
    pub const DEFAULT_GRAVITY: f32 = -9.81;
    /// 默认物理步长 (秒)
    pub const DEFAULT_TIMESTEP: f32 = 1.0 / 60.0;
    /// 默认速度迭代次数
    pub const DEFAULT_VELOCITY_ITERATIONS: usize = 8;
    /// 默认位置迭代次数
    pub const DEFAULT_POSITION_ITERATIONS: usize = 3;
    /// 默认弹性系数
    pub const DEFAULT_RESTITUTION: f32 = 0.3;
    /// 默认摩擦系数
    pub const DEFAULT_FRICTION: f32 = 0.5;
    /// 默认线性阻尼
    pub const DEFAULT_LINEAR_DAMPING: f32 = 0.01;
    /// 默认角阻尼
    pub const DEFAULT_ANGULAR_DAMPING: f32 = 0.01;
    /// 最大子步数
    pub const MAX_SUBSTEPS: usize = 4;
    /// 穿透容差
    pub const PENETRATION_SLOP: f32 = 0.005;
    /// Baumgarte稳定化系数
    pub const BAUMGARTE: f32 = 0.2;
    /// 睡眠阈值（速度）
    pub const SLEEP_VELOCITY_THRESHOLD: f32 = 0.1;
    /// 睡眠阈值（角速度）
    pub const SLEEP_ANGULAR_THRESHOLD: f32 = 0.1;
    /// 睡眠时间阈值（秒）
    pub const SLEEP_TIME_THRESHOLD: f32 = 1.0;
    /// 最大速度限制
    pub const MAX_VELOCITY: f32 = 1000.0;
    /// 最大角速度限制
    pub const MAX_ANGULAR_VELOCITY: f32 = 100.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_groups() {
        let group1 = CollisionGroups::new(0x0001, 0xFFFF);
        let group2 = CollisionGroups::new(0x0002, 0xFFFF);
        let group3 = CollisionGroups::new(0x0001, 0x0001);

        assert!(group1.can_collide_with(&group2));
        assert!(group2.can_collide_with(&group1));
        assert!(group1.can_collide_with(&group3));
        assert!(!group3.can_collide_with(&group2));
    }

    #[test]
    fn test_aabb() {
        let aabb1 =
            AABB::from_center_half_extents(engine_math::Vec3::ZERO, engine_math::Vec3::splat(1.0));
        let aabb2 = AABB::from_center_half_extents(
            engine_math::Vec3::new(2.0, 0.0, 0.0),
            engine_math::Vec3::splat(1.0),
        );

        assert!(aabb1.contains(engine_math::Vec3::ZERO));
        assert!(!aabb1.contains(engine_math::Vec3::new(2.0, 0.0, 0.0)));
        assert!(aabb1.intersects(&aabb2));
    }

    #[test]
    fn test_aabb_volume() {
        let aabb = AABB::from_center_half_extents(
            engine_math::Vec3::ZERO,
            engine_math::Vec3::new(1.0, 2.0, 3.0),
        );
        assert_eq!(aabb.volume(), 2.0 * 4.0 * 6.0);
    }

    #[test]
    fn test_aabb_merge() {
        let aabb1 =
            AABB::from_center_half_extents(engine_math::Vec3::ZERO, engine_math::Vec3::splat(1.0));
        let aabb2 = AABB::from_center_half_extents(
            engine_math::Vec3::new(3.0, 0.0, 0.0),
            engine_math::Vec3::splat(1.0),
        );
        let merged = aabb1.merge(&aabb2);
        assert_eq!(merged.min, engine_math::Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(merged.max, engine_math::Vec3::new(4.0, 1.0, 1.0));
    }

    #[test]
    fn test_entity_handle() {
        let handle = EntityHandle::new(0, 0);
        assert!(handle.is_valid());
        assert!(!EntityHandle::INVALID.is_valid());
    }

    #[test]
    fn test_axis() {
        assert_eq!(Axis::X.vector(), engine_math::Vec3::X);
        assert_eq!(Axis::Y.vector(), engine_math::Vec3::Y);
        assert_eq!(Axis::Z.vector(), engine_math::Vec3::Z);
    }

    #[test]
    fn test_mass_properties() {
        let mp = MassProperties::default();
        assert_eq!(mp.mass, 1.0);
        assert_eq!(mp.center_of_mass, engine_math::Vec3::ZERO);
    }

    #[test]
    fn test_combine_rule() {
        assert_eq!(CombineRule::default(), CombineRule::Average);
    }
}
