# Sprint 16 测试计划

## 概述

本文档定义 Sprint 16 的完整测试计划，覆盖资源商店、模板管理器、性能分析器、CI/CD、文档与社区、性能优化六大模块。

**Sprint 周期**: 4 周
**总需求数**: 960 条
**核心验收示例**: `examples/store_browse` 与 `examples/profiler_window`

---

## 测试阶段

### 第 1 周：单元测试与集成测试

| 日期 | 模块 | 测试内容 |
|------|------|----------|
| Day 1-2 | engine-asset-store | 核心 API 单元测试 |
| Day 3-4 | engine-template | 模板管理 API 单元测试 |
| Day 5-7 | engine-profiler | 采样器与可视化单元测试 |

### 第 2 周：Example 验证与集成测试

| 日期 | 模块 | 测试内容 |
|------|------|----------|
| Day 8-9 | examples | 所有 Example 构建验证 |
| Day 10-11 | integration | 集成测试完善 |
| Day 12-14 | CI/CD | GitHub Actions 工作流验证 |

### 第 3 周：性能测试与回归检测

| 日期 | 模块 | 测试内容 |
|------|------|----------|
| Day 15-16 | benchmark | criterion 基准测试 |
| Day 17-18 | fuzz | cargo fuzz 模糊测试 |
| Day 19-21 | sanitizer | ASan/TSan/MSan 检测 |

### 第 4 周：发布验收与文档

| 日期 | 模块 | 测试内容 |
|------|------|----------|
| Day 22-23 | release | 发布前检查清单 |
| Day 24-25 | docs | 文档覆盖率验证 |
| Day 26-28 | final | 最终验收与修复 |

---

## 模块测试详情

### 1. engine-asset-store 模块

#### 单元测试覆盖率目标: >= 80%

| 测试组 | 测试用例 | 需求覆盖 |
|--------|----------|----------|
| auth | login, logout, token_refresh, is_logged_in | 6-9, 37 |
| search | search, search_with_pagination, filters | 10, 38, 282 |
| browse | browse, browse_category, trending, featured | 11, 285-287 |
| download | download, download_async, pause, resume, cancel | 13, 40, 66-68, 336-339 |
| install | install, uninstall, verify | 14, 16, 41, 343-346 |
| rollback | create_snapshot, rollback, list_snapshots | 19, 70-72, 360-363 |
| rgepkg | pack, unpack, verify, sign | 31-35, 58-62, 328-339 |
| cart | add, remove, checkout | 76-80, 340-344 |
| dependency | resolve, detect_conflicts | 63-65, 347-349 |

#### 集成测试

```rust
// tests/integration/asset_store.rs

#[test]
fn test_full_purchase_flow() {
    // 1. 登录
    let client = AssetStoreClient::new(test_config());
    client.login("test@example.com", "password").unwrap();

    // 2. 搜索
    let results = client.search("platformer", SearchFilters::default()).unwrap();
    assert!(!results.is_empty());

    // 3. 获取详情
    let detail = client.get_asset(results[0].id).unwrap();

    // 4. 下载
    let path = client.download(detail.summary.id, |_| {}).unwrap();

    // 5. 安装
    let installed = client.install(&path, &test_dir()).unwrap();

    // 6. 卸载
    client.uninstall(installed.id).unwrap();
}

#[test]
fn test_rgepkg_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let pkg_path = temp_dir.path().join("test.rgepkg");

    // 创建测试资源
    let source = TempDir::new().unwrap();
    write_test_files(source.path());

    // 打包
    let key = SigningKey::generate();
    RgePkg::pack(source.path(), &pkg_path, &key).unwrap();

    // 解包
    let target = TempDir::new().unwrap();
    let metadata = RgePkg::unpack(&pkg_path, target.path(), true).unwrap();

    assert_eq!(metadata.name, "test_asset");
    assert!(RgePkg::verify(&pkg_path, &key.public_key()).unwrap());
}
```

---

### 2. engine-template 模块

#### 单元测试覆盖率目标: >= 80%

