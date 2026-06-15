# CI/CD 自动化模块

## 模块概述

CI/CD 模块建立完整的持续集成与持续部署流水线，支持多平台构建（Windows/Linux/macOS/Android/iOS/WebAssembly），实现自动测试、发布与多渠道包分发。

**周期**: 4 周
**优先级**: P0

---

## 需求清单

### 1. CI/CD 基础设施（需求 212-215, 239-263, 811-853）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 212 | CI/CD：GitHub Actions + Gitea + 自建 Runner | P0 |
| 239 | `.github/workflows/ci.yml` 主 CI 工作流 | P0 |
| 240 | `.github/workflows/release.yml` Release 工作流 | P0 |
| 241 | `.github/workflows/docs.yml` 文档部署工作流 | P0 |
| 242 | `.gitea/workflows/ci.yml` Gitea 备用 CI | P1 |
| 243 | 自建 Runner：Linux x86_64 / macOS arm64 / Windows x86_64 | P1 |
| 252 | CI 触发：push 到 `main` / PR / tag `v*` | P0 |
| 253 | CI 步骤：checkout → toolchain → cache → test → clippy → fmt → doc → audit → deny → build examples → benchmark | P0 |

#### GitHub Actions 工作流

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
  release:
    types: [published]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta, nightly]
      fail-fast: false
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - name: Run tests
        run: cargo test --workspace
      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
      - name: Run fmt
        run: cargo fmt --check --all --manifest-path .
      - name: Build examples
        run: cargo build --workspace --examples

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: stable
      - name: Generate coverage
        run: cargo llvm-cov --workspace
      - name: Upload to codecov
        uses: codecov/codecov-action@v3

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: nightly
      - name: Run benchmarks
        run: cargo bench --workspace
      - name: Compare with baseline
        run: cargo compare-bench
```

---

### 2. 构建矩阵（需求 213-227, 252-262）

```yaml
# 构建矩阵配置
os:
  - ubuntu-latest
  - macos-latest
  - windows-latest

rust:
  - stable
  - beta
  - nightly  # allow-failure

targets:
  # 桌面平台
  - x86_64-unknown-linux-gnu
  - x86_64-pc-windows-msvc
  - x86_64-apple-darwin
  - aarch64-apple-darwin  # Apple Silicon
  
  # 移动平台
  - armv7-linux-androideabi
  - aarch64-linux-android
  - aarch64-apple-ios
  
  # WebAssembly
  - wasm32-unknown-unknown
```

| 需求ID | 描述 | 平台 |
|--------|------|------|
| 213 | Windows x86_64 | x86_64-pc-windows-msvc |
| 214 | Linux x86_64 | x86_64-unknown-linux-gnu |
| 215 | macOS x86_64 | x86_64-apple-darwin |
| 216 | macOS arm64（Apple Silicon） | aarch64-apple-darwin |
| 217 | Android armv7 | armv7-linux-androideabi |
| 218 | Android arm64 | aarch64-linux-android |
| 219 | iOS arm64 | aarch64-apple-ios |
| 220 | WebAssembly | wasm32-unknown-unknown |

---

### 3. 跨平台编译（需求 262, 818-828）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 262 | CI 构建工具：`cross`（跨平台交叉编译） | P0 |
| 818 | CI Android 构建：NDK r26b | P0 |
| 819 | CI iOS 构建：Xcode 15 | P0 |
| 820 | CI WebAssembly 构建：`wasm-bindgen` + `wasm-opt` | P0 |

#### 构建配置

```yaml
# Android NDK 配置
env:
  ANDROID_NDK_HOME: /opt/android-ndk-r26b
  ANDROID_SDK_ROOT: /opt/android-sdk

# iOS 配置
xcode_version: "15.0"
minimum_ios_version: "14.0"

