//! engine-editor crate - 可视化编辑器基础框架
//!
//! 提供游戏引擎的可视化编辑器 UI，包括场景视图、层级面板、属性面板、
//! 资源面板、控制台面板、调试面板等核心功能。

#![warn(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use engine_ecs::Entity;
use engine_scene::{NodeHandle, SceneTree};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

// ============================================================================
// 常量定义
// ============================================================================

/// 默认撤销栈最大长度
const DEFAULT_ACTION_STACK_MAX_LEN: usize = 50;

/// 默认自动保存间隔（秒）
const DEFAULT_AUTO_SAVE_INTERVAL_SECS: u64 = 300;

/// 默认字体大小
const DEFAULT_FONT_SIZE: u32 = 14;

// ============================================================================
// 核心类型定义
// ============================================================================

/// 编辑器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorMode {
    /// 编辑模式
    #[default]
    Edit,
    /// 运行模式
    Play,
    /// 暂停模式
    Paused,
}

/// 编辑器主题
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EditorTheme {
    /// 深色主题
    #[default]
    Dark,
    /// 浅色主题
    Light,
}

/// 面板 ID
pub type PanelId = &'static str;

/// 编辑器工具
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorTool {
    /// 选择工具
    #[default]
    Select,
    /// 移动工具
    Move,
    /// 旋转工具
    Rotate,
    /// 缩放工具
    Scale,
}

// ============================================================================
// EditorSelection - 选择集
// ============================================================================

/// 编辑器选择集
///
/// 用于管理当前选中的实体集合，支持多选操作。
#[derive(Debug, Clone, Default)]
pub struct EditorSelection {
    /// 选中的实体集合
    entities: HashSet<Entity>,
}

/// 选择变更事件
#[derive(Debug, Clone)]
pub struct EditorSelectionChanged {
    /// 旧选择集
    old_selection: HashSet<Entity>,
    /// 新选择集
    new_selection: HashSet<Entity>,
}

impl EditorSelection {
    /// 创建新的空选择集
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 清空选择集
    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    /// 选择单个实体（替换当前选择）
    #[inline]
    pub fn select(&mut self, entity: Entity) {
        self.entities.clear();
        self.entities.insert(entity);
    }

    /// 切换实体选择状态
    #[inline]
    pub fn toggle(&mut self, entity: Entity) {
        if self.entities.contains(&entity) {
            self.entities.remove(&entity);
        } else {
            self.entities.insert(entity);
        }
    }

    /// 添加实体到选择集
    #[inline]
    pub fn add(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    /// 从选择集移除实体
    #[inline]
    pub fn remove(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    /// 检查实体是否在选择集中
    #[inline]
    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }

    /// 检查选择集是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// 获取选择集大小
    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// 获取选择集迭代器
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    /// 获取第一个选中实体
    #[inline]
    pub fn first(&self) -> Option<Entity> {
        self.entities.iter().next().copied()
    }

    /// 获取最后一个选中实体（无序集合中取任意一个）
    #[inline]
    pub fn last(&self) -> Option<Entity> {
        self.entities.iter().last().copied()
    }

    /// 获取所有选中实体
    #[inline]
    pub fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }
}

impl EditorSelectionChanged {
    /// 创建选择变更事件
    #[inline]
    pub fn new(old_selection: HashSet<Entity>, new_selection: HashSet<Entity>) -> Self {
        Self {
            old_selection,
            new_selection,
        }
    }

    /// 获取旧选择集
    #[inline]
    pub fn old(&self) -> &HashSet<Entity> {
        &self.old_selection
    }

    /// 获取新选择集
    #[inline]
    pub fn new_selection(&self) -> &HashSet<Entity> {
        &self.new_selection
    }
}

// ============================================================================
// EditorSettings - 编辑器配置
// ============================================================================

/// 外部工具配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalTools {
    /// VS Code 路径
    pub vscode_path: Option<PathBuf>,
    /// Git 路径
    pub git_path: Option<PathBuf>,
    /// Cargo 路径
    pub cargo_path: Option<PathBuf>,
}

/// 键位绑定配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyBindings {
    /// 绑定映射
    bindings: HashMap<String, Vec<String>>,
}

impl KeyBindings {
    /// 创建新的键位绑定配置
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取指定操作的键位
    #[inline]
    pub fn get(&self, action: &str) -> Option<&[String]> {
        self.bindings.get(action).map(|v| v.as_slice())
    }

    /// 设置指定操作的键位
    #[inline]
    pub fn set(&mut self, action: &str, keys: Vec<String>) {
        self.bindings.insert(action.to_string(), keys);
    }

    /// 移除指定操作的键位
    #[inline]
    pub fn remove(&mut self, action: &str) {
        self.bindings.remove(action);
    }
}

/// 编辑器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// 主题
    pub theme: EditorTheme,
    /// 键位绑定
    pub key_bindings: KeyBindings,
    /// 字体大小
    pub font_size: u32,
    /// 外部工具
    pub external_tools: ExternalTools,
    /// 默认创建路径
    pub default_create_path: PathBuf,
    /// 自动保存开关
    pub auto_save: bool,
    /// 自动保存间隔
    pub auto_save_interval: Duration,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            theme: EditorTheme::default(),
            key_bindings: KeyBindings::new(),
            font_size: DEFAULT_FONT_SIZE,
            external_tools: ExternalTools::default(),
            default_create_path: PathBuf::from("assets"),
            auto_save: false,
            auto_save_interval: Duration::from_secs(DEFAULT_AUTO_SAVE_INTERVAL_SECS),
        }
    }
}

