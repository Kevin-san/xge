//! 输入模块
//!
//! 处理 UI 输入事件，包含事件总线、拖拽、滚轮、焦点管理与事件冒泡逻辑。

use std::collections::HashMap;

use engine_ecs::{Component, Entity, Event, Events, World};
use engine_math::Vec2;
use engine_window::{KeyCode, MouseButton};

/// UI事件类型
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum UiEventType {
    /// 鼠标进入
    MouseEnter,
    /// 鼠标离开
    MouseLeave,
    /// 鼠标按下
    MouseDown,
    /// 鼠标释放
    MouseUp,
    /// 单击
    Click,
    /// 双击
    DoubleClick,
    /// 鼠标移动
    MouseMove,
    /// 鼠标滚轮
    MouseWheel,
    /// 键盘按下
    KeyDown,
    /// 键盘释放
    KeyUp,
    /// 文本输入
    TextInput,
    /// 获得焦点
    FocusIn,
    /// 失去焦点
    FocusOut,
    /// 拖拽开始
    DragStart,
    /// 拖拽中
    DragMove,
    /// 拖拽结束
    DragEnd,
}

/// UI事件
///
/// 包含事件类型、目标实体、鼠标位置、键盘输入等信息。
#[derive(Clone)]
pub struct UiEvent {
    event_type: UiEventType,
    target: Entity,
    mouse_position: Vec2,
    key_code: Option<KeyCode>,
    text: String,
    button: Option<MouseButton>,
    delta: Vec2,
    /// 是否冒泡（默认 true）
    bubbles: bool,
    /// 是否已处理（用于 stop_propagation）
    handled: bool,
    /// 拖拽增量（DragMove 事件专用）
    drag_delta: Vec2,
}

impl UiEvent {
    /// 创建新的UI事件
    pub fn new(event_type: UiEventType, target: Entity) -> Self {
        Self {
            event_type,
            target,
            mouse_position: Vec2::ZERO,
            key_code: None,
            text: String::new(),
            button: None,
            delta: Vec2::ZERO,
            bubbles: true,
            handled: false,
            drag_delta: Vec2::ZERO,
        }
    }

    /// 创建不冒泡的事件
    pub fn new_no_bubble(event_type: UiEventType, target: Entity) -> Self {
        let mut event = Self::new(event_type, target);
        event.bubbles = false;
        event
    }

    /// 获取事件类型
    pub fn event_type(&self) -> UiEventType {
        self.event_type
    }

    /// 获取目标实体
    pub fn target(&self) -> Entity {
        self.target
    }

    /// 获取鼠标位置
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// 设置鼠标位置
    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    /// 获取键盘码
    pub fn key_code(&self) -> Option<KeyCode> {
        self.key_code
    }

    /// 设置键盘码
    pub fn set_key_code(&mut self, key_code: KeyCode) {
        self.key_code = Some(key_code);
    }

    /// 获取文本输入
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 设置文本输入
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// 获取鼠标按钮
    pub fn button(&self) -> Option<MouseButton> {
        self.button
    }

    /// 设置鼠标按钮
    pub fn set_button(&mut self, button: MouseButton) {
        self.button = Some(button);
    }

    /// 获取滚动增量
    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    /// 设置滚动增量
    pub fn set_delta(&mut self, delta: Vec2) {
        self.delta = delta;
    }

    /// 获取拖拽增量
    pub fn drag_delta(&self) -> Vec2 {
        self.drag_delta
    }

    /// 设置拖拽增量
    pub fn set_drag_delta(&mut self, delta: Vec2) {
        self.drag_delta = delta;
    }

    /// 是否冒泡
    pub fn bubbles(&self) -> bool {
        self.bubbles
    }

    /// 设置是否冒泡
    pub fn set_bubbles(&mut self, bubbles: bool) {
        self.bubbles = bubbles;
    }

    /// 是否已处理
    pub fn is_handled(&self) -> bool {
        self.handled
    }

