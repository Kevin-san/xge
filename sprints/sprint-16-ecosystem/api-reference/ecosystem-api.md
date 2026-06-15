# 生态 API 清单

## 概述

本文档列出 Sprint 16 中所有新增 crate 的公共 API，包含 engine-asset-store、engine-template、engine-profiler、engine-docs 四个模块的所有公开接口。

---

## engine-asset-store 模块

### AssetStoreClient

资源商店客户端主接口。

```rust
// 构造与配置
pub fn new(config: AssetStoreConfig) -> Self

// 认证
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
pub fn trending(&self, limit: usize) -> Vec<AssetSummary>
pub fn featured(&self, limit: usize) -> Vec<AssetSummary>
pub fn new_releases(&self, limit: usize) -> Vec<AssetSummary>
pub fn top_rated(&self, limit: usize) -> Vec<AssetSummary>
pub fn most_downloaded(&self, limit: usize) -> Vec<AssetSummary>
pub fn related_assets(&self, id: AssetId, limit: usize) -> Vec<AssetSummary>

// 资源详情
pub fn get_asset(&self, id: AssetId) -> Result<AssetDetail>

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

// 用户
pub fn me(&self) -> Result<UserProfile>
pub fn orders(&self, page: u32) -> Result<Paged<Order>>
pub fn order_detail(&self, id: OrderId) -> Result<Order>
pub fn my_assets(&self) -> Result<Vec<OwnedAsset>>

// 订阅
pub fn subscribe(&self, asset_id: AssetId, tier: SubscriptionTier) -> Result<Subscription>
pub fn cancel_subscription(&self, id: SubscriptionId) -> Result<()>
pub fn active_subscriptions(&self) -> Vec<Subscription>

// 兼容性
pub fn is_compatible(
    &self,
    asset: &AssetDetail,
    current_engine_version: &str,
    current_platform: PlatformFlag,
) -> bool

// 错误处理
pub fn set_retry_policy(&self, max_attempts: u32, backoff: Duration) -> ()
pub fn set_timeout(&self, seconds: u64) -> ()
pub fn user_agent(&self) -> String
pub fn rate_limit_status(&self) -> RateLimitStatus
```

### AssetStoreConfig

```rust
pub struct AssetStoreConfig;

impl AssetStoreConfig {
    pub fn default() -> Self
    pub fn server_url(&self) -> &str
    pub fn cache_dir(&self) -> &Path
    pub fn install_dir(&self) -> &Path
    pub fn with_server_url(url: &str) -> Self
}
```

### 数据类型

```rust
// 资源 ID
pub struct AssetId(uuid::Uuid);

impl AssetId {
    pub fn new(uuid: Uuid) -> Self
    pub fn parse(string: &str) -> Result<Self>
}

// 资源版本
pub struct AssetVersion { /* semver */ }

impl AssetVersion {
    pub fn parse(string: &str) -> Result<Self>
    pub fn cmp(&self, other: &Self) -> Ordering
}

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

### 枚举类型

```rust
// 资源类型
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

// 资源分类
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

// 许可证
pub enum AssetLicense {
    MIT,
    Apache2,
    GPLv3,
    Proprietary,
    CreativeCommons,
    Custom(String),
}

// 定价模型
pub enum PriceModel {
    Free,
    Paid { amount: f64, currency: String },
    Subscription { amount: f64, currency: String, period: String },
}

// 平台标志
pub enum PlatformFlag {
    WINDOWS,
    LINUX,
    MACOS,
    ANDROID,
    IOS,
    WEBASSEMBLY,
}

// 排序方式
pub enum SortOrder {
    Relevance,
    Rating,
    Downloads,
    Newest,
    PriceAsc,
    PriceDesc,
}
```

### RgePkg 资源打包

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
```

### 签名密钥

```rust
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

### 已安装资源

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
```

### 回滚管理

