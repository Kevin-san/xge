# 构建与 CI 需求

## 模块名称与概述

本模块定义项目的构建系统配置和 CI/CD 流水线要求。包括 Cargo workspace 配置、Rust 工具链固定、代码格式化/检查工具配置，以及三平台矩阵 CI 脚本。

## 需求编号

对应原文档需求编号：1, 2, 3, 25, 26, 59-63, 67, 68, 69, 70, 71-73, 88, 89, 91-93, 126, 127, 266-278, 318-329

## 功能描述

### 1. Cargo Workspace 配置

**成员 crate：**
- `engine-core` — 核心 crate
- `engine-math` — 数学库
- `engine-platform` — 平台抽象
- `engine-log` — 日志系统
- `engine-utils` — 工具库

**配置要求：**
- 统一 `workspace.dependencies` 声明依赖
- 避免版本漂移
- MSRV（Minimum Supported Rust Version）统一策略
- CI 中检查 MSRV

### 2. Rust 工具链

**文件：**
- `rust-toolchain.toml` — 固定 Rust 工具链版本
- `rustfmt.toml` — 统一格式化规则
- `clippy.toml` — 统一 clippy 配置

### 3. 依赖管理

**错误处理：**
- `thiserror` — 错误类型定义
- `anyhow` — 错误处理

**同步原语：**
- `parking_lot` — 替换 std Mutex/RwLock

**其他依赖：**
- `ahash` — 默认 HashMap 哈希器
- `bytemuck` — 字节转换
- `serde` / `serde-json` — 序列化
- `log` / `env_logger` — 日志实现层
- `futures-lite` — 异步支持

### 4. Feature Flags

**定义：**
- `render-vulkan` — Vulkan 渲染后端
- `render-gl` — OpenGL 渲染后端（默认打开）
- `render-webgpu` — WebGPU 渲染后端
- `audio` — 音频系统（默认打开）
- `network` — 网络系统
- `editor` — 编辑器模式

**默认设置：**
- 默认打开 `render-gl + audio`
- WebAssembly 下自动禁用 host-only feature

### 5. 构建信息注入

**通过 build.rs 注入：**
- `ENGINE_VERSION` / `ENGINE_VERSION_MAJOR` / `MINOR` / `PATCH`
- `BUILD_COMMIT_HASH` — git commit hash
- `BUILD_TIMESTAMP` — 构建时间戳

**build.rs 配置：**
- `cargo:rerun-if-changed=build.rs`
- 通过 `git` 命令取 commit hash

### 6. CI 配置

**工具检查：**
- `cargo fmt --check` — 格式化检查
- `cargo clippy -- -D warnings` — 代码检查
- `cargo test --workspace` — 全量测试
- `cargo build --release --workspace` — Release 构建

**三平台矩阵：**
- Linux x86_64
- macOS aarch64
- Windows x64

**缓存：**
- 缓存 target 目录

**覆盖率（可选）：**
- `tarpaulin` 测试覆盖率收集

### 7. 代码规范

**命名规范：**
- snake_case — 函数、变量
- CamelCase — 结构体、枚举
- SCREAMING_SNAKE_CASE — 常量

**文档要求：**
- 每个公开项至少一条 doc comment
- unsafe 代码必须有 SAFETY 注释

**公开 API 限制：**
- 本 Sprint 公开 API 数量 <= 30 个函数

### 8. 其他文件

**Git：**
- `.gitignore` — 忽略 target/.vscode/.idea 等

**文档：**
- `README.md` — 包含运行方法、架构图、MSRV
- `CHANGELOG.md` — 首条版本 0.1.0-dev

**示例：**
- `examples/hello_engine` — 打印引擎版本
- `examples/minimal_app` — 最小可运行 Demo

## API 签名

### 构建信息
```rust
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ENGINE_VERSION_MAJOR: u32;
pub const ENGINE_VERSION_MINOR: u32;
pub const ENGINE_VERSION_PATCH: u32;
pub const BUILD_COMMIT_HASH: &str;
pub const BUILD_TIMESTAMP: &str;
```

### build.rs
```rust
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");

    let commit_hash = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    println!("cargo:rustc-env=BUILD_COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", chrono::Local::now().to_rfc3339());
}
```

## 输入/输出

### CI 构建
- **输入：** 代码提交/PR
- **输出：** 三平台构建结果、测试结果、覆盖率报告

### cargo doc
- **输入：** 代码
- **输出：** HTML 文档（无 warning）

## 验收标准

- [ ] `cargo build --workspace` 成功
- [ ] `cargo test --workspace` 全部通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] CI 三平台矩阵全部 green
- [ ] `cargo doc --workspace --no-deps` 成功生成
- [ ] `cargo doc` 无 warning
- [ ] `cargo clippy` 无 warning
- [ ] 本 Sprint 公开 API <= 30
- [ ] 本 Sprint 单元测试 >= 30
- [ ] `rust-toolchain.toml` 固定工具链
- [ ] `rustfmt.toml` 格式化规则统一
- [ ] `clippy.toml` 检查规则统一
- [ ] CHANGELOG 首条版本 0.1.0-dev
- [ ] README 包含运行方法、架构图、MSRV
- [ ] .gitignore 正确配置
- [ ] build.rs 正确注入构建信息

## 依赖关系

**依赖工具：**
- Rust toolchain
- cargo
- git
- tarpaulin（可选）

**被依赖：**
- 所有 crate

## 优先级

**P0（必须）：**
- Cargo workspace 配置
- rust-toolchain.toml
- CI 三平台矩阵
- fmt/clippy 检查

**P1（重要）：**
- build.rs 构建信息注入
- Feature flags 配置
- README / CHANGELOG

**P2（可选）：**
- tarpaulin 覆盖率收集
- pre-commit 钩子
