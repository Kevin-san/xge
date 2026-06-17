//! 资源商店核心结构

use crate::common::*;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// 资源商店
pub struct AssetStore {
    config: AssetStoreConfig,
    installed_assets: Arc<RwLock<HashMap<AssetId, InstalledAsset>>>,
    snapshots: Arc<RwLock<HashMap<AssetId, Vec<Snapshot>>>>,
}

impl AssetStore {
    pub fn new(config: AssetStoreConfig) -> Self {
        Self {
            config,
            installed_assets: Arc::new(RwLock::new(HashMap::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn config(&self) -> &AssetStoreConfig {
        &self.config
    }

    pub fn list_installed(&self) -> Vec<InstalledAsset> {
        self.installed_assets.read().values().cloned().collect()
    }

    pub fn get_installed(&self, id: &AssetId) -> Option<InstalledAsset> {
        self.installed_assets.read().get(id).cloned()
    }

    pub fn add_installed(&self, asset: InstalledAsset) {
        self.installed_assets
            .write()
            .insert(asset.id.clone(), asset);
    }

    pub fn remove_installed(&self, id: &AssetId) -> Option<InstalledAsset> {
        self.installed_assets.write().remove(id)
    }

    pub fn has_updates(&self) -> Vec<AssetId> {
        // 收集本地已安装资源的ID和版本
        let local_versions: Vec<(AssetId, AssetVersion)> = self
            .installed_assets
            .read()
            .iter()
            .map(|(id, asset)| (id.clone(), asset.version.clone()))
            .collect();

        if local_versions.is_empty() {
            return Vec::new();
        }

        // 使用 RemoteAssetClient 检查更新
        let client = RemoteAssetClient::new(self.config.server_url.clone());
        client.check_updates(&local_versions).unwrap_or_default()
    }

    /// 创建快照
    pub fn create_snapshot(
        &self,
        asset_id: &AssetId,
        from_version: AssetVersion,
        to_version: AssetVersion,
    ) -> SnapshotId {
        let id = SnapshotId::new();
        let snapshot = Snapshot {
            id: id.clone(),
            asset_id: asset_id.clone(),
            from_version,
            to_version,
            created_at: Utc::now(),
            size_bytes: 0,
        };
        self.snapshots
            .write()
            .entry(asset_id.clone())
            .or_default()
            .push(snapshot);
        id
    }

    /// 获取快照列表
    pub fn list_snapshots(&self, asset_id: &AssetId) -> Vec<Snapshot> {
        self.snapshots
            .read()
            .get(asset_id)
            .cloned()
            .unwrap_or_default()
    }

    /// 清理旧快照
    pub fn clean_old_snapshots(&self, max_count: usize) {
        for snapshots in self.snapshots.write().values_mut() {
            if snapshots.len() > max_count {
                snapshots.truncate(max_count);
            }
        }
    }
}

impl Default for AssetStore {
    fn default() -> Self {
        Self::new(AssetStoreConfig::default())
    }
}

/// 快照 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(uuid::Uuid);

impl SnapshotId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for SnapshotId {
    fn default() -> Self {
        Self::new()
    }
}

/// 快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub asset_id: AssetId,
    pub from_version: AssetVersion,
    pub to_version: AssetVersion,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
}

/// 资源包格式 (.rgepkg)
pub struct RgePkg {
    manifest: RgePkgManifest,
    files: Vec<FileEntry>,
    signature: Option<Vec<u8>>,
}

impl RgePkg {
    pub fn new(manifest: RgePkgManifest) -> Self {
        Self {
            manifest,
            files: Vec::new(),
            signature: None,
        }
    }

    pub fn manifest(&self) -> &RgePkgManifest {
        &self.manifest
    }

    pub fn file_entries(&self) -> &[FileEntry] {
        &self.files
    }

    pub fn add_file(&mut self, entry: FileEntry) {
        self.files.push(entry);
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    pub fn sign(&mut self, signature: Vec<u8>) {
        self.signature = Some(signature);
    }
}

/// 资源包清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgePkgManifest {
    pub name: String,
    pub version: AssetVersion,
    pub author: String,
    pub engine_version: String,
    pub asset_type: AssetType,
    pub dependencies: Vec<AssetId>,
    pub checksums: HashMap<String, String>,
}

impl RgePkgManifest {
    pub fn new(name: String, version: AssetVersion, author: String) -> Self {
        Self {
            name,
            version,
            author,
            engine_version: "1.0.0".to_string(),
            asset_type: AssetType::Model3D,
            dependencies: Vec::new(),
            checksums: HashMap::new(),
        }
    }
}

/// 文件条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub sha256: String,
}

impl FileEntry {
    pub fn new(path: PathBuf, size: u64, sha256: String) -> Self {
        Self { path, size, sha256 }
    }
}

/// 校验算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChecksumAlgorithm {
    SHA256,
    SHA512,
    BLAKE3,
}

/// 依赖解析器
pub struct DependencyResolver;

impl DependencyResolver {
    pub fn resolve(deps: Vec<AssetId>) -> anyhow::Result<ResolutionGraph> {
        Ok(ResolutionGraph { resolved: deps })
    }

