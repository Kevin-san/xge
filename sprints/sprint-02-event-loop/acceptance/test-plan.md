# 测试计划

> 本文档定义 Sprint 02 的测试策略、测试用例和验收标准。

**需求来源**: Sprint 02 · 事件循环 / 窗口 / 输入原语
**对应需求编号**: 99-101, 120, 122, 241-260, 282-308

---

## 测试策略

### 单元测试

- **目标**: `cargo test -p engine-window` 全部通过
- **覆盖率要求**: 公开 API 100% 覆盖
- **测试数量**: >= 10 条

### 集成测试

- **目标**: examples 目录下所有示例可运行
- **平台**: Windows / macOS / Linux
- **CI**: 三平台 green

### 代码质量

- **Clippy**: 无 warning
- **Fmt**: 检查通过
- **Doc**: `cargo doc --open` 正常生成

---

## 单元测试用例

### WindowBuilder 测试

**需求**: 单元测试 WindowBuilder 设置/最小/最大 尺寸

```rust
#[test]
fn test_window_builder_default() {
    let builder = WindowBuilder::new();
    // 默认值验证
}

#[test]
fn test_window_builder_with_inner_size() {
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1280, 720))
        .build()
        .unwrap();
    
    assert_eq!(window.inner_size().width, 1280);
    assert_eq!(window.inner_size().height, 720);
}

#[test]
fn test_window_builder_min_max_size() {
    let window = WindowBuilder::new()
        .with_min_inner_size(LogicalSize::new(640, 480))
        .with_max_inner_size(LogicalSize::new(1920, 1080))
        .build()
        .unwrap();
    
    // 验证窗口无法resize到限制外
}
```

**验收**: 
- [ ] `test_window_builder_default` 通过
- [ ] `test_window_builder_with_inner_size` 通过
- [ ] `test_window_builder_min_max_size` 通过

### Input clear vs reset 测试

**需求**: 单元测试 Input clear/reset 的区别

```rust
#[test]
fn test_input_clear_preserves_pressed() {
    let mut input = Input::new();
    
    // 模拟按下 W 键
    input.key_just_pressed(KeyCode::KeyW);
    input.key_pressed(KeyCode::KeyW, true);
    
    input.clear();
    
    // clear 后 just_pressed 被清除，但 pressed 保留
    assert!(!input.key_just_pressed(KeyCode::KeyW));
    assert!(input.key_pressed(KeyCode::KeyW));
}

#[test]
fn test_input_reset_clears_all() {
    let mut input = Input::new();
    
    // 模拟按下 W 键
    input.key_pressed(KeyCode::KeyW, true);
    input.mouse_pressed(MouseButton::Left, true);
    
    input.reset();
    
    // reset 后所有状态被清除
    assert!(!input.key_pressed(KeyCode::KeyW));
    assert!(!input.mouse_pressed(MouseButton::Left));
}
```

**验收**: 
- [ ] `test_input_clear_preserves_pressed` 通过
- [ ] `test_input_reset_clears_all` 通过

### 键盘状态测试

**需求**: 单元测试 key_pressed / key_just_pressed

```rust
#[test]
fn test_key_pressed() {
    let mut input = Input::new();
    
    // 初始状态
    assert!(!input.key_pressed(KeyCode::KeyA));
    
    // 按下
    input.key_pressed(KeyCode::KeyA, true);
    assert!(input.key_pressed(KeyCode::KeyA));
    
    // 释放
    input.key_pressed(KeyCode::KeyA, false);
    assert!(!input.key_pressed(KeyCode::KeyA));
}

#[test]
fn test_key_just_pressed() {
    let mut input = Input::new();
    
    // 第一次按下
    input.key_pressed(KeyCode::KeyA, true);
    assert!(input.key_just_pressed(KeyCode::KeyA));
    
    // 同一帧再次检查
    assert!(!input.key_just_pressed(KeyCode::KeyA));
    
    // clear 后再按
    input.clear();
    input.key_pressed(KeyCode::KeyA, true);
    assert!(input.key_just_pressed(KeyCode::KeyA));
}

#[test]
fn test_key_just_released() {
    let mut input = Input::new();
    
    // 先按下再释放
    input.key_pressed(KeyCode::KeyA, true);
    input.key_pressed(KeyCode::KeyA, false);
    assert!(input.key_just_released(KeyCode::KeyA));
}
```

**验收**: 
- [ ] `test_key_pressed` 通过
- [ ] `test_key_just_pressed` 通过
- [ ] `test_key_just_released` 通过

### 鼠标状态测试

**需求**: 单元测试 mouse_button_pressed / mouse_button_just_pressed

```rust
#[test]
fn test_mouse_button_pressed() {
    let mut input = Input::new();
    
    input.mouse_pressed(MouseButton::Left, true);
    assert!(input.mouse_pressed(MouseButton::Left));
    assert!(!input.mouse_pressed(MouseButton::Right));
    
    input.mouse_pressed(MouseButton::Left, false);
    assert!(!input.mouse_pressed(MouseButton::Left));
}

#[test]
fn test_mouse_button_just_pressed() {
    let mut input = Input::new();
    
    input.mouse_pressed(MouseButton::Left, true);
    assert!(input.mouse_just_pressed(MouseButton::Left));
    assert!(!input.mouse_just_pressed(MouseButton::Left));
    
    input.clear();
    input.mouse_pressed(MouseButton::Left, true);
    assert!(input.mouse_just_pressed(MouseButton::Left));
}

#[test]
fn test_mouse_position() {
    let mut input = Input::new();
    
    let pos = input.mouse_position();
    // 初始位置可以是 (0, 0) 或上次位置
}
```