impl EditorSettings {
    /// 创建新的编辑器配置
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取主题
    #[inline]
    pub fn theme(&self) -> EditorTheme {
        self.theme
    }

    /// 设置主题
    #[inline]
    pub fn set_theme(&mut self, theme: EditorTheme) {
        self.theme = theme;
    }

    /// 获取自动保存开关
    #[inline]
    pub fn auto_save(&self) -> bool {
        self.auto_save
    }

    /// 设置自动保存开关
    #[inline]
    pub fn set_auto_save(&mut self, auto_save: bool) {
        self.auto_save = auto_save;
    }

    /// 获取自动保存间隔
    #[inline]
    pub fn auto_save_interval(&self) -> Duration {
        self.auto_save_interval
    }

    /// 设置自动保存间隔
    #[inline]
    pub fn set_auto_save_interval(&mut self, interval: Duration) {
        self.auto_save_interval = interval;
    }

    /// 获取键位绑定
    #[inline]
    pub fn key_bindings(&self) -> &KeyBindings {
        &self.key_bindings
    }

    /// 获取键位绑定可变引用
    #[inline]
    pub fn key_bindings_mut(&mut self) -> &mut KeyBindings {
        &mut self.key_bindings
    }

    /// 保存配置到文件
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 从文件加载配置
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let settings = serde_json::from_str(&json)?;
        Ok(settings)
    }
}

// ============================================================================
// EditorAction - 操作抽象
// ============================================================================

/// 编辑器操作 trait
///
/// 所有可撤销的操作必须实现此 trait。
pub trait EditorAction: std::any::Any + Send + Sync {
    /// 应用操作
    fn apply(&mut self, editor: &mut EditorState);

    /// 撤销操作
    fn undo(&mut self, editor: &mut EditorState);

    /// 是否可合并（用于连续相同操作的合并）
    fn mergeable(&self) -> bool {
        false
    }

    /// 获取操作名称
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// 克隆操作（用于测试）
    fn clone_action(&self) -> Box<dyn EditorAction>;
}

/// 创建节点操作
#[derive(Debug)]
pub struct CreateNodeAction {
    /// 创建的节点句柄
    pub node: NodeHandle,
    /// 父节点
    pub parent: Option<NodeHandle>,
    /// 节点名称
    pub name: String,
}

impl CreateNodeAction {
    /// 创建新的创建节点操作
    #[inline]
    pub fn new(parent: Option<NodeHandle>, name: String) -> Self {
        Self {
            node: NodeHandle::null(),
            parent,
            name,
        }
    }
}

impl EditorAction for CreateNodeAction {
    fn apply(&mut self, editor: &mut EditorState) {
        let parent = self.parent.unwrap_or(editor.scene.root());
        self.node = editor.scene.add_2d_node(parent, &self.name);
    }

    fn undo(&mut self, editor: &mut EditorState) {
        if !self.node.is_null() {
            editor.scene.destroy_node(self.node);
            editor.selection.remove(node_to_entity(self.node));
        }
    }

    fn name(&self) -> &str {
        "CreateNode"
    }

    fn clone_action(&self) -> Box<dyn EditorAction> {
        Box::new(CreateNodeAction {
            node: self.node,
            parent: self.parent,
            name: self.name.clone(),
        })
    }
}

/// 删除节点操作
#[derive(Debug)]
pub struct DeleteNodeAction {
    /// 删除的节点句柄
    pub node: NodeHandle,
    /// 父节点
    pub parent: Option<NodeHandle>,
    /// 节点名称
    pub name: String,
}

impl DeleteNodeAction {
    /// 创建新的删除节点操作
    #[inline]
    pub fn new(node: NodeHandle, parent: Option<NodeHandle>, name: String) -> Self {
        Self { node, parent, name }
    }
}

impl EditorAction for DeleteNodeAction {
    fn apply(&mut self, editor: &mut EditorState) {
        editor.scene.destroy_node(self.node);
        editor.selection.remove(node_to_entity(self.node));
    }

    fn undo(&mut self, editor: &mut EditorState) {
        let parent = self.parent.unwrap_or(editor.scene.root());
        self.node = editor.scene.add_2d_node(parent, &self.name);
    }

    fn name(&self) -> &str {
        "DeleteNode"
    }

    fn clone_action(&self) -> Box<dyn EditorAction> {
        Box::new(DeleteNodeAction {
            node: self.node,
            parent: self.parent,
            name: self.name.clone(),
        })
    }
}

/// 重命名节点操作
#[derive(Debug)]
pub struct RenameNodeAction {
    /// 节点句柄
    pub node: NodeHandle,
    /// 旧名称
    pub old_name: String,
    /// 新名称
    pub new_name: String,
}

impl RenameNodeAction {
    /// 创建新的重命名节点操作
    #[inline]
    pub fn new(node: NodeHandle, old_name: String, new_name: String) -> Self {
        Self {
            node,
            old_name,
            new_name,
        }
    }
}

