use std::time::{Duration, Instant};

/// 时间管理器
pub struct Time {
    last_frame: Instant,
    delta: Duration,
    total: Duration,
    frame_count: u64,
    fixed_timestep_accumulator: f64,
    fixed_timestep: f64,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    pub fn new() -> Self {
        Self::new_with_fixed_timestep(1.0 / 60.0)
    }

    pub fn new_with_fixed_timestep(fixed_timestep: f64) -> Self {
        let now = Instant::now();
        Self {
            last_frame: now,
            delta: Duration::from_secs(0),
            total: Duration::from_secs(0),
            frame_count: 0,
            fixed_timestep_accumulator: 0.0,
            fixed_timestep,
        }
    }

    pub fn tick(&mut self) -> FixedTimestepSteps {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_frame);
        self.total += self.delta;
        self.last_frame = now;
        self.frame_count += 1;

        self.fixed_timestep_accumulator += self.delta.as_secs_f64();

        let steps = (self.fixed_timestep_accumulator / self.fixed_timestep).floor() as usize;
        self.fixed_timestep_accumulator -= steps as f64 * self.fixed_timestep;

        FixedTimestepSteps {
            steps,
            fixed_timestep: self.fixed_timestep,
        }
    }

    pub fn delta_seconds(&self) -> f64 {
        self.delta.as_secs_f64()
    }

    pub fn delta_seconds_f32(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn fps(&self) -> f64 {
        if self.delta.as_secs_f64() > 0.0 {
            1.0 / self.delta.as_secs_f64()
        } else {
            0.0
        }
    }

    pub fn total_seconds(&self) -> f64 {
        self.total.as_secs_f64()
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn set_fixed_timestep(&mut self, timestep: f64) {
        self.fixed_timestep = timestep;
    }

    pub fn fixed_timestep(&self) -> f64 {
        self.fixed_timestep
    }
}

/// 固定时间步迭代器
pub struct FixedTimestepSteps {
    pub steps: usize,
    pub fixed_timestep: f64,
}

impl FixedTimestepSteps {
    pub fn iter(&self) -> FixedTimestepIter {
        FixedTimestepIter {
            remaining: self.steps,
            fixed_timestep: self.fixed_timestep,
        }
    }

    pub fn steps(&self) -> usize {
        self.steps
    }
}

pub struct FixedTimestepIter {
    remaining: usize,
    fixed_timestep: f64,
}

impl Iterator for FixedTimestepIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            Some(self.fixed_timestep)
        } else {
            None
        }
    }
}

/// 计时器
pub struct Stopwatch {
    start: Option<Instant>,
    elapsed: Duration,
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
        }
    }

    pub fn start(&mut self) {
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start.take() {
            self.elapsed += Instant::now().duration_since(start);
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = Duration::from_secs(0);
    }

    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start {
            self.elapsed + Instant::now().duration_since(start)
        } else {
            self.elapsed
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    pub fn elapsed_secs_f32(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    pub fn is_running(&self) -> bool {
        self.start.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_delta() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(16));
        let _steps = time.tick();
        assert!(time.delta_seconds() > 0.0);
        // steps.steps()返回usize，可能为0或正整数
    }

    #[test]
    fn test_fps() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(16));
        time.tick();
        assert!(time.fps() > 0.0);
    }

    #[test]
    fn test_fixed_timestep_iter() {
        let steps = FixedTimestepSteps {
            steps: 3,
            fixed_timestep: 0.016,
        };
        let sum: f64 = steps.iter().sum();
        assert!((sum - 0.048).abs() < 1e-6);
    }

    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::new();
        assert!(!sw.is_running());

        sw.start();
        assert!(sw.is_running());

        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(sw.elapsed_secs() > 0.0);

        sw.stop();
        assert!(!sw.is_running());

        sw.reset();
        assert_eq!(sw.elapsed_secs(), 0.0);
    }
}
