# 性能分析器模块（engine-profiler）

## 模块概述

性能分析器模块提供多维度性能采样与可视化功能，支持 CPU/GPU/内存/渲染/网络/脚本六大维度，提供 FlameGraph、Timeline、Histogram 等可视化面板，支持远程采样与回归检测。

**Crate**: `engine-profiler`
**周期**: 4 周
**优先级**: P0

---

## 需求清单

### 1. 核心分析器（需求 3, 92-131, 486-537）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 3 | 建立 `engine-profiler` crate | P0 |
| 92 | `Profiler::new(config) -> Self` | P0 |
| 93 | `Profiler::begin_frame(&mut self) -> ()` | P0 |
| 94 | `Profiler::end_frame(&mut self) -> ()` | P0 |
| 95 | `Profiler::begin_scope(&mut self, name) -> ScopeGuard` | P0 |
| 96 | `Profiler::end_scope(&mut self) -> ()` | P0 |
| 97 | `Profiler::record_event(&mut self, name, data) -> ()` | P0 |
| 98 | `Profiler::cpu_samples(&self) -> &[CpuSample]` | P0 |
| 99 | `Profiler::gpu_samples(&self) -> &[GpuSample]` | P0 |
| 100 | `Profiler::memory_samples(&self) -> &[MemorySample]` | P0 |
| 101 | `Profiler::render_samples(&self) -> &[RenderSample]` | P0 |
| 102 | `Profiler::network_samples(&self) -> &[NetworkSample]` | P0 |
| 103 | `Profiler::script_samples(&self) -> &[ScriptSample]` | P0 |
| 131 | `SampleGranularity::Frame / Tick / System / Function / DrawCall` | P0 |
| 154 | `profile_scope!("name")` 宏 | P0 |
| 155 | `profile_event!("name", data)` 宏 | P0 |

#### API 签名详情

```rust
pub struct Profiler {
    // 配置
}

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

pub struct ProfilerConfig;

impl ProfilerConfig {
    pub fn default() -> Self
    pub fn sample_rate_hz(&self) -> u32
    pub fn max_frames(&self) -> usize
    pub fn enabled_categories(&self) -> ProfilerCategories
}

pub struct ProfilerCategories {
    pub cpu: bool,
    pub gpu: bool,
    pub memory: bool,
    pub render: bool,
    pub network: bool,
    pub script: bool,
}
```

---

### 2. Scope Guard（需求 123, 541, 557-558）

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

---

### 3. CPU 样本（需求 125, 499-500, 541）

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
```

---

### 4. GPU 样本（需求 126, 501）

```rust
pub struct GpuSample {
    pub scope_name: String,
    pub queue_index: u32,
    pub start_ns: u64,
    pub duration_ns: u64,
    pub gpu_timer_id: u32,
}
```

---

### 5. 内存样本（需求 127, 502-503）

```rust
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
```

---

### 6. 渲染样本（需求 128, 504）

```rust
pub struct RenderSample {
    pub draw_call_index: u32,
    pub pipeline: String,
    pub vertices: u32,
    pub indices: u32,
    pub textures_bound: Vec<String>,
    pub shader_name: String,
}
```

---

### 7. 网络样本（需求 129, 505）

```rust
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
```

---

### 8. 脚本样本（需求 130, 506）

```rust
pub struct ScriptSample {
    pub function_name: String,
    pub file: String,
    pub line: u32,
    pub duration_ns: u64,
    pub invocations: u32,
}
```

---

### 9. 帧样本聚合（需求 132, 514-516）

```rust
pub struct FrameSamples {
    pub frame_number: u64,
    pub cpu: Vec<CpuSample>,
    pub gpu: Vec<GpuSample>,
    pub memory: Vec<MemorySample>,
    pub render: Vec<RenderSample>,
    pub network: Vec<NetworkSample>,
    pub script: Vec<ScriptSample>,
}
```

---

### 10. Flame Graph 可视化（需求 105-108, 133-135, 521-530）

```rust
pub struct FlameGraph {
    // 内部数据
}

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

---

### 11. Timeline 可视化（需求 109-112, 136, 530-538）

```rust
pub struct Timeline {
    // 内部数据
}

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

pub struct TimelineView {
    // 渲染后的视图数据
}
```

---

### 12. Histogram 统计分析（需求 113-117, 140-144, 537-543）

```rust
pub struct Histogram {
    buckets: Vec<HistogramBucket>,
}

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

---

### 13. 其他图表（需求 118-120, 145-149）

```rust
pub struct PieChart {
    segments: Vec<PieSegment>,
}

impl PieChart {
    pub fn from_segments(segments: Vec<PieSegment>) -> Self
}

pub struct PieSegment {
    pub label: String,
    pub value: f64,
    pub color: String,
}