**验收**: 
- [ ] `test_mouse_button_pressed` 通过
- [ ] `test_mouse_button_just_pressed` 通过
- [ ] `test_mouse_position` 通过

### 鼠标位置/增量测试

**需求**: 单元测试 mouse_position / delta

```rust
#[test]
fn test_mouse_delta() {
    let mut input = Input::new();
    
    // 初始 delta 为 (0, 0)
    let delta = input.mouse_delta();
    assert_eq!(delta, Vec2::new(0.0, 0.0));
    
    // 模拟鼠标移动
    input.update_mouse_position(100.0, 100.0);
    input.update_mouse_position(150.0, 120.0);
    
    let delta = input.mouse_delta();
    assert_eq!(delta.x, 50.0);
    assert_eq!(delta.y, 20.0);
}
```

**验收**: 
- [ ] `test_mouse_delta` 通过

### EventLoopProxy 测试

**需求**: 单元测试 EventLoopProxy send_event

```rust
#[test]
fn test_event_loop_proxy_send() {
    // 需要模拟 EventLoop 环境
    // 验证 send_event 能正确发送事件
}
```

**验收**: 
- [ ] `test_event_loop_proxy_send` 通过

### EventBus 测试

**需求**: 单元测试 EventBus<WindowEvent>

```rust
#[test]
fn test_event_bus_subscribe() {
    // 验证订阅和事件派发
}

#[test]
fn test_event_bus_multiple_subscribers() {
    // 验证多个订阅者都能收到事件
}
```

**验收**: 
- [ ] `test_event_bus_subscribe` 通过
- [ ] `test_event_bus_multiple_subscribers` 通过

---

## 集成测试

### 示例运行测试

| 示例 | 测试命令 | 验收标准 |
|------|---------|---------|
| window_basic | `cargo run --example window_basic` | 窗口弹出并 3 秒后退出 |
| window_fullscreen | `cargo run --example window_fullscreen` | F 键切换全屏 |
| input_keys | `cargo run --example input_keys` | 打印按键事件 |
| input_mouse | `cargo run --example input_mouse` | 打印鼠标事件 |
| dpi | `cargo run --example dpi` | 打印 DPI 变化 |

### 跨平台测试

**需求**: CI 三平台 green

| 平台 | CI 配置 |
|------|---------|
| Linux | GitHub Actions ubuntu-latest |
| macOS | GitHub Actions macos-latest |
| Windows | GitHub Actions windows-latest |

### Headless 测试

**需求**: `--no-window` 模式

```bash
cargo test --no-window
```

---

## 代码质量检查

### Clippy

```bash
cargo clippy -p engine-window
```

**验收**: 无 warning

### Fmt

```bash
cargo fmt --check
```

**验收**: 检查通过

### Doc

```bash
cargo doc -p engine-window --no-deps
```

**验收**: 正常生成文档

---

## 验收检查清单

### 功能验收

- [ ] `cargo run --example window_basic` 弹出窗口并正常退出
- [ ] `cargo run --example input_keys` 实时打印按键
- [ ] `cargo run --example fullscreen` F 键切换全屏
- [ ] 所有 example 在 Windows/macOS/Linux 运行成功

### 单元测试验收

- [ ] `cargo test -p engine-window` 全部通过
- [ ] 单元测试覆盖 WindowBuilder
- [ ] 单元测试覆盖 Input clear/reset
- [ ] 单元测试覆盖 key_pressed / key_just_pressed
- [ ] 单元测试覆盖 mouse_button_pressed / mouse_button_just_pressed
- [ ] 单元测试覆盖 mouse_position / delta
- [ ] 单元测试覆盖 EventLoopProxy
- [ ] 单元测试覆盖 EventBus

### 代码质量验收

- [ ] clippy 无 warning
- [ ] fmt 检查通过
- [ ] cargo doc 成功
- [ ] 本 Sprint 结束 `unsafe` <= 3

### 文档验收

- [ ] 中文注释与英文注释并存
- [ ] Window API 所有公开项都有 doc comment
- [ ] Input API 所有公开项都有 doc comment
- [ ] 示例 10 个以上
- [ ] README.md 包含快速上手
- [ ] README.md 含有「如何创建窗口」章节
- [ ] README.md 含有「如何处理输入」章节

### 其他

- [ ] CHANGELOG 已更新
- [ ] 本 Sprint 公开 API <= 50
- [ ] 本 Sprint 公开函数 doc comment 覆盖率 100%
- [ ] 本 Sprint 新增文档 >= 200 行
- [ ] 本 Sprint 新增 example 工程 >= 10 个

---

## 测试时间表

| 阶段 | 时间 | 内容 |
|------|------|------|
| 单元测试 | 第 1 周 | 完成核心模块单元测试 |
| 集成测试 | 第 2 周 | 示例开发和跨平台测试 |
| 代码质量 | 第 2-3 周 | clippy/fmt/doc 检查 |
| 最终验收 | 第 3 周 | 完整验收清单检查 |

---

## 测试报告模板

```markdown
# Sprint 02 测试报告

## 测试环境
- OS: 
- Rust 版本: 
- 测试时间: 

## 测试结果

### 单元测试
| 测试项 | 状态 | 备注 |
|-------|------|------|
| WindowBuilder | PASS/FAIL | |
| Input | PASS/FAIL | |
| ... | ... | |

### 集成测试
| 示例 | 状态 | 备注 |
|------|------|------|
| window_basic | PASS/FAIL | |
| ... | ... | |

### 代码质量
| 检查项 | 状态 |
|--------|------|
| clippy | PASS/FAIL |
| fmt | PASS/FAIL |
| doc | PASS/FAIL |

## 总结
- 总测试用例: X
- 通过: Y
- 失败: Z
- 成功率: X%
```
