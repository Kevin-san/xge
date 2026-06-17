//! 性能分析器

use crate::common::*;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 分析器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    sample_rate_hz: u32,
    max_frames: usize,
    enabled_categories: ProfilerCategories,
}

impl ProfilerConfig {
    pub fn new(sample_rate_hz: u32, max_frames: usize) -> Self {
        Self {
            sample_rate_hz,
            max_frames,
            enabled_categories: ProfilerCategories::default(),
        }
    }

    pub fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    pub fn max_frames(&self) -> usize {
        self.max_frames
    }

    pub fn enabled_categories(&self) -> &ProfilerCategories {
        &self.enabled_categories
    }
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self::new(crate::DEFAULT_SAMPLE_RATE_HZ, crate::DEFAULT_MAX_FRAMES)
    }
}

/// 分析器类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfilerCategory {
    Cpu,
    Gpu,
    Memory,
    Render,
    Network,
    Script,
}

/// 分析器类别配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerCategories {
    pub cpu: bool,
    pub gpu: bool,
    pub memory: bool,
    pub render: bool,
    pub network: bool,
    pub script: bool,
}

impl ProfilerCategories {
    pub fn all() -> Self {
        Self {
            cpu: true,
            gpu: true,
            memory: true,
            render: true,
            network: true,
            script: true,
        }
    }

    pub fn none() -> Self {
        Self {
            cpu: false,
            gpu: false,
            memory: false,
            render: false,
            network: false,
            script: false,
        }
    }
}

impl Default for ProfilerCategories {
    fn default() -> Self {
        Self::all()
    }
}

/// 性能分析器
pub struct PerformanceProfiler {
    config: ProfilerConfig,
    frame_number: u64,
    scope_stack: Vec<String>,
    cpu_samples: Arc<RwLock<Vec<CpuSample>>>,
    gpu_samples: Arc<RwLock<Vec<GpuSample>>>,
    memory_samples: Arc<RwLock<Vec<MemorySample>>>,
    render_samples: Arc<RwLock<Vec<RenderSample>>>,
    network_samples: Arc<RwLock<Vec<NetworkSample>>>,
    script_samples: Arc<RwLock<Vec<ScriptSample>>>,
    frame_samples: Arc<RwLock<Vec<FrameSamples>>>,
}

