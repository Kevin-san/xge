//! 碰撞模块
//!
//! 提供 3D 碰撞检测和响应所需的数据结构。

use engine_math::Vec3;

use crate::collider::ColliderHandle;

/// 接触点
#[derive(Debug, Clone)]
pub struct ContactPoint3D {
    /// 接触点位置
    pub point: Vec3,
    /// 穿透深度
    pub penetration: f32,
    /// 法向冲量
    pub normal_impulse: f32,
    /// 切向冲量
    pub tangent_impulse: Vec3,
}

impl ContactPoint3D {
    /// 创建新的接触点
    pub fn new(point: Vec3, penetration: f32) -> Self {
        Self {
            point,
            penetration,
            normal_impulse: 0.0,
            tangent_impulse: Vec3::ZERO,
        }
    }

    /// 获取接触点位置
    pub fn point(&self) -> Vec3 {
        self.point
    }

    /// 获取穿透深度
    pub fn penetration(&self) -> f32 {
        self.penetration
    }
}

/// 接触对
///
/// 描述两个碰撞体之间的接触信息。
#[derive(Debug, Clone)]
pub struct ContactPair {
    /// 碰撞体 A 句柄
    pub collider_a: ColliderHandle,
    /// 碰撞体 B 句柄
    pub collider_b: ColliderHandle,
    /// 接触点列表
    pub points: Vec<ContactPoint3D>,
    /// 碰撞法线（从 A 指向 B）
    pub normal: Vec3,
    /// 穿透深度
    pub penetration: f32,
}

impl ContactPair {
    /// 创建新的接触对
    pub fn new(collider_a: ColliderHandle, collider_b: ColliderHandle) -> Self {
        Self {
            collider_a,
            collider_b,
            points: Vec::new(),
            normal: Vec3::ZERO,
            penetration: 0.0,
        }
    }

    /// 添加接触点
    pub fn add_contact(&mut self, contact: ContactPoint3D) {
        self.points.push(contact);
    }

    /// 获取接触点数量
    pub fn contact_count(&self) -> usize {
        self.points.len()
    }

    /// 获取法线
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// 获取接触点列表
    pub fn points(&self) -> &[ContactPoint3D] {
        &self.points
    }

    /// 获取最大穿透深度
    pub fn max_penetration(&self) -> f32 {
        self.points.iter().map(|p| p.penetration).fold(0.0, |a, b| a.max(b))
    }
}

/// 接触事件
#[derive(Debug, Clone)]
pub enum ContactEvent {
    /// 碰撞开始
    Started(ColliderHandle, ColliderHandle),
    /// 碰撞结束
    Stopped(ColliderHandle, ColliderHandle),
}

impl ContactEvent {
    /// 获取碰撞体 A
    pub fn collider_a(&self) -> ColliderHandle {
        match self {
            ContactEvent::Started(a, _) => *a,
            ContactEvent::Stopped(a, _) => *a,
        }
    }

    /// 获取碰撞体 B
    pub fn collider_b(&self) -> ColliderHandle {
        match self {
            ContactEvent::Started(_, b) => *b,
            ContactEvent::Stopped(_, b) => *b,
        }
    }

    /// 检查是否是碰撞开始
    pub fn is_started(&self) -> bool {
        matches!(self, ContactEvent::Started(_, _))
    }

    /// 检查是否是碰撞结束
    pub fn is_stopped(&self) -> bool {
        matches!(self, ContactEvent::Stopped(_, _))
    }
}

/// 相交事件
#[derive(Debug, Clone)]
pub enum IntersectionEvent {
    /// 相交开始
    Started(ColliderHandle, ColliderHandle),
    /// 相交结束
    Stopped(ColliderHandle, ColliderHandle),
}

impl IntersectionEvent {
    /// 获取碰撞体 A
    pub fn collider_a(&self) -> ColliderHandle {
        match self {
            IntersectionEvent::Started(a, _) => *a,
            IntersectionEvent::Stopped(a, _) => *a,
        }
    }

    /// 获取碰撞体 B
    pub fn collider_b(&self) -> ColliderHandle {
        match self {
            IntersectionEvent::Started(_, b) => *b,
            IntersectionEvent::Stopped(_, b) => *b,
        }
    }
}

/// 接触力事件
#[derive(Debug, Clone)]
pub struct ContactForceEvent {
    /// 碰撞体句柄
    pub handles: (ColliderHandle, ColliderHandle),
    /// 总接触力
    pub total_force: Vec3,
    /// 总力大小
    pub total_magnitude: f32,
}

