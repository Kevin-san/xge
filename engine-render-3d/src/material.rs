//! 材质模块
//!
//! 提供 Material3D 和 MaterialManager3D。

use engine_math::Vec4;
use engine_utils::Handle;

use crate::mesh::Mesh3D;

/// 3D 材质
///
/// 包含基础颜色、纹理、光照参数等。
#[derive(Debug, Clone)]
pub struct Material3D {
    /// 材质名称
    name: String,
    /// 基础颜色
    base_color: Vec4,
    /// 金属度
    metallic: f32,
    /// 粗糙度
    roughness: f32,
    /// 环境光系数
    ambient: Vec4,
    /// 高光系数
    shininess: f32,
    /// 漫反射颜色
    diffuse: Vec4,
    /// 自发光颜色
    emissive: Vec4,
    /// 漫反射贴图索引
    diffuse_texture: Option<Handle<Texture2D>>,
    /// 法线贴图索引
    normal_texture: Option<Handle<Texture2D>>,
    /// 是否启用光照
    lit: bool,
    /// 是否双面渲染
    double_sided: bool,
}

impl Material3D {
    /// 创建默认材质
    pub fn new() -> Self {
        Self {
            name: String::new(),
            base_color: Vec4::new(0.8, 0.8, 0.8, 1.0),
            metallic: 0.0,
            roughness: 0.5,
            ambient: Vec4::new(0.1, 0.1, 0.1, 1.0),
            shininess: 32.0,
            diffuse: Vec4::new(0.8, 0.8, 0.8, 1.0),
            emissive: Vec4::ZERO,
            diffuse_texture: None,
            normal_texture: None,
            lit: true,
            double_sided: false,
        }
    }

    /// 创建单色材质
    pub fn from_color(color: Vec4) -> Self {
        let mut m = Self::new();
        m.base_color = color;
        m.diffuse = color;
        m
    }

    /// 创建带名称的材质
    pub fn with_name(name: &str) -> Self {
        let mut m = Self::new();
        m.name = name.to_string();
        m
    }

    /// 获取名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 设置名称
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// 获取基础颜色
    pub fn base_color(&self) -> Vec4 {
        self.base_color
    }

    /// 设置基础颜色
    pub fn set_base_color(&mut self, color: Vec4) {
        self.base_color = color;
        self.diffuse = color;
    }

    /// 获取金属度
    pub fn metallic(&self) -> f32 {
        self.metallic
    }

    /// 设置金属度
    pub fn set_metallic(&mut self, value: f32) {
        self.metallic = value.clamp(0.0, 1.0);
    }

    /// 获取粗糙度
    pub fn roughness(&self) -> f32 {
        self.roughness
    }

    /// 设置粗糙度
    pub fn set_roughness(&mut self, value: f32) {
        self.roughness = value.clamp(0.0, 1.0);
    }

    /// 获取环境光
    pub fn ambient(&self) -> Vec4 {
        self.ambient
    }

    /// 设置环境光
    pub fn set_ambient(&mut self, color: Vec4) {
        self.ambient = color;
    }

    /// 获取高光系数
    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    /// 设置高光系数
    pub fn set_shininess(&mut self, value: f32) {
        self.shininess = value.max(0.0);
    }

    /// 获取漫反射颜色
    pub fn diffuse(&self) -> Vec4 {
        self.diffuse
    }

    /// 设置漫反射颜色
    pub fn set_diffuse(&mut self, color: Vec4) {
        self.diffuse = color;
    }

    /// 获取自发光颜色
    pub fn emissive(&self) -> Vec4 {
        self.emissive
    }

    /// 设置自发光颜色
    pub fn set_emissive(&mut self, color: Vec4) {
        self.emissive = color;
    }

    /// 获取漫反射贴图
    pub fn diffuse_texture(&self) -> Option<Handle<Texture2D>> {
        self.diffuse_texture.clone()
    }

    /// 设置漫反射贴图
    pub fn set_diffuse_texture(&mut self, handle: Option<Handle<Texture2D>>) {
        self.diffuse_texture = handle;
    }

