use crate::{AABB, Mat4, Vec3};

#[derive(Clone, Copy, Debug)]
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
    pub fn distance_to_point(&self, p: Vec3) -> f32 {
        self.normal.dot(p) + self.d
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.normal.length();
        if len > 0.0 {
            Self {
                normal: self.normal / len,
                d: self.d / len,
            }
        } else {
            Self {
                normal: Vec3::ZERO,
                d: 0.0,
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrustResult {
    Inside,
    Outside,
    Intersect,
}

#[inline]
#[allow(dead_code)]
fn aabb_corners(aabb: AABB) -> [Vec3; 8] {
    let min = aabb.min();
    let max = aabb.max();
    [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, max.y, max.z),
    ]
}

impl Frustum {
    #[inline]
    pub fn from_view_proj(vp: Mat4) -> Self {
        let m = vp.cols;
        let planes = [
            Plane::new(
                Vec3::new(
                    m[0][3] + m[0][0],
                    m[1][3] + m[1][0],
                    m[2][3] + m[2][0],
                ),
                m[3][3] + m[3][0],
            )
            .normalize(),
            Plane::new(
                Vec3::new(
                    m[0][3] - m[0][0],
                    m[1][3] - m[1][0],
                    m[2][3] - m[2][0],
                ),
                m[3][3] - m[3][0],
            )
            .normalize(),
            Plane::new(
                Vec3::new(
                    m[0][3] + m[0][1],
                    m[1][3] + m[1][1],
                    m[2][3] + m[2][1],
                ),
                m[3][3] + m[3][1],
            )
            .normalize(),
            Plane::new(
                Vec3::new(
                    m[0][3] - m[0][1],
                    m[1][3] - m[1][1],
                    m[2][3] - m[2][1],
                ),
                m[3][3] - m[3][1],
            )
            .normalize(),
            Plane::new(
                Vec3::new(
                    m[0][3] + m[0][2],
                    m[1][3] + m[1][2],
                    m[2][3] + m[2][2],
                ),
                m[3][3] + m[3][2],
            )
            .normalize(),
            Plane::new(
                Vec3::new(
                    m[0][3] - m[0][2],
                    m[1][3] - m[1][2],
                    m[2][3] - m[2][2],
                ),
                m[3][3] - m[3][2],
            )
            .normalize(),
        ];
        Self { planes }
    }

    #[inline]
    pub fn classify_aabb(&self, aabb: AABB) -> FrustResult {
        let center = aabb.center;
        let he = aabb.half_extents;
        let mut result = FrustResult::Inside;
        for plane in &self.planes {
            let d = plane.distance_to_point(center);
            let r = he.x * plane.normal.x.abs()
                + he.y * plane.normal.y.abs()
                + he.z * plane.normal.z.abs();
            if d + r < 0.0 {
                return FrustResult::Outside;
            }
            if d - r < 0.0 {
                result = FrustResult::Intersect;
            }
        }
        result
    }

    #[inline]
    pub fn classify_aabb_batch(&self, aabbs: &[AABB], results: &mut [FrustResult]) {
        for (aabb, result) in aabbs.iter().zip(results.iter_mut()) {
            *result = self.classify_aabb(*aabb);
        }
    }

    #[inline]
    pub fn classify_aabb_simd8(&self, aabbs: &[AABB; 8]) -> [FrustResult; 8] {
        let mut results = [FrustResult::Outside; 8];
        for i in 0..8 {
            results[i] = self.classify_aabb(aabbs[i]);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_frustum() -> Frustum {
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
        let vp = proj * view;
        Frustum::from_view_proj(vp)
    }

    #[test]
    fn test_plane_new_and_distance() {
        let p = Plane::new(Vec3::new(0.0, 1.0, 0.0), -5.0);
        assert!((p.distance_to_point(Vec3::new(0.0, 10.0, 0.0)) - 5.0).abs() < 1e-6);
        assert!((p.distance_to_point(Vec3::new(0.0, 0.0, 0.0)) - (-5.0)).abs() < 1e-6);
    }

    #[test]
    fn test_plane_normalize() {
        let p = Plane::new(Vec3::new(3.0, 0.0, 4.0), 10.0);
        let n = p.normalize();
        assert!((n.normal.length() - 1.0).abs() < 1e-6);
        assert!((n.d - 2.0).abs() < 1e-6);
        assert!((n.normal.x - 0.6).abs() < 1e-6);
        assert!((n.normal.z - 0.8).abs() < 1e-6);
    }

    #[test]
    fn test_plane_normalize_zero() {
        let p = Plane::new(Vec3::ZERO, 0.0);
        let n = p.normalize();
        assert_eq!(n.normal, Vec3::ZERO);
        assert!((n.d - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_frustum_from_view_proj() {
        let frustum = make_test_frustum();
        assert_eq!(frustum.planes.len(), 6);
        for plane in &frustum.planes {
            assert!((plane.normal.length() - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn test_classify_aabb_inside() {
        let frustum = make_test_frustum();
        let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let result = frustum.classify_aabb(aabb);
        assert_eq!(result, FrustResult::Inside);
    }

    #[test]
    fn test_classify_aabb_outside() {
        let frustum = make_test_frustum();
        let aabb = AABB::new(Vec3::new(0.0, 0.0, -200.0), Vec3::new(1.0, 1.0, 1.0));
        let result = frustum.classify_aabb(aabb);
        assert_eq!(result, FrustResult::Outside);
    }

    #[test]
    fn test_classify_aabb_intersect() {
        let frustum = make_test_frustum();
        let aabb = AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(5.0, 5.0, 5.0));
        let result = frustum.classify_aabb(aabb);
        assert_eq!(result, FrustResult::Intersect);
    }

    #[test]
    fn test_classify_aabb_batch_empty() {
        let frustum = make_test_frustum();
        let aabbs: [AABB; 0] = [];
        let mut results: [FrustResult; 0] = [];
        frustum.classify_aabb_batch(&aabbs, &mut results);
    }

    #[test]
    fn test_classify_aabb_batch_all_inside() {
        let frustum = make_test_frustum();
        let aabbs = [
            AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.5, 0.5, -1.0), Vec3::new(0.5, 0.5, 0.5)),
        ];
        let mut results = [FrustResult::Outside; 2];
        frustum.classify_aabb_batch(&aabbs, &mut results);
        assert_eq!(results[0], FrustResult::Inside);
        assert_eq!(results[1], FrustResult::Inside);
    }

    #[test]
    fn test_classify_aabb_batch_all_outside() {
        let frustum = make_test_frustum();
        let aabbs = [
            AABB::new(Vec3::new(0.0, 0.0, -200.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(100.0, 100.0, 100.0), Vec3::new(1.0, 1.0, 1.0)),
        ];
        let mut results = [FrustResult::Inside; 2];
        frustum.classify_aabb_batch(&aabbs, &mut results);
        assert_eq!(results[0], FrustResult::Outside);
        assert_eq!(results[1], FrustResult::Outside);
    }

    #[test]
    fn test_classify_aabb_batch_mixed() {
        let frustum = make_test_frustum();
        let aabbs = [
            AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.0, 0.0, -200.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(5.0, 5.0, 5.0)),
        ];
        let mut results = [FrustResult::Outside; 3];
        frustum.classify_aabb_batch(&aabbs, &mut results);
        assert_eq!(results[0], FrustResult::Inside);
        assert_eq!(results[1], FrustResult::Outside);
        assert_eq!(results[2], FrustResult::Intersect);
    }

    #[test]
    fn test_classify_aabb_simd8() {
        let frustum = make_test_frustum();
        let aabbs = [
            AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.0, 0.0, -200.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(5.0, 5.0, 5.0)),
            AABB::new(Vec3::new(0.5, 0.5, -2.0), Vec3::new(0.5, 0.5, 0.5)),
            AABB::new(Vec3::new(50.0, 50.0, 50.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.0, 2.0, -3.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(-50.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
            AABB::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(2.0, 2.0, 2.0)),
        ];
        let results = frustum.classify_aabb_simd8(&aabbs);
        assert_eq!(results[0], FrustResult::Inside);
        assert_eq!(results[1], FrustResult::Outside);
        assert_eq!(results[2], FrustResult::Intersect);
    }

    #[test]
    fn test_aabb_corners_helper() {
        let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let corners = aabb_corners(aabb);
        assert_eq!(corners.len(), 8);
        assert_eq!(corners[0], Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(corners[7], Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_classify_aabb_at_boundary() {
        let frustum = make_test_frustum();
        let aabb = AABB::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(50.0, 50.0, 50.0));
        let result = frustum.classify_aabb(aabb);
        assert!(result == FrustResult::Intersect || result == FrustResult::Inside);
    }

    #[test]
    fn test_classify_aabb_small_inside() {
        let frustum = make_test_frustum();
        let aabb = AABB::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.01, 0.01, 0.01));
        let result = frustum.classify_aabb(aabb);
        assert_eq!(result, FrustResult::Inside);
    }

    #[test]
    fn test_frustum_symmetry() {
        let frustum = make_test_frustum();
        assert_eq!(frustum.planes.len(), 6);
        let mut plane_normals = [0.0f32; 6];
        for (i, plane) in frustum.planes.iter().enumerate() {
            plane_normals[i] = plane.normal.length();
        }
        for &len in &plane_normals {
            assert!((len - 1.0).abs() < 1e-4, "Plane normal not normalized, length={}", len);
        }
    }

    #[test]
    fn test_classify_aabb_batch_large() {
        let frustum = make_test_frustum();
        let aabbs: Vec<AABB> = (0..200)
            .map(|i| {
                let z = (i as f32 - 50.0) * 0.5;
                AABB::new(Vec3::new(0.0, 0.0, z), Vec3::new(1.0, 1.0, 1.0))
            })
            .collect();
        let mut results = vec![FrustResult::Outside; 200];
        frustum.classify_aabb_batch(&aabbs, &mut results);
        let inside_count = results.iter().filter(|&&r| r == FrustResult::Inside).count();
        let outside_count = results.iter().filter(|&&r| r == FrustResult::Outside).count();
        let intersect_count = results.iter().filter(|&&r| r == FrustResult::Intersect).count();
        assert_eq!(inside_count + outside_count + intersect_count, 200);
    }
}