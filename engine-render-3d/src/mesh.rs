//! 3D 网格模块
//!
//! 提供 Mesh3D、Vertex3D、MeshBuilder3D 等 3D 渲染所需的核心网格类型。

use engine_math::{Vec2, Vec3, Vec4};
use engine_utils::Handle;

/// 3D 顶点结构
///
/// 包含位置、法线、UV 坐标和切线信息。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex3D {
    /// 位置
    pub position: Vec3,
    /// 法线
    pub normal: Vec3,
    /// UV 坐标
    pub texcoord: Vec2,
    /// 切线 (xyz = tangent, w = handedness)
    pub tangent: Vec4,
}

impl Vertex3D {
    /// 创建新顶点
    pub fn new(position: Vec3, normal: Vec3, texcoord: Vec2) -> Self {
        Self {
            position,
            normal,
            texcoord,
            tangent: Vec4::new(1.0, 0.0, 0.0, 1.0),
        }
    }

    /// 创建带切线的顶点
    pub fn with_tangent(mut self, tangent: Vec4) -> Self {
        self.tangent = tangent;
        self
    }

    /// 获取位置
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// 获取法线
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// 获取 UV 坐标
    pub fn texcoord(&self) -> Vec2 {
        self.texcoord
    }
}

/// 3D 网格结构
#[derive(Debug, Clone)]
pub struct Mesh3D {
    /// 顶点数据
    vertices: Vec<Vertex3D>,
    /// 索引数据
    indices: Vec<u32>,
    /// 子网格列表
    primitives: Vec<Primitive>,
    /// 本地空间包围盒
    aabb: AABB,
    /// 包围球
    bounding_sphere: Sphere,
}

impl Mesh3D {
    /// 从顶点和索引创建网格
    pub fn from_vertices(vertices: Vec<Vertex3D>, indices: Vec<u32>) -> Self {
        let index_count = indices.len() as u32;
        let aabb = AABB::from_points(vertices.iter().map(|v| v.position));
        let bounding_sphere = Sphere::from_aabb(&aabb);
        Self {
            vertices,
            indices,
            primitives: vec![Primitive::new(0, index_count, 0)],
            aabb,
            bounding_sphere,
        }
    }

