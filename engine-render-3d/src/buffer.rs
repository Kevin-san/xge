//! GPU buffer types

use crate::vertex::Vertex;
use alloc::vec::Vec;

/// Index format for index buffers
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IndexFormat {
    U16,
    U32,
}

impl IndexFormat {
    #[inline]
    pub fn byte_size(&self) -> usize {
        match self {
            IndexFormat::U16 => 2,
            IndexFormat::U32 => 4,
        }
    }
}

/// Vertex buffer (CPU-side representation, GPU upload pending)
#[derive(Clone, Debug)]
pub struct VertexBuffer {
    vertices: Vec<Vertex>,
}

impl VertexBuffer {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }

    #[inline]
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    #[inline]
    pub fn size_bytes(&self) -> usize {
        self.vertices.len() * crate::vertex::VertexLayout::STRIDE
    }

    /// Get raw bytes for GPU upload
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.vertices.as_ptr() as *const u8, self.size_bytes())
        }
    }
}

/// Index buffer (CPU-side representation)
#[derive(Clone, Debug)]
pub struct IndexBuffer {
    indices: Vec<u32>,
    format: IndexFormat,
}

impl IndexBuffer {
    pub fn new(indices: Vec<u32>) -> Self {
        Self {
            indices,
            format: IndexFormat::U32,
        }
    }

    pub fn new_u16(indices: Vec<u16>) -> Self {
        Self {
            indices: indices.iter().map(|i| *i as u32).collect(),
            format: IndexFormat::U16,
        }
    }

    #[inline]
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub fn format(&self) -> IndexFormat {
        self.format
    }

    #[inline]
    pub fn size_bytes(&self) -> usize {
        self.indices.len() * self.format.byte_size()
    }

    /// Get raw bytes for GPU upload
    pub fn as_bytes(&self) -> Vec<u8> {
        match self.format {
            IndexFormat::U16 => self
                .indices
                .iter()
                .flat_map(|i| (*i as u16).to_ne_bytes())
                .collect(),
            IndexFormat::U32 => self.indices.iter().flat_map(|i| i.to_ne_bytes()).collect(),
        }
    }
}
