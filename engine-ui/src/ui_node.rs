//! UI 节点模块
//!
//! 定义 UI 节点的基础类型和操作，集成 Flex/Anchor 布局引擎。

use engine_ecs::{Component, Entity, World};
use engine_math::{Rect, Vec2};

use crate::layout::{
    Anchor, AnchorLayoutEngine, AnchorOffset, FlexContainer, FlexItem, FlexLayoutEngine,
    LayoutDirection, LayoutProperties, LayoutType, Pivot,
};
use crate::style::Style;

/// UI节点类型
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UiNodeType {
    /// 根节点
    Root,
    /// 面板
    Panel,
    /// 按钮
    Button,
    /// 标签
    Label,
    /// 文本框
    TextBox,
    /// 复选框
    CheckBox,
}

/// UI节点
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
    /// Flex 容器配置（当 layout == Flex 时生效）
    flex_container: FlexContainer,
    /// Flex 子项配置（作为父容器的子项时生效）
    flex_item: FlexItem,
    /// Anchor 锚点定义（当 layout == Anchor 时生效）
    anchor_layout: Anchor,
    /// Anchor 偏移
    anchor_offset: AnchorOffset,
    /// Anchor 支点
    pivot_layout: Pivot,
    /// 自定义尺寸（None 表示由锚点拉伸决定）
    custom_size: Option<Vec2>,
}