    /// 标记为已处理（停止冒泡）
    pub fn stop_propagation(&mut self) {
        self.handled = true;
    }

    /// 检查是否为鼠标事件
    pub fn is_mouse_event(&self) -> bool {
        matches!(
            self.event_type,
            UiEventType::MouseEnter
                | UiEventType::MouseLeave
                | UiEventType::MouseDown
                | UiEventType::MouseUp
                | UiEventType::Click
                | UiEventType::DoubleClick
                | UiEventType::MouseMove
                | UiEventType::MouseWheel
                | UiEventType::DragStart
                | UiEventType::DragMove
                | UiEventType::DragEnd
        )
    }

    /// 检查是否为键盘事件
    pub fn is_keyboard_event(&self) -> bool {
        matches!(
            self.event_type,
            UiEventType::KeyDown | UiEventType::KeyUp | UiEventType::TextInput
        )
    }

    /// 检查是否为焦点事件
    pub fn is_focus_event(&self) -> bool {
        matches!(self.event_type, UiEventType::FocusIn | UiEventType::FocusOut)
    }

    /// 检查是否为拖拽事件
    pub fn is_drag_event(&self) -> bool {
        matches!(
            self.event_type,
            UiEventType::DragStart | UiEventType::DragMove | UiEventType::DragEnd
        )
    }
}

impl Event for UiEvent {}

/// 事件监听器回调类型
pub type EventListener = Box<dyn Fn(&UiEvent) + Send + Sync>;

/// 事件订阅句柄
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SubscriptionId(u64);

/// UI 事件总线
///
/// 提供事件派发、监听、冒泡能力。
/// 支持按事件类型注册监听器，事件派发时按目标→父级→祖父级冒泡。
pub struct UiEventBus {
    listeners: HashMap<UiEventType, Vec<(SubscriptionId, EventListener)>>,
    next_id: u64,
}

impl UiEventBus {
    /// 创建新的事件总线
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            next_id: 1,
        }
    }

    /// 注册事件监听器
    pub fn subscribe<F>(&mut self, event_type: UiEventType, callback: F) -> SubscriptionId
    where
        F: Fn(&UiEvent) + Send + Sync + 'static,
    {
        let id = SubscriptionId(self.next_id);
        self.next_id += 1;
        self.listeners
            .entry(event_type)
            .or_default()
            .push((id, Box::new(callback)));
        id
    }

    /// 取消事件监听
    pub fn unsubscribe(&mut self, event_type: UiEventType, id: SubscriptionId) -> bool {
        if let Some(list) = self.listeners.get_mut(&event_type) {
            let before = list.len();
            list.retain(|(sid, _)| *sid != id);
            return list.len() != before;
        }
        false
    }

    /// 派发事件给指定类型的所有监听器
    pub fn dispatch(&self, event: &UiEvent) {
        if let Some(list) = self.listeners.get(&event.event_type) {
            for (_, callback) in list {
                callback(event);
            }
        }
    }

    /// 派发事件并执行冒泡（从 target 向上遍历父级）
    ///
    /// `parent_lookup`：返回实体父级的函数
    ///
    /// 注意：`stop_propagation()` 仅阻止向父级冒泡，目标自身的监听器仍会触发。
    pub fn dispatch_with_bubble<F>(&self, event: &UiEvent, mut parent_lookup: F)
    where
        F: FnMut(Entity) -> Option<Entity>,
    {
        // 始终派发给目标自身的监听器
        self.dispatch(event);

        // 若事件已标记处理或不可冒泡，则停止向上传播
        if event.handled || !event.bubbles {
            return;
        }

        let mut current = parent_lookup(event.target);
        while let Some(parent) = current {
            if event.handled {
                break;
            }
            // 创建针对父级的事件副本
            let mut parent_event = event.clone();
            parent_event.target = parent;
            self.dispatch(&parent_event);
            current = parent_lookup(parent);
        }
    }

    /// 获取指定类型的监听器数量
    pub fn listener_count(&self, event_type: UiEventType) -> usize {
        self.listeners
            .get(&event_type)
            .map(|v: &Vec<(SubscriptionId, EventListener)>| v.len())
            .unwrap_or(0)
    }

    /// 清空所有监听器
    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}

