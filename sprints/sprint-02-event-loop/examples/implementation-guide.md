# 示例实现指南

> 本文档提供 `examples/` 目录下各个示例的实现指南和代码模板。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 91-118, 230-240, 253-256, 261, 267-280

---

## 示例列表

| 示例名称 | 需求编号 | 描述 | 优先级 |
|---------|---------|------|--------|
| window_basic | 91, 230 | 创建默认窗口 | P0 |
| window_fullscreen | 94, 231 | F 键切换全屏 | P0 |
| window_custom | 232 | 自定义标题和尺寸 | P1 |
| input_keys | 92, 233 | 打印键盘事件 | P0 |
| input_mouse | 93, 234 | 打印鼠标事件 | P0 |
| input_touch | 235 | 触摸输入（Web/移动端） | P2 |
| input_gamepad | 236 | 手柄输入（预留） | P2 |
| input_text | 237 | 文本输入（预留） | P2 |
| dpi | 95, 238 | DPI 变化感知 | P1 |
| multi_window | 239 | 多窗口（预留） | P2 |
| event_loop_proxy | 240 | 跨线程发送事件 | P1 |

---

## window_basic — 创建默认窗口

**需求**: `examples/window_basic` — 创建窗口并在 3 秒后自动退出

### 实现指南

```rust
// examples/window_basic.rs

use engine_window::{EventLoop, WindowBuilder, ControlFlow, Event};

fn main() {
    // 1. 创建 EventLoop
    let event_loop = EventLoop::new();
    
    // 2. 构建窗口
    let window = WindowBuilder::new()
        .with_title("Window Basic Example")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build()
        .unwrap();
    
    // 3. 运行事件循环
    let start_time = std::time::Instant::now();
    
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window closed, exiting...");
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::RedrawRequested => {
                        // 渲染逻辑（当前为空白）
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                // 每帧逻辑
                if start_time.elapsed().as_secs() >= 3 {
                    println!("3 seconds elapsed, closing...");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
```

### 验收标准

- [ ] 窗口正常弹出
- [ ] 窗口标题为 "Window Basic Example"
- [ ] 3 秒后自动退出

---

## window_fullscreen — 全屏切换

**需求**: `examples/window_fullscreen` — 按 F 键切换全屏

### 实现指南

```rust
// examples/window_fullscreen.rs

use engine_window::{
    EventLoop, WindowBuilder, ControlFlow, Event, WindowEvent,
    KeyCode, Fullscreen, CursorIcon,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fullscreen Example")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build()
        .unwrap();
    
    let mut is_fullscreen = false;
    
    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { keycode, state, .. } => {
                    // 按 F 键切换全屏
                    if keycode == Some(KeyCode::F) && state == ElementState::Pressed {
                        is_fullscreen = !is_fullscreen;
                        let mode = if is_fullscreen {
                            Fullscreen::Borderless(None)
                        } else {
                            Fullscreen::None
                        };
                        window.set_fullscreen(mode);
                    }
                }
                _ => {}
            }
        }
    });
}
```

### 验收标准

- [ ] 初始为窗口模式
- [ ] 按 F 键进入全屏
- [ ] 再次按 F 键退出全屏

---

## window_custom — 自定义窗口

**需求**: `examples/window_custom` — 1280x720 + 自定义标题

### 实现指南

```rust
// examples/window_custom.rs

use engine_window::{EventLoop, WindowBuilder, ControlFlow, Event};

fn main() {
    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("My Custom Game Window")
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_min_inner_size(LogicalSize::new(640, 480))
        .with_max_inner_size(LogicalSize::new(1920, 1080))
        .with_resizable(true)
        .with_decorations(true)
        .with_transparent(false)
        .with_visible(true)
        .build()
        .unwrap();
    
    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { window_id, event } = event {
            if matches!(event, WindowEvent::CloseRequested) {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
```

### 验收标准

- [ ] 窗口尺寸为 1280x720
- [ ] 标题为 "My Custom Game Window"
- [ ] 窗口可调整大小
- [ ] 最小尺寸 640x480，最大尺寸 1920x1080

---

## input_keys — 键盘输入

**需求**: `examples/input_keys` — 实时打印按键事件

### 实现指南

```rust
// examples/input_keys.rs

use engine_window::{
    EventLoop, WindowBuilder, ControlFlow, Event, WindowEvent,
    KeyCode, ElementState, Input, InputModule,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Input Keys Example")
        .build()
        .unwrap();
    
    let mut input_module = InputModule::new();
    
    event_loop.run(move |event, control_flow| {
        // 处理事件
        input_module.process_event(&event);
        
        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { keycode, state, .. } => {
                    if let Some(key) = keycode {
                        let state_str = match state {
                            ElementState::Pressed => "Pressed",
                            ElementState::Released => "Released",
                        };
                        println!("Key {:?} {}", key, state_str);
                    }
                }
                WindowEvent::ReceivedCharacter(c) => {
                    println!("Received character: '{}'", c);
                }
                _ => {}
            }
        }
        
        // 每帧清除瞬时状态
        if matches!(event, Event::AboutToWait) {
            input_module.input_mut().clear();
        }
    });
}
```