    pub fn detect_conflicts(_graph: &ResolutionGraph) -> Vec<Conflict> {
        Vec::new()
    }
}

/// 解析图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionGraph {
    pub resolved: Vec<AssetId>,
}

/// 冲突
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub asset_a: AssetId,
    pub asset_b: AssetId,
    pub reason: String,
}

/// 购物车
pub struct Cart {
    items: Vec<CartItem>,
}

impl Cart {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, asset_id: AssetId, name: String, price: PriceModel) {
        self.items.push(CartItem {
            asset_id,
            name,
            price,
            quantity: 1,
        });
    }

    pub fn remove(&mut self, asset_id: &AssetId) {
        self.items.retain(|item| &item.asset_id != asset_id);
    }

    pub fn items(&self) -> &[CartItem] {
        &self.items
    }

    pub fn total(&self) -> f64 {
        self.items
            .iter()
            .map(|item| match &item.price {
                PriceModel::Free => 0.0,
                PriceModel::Paid { amount, .. } => *amount,
                PriceModel::Subscription { amount, .. } => *amount,
            })
            .sum()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

/// 购物车项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub asset_id: AssetId,
    pub name: String,
    pub price: PriceModel,
    pub quantity: u32,
}

/// 评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub author: String,
    pub content: String,
    pub rating: u8,
    pub timestamp: DateTime<Utc>,
    pub helpful_votes: u32,
}

impl Comment {
    pub fn new(author: String, content: String, rating: u8) -> Self {
        Self {
            author,
            content,
            rating,
            timestamp: Utc::now(),
            helpful_votes: 0,
        }
    }
}

/// 评论 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommentId(uuid::Uuid);

impl CommentId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for CommentId {
    fn default() -> Self {
        Self::new()
    }
}

/// 评论系统
pub struct CommentSystem {
    comments: HashMap<AssetId, Vec<Comment>>,
}

impl CommentSystem {
    pub fn new() -> Self {
        Self {
            comments: HashMap::new(),
        }
    }

    pub fn post(&mut self, asset_id: AssetId, comment: Comment) -> CommentId {
        let id = CommentId::new();
        self.comments.entry(asset_id).or_default().push(comment);
        id
    }

    pub fn list(&self, asset_id: &AssetId) -> Vec<Comment> {
        self.comments.get(asset_id).cloned().unwrap_or_default()
    }

    pub fn vote(&mut self, asset_id: &AssetId, comment_idx: usize, helpful: bool) {
        if let Some(comments) = self.comments.get_mut(asset_id) {
            if let Some(comment) = comments.get_mut(comment_idx) {
                if helpful {
                    comment.helpful_votes += 1;
                } else {
                    comment.helpful_votes = comment.helpful_votes.saturating_sub(1);
                }
            }
        }
    }
}

