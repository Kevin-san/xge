//! Texture 模块 - 纹理与采样器
//!
//! 提供 Texture2D 纹理类型、Sampler 采样器、TextureHandle 句柄管理。

use engine_utils::Handle;
use parking_lot::RwLock;
use std::sync::Arc;

/// 纹理格式
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextureFormat {
    /// RGBA 8 位每通道
    RGBA8,
    /// RGBA 16 位浮点
    RGBA16F,
    /// 单通道 8 位
    R8,
    /// BGRA 8 位
    BGRA8,
    #[default]
    Unknown,
}

impl TextureFormat {
    /// 获取 OpenGL 格式
    pub fn to_gl_internal_format(&self) -> u32 {
        match self {
            TextureFormat::RGBA8 => glow::RGBA8,
            TextureFormat::RGBA16F => glow::RGBA16F,
            TextureFormat::R8 => glow::R8,
            TextureFormat::BGRA8 => glow::RGBA8, // OpenGL doesn't have BGRA8 internal format, we use RGBA8
            TextureFormat::Unknown => glow::RGBA8,
        }
    }

    /// 获取 OpenGL 格式（用于 glTexImage2D）
    pub fn to_gl_format(&self) -> u32 {
        match self {
            TextureFormat::RGBA8 | TextureFormat::RGBA16F | TextureFormat::BGRA8 => glow::RGBA,
            TextureFormat::R8 => glow::RED,
            TextureFormat::Unknown => glow::RGBA,
        }
    }

    /// 获取数据类型
    pub fn to_gl_type(&self) -> u32 {
        match self {
            TextureFormat::RGBA8 | TextureFormat::R8 | TextureFormat::BGRA8 => glow::UNSIGNED_BYTE,
            TextureFormat::RGBA16F => glow::HALF_FLOAT,
            TextureFormat::Unknown => glow::UNSIGNED_BYTE,
        }
    }

    /// 获取字节数
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            TextureFormat::RGBA8 | TextureFormat::BGRA8 => 4,
            TextureFormat::R8 => 1,
            TextureFormat::RGBA16F => 8,
            TextureFormat::Unknown => 4,
        }
    }
}

/// 过滤模式
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FilterMode {
    /// 线性过滤
    Linear,
    /// 最近邻过滤
    Nearest,
    #[default]
    Default,
}

impl FilterMode {
    /// 转换为 OpenGL 过滤模式
    pub fn to_gl(&self, mipmap: bool) -> u32 {
        match self {
            FilterMode::Linear => {
                if mipmap {
                    glow::LINEAR_MIPMAP_LINEAR
                } else {
                    glow::LINEAR
                }
            }
            FilterMode::Nearest => {
                if mipmap {
                    glow::NEAREST_MIPMAP_NEAREST
                } else {
                    glow::NEAREST
                }
            }
            FilterMode::Default => glow::LINEAR,
        }
    }
}

/// 环绕模式
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WrapMode {
    /// 钳制到边缘
    Clamp,
    /// 重复
    Repeat,
    /// 镜像重复
    MirrorRepeat,
    #[default]
    Default,
}

impl WrapMode {
    /// 转换为 OpenGL 环绕模式
    pub fn to_gl(&self) -> u32 {
        match self {
            WrapMode::Clamp => glow::CLAMP_TO_EDGE,
            WrapMode::Repeat => glow::REPEAT,
            WrapMode::MirrorRepeat => glow::MIRRORED_REPEAT,
            WrapMode::Default => glow::REPEAT,
        }
    }
}

/// 纹理句柄
pub type TextureHandle = Handle<Texture2D>;

/// 采样器句柄
pub type SamplerHandle = Handle<Sampler>;