### 验收标准

- [ ] 按键按下时打印 "Key KeyA Pressed"
- [ ] 按键释放时打印 "Key KeyA Released"
- [ ] 字符输入时打印 "Received character: 'a'"

---

## input_mouse — 鼠标输入

**需求**: `examples/input_mouse` — 实时打印鼠标位置和按钮

### 实现指南

```rust
// examples/input_mouse.rs

use engine_window::{
    EventLoop, WindowBuilder, ControlFlow, Event, WindowEvent,
    MouseButton, ElementState, InputModule,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Input Mouse Example")
        .build()
        .unwrap();
    
    let mut input_module = InputModule::new();
    
    event_loop.run(move |event, control_flow| {
        input_module.process_event(&event);
        
        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    println!("Mouse position: ({}, {})", position.x, position.y);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    let button_str = match button {
                        MouseButton::Left => "Left",
                        MouseButton::Middle => "Middle",
                        MouseButton::Right => "Right",
                        MouseButton::Other(n) => return,
                    };
                    let state_str = match state {
                        ElementState::Pressed => "Pressed",
                        ElementState::Released => "Released",
                    };
                    println!("Mouse {} {}", button_str, state_str);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    println!("Mouse wheel: {:?}", delta);
                }
                _ => {}
            }
        }
        
        if matches!(event, Event::AboutToWait) {
            input_module.input_mut().clear();
        }
    });
}
```

### 验收标准

- [ ] 鼠标移动时打印位置
- [ ] 鼠标按钮按下/释放时打印
- [ ] 滚轮滚动时打印增量

---

## dpi — DPI 变化感知

**需求**: `examples/dpi` — 打印 DPI 缩放变化

### 实现指南

```rust
// examples/dpi.rs

use engine_window::{
    EventLoop, WindowBuilder, ControlFlow, Event, WindowEvent,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("DPI Example")
        .build()
        .unwrap();
    
    println!("Initial scale factor: {}", window.scale_factor());
    
    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::DpiChanged { scale_factor, new_inner_size } => {
                    println!("DPI changed to {}%", (scale_factor * 100.0) as i32);
                    println!("New inner size: {}x{}", new_inner_size.width, new_inner_size.height);
                }
                _ => {}
            }
        }
    });
}
```

### 验收标准

- [ ] 初始打印缩放因子
- [ ] DPI 变化时打印新值

---

## event_loop_proxy — 跨线程事件

**需求**: `examples/event_loop_proxy` — 跨线程发送事件

### 实现指南

```rust
// examples/event_loop_proxy.rs

use engine_window::{EventLoop, WindowBuilder, ControlFlow, Event};

#[derive(Debug)]
enum CustomEvent {
    DoSomething,
    DoAnotherThing(String),
}

fn main() {
    let mut event_loop = EventLoop::<CustomEvent>::new();
    let proxy = EventLoopProxy::<CustomEvent>::create_proxy(&event_loop);
    
    // 在另一个线程发送事件
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        proxy.send_event(CustomEvent::DoSomething).unwrap();
        proxy.send_event(CustomEvent::DoAnotherThing("Hello".to_string())).unwrap();
    });
    
    event_loop.run(move |event, control_flow| {
        match event {
            Event::UserEvent(e) => {
                println!("Received user event: {:?}", e);
            }
            Event::WindowEvent { window_id, event } => {
                if matches!(event, WindowEvent::CloseRequested) {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
```

### 验收标准

- [ ] 主线程正常接收子线程发送的事件
- [ ] 事件按发送顺序处理

---

## 示例运行命令

```bash
# 运行所有示例
cargo run --example window_basic
cargo run --example window_fullscreen
cargo run --example window_custom
cargo run --example input_keys
cargo run --example input_mouse
cargo run --example dpi
cargo run --example event_loop_proxy
```

---

## README 文档要求

**需求**: README.md 包含快速上手、如何创建窗口、如何处理输入

### README.md 模板

```markdown
# engine-window

游戏引擎窗口与输入系统模块。

## 快速上手

### 创建窗口

```rust
let event_loop = EventLoop::new();
let window = WindowBuilder::new()
    .with_title("My Game")
    .with_inner_size(LogicalSize::new(1280, 720))
    .build()
    .unwrap();

event_loop.run(move |event, control_flow| {
    // 处理事件
});
```

### 处理输入

```rust
let mut input_module = InputModule::new();

event_loop.run(move |event, control_flow| {
    input_module.process_event(&event);
    
    if input_module.input().key_pressed(KeyCode::KeyW) {
        // W 键正在被按住
    }
});
```

## 示例

- `window_basic` - 基础窗口
- `window_fullscreen` - 全屏切换
- `input_keys` - 键盘输入
- `input_mouse` - 鼠标输入
- `dpi` - DPI 感知
```

---

## 验收标准

| 验收项 | 标准 |
|-------|------|
| 示例数量 | >= 10 个示例 |
| 可运行 | 所有示例 `cargo run --example <name>` 成功 |
| README | 包含快速上手章节 |
| README | 包含如何创建窗口章节 |
| README | 包含如何处理输入章节 |
