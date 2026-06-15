# 测试计划

## 概述

本文档定义 `engine-ui` crate 的测试策略，包括单元测试、集成测试和验收测试。所有测试应通过 `cargo test -p engine-ui` 执行。

---

## 测试统计

| 需求ID | 描述 |
|--------|------|
| 311 | 单测 `UiRect` 计算 |
| 312 | 单测 `Anchor` 计算 |
| 313 | 单测 `VerticalLayout` |
| 314 | 单测 `HorizontalLayout` |
| 315 | 单测 `GridLayout` |
| 316 | 单测 `Font` measure |
| 317 | 单测 `TextLayout` glyph 位置 |
| 318 | 单测 `TextLayout` word wrap |
| 319 | 单测 `UiButton` 状态切换 |
| 320 | 单测 `UiSlider` 值 clamp |
| 321 | 单测 `UiInput` 光标 + 输入 |
| 322 | 单测 `UiInput` Tab 切换 |
| 323 | 单测 `UiEvent` 派发 |
| 324 | 单测 `UiFocus` 顺序 |
| 325 | 集成测试 UI 渲染无崩溃 |
| 326 | 集成测试 UI 输入 |
| 327 | `cargo test -p engine-ui` 全部通过 |
| 328 | `cargo clippy --workspace -- -D warnings` 通过 |
| 329 | `cargo fmt --check --workspace` 通过 |
| 330 | `cargo doc --workspace --no-deps` 成功 |

---

## 单元测试

### UiRect 计算测试

```rust
#[test]
fn test_uirect_position() {
    let rect = UiRect {
        position: Vec2::new(10.0, 20.0),
        size: Vec2::new(100.0, 50.0),
        anchor: Anchor::TopLeft,
        margin: UiMargin::all(0.0),
        padding: UiPadding::all(0.0),
        z_index: 0,
    };
    assert_eq!(rect.position, Vec2::new(10.0, 20.0));
}

#[test]
fn test_uirect_desired_size() {
    let rect = UiRect {
        position: Vec2::ZERO,
        size: Vec2::new(100.0, 50.0),
        anchor: Anchor::Center,
        margin: UiMargin::all(0.0),
        padding: UiPadding::all(0.0),
        z_index: 0,
    };
    assert_eq!(rect.desired_size(), Vec2::new(100.0, 50.0));
}

#[test]
fn test_uirect_final_rect() {
    let rect = UiRect {
        position: Vec2::ZERO,
        size: Vec2::new(100.0, 50.0),
        anchor: Anchor::Center,
        margin: UiMargin::all(0.0),
        padding: UiPadding::all(0.0),
        z_index: 0,
    };
    let parent = Rect::new(0.0, 0.0, 800.0, 600.0);
    let final_rect = rect.final_rect(parent);
    // 居中时 position = (800-100)/2, (600-50)/2 = (350, 275)
    assert_eq!(final_rect.position, Vec2::new(350.0, 275.0));
}

#[test]
fn test_uirect_visible() {
    let visible = UiRect { visibility: UiVisibility::Visible, .. };
    let hidden = UiRect { visibility: UiVisibility::Hidden, .. };
    let collapsed = UiRect { visibility: UiVisibility::Collapsed, .. };

    assert!(visible.visible());
    assert!(!hidden.visible());
    assert!(!collapsed.visible()); // Collapsed 也返回 false
}
```

**验收标准**：所有 UiRect 相关测试通过

---

### Anchor 计算测试