impl EditorAction for RenameNodeAction {
    fn apply(&mut self, editor: &mut EditorState) {
        if let Some(node_mut) = editor.scene.get_node_mut(self.node) {
            node_mut.set_name(self.new_name.clone());
        }
    }

    fn undo(&mut self, editor: &mut EditorState) {
        if let Some(node_mut) = editor.scene.get_node_mut(self.node) {
            node_mut.set_name(self.old_name.clone());
        }
    }

    fn name(&self) -> &str {
        "RenameNode"
    }

    fn clone_action(&self) -> Box<dyn EditorAction> {
        Box::new(RenameNodeAction {
            node: self.node,
            old_name: self.old_name.clone(),
            new_name: self.new_name.clone(),
        })
    }
}

/// 批量操作
pub struct BatchAction {
    /// 子操作列表
    pub actions: Vec<Box<dyn EditorAction>>,
}

impl BatchAction {
    /// 创建新的批量操作
    #[inline]
    pub fn new(actions: Vec<Box<dyn EditorAction>>) -> Self {
        Self { actions }
    }
}

impl EditorAction for BatchAction {
    fn apply(&mut self, editor: &mut EditorState) {
        for action in &mut self.actions {
            action.apply(editor);
        }
    }

    fn undo(&mut self, editor: &mut EditorState) {
        // 撤销时逆序执行
        for action in self.actions.iter_mut().rev() {
            action.undo(editor);
        }
    }

    fn name(&self) -> &str {
        "BatchAction"
    }

    fn clone_action(&self) -> Box<dyn EditorAction> {
        Box::new(BatchAction {
            actions: self.actions.iter().map(|a| a.clone_action()).collect(),
        })
    }
}

// ============================================================================
// EditorActionStack - 撤销/重做栈
// ============================================================================

/// 编辑器操作栈
///
/// 用于管理撤销/重做操作。
pub struct EditorActionStack {
    /// 撤销栈
    undo_stack: Vec<Box<dyn EditorAction>>,
    /// 重做栈
    redo_stack: Vec<Box<dyn EditorAction>>,
    /// 最大长度
    max_len: usize,
}

impl Default for EditorActionStack {
    fn default() -> Self {
        Self::new(DEFAULT_ACTION_STACK_MAX_LEN)
    }
}

impl EditorActionStack {
    /// 创建新的操作栈
    #[inline]
    pub fn new(max_len: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_len,
        }
    }

    /// 推入操作
    pub fn push(&mut self, action: Box<dyn EditorAction>) {
        // 推入新操作时清空重做栈
        self.redo_stack.clear();

        // 尝试合并
        if let Some(last) = self.undo_stack.last_mut() {
            if last.mergeable() && action.mergeable() && last.name() == action.name() {
                // 合并操作（简化实现，实际需要更复杂的合并逻辑）
                self.undo_stack.push(action);
            } else {
                self.undo_stack.push(action);
            }
        } else {
            self.undo_stack.push(action);
        }

        // 超出最大长度时移除最旧的
        if self.undo_stack.len() > self.max_len {
            self.undo_stack.remove(0);
        }
    }

    /// 撤销 - 返回操作供外部调用
    pub fn pop_undo(&mut self) -> Option<Box<dyn EditorAction>> {
        self.undo_stack.pop()
    }

    /// 将操作推入重做栈（撤销后）
    pub fn push_redo(&mut self, action: Box<dyn EditorAction>) {
        self.redo_stack.push(action);
    }

    /// 重做 - 返回操作供外部调用
    pub fn pop_redo(&mut self) -> Option<Box<dyn EditorAction>> {
        self.redo_stack.pop()
    }

    /// 将操作推入撤销栈（重做后）
    pub fn push_undo(&mut self, action: Box<dyn EditorAction>) {
        self.undo_stack.push(action);
    }

    /// 清空栈
    #[inline]
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// 是否可撤销
    #[inline]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// 是否可重做
    #[inline]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// 获取撤销栈长度
    #[inline]
    pub fn len(&self) -> usize {
        self.undo_stack.len()
    }

    /// 检查撤销栈是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.undo_stack.is_empty()
    }

    /// 获取最大长度
    #[inline]
    pub fn max_len(&self) -> usize {
        self.max_len
    }
}

// ============================================================================
// EditorClipboard - 剪贴板
// ============================================================================

/// 编辑器剪贴板
///
/// 用于复制/粘贴节点、组件和资源路径。
#[derive(Debug, Default)]
pub struct EditorClipboard {
    /// 复制的节点列表
    nodes: Vec<NodeHandle>,
    /// 复制的资源路径列表
    asset_paths: Vec<PathBuf>,
}

impl EditorClipboard {
    /// 创建新的剪贴板
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 复制节点
    #[inline]
    pub fn copy_nodes(&mut self, nodes: &[NodeHandle]) {
        self.nodes = nodes.to_vec();
    }

    /// 获取复制的节点
    #[inline]
    pub fn nodes(&self) -> &[NodeHandle] {
        &self.nodes
    }

    /// 复制资源路径
    #[inline]
    pub fn copy_asset_paths(&mut self, paths: &[PathBuf]) {
        self.asset_paths = paths.to_vec();
    }

    /// 获取复制的资源路径
    #[inline]
    pub fn asset_paths(&self) -> &[PathBuf] {
        &self.asset_paths
    }