# WebAssembly 配置
wasm_target: wasm32-unknown-unknown
wasm_bindgen_version: "0.2.87"
wasm_opt_version: "0.114.0"
```

---

### 4. CI 缓存策略（需求 263, 829-831）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 263 | CI 缓存：`cargo cache` / `sccache` | P0 |
| 264 | CI 缓存 key：`os` + `rust-version` + `hash(Cargo.lock)` | P0 |
| 265 | CI 产物保留 30 天 | P1 |

```yaml
# 缓存配置
- name: Setup Rust cache
  uses: Swatinem/rust-cache@v2
  with:
    cache-targets: true

- name: Setup sccache
  uses: mozilla/sccache-action@v0.0.3
  with:
    cache-key: ${{ runner.os }}-rust-${{ hashFiles('**/Cargo.lock') }}

# 缓存清理
- name: Clean cache
  run: cargo cache --autoclean
```

---

### 5. 测试覆盖（需求 253-261, 836-842）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 253 | `cargo test --workspace` 全部通过 | P0 |
| 254 | `cargo clippy --workspace -- -D warnings` 通过 | P0 |
| 255 | `cargo fmt --check --workspace` 通过 | P0 |
| 256 | `cargo doc --workspace --no-deps` 成功 | P0 |
| 257 | 全量集成测试（examples 全部能跑） | P0 |
| 258 | 性能基准测试（criterion.rs） | P1 |
| 259 | fuzz 测试（cargo fuzz） | P1 |
| 260 | Miri 检测未定义行为（Miri） | P1 |
| 261 | ASan 检测内存错误（AddressSanitizer） | P1 |
| 262 | TSan 检测数据竞争（ThreadSanitizer） | P1 |
| 263 | MSan 检测未初始化读取（MemorySanitizer） | P1 |

#### 测试矩阵

```yaml
#  sanitizer 测试
sanitizers:
  - name: address
    flag: -Z sanitizer=address
    frequency: daily
  - name: thread
    flag: -Z sanitizer=thread
    frequency: daily
  - name: memory
    flag: -Z sanitizer=memory
    frequency: weekly  # nightly only

# fuzz 测试
fuzz:
  targets:
    - asset_pkg
    - profile_import
    - json_parser
  max_time: 600  # 10 minutes
  schedule: "0 2 * * *"  # 每天凌晨2点
```

---

### 6. 发布流程（需求 233-238, 778-812, 828-853）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 233 | 发布流程：alpha → beta → rc → stable → LTS | P0 |
| 234 | `alpha` 版本：功能不稳定，API 可能变化 | P0 |
| 235 | `beta` 版本：功能冻结，仅修 bug | P0 |
| 236 | `rc` 版本：发布候选，若两周无严重 bug 即 stable | P0 |
| 237 | `stable` 版本：正式发布 | P0 |
| 238 | `LTS` 版本：长期支持 3 年 | P0 |

#### 发布版本策略

```yaml
# 版本生命周期
releases:
  alpha:
    stability: unstable
    api_change: may_change
    bug_fixes: unlimited
    duration: ~6 weeks
    
  beta:
    stability: feature_frozen
    api_change: frozen
    bug_fixes: bug_only
    duration: ~6 weeks
    
  rc:
    stability: release_candidate
    testing_period: 14 days
    severity_threshold: critical
    
  stable:
    stability: production_ready
    support_period: 6 months
    
  lts:
    stability: production_ready
    support_period: 36 months
    security_patches: yes
    bug_patches: yes
