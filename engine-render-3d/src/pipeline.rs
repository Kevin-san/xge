//! 3D render pipeline
//!
//! Provides render pipeline abstraction with multi-backend support (OpenGL/wgpu),
//! render passes, draw call batching, frustum culling integration, and statistics tracking.

use crate::camera::Camera3D;
use crate::frustum::Frustum;
use crate::scene::Scene3D;
use alloc::string::String;
use alloc::vec::Vec;
use engine_math::{Mat4, Vec3};

/// Available render backends
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RenderBackend {
    OpenGL,
    Wgpu,
    Vulkan,
    Metal,
}

/// Types of render passes
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RenderPassType {
    Shadow,
    Geometry,
    Forward,
    Deferred,
    PostProcess,
    UI,
}

/// GPU render state
#[derive(Clone, Debug)]
pub struct RenderState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub blend: bool,
    pub cull_face: bool,
    pub wireframe: bool,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            depth_test: true,
            depth_write: true,
            blend: false,
            cull_face: true,
            wireframe: false,
        }
    }

    pub fn with_depth_test(mut self, enabled: bool) -> Self {
        self.depth_test = enabled;
        self
    }

    pub fn with_depth_write(mut self, enabled: bool) -> Self {
        self.depth_write = enabled;
        self
    }

    pub fn with_blend(mut self, enabled: bool) -> Self {
        self.blend = enabled;
        self
    }

    pub fn with_cull_face(mut self, enabled: bool) -> Self {
        self.cull_face = enabled;
        self
    }

    pub fn with_wireframe(mut self, enabled: bool) -> Self {
        self.wireframe = enabled;
        self
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self::new()
    }
}

/// Color format for render targets
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ColorFormat {
    RGBA8,
    RGBA16F,
    RGBA32F,
    R8,
    R16F,
    R32F,
}

impl ColorFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ColorFormat::RGBA8 => 4,
            ColorFormat::RGBA16F => 8,
            ColorFormat::RGBA32F => 16,
            ColorFormat::R8 => 1,
            ColorFormat::R16F => 2,
            ColorFormat::R32F => 4,
        }
    }
}

/// Render target (framebuffer)
#[derive(Clone, Debug)]
pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
    pub has_depth: bool,
    pub has_stencil: bool,
    pub color_format: ColorFormat,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            has_depth: false,
            has_stencil: false,
            color_format: ColorFormat::RGBA8,
        }
    }

    pub fn with_depth(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            has_depth: true,
            has_stencil: false,
            color_format: ColorFormat::RGBA8,
        }
    }
}

/// A single draw call
#[derive(Clone, Debug)]
pub struct DrawCall {
    pub mesh_handle: u32,
    pub material_index: Option<usize>,
    pub world_matrix: Mat4,
    pub index_count: usize,
    pub index_offset: usize,
    pub vertex_offset: usize,
    pub render_state: RenderState,
}

impl DrawCall {
    pub fn new(mesh_handle: u32, index_count: usize) -> Self {
        Self {
            mesh_handle,
            material_index: None,
            world_matrix: Mat4::IDENTITY,
            index_count,
            index_offset: 0,
            vertex_offset: 0,
            render_state: RenderState::new(),
        }
    }
}

/// Collects and sorts draw calls
#[derive(Clone, Debug)]
pub struct RenderQueue {
    pub draw_calls: Vec<DrawCall>,
    pub sorted: bool,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            draw_calls: Vec::new(),
            sorted: false,
        }
    }

    pub fn push(&mut self, call: DrawCall) {
        self.draw_calls.push(call);
        self.sorted = false;
    }

    /// Sort by material_index for batching
    pub fn sort_by_material(&mut self) {
        self.draw_calls
            .sort_by(|a, b| a.material_index.cmp(&b.material_index));
        self.sorted = true;
    }

    /// Sort by distance (back to front for transparency)
    pub fn sort_by_depth(&mut self, camera_pos: Vec3) {
        self.draw_calls.sort_by(|a, b| {
            let da = draw_call_distance(a, camera_pos);
            let db = draw_call_distance(b, camera_pos);
            db.partial_cmp(&da).unwrap_or(core::cmp::Ordering::Equal)
        });
        self.sorted = true;
    }

    pub fn clear(&mut self) {
        self.draw_calls.clear();
        self.sorted = false;
    }

    pub fn len(&self) -> usize {
        self.draw_calls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.draw_calls.is_empty()
    }

    pub fn draw_calls(&self) -> &[DrawCall] {
        &self.draw_calls
    }
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract world-space position from a draw call's world matrix translation
fn draw_call_distance(call: &DrawCall, camera_pos: Vec3) -> f32 {
    let pos = Vec3::new(
        call.world_matrix.cols[3][0],
        call.world_matrix.cols[3][1],
        call.world_matrix.cols[3][2],
    );
    camera_pos.distance(pos)
}