    /// 获取法线贴图
    pub fn normal_texture(&self) -> Option<Handle<Texture2D>> {
        self.normal_texture.clone()
    }

    /// 设置法线贴图
    pub fn set_normal_texture(&mut self, handle: Option<Handle<Texture2D>>) {
        self.normal_texture = handle;
    }

    /// 是否启用光照
    pub fn lit(&self) -> bool {
        self.lit
    }

    /// 设置是否启用光照
    pub fn set_lit(&mut self, lit: bool) {
        self.lit = lit;
    }

    /// 是否双面
    pub fn double_sided(&self) -> bool {
        self.double_sided
    }

    /// 设置双面
    pub fn set_double_sided(&mut self, double: bool) {
        self.double_sided = double;
    }

    /// 验证 PBR 参数是否在合理范围
    pub fn is_pbr_valid(&self) -> bool {
        self.metallic >= 0.0 && self.metallic <= 1.0
            && self.roughness >= 0.0
            && self.roughness <= 1.0
    }
}

impl Default for Material3D {
    fn default() -> Self {
        Self::new()
    }
}

/// 2D 纹理占位类型
#[derive(Debug, Clone)]
pub struct Texture2D {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    format: TextureFormat,
}

impl Texture2D {
    /// 创建空纹理
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        let pixel_size = format.pixel_size();
        Self {
            width,
            height,
            pixels: vec![0; (width * height) as usize * pixel_size],
            format,
        }
    }

    /// 从像素数据创建
    pub fn from_pixels(width: u32, height: u32, format: TextureFormat, pixels: Vec<u8>) -> Self {
        Self { width, height, pixels, format }
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 获取格式
    pub fn format(&self) -> TextureFormat {
        self.format
    }

    /// 获取像素数据
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

/// 纹理格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    /// RGBA 8 位
    Rgba8,
    /// RGB 8 位
    Rgb8,
    /// 灰度 8 位
    R8,
    /// 灰度+Alpha 8 位
    Rg8,
}

impl TextureFormat {
    /// 获取单像素字节数
    pub fn pixel_size(self) -> usize {
        match self {
            TextureFormat::Rgba8 => 4,
            TextureFormat::Rgb8 => 3,
            TextureFormat::R8 => 1,
            TextureFormat::Rg8 => 2,
        }
    }
}

/// 默认错误材质
pub fn default_error_material() -> Material3D {
    let mut m = Material3D::with_name("error");
    m.set_base_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
    m
}

/// 材质管理器
pub struct MaterialManager3D {
    materials: Vec<Material3D>,
}

impl MaterialManager3D {
    /// 创建新管理器
    pub fn new() -> Self {
        Self { materials: Vec::new() }
    }

    /// 添加材质
    pub fn add(&mut self, material: Material3D) -> Handle<Material3D> {
        let handle = Handle::new(self.materials.len() as u32, 0);
        self.materials.push(material);
        handle
    }

    /// 获取材质
    pub fn get(&self, handle: Handle<Material3D>) -> Option<&Material3D> {
        self.materials.get(handle.index() as usize)
    }

    /// 获取材质可变引用
    pub fn get_mut(&mut self, handle: Handle<Material3D>) -> Option<&mut Material3D> {
        self.materials.get_mut(handle.index() as usize)
    }

    /// 数量
    pub fn len(&self) -> usize {
        self.materials.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }
}

impl Default for MaterialManager3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_default() {
        let m = Material3D::new();
        assert_eq!(m.metallic(), 0.0);
        assert_eq!(m.roughness(), 0.5);
    }

    #[test]
    fn test_material_pbr_clamp() {
        let mut m = Material3D::new();
        m.set_metallic(2.0);
        assert_eq!(m.metallic(), 1.0);
        m.set_metallic(-0.5);
        assert_eq!(m.metallic(), 0.0);
    }

    #[test]
    fn test_material_manager() {
        let mut manager = MaterialManager3D::new();
        let m = Material3D::from_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
        let h = manager.add(m);
        assert!(manager.get(h).is_some());
    }
}

// 避免 Mesh3D 警告未使用
#[allow(dead_code)]
fn _force_use_mesh(_m: &Mesh3D) {}
