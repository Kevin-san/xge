//! Frustum for view culling

use crate::geometry::{Plane, Sphere, AABB};
use engine_math::Mat4;

/// Frustum planes indices
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrustumPlane {
    Left = 0,
    Right = 1,
    Bottom = 2,
    Top = 3,
    Near = 4,
    Far = 5,
}

/// View frustum for culling
#[derive(Clone, Debug)]
pub struct Frustum {
    planes: [Plane; 6],
}

impl Frustum {
    /// Extract frustum planes from view-projection matrix
    pub fn from_view_projection(vp: Mat4) -> Self {
        let m = vp.cols;

        // Left plane
        let left = Plane::new(
            Vec3::new(m[3][0] + m[0][0], m[3][1] + m[0][1], m[3][2] + m[0][2]),
            m[3][3] + m[0][3],
        )
        .normalize();

        // Right plane
        let right = Plane::new(
            Vec3::new(m[3][0] - m[0][0], m[3][1] - m[0][1], m[3][2] - m[0][2]),
            m[3][3] - m[0][3],
        )
        .normalize();

        // Bottom plane
        let bottom = Plane::new(
            Vec3::new(m[3][0] + m[1][0], m[3][1] + m[1][1], m[3][2] + m[1][2]),
            m[3][3] + m[1][3],
        )
        .normalize();

        // Top plane
        let top = Plane::new(
            Vec3::new(m[3][0] - m[1][0], m[3][1] - m[1][1], m[3][2] - m[1][2]),
            m[3][3] - m[1][3],
        )
        .normalize();

        // Near plane
        let near = Plane::new(
            Vec3::new(m[3][0] + m[2][0], m[3][1] + m[2][1], m[3][2] + m[2][2]),
            m[3][3] + m[2][3],
        )
        .normalize();

        // Far plane
        let far = Plane::new(
            Vec3::new(m[3][0] - m[2][0], m[3][1] - m[2][1], m[3][2] - m[2][2]),
            m[3][3] - m[2][3],
        )
        .normalize();

        Self {
            planes: [left, right, bottom, top, near, far],
        }
    }

    #[inline]
    pub fn planes(&self) -> &[Plane; 6] {
        &self.planes
    }

    /// Test if point is inside frustum
    pub fn contains_point(&self, p: Vec3) -> bool {
        for plane in &self.planes {
            if plane.distance(p) < 0.0 {
                return false;
            }
        }
        true
    }

    /// Test if AABB is fully inside frustum
    pub fn contains_aabb(&self, aabb: AABB) -> bool {
        // Test all 8 corners
        let corners = [
            Vec3::new(aabb.min.x, aabb.min.y, aabb.min.z),
            Vec3::new(aabb.max.x, aabb.min.y, aabb.min.z),
            Vec3::new(aabb.min.x, aabb.max.y, aabb.min.z),
            Vec3::new(aabb.max.x, aabb.max.y, aabb.min.z),
            Vec3::new(aabb.min.x, aabb.min.y, aabb.max.z),
            Vec3::new(aabb.max.x, aabb.min.y, aabb.max.z),
            Vec3::new(aabb.min.x, aabb.max.y, aabb.max.z),
            Vec3::new(aabb.max.x, aabb.max.y, aabb.max.z),
        ];

        for plane in &self.planes {
            let mut all_outside = true;
            for corner in &corners {
                if plane.distance(*corner) >= 0.0 {
                    all_outside = false;
                    break;
                }
            }
            if all_outside {
                return false;
            }
        }
        true
    }

    /// Test if sphere is inside frustum
    pub fn contains_sphere(&self, sphere: Sphere) -> bool {
        for plane in &self.planes {
            if plane.distance(sphere.center) < -sphere.radius {
                return false;
            }
        }
        true
    }

    /// Quick intersection test (may return true for partially visible)
    pub fn intersects_aabb(&self, aabb: AABB) -> bool {
        self.contains_aabb(aabb)
    }

    /// Get specific plane
    #[inline]
    pub fn get_plane(&self, index: FrustumPlane) -> Plane {
        self.planes[index as usize]
    }
}

use engine_math::Vec3;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera3D;
    use engine_math::Mat4;

    #[test]
    fn test_frustum_from_camera() {
        let mut cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        cam.set_position(Vec3::new(0.0, 0.0, 5.0));
        cam.look_at(Vec3::ZERO);
        let vp = cam.view_projection();
        let frustum = Frustum::from_view_projection(vp);

        // Point in front of camera should be inside
        let point_in_front = cam.position() + cam.forward() * -2.0; // 2 units in front
        assert!(frustum.contains_point(point_in_front));
    }

    #[test]
    fn test_frustum_contains_point() {
        let mut cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        cam.set_position(Vec3::new(0.0, 0.0, 5.0));
        cam.look_at(Vec3::ZERO);
        let vp = cam.view_projection();
        let frustum = Frustum::from_view_projection(vp);

        // Point far behind camera (beyond far plane) should be outside
        let point_behind = cam.position() + cam.forward() * 200.0; // Far beyond far plane
        assert!(!frustum.contains_point(point_behind));
    }

    #[test]
    fn test_frustum_contains_sphere() {
        let mut cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        cam.set_position(Vec3::new(0.0, 0.0, 5.0));
        cam.look_at(Vec3::ZERO);
        let vp = cam.view_projection();
        let frustum = Frustum::from_view_projection(vp);

        // Sphere in front of camera
        let sphere_center = cam.position() + cam.forward() * -5.0;
        let sphere = Sphere::new(sphere_center, 1.0);
        assert!(frustum.contains_sphere(sphere));
    }

    #[test]
    fn test_frustum_contains_aabb() {
        let mut cam = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        cam.set_position(Vec3::new(0.0, 0.0, 5.0));
        cam.look_at(Vec3::ZERO);
        let vp = cam.view_projection();
        let frustum = Frustum::from_view_projection(vp);

        // AABB in front of camera
        let center = cam.position() + cam.forward() * -5.0;
        let aabb = AABB::new(center - Vec3::ONE, center + Vec3::ONE);
        assert!(frustum.contains_aabb(aabb));
    }
}