| 测试组 | 测试用例 | 需求覆盖 |
|--------|----------|----------|
| manager | new, list, get, create_project | 72-76, 99-102 |
| template | from_zip, to_zip, validate | 80-81, 107-112, 453-456 |
| filter | filter, search, featured | 113-116, 454-455 |
| builder | category, game_type, add_file, build | 118, 475-482 |
| project | open, build, run, cargo_toml | 82-84, 109-112, 466-470 |

#### 集成测试

```rust
// tests/integration/template_creation.rs

#[test]
fn test_create_project_from_fps_template() {
    let manager = TemplateManager::new();
    let template = BuiltInTemplates::fps();

    let output = TempDir::new().unwrap();
    let project = manager
        .create_project(&template.id(), output.path(), "test_fps")
        .unwrap();

    // 验证项目结构
    assert!(project.cargo_toml().exists());
    assert!(project.main_scene().exists());
    assert!(project.path().join(".git").exists());

    // 验证 cargo.toml 内容
    let cargo: CargoToml = project.read_cargo_toml().unwrap();
    assert!(cargo.dependencies.contains_key("engine"));
    assert_eq!(cargo.package_name, "test_fps");
}

#[test]
fn test_custom_template_roundtrip() {
    // 构建自定义模板
    let template = TemplateBuilder::new("Custom Test")
        .category(TemplateType::Template2D)
        .game_type(TemplateGameType::Platformer)
        .add_file(test_file("Cargo.toml"), "Cargo.toml".into())
        .build()
        .unwrap();

    // 保存为 zip
    let zip_path = TempDir::new().unwrap().path().join("custom.zip");
    template.save_zip(&zip_path).unwrap();

    // 从 zip 加载
    let loaded = Template::load_zip(&zip_path).unwrap();
    assert_eq!(loaded.name, "Custom Test");
    assert_eq!(loaded.category, TemplateType::Template2D);

    // 使用加载的模板创建项目
    let manager = TemplateManager::new();
    manager.register_template(loaded);

    let project = manager
        .create_project(&TemplateId::parse("custom").unwrap(), TempDir::new().unwrap().path(), "custom_project")
        .unwrap();

    assert!(project.cargo_toml().exists());
}
```

---

### 3. engine-profiler 模块

#### 单元测试覆盖率目标: >= 80%

| 测试组 | 测试用例 | 需求覆盖 |
|--------|----------|----------|
| profiler | begin_frame, end_frame, scope | 92-96, 539-542 |
| samples | cpu, gpu, memory, render | 98-103, 499-506 |
| flame_graph | from_samples, render, hot_path | 105-108, 132-135, 521-530 |
| timeline | add_track, zoom, pan | 109-112, 136, 530-538 |
| histogram | mean, median, p95, p99 | 113-117, 140-144 |
| remote | server, client, session | 156-166, 577-587 |
| regression | baseline, compare, report | 140-143, 170-175, 602-613 |

#### 集成测试

```rust
// tests/integration/profiler_sampling.rs

#[test]
fn test_profiler_scope_guard_auto_end() {
    let config = ProfilerConfig::default();
    let mut profiler = Profiler::new(config);

    profiler.begin_frame();

    // 手动 scope
    let scope1 = profiler.begin_scope("outer");
    let samples_before = profiler.cpu_samples().len();

    {
        let scope2 = profiler.begin_scope("inner");
        // 内部操作
        profiler.record_event("test_event", json!({"key": "value"}));
    } // scope2 自动 end

    let samples_after = profiler.cpu_samples().len();
    assert!(samples_after > samples_before);

    drop(scope1); // scope1 结束
    profiler.end_frame();

    // 验证帧样本
    let frame_samples = profiler.samples_for_frame(0);
    assert_eq!(frame_samples.frame_number, 0);
}

#[test]
fn test_flame_graph_generation() {
    let mut profiler = Profiler::new(ProfilerConfig::default());

    // 采集帧
    for _ in 0..10 {
        profiler.begin_frame();
        let _scope = profiler.begin_scope("frame");
        simulate_workload();
        profiler.end_frame();
    }

    // 生成火焰图
    let samples = profiler.cpu_samples();
    let flame = FlameGraph::from_samples(samples);

    assert!(!flame.nodes().is_empty());
    assert!(flame.root().duration > 0);

    // 验证 SVG 渲染
    let svg = flame.render_text(800, 600);
    assert!(!svg.is_empty());
}

#[test]
fn test_baseline_regression_detection() {
    let baseline_manager = BaselineManager::new(TempDir::new().unwrap().path().into());

    // 创建基线
    let mut baseline_profiler = Profiler::new(ProfilerConfig::default());
    run_benchmark(&mut baseline_profiler);
    baseline_manager.save("v1_baseline", &baseline_profiler).unwrap();

    // 创建新基准（模拟退化）
    let mut new_profiler = Profiler::new(ProfilerConfig::default());
    simulate_degraded_benchmark(&mut new_profiler);

    // 加载基线并比较
    let baseline = baseline_manager.load("v1_baseline").unwrap();
    let report = baseline.compare(&new_profiler);

    assert!(report.has_regressions());
    assert!(report.regression_count() > 0);
}
```

