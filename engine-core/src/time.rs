use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct FixedTimestepSteps {
    pub steps: u32,
    pub remainder: f64,
}

/// 简单的秒表工具，用于局部计时
pub struct Stopwatch {
    start: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            start: None,
            elapsed: Duration::from_secs(0),
            running: false,
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.start = Some(Instant::now());
            self.running = true;
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            if let Some(start) = self.start {
                self.elapsed += start.elapsed();
            }
            self.start = None;
            self.running = false;
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = Duration::from_secs(0);
        self.running = false;
    }

    pub fn elapsed(&self) -> Duration {
        if self.running {
            if let Some(start) = self.start {
                self.elapsed + start.elapsed()
            } else {
                self.elapsed
            }
        } else {
            self.elapsed
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

/// 时间管理器
pub struct Time {
    last_frame: Instant,
    delta_time: Duration,
    total_time: Duration,
    frame_count: u64,
    fixed_timestep: f32,
    fixed_accumulator: f64,
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
            fixed_timestep: 1.0 / 60.0,
            fixed_accumulator: 0.0,
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

    /// 获取上一帧的时间增量（秒，f64）
    pub fn delta_time(&self) -> f64 {
        self.delta_time.as_secs_f64()
    }

    /// 获取上一帧的时间增量（秒，f32）
    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.as_secs_f32()
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

    /// 获取时间增量（Duration）
    pub fn delta(&self) -> Duration {
        self.delta_time
    }

    /// 获取启动至今的总时间（Duration）
    pub fn elapsed(&self) -> Duration {
        self.total_time
    }

    /// 设置固定时间步长
    pub fn set_fixed_timestep(&mut self, dt: f32) {
        self.fixed_timestep = dt;
    }

    /// 获取固定时间步长
    pub fn fixed_timestep(&self) -> f32 {
        self.fixed_timestep
    }

    /// 更新固定时间步长累加器，返回需要执行的固定步数
    pub fn update_fixed(&mut self) -> FixedTimestepSteps {
        self.fixed_accumulator += self.delta_time();
        let step_duration = self.fixed_timestep as f64;
        let steps = (self.fixed_accumulator / step_duration) as u32;
        self.fixed_accumulator -= steps as f64 * step_duration;
        FixedTimestepSteps {
            steps,
            remainder: self.fixed_accumulator,
        }
    }

    /// 获取固定步长累加器的剩余时间
    pub fn fixed_remainder(&self) -> f64 {
        self.fixed_accumulator
    }

    /// 获取 FPS（基于最近一帧的 dt）
    pub fn fps(&self) -> f32 {
        let dt = self.delta_seconds();
        if dt > 0.0 {
            1.0 / dt
        } else {
            0.0
        }
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

    #[test]
    fn test_time_delta_seconds() {
        let mut time = Time::new();
        assert_eq!(time.delta_seconds(), 0.0);
        
        thread::sleep(Duration::from_millis(100));
        time.update();
        assert!(time.delta_seconds() >= 0.1);
    }

    #[test]
    fn test_time_delta() {
        let mut time = Time::new();
        assert_eq!(time.delta(), Duration::from_secs(0));
        
        time.update();
        assert!(time.delta() >= Duration::from_secs(0));
    }

    #[test]
    fn test_time_elapsed() {
        let mut time = Time::new();
        assert_eq!(time.elapsed(), Duration::from_secs(0));
        
        thread::sleep(Duration::from_millis(50));
        time.update();
        assert!(time.elapsed() >= Duration::from_millis(50));
    }

    #[test]
    fn test_time_fixed_timestep() {
        let mut time = Time::new();
        assert!((time.fixed_timestep() - 1.0/60.0).abs() < 1e-6);
        
        time.set_fixed_timestep(0.02);
        assert!((time.fixed_timestep() - 0.02).abs() < 1e-6);
    }

    #[test]
    fn test_time_update_fixed() {
        let mut time = Time::new();
        time.set_fixed_timestep(0.016);
        
        time.update();
        let steps = time.update_fixed();
        assert_eq!(steps.steps, 0);
        
        time.update();
        let steps = time.update_fixed();
        assert!(steps.steps >= 0);
    }

    #[test]
    fn test_time_fps() {
        let mut time = Time::new();
        assert_eq!(time.fps(), 0.0);
        
        thread::sleep(Duration::from_millis(16));
        time.update();
        let fps = time.fps();
        assert!(fps > 0.0);
        assert!(fps < 100.0);
    }

    #[test]
    fn test_stopwatch_new() {
        let sw = Stopwatch::new();
        assert!(!sw.is_running());
        assert_eq!(sw.elapsed(), Duration::from_secs(0));
    }

    #[test]
    fn test_stopwatch_start_stop() {
        let mut sw = Stopwatch::new();
        
        sw.start();
        assert!(sw.is_running());
        
        thread::sleep(Duration::from_millis(10));
        
        sw.stop();
        assert!(!sw.is_running());
        assert!(sw.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_stopwatch_reset() {
        let mut sw = Stopwatch::new();
        
        sw.start();
        thread::sleep(Duration::from_millis(10));
        sw.stop();
        
        let elapsed = sw.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
        
        sw.reset();
        assert!(!sw.is_running());
        assert_eq!(sw.elapsed(), Duration::from_secs(0));
    }

    #[test]
    fn test_stopwatch_elapsed_secs() {
        let mut sw = Stopwatch::new();
        
        sw.start();
        thread::sleep(Duration::from_millis(100));
        sw.stop();
        
        assert!(sw.elapsed_secs() >= 0.1);
    }

    #[test]
    fn test_stopwatch_pause_resume() {
        let mut sw = Stopwatch::new();
        
        sw.start();
        thread::sleep(Duration::from_millis(10));
        sw.stop();
        
        let elapsed1 = sw.elapsed();
        assert!(elapsed1 >= Duration::from_millis(10));
        
        sw.start();
        thread::sleep(Duration::from_millis(10));
        sw.stop();
        
        let elapsed2 = sw.elapsed();
        assert!(elapsed2 >= Duration::from_millis(20));
    }

    #[test]
    fn test_fixed_timestep_steps() {
        let steps = FixedTimestepSteps { steps: 5, remainder: 0.5 };
        assert_eq!(steps.steps, 5);
        assert_eq!(steps.remainder, 0.5);
    }
}
