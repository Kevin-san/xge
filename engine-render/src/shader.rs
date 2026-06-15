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
    name: String,
    /// 大小（分量数）
    size: u32,
    /// 类型
    attr_type: VertexAttrType,
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
        self.stride += 8;
        self.attrs.push(VertexAttr {
            name: name.to_string(),
            size: 2,
            attr_type: VertexAttrType::Vec2,
        });
    }

    /// 添加 Vec4 属性
    pub fn push_vec4(&mut self, name: &str) {
        self.stride += 16;
        self.attrs.push(VertexAttr {
            name: name.to_string(),
            size: 4,
            attr_type: VertexAttrType::Vec4,
        });
    }

    /// 获取步长
    pub fn stride(&self) -> usize {
        self.stride
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