    /// 清空剪贴板
    #[inline]
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.asset_paths.clear();
    }

    /// 检查剪贴板是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.asset_paths.is_empty()
    }
}

// ============================================================================
// EditorPlugin - 插件系统
// ============================================================================

/// 编辑器插件 trait
///
/// 第三方扩展可通过实现此 trait 接入编辑器。
pub trait EditorPlugin: Send + Sync {
    /// 注册插件
    fn register(&mut self, editor: &mut EditorState);

    /// 更新插件
    fn update(&mut self, editor: &mut EditorState, dt: f32);

    /// 绘制插件 UI
    fn ui(&mut self, editor: &mut EditorState, ui: &mut UiContext);
}

/// 插件注册表
#[derive(Default)]
pub struct EditorPluginRegistry {
    /// 插件列表
    plugins: Vec<Box<dyn EditorPlugin>>,
}

impl EditorPluginRegistry {
    /// 创建新的插件注册表
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册插件
    #[inline]
    pub fn register(&mut self, plugin: Box<dyn EditorPlugin>) {
        self.plugins.push(plugin);
    }

    /// 更新所有插件
    pub fn update_all(&mut self, editor: &mut EditorState, dt: f32) {
        for plugin in &mut self.plugins {
            plugin.update(editor, dt);
        }
    }

    /// 绘制所有插件 UI
    pub fn ui_all(&mut self, editor: &mut EditorState, ui: &mut UiContext) {
        for plugin in &mut self.plugins {
            plugin.ui(editor, ui);
        }
    }

    /// 获取插件数量
    #[inline]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

// ============================================================================
// EditorState - 编辑器状态
// ============================================================================

/// 编辑器全局状态
///
/// 持有场景、选择集、撤销栈、剪贴板、设置和插件。
pub struct EditorState {
    /// 场景树
    pub scene: SceneTree,
    /// 选择集
    pub selection: EditorSelection,
    /// 操作栈
    pub action_stack: EditorActionStack,
    /// 剪贴板
    pub clipboard: EditorClipboard,
    /// 设置
    pub settings: EditorSettings,
    /// 插件注册表
    pub plugins: EditorPluginRegistry,
    /// 面板可见性
    panel_visibility: HashMap<PanelId, bool>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    /// 创建新的编辑器状态
    pub fn new() -> Self {
        let mut panel_visibility = HashMap::new();
        // 默认所有面板可见
        panel_visibility.insert("hierarchy", true);
        panel_visibility.insert("inspector", true);
        panel_visibility.insert("assets", true);
        panel_visibility.insert("console", true);
        panel_visibility.insert("debug", true);

        Self {
            scene: SceneTree::new(),
            selection: EditorSelection::new(),
            action_stack: EditorActionStack::default(),
            clipboard: EditorClipboard::new(),
            settings: EditorSettings::new(),
            plugins: EditorPluginRegistry::new(),
            panel_visibility,
        }
    }

    /// 更新编辑器状态
    pub fn tick(&mut self, dt: f32) {
        self.scene.update(dt);
        // 插件更新需要单独处理，避免借用冲突
        // 实际实现中应该使用更安全的方式
    }

    /// 显示面板
    #[inline]
    pub fn show_panel(&mut self, panel_id: PanelId) {
        self.panel_visibility.insert(panel_id, true);
    }

    /// 隐藏面板
    #[inline]
    pub fn hide_panel(&mut self, panel_id: PanelId) {
        self.panel_visibility.insert(panel_id, false);
    }

    /// 切换面板可见性
    #[inline]
    pub fn toggle_panel(&mut self, panel_id: PanelId) {
        let visible = self.panel_visibility.get(panel_id).copied().unwrap_or(true);
        self.panel_visibility.insert(panel_id, !visible);
    }

    /// 检查面板是否可见
    #[inline]
    pub fn is_panel_visible(&self, panel_id: PanelId) -> bool {
        self.panel_visibility.get(panel_id).copied().unwrap_or(true)
    }

    /// 重置布局
    pub fn reset_layout(&mut self) {
        self.panel_visibility.insert("hierarchy", true);
        self.panel_visibility.insert("inspector", true);
        self.panel_visibility.insert("assets", true);
        self.panel_visibility.insert("console", true);
        self.panel_visibility.insert("debug", true);
    }

