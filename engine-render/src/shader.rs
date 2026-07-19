//! Shader 模块 - 着色器与渲染管线抽象
//!
//! 提供 Shader、Pipeline、Buffer、BindGroup 等底层渲染抽象。

#![allow(dead_code)]
#![allow(unused_imports)]

use engine_math::Vec2;

/// 着色器阶段
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ShaderStage {
    /// 顶点着色器
    Vertex,
    /// 片段着色器
    #[default]
    Fragment,
    /// 计算着色器
    Compute,
}

/// 着色器源码
#[derive(Clone, Debug)]
pub struct Shader {
    /// 阶段
    stage: ShaderStage,
    /// 源码
    source: String,
    /// 编译后的 GL 着色器对象
    #[cfg(feature = "gl")]
    gl_shader: Option<u32>,
}

#[cfg(feature = "gl")]
unsafe impl Send for Shader {}

/// 着色器模块（包含多个阶段）
#[derive(Debug)]
pub struct ShaderModule {
    /// 顶点着色器
    vertex: Option<Shader>,
    /// 片段着色器
    fragment: Option<Shader>,
    /// 计算着色器
    compute: Option<Shader>,
    /// 程序对象
    #[cfg(feature = "gl")]
    gl_program: Option<u32>,
    /// Uniform 位置缓存
    uniform_locations: parking_lot::RwLock<std::collections::HashMap<String, i32>>,
}

impl ShaderModule {
    /// 从源码创建着色器模块
    pub fn from_source(vertex_src: &str, fragment_src: &str) -> anyhow::Result<Self> {
        Ok(Self {
            vertex: Some(Shader {
                stage: ShaderStage::Vertex,
                source: vertex_src.to_string(),
                #[cfg(feature = "gl")]
                gl_shader: None,
            }),
            fragment: Some(Shader {
                stage: ShaderStage::Fragment,
                source: fragment_src.to_string(),
                #[cfg(feature = "gl")]
                gl_shader: None,
            }),
            compute: None,
            #[cfg(feature = "gl")]
            gl_program: None,
            uniform_locations: parking_lot::RwLock::new(std::collections::HashMap::new()),
        })
    }
}

/// 顶点属性类型
#[derive(Clone, Debug)]
pub enum VertexAttrType {
    F32,
    Vec2,
    Vec3,
    Vec4,
}

/// 顶点属性
#[derive(Clone, Debug)]
pub struct VertexAttr {
    /// 属性名称
    pub name: String,
    /// 大小（分量数）
    pub size: u32,
    /// 类型
    pub attr_type: VertexAttrType,
    /// 字节偏移
    pub offset: usize,
}

impl VertexAttrType {
    /// 获取大小
    pub fn size(&self) -> u32 {
        match self {
            VertexAttrType::F32 => 4,
            VertexAttrType::Vec2 => 8,
            VertexAttrType::Vec3 => 12,
            VertexAttrType::Vec4 => 16,
        }
    }
}

/// 顶点布局
#[derive(Clone, Debug)]
pub struct VertexLayout {
    attrs: Vec<VertexAttr>,
    stride: usize,
}

impl VertexLayout {
    /// 创建新的顶点布局
    pub fn new() -> Self {
        Self {
            attrs: Vec::new(),
            stride: 0,
        }
    }

    /// 添加 Vec2 属性
    pub fn push_vec2(&mut self, name: &str) {
        let offset = self.stride;
        self.stride += 8;
        self.attrs.push(VertexAttr {
            name: name.to_string(),
            size: 2,
            attr_type: VertexAttrType::Vec2,
            offset,
        });
    }

    /// 添加 Vec4 属性
    pub fn push_vec4(&mut self, name: &str) {
        let offset = self.stride;
        self.stride += 16;
        self.attrs.push(VertexAttr {
            name: name.to_string(),
            size: 4,
            attr_type: VertexAttrType::Vec4,
            offset,
        });
    }

    /// 添加顶点属性（通用版本）
    pub fn push(&mut self, name: &str, format: super::buffer::VertexFormat) {
        let offset = self.stride;
        self.stride += format.size();
        let (size, attr_type) = match format {
            super::buffer::VertexFormat::Float2 => (2, VertexAttrType::Vec2),
            super::buffer::VertexFormat::Float3 => (3, VertexAttrType::Vec3),
            super::buffer::VertexFormat::Float4 => (4, VertexAttrType::Vec4),
            super::buffer::VertexFormat::Byte4 => (4, VertexAttrType::Vec4),
        };
        self.attrs.push(VertexAttr {
            name: name.to_string(),
            size,
            attr_type,
            offset,
        });
    }

    /// 获取步长
    pub fn stride(&self) -> usize {
        self.stride
    }

    /// 获取属性列表
    pub fn attributes(&self) -> &[VertexAttr] {
        &self.attrs
    }
}

impl Default for VertexLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// 2D 顶点
#[derive(Clone, Debug)]
pub struct Vertex2D {
    /// 位置
    pub position: Vec2,
    /// UV 坐标
    pub uv: Vec2,
}

/// 2D 网格
#[derive(Clone, Debug)]
pub struct Mesh2D {
    /// 顶点
    vertices: Vec<Vertex2D>,
    /// 索引
    indices: Vec<u32>,
}

