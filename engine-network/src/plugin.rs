//! Plugin system for extensibility.

use crate::error::{NetError, NetResult};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Plugin state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PluginState {
    /// Plugin is not loaded
    #[default]
    Unloaded,
    /// Plugin is loading
    Loading,
    /// Plugin is loaded and active
    Active,
    /// Plugin is paused
    Paused,
    /// Plugin has error
    Error,
}

/// Plugin identifier type
pub type PluginId = u64;

/// Plugin trait for network extensions
pub trait Plugin: Send + Sync {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Get plugin version
    fn version(&self) -> &str;

    /// Get plugin ID
    fn id(&self) -> PluginId;

    /// Initialize the plugin
    fn on_init(&self, context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    /// Called when plugin is activated
    fn on_activate(&self, context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    /// Called when plugin is deactivated
    fn on_deactivate(&self, context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    /// Called on each tick/update
    fn on_tick(&self, context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    /// Handle incoming message
    fn on_message(
        &self,
        context: &PluginContext,
        message_type: u32,
        data: &[u8],
    ) -> NetResult<Option<Vec<u8>>> {
        Ok(None)
    }

    /// Called before shutdown
    fn on_shutdown(&self, context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    /// Get plugin state
    fn state(&self) -> PluginState;
}

/// Plugin context providing access to network manager
pub struct PluginContext {
    /// Plugin manager reference
    manager: Arc<PluginManager>,
    /// Configuration data
    config: HashMap<String, String>,
}

impl PluginContext {
    /// Create a new plugin context
    pub fn new(manager: Arc<PluginManager>) -> Self {
        Self {
            manager,
            config: HashMap::new(),
        }
    }

    /// Get configuration value
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    /// Set configuration value
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    /// Get plugin manager reference
    pub fn manager(&self) -> &Arc<PluginManager> {
        &self.manager
    }
}

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Dependencies (other plugin names)
    pub dependencies: Vec<String>,
    /// Priority (higher = loaded first)
    pub priority: i32,
}

impl Default for PluginMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: String::new(),
            dependencies: Vec::new(),
            priority: 0,
        }
    }
}

/// Plugin manager for managing plugins
pub struct PluginManager {
    plugins: Mutex<HashMap<PluginId, Arc<dyn Plugin>>>,
    metadata: Mutex<HashMap<PluginId, PluginMetadata>>,
    states: Mutex<HashMap<PluginId, PluginState>>,
    next_plugin_id: Mutex<PluginId>,
    running: AtomicBool,
    tick_interval_ms: u64,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: Mutex::new(HashMap::new()),
            metadata: Mutex::new(HashMap::new()),
            states: Mutex::new(HashMap::new()),
            next_plugin_id: Mutex::new(1),
            running: AtomicBool::new(false),
            tick_interval_ms: 100,
        }
    }

    /// Set tick interval
    pub fn set_tick_interval(&mut self, interval_ms: u64) {
        self.tick_interval_ms = interval_ms;
    }

    /// Register a plugin
    pub fn register(
        &self,
        plugin: Arc<dyn Plugin>,
        metadata: PluginMetadata,
    ) -> NetResult<PluginId> {
        let mut next_id = self.next_plugin_id.lock();
        let id = *next_id;
        *next_id += 1;

        let mut plugins = self.plugins.lock();
        let mut meta = self.metadata.lock();
        let mut states = self.states.lock();

        plugins.insert(id, plugin.clone());
        meta.insert(id, metadata);
        states.insert(id, PluginState::Unloaded);

        Ok(id)
    }

    /// Unregister a plugin
    pub fn unregister(&self, id: PluginId) -> NetResult<()> {
        let mut plugins = self.plugins.lock();
        let mut meta = self.metadata.lock();
        let mut states = self.states.lock();

        // Shutdown plugin first if active
        if let Some(plugin) = plugins.get(&id) {
            if plugin.state() == PluginState::Active {
                let context = PluginContext::new(Arc::new(Self::new()));
                plugin.on_shutdown(&context)?;
            }
        }

        plugins.remove(&id);
        meta.remove(&id);
        states.remove(&id);

        Ok(())
    }

    /// Load a plugin
    pub fn load(&self, id: PluginId) -> NetResult<()> {
        let plugins = self.plugins.lock();
        let plugin = plugins
            .get(&id)
            .ok_or(NetError::Plugin("Plugin not found".to_string()))?;

        let context = PluginContext::new(Arc::new(Self::new()));

        // Update state to loading
        let mut states = self.states.lock();
        states.insert(id, PluginState::Loading);

        // Initialize plugin
        plugin.on_init(&context)?;

        // Activate plugin
        plugin.on_activate(&context)?;

        // Update state to active
        states.insert(id, PluginState::Active);

        Ok(())
    }

    /// Unload a plugin
    pub fn unload(&self, id: PluginId) -> NetResult<()> {
        let plugins = self.plugins.lock();
        let plugin = plugins
            .get(&id)
            .ok_or(NetError::Plugin("Plugin not found".to_string()))?;

        let context = PluginContext::new(Arc::new(Self::new()));

        // Deactivate plugin
        plugin.on_deactivate(&context)?;

        // Shutdown plugin
        plugin.on_shutdown(&context)?;

        // Update state to unloaded
        let mut states = self.states.lock();
        states.insert(id, PluginState::Unloaded);

        Ok(())
    }

    /// Pause a plugin
    pub fn pause(&self, id: PluginId) -> NetResult<()> {
        // Check manager's state tracking instead of plugin's internal state
        let current_state = self.get_state(id);
        if current_state != Some(PluginState::Active) {
            return Err(NetError::InvalidState("Plugin not active".to_string()));
        }

        let plugins = self.plugins.lock();
        let plugin = plugins
            .get(&id)
            .ok_or(NetError::Plugin("Plugin not found".to_string()))?;

        let context = PluginContext::new(Arc::new(Self::new()));
        plugin.on_deactivate(&context)?;

        let mut states = self.states.lock();
        states.insert(id, PluginState::Paused);

        Ok(())
    }

    /// Resume a paused plugin
    pub fn resume(&self, id: PluginId) -> NetResult<()> {
        // Check manager's state tracking instead of plugin's internal state
        let current_state = self.get_state(id);
        if current_state != Some(PluginState::Paused) {
            return Err(NetError::InvalidState("Plugin not paused".to_string()));
        }

        let plugins = self.plugins.lock();
        let plugin = plugins
            .get(&id)
            .ok_or(NetError::Plugin("Plugin not found".to_string()))?;

        let context = PluginContext::new(Arc::new(Self::new()));
        plugin.on_activate(&context)?;

        let mut states = self.states.lock();
        states.insert(id, PluginState::Active);

        Ok(())
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, id: PluginId) -> Option<Arc<dyn Plugin>> {
        let plugins = self.plugins.lock();
        plugins.get(&id).cloned()
    }

    /// Get plugin state
    pub fn get_state(&self, id: PluginId) -> Option<PluginState> {
        let states = self.states.lock();
        states.get(&id).copied()
    }

    /// Get plugin metadata
    pub fn get_metadata(&self, id: PluginId) -> Option<PluginMetadata> {
        let meta = self.metadata.lock();
        meta.get(&id).cloned()
    }

    /// Get all plugin IDs
    pub fn plugin_ids(&self) -> Vec<PluginId> {
        let plugins = self.plugins.lock();
        plugins.keys().copied().collect()
    }

    /// Get number of registered plugins
    pub fn plugin_count(&self) -> usize {
        let plugins = self.plugins.lock();
        plugins.len()
    }

    /// Get active plugin IDs
    pub fn active_plugins(&self) -> Vec<PluginId> {
        let states = self.states.lock();
        states
            .iter()
            .filter(|(_, state)| **state == PluginState::Active)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Tick all active plugins
    pub fn tick(&self) -> NetResult<()> {
        let context = PluginContext::new(Arc::new(Self::new()));
        let plugins = self.plugins.lock();

        for plugin in plugins.values() {
            if plugin.state() == PluginState::Active {
                plugin.on_tick(&context)?;
            }
        }

        Ok(())
    }

    /// Handle message through plugins
    pub fn handle_message(&self, message_type: u32, data: &[u8]) -> NetResult<Option<Vec<u8>>> {
        let context = PluginContext::new(Arc::new(Self::new()));
        let plugins = self.plugins.lock();

        for plugin in plugins.values() {
            if plugin.state() == PluginState::Active {
                if let Some(result) = plugin.on_message(&context, message_type, data)? {
                    return Ok(Some(result));
                }
            }
        }

        Ok(None)
    }

    /// Start the plugin manager
    pub fn start(&self) -> NetResult<()> {
        self.running.store(true, Ordering::SeqCst);

        // Load all registered plugins
        let ids = self.plugin_ids();
        for id in ids {
            self.load(id)?;
        }

        Ok(())
    }

    /// Stop the plugin manager
    pub fn stop(&self) -> NetResult<()> {
        self.running.store(false, Ordering::SeqCst);

        // Unload all plugins
        let ids = self.plugin_ids();
        for id in ids {
            self.unload(id)?;
        }

        Ok(())
    }

    /// Check if manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Example plugin implementation
pub struct ExamplePlugin {
    id: PluginId,
    name: String,
    version: String,
    state: AtomicBool,
}

impl ExamplePlugin {
    /// Create a new example plugin
    pub fn new(name: String, version: String) -> Self {
        Self {
            id: 0,
            name,
            version,
            state: AtomicBool::new(false),
        }
    }

    /// Set plugin ID
    pub fn set_id(&mut self, id: PluginId) {
        self.id = id;
    }
}

impl Plugin for ExamplePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn id(&self) -> PluginId {
        self.id
    }

    fn on_init(&self, _context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    fn on_activate(&self, _context: &PluginContext) -> NetResult<()> {
        self.state.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn on_deactivate(&self, _context: &PluginContext) -> NetResult<()> {
        self.state.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn on_shutdown(&self, _context: &PluginContext) -> NetResult<()> {
        self.state.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn state(&self) -> PluginState {
        if self.state.load(Ordering::SeqCst) {
            PluginState::Active
        } else {
            PluginState::Unloaded
        }
    }
}

/// Hot update plugin for runtime updates
pub struct HotUpdatePlugin {
    id: PluginId,
    name: String,
    version: String,
    state: AtomicBool,
    pending_updates: Mutex<Vec<String>>,
}

impl Default for HotUpdatePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl HotUpdatePlugin {
    /// Create a new hot update plugin
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "hot-update".to_string(),
            version: "0.1.0".to_string(),
            state: AtomicBool::new(false),
            pending_updates: Mutex::new(Vec::new()),
        }
    }

    /// Set plugin ID
    pub fn set_id(&mut self, id: PluginId) {
        self.id = id;
    }

    /// Add pending update
    pub fn add_pending_update(&self, update_path: String) {
        let mut updates = self.pending_updates.lock();
        updates.push(update_path);
    }

    /// Get pending updates
    pub fn pending_updates(&self) -> Vec<String> {
        let updates = self.pending_updates.lock();
        updates.clone()
    }

    /// Apply updates
    pub fn apply_updates(&self) -> NetResult<()> {
        let mut updates = self.pending_updates.lock();
        updates.clear();
        Ok(())
    }
}

impl Plugin for HotUpdatePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn id(&self) -> PluginId {
        self.id
    }

    fn on_init(&self, _context: &PluginContext) -> NetResult<()> {
        Ok(())
    }

    fn on_activate(&self, _context: &PluginContext) -> NetResult<()> {
        self.state.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn on_deactivate(&self, _context: &PluginContext) -> NetResult<()> {
        self.state.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn on_tick(&self, _context: &PluginContext) -> NetResult<()> {
        // Check for pending updates and apply them
        if !self.pending_updates().is_empty() {
            self.apply_updates()?;
        }
        Ok(())
    }

    fn on_shutdown(&self, _context: &PluginContext) -> NetResult<()> {
        self.state.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn state(&self) -> PluginState {
        if self.state.load(Ordering::SeqCst) {
            PluginState::Active
        } else {
            PluginState::Unloaded
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugin_count(), 0);
        assert!(!manager.is_running());
    }

    #[test]
    fn test_plugin_registration() {
        let manager = PluginManager::new();
        let plugin = Arc::new(ExamplePlugin::new(
            "test-plugin".to_string(),
            "1.0.0".to_string(),
        ));

        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            dependencies: Vec::new(),
            priority: 0,
        };

        let id = manager.register(plugin, metadata).unwrap();
        assert_eq!(manager.plugin_count(), 1);
        assert!(manager.get_plugin(id).is_some());
    }

    #[test]
    fn test_plugin_load_unload() {
        let manager = PluginManager::new();
        let plugin = Arc::new(ExamplePlugin::new(
            "test-plugin".to_string(),
            "1.0.0".to_string(),
        ));

        let metadata = PluginMetadata::default();
        let id = manager.register(plugin, metadata).unwrap();

        manager.load(id).unwrap();
        assert_eq!(manager.get_state(id), Some(PluginState::Active));

        manager.unload(id).unwrap();
        assert_eq!(manager.get_state(id), Some(PluginState::Unloaded));
    }

    #[test]
    fn test_plugin_pause_resume() {
        let manager = PluginManager::new();
        let plugin = Arc::new(ExamplePlugin::new(
            "test-plugin".to_string(),
            "1.0.0".to_string(),
        ));

        let metadata = PluginMetadata::default();
        let id = manager.register(plugin, metadata).unwrap();

        manager.load(id).unwrap();
        assert_eq!(manager.get_state(id), Some(PluginState::Active));

        manager.pause(id).unwrap();
        assert_eq!(manager.get_state(id), Some(PluginState::Paused));

        manager.resume(id).unwrap();
        assert_eq!(manager.get_state(id), Some(PluginState::Active));
    }

    #[test]
    fn test_plugin_manager_start_stop() {
        let manager = PluginManager::new();
        let plugin = Arc::new(ExamplePlugin::new(
            "test-plugin".to_string(),
            "1.0.0".to_string(),
        ));

        let metadata = PluginMetadata::default();
        manager.register(plugin, metadata).unwrap();

        manager.start().unwrap();
        assert!(manager.is_running());

        manager.stop().unwrap();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_hot_update_plugin() {
        let plugin = HotUpdatePlugin::new();
        plugin.add_pending_update("update1.dll".to_string());
        plugin.add_pending_update("update2.dll".to_string());

        let updates = plugin.pending_updates();
        assert_eq!(updates.len(), 2);
        assert!(updates.contains(&"update1.dll".to_string()));
    }

    #[test]
    fn test_plugin_context() {
        let manager = Arc::new(PluginManager::new());
        let mut context = PluginContext::new(manager.clone());

        context.set_config("key1".to_string(), "value1".to_string());
        assert_eq!(context.get_config("key1"), Some(&"value1".to_string()));
    }
}
