use core::fmt;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct AABB {
    pub center: super::Vec3,
    pub half_extents: super::Vec3,
}

impl AABB {
    #[inline]
    pub fn new(center: super::Vec3, half_extents: super::Vec3) -> Self {
        Self {
            center,
            half_extents,
        }
    }

    #[inline]
    pub fn from_min_max(min: super::Vec3, max: super::Vec3) -> Self {
        let center = (min + max) * 0.5;
        let half_extents = (max - min) * 0.5;
        Self {
            center,
            half_extents,
        }
    }

    #[inline]
    pub fn min(&self) -> super::Vec3 {
        self.center - self.half_extents
    }

    #[inline]
    pub fn max(&self) -> super::Vec3 {
        self.center + self.half_extents
    }

    #[inline]
    pub fn contains(&self, point: super::Vec3) -> bool {
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
        let self_min = self.min();
        let self_max = self.max();
        let other_min = other.min();
        let other_max = other.max();

        self_min.x <= other_max.x
            && self_max.x >= other_min.x
            && self_min.y <= other_max.y
            && self_max.y >= other_min.y
            && self_min.z <= other_max.z
            && self_max.z >= other_min.z
    }

    #[inline]
    pub fn size(&self) -> super::Vec3 {
        self.half_extents * 2.0
    }

    #[inline]
    pub fn merge(&self, other: &Self) -> Self {
        let self_min = self.min();
        let self_max = self.max();
        let other_min = other.min();
        let other_max = other.max();

        let min = super::Vec3::new(
            self_min.x.min(other_min.x),
            self_min.y.min(other_min.y),
            self_min.z.min(other_min.z),
        );
        let max = super::Vec3::new(
            self_max.x.max(other_max.x),
            self_max.y.max(other_max.y),
            self_max.z.max(other_max.z),
        );

        Self::from_min_max(min, max)
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
    use super::super::Vec3;
    use super::AABB;

    #[test]
    fn test_contains() {
        let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(aabb.contains(Vec3::new(0.0, 0.0, 0.0)));
        assert!(aabb.contains(Vec3::new(0.5, 0.5, 0.5)));
        assert!(aabb.contains(Vec3::new(1.0, 1.0, 1.0))); // on boundary
        assert!(!aabb.contains(Vec3::new(1.5, 0.0, 0.0)));
        assert!(!aabb.contains(Vec3::new(0.0, -1.5, 0.0)));
    }

    #[test]
    fn test_intersects() {
        let a = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Vec3::new(1.5, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(a.intersects(&b)); // touching at edge

        let c = AABB::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(!a.intersects(&c)); // separated

        let d = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 0.5, 0.5));
        assert!(a.intersects(&d)); // contained
    }

    #[test]
    fn test_size() {
        let aabb = AABB::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0));
        let size = aabb.size();
        assert_eq!(size, Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_merge() {
        let a = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let merged = a.merge(&b);
        // merged should cover from -1 to 3 on x, -1 to 1 on y/z
        assert_eq!(merged.min(), Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(merged.max(), Vec3::new(3.0, 1.0, 1.0));
    }

    #[test]
    fn test_min_max() {
        let aabb = AABB::from_min_max(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(aabb.min(), Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(aabb.max(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(aabb.center, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(aabb.half_extents, Vec3::new(1.0, 2.0, 3.0));
    }
}