```rust
pub struct RollbackManager;

impl RollbackManager {
    pub fn snapshots(&self, asset_id: AssetId) -> Vec<Snapshot>
    pub fn rollback(&self, snapshot_id: SnapshotId) -> Result<()>
    pub fn clean_old(&self, max_count: usize) -> ()
    pub fn create_snapshot(&self, asset_id: AssetId) -> SnapshotId
    pub fn list_snapshots(&self, asset_id: AssetId) -> Vec<Snapshot>
    pub fn delete_snapshot(&self, id: SnapshotId) -> Result<()>
}
```

### 依赖解析

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

### 开发者中心

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

pub struct RevenueSplit;

impl RevenueSplit {
    pub fn default_70_30() -> Self
    pub fn premium() -> Self
    pub fn custom(dev_percent: f32, platform_percent: f32) -> Self
}
```

### 评论系统

```rust
pub struct CommentSystem;

impl CommentSystem {
    pub fn post(&self, asset_id: AssetId, comment: Comment) -> Result<CommentId>
    pub fn list(&self, asset_id: AssetId, page: u32) -> Vec<Comment>
    pub fn vote(&self, comment_id: CommentId, helpful: bool) -> Result<()>
}

impl AssetRating {
    pub fn stars(&self) -> f32
    pub fn review_count(&self) -> usize
    pub fn distribution(&self) -> RatingDistribution
}
```

### 购物车与订单

```rust
pub struct Cart;

impl Cart {
    pub fn add(&mut self, asset_id: AssetId) -> Result<()>
    pub fn remove(&mut self, asset_id: AssetId) -> Result<()>
    pub fn items(&self) -> Vec<CartItem>
    pub fn total(&self) -> Money
    pub fn checkout(&self, payment_method: PaymentMethod) -> Result<Order>
}

pub enum PaymentMethod {
    CreditCard,
    PayPal,
    Alipay,
    WeChatPay,
    StoreCredit,
}
```

### 本地库

```rust
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

## engine-template 模块

### TemplateManager

```rust
pub struct TemplateManager;

impl TemplateManager {
    pub fn new() -> Self
    pub fn list_templates(&self) -> Vec<Template>
    pub fn list_templates_by_category(&self, cat: TemplateType) -> Vec<Template>
    pub fn get_template(&self, id: &TemplateId) -> Option<&Template>
    pub fn create_project(
        &self,
        template_id: &TemplateId,
        output_dir: &Path,
        project_name: &str,
    ) -> Result<Project>
    pub fn create_project_with_options(
        &self,
        id: &TemplateId,
        options: CreateProjectOptions,
    ) -> Result<Project>
    pub fn register_template(&mut self, template: Template) -> TemplateId
    pub fn unregister_template(&self, id: &TemplateId) -> Result<()>
    pub fn template_count(&self) -> usize
    pub fn reload(&mut self) -> Result<()>
    pub fn filter(&self, filter: TemplateFilter) -> Vec<Template>
    pub fn search(&self, keyword: &str) -> Vec<Template>
    pub fn featured(&self) -> Vec<Template>
    pub fn recent(&self) -> Vec<Template>
}
```

### Template

```rust
pub struct Template {
    pub id: TemplateId,
    pub name: String,
    pub description: String,
    pub category: TemplateType,
    pub game_type: TemplateGameType,
    pub thumbnail: Option<PathBuf>,
    pub engine_version: String,
    pub files: Vec<TemplateFile>,
}

impl Template {
    pub fn version(&self) -> &str
    pub fn engine_version_required(&self) -> &str
    pub fn is_compatible(&self, engine_version: &str) -> bool
    pub fn files_count(&self) -> usize
    pub fn thumbnail_path(&self) -> Option<&Path>
    pub fn readme_content(&self) -> Option<&str>
    pub fn tags(&self) -> &[String]
    pub fn from_zip(path: &Path) -> Result<Self>
    pub fn to_zip(&self, output_path: &Path) -> Result<()>
    pub fn save_zip(&self, path: &Path) -> Result<()>
    pub fn load_zip(path: &Path) -> Result<Self>
    pub fn validate(&self) -> Result<()>
    pub fn validate_required_files(&self) -> Result<()>
    pub fn validate_engine_version(&self) -> Result<()>
    pub fn validate_manifest(&self) -> Result<()>
}
```

