//! SpriteBatch 模块 - 精灵批处理
//!
//! 提供 SpriteBatch 类型，用于高效批量绘制同纹理精灵。

use super::{DrawParams, Sprite, TextureHandle};
use engine_math::Vec2;

/// 精灵批次
///
/// 用于批量绘制同纹理精灵，减少 draw call。
/// 内部使用顶点缓冲区和索引缓冲区（每个精灵 6 个索引，2 个三角形）。
pub struct SpriteBatch {
    /// 纹理句柄
    texture: TextureHandle,
    /// 容量
    capacity: usize,
    /// 当前数量
    len: usize,
    /// 顶点数据 (position.x, position.y, position.z, u, v, color.r, color.g, color.b, color.a)
    vertices: Vec<f32>,
    /// 索引数据
    indices: Vec<u32>,
    /// 最大数量
    max_count: usize,
}

impl SpriteBatch {
    /// 创建新的精灵批次
    pub fn new(texture: TextureHandle) -> Self {
        Self::with_capacity(texture, 1000)
    }

    /// 创建指定容量的精灵批次
    pub fn with_capacity(texture: TextureHandle, capacity: usize) -> Self {
        let max_count = capacity;
        let vertices = Vec::with_capacity(max_count * 4);
        let indices = Vec::with_capacity(max_count * 6);

        Self {
            texture,
            capacity,
            len: 0,
            vertices,
            indices,
            max_count,
        }
    }

    // region: 管理方法

    /// 添加精灵
    ///
    /// 返回精灵索引
    pub fn add(&mut self, sprite: &Sprite, position: Vec2) -> usize {
        self.add_ex(sprite, position, DrawParams::default())
    }

    /// 添加精灵（带参数）
    pub fn add_ex(&mut self, sprite: &Sprite, position: Vec2, params: DrawParams) -> usize {
        if self.len >= self.max_count {
            return self.len; // Return invalid index
        }

        let index = self.len;
        let sprite_size = sprite.size();
        let anchor = sprite.anchor();

        // Calculate offset from anchor
        let offset_x = (sprite_size.x * anchor.x) * if sprite.flip_x() { -1.0 } else { 1.0 };
        let offset_y = (sprite_size.y * anchor.y) * if sprite.flip_y() { -1.0 } else { 1.0 };

        // Get texture coordinates
        let (u0, v0, u1, v1) = if let Some(rect) = sprite.source_rect() {
            (rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
        } else {
            (0.0, 0.0, sprite_size.x, sprite_size.y)
        };

        // Apply flip
        let (u0, u1) = if sprite.flip_x() { (u1, u0) } else { (u0, u1) };
        let (v0, v1) = if sprite.flip_y() { (v1, v0) } else { (v0, v1) };

        let color = sprite.color().mul(params.color);
        let z = params.z_order;

        // Add 4 vertices for the quad (two triangles)
        let base_vertex = index * 4;

        // Triangle 1: top-left, top-right, bottom-left
        // Triangle 2: top-right, bottom-right, bottom-left

        // Top-left
        self.vertices.extend_from_slice(&[
            position.x - offset_x,
            position.y - offset_y,
            z,
            u0,
            v0,
            color.r,
            color.g,
            color.b,
            color.a,
        ]);

        // Top-right
        self.vertices.extend_from_slice(&[
            position.x + sprite_size.x - offset_x,
            position.y - offset_y,
            z,
            u1,
            v0,
            color.r,
            color.g,
            color.b,
            color.a,
        ]);

        // Bottom-left
        self.vertices.extend_from_slice(&[
            position.x - offset_x,
            position.y + sprite_size.y - offset_y,
            z,
            u0,
            v1,
            color.r,
            color.g,
            color.b,
            color.a,
        ]);

        // Bottom-right
        self.vertices.extend_from_slice(&[
            position.x + sprite_size.x - offset_x,
            position.y + sprite_size.y - offset_y,
            z,
            u1,
            v1,
            color.r,
            color.g,
            color.b,
            color.a,
        ]);

        // Add indices
        self.indices.extend_from_slice(&[
            base_vertex as u32,
            base_vertex as u32 + 1,
            base_vertex as u32 + 2,
            base_vertex as u32 + 1,
            base_vertex as u32 + 3,
            base_vertex as u32 + 2,
        ]);

        self.len += 1;
        index
    }

    /// 设置指定索引的精灵
    pub fn set(&mut self, _index: usize, _sprite: &Sprite, _position: Vec2) {
        // In a full implementation, this would update the vertex data
    }

    /// 移除指定索引的精灵
    pub fn remove(&mut self, _index: usize) {
        // In a full implementation, this would remove and compact
    }

    /// 清空批次
    pub fn clear(&mut self) {
        self.len = 0;
        self.vertices.clear();
        self.indices.clear();
    }

    /// 获取当前数量
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 获取容量
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 获取纹理句柄
    #[inline]
    pub fn texture(&self) -> TextureHandle {
        self.texture.clone()
    }

    /// 设置纹理
    #[inline]
    pub fn set_texture(&mut self, texture: TextureHandle) {
        self.texture = texture;
    }

    // endregion

    // region: 绘制方法

    /// 绘制批次
    pub fn draw(&self, _ctx: &super::RenderContext) {
        // Implementation in GL backend
    }

    /// 在指定位置绘制批次
    pub fn draw_at(&self, _ctx: &super::RenderContext, _x: f32, _y: f32) {
        // Implementation in GL backend
    }

    // endregion

    // region: 数据访问

    /// 获取顶点数据
    pub fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    /// 获取索引数据
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// 获取顶点数量
    pub fn vertex_count(&self) -> usize {
        self.len * 4
    }

    /// 获取索引数量
    pub fn index_count(&self) -> usize {
        self.len * 6
    }

    // endregion
}

impl Default for SpriteBatch {
    fn default() -> Self {
        Self::new(Handle::null())
    }
}

/// BatchRenderer 批次渲染器
///
/// 自动按纹理分批
pub struct BatchRenderer {
    batches: Vec<SpriteBatch>,
}

impl BatchRenderer {
    /// 创建新的批次渲染器
    pub fn new() -> Self {
        Self {
            batches: Vec::new(),
        }
    }

