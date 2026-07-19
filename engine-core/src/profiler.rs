use std::collections::HashMap;
use std::time::Instant;

/// 性能分析器
pub struct Profiler {
    scopes: HashMap<String, ScopeData>,
    active_stack: Vec<(String, Instant)>,
}

#[derive(Clone, Debug)]
struct ScopeData {
    total_ns: u64,
    call_count: u64,
}

/// RAII 作用域守卫
pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
    name: String,
    start: Instant,
}

impl Drop for ScopeGuard<'_> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_nanos() as u64;
        let data = self.profiler.scopes.entry(self.name.clone()).or_insert(ScopeData {
            total_ns: 0,
            call_count: 0,
        });
        data.total_ns += elapsed;
        data.call_count += 1;
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            scopes: HashMap::new(),
            active_stack: Vec::new(),
        }
    }

    pub fn begin_scope(&mut self, name: &str) {
        self.active_stack.push((name.to_string(), Instant::now()));
    }

    pub fn end_scope(&mut self) {
        if let Some((name, start)) = self.active_stack.pop() {
            let elapsed = start.elapsed().as_nanos() as u64;
            let data = self.scopes.entry(name).or_insert(ScopeData {
                total_ns: 0,
                call_count: 0,
            });
            data.total_ns += elapsed;
            data.call_count += 1;
        }
    }

    pub fn scope<'a>(&'a mut self, name: &str) -> ScopeGuard<'a> {
        ScopeGuard {
            profiler: self,
            name: name.to_string(),
            start: Instant::now(),
        }
    }

    pub fn dump(&self) -> Vec<(String, u64, u64)> {
        let mut result: Vec<_> = self
            .scopes
            .iter()
            .map(|(name, data)| (name.clone(), data.total_ns, data.call_count))
            .collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_profiler_new() {
        let profiler = Profiler::new();
        assert!(profiler.dump().is_empty());
    }

    #[test]
    fn test_begin_end_scope() {
        let mut profiler = Profiler::new();
        profiler.begin_scope("test_scope");
        thread::sleep(Duration::from_micros(100));
        profiler.end_scope();

        let dump = profiler.dump();
        assert_eq!(dump.len(), 1);
        assert_eq!(dump[0].0, "test_scope");
        assert!(dump[0].1 > 0);
        assert_eq!(dump[0].2, 1);
    }

    #[test]
    fn test_scope_guard() {
        let mut profiler = Profiler::new();
        {
            let _guard = profiler.scope("guarded");
            thread::sleep(Duration::from_micros(50));
        }

        let dump = profiler.dump();
        assert_eq!(dump.len(), 1);
        assert!(dump[0].1 > 0);
    }

    #[test]
    fn test_multiple_calls() {
        let mut profiler = Profiler::new();
        for _ in 0..3 {
            profiler.begin_scope("multi");
            profiler.end_scope();
        }

        let dump = profiler.dump();
        assert_eq!(dump[0].2, 3);
    }

    #[test]
    fn test_dump_sorted() {
        let mut profiler = Profiler::new();
        profiler.begin_scope("fast");
        profiler.end_scope();

        profiler.begin_scope("slow");
        thread::sleep(Duration::from_millis(1));
        profiler.end_scope();

        let dump = profiler.dump();
        assert_eq!(dump[0].0, "slow");
    }
}
