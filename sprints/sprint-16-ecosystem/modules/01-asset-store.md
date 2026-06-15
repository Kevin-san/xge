# 资源商店模块（engine-asset-store）

## 模块概述

资源商店模块提供完整的资源浏览、购买、下载、安装、更新与管理功能，支持`.rgepkg`资源打包格式，实现开发者中心与评论系统，构建完善的资源生态体系。

**Crate**: `engine-asset-store`
**周期**: 4 周
**优先级**: P0

---

## 需求清单

### 1. 核心客户端（需求 1-69, 266-437）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 1 | 建立 `engine-asset-store` crate | P0 |
| 5 | `AssetStoreClient::new(config) -> Self` | P0 |
| 6 | `AssetStoreClient::login(username, password) -> Result<()>` | P0 |
| 7 | `AssetStoreClient::login_with_token(token) -> Result<()>` | P0 |
| 8 | `AssetStoreClient::logout(&mut self) -> ()` | P0 |
| 9 | `AssetStoreClient::is_logged_in(&self) -> bool` | P0 |
| 10 | `AssetStoreClient::search(keyword, filters) -> Result<Vec<AssetSummary>>` | P0 |
| 11 | `AssetStoreClient::browse(category, page, page_size) -> Result<Vec<AssetSummary>>` | P0 |
| 12 | `AssetStoreClient::get_asset(id) -> Result<AssetDetail>` | P0 |
| 13 | `AssetStoreClient::download(id, progress_cb) -> Result<PathBuf>` | P0 |
| 14 | `AssetStoreClient::install(downloaded_path, target_dir) -> Result<InstalledAsset>` | P0 |
| 15 | `AssetStoreClient::update(id) -> Result<InstalledAsset>` | P0 |
| 16 | `AssetStoreClient::uninstall(id) -> Result<()>` | P0 |
| 17 | `AssetStoreClient::list_installed(&self) -> Vec<InstalledAsset>` | P0 |
| 18 | `AssetStoreClient::has_updates(&self) -> Vec<AssetId>` | P0 |
| 19 | `AssetStoreClient::rollback(id, version) -> Result<()>` | P0 |
| 37 | `AssetStoreClient::refresh_token(&mut self) -> Result<()>` | P1 |
| 45 | `AssetStoreClient::set_update_policy(policy) -> ()` | P1 |
| 46 | `AssetStoreClient::update_all(&mut self) -> Vec<Result<InstalledAsset>>` | P1 |
| 66 | `AssetStoreClient::cancel_download(id) -> bool` | P1 |
| 67 | `AssetStoreClient::pause_download(id) -> bool` | P1 |
| 68 | `AssetStoreClient::resume_download(id) -> bool` | P1 |
| 69 | `AssetStoreClient::rate_limit_status(&self) -> RateLimitStatus` | P2 |

#### API 签名详情

