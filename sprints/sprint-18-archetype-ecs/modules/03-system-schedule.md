# Module 03 — 并行 System 调度

> 上游 sprint: [Sprint 18](../sprint-18-archetype-ecs.md)
> 文件位置: `engine-ecs/src/schedule/mod.rs`, `engine-ecs/src/system/`

---

## 1. 目标

实现 **拓扑排序 + 多线程执行** 的 Schedule：
- Stage（阶段）：逻辑分组（PreUpdate / Update / PostUpdate）
- System 自动按资源访问冲突分组并行层
- SystemParam 自动派生（Query/Res/ResMut/EventReader/EventWriter/Commands）

## 2. System Trait

```rust
// engine-ecs/src/system/mod.rs

pub trait System: Send + Sync {
    type Input;
    type Output;
    
    fn init(&mut self, world: &mut World);
    fn run(&mut self, world: &mut World, input: Self::Input) -> Self::Output;
    fn access(&self) -> &SystemAccess;
    fn name(&self) -> &str;
}

pub trait IntoSystem<Marker> {
    type System: System;
    fn into_system(self) -> Self::System;
}

// 自动为 fn 派生
impl<F, Marker> IntoSystem<Marker> for F
where F: SystemParamFunction<Marker>,
{
    type System = FunctionSystem<Marker, F>;
    fn into_system(self) -> Self::System {
        FunctionSystem::new(self)
    }
}
```

## 3. SystemParam

```rust
pub trait SystemParam {
    type State: Send + Sync;
    type Item<'w>;
    
    fn init_state(world: &World) -> Self::State;
    fn get_param<'w>(state: &'w Self::State, world: &'w World) -> Self::Item<'w>;
}

// 自动派生宏
#[derive(SystemParam)]
pub struct MyParams<'w> {
    pub query: Query<'w, &'w Position, &'w Velocity>,
    pub time: Res<'w, Time>,
    pub mut events: EventWriter<'w, MyEvent>,
}
```

## 4. Resource Access

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessKind {
    Read,
    Write,
}

#[derive(Default)]
pub struct SystemAccess {
    pub resources: HashMap<TypeId, AccessKind>,
    pub components: HashSet<TypeId>,  // Query 访问
    pub events: HashSet<TypeId>,
}

impl SystemAccess {
    /// 检测与另一个 system 的访问冲突
    pub fn conflicts_with(&self, other: &SystemAccess) -> bool {
        // 资源：任一写冲突
        for (id, kind) in &self.resources {
            if let Some(&other_kind) = other.resources.get(id) {
                if *kind == AccessKind::Write || other_kind == AccessKind::Write {
                    return true;
                }
            }
        }
        // 组件：只读不冲突；写冲突
        // ...
    }
}
```

## 5. Schedule

```rust
pub struct Schedule {
    stages: Vec<Stage>,
}

pub struct Stage {
    label: StageLabel,
    systems: Vec<Box<dyn System>>,
    /// 拓扑排序后的执行层（每层可并行）
    parallel_layers: Vec<Vec<usize>>,  // layer_idx → system indices
}

impl Schedule {
    pub fn add_system<S: System>(&mut self, stage: StageLabel, system: S);
    pub fn run(&mut self, world: &mut World);
    
    /// 拓扑排序：按访问冲突分组
    fn build_parallel_layers(&mut self) {
        let n = self.systems.len();
        let mut layers: Vec<Vec<usize>> = Vec::new();
        let mut assigned = vec![None; n];
        
        for i in 0..n {
            // 找到第一个不与 i 冲突的 layer
            for (layer_idx, layer) in layers.iter_mut().enumerate() {
                let can_assign = layer.iter().all(|&j| {
                    !self.systems[i].access().conflicts_with(self.systems[j].access())
                });
                if can_assign {
                    layer.push(i);
                    assigned[i] = Some(layer_idx);
                    break;
                }
            }
            if assigned[i].is_none() {
                layers.push(vec![i]);
            }
        }
        self.parallel_layers = layers;
    }
}
```

## 6. 并行 Executor

```rust
// engine-ecs/src/schedule/executor.rs

pub trait Executor: Send + Sync {
    fn run(&mut self, schedule: &mut Schedule, world: &mut World);
}

pub struct SingleThreaded;
impl Executor for SingleThreaded {
    fn run(&mut self, schedule: &mut Schedule, world: &mut World) {
        for layer in &schedule.parallel_layers {
            for &i in layer {
                schedule.systems[i].run(world, ());
            }
        }
    }
}

pub struct MultiThreaded {
    pool: rayon::ThreadPool,
}

impl Executor for MultiThreaded {
    fn run(&mut self, schedule: &mut Schedule, world: &mut World) {
        // 借出每个 system 不可变借用，避免冲突
        // 使用 Arc<Mutex<World>> 拆分（性能损失）或
        // ScheduleGraph 自动按资源切分到子 World
        
        for layer in &schedule.parallel_layers {
            self.pool.install(|| {
                layer.par_iter().for_each(|&i| {
                    // 每个 system 在互不冲突的资源上工作
                    // 通过 SystemParam 借用检查保证
                });
            });
        }
    }
}
```

## 7. Stage Label

```rust
pub trait StageLabel: Send + Sync + 'static {
    fn as_str(&self) -> &str;
}

pub struct PreUpdate;
pub struct Update;
pub struct PostUpdate;

impl StageLabel for PreUpdate { ... }
```

## 8. 验收

- [ ] 100 system 拓扑排序 < 10 ms
- [ ] 8 核 CPU 上 64 个并行 system 提速 ≥ 4x
- [ ] 资源冲突编译期 + 运行期检测
- [ ] 测试：竞态检测（rayon + arc_mutex 模拟）
- [ ] cargo bench：100 system Update 阶段 < 5 ms

## 9. 风险

| 风险 | 缓解 |
|------|------|
| 多线程数据竞争 | 严格 borrow check + arc mutex 兜底 |
| 调度开销 | 拓扑结果缓存，entity 数量 < 阈值时单线程 |
| SystemParam 派生宏复杂 | 限制派生范围，先支持基本类型 |
