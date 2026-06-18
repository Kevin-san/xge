//! Image 模块 - CPU 端像素数据
//!
//! 提供 Image 类型，表示未上传到 GPU 的像素数据，支持 PNG/JPG/BMP/GIF 加载。

use anyhow::{anyhow, Result};
use image::{DynamicImage, ImageBuffer, Rgba};

/// Image CPU 端像素数据
#[derive(Clone, Debug)]
pub struct Image {
    width: u32,
    height: u32,
    channels: u8,
    data: Vec<u8>,
}

impl Image {
    // region: 构造方法

    /// 从像素数据创建图像
    ///
    /// # Arguments
    /// * `width` - 宽度
    /// * `height` - 高度
    /// * `data` - RGBA 格式像素数据
    pub fn from_pixels(width: u32, height: u32, data: Vec<u8>) -> Self {
        let channels = 4; // Assume RGBA
        Self {
            width,
            height,
            channels,
            data,
        }
    }

    /// 从 RGBA 数据创建图像
    pub fn from_rgba(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self::from_pixels(width, height, data)
    }

    /// 从文件加载图像
    ///
    /// 支持 PNG、JPG、BMP、GIF 格式
    pub fn from_file(path: &str) -> Result<Self> {
        let img = image::open(path).map_err(|e| anyhow!("Failed to open image {}: {}", path, e))?;
        Ok(Self::from_dynamic_image(&img))
    }

