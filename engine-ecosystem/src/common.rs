//! 公共类型和工具函数

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// 资源 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(Uuid);

impl AssetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let uuid = Uuid::parse_str(s)?;
        Ok(Self(uuid))
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for AssetId {
    fn default() -> Self {
        Self::new()
    }
}

/// 资源版本（语义化版本）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AssetVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl AssetVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid version format"));
        }
        Ok(Self {
            major: parts[0].parse()?,
            minor: parts[1].parse()?,
            patch: parts[2].parse()?,
        })
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> u32 {
        self.patch
    }
}

impl std::fmt::Display for AssetVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for AssetVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Texture2D,
    Texture3D,
    CubeMap,
    Model3D,
    Material,
    Sound,
    Scene,
    Script,
    Plugin,
    FullProject,
    Shader,
    UIKit,
    ParticlePack,
    PostFXPack,
}

/// 资源许可证类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetLicense {
    MIT,
    Apache2,
    GPLv3,
    Proprietary,
    CreativeCommons,
    Custom(String),
}

/// 定价模型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PriceModel {
    Free,
    Paid { amount: f64, currency: String },
    Subscription { amount: f64, currency: String, period: String },
}

/// 资源分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetCategory {
    All,
    Art2D,
    Models3D,
    Materials,
    Audio,
    Scenes,
    Scripts,
    Plugins,
    Templates,
    Shaders,
    UIKits,
    ParticlePacks,
    PostFX,
    FullProjects,
}

/// 资源评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRating {
    stars: f32,
    review_count: usize,
}

impl AssetRating {
    pub fn new(stars: f32, review_count: usize) -> Self {
        Self { stars, review_count }
    }

    pub fn stars(&self) -> f32 {
        self.stars
    }

    pub fn review_count(&self) -> usize {
        self.review_count
    }
}

impl Default for AssetRating {
    fn default() -> Self {
        Self::new(0.0, 0)
    }
}

/// 资源摘要信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSummary {
    pub id: AssetId,
    pub name: String,
    pub author: String,
    pub version: AssetVersion,
    pub tags: Vec<String>,
    pub category: AssetCategory,
    pub rating: AssetRating,
    pub downloads: u64,
    pub price: PriceModel,
    pub license: AssetLicense,
    pub thumbnail_url: String,
}

/// 资源详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDetail {
    pub summary: AssetSummary,
    pub description: String,
    pub screenshots: Vec<String>,
    pub videos: Vec<String>,
    pub dependencies: Vec<AssetId>,
    pub changelog: String,
}

/// 已安装资源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledAsset {
    pub id: AssetId,
    pub name: String,
    pub version: AssetVersion,
    pub install_path: PathBuf,
    pub install_time: DateTime<Utc>,
    pub files: Vec<PathBuf>,
}

impl InstalledAsset {
    pub fn total_size_bytes(&self) -> u64 {
        self.files.iter().filter_map(|f| f.metadata().ok().map(|m| m.len())).sum()
    }

    pub fn installed_files(&self) -> &[PathBuf] {
        &self.files
    }
}

/// 商店配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStoreConfig {
    pub server_url: String,
    pub cache_dir: PathBuf,
    pub install_dir: PathBuf,
}

impl AssetStoreConfig {
    pub fn new(server_url: String, cache_dir: PathBuf, install_dir: PathBuf) -> Self {
        Self { server_url, cache_dir, install_dir }
    }

    pub fn server_url(&self) -> &str {
        &self.server_url
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    pub fn install_dir(&self) -> &PathBuf {
        &self.install_dir
    }
}

impl Default for AssetStoreConfig {
    fn default() -> Self {
        Self::new(
            crate::ASSET_STORE_ENDPOINT.to_string(),
            PathBuf::from(crate::DEFAULT_CACHE_DIR),
            PathBuf::from(crate::DEFAULT_INSTALL_DIR),
        )
    }
}

/// 搜索过滤器
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFilters {
    pub categories: Vec<AssetCategory>,
    pub min_rating: Option<f32>,
    pub max_price: Option<f64>,
    pub license_types: Vec<AssetLicense>,
}

/// 排序顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Relevance,
    Rating,
    Downloads,
    Newest,
    PriceAsc,
    PriceDesc,
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paged<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub total_items: u64,
}

/// 时间戳工具函数
pub fn current_timestamp_ns() -> u64 {
    let now = Utc::now();
    now.timestamp_nanos_opt().unwrap_or(0) as u64
}

/// 线程 ID 工具函数
pub fn current_thread_id() -> u64 {
    // 使用简化的线程 ID 实现
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}