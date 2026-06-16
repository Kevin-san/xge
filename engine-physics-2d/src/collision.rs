//! 碰撞模块
//!
//! 提供碰撞检测和响应所需的数据结构。

use engine_math::Vec2;

/// 接触点
#[derive(Debug, Clone)]
pub struct Contact {
    /// 接触点位置
    pub position: Vec2,
    /// 碰撞法线（从 A 指向 B）
    pub normal: Vec2,
    /// 穿透深度
    pub penetration: f32,
    /// 法向冲量
    pub normal_impulse: f32,
    /// 切向冲量
    pub tangent_impulse: f32,
}

impl Contact {
    /// 创建新的接触点
    pub fn new(position: Vec2, normal: Vec2, penetration: f32) -> Self {
        Self {
            position,
            normal,
            penetration,
            normal_impulse: 0.0,
            tangent_impulse: 0.0,
        }
    }
}

/// 碰撞流形
///
/// 描述两个碰撞体之间的接触信息。
#[derive(Debug, Clone)]
pub struct Manifold {
    /// 碰撞体 A 索引
    pub body_a: usize,
    /// 碰撞体 B 索引
    pub body_b: usize,
    /// 接触点列表
    pub contacts: Vec<Contact>,
    /// 碰撞法线（从 A 指向 B）
    pub normal: Vec2,
    /// 穿透深度
    pub penetration: f32,
}

impl Manifold {
    /// 创建新的碰撞流形
    pub fn new(body_a: usize, body_b: usize) -> Self {
        Self {
            body_a,
            body_b,
            contacts: Vec::new(),
            normal: Vec2::ZERO,
            penetration: 0.0,
        }
    }

    /// 添加接触点
    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    /// 获取接触点数量
    pub fn contact_count(&self) -> usize {
        self.contacts.len()
    }
}

/// 碰撞事件类型
#[derive(Debug, Clone)]
pub enum CollisionEvent {
    /// 碰撞开始
    Started {
        /// 碰撞体 A 索引
        body_a: usize,
        /// 碰撞体 B 索引
        body_b: usize,
        /// 碰撞流形
        manifold: Manifold,
    },
    /// 碰撞结束
    Ended {
        /// 碰撞体 A 索引
        body_a: usize,
        /// 碰撞体 B 索引
        body_b: usize,
    },
}

/// 碰撞分组
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionGroup(u32);

impl CollisionGroup {
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
    pub fn intersects(&self, other: &CollisionGroup) -> bool {
        (self.0 & other.0) != 0
    }
}

impl Default for CollisionGroup {
    fn default() -> Self {
        Self::all()
    }
}

/// 碰撞过滤规则
#[derive(Debug, Clone)]
pub struct CollisionFilter {
    /// 分组
    pub group: CollisionGroup,
    /// 掩码 A
    pub mask_a: u32,
    /// 掩码 B
    pub mask_b: u32,
}

impl CollisionFilter {
    /// 检查两个过滤器是否应该产生碰撞
    pub fn should_collide(&self, other: &CollisionFilter) -> bool {
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

impl Default for CollisionFilter {
    fn default() -> Self {
        Self {
            group: CollisionGroup::all(),
            mask_a: 0xFFFFFFFF,
            mask_b: 0xFFFFFFFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifold() {
        let mut manifold = Manifold::new(0, 1);
        manifold.normal = Vec2::new(1.0, 0.0);
        manifold.penetration = 0.1;

        manifold.add_contact(Contact::new(
            Vec2::new(0.5, 0.0),
            Vec2::new(-1.0, 0.0),
            0.1,
        ));

        assert_eq!(manifold.contact_count(), 1);
    }

    #[test]
    fn test_collision_group() {
        let group1 = CollisionGroup::new(0b0001);
        let group2 = CollisionGroup::new(0b0010);
        let group3 = CollisionGroup::new(0b0001 | 0b0010);

        assert!(group1.intersects(&group3));
        assert!(!group1.intersects(&group2));
    }

    #[test]
    fn test_collision_filter() {
        let filter1 = CollisionFilter {
            group: CollisionGroup::new(0b0001),
            mask_a: 0b0001,
            mask_b: 0b0001,
        };

        let filter2 = CollisionFilter {
            group: CollisionGroup::new(0b0001),
            mask_a: 0b0001,
            mask_b: 0b0001,
        };

        let filter3 = CollisionFilter {
            group: CollisionGroup::new(0b0010),
            mask_a: 0b0010,
            mask_b: 0b0010,
        };

        assert!(filter1.should_collide(&filter2));
        assert!(!filter1.should_collide(&filter3));
    }

    #[test]
    fn test_contact() {
        let contact = Contact::new(
            Vec2::new(1.0, 2.0),
            Vec2::new(0.0, 1.0),
            0.5,
        );

        assert_eq!(contact.position, Vec2::new(1.0, 2.0));
        assert_eq!(contact.normal, Vec2::new(0.0, 1.0));
        assert_eq!(contact.penetration, 0.5);
        assert_eq!(contact.normal_impulse, 0.0);
    }
}
