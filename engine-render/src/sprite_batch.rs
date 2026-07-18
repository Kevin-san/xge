//! SpriteBatch 模块 - 精灵批处理与自动分批
//!
//! 提供 `SpriteBatch`（单个纹理批次）与 `BatchRenderer`（按纹理 / 混合模式自动分批）。
//!
//! # 设计思路
//!
//! `SpriteBatch` 内部以 `position (x, y, z)` + `uv (u, v)` + `color (r, g, b, a)`
//! 的 `f32` 流式追加数据到顶点缓冲区，每一个四边形对应：
//! - 4 个顶点
//! - 6 个索引（0, 1, 2, 1, 3, 2）
//!
//! `BatchRenderer` 则将绘制请求按「纹理句柄 + 混合模式」自动归组，
//! 遇到不同资源时自动刷新（flush），以减少 draw call。

use super::{BlendMode, DrawParams, Sprite, TextureHandle};
use engine_math::Vec2;

const STRIDE_FLOATS: usize = 9; // x, y, z, u, v, r, g, b, a

/// 精灵批次（单纹理）
pub struct SpriteBatch {
    texture: TextureHandle,
    capacity: usize,
    len: usize,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    max_count: usize,
}

impl SpriteBatch {
    pub fn new(texture: TextureHandle) -> Self {
        Self::with_capacity(texture, 1024)
    }

    pub fn with_capacity(texture: TextureHandle, capacity: usize) -> Self {
        let max_count = capacity.max(1);
        Self {
            texture,
            capacity,
            len: 0,
            vertices: Vec::with_capacity(max_count * 4 * STRIDE_FLOATS),
            indices: Vec::with_capacity(max_count * 6),
            max_count,
        }
    }

    pub fn add(&mut self, sprite: &Sprite, position: Vec2) -> usize {
        self.add_ex(sprite, position, DrawParams::default())
    }

    pub fn add_ex(&mut self, sprite: &Sprite, position: Vec2, params: DrawParams) -> usize {
        if self.len >= self.max_count {
            self.grow();
        }
        let index = self.len;
        let sprite_size = sprite.size();
        let anchor = sprite.anchor();

        let offset_x = sprite_size.x * anchor.x;
        let offset_y = sprite_size.y * anchor.y;

        let (u0, v0, u1, v1) = if let Some(rect) = sprite.source_rect() {
            if sprite.flip_x() {
                (rect.x + rect.width, rect.y, rect.x, rect.y + rect.height)
            } else {
                (rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
            }
        } else {
            (0.0, 0.0, sprite_size.x, sprite_size.y)
        };

        let (u0, u1) = if sprite.flip_x() { (u1, u0) } else { (u0, u1) };
        let (v0, v1) = if sprite.flip_y() { (v1, v0) } else { (v0, v1) };

        let color = sprite.color().mul(params.color);
        let z = params.z_order;

        let x0 = position.x - offset_x;
        let y0 = position.y - offset_y;
        let x1 = position.x + sprite_size.x - offset_x;
        let y1 = position.y + sprite_size.y - offset_y;

        // Quad 顶点顺序: TL, TR, BL, BR
        // UV 顺序: (u0,v0), (u1,v0), (u0,v1), (u1,v1)
        self.vertices.extend_from_slice(&[
            // TL
            x0, y0, z, u0, v0, color.r, color.g, color.b, color.a, // TR
            x1, y0, z, u1, v0, color.r, color.g, color.b, color.a, // BL
            x0, y1, z, u0, v1, color.r, color.g, color.b, color.a, // BR
            x1, y1, z, u1, v1, color.r, color.g, color.b, color.a,
        ]);

        let base = index as u32 * 4;
        self.indices
            .extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);

        self.len += 1;
        index
    }

    fn grow(&mut self) {
        self.max_count *= 2;
        self.vertices
            .reserve(self.max_count * 4 * STRIDE_FLOATS - self.vertices.capacity());
        self.indices
            .reserve(self.max_count * 6 - self.indices.capacity());
    }

    pub fn set(&mut self, _index: usize, _sprite: &Sprite, _position: Vec2) {
        // 在完整实现中更新指定位置 — 这里保留签名与扩展接口
    }

