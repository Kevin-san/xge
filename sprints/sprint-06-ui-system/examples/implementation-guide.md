# 示例实现指南

## 概述

本文档提供 `engine-ui` crate 各示例的实现指南，帮助开发者理解如何使用 UI 系统构建常见界面。每个示例都应展示特定的 UI 功能。

---

## 示例列表

| 示例名称 | 需求ID | 描述 |
|----------|--------|------|
| ui_demo | 107, 301 | 主菜单（开始/设置/退出） |
| ui_settings | 108, 302 | 设置面板（分辨率/全屏/音量/主题） |
| ui_hud | 109, 303 | 游戏内 HUD（分数/血条/时间） |
| ui_input | 110, 304 | 输入框 + 键盘输入 |
| ui_scroll | 111, 305 | 长列表滚动 |
| ui_richtext | 112, 306 | 富文本展示 |
| ui_layout | 113, 307 | 多种布局展示 |
| ui_animation | 114, 308 | 按钮/面板动画 |
| ui_theme | 115, 309 | 亮色/暗色主题切换 |
| ui_game_menu | 116, 310 | 主菜单 + 游戏内暂停菜单 |

---

## ui_demo - 主菜单

### 功能需求

- [ ] 标题 "Game Title"
- [ ] "Start Game" 按钮
- [ ] "Settings" 按钮
- [ ] "Exit" 按钮
- [ ] 点击 Start Game 打印 "Starting game..."
- [ ] 点击 Settings 切换到设置面板
- [ ] 点击 Exit 退出游戏

### 实现要点

```rust
// 创建画布
fn setup_menu(mut commands: UiCommands) {
    // 主面板居中
    let panel = commands.spawn_panel();
    commands.set_layout_mode(panel, LayoutMode::Vertical);
    commands.set_align(panel, AlignItems::Center);

    // 标题
    let title = commands.spawn_text("Game Title");
    commands.set_font_size(title, 48.0);

    // 开始按钮
    let start_btn = commands.spawn_button("Start Game");
    commands.add_event_handler::<ClickEvent>(start_btn, |world, entity| {
        info!("Starting game...");
    });

    // 设置按钮
    let settings_btn = commands.spawn_button("Settings");
    commands.add_event_handler::<ClickEvent>(settings_btn, |world, entity| {
        // 切换到设置面板
    });

    // 退出按钮
    let exit_btn = commands.spawn_button("Exit");
    commands.add_event_handler::<ClickEvent>(exit_btn, |world, entity| {
        std::process::exit(0);
    });
}
```

### 验收标准

- [ ] 菜单正确居中显示
- [ ] 三个按钮垂直排列
- [ ] 点击各按钮响应正确

---

## ui_settings - 设置面板

### 功能需求

- [ ] 分辨率选择下拉框
- [ ] 全屏切换开关
- [ ] 音量滑块 (0-100)
- [ ] 主题切换按钮（亮/暗）
- [ ] 应用按钮
- [ ] 取消按钮

### 实现要点

```rust
fn setup_settings(mut commands: UiCommands) {
    let panel = commands.spawn_panel();
    commands.set_layout_mode(panel, LayoutMode::Vertical);
    commands.set_gap(panel, 20.0);

    // 分辨率
    let res_label = commands.spawn_text("Resolution:");
    let res_dropdown = commands.spawn_dropdown();
    commands.add_item(res_dropdown, "1920x1080");
    commands.add_item(res_dropdown, "1280x720");
    commands.add_item(res_dropdown, "800x600");

    // 全屏
    let fs_label = commands.spawn_text("Fullscreen:");
    let fs_toggle = commands.spawn_toggle();
    fs_toggle.set_value(false);

    // 音量
    let vol_label = commands.spawn_text("Volume:");
    let vol_slider = commands.spawn_slider(0.0, 100.0, 50.0);
    let vol_value = commands.spawn_text("50%");
    commands.add_event_handler::<ValueChangedSliderEvent>(vol_slider, move |world, entity, value| {
        vol_value.set_text(&format!("{}%", value as i32));
    });

    // 主题
    let theme_btn = commands.spawn_button("Toggle Theme");
    commands.add_event_handler::<ClickEvent>(theme_btn, |world, entity| {
        let theme = world.query::<&mut UiTheme>().single();
        // 切换主题
    });

    // 按钮行
    let btn_panel = commands.spawn_panel();
    commands.set_layout_mode(btn_panel, LayoutMode::Horizontal);
    let apply_btn = commands.spawn_button("Apply");
    let cancel_btn = commands.spawn_button("Cancel");
}
```