```rust
// 客户端构造
pub fn new(config: AssetStoreConfig) -> Self
pub fn login(&self, username: &str, password: &str) -> Result<()>
pub fn login_with_token(&self, token: &str) -> Result<()>
pub fn logout(&mut self) -> ()
pub fn is_logged_in(&self) -> bool
pub fn refresh_token(&mut self) -> Result<()>

// 浏览与搜索
pub fn search(&self, keyword: &str, filters: SearchFilters) -> Result<Vec<AssetSummary>>
pub fn browse(&self, category: AssetCategory, page: u32, page_size: u32) -> Result<Vec<AssetSummary>>
pub fn search_with_pagination(&self, query: SearchQuery, page: u32, page_size: u32) -> Result<Paged<AssetSummary>>
pub fn browse_category(&self, cat: AssetCategory, sort: SortOrder, page: u32) -> Result<Paged<AssetSummary>>

// 资源详情
pub fn get_asset(&self, id: AssetId) -> Result<AssetDetail>
pub fn trending(&self, limit: usize) -> Vec<AssetSummary>
pub fn featured(&self, limit: usize) -> Vec<AssetSummary>
pub fn new_releases(&self, limit: usize) -> Vec<AssetSummary>
pub fn top_rated(&self, limit: usize) -> Vec<AssetSummary>
pub fn most_downloaded(&self, limit: usize) -> Vec<AssetSummary>
pub fn related_assets(&self, id: AssetId, limit: usize) -> Vec<AssetSummary>

// 下载与安装
pub fn download(&self, id: AssetId, progress_cb: F) -> Result<PathBuf>
pub fn download_async(&self, id: AssetId, on_progress: F) -> JoinHandle<Result<PathBuf>>
pub fn install(&self, downloaded_path: &Path, target_dir: &Path) -> Result<InstalledAsset>
pub fn update(&self, id: AssetId) -> Result<InstalledAsset>
pub fn uninstall(&self, id: AssetId) -> Result<()>
pub fn cancel_download(&self, id: AssetId) -> bool
pub fn pause_download(&self, id: AssetId) -> bool
pub fn resume_download(&self, id: AssetId) -> bool

// 已安装资源
pub fn list_installed(&self) -> Vec<InstalledAsset>
pub fn has_updates(&self) -> Vec<AssetId>
pub fn rollback(&self, id: AssetId, version: AssetVersion) -> Result<()>
pub fn set_update_policy(&self, policy: UpdatePolicy) -> ()
pub fn update_all(&mut self) -> Vec<Result<InstalledAsset>>
```

#### 输入/输出

| 操作 | 输入 | 输出 |
|------|------|------|
| login | username: &str, password: &str | Result<()> |
| search | keyword: &str, filters: SearchFilters | Result<Vec<AssetSummary>> |
| browse | category: AssetCategory, page: u32, page_size: u32 | Result<Vec<AssetSummary>> |
| download | id: AssetId, progress_cb: F | Result<PathBuf> |
| install | downloaded_path: &Path, target_dir: &Path | Result<InstalledAsset> |

---

### 2. 数据类型（需求 20-55, 266-395）

#### 资源类型枚举

```rust
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
```

#### 核心数据结构

```rust
// 资源摘要
pub struct AssetSummary {
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
    pub dependency_summary: String,
}

// 资源详情
pub struct AssetDetail {
    pub summary: AssetSummary,
    pub description: String,
    pub screenshots: Vec<AssetScreenshot>,
    pub videos: Vec<AssetVideo>,
    pub dependency_graph: DependencyGraph,
    pub comments: Vec<Comment>,
    pub changelog: String,
}

// 资源元数据
pub struct AssetMetadata {
    pub name: String,
    pub author: String,
    pub version: AssetVersion,
    pub tags: Vec<String>,
    pub category: AssetCategory,
    pub rating: AssetRating,
    pub downloads: u64,
    pub price: PriceModel,
    pub license: AssetLicense,
    pub screenshots: Vec<String>,
    pub videos: Vec<String>,
    pub dependencies: Vec<AssetDependency>,
    pub compatibility: AssetCompatibility,
    pub min_engine_version: String,
}
```

#### 许可证与定价

```rust
pub enum AssetLicense {
    MIT,
    Apache2,
    GPLv3,
    Proprietary,
    CreativeCommons,
    Custom(String),
}

pub enum PriceModel {
    Free,
    Paid { amount: f64, currency: String },
    Subscription { amount: f64, currency: String, period: String },
}
```

#### 版本管理

```rust
pub struct AssetVersion {
    // semver 语义化版本
}

impl AssetVersion {
    pub fn parse(string: &str) -> Result<Self>
    pub fn cmp(&self, other: &Self) -> Ordering
}
```

#### 依赖解析

```rust
pub struct DependencyResolver;

impl DependencyResolver {
    pub fn resolve(deps: Vec<AssetDependency>) -> Result<ResolutionGraph>
    pub fn detect_conflicts(graph: &ResolutionGraph) -> Vec<Conflict>
}

pub struct Conflict {
    pub asset_a: AssetId,
    pub asset_b: AssetId,
    pub reason: String,
}
```

---

### 3. 资源打包格式 .rgepkg（需求 30-35, 328-339）

#### 格式规范