    /// 新建场景
    pub fn new_scene(&mut self) {
        self.scene = SceneTree::new();
        self.selection.clear();
        self.action_stack.clear();
    }
}

// ============================================================================
// EditorApp - 编辑器主结构体
// ============================================================================

/// UI 上下文（简化版，实际实现需要完整的 UI 框架）
pub struct UiContext {
    /// 当前时间
    pub dt: f32,
}

impl UiContext {
    /// 创建新的 UI 上下文
    #[inline]
    pub fn new(dt: f32) -> Self {
        Self { dt }
    }
}

/// 编辑器主结构体
///
/// 持有引擎、场景、选择集、撤销栈等核心组件。
pub struct EditorApp {
    /// 编辑器状态
    state: RwLock<EditorState>,
    /// 编辑器模式
    mode: EditorMode,
    /// 当前工具
    tool: EditorTool,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorApp {
    /// 创建新的编辑器
    #[inline]
    pub fn new() -> Self {
        Self {
            state: RwLock::new(EditorState::new()),
            mode: EditorMode::default(),
            tool: EditorTool::default(),
        }
    }

    /// 更新编辑器
    pub fn update(&mut self, dt: f32) {
        self.state.write().tick(dt);
    }

    /// 获取编辑器模式
    #[inline]
    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    /// 设置编辑器模式
    #[inline]
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    /// 获取当前工具
    #[inline]
    pub fn tool(&self) -> EditorTool {
        self.tool
    }

    /// 设置当前工具
    #[inline]
    pub fn set_tool(&mut self, tool: EditorTool) {
        self.tool = tool;
    }

    /// 获取编辑器状态
    #[inline]
    pub fn state(&self) -> &RwLock<EditorState> {
        &self.state
    }

    /// 获取编辑器状态可变引用
    #[inline]
    pub fn state_mut(&mut self) -> &mut RwLock<EditorState> {
        &mut self.state
    }

    /// 执行操作
    pub fn execute_action(&mut self, mut action: Box<dyn EditorAction>) {
        let mut state = self.state.write();
        action.apply(&mut state);
        state.action_stack.push(action);
    }

    /// 撤销
    pub fn undo(&mut self) -> bool {
        let mut state = self.state.write();
        if let Some(mut action) = state.action_stack.pop_undo() {
            action.undo(&mut state);
            state.action_stack.push_redo(action);
            true
        } else {
            false
        }
    }

    /// 重做
    pub fn redo(&mut self) -> bool {
        let mut state = self.state.write();
        if let Some(mut action) = state.action_stack.pop_redo() {
            action.apply(&mut state);
            state.action_stack.push_undo(action);
            true
        } else {
            false
        }
    }

    /// 新建场景
    pub fn new_scene(&mut self) {
        self.state.write().new_scene();
    }

    /// 保存场景
    pub fn save_scene(&self, path: &Path) -> anyhow::Result<()> {
        let state = self.state.read();
        engine_scene::SceneLoader::save_json(&state.scene, path)
    }

    /// 加载场景
    pub fn load_scene(&mut self, path: &Path) -> anyhow::Result<()> {
        let scene = engine_scene::SceneLoader::load_json(path)?;
        let mut state = self.state.write();
        state.scene = scene;
        state.selection.clear();
        state.action_stack.clear();
        Ok(())
    }

    /// 保存设置
    pub fn save_settings(&self, path: &Path) -> anyhow::Result<()> {
        self.state.read().settings.save(path)
    }

    /// 加载设置
    pub fn load_settings(&mut self, path: &Path) -> anyhow::Result<()> {
        let settings = EditorSettings::load(path)?;
        self.state.write().settings = settings;
        Ok(())
    }
}

// ============================================================================
// Panel Trait - 面板接口
// ============================================================================

/// 面板 trait
///
/// 所有编辑器面板必须实现此 trait。
pub trait Panel: 'static {
    /// 获取面板标题
    fn title(&self) -> &str;

    /// 获取面板 ID
    fn id(&self) -> PanelId;

    /// 绘制面板 UI
    fn ui(&mut self, editor: &mut EditorState, ui: &mut UiContext);
}

// ============================================================================
// HierarchyPanel - 层级面板
// ============================================================================

/// 层级面板
///
/// 显示场景树视图，支持选中、拖放、重命名等操作。
#[derive(Debug, Default)]
pub struct HierarchyPanel {
    /// 搜索查询
    search_query: String,
    /// 展开的节点
    expanded_nodes: HashSet<NodeHandle>,
}

impl HierarchyPanel {
    /// 创建新的层级面板
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 展开/折叠节点
    #[inline]
    pub fn toggle_expanded(&mut self, node: NodeHandle) {
        if self.expanded_nodes.contains(&node) {
            self.expanded_nodes.remove(&node);
        } else {
            self.expanded_nodes.insert(node);
        }
    }

    /// 检查节点是否展开
    #[inline]
    pub fn is_expanded(&self, node: NodeHandle) -> bool {
        self.expanded_nodes.contains(&node)
    }

    /// 设置搜索查询
    #[inline]
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    /// 获取搜索查询
    #[inline]
    pub fn search_query(&self) -> &str {
        &self.search_query
    }
}

impl Panel for HierarchyPanel {
    fn title(&self) -> &str {
        "Hierarchy"
    }

    fn id(&self) -> PanelId {
        "hierarchy"
    }

    fn ui(&mut self, editor: &mut EditorState, _ui: &mut UiContext) {
        // 简化实现，实际需要完整的 UI 渲染逻辑
        let root = editor.scene.root();
        self.render_node(editor, root, 0);
    }
}

impl HierarchyPanel {
    /// 递归渲染节点
    fn render_node(&self, editor: &mut EditorState, node: NodeHandle, depth: usize) {
        if node.is_null() {
            return;
        }

        // 检查搜索过滤
        if let Some(node_ref) = editor.scene.get_node(node) {
            if !self.search_query.is_empty() && !node_ref.name().contains(&self.search_query) {
                return;
            }
        }

        // 渲染节点（简化版）
        let _ = depth; // 用于缩进显示

        // 渲染子节点
        if self.is_expanded(node) {
            if let Some(node_ref) = editor.scene.get_node(node) {
                let children: Vec<NodeHandle> = node_ref.children().to_vec();
                for child in children {
                    self.render_node(editor, child, depth + 1);
                }
            }
        }
    }
}

// ============================================================================
// InspectorPanel - 检视面板
// ============================================================================

/// 检视面板
///
/// 显示选中实体的组件列表，支持属性编辑。
#[derive(Debug, Default)]
pub struct InspectorPanel {
    /// 展开的组件
    expanded_components: HashSet<String>,
}

impl InspectorPanel {
    /// 创建新的检视面板
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 展开/折叠组件
    #[inline]
    pub fn toggle_component(&mut self, component_name: &str) {
        let name = component_name.to_string();
        if self.expanded_components.contains(&name) {
            self.expanded_components.remove(&name);
        } else {
            self.expanded_components.insert(name);
        }
    }

