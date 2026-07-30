//! Vertex structures and layouts

use engine_math::{Vec2, Vec3};

/// Standard vertex format for 3D meshes
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
}

impl Vertex {
    #[inline]
    pub fn new(position: Vec3, normal: Vec3, texcoord: Vec2) -> Self {
        Self {
            position,
            normal,
            texcoord,
        }
    }

    #[inline]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    #[inline]
    pub fn texcoord(&self) -> Vec2 {
        self.texcoord
    }
}

/// Vertex layout constants for GPU binding
pub struct VertexLayout;

impl VertexLayout {
    /// Position offset in bytes
    pub const POS_OFFSET: usize = 0;
    /// Normal offset in bytes
    pub const NORMAL_OFFSET: usize = 12;
    /// Texcoord offset in bytes
    pub const UV_OFFSET: usize = 24;
    /// Total stride in bytes
    pub const STRIDE: usize = 32;

    /// Layout: POS3F_NORMAL3F_UV2F
    pub const POS3F_NORMAL3F_UV2F: [usize; 3] =
        [Self::POS_OFFSET, Self::NORMAL_OFFSET, Self::UV_OFFSET];
}

/// Extended vertex with tangent and color
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct VertexExt {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
    pub tangent: Vec3,
    pub color: [f32; 4],
}

#[allow(dead_code)]
impl VertexExt {
    #[inline]
    pub fn new(position: Vec3, normal: Vec3, texcoord: Vec2) -> Self {
        Self {
            position,
            normal,
            texcoord,
            tangent: Vec3::ZERO,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    #[inline]
    pub fn with_tangent(mut self, tangent: Vec3) -> Self {
        self.tangent = tangent;
        self
    }

    #[inline]
    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}

/// Extended vertex layout
#[allow(dead_code)]
pub struct VertexLayoutExt;

#[allow(dead_code)]
impl VertexLayoutExt {
    pub const POS_OFFSET: usize = 0;
    pub const NORMAL_OFFSET: usize = 12;
    pub const UV_OFFSET: usize = 24;
    pub const TANGENT_OFFSET: usize = 32;
    pub const COLOR_OFFSET: usize = 44;
    pub const STRIDE: usize = 60;
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_math::{Vec2, Vec3};

    #[test]
    fn test_vertex_new() {
        let v = Vertex::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(0.5, 0.75),
        );
        assert_eq!(v.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(v.normal, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(v.texcoord, Vec2::new(0.5, 0.75));
    }

    #[test]
    fn test_vertex_default() {
        let v = Vertex::default();
        assert_eq!(v.position, Vec3::ZERO);
        assert_eq!(v.normal, Vec3::ZERO);
        assert_eq!(v.texcoord, Vec2::ZERO);
    }

    #[test]
    fn test_vertex_position_accessor() {
        let v = Vertex::new(Vec3::ONE, Vec3::Y, Vec2::ONE);
        assert_eq!(v.position(), Vec3::ONE);
    }

    #[test]
    fn test_vertex_normal_accessor() {
        let v = Vertex::new(Vec3::ONE, Vec3::Y, Vec2::ONE);
        assert_eq!(v.normal(), Vec3::Y);
    }

    #[test]
    fn test_vertex_texcoord_accessor() {
        let v = Vertex::new(Vec3::ONE, Vec3::Y, Vec2::ONE);
        assert_eq!(v.texcoord(), Vec2::ONE);
    }

    #[test]
    fn test_vertex_layout_stride() {
        assert_eq!(VertexLayout::STRIDE, 32);
    }

    #[test]
    fn test_vertex_layout_pos_offset() {
        assert_eq!(VertexLayout::POS_OFFSET, 0);
    }

    #[test]
    fn test_vertex_layout_normal_offset() {
        assert_eq!(VertexLayout::NORMAL_OFFSET, 12);
    }

    #[test]
    fn test_vertex_layout_uv_offset() {
        assert_eq!(VertexLayout::UV_OFFSET, 24);
    }

    #[test]
    fn test_vertex_ext_new() {
        let v = VertexExt::new(Vec3::ONE, Vec3::Y, Vec2::ONE);
        assert_eq!(v.position, Vec3::ONE);
        assert_eq!(v.tangent, Vec3::ZERO);
    }

    #[test]
    fn test_vertex_ext_with_tangent() {
        let v = VertexExt::new(Vec3::ONE, Vec3::Y, Vec2::ONE).with_tangent(Vec3::X);
        assert_eq!(v.tangent, Vec3::X);
    }

    #[test]
    fn test_vertex_ext_with_color() {
        let v = VertexExt::new(Vec3::ONE, Vec3::Y, Vec2::ONE).with_color([1.0, 0.5, 0.2, 1.0]);
        assert_eq!(v.color, [1.0, 0.5, 0.2, 1.0]);
    }

    #[test]
    fn test_vertex_ext_default() {
        let v = VertexExt::default();
        assert_eq!(v.position, Vec3::ZERO);
        assert_eq!(v.tangent, Vec3::ZERO);
        assert_eq!(v.color, [0.0, 0.0, 0.0, 0.0]);
    }
}
