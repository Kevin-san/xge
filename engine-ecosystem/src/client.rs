//! 资源商店客户端

use crate::asset_store::*;
use crate::common::*;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

/// 资源商店客户端
pub struct AssetStoreClient {
    config: AssetStoreConfig,
    auth_token: Option<AuthToken>,
    user_profile: Option<UserProfile>,
    store: Arc<AssetStore>,
    cart: Arc<RwLock<Cart>>,
    downloads: Arc<RwLock<HashMap<AssetId, DownloadTask>>>,
}

impl AssetStoreClient {
    pub fn new(config: AssetStoreConfig) -> Self {
        let store_config = config.clone();
        Self {
            config,
            auth_token: None,
            user_profile: None,
            store: Arc::new(AssetStore::new(store_config)),
            cart: Arc::new(RwLock::new(Cart::new())),
            downloads: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn config(&self) -> &AssetStoreConfig {
        &self.config
    }

    /// 登录
    pub fn login(&mut self, username: &str, _password: &str) -> Result<(), AssetStoreError> {
        // 简化实现，实际需要 HTTP 请求
        self.auth_token = Some(AuthToken::new("mock_token".to_string()));
        self.user_profile = Some(UserProfile {
            username: username.to_string(),
            email: "user@example.com".to_string(),
            display_name: username.to_string(),
            avatar_url: None,
            joined_at: Utc::now(),
        });
        Ok(())
    }

    /// 使用 Token 登录
    pub fn login_with_token(&mut self, token: &str) -> Result<(), AssetStoreError> {
        self.auth_token = Some(AuthToken::new(token.to_string()));
        Ok(())
    }

    /// 登出
    pub fn logout(&mut self) {
        self.auth_token = None;
        self.user_profile = None;
    }

    /// 是否已登录
    pub fn is_logged_in(&self) -> bool {
        self.auth_token.is_some()
    }

    /// 刷新 Token
    pub fn refresh_token(&mut self) -> Result<(), AssetStoreError> {
        if let Some(token) = &self.auth_token {
            self.auth_token = Some(token.refresh());
        }
        Ok(())
    }

    /// 获取用户信息
    pub fn me(&self) -> Result<UserProfile, AssetStoreError> {
        self.user_profile.clone().ok_or(AssetStoreError::Auth("Not logged in".to_string()))
    }

    /// 搜索资源
    pub fn search(&self, keyword: &str, _filters: SearchFilters) -> Result<Vec<AssetSummary>, AssetStoreError> {
        // 简化实现，返回模拟数据
        Ok(vec![AssetSummary {
            id: AssetId::new(),
            name: keyword.to_string(),
            author: "Mock Author".to_string(),
            version: AssetVersion::default(),
            tags: vec!["test".to_string()],
            category: AssetCategory::Models3D,
            rating: AssetRating::default(),
            downloads: 100,
            price: PriceModel::Free,
            license: AssetLicense::MIT,
            thumbnail_url: "https://example.com/thumb.png".to_string(),
        }])
    }

    /// 浏览分类
    pub fn browse(&self, _category: AssetCategory, _page: u32, _page_size: u32) -> Result<Vec<AssetSummary>, AssetStoreError> {
        // 简化实现
        Ok(Vec::new())
    }

    /// 分页搜索
    pub fn search_with_pagination(&self, query: SearchQuery, page: u32, page_size: u32) -> Result<Paged<AssetSummary>, AssetStoreError> {
        let items = self.search(&query.keyword, SearchFilters::default())?;
        let total_items = items.len() as u64;
        Ok(Paged {
            items,
            page,
            page_size,
            total_pages: 1,
            total_items,
        })
    }

    /// 获取资源详情
    pub fn get_asset(&self, id: &AssetId) -> Result<AssetDetail, AssetStoreError> {
        Ok(AssetDetail {
            summary: AssetSummary {
                id: id.clone(),
                name: "Mock Asset".to_string(),
                author: "Mock Author".to_string(),
                version: AssetVersion::default(),
                tags: vec!["test".to_string()],
                category: AssetCategory::Models3D,
                rating: AssetRating::default(),
                downloads: 100,
                price: PriceModel::Free,
                license: AssetLicense::MIT,
                thumbnail_url: "https://example.com/thumb.png".to_string(),
            },
            description: "Mock description".to_string(),
            screenshots: vec!["https://example.com/screenshot.png".to_string()],
            videos: vec!["https://example.com/video.mp4".to_string()],
            dependencies: Vec::new(),
            changelog: "v1.0.0 - Initial release".to_string(),
        })
    }

    /// 获取热门资源
    pub fn trending(&self, _limit: usize) -> Vec<AssetSummary> {
        Vec::new()
    }

    /// 获取推荐资源
    pub fn featured(&self, _limit: usize) -> Vec<AssetSummary> {
        Vec::new()
    }

    /// 获取最新资源
    pub fn new_releases(&self, _limit: usize) -> Vec<AssetSummary> {
        Vec::new()
    }

    /// 获取高分资源
    pub fn top_rated(&self, _limit: usize) -> Vec<AssetSummary> {
        Vec::new()
    }

    /// 获取下载最多资源
    pub fn most_downloaded(&self, _limit: usize) -> Vec<AssetSummary> {
        Vec::new()
    }

    /// 获取相关资源
    pub fn related_assets(&self, _id: &AssetId, _limit: usize) -> Vec<AssetSummary> {
        Vec::new()
    }

    /// 下载资源
    pub fn download(&mut self, id: &AssetId) -> Result<PathBuf, AssetStoreError> {
        let task = DownloadTask {
            asset_id: id.clone(),
            state: DownloadState::Downloading(DownloadProgress {
                bytes_downloaded: 0,
                bytes_total: 1000,
                speed_kbps: 100.0,
                eta_seconds: 10,
            }),
            start_time: Utc::now(),
        };
        self.downloads.write().insert(id.clone(), task);

        // 模拟下载完成
        let path = self.config.cache_dir.join(format!("{}.rgepkg", id));
        Ok(path)
    }

    /// 取消下载
    pub fn cancel_download(&mut self, id: &AssetId) -> bool {
        self.downloads.write().remove(id).is_some()
    }

    /// 暂停下载
    pub fn pause_download(&mut self, id: &AssetId) -> bool {
        if let Some(task) = self.downloads.write().get_mut(id) {
            task.state = DownloadState::Idle;
            true
        } else {
            false
        }
    }

    /// 恢复下载
    pub fn resume_download(&mut self, id: &AssetId) -> bool {
        if let Some(task) = self.downloads.write().get_mut(id) {
            task.state = DownloadState::Downloading(DownloadProgress {
                bytes_downloaded: 0,
                bytes_total: 1000,
                speed_kbps: 100.0,
                eta_seconds: 10,
            });
            true
        } else {
            false
        }
    }

    /// 获取下载任务列表
    pub fn download_tasks(&self) -> Vec<DownloadTask> {
        self.downloads.read().values().cloned().collect()
    }

    /// 安装资源
    pub fn install(&mut self, _downloaded_path: &Path, target_dir: &Path) -> Result<InstalledAsset, AssetStoreError> {
        let asset = InstalledAsset {
            id: AssetId::new(),
            name: "Mock Asset".to_string(),
            version: AssetVersion::default(),
            install_path: target_dir.to_path_buf(),
            install_time: Utc::now(),
            files: vec![target_dir.join("asset.bin")],
        };
        self.store.add_installed(asset.clone());
        Ok(asset)
    }

    /// 更新资源
    pub fn update(&mut self, id: &AssetId) -> Result<InstalledAsset, AssetStoreError> {
        if let Some(asset) = self.store.get_installed(id) {
            let updated = InstalledAsset {
                id: asset.id.clone(),
                name: asset.name.clone(),
                version: AssetVersion::new(2, 0, 0),
                install_path: asset.install_path.clone(),
                install_time: Utc::now(),
                files: asset.files.clone(),
            };
            self.store.add_installed(updated.clone());
            Ok(updated)
        } else {
            Err(AssetStoreError::NotFound)
        }
    }

    /// 卸载资源
    pub fn uninstall(&mut self, id: &AssetId) -> Result<(), AssetStoreError> {
        self.store.remove_installed(id);
        Ok(())
    }

    /// 列出已安装资源
    pub fn list_installed(&self) -> Vec<InstalledAsset> {
        self.store.list_installed()
    }

    /// 检查更新
    pub fn has_updates(&self) -> Vec<AssetId> {
        self.store.has_updates()
    }

    /// 回滚到指定版本
    pub fn rollback(&mut self, _id: &AssetId, _version: AssetVersion) -> Result<(), AssetStoreError> {
        // 简化实现
        Ok(())
    }

    /// 添加到购物车
    pub fn add_to_cart(&mut self, asset_id: AssetId, name: String, price: PriceModel) {
        self.cart.write().add(asset_id, name, price);
    }

    /// 从购物车移除
    pub fn remove_from_cart(&mut self, asset_id: &AssetId) {
        self.cart.write().remove(asset_id);
    }

    /// 获取购物车内容
    pub fn cart_items(&self) -> Vec<CartItem> {
        self.cart.read().items().to_vec()
    }

    /// 获取购物车总价
    pub fn cart_total(&self) -> f64 {
        self.cart.read().total()
    }

    /// 结账
    pub fn checkout(&mut self, _payment_method: PaymentMethod) -> Result<Order, AssetStoreError> {
        let items = self.cart.read().items().to_vec();
        let total = self.cart.read().total();

        let order = Order {
            id: OrderId::new(),
            items,
            total: Money::new(total, "USD".to_string()),
            status: OrderStatus::Paid,
            created_at: Utc::now(),
            payment_id: "payment_123".to_string(),
        };

        self.cart.write().clear();
        Ok(order)
    }

    /// 获取订单列表
    pub fn orders(&self, page: u32) -> Result<Paged<Order>, AssetStoreError> {
        Ok(Paged {
            items: Vec::new(),
            page,
            page_size: 10,
            total_pages: 0,
            total_items: 0,
        })
    }

    /// 获取订单详情
    pub fn order_detail(&self, _id: &OrderId) -> Result<Order, AssetStoreError> {
        Err(AssetStoreError::NotFound)
    }

    /// 获取我的资源
    pub fn my_assets(&self) -> Vec<OwnedAsset> {
        Vec::new()
    }

    /// 检查兼容性
    pub fn is_compatible(&self, _asset: &AssetDetail, _engine_version: &str, _platform: PlatformFlag) -> bool {
        true // 简化实现
    }

    /// 获取速率限制状态
    pub fn rate_limit_status(&self) -> RateLimitStatus {
        RateLimitStatus {
            requests_remaining: 100,
            reset_timestamp: Utc::now() + chrono::Duration::hours(1),
        }
    }

    /// 设置超时
    pub fn set_timeout(&mut self, _seconds: u64) {
        // 简化实现
    }

    /// 设置重试策略
    pub fn set_retry_policy(&mut self, _max_attempts: u32, _backoff_ms: u64) {
        // 简化实现
    }

    /// 获取 User Agent
    pub fn user_agent(&self) -> String {
        "engine-ecosystem/1.0".to_string()
    }
}

impl Default for AssetStoreClient {
    fn default() -> Self {
        Self::new(AssetStoreConfig::default())
    }
}

/// 认证 Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    token: String,
    expires_at: DateTime<Utc>,
}

impl AuthToken {
    pub fn new(token: String) -> Self {
        Self {
            token,
            expires_at: Utc::now() + chrono::Duration::hours(24),
        }
    }
}

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.token.fmt(f)
    }
}

