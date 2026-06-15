#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AABB {
    pub center: crate::Vec3,
    pub half_extents: crate::Vec3,
}

impl AABB {
    pub fn new(center: crate::Vec3, half_extents: crate::Vec3) -> Self {
        Self { center, half_extents }
    }

    pub fn min(&self) -> crate::Vec3 {
        self.center - self.half_extents
    }

    pub fn max(&self) -> crate::Vec3 {
        self.center + self.half_extents
    }

    pub fn contains(&self, point: crate::Vec3) -> bool {
        let min = self.min();
        let max = self.max();
        point.x >= min.x && point.x <= max.x &&
        point.y >= min.y && point.y <= max.y &&
        point.z >= min.z && point.z <= max.z
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let other_min = other.min();
        let other_max = other.max();

        self_min.x < other_max.x && self_max.x > other_min.x &&
        self_min.y < other_max.y && self_max.y > other_min.y &&
        self_min.z < other_max.z && self_max.z > other_min.z
    }

    pub fn from_min_max(min: crate::Vec3, max: crate::Vec3) -> Self {
        let center = (min + max) * 0.5;
        let half_extents = (max - min) * 0.5;
        Self { center, half_extents }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_new() {
        let aabb = AABB::new(crate::Vec3::ZERO, crate::Vec3::ONE);
        assert_eq!(aabb.center, crate::Vec3::ZERO);
        assert_eq!(aabb.half_extents, crate::Vec3::ONE);
    }

    #[test]
    fn aabb_min_max() {
        let aabb = AABB::new(crate::Vec3::ZERO, crate::Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(aabb.min(), crate::Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(aabb.max(), crate::Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn aabb_contains() {
        let aabb = AABB::new(crate::Vec3::ZERO, crate::Vec3::ONE);
        assert!(aabb.contains(crate::Vec3::ZERO));
        assert!(aabb.contains(crate::Vec3::new(0.5, 0.5, 0.5)));
        assert!(!aabb.contains(crate::Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn aabb_intersects() {
        let aabb1 = AABB::new(crate::Vec3::ZERO, crate::Vec3::ONE);
        let aabb2 = AABB::new(crate::Vec3::new(1.5, 0.0, 0.0), crate::Vec3::ONE);
        let aabb3 = AABB::new(crate::Vec3::new(3.0, 0.0, 0.0), crate::Vec3::ONE);
        
        assert!(aabb1.intersects(&aabb2));
        assert!(!aabb1.intersects(&aabb3));
    }
}