impl Default for UiEventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// 拖拽状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DragState {
    /// 空闲
    Idle,
    /// 按下等待拖拽（未达到阈值）
    Pending,
    /// 正在拖拽
    Dragging,
}

/// 拖拽配置
#[derive(Clone, Copy, Debug)]
pub struct DragConfig {
    /// 拖拽触发阈值（像素）
    pub threshold: f32,
    /// 触发拖拽的鼠标按钮
    pub button: MouseButton,
}

impl Default for DragConfig {
    fn default() -> Self {
        Self {
            threshold: 4.0,
            button: MouseButton::Left,
        }
    }
}

/// UI输入处理器
///
/// 管理鼠标位置、悬停实体、焦点实体、拖拽状态，处理输入事件。
pub struct UiInput {
    mouse_position: Vec2,
    last_click_time: f64,
    double_click_threshold: f64,
    hovered_entity: Option<Entity>,
    focused_entity: Option<Entity>,
    /// 拖拽配置
    drag_config: DragConfig,
    /// 拖拽状态
    drag_state: DragState,
    /// 拖拽起始位置
    drag_start_pos: Vec2,
    /// 拖拽目标实体
    drag_target: Option<Entity>,
    /// 拖拽最后位置
    drag_last_pos: Vec2,
    /// 鼠标按下状态
    mouse_down: bool,
    /// 鼠标按下时的实体
    mouse_down_entity: Option<Entity>,
}

impl UiInput {
    /// 创建新的UI输入处理器
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取鼠标位置
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// 设置鼠标位置
    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    /// 获取当前悬停的实体
    pub fn hovered_entity(&self) -> Option<Entity> {
        self.hovered_entity
    }

    /// 设置当前悬停的实体
    pub fn set_hovered_entity(&mut self, entity: Option<Entity>) {
        self.hovered_entity = entity;
    }

    /// 获取当前焦点的实体
    pub fn focused_entity(&self) -> Option<Entity> {
        self.focused_entity
    }

    /// 设置当前焦点的实体，并派发 FocusIn/FocusOut 事件
    pub fn set_focused_entity(&mut self, entity: Option<Entity>, events: &mut Events<UiEvent>) {
        if self.focused_entity == entity {
            return;
        }
        // 失去焦点
        if let Some(old) = self.focused_entity {
            let event = UiEvent::new(UiEventType::FocusOut, old);
            events.send(event);
        }
        // 获得焦点
        if let Some(new) = entity {
            let event = UiEvent::new(UiEventType::FocusIn, new);
            events.send(event);
        }
        self.focused_entity = entity;
    }

    /// 直接设置焦点实体（不派发事件，用于内部同步）
    pub fn set_focused_entity_silent(&mut self, entity: Option<Entity>) {
        self.focused_entity = entity;
    }

    /// 获取拖拽配置
    pub fn drag_config(&self) -> DragConfig {
        self.drag_config
    }

    /// 设置拖拽配置
    pub fn set_drag_config(&mut self, config: DragConfig) {
        self.drag_config = config;
    }

    /// 是否正在拖拽
    pub fn is_dragging(&self) -> bool {
        self.drag_state == DragState::Dragging
    }

    /// 获取拖拽目标
    pub fn drag_target(&self) -> Option<Entity> {
        self.drag_target
    }

