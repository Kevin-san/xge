#[derive(Debug, Clone, Copy, Default)]
pub struct FrameStats {
    pub frame_time: f64,
    pub update_time: f64,
    pub render_time: f64,
    pub fps: f64,
    pub frame_count: u64,
}

impl FrameStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_fps(&mut self, frame_time: f64) {
        self.frame_time = frame_time;
        self.fps = if frame_time > 0.0 {
            1.0 / frame_time
        } else {
            0.0
        };
        self.frame_count += 1;
    }
}

#[derive(Debug, Clone, Default)]
pub struct EngineStats {
    pub frame_stats: FrameStats,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub uptime: f64,
    pub module_count: usize,
    pub loaded_assets: usize,
}

impl EngineStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, frame_time: f64, uptime: f64) {
        self.frame_stats.update_fps(frame_time);
        self.uptime = uptime;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_stats_new() {
        let stats = FrameStats::new();
        assert_eq!(stats.frame_time, 0.0);
        assert_eq!(stats.fps, 0.0);
        assert_eq!(stats.frame_count, 0);
    }

    #[test]
    fn test_frame_stats_update_fps() {
        let mut stats = FrameStats::new();
        stats.update_fps(0.016666);
        assert!((stats.fps - 60.0).abs() < 1.0);
        assert_eq!(stats.frame_count, 1);
    }

    #[test]
    fn test_engine_stats_new() {
        let stats = EngineStats::new();
        assert_eq!(stats.frame_stats.frame_count, 0);
        assert_eq!(stats.memory_usage, 0);
        assert_eq!(stats.cpu_usage, 0.0);
    }

    #[test]
    fn test_engine_stats_update() {
        let mut stats = EngineStats::new();
        stats.update(0.016666, 10.0);
        assert!((stats.frame_stats.fps - 60.0).abs() < 1.0);
        assert_eq!(stats.uptime, 10.0);
    }
}