impl AuthToken {

    pub fn expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    pub fn refresh(&self) -> Self {
        Self::new(format!("{}_refreshed", self.token))
    }
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub joined_at: DateTime<Utc>,
}

/// 搜索查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
    pub categories: Vec<AssetCategory>,
    pub tags: Vec<String>,
    pub min_rating: Option<f32>,
    pub price_range: Option<PriceRange>,
    pub license_types: Vec<AssetLicense>,
    pub sort_by: SortOrder,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            keyword: "".to_string(),
            categories: Vec::new(),
            tags: Vec::new(),
            min_rating: None,
            price_range: None,
            license_types: Vec::new(),
            sort_by: SortOrder::Relevance,
        }
    }
}

/// 价格范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    pub min: f64,
    pub max: f64,
}

/// 平台标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformFlag {
    Windows,
    Linux,
    MacOS,
    Android,
    Ios,
    WebAssembly,
}

/// 下载状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadState {
    Idle,
    Queued,
    Downloading(DownloadProgress),
    Completed,
    Failed(String),
}

/// 下载进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub bytes_total: u64,
    pub speed_kbps: f64,
    pub eta_seconds: u64,
}

impl DownloadProgress {
    pub fn percent(&self) -> f64 {
        if self.bytes_total > 0 {
            (self.bytes_downloaded as f64 / self.bytes_total as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// 下载任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub asset_id: AssetId,
    pub state: DownloadState,
    pub start_time: DateTime<Utc>,
}

/// 订单 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(uuid::Uuid);

impl OrderId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for OrderId {
    fn default() -> Self {
        Self::new()
    }
}

/// 订单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub items: Vec<CartItem>,
    pub total: Money,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub payment_id: String,
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Completed,
    Refunded,
    Cancelled,
}

/// 金额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: String,
}

impl Money {
    pub fn new(amount: f64, currency: String) -> Self {
        Self { amount, currency }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

/// 支付方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard,
    PayPal,
    Alipay,
    WeChatPay,
    StoreCredit,
}

/// 已拥有资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedAsset {
    pub asset_id: AssetId,
    pub purchase_date: DateTime<Utc>,
    pub license_key: Option<String>,
    pub download_count: u32,
}

/// 速率限制状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    pub requests_remaining: u32,
    pub reset_timestamp: DateTime<Utc>,
}

