// 调度器模块

/// 执行阶段
struct Stage {
    #[allow(dead_code)]
    name: String,
    func: Box<dyn FnMut() + Send + Sync + 'static>,
}

/// 任务调度器
pub struct Schedule {
    stages: Vec<Stage>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

impl Schedule {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_stage<F>(&mut self, name: impl Into<String>, func: F) -> &mut Self
    where
        F: FnMut() + Send + Sync + 'static,
    {
        self.stages.push(Stage {
            name: name.into(),
            func: Box::new(func),
        });
        self
    }

    pub fn run(&mut self) {
        for stage in &mut self.stages {
            (stage.func)();
        }
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}