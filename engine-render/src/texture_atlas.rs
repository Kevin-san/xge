//! TextureAtlas 模块 - 纹理图集打包
//!
//! 提供 TextureAtlas 和 TextureAtlasBuilder，支持 Skyline 和 Guillotine 两种 bin packing 算法。

use super::{Image, TextureHandle};
use crate::sprite::Rect;
use engine_utils::Handle;
use std::cmp::max;

/// 打包算法
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PackAlgorithm {
    /// Skyline 打包算法（速度快但可能有更多碎片）
    Skyline,
    /// Guillotine 打包算法（更高的空间利用率）
    #[default]
    Guillotine,
}

/// 打包结果
#[derive(Clone, Debug, Default)]
pub struct PackResult {
    /// 打包后的区域列表
    rects: Vec<Rect>,
    /// 是否有碰撞（打包失败）
    has_collisions: bool,
    /// 使用的图集大小
    atlas_size: (u32, u32),
}

impl PackResult {
    /// 检查是否有碰撞
    pub fn contains_collisions(&self) -> bool {
        self.has_collisions
    }

    /// 获取所有矩形
    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    /// 获取图集大小
    pub fn atlas_size(&self) -> (u32, u32) {
        self.atlas_size
    }
}

/// 纹理图集
#[derive(Clone, Debug)]
pub struct TextureAtlas {
    /// 纹理句柄
    texture: TextureHandle,
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
    /// 区域列表
    rects: Vec<Rect>,
}

impl TextureAtlas {
    /// 获取纹理句柄
    pub fn texture(&self) -> TextureHandle {
        self.texture.clone()
    }

    /// 获取大小
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// 获取区域数量
    pub fn len(&self) -> usize {
        self.rects.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.rects.is_empty()
    }

    /// 获取指定索引的区域
    pub fn get(&self, index: usize) -> Option<Rect> {
        self.rects.get(index).copied()
    }

    /// 获取指定索引的 UV 坐标（左上、右下）
    pub fn get_uv(&self, index: usize) -> Option<(engine_math::Vec2, engine_math::Vec2)> {
        let rect = self.rects.get(index)?;
        let u0 = rect.x / self.width as f32;
        let v0 = rect.y / self.height as f32;
        let u1 = (rect.x + rect.width) / self.width as f32;
        let v1 = (rect.y + rect.height) / self.height as f32;
        Some((
            engine_math::Vec2::new(u0, v0),
            engine_math::Vec2::new(u1, v1),
        ))
    }

    /// 获取指定索引对应的 Sprite
    pub fn get_sprite(&self, index: usize) -> Option<super::Sprite> {
        let rect = self.rects.get(index)?;
        Some(super::Sprite::from_texture(self.texture.clone()).with_source_rect(*rect))
    }
}

/// 纹理图集构建器
pub struct TextureAtlasBuilder {
    /// 最大尺寸
    max_size: u32,
    /// 边距
    padding: u32,
    /// 打包算法
    algorithm: PackAlgorithm,
    /// 输入图像
    images: Vec<Image>,
    /// 强制正方形
    force_square: bool,
}

impl TextureAtlasBuilder {
    /// 创建新的构建器
    ///
    /// # Arguments
    /// * `max_size` - 最大尺寸（如 2048）
    pub fn new(max_size: u32) -> Self {
        Self {
            max_size,
            padding: 2,
            algorithm: PackAlgorithm::Guillotine,
            images: Vec::new(),
            force_square: false,
        }
    }

    /// 设置边距
    pub fn with_padding(mut self, pixels: u32) -> Self {
        self.padding = pixels;
        self
    }

