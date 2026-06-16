//! UI 节点模块
//!
//! 定义 UI 节点的基础类型和操作。

use engine_ecs::{Component, Entity, World};
use engine_math::{Rect, Vec2};

use crate::layout::{LayoutDirection, LayoutProperties, LayoutType};
use crate::style::Style;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UiNodeType {
    Root,
    Panel,
    Button,
    Label,
    TextBox,
    CheckBox,
}

pub struct UiNode {
    parent: Option<Entity>,
    children: Vec<Entity>,
    rect: Rect,
    anchor: Vec2,
    pivot: Vec2,
    visible: bool,
    enabled: bool,
    node_type: UiNodeType,
    layout: LayoutType,
    layout_dir: LayoutDirection,
    layout_props: LayoutProperties,
    style: Style,
}

impl UiNode {
    pub fn new(node_type: UiNodeType) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            rect: Rect::new(0.0, 0.0, 100.0, 100.0),
            anchor: Vec2::new(0.5, 0.5),
            pivot: Vec2::new(0.5, 0.5),
            visible: true,
            enabled: true,
            node_type,
            layout: LayoutType::None,
            layout_dir: LayoutDirection::Horizontal,
            layout_props: LayoutProperties::default(),
            style: Style::default(),
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn anchor(&self) -> Vec2 {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    pub fn pivot(&self) -> Vec2 {
        self.pivot
    }

    pub fn set_pivot(&mut self, pivot: Vec2) {
        self.pivot = pivot;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn node_type(&self) -> UiNodeType {
        self.node_type
    }

    pub fn layout(&self) -> LayoutType {
        self.layout
    }

    pub fn set_layout(&mut self, layout: LayoutType) {
        self.layout = layout;
    }

    pub fn layout_dir(&self) -> LayoutDirection {
        self.layout_dir
    }

    pub fn set_layout_dir(&mut self, dir: LayoutDirection) {
        self.layout_dir = dir;
    }

    pub fn layout_props(&self) -> &LayoutProperties {
        &self.layout_props
    }

    pub fn layout_props_mut(&mut self) -> &mut LayoutProperties {
        &mut self.layout_props
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn parent(&self) -> Option<Entity> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: Option<Entity>) {
        self.parent = parent;
    }

    pub fn children(&self) -> &[Entity] {
        &self.children
    }

    pub fn add_child(&mut self, child: Entity) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: Entity) -> bool {
        if let Some(index) = self.children.iter().position(|&c| c == child) {
            self.children.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_child(&self, child: Entity) -> bool {
        self.children.contains(&child)
    }

    pub fn world_position(&self, world: &World) -> Vec2 {
        let mut pos = Vec2::new(self.rect.x, self.rect.y);
        let mut current = self.parent;

        while let Some(parent_entity) = current {
            if let Some(parent_node) = world.get_component::<UiNode>(parent_entity) {
                pos.x += parent_node.rect.x;
                pos.y += parent_node.rect.y;
                current = parent_node.parent;
            } else {
                break;
            }
        }

        pos
    }

    pub fn update_layout(&self, world: &mut World) {
        match self.layout {
            LayoutType::Horizontal | LayoutType::Vertical => {
                self.layout_children(world);
            }
            LayoutType::None => {}
        }
    }

    pub fn update_layout_internal(&mut self, world: &mut World) {
        match self.layout {
            LayoutType::Horizontal | LayoutType::Vertical => {
                self.layout_children(world);
            }
            LayoutType::None => {}
        }
    }

    fn layout_children(&self, world: &mut World) {
        let mut pos = Vec2::new(
            self.layout_props.padding.left,
            self.layout_props.padding.top,
        );

        for &child_entity in &self.children {
            if let Some(child_node) = world.get_component_mut::<UiNode>(child_entity) {
                if self.layout_dir == LayoutDirection::Horizontal {
                    child_node.rect.x = pos.x + child_node.layout_props.margin.left;
                    child_node.rect.y =
                        self.layout_props.padding.top + child_node.layout_props.margin.top;
                    pos.x += child_node.rect.w
                        + child_node.layout_props.margin.right
                        + self.layout_props.spacing;
                } else {
                    child_node.rect.x =
                        self.layout_props.padding.left + child_node.layout_props.margin.left;
                    child_node.rect.y = pos.y + child_node.layout_props.margin.top;
                    pos.y += child_node.rect.h
                        + child_node.layout_props.margin.bottom
                        + self.layout_props.spacing;
                }
            }
        }
    }
}

impl Component for UiNode {}

#[derive(Clone)]
pub struct UiRoot {
    root_entity: Entity,
    canvas_size: Vec2,
}

impl UiRoot {
    pub fn new(root_entity: Entity, canvas_size: Vec2) -> Self {
        Self {
            root_entity,
            canvas_size,
        }
    }

    pub fn root_entity(&self) -> Entity {
        self.root_entity
    }

    pub fn canvas_size(&self) -> Vec2 {
        self.canvas_size
    }

    pub fn set_canvas_size(&mut self, size: Vec2) {
        self.canvas_size = size;
    }

    pub fn update(&self, world: &mut World) {
        if let Some(node) = world.get_component_mut::<UiNode>(self.root_entity) {
            node.rect.w = self.canvas_size.x;
            node.rect.h = self.canvas_size.y;
        }
        self.update_layout_recursive(world, self.root_entity);
    }

    fn update_layout_recursive(&self, world: &mut World, entity: Entity) {
        let children = if let Some(node) = world.get_component::<UiNode>(entity) {
            let children = node.children().to_vec();
            self.layout_node_children(world, entity);
            children
        } else {
            Vec::new()
        };
        for child in children {
            self.update_layout_recursive(world, child);
        }
    }

    fn layout_node_children(&self, world: &mut World, entity: Entity) {
        if let Some(node) = world.get_component::<UiNode>(entity) {
            match node.layout {
                LayoutType::Horizontal | LayoutType::Vertical => {
                    self.layout_children_internal(world, entity);
                }
                LayoutType::None => {}
            }
        }
    }

    fn layout_children_internal(&self, world: &mut World, entity: Entity) {
        let layout_data = world.get_component::<UiNode>(entity).map(|node| {
            (
                node.layout_dir,
                node.layout_props.padding.left,
                node.layout_props.padding.top,
                node.layout_props.spacing,
                node.children().to_vec(),
            )
        });

        if let Some((layout_dir, padding_left, padding_top, spacing, children)) = layout_data {
            let mut pos = Vec2::new(padding_left, padding_top);

            for &child_entity in &children {
                if let Some(child_node) = world.get_component_mut::<UiNode>(child_entity) {
                    if layout_dir == LayoutDirection::Horizontal {
                        child_node.rect.x = pos.x + child_node.layout_props.margin.left;
                        child_node.rect.y = padding_top + child_node.layout_props.margin.top;
                        pos.x += child_node.rect.w + child_node.layout_props.margin.right + spacing;
                    } else {
                        child_node.rect.x = padding_left + child_node.layout_props.margin.left;
                        child_node.rect.y = pos.y + child_node.layout_props.margin.top;
                        pos.y +=
                            child_node.rect.h + child_node.layout_props.margin.bottom + spacing;
                    }
                }
            }
        }
    }

    pub fn find_node_at_position(&self, world: &World, position: Vec2) -> Option<Entity> {
        self.find_node_at_position_recursive(world, self.root_entity, position)
    }

    fn find_node_at_position_recursive(
        &self,
        world: &World,
        entity: Entity,
        position: Vec2,
    ) -> Option<Entity> {
        if let Some(node) = world.get_component::<UiNode>(entity) {
            if !node.visible {
                return None;
            }

            let world_pos = node.world_position(world);
            let node_rect = Rect::new(world_pos.x, world_pos.y, node.rect.w, node.rect.h);

            if node_rect.contains(position) {
                for &child in node.children().iter().rev() {
                    if let Some(found) =
                        self.find_node_at_position_recursive(world, child, position)
                    {
                        return Some(found);
                    }
                }
                return Some(entity);
            }
        }
        None
    }
}

impl Component for UiRoot {}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_ecs::World;

    #[test]
    fn test_ui_node_creation() {
        let node = UiNode::new(UiNodeType::Panel);
        assert_eq!(node.node_type(), UiNodeType::Panel);
        assert!(node.visible());
        assert!(node.enabled());
    }

    #[test]
    fn test_ui_node_rect() {
        let mut node = UiNode::new(UiNodeType::Panel);
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        node.set_rect(rect);
        assert_eq!(node.rect(), &rect);
    }

    #[test]
    fn test_ui_root_update() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, UiNode::new(UiNodeType::Root));
        world.insert(
            root_entity,
            UiRoot::new(root_entity, Vec2::new(800.0, 600.0)),
        );

        let root = world.get_component::<UiRoot>(root_entity).unwrap().clone();
        root.update(&mut world);

        let node = world.get_component::<UiNode>(root_entity).unwrap();
        assert_eq!(node.rect().w, 800.0);
        assert_eq!(node.rect().h, 600.0);
    }

