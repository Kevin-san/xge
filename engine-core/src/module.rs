use crate::Engine;
use std::any::Any;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct CycleError {
    message: String,
}

impl CycleError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CycleError: {}", self.message)
    }
}

impl std::error::Error for CycleError {}

pub trait Module: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn dependencies(&self) -> Vec<&str>;
    fn on_init(&mut self, engine: &Engine);
    fn on_update(&mut self, engine: &mut Engine, dt: f64);
    fn on_render(&mut self, engine: &mut Engine);
    fn on_shutdown(&mut self, engine: &Engine);
    fn enabled(&self) -> bool;
}

pub struct ModuleRegistry {
    modules: Vec<Box<dyn Module>>,
    name_to_index: HashMap<String, usize>,
    type_names: HashMap<String, String>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            name_to_index: HashMap::new(),
            type_names: HashMap::new(),
        }
    }

    pub fn register<M: Module + 'static>(&mut self, module: M) {
        let name = module.name().to_string();
        let type_name = std::any::type_name::<M>().to_string();
        
        if self.name_to_index.contains_key(&name) {
            return;
        }

        let index = self.modules.len();
        self.modules.push(Box::new(module));
        self.name_to_index.insert(name.clone(), index);
        self.type_names.insert(type_name, name);
    }

    pub fn get<M: Module + 'static>(&self) -> Option<&M> {
        let type_name = std::any::type_name::<M>();
        if let Some(name) = self.type_names.get(type_name) {
            if let Some(&index) = self.name_to_index.get(name) {
                self.modules[index].as_any().downcast_ref::<M>()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_mut<M: Module + 'static>(&mut self) -> Option<&mut M> {
        let type_name = std::any::type_name::<M>();
        if let Some(name) = self.type_names.get(type_name) {
            if let Some(&index) = self.name_to_index.get(name) {
                self.modules[index].as_any_mut().downcast_mut::<M>()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<&dyn Module> {
        if let Some(&index) = self.name_to_index.get(name) {
            Some(&*self.modules[index])
        } else {
            None
        }
    }

    pub fn initialize_all(&mut self, engine: &Engine) -> Result<(), CycleError> {
        let order = self.topological_sort()?;
        
        for &index in &order {
            if self.modules[index].enabled() {
                self.modules[index].on_init(engine);
            }
        }
        
        Ok(())
    }

    pub fn update_all(&mut self, engine: &mut Engine, dt: f64) {
        for module in self.modules.iter_mut() {
            if module.enabled() {
                module.on_update(engine, dt);
            }
        }
    }

    pub fn shutdown_all(&mut self, engine: &Engine) {
        for module in self.modules.iter_mut().rev() {
            if module.enabled() {
                module.on_shutdown(engine);
            }
        }
    }

    fn topological_sort(&self) -> Result<Vec<usize>, CycleError> {
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();
        let mut order = Vec::new();

        for (name, &index) in &self.name_to_index {
            if !visited.contains(name) {
                self.dfs(name, index, &mut visited, &mut in_stack, &mut order)?;
            }
        }

        order.reverse();
        Ok(order)
    }

    fn dfs(
        &self,
        name: &str,
        index: usize,
        visited: &mut HashSet<String>,
        in_stack: &mut HashSet<String>,
        order: &mut Vec<usize>,
    ) -> Result<(), CycleError> {
        if in_stack.contains(name) {
            return Err(CycleError::new(format!("Dependency cycle detected involving {}", name)));
        }

        if visited.contains(name) {
            return Ok(());
        }

        visited.insert(name.to_string());
        in_stack.insert(name.to_string());

        let module = &self.modules[index];
        for dep_name in module.dependencies() {
            if let Some(&dep_index) = self.name_to_index.get(dep_name) {
                self.dfs(dep_name, dep_index, visited, in_stack, order)?;
            } else {
                return Err(CycleError::new(format!("Missing dependency: {} for {}", dep_name, name)));
            }
        }

        in_stack.remove(name);
        order.push(index);

        Ok(())
    }
}

trait ModuleExt: Module {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<M: Module> ModuleExt for M {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ModuleA {
        name: String,
    }

    impl ModuleA {
        fn new() -> Self {
            Self { name: "ModuleA".to_string() }
        }
    }

    impl Module for ModuleA {
        fn name(&self) -> &str { &self.name }
        fn dependencies(&self) -> Vec<&str> { Vec::new() }
        fn on_init(&mut self, _engine: &Engine) {}
        fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
        fn on_render(&mut self, _engine: &Engine) {}
        fn on_shutdown(&mut self, _engine: &Engine) {}
        fn enabled(&self) -> bool { true }
    }

    struct ModuleB {
        name: String,
    }

    impl ModuleB {
        fn new() -> Self {
            Self { name: "ModuleB".to_string() }
        }
    }

    impl Module for ModuleB {
        fn name(&self) -> &str { &self.name }
        fn dependencies(&self) -> Vec<&str> { vec!["ModuleA"] }
        fn on_init(&mut self, _engine: &Engine) {}
        fn on_update(&mut self, _engine: &mut Engine, _dt: f64) {}
        fn on_render(&mut self, _engine: &Engine) {}
        fn on_shutdown(&mut self, _engine: &Engine) {}
        fn enabled(&self) -> bool { true }
    }

    #[test]
    fn module_name() {
        let module = ModuleA::new();
        assert_eq!(module.name(), "ModuleA");
    }

    #[test]
    fn module_dependencies() {
        let module = ModuleB::new();
        assert_eq!(module.dependencies(), vec!["ModuleA"]);
    }

    #[test]
    fn registry_register() {
        let mut registry = ModuleRegistry::new();
        registry.register(ModuleA::new());
        assert!(registry.get_by_name("ModuleA").is_some());
    }

    #[test]
    fn registry_get_by_type() {
        let mut registry = ModuleRegistry::new();
        registry.register(ModuleA::new());
        let module = registry.get::<ModuleA>();
        assert!(module.is_some());
    }

    #[test]
    fn registry_get_by_name() {
        let mut registry = ModuleRegistry::new();
        registry.register(ModuleA::new());
        let module = registry.get_by_name("ModuleA");
        assert!(module.is_some());
    }

    #[test]
    fn registry_init_order() {
        let mut registry = ModuleRegistry::new();
        registry.register(ModuleB::new());
        registry.register(ModuleA::new());
        
        let order = registry.topological_sort().unwrap();
        let a_index = registry.name_to_index["ModuleA"];
        let b_index = registry.name_to_index["ModuleB"];
        
        assert!(order.iter().position(|&i| i == a_index) < order.iter().position(|&i| i == b_index));
    }

    #[test]
    fn registry_missing_dep() {
        let mut registry = ModuleRegistry::new();
        registry.register(ModuleB::new());
        
        let result = registry.initialize_all(&Engine::new(EngineConfig::default()));
        assert!(result.is_err());
    }
}