impl ContactForceEvent {
    /// 创建新的接触力事件
    pub fn new(collider_a: ColliderHandle, collider_b: ColliderHandle) -> Self {
        Self {
            handles: (collider_a, collider_b),
            total_force: Vec3::ZERO,
            total_magnitude: 0.0,
        }
    }

    /// 创建带阈值的接触力事件
    pub fn with_threshold(threshold: f32) -> Self {
        Self {
            handles: (ColliderHandle::INVALID, ColliderHandle::INVALID),
            total_force: Vec3::ZERO,
            total_magnitude: threshold,
        }
    }

    /// 获取总接触力
    pub fn total_force(&self) -> Vec3 {
        self.total_force
    }

    /// 获取总力大小
    pub fn total_magnitude(&self) -> f32 {
        self.total_magnitude
    }

    /// 获取碰撞体 A
    pub fn collider_a(&self) -> ColliderHandle {
        self.handles.0
    }

    /// 获取碰撞体 B
    pub fn collider_b(&self) -> ColliderHandle {
        self.handles.1
    }

    /// 设置总接触力
    pub fn set_total_force(&mut self, force: Vec3) {
        self.total_force = force;
        self.total_magnitude = force.length();
    }
}

/// 碰撞流形
///
/// 描述两个碰撞体之间的详细碰撞信息。
#[derive(Debug, Clone)]
pub struct Manifold3D {
    /// 碰撞体 A 索引
    pub body_a: usize,
    /// 碰撞体 B 索引
    pub body_b: usize,
    /// 接触点列表
    pub contacts: Vec<ContactPoint3D>,
    /// 碰撞法线（从 A 指向 B）
    pub normal: Vec3,
    /// 穿透深度
    pub penetration: f32,
}

impl Manifold3D {
    /// 创建新的碰撞流形
    pub fn new(body_a: usize, body_b: usize) -> Self {
        Self {
            body_a,
            body_b,
            contacts: Vec::new(),
            normal: Vec3::ZERO,
            penetration: 0.0,
        }
    }

    /// 添加接触点
    pub fn add_contact(&mut self, contact: ContactPoint3D) {
        self.contacts.push(contact);
    }

    /// 获取接触点数量
    pub fn contact_count(&self) -> usize {
        self.contacts.len()
    }
}

/// 碰撞事件类型
#[derive(Debug, Clone)]
pub enum CollisionEvent3D {
    /// 碰撞开始
    Started {
        /// 碰撞体 A 索引
        body_a: usize,
        /// 碰撞体 B 索引
        body_b: usize,
        /// 碰撞流形
        manifold: Manifold3D,
    },
    /// 碰撞结束
    Ended {
        /// 碰撞体 A 索引
        body_a: usize,
        /// 碰撞体 B 索引
        body_b: usize,
    },
}

impl CollisionEvent3D {
    /// 获取碰撞体 A
    pub fn body_a(&self) -> usize {
        match self {
            CollisionEvent3D::Started { body_a, .. } => *body_a,
            CollisionEvent3D::Ended { body_a, .. } => *body_a,
        }
    }

    /// 获取碰撞体 B
    pub fn body_b(&self) -> usize {
        match self {
            CollisionEvent3D::Started { body_b, .. } => *body_b,
            CollisionEvent3D::Ended { body_b, .. } => *body_b,
        }
    }
}

/// 碰撞分组（用于过滤碰撞）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionGroup3D(u32);

impl CollisionGroup3D {
    /// 创建新的碰撞分组
    pub fn new(bits: u32) -> Self {
        Self(bits)
    }

    /// 空分组
    pub fn none() -> Self {
        Self(0)
    }

    /// 默认分组
    pub fn all() -> Self {
        Self(0xFFFFFFFF)
    }

    /// 获取位
    pub fn bits(&self) -> u32 {
        self.0
    }

    /// 设置位
    pub fn with_bit(mut self, bit: u32) -> Self {
        self.0 |= 1 << bit;
        self
    }

    /// 检查是否包含某位
    pub fn contains(&self, bit: u32) -> bool {
        (self.0 & (1 << bit)) != 0
    }

    /// 检查是否与其他分组有交集
    pub fn intersects(&self, other: &CollisionGroup3D) -> bool {
        (self.0 & other.0) != 0
    }
}

impl Default for CollisionGroup3D {
    fn default() -> Self {
        Self::all()
    }
}