### 验收标准

- [ ] 所有控件正确渲染
- [ ] 滑块值实时更新显示
- [ ] 主题切换正常工作

---

## ui_hud - 游戏 HUD

### 功能需求

- [ ] 左上角：分数显示 "Score: X"
- [ ] 右上角：游戏时间 "Time: MM:SS"
- [ ] 底部中央：血条 (0-100%)
- [ ] HUD 元素不阻塞游戏输入

### 实现要点

```rust
fn setup_hud(world: &mut World) {
    // 分数
    let score_panel = commands.spawn_panel();
    commands.set_anchor(score_panel, Anchor::TopLeft);
    commands.set_margin(score_panel, 20.0, 20.0, 0.0, 0.0);
    let score_text = commands.spawn_text("Score: 0");
    score_text.set_font_size(24.0);
    score_text.set_color(Color::YELLOW);

    // 时间
    let time_panel = commands.spawn_panel();
    commands.set_anchor(time_panel, Anchor::TopRight);
    commands.set_margin(time_panel, 0.0, 20.0, 20.0, 0.0);
    let time_text = commands.spawn_text("Time: 00:00");
    time_text.set_font_size(24.0);

    // 血条
    let health_panel = commands.spawn_panel();
    commands.set_anchor(health_panel, Anchor::BottomCenter);
    commands.set_margin(health_panel, 0.0, 0.0, 40.0, 0.0);
    let health_bar = commands.spawn_progress_bar();
    health_bar.set_progress(1.0); // 100%
}
```

### 验收标准

- [ ] HUD 元素正确放置在角落
- [ ] 血条显示正确
- [ ] HUD 不会阻挡游戏操作

---

## ui_input - 输入框

### 功能需求

- [ ] 用户名输入框
- [ ] 密码输入框（显示 *）
- [ ] 数字输入框（仅数字）
- [ ] 多行文本输入框
- [ ] placeholder 显示

### 实现要点

```rust
fn setup_input(mut commands: UiCommands) {
    // 用户名
    let user_input = commands.spawn_input();
    user_input.set_placeholder("Username");
    user_input.set_max_length(Some(32));

    // 密码
    let pass_input = commands.spawn_input();
    pass_input.set_placeholder("Password");
    pass_input.set_password(true);
    pass_input.set_password_char('*');

    // 数字
    let num_input = commands.spawn_input();
    num_input.set_placeholder("Age");
    num_input.set_numeric_mode(true);
    num_input.set_max_length(Some(3));

    // 多行
    let multi_input = commands.spawn_input();
    multi_input.set_placeholder("Description...");
    // 设置多行模式
}
```

### 验收标准

- [ ] 输入框可正常输入
- [ ] 密码模式正确隐藏字符
- [ ] 数字模式仅接受数字
- [ ] placeholder 正确显示

---

## ui_scroll - 滚动面板

### 功能需求

- [ ] 垂直滚动列表
- [ ] 100+ 个列表项
- [ ] 鼠标滚轮滚动
- [ ] 滚动条显示/隐藏

### 实现要点

```rust
fn setup_scroll(mut commands: UiCommands) {
    let scroll_view = commands.spawn_scroll();
    scroll_view.set_direction(Direction::Vertical);

    let content = scroll_view.content();
    commands.set_layout_mode(content, LayoutMode::Vertical);
    commands.set_gap(content, 8.0);

    // 生成 100 个列表项
    for i in 0..100 {
        let item = commands.spawn_panel();
        let text = commands.spawn_text(&format!("Item #{}", i));
        commands.add_child(content, item);
    }
}
```

