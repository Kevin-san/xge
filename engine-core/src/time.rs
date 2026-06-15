use std::time::{Duration, Instant};

/// 时间管理器
pub struct Time {
    last_frame: Instant,
    delta_time: Duration,
    total_time: Duration,
    frame_count: u64,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_frame: now,
            delta_time: Duration::from_secs(0),
            total_time: Duration::from_secs(0),
            frame_count: 0,
        }
    }

    /// 更新内部时间状态
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame);
        self.total_time += self.delta_time;
        self.last_frame = now;
        self.frame_count += 1;
    }

    /// 获取上一帧的时间增量（秒）
    pub fn delta_time(&self) -> f64 {
        self.delta_time.as_secs_f64()
    }

    /// 获取上一帧的时间增量（毫秒）
    pub fn delta_time_ms(&self) -> f64 {
        self.delta_time.as_secs_f64() * 1000.0
    }

    /// 获取从引擎启动以来的总时间（秒）
    pub fn total_time(&self) -> f64 {
        self.total_time.as_secs_f64()
    }

    /// 获取当前帧数
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}
