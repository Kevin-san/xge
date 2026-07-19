use std::collections::HashMap;

/// 执行阶段名称
pub const STARTUP: &str = "Startup";
pub const UPDATE: &str = "Update";
pub const RENDER: &str = "Render";
pub const SHUTDOWN: &str = "Shutdown";

/// 执行阶段
struct Stage {
    #[allow(dead_code)]
    name: String,
    systems: Vec<Box<dyn FnMut() + Send + Sync + 'static>>,
}

/// 任务调度器
///
/// 允许注册多个执行阶段，按注册顺序执行
///
/// # Example
/// ```ignore
/// let mut schedule = Schedule::new();
/// schedule.add_system_to_stage(UPDATE, |engine| {
///     // 更新逻辑
/// });
/// schedule.add_system_to_stage(RENDER, |engine| {
///     // 渲染逻辑
/// });
/// schedule.run_stage(UPDATE);
/// ```
pub struct Schedule {
    stages: HashMap<String, Stage>,
    stage_order: Vec<String>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

impl Schedule {
    /// 创建新的调度器，包含默认四阶段
    pub fn new() -> Self {
        let mut schedule = Self {
            stages: HashMap::new(),
            stage_order: Vec::new(),
        };

        // 添加默认阶段
        schedule.add_stage(STARTUP);
        schedule.add_stage(UPDATE);
        schedule.add_stage(RENDER);
        schedule.add_stage(SHUTDOWN);

        schedule
    }

    /// 注册一个新的执行阶段
    pub fn add_stage(&mut self, name: impl Into<String>) -> &mut Self {
        let name = name.into();
        if !self.stages.contains_key(&name) {
            self.stages.insert(
                name.clone(),
                Stage {
                    name: name.clone(),
                    systems: Vec::new(),
                },
            );
            self.stage_order.push(name);
        }
        self
    }

    /// 向指定阶段添加系统
    pub fn add_system_to_stage<F>(&mut self, stage_name: &str, system: F) -> &mut Self
    where
        F: FnMut() + Send + Sync + 'static,
    {
        if let Some(stage) = self.stages.get_mut(stage_name) {
            stage.systems.push(Box::new(system));
        }
        self
    }

    /// 执行指定阶段的所有系统
    pub fn run_stage(&mut self, stage_name: &str) {
        if let Some(stage) = self.stages.get_mut(stage_name) {
            for system in &mut stage.systems {
                system();
            }
        }
    }

    /// 执行所有已注册的阶段
    pub fn run(&mut self) {
        let stage_order = self.stage_order.clone();
        for stage_name in &stage_order {
            self.run_stage(stage_name);
        }
    }

    /// 清空指定阶段的所有系统
    pub fn clear_stage(&mut self, stage_name: &str) {
        if let Some(stage) = self.stages.get_mut(stage_name) {
            stage.systems.clear();
        }
    }

    /// 获取已注册阶段的数量
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// 获取指定阶段的系统数量
    pub fn system_count(&self, stage_name: &str) -> usize {
        self.stages
            .get(stage_name)
            .map(|s| s.systems.len())
            .unwrap_or(0)
    }

    /// 获取所有阶段名称
    pub fn stage_names(&self) -> &[String] {
        &self.stage_order
    }

    /// 获取阶段顺序
    pub fn stage_order(&self) -> &[String] {
        &self.stage_order
    }

    /// 设置运行条件（预留接口）
    pub fn set_run_criteria<F>(&mut self, _criteria: F)
    where
        F: Fn() -> bool + Send + Sync + 'static,
    {
        // 预留接口，本 Sprint 不实现
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_default_stages() {
        let schedule = Schedule::new();
        assert_eq!(schedule.stage_count(), 4);
        assert!(schedule.stage_names().contains(&STARTUP.to_string()));
        assert!(schedule.stage_names().contains(&UPDATE.to_string()));
    }

    #[test]
    fn test_add_system() {
        let mut schedule = Schedule::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let c = counter.clone();
        schedule.add_system_to_stage(UPDATE, move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        schedule.run_stage(UPDATE);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiple_systems() {
        let mut schedule = Schedule::new();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..3 {
            let c = counter.clone();
            schedule.add_system_to_stage(UPDATE, move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        schedule.run_stage(UPDATE);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_run_all_stages() {
        let mut schedule = Schedule::new();
        let counter = Arc::new(AtomicUsize::new(0));

        for stage in [STARTUP, UPDATE, RENDER, SHUTDOWN] {
            let c = counter.clone();
            schedule.add_system_to_stage(stage, move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        schedule.run();
        assert_eq!(counter.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn test_clear_stage() {
        let mut schedule = Schedule::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let c = counter.clone();
        schedule.add_system_to_stage(UPDATE, move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        schedule.clear_stage(UPDATE);
        schedule.run_stage(UPDATE);
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}