### TemplateId

```rust
pub struct TemplateId(uuid::Uuid);

impl TemplateId {
    pub fn new(uuid: Uuid) -> Self
    pub fn parse(s: &str) -> Result<Self>
}
```

### 模板类型

```rust
pub enum TemplateType {
    Template2D,
    Template3D,
    TemplateVR,
    TemplateAR,
    TemplateEmpty,
    TemplateTutorial,
}

pub enum TemplateGameType {
    FPS,
    TPS,
    RPG,
    RTS,
    MOBA,
    Racing,
    Platformer,
    Puzzle,
    Card,
    Roguelike,
    VisualNovel,
    TowerDefense,
}
```

### Project

```rust
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub main_scene_path: PathBuf,
}

impl Project {
    pub fn open(path: &Path) -> Result<Self>
    pub fn name(&self) -> &str
    pub fn path(&self) -> &Path
    pub fn cargo_toml(&self) -> &Path
    pub fn main_scene(&self) -> &Path
    pub fn exists(&self) -> bool
    pub fn is_initialized(&self) -> bool
    pub fn build(&self) -> Result<Output>
    pub fn run(&self) -> Result<Output>
    pub fn test(&self) -> Result<Output>
    pub fn run_cargo(&self, args: &[String]) -> Result<Output>
    pub fn read_cargo_toml(&self) -> Result<CargoToml>
}
```

### 内置模板

```rust
pub struct BuiltInTemplates;

impl BuiltInTemplates {
    pub fn all() -> Vec<Template>
    pub fn empty_2d() -> Template
    pub fn empty_3d() -> Template
    pub fn empty_vr() -> Template
    pub fn empty_ar() -> Template
    pub fn fps() -> Template
    pub fn tps() -> Template
    pub fn rpg() -> Template
    pub fn racing() -> Template
    pub fn platformer_2d() -> Template
    pub fn puzzle() -> Template
    pub fn card_game() -> Template
    pub fn roguelike() -> Template
    pub fn visual_novel() -> Template
    pub fn tower_defense() -> Template
    pub fn tutorial_first_project() -> Template
}
```

### 模板构建器

```rust
pub struct TemplateBuilder {
    pub name: String,
}

impl TemplateBuilder {
    pub fn new(name: &str) -> Self
    pub fn category(&mut self, cat: TemplateType) -> &mut Self
    pub fn game_type(&mut self, gt: TemplateGameType) -> &mut Self
    pub fn description(&mut self, s: &str) -> &mut Self
    pub fn add_file(&mut self, source: &Path, target: &Path) -> &mut Self
    pub fn add_directory(&mut self, dir: &Path) -> &mut Self
    pub fn thumbnail(&mut self, path: &Path) -> &mut Self
    pub fn build(&self) -> Result<Template>
}
```

---

## engine-profiler 模块

### Profiler

```rust
pub struct Profiler;

impl Profiler {
    pub fn new(config: ProfilerConfig) -> Self
    pub fn with_config(config: ProfilerConfig) -> Self
    pub fn toggle(&mut self, category: ProfilerCategory) -> ()
    pub fn is_enabled(&self, category: ProfilerCategory) -> bool
    pub fn clear(&mut self) -> ()
    pub fn frame_count(&self) -> usize
    pub fn current_frame_number(&self) -> u64

    // 帧控制
    pub fn begin_frame(&mut self) -> ()
    pub fn end_frame(&mut self) -> ()

    // Scope 控制
    pub fn begin_scope(&mut self, name: &str) -> ScopeGuard
    pub fn end_scope(&mut self) -> ()

    // 事件记录
    pub fn record_event(&mut self, name: &str, data: Value) -> ()

    // 样本访问
    pub fn cpu_samples(&self) -> &[CpuSample]
    pub fn gpu_samples(&self) -> &[GpuSample]
    pub fn memory_samples(&self) -> &[MemorySample]
    pub fn render_samples(&self) -> &[RenderSample]
    pub fn network_samples(&self) -> &[NetworkSample]
    pub fn script_samples(&self) -> &[ScriptSample]

    // 帧样本
    pub fn samples_for_frame(&self, frame_idx: usize) -> FrameSamples

    // 添加样本
    pub fn add_cpu_sample(&mut self, sample: CpuSample) -> ()
    pub fn add_gpu_sample(&mut self, sample: GpuSample) -> ()
    pub fn add_memory_sample(&mut self, sample: MemorySample) -> ()
    pub fn add_render_sample(&mut self, sample: RenderSample) -> ()
    pub fn add_network_sample(&mut self, sample: NetworkSample) -> ()
    pub fn add_script_sample(&mut self, sample: ScriptSample) -> ()
}
```