    /// 检查组件是否展开
    #[inline]
    pub fn is_component_expanded(&self, component_name: &str) -> bool {
        self.expanded_components.contains(component_name)
    }
}

impl Panel for InspectorPanel {
    fn title(&self) -> &str {
        "Inspector"
    }

    fn id(&self) -> PanelId {
        "inspector"
    }

    fn ui(&mut self, editor: &mut EditorState, _ui: &mut UiContext) {
        // 简化实现，实际需要完整的属性编辑 UI
        if editor.selection.is_empty() {
            return;
        }

        // 显示选中实体的属性
        for entity in editor.selection.iter() {
            let _ = entity; // 实际需要获取实体组件并渲染属性编辑器
        }
    }
}

// ============================================================================
// AssetPanel - 资源面板
// ============================================================================

/// 资源面板
///
/// 显示项目资源文件树和网格视图。
#[derive(Debug, Default)]
pub struct AssetPanel {
    /// 当前选中路径
    selected_path: Option<PathBuf>,
    /// 搜索查询
    search_query: String,
}

impl AssetPanel {
    /// 创建新的资源面板
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置选中路径
    #[inline]
    pub fn set_selected_path(&mut self, path: Option<PathBuf>) {
        self.selected_path = path;
    }

    /// 获取选中路径
    #[inline]
    pub fn selected_path(&self) -> Option<&PathBuf> {
        self.selected_path.as_ref()
    }

    /// 设置搜索查询
    #[inline]
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    /// 获取搜索查询
    #[inline]
    pub fn search_query(&self) -> &str {
        &self.search_query
    }
}

impl Panel for AssetPanel {
    fn title(&self) -> &str {
        "Assets"
    }

    fn id(&self) -> PanelId {
        "assets"
    }

    fn ui(&mut self, editor: &mut EditorState, _ui: &mut UiContext) {
        // 简化实现，实际需要完整的文件树和网格视图渲染
        let asset_dir = &editor.settings.default_create_path;
        let _ = asset_dir; // 实际需要扫描目录并渲染文件树
    }
}

// ============================================================================
// ConsolePanel - 控制台面板
// ============================================================================

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LogLevel {
    /// 调试
    Debug,
    /// 信息
    #[default]
    Info,
    /// 警告
    Warn,
    /// 错误
    Error,
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// 日志级别
    pub level: LogLevel,
    /// 日志消息
    pub message: String,
    /// 文件路径
    pub file: Option<PathBuf>,
    /// 行号
    pub line: Option<u32>,
}

impl LogEntry {
    /// 创建新的日志条目
    #[inline]
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            level,
            message,
            file: None,
            line: None,
        }
    }

    /// 创建带位置的日志条目
    #[inline]
    pub fn with_location(level: LogLevel, message: String, file: PathBuf, line: u32) -> Self {
        Self {
            level,
            message,
            file: Some(file),
            line: Some(line),
        }
    }
}

/// 控制台面板
///
/// 显示日志信息，支持级别过滤和搜索。
#[derive(Debug, Default)]
pub struct ConsolePanel {
    /// 日志条目列表
    entries: Vec<LogEntry>,
    /// 过滤级别
    filter_level: LogLevel,
    /// 过滤文本
    filter_text: String,
}

impl ConsolePanel {
    /// 创建新的控制台面板
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加日志条目
    #[inline]
    pub fn add_entry(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }

    /// 清空日志
    #[inline]
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// 设置过滤级别
    #[inline]
    pub fn set_filter_level(&mut self, level: LogLevel) {
        self.filter_level = level;
    }

    /// 设置过滤文本
    #[inline]
    pub fn set_filter_text(&mut self, text: String) {
        self.filter_text = text;
    }

    /// 获取过滤后的日志条目
    pub fn filtered_entries(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter().filter(|entry| {
            // 级别过滤：只显示高于或等于过滤级别的日志
            let level_match = entry.level >= self.filter_level;
            // 文本过滤
            let text_match =
                self.filter_text.is_empty() || entry.message.contains(&self.filter_text);
            level_match && text_match
        })
    }

