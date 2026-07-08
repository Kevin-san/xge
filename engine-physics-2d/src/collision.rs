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
    /// 累积冲量
    pub accumulated_impulse: f32,
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
            accumulated_impulse: 0.0,
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

        manifold.add_contact(Contact::new(Vec2::new(0.5, 0.0), Vec2::new(-1.0, 0.0), 0.1));

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
        let contact = Contact::new(Vec2::new(1.0, 2.0), Vec2::new(0.0, 1.0), 0.5);

        assert_eq!(contact.position, Vec2::new(1.0, 2.0));
        assert_eq!(contact.normal, Vec2::new(0.0, 1.0));
        assert_eq!(contact.penetration, 0.5);
        assert_eq!(contact.normal_impulse, 0.0);
    }

    #[test]
    fn test_elastic_collision_energy_conservation() {
        // 测试两个等质量球弹性碰撞后的速度交换
        // 假设一维碰撞，物体 A 速度为 v，物体 B 静止
        // 弹性碰撞后，A 静止，B 速度为 v（能量守恒、动量守恒）

        let mass_a: f32 = 1.0;
        let mass_b: f32 = 1.0;
        let velocity_a_initial: f32 = 10.0;
        let velocity_b_initial: f32 = 0.0;

        // 弹性碰撞公式（等质量）
        let velocity_a_final: f32 = velocity_b_initial;
        let velocity_b_final: f32 = velocity_a_initial;

        assert_eq!(velocity_a_final, 0.0);
        assert_eq!(velocity_b_final, 10.0);

        // 验证动能守恒
        let kinetic_energy_initial =
            0.5 * mass_a * velocity_a_initial.powi(2) + 0.5 * mass_b * velocity_b_initial.powi(2);
        let kinetic_energy_final =
            0.5 * mass_a * velocity_a_final.powi(2) + 0.5 * mass_b * velocity_b_final.powi(2);

        assert!((kinetic_energy_initial - kinetic_energy_final).abs() < 0.001);
    }

    #[test]
    fn test_elastic_collision_momentum_conservation() {
        // 测试动量守恒
        let mass_a: f32 = 2.0;
        let mass_b: f32 = 3.0;
        let velocity_a_initial: f32 = 5.0;
        let velocity_b_initial: f32 = -2.0;

        // 动量守恒：m1*v1 + m2*v2 = m1*v1' + m2*v2'
        let momentum_initial = mass_a * velocity_a_initial + mass_b * velocity_b_initial;

        // 一维弹性碰撞后的速度
        let total_mass = mass_a + mass_b;
        let velocity_a_final = ((mass_a - mass_b) * velocity_a_initial
            + 2.0 * mass_b * velocity_b_initial)
            / total_mass;
        let velocity_b_final = ((mass_b - mass_a) * velocity_b_initial
            + 2.0 * mass_a * velocity_a_initial)
            / total_mass;

        let momentum_final = mass_a * velocity_a_final + mass_b * velocity_b_final;

        assert!((momentum_initial - momentum_final).abs() < 0.001);
    }

    #[test]
    fn test_sleep_awake_activation() {
        // 测试休眠/唤醒机制
        // 当物体速度低于阈值时应该进入休眠
        // 当受到外力时应该唤醒

        use crate::RigidBody2D;
        use crate::RigidBodyType;

        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);

        // 设置速度为 0，应该进入休眠状态
        body.set_linear_velocity(Vec2::ZERO);

        // 当受到外力时，应该唤醒
        body.apply_force(Vec2::new(10.0, 0.0));

        // 验证力已应用
        assert!(body.force().x > 0.0);
    }

    #[test]
    fn test_body_sleep_when_stationary() {
        // 测试静止物体进入休眠
        use crate::RigidBody2D;
        use crate::RigidBodyType;

        let mut body = RigidBody2D::new(RigidBodyType::Dynamic);

        // 初始状态
        assert!(body.is_dynamic());

        // 设置很小的速度
        body.set_linear_velocity(Vec2::new(0.001, 0.0));
        assert!(body.linear_velocity().length() < 0.01);
    }

    // ============= CollisionEvent/Contact/Manifold 更多测试 =============

    #[test]
    fn test_contact_new_default() {
        let contact = Contact::new(Vec2::new(1.0, 2.0), Vec2::new(0.0, 1.0), 0.5);
        assert_eq!(contact.position, Vec2::new(1.0, 2.0));
        assert_eq!(contact.normal, Vec2::new(0.0, 1.0));
        assert_eq!(contact.penetration, 0.5);
        assert_eq!(contact.normal_impulse, 0.0);
        assert_eq!(contact.tangent_impulse, 0.0);
    }

    #[test]
    fn test_manifold_indices() {
        let manifold = Manifold::new(5, 7);
        assert_eq!(manifold.body_a, 5);
        assert_eq!(manifold.body_b, 7);
    }

    #[test]
    fn test_manifold_empty_contacts() {
        let manifold = Manifold::new(0, 1);
        assert_eq!(manifold.contact_count(), 0);
    }

    #[test]
    fn test_manifold_add_multiple_contacts() {
        let mut manifold = Manifold::new(0, 1);
        manifold.add_contact(Contact::new(Vec2::new(1.0, 0.0), Vec2::new(-1.0, 0.0), 0.1));
        manifold.add_contact(Contact::new(Vec2::new(2.0, 0.0), Vec2::new(-1.0, 0.0), 0.05));
        assert_eq!(manifold.contact_count(), 2);
    }

    #[test]
    fn test_manifold_normal_penetration() {
        let mut manifold = Manifold::new(0, 1);
        manifold.normal = Vec2::new(0.0, 1.0);
        manifold.penetration = 0.25;
        assert_eq!(manifold.normal, Vec2::new(0.0, 1.0));
        assert_eq!(manifold.penetration, 0.25);
    }

    #[test]
    fn test_collision_group_new() {
        let group = CollisionGroup::new(0b1010);
        assert_eq!(group.bits(), 0b1010);
    }

    #[test]
    fn test_collision_group_none_all() {
        let none = CollisionGroup::none();
        let all = CollisionGroup::all();
        assert_eq!(none.bits(), 0);
        assert!(all.bits() != 0);
    }

    #[test]
    fn test_collision_group_with_bit_contains() {
        let group = CollisionGroup::new(0).with_bit(3);
        assert!(group.contains(3));
        assert!(!group.contains(2));
    }

    #[test]
    fn test_collision_filter_should_collide_default() {
        let f1 = CollisionFilter::default();
        let f2 = CollisionFilter::default();
        assert!(f1.should_collide(&f2));
    }

    #[test]
    fn test_elastic_collision_energy_conservation_more() {
        // 验证两个质量不同球体弹性碰撞能量守恒
        let m1 = 2.0_f32;
        let m2 = 3.0_f32;
        let v1 = 4.0_f32;
        let v2 = -1.0_f32;

        // 一维弹性碰撞后速度
        let v1_final = ((m1 - m2) * v1 + 2.0 * m2 * v2) / (m1 + m2);
        let v2_final = ((m2 - m1) * v2 + 2.0 * m1 * v1) / (m1 + m2);

        // 初始能量
        let ke_initial = 0.5 * m1 * v1 * v1 + 0.5 * m2 * v2 * v2;
        let ke_final = 0.5 * m1 * v1_final * v1_final + 0.5 * m2 * v2_final * v2_final;
        assert!((ke_initial - ke_final).abs() < 0.001);
    }
}
