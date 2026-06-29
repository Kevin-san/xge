//! 3D Mesh structures and primitives

use crate::geometry::{Mat4Transform3D, Sphere, AABB};
use crate::vertex::Vertex;
use alloc::vec::Vec;
use engine_math::{Mat4, Vec2, Vec3};

/// Mesh primitive (submesh with material index)
#[derive(Clone, Debug)]
pub struct Primitive {
    pub indices: Vec<u32>,
    pub material_index: Option<usize>,
    pub first_vertex: usize,
    pub vertex_count: usize,
}

impl Primitive {
    pub fn new(indices: Vec<u32>) -> Self {
        Self {
            indices,
            material_index: None,
            first_vertex: 0,
            vertex_count: 0,
        }
    }

    #[inline]
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

/// 3D Mesh with vertices, indices, and primitives
#[derive(Clone, Debug)]
pub struct Mesh3D {
    vertices: Vec<Vertex>,
    primitives: Vec<Primitive>,
    aabb: AABB,
    has_normals: bool,
    has_uv: bool,
}

impl Mesh3D {
    /// Create empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            primitives: Vec::new(),
            aabb: AABB::EMPTY,
            has_normals: false,
            has_uv: false,
        }
    }

    /// Create mesh from vertices and indices
    pub fn from_vertices(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let has_normals = vertices.iter().any(|v| v.normal != Vec3::ZERO);
        let has_uv = vertices.iter().any(|v| v.texcoord != Vec2::ZERO);

        let aabb = compute_aabb(&vertices);

        Self {
            vertices,
            primitives: vec![Primitive::new(indices)],
            aabb,
            has_normals,
            has_uv,
        }
    }

    /// Create mesh with multiple primitives
    pub fn with_primitives(vertices: Vec<Vertex>, primitives: Vec<Primitive>) -> Self {
        let has_normals = vertices.iter().any(|v| v.normal != Vec3::ZERO);
        let has_uv = vertices.iter().any(|v| v.texcoord != Vec2::ZERO);
        let aabb = compute_aabb(&vertices);

        Self {
            vertices,
            primitives,
            aabb,
            has_normals,
            has_uv,
        }
    }

    #[inline]
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    #[inline]
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }

    #[inline]
    pub fn aabb(&self) -> AABB {
        self.aabb
    }

    #[inline]
    pub fn bounding_sphere(&self) -> Sphere {
        Sphere::new(self.aabb.center(), self.aabb.half_extents().length())
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn triangle_count(&self) -> usize {
        self.primitives.iter().map(|p| p.triangle_count()).sum()
    }

    #[inline]
    pub fn primitive_count(&self) -> usize {
        self.primitives.len()
    }

    #[inline]
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }

    #[inline]
    pub fn has_uv(&self) -> bool {
        self.has_uv
    }

    /// Compute normals if missing
    pub fn compute_normals(&mut self) {
        for prim in &self.primitives {
            for tri_idx in (0..prim.indices.len()).step_by(3) {
                if tri_idx + 2 >= prim.indices.len() {
                    break;
                }
                let i0 = prim.indices[tri_idx] as usize;
                let i1 = prim.indices[tri_idx + 1] as usize;
                let i2 = prim.indices[tri_idx + 2] as usize;

                if i0 >= self.vertices.len()
                    || i1 >= self.vertices.len()
                    || i2 >= self.vertices.len()
                {
                    continue;
                }

                let v0 = self.vertices[i0].position;
                let v1 = self.vertices[i1].position;
                let v2 = self.vertices[i2].position;

                let edge1 = v1 - v0;
                let edge2 = v2 - v0;
                let normal = edge1.cross(edge2).normalize();

                self.vertices[i0].normal = normal;
                self.vertices[i1].normal = normal;
                self.vertices[i2].normal = normal;
            }
        }
        self.has_normals = true;
    }

    /// Recalculate AABB from vertices
    pub fn recalculate_aabb(&mut self) {
        self.aabb = compute_aabb(&self.vertices);
    }

    /// Transform all vertices by matrix
    pub fn transform(&mut self, mat: Mat4) {
        for v in &mut self.vertices {
            v.position = mat.transform_point3(v.position);
            v.normal = mat.transform_direction3(v.normal);
        }
        self.recalculate_aabb();
    }

    /// Invert V coordinate of UVs
    pub fn invert_v(&mut self) {
        for v in &mut self.vertices {
            v.texcoord.y = 1.0 - v.texcoord.y;
        }
    }

    // --- Primitive generators ---

    /// Create unit cube centered at origin
    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;

        let vertices = vec![
            // Front face (Z+)
            Vertex::new(Vec3::new(-half, -half, half), Vec3::Z, Vec2::new(0.0, 0.0)),
            Vertex::new(Vec3::new(half, -half, half), Vec3::Z, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new(half, half, half), Vec3::Z, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(-half, half, half), Vec3::Z, Vec2::new(0.0, 1.0)),
            // Back face (Z-)
            Vertex::new(Vec3::new(half, -half, -half), -Vec3::Z, Vec2::new(0.0, 0.0)),
            Vertex::new(
                Vec3::new(-half, -half, -half),
                -Vec3::Z,
                Vec2::new(1.0, 0.0),
            ),
            Vertex::new(Vec3::new(-half, half, -half), -Vec3::Z, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(half, half, -half), -Vec3::Z, Vec2::new(0.0, 1.0)),
            // Top face (Y+)
            Vertex::new(Vec3::new(-half, half, half), Vec3::Y, Vec2::new(0.0, 0.0)),
            Vertex::new(Vec3::new(half, half, half), Vec3::Y, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new(half, half, -half), Vec3::Y, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(-half, half, -half), Vec3::Y, Vec2::new(0.0, 1.0)),
            // Bottom face (Y-)
            Vertex::new(
                Vec3::new(-half, -half, -half),
                -Vec3::Y,
                Vec2::new(0.0, 0.0),
            ),
            Vertex::new(Vec3::new(half, -half, -half), -Vec3::Y, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new(half, -half, half), -Vec3::Y, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(-half, -half, half), -Vec3::Y, Vec2::new(0.0, 1.0)),
            // Right face (X+)
            Vertex::new(Vec3::new(half, -half, half), Vec3::X, Vec2::new(0.0, 0.0)),
            Vertex::new(Vec3::new(half, -half, -half), Vec3::X, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new(half, half, -half), Vec3::X, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(half, half, half), Vec3::X, Vec2::new(0.0, 1.0)),
            // Left face (X-)
            Vertex::new(
                Vec3::new(-half, -half, -half),
                -Vec3::X,
                Vec2::new(0.0, 0.0),
            ),
            Vertex::new(Vec3::new(-half, -half, half), -Vec3::X, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new(-half, half, half), -Vec3::X, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(-half, half, -half), -Vec3::X, Vec2::new(0.0, 1.0)),
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3, // Front
            4, 5, 6, 4, 6, 7, // Back
            8, 9, 10, 8, 10, 11, // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];

        Self::from_vertices(vertices, indices)
    }

    /// Create sphere using UV sphere algorithm
    pub fn sphere(radius: f32, segments: usize, rings: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for ring in 0..=rings {
            let theta = (ring as f32 / rings as f32) * core::f32::consts::PI;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for seg in 0..=segments {
                let phi = (seg as f32 / segments as f32) * 2.0 * core::f32::consts::PI;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                let position = Vec3::new(x * radius, y * radius, z * radius);
                let normal = Vec3::new(x, y, z);
                let texcoord = Vec2::new(seg as f32 / segments as f32, ring as f32 / rings as f32);

                vertices.push(Vertex::new(position, normal, texcoord));
            }
        }

        // Generate indices
        for ring in 0..rings {
            for seg in 0..segments {
                let first = ring * (segments + 1) + seg;
                let second = first + segments + 1;

                indices.push(first as u32);
                indices.push(second as u32);
                indices.push((first + 1) as u32);

                indices.push((first + 1) as u32);
                indices.push(second as u32);
                indices.push((second + 1) as u32);
            }
        }

        Self::from_vertices(vertices, indices)
    }

    /// Create plane grid
    pub fn plane(size: f32, segments: usize) -> Self {
        let half = size / 2.0;
        let step = size / segments as f32;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for y in 0..=segments {
            for x in 0..=segments {
                let px = -half + x as f32 * step;
                let py = -half + y as f32 * step;
                vertices.push(Vertex::new(
                    Vec3::new(px, 0.0, py),
                    Vec3::Y,
                    Vec2::new(x as f32 / segments as f32, y as f32 / segments as f32),
                ));
            }
        }

        // Generate indices
        let row_verts = segments + 1;
        for y in 0..segments {
            for x in 0..segments {
                let i = y * row_verts + x;
                indices.push(i as u32);
                indices.push((i + row_verts) as u32);
                indices.push((i + 1) as u32);

                indices.push((i + 1) as u32);
                indices.push((i + row_verts) as u32);
                indices.push((i + row_verts + 1) as u32);
            }
        }

        Self::from_vertices(vertices, indices)
    }

    /// Create cylinder
    pub fn cylinder(radius: f32, height: f32, segments: usize) -> Self {
        let half_height = height / 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Side vertices
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * core::f32::consts::PI;
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            // Top vertex
            vertices.push(Vertex::new(
                Vec3::new(cos_a * radius, half_height, sin_a * radius),
                Vec3::new(cos_a, 0.0, sin_a),
                Vec2::new(i as f32 / segments as f32, 0.0),
            ));
            // Bottom vertex
            vertices.push(Vertex::new(
                Vec3::new(cos_a * radius, -half_height, sin_a * radius),
                Vec3::new(cos_a, 0.0, sin_a),
                Vec2::new(i as f32 / segments as f32, 1.0),
            ));
        }

        // Side indices
        for i in 0..segments {
            let top = i * 2;
            let bottom = top + 1;
            let next_top = top + 2;
            let next_bottom = bottom + 2;

            indices.push(top as u32);
            indices.push(bottom as u32);
            indices.push(next_top as u32);

            indices.push(next_top as u32);
            indices.push(bottom as u32);
            indices.push(next_bottom as u32);
        }

        // Top cap center
        let top_center_idx = vertices.len();
        vertices.push(Vertex::new(
            Vec3::new(0.0, half_height, 0.0),
            Vec3::Y,
            Vec2::new(0.5, 0.5),
        ));

        // Top cap vertices
        let top_cap_start = vertices.len();
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * core::f32::consts::PI;
            vertices.push(Vertex::new(
                Vec3::new(angle.cos() * radius, half_height, angle.sin() * radius),
                Vec3::Y,
                Vec2::new(angle.cos() * 0.5 + 0.5, angle.sin() * 0.5 + 0.5),
            ));
        }

        // Top cap indices
        for i in 0..segments {
            indices.push(top_center_idx as u32);
            indices.push((top_cap_start + i + 1) as u32);
            indices.push((top_cap_start + i) as u32);
        }

        // Bottom cap center
        let bottom_center_idx = vertices.len();
        vertices.push(Vertex::new(
            Vec3::new(0.0, -half_height, 0.0),
            -Vec3::Y,
            Vec2::new(0.5, 0.5),
        ));

        // Bottom cap vertices
        let bottom_cap_start = vertices.len();
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * core::f32::consts::PI;
            vertices.push(Vertex::new(
                Vec3::new(angle.cos() * radius, -half_height, angle.sin() * radius),
                -Vec3::Y,
                Vec2::new(angle.cos() * 0.5 + 0.5, angle.sin() * 0.5 + 0.5),
            ));
        }

        // Bottom cap indices
        for i in 0..segments {
            indices.push(bottom_center_idx as u32);
            indices.push((bottom_cap_start + i) as u32);
            indices.push((bottom_cap_start + i + 1) as u32);
        }

        Self::from_vertices(vertices, indices)
    }

    /// Create cone
    pub fn cone(radius: f32, height: f32, segments: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Apex vertex
        let apex_idx = 0;
        vertices.push(Vertex::new(
            Vec3::new(0.0, height, 0.0),
            Vec3::Y,
            Vec2::new(0.5, 0.0),
        ));

        // Base vertices
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * core::f32::consts::PI;
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            // Compute normal for cone side
            let slope = radius / height;
            let normal = Vec3::new(cos_a, slope, sin_a).normalize();

            vertices.push(Vertex::new(
                Vec3::new(cos_a * radius, 0.0, sin_a * radius),
                normal,
                Vec2::new(i as f32 / segments as f32, 1.0),
            ));
        }

        // Side indices
        for i in 0..segments {
            indices.push(apex_idx as u32);
            indices.push((i + 1) as u32);
            indices.push((i + 2) as u32);
        }

        // Bottom cap center
        let bottom_center_idx = vertices.len();
        vertices.push(Vertex::new(
            Vec3::new(0.0, 0.0, 0.0),
            -Vec3::Y,
            Vec2::new(0.5, 0.5),
        ));

        // Bottom cap indices
        for i in 0..segments {
            indices.push(bottom_center_idx as u32);
            indices.push((i + 2) as u32);
            indices.push((i + 1) as u32);
        }

        Self::from_vertices(vertices, indices)
    }

    /// Create torus (donut shape)
    pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: usize,
        minor_segments: usize,
    ) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for major in 0..=major_segments {
            let major_angle = (major as f32 / major_segments as f32) * 2.0 * core::f32::consts::PI;
            let cos_major = major_angle.cos();
            let sin_major = major_angle.sin();

            for minor in 0..=minor_segments {
                let minor_angle =
                    (minor as f32 / minor_segments as f32) * 2.0 * core::f32::consts::PI;
                let cos_minor = minor_angle.cos();
                let sin_minor = minor_angle.sin();

                let x = (major_radius + minor_radius * cos_minor) * cos_major;
                let y = minor_radius * sin_minor;
                let z = (major_radius + minor_radius * cos_minor) * sin_major;

                let nx = cos_minor * cos_major;
                let ny = sin_minor;
                let nz = cos_minor * sin_major;

                vertices.push(Vertex::new(
                    Vec3::new(x, y, z),
                    Vec3::new(nx, ny, nz),
                    Vec2::new(
                        major as f32 / major_segments as f32,
                        minor as f32 / minor_segments as f32,
                    ),
                ));
            }
        }

        // Generate indices
        let minor_verts = minor_segments + 1;
        for major in 0..major_segments {
            for minor in 0..minor_segments {
                let i = major * minor_verts + minor;
                let next_major = i + minor_verts;

                indices.push(i as u32);
                indices.push(next_major as u32);
                indices.push((i + 1) as u32);

                indices.push((i + 1) as u32);
                indices.push(next_major as u32);
                indices.push((next_major + 1) as u32);
            }
        }

        Self::from_vertices(vertices, indices)
    }
}