### ScopeGuard

```rust
pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
    name: String,
}

impl<'a> ScopeGuard<'a> {
    pub fn new(profiler: &'a mut Profiler, name: &str) -> Self
}

impl Drop for ScopeGuard<'_> {
    fn drop(&mut self) -> ()
}
```

### ProfilerConfig

```rust
pub struct ProfilerConfig;

impl ProfilerConfig {
    pub fn default() -> Self
    pub fn sample_rate_hz(&self) -> u32
    pub fn max_frames(&self) -> usize
    pub fn enabled_categories(&self) -> ProfilerCategories
}
```

### 样本类型

```rust
pub struct CpuSample {
    pub scope_name: String,
    pub thread_id: u64,
    pub start_ns: u64,
    pub duration_ns: u64,
    pub parent_index: Option<usize>,
}

impl CpuSample {
    pub fn exclusive_duration(&self) -> u64
}

pub struct GpuSample {
    pub scope_name: String,
    pub queue_index: u32,
    pub start_ns: u64,
    pub duration_ns: u64,
    pub gpu_timer_id: u32,
}

pub struct MemorySample {
    pub event_type: MemoryEventType,
    pub bytes: u64,
    pub address: u64,
    pub timestamp_ns: u64,
    pub thread_id: u64,
}

pub enum MemoryEventType {
    Alloc,
    Dealloc,
    Realloc,
    GarbageCollect,
}

pub struct RenderSample {
    pub draw_call_index: u32,
    pub pipeline: String,
    pub vertices: u32,
    pub indices: u32,
    pub textures_bound: Vec<String>,
    pub shader_name: String,
}

pub struct NetworkSample {
    pub direction: NetworkDirection,
    pub bytes: u64,
    pub protocol: String,
    pub remote_addr: String,
    pub timestamp_ns: u64,
}

pub enum NetworkDirection {
    Incoming,
    Outgoing,
}

pub struct ScriptSample {
    pub function_name: String,
    pub file: String,
    pub line: u32,
    pub duration_ns: u64,
    pub invocations: u32,
}
```

### FlameGraph

```rust
pub struct FlameGraph;

impl FlameGraph {
    pub fn from_samples(samples: &[CpuSample]) -> Self
    pub fn nodes(&self) -> &[FlameNode]
    pub fn root(&self) -> &FlameNode
    pub fn search(&self, keyword: &str) -> Vec<&FlameNode>
    pub fn hot_path(&self) -> Vec<&FlameNode>
    pub fn render_svg(&self, path: &Path) -> Result<()>
    pub fn render_text(&self, width: usize, height: usize) -> String
    pub fn render_json(&self) -> String
}

pub struct FlameNode {
    pub name: String,
    pub start: u64,
    pub duration: u64,
    pub children: Vec<FlameNode>,
}

impl FlameNode {
    pub fn total_duration(&self) -> u64
    pub fn self_duration(&self) -> u64
    pub fn percent_of_parent(&self) -> f64
    pub fn percent_of_total(&self) -> f64
}
```

### Timeline