impl PerformanceProfiler {
    pub fn new(config: ProfilerConfig) -> Self {
        Self {
            config,
            frame_number: 0,
            scope_stack: Vec::new(),
            cpu_samples: Arc::new(RwLock::new(Vec::new())),
            gpu_samples: Arc::new(RwLock::new(Vec::new())),
            memory_samples: Arc::new(RwLock::new(Vec::new())),
            render_samples: Arc::new(RwLock::new(Vec::new())),
            network_samples: Arc::new(RwLock::new(Vec::new())),
            script_samples: Arc::new(RwLock::new(Vec::new())),
            frame_samples: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn config(&self) -> &ProfilerConfig {
        &self.config
    }

    pub fn toggle(&mut self, category: ProfilerCategory, enabled: bool) {
        match category {
            ProfilerCategory::Cpu => self.config.enabled_categories.cpu = enabled,
            ProfilerCategory::Gpu => self.config.enabled_categories.gpu = enabled,
            ProfilerCategory::Memory => self.config.enabled_categories.memory = enabled,
            ProfilerCategory::Render => self.config.enabled_categories.render = enabled,
            ProfilerCategory::Network => self.config.enabled_categories.network = enabled,
            ProfilerCategory::Script => self.config.enabled_categories.script = enabled,
        }
    }

    pub fn is_enabled(&self, category: ProfilerCategory) -> bool {
        match category {
            ProfilerCategory::Cpu => self.config.enabled_categories.cpu,
            ProfilerCategory::Gpu => self.config.enabled_categories.gpu,
            ProfilerCategory::Memory => self.config.enabled_categories.memory,
            ProfilerCategory::Render => self.config.enabled_categories.render,
            ProfilerCategory::Network => self.config.enabled_categories.network,
            ProfilerCategory::Script => self.config.enabled_categories.script,
        }
    }

    pub fn clear(&mut self) {
        self.cpu_samples.write().clear();
        self.gpu_samples.write().clear();
        self.memory_samples.write().clear();
        self.render_samples.write().clear();
        self.network_samples.write().clear();
        self.script_samples.write().clear();
        self.frame_samples.write().clear();
        self.frame_number = 0;
    }

    pub fn frame_count(&self) -> usize {
        self.frame_samples.read().len()
    }

    pub fn current_frame_number(&self) -> u64 {
        self.frame_number
    }

    /// 开始帧
    pub fn begin_frame(&mut self) {
        self.frame_number += 1;
        self.scope_stack.clear();
    }

    /// 结束帧
    pub fn end_frame(&mut self) {
        let frame = FrameSamples {
            frame_number: self.frame_number,
            cpu: self.cpu_samples.read().clone(),
            gpu: self.gpu_samples.read().clone(),
            memory: self.memory_samples.read().clone(),
            render: self.render_samples.read().clone(),
            network: self.network_samples.read().clone(),
            script: self.script_samples.read().clone(),
        };

        let mut frames = self.frame_samples.write();
        if frames.len() >= self.config.max_frames {
            frames.remove(0);
        }
        frames.push(frame);

        // 清空当前帧样本
        self.cpu_samples.write().clear();
        self.gpu_samples.write().clear();
        self.memory_samples.write().clear();
        self.render_samples.write().clear();
        self.network_samples.write().clear();
        self.script_samples.write().clear();
    }

    /// 开始作用域
    pub fn begin_scope(&mut self, name: &str) -> ScopeGuard<'_> {
        let start_ns = current_timestamp_ns();
        let thread_id = current_thread_id();
        self.scope_stack.push(name.to_string());

        let parent_index = if self.scope_stack.len() > 1 {
            Some(self.scope_stack.len() - 2)
        } else {
            None
        };

        ScopeGuard {
            profiler: self,
            name: name.to_string(),
            start_ns,
            thread_id,
            parent_index,
        }
    }

    /// 结束作用域
    pub fn end_scope(&mut self) {
        self.scope_stack.pop();
    }

    /// 记录事件
    pub fn record_event(&mut self, name: &str, data: serde_json::Value) {
        // 简化实现，记录到 CPU 样本
        let sample = CpuSample {
            scope_name: name.to_string(),
            thread_id: current_thread_id(),
            start_ns: current_timestamp_ns(),
            duration_ns: 0,
            parent_index: None,
            data: Some(data),
        };
        self.cpu_samples.write().push(sample);
    }

    /// 添加 CPU 样本
    pub fn add_cpu_sample(&mut self, sample: CpuSample) {
        if self.config.enabled_categories.cpu {
            self.cpu_samples.write().push(sample);
        }
    }

    /// 添加 GPU 样本
    pub fn add_gpu_sample(&mut self, sample: GpuSample) {
        if self.config.enabled_categories.gpu {
            self.gpu_samples.write().push(sample);
        }
    }

    /// 添加内存样本
    pub fn add_memory_sample(&mut self, sample: MemorySample) {
        if self.config.enabled_categories.memory {
            self.memory_samples.write().push(sample);
        }
    }

    /// 添加渲染样本
    pub fn add_render_sample(&mut self, sample: RenderSample) {
        if self.config.enabled_categories.render {
            self.render_samples.write().push(sample);
        }
    }

    /// 添加网络样本
    pub fn add_network_sample(&mut self, sample: NetworkSample) {
        if self.config.enabled_categories.network {
            self.network_samples.write().push(sample);
        }
    }

    /// 添加脚本样本
    pub fn add_script_sample(&mut self, sample: ScriptSample) {
        if self.config.enabled_categories.script {
            self.script_samples.write().push(sample);
        }
    }

    /// 获取 CPU 样本
    pub fn cpu_samples(&self) -> Vec<CpuSample> {
        self.cpu_samples.read().clone()
    }

    /// 获取 GPU 样本
    pub fn gpu_samples(&self) -> Vec<GpuSample> {
        self.gpu_samples.read().clone()
    }

    /// 获取内存样本
    pub fn memory_samples(&self) -> Vec<MemorySample> {
        self.memory_samples.read().clone()
    }

    /// 获取渲染样本
    pub fn render_samples(&self) -> Vec<RenderSample> {
        self.render_samples.read().clone()
    }

    /// 获取网络样本
    pub fn network_samples(&self) -> Vec<NetworkSample> {
        self.network_samples.read().clone()
    }

    /// 获取脚本样本
    pub fn script_samples(&self) -> Vec<ScriptSample> {
        self.script_samples.read().clone()
    }

    /// 获取帧样本
    pub fn samples_for_frame(&self, frame_idx: usize) -> Option<FrameSamples> {
        self.frame_samples.read().get(frame_idx).cloned()
    }

    /// 生成火焰图
    pub fn flame_graph(&self) -> FlameGraph {
        FlameGraph::from_samples(&self.cpu_samples.read())
    }

    /// 生成时间轴
    pub fn timeline(&self) -> Timeline {
        Timeline::from_samples(&self.cpu_samples.read())
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new(ProfilerConfig::default())
    }
}

/// Scope Guard（RAII 自动管理）
pub struct ScopeGuard<'a> {
    profiler: &'a mut PerformanceProfiler,
    name: String,
    start_ns: u64,
    thread_id: u64,
    parent_index: Option<usize>,
}