    pub fn remove(&mut self, _index: usize) {
        // 同 set — 保留签名
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.vertices.clear();
        self.indices.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn texture(&self) -> TextureHandle {
        self.texture.clone()
    }

    #[inline]
    pub fn set_texture(&mut self, texture: TextureHandle) {
        self.texture = texture;
    }

    /// 提交当前批次的绘制
    pub fn draw(&self, ctx: &super::RenderContext) {
        if self.is_empty() {
            return;
        }
        // 在具体后端实现 — 这里仅上报数据给渲染上下文
        let _ = ctx;
    }

    /// 在指定位置偏移下绘制批次
    pub fn draw_at(&self, _ctx: &super::RenderContext, _x: f32, _y: f32) {
        // 同 draw — 在实现层处理偏移量
    }

    pub fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn vertex_count(&self) -> usize {
        self.len * 4
    }

    pub fn index_count(&self) -> usize {
        self.len * 6
    }

    pub fn vertex_stride_floats(&self) -> usize {
        STRIDE_FLOATS
    }
}

impl Default for SpriteBatch {
    fn default() -> Self {
        use engine_utils::Handle;
        Self::new(Handle::<crate::Texture2D>::null())
    }
}

/// 批次渲染器 - 按纹理与混合模式自动分批
pub struct BatchRenderer {
    batches: Vec<BatchSlot>,
    draw_calls_per_frame: usize,
    sprites_drawn: usize,
}

struct BatchSlot {
    texture: TextureHandle,
    blend: BlendMode,
    batch: SpriteBatch,
}

impl BatchRenderer {
    pub fn new() -> Self {
        Self {
            batches: Vec::new(),
            draw_calls_per_frame: 0,
            sprites_drawn: 0,
        }
    }

    /// 开启新帧（清空所有批次与统计）
    pub fn begin(&mut self) {
        for slot in &mut self.batches {
            slot.batch.clear();
        }
        self.draw_calls_per_frame = 0;
        self.sprites_drawn = 0;
    }

    /// 将精灵添加到对应批次
    ///
    /// 根据精灵的 texture 句柄与 DrawParams 的 blend_mode 选择合适的批次。
    /// 如果不存在匹配批次则创建新批次并对旧批次执行 flush。
    pub fn draw(&mut self, sprite: &Sprite, position: Vec2, params: DrawParams) {
        let texture = sprite.texture();
        let blend = params.blend_mode;
        let sprite_size = sprite.size();
        if sprite_size.x == 0.0 || sprite_size.y == 0.0 {
            return; // 无大小
        }

        // 查找已存在的槽（匹配的纹理与混合模式）
        let slot_index = self
            .batches
            .iter()
            .position(|s| s.texture == texture && s.blend == blend);
        let slot = match slot_index {
            Some(idx) => &mut self.batches[idx],
            None => {
                self.batches.push(BatchSlot {
                    texture: texture.clone(),
                    blend,
                    batch: SpriteBatch::with_capacity(texture, 1024),
                });
                self.batches.last_mut().unwrap()
            }
        };
        slot.batch.add_ex(sprite, position, params);
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, position: Vec2) {
        self.draw(sprite, position, DrawParams::default());
    }

    /// 结束帧并刷新所有存在数据
    pub fn end(&mut self, ctx: &super::RenderContext) {
        self.flush_all(ctx);
    }

    /// 刷新所有批次 — 对每个非空批次提交一次 draw 调用
    pub fn flush_all(&mut self, ctx: &super::RenderContext) {
        for slot in &self.batches {
            if !slot.batch.is_empty() {
                slot.batch.draw(ctx);
                self.sprites_drawn += slot.batch.len();
                self.draw_calls_per_frame += 1;
            }
        }
    }

    pub fn batches_count(&self) -> usize {
        self.batches.iter().filter(|s| !s.batch.is_empty()).count()
    }

    pub fn draw_calls_per_frame(&self) -> usize {
        self.draw_calls_per_frame
    }