```rust
#[test]
fn test_anchor_top_left() {
    let anchor = Anchor::TopLeft;
    let offset = anchor.offset(Vec2::new(800.0, 600.0));
    assert_eq!(offset, Vec2::new(0.0, 0.0));
}

#[test]
fn test_anchor_top_center() {
    let anchor = Anchor::TopCenter;
    let offset = anchor.offset(Vec2::new(800.0, 600.0));
    assert_eq!(offset, Vec2::new(400.0, 0.0));
}

#[test]
fn test_anchor_center() {
    let anchor = Anchor::Center;
    let offset = anchor.offset(Vec2::new(800.0, 600.0));
    assert_eq!(offset, Vec2::new(400.0, 300.0));
}

#[test]
fn test_anchor_bottom_right() {
    let anchor = Anchor::BottomRight;
    let offset = anchor.offset(Vec2::new(800.0, 600.0));
    assert_eq!(offset, Vec2::new(800.0, 600.0));
}

#[test]
fn test_anchor_custom() {
    let anchor = Anchor::Custom(Vec2::new(0.25, 0.75));
    let offset = anchor.offset(Vec2::new(800.0, 600.0));
    assert_eq!(offset, Vec2::new(200.0, 450.0));
}
```

**验收标准**：所有 Anchor 计算测试通过

---

### 布局系统测试

#### VerticalLayout

```rust
#[test]
fn test_vertical_layout() {
    let mut layout = VerticalLayout::new();
    layout.add_child(UiRect { size: Vec2::new(100.0, 20.0), .. });
    layout.add_child(UiRect { size: Vec2::new(100.0, 30.0), .. });
    layout.add_child(UiRect { size: Vec2::new(100.0, 40.0), .. });

    let result = layout.calculate(Vec2::new(800.0, 600.0));

    assert_eq!(result[0].position, Vec2::new(0.0, 0.0));
    assert_eq!(result[1].position, Vec2::new(0.0, 20.0));
    assert_eq!(result[2].position, Vec2::new(0.0, 50.0));
    assert_eq!(result.total_height(), 90.0);
}
```

#### HorizontalLayout

```rust
#[test]
fn test_horizontal_layout() {
    let mut layout = HorizontalLayout::new();
    layout.add_child(UiRect { size: Vec2::new(50.0, 100.0), .. });
    layout.add_child(UiRect { size: Vec2::new(60.0, 100.0), .. });
    layout.add_child(UiRect { size: Vec2::new(70.0, 100.0), .. });

    let result = layout.calculate(Vec2::new(800.0, 600.0));

    assert_eq!(result[0].position, Vec2::new(0.0, 0.0));
    assert_eq!(result[1].position, Vec2::new(50.0, 0.0));
    assert_eq!(result[2].position, Vec2::new(110.0, 0.0));
    assert_eq!(result.total_width(), 180.0);
}
```

#### GridLayout

```rust
#[test]
fn test_grid_layout_3x3() {
    let mut layout = GridLayout::new(3, 3);
    layout.set_gap(Vec2::new(10.0, 10.0));

    for i in 0..9 {
        layout.add_child(UiRect { size: Vec2::new(50.0, 50.0), .. });
    }

    let result = layout.calculate(Vec2::new(800.0, 600.0));

    // 第一行
    assert_eq!(result[0].position, Vec2::new(0.0, 0.0));
    assert_eq!(result[1].position, Vec2::new(60.0, 0.0));
    assert_eq!(result[2].position, Vec2::new(120.0, 0.0));

    // 第二行
    assert_eq!(result[3].position, Vec2::new(0.0, 60.0));
    assert_eq!(result[4].position, Vec2::new(60.0, 60.0));
    assert_eq!(result[5].position, Vec2::new(120.0, 60.0));
}
```

**验收标准**：所有布局测试通过

---

### Font 测试

```rust
#[test]
fn test_font_measure() {
    let font = Font::from_file("assets/fonts/arial.ttf").unwrap();
    let size = font.measure("Hello", 16.0);
    assert!(size.x > 0.0);
    assert!(size.y > 0.0);
}

#[test]
fn test_font_measure_empty() {
    let font = Font::from_file("assets/fonts/arial.ttf").unwrap();
    let size = font.measure("", 16.0);
    assert_eq!(size.x, 0.0);
    assert!(size.y > 0.0);
}
```

**验收标准**：Font measure 测试通过

---

### TextLayout 测试

#### Glyph 位置

