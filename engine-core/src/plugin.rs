/// 插件 trait
///
/// 与 Module 的简化版，便于后续生态扩展
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn build(&self, app: &mut PluginApp);
}

/// 插件构建器
pub struct PluginApp {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginApp {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginApp {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) {
        self.plugins.push(Box::new(plugin));
    }
}

/// 插件组 — 成组安装插件
pub trait PluginGroup: Send + Sync {
    fn build(&self, app: &mut PluginApp);
}

/// 默认插件组（后续逐步补充）
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&self, _app: &mut PluginApp) {
        // 后续补充默认插件
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;
    impl Plugin for TestPlugin {
        fn name(&self) -> &str { "TestPlugin" }
        fn build(&self, _app: &mut PluginApp) {}
    }

    #[test]
    fn test_plugin_app() {
        let mut app = PluginApp::new();
        app.add_plugin(TestPlugin);
        assert_eq!(app.plugins.len(), 1);
        assert_eq!(app.plugins[0].name(), "TestPlugin");
    }

    #[test]
    fn test_default_plugins() {
        let mut app = PluginApp::new();
        DefaultPlugins.build(&mut app);
        // Currently no default plugins
        assert!(app.plugins.is_empty());
    }

    #[test]
    fn test_plugin_app_default() {
        let app = PluginApp::default();
        assert!(app.plugins.is_empty());
    }
}
