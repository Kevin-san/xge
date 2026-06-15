use std::time::{Instant, Duration};

pub struct Time {
    delta: Duration,
    elapsed: Duration,
    frame_count: u64,
    fixed_timestep: f32,
    fps: f32,
    last_time: Instant,
    fps_update_interval: Duration,
    frames_since_fps_update: u64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta: Duration::from_nanos(0),
            elapsed: Duration::from_nanos(0),
            frame_count: 0,
            fixed_timestep: 1.0 / 60.0,
            fps: 0.0,
            last_time: Instant::now(),
            fps_update_interval: Duration::from_secs(1),
            frames_since_fps_update: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_time);
        self.last_time = now;
        self.elapsed += self.delta;
        self.frame_count += 1;
        self.frames_since_fps_update += 1;

        if self.elapsed >= self.fps_update_interval {
            self.fps = self.frames_since_fps_update as f32 / self.fps_update_interval.as_secs_f32();
            self.frames_since_fps_update = 0;
        }
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn set_fixed_timestep(&mut self, dt: f32) {
        self.fixed_timestep = dt;
    }

    pub fn fixed_timestep(&self) -> f32 {
        self.fixed_timestep
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FixedTimestepSteps {
    accumulator: f64,
    steps: u32,
}

impl FixedTimestepSteps {
    pub fn new() -> Self {
        Self {
            accumulator: 0.0,
            steps: 0,
        }
    }

    pub fn update(&mut self, delta_seconds: f64, fixed_timestep: f64) -> u32 {
        self.accumulator += delta_seconds;
        self.steps = (self.accumulator / fixed_timestep) as u32;
        self.accumulator -= self.steps as f64 * fixed_timestep;
        self.steps
    }

    pub fn steps(&self) -> u32 {
        self.steps
    }

    pub fn accumulator(&self) -> f64 {
        self.accumulator
    }
}

pub struct Stopwatch {
    start: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            start: None,
            elapsed: Duration::from_nanos(0),
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
                self.elapsed += Instant::now().duration_since(start);
            }
            self.start = None;
            self.running = false;
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = Duration::from_nanos(0);
        self.running = false;
    }

    pub fn elapsed(&self) -> Duration {
        if self.running {
            if let Some(start) = self.start {
                self.elapsed + Instant::now().duration_since(start)
            } else {
                self.elapsed
            }
        } else {
            self.elapsed
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn time_new() {
        let time = Time::new();
        assert_eq!(time.frame_count(), 0);
        assert_eq!(time.fixed_timestep(), 1.0 / 60.0);
    }

    #[test]
    fn time_tick() {
        let mut time = Time::new();
        thread::sleep(Duration::from_millis(16));
        time.tick();
        
        assert_eq!(time.frame_count(), 1);
        assert!(time.delta_seconds() > 0.0);
    }

    #[test]
    fn time_delta() {
        let mut time = Time::new();
        thread::sleep(Duration::from_millis(10));
        time.tick();
        
        let delta = time.delta();
        assert!(delta >= Duration::from_millis(10));
    }

    #[test]
    fn time_elapsed() {
        let mut time = Time::new();
        thread::sleep(Duration::from_millis(5));
        time.tick();
        
        assert!(time.elapsed() >= Duration::from_millis(5));
    }

    #[test]
    fn time_fixed_timestep() {
        let mut time = Time::new();
        time.set_fixed_timestep(0.02);
        assert_eq!(time.fixed_timestep(), 0.02);
    }

    #[test]
    fn stopwatch_new() {
        let sw = Stopwatch::new();
        assert!(!sw.is_running());
        assert_eq!(sw.elapsed(), Duration::from_nanos(0));
    }

    #[test]
    fn stopwatch_start_stop() {
        let mut sw = Stopwatch::new();
        sw.start();
        assert!(sw.is_running());
        
        thread::sleep(Duration::from_millis(10));
        
        sw.stop();
        assert!(!sw.is_running());
        assert!(sw.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn stopwatch_reset() {
        let mut sw = Stopwatch::new();
        sw.start();
        thread::sleep(Duration::from_millis(5));
        sw.stop();
        sw.reset();
        
        assert_eq!(sw.elapsed(), Duration::from_nanos(0));
    }

    #[test]
    fn fixed_timestep_steps() {
        let mut fts = FixedTimestepSteps::new();
        let steps = fts.update(0.1, 0.016);
        
        assert_eq!(steps, 6);
        assert!(fts.accumulator() < 0.016);
    }
}