`.rgepkg` 文件结构：
- `manifest.yaml` - 清单文件
- `files/` - 资源文件目录
- `signature.bin` - 签名文件
- `checksums.txt` - 校验和文件

```rust
pub struct RgePkg;

impl RgePkg {
    pub fn pack(source_dir: &Path, output_path: &Path, signing_key: &SigningKey) -> Result<()>
    pub fn unpack(pkg_path: &Path, target_dir: &Path, verify_signature: bool) -> Result<AssetMetadata>
    pub fn verify(pkg_path: &Path, public_key: &PublicKey) -> Result<bool>
    pub fn manifest(&self) -> &RgePkgManifest
    pub fn file_entries(&self) -> Vec<FileEntry>
    pub fn extract_file(&self, member_path: &Path, output: &Path) -> Result<()>
    pub fn sign(&mut self, key: &SigningKey) -> Result<()>
    pub fn has_signature(&self) -> bool
    pub fn signer_key_id(&self) -> Option<String>
}

pub enum ChecksumAlgorithm {
    SHA256,
    SHA512,
    BLAKE3,
}

pub struct SigningKey;

impl SigningKey {
    pub fn generate() -> Self
    pub fn from_pem(path: &Path) -> Result<Self>
    pub fn public_key(&self) -> PublicKey
}

pub struct PublicKey;

impl PublicKey {
    pub fn verify(&self, signature: &[u8], message: &[u8]) -> bool
}
```

---

### 4. 安装与回滚（需求 40-72, 306-363）

```rust
pub struct InstalledAsset {
    pub id: AssetId,
    pub name: String,
    pub version: AssetVersion,
    pub install_path: PathBuf,
    pub install_time: DateTime,
    pub files: Vec<PathBuf>,
}

impl InstalledAsset {
    pub fn check_update(&self, client: &AssetStoreClient) -> Result<Option<AssetVersion>>
    pub fn installed_files(&self) -> &[PathBuf]
    pub fn total_size_bytes(&self) -> u64
}

pub struct RollbackManager;

impl RollbackManager {
    pub fn snapshots(&self, asset_id: AssetId) -> Vec<Snapshot>
    pub fn rollback(&self, snapshot_id: SnapshotId) -> Result<()>
    pub fn clean_old(&self, max_count: usize) -> ()
    pub fn create_snapshot(&self, asset_id: AssetId) -> SnapshotId
    pub fn list_snapshots(&self, asset_id: AssetId) -> Vec<Snapshot>
    pub fn delete_snapshot(&self, id: SnapshotId) -> Result<()>
}

pub struct Snapshot {
    pub id: SnapshotId,
    pub asset_id: AssetId,
    pub from_version: AssetVersion,
    pub to_version: AssetVersion,
    pub created_at: DateTime,
    pub size_bytes: u64,
}
```

---

### 5. 购物车与订单（需求 340-389）

```rust
pub struct Cart;

impl Cart {
    pub fn add(&mut self, asset_id: AssetId) -> Result<()>
    pub fn remove(&mut self, asset_id: AssetId) -> Result<()>
    pub fn items(&self) -> Vec<CartItem>
    pub fn total(&self) -> Money
    pub fn checkout(&self, payment_method: PaymentMethod) -> Result<Order>
}

pub struct CartItem {
    pub asset_id: AssetId,
    pub name: String,
    pub price: Money,
    pub quantity: u32,
}

pub enum PaymentMethod {
    CreditCard,
    PayPal,
    Alipay,
    WeChatPay,
    StoreCredit,
}

pub struct Order {
    pub id: OrderId,
    pub items: Vec<CartItem>,
    pub total: Money,
    pub status: OrderStatus,
    pub created_at: DateTime,
    pub payment_id: String,
}

pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Completed,
    Refunded,
    Cancelled,
}

impl AssetStoreClient {
    pub fn orders(&self, page: u32) -> Result<Paged<Order>>
    pub fn order_detail(&self, id: OrderId) -> Result<Order>
    pub fn my_assets(&self) -> Result<Vec<OwnedAsset>>
}

pub struct OwnedAsset {
    pub asset_id: AssetId,
    pub purchase_date: DateTime,
    pub license_key: Option<String>,
    pub download_count: u32,
}
```

