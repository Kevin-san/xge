//! 引擎统计信息 — 每帧统计和全局统计

/// 每帧统计信息
#[derive(Debug, Clone, Default)]
pub struct FrameStats {
    pub frame_number: u64,
    pub dt_seconds: f64,
    pub cpu_time_us: u64,
}

/// 全局引擎统计信息
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub total_frames: u64,
    pub uptime_seconds: f64,
    pub avg_fps: f64,
}

impl Default for EngineStats {
    fn default() -> Self {
        Self {
            total_frames: 0,
            uptime_seconds: 0.0,
            avg_fps: 0.0,
        }
    }
}