    /// 从字节数组加载图像
    ///
    /// 支持 PNG、JPG、BMP、GIF 格式
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let img =
            image::load_from_memory(bytes).map_err(|e| anyhow!("Failed to decode image: {}", e))?;
        Ok(Self::from_dynamic_image(&img))
    }

    /// 从 image crate 的 DynamicImage 转换
    fn from_dynamic_image(img: &DynamicImage) -> Self {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let data = rgba.into_raw();
        Self {
            width,
            height,
            channels: 4,
            data,
        }
    }

    // endregion

    // region: 属性方法

    /// 获取宽度
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 获取尺寸
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// 获取通道数
    #[inline]
    pub fn channels(&self) -> u8 {
        self.channels
    }

    /// 获取像素数据引用
    #[inline]
    pub fn pixels(&self) -> &[u8] {
        &self.data
    }

    /// 获取像素数据可变引用
    #[inline]
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // endregion

    // region: 图像操作

    /// 保存图像到文件
    ///
    /// 根据文件扩展名自动选择格式（PNG、JPG、BMP、GIF）
    pub fn save(&self, path: &str) -> Result<()> {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, self.data.clone())
                .ok_or_else(|| anyhow!("Failed to create image buffer"))?;

        img.save(path)
            .map_err(|e| anyhow!("Failed to save image to {}: {}", path, e))?;
        Ok(())
    }

    /// 裁剪图像区域
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, self.data.clone())
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let cropped = image::imageops::crop_imm(&img, x, y, width, height).to_image();
        Self::from_dynamic_image(&DynamicImage::ImageRgba8(cropped))
    }

    /// 获取图像区域（返回新 Image）
    pub fn region(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        self.crop(x, y, width, height)
    }

    /// 水平翻转
    pub fn flip_horizontal(&mut self) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, std::mem::take(&mut self.data))
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let flipped = image::imageops::flip_horizontal(&img);
        self.data = flipped.into_raw();
    }

    /// 垂直翻转
    pub fn flip_vertical(&mut self) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, std::mem::take(&mut self.data))
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let flipped = image::imageops::flip_vertical(&img);
        self.data = flipped.into_raw();
    }

    /// 旋转 90 度
    pub fn rotate_90(&mut self) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, std::mem::take(&mut self.data))
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let rotated = image::imageops::rotate90(&img);
        let (w, h) = rotated.dimensions();
        self.width = w;
        self.height = h;
        self.data = rotated.into_raw();
    }

    /// 旋转 180 度
    pub fn rotate_180(&mut self) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, std::mem::take(&mut self.data))
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let rotated = image::imageops::rotate180(&img);
        self.data = rotated.into_raw();
    }

    /// 旋转 270 度
    pub fn rotate_270(&mut self) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, std::mem::take(&mut self.data))
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let rotated = image::imageops::rotate270(&img);
        let (w, h) = rotated.dimensions();
        self.width = w;
        self.height = h;
        self.data = rotated.into_raw();
    }

    /// 调整图像大小
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width, self.height, std::mem::take(&mut self.data))
                .unwrap_or_else(|| ImageBuffer::new(1, 1));

        let resized = image::imageops::resize(
            &img,
            new_width,
            new_height,
            image::imageops::FilterType::Triangle,
        );
        self.width = new_width;
        self.height = new_height;
        self.data = resized.into_raw();
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        // Create a simple 2x2 RGBA image
        let data = vec![
            255, 0, 0, 255, // Red pixel
            0, 255, 0, 255, // Green pixel
            0, 0, 255, 255, // Blue pixel
            255, 255, 0, 255, // Yellow pixel
        ];

        let result = Image::from_rgba(2, 2, data);
        assert_eq!(result.width(), 2);
        assert_eq!(result.height(), 2);
        assert_eq!(result.channels(), 4);
    }

    #[test]
    fn test_pixels_access() {
        let data = vec![
            255u8, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
        ];
        let img = Image::from_rgba(2, 2, data);
        let pixels = img.pixels();
        assert_eq!(pixels.len(), 16); // 4 pixels * 4 channels
    }

    #[test]
    fn test_flip_horizontal() {
        let data = vec![
            255, 0, 0, 255, 0, 0, 255, 255, // Red, Blue
            0, 255, 0, 255, 255, 255, 0, 255, // Green, Yellow
        ];
        let mut img = Image::from_rgba(2, 2, data);
        img.flip_horizontal();
        // After horizontal flip, pixel (0,0) should have been Blue and (0,1) should have been Yellow
        let pixels = img.pixels();
        assert_eq!(pixels[0..4], [0, 0, 255, 255]); // Now Blue
    }

    #[test]
    fn test_flip_vertical() {
        let data = vec![
            255, 0, 0, 255, 0, 0, 255, 255, // Red, Blue (top row)
            0, 255, 0, 255, 255, 255, 0, 255, // Green, Yellow (bottom row)
        ];
        let mut img = Image::from_rgba(2, 2, data);
        img.flip_vertical();
        // After vertical flip, top row should have bottom row's colors
        let pixels = img.pixels();
        assert_eq!(pixels[0..4], [0, 255, 0, 255]); // Now Green
    }

    #[test]
    fn test_rotate_90() {
        let mut img = Image::from_rgba(2, 1, vec![255, 0, 0, 255, 0, 255, 0, 255]);
        img.rotate_90();
        assert_eq!(img.width(), 1);
        assert_eq!(img.height(), 2);
    }

    #[test]
    fn test_resize() {
        let data = vec![255u8; 400]; // 10*10*4
        let mut img = Image::from_rgba(10, 10, data);
        img.resize(5, 5);
        assert_eq!(img.width(), 5);
        assert_eq!(img.height(), 5);
    }

    #[test]
    fn test_region() {
        let data = vec![255u8; 256]; // 4x4 image * 4 channels
        let img = Image::from_rgba(4, 4, data);
        let region = img.region(1, 1, 2, 2);
        assert_eq!(region.width(), 2);
        assert_eq!(region.height(), 2);
    }

    #[test]
    fn test_crop() {
        let data = vec![255u8; 256]; // 4x4 image * 4 channels
        let img = Image::from_rgba(4, 4, data);
        let cropped = img.crop(1, 1, 2, 2);
        assert_eq!(cropped.width(), 2);
        assert_eq!(cropped.height(), 2);
    }
}
