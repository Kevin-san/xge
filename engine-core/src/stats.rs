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

    #[test]
    fn test_frame_stats_with_values() {
        let stats = FrameStats {
            frame_number: 1,
            dt: 0.016,
            cpu_time_us: 100,
        };
        assert_eq!(stats.frame_number, 1);
        assert_eq!(stats.dt, 0.016);
        assert_eq!(stats.cpu_time_us, 100);
    }

    #[test]
    fn test_engine_stats_with_values() {
        let stats = EngineStats {
            uptime_seconds: 10.0,
            total_frames: 600,
            avg_fps: 60.0,
        };
        assert_eq!(stats.uptime_seconds, 10.0);
        assert_eq!(stats.total_frames, 600);
        assert_eq!(stats.avg_fps, 60.0);
    }

    #[test]
    fn test_frame_stats_clone() {
        let stats = FrameStats {
            frame_number: 5,
            dt: 0.033,
            cpu_time_us: 200,
        };
        let cloned = stats.clone();
        assert_eq!(cloned.frame_number, 5);
    }

    #[test]
    fn test_engine_stats_clone() {
        let stats = EngineStats {
            uptime_seconds: 5.0,
            total_frames: 300,
            avg_fps: 60.0,
        };
        let cloned = stats.clone();
        assert_eq!(cloned.total_frames, 300);
    }
}