/// Pipeline statistics
#[derive(Clone, Debug)]
pub struct RenderPipelineStats {
    pub draw_call_count: usize,
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub culled_objects: usize,
    pub render_time_ms: f32,
    pub pass_count: usize,
}

impl RenderPipelineStats {
    pub fn new() -> Self {
        Self {
            draw_call_count: 0,
            vertex_count: 0,
            triangle_count: 0,
            culled_objects: 0,
            render_time_ms: 0.0,
            pass_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.draw_call_count = 0;
        self.vertex_count = 0;
        self.triangle_count = 0;
        self.culled_objects = 0;
        self.render_time_ms = 0.0;
        self.pass_count = 0;
    }
}

impl Default for RenderPipelineStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Main render pipeline
pub struct RenderPipeline3D {
    pub backend: RenderBackend,
    pub render_queue: RenderQueue,
    pub stats: RenderPipelineStats,
    pub current_target: Option<RenderTarget>,
    pub frustum_culling_enabled: bool,
    pub current_pass: Option<RenderPassType>,
}

impl RenderPipeline3D {
    pub fn new(backend: RenderBackend) -> Self {
        Self {
            backend,
            render_queue: RenderQueue::new(),
            stats: RenderPipelineStats::new(),
            current_target: None,
            frustum_culling_enabled: true,
            current_pass: None,
        }
    }

    pub fn set_render_target(&mut self, target: RenderTarget) {
        self.current_target = Some(target);
    }

    /// Begin a render pass
    pub fn begin_pass(&mut self, pass_type: RenderPassType) {
        self.current_pass = Some(pass_type);
        self.stats.pass_count += 1;
    }

    /// End current pass
    pub fn end_pass(&mut self) {
        self.current_pass = None;
    }

    /// Add draw call to queue
    pub fn submit_draw_call(&mut self, call: DrawCall) {
        self.render_queue.push(call);
    }

    /// Submit all visible entities from scene as draw calls, using frustum culling
    pub fn submit_scene(&mut self, scene: &Scene3D, camera: &Camera3D) -> Result<(), String> {
        let frustum = if self.frustum_culling_enabled {
            Some(Frustum::from_view_projection(camera.view_projection()))
        } else {
            None
        };

        for entity in scene.visible_entities() {
            if let Some(ref frustum) = frustum {
                if !frustum.contains_aabb(entity.aabb) {
                    self.stats.culled_objects += 1;
                    continue;
                }
            }

            let mut call = DrawCall::new(entity.mesh.index(), 0);
            call.material_index = entity.material_index;
            call.world_matrix = entity.world_matrix;
            self.render_queue.push(call);
        }

        Ok(())
    }

    /// Process all queued draw calls (in real impl would call GPU; here just updates stats)
    pub fn flush(&mut self) {
        let queued = self.render_queue.len();
        let triangles: usize = self
            .render_queue
            .draw_calls
            .iter()
            .map(|c| c.index_count / 3)
            .sum();

        self.stats.draw_call_count += queued;
        self.stats.triangle_count += triangles;
        self.render_queue.clear();
    }

    pub fn enable_frustum_culling(&mut self, enabled: bool) {
        self.frustum_culling_enabled = enabled;
    }

    pub fn frustum_culling_enabled(&self) -> bool {
        self.frustum_culling_enabled
    }

    pub fn stats(&self) -> &RenderPipelineStats {
        &self.stats
    }

