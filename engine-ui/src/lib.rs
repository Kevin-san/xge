//! engine-ui crate - UI 系统
//!
//! 提供游戏引擎的 UI 系统实现，包括布局、控件、文本渲染等功能。

#![warn(missing_docs)]

pub mod input;
pub mod layout;
pub mod style;
pub mod text;
pub mod ui_node;
pub mod widgets;

pub use input::{
    DragConfig, EventListener, SubscriptionId, UiEvent, UiEventBus, UiEventType, UiInput,
};
pub use layout::{
    Alignment, Anchor, AnchorLayoutEngine, AnchorOffset, Dimension, FlexContainer, FlexDirection,
    FlexItem, FlexLayoutEngine, FlexLayoutResult, FlexWrap, JustifyContent, LayoutDirection,
    LayoutProperties, LayoutType, Margin, Padding, Pivot, Stretch, AlignItems, AlignSelf,
};
pub use text::{
    Font, FontHeader, FontLoadError, FontLoader, FontMetrics, FontParser, FontSize,
    HorizontalHeader, KerningPair, Os2Metrics, ParsedFont, TextAlign, TextLayoutEngine,
    TextLayoutOptions, TextLine, TextRenderer,
};
pub use ui_node::{UiNode, UiNodeType, UiRoot};
pub use widgets::{
    Button, CheckBox, Grid, GridFlow, Label, Panel, ProgressBar, ScrollPanel, Slider,
    SliderDirection, TextBox,
};

#[cfg(test)]
mod tests {
    use engine_ecs::{Event, Events, World};
    use engine_math::{Rect, Vec2};

    use super::*;
    use crate::style::{Style, TextStyle};
    use crate::widgets::Button as ButtonWidget;

    #[test]
    fn test_uisystem_create_world() {
        let world = World::new();
        let _ = world;
    }

