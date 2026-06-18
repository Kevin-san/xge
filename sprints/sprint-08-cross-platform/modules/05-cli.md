# 模块五：CLI 工具（Command Line Interface）

## 1. 模块概述

CLI 工具提供命令行接口，用于创建、构建、运行和打包游戏项目。它是用户与构建系统交互的主要方式，支持跨平台构建、多配置切换和热更新等操作。

### 核心职责

- 提供 `engine` 主命令
- 支持子命令：`new`、`build`、`run`、`clean`、`package`、`hot-update`、`doctor`、`info`
- 解析命令行参数
- 调用内部构建管线

### 需求来源

对应原文档需求编号：**117-121, 144-149, 334-343**

---

## 2. engine 主命令

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-117 | `build_cli` 子命令：`engine new` / `engine build` / `engine run` / `engine clean` / `engine package` / `engine hot-update` |
| REQ-342 | `engine --help` / `-h` 帮助信息 |
| REQ-343 | `engine --version` 版本信息 |

### CLI 结构

```
engine
├── new <name>     # 创建新工程
├── build          # 构建项目
├── run            # 运行项目
├── clean          # 清理构建产物
├── package        # 打包项目
├── hot-update     # 生成/应用热更新
├── doctor         # 检测构建环境
└── info           # 打印引擎信息
```

### API 签名

```rust
pub struct EngineCLI {
    name: String,
    version: String,
}

impl EngineCLI {
    pub fn new() -> Self;
    pub fn run(&self, args: Vec<String>) -> Result<()>;
}
```

### 全局参数

| 参数 | 说明 |
|------|------|
| `--help`, `-h` | 显示帮助信息 |
| `--version`, `-V` | 显示版本信息 |
| `--verbose`, `-v` | 详细输出 |
| `--quiet`, `-q` | 静默输出 |

### 输入

- 命令行参数

### 输出

- 命令执行结果

### 验收标准

- [ ] `--help` 显示帮助信息
- [ ] `--version` 显示版本
- [ ] 子命令正确路由

### 依赖关系

- 依赖 `clap` crate
- 依赖 `BuildPipeline`

### 优先级

**P0**

---

## 3. engine new 创建工程

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-339 | `engine new <name>`：创建工程模板 |

### CLI 用法

```bash
# 创建新工程
engine new my-game

# 在指定目录创建
engine new my-game --path ./projects

# 指定模板
engine new my-game --template mobile
```

### API 签名

```rust
pub struct NewCommand {
    name: String,
    path: Option<PathBuf>,
    template: String,
}

impl NewCommand {
    pub fn new(name: String) -> Self;
    pub fn path(mut self, path: impl AsRef<Path>) -> Self;
    pub fn template(mut self, template: &str) -> Self;
    pub fn execute(&self) -> Result<()>;
}
```

### 生成的文件结构

```
my-game/
├── Cargo.toml
├── game.toml              # 游戏配置
├── src/
│   └── lib.rs
├── assets/
│   └── (资源目录)
└── build.toml             # 构建配置
```

### 输入

- 工程名称
- 目标路径
- 模板类型

### 输出

- 工程目录和文件

### 验收标准

- [ ] 正确创建工程目录
- [ ] 正确生成 Cargo.toml
- [ ] 正确生成配置文件
- [ ] 正确创建资源目录

### 依赖关系

- 依赖文件系统

### 优先级

**P0**

---

## 4. engine build 构建项目

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-118 | `engine build --target <target> --profile <profile> --config <toml>` 构建命令 |
| REQ-334 | `engine build --target <target> --profile <profile> --config <toml>` 构建命令（重复） |

### CLI 用法

```bash
# 基本构建
engine build

# 指定目标平台
engine build --target android

# 指定构建配置
engine build --profile release

# 指定配置文件
engine build --config production.toml

# 组合参数
engine build --target android --profile release --config production.toml
```

### API 签名

```rust
pub struct BuildCommand {
    target: Option<PlatformTarget>,
    profile: Option<Profile>,
    config: Option<PathBuf>,
    verbose: bool,
}

impl BuildCommand {
    pub fn new() -> Self;
    pub fn target(mut self, target: PlatformTarget) -> Self;
    pub fn profile(mut self, profile: Profile) -> Self;
    pub fn config(mut self, config: impl AsRef<Path>) -> Self;
    pub fn execute(&self) -> Result<BuildArtifact>;
}
```

