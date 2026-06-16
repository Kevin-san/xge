use engine_ecs::World;
use engine_math::Vec2;
use engine_ui::{Button, Label, UiNode, UiNodeType, UiRoot};

fn main() {
    let mut world = World::new();

    let ui_root = world.spawn();
    world.insert(ui_root, UiNode::new(UiNodeType::Root));
    world.insert(ui_root, UiRoot::new(ui_root, Vec2::new(800.0, 600.0)));

    let button_entity = world.spawn();
    let mut button_node = UiNode::new(UiNodeType::Button);
    button_node.rect_mut().x = 350.0;
    button_node.rect_mut().y = 250.0;
    button_node.rect_mut().w = 100.0;
    button_node.rect_mut().h = 50.0;
    world.insert(button_entity, button_node);
    world.insert(button_entity, Button::new("Click Me"));

    let label_entity = world.spawn();
    let mut label_node = UiNode::new(UiNodeType::Label);
    label_node.rect_mut().x = 350.0;
    label_node.rect_mut().y = 320.0;
    label_node.rect_mut().w = 100.0;
    label_node.rect_mut().h = 30.0;
    world.insert(label_entity, label_node);
    world.insert(label_entity, Label::new("Clicks: 0"));

    if let Some(root_node) = world.get_component_mut::<UiNode>(ui_root) {
        root_node.add_child(button_entity);
        root_node.add_child(label_entity);
    }

    println!("UI World created successfully!");

    let root = world.get_component::<UiRoot>(ui_root).unwrap().clone();
    root.update(&mut world);
    println!("UI updated successfully!");

    println!("Example completed successfully!");
}