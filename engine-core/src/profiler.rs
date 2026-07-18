//! 轻量性能分析器 — 支持作用域计时

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 性能分析器
pub struct Profiler {
    scopes: HashMap<String, ScopeRecord>,
    current_scope: Option<Instant>,
    current_scope_name: Option<String>,
}

struct ScopeRecord {
    total_duration: Duration,
    call_count: usize,
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
            current_scope: None,
            current_scope_name: None,
        }
    }

    /// 开始一个计时作用域
    pub fn begin_scope(&mut self, name: &str) {
        self.end_current_scope(); // 结束之前的作用域
        self.current_scope_name = Some(name.to_string());
        self.current_scope = Some(Instant::now());
    }

    /// 结束当前作用域
    pub fn end_scope(&mut self) {
        self.end_current_scope();
    }

    /// RAII 守卫，用于自动结束作用域
    pub fn scope<'a>(&'a mut self, name: &'a str) -> ScopeGuard<'a> {
        self.begin_scope(name);
        ScopeGuard { profiler: self }
    }

    /// 输出所有作用域的耗时汇总
    pub fn dump(&self) {
        for (name, record) in &self.scopes {
            println!(
                "[Profiler] {}: {:.3}ms ({} calls)",
                name,
                record.total_duration.as_secs_f64() * 1000.0,
                record.call_count
            );
        }
    }

    /// 获取指定作用域的总耗时
    pub fn scope_total_ms(&self, name: &str) -> Option<f64> {
        self.scopes.get(name).map(|r| r.total_duration.as_secs_f64() * 1000.0)
    }

    /// 获取指定作用域的平均耗时
    pub fn scope_avg_ms(&self, name: &str) -> Option<f64> {
        self.scopes.get(name).map(|r| {
            if r.call_count > 0 {
                r.total_duration.as_secs_f64() * 1000.0 / r.call_count as f64
            } else {
                0.0
            }
        })
    }

    fn end_current_scope(&mut self) {
        if let (Some(start), Some(name)) = (self.current_scope.take(), self.current_scope_name.take()) {
            let elapsed = start.elapsed();
            let entry = self.scopes.entry(name).or_insert(ScopeRecord {
                total_duration: Duration::ZERO,
                call_count: 0,
            });
            entry.total_duration += elapsed;
            entry.call_count += 1;
        }
    }
}

/// RAII 作用域守卫
pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
}

impl Drop for ScopeGuard<'_> {
    fn drop(&mut self) {
        self.profiler.end_current_scope();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_profiler_basic() {
        let mut profiler = Profiler::new();
        profiler.begin_scope("test_scope");
        thread::sleep(Duration::from_millis(10));
        profiler.end_scope();

        assert!(profiler.scope_total_ms("test_scope").is_some());
        assert!(profiler.scope_total_ms("test_scope").unwrap() >= 5.0);
    }

    #[test]
    fn test_profiler_guard() {
        let mut profiler = Profiler::new();
        {
            let _guard = profiler.scope("auto_scope");
            thread::sleep(Duration::from_millis(5));
        }

        assert!(profiler.scope_avg_ms("auto_scope").is_some());
    }

    #[test]
    fn test_profiler_multiple_scopes() {
        let mut profiler = Profiler::new();

        profiler.begin_scope("scope_a");
        thread::sleep(Duration::from_millis(5));
        profiler.end_scope();

        profiler.begin_scope("scope_b");
        thread::sleep(Duration::from_millis(3));
        profiler.end_scope();

        profiler.dump(); // Just verify it doesn't panic
    }
}
