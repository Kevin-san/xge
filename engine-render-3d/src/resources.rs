//! 资源管理模块
//!
//! 提供 MeshManager（网格管理）和 AssetLoader（资源加载）。

use engine_utils::Handle;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::material::{Material3D, Texture2D, TextureFormat};
use crate::mesh::Mesh3D;

#[cfg(feature = "gltf-loader")]
use crate::gltf_loader::{GltfLoadOptions, GltfModel};

/// 资源句柄
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetHandle(Handle<u32>);

impl AssetHandle {
    /// 获取索引
    pub fn index(&self) -> u32 {
        self.0.index()
    }
}

/// 网格资源
#[derive(Debug, Clone)]
pub struct MeshAsset {
    pub mesh: Mesh3D,
    pub path: String,
    pub last_modified: SystemTime,
}

/// 材质资源
#[derive(Debug, Clone)]
pub struct MaterialAsset {
    pub material: Material3D,
    pub path: String,
}

/// 纹理资源
#[derive(Debug, Clone)]
pub struct TextureAsset {
    pub texture: Texture2D,
    pub path: String,
}

/// 资源统计
#[derive(Debug, Clone, Default)]
pub struct AssetStats {
    pub meshes_loaded: usize,
    pub materials_loaded: usize,
    pub textures_loaded: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

/// 网格资源管理器
pub struct MeshManager {
    meshes: Vec<Mesh3D>,
    /// 路径到句柄的映射
    path_map: HashMap<String, AssetHandle>,
    /// 网格资源（包含元数据）
    assets: HashMap<AssetHandle, MeshAsset>,
    /// LRU 缓存顺序
    access_order: Vec<AssetHandle>,
    /// 最大缓存数量
    max_cache_size: usize,
    /// 统计信息
    stats: AssetStats,
}

impl MeshManager {
    /// 创建新管理器
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            path_map: HashMap::new(),
            assets: HashMap::new(),
            access_order: Vec::new(),
            max_cache_size: 64,
            stats: AssetStats::default(),
        }
    }

    /// 设置最大缓存大小
    pub fn set_max_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;
    }

    /// 加载网格（从文件）
    pub fn load(&mut self, path: &str) -> AssetHandle {
        // 检查缓存
        let cached = self.path_map.get(path).cloned();
        if let Some(handle) = cached {
            self.stats.cache_hits += 1;
            self.touch(handle.clone());
            return handle;
        }

        self.stats.cache_misses += 1;

        // 实际加载
        let mesh = match Mesh3D::from_file(path) {
            Ok(m) => m,
            Err(_) => {
                // 加载失败时使用错误网格
                Mesh3D::cube(1.0)
            }
        };

        self.add(mesh, path.to_string())
    }

    /// 直接添加网格
    pub fn add(&mut self, mesh: Mesh3D, path: String) -> AssetHandle {
        let internal_handle = Handle::new(self.meshes.len() as u32, 0);
        self.meshes.push(mesh.clone());

        let asset_handle = AssetHandle(internal_handle);
        let asset = MeshAsset {
            mesh,
            path: path.clone(),
            last_modified: SystemTime::now(),
        };
        self.assets.insert(asset_handle.clone(), asset);
        self.path_map.insert(path, asset_handle.clone());
        self.access_order.push(asset_handle.clone());

        self.stats.meshes_loaded += 1;
        self.evict_if_needed();

        asset_handle
    }

    /// 获取网格
    pub fn get(&self, handle: &AssetHandle) -> Option<&Mesh3D> {
        self.meshes.get(handle.index() as usize)
    }

    /// 通过路径获取
    pub fn get_by_path(&self, path: &str) -> Option<&Mesh3D> {
        self.path_map.get(path).and_then(|h| self.get(h))
    }

    /// 重新加载资源
    pub fn reload(&mut self, path: &str) -> bool {
        if let Some(handle) = self.path_map.get(path) {
            let handle = handle.clone();
            let new_mesh = match Mesh3D::from_file(path) {
                Ok(m) => m,
                Err(_) => return false,
            };

            if let Some(slot) = self.meshes.get_mut(handle.index() as usize) {
                *slot = new_mesh.clone();
            }
            if let Some(asset) = self.assets.get_mut(&handle) {
                asset.mesh = new_mesh;
                asset.last_modified = SystemTime::now();
            }
            true
        } else {
            false
        }
    }

    /// 数量
    pub fn len(&self) -> usize {
        self.meshes.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }

    /// 统计信息
    pub fn stats(&self) -> &AssetStats {
        &self.stats
    }

    /// 重置统计
    pub fn reset_stats(&mut self) {
        self.stats = AssetStats::default();
    }

    /// 触摸句柄（更新 LRU 顺序）
    fn touch(&mut self, handle: AssetHandle) {
        self.access_order.retain(|h| h != &handle);
        self.access_order.push(handle);
    }

    /// LRU 淘汰
    fn evict_if_needed(&mut self) {
        while self.meshes.len() > self.max_cache_size && self.access_order.len() > 1 {
            if let Some(oldest) = self.access_order.first().cloned() {
                if self.access_order.len() <= 1 {
                    break;
                }
                self.access_order.remove(0);
                if let Some(asset) = self.assets.remove(&oldest) {
                    self.path_map.remove(&asset.path);
                    // 标记 mesh 为空
                    if let Some(slot) = self.meshes.get_mut(oldest.index() as usize) {
                        *slot = Mesh3D::new();
                    }
                }
            } else {
                break;
            }
        }
    }
}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh3D {
    /// 从文件加载
    #[cfg(feature = "gltf-loader")]
    pub fn from_file(path: &str) -> Result<Self, String> {
        GltfModel::from_file(path)
            .map(|model| {
                model.meshes.into_iter().next().unwrap_or_else(Mesh3D::new)
            })
            .map_err(|e| e.to_string())
    }

    /// 占位实现
    #[cfg(not(feature = "gltf-loader"))]
    pub fn from_file(_path: &str) -> Result<Self, String> {
        Err("GLTF loader feature not enabled".to_string())
    }
}

