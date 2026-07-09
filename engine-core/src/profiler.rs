use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ScopeStats {
    pub name: &'static str,
    pub count: u64,
    pub total_time: Duration,
    pub max_time: Duration,
    pub min_time: Duration,
}

impl ScopeStats {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            count: 0,
            total_time: Duration::from_secs(0),
            max_time: Duration::from_secs(0),
            min_time: Duration::from_secs(u64::MAX),
        }
    }

    fn record(&mut self, duration: Duration) {
        self.count += 1;
        self.total_time += duration;
        if duration > self.max_time {
            self.max_time = duration;
        }
        if duration < self.min_time {
            self.min_time = duration;
        }
    }

    pub fn avg_time(&self) -> Duration {
        if self.count == 0 {
            Duration::from_secs(0)
        } else {
            self.total_time / self.count as u32
        }
    }
}

struct Inner {
    scopes: HashMap<&'static str, ScopeStats>,
    stack: Vec<(&'static str, Instant)>,
}

pub struct Profiler {
    inner: RefCell<Inner>,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(Inner {
                scopes: HashMap::new(),
                stack: Vec::new(),
            }),
        }
    }

    pub fn begin_scope(&self, name: &'static str) {
        let mut inner = self.inner.borrow_mut();
        inner.stack.push((name, Instant::now()));
    }

    pub fn end_scope(&self) {
        let mut inner = self.inner.borrow_mut();
        if let Some((name, start)) = inner.stack.pop() {
            let duration = start.elapsed();
            inner
                .scopes
                .entry(name)
                .or_insert_with(|| ScopeStats::new(name))
                .record(duration);
        }
    }

    pub fn scope<'a>(&'a self, name: &'static str) -> ScopeGuard<'a> {
        self.begin_scope(name);
        ScopeGuard {
            profiler: self,
        }
    }

    pub fn get_stats(&self, name: &str) -> Option<ScopeStats> {
        self.inner.borrow().scopes.get(name).cloned()
    }

    pub fn all_stats(&self) -> HashMap<&'static str, ScopeStats> {
        self.inner.borrow().scopes.clone()
    }

    pub fn reset(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.scopes.clear();
        inner.stack.clear();
    }

    pub fn dump(&self) -> String {
        let inner = self.inner.borrow();
        let mut result = String::from("=== Profiler Stats ===\n");
        let mut scopes: Vec<&ScopeStats> = inner.scopes.values().collect();
        scopes.sort_by(|a, b| b.total_time.cmp(&a.total_time));

        for stats in scopes {
            result.push_str(&format!(
                "  {}: count={}, total={:?}, avg={:?}, min={:?}, max={:?}\n",
                stats.name,
                stats.count,
                stats.total_time,
                stats.avg_time(),
                if stats.min_time == Duration::from_secs(u64::MAX) {
                    Duration::from_secs(0)
                } else {
                    stats.min_time
                },
                stats.max_time,
            ));
        }
        result
    }
}

pub struct ScopeGuard<'a> {
    profiler: &'a Profiler,
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.profiler.end_scope();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new() {
        let p = Profiler::new();
        assert!(p.all_stats().is_empty());
    }

    #[test]
    fn test_default() {
        let _p: Profiler = Default::default();
    }

    #[test]
    fn test_begin_end_scope() {
        let p = Profiler::new();
        p.begin_scope("test");
        thread::sleep(Duration::from_millis(1));
        p.end_scope();

        let stats = p.get_stats("test").unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.total_time > Duration::from_secs(0));
    }

    #[test]
    fn test_scope_guard() {
        let p = Profiler::new();
        {
            let _guard = p.scope("guard_test");
            thread::sleep(Duration::from_millis(1));
        }

        let stats = p.get_stats("guard_test").unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.total_time > Duration::from_secs(0));
    }

    #[test]
    fn test_multiple_scopes() {
        let p = Profiler::new();
        for _ in 0..5 {
            let _guard = p.scope("multi");
            thread::sleep(Duration::from_millis(1));
        }

        let stats = p.get_stats("multi").unwrap();
        assert_eq!(stats.count, 5);
    }

    #[test]
    fn test_nested_scopes() {
        let p = Profiler::new();
        {
            let _outer = p.scope("outer");
            {
                let _inner = p.scope("inner");
                thread::sleep(Duration::from_millis(1));
            }
        }

        assert!(p.get_stats("outer").is_some());
        assert!(p.get_stats("inner").is_some());
    }

    #[test]
    fn test_reset() {
        let p = Profiler::new();
        p.begin_scope("test");
        p.end_scope();
        assert!(!p.all_stats().is_empty());

        p.reset();
        assert!(p.all_stats().is_empty());
    }

    #[test]
    fn test_dump() {
        let p = Profiler::new();
        p.begin_scope("dump_test");
        p.end_scope();

        let dump = p.dump();
        assert!(dump.contains("dump_test"));
        assert!(dump.contains("Profiler Stats"));
    }

    #[test]
    fn test_scope_stats_avg() {
        let mut stats = ScopeStats::new("test");
        stats.record(Duration::from_millis(10));
        stats.record(Duration::from_millis(20));
        stats.record(Duration::from_millis(30));

        assert_eq!(stats.count, 3);
        assert_eq!(stats.avg_time(), Duration::from_millis(20));
        assert_eq!(stats.min_time, Duration::from_millis(10));
        assert_eq!(stats.max_time, Duration::from_millis(30));
    }

    #[test]
    fn test_end_scope_without_begin() {
        let p = Profiler::new();
        p.end_scope();
        assert!(p.all_stats().is_empty());
    }
}
