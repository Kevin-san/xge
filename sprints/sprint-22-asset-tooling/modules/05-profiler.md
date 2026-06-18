# Module 05 — 性能分析器

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)
> 文件位置: `engine-profiler/src/`

## 1. CPU Profiler

```rust
pub struct CpuProfiler {
    enabled: bool,
    samples: Vec<Sample>,
    next_id: u32,
    call_stack: Vec<u32>,
    start_time: Instant,
}

pub struct Sample {
    pub id: u32,
    pub name: String,
    pub start_ns: u64,
    pub duration_ns: u64,
    pub thread_id: u32,
    pub parent_id: Option<u32>,
}

impl CpuProfiler {
    pub fn begin(&mut self, name: &str) -> ScopeGuard {
        let id = self.next_id;
        self.next_id += 1;
        let parent = self.call_stack.last().copied();
        self.call_stack.push(id);
        let start_ns = self.now_ns();
        ScopeGuard { id, parent, start_ns, profiler: self }
    }
    
    pub fn end(&mut self, scope: ScopeGuard) {
        let duration = self.now_ns() - scope.start_ns;
        self.call_stack.pop();
        self.samples.push(Sample {
            id: scope.id,
            name: /* ... */,
            start_ns: scope.start_ns,
            duration_ns: duration,
            thread_id: current_thread_id(),
            parent_id: scope.parent,
        });
    }
}

pub struct ScopeGuard<'a> { /* RAII */ }
impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) { self.profiler.end(*self); }
}

#[macro_export]
macro_rules! profile_scope {
    ($profiler:expr, $name:expr) => {
        let _scope = $profiler.begin($name);
    };
}
```

## 2. GPU Profiler

```rust
pub struct GpuProfiler {
    queries: Vec<TimeQuery>,
    next_id: u32,
}

pub struct TimeQuery {
    pub id: u32,
    pub name: String,
    pub start: GpuQuery,
    pub end: GpuQuery,
    pub result_ns: u64,
}

impl GpuProfiler {
    pub fn begin_pass(&mut self, name: &str, ctx: &mut RenderContext);
    pub fn end_pass(&mut self, name: &str, ctx: &mut RenderContext);
    pub fn collect(&mut self) -> Vec<TimeQuery>;
}
```

## 3. Memory Profiler

```rust
pub struct MemoryProfiler {
    allocations: AtomicUsize,
    bytes_allocated: AtomicUsize,
    peak_bytes: AtomicUsize,
    by_site: Mutex<HashMap<&'static str, SiteStats>>,
}

pub struct SiteStats {
    pub count: AtomicUsize,
    pub bytes: AtomicUsize,
    pub max_bytes: AtomicUsize,
}

impl MemoryProfiler {
    pub fn track_alloc(&self, size: usize, site: &'static str);
    pub fn track_dealloc(&self, size: usize, site: &'static str);
    pub fn snapshot(&self) -> MemorySnapshot;
    pub fn detect_leaks(&self) -> Vec<LeakInfo>;
}
```

## 4. Frame Profiler

```rust
pub struct FrameProfiler {
    frame_times: RingBuffer<f32>,  // 最近 N 帧
    cpu_time: f32,
    gpu_time: f32,
    pub target_fps: f32,
}

impl FrameProfiler {
    pub fn begin_frame(&mut self);
    pub fn end_frame(&mut self);
    
    pub fn stats(&self) -> FrameStats {
        let times = self.frame_times.sorted();
        FrameStats {
            avg: times.iter().sum::<f32>() / times.len() as f32,
            p50: times[times.len() / 2],
            p95: times[(times.len() as f32 * 0.95) as usize],
            p99: times[(times.len() as f32 * 0.99) as usize],
            min: *times.first().unwrap(),
            max: *times.last().unwrap(),
        }
    }
}
```

## 5. Chrome Tracing 导出

```rust
pub fn export_chrome_tracing(samples: &[Sample], path: &Path) -> Result<(), Error> {
    let mut output = String::new();
    output.push_str("{\"traceEvents\":[");
    for sample in samples {
        output.push_str(&format!(
            r#"{{"name":"{}","ph":"X","ts":{},"dur":{},"pid":0,"tid":{}}},"#,
            sample.name, sample.start_ns / 1000, sample.duration_ns / 1000, sample.thread_id,
        ));
    }
    output.push_str("]}");
    std::fs::write(path, output)?;
    Ok(())
}
```

## 6. Tracy 集成（可选）

```rust
pub struct TracyClient {
    connection: Option<net::TcpStream>,
}

impl TracyClient {
    pub fn connect(host: &str, port: u16) -> Self;
    pub fn submit_sample(&mut self, sample: &Sample);
    pub fn submit_gpu_query(&mut self, query: &TimeQuery);
}
```

## 7. 验收

- [ ] CPU 采样开销 < 5%
- [ ] GPU 时间戳精度：1 ms
- [ ] 内存分配跟踪：零开销 fallback
- [ ] Chrome 浏览器打开火焰图
- [ ] Tracy 实时分析
- [ ] 99 / 95 / 50 百分位