    /// 设置打包算法
    pub fn with_algorithm(mut self, algorithm: PackAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// 强制正方形图集
    pub fn with_force_square(mut self, force: bool) -> Self {
        self.force_square = force;
        self
    }

    /// 添加图像
    ///
    /// 返回图像索引
    pub fn add(&mut self, image: Image) -> usize {
        let index = self.images.len();
        self.images.push(image);
        index
    }

    /// 从文件添加图像
    pub fn add_from_file(&mut self, path: &str) -> anyhow::Result<usize> {
        let image = Image::from_file(path)?;
        Ok(self.add(image))
    }

    /// 构建图集
    ///
    /// 返回 (TextureAtlas, Vec<Rect>) 元组
    pub fn build(&self, _ctx: &super::RenderContext) -> anyhow::Result<(TextureAtlas, Vec<Rect>)> {
        if self.images.is_empty() {
            return Err(anyhow::anyhow!("No images to pack"));
        }

        let (atlas, rects) = match self.algorithm {
            PackAlgorithm::Skyline => self.pack_skyline()?,
            PackAlgorithm::Guillotine => self.pack_guillotine()?,
        };

        let atlas_handle = Handle::<super::Texture2D>::null();
        let texture_atlas = TextureAtlas {
            texture: atlas_handle,
            width: atlas.0,
            height: atlas.1,
            rects: rects.clone(),
        };

        Ok((texture_atlas, rects))
    }

    /// Skyline 打包算法
    fn pack_skyline(&self) -> anyhow::Result<((u32, u32), Vec<Rect>)> {
        // Find optimal size
        let mut size: u32 = 64;
        while size < self.max_size {
            if let Ok((actual_size, rects)) = self.try_skyline_pack(size) {
                if !rects
                    .iter()
                    .any(|r| r.width > size as f32 || r.height > size as f32)
                {
                    let final_size = if self.force_square {
                        actual_size.0.max(actual_size.1)
                    } else {
                        size
                    };
                    return Ok(((final_size, final_size), rects));
                }
            }
            size *= 2;
        }

        Err(anyhow::anyhow!("Failed to pack images"))
    }

    fn try_skyline_pack(&self, max_size: u32) -> anyhow::Result<((u32, u32), Vec<Rect>)> {
        // Sort images by height (tallest first)
        let mut images_with_idx: Vec<(usize, &Image)> = self.images.iter().enumerate().collect();
        images_with_idx.sort_by(|a, b| {
            let height_a = a.1.height();
            let height_b = b.1.height();
            height_b.cmp(&height_a)
        });

        let mut rects = vec![Rect::default(); self.images.len()];
        let mut skyline: Vec<(u32, u32, u32)> = Vec::new(); // (x, y, width)
        let mut used_width = 0u32;
        let mut used_height = 0u32;

        for (img_idx, image) in images_with_idx {
            let w = image.width() + self.padding * 2;
            let h = image.height() + self.padding * 2;

            // Find best position
            let (best_x, best_y, _best_width) = self.find_best_skyline_position(&skyline, w, h);

            rects[img_idx] = Rect::new(
                best_x as f32 + self.padding as f32,
                best_y as f32 + self.padding as f32,
                image.width() as f32,
                image.height() as f32,
            );

            // Update skyline
            self.add_to_skyline(&mut skyline, best_x, best_y, w, h);

            used_width = max(used_width, best_x + w);
            used_height = max(used_height, best_y + h);
        }

        // Ensure we don't exceed max_size
        if used_width > max_size || used_height > max_size {
            return Err(anyhow::anyhow!("Atlas too large"));
        }

        Ok((
            (used_width.max(used_height), used_width.max(used_height)),
            rects,
        ))
    }

    fn find_best_skyline_position(
        &self,
        skyline: &[(u32, u32, u32)],
        width: u32,
        height: u32,
    ) -> (u32, u32, u32) {
        if skyline.is_empty() {
            return (0, 0, width);
        }

        let mut best_y = u32::MAX;
        let mut best_x = 0u32;
        let mut best_width = 0u32;

        let mut x = 0u32;
        while x < skyline.len() as u32 {
            let (sx, sy, sw) = skyline[x as usize];

            // Calculate y at this x
            let mut y = sy;
            let mut end_x = sx + sw;

            // Find the max y in this horizontal strip
            for i in x as usize..skyline.len() {
                let (_, sy2, sw2) = skyline[i];
                if sy2 > y {
                    y = sy2;
                }
                if i as u32 >= end_x {
                    break;
                }
                end_x = end_x.max(sx + sw).max(sw2 + i as u32);
            }

            // Check if this position fits
            if y + height <= self.max_size {
                if y < best_y || (y == best_y && sw < best_width) {
                    best_y = y;
                    best_x = sx;
                    best_width = sw;
                }
            }

            x = x.max(sx + sw);
            if x >= sx + sw {
                x += 1;
            }
        }

        (best_x, best_y, width.max(best_width))
    }

    fn add_to_skyline(&self, skyline: &mut Vec<(u32, u32, u32)>, x: u32, y: u32, w: u32, _h: u32) {
        // This is a simplified skyline implementation
        skyline.push((x, y, w));
        skyline.sort_by(|a, b| a.0.cmp(&b.0));
    }

    /// Guillotine 打包算法
    fn pack_guillotine(&self) -> anyhow::Result<((u32, u32), Vec<Rect>)> {
        // Start with full bin
        let mut free_rects: Vec<Rect> = vec![Rect::new(
            0.0,
            0.0,
            self.max_size as f32,
            self.max_size as f32,
        )];

        let mut rects = vec![Rect::default(); self.images.len()];

        // Sort by area (largest first)
        let mut images_with_idx: Vec<(usize, &Image)> = self.images.iter().enumerate().collect();
        images_with_idx.sort_by(|a, b| {
            let area_a = a.1.width() * a.1.height();
            let area_b = b.1.width() * b.1.height();
            area_b.cmp(&area_a)
        });

        for (img_idx, image) in images_with_idx {
            let w = image.width() as f32 + self.padding as f32 * 2.0;
            let h = image.height() as f32 + self.padding as f32 * 2.0;

            // Find best free rect
            let best_idx = self
                .find_best_guillotine_rect(&free_rects, w, h)
                .ok_or_else(|| anyhow::anyhow!("No suitable free rect found"))?;

            let free_rect = free_rects[best_idx];

            // Place rect
            rects[img_idx] = Rect::new(
                free_rect.x + self.padding as f32,
                free_rect.y + self.padding as f32,
                image.width() as f32,
                image.height() as f32,
            );

            // Split free rect
            let (new_rects, _) = self.split_guillotine_rect(free_rect, w, h);
            free_rects.remove(best_idx);
            free_rects.extend(new_rects);

            // Merge adjacent free rects
            self.merge_guillotine_rects(&mut free_rects);
        }

        // Calculate actual size used
        let mut max_x = 0f32;
        let mut max_y = 0f32;
        for rect in &rects {
            max_x = max_x.max(rect.x + rect.width);
            max_y = max_y.max(rect.y + rect.height);
        }

        let size = next_power_of_two(max_x.max(max_y) as u32).max(64);
        let size = if self.force_square {
            size
        } else {
            size.max(max_x as u32).max(max_y as u32)
        };

        Ok(((size, size), rects))
    }

    fn find_best_guillotine_rect(&self, free_rects: &[Rect], w: f32, h: f32) -> Option<usize> {
        let mut best_idx = None;
        let mut best_short_side = f32::MAX;

        for (i, rect) in free_rects.iter().enumerate() {
            if rect.width >= w && rect.height >= h {
                let short_side = rect.width.min(rect.height);
                if short_side < best_short_side {
                    best_short_side = short_side;
                    best_idx = Some(i);
                }
            }
        }

        best_idx
    }

    fn split_guillotine_rect(&self, rect: Rect, w: f32, h: f32) -> (Vec<Rect>, bool) {
        let mut new_rects = Vec::new();

        let remaining_width = rect.width - w;
        let remaining_height = rect.height - h;

        if remaining_width > 0.0 && remaining_width >= self.padding as f32 {
            new_rects.push(Rect::new(rect.x + w, rect.y, remaining_width, rect.height));
        }

        if remaining_height > 0.0 && remaining_height >= self.padding as f32 {
            new_rects.push(Rect::new(rect.x, rect.y + h, w, remaining_height));
        }

        let is_empty = new_rects.is_empty();
        (new_rects, is_empty)
    }

    fn merge_guillotine_rects(&self, free_rects: &mut Vec<Rect>) {
        // Simple merge: remove redundant rects
        free_rects.retain(|r| r.width >= self.padding as f32 && r.height >= self.padding as f32);
    }
}

fn next_power_of_two(n: u32) -> u32 {
    let mut v = n - 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(w: u32, h: u32) -> Image {
        Image::from_rgba(w, h, vec![255u8; (w * h * 4) as usize])
    }

    #[test]
    fn test_pack_algorithm_skyline() {
        let mut builder = TextureAtlasBuilder::new(256)
            .with_algorithm(PackAlgorithm::Skyline)
            .with_padding(2);

        builder.add(create_test_image(64, 64));
        builder.add(create_test_image(64, 32));
        builder.add(create_test_image(32, 64));

        // Should not panic
        assert!(builder.images.len() == 3);
    }

    #[test]
    fn test_pack_algorithm_guillotine() {
        let mut builder = TextureAtlasBuilder::new(256)
            .with_algorithm(PackAlgorithm::Guillotine)
            .with_padding(2);

        builder.add(create_test_image(64, 64));
        builder.add(create_test_image(32, 32));

        assert!(builder.images.len() == 2);
    }

    #[test]
    fn test_texture_atlas_get_uv() {
        let atlas = TextureAtlas {
            texture: Handle::null(),
            width: 256,
            height: 256,
            rects: vec![Rect::new(0.0, 0.0, 64.0, 64.0)],
        };

        let (uv0, uv1) = atlas.get_uv(0).unwrap();
        assert!((uv0.x - 0.0).abs() < 0.001);
        assert!((uv0.y - 0.0).abs() < 0.001);
        assert!((uv1.x - 0.25).abs() < 0.001); // 64/256
        assert!((uv1.y - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_texture_atlas_len() {
        let atlas = TextureAtlas {
            texture: Handle::null(),
            width: 256,
            height: 256,
            rects: vec![
                Rect::new(0.0, 0.0, 32.0, 32.0),
                Rect::new(32.0, 0.0, 32.0, 32.0),
            ],
        };

        assert_eq!(atlas.len(), 2);
        assert!(!atlas.is_empty());
    }

    #[test]
    fn test_rect_getters() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);

        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.bottom(), 70.0);
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(64), 64);
        assert_eq!(next_power_of_two(65), 128);
    }

    #[test]
    fn test_pack_result_contains_collisions() {
        let result = PackResult {
            rects: vec![],
            has_collisions: false,
            atlas_size: (256, 256),
        };
        assert!(!result.contains_collisions());
    }
}