    /// 创建空网格
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            primitives: Vec::new(),
            aabb: AABB::new(Vec3::ZERO, Vec3::ZERO),
            bounding_sphere: Sphere::new(Vec3::ZERO, 0.0),
        }
    }

    /// 获取顶点数量
    pub fn vertices(&self) -> usize {
        self.vertices.len()
    }

    /// 获取三角面数量
    pub fn triangles(&self) -> usize {
        self.indices.len() / 3
    }

    /// 获取索引数量
    pub fn indices(&self) -> usize {
        self.indices.len()
    }

    /// 获取子网格数量
    pub fn primitive_count(&self) -> usize {
        self.primitives.len()
    }

    /// 获取所有顶点
    pub fn vertices_array(&self) -> &[Vertex3D] {
        &self.vertices
    }

    /// 获取所有索引
    pub fn indices_array(&self) -> &[u32] {
        &self.indices
    }

    /// 获取子网格列表
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }

    /// 获取本地包围盒
    pub fn aabb(&self) -> AABB {
        self.aabb
    }

    /// 获取包围球
    pub fn bounding_sphere(&self) -> Sphere {
        self.bounding_sphere
    }

    /// 检查是否有法线
    pub fn has_normals(&self) -> bool {
        !self.vertices.is_empty() && self.vertices.iter().all(|v| v.normal != Vec3::ZERO)
    }

    /// 检查是否有 UV
    pub fn has_uv(&self) -> bool {
        !self.vertices.is_empty() && self.vertices.iter().all(|v| v.texcoord != Vec2::ZERO)
    }

    /// 检查是否有切线
    pub fn has_tangents(&self) -> bool {
        !self.vertices.is_empty()
            && self
                .vertices
                .iter()
                .all(|v| v.tangent != Vec4::new(1.0, 0.0, 0.0, 1.0))
    }

    /// 计算法线（如果缺失）
    pub fn compute_normals(&mut self) {
        if self.has_normals() {
            return;
        }

        // 重置所有法线为零
        for vertex in &mut self.vertices {
            vertex.normal = Vec3::ZERO;
        }

        // 计算每个三角形的法线并累加到顶点
        for chunk in self.indices.chunks(3) {
            if chunk.len() < 3 {
                continue;
            }
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            if i0 >= self.vertices.len() || i1 >= self.vertices.len() || i2 >= self.vertices.len() {
                continue;
            }

            let v0 = &self.vertices[i0];
            let v1 = &self.vertices[i1];
            let v2 = &self.vertices[i2];

            let edge1 = v1.position - v0.position;
            let edge2 = v2.position - v0.position;
            let face_normal = edge1.cross(edge2);

            self.vertices[i0].normal = self.vertices[i0].normal + face_normal;
            self.vertices[i1].normal = self.vertices[i1].normal + face_normal;
            self.vertices[i2].normal = self.vertices[i2].normal + face_normal;
        }

        // 归一化所有法线
        for vertex in &mut self.vertices {
            if vertex.normal != Vec3::ZERO {
                vertex.normal = vertex.normal.normalize();
            } else {
                vertex.normal = Vec3::new(0.0, 1.0, 0.0);
            }
        }
    }

    /// 计算切线（如果缺失）
    pub fn compute_tangents(&mut self) {
        if self.has_tangents() || !self.has_uv() {
            return;
        }

        // 重置所有切线为零
        for vertex in &mut self.vertices {
            vertex.tangent = Vec4::ZERO;
        }

        // 计算每个三角形的切线
        for chunk in self.indices.chunks(3) {
            if chunk.len() < 3 {
                continue;
            }
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            if i0 >= self.vertices.len() || i1 >= self.vertices.len() || i2 >= self.vertices.len() {
                continue;
            }

            let v0 = &self.vertices[i0];
            let v1 = &self.vertices[i1];
            let v2 = &self.vertices[i2];

            let edge1 = v1.position - v0.position;
            let edge2 = v2.position - v0.position;

            let delta_uv1 = v1.texcoord - v0.texcoord;
            let delta_uv2 = v2.texcoord - v0.texcoord;

            let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);

            let tangent = Vec3::new(
                (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x) * r,
                (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y) * r,
                (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z) * r,
            );

            self.vertices[i0].tangent = self.vertices[i0].tangent + Vec4::new(tangent.x, tangent.y, tangent.z, 0.0);
            self.vertices[i1].tangent = self.vertices[i1].tangent + Vec4::new(tangent.x, tangent.y, tangent.z, 0.0);
            self.vertices[i2].tangent = self.vertices[i2].tangent + Vec4::new(tangent.x, tangent.y, tangent.z, 0.0);
        }

        // 归一化并计算 handedness
        for vertex in &mut self.vertices {
            let tangent = Vec3::new(vertex.tangent.x, vertex.tangent.y, vertex.tangent.z);
            if tangent != Vec3::ZERO {
                let tangent_norm = tangent.normalize();
                let bitangent = vertex.normal.cross(tangent_norm);
                let handedness = if bitangent.dot(Vec3::new(vertex.tangent.x, vertex.tangent.y, vertex.tangent.z)) < 0.0 {
                    -1.0
                } else {
                    1.0
                };
                vertex.tangent = Vec4::new(tangent_norm.x, tangent_norm.y, tangent_norm.z, handedness);
            } else {
                vertex.tangent = Vec4::new(1.0, 0.0, 0.0, 1.0);
            }
        }
    }

    /// 重新计算包围盒
    pub fn recalculate_aabb(&mut self) {
        self.aabb = AABB::from_points(self.vertices.iter().map(|v| v.position));
        self.bounding_sphere = Sphere::from_aabb(&self.aabb);
    }

    /// 翻转 V 坐标
    pub fn invert_v(&mut self) {
        for vertex in &mut self.vertices {
            vertex.texcoord = Vec2::new(vertex.texcoord.x, 1.0 - vertex.texcoord.y);
        }
    }

    /// 创建立方体网格
    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;
        let vertices = vec![
            // Front face
            Vertex3D::new(Vec3::new(-half, -half, half), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
            Vertex3D::new(Vec3::new(half, -half, half), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
            Vertex3D::new(Vec3::new(half, half, half), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),
            Vertex3D::new(Vec3::new(-half, half, half), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
            // Back face
            Vertex3D::new(Vec3::new(-half, -half, -half), Vec3::new(0.0, 0.0, -1.0), Vec2::new(1.0, 0.0)),
            Vertex3D::new(Vec3::new(-half, half, -half), Vec3::new(0.0, 0.0, -1.0), Vec2::new(1.0, 1.0)),
            Vertex3D::new(Vec3::new(half, half, -half), Vec3::new(0.0, 0.0, -1.0), Vec2::new(0.0, 1.0)),
            Vertex3D::new(Vec3::new(half, -half, -half), Vec3::new(0.0, 0.0, -1.0), Vec2::new(0.0, 0.0)),
            // Top face
            Vertex3D::new(Vec3::new(-half, half, -half), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0)),
            Vertex3D::new(Vec3::new(-half, half, half), Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 1.0)),
            Vertex3D::new(Vec3::new(half, half, half), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 1.0)),
            Vertex3D::new(Vec3::new(half, half, -half), Vec3::new(0.0, 1.0, 0.0), Vec2::new(1.0, 0.0)),
            // Bottom face
            Vertex3D::new(Vec3::new(-half, -half, -half), Vec3::new(0.0, -1.0, 0.0), Vec2::new(0.0, 1.0)),
            Vertex3D::new(Vec3::new(half, -half, -half), Vec3::new(0.0, -1.0, 0.0), Vec2::new(1.0, 1.0)),
            Vertex3D::new(Vec3::new(half, -half, half), Vec3::new(0.0, -1.0, 0.0), Vec2::new(1.0, 0.0)),
            Vertex3D::new(Vec3::new(-half, -half, half), Vec3::new(0.0, -1.0, 0.0), Vec2::new(0.0, 0.0)),
            // Right face
            Vertex3D::new(Vec3::new(half, -half, -half), Vec3::new(1.0, 0.0, 0.0), Vec2::new(0.0, 0.0)),
            Vertex3D::new(Vec3::new(half, half, -half), Vec3::new(1.0, 0.0, 0.0), Vec2::new(0.0, 1.0)),
            Vertex3D::new(Vec3::new(half, half, half), Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 1.0)),
            Vertex3D::new(Vec3::new(half, -half, half), Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 0.0)),
            // Left face
            Vertex3D::new(Vec3::new(-half, -half, -half), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(1.0, 0.0)),
            Vertex3D::new(Vec3::new(-half, -half, half), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(1.0, 1.0)),
            Vertex3D::new(Vec3::new(-half, half, half), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(0.0, 1.0)),
            Vertex3D::new(Vec3::new(-half, half, -half), Vec3::new(-1.0, 0.0, 0.0), Vec2::new(0.0, 0.0)),
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

    /// 创建球体网格
    pub fn sphere(radius: f32, segments: u32, rings: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..=rings {
            let phi = std::f32::consts::PI * i as f32 / rings as f32;
            for j in 0..=segments {
                let theta = 2.0 * std::f32::consts::PI * j as f32 / segments as f32;

                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos();
                let z = radius * phi.sin() * theta.sin();

                let nx = x / radius;
                let ny = y / radius;
                let nz = z / radius;

                vertices.push(Vertex3D::new(
                    Vec3::new(x, y, z),
                    Vec3::new(nx, ny, nz),
                    Vec2::new(j as f32 / segments as f32, i as f32 / rings as f32),
                ));
            }
        }

        for i in 0..rings {
            for j in 0..segments {
                let first = i * (segments + 1) + j;
                let second = first + segments + 1;

                indices.push(first as u32);
                indices.push(second as u32);
                indices.push(first as u32 + 1);

                indices.push(second as u32);
                indices.push(second as u32 + 1);
                indices.push(first as u32 + 1);
            }
        }

        Self::from_vertices(vertices, indices)
    }

    /// 创建平面网格
    pub fn plane(size: f32, segments: u32) -> Self {
        let half = size / 2.0;
        let segment_size = size / segments as f32;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..=segments {
            for j in 0..=segments {
                let x = -half + j as f32 * segment_size;
                let z = -half + i as f32 * segment_size;
                vertices.push(Vertex3D::new(
                    Vec3::new(x, 0.0, z),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec2::new(j as f32 / segments as f32, i as f32 / segments as f32),
                ));
            }
        }

        for i in 0..segments {
            for j in 0..segments {
                let row_start = i * (segments + 1);
                indices.push(row_start + j);
                indices.push(row_start + j + 1);
                indices.push(row_start + j + segments + 1);

                indices.push(row_start + j + 1);
                indices.push(row_start + j + segments + 2);
                indices.push(row_start + j + segments + 1);
            }
        }

        Self::from_vertices(vertices, indices)
    }

    /// 创建圆柱网格
    pub fn cylinder(radius: f32, height: f32, segments: u32) -> Self {
        let half_height = height / 2.0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // 侧面
        for i in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            let x = radius * theta.cos();
            let z = radius * theta.sin();

            vertices.push(Vertex3D::new(
                Vec3::new(x, -half_height, z),
                Vec3::new(x / radius, 0.0, z / radius),
                Vec2::new(i as f32 / segments as f32, 0.0),
            ));
            vertices.push(Vertex3D::new(
                Vec3::new(x, half_height, z),
                Vec3::new(x / radius, 0.0, z / radius),
                Vec2::new(i as f32 / segments as f32, 1.0),
            ));
        }

        for i in 0..segments {
            let base = i * 2;
            indices.push(base as u32);
            indices.push(base as u32 + 2);
            indices.push(base as u32 + 1);

            indices.push(base as u32 + 1);
            indices.push(base as u32 + 2);
            indices.push(base as u32 + 3);
        }

        // 顶面
        let top_center_idx = vertices.len() as u32;
        vertices.push(Vertex3D::new(
            Vec3::new(0.0, half_height, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.5, 0.5),
        ));

        for i in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            vertices.push(Vertex3D::new(
                Vec3::new(radius * theta.cos(), half_height, radius * theta.sin()),
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(0.5 + 0.5 * theta.cos(), 0.5 + 0.5 * theta.sin()),
            ));
        }

        for i in 0..segments {
            indices.push(top_center_idx);
            indices.push(top_center_idx + i as u32 + 2);
            indices.push(top_center_idx + i as u32 + 1);
        }

        // 底面
        let bottom_center_idx = vertices.len() as u32;
        vertices.push(Vertex3D::new(
            Vec3::new(0.0, -half_height, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec2::new(0.5, 0.5),
        ));

        for i in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            vertices.push(Vertex3D::new(
                Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin()),
                Vec3::new(0.0, -1.0, 0.0),
                Vec2::new(0.5 + 0.5 * theta.cos(), 0.5 + 0.5 * theta.sin()),
            ));
        }

        for i in 0..segments {
            indices.push(bottom_center_idx);
            indices.push(bottom_center_idx + i as u32 + 1);
            indices.push(bottom_center_idx + i as u32 + 2);
        }

        Self::from_vertices(vertices, indices)
    }

    /// 创建胶囊网格
    pub fn capsule(radius: f32, height: f32, segments: u32) -> Self {
        let half_height = height / 2.0 - radius;
        let mut mesh = MeshBuilder3D::new();

        // 圆柱部分
        let cylinder_segments = segments;
        for i in 0..=cylinder_segments {
            let theta = 2.0 * std::f32::consts::PI * i as f32 / cylinder_segments as f32;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            // 底面圆
            mesh.vertex(Vertex3D::new(
                Vec3::new(radius * cos_theta, -half_height, radius * sin_theta),
                Vec3::new(cos_theta, 0.0, sin_theta),
                Vec2::new(i as f32 / cylinder_segments as f32, 0.0),
            ));
            // 顶面圆
            mesh.vertex(Vertex3D::new(
                Vec3::new(radius * cos_theta, half_height, radius * sin_theta),
                Vec3::new(cos_theta, 0.0, sin_theta),
                Vec2::new(i as f32 / cylinder_segments as f32, 1.0),
            ));
        }

        for i in 0..cylinder_segments {
            let base = i * 2;
            mesh.triangle(base as u32, base as u32 + 2, base as u32 + 1);
            mesh.triangle(base as u32 + 1, base as u32 + 2, base as u32 + 3);
        }

        // 球形顶部
        let sphere_segments = segments;
        let sphere_rings = segments / 2;
        let top_center_idx = mesh.vertex_count() as u32;

        mesh.vertex(Vertex3D::new(
            Vec3::new(0.0, half_height + radius, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.5, 0.0),
        ));

        for i in 1..=sphere_rings {
            let phi = std::f32::consts::PI * i as f32 / sphere_rings as f32;
            for j in 0..=sphere_segments {
                let theta = 2.0 * std::f32::consts::PI * j as f32 / sphere_segments as f32;
                let y = half_height + radius - radius * phi.cos();
                let r = radius * phi.sin();

                mesh.vertex(Vertex3D::new(
                    Vec3::new(r * theta.cos(), y, r * theta.sin()),
                    Vec3::new(theta.cos() * phi.sin(), phi.cos(), theta.sin() * phi.sin()),
                    Vec2::new(j as f32 / sphere_segments as f32, i as f32 / sphere_rings as f32),
                ));
            }
        }

        for i in 0..sphere_rings {
            for j in 0..sphere_segments {
                let first = top_center_idx + 1 + i * (sphere_segments + 1) + j;
                let second = first + sphere_segments + 1;

                if i == 0 {
                    mesh.triangle(top_center_idx, first as u32, second as u32);
                    mesh.triangle(top_center_idx, second as u32, (first + 1) as u32);
                } else {
                    mesh.triangle(first as u32, second as u32, (first + 1) as u32);
                    mesh.triangle((first + 1) as u32, second as u32, (second + 1) as u32);
                }
            }
        }

        // 球形底部
        let bottom_center_idx = mesh.vertex_count() as u32;

        mesh.vertex(Vertex3D::new(
            Vec3::new(0.0, -half_height - radius, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec2::new(0.5, 1.0),
        ));

        for i in 1..=sphere_rings {
            let phi = std::f32::consts::PI * i as f32 / sphere_rings as f32;
            for j in 0..=sphere_segments {
                let theta = 2.0 * std::f32::consts::PI * j as f32 / sphere_segments as f32;
                let y = -half_height - radius + radius * phi.cos();
                let r = radius * phi.sin();

                mesh.vertex(Vertex3D::new(
                    Vec3::new(r * theta.cos(), y, r * theta.sin()),
                    Vec3::new(-theta.cos() * phi.sin(), -phi.cos(), -theta.sin() * phi.sin()),
                    Vec2::new(j as f32 / sphere_segments as f32, 1.0 - i as f32 / sphere_rings as f32),
                ));
            }
        }

        for i in 0..sphere_rings {
            for j in 0..sphere_segments {
                let first = bottom_center_idx + 1 + i * (sphere_segments + 1) + j;
                let second = first + sphere_segments + 1;

                if i == sphere_rings - 1 {
                    mesh.triangle(bottom_center_idx, (second + 1) as u32, first as u32);
                    mesh.triangle(bottom_center_idx, (first + 1) as u32, (second + 1) as u32);
                } else {
                    mesh.triangle(first as u32, (first + 1) as u32, second as u32);
                    mesh.triangle((first + 1) as u32, (second + 1) as u32, second as u32);
                }
            }
        }

        mesh.build()
    }
}