### 验收标准

- [ ] 可滚动查看所有列表项
- [ ] 滚轮滚动正常工作
- [ ] 滚动条正确显示

---

## ui_richtext - 富文本

### 功能需求

- [ ] 多颜色文本
- [ ] 多字号文本
- [ ] 粗体/斜体文本
- [ ] 换行文本

### 实现要点

```rust
fn setup_richtext(mut commands: UiCommands) {
    let mut rich = RichText::new();

    rich.push(TextSection::new("Red ", TextStyle::default())
        .with_color(Color::RED));
    rich.push(TextSection::new("Green ", TextStyle::default())
        .with_color(Color::GREEN));
    rich.push(TextSection::new("Blue\n", TextStyle::default())
        .with_color(Color::BLUE));

    rich.push(TextSection::new("Large ", TextStyle::default())
        .with_size(32.0));
    rich.push(TextSection::new("Small\n", TextStyle::default())
        .with_size(12.0));

    rich.push(TextSection::new("Bold", TextStyle::default())
        .with_bold());
    rich.push(TextSection::new(" Italic\n", TextStyle::default())
        .with_italic());

    let text = commands.spawn_rich_text(rich);
}
```

### 验收标准

- [ ] 多颜色正确显示
- [ ] 多字号正确显示
- [ ] 粗体/斜体正确显示
- [ ] 换行正确处理

---

## ui_layout - 布局展示

### 功能需求

- [ ] 垂直布局示例
- [ ] 水平布局示例
- [ ] 网格布局示例
- [ ] Flex 布局示例
- [ ] 锚点布局示例

### 实现要点

```rust
fn setup_layout_demos(mut commands: UiCommands) {
    // 垂直布局
    let v_layout = commands.spawn_panel();
    commands.set_layout_mode(v_layout, LayoutMode::Vertical);
    for i in 0..5 {
        commands.spawn_text(&format!("Vertical Item {}", i));
    }

    // 水平布局
    let h_layout = commands.spawn_panel();
    commands.set_layout_mode(h_layout, LayoutMode::Horizontal);
    for i in 0..5 {
        commands.spawn_text(&format!("H Item {}", i));
    }

    // 网格布局
    let grid = commands.spawn_grid(3, 3);
    for i in 0..9 {
        let cell = commands.spawn_text(&format!("Cell {}", i));
        commands.add_child(grid, cell);
    }

    // Flex 布局
    let flex = commands.spawn_panel();
    commands.set_layout_mode(flex, LayoutMode::Flex);
    let flex_item = commands.spawn_text("Flex Item");
    commands.set_flex_grow(flex_item, 1.0);
}
```

### 验收标准

- [ ] 所有布局类型正确渲染
- [ ] 布局嵌套正常工作
- [ ] 布局响应尺寸变化

---

## ui_animation - 动画

### 功能需求

- [ ] 按钮 hover 缩放动画
- [ ] 按钮 press 下沉动画
- [ ] 面板渐显/渐隐动画
- [ ] 面板滑入/滑出动画

### 实现要点

```rust
fn setup_animations(mut commands: UiCommands) {
    let btn = commands.spawn_button("Hover Me");

    // Hover 缩放
    commands.add_animation(btn, Animation::Scale {
        from: Vec2::ONE,
        to: Vec2::new(1.1, 1.1),
        duration: 0.2,
        easing: Easing::EaseOut,
    }).trigger_on(HoverEnter);

    commands.add_animation(btn, Animation::Scale {
        from: Vec2::new(1.1, 1.1),
        to: Vec2::ONE,
        duration: 0.2,
        easing: Easing::EaseOut,
    }).trigger_on(HoverLeave);

    // Press 下沉
    commands.add_animation(btn, Animation::Translate {
        from: Vec2::ZERO,
        to: Vec2::new(0.0, 2.0),
        duration: 0.1,
        easing: Easing::Linear,
    }).trigger_on(Press);

    // 面板动画
    let panel = commands.spawn_panel();
    commands.add_animation(panel, Animation::Fade {
        from: 0.0,
        to: 1.0,
        duration: 0.5,
    }).trigger_on(Show);

    commands.add_animation(panel, Animation::Slide {
        from: Vec2::new(-200.0, 0.0),
        to: Vec2::ZERO,
        duration: 0.3,
        easing: Easing::EaseOut,
    }).trigger_on(Show);
}
```