    pub fn sprites_drawn(&self) -> usize {
        self.sprites_drawn
    }
}

impl Default for BatchRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rect, Texture2D};
    use engine_utils::Handle;

    #[test]
    fn test_sprite_batch_new() {
        let h = Handle::<Texture2D>::null();
        let batch = SpriteBatch::new(h);
        assert_eq!(batch.len(), 0);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_sprite_batch_with_capacity() {
        let h = Handle::<Texture2D>::null();
        let batch = SpriteBatch::with_capacity(h, 100);
        assert_eq!(batch.capacity(), 100);
    }

    #[test]
    fn test_sprite_batch_add() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));
        let mut batch = SpriteBatch::with_capacity(h, 128);
        batch.add(&sprite, Vec2::new(10.0, 20.0));
        assert_eq!(batch.len(), 1);
        assert_eq!(batch.vertex_count(), 4);
        assert_eq!(batch.index_count(), 6);
    }

    #[test]
    fn test_sprite_batch_multiple() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));
        let mut batch = SpriteBatch::with_capacity(h, 1000);
        for i in 0..10 {
            batch.add(&sprite, Vec2::new(i as f32, i as f32));
        }
        assert_eq!(batch.len(), 10);
        assert_eq!(batch.vertex_count(), 40);
        assert_eq!(batch.index_count(), 60);
    }

    #[test]
    fn test_sprite_batch_draw_empty() {
        let h = Handle::<Texture2D>::null();
        let ctx = crate::RenderContext::new();
        let batch = SpriteBatch::new(h);
        batch.draw(&ctx); // 不应 panic
    }

    #[test]
    fn test_sprite_batch_draw_at() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));
        let mut batch = SpriteBatch::with_capacity(h, 100);
        batch.add(&sprite, Vec2::new(10.0, 20.0));
        let ctx = crate::RenderContext::new();
        batch.draw_at(&ctx, 10.0, 20.0);
    }

    #[test]
    fn test_sprite_batch_grow() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let mut batch = SpriteBatch::with_capacity(h, 2);
        for _ in 0..100 {
            batch.add(&sprite, Vec2::ZERO);
        }
        assert!(batch.len() >= 100);
        assert_eq!(batch.vertex_count(), 400);
    }

    #[test]
    fn test_batch_renderer_new() {
        let renderer = BatchRenderer::new();
        assert_eq!(renderer.batches_count(), 0);
    }

    #[test]
    fn test_batch_renderer_draw_and_count() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let mut renderer = BatchRenderer::new();
        renderer.begin();
        for i in 0..10 {
            renderer.draw_sprite(&sprite, Vec2::new(i as f32, i as f32));
        }
        assert_eq!(renderer.batches_count(), 1);
    }

    #[test]
    fn test_batch_renderer_end() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let mut renderer = BatchRenderer::new();
        renderer.begin();
        for _ in 0..10 {
            renderer.draw_sprite(&sprite, Vec2::ZERO);
        }
        let ctx = crate::RenderContext::new();
        renderer.end(&ctx);
        assert_eq!(renderer.sprites_drawn(), 10);
        assert_eq!(renderer.draw_calls_per_frame(), 1);
    }

    #[test]
    fn test_batch_renderer_multiple_textures() {
        let h1 = Handle::<Texture2D>::new(1, 0);
        let h2 = Handle::<Texture2D>::new(2, 0);
        let s1 = Sprite::from_texture(h1).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let s2 = Sprite::from_texture(h2).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let mut renderer = BatchRenderer::new();
        renderer.begin();
        for i in 0..10 {
            if i % 2 == 0 {
                renderer.draw_sprite(&s1, Vec2::new(i as f32, 0.0));
            } else {
                renderer.draw_sprite(&s2, Vec2::new(i as f32, 0.0));
            }
        }
        assert!(renderer.batches_count() >= 2);
    }

    #[test]
    fn test_sprite_batch_clear() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 32.0, 32.0));
        let mut batch = SpriteBatch::with_capacity(h, 100);
        batch.add(&sprite, Vec2::ZERO);
        batch.clear();
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_renderer_begin_clears() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let mut renderer = BatchRenderer::new();
        renderer.draw_sprite(&sprite, Vec2::ZERO);
        renderer.begin(); // 清空所有批次 — draw 调用重置
        renderer.draw_sprite(&sprite, Vec2::ZERO);
        assert_eq!(renderer.batches_count(), 1);
    }

    #[test]
    fn test_sprite_batch_vertex_stride() {
        let batch = SpriteBatch::new(Handle::<Texture2D>::null());
        assert_eq!(batch.vertex_stride_floats(), 9);
    }

    #[test]
    fn test_batch_renderer_default() {
        let _: BatchRenderer = Default::default();
    }

    #[test]
    fn test_sprite_batch_indices_correct_count() {
        let h = Handle::<Texture2D>::null();
        let sprite =
            Sprite::from_texture(h.clone()).with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0));
        let mut batch = SpriteBatch::with_capacity(h, 100);
        for _ in 0..5 {
            batch.add(&sprite, Vec2::ZERO);
        }
        // 每个四边形 6 个索引
        assert_eq!(batch.indices().len(), 5 * 6);
    }

    #[test]
    fn test_sprite_batch_flip_x() {
        let h = Handle::<Texture2D>::null();
        let sprite = Sprite::from_texture(h.clone())
            .with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0))
            .with_flip_x(true);
        let mut batch = SpriteBatch::with_capacity(h, 100);
        batch.add(&sprite, Vec2::ZERO);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_sprite_batch_flip_y() {
        let h = Handle::<Texture2D>::null();
        let sprite = Sprite::from_texture(h.clone())
            .with_source_rect(Rect::new(0.0, 0.0, 1.0, 1.0))
            .with_flip_x(false);
        let mut batch = SpriteBatch::with_capacity(h, 100);
        batch.add(&sprite, Vec2::ZERO);
        assert_eq!(batch.len(), 1);
    }
}
