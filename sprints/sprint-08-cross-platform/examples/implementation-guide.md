# 示例实现指南

## 概述

本文档提供 Sprint 08 中各示例项目的实现指南和参考代码，包括 `build_cli`、`wasm_demo`、`miniapp_demo`、`android_demo` 等示例。

### 需求来源

对应原文档需求编号：**149-151, 175-179, 355-378**

---

## 1. examples/build_cli

### 需求

- REQ-149: `examples/build_cli`：`engine build --target android --profile release`
- REQ-175: `examples/build_cli`：CLI 示例
- REQ-355: `examples/build_cli` 打印帮助
- REQ-356: `examples/build_cli` 在本机构建 debug

### 项目结构

```
examples/build_cli/
├── Cargo.toml
└── src/
    └── main.rs
```

### Cargo.toml

```toml
[package]
name = "build_cli"
version = "0.8.0"
edition = "2021"

[dependencies]
engine-build = { path = "../../crates/engine-build" }
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
```

### main.rs

```rust
use anyhow::Result;
use clap::Parser;
use engine_build::{BuildConfig, BuildPipeline, PlatformTarget, Profile};

#[derive(Parser)]
#[command(name = "engine")]
#[command(version = "0.8.0")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// 构建项目
    Build {
        /// 目标平台
        #[arg(short, long)]
        target: Option<String>,
        
        /// 构建配置
        #[arg(short, long, default_value = "debug")]
        profile: String,
        
        /// 配置文件路径
        #[arg(short, long)]
        config: Option<String>,
    },
    /// 清理构建产物
    Clean,
    /// 打印帮助信息
    Help,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Command::Build { target, profile, config } => {
            let config = if let Some(config_path) = config {
                BuildConfig::from_file(&config_path)?
            } else {
                BuildConfig::default()
            };
            
            let target = target
                .map(|t| parse_target(&t))
                .unwrap_or_else(PlatformTarget::current);
            
            let profile = parse_profile(&profile);
            
            let config = config
                .with_output_dir("./target")
                .with_temp_dir("./target/.tmp");
            
            let pipeline = BuildPipeline::new(config)?;
            
            println!("Building for {} with {:?} profile...", target, profile);
            let artifact = pipeline.build()?;
            
            println!("Build successful!");
            println!("  Artifact: {:?}", artifact.path());
            println!("  Size: {} bytes", artifact.size());
        }
        Command::Clean => {
            println!("Cleaning build artifacts...");
            // 实现清理逻辑
        }
        Command::Help => {
            println!("Engine Build CLI v0.8.0");
            println!();
            println!("Usage:");
            println!("  engine build --target <target> --profile <profile>");
            println!("  engine clean");
            println!();
            println!("Targets:");
            println!("  windows, macos, linux, android, ios, web, miniapp");
            println!();
            println!("Profiles:");
            println!("  debug, release, ship");
        }
    }
    
    Ok(())
}

fn parse_target(s: &str) -> PlatformTarget {
    match s.to_lowercase().as_str() {
        "windows" | "win" => PlatformTarget::Windows,
        "macos" | "mac" | "osx" => PlatformTarget::MacOS,
        "linux" => PlatformTarget::Linux,
        "android" | "arm" => PlatformTarget::Android,
        "ios" => PlatformTarget::Ios,
        "web" | "wasm" => PlatformTarget::Web,
        _ => PlatformTarget::current(),
    }
}

fn parse_profile(s: &str) -> Profile {
    match s.to_lowercase().as_str() {
        "debug" => Profile::Debug,
        "release" => Profile::Release,
        "ship" => Profile::Ship,
        _ => Profile::Debug,
    }
}
```

### 运行示例

```bash
# 打印帮助
cargo run --example build_cli -- help

# 在本机构建 debug
cargo run --example build_cli -- build --profile debug

# 指定目标构建
cargo run --example build_cli -- build --target android --profile release
```

### 验收标准

- [ ] `--help` 打印帮助信息
- [ ] `build --profile debug` 在本机构建成功
- [ ] CLI 参数正确解析

---

## 2. examples/wasm_demo

### 需求

