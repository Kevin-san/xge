//! 输入模块
//!
//! 处理 UI 输入事件。

use engine_ecs::{Component, Entity, Event, Events, World};
use engine_math::Vec2;
use engine_window::{KeyCode, MouseButton};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UiEventType {
    MouseEnter,
    MouseLeave,
    MouseDown,
    MouseUp,
    Click,
    DoubleClick,
    MouseMove,
    MouseWheel,
    KeyDown,
    KeyUp,
    TextInput,
    FocusIn,
    FocusOut,
}

#[derive(Clone)]
pub struct UiEvent {
    event_type: UiEventType,
    target: Entity,
    mouse_position: Vec2,
    key_code: Option<KeyCode>,
    text: String,
    button: Option<MouseButton>,
    delta: Vec2,
}

// SAFETY: UiEvent only contains Send + Sync fields
unsafe impl Send for UiEvent {}
unsafe impl Sync for UiEvent {}

impl UiEvent {
    pub fn new(event_type: UiEventType, target: Entity) -> Self {
        Self {
            event_type,
            target,
            mouse_position: Vec2::ZERO,
            key_code: None,
            text: String::new(),
            button: None,
            delta: Vec2::ZERO,
        }
    }

    pub fn event_type(&self) -> UiEventType {
        self.event_type
    }

    pub fn target(&self) -> Entity {
        self.target
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    pub fn key_code(&self) -> Option<KeyCode> {
        self.key_code
    }

    pub fn set_key_code(&mut self, key_code: KeyCode) {
        self.key_code = Some(key_code);
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn button(&self) -> Option<MouseButton> {
        self.button
    }

    pub fn set_button(&mut self, button: MouseButton) {
        self.button = Some(button);
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    pub fn set_delta(&mut self, delta: Vec2) {
        self.delta = delta;
    }

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
        )
    }

    pub fn is_keyboard_event(&self) -> bool {
        matches!(
            self.event_type,
            UiEventType::KeyDown | UiEventType::KeyUp | UiEventType::TextInput
        )
    }
}

impl Event for UiEvent {}

pub struct UiInput {
    mouse_position: Vec2,
    last_click_time: f64,
    double_click_threshold: f64,
    hovered_entity: Option<Entity>,
    focused_entity: Option<Entity>,
}

impl UiInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    pub fn hovered_entity(&self) -> Option<Entity> {
        self.hovered_entity
    }

    pub fn set_hovered_entity(&mut self, entity: Option<Entity>) {
        self.hovered_entity = entity;
    }

    pub fn focused_entity(&self) -> Option<Entity> {
        self.focused_entity
    }

    pub fn set_focused_entity(&mut self, entity: Option<Entity>) {
        self.focused_entity = entity;
    }

    pub fn process_mouse_move(
        &mut self,
        world: &World,
        position: Vec2,
        root_entity: Entity,
        events: &mut Events<UiEvent>,
    ) {
        self.mouse_position = position;

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

    pub fn process_mouse_down(
        &mut self,
        world: &World,
        position: Vec2,
        button: MouseButton,
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
            let mut event = UiEvent::new(UiEventType::MouseDown, found);
            event.set_mouse_position(position);
            event.set_button(button);
            events.send(event);

            self.set_focused_entity(Some(found));
        }
    }

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

    pub fn process_key_down(&mut self, key_code: KeyCode, events: &mut Events<UiEvent>) {
        if let Some(focused) = self.focused_entity {
            let mut event = UiEvent::new(UiEventType::KeyDown, focused);
            event.set_key_code(key_code);
            events.send(event);
        }
    }

    pub fn process_key_up(&mut self, key_code: KeyCode, events: &mut Events<UiEvent>) {
        if let Some(focused) = self.focused_entity {
            let mut event = UiEvent::new(UiEventType::KeyUp, focused);
            event.set_key_code(key_code);
            events.send(event);
        }
    }

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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_ecs::World;

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
}
