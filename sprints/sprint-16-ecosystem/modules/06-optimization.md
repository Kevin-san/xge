# 性能优化需求

## 模块概述

性能优化模块建立完整的性能基准测试、回归检测与优化指南体系，包括 criterion.rs 基准测试、fuzz 模糊测试、多 sanitizer 检测，确保引擎在各种场景下的性能稳定性。

**周期**: 4 周
**优先级**: P0

---

## 需求清单

### 1. 基准测试（需求 257-258, 284, 837-838, 937-942）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 257 | 性能基准测试（criterion.rs） | P0 |
| 284 | `cargo bench --workspace` 成功 | P0 |
| 837 | CI 基准测试：`criterion` 每次 main 分支 commit 跑基准 | P0 |
| 838 | CI 基准回归检测：基准较基线下降 >= 10% 标记警告 | P0 |
| 937 | criterion 基准 `benches/asset_store.rs` | P0 |
| 938 | criterion 基准 `benches/profiler.rs` | P0 |
| 939 | criterion 基准 `benches/ecs.rs` | P0 |
| 940 | criterion 基准 `benches/render.rs` | P0 |
| 941 | criterion 基准报告 `target/criterion/` | P1 |

#### 基准测试结构

```rust
// benches/ecs.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use engine_ecs::{World, Entity, Component};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position(f32, f32, f32);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity(f32, f32, f32);

fn spawn_entities(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecs/spawn");

    for entity_count in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(entity_count),
            entity_count,
            |b, &count| {
                b.iter(|| {
                    let mut world = World::new();
                    for _ in 0..count {
                        let entity = world.spawn();
                        world.insert(entity, Position(0.0, 0.0, 0.0));
                        world.insert(entity, Velocity(1.0, 1.0, 1.0));
                    }
                });
            },
        );
    }

    group.finish();
}

fn query_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecs/query");

    let mut world = World::new();
    let entities: Vec<Entity> = (0..10000)
        .map(|_| {
            let entity = world.spawn();
            world.insert(entity, Position(0.0, 0.0, 0.0));
            world.insert(entity, Velocity(1.0, 1.0, 1.0));
            entity
        })
        .collect();

    group.bench_function("query_10000_entities", |b| {
        b.iter(|| {
            let positions: Vec<&Position> = world.query::<&Position>().iter().collect();
            black_box(positions);
        });
    });

    group.finish();
}

criterion_group! {
    name = ecs_benches;
    config = Criterion::default().sample_size(100);
    targets = spawn_entities, query_iteration
}
criterion_main!(ecs_benches);
```

```rust
// benches/asset_store.rs
fn search_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("asset_store/search");

    let client = AssetStoreClient::new(AssetStoreConfig::default());
    let cache = AssetCache::new();

    for query_len in [3, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(query_len),
            query_len,
            |b, &len| {
                let keyword = "a".repeat(len);
                b.iter(|| {
                    let results = client.search(&keyword, SearchFilters::default());
                    black_box(results);
                });
            },
        );
    }

    group.finish();
}

fn download_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("asset_store/download");

    let client = AssetStoreClient::new(AssetStoreConfig::default());

    group.bench_function("download_100mb", |b| {
        b.iter(|| {
            let path = client.download(black_box(TEST_ASSET_ID), |_| {});
            black_box(path);
        });
    });

    group.finish();
}
```

---

### 2. Fuzz 模糊测试（需求 259, 842, 943-947）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 259 | fuzz 测试（cargo fuzz） | P0 |
| 842 | CI fuzz 测试：`cargo fuzz` 10 分钟定时任务 | P0 |
| 943 | `cargo fuzz init` fuzz 目录 | P1 |
| 944 | `fuzz/fuzz_targets/asset_pkg.rs` 打包 fuzz | P0 |
| 945 | `fuzz/fuzz_targets/profile_import.rs` .rgeprofile 导入 fuzz | P0 |
| 946 | `fuzz/fuzz_targets/json_parser.rs` JSON 解析 fuzz | P0 |
| 947 | `cargo fuzz run asset_pkg -- -max_total_time=60` | P0 |

#### Fuzz 测试目标

```rust
// fuzz/fuzz_targets/asset_pkg.rs
#![no_main]

use libfuzzer_sys::fuzz_target;
use engine_asset_store::RgePkg;

fuzz_target!(|data: &[u8]| {
    // 测试 .rgepkg 打包解析
    if let Ok(pkg) = RgePkg::unpack_from_bytes(data) {
        // 验证 manifest
        let manifest = pkg.manifest();
        
        // 验证文件列表
        for entry in pkg.file_entries() {
            let _ = entry.path();
            let _ = entry.size();
        }
        
        // 尝试提取文件
        if let Some(first_file) = pkg.file_entries().first() {
            let _ = pkg.extract_file(first_file.path(), std::path::Path::new("/tmp"));
        }
    }
});
```