### 参数说明

| 参数 | 缩写 | 默认值 | 说明 |
|------|------|--------|------|
| `--target` | `-t` | 当前平台 | 目标平台 |
| `--profile` | `-p` | Debug | 构建配置 |
| `--config` | `-c` | build.toml | 配置文件路径 |

### 支持的平台

- `windows`, `win`
- `macos`, `mac`, `osx`
- `linux`, `linux`
- `android`, `android`, `arm`
- `ios`
- `web`, `wasm`
- `miniapp`, `wechat`, `bytedance`, `qq`

### 支持的配置

- `debug`
- `release`
- `ship`

### 输入

- 命令行参数
- 配置文件

### 输出

- 构建产物路径

### 验收标准

- [ ] 正确识别目标平台
- [ ] 正确识别构建配置
- [ ] 正确加载配置文件
- [ ] 构建成功生成产物

### 依赖关系

- 依赖 `BuildPipeline`
- 依赖 `BuildConfig`

### 优先级

**P0**

---

## 5. engine run 运行项目

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-119 | `engine run --profile <profile>` 运行命令 |
| REQ-335 | `engine run --profile <profile>` 运行命令（重复） |

### CLI 用法

```bash
# 运行项目
engine run

# 指定配置运行
engine run --profile release

# 带参数运行
engine run -- --arg1 value1
```

### API 签名

```rust
pub struct RunCommand {
    profile: Option<Profile>,
    args: Vec<String>,
}

impl RunCommand {
    pub fn new() -> Self;
    pub fn profile(mut self, profile: Profile) -> Self;
    pub fn args(mut self, args: Vec<String>) -> Self;
    pub fn execute(&self) -> Result<()>;
}
```

### 限制

- 仅支持本机目标
- Web 目标启动本地服务器
- Android 目标启动 adb

### 输入

- 构建配置
- 运行参数

### 输出

- 应用启动

### 验收标准

- [ ] 构建并运行成功
- [ ] 参数正确传递
- [ ] 仅本机目标可用

### 依赖关系

- 依赖 `BuildPipeline::run()`

### 优先级

**P0**

---

## 6. engine clean 清理产物

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-336 | `engine clean` 清理命令 |

### CLI 用法

```bash
# 清理所有构建产物
engine clean

# 清理并清理缓存
engine clean --cache
```

### API 签名

```rust
pub struct CleanCommand {
    clean_cache: bool,
}

impl CleanCommand {
    pub fn new() -> Self;
    pub fn clean_cache(mut self) -> Self;
    pub fn execute(&self) -> Result<()>;
}
```

### 清理范围

| 选项 | 清理内容 |
|------|---------|
| 默认 | target/ 目录 |
| `--cache` | target/ + 缓存目录 |

### 输入

- 无

### 输出

- 删除构建产物

### 验收标准

- [ ] 正确删除 target/ 目录
- [ ] 正确删除缓存（如指定）

### 依赖关系

- 依赖文件系统

### 优先级

**P1**

---

## 7. engine package 打包项目

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-120 | `engine package --target <target> --output <dir>` 打包命令 |
| REQ-337 | `engine package --target <target> --output <dir>` 打包命令（重复） |

### CLI 用法

```bash
# 打包为 ZIP
engine package

# 指定目标
engine package --target android

# 指定输出目录
engine package --output ./dist

# 组合
engine package --target android --output ./dist
```

### API 签名

```rust
pub struct PackageCommand {
    target: Option<PlatformTarget>,
    output: Option<PathBuf>,
    format: Option<PackageFormat>,
}

impl PackageCommand {
    pub fn new() -> Self;
    pub fn target(mut self, target: PlatformTarget) -> Self;
    pub fn output(mut self, output: impl AsRef<Path>) -> Self;
    pub fn format(mut self, format: PackageFormat) -> Self;
    pub fn execute(&self) -> Result<BuildArtifact>;
}
```

### 打包格式

| 格式 | 平台 | 扩展名 |
|------|------|--------|
| Dir | 所有 | 目录 |
| Zip | 所有 | .zip |
| Apk | Android | .apk |
| Ipa | iOS | .ipa |
| Wasm | Web | .zip |
| MiniApp | 小程序 | .zip |

### 输入

- 目标平台
- 输出目录
- 打包格式

### 输出

- 打包产物

### 验收标准