impl Default for Mesh3D {
    fn default() -> Self {
        Self::new()
    }
}

/// Mesh builder for procedural mesh generation
#[derive(Debug, Default)]
pub struct MeshBuilder3D {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl MeshBuilder3D {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertex(&mut self, v: Vertex) -> &mut Self {
        self.vertices.push(v);
        self
    }

    pub fn index(&mut self, i: u32) -> &mut Self {
        self.indices.push(i);
        self
    }

    pub fn triangle(&mut self, a: u32, b: u32, c: u32) -> &mut Self {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
        self
    }

    pub fn quad(&mut self, a: u32, b: u32, c: u32, d: u32) -> &mut Self {
        self.triangle(a, b, c);
        self.triangle(a, c, d);
        self
    }

    pub fn build(&self) -> Mesh3D {
        Mesh3D::from_vertices(self.vertices.clone(), self.indices.clone())
    }
}

/// Compute AABB from vertices
fn compute_aabb(vertices: &[Vertex]) -> AABB {
    if vertices.is_empty() {
        return AABB::EMPTY;
    }
    let mut aabb = AABB::EMPTY;
    for v in vertices {
        aabb = aabb.extend(v.position);
    }
    aabb
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_math::{Vec2, Vec3};

    #[test]
    fn test_mesh_cube() {
        let cube = Mesh3D::cube(1.0);
        assert_eq!(cube.vertex_count(), 24);
        assert_eq!(cube.triangle_count(), 12);
        assert!(cube.has_normals());
    }

    #[test]
    fn test_mesh_sphere() {
        let sphere = Mesh3D::sphere(1.0, 16, 8);
        let expected_verts = (16 + 1) * (8 + 1);
        assert_eq!(sphere.vertex_count(), expected_verts);
        assert!(sphere.has_normals());
    }

    #[test]
    fn test_mesh_plane() {
        let plane = Mesh3D::plane(10.0, 4);
        let expected_verts = (4 + 1) * (4 + 1);
        assert_eq!(plane.vertex_count(), expected_verts);
    }

    #[test]
    fn test_mesh_builder() {
        let mut builder = MeshBuilder3D::new();
        builder
            .vertex(Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO))
            .vertex(Vertex::new(Vec3::X, Vec3::Y, Vec2::X))
            .vertex(Vertex::new(Vec3::Z, Vec3::Y, Vec2::Y))
            .triangle(0, 1, 2);

        let mesh = builder.build();
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_mesh_aabb() {
        let cube = Mesh3D::cube(2.0);
        let aabb = cube.aabb();
        assert_eq!(aabb.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(aabb.max, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_mesh_compute_normals() {
        let mut mesh = Mesh3D::from_vertices(
            vec![
                Vertex::new(Vec3::ZERO, Vec3::ZERO, Vec2::ZERO),
                Vertex::new(Vec3::X, Vec3::ZERO, Vec2::ZERO),
                Vertex::new(Vec3::Y, Vec3::ZERO, Vec2::ZERO),
            ],
            vec![0, 1, 2],
        );
        assert!(!mesh.has_normals());
        mesh.compute_normals();
        assert!(mesh.has_normals());
    }

    #[test]
    fn test_mesh_primitive_triangle_count() {
        let p = Primitive::new(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(p.triangle_count(), 2);
    }

    #[test]
    fn test_mesh_primitive_empty() {
        let p = Primitive::new(Vec::new());
        assert_eq!(p.triangle_count(), 0);
    }

    #[test]
    fn test_mesh_new_empty() {
        let mesh = Mesh3D::new();
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
        assert!(!mesh.has_normals());
        assert!(!mesh.has_uv());
    }

    #[test]
    fn test_mesh_default() {
        let mesh = Mesh3D::default();
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_mesh_from_vertices_has_uv() {
        let vertices = vec![
            Vertex::new(Vec3::ZERO, Vec3::ZERO, Vec2::new(0.5, 0.5)),
            Vertex::new(Vec3::X, Vec3::ZERO, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::Y, Vec3::ZERO, Vec2::new(0.0, 1.0)),
        ];
        let mesh = Mesh3D::from_vertices(vertices, vec![0, 1, 2]);
        assert!(mesh.has_uv());
    }

    #[test]
    fn test_mesh_cube_vertex_count() {
        let cube = Mesh3D::cube(1.0);
        assert_eq!(cube.vertex_count(), 24);
        assert_eq!(cube.triangle_count(), 12);
    }

    #[test]
    fn test_mesh_cube_aabb() {
        let cube = Mesh3D::cube(2.0);
        let aabb = cube.aabb();
        assert_eq!(aabb.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(aabb.max, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_mesh_cube_bounding_sphere() {
        let cube = Mesh3D::cube(2.0);
        let sphere = cube.bounding_sphere();
        // Center should be at origin
        assert!((sphere.center.x).abs() < 1e-5);
        assert!((sphere.center.y).abs() < 1e-5);
        assert!((sphere.center.z).abs() < 1e-5);
        // Radius should be half-extents length (~sqrt(3))
        assert!(sphere.radius > 1.7);
        assert!(sphere.radius < 1.8);
    }

    #[test]
    fn test_mesh_sphere_resolution() {
        let sphere_low = Mesh3D::sphere(1.0, 8, 4);
        let sphere_high = Mesh3D::sphere(1.0, 16, 8);
        assert!(sphere_low.vertex_count() < sphere_high.vertex_count());
    }

    #[test]
    fn test_mesh_sphere_has_normals() {
        let sphere = Mesh3D::sphere(1.0, 8, 4);
        assert!(sphere.has_normals());
    }

    #[test]
    fn test_mesh_plane_segments() {
        let plane = Mesh3D::plane(10.0, 10);
        let expected_verts = (10 + 1) * (10 + 1);
        assert_eq!(plane.vertex_count(), expected_verts);
    }

    #[test]
    fn test_mesh_cylinder_vertices() {
        let cylinder = Mesh3D::cylinder(1.0, 2.0, 12);
        assert!(cylinder.vertex_count() > 0);
        assert!(cylinder.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_cone_vertices() {
        let cone = Mesh3D::cone(1.0, 2.0, 12);
        assert!(cone.vertex_count() > 0);
        assert!(cone.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_torus_vertices() {
        let torus = Mesh3D::torus(1.0, 0.3, 16, 8);
        assert!(torus.vertex_count() > 0);
        assert!(torus.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_transform_translation() {
        let mut mesh = Mesh3D::cube(1.0);
        let transform = Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0));
        mesh.transform(transform);
        let aabb = mesh.aabb();
        assert!(aabb.min.x > 3.0);
        assert!(aabb.max.x > 3.0);
    }

    #[test]
    fn test_mesh_recalculate_aabb() {
        let mut mesh = Mesh3D::cube(1.0);
        // Manually move vertices and recalculate
        mesh.recalculate_aabb();
        let aabb = mesh.aabb();
        assert!(aabb.min.x >= -0.6 && aabb.min.x <= -0.4);
        assert!(aabb.max.x >= 0.4 && aabb.max.x <= 0.6);
    }

    #[test]
    fn test_mesh_invert_v() {
        let mut mesh = Mesh3D::cube(1.0);
        let original_uv = mesh.vertices()[0].texcoord.y;
        mesh.invert_v();
        let inverted_uv = mesh.vertices()[0].texcoord.y;
        assert!((inverted_uv - (1.0 - original_uv)).abs() < 1e-5);
    }

    #[test]
    fn test_mesh_with_primitives() {
        let vertices = vec![
            Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO),
            Vertex::new(Vec3::X, Vec3::Y, Vec2::X),
            Vertex::new(Vec3::Z, Vec3::Y, Vec2::Y),
            Vertex::new(-Vec3::X, Vec3::Y, -Vec2::X),
        ];
        let primitives = vec![Primitive::new(vec![0, 1, 2]), Primitive::new(vec![0, 2, 3])];
        let mesh = Mesh3D::with_primitives(vertices, primitives);
        assert_eq!(mesh.primitive_count(), 2);
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.triangle_count(), 2);
    }

    #[test]
    fn test_mesh_builder_triangle() {
        let mut builder = MeshBuilder3D::new();
        builder
            .vertex(Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO))
            .vertex(Vertex::new(Vec3::new(1.0, 0.0, 0.0), Vec3::Y, Vec2::X))
            .vertex(Vertex::new(Vec3::new(0.0, 0.0, 1.0), Vec3::Y, Vec2::Y))
            .triangle(0, 1, 2);
        let mesh = builder.build();
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_mesh_builder_quad() {
        let mut builder = MeshBuilder3D::new();
        builder
            .vertex(Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO))
            .vertex(Vertex::new(Vec3::X, Vec3::Y, Vec2::X))
            .vertex(Vertex::new(Vec3::new(1.0, 0.0, 1.0), Vec3::Y, Vec2::ONE))
            .vertex(Vertex::new(Vec3::Z, Vec3::Y, Vec2::Y))
            .quad(0, 1, 2, 3);
        let mesh = builder.build();
        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.triangle_count(), 2);
    }

    #[test]
    fn test_mesh_builder_default() {
        let builder = MeshBuilder3D::default();
        let mesh = builder.build();
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_mesh_cube_different_sizes() {
        let small = Mesh3D::cube(0.5);
        let large = Mesh3D::cube(2.0);
        assert_eq!(small.vertex_count(), large.vertex_count());
        let small_aabb = small.aabb();
        let large_aabb = large.aabb();
        assert!(small_aabb.min.x > large_aabb.min.x);
        assert!(small_aabb.max.x < large_aabb.max.x);
    }

    #[test]
    fn test_primitive_material_default() {
        let p = Primitive::new(vec![0, 1, 2]);
        assert_eq!(p.material_index, None);
        assert_eq!(p.first_vertex, 0);
        assert_eq!(p.vertex_count, 0);
    }

    #[test]
    fn test_mesh_cylinder_resolution() {
        let low = Mesh3D::cylinder(1.0, 2.0, 8);
        let high = Mesh3D::cylinder(1.0, 2.0, 32);
        assert!(low.vertex_count() < high.vertex_count());
    }

    #[test]
    fn test_mesh_cone_resolution() {
        let low = Mesh3D::cone(1.0, 2.0, 8);
        let high = Mesh3D::cone(1.0, 2.0, 32);
        assert!(low.vertex_count() < high.vertex_count());
    }

    #[test]
    fn test_mesh_torus_resolution() {
        let low = Mesh3D::torus(1.0, 0.3, 8, 4);
        let high = Mesh3D::torus(1.0, 0.3, 32, 16);
        assert!(low.vertex_count() < high.vertex_count());
    }
}
