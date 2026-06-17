//! 碰撞分组系统 - 使用位掩码进行分组和过滤

/// 碰撞分组
///
/// 使用两个 32 位掩码进行碰撞过滤：
/// - memberships: 该物体所属的分组
/// - filters: 该物体可以与哪些分组碰撞
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionGroup(u32);

impl Default for CollisionGroup {
    fn default() -> Self {
        Self::with_all()
    }
}

impl CollisionGroup {
    /// 创建新的碰撞分组
    ///
    /// # Arguments
    /// * `memberships` - 该物体所属的分组位掩码
    /// * `filters` - 该物体可以与哪些分组碰撞的位掩码
    pub fn new(memberships: u32, filters: u32) -> Self {
        Self(memberships | (filters << 16))
    }

    /// 获取低 16 位（memberships）
    fn memberships_bits(&self) -> u32 {
        self.0 & 0xFFFF
    }

    /// 获取高 16 位（filters）
    fn filters_bits(&self) -> u32 {
        (self.0 >> 16) & 0xFFFF
    }

    /// 与所有分组交互（memberships=0xFFFF, filters=0xFFFF）
    pub fn with_all() -> Self {
        Self(0xFFFFFFFF)
    }

    /// 不与任何分组交互（memberships=0, filters=0）
    pub fn with_none() -> Self {
        Self(0)
    }

    /// 获取 memberships 位掩码
    pub fn memberships(&self) -> u32 {
        self.memberships_bits()
    }

    /// 获取 filters 位掩码
    pub fn filters(&self) -> u32 {
        self.filters_bits()
    }

    /// 检查是否包含指定分组
    pub fn contains(&self, group: u32) -> bool {
        (self.memberships() & (1 << group)) != 0
    }

    /// 添加到指定分组
    pub fn add_group(self, group: u32) -> Self {
        let memberships = self.memberships() | (1 << group);
        let filters = self.filters();
        Self(memberships | (filters << 16))
    }

    /// 从指定分组移除
    pub fn remove_group(self, group: u32) -> Self {
        let memberships = self.memberships() & !(1 << group);
        let filters = self.filters();
        Self(memberships | (filters << 16))
    }

    /// 设置 filters
    pub fn with_filters(self, filters: u32) -> Self {
        let memberships = self.memberships();
        Self(memberships | ((filters & 0xFFFF) << 16))
    }

    /// 检查是否与其他分组可以交互
    ///
    /// 两个物体可以碰撞的条件：
    /// 1. A 的 memberships 与 B 的 memberships 有交集
    /// 2. A 的 filters 与 B 的 filters 有交集
    pub fn can_interact_with(self, other: CollisionGroup) -> bool {
        let a_memberships = self.memberships();
        let a_filters = self.filters();
        let b_memberships = other.memberships();
        let b_filters = other.filters();

        // A 的 memberships 与 B 的 memberships 有交集
        let cond1 = (a_memberships & b_memberships) != 0;
        // A 的 filters 与 B 的 filters 有交集
        let cond2 = (a_filters & b_filters) != 0;

        cond1 && cond2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_group_default() {
        let group = CollisionGroup::default();
        assert_eq!(group.memberships(), 0xFFFF);
        assert_eq!(group.filters(), 0xFFFF);
    }

    #[test]
    fn test_collision_group_with_all() {
        let group = CollisionGroup::with_all();
        assert_eq!(group.memberships(), 0xFFFF);
        assert_eq!(group.filters(), 0xFFFF);
    }

    #[test]
    fn test_collision_group_with_none() {
        let group = CollisionGroup::with_none();
        assert_eq!(group.memberships(), 0);
        assert_eq!(group.filters(), 0);
    }

    #[test]
    fn test_collision_group_new() {
        let group = CollisionGroup::new(0b0001, 0b0010);
        assert_eq!(group.memberships(), 0b0001);
        assert_eq!(group.filters(), 0b0010);
    }

    #[test]
    fn test_collision_group_contains() {
        let group = CollisionGroup::new(0b0001, 0b0000);
        assert!(group.contains(0));
        assert!(!group.contains(1));
        assert!(!group.contains(2));
    }

    #[test]
    fn test_collision_group_add_remove_group() {
        let group = CollisionGroup::new(0b0001, 0b0000);
        let group2 = group.add_group(2);
        assert!(group2.contains(0));
        assert!(group2.contains(2));
        assert!(!group2.contains(1));

        let group3 = group2.remove_group(0);
        assert!(!group3.contains(0));
        assert!(group3.contains(2));
    }

    #[test]
    fn test_can_interact_with_both_all() {
        // 两个都和所有组交互的物体可以碰撞
        let group_a = CollisionGroup::with_all();
        let group_b = CollisionGroup::with_all();
        assert!(group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_both_none() {
        // 两个都不与任何组交互的物体不能碰撞
        let group_a = CollisionGroup::with_none();
        let group_b = CollisionGroup::with_none();
        assert!(!group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_matching_groups() {
        // 两个同组的物体可以碰撞
        let group_a = CollisionGroup::new(0b0001, 0b0001);
        let group_b = CollisionGroup::new(0b0001, 0b0001);
        assert!(group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_different_groups() {
        // 两个不同组且不互相过滤的物体不能碰撞
        let group_a = CollisionGroup::new(0b0001, 0b0001); // 只在组 0，过滤组 0
        let group_b = CollisionGroup::new(0b0010, 0b0010); // 只在组 1，过滤组 1
        assert!(!group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_cross_filter() {
        // A 在组 0 过滤组 1，B 在组 1 过滤组 0 - 不能碰撞
        let group_a = CollisionGroup::new(0b0001, 0b0010);
        let group_b = CollisionGroup::new(0b0010, 0b0001);
        assert!(!group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_one_way_filter() {
        // A 在组 0 过滤组 0、1，B 在组 1 过滤组 1
        // A memberships=1, A filters=0|1=3
        // B memberships=2, B filters=2
        // A memberships & B filters = 1 & 2 = 0 -> 失败
        let group_a = CollisionGroup::new(0b0001, 0b0011);
        let group_b = CollisionGroup::new(0b0010, 0b0010);
        assert!(!group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_filtering() {
        // 当一个物体设置 filters=0 时，它不与任何物体碰撞
        let group_a = CollisionGroup::new(0b0001, 0b0000);
        let group_b = CollisionGroup::new(0b0001, 0b0001);
        assert!(!group_a.can_interact_with(group_b));
    }

    #[test]
    fn test_can_interact_with_symmetric() {
        // can_interact_with 应该是对称的
        let group_a = CollisionGroup::new(0b0001, 0b0011);
        let group_b = CollisionGroup::new(0b0010, 0b0011);
        assert_eq!(
            group_a.can_interact_with(group_b),
            group_b.can_interact_with(group_a)
        );
    }
}
