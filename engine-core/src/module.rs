use std::any::Any;
use std::collections::HashMap;

pub struct EngineContext<'a> {
    config: &'a crate::engine::EngineConfig,
    time: &'a engine_platform::Time,
    filesystem: &'a engine_platform::FileSystem,
    thread_pool: &'a engine_platform::ThreadPool,
}

impl<'a> EngineContext<'a> {
    pub fn new(
        config: &'a crate::engine::EngineConfig,
        time: &'a engine_platform::Time,
        filesystem: &'a engine_platform::FileSystem,
        thread_pool: &'a engine_platform::ThreadPool,
    ) -> Self {
        Self {
            config,
            time,
            filesystem,
            thread_pool,
        }
    }

    pub fn config(&self) -> &'a crate::engine::EngineConfig {
        self.config
    }

    pub fn time(&self) -> &'a engine_platform::Time {
        self.time
    }

    pub fn filesystem(&self) -> &'a engine_platform::FileSystem {
        self.filesystem
    }

    pub fn spawn_task<F>(&self, future: F)
    where
        F: futures_lite::Future<Output = ()> + Send + 'static,
    {
        self.thread_pool.spawn_future(future);
    }
}

pub trait Module: Send + Sync + Any {
    fn name(&self) -> &str;

    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }

    fn on_init(&mut self, _engine: &EngineContext<'_>) {}

    fn on_update(&mut self, _engine: &EngineContext<'_>, _dt: f64) {}

    fn on_render(&mut self, _engine: &EngineContext<'_>) {}

    fn on_shutdown(&mut self, _engine: &EngineContext<'_>) {}

    fn enabled(&self) -> bool {
        true
    }
}

impl dyn Module {
    pub fn is<T: Module + 'static>(&self) -> bool {
        (self as &dyn Any).is::<T>()
    }

    pub fn downcast_ref<T: Module + 'static>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Module + 'static>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut::<T>()
    }
}

pub struct ModuleRegistry {
    modules: HashMap<String, Box<dyn Module>>,
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, module: Box<dyn Module>) {
        let name = module.name().to_string();
        self.modules.insert(name, module);
    }

    pub fn get<T: Module + 'static>(&self) -> Option<&T> {
        self.modules
            .values()
            .find(|m| m.is::<T>())
            .and_then(|m| m.downcast_ref::<T>())
    }

    pub fn get_mut<T: Module + 'static>(&mut self) -> Option<&mut T> {
        self.modules
            .values_mut()
            .find(|m| m.is::<T>())
            .and_then(|m| m.downcast_mut::<T>())
    }

    pub fn get_by_name(&self, name: &str) -> Option<&dyn Module> {
        self.modules.get(name).map(|m| m.as_ref())
    }

    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut dyn Module> {
        self.modules.get_mut(name).map(|m| m.as_mut())
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub fn initialize_all(&mut self, context: &EngineContext<'_>) -> Result<(), CycleError> {
        let sorted = topological_sort(&mut self.modules)?;

        for name in sorted {
            if let Some(module) = self.modules.get_mut(&name) {
                if module.enabled() {
                    module.on_init(context);
                }
            }
        }

        Ok(())
    }

    pub fn update_all(&mut self, context: &EngineContext<'_>, dt: f64) {
        for module in self.modules.values_mut() {
            if module.enabled() {
                module.on_update(context, dt);
            }
        }
    }

    pub fn render_all(&mut self, context: &EngineContext<'_>) {
        for module in self.modules.values_mut() {
            if module.enabled() {
                module.on_render(context);
            }
        }
    }

    pub fn shutdown_all(&mut self, context: &EngineContext<'_>) {
        let names: Vec<String> = self.modules.keys().cloned().collect();
        for name in names.into_iter().rev() {
            if let Some(mut module) = self.modules.remove(&name) {
                if module.enabled() {
                    module.on_shutdown(context);
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
                if !modules.contains_key(dep) {
                    return Err(CycleError(format!("Missing dependency '{}' for '{}'", dep, name)));
                }
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
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CycleError {}