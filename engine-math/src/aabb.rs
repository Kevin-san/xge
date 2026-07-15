use core::fmt;

use crate::Vec3;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct AABB {
    pub center: Vec3,
    pub half_extents: Vec3,
}

impl AABB {
    #[inline]
    pub fn new(center: Vec3, half_extents: Vec3) -> Self {
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
    pub fn size(&self) -> Vec3 {
        self.half_extents * 2.0
    }

    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        let min = self.min();
        let max = self.max();
        point.x >= min.x
            && point.x <= max.x
            && point.y >= min.y
            && point.y <= max.y
            && point.z >= min.z
            && point.z <= max.z
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        let a_min = self.min();
        let a_max = self.max();
        let b_min = other.min();
        let b_max = other.max();

        a_min.x < b_max.x
            && a_max.x > b_min.x
            && a_min.y < b_max.y
            && a_max.y > b_min.y
            && a_min.z < b_max.z
            && a_max.z > b_min.z
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
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        let min = Vec3::new(
            self.min().x.max(other.min().x),
            self.min().y.max(other.min().y),
            self.min().z.max(other.min().z),
        );
        let max = Vec3::new(
            self.max().x.min(other.max().x),
            self.max().y.min(other.max().y),
            self.max().z.min(other.max().z),
        );
        Some(Self::from_min_max(min, max))
    }

    #[inline]
    pub fn translate(&self, offset: Vec3) -> Self {
        Self {
            center: self.center + offset,
            ..*self
        }
    }

    #[inline]
    pub fn scale(&self, factor: Vec3) -> Self {
        Self {
            half_extents: self.half_extents * factor,
            ..*self
        }
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
        assert!(aabb.contains(Vec3::ONE));
        assert!(!aabb.contains(Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_intersects() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let b = AABB::new(Vec3::new(1.0, 1.0, 1.0), Vec3::ONE);
        let c = AABB::new(Vec3::new(3.0, 0.0, 0.0), Vec3::ONE);

        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_union() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let b = AABB::new(Vec3::new(3.0, 0.0, 0.0), Vec3::ONE);
        let u = a.union(&b);

        assert_eq!(u.center, Vec3::new(1.5, 0.0, 0.0));
        assert_eq!(u.half_extents, Vec3::new(2.5, 1.0, 1.0));
    }

    #[test]
    fn test_intersection() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let b = AABB::new(Vec3::new(1.0, 0.0, 0.0), Vec3::ONE);
        let i = a.intersection(&b);

        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(i.center, Vec3::new(0.5, 0.0, 0.0));
        assert_eq!(i.half_extents, Vec3::new(0.5, 1.0, 1.0));
    }

    #[test]
    fn test_translate() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let t = a.translate(Vec3::X);
        assert_eq!(t.center, Vec3::X);
        assert_eq!(t.half_extents, Vec3::ONE);
    }

    #[test]
    fn test_scale() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let s = a.scale(Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(s.center, Vec3::ZERO);
        assert_eq!(s.half_extents, Vec3::new(2.0, 2.0, 2.0));
    }
}