---

### 6. 开发者中心（需求 53-62, 356-412）

```rust
pub struct DeveloperCenter;

impl DeveloperCenter {
    pub fn publish_asset(&self, draft: DeveloperDraft) -> Result<PublishedAsset>
    pub fn submit_for_review(&self, asset_id: AssetId) -> Result<()>
    pub fn set_price(&self, asset_id: AssetId, price_model: PriceModel) -> Result<()>
    pub fn revenue_report(&self, asset_id: AssetId, range: DateRange) -> RevenueReport
    pub fn download_stats(&self, asset_id: AssetId, range: DateRange) -> DownloadStats
    pub fn review_status(&self, asset_id: AssetId) -> Result<ReviewStatus>
    pub fn withdraw_from_review(&self, asset_id: AssetId) -> Result<()>
}

pub enum ReviewStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected(String),
}

pub struct RevenueReport {
    pub period: DateRange,
    pub gross_revenue: Money,
    pub net_revenue: Money,
    pub downloads: u64,
    pub refunds: u64,
}

pub struct RevenueSplit;

impl RevenueSplit {
    pub fn default_70_30() -> Self
    pub fn premium() -> Self
    pub fn custom(dev_percent: f32, platform_percent: f32) -> Self
}
```

---

### 7. 评论系统（需求 59-62, 368-375）

```rust
pub struct Comment {
    pub author: String,
    pub content: String,
    pub rating: u8,
    pub timestamp: DateTime,
    pub helpful_votes: u32,
}

pub struct CommentSystem;

impl CommentSystem {
    pub fn post(&self, asset_id: AssetId, comment: Comment) -> Result<CommentId>
    pub fn list(&self, asset_id: AssetId, page: u32) -> Vec<Comment>
    pub fn vote(&self, comment_id: CommentId, helpful: bool) -> Result<()>
}

pub struct RatingDistribution {
    pub one_star: u32,
    pub two_star: u32,
    pub three_star: u32,
    pub four_star: u32,
    pub five_star: u32,
}

impl AssetRating {
    pub fn stars(&self) -> f32
    pub fn review_count(&self) -> usize
    pub fn distribution(&self) -> RatingDistribution
}
```

---

### 8. 订阅系统（需求 64-66, 376-380）

```rust
pub enum SubscriptionTier {
    Basic,
    Pro,
    Enterprise,
}

pub struct Subscription {
    pub tier: SubscriptionTier,
    pub price: Money,
    pub period: String,
    pub start_date: DateTime,
    pub next_billing_date: DateTime,
    pub status: SubscriptionStatus,
}

pub enum SubscriptionStatus {
    Active,
    Cancelled,
    Expired,
    PastDue,
}

impl AssetStoreClient {
    pub fn subscribe(&self, asset_id: AssetId, tier: SubscriptionTier) -> Result<Subscription>
    pub fn cancel_subscription(&self, id: SubscriptionId) -> Result<()>
    pub fn active_subscriptions(&self) -> Vec<Subscription>
}
```

---

### 9. 离线模式与本地库（需求 67-68, 381-389）

```rust
pub struct OfflineMode;

impl OfflineMode {
    pub fn enable() -> ()
    pub fn disable() -> ()
    pub fn is_enabled() -> bool
}

pub struct OfflineCache;

impl OfflineCache {
    pub fn prefetch(&self, asset_ids: Vec<AssetId>) -> Result<()>
    pub fn available_offline(&self) -> Vec<InstalledAsset>
    pub fn last_sync_time(&self) -> Option<DateTime>
    pub fn sync(&mut self) -> Result<()>
}

pub struct LocalLibrary;

impl LocalLibrary {
    pub fn list(&self) -> Vec<InstalledAsset>
    pub fn scan(&self, path: &Path) -> Result<Vec<InstalledAsset>>
    pub fn import_manual(&self, pkg_path: &Path) -> Result<InstalledAsset>
    pub fn add(&mut self, installed: InstalledAsset) -> Result<()>
    pub fn remove(&mut self, asset_id: AssetId) -> Result<()>
    pub fn export_collection(&self, path: &Path) -> Result<()>
    pub fn import_collection(&self, path: &Path) -> Result<Vec<InstalledAsset>>
}
```