```rust
// fuzz/fuzz_targets/profile_import.rs
fuzz_target!(|data: &[u8]| {
    // 测试 .rgeprofile 导入
    if let Ok(profile) = RgeProfile::import_from_bytes(data) {
        let summary = profile.summary();
        
        // 验证元数据
        let _ = summary.total_frames();
        let _ = summary.avg_fps();
        
        // 验证样本
        for frame in profile.data().frames.iter().take(100) {
            let _ = frame.frame_number;
        }
    }
});
```

---

### 3. Sanitizer 检测（需求 260-263, 893-896）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 260 | Miri 检测未定义行为（Miri） | P1 |
| 261 | ASan 检测内存错误（AddressSanitizer） | P1 |
| 262 | TSan 检测数据竞争（ThreadSanitizer） | P1 |
| 263 | MSan 检测未初始化读取（MemorySanitizer） | P1 |
| 893 | CI Miri：`cargo miri test` 每周一次 | P1 |
| 894 | CI ASan：`-Z sanitizer=address` 每日一次 | P1 |
| 895 | CI TSan：`-Z sanitizer=thread` 每日一次 | P1 |
| 896 | CI MSan：`-Z sanitizer=memory` 每日一次（nightly） | P1 |

#### Sanitizer CI 配置

```yaml
# .github/workflows/sanitizers.yml
name: Sanitizers

on:
  schedule:
    # 每天凌晨 3 点运行
    - cron: '0 3 * * *'
  workflow_dispatch:

jobs:
  miri:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Miri
        run: |
          rustup toolchain install nightly
          cargo +nightly miri install
      - name: Run Miri tests
        run: cargo +nightly miri test --workspace
        env:
          MIRIFLAGS: "-Zmiri-strict-provenance"

  address-sanitizer:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run ASan tests
        run: |
          RUSTFLAGS="-Z sanitizer=address" \
          cargo test --workspace --target x86_64-unknown-linux-gnu
        env:
          ASAN_OPTIONS: "detect_leaks=1"

  thread-sanitizer:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run TSan tests
        run: |
          RUSTFLAGS="-Z sanitizer=thread" \
          cargo test --workspace --target x86_64-unknown-linux-gnu
        env:
          TSAN_OPTIONS: "halt_on_error=1"

  memory-sanitizer:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run MSan tests
        run: |
          rustup toolchain install nightly
          RUSTFLAGS="-Z sanitizer=memory" \
          cargo +nightly test --workspace --target x86_64-unknown-linux-gnu
        env:
          MSAN_OPTIONS: "halt_on_error=1"
```

---

### 4. 集成测试（需求 256, 906-912）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 256 | 全量集成测试（examples 全部能跑） | P0 |
| 906 | `cargo test -p engine-asset-store` 通过 | P0 |
| 907 | `cargo test -p engine-template` 通过 | P0 |
| 908 | `cargo test -p engine-profiler` 通过 | P0 |
| 909 | `cargo test -p engine-docs-meta` 通过 | P0 |
| 910 | 集成测试 `tests/integration/` 目录 | P0 |
| 911 | 集成测试 `tests/integration/asset_store.rs` | P0 |
| 912 | 集成测试 `tests/integration/template_creation.rs` | P0 |
| 913 | 集成测试 `tests/integration/profiler_sampling.rs` | P0 |
| 914 | 集成测试 `tests/integration/profile_roundtrip.rs` | P0 |

#### 集成测试示例

