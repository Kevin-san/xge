use anyhow::Result;
use engine_core::Application;
use engine_ecs::{Entity, World};
use engine_math::Vec2;
use engine_render::Color;
use engine_ui::{Button, Label, UiNode, UiNodeType, UiRoot};
use engine_window::{Event, KeyCode, Window};

struct TestApp {
    world: World,
    ui_root: Entity,
    button_entity: Entity,
    label_entity: Entity,
    click_count: usize,
}

impl Application for TestApp {
    fn new() -> Self {
        let mut world = World::new();

        let ui_root = world.spawn();
        world.insert(ui_root, UiNode::new(UiNodeType::Root));
        world.insert(ui_root, UiRoot::new(ui_root, Vec2::new(800.0, 600.0)));

        let button_entity = world.spawn();
        let mut button_node = UiNode::new(UiNodeType::Button);
        button_node.rect.x = 350.0;
        button_node.rect.y = 250.0;
        button_node.rect.w = 100.0;
        button_node.rect.h = 50.0;
        world.insert(button_entity, button_node);
        world.insert(button_entity, Button::new("Click Me"));

        let label_entity = world.spawn();
        let mut label_node = UiNode::new(UiNodeType::Label);
        label_node.rect.x = 350.0;
        label_node.rect.y = 320.0;
        label_node.rect.w = 100.0;
        label_node.rect.h = 30.0;
        world.insert(label_entity, label_node);
        world.insert(label_entity, Label::new("Clicks: 0"));

        if let Some(mut root_node) = world.get_component_mut::<UiNode>(ui_root) {
            root_node.add_child(button_entity);
            root_node.add_child(label_entity);
        }

        Self {
            world,
            ui_root,
            button_entity,
            label_entity,
            click_count: 0,
        }
    }

    fn update(&mut self, _delta_time: f32) {
        if let Some(root) = self.world.get_component_mut::<UiRoot>(self.ui_root) {
            root.update(&mut self.world);
        }
    }

    fn render(&mut self, window: &mut Window) {
        window.clear(Color::new(0.1, 0.1, 0.1, 1.0));

        self.render_ui(window);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseButtonDown(_, _) => {
                self.click_count += 1;
                if let Some(label) = self.world.get_component_mut::<Label>(self.label_entity) {
                    label.set_text(&format!("Clicks: {}", self.click_count));
                }
            }
            Event::KeyDown(KeyCode::Escape) => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

impl TestApp {
    fn render_ui(&self, window: &mut Window) {
        self.render_node(window, self.ui_root);
    }

    fn render_node(&self, window: &mut Window, entity: Entity) {
        if let Some(node) = self.world.get_component::<UiNode>(entity) {
            let color = match node.node_type {
                UiNodeType::Root => Color::TRANSPARENT,
                UiNodeType::Panel => Color::new(0.3, 0.3, 0.3, 1.0),
                UiNodeType::Button => {
                    if let Some(button) = self.world.get_component::<Button>(entity) {
                        if button.is_pressed() {
                            Color::new(0.4, 0.6, 0.8, 1.0)
                        } else {
                            Color::new(0.2, 0.4, 0.6, 1.0)
                        }
                    } else {
                        Color::new(0.2, 0.4, 0.6, 1.0)
                    }
                }
                UiNodeType::Label => Color::TRANSPARENT,
                _ => Color::new(0.5, 0.5, 0.5, 1.0),
            };

            if color.alpha > 0.0 {
                window.fill_rect(node.rect.x, node.rect.y, node.rect.w, node.rect.h, color);
            }

            if let Some(label) = self.world.get_component::<Label>(entity) {
                window.draw_text(
                    label.text(),
                    node.rect.x + 10.0,
                    node.rect.y + 20.0,
                    Color::WHITE,
                );
            }

            if let Some(button) = self.world.get_component::<Button>(entity) {
                window.draw_text(
                    button.text(),
                    node.rect.x + 20.0,
                    node.rect.y + 15.0,
                    Color::WHITE,
                );
            }

            for &child in node.children() {
                self.render_node(window, child);
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let mut window = Window::new("Engine Example", 800, 600)?;
    let mut app = TestApp::new();

    window.run(&mut app)
}