impl Default for Mesh3D {
    fn default() -> Self {
        Self::new()
    }
}

/// 子网格（图元）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Primitive {
    /// 起始索引
    start: u32,
    /// 索引数量
    count: u32,
    /// 材质索引
    material_index: u32,
}

impl Primitive {
    /// 创建新图元
    pub fn new(start: u32, count: u32, material_index: u32) -> Self {
        Self {
            start,
            count,
            material_index,
        }
    }

    /// 获取起始索引
    pub fn start(&self) -> u32 {
        self.start
    }

    /// 获取索引数量
    pub fn count(&self) -> u32 {
        self.count
    }

    /// 获取材质索引
    pub fn material_index(&self) -> u32 {
        self.material_index
    }
}

/// 3D 网格构建器
#[derive(Debug, Clone, Default)]
pub struct MeshBuilder3D {
    vertices: Vec<Vertex3D>,
    indices: Vec<u32>,
}

impl MeshBuilder3D {
    /// 创建新构建器
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// 添加顶点
    pub fn vertex(&mut self, vertex: Vertex3D) {
        self.vertices.push(vertex);
    }

    /// 添加索引
    pub fn index(&mut self, index: u32) {
        self.indices.push(index);
    }

    /// 添加三角面
    pub fn triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }

    /// 添加四边形（拆分为两个三角）
    pub fn quad(&mut self, a: u32, b: u32, c: u32, d: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
        self.indices.push(a);
        self.indices.push(c);
        self.indices.push(d);
    }

    /// 获取顶点数量
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// 获取索引数量
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    /// 构建网格
    pub fn build(&self) -> Mesh3D {
        Mesh3D::from_vertices(self.vertices.clone(), self.indices.clone())
    }
}

