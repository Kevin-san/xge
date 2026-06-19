//! System 模块
//!
//! 定义 System trait 和系统参数。

use super::World;

/// System trait
///
/// 所有系统必须实现此 trait。
pub trait System: Send + Sync + 'static {
    /// 运行系统
    fn run(&mut self, world: &mut World);
}

/// System 参数
pub trait SystemParam {
    type Item<'a>
    where
        Self: 'a;
}

impl SystemParam for World {
    type Item<'a> = &'a World;
}

/// IntoSystem trait
///
/// 将函数转换为系统。
pub trait IntoSystem<S: System> {
    /// 将自身转换为系统
    fn into_system(self) -> S;
}

/// 函数系统
pub struct FnSystem<F> {
    func: F,
}

impl<F: FnMut(&mut World) + Send + Sync + 'static> System for FnSystem<F> {
    fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
}

impl<F: FnMut(&mut World) + Send + Sync + 'static> IntoSystem<FnSystem<F>> for F {
    fn into_system(self) -> FnSystem<F> {
        FnSystem { func: self }
    }
}

/// 系统调度器
pub struct Schedule {
    stages: Vec<Box<dyn System>>,
}

impl Schedule {
    /// 创建新的调度器
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    /// 添加系统到调度器
    pub fn add_system(&mut self, system: impl System) {
        self.stages.push(Box::new(system));
    }

    /// 运行所有系统
    pub fn run(&mut self, world: &mut World) {
        for stage in &mut self.stages {
            stage.run(world);
        }
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

/// Stage label trait
pub trait StageLabel: Send + Sync + 'static {
    /// 获取阶段标签
    fn label(&self) -> &'static str;
}

/// System set trait
pub trait SystemSet: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSystem;
    impl System for TestSystem {
        fn run(&mut self, _world: &mut World) {}
    }

    #[test]
    fn test_schedule() {
        let mut schedule = Schedule::new();
        schedule.add_system(TestSystem);
        // Schedule 创建成功
    }

    #[test]
    fn test_system_schedule_new_empty() {
        let mut schedule: Schedule = Schedule::new();
        schedule.run(&mut World::new());
    }

    #[test]
    fn test_system_schedule_add_multiple() {
        let mut schedule: Schedule = Schedule::new();
        schedule.add_system(TestSystem);
        schedule.add_system(TestSystem);
        schedule.run(&mut World::new());
    }
}