/// 资源商店错误
#[derive(Debug, Clone, Error)]
pub enum AssetStoreError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Asset not found")]
    NotFound,
    #[error("Rate limit exceeded")]
    RateLimit,
    #[error("Payment error: {0}")]
    Payment(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Signature verification failed: {0}")]
    Signature(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl AssetStoreError {
    pub fn message(&self) -> String {
        self.to_string()
    }
}

/// 更新策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdatePolicy {
    Auto,
    Notify,
    Manual,
}

/// 开发者中心
pub struct DeveloperCenter {
    drafts: HashMap<DraftId, DeveloperDraft>,
    published: HashMap<AssetId, PublishedAsset>,
}

impl DeveloperCenter {
    pub fn new() -> Self {
        Self {
            drafts: HashMap::new(),
            published: HashMap::new(),
        }
    }

    /// 发布资源
    pub fn publish_asset(&mut self, draft: DeveloperDraft) -> Result<PublishedAsset, AssetStoreError> {
        let id = AssetId::new();
        let published = PublishedAsset {
            id: id.clone(),
            name: draft.title.clone(),
            author: "Developer".to_string(),
            version: AssetVersion::default(),
            status: ReviewStatus::Submitted,
        };
        self.published.insert(id.clone(), published.clone());
        Ok(published)
    }

    /// 提交审核
    pub fn submit_for_review(&mut self, asset_id: &AssetId) -> Result<(), AssetStoreError> {
        if let Some(asset) = self.published.get_mut(asset_id) {
            asset.status = ReviewStatus::UnderReview;
        }
        Ok(())
    }

    /// 设置价格
    pub fn set_price(&mut self, _asset_id: &AssetId, _price: PriceModel) -> Result<(), AssetStoreError> {
        Ok(())
    }

    /// 获取收益报告
    pub fn revenue_report(&self, _asset_id: &AssetId, range: DateRange) -> RevenueReport {
        RevenueReport {
            period: range,
            gross_revenue: Money::new(0.0, "USD".to_string()),
            net_revenue: Money::new(0.0, "USD".to_string()),
            downloads: 0,
            refunds: 0,
        }
    }

    /// 获取下载统计
    pub fn download_stats(&self, _asset_id: &AssetId, range: DateRange) -> DownloadStats {
        DownloadStats {
            period: range,
            total_downloads: 0,
            by_country: HashMap::new(),
            by_platform: HashMap::new(),
        }
    }

    /// 获取审核状态
    pub fn review_status(&self, asset_id: &AssetId) -> Result<ReviewStatus, AssetStoreError> {
        self.published.get(asset_id)
            .map(|a| a.status)
            .ok_or(AssetStoreError::NotFound)
    }

    /// 撤回审核
    pub fn withdraw_from_review(&mut self, asset_id: &AssetId) -> Result<(), AssetStoreError> {
        if let Some(asset) = self.published.get_mut(asset_id) {
            asset.status = ReviewStatus::Draft;
        }
        Ok(())
    }

    /// 保存草稿
    pub fn save_draft(&mut self, draft: DeveloperDraft) -> DraftId {
        let id = DraftId::new();
        self.drafts.insert(id.clone(), draft);
        id
    }

    /// 获取草稿列表
    pub fn drafts(&self) -> Vec<DeveloperDraft> {
        self.drafts.values().cloned().collect()
    }
}

impl Default for DeveloperCenter {
    fn default() -> Self {
        Self::new()
    }
}

/// 草稿 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DraftId(uuid::Uuid);

impl DraftId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for DraftId {
    fn default() -> Self {
        Self::new()
    }
}

/// 开发者草稿
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperDraft {
    pub title: String,
    pub description: String,
    pub category: AssetCategory,
    pub tags: Vec<String>,
    pub price_model: PriceModel,
    pub files_dir: PathBuf,
    pub screenshots: Vec<String>,
    pub videos: Vec<String>,
}

/// 已发布资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedAsset {
    pub id: AssetId,
    pub name: String,
    pub author: String,
    pub version: AssetVersion,
    pub status: ReviewStatus,
}

/// 审核状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
}

/// 日期范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// 收益报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueReport {
    pub period: DateRange,
    pub gross_revenue: Money,
    pub net_revenue: Money,
    pub downloads: u64,
    pub refunds: u64,
}

/// 下载统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStats {
    pub period: DateRange,
    pub total_downloads: u64,
    pub by_country: HashMap<String, u64>,
    pub by_platform: HashMap<String, u64>,
}

/// 收益分成
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSplit {
    pub developer_percent: f32,
    pub platform_percent: f32,
}

impl RevenueSplit {
    pub fn default_70_30() -> Self {
        Self {
            developer_percent: 70.0,
            platform_percent: 30.0,
        }
    }

    pub fn premium() -> Self {
        Self {
            developer_percent: 80.0,
            platform_percent: 20.0,
        }
    }

    pub fn custom(dev_percent: f32, platform_percent: f32) -> Self {
        Self {
            developer_percent: dev_percent,
            platform_percent,
        }
    }
}