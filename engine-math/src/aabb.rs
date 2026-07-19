use crate::Vec3;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct AABB {
    pub center: Vec3,
    pub half_extents: Vec3,
}

impl AABB {
    pub const ZERO: Self = Self {
        center: Vec3::ZERO,
        half_extents: Vec3::ZERO,
    };

    #[inline]
    pub const fn new(center: Vec3, half_extents: Vec3) -> Self {
        Self { center, half_extents }
    }

    #[inline]
    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        let center = (min + max) * 0.5;
        let half_extents = (max - min) * 0.5;
        Self { center, half_extents }
    }

    #[inline]
    pub fn min(&self) -> Vec3 {
        self.center - self.half_extents
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        self.center + self.half_extents
    }

    pub fn contains(&self, point: Vec3) -> bool {
        let min = self.min();
        let max = self.max();
        point.x >= min.x && point.x <= max.x
            && point.y >= min.y && point.y <= max.y
            && point.z >= min.z && point.z <= max.z
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let diff = (self.center - other.center).abs();
        let sum = self.half_extents + other.half_extents;
        diff.x <= sum.x && diff.y <= sum.y && diff.z <= sum.z
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::from_min_max(
            self.min().zip(other.min(), f32::min),
            self.max().zip(other.max(), f32::max),
        )
    }
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AABB(center: {}, half_extents: {})", self.center, self.half_extents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_accessors() {
        let aabb = AABB::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(aabb.center, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(aabb.half_extents, Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(aabb.min(), Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(aabb.max(), Vec3::new(1.5, 3.0, 4.5));
    }

    #[test]
    fn test_contains() {
        let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(aabb.contains(Vec3::new(0.0, 0.0, 0.0)));
        assert!(aabb.contains(Vec3::new(0.5, 0.5, 0.5)));
        assert!(aabb.contains(Vec3::new(1.0, 1.0, 1.0)));
        assert!(!aabb.contains(Vec3::new(1.5, 0.0, 0.0)));
        assert!(!aabb.contains(Vec3::new(0.0, -1.5, 0.0)));
    }

    #[test]
    fn test_intersects() {
        let a = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(a.intersects(&b));

        let c = AABB::new(Vec3::new(3.0, 3.0, 3.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_from_min_max() {
        let min = Vec3::new(-1.0, -2.0, -3.0);
        let max = Vec3::new(3.0, 4.0, 5.0);
        let aabb = AABB::from_min_max(min, max);
        assert_eq!(aabb.center, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(aabb.half_extents, Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(aabb.min(), min);
        assert_eq!(aabb.max(), max);
    }

    #[test]
    fn test_union() {
        let a = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(1.0, 1.0, 1.0));
        let u = a.union(&b);
        assert_eq!(u.min(), Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(u.max(), Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_aabb_contains_corner() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
        // All corners should be contained
        assert!(aabb.contains(Vec3::new(1.0, 1.0, 1.0)));
        assert!(aabb.contains(Vec3::new(-1.0, -1.0, -1.0)));
    }

    #[test]
    fn test_aabb_no_intersect() {
        let a = AABB::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Vec3::new(5.0, 5.0, 5.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(!a.intersects(&b));
    }

    #[test]
    fn test_aabb_touching_intersects() {
        let a = AABB::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        // Touching at edge should intersect (<= comparison)
        assert!(a.intersects(&b));
    }

    #[test]
    fn test_aabb_zero() {
        let aabb = AABB::ZERO;
        assert_eq!(aabb.center, Vec3::ZERO);
        assert_eq!(aabb.half_extents, Vec3::ZERO);
        assert!(aabb.contains(Vec3::ZERO));
        assert!(!aabb.contains(Vec3::new(0.001, 0.0, 0.0)));
    }

    #[test]
    fn test_aabb_union_same() {
        let a = AABB::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
        let result = a.union(&a);
        assert!((result.center.x - 0.0).abs() < 1e-5);
        assert!((result.half_extents.x - 1.0).abs() < 1e-5);
    }
}