---

### 4. CI/CD 测试

#### 自动化测试矩阵

| 触发条件 | 执行的 job | 预期结果 |
|----------|------------|----------|
| push to main | test, clippy, fmt, doc, examples | 全部通过 |
| PR created | test, clippy, fmt | 全部通过 |
| tag v* | test, release, docker, homebrew | 全部通过 |
| daily | sanitizer (ASan/TSan/MSan) | 无错误 |
| weekly | miri, fuzz | 无错误 |

#### 发布前检查清单

```bash
#!/bin/bash
# scripts/release-check.sh

set -e

echo "Running release checks..."

# 1. 测试
cargo test --workspace || { echo "Tests failed"; exit 1; }

# 2. Clippy
cargo clippy --workspace -- -D warnings || { echo "Clippy failed"; exit 1; }

# 3. Fmt
cargo fmt --check --all || { echo "Fmt failed"; exit 1; }

# 4. Audit
cargo audit || { echo "Audit failed"; exit 1; }

# 5. Examples
cargo build --workspace --examples || { echo "Examples build failed"; exit 1; }

# 6. Docs
cargo doc --workspace --no-deps || { echo "Docs failed"; exit 1; }

# 7. CHANGELOG 检查
grep -q "## \[$VERSION\]" CHANGELOG.md || { echo "CHANGELOG not updated"; exit 1; }

echo "All release checks passed!"
```

---

### 5. Examples 验证矩阵

| 示例 | 构建验证 | 运行验证 | headless | 验收标准 |
|------|----------|----------|----------|----------|
| hello_world | cargo build | 3秒退出 | - | 窗口显示标题 |
| 2d_platformer | cargo build | headless | true | 启动无错误 |
| 3d_mini | cargo build | headless | true | 启动无错误 |
| ui_demo | cargo build | headless | true | UI 渲染正常 |
| physics_demo | cargo build | headless | true | 物理模拟正常 |
| animation_demo | cargo build | headless | true | 动画播放正常 |
| particles_demo | cargo build | headless | true | 粒子效果正常 |
| network_demo | cargo build | client+server | false | 双向通信正常 |
| blueprint_demo | cargo build | headless | true | 蓝图执行正常 |
| store_browse | cargo build | UI 窗口 | false | **核心验收** |
| store_purchase | cargo build | 命令行 | - | 订单创建成功 |
| store_install | cargo build | 命令行 | - | 安装/回滚成功 |
| template_new | cargo build | 命令行 | - | 项目创建成功 |
| template_custom | cargo build | 命令行 | - | zip 导出/导入成功 |
| profiler_window | cargo build | UI 窗口 | false | **核心验收** |
| profiler_remote | cargo build | 网络连接 | false | 远程采样正常 |
| profiler_bench | cargo build | 命令行 | - | 基准测试正常 |

---

### 6. 性能测试

#### 基准测试（criterion.rs）