    /// 处理鼠标移动事件
    pub fn process_mouse_move(
        &mut self,
        world: &World,
        position: Vec2,
        root_entity: Entity,
        events: &mut Events<UiEvent>,
    ) {
        self.mouse_position = position;

        // 处理拖拽
        if self.drag_state == DragState::Pending {
            let dist = (position - self.drag_start_pos).length();
            if dist >= self.drag_config.threshold {
                // 触发拖拽开始
                self.drag_state = DragState::Dragging;
                if let Some(target) = self.drag_target {
                    let mut event = UiEvent::new(UiEventType::DragStart, target);
                    event.set_mouse_position(self.drag_start_pos);
                    event.set_button(self.drag_config.button);
                    events.send(event);
                }
            }
        }

        if self.drag_state == DragState::Dragging {
            if let Some(target) = self.drag_target {
                let delta = position - self.drag_last_pos;
                let mut event = UiEvent::new(UiEventType::DragMove, target);
                event.set_mouse_position(position);
                event.set_drag_delta(delta);
                event.set_button(self.drag_config.button);
                events.send(event);
                self.drag_last_pos = position;
            }
            return; // 拖拽时不处理悬停
        }

        let root = world.get_component::<crate::UiRoot>(root_entity);
        if root.is_none() {
            return;
        }
        let root = root.unwrap();

        let found = root.find_node_at_position(world, position);

        if found != self.hovered_entity {
            if let Some(old_hovered) = self.hovered_entity {
                let mut event = UiEvent::new(UiEventType::MouseLeave, old_hovered);
                event.set_mouse_position(position);
                events.send(event);
            }

            if let Some(new_hovered) = found {
                let mut event = UiEvent::new(UiEventType::MouseEnter, new_hovered);
                event.set_mouse_position(position);
                events.send(event);
            }

            self.hovered_entity = found;
        }

        if let Some(hovered) = self.hovered_entity {
            let mut event = UiEvent::new(UiEventType::MouseMove, hovered);
            event.set_mouse_position(position);
            events.send(event);
        }
    }

    /// 处理鼠标按下事件
    pub fn process_mouse_down(
        &mut self,
        world: &World,
        position: Vec2,
        button: MouseButton,
        root_entity: Entity,
        events: &mut Events<UiEvent>,
    ) {
        self.mouse_position = position;
        self.mouse_down = true;

        let root = world.get_component::<crate::UiRoot>(root_entity);
        if root.is_none() {
            return;
        }
        let root = root.unwrap();

        if let Some(found) = root.find_node_at_position(world, position) {
            let mut event = UiEvent::new(UiEventType::MouseDown, found);
            event.set_mouse_position(position);
            event.set_button(button);
            events.send(event);

            self.set_focused_entity(Some(found), events);
            self.mouse_down_entity = Some(found);

            // 准备拖拽
            if button == self.drag_config.button {
                self.drag_state = DragState::Pending;
                self.drag_start_pos = position;
                self.drag_last_pos = position;
                self.drag_target = Some(found);
            }
        } else {
            // 点击空白处，清除焦点
            self.set_focused_entity(None, events);
            self.mouse_down_entity = None;
        }
    }

    /// 处理鼠标释放事件
    pub fn process_mouse_up(
        &mut self,
        world: &World,
        position: Vec2,
        button: MouseButton,
        root_entity: Entity,
        events: &mut Events<UiEvent>,
        current_time: f64,
    ) {
        self.mouse_position = position;
        self.mouse_down = false;

        // 处理拖拽结束
        if self.drag_state == DragState::Dragging {
            if let Some(target) = self.drag_target {
                let mut event = UiEvent::new(UiEventType::DragEnd, target);
                event.set_mouse_position(position);
                event.set_button(button);
                events.send(event);
            }
            self.drag_state = DragState::Idle;
            self.drag_target = None;
            self.mouse_down_entity = None;
            return;
        }

        // 取消挂起的拖拽
        if self.drag_state == DragState::Pending {
            self.drag_state = DragState::Idle;
        }

        let root = world.get_component::<crate::UiRoot>(root_entity);
        if root.is_none() {
            return;
        }
        let root = root.unwrap();

        if let Some(found) = root.find_node_at_position(world, position) {
            let mut event = UiEvent::new(UiEventType::MouseUp, found);
            event.set_mouse_position(position);
            event.set_button(button);
            events.send(event);

            // 仅当按下和释放在同一实体上时才触发 Click
            if self.mouse_down_entity == Some(found) {
                let mut click_event = UiEvent::new(UiEventType::Click, found);
                click_event.set_mouse_position(position);
                click_event.set_button(button);
                events.send(click_event);

                if current_time - self.last_click_time < self.double_click_threshold {
                    let mut double_click_event = UiEvent::new(UiEventType::DoubleClick, found);
                    double_click_event.set_mouse_position(position);
                    double_click_event.set_button(button);
                    events.send(double_click_event);
                }

                self.last_click_time = current_time;
            }
        }

        self.mouse_down_entity = None;
    }