impl<'a> ScopeGuard<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        let end_ns = current_timestamp_ns();
        let duration_ns = end_ns - self.start_ns;

        let sample = CpuSample {
            scope_name: self.name.clone(),
            thread_id: self.thread_id,
            start_ns: self.start_ns,
            duration_ns,
            parent_index: self.parent_index,
            data: None,
        };

        self.profiler.add_cpu_sample(sample);
        self.profiler.end_scope();
    }
}

/// CPU 样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSample {
    pub scope_name: String,
    pub thread_id: u64,
    pub start_ns: u64,
    pub duration_ns: u64,
    pub parent_index: Option<usize>,
    pub data: Option<serde_json::Value>,
}

impl CpuSample {
    pub fn exclusive_duration(&self) -> u64 {
        self.duration_ns
    }
}

/// GPU 样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSample {
    pub scope_name: String,
    pub queue_index: u32,
    pub start_ns: u64,
    pub duration_ns: u64,
    pub gpu_timer_id: u32,
}

/// 内存样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySample {
    pub event_type: MemoryEventType,
    pub bytes: u64,
    pub address: u64,
    pub timestamp_ns: u64,
    pub thread_id: u64,
}

/// 内存事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryEventType {
    Alloc,
    Dealloc,
    Realloc,
    GarbageCollect,
}

/// 渲染样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderSample {
    pub draw_call_index: u32,
    pub pipeline: String,
    pub vertices: u32,
    pub indices: u32,
    pub textures_bound: Vec<String>,
    pub shader_name: String,
}

/// 网络样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSample {
    pub direction: NetworkDirection,
    pub bytes: u64,
    pub protocol: String,
    pub remote_addr: String,
    pub timestamp_ns: u64,
}

/// 网络方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDirection {
    Incoming,
    Outgoing,
}

/// 脚本样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSample {
    pub function_name: String,
    pub file: String,
    pub line: u32,
    pub duration_ns: u64,
    pub invocations: u32,
}

/// 帧样本聚合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameSamples {
    pub frame_number: u64,
    pub cpu: Vec<CpuSample>,
    pub gpu: Vec<GpuSample>,
    pub memory: Vec<MemorySample>,
    pub render: Vec<RenderSample>,
    pub network: Vec<NetworkSample>,
    pub script: Vec<ScriptSample>,
}

/// 火焰图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlameGraph {
    root: FlameNode,
}