pub struct LineChart {
    points: Vec<LineChartPoint>,
}

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

---

### 14. 指标收集（需求 120, 147-149, 548-560）

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

pub struct FrameMetrics {
    pub frame_number: u64,
    pub frame_time: f64,
    pub gpu_time: f64,
    pub cpu_time: f64,
    pub draw_calls: u32,
    pub triangles: u64,
    pub vertices: u64,
}

pub struct MetricsCollector;

impl MetricsCollector {
    pub fn snapshot(&mut self) -> MetricsSnapshot
    pub fn history(&self, window_seconds: f64) -> Vec<MetricsSnapshot>
    pub fn moving_average(&self, window_seconds: f64) -> MetricsSnapshot
}

pub struct MetricsSnapshot {
    pub timestamp: u64,
    pub frame_time_ms: f64,
    pub fps: f64,
    pub cpu: f64,
    pub gpu: f64,
    pub memory_rss_mb: f64,
    pub memory_vss_mb: f64,
    pub net_in_kbs: f64,
    pub net_out_kbs: f64,
    pub disk_read_kbs: f64,
    pub disk_write_kbs: f64,
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

---

### 15. 调用栈与符号解析（需求 122, 150-151, 558-569）

```rust
pub struct CallStackSample {
    pub addresses: Vec<u64>,
    pub symbols: Vec<Symbol>,
    pub thread_id: u64,
}

pub struct CallStack {
    pub frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn frames(&self) -> &[StackFrame]
    pub fn depth(&self) -> usize
}

pub struct StackFrame {
    pub function: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub address: u64,
}

impl StackFrame {
    pub fn display(&self) -> String
}

pub struct Symbol {
    pub name: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

pub struct SymbolResolver;

impl SymbolResolver {
    pub fn load_symbols(executable: &Path) -> Result<Self>
    pub fn resolve(&self, address: u64) -> Result<Symbol>
    pub fn cache(&self) -> &SymbolCache
}

pub struct SymbolCache;

impl SymbolCache {
    pub fn get(&self, address: u64) -> Option<&Symbol>
    pub fn insert(&mut self, address: u64, symbol: Symbol) -> ()
    pub fn invalidate(&mut self) -> ()
}
```

---

### 16. 硬件计数器（需求 125-126, 152-153, 567-576）

```rust
pub enum HardwareCounter {
    CpuCycles,
    CacheMisses,
    BranchMisses,
    Instructions,
    GpuVertexTime,
    GpuFragmentTime,
    GpuComputeTime,
}

impl HardwareCounter {
    pub fn read(&self) -> Result<u64>
}

pub enum CpuCounter {
    Cycles,
    Instructions,
    CacheReferences,
    CacheMisses,
    BranchInstructions,
    BranchMisses,
    BusCycles,
}

pub enum GpuCounter {
    VertexShaderNs,
    FragmentShaderNs,
    ComputeShaderNs,
    TessControlNs,
    TessEvalNs,
    GeometryShaderNs,
    TransferNs,
}

pub struct HardwareCounterSet;

impl HardwareCounterSet {
    pub fn available_counters() -> Vec<HardwareCounter>
    pub fn start_all(&mut self) -> ()
    pub fn stop_all(&mut self) -> ()
    pub fn read_all(&self) -> HashMap<HardwareCounter, u64>
}

pub struct PerfData;

impl PerfData {
    pub fn cpu_counters(&self) -> HashMap<CpuCounter, u64>
    pub fn gpu_counters(&self) -> HashMap<GpuCounter, u64>
}
```

---

### 17. 远程分析（需求 156-166, 577-587）

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

pub enum RemoteProfilerProtocol {
    VERSION,
    HELLO,
    SAMPLE_BATCH,
    DEVICE_INFO,
    DISCONNECT,
}

impl RemoteProfilerProtocol {
    pub const VERSION: u32 = 1;
}

pub struct RemoteMessage;

impl RemoteMessage {
    pub fn serialize(&self) -> Vec<u8>
    pub fn deserialize(buf: &[u8]) -> Result<Self>
}

pub struct DeviceInfo {
    pub name: String,
    pub os: String,
    pub cpu_brand: String,
    pub gpu_brand: String,
    pub ram_gb: f64,
    pub screen_resolution: String,
}

pub struct SampleBatch {
    pub timestamp: u64,
    pub device_id: String,
    pub cpu_samples: Vec<CpuSample>,
    pub gpu_samples: Vec<GpuSample>,
    pub metrics: MetricsSnapshot,
}

pub struct ServerInfo {
    pub version: String,
    pub uptime_seconds: u64,
}
```

---

### 18. 性能诊断（需求 163-169, 588-601）

```rust
pub struct PerformanceDiagnosticEngine;

impl PerformanceDiagnosticEngine {
    pub fn new(rules: DiagnosticRuleSet) -> Self
    pub fn add_rule(&mut self, rule: DiagnosticRule) -> ()
    pub fn run(&self, profile: &ProfileData) -> Vec<PerformanceWarning>
}

pub struct DiagnosticRuleSet;

impl DiagnosticRuleSet {
    pub fn default() -> Self
    pub fn all_rules() -> Vec<DiagnosticRule>
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

pub struct PerformanceWarning {
    pub category: String,
    pub message: String,
    pub severity: WarningSeverity,
    pub suggestion: String,
}

impl PerformanceWarning {
    pub fn suggestion_code(&self) -> Option<&str>
    pub fn documentation_url(&self) -> Option<&str>
    pub fn severity_rank(&self) -> u8
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

---

### 19. 基线与回归检测（需求 170-175, 140-143, 602-613）

```rust
pub struct BaselineProfile {
    pub name: String,
    pub samples: ProfileData,
}

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

pub struct Regression {
    pub metric: String,
    pub delta: f64,
    pub p_value: f64,
    pub statistically_significant: bool,
}

pub struct BaselineComparison {
    pub metric_name: String,
    pub baseline_mean: f64,
    pub new_mean: f64,
    pub delta_percent: f64,
    pub is_regression: bool,
}

pub struct RegressionDetector;

impl RegressionDetector {
    pub fn t_test(&self, baseline: &[f64], samples: &[f64], alpha: f64) -> bool
    pub fn threshold_check(baseline: f64, samples: &[f64], percent_threshold: f64) -> bool
}
```

---

### 20. Profile 格式（需求 144-148, 171-174, 652-669）

`.rgeprofile` 文件格式：
- 头部：magic / version / compression / encryption_flag / metadata_len
- 压缩：zstd / gzip / none
- 可选 AES-256-GCM 加密

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

pub struct ProfileMetadata {
    pub engine_version: String,
    pub captured_at: DateTime,
    pub device_info: DeviceInfo,
    pub total_samples: u64,
    pub duration_seconds: f64,
}

pub struct ProfileData {
    pub frames: Vec<FrameSamples>,
    pub cpu_samples: Vec<CpuSample>,
    pub gpu_samples: Vec<GpuSample>,
    pub memory_samples: Vec<MemorySample>,
    pub events: Vec<Event>,
    pub symbols: Vec<Symbol>,
}

pub struct ProfileSummary {
    pub total_frames: u64,
    pub avg_fps: f64,
    pub avg_frame_time_ms: f64,
    pub total_samples: u64,
    pub file_size_bytes: u64,
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

---

## 验收标准

### 功能验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-1 | `Profiler::begin_frame/end_frame` 正常记录帧 | 单元测试 |
| AC-2 | `ScopeGuard` 自动管理 scope 生命周期 | 单元测试 |
| AC-3 | `FlameGraph::from_samples` 正确生成火焰图 | 单元测试 |
| AC-4 | `Timeline` 正确渲染时间轴 | 集成测试 |
| AC-5 | `Histogram` 统计计算正确 | 单元测试 |
| AC-6 | `RemoteProfilerServer/Client` 远程通信正常 | 集成测试 |
| AC-7 | `.rgeprofile` 格式导入导出正常 | 单元测试 |
| AC-8 | 基线比较与回归检测正常工作 | 集成测试 |

### 示例验收

| 示例 | 验收条件 |
|------|----------|
| `profiler_window` | 打开窗口显示 CPU/GPU/内存/渲染多 Tab |
| `profiler_window` | Flame Graph 可视化展示 |
| `profiler_window` | Timeline 时间轴展示 |
| `profiler_window` | FPS/帧时间折线图实时刷新 |
| `profiler_window` | 点击帧可跳转到具体帧详情 |
| `profiler_remote` | 启动远程采样服务器，移动设备连接 |
| `profiler_remote` | 桌面端接收远程样本并显示 |
| `profiler_bench` | 创建基准并与 baseline 比较 |
| `profiler_bench` | 导出 .rgeprofile 文件 |
| `profiler_bench` | 检测回归并输出报告 |

---

## 依赖关系

### 内部依赖

- `engine-core`: 基础类型定义
- `engine-render`: 渲染相关指标
- `engine-ecs`: ECS 系统采样

### 外部依赖

- `serde`: 序列化
- `zstd`: 压缩
- `aes-gcm`: 加密
- `parking_lot`: 同步

---

## 优先级说明

- **P0**: 核心功能，MVP 必须包含
- **P1**: 重要功能，下一迭代包含
- **P2**: 增强功能，后续迭代包含