    /// 处理鼠标滚轮事件
    pub fn process_mouse_wheel(
        &mut self,
        world: &World,
        position: Vec2,
        delta: Vec2,
        root_entity: Entity,
        events: &mut Events<UiEvent>,
    ) {
        self.mouse_position = position;

        let root = world.get_component::<crate::UiRoot>(root_entity);
        if root.is_none() {
            return;
        }
        let root = root.unwrap();

        if let Some(found) = root.find_node_at_position(world, position) {
            let mut event = UiEvent::new(UiEventType::MouseWheel, found);
            event.set_mouse_position(position);
            event.set_delta(delta);
            events.send(event);
        }
    }

    /// 处理键盘按下事件
    pub fn process_key_down(&mut self, key_code: KeyCode, events: &mut Events<UiEvent>) {
        if let Some(focused) = self.focused_entity {
            let mut event = UiEvent::new(UiEventType::KeyDown, focused);
            event.set_key_code(key_code);
            events.send(event);
        }
    }

    /// 处理键盘释放事件
    pub fn process_key_up(&mut self, key_code: KeyCode, events: &mut Events<UiEvent>) {
        if let Some(focused) = self.focused_entity {
            let mut event = UiEvent::new(UiEventType::KeyUp, focused);
            event.set_key_code(key_code);
            events.send(event);
        }
    }

    /// 处理文本输入事件
    pub fn process_text_input(&mut self, text: &str, events: &mut Events<UiEvent>) {
        if let Some(focused) = self.focused_entity {
            let mut event = UiEvent::new(UiEventType::TextInput, focused);
            event.set_text(text);
            events.send(event);
        }
    }
}

impl Component for UiInput {}