- [ ] 正确打包为指定格式
- [ ] 输出到正确目录

### 依赖关系

- 依赖 `Package`

### 优先级

**P1**

---

## 8. engine hot-update 热更新

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-121 | `engine hot-update --from <v1> --to <v2> --output <patch>` 热更新命令 |
| REQ-338 | `engine hot-update --from <v1_manifest> --to <v2_manifest> --output <patch>` 热更新命令（重复） |

### CLI 用法

```bash
# 生成补丁
engine hot-update generate --from v1.0.0 --to v1.1.0 --output ./patch.zip

# 应用补丁
engine hot-update apply --patch ./patch.zip --dir ./assets

# 指定清单文件
engine hot-update generate --from old.manifest --to new.manifest --output ./patch.zip
```

### API 签名

```rust
pub struct HotUpdateCommand {
    action: HotUpdateAction,
    from: String,
    to: String,
    output: Option<PathBuf>,
    patch: Option<PathBuf>,
    dir: Option<PathBuf>,
}

pub enum HotUpdateAction {
    Generate,
    Apply,
}

impl HotUpdateCommand {
    pub fn new(action: HotUpdateAction) -> Self;
    pub fn execute(&self) -> Result<()>;
}
```

### 子命令

| 子命令 | 说明 |
|--------|------|
| `generate` | 生成补丁包 |
| `apply` | 应用补丁包 |

### 输入

- 源版本
- 目标版本
- 输出/输入路径

### 输出

- 补丁文件或应用结果

### 验收标准

- [ ] 正确生成补丁
- [ ] 正确应用补丁

### 依赖关系

- 依赖 `HotUpdate`

### 优先级

**P1**

---

## 9. engine doctor 环境检测

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-340 | `engine doctor`：检测构建环境 |

### CLI 用法

```bash
# 检测构建环境
engine doctor
```

### 输出示例

```
=== Build Environment Doctor ===

[✓] Rust 1.75.0
[✗] Android NDK (not found)
[✓] Node.js 20.10.0
[✓] wasm-bindgen 0.2.87

Platform: Linux x86_64
Target: x86_64-unknown-linux-gnu

Build tools:
  - cargo: 1.75.0
  - rustc: 1.75.0

SDK:
  - Android SDK: /opt/android-sdk
  - Xcode: not installed

Recommendations:
  - Install Android NDK for Android builds
  - Install Xcode for iOS/macOS builds
```

### API 签名

```rust
pub struct DoctorCommand;

impl DoctorCommand {
    pub fn new() -> Self;
    pub fn execute(&self) -> Result<()>;
    pub fn check_toolchain(&self) -> ToolchainCheckResult;
    pub fn print_report(&self, result: &ToolchainCheckResult);
}
```

### 检测项

| 检测项 | 说明 |
|--------|------|
| Rust | rustc/cargo 版本 |
| NDK | Android NDK（目标为 Android 时必需） |
| SDK | Android SDK |
| Xcode | Xcode（目标为 iOS/macOS 时必需） |
| wasm-bindgen | Web 编译必需 |
| wasm-opt | Web 优化（可选） |
| Node.js | 小程序编译必需 |

### 输入

- 无

### 输出

- 环境检测报告

### 验收标准

- [ ] 正确检测所有工具
- [ ] 正确显示检测结果
- [ ] 正确给出建议

### 依赖关系

- 依赖 `Toolchain`

### 优先级

**P1**

---

## 10. engine info 引擎信息

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-341 | `engine info`：打印引擎版本与配置 |

### CLI 用法

```bash
# 打印引擎信息
engine info
```

### 输出示例

```
=== Engine Build System ===

Version: 0.8.0
Commit: a1b2c3d4

Configuration:
  Default target: linux
  Default profile: debug
  Cache dir: ~/.cache/engine-build

Features:
  - android
  - ios
  - web
  - miniapp

Supported targets:
  - windows
  - macos
  - linux
  - android
  - ios
  - web
  - wechat
  - bytedance
  - qq
```

### API 签名

```rust
pub struct InfoCommand;

impl InfoCommand {
    pub fn new() -> Self;
    pub fn execute(&self) -> Result<()>;
    pub fn print_version(&self);
    pub fn print_config(&self);
    pub fn print_features(&self);
}
```

### 输入

- 无

### 输出

- 引擎信息

### 验收标准

- [ ] 正确显示版本信息
- [ ] 正确显示配置信息
- [ ] 正确显示支持的目标