/// 采样器
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sampler {
    /// 放大过滤
    pub mag_filter: FilterMode,
    /// 缩小过滤
    pub min_filter: FilterMode,
    /// S 方向环绕
    pub wrap_s: WrapMode,
    /// T 方向环绕
    pub wrap_t: WrapMode,
    /// 各向异性过滤级别
    pub anisotropy: f32,
    /// 是否生成 mipmap
    pub mipmap: bool,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            wrap_s: WrapMode::Repeat,
            wrap_t: WrapMode::Repeat,
            anisotropy: 1.0,
            mipmap: false,
        }
    }
}

/// Sampler 构建器
pub struct SamplerBuilder {
    mag_filter: FilterMode,
    min_filter: FilterMode,
    wrap_s: WrapMode,
    wrap_t: WrapMode,
    anisotropy: f32,
    mipmap: bool,
}

impl SamplerBuilder {
    /// 创建新的采样器构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置过滤模式
    pub fn with_filter(mut self, mag: FilterMode, min: FilterMode) -> Self {
        self.mag_filter = mag;
        self.min_filter = min;
        self
    }

    /// 设置环绕模式
    pub fn with_wrap(mut self, s: WrapMode, t: WrapMode) -> Self {
        self.wrap_s = s;
        self.wrap_t = t;
        self
    }

    /// 设置各向异性过滤
    pub fn with_anisotropy(mut self, level: f32) -> Self {
        self.anisotropy = level;
        self
    }

    /// 设置 mipmap 过滤模式
    pub fn with_mipmap_filter(mut self, mode: FilterMode) -> Self {
        self.min_filter = mode;
        self.mipmap = true;
        self
    }

    /// 构建采样器
    pub fn build(&self) -> Sampler {
        Sampler {
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            wrap_s: self.wrap_s,
            wrap_t: self.wrap_t,
            anisotropy: self.anisotropy,
            mipmap: self.mipmap,
        }
    }
}

impl Default for SamplerBuilder {
    fn default() -> Self {
        Self {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            wrap_s: WrapMode::Repeat,
            wrap_t: WrapMode::Repeat,
            anisotropy: 1.0,
            mipmap: false,
        }
    }
}

/// 纹理2D
#[derive(Clone, Debug)]
pub struct Texture2D {
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
    /// 格式
    format: TextureFormat,
    /// OpenGL 纹理对象（仅在 gl 后端使用）
    #[cfg(feature = "gl")]
    gl_texture: Option<glow::Texture>,
    /// 是否使用 mipmap
    mipmap: bool,
}

impl Texture2D {
    /// 创建空纹理
    pub fn empty(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            width,
            height,
            format,
            #[cfg(feature = "gl")]
            gl_texture: None,
            mipmap: false,
        }
    }

    /// 从图像创建纹理
    pub fn from_image(image: &super::Image) -> Self {
        let format = match image.channels() {
            4 => TextureFormat::RGBA8,
            3 => TextureFormat::RGBA8, // Treat RGB as RGBA
            1 => TextureFormat::R8,
            _ => TextureFormat::RGBA8,
        };

        Self {
            width: image.width(),
            height: image.height(),
            format,
            #[cfg(feature = "gl")]
            gl_texture: None,
            mipmap: false,
        }
    }

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

    /// 获取格式
    #[inline]
    pub fn format(&self) -> TextureFormat {
        self.format
    }

    /// 设置过滤模式
    pub fn set_filter(&mut self, _filter: FilterMode) {
        // OpenGL-specific, handled in gl_backend
    }

    /// 设置环绕模式
    pub fn set_wrap(&mut self, _wrap: WrapMode) {
        // OpenGL-specific, handled in gl_backend
    }

    /// 更新纹理区域
    pub fn update(&mut self, _x: u32, _y: u32, _width: u32, _height: u32, _data: &[u8]) {
        // OpenGL-specific, handled in gl_backend
    }

    /// 生成 mipmap
    pub fn generate_mipmaps(&mut self) {
        self.mipmap = true;
    }

    /// 获取 OpenGL 纹理对象
    #[cfg(feature = "gl")]
    pub fn gl_texture(&self) -> Option<glow::Texture> {
        self.gl_texture
    }

    #[cfg(feature = "gl")]
    pub fn set_gl_texture(&mut self, tex: glow::Texture) {
        self.gl_texture = Some(tex);
    }
}