// 扩展 Mesh3D 处理 GltfLoadOptions
#[cfg(feature = "gltf-loader")]
trait MeshManagerExt {
    fn load_with_options(&mut self, path: &str, options: &GltfLoadOptions) -> AssetHandle;
}

#[cfg(feature = "gltf-loader")]
impl MeshManagerExt for MeshManager {
    fn load_with_options(&mut self, path: &str, options: &GltfLoadOptions) -> AssetHandle {
        // 检查缓存
        if let Some(handle) = self.path_map.get(path) {
            self.stats.cache_hits += 1;
            return handle.clone();
        }

        self.stats.cache_misses += 1;

        let mesh = match GltfModel::from_file_with_options(path, options) {
            Ok(model) => model.meshes.into_iter().next().unwrap_or_else(Mesh3D::new),
            Err(_) => Mesh3D::cube(1.0),
        };

        self.add(mesh, path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_manager_basic() {
        let mut manager = MeshManager::new();
        let cube = Mesh3D::cube(1.0);
        let h1 = manager.add(cube, "test_cube".to_string());
        let h2 = manager.add(Mesh3D::sphere(1.0, 8, 4), "test_sphere".to_string());

        assert_eq!(manager.len(), 2);
        assert!(manager.get(&h1).is_some());
        assert!(manager.get(&h2).is_some());
    }

    #[test]
    fn test_mesh_manager_cache() {
        let mut manager = MeshManager::new();
        manager.set_max_cache_size(2);
        let h1 = manager.add(Mesh3D::cube(1.0), "a".to_string());
        let _h2 = manager.add(Mesh3D::cube(1.0), "b".to_string());
        let _h3 = manager.add(Mesh3D::cube(1.0), "c".to_string());

        // 触达 h1，更新访问顺序
        manager.touch(h1.clone());

        // h1 仍可访问
        assert!(manager.get(&h1).is_some());
    }

    #[test]
    fn test_texture_loading() {
        let tex = Texture2D::new(64, 64, TextureFormat::Rgba8);
        assert_eq!(tex.width(), 64);
        assert_eq!(tex.height(), 64);
    }
}