| 基准 | 目标 | 回归阈值 |
|------|------|----------|
| ecs/spawn_100 | < 1ms | 10% |
| ecs/spawn_1000 | < 10ms | 10% |
| ecs/spawn_10000 | < 100ms | 10% |
| ecs/query | < 1ms | 10% |
| asset_store/search | < 50ms | 15% |
| asset_store/download_100mb | > 10 MB/s | 20% |
| profiler/flame_graph | < 10ms | 15% |

#### Sanitizer 检测

| Sanitizer | 目标平台 | 频率 | 允许的错误 |
|------------|----------|------|------------|
| ASan | Linux x64 | 每日 | 0 |
| TSan | Linux x64 | 每日 | 0 |
| MSan | Linux x64 | 每周 | 0 |
| Miri | Linux x64 | 每周 | 0 |

#### Fuzz 测试

| Target | 运行时间 | 预期结果 |
|--------|----------|----------|
| asset_pkg | 10 分钟/次 | 无崩溃 |
| profile_import | 10 分钟/次 | 无崩溃 |
| json_parser | 10 分钟/次 | 无崩溃 |

---

### 7. 文档测试

| 检查项 | 目标 | 测试方式 |
|--------|------|----------|
| `cargo doc --workspace --no-deps` | 成功 | CI |
| 公开函数文档覆盖率 | >= 95% | tool |
| 公开结构体文档覆盖率 | 100% | tool |
| 公开枚举文档覆盖率 | 100% | tool |
| 文档示例 `cargo test --doc` | 全部通过 | CI |
| mdbook 构建 | 成功 | CI |
| 中英文一致性 | 完整 | 人工审查 |

---

## 验收标准

### 功能验收

| 模块 | 验收条件 | 优先级 |
|------|----------|--------|
| asset-store | 所有 API 单元测试通过 | P0 |
| template | 所有 API 单元测试通过 | P0 |
| profiler | 所有采样和可视化测试通过 | P0 |
| CI/CD | GitHub Actions 工作流正常 | P0 |
| docs | 文档站点可构建 | P0 |
| **store_browse** | **可运行，UI 正常显示** | **P0** |
| **profiler_window** | **可运行，多 Tab 显示** | **P0** |

### 质量验收

| 检查项 | 目标 | 优先级 |
|--------|------|--------|
| `cargo test --workspace` | 100% 通过 | P0 |
| `cargo clippy --workspace` | 0 警告 | P0 |
| `cargo fmt --check` | 通过 | P0 |
| 单元测试覆盖率 | >= 70% | P1 |
| 核心 crate 覆盖率 | >= 80% | P1 |
| `cargo audit` | 无严重漏洞 | P0 |

### 发布验收

| 检查项 | 目标 | 优先级 |
|--------|------|--------|
| CHANGELOG | v1.0.0 完整 | P0 |
| README | v1.0.0 版本号 | P0 |
| crates.io | 所有 crate 发布 | P0 |
| GitHub Release | Assets 完整 | P0 |
| Docker Hub | 镜像可下载 | P0 |

---

## 测试报告

### Sprint 16 完成标准

1. **所有单元测试通过** (`cargo test --workspace`)
2. **所有 Example 构建成功** (`cargo build --workspace --examples`)
3. **核心验收示例可运行**:
   - `store_browse` - 资源浏览 UI
   - `profiler_window` - 性能分析器面板
4. **CI/CD 工作流正常**:
   - push 到 main 触发完整 CI
   - tag 触发 Release 工作流
5. **文档完整**:
   - mdbook 构建成功
   - API 文档覆盖率 >= 95%
6. **性能测试通过**:
   - criterion 基准测试完成
   - 无 sanitizer 错误
7. **发布准备完成**:
   - CHANGELOG.md 更新
   - README 版本号正确
   - crates.io 发布就绪

### 测试指标汇总

| 指标 | Sprint 16 目标 |
|------|----------------|
| 需求总数 | 960 |
| 单元测试用例 | >= 500 |
| 集成测试用例 | >= 50 |
| 测试覆盖率 | >= 70% |
| 文档覆盖率 | >= 95% |
| Examples 通过率 | 100% |
| CI 成功率 | 100% |
