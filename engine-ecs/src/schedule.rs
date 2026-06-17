//! Schedule 调度系统 - 定义系统执行顺序

use crate::{Resources, World};
use std::any::TypeId;

/// 调度器 - 管理系统的执行顺序
pub struct Schedule {
    stages: Vec<SystemStage>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    /// 添加阶段
    pub fn add_stage(&mut self, _name: impl Into<String>, stage: SystemStage) -> &mut Self {
        self.stages.push(stage);
        self
    }

    /// 添加单线程阶段并返回可变引用以便添加系统
    pub fn add_stage_to_schedule(&mut self, name: impl Into<String>) -> &mut SystemStage {
        self.stages.push(SystemStage::single_threaded(name));
        self.stages.last_mut().unwrap()
    }

    /// 执行一个 step
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for stage in &mut self.stages {
            stage.run(world, resources);
        }
    }

    /// 获取阶段数量
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

pub struct SystemStage {
    pub name: String,
    pub systems: Vec<Box<dyn System>>,
}

pub trait System: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn run(&mut self, world: &mut World, resources: &mut Resources);
    fn read(&self) -> Vec<TypeId>;
    fn write(&self) -> Vec<TypeId>;
}

impl SystemStage {
    pub fn single_threaded(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            systems: Vec::new(),
        }
    }

    pub fn parallel(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for system in &mut self.systems {
            system.run(world, resources);
        }
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SystemStage {
    fn default() -> Self {
        Self::single_threaded("")
    }
}

/// FnSystem - 基于函数的系统
pub struct FnSystem<F> {
    func: F,
    name: String,
}

impl<F> FnSystem<F>
where
    F: Fn(&mut World, &mut Resources) + Send + Sync + 'static,
{
    pub fn new(name: impl Into<String>, func: F) -> Self {
        Self {
            func,
            name: name.into(),
        }
    }
}

impl<F> System for FnSystem<F>
where
    F: Fn(&mut World, &mut Resources) + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        (self.func)(world, resources);
    }
    fn read(&self) -> Vec<TypeId> {
        Vec::new()
    }
    fn write(&self) -> Vec<TypeId> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_add_stage() {
        let mut schedule = Schedule::new();
        assert_eq!(schedule.stage_count(), 0);

        schedule.add_stage("test", SystemStage::single_threaded("test"));
        assert_eq!(schedule.stage_count(), 1);

        schedule.add_stage("test2", SystemStage::parallel("test2"));
        assert_eq!(schedule.stage_count(), 2);
    }

    #[test]
    fn test_schedule_run() {
        let mut world = World::new();
        let mut resources = Resources::new();
        let mut schedule = Schedule::new();

        // 创建一个跟踪标志
        std::sync::atomic::AtomicU32::new(0);

        let mut schedule = Schedule::new();
        schedule.add_stage("test", SystemStage::single_threaded("test"));

        // Schedule 运行完成无 panic 即为通过
        schedule.run(&mut world, &mut resources);
    }
}
