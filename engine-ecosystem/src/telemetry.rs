//! 遥测系统

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// 遥测配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    endpoint: String,
    enabled: bool,
    sample_interval_ms: u64,
    max_buffer_size: usize,
}

impl TelemetryConfig {
    pub fn new(endpoint: String, enabled: bool) -> Self {
        Self {
            endpoint,
            enabled,
            sample_interval_ms: 1000,
            max_buffer_size: 1000,
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn sample_interval_ms(&self) -> u64 {
        self.sample_interval_ms
    }

    pub fn max_buffer_size(&self) -> usize {
        self.max_buffer_size
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self::new(crate::TELEMETRY_ENDPOINT.to_string(), true)
    }
}

/// 遥测系统
pub struct Telemetry {
    config: TelemetryConfig,
    buffer: Arc<RwLock<Vec<TelemetryEvent>>>,
    device_info: DeviceInfo,
    session_id: String,
}

impl Telemetry {
    pub fn new(config: TelemetryConfig) -> Self {
        Self {
            config,
            buffer: Arc::new(RwLock::new(Vec::new())),
            device_info: DeviceInfo::default(),
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn config(&self) -> &TelemetryConfig {
        &self.config
    }

    pub fn enable(&mut self) {
        self.config.enabled = true;
    }

    pub fn disable(&mut self) {
        self.config.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// 记录事件
    pub fn record(&mut self, event: TelemetryEvent) {
        if !self.config.enabled {
            return;
        }

        let mut buffer = self.buffer.write();
        if buffer.len() >= self.config.max_buffer_size {
            buffer.remove(0);
        }
        buffer.push(event);
    }

    /// 记录性能事件
    pub fn record_performance(&mut self, metrics: crate::profiler::Metrics) {
        self.record(TelemetryEvent::Performance {
            timestamp: Utc::now(),
            session_id: self.session_id.clone(),
            metrics,
        });
    }

    /// 记录错误事件
    pub fn record_error(&mut self, error: ErrorEvent) {
        self.record(TelemetryEvent::Error {
            timestamp: Utc::now(),
            session_id: self.session_id.clone(),
            error,
        });
    }

    /// 记录用户行为事件
    pub fn record_user_action(&mut self, action: UserAction) {
        self.record(TelemetryEvent::UserAction {
            timestamp: Utc::now(),
            session_id: self.session_id.clone(),
            action,
        });
    }

    /// 获取缓冲区大小
    pub fn buffer_size(&self) -> usize {
        self.buffer.read().len()
    }

    /// 清空缓冲区
    pub fn clear_buffer(&mut self) {
        self.buffer.write().clear();
    }

    /// 获取所有事件
    pub fn events(&self) -> Vec<TelemetryEvent> {
        self.buffer.read().clone()
    }

    /// 获取设备信息
    pub fn device_info(&self) -> &DeviceInfo {
        &self.device_info
    }

    /// 设置设备信息
    pub fn set_device_info(&mut self, info: DeviceInfo) {
        self.device_info = info;
    }

    /// 获取会话 ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// 刷新事件到远程服务器（模拟）
    pub fn flush(&mut self) -> anyhow::Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // 简化实现，实际需要发送 HTTP 请求
        self.clear_buffer();
        Ok(())
    }

    /// 生成遥测报告
    pub fn generate_report(&self) -> TelemetryReport {
        let events = self.events();
        let performance_events = events
            .iter()
            .filter(|e| matches!(e, TelemetryEvent::Performance { .. }))
            .count();
        let error_events = events
            .iter()
            .filter(|e| matches!(e, TelemetryEvent::Error { .. }))
            .count();
        let user_action_events = events
            .iter()
            .filter(|e| matches!(e, TelemetryEvent::UserAction { .. }))
            .count();

        TelemetryReport {
            session_id: self.session_id.clone(),
            device_info: self.device_info.clone(),
            total_events: events.len(),
            performance_events,
            error_events,
            user_action_events,
            start_time: Utc::now(),
        }
    }
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::new(TelemetryConfig::default())
    }
}

/// 遥测事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TelemetryEvent {
    Performance {
        timestamp: DateTime<Utc>,
        session_id: String,
        metrics: crate::profiler::Metrics,
    },
    Error {
        timestamp: DateTime<Utc>,
        session_id: String,
        error: ErrorEvent,
    },
    UserAction {
        timestamp: DateTime<Utc>,
        session_id: String,
        action: UserAction,
    },
    System {
        timestamp: DateTime<Utc>,
        session_id: String,
        event: SystemEvent,
    },
}

/// 错误事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub error_type: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub severity: ErrorSeverity,
}

impl ErrorEvent {
    pub fn new(error_type: String, message: String) -> Self {
        Self {
            error_type,
            message,
            stack_trace: None,
            file: None,
            line: None,
            severity: ErrorSeverity::Error,
        }
    }

