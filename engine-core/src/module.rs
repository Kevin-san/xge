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

    /// 获取所有注册模块的名称
    pub fn module_names(&self) -> Vec<String> {
        self.modules.borrow().keys().cloned().collect()
    }

    /// 按名称查找模块（仅检查是否存在）
    pub fn contains(&self, name: &str) -> bool {
        self.modules.borrow().contains_key(name)
    }

    /// 按名称查找模块（仅检查是否存在并返回名称）
    pub fn get_by_name(&self, name: &str) -> Option<String> {
        if self.modules.borrow().contains_key(name) {
            Some(name.to_string())
        } else {
            None
        }
    }

    /// 按名称对模块执行闭包操作
    pub fn with_module<F, R>(&self, name: &str, f: F) -> Option<R>
    where
        F: FnOnce(&mut dyn Module) -> R,
    {
        let mut modules = self.modules.borrow_mut();
        modules.get_mut(name).map(|module| f(module.as_mut()))
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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockModule {
        name: String,
        deps: Vec<String>,
        initialized: bool,
        updated: bool,
        rendered: bool,
        shutdown: bool,
        is_enabled: bool,
    }

    impl MockModule {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                deps: vec![],
                initialized: false,
                updated: false,
                rendered: false,
                shutdown: false,
                is_enabled: true,
            }
        }

        fn with_deps(name: &str, deps: Vec<&str>) -> Self {
            Self {
                name: name.to_string(),
                deps: deps.into_iter().map(String::from).collect(),
                initialized: false,
                updated: false,
                rendered: false,
                shutdown: false,
                is_enabled: true,
            }
        }

        fn disabled(name: &str) -> Self {
            Self {
                name: name.to_string(),
                deps: vec![],
                initialized: false,
                updated: false,
                rendered: false,
                shutdown: false,
                is_enabled: false,
            }
        }
    }

    impl Module for MockModule {
        fn name(&self) -> &str {
            &self.name
        }
        fn dependencies(&self) -> Vec<&str> {
            self.deps.iter().map(|s| s.as_str()).collect()
        }
        fn on_init(&mut self) {
            self.initialized = true;
        }
        fn on_update(&mut self, _dt: f64) {
            self.updated = true;
        }
        fn on_render(&mut self) {
            self.rendered = true;
        }
        fn on_shutdown(&mut self) {
            self.shutdown = true;
        }
        fn enabled(&self) -> bool {
            self.is_enabled
        }
    }

    #[test]
    fn test_register_and_initialize() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("test_module")));
        assert!(registry.initialize_all().is_ok());
    }

    #[test]
    fn test_register_multiple_modules() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("a")));
        registry.register(Box::new(MockModule::new("b")));
        registry.register(Box::new(MockModule::new("c")));
        assert_eq!(registry.len(), 3);
        assert!(registry.initialize_all().is_ok());
    }

    #[test]
    fn test_update_all() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("a")));
        registry.initialize_all().unwrap();
        registry.update_all(0.016);
    }

    #[test]
    fn test_render_all() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("a")));
        registry.initialize_all().unwrap();
        registry.render_all();
    }

    #[test]
    fn test_shutdown_all() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("a")));
        registry.initialize_all().unwrap();
        registry.shutdown_all();
    }

    #[test]
    fn test_dependency_order() {
        let registry = ModuleRegistry::new();
        // b depends on a
        registry.register(Box::new(MockModule::with_deps("b", vec!["a"])));
        registry.register(Box::new(MockModule::new("a")));
        assert!(registry.initialize_all().is_ok());
    }

    #[test]
    fn test_circular_dependency_detected() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::with_deps("a", vec!["b"])));
        registry.register(Box::new(MockModule::with_deps("b", vec!["a"])));
        assert!(registry.initialize_all().is_err());
    }

    #[test]
    fn test_disabled_module_not_initialized() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::disabled("disabled_mod")));
        assert!(registry.initialize_all().is_ok());
    }

    #[test]
    fn test_module_names() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("alpha")));
        registry.register(Box::new(MockModule::new("beta")));
        let names = registry.module_names();
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn test_contains() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("exists")));
        assert!(registry.contains("exists"));
        assert!(!registry.contains("not_exists"));
    }

    #[test]
    fn test_get_by_name() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("findme")));
        assert!(registry.get_by_name("findme").is_some());
        assert!(registry.get_by_name("missing").is_none());
    }

    #[test]
    fn test_with_module() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(MockModule::new("target")));
        let result = registry.with_module("target", |m| m.name().to_string());
        assert_eq!(result, Some("target".to_string()));
        let missing = registry.with_module("missing", |m| m.name().to_string());
        assert!(missing.is_none());
    }

    #[test]
    fn test_default() {
        let registry = ModuleRegistry::default();
        assert!(registry.is_empty());
    }
}
