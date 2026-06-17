//! Ray casting and intersection tests

use crate::geometry::{Mat4Transform3D, Plane, Sphere, AABB};
use crate::mesh::Mesh3D;
use crate::transform::Transform3D;
use engine_math::Vec3;

/// 3D Ray for ray casting
#[derive(Clone, Copy, Debug)]
pub struct Ray3 {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray3 {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Get point along ray at distance t
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Test intersection with AABB using slab method
    pub fn hit_aabb(&self, aabb: AABB) -> Option<f32> {
        let mut t_min: f32 = 0.0;
        let mut t_max: f32 = f32::INFINITY;

        let mins = [aabb.min.x, aabb.min.y, aabb.min.z];
        let maxs = [aabb.max.x, aabb.max.y, aabb.max.z];
        let dirs = [self.direction.x, self.direction.y, self.direction.z];
        let origins = [self.origin.x, self.origin.y, self.origin.z];

        for i in 0..3 {
            let axis = mins[i];
            let axis_max = maxs[i];
            let dir_component = dirs[i];
            let origin_component = origins[i];

            if dir_component.abs() < 1e-8 {
                // Ray parallel to slab
                if origin_component < axis || origin_component > axis_max {
                    return None;
                }
            } else {
                let inv_d = 1.0 / dir_component;
                let t1 = (axis - origin_component) * inv_d;
                let t2 = (axis_max - origin_component) * inv_d;

                let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                t_min = t_min.max(t_near);
                t_max = t_max.min(t_far);

                if t_min > t_max {
                    return None;
                }
            }
        }

        if t_min >= 0.0 {
            Some(t_min)
        } else if t_max >= 0.0 {
            Some(t_max)
        } else {
            None
        }
    }

    /// Test intersection with sphere
    pub fn hit_sphere(&self, sphere: Sphere) -> Option<f32> {
        let oc = self.origin - sphere.center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        if t1 >= 0.0 {
            Some(t1)
        } else if t2 >= 0.0 {
            Some(t2)
        } else {
            None
        }
    }

    /// Test intersection with triangle using Möller-Trumbore algorithm
    pub fn hit_triangle(&self, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32> {
        const EPSILON: f32 = 1e-8;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = self.direction.cross(edge2);
        let a = edge1.dot(h);

        if a.abs() < EPSILON {
            return None; // Ray parallel to triangle
        }

        let f = 1.0 / a;
        let s = self.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * self.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > EPSILON {
            Some(t)
        } else {
            None
        }
    }

    /// Test intersection with plane
    pub fn hit_plane(&self, plane: Plane) -> Option<f32> {
        let denom = plane.normal.dot(self.direction);
        if denom.abs() < 1e-8 {
            return None; // Ray parallel to plane
        }

        let t = (plane.d - plane.normal.dot(self.origin)) / denom;
        if t >= 0.0 {
            Some(t)
        } else {
            None
        }
    }

    /// Test intersection with mesh (returns closest hit)
    pub fn hit_mesh(&self, mesh: &Mesh3D, transform: &Transform3D) -> Option<HitResult> {
        let inv_transform = transform.inverse_matrix();
        let local_ray = Ray3::new(
            inv_transform.transform_point3(self.origin),
            inv_transform.transform_direction3(self.direction),
        );

        let mut best_hit: Option<HitResult> = None;
        let vertices = mesh.vertices();

        for (prim_idx, prim) in mesh.primitives().iter().enumerate() {
            let indices = &prim.indices;
            for tri_idx in (0..indices.len()).step_by(3) {
                if tri_idx + 2 >= indices.len() {
                    break;
                }

                let i0 = indices[tri_idx] as usize;
                let i1 = indices[tri_idx + 1] as usize;
                let i2 = indices[tri_idx + 2] as usize;

                if i0 >= vertices.len() || i1 >= vertices.len() || i2 >= vertices.len() {
                    continue;
                }

                let v0 = vertices[i0].position;
                let v1 = vertices[i1].position;
                let v2 = vertices[i2].position;

                if let Some(t) = local_ray.hit_triangle(v0, v1, v2) {
                    let should_update = match &best_hit {
                        None => true,
                        Some(h) => t < h.t,
                    };
                    if should_update {
                        let local_point = local_ray.at(t);
                        let world_point = transform.transform_point(local_point);

                        // Compute normal (average of vertex normals)
                        let normal = if mesh.has_normals() {
                            let n0 = vertices[i0].normal;
                            let n1 = vertices[i1].normal;
                            let n2 = vertices[i2].normal;
                            transform.transform_direction((n0 + n1 + n2).normalize())
                        } else {
                            // Compute face normal
                            let edge1 = v1 - v0;
                            let edge2 = v2 - v0;
                            transform.transform_direction(edge1.cross(edge2).normalize())
                        };

                        best_hit = Some(HitResult {
                            t: (world_point - self.origin).length(),
                            point: world_point,
                            normal,
                            uv: None,
                            primitive_index: prim_idx,
                        });
                    }
                }
            }
        }

        best_hit
    }
}

/// Hit result from ray intersection
#[derive(Clone, Debug)]
pub struct HitResult {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub uv: Option<(f32, f32)>,
    pub primitive_index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let ray = Ray3::new(Vec3::ZERO, Vec3::X);
        assert_eq!(ray.at(5.0), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_hit_sphere() {
        let ray = Ray3::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::X);
        let sphere = Sphere::new(Vec3::ZERO, 1.0);
        let hit = ray.hit_sphere(sphere);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 4.0).abs() < 1e-5);
    }

    #[test]
    fn test_ray_hit_aabb() {
        let ray = Ray3::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X);
        let aabb = AABB::new(Vec3::ZERO, Vec3::ONE);
        let hit = ray.hit_aabb(aabb);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_hit_triangle() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
        let v0 = Vec3::new(-1.0, -1.0, 0.0);
        let v1 = Vec3::new(1.0, -1.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let hit = ray.hit_triangle(v0, v1, v2);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_ray_miss_triangle() {
        let ray = Ray3::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
        // Triangle far away from ray path
        let v0 = Vec3::new(10.0, 10.0, 10.0);
        let v1 = Vec3::new(11.0, 10.0, 10.0);
        let v2 = Vec3::new(10.0, 11.0, 10.0);
        let hit = ray.hit_triangle(v0, v1, v2);
        assert!(hit.is_none());
    }
}