### 依赖关系

- 无

### 优先级

**P2**

---

## 11. 构建产物输出

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-122 | `wasm-bindgen` 支持（Web 目标） |
| REQ-123 | `wasm-opt` 支持（可选） |
| REQ-124 | Web 构建产物：`index.html` + `*.js` + `*.wasm` + 资源 |
| REQ-125 | Web 构建支持 service worker（离线 PWA） |
| REQ-126 | Android 构建产物：`app-debug.apk` / `app-release.apk` |
| REQ-127 | Android 构建支持 `aarch64-linux-android` + `armv7-linux-androideabi` + `x86_64-linux-android` |
| REQ-128 | Android 构建支持动态权限弹窗（运行时权限） |
| REQ-129 | iOS 构建产物：`.app` 或 `.ipa`（仅 macOS 支持） |
| REQ-130 | iOS 构建支持设备/模拟器双 target |
| REQ-131 | Windows 构建产物：`.exe` + 依赖 DLL + 资源目录 |
| REQ-132 | Windows 构建支持 console / window subsystem |
| REQ-133 | Windows 构建支持 icon 嵌入 |
| REQ-134 | macOS 构建产物：`.app` |
| REQ-135 | macOS 构建支持 Info.plist 与签名 |
| REQ-136 | Linux 构建产物：可执行二进制 + 资源目录（或 `.AppImage`，可选） |
| REQ-137 | 微信/抖音/QQ 小程序构建：把 Web 产物适配为小程序 canvas + JS |
| REQ-138 | 小程序构建输出 `miniapp.zip` 或直接输出工程目录 |

### 构建产物对照表

| 平台 | 产物 | 路径 |
|------|------|------|
| Windows | .exe + DLL + 资源 | target/windows/ |
| macOS | .app | target/macos/ |
| Linux | 可执行文件 + 资源 | target/linux/ |
| Android | .apk | target/android/ |
| iOS | .app / .ipa | target/ios/ |
| Web | index.html + JS + WASM | target/web/ |
| 小程序 | .zip / 工程目录 | target/miniapp/ |

### 输入

- 构建配置
- 目标平台

### 输出

- 平台特定产物

### 验收标准

- [ ] Windows .exe 正确生成
- [ ] Android .apk 正确生成
- [ ] iOS .app/.ipa 正确生成
- [ ] Web WASM + HTML 正确生成
- [ ] 小程序工程正确生成

### 依赖关系

- 依赖各平台工具链

### 优先级

**P0**

---

## 12. 优先级汇总

| 优先级 | 需求编号 | 模块 |
|-------|---------|------|
| P0 | REQ-117~120, REQ-122~138, REQ-334~338 | build, run, package, 产物 |
| P1 | REQ-121, REQ-336~341 | clean, hot-update, doctor |
| P2 | REQ-341 | info |

---

## 13. CLI 参数解析示例

### 使用 clap 实现

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "engine")]
#[command(version = "0.8.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    New {
        name: String,
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    Build {
        #[arg(short, long)]
        target: Option<String>,
        #[arg(short, long, default_value = "debug")]
        profile: String,
        #[arg(short, long)]
        config: Option<String>,
    },
    Run {
        #[arg(short, long, default_value = "debug")]
        profile: String,
    },
    Clean {
        #[arg(long)]
        cache: bool,
    },
    Package {
        #[arg(short, long)]
        target: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
    },
    HotUpdate {
        #[command(subcommand)]
        action: HotUpdateActions,
    },
    Doctor,
    Info,
}

#[derive(Subcommand)]
enum HotUpdateActions {
    Generate {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        output: String,
    },
    Apply {
        #[arg(long)]
        patch: String,
        #[arg(long)]
        dir: String,
    },
}
```

---

## 14. 验收清单

- [ ] `engine --help` 显示帮助
- [ ] `engine --version` 显示版本
- [ ] `engine new my-game` 正确创建工程
- [ ] `engine build --target linux --profile debug` 构建成功
- [ ] `engine run --profile release` 构建并运行
- [ ] `engine clean` 正确清理
- [ ] `engine package --target android` 正确打包
- [ ] `engine hot-update generate` 正确生成补丁
- [ ] `engine hot-update apply` 正确应用补丁
- [ ] `engine doctor` 正确检测环境
- [ ] `engine info` 正确显示信息