    pub fn render_queue(&self) -> &RenderQueue {
        &self.render_queue
    }

    pub fn backend(&self) -> RenderBackend {
        self.backend
    }

    pub fn current_pass(&self) -> Option<RenderPassType> {
        self.current_pass
    }
}

/// Stub OpenGL backend (no actual GL calls, just tracks state)
pub struct OpenGLBackend {
    pub program_id: u32,
    pub vao_id: u32,
    pub vbo_id: u32,
    pub ibo_id: u32,
    pub bound_textures: [u32; 32],
    pub viewport: (i32, i32, i32, i32),
    pub enabled_attributes: Vec<u32>,
    draw_call_count: u64,
}

impl OpenGLBackend {
    pub fn new() -> Self {
        Self {
            program_id: 0,
            vao_id: 0,
            vbo_id: 0,
            ibo_id: 0,
            bound_textures: [0u32; 32],
            viewport: (0, 0, 0, 0),
            enabled_attributes: Vec::new(),
            draw_call_count: 0,
        }
    }

    pub fn bind_vertex_array(&mut self, id: u32) {
        self.vao_id = id;
    }

    pub fn bind_vertex_buffer(&mut self, id: u32) {
        self.vbo_id = id;
    }

    pub fn bind_index_buffer(&mut self, id: u32) {
        self.ibo_id = id;
    }

    pub fn use_program(&mut self, id: u32) {
        self.program_id = id;
    }

    pub fn set_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.viewport = (x, y, w, h);
    }

    pub fn enable_vertex_attrib_array(&mut self, index: u32) {
        if !self.enabled_attributes.contains(&index) {
            self.enabled_attributes.push(index);
        }
    }

    pub fn disable_vertex_attrib_array(&mut self, index: u32) {
        self.enabled_attributes.retain(|&i| i != index);
    }

    pub fn active_texture(&mut self, unit: usize, texture: u32) {
        if unit < self.bound_textures.len() {
            self.bound_textures[unit] = texture;
        }
    }

    /// Record draw call
    pub fn draw_elements(&mut self, _index_count: usize, _offset: usize) {
        self.draw_call_count += 1;
    }

    pub fn draw_arrays(&mut self, _vertex_count: usize, _offset: usize) {
        self.draw_call_count += 1;
    }

    pub fn draw_call_count(&self) -> u64 {
        self.draw_call_count
    }
}

impl Default for OpenGLBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub wgpu backend (placeholder for future)
pub struct WgpuBackend {
    pub device_id: u32,
    pub queue_id: u32,
    pub surface_format: Option<ColorFormat>,
}

impl WgpuBackend {
    pub fn new() -> Self {
        Self {
            device_id: 0,
            queue_id: 0,
            surface_format: None,
        }
    }

    pub fn configure(&mut self, format: ColorFormat) {
        self.surface_format = Some(format);
    }

    pub fn is_configured(&self) -> bool {
        self.surface_format.is_some()
    }
}

