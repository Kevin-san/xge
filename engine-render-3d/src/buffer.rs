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

#[cfg(test)]
mod tests {
    use super::*;
    use engine_math::{Vec2, Vec3};

    #[test]
    fn test_index_format_u16_size() {
        assert_eq!(IndexFormat::U16.byte_size(), 2);
    }

    #[test]
    fn test_index_format_u32_size() {
        assert_eq!(IndexFormat::U32.byte_size(), 4);
    }

    #[test]
    fn test_vertex_buffer_new() {
        let vertices = vec![
            Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO),
            Vertex::new(Vec3::X, Vec3::Y, Vec2::X),
        ];
        let vb = VertexBuffer::new(vertices);
        assert_eq!(vb.len(), 2);
    }

    #[test]
    fn test_vertex_buffer_is_empty() {
        let vb = VertexBuffer::new(Vec::new());
        assert!(vb.is_empty());
    }

    #[test]
    fn test_vertex_buffer_size_bytes() {
        let vertices = vec![
            Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO),
            Vertex::new(Vec3::X, Vec3::Y, Vec2::X),
        ];
        let vb = VertexBuffer::new(vertices);
        // stride = 32 bytes, 2 vertices
        assert_eq!(vb.size_bytes(), 64);
    }

    #[test]
    fn test_vertex_buffer_vertices_access() {
        let vertices = vec![Vertex::new(Vec3::ONE, Vec3::Y, Vec2::ONE)];
        let vb = VertexBuffer::new(vertices);
        let verts = vb.vertices();
        assert_eq!(verts[0].position, Vec3::ONE);
    }

    #[test]
    fn test_index_buffer_new_u32() {
        let indices = vec![0u32, 1, 2, 3, 4, 5];
        let ib = IndexBuffer::new(indices);
        assert_eq!(ib.index_count(), 6);
        assert_eq!(ib.format(), IndexFormat::U32);
    }

    #[test]
    fn test_index_buffer_new_u16() {
        let indices = vec![0u16, 1, 2];
        let ib = IndexBuffer::new_u16(indices);
        assert_eq!(ib.index_count(), 3);
        assert_eq!(ib.format(), IndexFormat::U16);
    }

    #[test]
    fn test_index_buffer_u16_size_bytes() {
        let indices = vec![0u16, 1, 2, 3, 4, 5];
        let ib = IndexBuffer::new_u16(indices);
        // 6 indices * 2 bytes
        assert_eq!(ib.size_bytes(), 12);
    }

    #[test]
    fn test_index_buffer_u32_size_bytes() {
        let indices = vec![0u32, 1, 2, 3, 4, 5];
        let ib = IndexBuffer::new(indices);
        // 6 indices * 4 bytes
        assert_eq!(ib.size_bytes(), 24);
    }

    #[test]
    fn test_index_buffer_indices_access() {
        let indices = vec![10u32, 20, 30];
        let ib = IndexBuffer::new(indices);
        assert_eq!(ib.indices(), &[10, 20, 30]);
    }

    #[test]
    fn test_index_buffer_as_bytes_u32() {
        let indices = vec![1u32];
        let ib = IndexBuffer::new(indices);
        let bytes = ib.as_bytes();
        assert_eq!(bytes.len(), 4);
    }

    #[test]
    fn test_index_buffer_as_bytes_u16() {
        let indices = vec![1u16, 2u16];
        let ib = IndexBuffer::new_u16(indices);
        let bytes = ib.as_bytes();
        assert_eq!(bytes.len(), 4);
    }

    #[test]
    fn test_vertex_buffer_empty_size_bytes() {
        let vb = VertexBuffer::new(Vec::new());
        assert_eq!(vb.size_bytes(), 0);
    }

    #[test]
    fn test_index_buffer_empty_u32() {
        let ib = IndexBuffer::new(Vec::new());
        assert_eq!(ib.index_count(), 0);
        assert_eq!(ib.size_bytes(), 0);
    }
}