```rust
pub struct Timeline;

impl Timeline {
    pub fn new() -> Self
    pub fn add_track(&mut self, track: TimelineTrack) -> ()
    pub fn tracks(&self) -> &[TimelineTrack]
    pub fn render(&self) -> TimelineView
    pub fn total_duration(&self) -> u64
    pub fn track_count(&self) -> usize
    pub fn zoom(&mut self, start_ratio: f64, end_ratio: f64) -> ()
    pub fn pan(&mut self, offset_ratio: f64) -> ()
    pub fn cursor_time(&self) -> u64
    pub fn set_cursor(&mut self, time_ns: u64) -> ()
}

pub struct TimelineTrack {
    pub name: String,
    pub samples: Vec<u64>,
    pub color: String,
    pub row_index: usize,
}
```

### Histogram

```rust
pub struct Histogram;

impl Histogram {
    pub fn from_values(values: &[f64], bucket_count: usize) -> Self
    pub fn mean(&self) -> f64
    pub fn median(&self) -> f64
    pub fn p95(&self) -> f64
    pub fn p99(&self) -> f64
    pub fn min(&self) -> f64
    pub fn max(&self) -> f64
    pub fn std_dev(&self) -> f64
    pub fn buckets(&self) -> &[HistogramBucket]
    pub fn render_text(&self, width: usize, height: usize) -> String
}

pub struct HistogramBucket {
    pub start: f64,
    pub end: f64,
    pub count: u64,
}
```

### LineChart

```rust
pub struct LineChart;

impl LineChart {
    pub fn new() -> Self
    pub fn push(&mut self, timestamp: u64, value: f64) -> ()
    pub fn points(&self) -> &[LineChartPoint]
    pub fn min_value(&self) -> f64
    pub fn max_value(&self) -> f64
    pub fn trend_slope(&self) -> f64
}

pub struct LineChartPoint {
    pub timestamp: u64,
    pub value: f64,
}
```

### Metrics

```rust
pub struct Metrics {
    pub frame_time_ms: f64,
    pub fps: f64,
    pub cpu_usage_percent: f64,
    pub gpu_usage_percent: f64,
    pub memory_rss_kb: u64,
    pub memory_vss_kb: u64,
    pub network_in_kbs: f64,
    pub network_out_kbs: f64,
    pub disk_read_kbs: f64,
    pub disk_write_kbs: f64,
}

pub struct MetricsCollector;

impl MetricsCollector {
    pub fn snapshot(&mut self) -> MetricsSnapshot
    pub fn history(&self, window_seconds: f64) -> Vec<MetricsSnapshot>
    pub fn moving_average(&self, window_seconds: f64) -> MetricsSnapshot
}

pub struct FrameMetricsAggregator;

impl FrameMetricsAggregator {
    pub fn push(&mut self, frame: FrameMetrics) -> ()
    pub fn average_frame_time(&self) -> f64
    pub fn average_fps(&self) -> f64
    pub fn average_draw_calls(&self) -> f64
    pub fn total_triangles(&self) -> u64
    pub fn total_vertices(&self) -> u64
}
```

### 远程分析

```rust
pub struct RemoteProfilerServer;

impl RemoteProfilerServer {
    pub fn bind(addr: &str) -> Result<Self>
    pub fn accept(&mut self) -> Result<RemoteSession>
    pub fn stream_samples(&mut self) -> ()
    pub fn connected_clients(&self) -> usize
    pub fn broadcast(&mut self, message: RemoteMessage) -> Result<usize>
    pub fn sessions(&self) -> Vec<RemoteSession>
}

pub struct RemoteProfilerClient;

impl RemoteProfilerClient {
    pub fn connect(addr: &str) -> Result<Self>
    pub fn receive_samples(&mut self) -> Result<SampleBatch>
    pub fn server_info(&self) -> ServerInfo
    pub fn stream(&mut self, callback: F) -> Result<()>
}

pub struct RemoteSession;

impl RemoteSession {
    pub fn device_info(&self) -> DeviceInfo
    pub fn disconnect(&mut self) -> ()
}

pub struct DeviceInfo {
    pub name: String,
    pub os: String,
    pub cpu_brand: String,
    pub gpu_brand: String,
    pub ram_gb: f64,
    pub screen_resolution: String,
}
```