```rust
// tests/integration/asset_store.rs
#[test]
fn test_search_and_download() {
    let config = AssetStoreConfig::test();
    let client = AssetStoreClient::new(config);

    // 搜索资源
    let results = client.search("platformer", SearchFilters::default()).unwrap();
    assert!(!results.is_empty());

    // 获取第一个资源详情
    let asset_id = results[0].id;
    let detail = client.get_asset(asset_id).unwrap();
    assert_eq!(detail.id, asset_id);

    // 下载资源（模拟）
    // let path = client.download(asset_id, |_| {}).unwrap();
    // assert!(path.exists());
}

#[test]
fn test_install_and_uninstall() {
    let config = AssetStoreConfig::test();
    let client = AssetStoreClient::new(config);

    // 安装
    let installed = client.install(&test_pkg_path(), &test_dir()).unwrap();
    assert!(installed.install_path.exists());

    // 卸载
    client.uninstall(installed.id).unwrap();
    assert!(!installed.install_path.exists());
}

// tests/integration/template_creation.rs
#[test]
fn test_create_project_from_template() {
    let manager = TemplateManager::new();
    let template = manager.get_template(&TemplateId::from("fps-template")).unwrap();

    let project = manager.create_project(
        &template.id(),
        &temp_dir(),
        "my_game",
    ).unwrap();

    assert!(project.cargo_toml().exists());
    assert!(project.main_scene().exists());
}

// tests/integration/profiler_sampling.rs
#[test]
fn test_profiler_scope_guard() {
    let config = ProfilerConfig::default();
    let mut profiler = Profiler::new(config);

    profiler.begin_frame();
    {
        let _scope = profiler.begin_scope("test_operation");
        // 模拟操作
        std::thread::sleep(Duration::from_millis(1));
    }
    profiler.end_frame();

    let samples = profiler.cpu_samples();
    assert!(!samples.is_empty());
}

// tests/integration/profile_roundtrip.rs
#[test]
fn test_rgeprofile_export_import() {
    let mut profiler = Profiler::new(ProfilerConfig::default());
    
    // 采集样本
    profiler.begin_frame();
    let _scope = profiler.begin_scope("test");
    profiler.end_frame();

    // 导出
    let path = temp_dir().join("profile.rgeprofile");
    profiler.export(&path).unwrap();

    // 导入
    let loaded = RgeProfile::import(&path).unwrap();
    assert_eq!(loaded.summary().total_frames(), 1);
}
```

---

### 5. Examples 构建与运行（需求 256, 919-936）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 919 | `cargo build --workspace --examples` 成功 | P0 |
| 920 | `cargo run --example hello_world` 3 秒后正常退出 | P0 |
| 921 | `cargo run --example 2d_platformer`（headless 模式验证启动） | P0 |
| 922 | `cargo run --example 3d_mini`（headless 模式验证启动） | P0 |
| 923 | `cargo run --example ui_demo`（headless 模式验证启动） | P0 |
| 924 | `cargo run --example physics_demo`（headless 模式验证启动） | P0 |
| 925 | `cargo run --example animation_demo`（headless 模式验证启动） | P0 |
| 926 | `cargo run --example particles_demo`（headless 模式验证启动） | P0 |
| 927 | `cargo run --example network_demo`（客户端 + 服务器） | P0 |
| 928 | `cargo run --example blueprint_demo`（headless 模式验证启动） | P0 |
| 929 | `cargo run --example store_browse`（headless 模式验证启动） | P0 |
| 930 | `cargo run --example store_purchase`（headless 模式验证启动） | P0 |
| 931 | `cargo run --example store_install`（headless 模式验证启动） | P0 |
| 932 | `cargo run --example template_new`（headless 模式验证启动） | P0 |
| 933 | `cargo run --example template_custom`（headless 模式验证启动） | P0 |
| 934 | `cargo run --example profiler_window`（headless 模式验证启动） | P0 |
| 935 | `cargo run --example profiler_remote`（headless 模式验证启动） | P0 |
| 936 | `cargo run --example profiler_bench`（headless 模式验证启动） | P0 |

---

### 6. 代码质量检查（需求 253-255, 973-977）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 253 | `cargo test --workspace` 全部通过 | P0 |
| 254 | `cargo clippy --workspace --all-targets -- -D warnings` | P0 |
| 255 | `cargo fmt --check --all --manifest-path .` | P0 |
| 973 | `cargo clippy --workspace --all-targets -- -D warnings` | P0 |
| 974 | `cargo fmt --check --all --manifest-path .` | P0 |
| 975 | `cargo doc --workspace --no-deps --document-private-items` | P0 |
| 976 | `cargo doc --workspace --all-features` | P1 |
| 977 | `cargo doc --workspace --no-default-features` | P1 |

---