impl Default for CommentSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Remote Asset Client (HTTP)
// ============================================================================

/// HTTP 客户端，用于与资源商店服务端通信。
/// 使用同步HTTP请求（ureq）获取资源列表、检查更新和下载资源。
pub struct RemoteAssetClient {
    base_url: String,
    /// 连接超时（毫秒）
    timeout_ms: u64,
    /// API 密钥（可选）
    api_key: Option<String>,
}

impl RemoteAssetClient {
    /// 创建新的远程资源客户端
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            timeout_ms: 10000,
            api_key: None,
        }
    }

    /// 设置超时
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// 设置 API 密钥
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// 检查服务端是否可达
    pub fn ping(&self) -> anyhow::Result<()> {
        let client = ureq::Agent::new();
        let url = format!("{}/ping", self.base_url);
        let mut req = client.get(&url).timeout(std::time::Duration::from_millis(self.timeout_ms));
        if let Some(ref key) = self.api_key {
            req = req.set("Authorization", &format!("Bearer {}", key));
        }
        let response = req.call()?;
        if response.status() < 400 {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Server returned {}",
                response.status()
            ))
        }
    }

    /// 获取资源列表
    pub fn list_assets(&self) -> anyhow::Result<Vec<AssetSummary>> {
        let client = ureq::Agent::new();
        let url = format!("{}/api/v1/assets", self.base_url);
        let mut req = client.get(&url).timeout(std::time::Duration::from_millis(self.timeout_ms));
        if let Some(ref key) = self.api_key {
            req = req.set("Authorization", &format!("Bearer {}", key));
        }
        let response = req.call()?;
        if response.status() >= 400 {
            return Err(anyhow::anyhow!(
                "Server returned {}",
                response.status()
            ));
        }
        let assets: Vec<AssetSummary> = response.into_json()?;
        Ok(assets)
    }

    /// 检查资源是否有更新
    pub fn check_updates(&self, local_versions: &[(AssetId, AssetVersion)]) -> anyhow::Result<Vec<AssetId>> {
        let client = ureq::Agent::new();
        let url = format!("{}/api/v1/updates", self.base_url);
        let mut req = client
            .post(&url)
            .timeout(std::time::Duration::from_millis(self.timeout_ms));
        if let Some(ref key) = self.api_key {
            req = req.set("Authorization", &format!("Bearer {}", key));
        }
        let response = req.send_json(local_versions)?;
        if response.status() >= 400 {
            return Err(anyhow::anyhow!(
                "Server returned {}",
                response.status()
            ));
        }
        let updates: Vec<AssetId> = response.into_json()?;
        Ok(updates)
    }

    /// 下载资源到指定目录
    pub fn download_asset(&self, asset_id: &AssetId, dest_dir: &std::path::Path) -> anyhow::Result<()> {
        let client = ureq::Agent::new();
        let url = format!("{}/api/v1/assets/{}/download", self.base_url, asset_id);
        let mut req = client.get(&url).timeout(std::time::Duration::from_millis(self.timeout_ms * 10)); // 下载超时更长
        if let Some(ref key) = self.api_key {
            req = req.set("Authorization", &format!("Bearer {}", key));
        }
        let response = req.call()?;
        if response.status() >= 400 {
            return Err(anyhow::anyhow!(
                "Download failed: {}",
                response.status()
            ));
        }

        // 确保目标目录存在
        if let Some(parent) = dest_dir.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        // 将响应体写入文件
        let mut dest_path = PathBuf::from(dest_dir);
        dest_path.push(format!("{}.rgepkg", asset_id));
        let mut file = std::fs::File::create(&dest_path)?;
        std::io::copy(&mut response.into_reader(), &mut file)?;
        Ok(())
    }

    /// 获取服务端点
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
