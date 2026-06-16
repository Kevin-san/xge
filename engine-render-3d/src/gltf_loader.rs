//! GLTF 模型加载模块
//!
//! 通过 `gltf` crate 加载 GLTF/GLB 格式的 3D 模型。

use anyhow::{anyhow, Context, Result};
use engine_math::{Quat, Vec2, Vec3, Vec4};

use crate::material::Material3D;
use crate::mesh::{Mesh3D, Primitive, Vertex3D};

/// GLTF 模型
#[derive(Debug, Clone)]
pub struct GltfModel {
    /// 网格列表
    pub meshes: Vec<Mesh3D>,
    /// 材质列表
    pub materials: Vec<Material3D>,
    /// 节点列表
    pub nodes: Vec<GltfNode>,
    /// 场景中的根节点
    pub scene_roots: Vec<usize>,
    /// 全局包围盒
    pub aabb: crate::geometry::AABB,
}

/// GLTF 节点
#[derive(Debug, Clone)]
pub struct GltfNode {
    /// 节点名称
    pub name: String,
    /// 节点位置
    pub translation: Vec3,
    /// 节点旋转
    pub rotation: Quat,
    /// 节点缩放
    pub scale: Vec3,
    /// 关联的网格索引
    pub mesh: Option<usize>,
    /// 关联的材质索引
    pub material: Option<usize>,
    /// 子节点索引
    pub children: Vec<usize>,
}

/// GLTF 加载选项
#[derive(Debug, Clone, Default)]
pub struct GltfLoadOptions {
    /// 是否计算法线
    pub compute_normals: bool,
    /// 是否计算切线
    pub compute_tangents: bool,
    /// 是否翻转 V 坐标
    pub invert_v: bool,
}

impl GltfLoadOptions {
    /// 默认选项
    pub fn default_for_engine() -> Self {
        Self {
            compute_normals: true,
            compute_tangents: false,
            invert_v: false,
        }
    }
}

impl GltfModel {
    /// 从文件加载
    pub fn from_file(path: &str) -> Result<Self> {
        let options = GltfLoadOptions::default_for_engine();
        Self::from_file_with_options(path, &options)
    }

    /// 从文件加载（带选项）
    pub fn from_file_with_options(path: &str, options: &GltfLoadOptions) -> Result<Self> {
        let (doc, buffers, _images) = gltf::import(path)
            .with_context(|| format!("Failed to import GLTF file: {}", path))?;

        Self::from_document(&doc, &buffers, options)
    }

    /// 从字节数组加载
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let options = GltfLoadOptions::default_for_engine();
        Self::from_bytes_with_options(bytes, &options)
    }

    /// 从字节数组加载（带选项）
    pub fn from_bytes_with_options(bytes: &[u8], options: &GltfLoadOptions) -> Result<Self> {
        let (doc, buffers, _images) = gltf::import_slice(bytes)
            .context("Failed to import GLTF slice")?;
        Self::from_document(&doc, &buffers, options)
    }

    /// 从 GLTF document 解析
    fn from_document(
        doc: &gltf::Document,
        buffers: &[gltf::buffer::Data],
        options: &GltfLoadOptions,
    ) -> Result<Self> {
        // 1. 加载所有材质
        let mut materials = Vec::new();
        for mat in doc.materials() {
            materials.push(convert_material(&mat));
        }

        // 2. 加载所有网格
        let mut meshes = Vec::new();
        for mesh in doc.meshes() {
            meshes.push(convert_mesh(&mesh, buffers, options)?);
        }

        // 3. 加载所有节点
        let mut nodes = Vec::new();
        for node in doc.nodes() {
            nodes.push(convert_node(&node));
        }

        // 4. 解析场景根节点
        let mut scene_roots = Vec::new();
        if let Some(scene) = doc.scenes().next() {
            for node in scene.nodes() {
                scene_roots.push(node.index());
            }
        } else {
            // 没有场景，根节点使用所有顶层节点
            for node in doc.nodes() {
                scene_roots.push(node.index());
            }
        }

        // 5. 计算全局 AABB
        let aabb = compute_scene_aabb(&meshes, &nodes);

        Ok(Self {
            meshes,
            materials,
            nodes,
            scene_roots,
            aabb,
        })
    }
}

/// 转换 GLTF 节点
fn convert_node(node: &gltf::Node) -> GltfNode {
    let (translation, rotation, scale) = node.transform().decomposed();
    let children = node.children().map(|c| c.index()).collect();
    GltfNode {
        name: node.name().unwrap_or("").to_string(),
        translation: Vec3::new(translation[0], translation[1], translation[2]),
        rotation: Quat::new(rotation[0], rotation[1], rotation[2], rotation[3]),
        scale: Vec3::new(scale[0], scale[1], scale[2]),
        mesh: node.mesh().map(|m| m.index()),
        material: node.mesh().and_then(|m| m.primitives().next().and_then(|p| p.material().index())),
        children,
    }
}