impl Default for UiInput {
    fn default() -> Self {
        Self {
            mouse_position: Vec2::ZERO,
            last_click_time: 0.0,
            double_click_threshold: 0.5,
            hovered_entity: None,
            focused_entity: None,
            drag_config: DragConfig::default(),
            drag_state: DragState::Idle,
            drag_start_pos: Vec2::ZERO,
            drag_target: None,
            drag_last_pos: Vec2::ZERO,
            mouse_down: false,
            mouse_down_entity: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_ecs::{Events, World};
    use engine_math::Vec2;

    #[test]
    fn test_ui_event_creation() {
        let entity = Entity::new(0, 0);
        let event = UiEvent::new(UiEventType::Click, entity);

        assert_eq!(event.event_type(), UiEventType::Click);
        assert_eq!(event.target(), entity);
    }

    #[test]
    fn test_ui_event_mouse_event() {
        let entity = Entity::new(0, 0);

        let enter_event = UiEvent::new(UiEventType::MouseEnter, entity);
        assert!(enter_event.is_mouse_event());

        let click_event = UiEvent::new(UiEventType::Click, entity);
        assert!(click_event.is_mouse_event());

        let key_event = UiEvent::new(UiEventType::KeyDown, entity);
        assert!(!key_event.is_mouse_event());
    }

    #[test]
    fn test_ui_event_keyboard_event() {
        let entity = Entity::new(0, 0);

        let key_down_event = UiEvent::new(UiEventType::KeyDown, entity);
        assert!(key_down_event.is_keyboard_event());

        let text_event = UiEvent::new(UiEventType::TextInput, entity);
        assert!(text_event.is_keyboard_event());

        let click_event = UiEvent::new(UiEventType::Click, entity);
        assert!(!click_event.is_keyboard_event());
    }

    #[test]
    fn test_ui_input_new_default() {
        let input = UiInput::new();
        assert_eq!(input.mouse_position(), Vec2::ZERO);
        assert!(input.hovered_entity().is_none());
        assert!(input.focused_entity().is_none());
    }

    #[test]
    fn test_ui_input_set_mouse_position() {
        let mut input = UiInput::new();
        input.set_mouse_position(Vec2::new(100.0, 200.0));
        assert_eq!(input.mouse_position(), Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_ui_input_set_hovered_entity() {
        let mut input = UiInput::new();
        let entity = Entity::new(1, 0);
        input.set_hovered_entity(Some(entity));
        assert_eq!(input.hovered_entity(), Some(entity));
        input.set_hovered_entity(None);
        assert!(input.hovered_entity().is_none());
    }

    #[test]
    fn test_ui_input_set_focused_entity() {
        let mut input = UiInput::new();
        let entity = Entity::new(2, 0);
        input.set_focused_entity_silent(Some(entity));
        assert_eq!(input.focused_entity(), Some(entity));
    }

    #[test]
    fn test_ui_event_set_mouse_position() {
        let entity = Entity::new(0, 0);
        let mut event = UiEvent::new(UiEventType::MouseMove, entity);
        event.set_mouse_position(Vec2::new(50.0, 75.0));
        assert_eq!(event.mouse_position(), Vec2::new(50.0, 75.0));
    }

    #[test]
    fn test_ui_event_set_text() {
        let entity = Entity::new(0, 0);
        let mut event = UiEvent::new(UiEventType::TextInput, entity);
        event.set_text("hello");
        assert_eq!(event.text(), "hello");
    }

    #[test]
    fn test_ui_event_set_delta() {
        let entity = Entity::new(0, 0);
        let mut event = UiEvent::new(UiEventType::MouseWheel, entity);
        event.set_delta(Vec2::new(0.0, 5.0));
        assert_eq!(event.delta(), Vec2::new(0.0, 5.0));
    }

    #[test]
    fn test_ui_input_process_mouse_move_over_root() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, crate::ui_node::UiNode::new(crate::ui_node::UiNodeType::Root));
        world.insert(
            root_entity,
            crate::ui_node::UiRoot::new(root_entity, Vec2::new(800.0, 600.0)),
        );
        let mut events = Events::<UiEvent>::new();
        let mut input = UiInput::new();
        input.process_mouse_move(&world, Vec2::new(100.0, 100.0), root_entity, &mut events);
        assert!(input.hovered_entity().is_some());
    }

    #[test]
    fn test_ui_event_type_focus_in_out() {
        let entity = Entity::new(0, 0);
        let focus_in = UiEvent::new(UiEventType::FocusIn, entity);
        assert_eq!(focus_in.event_type(), UiEventType::FocusIn);
        assert!(focus_in.is_focus_event());
        let focus_out = UiEvent::new(UiEventType::FocusOut, entity);
        assert_eq!(focus_out.event_type(), UiEventType::FocusOut);
        assert!(focus_out.is_focus_event());
    }

    #[test]
    fn test_ui_input_process_key_down_sends_event() {
        let mut events = Events::<UiEvent>::new();
        let mut input = UiInput::new();
        let entity = Entity::new(1, 0);
        input.set_focused_entity_silent(Some(entity));
        input.process_text_input("hello", &mut events);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_ui_input_no_focus_no_key_event() {
        let mut events = Events::<UiEvent>::new();
        let mut input = UiInput::new();
        input.process_text_input("hello", &mut events);
        assert_eq!(events.len(), 0);
    }

    // ===== 事件总线测试 =====

    #[test]
    fn test_event_bus_subscribe_dispatch() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let mut bus = UiEventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        bus.subscribe(UiEventType::Click, move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let entity = Entity::new(0, 0);
        let event = UiEvent::new(UiEventType::Click, entity);
        bus.dispatch(&event);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_bus_unsubscribe() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let mut bus = UiEventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        let id = bus.subscribe(UiEventType::Click, move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let entity = Entity::new(0, 0);
        let event = UiEvent::new(UiEventType::Click, entity);
        bus.dispatch(&event);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        assert!(bus.unsubscribe(UiEventType::Click, id));
        bus.dispatch(&event);
        assert_eq!(counter.load(Ordering::SeqCst), 1); // 不再增加
    }

    #[test]
    fn test_event_bus_listener_count() {
        let mut bus = UiEventBus::new();
        assert_eq!(bus.listener_count(UiEventType::Click), 0);
        let _id1 = bus.subscribe(UiEventType::Click, |_| {});
        let _id2 = bus.subscribe(UiEventType::Click, |_| {});
        assert_eq!(bus.listener_count(UiEventType::Click), 2);
        assert_eq!(bus.listener_count(UiEventType::KeyDown), 0);
    }

    #[test]
    fn test_event_bus_clear() {
        let mut bus = UiEventBus::new();
        let _id1 = bus.subscribe(UiEventType::Click, |_| {});
        let _id2 = bus.subscribe(UiEventType::KeyDown, |_| {});
        bus.clear();
        assert_eq!(bus.listener_count(UiEventType::Click), 0);
        assert_eq!(bus.listener_count(UiEventType::KeyDown), 0);
    }

    #[test]
    fn test_event_bus_bubble() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let mut bus = UiEventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        bus.subscribe(UiEventType::Click, move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // 构建父子链：child -> parent -> grandparent
        let child = Entity::new(1, 0);
        let parent = Entity::new(2, 0);
        let grandparent = Entity::new(3, 0);

        let mut parents = std::collections::HashMap::new();
        parents.insert(child, parent);
        parents.insert(parent, grandparent);

        let event = UiEvent::new(UiEventType::Click, child);
        bus.dispatch_with_bubble(&event, |e| parents.get(&e).copied());

        // 应该触发 3 次（child + parent + grandparent）
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_event_bus_no_bubble() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let mut bus = UiEventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        bus.subscribe(UiEventType::Click, move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let child = Entity::new(1, 0);
        let parent = Entity::new(2, 0);

        let mut parents = std::collections::HashMap::new();
        parents.insert(child, parent);

        let event = UiEvent::new_no_bubble(UiEventType::Click, child);
        bus.dispatch_with_bubble(&event, |e| parents.get(&e).copied());

        // 不冒泡，只触发 1 次
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_stop_propagation() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let mut bus = UiEventBus::new();
        let counter = Arc::new(AtomicU32::new(0));

        // 第一个监听器停止冒泡
        let counter_clone1 = counter.clone();
        bus.subscribe(UiEventType::Click, move |_event| {
            counter_clone1.fetch_add(1, Ordering::SeqCst);
        });

        let child = Entity::new(1, 0);
        let parent = Entity::new(2, 0);

        let mut parents = std::collections::HashMap::new();
        parents.insert(child, parent);

        let mut event = UiEvent::new(UiEventType::Click, child);
        event.stop_propagation();
        bus.dispatch_with_bubble(&event, |e| parents.get(&e).copied());

        // 已处理，不冒泡，但当前 target 仍触发
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    // ===== 拖拽测试 =====

    #[test]
    fn test_drag_config_default() {
        let config = DragConfig::default();
        assert_eq!(config.threshold, 4.0);
        assert_eq!(config.button, MouseButton::Left);
    }

    #[test]
    fn test_ui_input_drag_config_get_set() {
        let mut input = UiInput::new();
        let config = DragConfig {
            threshold: 10.0,
            button: MouseButton::Right,
        };
        input.set_drag_config(config);
        assert_eq!(input.drag_config().threshold, 10.0);
        assert_eq!(input.drag_config().button, MouseButton::Right);
    }

    #[test]
    fn test_ui_input_initial_not_dragging() {
        let input = UiInput::new();
        assert!(!input.is_dragging());
        assert!(input.drag_target().is_none());
    }

    #[test]
    fn test_ui_input_focus_change_emits_events() {
        let mut input = UiInput::new();
        let mut events = Events::<UiEvent>::new();
        let entity = Entity::new(1, 0);

        // 设置焦点应派发 FocusIn
        input.set_focused_entity(Some(entity), &mut events);
        assert_eq!(events.len(), 1);

        // 切换焦点应派发 FocusOut + FocusIn
        let entity2 = Entity::new(2, 0);
        input.set_focused_entity(Some(entity2), &mut events);
        assert_eq!(events.len(), 3);

        // 清除焦点应派发 FocusOut
        input.set_focused_entity(None, &mut events);
        assert_eq!(events.len(), 4);
    }

    #[test]
    fn test_ui_input_focus_same_no_event() {
        let mut input = UiInput::new();
        let mut events = Events::<UiEvent>::new();
        let entity = Entity::new(1, 0);

        input.set_focused_entity(Some(entity), &mut events);
        assert_eq!(events.len(), 1);

        // 相同实体不派发事件
        input.set_focused_entity(Some(entity), &mut events);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_ui_event_drag_properties() {
        let entity = Entity::new(0, 0);
        let mut event = UiEvent::new(UiEventType::DragMove, entity);
        event.set_drag_delta(Vec2::new(10.0, 20.0));
        assert_eq!(event.drag_delta(), Vec2::new(10.0, 20.0));
        assert!(event.is_drag_event());
    }

    #[test]
    fn test_ui_event_bubbles_default_true() {
        let entity = Entity::new(0, 0);
        let event = UiEvent::new(UiEventType::Click, entity);
        assert!(event.bubbles());
    }

    #[test]
    fn test_ui_event_new_no_bubble() {
        let entity = Entity::new(0, 0);
        let event = UiEvent::new_no_bubble(UiEventType::Click, entity);
        assert!(!event.bubbles());
    }

    #[test]
    fn test_ui_event_set_bubbles() {
        let entity = Entity::new(0, 0);
        let mut event = UiEvent::new(UiEventType::Click, entity);
        event.set_bubbles(false);
        assert!(!event.bubbles());
        event.set_bubbles(true);
        assert!(event.bubbles());
    }

    #[test]
    fn test_ui_event_stop_propagation_marks_handled() {
        let entity = Entity::new(0, 0);
        let mut event = UiEvent::new(UiEventType::Click, entity);
        assert!(!event.is_handled());
        event.stop_propagation();
        assert!(event.is_handled());
    }

    #[test]
    fn test_ui_input_process_mouse_wheel() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, crate::ui_node::UiNode::new(crate::ui_node::UiNodeType::Root));
        world.insert(
            root_entity,
            crate::ui_node::UiRoot::new(root_entity, Vec2::new(800.0, 600.0)),
        );
        let mut events = Events::<UiEvent>::new();
        let mut input = UiInput::new();
        input.process_mouse_wheel(
            &world,
            Vec2::new(100.0, 100.0),
            Vec2::new(0.0, 10.0),
            root_entity,
            &mut events,
        );
        assert_eq!(events.len(), 1);
    }

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn test_ui_event_send_sync() {
        assert_send::<UiEvent>();
        assert_sync::<UiEvent>();
    }
}