    /// 获取日志数量
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 检查日志是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Panel for ConsolePanel {
    fn title(&self) -> &str {
        "Console"
    }

    fn id(&self) -> PanelId {
        "console"
    }

    fn ui(&mut self, _editor: &mut EditorState, _ui: &mut UiContext) {
        // 简化实现，实际需要完整的日志列表渲染
        for entry in self.filtered_entries() {
            let _ = entry; // 实际需要渲染日志行
        }
    }
}

// ============================================================================
// DebugPanel - 调试面板
// ============================================================================

/// 调试面板
///
/// 显示性能统计和 ECS 信息。
#[derive(Debug, Default)]
pub struct DebugPanel {
    /// FPS 历史
    fps_history: Vec<f32>,
    /// 帧时间历史
    frame_time_history: Vec<f32>,
}

impl DebugPanel {
    /// 创建新的调试面板
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录帧数据
    #[inline]
    pub fn record_frame(&mut self, fps: f32, frame_time: f32) {
        self.fps_history.push(fps);
        self.frame_time_history.push(frame_time);

        // 保持历史记录在合理范围内
        const MAX_HISTORY: usize = 100;
        if self.fps_history.len() > MAX_HISTORY {
            self.fps_history.remove(0);
        }
        if self.frame_time_history.len() > MAX_HISTORY {
            self.frame_time_history.remove(0);
        }
    }

    /// 获取最新 FPS
    #[inline]
    pub fn current_fps(&self) -> Option<f32> {
        self.fps_history.last().copied()
    }

    /// 获取最新帧时间
    #[inline]
    pub fn current_frame_time(&self) -> Option<f32> {
        self.frame_time_history.last().copied()
    }

    /// 获取平均 FPS
    pub fn average_fps(&self) -> Option<f32> {
        if self.fps_history.is_empty() {
            return None;
        }
        let sum = self.fps_history.iter().sum::<f32>();
        Some(sum / self.fps_history.len() as f32)
    }

    /// 获取 FPS 历史
    #[inline]
    pub fn fps_history(&self) -> &[f32] {
        &self.fps_history
    }

    /// 获取帧时间历史
    #[inline]
    pub fn frame_time_history(&self) -> &[f32] {
        &self.frame_time_history
    }
}

impl Panel for DebugPanel {
    fn title(&self) -> &str {
        "Debug"
    }

    fn id(&self) -> PanelId {
        "debug"
    }