- REQ-149: `examples/wasm_demo`：Web 版 hello world
- REQ-150: `examples/wasm_demo`：`engine build --target web --profile release` 成功生成 WASM + HTML
- REQ-151: `examples/miniapp_demo`：微信小游戏示例（需开发者工具导入）
- REQ-152: `examples/android_demo`：Android 示例（需 Android Studio / adb）
- REQ-357: `examples/wasm_demo` 生成 WASM 产物
- REQ-353: `examples/wasm_demo` 正常运行
- REQ-354: `examples/wasm_demo` 中 WebGL 能绘制精灵

### 项目结构

```
examples/wasm_demo/
├── Cargo.toml
├── src/
│   └── lib.rs
├── index.html
└── pkg/                  # wasm-pack 输出目录
    ├── wasm_demo.js
    └── wasm_demo_bg.wasm
```

### Cargo.toml

```toml
[package]
name = "wasm_demo"
version = "0.8.0"
edition = "2021"
crate-type = ["cdylib"]

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [
    "Document",
    "Element",
    "HtmlElement",
    "Node",
    "Window",
    "console",
    "ImageData",
    "CanvasRenderingContext2d",
] }

[profile.release]
opt-level = "s"
lto = true
```

### lib.rs

```rust
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

/// 初始化 WebGL 上下文
#[wasm_bindgen]
pub fn init_canvas(canvas_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let _ctx: CanvasRenderingContext2d = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()?
        .get_context("2d")?
        .unwrap()
        .dyn_into()?;
    
    Ok(())
}

/// 绘制精灵
#[wasm_bindgen]
pub fn draw_sprite(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: &str,
) -> Result<(), JsValue> {
    ctx.begin_path();
    ctx.set_fill_style_str(color);
    ctx.fill_rect(x, y, width, height);
    ctx.stroke();
    Ok(())
}

/// 清除画布
#[wasm_bindgen]
pub fn clear_canvas(ctx: &CanvasRenderingContext2d, r: u8, g: u8, b: u8) -> Result<(), JsValue> {
    let width = ctx.canvas().unwrap().width() as f64;
    let height = ctx.canvas().unwrap().height() as f64;
    
    ctx.begin_path();
    ctx.set_fill_style_str(&format!("rgb({},{},{})", r, g, b));
    ctx.fill_rect(0.0, 0.0, width, height);
    Ok(())
}
```

### index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WASM Demo</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            background: #1a1a2e;
        }
        canvas {
            border: 2px solid #16213e;
            border-radius: 8px;
        }
    </style>
</head>
<body>
    <canvas id="game" width="800" height="600"></canvas>
    <script type="module">
        import init, { init_canvas, draw_sprite, clear_canvas } from './pkg/wasm_demo.js';
        
        async function run() {
            await init();
            
            const canvas = document.getElementById('game');
            const ctx = canvas.get_context('2d');
            
            // 清除画布为深蓝色
            clear_canvas(ctx, 26, 26, 46);
            
            // 绘制一个红色精灵
            draw_sprite(ctx, 100, 100, 64, 64, 'rgb(239, 69, 104)');
            
            // 绘制一个绿色精灵
            draw_sprite(ctx, 200, 200, 64, 64, 'rgb(46, 213, 115)');
            
            console.log('WASM Demo initialized successfully!');
        }
        
        run();
    </script>
</body>
</html>
```

### 构建命令

```bash
# 使用 wasm-pack 构建
wasm-pack build --target web --out-dir pkg

# 或使用 cargo 构建为 WASM
cargo build --target wasm32-unknown-unknown --release
```

### 运行方式

```bash
# 启动本地服务器
python -m http.server 8080

# 浏览器访问
# http://localhost:8080/examples/wasm_demo/
```

### 验收标准

- [ ] WASM 产物正确生成
- [ ] 浏览器可加载并运行
- [ ] WebGL/Canvas 能绘制精灵
- [ ] WASM 产物 < 5MB（release）
- [ ] 启动时间 < 1s

---

## 3. examples/miniapp_demo

### 需求

- REQ-358: `examples/miniapp_demo` 生成小程序工程

### 项目结构

```
examples/miniapp_demo/
├── Cargo.toml
├── src/
│   └── lib.rs
├── wechat/                    # 微信小程序
│   ├── app.js
│   ├── app.json
│   ├── app.wxss
│   ├── pages/
│   │   └── index/
│   │       ├── index.js
│   │       ├── index.wxml
│   │       └── index.wxss
│   └── game.js
└── bytedance/                  # 抖音小程序
    ├── app.js
    ├── app.json
    ├── pages/
    │   └── index/
    │       ├── index.js
    │       ├── index.ttml
    │       └── index.ttss
    └── game.js