### 性能诊断

```rust
pub struct PerformanceDiagnosticEngine;

impl PerformanceDiagnosticEngine {
    pub fn new(rules: DiagnosticRuleSet) -> Self
    pub fn add_rule(&mut self, rule: DiagnosticRule) -> ()
    pub fn run(&self, profile: &ProfileData) -> Vec<PerformanceWarning>
}

pub enum DiagnosticRule {
    ExcessiveDrawCalls,
    HighFrameTimeVariance,
    MemoryLeakSuspect,
    GpuBound,
    CpuBound,
    ScriptGcStall,
    ExcessiveGcPauses,
    HighGpuLatency,
    TextureUploadStall,
    AssetLoadOnCriticalPath,
    UnbatchedDrawCalls,
}

pub enum WarningSeverity {
    Info,
    Warning,
    Critical,
}

pub struct DiagnosticReport;

impl DiagnosticReport {
    pub fn warnings(&self) -> &[PerformanceWarning]
    pub fn summary(&self) -> String
    pub fn critical_count(&self) -> usize
    pub fn warning_count(&self) -> usize
    pub fn info_count(&self) -> usize
}
```

### 基线与回归

```rust
pub struct BaselineProfile;

impl BaselineProfile {
    pub fn new(name: &str, samples: ProfileData) -> Self
    pub fn compare(&self, new_samples: &ProfileData) -> RegressionReport
}

pub struct BaselineManager;

impl BaselineManager {
    pub fn new(dir: &Path) -> Self
    pub fn save(&self, name: &str, profile: &ProfileData) -> Result<()>
    pub fn load(&self, name: &str) -> Result<BaselineProfile>
    pub fn list(&self) -> Vec<String>
    pub fn delete(&self, name: &str) -> Result<()>
}

pub struct RegressionReport;

impl RegressionReport {
    pub fn regressions(&self) -> Vec<Regression>
    pub fn print(&self) -> String
    pub fn to_json(&self) -> String
    pub fn has_regressions(&self) -> bool
    pub fn regression_count(&self) -> usize
}
```

### Profile 格式

```rust
pub struct RgeProfile;

impl RgeProfile {
    pub fn export(path: &Path, profile: &ProfileData) -> Result<()>
    pub fn import(path: &Path) -> Result<ProfileData>
    pub fn summary(&self) -> ProfileSummary
    pub fn set_encryption_key(&mut self, key: &[u8]) -> ()
    pub fn has_encryption(&self) -> bool
    pub fn file_size_bytes(&self) -> u64
    pub fn metadata(&self) -> ProfileMetadata
}

pub struct ProfileViewer;

impl ProfileViewer {
    pub fn open(path: &Path) -> Result<Self>
    pub fn frame_summary(&self, frame_idx: usize) -> FrameSummary
    pub fn flame_graph(&self, frame_range: Range<usize>) -> FlameGraph
    pub fn timeline(&self, frame_range: Range<usize>) -> Timeline
    pub fn export_csv(&self, path: &Path) -> Result<()>
    pub fn export_json(&self, path: &Path) -> Result<()>
}
```

### 宏

```rust
#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => { ... }
}

#[macro_export]
macro_rules! profile_event {
    ($name:expr, $data:expr) => { ... }
}

#[macro_export]
macro_rules! profile_scope_data {
    ($tag:expr, $key:ident = $value:expr) => { ... }
}
```

---

## 版本与兼容性

所有公共 API 均遵循 semver 2.0 版本规范。

| Crate | 版本 | MSRV |
|-------|------|------|
| engine-asset-store | 1.0.0 | Rust 1.70 |
| engine-template | 1.0.0 | Rust 1.70 |
| engine-profiler | 1.0.0 | Rust 1.70 |
| engine-docs | 1.0.0 | N/A |
