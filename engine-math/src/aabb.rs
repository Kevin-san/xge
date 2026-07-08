use crate::Vec3;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct AABB {
    pub center: Vec3,
    pub half_extents: Vec3,
}

impl AABB {
    #[inline]
    pub const fn new(center: Vec3, half_extents: Vec3) -> Self {
        Self {
            center,
            half_extents,
        }
    }

    #[inline]
    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        Self {
            center: (min + max) * 0.5,
            half_extents: (max - min) * 0.5,
        }
    }

    #[inline]
    pub fn min(&self) -> Vec3 {
        self.center - self.half_extents
    }

    #[inline]
    pub fn max(&self) -> Vec3 {
        self.center + self.half_extents
    }

    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        let min = self.min();
        let max = self.max();
        point.x >= min.x && point.x <= max.x
            && point.y >= min.y && point.y <= max.y
            && point.z >= min.z && point.z <= max.z
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        let min_a = self.min();
        let max_a = self.max();
        let min_b = other.min();
        let max_b = other.max();

        min_a.x < max_b.x && max_a.x > min_b.x
            && min_a.y < max_b.y && max_a.y > min_b.y
            && min_a.z < max_b.z && max_a.z > min_b.z
    }

    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        let min = Vec3::new(
            self.min().x.min(other.min().x),
            self.min().y.min(other.min().y),
            self.min().z.min(other.min().z),
        );
        let max = Vec3::new(
            self.max().x.max(other.max().x),
            self.max().y.max(other.max().y),
            self.max().z.max(other.max().z),
        );
        Self::from_min_max(min, max)
    }

    #[inline]
    pub fn expand(&self, point: Vec3) -> Self {
        let min = Vec3::new(
            self.min().x.min(point.x),
            self.min().y.min(point.y),
            self.min().z.min(point.z),
        );
        let max = Vec3::new(
            self.max().x.max(point.x),
            self.max().y.max(point.y),
            self.max().z.max(point.z),
        );
        Self::from_min_max(min, max)
    }

    #[inline]
    pub fn size(&self) -> Vec3 {
        self.half_extents * 2.0
    }

    #[inline]
    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AABB(center: {}, half_extents: {})",
            self.center, self.half_extents
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::ONE);
        assert_eq!(aabb.center, Vec3::ZERO);
        assert_eq!(aabb.half_extents, Vec3::ONE);
    }

    #[test]
    fn test_from_min_max() {
        let aabb = AABB::from_min_max(Vec3::ZERO, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(aabb.center, Vec3::ONE);
        assert_eq!(aabb.half_extents, Vec3::ONE);
    }

    #[test]
    fn test_min_max() {
        let aabb = AABB::new(Vec3::ONE, Vec3::ONE);
        assert_eq!(aabb.min(), Vec3::ZERO);
        assert_eq!(aabb.max(), Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_contains() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::ONE);
        assert!(aabb.contains(Vec3::ZERO));
        assert!(aabb.contains(Vec3::new(0.5, 0.5, 0.5)));
        assert!(!aabb.contains(Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_intersects() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let b = AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::ONE);
        let c = AABB::new(Vec3::new(10.0, 0.0, 0.0), Vec3::ONE);

        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_union() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let b = AABB::new(Vec3::new(3.0, 0.0, 0.0), Vec3::ONE);
        let u = a.union(&b);

        assert_eq!(u.min(), Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(u.max(), Vec3::new(4.0, 1.0, 1.0));
    }

    #[test]
    fn test_expand() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::ONE);
        let expanded = aabb.expand(Vec3::new(5.0, 0.0, 0.0));

        assert_eq!(expanded.min().x, -1.0);
        assert_eq!(expanded.max().x, 5.0);
    }

    #[test]
    fn test_size() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(aabb.size(), Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_volume() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(aabb.volume(), 48.0);
    }
}