    #[test]
    fn test_uisystem_spawn_ui_node() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, UiNode::new(UiNodeType::Panel));
        assert!(world.get_component::<UiNode>(entity).is_some());
    }

    #[test]
    fn test_uisystem_spawn_multiple_nodes() {
        let mut world = World::new();
        let entities: Vec<_> = (0..5).map(|_| world.spawn()).collect();
        for (i, &e) in entities.iter().enumerate() {
            let node_type = match i % 3 {
                0 => UiNodeType::Button,
                1 => UiNodeType::Label,
                _ => UiNodeType::Panel,
            };
            world.insert(e, UiNode::new(node_type));
        }
        for &e in &entities {
            assert!(world.get_component::<UiNode>(e).is_some());
        }
    }

    #[test]
    fn test_uisystem_root_setup() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, UiNode::new(UiNodeType::Root));
        world.insert(root_entity, UiRoot::new(root_entity, Vec2::new(800.0, 600.0)));
        assert!(world.get_component::<UiRoot>(root_entity).is_some());
    }

    #[test]
    fn test_uisystem_root_update_layout() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, UiNode::new(UiNodeType::Root));
        world.insert(root_entity, UiRoot::new(root_entity, Vec2::new(1024.0, 768.0)));
        let root = world.get_component::<UiRoot>(root_entity).unwrap().clone();
        root.update(&mut world);
        let node = world.get_component::<UiNode>(root_entity).unwrap();
        assert_eq!(node.rect().w, 1024.0);
        assert_eq!(node.rect().h, 768.0);
    }

    #[test]
    fn test_uisystem_event_dispatch_button_click() {
        let mut events = Events::<UiEvent>::new();
        let mut world = World::new();
        let entity = world.spawn();
        let event = UiEvent::new(UiEventType::Click, entity);
        events.send(event);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_uisystem_event_mouse_move_dispatch() {
        let mut events = Events::<UiEvent>::new();
        let mut world = World::new();
        let entity = world.spawn();
        let mut event = UiEvent::new(UiEventType::MouseMove, entity);
        event.set_mouse_position(Vec2::new(100.0, 200.0));
        events.send(event);
        let iter_count = events.iter().count();
        assert_eq!(iter_count, 1);
    }

    #[test]
    fn test_uisystem_event_key_down_dispatch() {
        let mut events = Events::<UiEvent>::new();
        let mut world = World::new();
        let entity = world.spawn();
        let mut event = UiEvent::new(UiEventType::KeyDown, entity);
        event.set_text("A");
        events.send(event);
        assert!(!events.is_empty());
    }

    #[test]
    fn test_uisystem_events_update_clears_current() {
        let mut events = Events::<UiEvent>::new();
        let mut world = World::new();
        let entity = world.spawn();
        events.send(UiEvent::new(UiEventType::Click, entity));
        events.send(UiEvent::new(UiEventType::MouseDown, entity));
        assert_eq!(events.len(), 2);
        events.update();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_uisystem_register_button_component() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, UiNode::new(UiNodeType::Button));
        world.insert(entity, ButtonWidget::new("OK"));
        assert!(world.get_component::<ButtonWidget>(entity).is_some());
    }

    #[test]
    fn test_uisystem_register_label_component() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, UiNode::new(UiNodeType::Label));
        world.insert(entity, Label::new("Hello"));
        let label = world.get_component::<Label>(entity).unwrap();
        assert_eq!(label.text(), "Hello");
    }

    #[test]
    fn test_uisystem_register_panel_component() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, UiNode::new(UiNodeType::Panel));
        world.insert(entity, Panel::new());
        assert!(world.get_component::<Panel>(entity).is_some());
    }

    #[test]
    fn test_uisystem_register_textbox_component() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, UiNode::new(UiNodeType::TextBox));
        world.insert(entity, TextBox::new("input"));
        let tb = world.get_component::<TextBox>(entity).unwrap();
        assert_eq!(tb.text(), "input");
    }

    #[test]
    fn test_uisystem_register_checkbox_component() {
        let mut world = World::new();
        let entity = world.spawn();
        world.insert(entity, UiNode::new(UiNodeType::CheckBox));
        world.insert(entity, CheckBox::new("Enable"));
        let cb = world.get_component::<CheckBox>(entity).unwrap();
        assert!(!cb.is_checked());
    }

    #[test]
    fn test_uisystem_child_hierarchy_with_world_position() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        let mut parent_node = UiNode::new(UiNodeType::Panel);
        parent_node.set_rect(Rect::new(50.0, 50.0, 200.0, 100.0));
        parent_node.add_child(child);
        let mut child_node = UiNode::new(UiNodeType::Button);
        child_node.set_rect(Rect::new(10.0, 10.0, 80.0, 30.0));
        child_node.set_parent(Some(parent));
        world.insert(parent, parent_node);
        world.insert(child, child_node);
        let child_node_ref = world.get_component::<UiNode>(child).unwrap();
        let pos = child_node_ref.world_position(&world);
        assert_eq!(pos.x, 60.0);
        assert_eq!(pos.y, 60.0);
    }

    #[test]
    fn test_uisystem_style_default_values() {
        let style = Style::new();
        assert_eq!(style.background_color, engine_render::Color::WHITE);
        let text_style = TextStyle::new();
        assert_eq!(text_style.font_family, "Arial");
        assert_eq!(text_style.font_size, 16.0);
    }

    #[test]
    fn test_uisystem_uievent_is_send_sync_bound() {
        fn require_send_sync<T: Send + Sync>() {}
        require_send_sync::<UiEvent>();
    }

    #[test]
    fn test_uisystem_event_trait_impl() {
        fn require_event<E: Event>() {}
        require_event::<UiEvent>();
    }

    #[test]
    fn test_uisystem_layout_type_enum_variants() {
        let _none = LayoutType::None;
        let _h = LayoutType::Horizontal;
        let _v = LayoutType::Vertical;
    }

    #[test]
    fn test_uisystem_font_size_variants() {
        let small = FontSize::Small;
        assert_eq!(small.to_f32(), 12.0);
        assert_eq!(FontSize::Medium.to_f32(), 16.0);
    }
}