/// 转换 GLTF 材质
fn convert_material(mat: &gltf::Material) -> Material3D {
    let pbr = mat.pbr_metallic_roughness();
    let base_color = pbr.base_color_factor();

    let mut material = Material3D::new();
    material.set_name(mat.name().unwrap_or("default"));
    material.set_base_color(Vec4::new(
        base_color[0],
        base_color[1],
        base_color[2],
        base_color[3],
    ));
    material.set_metallic(pbr.metallic_factor());
    material.set_roughness(pbr.roughness_factor());

    let emissive = mat.emissive_factor();
    material.set_emissive(Vec4::new(emissive[0], emissive[1], emissive[2], 1.0));

    if mat.double_sided() {
        material.set_double_sided(true);
    }

    material
}

/// 转换 GLTF 网格
fn convert_mesh(
    mesh: &gltf::Mesh,
    buffers: &[gltf::buffer::Data],
    options: &GltfLoadOptions,
) -> Result<Mesh3D> {
    let mut all_vertices = Vec::new();
    let mut all_indices = Vec::new();
    let mut primitives = Vec::new();

    for primitive in mesh.primitives() {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let positions = reader
            .read_positions()
            .ok_or_else(|| anyhow!("Primitive has no POSITION attribute"))?
            .collect::<Vec<_>>();

        if positions.is_empty() {
            continue;
        }

        let normals: Vec<[f32; 3]> = if let Some(n) = reader.read_normals() {
            n.collect()
        } else {
            vec![[0.0, 1.0, 0.0]; positions.len()]
        };

        let tex_coords: Vec<[f32; 2]> = if let Some(t) = reader.read_tex_coords(0) {
            t.into_f32().collect()
        } else {
            vec![[0.0, 0.0]; positions.len()]
        };

        let tangents: Vec<[f32; 4]> = if let Some(t) = reader.read_tangents() {
            t.collect()
        } else {
            vec![[1.0, 0.0, 0.0, 1.0]; positions.len()]
        };

        let colors: Vec<[f32; 4]> = if let Some(c) = reader.read_colors(0) {
            c.into_rgba_f32().collect()
        } else {
            vec![[1.0, 1.0, 1.0, 1.0]; positions.len()]
        };

        // 收集当前 primitive 的索引
        let mut primitive_indices: Vec<u32> = Vec::new();
        if let Some(indices_reader) = reader.read_indices() {
            for idx in indices_reader.into_u32() {
                primitive_indices.push(idx);
            }
        } else {
            // 没有索引则按顺序生成
            for i in 0..positions.len() as u32 {
                primitive_indices.push(i);
            }
        }

        let base_index = all_vertices.len() as u32;
        let base_vertex = all_vertices.len();

        // 构造顶点
        for i in 0..positions.len() {
            let pos = positions[i];
            let n = normals.get(i).copied().unwrap_or([0.0, 1.0, 0.0]);
            let uv = tex_coords.get(i).copied().unwrap_or([0.0, 0.0]);
            let tan = tangents.get(i).copied().unwrap_or([1.0, 0.0, 0.0, 1.0]);
            let _col = colors.get(i).copied().unwrap_or([1.0, 1.0, 1.0, 1.0]);

            let mut v = Vertex3D::new(
                Vec3::new(pos[0], pos[1], pos[2]),
                Vec3::new(n[0], n[1], n[2]),
                Vec2::new(uv[0], uv[1]),
            );
            v = v.with_tangent(Vec4::new(tan[0], tan[1], tan[2], tan[3]));
            all_vertices.push(v);
        }

        // 重新索引
        for &idx in &primitive_indices {
            all_indices.push(base_index + idx);
        }

        let material_index = primitive.material().index().unwrap_or(0) as u32;
        primitives.push(Primitive::new(
            base_vertex as u32,
            primitive_indices.len() as u32,
            material_index,
        ));
    }

    if all_vertices.is_empty() {
        return Err(anyhow!("Mesh has no vertices"));
    }

    let mut mesh3d = Mesh3D::from_vertices(all_vertices, all_indices);

    // 替换 primitive 列表
    mesh3d.set_primitives(primitives);

    if options.compute_normals && !mesh3d.has_normals() {
        mesh3d.compute_normals();
    }
    if options.compute_tangents {
        mesh3d.compute_tangents();
    }
    if options.invert_v {
        mesh3d.invert_v();
    }

    Ok(mesh3d)
}

/// 计算场景全局 AABB
fn compute_scene_aabb(meshes: &[Mesh3D], nodes: &[GltfNode]) -> crate::geometry::AABB {
    use crate::geometry::AABB;
    let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
    let mut has_geometry = false;

    for (i, node) in nodes.iter().enumerate() {
        if let Some(mesh_idx) = node.mesh {
            if let Some(mesh) = meshes.get(mesh_idx) {
                let local = mesh.aabb();
                // 简单应用节点变换（平移）
                let world_min = local.min() + node.translation;
                let world_max = local.max() + node.translation;
                let _ = i;
                min = min.min(world_min);
                max = max.max(world_max);
                has_geometry = true;
            }
        }
    }

    if !has_geometry {
        return AABB::new(Vec3::ZERO, Vec3::ZERO);
    }

    AABB::new(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gltf_options_default() {
        let opts = GltfLoadOptions::default_for_engine();
        assert!(opts.compute_normals);
    }
}