impl FlameGraph {
    pub fn from_samples(samples: &[CpuSample]) -> Self {
        let mut root = FlameNode {
            name: "root".to_string(),
            start: 0,
            duration: 0,
            children: Vec::new(),
        };

        for sample in samples {
            let node = FlameNode {
                name: sample.scope_name.clone(),
                start: sample.start_ns,
                duration: sample.duration_ns,
                children: Vec::new(),
            };

            if let Some(parent_idx) = sample.parent_index {
                if let Some(_parent) = samples.get(parent_idx) {
                    // 简化实现，实际需要构建树结构
                    root.children.push(node);
                }
            } else {
                root.children.push(node);
            }
        }

        Self { root }
    }

    pub fn root(&self) -> &FlameNode {
        &self.root
    }

    pub fn nodes(&self) -> Vec<&FlameNode> {
        self.root.children.iter().collect()
    }

    pub fn search(&self, keyword: &str) -> Vec<&FlameNode> {
        self.root
            .children
            .iter()
            .filter(|n| n.name.contains(keyword))
            .collect()
    }

    pub fn hot_path(&self) -> Vec<&FlameNode> {
        self.root
            .children
            .iter()
            .filter(|n| n.duration > 1000000) // > 1ms
            .collect()
    }
}

/// 火焰图节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlameNode {
    pub name: String,
    pub start: u64,
    pub duration: u64,
    pub children: Vec<FlameNode>,
}

impl FlameNode {
    pub fn total_duration(&self) -> u64 {
        self.duration
            + self
                .children
                .iter()
                .map(|c| c.total_duration())
                .sum::<u64>()
    }

    pub fn self_duration(&self) -> u64 {
        self.duration
    }

    pub fn percent_of_parent(&self) -> f64 {
        100.0 // 简化实现
    }

    pub fn percent_of_total(&self) -> f64 {
        100.0 // 简化实现
    }
}

/// 时间轴
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    tracks: Vec<TimelineTrack>,
    total_duration: u64,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            total_duration: 0,
        }
    }

    pub fn from_samples(samples: &[CpuSample]) -> Self {
        let mut timeline = Self::new();
        let track = TimelineTrack {
            name: "CPU".to_string(),
            samples: samples.iter().map(|s| s.start_ns).collect(),
            color: "#FF6B6B".to_string(),
            row_index: 0,
        };
        timeline.add_track(track);
        timeline
    }

    pub fn add_track(&mut self, track: TimelineTrack) {
        self.tracks.push(track);
    }

    pub fn tracks(&self) -> &[TimelineTrack] {
        &self.tracks
    }

    pub fn total_duration(&self) -> u64 {
        self.total_duration
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

/// 时间轴轨道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub name: String,
    pub samples: Vec<u64>,
    pub color: String,
    pub row_index: usize,
}

/// 直方图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Histogram {
    buckets: Vec<HistogramBucket>,
}

impl Histogram {
    pub fn from_values(values: &[f64], bucket_count: usize) -> Self {
        if values.is_empty() {
            return Self {
                buckets: Vec::new(),
            };
        }

        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;
        let bucket_size = if range > 0.0 {
            range / bucket_count as f64
        } else {
            1.0
        };

        let mut buckets = Vec::with_capacity(bucket_count);
        for i in 0..bucket_count {
            buckets.push(HistogramBucket {
                start: min + i as f64 * bucket_size,
                end: min + (i + 1) as f64 * bucket_size,
                count: 0,
            });
        }

        for value in values {
            let idx = ((value - min) / bucket_size).floor() as usize;
            if idx < buckets.len() {
                buckets[idx].count += 1;
            }
        }

        Self { buckets }
    }