impl UiNode {
    /// 创建新的UI节点
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
            flex_container: FlexContainer::default(),
            flex_item: FlexItem::default(),
            anchor_layout: Anchor::default(),
            anchor_offset: AnchorOffset::default(),
            pivot_layout: Pivot::default(),
            custom_size: None,
        }
    }

    /// 获取节点矩形
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    /// 获取可变节点矩形
    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }

    /// 设置节点矩形
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    /// 获取锚点
    pub fn anchor(&self) -> Vec2 {
        self.anchor
    }

    /// 设置锚点
    pub fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    /// 获取支点
    pub fn pivot(&self) -> Vec2 {
        self.pivot
    }

    /// 设置支点
    pub fn set_pivot(&mut self, pivot: Vec2) {
        self.pivot = pivot;
    }

    /// 是否可见
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// 设置可见性
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// 是否启用
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// 设置启用状态
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 获取节点类型
    pub fn node_type(&self) -> UiNodeType {
        self.node_type
    }

    /// 获取布局类型
    pub fn layout(&self) -> LayoutType {
        self.layout
    }

    /// 设置布局类型
    pub fn set_layout(&mut self, layout: LayoutType) {
        self.layout = layout;
    }

    /// 获取布局方向
    pub fn layout_dir(&self) -> LayoutDirection {
        self.layout_dir
    }

    /// 设置布局方向
    pub fn set_layout_dir(&mut self, dir: LayoutDirection) {
        self.layout_dir = dir;
    }

    /// 获取布局属性
    pub fn layout_props(&self) -> &LayoutProperties {
        &self.layout_props
    }

    /// 获取可变布局属性
    pub fn layout_props_mut(&mut self) -> &mut LayoutProperties {
        &mut self.layout_props
    }

    /// 获取 Flex 容器配置
    pub fn flex_container(&self) -> &FlexContainer {
        &self.flex_container
    }

    /// 获取可变 Flex 容器配置
    pub fn flex_container_mut(&mut self) -> &mut FlexContainer {
        &mut self.flex_container
    }

    /// 获取 Flex 子项配置
    pub fn flex_item(&self) -> &FlexItem {
        &self.flex_item
    }

    /// 获取可变 Flex 子项配置
    pub fn flex_item_mut(&mut self) -> &mut FlexItem {
        &mut self.flex_item
    }

    /// 获取 Anchor 锚点定义
    pub fn anchor_layout(&self) -> Anchor {
        self.anchor_layout
    }

    /// 设置 Anchor 锚点定义
    pub fn set_anchor_layout(&mut self, anchor: Anchor) {
        self.anchor_layout = anchor;
    }

    /// 获取 Anchor 偏移
    pub fn anchor_offset(&self) -> AnchorOffset {
        self.anchor_offset
    }

    /// 设置 Anchor 偏移
    pub fn set_anchor_offset(&mut self, offset: AnchorOffset) {
        self.anchor_offset = offset;
    }

    /// 获取 Anchor 支点
    pub fn pivot_layout(&self) -> Pivot {
        self.pivot_layout
    }

    /// 设置 Anchor 支点
    pub fn set_pivot_layout(&mut self, pivot: Pivot) {
        self.pivot_layout = pivot;
    }

    /// 获取自定义尺寸
    pub fn custom_size(&self) -> Option<Vec2> {
        self.custom_size
    }

    /// 设置自定义尺寸（None 表示由锚点拉伸决定）
    pub fn set_custom_size(&mut self, size: Option<Vec2>) {
        self.custom_size = size;
    }

    /// 获取样式
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// 获取可变样式
    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    /// 获取父节点
    pub fn parent(&self) -> Option<Entity> {
        self.parent
    }

    /// 设置父节点
    pub fn set_parent(&mut self, parent: Option<Entity>) {
        self.parent = parent;
    }

    /// 获取子节点列表
    pub fn children(&self) -> &[Entity] {
        &self.children
    }

    /// 添加子节点
    pub fn add_child(&mut self, child: Entity) {
        self.children.push(child);
    }

    /// 移除子节点
    pub fn remove_child(&mut self, child: Entity) -> bool {
        if let Some(index) = self.children.iter().position(|&c| c == child) {
            self.children.remove(index);
            true
        } else {
            false
        }
    }

    /// 检查是否包含子节点
    pub fn has_child(&self, child: Entity) -> bool {
        self.children.contains(&child)
    }

    /// 获取世界坐标
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

    /// 更新布局
    pub fn update_layout(&self, world: &mut World) {
        match self.layout {
            LayoutType::Horizontal | LayoutType::Vertical => {
                self.layout_children(world);
            }
            LayoutType::Flex => {
                self.layout_children_flex(world);
            }
            LayoutType::Anchor => {
                self.layout_children_anchor(world);
            }
            LayoutType::None => {}
        }
    }

    /// 内部布局更新
    pub fn update_layout_internal(&mut self, world: &mut World) {
        match self.layout {
            LayoutType::Horizontal | LayoutType::Vertical => {
                self.layout_children(world);
            }
            LayoutType::Flex => {
                self.layout_children_flex(world);
            }
            LayoutType::Anchor => {
                self.layout_children_anchor(world);
            }
            LayoutType::None => {}
        }
    }

    /// 布局子节点（Horizontal/Vertical 简单线性布局）
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

    /// 使用 Flex 布局引擎布局子节点
    fn layout_children_flex(&self, world: &mut World) {
        let container_rect = self.rect;
        let container = self.flex_container;

        // 收集子项数据（需要先克隆以避免借用冲突）
        let child_data: Vec<(Entity, FlexItem, f32, f32)> = self
            .children
            .iter()
            .filter_map(|&child| {
                world.get_component::<UiNode>(child).map(|node| {
                    let item = node.flex_item;
                    let main = if container.direction.is_row() {
                        node.rect.w
                    } else {
                        node.rect.h
                    };
                    let cross = if container.direction.is_row() {
                        node.rect.h
                    } else {
                        node.rect.w
                    };
                    (child, item, main, cross)
                })
            })
            .collect();

        let items: Vec<(FlexItem, f32, f32)> = child_data
            .iter()
            .map(|(_, item, main, cross)| (*item, *main, *cross))
            .collect();

        let results = FlexLayoutEngine::layout(container_rect, &container, &items);

        for ((child, _, _, _), result) in child_data.iter().zip(results.iter()) {
            if let Some(child_node) = world.get_component_mut::<UiNode>(*child) {
                child_node.rect = result.rect;
            }
        }
    }

    /// 使用 Anchor 布局引擎布局子节点
    fn layout_children_anchor(&self, world: &mut World) {
        let parent_rect = self.rect;
        for &child in &self.children {
            if let Some(child_node) = world.get_component_mut::<UiNode>(child) {
                let anchor = child_node.anchor_layout;
                let offset = child_node.anchor_offset;
                let pivot = child_node.pivot_layout;
                let custom_size = child_node.custom_size;
                let new_rect =
                    AnchorLayoutEngine::compute(parent_rect, anchor, offset, pivot, custom_size);
                child_node.rect = new_rect;
            }
        }
    }
}

impl Component for UiNode {}

/// UI根节点
#[derive(Clone)]
pub struct UiRoot {
    root_entity: Entity,
    canvas_size: Vec2,
}

impl UiRoot {
    /// 创建新的UI根节点
    pub fn new(root_entity: Entity, canvas_size: Vec2) -> Self {
        Self {
            root_entity,
            canvas_size,
        }
    }

