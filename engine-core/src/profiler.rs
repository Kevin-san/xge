use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

pub struct Profiler {
    timers: Mutex<HashMap<String, Timer>>,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            timers: Mutex::new(HashMap::new()),
        }
    }

    pub fn start(&self, name: &str) {
        let mut timers = self.timers.lock().unwrap();
        let timer = timers.entry(name.to_string()).or_insert_with(Timer::new);
        timer.start();
    }

    pub fn stop(&self, name: &str) {
        let mut timers = self.timers.lock().unwrap();
        if let Some(timer) = timers.get_mut(name) {
            timer.stop();
        }
    }

    pub fn elapsed(&self, name: &str) -> f64 {
        let timers = self.timers.lock().unwrap();
        timers.get(name).map(|t| t.elapsed).unwrap_or(0.0)
    }

    pub fn frame_elapsed(&self, name: &str) -> f64 {
        let timers = self.timers.lock().unwrap();
        timers.get(name).map(|t| t.frame_elapsed).unwrap_or(0.0)
    }

    pub fn sample_count(&self, name: &str) -> u64 {
        let timers = self.timers.lock().unwrap();
        timers.get(name).map(|t| t.sample_count).unwrap_or(0)
    }

    pub fn reset(&self) {
        let mut timers = self.timers.lock().unwrap();
        for timer in timers.values_mut() {
            timer.reset();
        }
    }

    pub fn reset_timer(&self, name: &str) {
        let mut timers = self.timers.lock().unwrap();
        if let Some(timer) = timers.get_mut(name) {
            timer.reset();
        }
    }

    pub fn clear(&self) {
        let mut timers = self.timers.lock().unwrap();
        timers.clear();
    }
}

struct Timer {
    start: Option<Instant>,
    elapsed: f64,
    frame_elapsed: f64,
    sample_count: u64,
}

impl Timer {
    fn new() -> Self {
        Self {
            start: None,
            elapsed: 0.0,
            frame_elapsed: 0.0,
            sample_count: 0,
        }
    }

    fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    fn stop(&mut self) {
        if let Some(start) = self.start.take() {
            let delta = start.elapsed().as_secs_f64();
            self.frame_elapsed = delta;
            self.elapsed += delta;
            self.sample_count += 1;
        }
    }

    fn reset(&mut self) {
        self.start = None;
        self.elapsed = 0.0;
        self.frame_elapsed = 0.0;
        self.sample_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_start_stop() {
        let profiler = Profiler::new();
        profiler.start("test");
        std::thread::sleep(std::time::Duration::from_millis(10));
        profiler.stop("test");

        assert!(profiler.elapsed("test") > 0.0);
        assert!(profiler.sample_count("test") > 0);
    }

    #[test]
    fn test_profiler_reset() {
        let profiler = Profiler::new();
        profiler.start("test");
        std::thread::sleep(std::time::Duration::from_millis(10));
        profiler.stop("test");

        assert!(profiler.elapsed("test") > 0.0);
        profiler.reset();
        assert_eq!(profiler.elapsed("test"), 0.0);
    }

    #[test]
    fn test_profiler_clear() {
        let profiler = Profiler::new();
        profiler.start("test");
        profiler.stop("test");

        assert!(profiler.elapsed("test") > 0.0);
        profiler.clear();
        assert_eq!(profiler.elapsed("test"), 0.0);
    }
}
