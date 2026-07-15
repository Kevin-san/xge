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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_time_initial_state() {
        let time = Time::new();
        assert_eq!(time.frame_count(), 0);
        assert_eq!(time.delta_time(), 0.0);
        assert_eq!(time.total_time(), 0.0);
    }

    #[test]
    fn test_time_update() {
        let mut time = Time::new();

        time.update();
        assert_eq!(time.frame_count(), 1);
        assert!(time.delta_time() >= 0.0);
        assert!(time.total_time() >= 0.0);

        thread::sleep(Duration::from_millis(10));
        time.update();
        assert_eq!(time.frame_count(), 2);
        assert!(time.delta_time() > 0.0);
    }

    #[test]
    fn test_time_delta_time_ms() {
        let mut time = Time::new();

        thread::sleep(Duration::from_millis(50));
        time.update();

        assert!(time.delta_time_ms() >= 50.0);
        assert!(time.delta_time_ms() < 100.0);
    }

    #[test]
    fn test_time_total_time() {
        let mut time = Time::new();

        thread::sleep(Duration::from_millis(10));
        time.update();
        let t1 = time.total_time();

        thread::sleep(Duration::from_millis(10));
        time.update();
        let t2 = time.total_time();

        assert!(t2 > t1);
        assert!(t2 - t1 >= 0.01);
    }

    #[test]
    fn test_time_multiple_updates() {
        let mut time = Time::new();

        for i in 1..=10 {
            time.update();
            assert_eq!(time.frame_count(), i as u64);
        }

        assert_eq!(time.frame_count(), 10);
    }
}
