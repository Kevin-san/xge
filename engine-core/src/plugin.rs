//! 插件系统接口

/// 插件 trait — 简化版 Module，用于生态扩展
pub trait Plugin: Send + Sync + 'static {
    /// 获取插件唯一名称
    fn name(&self) -> &str;

    /// 构建插件（注册模块等）
    fn build(&self, app: &mut crate::AppBuilder);
}

/// 插件组 — 成组安装插件
pub struct PluginGroup(Vec<Box<dyn Plugin>>);

impl PluginGroup {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        self.0.push(Box::new(plugin));
        self
    }

    pub fn build(self, app: &mut crate::AppBuilder) {
        for plugin in self.0 {
            plugin.build(app);
        }
    }
}

impl Default for PluginGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// 默认插件组
pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn name(&self) -> &str {
        "DefaultPlugins"
    }

    fn build(&self, _app: &mut crate::AppBuilder) {
        // Sprint 01 阶段暂不注册任何实际插件
        // 后续 sprint 将逐步补充：日志、时间、文件系统等
    }
}
