//! TextureManager 模块 - 纹理资源集中管理
//!
//! 提供 HashMap 驱动的 TextureManager，支持纹理的注册、查询、卸载和迭代。

use std::collections::HashMap;
use crate::{Texture2D, TextureHandle};

/// 纹理管理器 — 集中管理纹理资源的加载、查询和卸载
pub struct TextureManager {
    textures: HashMap<TextureHandle, Texture2D>,
    next_id: u64,
}

impl TextureManager {
    /// 创建新的纹理管理器
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            next_id: 1,
        }
    }

    /// 注册纹理，返回句柄
    pub fn insert(&mut self, texture: Texture2D) -> TextureHandle {
        let handle = TextureHandle::new(self.next_id as u32, 0);
        self.next_id += 1;
        self.textures.insert(handle.clone(), texture);
        handle
    }

    /// 获取纹理引用
    pub fn get(&self, handle: &TextureHandle) -> Option<&Texture2D> {
        self.textures.get(handle)
    }

    /// 获取纹理可变引用
    pub fn get_mut(&mut self, handle: &TextureHandle) -> Option<&mut Texture2D> {
        self.textures.get_mut(handle)
    }

    /// 卸载纹理
    pub fn unload(&mut self, handle: &TextureHandle) {
        self.textures.remove(handle);
    }

    /// 获取已加载纹理数量
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// 迭代所有纹理
    pub fn iter(&self) -> impl Iterator<Item = (TextureHandle, &Texture2D)> {
        self.textures.iter().map(|(h, t)| (h.clone(), t))
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
    use crate::{TextureFormat};

    #[test]
    fn test_texture_manager_new() {
        let mgr = TextureManager::new();
        assert!(mgr.is_empty());
        assert_eq!(mgr.len(), 0);
    }

    #[test]
    fn test_texture_manager_insert_and_get() {
        let mut mgr = TextureManager::new();
        let tex = Texture2D::empty(256, 256, TextureFormat::RGBA8);
        let handle = mgr.insert(tex);
        assert!(mgr.get(&handle).is_some());
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_texture_manager_unload() {
        let mut mgr = TextureManager::new();
        let tex = Texture2D::empty(256, 256, TextureFormat::RGBA8);
        let handle = mgr.insert(tex);
        mgr.unload(&handle);
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_texture_manager_iter() {
        let mut mgr = TextureManager::new();
        let tex1 = Texture2D::empty(256, 256, TextureFormat::RGBA8);
        let tex2 = Texture2D::empty(512, 512, TextureFormat::RGBA8);
        mgr.insert(tex1);
        mgr.insert(tex2);
        assert_eq!(mgr.iter().count(), 2);
    }

    #[test]
    fn test_texture_manager_get_mut() {
        let mut mgr = TextureManager::new();
        let tex = Texture2D::empty(256, 256, TextureFormat::RGBA8);
        let handle = mgr.insert(tex);
        if let Some(t) = mgr.get_mut(&handle) {
            t.generate_mipmaps();
        }
        assert!(mgr.get(&handle).is_some());
    }

    #[test]
    fn test_texture_manager_default() {
        let mgr: TextureManager = Default::default();
        assert!(mgr.is_empty());
    }
}