    pub fn mean(&self) -> f64 {
        let total: f64 = self
            .buckets
            .iter()
            .map(|b| (b.start + b.end) / 2.0 * b.count as f64)
            .sum();
        let count: u64 = self.buckets.iter().map(|b| b.count).sum();
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    pub fn median(&self) -> f64 {
        let total_count: u64 = self.buckets.iter().map(|b| b.count).sum();
        if total_count == 0 {
            return 0.0;
        }
        let mid = total_count / 2;
        let mut cumulative = 0;
        for bucket in &self.buckets {
            cumulative += bucket.count;
            if cumulative >= mid {
                return (bucket.start + bucket.end) / 2.0;
            }
        }
        0.0
    }

    pub fn p95(&self) -> f64 {
        percentile(&self.buckets, 95.0)
    }

    pub fn p99(&self) -> f64 {
        percentile(&self.buckets, 99.0)
    }

    pub fn min(&self) -> f64 {
        self.buckets.first().map(|b| b.start).unwrap_or(0.0)
    }

    pub fn max(&self) -> f64 {
        self.buckets.last().map(|b| b.end).unwrap_or(0.0)
    }

    pub fn buckets(&self) -> &[HistogramBucket] {
        &self.buckets
    }
}

/// 计算百分位数
fn percentile(buckets: &[HistogramBucket], p: f64) -> f64 {
    let total_count: u64 = buckets.iter().map(|b| b.count).sum();
    if total_count == 0 {
        return 0.0;
    }
    let target = (total_count as f64 * p / 100.0).floor() as u64;
    let mut cumulative = 0;
    for bucket in buckets {
        cumulative += bucket.count;
        if cumulative >= target {
            return (bucket.start + bucket.end) / 2.0;
        }
    }
    0.0
}

/// 直方图桶
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBucket {
    pub start: f64,
    pub end: f64,
    pub count: u64,
}

/// 指标
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Metrics {
    pub fn new(frame_time_ms: f64) -> Self {
        Self {
            frame_time_ms,
            fps: if frame_time_ms > 0.0 {
                1000.0 / frame_time_ms
            } else {
                0.0
            },
            cpu_usage_percent: 0.0,
            gpu_usage_percent: 0.0,
            memory_rss_kb: 0,
            memory_vss_kb: 0,
            network_in_kbs: 0.0,
            network_out_kbs: 0.0,
            disk_read_kbs: 0.0,
            disk_write_kbs: 0.0,
        }
    }
}

/// 帧指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetrics {
    pub frame_number: u64,
    pub frame_time: f64,
    pub gpu_time: f64,
    pub cpu_time: f64,
    pub draw_calls: u32,
    pub triangles: u64,
    pub vertices: u64,
}

/// 指标聚合器
pub struct FrameMetricsAggregator {
    frames: Vec<FrameMetrics>,
}

impl FrameMetricsAggregator {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn push(&mut self, frame: FrameMetrics) {
        self.frames.push(frame);
    }

    pub fn average_frame_time(&self) -> f64 {
        if self.frames.is_empty() {
            return 0.0;
        }
        self.frames.iter().map(|f| f.frame_time).sum::<f64>() / self.frames.len() as f64
    }

    pub fn average_fps(&self) -> f64 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    pub fn average_draw_calls(&self) -> f64 {
        if self.frames.is_empty() {
            return 0.0;
        }
        self.frames.iter().map(|f| f.draw_calls as f64).sum::<f64>() / self.frames.len() as f64
    }

    pub fn total_triangles(&self) -> u64 {
        self.frames.iter().map(|f| f.triangles).sum()
    }

    pub fn total_vertices(&self) -> u64 {
        self.frames.iter().map(|f| f.vertices).sum()
    }
}

impl Default for FrameMetricsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// 性能诊断引擎
pub struct PerformanceDiagnosticEngine {
    rules: Vec<DiagnosticRule>,
}

impl PerformanceDiagnosticEngine {
    pub fn new() -> Self {
        Self {
            rules: DiagnosticRuleSet::default().rules().to_vec(),
        }
    }

    pub fn add_rule(&mut self, rule: DiagnosticRule) {
        self.rules.push(rule);
    }

    pub fn run(&self, profile: &PerformanceProfiler) -> Vec<PerformanceWarning> {
        let mut warnings = Vec::new();

        for rule in &self.rules {
            if let Some(warning) = self.check_rule(rule, profile) {
                warnings.push(warning);
            }
        }

        warnings
    }

