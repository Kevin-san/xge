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
        Self { position, normal, texcoord }
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
    pub const POS3F_NORMAL3F_UV2F: [usize; 3] = [
        Self::POS_OFFSET,
        Self::NORMAL_OFFSET,
        Self::UV_OFFSET,
    ];
}

/// Extended vertex with tangent and color
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct VertexExt {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
    pub tangent: Vec3,
    pub color: [f32; 4],
}

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
pub struct VertexLayoutExt;

impl VertexLayoutExt {
    pub const POS_OFFSET: usize = 0;
    pub const NORMAL_OFFSET: usize = 12;
    pub const UV_OFFSET: usize = 24;
    pub const TANGENT_OFFSET: usize = 32;
    pub const COLOR_OFFSET: usize = 44;
    pub const STRIDE: usize = 60;
}