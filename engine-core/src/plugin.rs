use crate::engine::Engine;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;

    fn build(&self, _engine: &mut Engine) {}
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn build_all(&self, engine: &mut Engine) {
        for plugin in &self.plugins {
            plugin.build(engine);
        }
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    struct TestPlugin {
        built: Arc<AtomicBool>,
    }

    impl TestPlugin {
        fn new(built: Arc<AtomicBool>) -> Self {
            Self { built }
        }
    }

    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "TestPlugin"
        }

        fn build(&self, _engine: &mut Engine) {
            self.built.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_plugin_manager_add() {
        let mut manager = PluginManager::new();
        assert_eq!(manager.len(), 0);

        let built = Arc::new(AtomicBool::new(false));
        manager.add_plugin(TestPlugin::new(built));
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_plugin_manager_build_all() {
        let mut manager = PluginManager::new();
        let built = Arc::new(AtomicBool::new(false));

        manager.add_plugin(TestPlugin::new(built.clone()));
        manager.build_all(&mut Engine::default());

        assert!(built.load(Ordering::SeqCst));
    }

    #[test]
    fn test_plugin_manager_empty() {
        let manager = PluginManager::new();
        assert!(manager.is_empty());
    }
}