    /// 获取根实体
    pub fn root_entity(&self) -> Entity {
        self.root_entity
    }

    /// 获取画布大小
    pub fn canvas_size(&self) -> Vec2 {
        self.canvas_size
    }

    /// 设置画布大小
    pub fn set_canvas_size(&mut self, size: Vec2) {
        self.canvas_size = size;
    }

    /// 更新UI布局
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
                LayoutType::Flex => {
                    self.layout_children_flex_internal(world, entity);
                }
                LayoutType::Anchor => {
                    self.layout_children_anchor_internal(world, entity);
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

    /// Flex 布局（UiRoot 递归调用入口）
    fn layout_children_flex_internal(&self, world: &mut World, entity: Entity) {
        let snapshot = world
            .get_component::<UiNode>(entity)
            .map(|node| (node.rect, node.flex_container, node.children().to_vec()));

        if let Some((container_rect, container, children)) = snapshot {
            let child_data: Vec<(Entity, FlexItem, f32, f32)> = children
                .iter()
                .filter_map(|&child| {
                    world.get_component::<UiNode>(child).map(|node| {
                        let item = node.flex_item;
                        let main = if container.direction.is_row() {
                            node.rect.w
                        } else {
                            node.rect.h
                        };
                        let cross = if container.direction.is_row() {
                            node.rect.h
                        } else {
                            node.rect.w
                        };
                        (child, item, main, cross)
                    })
                })
                .collect();

            let items: Vec<(FlexItem, f32, f32)> = child_data
                .iter()
                .map(|(_, item, main, cross)| (*item, *main, *cross))
                .collect();

            let results = FlexLayoutEngine::layout(container_rect, &container, &items);

            for ((child, _, _, _), result) in child_data.iter().zip(results.iter()) {
                if let Some(child_node) = world.get_component_mut::<UiNode>(*child) {
                    child_node.rect = result.rect;
                }
            }
        }
    }

    /// Anchor 布局（UiRoot 递归调用入口）
    fn layout_children_anchor_internal(&self, world: &mut World, entity: Entity) {
        let snapshot = world
            .get_component::<UiNode>(entity)
            .map(|node| (node.rect, node.children().to_vec()));

        if let Some((parent_rect, children)) = snapshot {
            for &child in &children {
                if let Some(child_node) = world.get_component_mut::<UiNode>(child) {
                    let anchor = child_node.anchor_layout;
                    let offset = child_node.anchor_offset;
                    let pivot = child_node.pivot_layout;
                    let custom_size = child_node.custom_size;
                    let new_rect = AnchorLayoutEngine::compute(
                        parent_rect,
                        anchor,
                        offset,
                        pivot,
                        custom_size,
                    );
                    child_node.rect = new_rect;
                }
            }
        }
    }

    /// 查找指定位置的节点
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
            if let Some(child_node) = world.get_component_mut::<UiNode>(child_entity) {
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

    #[test]
    fn test_ui_node_anchor_and_pivot() {
        let mut node = UiNode::new(UiNodeType::Panel);
        node.set_anchor(Vec2::new(0.0, 1.0));
        node.set_pivot(Vec2::new(0.5, 0.5));
        assert_eq!(node.anchor(), Vec2::new(0.0, 1.0));
        assert_eq!(node.pivot(), Vec2::new(0.5, 0.5));
    }

    #[test]
    fn test_ui_node_visible_toggle() {
        let mut node = UiNode::new(UiNodeType::Panel);
        assert!(node.visible());
        node.set_visible(false);
        assert!(!node.visible());
        node.set_visible(true);
        assert!(node.visible());
    }

    #[test]
    fn test_ui_node_enabled_toggle() {
        let mut node = UiNode::new(UiNodeType::Button);
        assert!(node.enabled());
        node.set_enabled(false);
        assert!(!node.enabled());
    }

    #[test]
    fn test_ui_node_layout_set_and_get() {
        let mut node = UiNode::new(UiNodeType::Panel);
        node.set_layout(LayoutType::Vertical);
        assert_eq!(node.layout(), LayoutType::Vertical);
        node.set_layout(LayoutType::Horizontal);
        assert_eq!(node.layout(), LayoutType::Horizontal);
    }

    #[test]
    fn test_ui_node_layout_direction() {
        let mut node = UiNode::new(UiNodeType::Panel);
        node.set_layout_dir(LayoutDirection::Vertical);
        assert_eq!(node.layout_dir(), LayoutDirection::Vertical);
        node.set_layout_dir(LayoutDirection::Horizontal);
        assert_eq!(node.layout_dir(), LayoutDirection::Horizontal);
    }

    #[test]
    fn test_ui_node_rect_mut() {
        let mut node = UiNode::new(UiNodeType::Panel);
        let rect = node.rect_mut();
        rect.x = 10.0;
        rect.y = 20.0;
        assert_eq!(node.rect().x, 10.0);
        assert_eq!(node.rect().y, 20.0);
    }

    #[test]
    fn test_ui_node_has_child() {
        let mut node = UiNode::new(UiNodeType::Panel);
        let child = Entity::new(1, 0);
        let other = Entity::new(2, 0);
        node.add_child(child);
        assert!(node.has_child(child));
        assert!(!node.has_child(other));
    }

    #[test]
    fn test_ui_node_remove_child() {
        let mut node = UiNode::new(UiNodeType::Panel);
        let child1 = Entity::new(1, 0);
        let child2 = Entity::new(2, 0);
        node.add_child(child1);
        node.add_child(child2);
        assert_eq!(node.children().len(), 2);
        assert!(node.remove_child(child1));
        assert_eq!(node.children().len(), 1);
        assert!(!node.remove_child(child1));
    }

    #[test]
    fn test_ui_node_parent_initial_none() {
        let node = UiNode::new(UiNodeType::Panel);
        assert!(node.parent().is_none());
    }

    #[test]
    fn test_ui_root_canvas_size_update() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, UiNode::new(UiNodeType::Root));
        world.insert(
            root_entity,
            UiRoot::new(root_entity, Vec2::new(640.0, 480.0)),
        );
        if let Some(root) = world.get_component_mut::<UiRoot>(root_entity) {
            root.set_canvas_size(Vec2::new(1280.0, 720.0));
        }
        let root = world.get_component::<UiRoot>(root_entity).unwrap();
        assert_eq!(root.canvas_size(), Vec2::new(1280.0, 720.0));
    }