/// 网格管理器
pub struct MeshManager {
    meshes: Vec<Mesh3D>,
}

impl MeshManager {
    /// 创建新管理器
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    /// 获取网格数量
    pub fn len(&self) -> usize {
        self.meshes.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }
}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}

// 导入必要的类型
use crate::geometry::{AABB, Sphere};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_triangles() {
        let cube = Mesh3D::cube(1.0);
        assert_eq!(cube.triangles(), 12, "Cube should have 12 triangles");
    }

    #[test]
    fn test_sphere_vertices() {
        let sphere = Mesh3D::sphere(1.0, 8, 4);
        // (rings+1) * (segments+1) vertices
        assert_eq!(sphere.vertices(), 45);
    }

    #[test]
    fn test_mesh_builder() {
        let mut builder = MeshBuilder3D::new();
        builder.vertex(Vertex3D::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.0, 0.0),
        ));
        builder.vertex(Vertex3D::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(1.0, 0.0),
        ));
        builder.vertex(Vertex3D::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ));
        builder.triangle(0, 1, 2);

        let mesh = builder.build();
        assert_eq!(mesh.vertices(), 3);
        assert_eq!(mesh.triangles(), 1);
    }

    #[test]
    fn test_compute_normals() {
        let mut mesh = Mesh3D::cube(1.0);
        // Temporarily corrupt normals to test computation
        for v in &mut mesh.vertices {
            v.normal = Vec3::ZERO;
        }
        mesh.compute_normals();
        assert!(mesh.has_normals());
    }
}
