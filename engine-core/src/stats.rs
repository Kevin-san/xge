/// 每帧统计信息
#[derive(Debug, Clone, Default)]
pub struct FrameStats {
    pub frame_number: u64,
    pub dt: f64,
    pub cpu_time_us: u64,
}

/// 引擎全局统计
#[derive(Debug, Clone, Default)]
pub struct EngineStats {
    pub uptime_seconds: f64,
    pub total_frames: u64,
    pub avg_fps: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_stats_default() {
        let stats = FrameStats::default();
        assert_eq!(stats.frame_number, 0);
        assert_eq!(stats.dt, 0.0);
        assert_eq!(stats.cpu_time_us, 0);
    }

    #[test]
    fn test_engine_stats_default() {
        let stats = EngineStats::default();
        assert_eq!(stats.uptime_seconds, 0.0);
        assert_eq!(stats.total_frames, 0);
        assert_eq!(stats.avg_fps, 0.0);
    }

    #[test]
    fn test_frame_stats_fields() {
        let stats = FrameStats {
            frame_number: 100,
            dt: 0.016,
            cpu_time_us: 500,
        };
        assert_eq!(stats.frame_number, 100);
        assert_eq!(stats.dt, 0.016);
        assert_eq!(stats.cpu_time_us, 500);
    }
}