### 验收标准

- [ ] hover 动画平滑播放
- [ ] press 动画正确触发
- [ ] 渐显渐隐正常工作
- [ ] 滑入滑出正常工作

---

## ui_theme - 主题切换

### 功能需求

- [ ] 亮色主题按钮
- [ ] 暗色主题按钮
- [ ] 主题实时切换
- [ ] 所有控件反映主题变化

### 实现要点

```rust
fn setup_theme(mut commands: UiCommands) {
    let light_btn = commands.spawn_button("Light Theme");
    commands.add_event_handler::<ClickEvent>(light_btn, |world, entity| {
        let theme = UiTheme::default_light();
        world.insert_resource(theme);
    });

    let dark_btn = commands.spawn_button("Dark Theme");
    commands.add_event_handler::<ClickEvent>(dark_btn, |world, entity| {
        let theme = UiTheme::default_dark();
        world.insert_resource(theme);
    });

    // 预览面板
    let preview = commands.spawn_panel();
    preview.set_background_color(theme.panel_bg);
    let preview_btn = commands.spawn_button("Preview Button");
    let preview_text = commands.spawn_text("Preview Text");
}
```

### 验收标准

- [ ] 主题正确切换
- [ ] 所有控件反映新主题
- [ ] 切换无闪烁

---

## ui_game_menu - 游戏菜单

### 功能需求

- [ ] 主菜单界面
- [ ] 游戏内暂停菜单
- [ ] ESC 键切换暂停
- [ ] 暂停时游戏暂停
- [ ] 菜单项：继续/设置/返回主菜单

### 实现要点

```rust
fn setup_game_menu(mut commands: UiCommands) {
    // 主菜单
    let main_menu = commands.spawn_panel();
    main_menu.set_id("main_menu");
    // ... 菜单项

    // 暂停菜单
    let pause_menu = commands.spawn_panel();
    pause_menu.set_id("pause_menu");
    pause_menu.set_visible(false);

    // ESC 切换
    commands.add_input_handler(KeyDown(Escape), |world| {
        let pause = world.query::<&mut UiPanel>()
            .with_id("pause_menu");
        let visible = pause.get_visible();
        pause.set_visible(!visible);
        // 切换游戏暂停状态
    });

    // 继续
    let resume_btn = commands.spawn_button("Resume");
    commands.add_event_handler::<ClickEvent>(resume_btn, |world, entity| {
        // 关闭暂停菜单
    });

    // 设置
    let settings_btn = commands.spawn_button("Settings");
    // ...

    // 返回主菜单
    let main_menu_btn = commands.spawn_button("Main Menu");
    commands.add_event_handler::<ClickEvent>(main_menu_btn, |world, entity| {
        // 切换到主菜单
    });
}
```

### 验收标准

- [ ] 主菜单正常显示
- [ ] ESC 正确切换暂停
- [ ] 暂停时游戏确实暂停
- [ ] 所有菜单按钮正常工作

---

## 示例开发要求

| 需求ID | 要求 |
|--------|------|
| 140 | 本 Sprint 新增 example 工程 >= 10 个 |
| 131 | `cargo test -p engine-ui` 全部通过 |
| 132 | `cargo clippy --workspace -- -D warnings` 通过 |
| 133 | `cargo fmt --check --workspace` 通过 |
| 134 | `cargo doc --workspace --no-deps` 成功 |