/// 碰撞过滤规则
#[derive(Debug, Clone)]
pub struct CollisionFilter3D {
    /// 分组
    pub group: CollisionGroup3D,
    /// 掩码 A
    pub mask_a: u32,
    /// 掩码 B
    pub mask_b: u32,
}

impl CollisionFilter3D {
    /// 检查两个过滤器是否应该产生碰撞
    pub fn should_collide(&self, other: &CollisionFilter3D) -> bool {
        // 检查分组
        if !self.group.intersects(&other.group) {
            return false;
        }

        // 检查掩码
        if (self.mask_a & other.mask_b) == 0 {
            return false;
        }
        if (self.mask_b & other.mask_a) == 0 {
            return false;
        }

        true
    }
}

impl Default for CollisionFilter3D {
    fn default() -> Self {
        Self {
            group: CollisionGroup3D::all(),
            mask_a: 0xFFFFFFFF,
            mask_b: 0xFFFFFFFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_point() {
        let contact = ContactPoint3D::new(Vec3::new(1.0, 2.0, 3.0), 0.5);
        assert_eq!(contact.point(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(contact.penetration(), 0.5);
    }

    #[test]
    fn test_contact_pair() {
        let mut pair = ContactPair::new(ColliderHandle::new(0, 0), ColliderHandle::new(1, 0));
        pair.add_contact(ContactPoint3D::new(Vec3::ZERO, 0.1));
        assert_eq!(pair.contact_count(), 1);
    }

    #[test]
    fn test_contact_event() {
        let event = ContactEvent::Started(ColliderHandle::new(0, 0), ColliderHandle::new(1, 0));
        assert!(event.is_started());
        assert!(!event.is_stopped());
    }

    #[test]
    fn test_intersection_event() {
        let event = IntersectionEvent::Stopped(ColliderHandle::new(0, 0), ColliderHandle::new(1, 0));
        assert_eq!(event.collider_a(), ColliderHandle::new(0, 0));
        assert_eq!(event.collider_b(), ColliderHandle::new(1, 0));
    }

    #[test]
    fn test_contact_force_event() {
        let mut event = ContactForceEvent::new(ColliderHandle::new(0, 0), ColliderHandle::new(1, 0));
        event.set_total_force(Vec3::new(10.0, 0.0, 0.0));
        assert_eq!(event.total_force(), Vec3::new(10.0, 0.0, 0.0));
        assert!((event.total_magnitude() - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_manifold() {
        let mut manifold = Manifold3D::new(0, 1);
        manifold.normal = Vec3::new(1.0, 0.0, 0.0);
        manifold.penetration = 0.1;

        manifold.add_contact(ContactPoint3D::new(Vec3::new(0.5, 0.0, 0.0), 0.1));

        assert_eq!(manifold.contact_count(), 1);
    }

    #[test]
    fn test_collision_group() {
        let group1 = CollisionGroup3D::new(0b0001);
        let group2 = CollisionGroup3D::new(0b0010);
        let group3 = CollisionGroup3D::new(0b0001 | 0b0010);

        assert!(group1.intersects(&group3));
        assert!(!group1.intersects(&group2));
    }

    #[test]
    fn test_collision_filter() {
        let filter1 = CollisionFilter3D {
            group: CollisionGroup3D::new(0b0001),
            mask_a: 0b0001,
            mask_b: 0b0001,
        };

        let filter2 = CollisionFilter3D {
            group: CollisionGroup3D::new(0b0001),
            mask_a: 0b0001,
            mask_b: 0b0001,
        };

        let filter3 = CollisionFilter3D {
            group: CollisionGroup3D::new(0b0010),
            mask_a: 0b0010,
            mask_b: 0b0010,
        };

        assert!(filter1.should_collide(&filter2));
        assert!(!filter1.should_collide(&filter3));
    }

    #[test]
    fn test_collision_event_3d() {
        let event = CollisionEvent3D::Started {
            body_a: 0,
            body_b: 1,
            manifold: Manifold3D::new(0, 1),
        };
        assert_eq!(event.body_a(), 0);
        assert_eq!(event.body_b(), 1);
    }

    #[test]
    fn test_contact_pair_max_penetration() {
        let mut pair = ContactPair::new(ColliderHandle::new(0, 0), ColliderHandle::new(1, 0));
        pair.add_contact(ContactPoint3D::new(Vec3::ZERO, 0.1));
        pair.add_contact(ContactPoint3D::new(Vec3::ZERO, 0.5));
        pair.add_contact(ContactPoint3D::new(Vec3::ZERO, 0.2));
        assert_eq!(pair.max_penetration(), 0.5);
    }
}