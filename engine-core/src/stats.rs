use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub struct FrameStats {
    pub frame_number: u64,
    pub dt: f64,
    pub cpu_time_us: u64,
    pub gpu_time_us: u64,
    pub draw_call_count: u32,
    pub triangle_count: u32,
    pub vertex_count: u32,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self {
            frame_number: 0,
            dt: 0.0,
            cpu_time_us: 0,
            gpu_time_us: 0,
            draw_call_count: 0,
            triangle_count: 0,
            vertex_count: 0,
        }
    }
}

impl FrameStats {
    pub fn new(frame_number: u64, dt: f64) -> Self {
        Self {
            frame_number,
            dt,
            ..Self::default()
        }
    }

    pub fn reset_draw_calls(&mut self) {
        self.draw_call_count = 0;
        self.triangle_count = 0;
        self.vertex_count = 0;
    }
}

#[derive(Debug, Clone)]
pub struct EngineStats {
    pub uptime_seconds: f64,
    pub total_frames: u64,
    pub avg_fps: f32,
    pub min_fps: f32,
    pub max_fps: f32,
    pub frame_time_average_us: u64,
    pub frame_time_min_us: u64,
    pub frame_time_max_us: u64,
    pub memory_usage_bytes: u64,
}

impl Default for EngineStats {
    fn default() -> Self {
        Self {
            uptime_seconds: 0.0,
            total_frames: 0,
            avg_fps: 0.0,
            min_fps: f32::MAX,
            max_fps: 0.0,
            frame_time_average_us: 0,
            frame_time_min_us: u64::MAX,
            frame_time_max_us: 0,
            memory_usage_bytes: 0,
        }
    }
}

impl EngineStats {
    pub fn update(&mut self, dt: f64, cpu_time_us: u64) {
        self.total_frames += 1;
        self.uptime_seconds += dt;

        let fps = if dt > 0.0 {
            1.0 / dt as f32
        } else {
            0.0
        };

        self.avg_fps = (self.avg_fps * (self.total_frames - 1) as f32 + fps) / self.total_frames as f32;
        self.min_fps = self.min_fps.min(fps);
        self.max_fps = self.max_fps.max(fps);

        let frame_time_us = cpu_time_us;
        self.frame_time_average_us =
            (self.frame_time_average_us * (self.total_frames - 1) + frame_time_us) / self.total_frames;
        self.frame_time_min_us = self.frame_time_min_us.min(frame_time_us);
        self.frame_time_max_us = self.frame_time_max_us.max(frame_time_us);

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.memory_usage_bytes = get_memory_usage();
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn fps_string(&self) -> String {
        format!(
            "FPS: {:.1} (min: {:.1}, max: {:.1})",
            self.avg_fps, self.min_fps, self.max_fps
        )
    }

    pub fn frame_time_string(&self) -> String {
        format!(
            "Frame Time: {}us (avg: {}us, min: {}us, max: {}us)",
            self.frame_time_max_us,
            self.frame_time_average_us,
            self.frame_time_min_us,
            self.frame_time_max_us
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_memory_usage() -> u64 {
    #[cfg(target_os = "linux")]
    {
        use std::fs::File;
        use std::io::Read;

        if let Ok(mut file) = File::open("/proc/self/statm") {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(size_kb) = parts[1].parse::<u64>() {
                        return size_kb * 1024;
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("ps")
            .arg("-o")
            .arg("rss=")
            .arg("-p")
            .arg(std::process::id().to_string())
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(size_kb) = s.trim().parse::<u64>() {
                    return size_kb * 1024;
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        use winapi::um::psapi::GetProcessMemoryInfo;
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::PROCESS_MEMORY_COUNTERS_EX;

        unsafe {
            let mut mem_info: PROCESS_MEMORY_COUNTERS_EX = std::mem::zeroed();
            mem_info.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
            if GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut mem_info as *mut _ as *mut _,
                mem_info.cb,
            ) != 0
            {
                return mem_info.PrivateUsage;
            }
        }
    }

    0
}

#[derive(Debug)]
pub struct StatsManager {
    frame_stats: FrameStats,
    engine_stats: EngineStats,
    last_frame_start: std::time::Instant,
}

impl Default for StatsManager {
    fn default() -> Self {
        Self {
            frame_stats: FrameStats::default(),
            engine_stats: EngineStats::default(),
            last_frame_start: std::time::Instant::now(),
        }
    }
}

impl StatsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_frame(&mut self) {
        self.last_frame_start = std::time::Instant::now();
        self.frame_stats.reset_draw_calls();
    }

    pub fn end_frame(&mut self, dt: f64) {
        let cpu_time_us = self.last_frame_start.elapsed().as_micros() as u64;
        
        self.frame_stats.frame_number += 1;
        self.frame_stats.dt = dt;
        self.frame_stats.cpu_time_us = cpu_time_us;

        self.engine_stats.update(dt, cpu_time_us);
    }

    pub fn frame_stats(&self) -> &FrameStats {
        &self.frame_stats
    }

    pub fn frame_stats_mut(&mut self) -> &mut FrameStats {
        &mut self.frame_stats
    }

    pub fn engine_stats(&self) -> &EngineStats {
        &self.engine_stats
    }

    pub fn engine_stats_mut(&mut self) -> &mut EngineStats {
        &mut self.engine_stats
    }

    pub fn add_draw_call(&mut self, triangles: u32, vertices: u32) {
        self.frame_stats.draw_call_count += 1;
        self.frame_stats.triangle_count += triangles;
        self.frame_stats.vertex_count += vertices;
    }
}

pub type SharedStatsManager = Arc<Mutex<StatsManager>>;

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
    fn test_engine_stats_update() {
        let mut stats = EngineStats::default();
        stats.update(1.0 / 60.0, 16_666);

        assert_eq!(stats.total_frames, 1);
        assert!((stats.avg_fps - 60.0).abs() < 1.0);
    }

    #[test]
    fn test_engine_stats_reset() {
        let mut stats = EngineStats::default();
        stats.update(1.0 / 60.0, 16_666);
        stats.reset();

        assert_eq!(stats.total_frames, 0);
        assert_eq!(stats.uptime_seconds, 0.0);
    }

    #[test]
    fn test_stats_manager() {
        let mut manager = StatsManager::new();
        manager.begin_frame();
        std::thread::sleep(std::time::Duration::from_millis(1));
        manager.end_frame(1.0 / 60.0);

        assert_eq!(manager.frame_stats().frame_number, 1);
        assert_eq!(manager.engine_stats().total_frames, 1);
    }

    #[test]
    fn test_add_draw_call() {
        let mut manager = StatsManager::new();
        manager.add_draw_call(100, 300);
        manager.add_draw_call(200, 600);

        assert_eq!(manager.frame_stats().draw_call_count, 2);
        assert_eq!(manager.frame_stats().triangle_count, 300);
        assert_eq!(manager.frame_stats().vertex_count, 900);
    }

    #[test]
    fn test_fps_string() {
        let mut stats = EngineStats::default();
        stats.avg_fps = 60.0;
        stats.min_fps = 30.0;
        stats.max_fps = 120.0;

        let s = stats.fps_string();
        assert!(s.contains("60.0"));
        assert!(s.contains("30.0"));
        assert!(s.contains("120.0"));
    }
}