```rust
#[test]
fn test_textlayout_glyph_positions() {
    let font = Font::from_file("assets/fonts/arial.ttf").unwrap();
    let layout = TextLayout::new(&font, 16.0, "ABC", 100.0, TextAlign::Left);

    let glyphs = layout.glyphs();
    assert_eq!(glyphs.len(), 3);

    // 检查字形位置递增
    let mut prev_x = 0.0;
    for glyph in glyphs {
        assert!(glyph.position.x >= prev_x);
        prev_x = glyph.position.x;
    }
}
```

#### Word Wrap

```rust
#[test]
fn test_textlayout_word_wrap() {
    let font = Font::from_file("assets/fonts/arial.ttf").unwrap();

    // 构造一个长单词超过 max_width 的情况
    let layout = TextLayout::new(&font, 16.0, "SuperLongWordThatExceedsWidth", 50.0, TextAlign::Left);

    let lines = layout.lines();
    // 长单词应该被换行或截断
    assert!(lines.len() >= 1);
}
```

**验收标准**：TextLayout 测试通过

---

### UiButton 状态机测试

```rust
#[test]
fn test_button_default_state() {
    let btn = UiButton::new("Click me");
    assert_eq!(btn.state(), ButtonState::Normal);
    assert!(!btn.is_disabled());
}

#[test]
fn test_button_state_transitions() {
    let mut btn = UiButton::new("Click me");

    btn.set_state(ButtonState::Hovered);
    assert_eq!(btn.state(), ButtonState::Hovered);

    btn.set_state(ButtonState::Pressed);
    assert_eq!(btn.state(), ButtonState::Pressed);

    btn.set_state(ButtonState::Normal);
    assert_eq!(btn.state(), ButtonState::Normal);
}

#[test]
fn test_button_disabled() {
    let mut btn = UiButton::new("Click me");
    assert!(!btn.is_disabled());

    btn.set_disabled(true);
    assert!(btn.is_disabled());
    assert_eq!(btn.state(), ButtonState::Disabled);

    btn.set_disabled(false);
    assert!(!btn.is_disabled());
}
```

**验收标准**：Button 状态机测试通过

---

### UiSlider 值测试

```rust
#[test]
fn test_slider_value_clamp() {
    let mut slider = UiSlider::new(0.0, 100.0, 50.0);

    slider.set_value(75.0);
    assert_eq!(slider.value(), 75.0);

    // 测试超过最大值
    slider.set_value(150.0);
    assert_eq!(slider.value(), 100.0);

    // 测试小于最小值
    slider.set_value(-20.0);
    assert_eq!(slider.value(), 0.0);
}

#[test]
fn test_slider_step() {
    let mut slider = UiSlider::new(0.0, 100.0, 50.0);
    slider.set_step(10.0);

    slider.set_value(55.0);
    // 55 应该被 round 到 60
    assert_eq!(slider.value(), 60.0);
}
```

**验收标准**：Slider 值测试通过

---

### UiInput 测试

#### 光标与输入

```rust
#[test]
fn test_input_cursor_position() {
    let mut input = UiInput::new();
    input.set_text("Hello");

    assert_eq!(input.cursor(), 0); // 默认在开头

    input.set_cursor(3);
    assert_eq!(input.cursor(), 3);

    input.insert_char('X'); // 在位置 3 插入
    assert_eq!(input.text(), "HelXlo");
    assert_eq!(input.cursor(), 4);
}

#[test]
fn test_input_delete() {
    let mut input = UiInput::new();
    input.set_text("Hello");

    input.set_cursor(5);
    input.delete_backward();
    assert_eq!(input.text(), "Hell");

    input.set_cursor(0);
    input.delete_forward();
    assert_eq!(input.text(), "ell");
}

#[test]
fn test_input_selection() {
    let mut input = UiInput::new();
    input.set_text("Hello");
    input.set_cursor(2);
    // 选中 2-4
    input.select_range(2..4);
    assert_eq!(input.select_range(), 2..4);
}
```

#### Tab 切换