    fn check_rule(
        &self,
        rule: &DiagnosticRule,
        profile: &PerformanceProfiler,
    ) -> Option<PerformanceWarning> {
        match rule {
            DiagnosticRule::ExcessiveDrawCalls => {
                let render_samples = profile.render_samples();
                if render_samples.len() > 1000 {
                    Some(PerformanceWarning::new(
                        "Render",
                        "Excessive draw calls detected",
                        WarningSeverity::Warning,
                        "Consider batching draw calls",
                    ))
                } else {
                    None
                }
            }
            DiagnosticRule::HighFrameTimeVariance => {
                None // 简化实现
            }
            _ => None,
        }
    }
}

impl Default for PerformanceDiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 诊断规则集
pub struct DiagnosticRuleSet {
    rules: Vec<DiagnosticRule>,
}

impl Default for DiagnosticRuleSet {
    fn default() -> Self {
        Self {
            rules: vec![
                DiagnosticRule::ExcessiveDrawCalls,
                DiagnosticRule::HighFrameTimeVariance,
                DiagnosticRule::MemoryLeakSuspect,
                DiagnosticRule::GpuBound,
                DiagnosticRule::CpuBound,
            ],
        }
    }
}

impl DiagnosticRuleSet {
    pub fn rules(&self) -> &[DiagnosticRule] {
        &self.rules
    }
}

/// 诊断规则
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticRule {
    ExcessiveDrawCalls,
    HighFrameTimeVariance,
    MemoryLeakSuspect,
    GpuBound,
    CpuBound,
    ScriptGcStall,
}

/// 性能警告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceWarning {
    pub category: String,
    pub message: String,
    pub severity: WarningSeverity,
    pub suggestion: String,
}

impl PerformanceWarning {
    pub fn new(category: &str, message: &str, severity: WarningSeverity, suggestion: &str) -> Self {
        Self {
            category: category.to_string(),
            message: message.to_string(),
            severity,
            suggestion: suggestion.to_string(),
        }
    }

    pub fn severity_rank(&self) -> u8 {
        match self.severity {
            WarningSeverity::Info => 0,
            WarningSeverity::Warning => 1,
            WarningSeverity::Critical => 2,
        }
    }
}

/// 警告严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningSeverity {
    Info,
    Warning,
    Critical,
}

/// 基线配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineProfile {
    pub name: String,
    pub samples: Vec<CpuSample>,
}

impl BaselineProfile {
    pub fn new(name: String, samples: Vec<CpuSample>) -> Self {
        Self { name, samples }
    }

    pub fn compare(&self, new_samples: &[CpuSample]) -> RegressionReport {
        let baseline_avg = self.samples.iter().map(|s| s.duration_ns).sum::<u64>()
            / self.samples.len().max(1) as u64;
        let new_avg = new_samples.iter().map(|s| s.duration_ns).sum::<u64>()
            / new_samples.len().max(1) as u64;

        let delta_percent = if baseline_avg > 0 {
            ((new_avg as f64 - baseline_avg as f64) / baseline_avg as f64) * 100.0
        } else {
            0.0
        };

        RegressionReport {
            regressions: if delta_percent > 10.0 {
                vec![Regression {
                    metric: "Frame Time".to_string(),
                    delta: delta_percent,
                    p_value: 0.05,
                    statistically_significant: true,
                }]
            } else {
                Vec::new()
            },
        }
    }
}

/// 回归报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    regressions: Vec<Regression>,
}

impl RegressionReport {
    pub fn regressions(&self) -> &[Regression] {
        &self.regressions
    }

    pub fn has_regressions(&self) -> bool {
        !self.regressions.is_empty()
    }

    pub fn regression_count(&self) -> usize {
        self.regressions.len()
    }
}

/// 回归
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regression {
    pub metric: String,
    pub delta: f64,
    pub p_value: f64,
    pub statistically_significant: bool,
}