```

### lib.rs

```rust
use wasm_bindgen::prelude::*;

/// 小程序入口
#[wasm_bindgen]
pub fn on_game_create() {
    #[cfg(target_arch = "wasm32")]
    {
        // 初始化游戏逻辑
    }
}

/// 游戏循环
#[wasm_bindgen]
pub fn on_game_update(delta_time: f32) {
    // 更新游戏状态
}

/// 渲染
#[wasm_bindgen]
pub fn on_game_render(ctx: &mut CanvasContext) {
    // 渲染画面
}

/// 触摸事件
#[wasm_bindgen]
pub fn on_touch_start(x: f32, y: f32) {
    // 处理触摸开始
}

#[wasm_bindgen]
pub fn on_touch_move(x: f32, y: f32) {
    // 处理触摸移动
}

#[wasm_bindgen]
pub fn on_touch_end() {
    // 处理触摸结束
}
```

### wechat/app.js

```javascript
const game = require('./game.js');

App({
    onLaunch: function() {
        console.log('MiniApp Demo launched');
        game.on_game_create();
    },
    onShow: function() {
        // 游戏显示时启动循环
        this.gameLoop();
    },
    onHide: function() {
        // 游戏隐藏时暂停
    },
    gameLoop: function() {
        const update = () => {
            game.on_game_update(16.67); // ~60fps
            requestAnimationFrame(update);
        };
        update();
    }
});
```

### wechat/app.json

```json
{
    "pages": [
        "pages/index/index"
    ],
    "window": {
        "backgroundTextStyle": "light",
        "navigationBarBackgroundColor": "#1a1a2e",
        "navigationBarTitleText": "MiniApp Demo",
        "navigationBarTextStyle": "white"
    }
}
```

### wechat/pages/index/index.js

```javascript
const game = require('../../game.js');

Page({
    onLoad: function() {
        console.log('Index page loaded');
    },
    onReady: function() {
        // 获取 Canvas context
        const query = wx.createSelectorQuery();
        query.select('#gameCanvas')
            .fields({ node: true, context: true })
            .exec((res) => {
                this.canvas = res[0].node;
                this.ctx = this.canvas.getContext('2d');
                this.startGameLoop();
            });
    },
    startGameLoop: function() {
        const loop = () => {
            game.on_game_update(16.67);
            loop();
        };
        loop();
    },
    onTouchStart: function(e) {
        const touch = e.touches[0];
        game.on_touch_start(touch.x, touch.y);
    },
    onTouchMove: function(e) {
        const touch = e.touches[0];
        game.on_touch_move(touch.x, touch.y);
    },
    onTouchEnd: function() {
        game.on_touch_end();
    }
});
```

### 构建命令

```bash
# 生成微信小程序
engine build --target miniapp --miniapp-platform wechat

# 生成抖音小程序
engine build --target miniapp --miniapp-platform bytedance
```

### 验收标准

- [ ] 微信小程序工程正确生成
- [ ] 抖音小程序工程正确生成
- [ ] 小程序可导入开发者工具

---

## 4. examples/android_demo

### 需求

- REQ-359: `examples/android_demo` 生成 APK

### 项目结构

```
examples/android_demo/
├── Cargo.toml
├── src/
│   └── lib.rs
├── android/
│   ├── app/
│   │   ├── build.gradle
│   │   └── src/
│   │       └── main/
│   │           ├── AndroidManifest.xml
│   │           ├── java/
│   │           │   └── com/
│   │           │       └── example/
│   │           │           └── demo/
│   │           │               └── MainActivity.kt
│   │           └── res/
│   │               ├── drawable/
│   │               └── values/
│   └── gradle/
└── build.sh
```

### src/lib.rs

```rust
use ndk_glue::NativeActivity;

#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut NativeActivity,
    saved_state: *mut std::ffi::c_void,
    saved_state_size: usize,
) {
    // 初始化游戏引擎
    // 进入游戏循环
}
```

### build.sh

```bash
#!/bin/bash
set -e