### 7. 发布前检查清单（需求 788-794, 836-856）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 788 | 发布前检查：`cargo test --workspace` 通过 | P0 |
| 789 | 发布前检查：`cargo clippy --workspace -- -D warnings` 通过 | P0 |
| 790 | 发布前检查：`cargo fmt --check --workspace` 通过 | P0 |
| 791 | 发布前检查：`cargo audit` 无严重漏洞 | P0 |
| 792 | 发布前检查：`cargo deny check licenses` 通过 | P0 |
| 793 | 发布前检查：所有 examples 至少一次 `cargo build --workspace --examples` 通过 | P0 |
| 794 | 发布前检查：CHANGELOG 更新 | P0 |
| 795 | 发布前检查：README 版本号同步 | P0 |
| 796 | 发布前检查：docs 版本切换器新增版本 | P0 |
| 797 | 发布前检查：Git tag 签名 | P0 |
| 798 | 发布前检查：GitHub Release Draft 准备 | P0 |

#### 发布检查脚本

```bash
#!/bin/bash
# scripts/release-check.sh

set -e

echo "Running release checks..."

echo "1. Running tests..."
cargo test --workspace

echo "2. Running clippy..."
cargo clippy --workspace -- -D warnings

echo "3. Running fmt check..."
cargo fmt --check --all

echo "4. Running cargo audit..."
cargo audit

echo "5. Running cargo deny..."
cargo deny check licenses

echo "6. Building examples..."
cargo build --workspace --examples

echo "7. Checking CHANGELOG..."
grep -q "## \[$1\]" CHANGELOG.md || { echo "CHANGELOG not updated!"; exit 1; }

echo "8. Checking README version..."
grep -q "v$1" README.md || { echo "README not updated!"; exit 1; }

echo "All checks passed!"
```

---

### 8. v1.0.0 发布验收（需求 960, 1012-1021）

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 960 | crates.io `categories`：`game-engines / rendering / simulation / wasm` | P0 |
| 1012 | `CHANGELOG.md` 更新至 v1.0.0 | P0 |
| 1013 | `CHANGELOG.md` 包含 Added / Changed / Deprecated / Removed / Fixed / Security | P0 |
| 1014 | `README.md` 版本号改为 v1.0.0 | P0 |
| 1015 | `README.md` 包含 Badges / 简介 / 特性 / 快速开始 / 示例 / 文档 / 生态 / 社区 / 许可证 | P0 |
| 1016 | 各 crate `README.md` 更新到 v1.0.0 | P0 |
| 1017 | 正式发布到 crates.io：`cargo publish`（各 crate 按依赖顺序） | P0 |
| 1018 | crates.io 页面描述 / 关键字 / 分类 / 仓库链接正确 | P0 |
| 1019 | crates.io `keywords`：`engine / gfx / ecs / 3d / 2d` | P0 |

---

### 9. 性能调优指南（需求 220, 701）

```markdown
# docs/src/performance_tuning.md

# 性能调优指南

## 帧时间优化

### CPU 瓶颈
1. 使用 Profiler 定位热点
2. 减少 ECS 系统复杂度
3. 使用多线程并行处理

### GPU 瓶颈
1. 减少 Draw Calls
2. 使用批处理
3. 优化着色器

### 内存优化
1. 使用对象池
2. 减少内存分配
3. 使用 TypedArena

## 渲染优化

### 2D 渲染
- 使用 Sprite Batching
- 合批静态精灵
- 使用纹理图集

### 3D 渲染
- 使用 LOD
- 视锥体裁剪
- 遮挡剔除
- PBR 材质优化
```

---

## 验收标准

### 性能测试验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-1 | `cargo bench --workspace` 成功 | CI 执行 |
| AC-2 | 基准回归检测正常 | CI 执行 |
| AC-3 | `cargo fuzz run asset_pkg` 无崩溃 | CI 执行 |
| AC-4 | ASan 检测无内存错误 | CI 执行 |
| AC-5 | TSan 检测无数据竞争 | CI 执行 |
| AC-6 | 所有集成测试通过 | CI 执行 |
| AC-7 | 所有 examples 构建成功 | CI 执行 |
| AC-8 | `cargo test --doc` 所有文档示例通过 | CI 执行 |

### 发布验收

| ID | 验收条件 | 测试方式 |
|----|----------|----------|
| AC-9 | v1.0.0 发布到 crates.io | 手动验证 |
| AC-10 | crates.io 页面信息完整 | 人工审查 |
| AC-11 | GitHub Release 创建成功 | CI 执行 |

---

## 依赖关系

### 外部依赖

- criterion.rs: 基准测试
- cargo-fuzz: 模糊测试
- cargo-miri: Miri 工具
- AddressSanitizer: ASan
- ThreadSanitizer: TSan
- MemorySanitizer: MSan

---

## 优先级说明

- **P0**: 核心功能，MVP 必须包含
- **P1**: 重要功能，下一迭代包含
- **P2**: 增强功能，后续迭代包含