---

### 10. 资源商店 UI（需求 46-55）

```rust
pub struct AssetStoreUI;

impl AssetStoreUI {
    pub fn home_page() -> HomePageView
    pub fn category_page(category: AssetCategory) -> CategoryView
    pub fn search_page(keyword: &str) -> SearchView
    pub fn detail_page(id: AssetId) -> DetailView
    pub fn cart_page() -> CartView
    pub fn orders_page() -> OrdersView
    pub fn my_assets_page() -> MyAssetsView
}
```

---

### 11. 资源兼容性（需求 291-297）

```rust
pub struct AssetCompatibility {
    pub min_engine_version: String,
    pub max_engine_version: String,
    pub platforms: Vec<PlatformFlag>,
}

pub enum PlatformFlag {
    WINDOWS,
    LINUX,
    MACOS,
    ANDROID,
    IOS,
    WEBASSEMBLY,
}

impl AssetStoreClient {
    pub fn is_compatible(
        &self,
        asset: &AssetDetail,
        current_engine_version: &str,
        current_platform: PlatformFlag,
    ) -> bool
}
```

---

### 12. 下载管理（需求 298-305）

```rust
pub enum DownloadState {
    Idle,
    Queued,
    Downloading(DownloadProgress),
    Completed,
    Failed(String),
}

pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub bytes_total: u64,
    pub speed_kbps: f64,
    pub eta_seconds: u64,
}

pub struct DownloadManager;

impl DownloadManager {
    pub fn queue(&self) -> Vec<DownloadTask>
}

pub struct DownloadTask {
    pub asset_id: AssetId,
    pub state: DownloadState,
    pub start_time: DateTime,
}
```

---

### 13. 错误处理（需求 392-396）

```rust
pub enum AssetStoreError {
    Network(String),
    Auth(String),
    NotFound,
    RateLimit,
    Payment(String),
    Conflict(String),
    Signature(String),
    Unknown(String),
}

impl AssetStoreError {
    pub fn message(&self) -> String
}

impl AssetStoreClient {
    pub fn set_retry_policy(&self, max_attempts: u32, backoff: Duration) -> ()
    pub fn set_timeout(&self, seconds: u64) -> ()
    pub fn user_agent(&self) -> String
}
```

---

## 验收标准

### 功能验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-1 | `AssetStoreClient::login` 成功返回 | 单元测试 |
| AC-2 | `search` 返回匹配的 AssetSummary 列表 | 集成测试 |
| AC-3 | `download` 正确下载并返回文件路径 | 集成测试 |
| AC-4 | `install` 正确解压并安装资源 | 集成测试 |
| AC-5 | `rollback` 能恢复到指定版本 | 集成测试 |
| AC-6 | `.rgepkg` 格式能正确打包/解包 | 单元测试 |
| AC-7 | `cart` 购物车流程完整 | 示例测试 |
| AC-8 | `order` 订单流程完整 | 示例测试 |

### 示例验收

| 示例 | 验收条件 |
|------|----------|
| `store_browse` | 首页 + 分类 + 搜索 + 详情页 UI 可运行 |
| `store_purchase` | 购物车 + 支付模拟 + 订单查看可运行 |
| `store_install` | 下载进度条 + 安装 + 更新 + 回滚演示可运行 |
| `store_install` | 离线模式演示可运行 |

---

## 依赖关系

### 内部依赖

- `engine-core`: 基础类型定义
- `engine-asset`: 资源系统接口
- `engine-network`: 网络通信

### 外部依赖

- `reqwest`: HTTP 客户端
- `tokio`: 异步运行时
- `sha2`/`blake3`: 校验和计算
- `rsa`/`ed25519-dalek`: 签名验证
- `zip`: 压缩支持

---

## 优先级说明

- **P0**: 核心功能，MVP 必须包含
- **P1**: 重要功能，下一迭代包含
- **P2**: 增强功能，后续迭代包含