# 构建 Rust 库
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi build --release

# 复制 .so 文件到 Android 项目
mkdir -p android/app/src/main/jniLibs/arm64-v8a
mkdir -p android/app/src/main/jniLibs/armeabi-v7a

cp target/aarch64-linux-android/release/libdemo.so android/app/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libdemo.so android/app/src/main/jniLibs/armeabi-v7a/

# 构建 APK
cd android
./gradlew assembleRelease
cd ..

echo "APK generated at: android/app/build/outputs/apk/release/app-release.apk"
```

### 构建命令

```bash
# 需要安装 Android NDK
export ANDROID_NDK_HOME=/path/to/ndk

# 运行构建脚本
./android/build.sh

# 或使用 engine CLI
engine build --target android --profile release
```

### 验收标准

- [ ] APK 正确生成
- [ ] APK 可安装到设备
- [ ] APK 可运行

---

## 5. 其他示例项目要求

### 示例数量要求

根据 REQ-200：

> 新增 example 工程 >= 4 个

已实现的示例：

1. `examples/build_cli` - CLI 示例
2. `examples/wasm_demo` - Web 示例
3. `examples/miniapp_demo` - 小程序示例
4. `examples/android_demo` - Android 示例

### 示例项目通用规范

每个示例项目应包含：

1. `Cargo.toml` - 项目配置
2. `src/lib.rs` 或 `src/main.rs` - 源代码
3. `README.md` - 使用说明（可选）
4. 构建脚本（如需要）

### 公开 API doc comment 覆盖率

根据 REQ-197：

> 公开 API doc comment 覆盖率 100%

所有公开函数、结构体必须包含文档注释。

---

## 6. unsafe 块限制

根据 REQ-198：

> 本 Sprint `unsafe` 块 <= 5（主要调用外部工具）

### 允许的 unsafe 使用场景

1. 调用外部工具（NDK, Xcode, etc.）
2. FFI 调用
3. 直接操作原始指针（资源处理）

### 禁止的 unsafe 使用场景

1. 绕过借用检查
2. 原始内存分配
3. 未检查的类型转换

### 示例：安全的外部工具调用

```rust
use std::process::Command;

pub fn run_ndk_tool(tool: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(tool)
        .args(args)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow::anyhow!(
            "Tool {} failed: {}",
            tool,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
```

---

## 7. README 章节要求

根据 REQ-167~169, REQ-372~375：

> README.md 加入「跨平台打包」章节
> README.md 加入「资源管线」章节
> README.md 加入「CLI 使用指南」章节
> README.md 加入「构建产物部署」章节

### README.md 模板

```markdown
# Engine Build System

## 跨平台打包

本引擎支持以下平台的构建：

| 平台 | 产物 | 命令 |
|------|------|------|
| Windows | .exe | `engine build --target windows` |
| macOS | .app | `engine build --target macos` |
| Linux | Binary | `engine build --target linux` |
| Android | .apk | `engine build --target android` |
| iOS | .ipa | `engine build --target ios` |
| Web | WASM | `engine build --target web` |
| 小程序 | .zip | `engine build --target miniapp` |

## 资源管线

资源处理管线负责：

- 扫描资源目录
- 导入和处理资源
- 压缩和加密
- 生成资源清单

## CLI 使用指南

```bash
# 创建工程
engine new my-game

# 构建
engine build --target <target> --profile <profile>

# 运行
engine run --profile <profile>

# 打包
engine package --target <target> --output <dir>

# 热更新
engine hot-update --from <v1> --to <v2> --output <patch>
```

## 构建产物部署

### Android
1. 签名 APK
2. 上传到应用市场

### iOS
1. 签名 IPA
2. 通过 TestFlight 或 App Store 分发

### Web
1. 部署到 CDN
2. 配置 Service Worker

### 小程序
1. 导入开发者工具
2. 提交审核
```

---

## 8. 验收清单

- [ ] `examples/build_cli` 正确运行
- [ ] `examples/wasm_demo` 正确运行
- [ ] `examples/miniapp_demo` 正确生成
- [ ] `examples/android_demo` 正确生成 APK
- [ ] 所有示例 unsafe 块 <= 5
- [ ] README 包含所有必需章节