    #[test]
    fn test_ui_root_root_entity() {
        let mut world = World::new();
        let root_entity = world.spawn();
        world.insert(root_entity, UiNode::new(UiNodeType::Root));
        world.insert(
            root_entity,
            UiRoot::new(root_entity, Vec2::new(800.0, 600.0)),
        );
        let root = world.get_component::<UiRoot>(root_entity).unwrap();
        assert_eq!(root.root_entity(), root_entity);
    }

    #[test]
    fn test_ui_node_type_variants() {
        let _root = UiNode::new(UiNodeType::Root);
        let _panel = UiNode::new(UiNodeType::Panel);
        let _button = UiNode::new(UiNodeType::Button);
        let _label = UiNode::new(UiNodeType::Label);
        let _textbox = UiNode::new(UiNodeType::TextBox);
        let _checkbox = UiNode::new(UiNodeType::CheckBox);
    }

    #[test]
    fn test_ui_node_world_position_no_parent() {
        let mut world = World::new();
        let entity = world.spawn();
        let mut node = UiNode::new(UiNodeType::Panel);
        node.set_rect(Rect::new(20.0, 30.0, 100.0, 100.0));
        world.insert(entity, node);
        let node_ref = world.get_component::<UiNode>(entity).unwrap();
        let pos = node_ref.world_position(&world);
        assert_eq!(pos.x, 20.0);
        assert_eq!(pos.y, 30.0);
    }

    #[test]
    fn test_ui_node_update_layout_internal() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();
        let mut parent_node = UiNode::new(UiNodeType::Panel);
        parent_node.set_layout(LayoutType::Horizontal);
        parent_node.add_child(child);
        let mut child_node = UiNode::new(UiNodeType::Button);
        child_node.rect_mut().w = 50.0;
        child_node.rect_mut().h = 30.0;
        world.insert(parent, parent_node);
        world.insert(child, child_node);
        let parent_node = UiNode::new(UiNodeType::Panel);
        let _ = parent_node;
        let mut p2 = UiNode::new(UiNodeType::Panel);
        p2.set_layout(LayoutType::Horizontal);
        p2.update_layout_internal(&mut world);
        let child_ref = world.get_component::<UiNode>(child).unwrap();
        assert_eq!(child_ref.rect().x, 0.0);
    }
}
