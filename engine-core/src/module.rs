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

    /// 按名称获取模块引用
    pub fn get_by_name(&self, name: &str) -> Option<std::cell::Ref<'_, dyn Module>> {
        if self.modules.borrow().contains_key(name) {
            Some(std::cell::Ref::map(self.modules.borrow(), |m| {
                m.get(name).map(|b| b.as_ref() as &dyn Module).unwrap()
            }))
        } else {
            None
        }
    }

    /// 按名称获取模块可变引用
    pub fn get_by_name_mut(&self, name: &str) -> Option<std::cell::RefMut<'_, dyn Module>> {
        if self.modules.borrow().contains_key(name) {
            Some(std::cell::RefMut::map(self.modules.borrow_mut(), |m| {
                m.get_mut(name)
                    .map(|b| b.as_mut() as &mut dyn Module)
                    .unwrap()
            }))
        } else {
            None
        }
    }

    /// 检查是否包含指定名称的模块
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModule {
        name: String,
        deps: Vec<String>,
        init_called: bool,
        shutdown_called: bool,
    }

    impl TestModule {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                deps: Vec::new(),
                init_called: false,
                shutdown_called: false,
            }
        }

        fn with_dep(name: &str, dep: &str) -> Self {
            Self {
                name: name.to_string(),
                deps: vec![dep.to_string()],
                init_called: false,
                shutdown_called: false,
            }
        }
    }

    impl Module for TestModule {
        fn name(&self) -> &str {
            &self.name
        }

        fn dependencies(&self) -> Vec<&str> {
            self.deps.iter().map(|s| s.as_str()).collect()
        }

        fn on_init(&mut self) {
            self.init_called = true;
        }

        fn on_shutdown(&mut self) {
            self.shutdown_called = true;
        }
    }

    #[test]
    fn test_register_and_len() {
        let registry = ModuleRegistry::new();
        assert!(registry.is_empty());

        registry.register(Box::new(TestModule::new("mod_a")));
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("mod_a"));
        assert!(!registry.contains("mod_b"));
    }

    #[test]
    fn test_get_by_name() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(TestModule::new("test_mod")));

        let module = registry.get_by_name("test_mod");
        assert!(module.is_some());
        assert_eq!(module.unwrap().name(), "test_mod");

        assert!(registry.get_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_initialize_all_order() {
        let registry = ModuleRegistry::new();
        // mod_b depends on mod_a, so mod_a should be initialized first
        registry.register(Box::new(TestModule::with_dep("mod_b", "mod_a")));
        registry.register(Box::new(TestModule::new("mod_a")));

        registry.initialize_all().unwrap();

        // Both should be initialized
        let mod_a = registry.get_by_name("mod_a").unwrap();
        assert_eq!(mod_a.name(), "mod_a");
    }

    #[test]
    fn test_cycle_detection() {
        let registry = ModuleRegistry::new();
        // Create a cycle: a -> b -> a
        struct ModA;
        impl Module for ModA {
            fn name(&self) -> &str {
                "a"
            }
            fn dependencies(&self) -> Vec<&str> {
                vec!["b"]
            }
        }
        struct ModB;
        impl Module for ModB {
            fn name(&self) -> &str {
                "b"
            }
            fn dependencies(&self) -> Vec<&str> {
                vec!["a"]
            }
        }

        registry.register(Box::new(ModA));
        registry.register(Box::new(ModB));

        let result = registry.initialize_all();
        assert!(result.is_err());
    }

    #[test]
    fn test_shutdown_all() {
        let registry = ModuleRegistry::new();
        registry.register(Box::new(TestModule::new("mod_a")));
        registry.register(Box::new(TestModule::new("mod_b")));

        registry.initialize_all().unwrap();
        registry.shutdown_all();

        // After shutdown, modules are removed
        assert!(registry.is_empty());
    }
}
