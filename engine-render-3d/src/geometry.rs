//! Geometry primitives for 3D rendering

use alloc::vec::Vec;
use engine_math::{Mat4, Vec3};

/// Axis-Aligned Bounding Box
#[derive(Clone, Copy, Debug, Default)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    /// Empty AABB (invalid state)
    pub const EMPTY: Self = Self {
        min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
    };

    /// Unit AABB centered at origin
    pub const UNIT: Self = Self {
        min: Vec3::new(-0.5, -0.5, -0.5),
        max: Vec3::new(0.5, 0.5, 0.5),
    };

    #[inline]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn from_points(points: &[Vec3]) -> Self {
        if points.is_empty() {
            return Self::EMPTY;
        }
        let mut aabb = Self::EMPTY;
        for p in points {
            aabb = aabb.extend(*p);
        }
        aabb
    }

    #[inline]
    pub fn center(self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    #[inline]
    pub fn half_extents(self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    #[inline]
    pub fn size(self) -> Vec3 {
        self.max - self.min
    }

    #[inline]
    pub fn contains_point(self, p: Vec3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    #[inline]
    pub fn intersects_aabb(self, other: AABB) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    #[inline]
    pub fn extend(self, point: Vec3) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(point.x),
                self.min.y.min(point.y),
                self.min.z.min(point.z),
            ),
            max: Vec3::new(
                self.max.x.max(point.x),
                self.max.y.max(point.y),
                self.max.z.max(point.z),
            ),
        }
    }

    #[inline]
    pub fn merge(self, other: AABB) -> Self {
        Self {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    /// Transform AABB by matrix (expands to cover all corners)
    pub fn transform_by(self, mat: Mat4) -> Self {
        let corners = [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ];

        let transformed: Vec<Vec3> = corners.iter().map(|c| mat.transform_point3(*c)).collect();

        Self::from_points(&transformed)
    }
}

/// Bounding Sphere
#[derive(Clone, Copy, Debug, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    #[inline]
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    #[inline]
    pub fn contains_point(self, p: Vec3) -> bool {
        (p - self.center).length_squared() <= self.radius * self.radius
    }

    #[inline]
    pub fn intersects_sphere(self, other: Sphere) -> bool {
        let dist_sq = (self.center - other.center).length_squared();
        let radius_sum = self.radius + other.radius;
        dist_sq <= radius_sum * radius_sum
    }

    #[inline]
    pub fn merge(self, other: Sphere) -> Self {
        let center = (self.center + other.center) * 0.5;
        let dist = (self.center - other.center).length();
        let radius = (self.radius + other.radius + dist) * 0.5;
        Self::new(center, radius)
    }
}

/// Plane defined by normal and distance from origin
#[derive(Clone, Copy, Debug, Default)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    #[inline]
    pub fn new(normal: Vec3, d: f32) -> Self {
        Self { normal, d }
    }

    #[inline]
    pub fn from_normal_and_point(normal: Vec3, point: Vec3) -> Self {
        let n = normal.normalize();
        Self {
            normal: n,
            d: n.dot(point),
        }
    }

    #[inline]
    pub fn distance(self, p: Vec3) -> f32 {
        self.normal.dot(p) - self.d
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.normal.length();
        if len > 0.0 {
            Self {
                normal: self.normal / len,
                d: self.d / len,
            }
        } else {
            self
        }
    }
}

/// Helper trait for Mat4 point/direction transformation
pub trait Mat4Transform3D {
    fn transform_point3(&self, p: Vec3) -> Vec3;
    fn transform_direction3(&self, d: Vec3) -> Vec3;
}

impl Mat4Transform3D for Mat4 {
    fn transform_point3(&self, p: Vec3) -> Vec3 {
        let x =
            self.cols[0][0] * p.x + self.cols[1][0] * p.y + self.cols[2][0] * p.z + self.cols[3][0];
        let y =
            self.cols[0][1] * p.x + self.cols[1][1] * p.y + self.cols[2][1] * p.z + self.cols[3][1];
        let z =
            self.cols[0][2] * p.x + self.cols[1][2] * p.y + self.cols[2][2] * p.z + self.cols[3][2];
        Vec3::new(x, y, z)
    }

    fn transform_direction3(&self, d: Vec3) -> Vec3 {
        let x = self.cols[0][0] * d.x + self.cols[1][0] * d.y + self.cols[2][0] * d.z;
        let y = self.cols[0][1] * d.x + self.cols[1][1] * d.y + self.cols[2][1] * d.z;
        let z = self.cols[0][2] * d.x + self.cols[1][2] * d.y + self.cols[2][2] * d.z;
        Vec3::new(x, y, z).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_contains_point() {
        let aabb = AABB::new(Vec3::ZERO, Vec3::ONE);
        assert!(aabb.contains_point(Vec3::new(0.5, 0.5, 0.5)));
        assert!(!aabb.contains_point(Vec3::new(1.5, 0.5, 0.5)));
    }

    #[test]
    fn test_aabb_merge() {
        let a = AABB::new(Vec3::ZERO, Vec3::ONE);
        let b = AABB::new(Vec3::ONE, Vec3::new(2.0, 2.0, 2.0));
        let merged = a.merge(b);
        assert_eq!(merged.min, Vec3::ZERO);
        assert_eq!(merged.max, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_sphere_contains_point() {
        let sphere = Sphere::new(Vec3::ZERO, 1.0);
        assert!(sphere.contains_point(Vec3::new(0.5, 0.5, 0.5)));
        assert!(!sphere.contains_point(Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_plane_distance() {
        let plane = Plane::from_normal_and_point(Vec3::Y, Vec3::new(0.0, 5.0, 0.0));
        assert_eq!(plane.distance(Vec3::new(0.0, 10.0, 0.0)), 5.0);
        assert_eq!(plane.distance(Vec3::new(0.0, 0.0, 0.0)), -5.0);
    }
}