    #[test]
    fn test_ui_node_layout_children() {
        let mut world = World::new();
        let parent = world.spawn();
        let child1 = world.spawn();
        let child2 = world.spawn();

        let mut parent_node = UiNode::new(UiNodeType::Panel);
        parent_node.set_layout(LayoutType::Horizontal);
        parent_node.layout_props_mut().spacing = 10.0;
        parent_node.add_child(child1);
        parent_node.add_child(child2);

        let mut child1_node = UiNode::new(UiNodeType::Button);
        child1_node.rect_mut().w = 50.0;
        child1_node.rect_mut().h = 30.0;

        let mut child2_node = UiNode::new(UiNodeType::Button);
        child2_node.rect_mut().w = 50.0;
        child2_node.rect_mut().h = 30.0;

        world.insert(parent, parent_node);
        world.insert(child1, child1_node);
        world.insert(child2, child2_node);

        let layout_dir = world.get_component::<UiNode>(parent).unwrap().layout_dir;
        let padding_left = world
            .get_component::<UiNode>(parent)
            .unwrap()
            .layout_props
            .padding
            .left;
        let padding_top = world
            .get_component::<UiNode>(parent)
            .unwrap()
            .layout_props
            .padding
            .top;
        let spacing = world
            .get_component::<UiNode>(parent)
            .unwrap()
            .layout_props
            .spacing;
        let children = world
            .get_component::<UiNode>(parent)
            .unwrap()
            .children()
            .to_vec();

        let mut pos = Vec2::new(padding_left, padding_top);
        for &child_entity in &children {
            if let Some(mut child_node) = world.get_component_mut::<UiNode>(child_entity) {
                if layout_dir == LayoutDirection::Horizontal {
                    child_node.rect.x = pos.x + child_node.layout_props.margin.left;
                    child_node.rect.y = padding_top + child_node.layout_props.margin.top;
                    pos.x += child_node.rect.w + child_node.layout_props.margin.right + spacing;
                }
            }
        }

        let child1_node = world.get_component::<UiNode>(child1).unwrap();
        let child2_node = world.get_component::<UiNode>(child2).unwrap();

        assert_eq!(child1_node.rect().x, 0.0);
        assert_eq!(child2_node.rect().x, 60.0);
    }
}
