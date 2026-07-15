use std::cell::RefCell;
use std::collections::HashMap;

/// 模块 trait
///
/// 所有引擎模块必须实现此 trait
pub trait Module: Send + Sync {
    /// 获取模块唯一名称
    fn name(&self) -> &str;

    /// 获取依赖的模块名称列表
    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }

    /// 模块初始化回调
    fn on_init(&mut self) {}

    /// 模块每帧更新回调
    fn on_update(&mut self, _dt: f64) {}

    /// 模块渲染前回调
    fn on_render(&mut self) {}

    /// 模块关闭回调
    fn on_shutdown(&mut self) {}

    /// 检查模块是否启用
    fn enabled(&self) -> bool {
        true
    }
}

/// 模块注册表
pub struct ModuleRegistry {
    modules: RefCell<HashMap<String, Box<dyn Module>>>,
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: RefCell::new(HashMap::new()),
        }
    }

    pub fn register(&self, module: Box<dyn Module>) {
        let name = module.name().to_string();
        self.modules.borrow_mut().insert(name, module);
    }

    pub fn len(&self) -> usize {
        self.modules.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.borrow().is_empty()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.modules.borrow().contains_key(name)
    }

    pub fn initialize_all(&self) -> Result<(), CycleError> {
        let sorted = topological_sort(&mut self.modules.borrow_mut())?;

        for name in sorted {
            if let Some(module) = self.modules.borrow_mut().get_mut(&name) {
                if module.enabled() {
                    module.on_init();
                }
            }
        }

        Ok(())
    }

    pub fn update_all(&self, dt: f64) {
        for module in self.modules.borrow_mut().values_mut() {
            if module.enabled() {
                module.on_update(dt);
            }
        }
    }

    pub fn render_all(&self) {
        for module in self.modules.borrow_mut().values_mut() {
            if module.enabled() {
                module.on_render();
            }
        }
    }

    pub fn shutdown_all(&self) {
        let names: Vec<String> = self.modules.borrow().keys().cloned().collect();
        for name in names.into_iter().rev() {
            if let Some(mut module) = self.modules.borrow_mut().remove(&name) {
                if module.enabled() {
                    module.on_shutdown();
                }
            }
        }
    }
}

fn topological_sort(
    modules: &mut HashMap<String, Box<dyn Module>>,
) -> Result<Vec<String>, CycleError> {
    let mut result = Vec::new();
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut in_stack: HashMap<String, bool> = HashMap::new();

    for name in modules.keys() {
        visited.insert(name.clone(), false);
        in_stack.insert(name.clone(), false);
    }

    fn visit(
        name: &str,
        modules: &HashMap<String, Box<dyn Module>>,
        visited: &mut HashMap<String, bool>,
        in_stack: &mut HashMap<String, bool>,
        result: &mut Vec<String>,
    ) -> Result<(), CycleError> {
        if *visited.get(name).unwrap_or(&false) {
            return Ok(());
        }

        if *in_stack.get(name).unwrap_or(&false) {
            return Err(CycleError(name.to_string()));
        }

        in_stack.insert(name.to_string(), true);

        if let Some(module) = modules.get(name) {
            for dep in module.dependencies() {
                visit(dep, modules, visited, in_stack, result)?;
            }
        }

        in_stack.insert(name.to_string(), false);
        visited.insert(name.to_string(), true);
        result.push(name.to_string());

        Ok(())
    }

    for name in modules.keys() {
        if !*visited.get(name).unwrap_or(&false) {
            visit(name, modules, &mut visited, &mut in_stack, &mut result)?;
        }
    }

    Ok(result)
}

#[derive(Debug)]
pub struct CycleError(pub String);

impl std::fmt::Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Circular dependency detected involving module: {}",
            self.0
        )
    }
}

impl std::error::Error for CycleError {}