    fn ui(&mut self, editor: &mut EditorState, _ui: &mut UiContext) {
        // 简化实现，实际需要完整的统计信息渲染
        let node_count = editor.scene.node_count();
        let selection_count = editor.selection.len();
        let _ = node_count;
        let _ = selection_count;
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 将 NodeHandle 转换为 Entity（简化实现）
fn node_to_entity(node: NodeHandle) -> Entity {
    Entity::new(node.index(), 0)
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_selection_basic() {
        let mut selection = EditorSelection::new();
        assert!(selection.is_empty());
        assert_eq!(selection.len(), 0);

        let entity = Entity::new(1, 0);
        selection.select(entity);
        assert!(!selection.is_empty());
        assert_eq!(selection.len(), 1);
        assert!(selection.contains(entity));

        selection.clear();
        assert!(selection.is_empty());
    }

    #[test]
    fn test_editor_selection_toggle() {
        let mut selection = EditorSelection::new();
        let entity = Entity::new(1, 0);

        selection.toggle(entity);
        assert!(selection.contains(entity));

        selection.toggle(entity);
        assert!(!selection.contains(entity));
    }

    #[test]
    fn test_editor_selection_multi() {
        let mut selection = EditorSelection::new();
        let e1 = Entity::new(1, 0);
        let e2 = Entity::new(2, 0);
        let e3 = Entity::new(3, 0);

        selection.add(e1);
        selection.add(e2);
        selection.add(e3);

        assert_eq!(selection.len(), 3);
        assert!(selection.contains(e1));
        assert!(selection.contains(e2));
        assert!(selection.contains(e3));

        selection.remove(e2);
        assert_eq!(selection.len(), 2);
        assert!(!selection.contains(e2));
    }

    #[test]
    fn test_editor_settings_default() {
        let settings = EditorSettings::default();
        assert_eq!(settings.theme, EditorTheme::Dark);
        assert_eq!(settings.font_size, DEFAULT_FONT_SIZE);
        assert!(!settings.auto_save);
    }

    #[test]
    fn test_editor_settings_theme() {
        let mut settings = EditorSettings::new();
        assert_eq!(settings.theme(), EditorTheme::Dark);

        settings.set_theme(EditorTheme::Light);
        assert_eq!(settings.theme(), EditorTheme::Light);
    }

    #[test]
    fn test_editor_action_stack_basic() {
        let stack = EditorActionStack::new(10);
        assert!(stack.is_empty());
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
        assert_eq!(stack.max_len(), 10);
    }

    #[test]
    fn test_editor_action_stack_push() {
        let mut stack = EditorActionStack::new(10);
        let action = Box::new(CreateNodeAction::new(None, "test".to_string()));

        stack.push(action);
        assert!(!stack.is_empty());
        assert!(stack.can_undo());
        assert!(!stack.can_redo());
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_editor_action_stack_undo_redo() {
        let mut state = EditorState::new();
        let mut stack = EditorActionStack::new(10);

        // 创建并执行操作
        let action = Box::new(CreateNodeAction::new(None, "test".to_string()));
        stack.push(action);

        // 撤销
        if let Some(mut action) = stack.pop_undo() {
            action.undo(&mut state);
            stack.push_redo(action);
        }
        assert!(!stack.can_undo());
        assert!(stack.can_redo());

        // 重做
        if let Some(mut action) = stack.pop_redo() {
            action.apply(&mut state);
            stack.push_undo(action);
        }
        assert!(stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_editor_action_stack_clear() {
        let mut stack = EditorActionStack::new(10);
        stack.push(Box::new(CreateNodeAction::new(None, "test".to_string())));

        stack.clear();
        assert!(stack.is_empty());
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_editor_action_stack_max_len() {
        let mut stack = EditorActionStack::new(3);

        // 推入超过最大长度的操作
        for i in 0..5 {
            stack.push(Box::new(CreateNodeAction::new(None, format!("test{}", i))));
        }

        // 应该只保留最后 3 个
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_editor_clipboard() {
        let mut clipboard = EditorClipboard::new();
        assert!(clipboard.is_empty());

        clipboard.copy_nodes(&[NodeHandle::new(1), NodeHandle::new(2)]);
        assert!(!clipboard.is_empty());
        assert_eq!(clipboard.nodes().len(), 2);

        clipboard.clear();
        assert!(clipboard.is_empty());
    }

    #[test]
    fn test_editor_mode() {
        let mut app = EditorApp::new();
        assert_eq!(app.mode(), EditorMode::Edit);

        app.set_mode(EditorMode::Play);
        assert_eq!(app.mode(), EditorMode::Play);

        app.set_mode(EditorMode::Paused);
        assert_eq!(app.mode(), EditorMode::Paused);
    }

    #[test]
    fn test_editor_tool() {
        let mut app = EditorApp::new();
        assert_eq!(app.tool(), EditorTool::Select);

        app.set_tool(EditorTool::Move);
        assert_eq!(app.tool(), EditorTool::Move);

        app.set_tool(EditorTool::Rotate);
        assert_eq!(app.tool(), EditorTool::Rotate);

        app.set_tool(EditorTool::Scale);
        assert_eq!(app.tool(), EditorTool::Scale);
    }

    #[test]
    fn test_editor_state_new_scene() {
        let mut state = EditorState::new();
        let initial_count = state.scene.node_count();

        // 添加节点
        let _node = state.scene.add_2d_node(state.scene.root(), "test");
        assert!(state.scene.node_count() > initial_count);

        // 新建场景
        state.new_scene();
        assert_eq!(state.scene.node_count(), 1); // 只有根节点
        assert!(state.selection.is_empty());
        assert!(state.action_stack.is_empty());
    }

    #[test]
    fn test_editor_state_panel_visibility() {
        let mut state = EditorState::new();

        assert!(state.is_panel_visible("hierarchy"));
        assert!(state.is_panel_visible("inspector"));

        state.hide_panel("hierarchy");
        assert!(!state.is_panel_visible("hierarchy"));

        state.toggle_panel("hierarchy");
        assert!(state.is_panel_visible("hierarchy"));

        state.reset_layout();
        assert!(state.is_panel_visible("hierarchy"));
        assert!(state.is_panel_visible("inspector"));
    }

    #[test]
    fn test_hierarchy_panel() {
        let mut panel = HierarchyPanel::new();
        let node = NodeHandle::new(1);

        assert!(!panel.is_expanded(node));
        panel.toggle_expanded(node);
        assert!(panel.is_expanded(node));
        panel.toggle_expanded(node);
        assert!(!panel.is_expanded(node));

        panel.set_search_query("test".to_string());
        assert_eq!(panel.search_query(), "test");
    }

    #[test]
    fn test_console_panel() {
        let mut panel = ConsolePanel::new();
        assert!(panel.is_empty());

        panel.add_entry(LogEntry::new(LogLevel::Info, "test message".to_string()));
        assert_eq!(panel.len(), 1);

        panel.set_filter_level(LogLevel::Warn);
        // Info 级别低于 Warn，应该被过滤
        assert_eq!(panel.filtered_entries().count(), 0);

        panel.add_entry(LogEntry::new(LogLevel::Error, "error message".to_string()));
        assert_eq!(panel.filtered_entries().count(), 1);

        panel.clear();
        assert!(panel.is_empty());
    }

    #[test]
    fn test_debug_panel() {
        let mut panel = DebugPanel::new();
        assert!(panel.current_fps().is_none());
        assert!(panel.average_fps().is_none());

        panel.record_frame(60.0, 16.67);
        assert_eq!(panel.current_fps(), Some(60.0));
        assert_eq!(panel.current_frame_time(), Some(16.67));

        panel.record_frame(30.0, 33.33);
        assert_eq!(panel.average_fps(), Some(45.0));
    }

    #[test]
    fn test_batch_action() {
        let mut state = EditorState::new();
        let mut batch = BatchAction::new(vec![
            Box::new(CreateNodeAction::new(None, "node1".to_string())),
            Box::new(CreateNodeAction::new(None, "node2".to_string())),
        ]);

        batch.apply(&mut state);
        assert!(state.scene.node_count() > 1);

        batch.undo(&mut state);
        // 撤销后节点应该被删除
    }

    #[test]
    fn test_key_bindings() {
        let mut bindings = KeyBindings::new();

        bindings.set("save", vec!["Ctrl+S".to_string()]);
        assert_eq!(bindings.get("save"), Some(&["Ctrl+S".to_string()][..]));

        bindings.remove("save");
        assert!(bindings.get("save").is_none());
    }
}