/// 纹理管理器
pub struct TextureManager {
    textures: RwLock<Vec<Arc<RwLock<Texture2D>>>>,
}

impl TextureManager {
    /// 创建新的纹理管理器
    pub fn new() -> Self {
        Self {
            textures: RwLock::new(Vec::new()),
        }
    }

    /// 添加纹理
    pub fn add(&self, texture: Texture2D) -> TextureHandle {
        let mut textures = self.textures.write();
        let index = textures.len() as u32;
        let handle = Handle::new(index, 0);
        textures.push(Arc::new(RwLock::new(texture)));
        handle
    }

    /// 获取纹理
    pub fn get(&self, handle: TextureHandle) -> Option<Arc<RwLock<Texture2D>>> {
        let textures = self.textures.read();
        textures.get(handle.index() as usize).cloned()
    }

    /// 移除纹理
    pub fn remove(&self, _handle: TextureHandle) {
        // In a real implementation, we'd also free the GPU memory
    }

    /// 迭代器
    pub fn iter(&self) -> impl Iterator<Item = Arc<RwLock<Texture2D>>> + '_ {
        self.textures.read().clone().into_iter()
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_format_bytes_per_pixel() {
        assert_eq!(TextureFormat::RGBA8.bytes_per_pixel(), 4);
        assert_eq!(TextureFormat::R8.bytes_per_pixel(), 1);
        assert_eq!(TextureFormat::RGBA16F.bytes_per_pixel(), 8);
    }

    #[test]
    fn test_filter_mode_to_gl() {
        assert_eq!(FilterMode::Linear.to_gl(false), glow::LINEAR);
        assert_eq!(FilterMode::Nearest.to_gl(false), glow::NEAREST);
        assert_eq!(FilterMode::Linear.to_gl(true), glow::LINEAR_MIPMAP_LINEAR);
        assert_eq!(
            FilterMode::Nearest.to_gl(true),
            glow::NEAREST_MIPMAP_NEAREST
        );
    }

    #[test]
    fn test_wrap_mode_to_gl() {
        assert_eq!(WrapMode::Clamp.to_gl(), glow::CLAMP_TO_EDGE);
        assert_eq!(WrapMode::Repeat.to_gl(), glow::REPEAT);
        assert_eq!(WrapMode::MirrorRepeat.to_gl(), glow::MIRRORED_REPEAT);
    }

    #[test]
    fn test_texture_empty() {
        let tex = Texture2D::empty(100, 200, TextureFormat::RGBA8);
        assert_eq!(tex.width(), 100);
        assert_eq!(tex.height(), 200);
        assert_eq!(tex.format(), TextureFormat::RGBA8);
    }

    #[test]
    fn test_sampler_builder() {
        let sampler = SamplerBuilder::new()
            .with_filter(FilterMode::Nearest, FilterMode::Nearest)
            .with_wrap(WrapMode::Clamp, WrapMode::Clamp)
            .with_anisotropy(4.0)
            .build();

        assert_eq!(sampler.mag_filter, FilterMode::Nearest);
        assert_eq!(sampler.min_filter, FilterMode::Nearest);
        assert_eq!(sampler.wrap_s, WrapMode::Clamp);
        assert_eq!(sampler.wrap_t, WrapMode::Clamp);
        assert_eq!(sampler.anisotropy, 4.0);
    }

    #[test]
    fn test_texture_manager() {
        let manager = TextureManager::new();
        let tex = Texture2D::empty(100, 100, TextureFormat::RGBA8);
        let handle = manager.add(tex);

        let retrieved = manager.get(handle);
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        let tex_read = retrieved.read();
        assert_eq!(tex_read.width(), 100);
    }
}