    pub fn with_stack_trace(mut self, stack_trace: String) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }

    pub fn with_location(mut self, file: String, line: u32) -> Self {
        self.file = Some(file);
        self.line = Some(line);
        self
    }
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
    Fatal,
}

/// 用户行为事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAction {
    pub action_type: String,
    pub target: String,
    pub details: HashMap<String, String>,
}

impl UserAction {
    pub fn new(action_type: String, target: String) -> Self {
        Self {
            action_type,
            target,
            details: HashMap::new(),
        }
    }

    pub fn with_detail(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }
}

/// 系统事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_type: String,
    pub data: HashMap<String, serde_json::Value>,
}

impl SystemEvent {
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            data: HashMap::new(),
        }
    }

    pub fn with_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.data.insert(key, value);
        self
    }
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_name: String,
    pub os: String,
    pub os_version: String,
    pub cpu_brand: String,
    pub gpu_brand: String,
    pub ram_gb: f64,
    pub screen_resolution: String,
    pub engine_version: String,
}

impl DeviceInfo {
    pub fn new() -> Self {
        Self {
            device_name: "Unknown".to_string(),
            os: "Unknown".to_string(),
            os_version: "Unknown".to_string(),
            cpu_brand: "Unknown".to_string(),
            gpu_brand: "Unknown".to_string(),
            ram_gb: 0.0,
            screen_resolution: "Unknown".to_string(),
            engine_version: "1.0.0".to_string(),
        }
    }

    pub fn detect() -> Self {
        Self {
            device_name: std::env::consts::ARCH.to_string(),
            os: std::env::consts::OS.to_string(),
            os_version: "Unknown".to_string(),
            cpu_brand: "Unknown".to_string(),
            gpu_brand: "Unknown".to_string(),
            ram_gb: 0.0,
            screen_resolution: "Unknown".to_string(),
            engine_version: "1.0.0".to_string(),
        }
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self::detect()
    }
}

/// 遥测报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryReport {
    pub session_id: String,
    pub device_info: DeviceInfo,
    pub total_events: usize,
    pub performance_events: usize,
    pub error_events: usize,
    pub user_action_events: usize,
    pub start_time: DateTime<Utc>,
}

/// 遥测统计
pub struct TelemetryStats {
    events_by_type: HashMap<String, usize>,
}

impl TelemetryStats {
    pub fn from_events(events: &[TelemetryEvent]) -> Self {
        let mut events_by_type = HashMap::new();

        for event in events {
            let type_name = match event {
                TelemetryEvent::Performance { .. } => "Performance",
                TelemetryEvent::Error { .. } => "Error",
                TelemetryEvent::UserAction { .. } => "UserAction",
                TelemetryEvent::System { .. } => "System",
            };
            events_by_type
                .entry(type_name.to_string())
                .or_insert(0usize);
            *events_by_type.get_mut(type_name).unwrap() += 1;
        }

        Self { events_by_type }
    }

    pub fn events_by_type(&self) -> &HashMap<String, usize> {
        &self.events_by_type
    }

    pub fn total_events(&self) -> usize {
        self.events_by_type.values().sum()
    }
}

/// 遥测聚合器
pub struct TelemetryAggregator {
    events: Vec<TelemetryEvent>,
}

impl TelemetryAggregator {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add(&mut self, event: TelemetryEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[TelemetryEvent] {
        &self.events
    }

    pub fn stats(&self) -> TelemetryStats {
        TelemetryStats::from_events(&self.events)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for TelemetryAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// 远程遥测客户端
pub struct RemoteTelemetryClient {
    #[allow(dead_code)]
    endpoint: String,
    connected: bool,
}

impl RemoteTelemetryClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            connected: false,
        }
    }

    pub fn connect(&mut self) -> anyhow::Result<()> {
        // 简化实现
        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn send(&mut self, _events: &[TelemetryEvent]) -> anyhow::Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected"));
        }
        // 简化实现，实际需要 HTTP POST
        Ok(())
    }
}

/// 远程遥测服务器（模拟）
pub struct RemoteTelemetryServer {
    #[allow(dead_code)]
    port: u16,
    running: bool,
    received_events: Vec<TelemetryEvent>,
}

impl RemoteTelemetryServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            running: false,
            received_events: Vec::new(),
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn received_events(&self) -> &[TelemetryEvent] {
        &self.received_events
    }

    pub fn receive(&mut self, events: Vec<TelemetryEvent>) {
        self.received_events.extend(events);
    }
}
