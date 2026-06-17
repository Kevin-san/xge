use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone)]
struct ScopeData {
    total_time: std::time::Duration,
    call_count: u64,
    min_time: std::time::Duration,
    max_time: std::time::Duration,
}

impl Default for ScopeData {
    fn default() -> Self {
        Self {
            total_time: std::time::Duration::new(0, 0),
            call_count: 0,
            min_time: std::time::Duration::new(u64::MAX, 0),
            max_time: std::time::Duration::new(0, 0),
        }
    }
}

impl ScopeData {
    fn record(&mut self, duration: std::time::Duration) {
        self.total_time += duration;
        self.call_count += 1;
        self.min_time = self.min_time.min(duration);
        self.max_time = self.max_time.max(duration);
    }

    fn avg_time(&self) -> std::time::Duration {
        if self.call_count == 0 {
            std::time::Duration::new(0, 0)
        } else {
            self.total_time / self.call_count
        }
    }

    fn as_string(&self) -> String {
        format!(
            "count: {}, total: {:?}, avg: {:?}, min: {:?}, max: {:?}",
            self.call_count,
            self.total_time,
            self.avg_time(),
            self.min_time,
            self.max_time
        )
    }
}

#[derive(Debug)]
struct ActiveScope {
    name: String,
    start_time: Instant,
    parent: Option<usize>,
}

#[derive(Debug)]
pub struct Profiler {
    scopes: HashMap<String, ScopeData>,
    active_scopes: Vec<ActiveScope>,
    enabled: bool,
}

impl Default for Profiler {
    fn default() -> Self {
        Self {
            scopes: HashMap::new(),
            active_scopes: Vec::new(),
            enabled: true,
        }
    }
}

impl Profiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn begin_scope(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        let parent_index = if self.active_scopes.is_empty() {
            None
        } else {
            Some(self.active_scopes.len() - 1)
        };

        self.active_scopes.push(ActiveScope {
            name: name.to_string(),
            start_time: Instant::now(),
            parent,
        });
    }

    pub fn end_scope(&mut self) {
        if !self.enabled || self.active_scopes.is_empty() {
            return;
        }

        if let Some(scope) = self.active_scopes.pop() {
            let duration = scope.start_time.elapsed();
            let data = self.scopes.entry(scope.name).or_insert_with(ScopeData::default);
            data.record(duration);
        }
    }

    pub fn scope<'a>(&'a mut self, name: &str) -> ScopeGuard<'a> {
        self.begin_scope(name);
        ScopeGuard { profiler: self }
    }

    pub fn dump(&self) -> String {
        let mut result = String::from("=== Profiler Results ===\n");
        
        let mut scopes: Vec<_> = self.scopes.iter().collect();
        scopes.sort_by(|a, b| b.1.total_time.cmp(&a.1.total_time));

        for (name, data) in scopes {
            result.push_str(&format!("{}: {}\n", name, data.as_string()));
        }

        result.push_str("=== End Profiler Results ===");
        result
    }

    pub fn print(&self) {
        println!("{}", self.dump());
    }

    pub fn clear(&mut self) {
        self.scopes.clear();
        self.active_scopes.clear();
    }

    pub fn get_scope_data(&self, name: &str) -> Option<&ScopeData> {
        self.scopes.get(name)
    }

    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    pub fn total_calls(&self) -> u64 {
        self.scopes.values().map(|d| d.call_count).sum()
    }

    pub fn total_time(&self) -> std::time::Duration {
        self.scopes.values().map(|d| d.total_time).sum()
    }
}

pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.profiler.end_scope();
    }
}

pub type SharedProfiler = Arc<Mutex<Profiler>>;

#[macro_export]
macro_rules! profile_scope {
    ($profiler:expr, $name:expr) => {{
        $profiler.begin_scope($name);
        let _guard = scopeguard::guard((), |_| $profiler.end_scope());
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_profiler_basic() {
        let mut profiler = Profiler::new();
        
        profiler.begin_scope("test");
        thread::sleep(Duration::from_millis(1));
        profiler.end_scope();

        assert_eq!(profiler.scope_count(), 1);
        assert_eq!(profiler.total_calls(), 1);
    }

    #[test]
    fn test_profiler_scope_guard() {
        let mut profiler = Profiler::new();
        
        {
            let _guard = profiler.scope("test_scope");
            thread::sleep(Duration::from_millis(1));
        }

        assert_eq!(profiler.scope_count(), 1);
        let data = profiler.get_scope_data("test_scope").unwrap();
        assert_eq!(data.call_count, 1);
    }

    #[test]
    fn test_profiler_disabled() {
        let mut profiler = Profiler::new();
        profiler.disable();
        
        profiler.begin_scope("test");
        thread::sleep(Duration::from_millis(1));
        profiler.end_scope();

        assert_eq!(profiler.scope_count(), 0);
    }

    #[test]
    fn test_profiler_multiple_calls() {
        let mut profiler = Profiler::new();
        
        for _ in 0..3 {
            profiler.begin_scope("loop");
            thread::sleep(Duration::from_millis(1));
            profiler.end_scope();
        }

        let data = profiler.get_scope_data("loop").unwrap();
        assert_eq!(data.call_count, 3);
    }

    #[test]
    fn test_profiler_dump() {
        let mut profiler = Profiler::new();
        
        profiler.begin_scope("test");
        thread::sleep(Duration::from_millis(1));
        profiler.end_scope();

        let dump = profiler.dump();
        assert!(dump.contains("test"));
        assert!(dump.contains("count"));
    }

    #[test]
    fn test_profiler_clear() {
        let mut profiler = Profiler::new();
        
        profiler.begin_scope("test");
        thread::sleep(Duration::from_millis(1));
        profiler.end_scope();

        assert_eq!(profiler.scope_count(), 1);
        profiler.clear();
        assert_eq!(profiler.scope_count(), 0);
    }

    #[test]
    fn test_scope_data_avg() {
        let mut data = ScopeData::default();
        data.record(Duration::from_millis(10));
        data.record(Duration::from_millis(20));
        
        assert_eq!(data.call_count, 2);
        assert_eq!(data.total_time, Duration::from_millis(30));
        assert_eq!(data.avg_time(), Duration::from_millis(15));
    }

    #[test]
    fn test_scope_data_min_max() {
        let mut data = ScopeData::default();
        data.record(Duration::from_millis(10));
        data.record(Duration::from_millis(5));
        data.record(Duration::from_millis(20));
        
        assert_eq!(data.min_time, Duration::from_millis(5));
        assert_eq!(data.max_time, Duration::from_millis(20));
    }
}