```rust
#[test]
fn test_input_tab_order() {
    let world = &mut World::new();
    let input1 = world.spawn(UiNode);
    let input2 = world.spawn(UiNode);

    let mut focus = UiFocus::default();
    focus.add_to_tab_order(input1);
    focus.add_to_tab_order(input2);
    focus.set_focus(input1);

    focus.next_tab();
    assert_eq!(focus.current(), Some(input2));

    focus.next_tab();
    assert_eq!(focus.current(), Some(input1)); // 循环
}
```

**验收标准**：Input 测试通过

---

### UiEvent 测试

```rust
#[test]
fn test_event_click() {
    let world = &mut World::new();
    let entity = world.spawn(UiNode);

    let mut writer = UiEventWriter::<UiEvent>::new();
    writer.write(entity, UiEvent::Click(entity));

    let mut reader = UiEventReader::<UiEvent>::new();
    let events = reader.read(world);

    assert_eq!(events.len(), 1);
    match &events[0] {
        UiEvent::Click(e) => assert_eq!(*e, entity),
        _ => panic!("Expected Click event"),
    }
}

#[test]
fn test_event_hover() {
    let world = &mut World::new();
    let entity = world.spawn(UiNode);

    let mut writer = UiEventWriter::<UiEvent>::new();
    writer.write(entity, UiEvent::HoverEnter(entity));

    let mut reader = UiEventReader::<UiEvent>::new();
    let events = reader.read(world);

    assert_eq!(events.len(), 1);
    match &events[0] {
        UiEvent::HoverEnter(e) => assert_eq!(*e, entity),
        _ => panic!("Expected HoverEnter event"),
    }
}
```

**验收标准**：Event 测试通过

---

## 集成测试

### UI 渲染测试

```rust
#[test]
fn test_ui_render_no_crash() {
    let mut app = App::new();
    app.add_plugin(UiPlugin);
    app.add_plugin(RenderPlugin);
    app.add_example(ui_demo);

    // 运行几帧，不崩溃即可
    for _ in 0..100 {
        app.update();
    }
}
```

**验收标准**：渲染无崩溃

---

### UI 输入测试

```rust
#[test]
fn test_ui_input_works() {
    let mut app = App::new();
    app.add_plugin(UiPlugin);
    app.add_example(ui_input);

    // 聚焦输入框
    let input_entity = app.world.query::<&UiInput>().single();

    // 模拟键盘输入
    app.send_input('H');
    app.send_input('e');
    app.send_input('l');
    app.send_input('l');
    app.send_input('o');

    app.update();

    // 读取输入框文本
    let text = app.world.query::<&UiInput>()
        .with_id(input_entity)
        .single()
        .text();

    assert_eq!(text, "Hello");
}
```

**验收标准**：输入测试通过

---

## 代码质量检查

### Clippy

```bash
cargo clippy --workspace -- -D warnings
```

**验收标准**：无 warning

---

###Fmt

```bash
cargo fmt --check --workspace
```

**验收标准**：格式检查通过

---

### Doc

```bash
cargo doc --workspace --no-deps
```

**验收标准**：文档生成成功

---

## CI 要求

| 需求ID | 描述 |
|--------|------|
| 331 | CI 三平台 green |
| 332 | CHANGELOG 记录 0.6.0 |
| 333-337 | README.md 加入各章节 |
| 338 | 公开 API doc comment 覆盖率 100% |
| 339 | `unsafe` 块 <= 2 |
| 340 | 新增 example 工程 >= 10 个 |

---

## 测试执行

### 运行所有测试

```bash
cargo test -p engine-ui
```

### 运行特定测试

```bash
cargo test -p engine-ui uirect
cargo test -p engine-ui anchor
cargo test -p engine-ui layout
```

### 运行集成测试

```bash
cargo test -p engine-ui --test integration
```

---

## 验收标准汇总

- [ ] `cargo test -p engine-ui` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] `cargo doc --workspace --no-deps` 成功
- [ ] CI 三平台 green
- [ ] CHANGELOG 记录 0.6.0