impl Mesh2D {
    /// 创建新的网格
    pub fn new(vertices: Vec<Vertex2D>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    /// 获取顶点列表
    pub fn vertices(&self) -> &[Vertex2D] {
        &self.vertices
    }

    /// 获取索引列表
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// 创建四边形
    pub fn quad(width: f32, height: f32, _color: super::Color) -> Self {
        let hw = width / 2.0;
        let hh = height / 2.0;

        let vertices = vec![
            Vertex2D {
                position: Vec2::new(-hw, -hh),
                uv: Vec2::new(0.0, 0.0),
            },
            Vertex2D {
                position: Vec2::new(hw, -hh),
                uv: Vec2::new(1.0, 0.0),
            },
            Vertex2D {
                position: Vec2::new(hw, hh),
                uv: Vec2::new(1.0, 1.0),
            },
            Vertex2D {
                position: Vec2::new(-hw, hh),
                uv: Vec2::new(0.0, 1.0),
            },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Self { vertices, indices }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::VertexFormat;
    use crate::Color;

    // ===== Vertex2D 测试 =====
    #[test]
    fn test_vertex2d_new() {
        let v = Vertex2D {
            position: Vec2::new(1.0, 2.0),
            uv: Vec2::new(0.5, 0.5),
        };
        assert!((v.position.x - 1.0).abs() < 0.001);
        assert!((v.position.y - 2.0).abs() < 0.001);
        assert!((v.uv.x - 0.5).abs() < 0.001);
    }

    // ===== Mesh2D 测试 =====
    #[test]
    fn test_mesh2d_new() {
        let vertices = vec![
            Vertex2D { position: Vec2::new(0.0, 0.0), uv: Vec2::ZERO },
            Vertex2D { position: Vec2::new(1.0, 0.0), uv: Vec2::new(1.0, 0.0) },
            Vertex2D { position: Vec2::new(1.0, 1.0), uv: Vec2::ONE },
        ];
        let mesh = Mesh2D::new(vertices, vec![0, 1, 2]);
        assert_eq!(mesh.vertices().len(), 3);
        assert_eq!(mesh.indices().len(), 3);
    }

    #[test]
    fn test_mesh2d_quad() {
        let mesh = Mesh2D::quad(100.0, 50.0, Color::WHITE);
        assert_eq!(mesh.vertices().len(), 4);
        assert_eq!(mesh.indices().len(), 6);
        // Check first vertex position (bottom-left of quad centered at origin)
        assert!((mesh.vertices()[0].position.x + 50.0).abs() < 0.001);
        assert!((mesh.vertices()[0].position.y + 25.0).abs() < 0.001);
    }

    // ===== VertexLayout 测试 =====
    #[test]
    fn test_vertex_layout_new() {
        let layout = VertexLayout::new();
        assert_eq!(layout.stride(), 0);
        assert!(layout.attributes().is_empty());
    }

    #[test]
    fn test_vertex_layout_push_vec2() {
        let mut layout = VertexLayout::new();
        layout.push_vec2("position");
        assert_eq!(layout.stride(), 8);
        assert_eq!(layout.attributes().len(), 1);
        assert_eq!(layout.attributes()[0].name, "position");
        assert_eq!(layout.attributes()[0].offset, 0);
    }

    #[test]
    fn test_vertex_layout_push_vec4() {
        let mut layout = VertexLayout::new();
        layout.push_vec4("color");
        assert_eq!(layout.stride(), 16);
        assert_eq!(layout.attributes()[0].name, "color");
        assert_eq!(layout.attributes()[0].offset, 0);
    }

    #[test]
    fn test_vertex_layout_push_generic() {
        let mut layout = VertexLayout::new();
        layout.push("position", VertexFormat::Float2);
        layout.push("uv", VertexFormat::Float2);
        layout.push("color", VertexFormat::Float4);
        assert_eq!(layout.stride(), 32); // 8 + 8 + 16
        assert_eq!(layout.attributes().len(), 3);
        assert_eq!(layout.attributes()[0].offset, 0);
        assert_eq!(layout.attributes()[1].offset, 8);
        assert_eq!(layout.attributes()[2].offset, 16);
    }

    #[test]
    fn test_vertex_layout_push_mixed() {
        let mut layout = VertexLayout::new();
        layout.push_vec2("position");
        layout.push("color", VertexFormat::Float4);
        assert_eq!(layout.stride(), 24); // 8 + 16
        assert_eq!(layout.attributes().len(), 2);
        assert_eq!(layout.attributes()[0].offset, 0);
        assert_eq!(layout.attributes()[1].offset, 8);
    }

    #[test]
    fn test_vertex_layout_default() {
        let layout = VertexLayout::default();
        assert_eq!(layout.stride(), 0);
    }

    // ===== VertexAttr 测试 =====
    #[test]
    fn test_vertex_attr_offset_tracking() {
        let mut layout = VertexLayout::new();
        layout.push_vec2("a");
        layout.push_vec2("b");
        layout.push_vec4("c");
        let attrs = layout.attributes();
        assert_eq!(attrs[0].offset, 0);
        assert_eq!(attrs[1].offset, 8);
        assert_eq!(attrs[2].offset, 16);
    }
}