impl Default for WgpuBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera3D;
    use crate::frustum::Frustum;
    use crate::geometry::AABB;
    use crate::mesh::Mesh3D;
    use crate::scene::{Node3D, Scene3D};
    use engine_math::{Mat4, Vec3};
    use engine_utils::Handle;

    // --- RenderState tests ---

    #[test]
    fn test_render_state_defaults() {
        let state = RenderState::new();
        assert!(state.depth_test);
        assert!(state.depth_write);
        assert!(!state.blend);
        assert!(state.cull_face);
        assert!(!state.wireframe);
    }

    #[test]
    fn test_render_state_default_trait() {
        let state = RenderState::default();
        assert!(state.depth_test);
        assert!(state.depth_write);
        assert!(!state.blend);
    }

    #[test]
    fn test_render_state_builder() {
        let state = RenderState::new()
            .with_depth_test(false)
            .with_depth_write(false)
            .with_blend(true)
            .with_cull_face(false)
            .with_wireframe(true);
        assert!(!state.depth_test);
        assert!(!state.depth_write);
        assert!(state.blend);
        assert!(!state.cull_face);
        assert!(state.wireframe);
    }

    #[test]
    fn test_render_state_builder_partial() {
        let state = RenderState::new().with_blend(true);
        assert!(state.depth_test);
        assert!(state.blend);
        assert!(!state.wireframe);
    }

    // --- ColorFormat tests ---

    #[test]
    fn test_color_format_bytes_per_pixel() {
        assert_eq!(ColorFormat::RGBA8.bytes_per_pixel(), 4);
        assert_eq!(ColorFormat::RGBA16F.bytes_per_pixel(), 8);
        assert_eq!(ColorFormat::RGBA32F.bytes_per_pixel(), 16);
        assert_eq!(ColorFormat::R8.bytes_per_pixel(), 1);
        assert_eq!(ColorFormat::R16F.bytes_per_pixel(), 2);
        assert_eq!(ColorFormat::R32F.bytes_per_pixel(), 4);
    }

    #[test]
    fn test_color_format_equality() {
        assert_eq!(ColorFormat::RGBA8, ColorFormat::RGBA8);
        assert_ne!(ColorFormat::RGBA8, ColorFormat::RGBA16F);
    }

    // --- RenderTarget tests ---

    #[test]
    fn test_render_target_new() {
        let target = RenderTarget::new(800, 600);
        assert_eq!(target.width, 800);
        assert_eq!(target.height, 600);
        assert!(!target.has_depth);
        assert!(!target.has_stencil);
        assert_eq!(target.color_format, ColorFormat::RGBA8);
    }

    #[test]
    fn test_render_target_with_depth() {
        let target = RenderTarget::with_depth(1920, 1080);
        assert_eq!(target.width, 1920);
        assert_eq!(target.height, 1080);
        assert!(target.has_depth);
        assert!(!target.has_stencil);
    }

    // --- DrawCall tests ---

    #[test]
    fn test_draw_call_new() {
        let call = DrawCall::new(42, 36);
        assert_eq!(call.mesh_handle, 42);
        assert_eq!(call.index_count, 36);
        assert_eq!(call.material_index, None);
        assert_eq!(call.index_offset, 0);
        assert_eq!(call.vertex_offset, 0);
        assert_eq!(call.world_matrix, Mat4::IDENTITY);
        assert!(call.render_state.depth_test);
    }

    #[test]
    fn test_draw_call_default_render_state() {
        let call = DrawCall::new(0, 0);
        assert!(call.render_state.depth_test);
        assert!(!call.render_state.blend);
    }

    // --- RenderQueue tests ---

    #[test]
    fn test_render_queue_new() {
        let queue = RenderQueue::new();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert!(!queue.sorted);
    }

    #[test]
    fn test_render_queue_push() {
        let mut queue = RenderQueue::new();
        queue.push(DrawCall::new(1, 6));
        queue.push(DrawCall::new(2, 3));
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());
        assert!(!queue.sorted);
    }

    #[test]
    fn test_render_queue_sort_by_material() {
        let mut queue = RenderQueue::new();
        let mut c1 = DrawCall::new(1, 6);
        c1.material_index = Some(2);
        let mut c2 = DrawCall::new(2, 6);
        c2.material_index = Some(0);
        let mut c3 = DrawCall::new(3, 6);
        c3.material_index = Some(1);
        let mut c4 = DrawCall::new(4, 6);
        c4.material_index = None;
        queue.push(c1);
        queue.push(c2);
        queue.push(c3);
        queue.push(c4);

        queue.sort_by_material();
        assert!(queue.sorted);
        let calls = queue.draw_calls();
        assert_eq!(calls[0].material_index, None);
        assert_eq!(calls[1].material_index, Some(0));
        assert_eq!(calls[2].material_index, Some(1));
        assert_eq!(calls[3].material_index, Some(2));
    }

    #[test]
    fn test_render_queue_sort_by_depth() {
        let mut queue = RenderQueue::new();
        let mut near = DrawCall::new(1, 3);
        near.world_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, -1.0));
        let mut far = DrawCall::new(2, 3);
        far.world_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, -10.0));
        let mut mid = DrawCall::new(3, 3);
        mid.world_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, -5.0));
        queue.push(near);
        queue.push(far);
        queue.push(mid);

        let camera_pos = Vec3::ZERO;
        queue.sort_by_depth(camera_pos);
        assert!(queue.sorted);
        let calls = queue.draw_calls();
        // Back to front: farthest first
        assert_eq!(calls[0].mesh_handle, 2);
        assert_eq!(calls[1].mesh_handle, 3);
        assert_eq!(calls[2].mesh_handle, 1);
    }

    #[test]
    fn test_render_queue_clear() {
        let mut queue = RenderQueue::new();
        queue.push(DrawCall::new(1, 6));
        queue.push(DrawCall::new(2, 3));
        assert_eq!(queue.len(), 2);
        queue.clear();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert!(!queue.sorted);
    }

    #[test]
    fn test_render_queue_len() {
        let mut queue = RenderQueue::new();
        assert_eq!(queue.len(), 0);
        queue.push(DrawCall::new(1, 3));
        assert_eq!(queue.len(), 1);
        queue.push(DrawCall::new(2, 3));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_render_queue_is_empty() {
        let mut queue = RenderQueue::new();
        assert!(queue.is_empty());
        queue.push(DrawCall::new(1, 3));
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_render_queue_draw_calls_access() {
        let mut queue = RenderQueue::new();
        queue.push(DrawCall::new(5, 12));
        let calls = queue.draw_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].mesh_handle, 5);
        assert_eq!(calls[0].index_count, 12);
    }

    #[test]
    fn test_render_queue_push_resets_sorted() {
        let mut queue = RenderQueue::new();
        queue.push(DrawCall::new(1, 3));
        queue.sort_by_material();
        assert!(queue.sorted);
        queue.push(DrawCall::new(2, 3));
        assert!(!queue.sorted);
    }

    // --- RenderPipelineStats tests ---

    #[test]
    fn test_render_pipeline_stats_new() {
        let stats = RenderPipelineStats::new();
        assert_eq!(stats.draw_call_count, 0);
        assert_eq!(stats.vertex_count, 0);
        assert_eq!(stats.triangle_count, 0);
        assert_eq!(stats.culled_objects, 0);
        assert_eq!(stats.render_time_ms, 0.0);
        assert_eq!(stats.pass_count, 0);
    }

    #[test]
    fn test_render_pipeline_stats_reset() {
        let mut stats = RenderPipelineStats::new();
        stats.draw_call_count = 10;
        stats.vertex_count = 100;
        stats.triangle_count = 50;
        stats.culled_objects = 5;
        stats.render_time_ms = 16.6;
        stats.pass_count = 3;
        stats.reset();
        assert_eq!(stats.draw_call_count, 0);
        assert_eq!(stats.vertex_count, 0);
        assert_eq!(stats.triangle_count, 0);
        assert_eq!(stats.culled_objects, 0);
        assert_eq!(stats.render_time_ms, 0.0);
        assert_eq!(stats.pass_count, 0);
    }

    // --- RenderPipeline3D tests ---

    #[test]
    fn test_pipeline_new() {
        let pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        assert_eq!(pipeline.backend(), RenderBackend::OpenGL);
        assert!(pipeline.render_queue().is_empty());
        assert!(pipeline.current_target.is_none());
        assert!(pipeline.frustum_culling_enabled());
        assert!(pipeline.current_pass().is_none());
        assert_eq!(pipeline.stats().draw_call_count, 0);
    }

    #[test]
    fn test_pipeline_set_render_target() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::Wgpu);
        assert!(pipeline.current_target.is_none());
        pipeline.set_render_target(RenderTarget::with_depth(1024, 768));
        let target = pipeline.current_target.as_ref().unwrap();
        assert_eq!(target.width, 1024);
        assert_eq!(target.height, 768);
        assert!(target.has_depth);
    }

    #[test]
    fn test_pipeline_begin_end_pass() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        assert!(pipeline.current_pass().is_none());
        pipeline.begin_pass(RenderPassType::Geometry);
        assert_eq!(pipeline.current_pass(), Some(RenderPassType::Geometry));
        assert_eq!(pipeline.stats().pass_count, 1);
        pipeline.begin_pass(RenderPassType::Forward);
        assert_eq!(pipeline.current_pass(), Some(RenderPassType::Forward));
        assert_eq!(pipeline.stats().pass_count, 2);
        pipeline.end_pass();
        assert!(pipeline.current_pass().is_none());
    }

    #[test]
    fn test_pipeline_submit_draw_call() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        pipeline.submit_draw_call(DrawCall::new(1, 6));
        pipeline.submit_draw_call(DrawCall::new(2, 3));
        assert_eq!(pipeline.render_queue().len(), 2);
        assert_eq!(pipeline.render_queue().draw_calls()[0].mesh_handle, 1);
        assert_eq!(pipeline.render_queue().draw_calls()[1].mesh_handle, 2);
    }

    #[test]
    fn test_pipeline_flush() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        pipeline.submit_draw_call(DrawCall::new(1, 6));
        pipeline.submit_draw_call(DrawCall::new(2, 3));
        assert_eq!(pipeline.render_queue().len(), 2);
        pipeline.flush();
        assert_eq!(pipeline.stats().draw_call_count, 2);
        assert_eq!(pipeline.stats().triangle_count, 3);
        assert!(pipeline.render_queue().is_empty());
    }

    #[test]
    fn test_pipeline_flush_accumulates() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        pipeline.submit_draw_call(DrawCall::new(1, 6));
        pipeline.flush();
        pipeline.submit_draw_call(DrawCall::new(2, 3));
        pipeline.flush();
        assert_eq!(pipeline.stats().draw_call_count, 2);
        assert_eq!(pipeline.stats().triangle_count, 3);
    }

    #[test]
    fn test_pipeline_enable_frustum_culling() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        assert!(pipeline.frustum_culling_enabled());
        pipeline.enable_frustum_culling(false);
        assert!(!pipeline.frustum_culling_enabled());
        pipeline.enable_frustum_culling(true);
        assert!(pipeline.frustum_culling_enabled());
    }

    #[test]
    fn test_pipeline_stats_access() {
        let pipeline = RenderPipeline3D::new(RenderBackend::Vulkan);
        let stats = pipeline.stats();
        assert_eq!(stats.draw_call_count, 0);
        assert_eq!(stats.pass_count, 0);
    }

    #[test]
    fn test_pipeline_backend() {
        let gl = RenderPipeline3D::new(RenderBackend::OpenGL);
        let wgpu = RenderPipeline3D::new(RenderBackend::Wgpu);
        let vk = RenderPipeline3D::new(RenderBackend::Vulkan);
        let mtl = RenderPipeline3D::new(RenderBackend::Metal);
        assert_eq!(gl.backend(), RenderBackend::OpenGL);
        assert_eq!(wgpu.backend(), RenderBackend::Wgpu);
        assert_eq!(vk.backend(), RenderBackend::Vulkan);
        assert_eq!(mtl.backend(), RenderBackend::Metal);
    }

    #[test]
    fn test_pipeline_current_pass() {
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        assert!(pipeline.current_pass().is_none());
        pipeline.begin_pass(RenderPassType::Shadow);
        assert_eq!(pipeline.current_pass(), Some(RenderPassType::Shadow));
        pipeline.end_pass();
        assert!(pipeline.current_pass().is_none());
    }

    #[test]
    fn test_pipeline_submit_scene() {
        let mut scene = Scene3D::new();
        let handle = scene.add_root_node(Node3D::with_mesh(Handle::<Mesh3D>::new(0, 0)));
        if let Some(node) = scene.node_mut(handle) {
            // Place AABB in front of camera (camera at z=5 looking at origin)
            node.set_aabb(AABB::new(
                Vec3::new(-1.0, -1.0, 9.0),
                Vec3::new(1.0, 1.0, 11.0),
            ));
        }

        let mut camera = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        camera.set_position(Vec3::new(0.0, 0.0, 5.0));
        camera.look_at(Vec3::ZERO);

        // Use camera's VP for scene culling so both frustums match
        let frustum = Frustum::from_view_projection(camera.view_projection());
        scene.cull(&frustum);

        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        let result = pipeline.submit_scene(&scene, &camera);
        assert!(result.is_ok());
        assert_eq!(pipeline.render_queue().len(), 1);
        assert_eq!(pipeline.render_queue().draw_calls()[0].mesh_handle, 0);
    }

    #[test]
    fn test_pipeline_submit_scene_no_culling() {
        let mut scene = Scene3D::new();
        let handle = scene.add_root_node(Node3D::with_mesh(Handle::<Mesh3D>::new(3, 0)));
        if let Some(node) = scene.node_mut(handle) {
            node.set_aabb(AABB::new(
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(1.0, 1.0, 1.0),
            ));
        }

        let frustum = Frustum::from_view_projection(Mat4::IDENTITY);
        scene.cull(&frustum);

        let camera = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        pipeline.enable_frustum_culling(false);
        pipeline.submit_scene(&scene, &camera).unwrap();
        assert_eq!(pipeline.render_queue().len(), 1);
        assert_eq!(pipeline.render_queue().draw_calls()[0].mesh_handle, 3);
    }

    #[test]
    fn test_pipeline_submit_scene_empty() {
        let scene = Scene3D::new();
        let camera = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        pipeline.submit_scene(&scene, &camera).unwrap();
        assert!(pipeline.render_queue().is_empty());
    }

    #[test]
    fn test_pipeline_submit_scene_multiple_entities() {
        let mut scene = Scene3D::new();
        let h1 = scene.add_root_node(Node3D::with_mesh(Handle::<Mesh3D>::new(0, 0)));
        let h2 = scene.add_root_node(Node3D::with_mesh(Handle::<Mesh3D>::new(1, 0)));
        let h3 = scene.add_root_node(Node3D::with_mesh(Handle::<Mesh3D>::new(2, 0)));
        for h in [h1, h2, h3] {
            if let Some(node) = scene.node_mut(h) {
                node.set_aabb(AABB::new(
                    Vec3::new(-1.0, -1.0, -1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ));
            }
        }

        let frustum = Frustum::from_view_projection(Mat4::IDENTITY);
        scene.cull(&frustum);

        let camera = Camera3D::perspective(45.0, 1.0, 0.1, 100.0);
        let mut pipeline = RenderPipeline3D::new(RenderBackend::OpenGL);
        pipeline.enable_frustum_culling(false);
        pipeline.submit_scene(&scene, &camera).unwrap();
        assert_eq!(pipeline.render_queue().len(), 3);
    }

    // --- OpenGLBackend tests ---

    #[test]
    fn test_opengl_backend_new() {
        let gl = OpenGLBackend::new();
        assert_eq!(gl.program_id, 0);
        assert_eq!(gl.vao_id, 0);
        assert_eq!(gl.vbo_id, 0);
        assert_eq!(gl.ibo_id, 0);
        assert_eq!(gl.viewport, (0, 0, 0, 0));
        assert!(gl.enabled_attributes.is_empty());
        assert_eq!(gl.draw_call_count(), 0);
        for t in gl.bound_textures.iter() {
            assert_eq!(*t, 0);
        }
    }

    #[test]
    fn test_opengl_backend_bind_vertex_array() {
        let mut gl = OpenGLBackend::new();
        gl.bind_vertex_array(42);
        assert_eq!(gl.vao_id, 42);
    }

    #[test]
    fn test_opengl_backend_bind_vertex_buffer() {
        let mut gl = OpenGLBackend::new();
        gl.bind_vertex_buffer(10);
        assert_eq!(gl.vbo_id, 10);
    }

    #[test]
    fn test_opengl_backend_bind_index_buffer() {
        let mut gl = OpenGLBackend::new();
        gl.bind_index_buffer(20);
        assert_eq!(gl.ibo_id, 20);
    }

    #[test]
    fn test_opengl_backend_use_program() {
        let mut gl = OpenGLBackend::new();
        gl.use_program(7);
        assert_eq!(gl.program_id, 7);
    }

    #[test]
    fn test_opengl_backend_set_viewport() {
        let mut gl = OpenGLBackend::new();
        gl.set_viewport(0, 0, 800, 600);
        assert_eq!(gl.viewport, (0, 0, 800, 600));
    }

    #[test]
    fn test_opengl_backend_vertex_attrib_array() {
        let mut gl = OpenGLBackend::new();
        gl.enable_vertex_attrib_array(0);
        gl.enable_vertex_attrib_array(1);
        gl.enable_vertex_attrib_array(2);
        assert_eq!(gl.enabled_attributes.len(), 3);
        // Enabling same index again should not duplicate
        gl.enable_vertex_attrib_array(1);
        assert_eq!(gl.enabled_attributes.len(), 3);
        gl.disable_vertex_attrib_array(1);
        assert_eq!(gl.enabled_attributes.len(), 2);
        assert!(!gl.enabled_attributes.contains(&1));
    }

    #[test]
    fn test_opengl_backend_active_texture() {
        let mut gl = OpenGLBackend::new();
        gl.active_texture(0, 100);
        gl.active_texture(1, 200);
        assert_eq!(gl.bound_textures[0], 100);
        assert_eq!(gl.bound_textures[1], 200);
        // Out of range unit should be ignored
        gl.active_texture(32, 999);
        assert_eq!(gl.bound_textures[31], 0);
    }

    #[test]
    fn test_opengl_backend_draw_elements() {
        let mut gl = OpenGLBackend::new();
        assert_eq!(gl.draw_call_count(), 0);
        gl.draw_elements(6, 0);
        gl.draw_elements(12, 24);
        assert_eq!(gl.draw_call_count(), 2);
    }

    #[test]
    fn test_opengl_backend_draw_arrays() {
        let mut gl = OpenGLBackend::new();
        gl.draw_arrays(3, 0);
        gl.draw_arrays(6, 3);
        gl.draw_arrays(9, 9);
        assert_eq!(gl.draw_call_count(), 3);
    }

    #[test]
    fn test_opengl_backend_draw_call_count_mixed() {
        let mut gl = OpenGLBackend::new();
        gl.draw_elements(6, 0);
        gl.draw_arrays(3, 0);
        gl.draw_elements(12, 0);
        assert_eq!(gl.draw_call_count(), 3);
    }

    #[test]
    fn test_opengl_backend_default() {
        let gl = OpenGLBackend::default();
        assert_eq!(gl.draw_call_count(), 0);
    }

    // --- WgpuBackend tests ---

    #[test]
    fn test_wgpu_backend_new() {
        let backend = WgpuBackend::new();
        assert_eq!(backend.device_id, 0);
        assert_eq!(backend.queue_id, 0);
        assert!(backend.surface_format.is_none());
        assert!(!backend.is_configured());
    }

    #[test]
    fn test_wgpu_backend_configure() {
        let mut backend = WgpuBackend::new();
        assert!(!backend.is_configured());
        backend.configure(ColorFormat::RGBA16F);
        assert!(backend.is_configured());
        assert_eq!(backend.surface_format, Some(ColorFormat::RGBA16F));
    }

    #[test]
    fn test_wgpu_backend_is_configured() {
        let mut backend = WgpuBackend::new();
        assert!(!backend.is_configured());
        backend.configure(ColorFormat::RGBA8);
        assert!(backend.is_configured());
    }

    #[test]
    fn test_wgpu_backend_default() {
        let backend = WgpuBackend::default();
        assert!(!backend.is_configured());
    }

    // --- RenderBackend enum tests ---

    #[test]
    fn test_render_backend_variants() {
        assert_eq!(RenderBackend::OpenGL, RenderBackend::OpenGL);
        assert_ne!(RenderBackend::OpenGL, RenderBackend::Wgpu);
        assert_ne!(RenderBackend::Vulkan, RenderBackend::Metal);
    }

    // --- RenderPassType enum tests ---

    #[test]
    fn test_render_pass_type_variants() {
        assert_eq!(RenderPassType::Shadow, RenderPassType::Shadow);
        assert_ne!(RenderPassType::Geometry, RenderPassType::Forward);
        assert_ne!(RenderPassType::Deferred, RenderPassType::PostProcess);
        assert_ne!(RenderPassType::UI, RenderPassType::Shadow);
    }
}