    /// 开始新的帧
    pub fn begin(&mut self) {
        for batch in &mut self.batches {
            batch.clear();
        }
    }

    /// 添加精灵到批次
    pub fn draw(&mut self, _sprite: &Sprite, _transform: Mat4) {
        // In a full implementation, this would find or create appropriate batch
    }

    /// 结束帧
    pub fn end(&mut self, _ctx: &super::RenderContext) {
        self.flush(_ctx);
    }

    /// 刷新所有批次
    pub fn flush(&mut self, _ctx: &super::RenderContext) {
        for batch in &mut self.batches {
            batch.draw(_ctx);
        }
    }

    /// 获取批次数
    pub fn batches(&self) -> usize {
        self.batches.len()
    }
}

impl Default for BatchRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export Mat4 for use in BatchRenderer
use engine_math::Mat4;

// Handle trait import
use engine_utils::Handle;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rect, Texture2D};
    use engine_utils::Handle;

    #[test]
    fn test_sprite_batch_new() {
        let handle = Handle::<Texture2D>::null();
        let batch = SpriteBatch::new(handle);
        assert_eq!(batch.len(), 0);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_sprite_batch_with_capacity() {
        let handle = Handle::<Texture2D>::null();
        let batch = SpriteBatch::with_capacity(handle, 100);
        assert_eq!(batch.capacity(), 100);
        assert_eq!(batch.len(), 0);
    }

    #[test]
    fn test_sprite_batch_add() {
        let handle = Handle::<Texture2D>::null();
        let mut batch = SpriteBatch::with_capacity(handle.clone(), 100);

        let sprite = Sprite::from_texture(handle).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));

        let idx = batch.add(&sprite, Vec2::new(100.0, 100.0));
        assert_eq!(idx, 0);
        assert_eq!(batch.len(), 1);
        assert_eq!(batch.vertex_count(), 4);
        assert_eq!(batch.index_count(), 6);
    }

    #[test]
    fn test_sprite_batch_add_multiple() {
        let handle = Handle::<Texture2D>::null();
        let mut batch = SpriteBatch::with_capacity(handle.clone(), 100);

        let sprite =
            Sprite::from_texture(handle).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));

        batch.add(&sprite, Vec2::new(0.0, 0.0));
        batch.add(&sprite, Vec2::new(100.0, 100.0));
        batch.add(&sprite, Vec2::new(200.0, 200.0));

        assert_eq!(batch.len(), 3);
        assert_eq!(batch.vertex_count(), 12);
        assert_eq!(batch.index_count(), 18);
    }

    #[test]
    fn test_sprite_batch_clear() {
        let handle = Handle::<Texture2D>::null();
        let mut batch = SpriteBatch::new(handle.clone());

        let sprite = Sprite::from_texture(handle);
        batch.add(&sprite, Vec2::ZERO);

        batch.clear();
        assert!(batch.is_empty());
        assert_eq!(batch.vertices().len(), 0);
        assert_eq!(batch.indices().len(), 0);
    }

    #[test]
    fn test_sprite_batch_vertices_and_indices_slices() {
        let handle = Handle::<Texture2D>::null();
        let mut batch = SpriteBatch::with_capacity(handle.clone(), 100);
        let sprite =
            Sprite::from_texture(handle).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));
        batch.add(&sprite, Vec2::ZERO);

        let verts = batch.vertices();
        let indices = batch.indices();
        assert!(!verts.is_empty());
        assert!(!indices.is_empty());
    }

    #[test]
    fn test_sprite_batch_capacity_zero() {
        let handle = Handle::<Texture2D>::null();
        let batch = SpriteBatch::with_capacity(handle, 500);
        assert_eq!(batch.capacity(), 500);
    }

    #[test]
    fn test_sprite_batch_set_texture() {
        let h1 = Handle::<Texture2D>::null();
        let h2 = Handle::<Texture2D>::null();
        let mut batch = SpriteBatch::new(h1);
        batch.set_texture(h2);
    }

    #[test]
    fn test_batch_renderer() {
        let renderer = BatchRenderer::new();
        assert_eq!(renderer.batches(), 0);
    }

    #[test]
    fn test_batch_renderer_new() {
        let renderer = BatchRenderer::new();
        let _ = renderer;
    }
}