```

#### 版本号规范

```
MAJOR.MINOR.PATCH[-PRE-RELEASE]
例如：
- 1.0.0       stable
- 1.0.0-alpha.1  alpha
- 1.0.0-beta.1   beta
- 1.0.0-rc.1     rc
- 1.0.1       stable patch
```

---

### 7. 发布工作流（需求 262, 795-809, 828-853）

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  # crates.io 发布
  publish-crates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Publish crates
        run: |
          cargo release --level ${{ github.ref_name }}
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}

  # GitHub Release
  create-release:
    runs-on: ubuntu-latest
    needs: publish-crates
    steps:
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            target/release/**/*.exe
            target/release/**/*.dylib
            target/release/**/*.so
          draft: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

  # Docker Hub
  docker:
    runs-on: ubuntu-latest
    needs: create-release
    steps:
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_HUB_USERNAME }}
          password: ${{ secrets.DOCKER_HUB_TOKEN }}
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            engine/engine:latest
            engine/engine:${{ github.ref_name }}

  # Homebrew
  homebrew:
    runs-on: ubuntu-latest
    needs: create-release
    steps:
      - name: Push to Homebrew Tap
        uses: homebrew/actions/push-to-git@master
        with:
          repository: engine/homebrew-tap
          source_branch: main
          target_branch: master

  # Scoop
  scoop:
    runs-on: windows-latest
    needs: create-release
    steps:
      - name: Update Scoop bucket
        run: |
          scoop bucket add engine https://github.com/engine/scoop-bucket
          scoop update

  # Chocolatey
  chocolatey:
    runs-on: windows-latest
    needs: create-release
    steps:
      - name: Pack and push
        run: choco push engine.nuskg
        env:
          CHOCOLATEY_API_KEY: ${{ secrets.CHOCOLATEY_API_KEY }}
```

---

### 8. 包管理器发布（需求 249-258）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 249 | `cargo publish` 自动发布到 crates.io | P0 |
| 250 | GitHub Release 自动上传二进制 | P0 |
| 251 | Docker 镜像构建与推送 | P0 |
| 252 | Homebrew 包发布 | P1 |
| 253 | Scoop 包发布 | P1 |
| 254 | Chocolatey 包发布 | P1 |
| 255 | Flatpak 包发布 | P2 |
| 256 | Snap 包发布 | P2 |

---

### 9. CI 通知与状态（需求 263, 853）

```yaml
# CI 失败通知
- name: Notify on failure
  if: failure()
  uses: slackapi/slack-github-action@v1
  with:
    channel-id: 'CI/CD'
    payload: |
      {
        "text": "CI build failed!",
        "blocks": [
          {
            "type": "section",
            "text": {
              "type": "mrkdwn",
              "text": "*CI Build Failed*\n<${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}|View logs>"
            }
          }
        ]
      }
  env:
    SLACK_BOT_TOKEN: ${{ secrets.SLACK_BOT_TOKEN }}
```

---

## 验收标准

### CI/CD 验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-1 | push 到 main 触发完整 CI | CI 执行 |
| AC-2 | PR 创建触发 CI | CI 执行 |
| AC-3 | tag `v*` 触发 Release 工作流 | CI 执行 |
| AC-4 | 所有平台构建成功 | CI 执行 |
| AC-5 | `cargo test --workspace` 通过 | CI 执行 |
| AC-6 | `cargo clippy` 无警告 | CI 执行 |
| AC-7 | `cargo fmt` 检查通过 | CI 执行 |
| AC-8 | crates.io 发布成功 | 手动验证 |
| AC-9 | GitHub Release 创建成功 | CI 执行 |
| AC-10 | Docker 镜像推送成功 | CI 执行 |

### 构建产物验收

| 平台 | 验收条件 |
|------|----------|
| Windows x64 | .exe / .dll 可执行 |
| Linux x64 | ELF 二进制可执行 |
| macOS x64/arm64 | Mach-O 可执行 |
| Android | APK 可安装 |
| iOS | IPA 可安装 |
| WebAssembly | .wasm 可在浏览器运行 |

---

## 依赖关系

### 外部服务

- GitHub Actions
- Gitea（备用）
- crates.io
- Docker Hub
- Homebrew Tap
- Scoop Bucket
- Chocolatey

### 内部系统

- `engine-core`: 核心库
- `engine-asset-store`: 资源商店
- `engine-template`: 模板管理
- `engine-profiler`: 性能分析
- `engine-docs`: 文档

---

## 优先级说明

- **P0**: 核心功能，MVP 必须包含
- **P1**: 重要功能，下一迭代包含
- **P2**: 增强功能，后续迭代包含
