# 示例实现指南

## 概述

本文档提供 Sprint 16 中所有示例项目的实现指南，包含资源商店、模板管理、性能分析器三大模块的示例代码与运行说明。

---

## 资源商店示例

### examples/store_browse

浏览与搜索资源示例。

**功能**:
- 首页展示（推荐、热门、新品）
- 分类浏览
- 关键词搜索
- 资源详情页

**目录结构**:

```
examples/store_browse/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

**核心代码**:

```rust
// src/main.rs
use engine_asset_store::{AssetStoreClient, AssetStoreConfig, SearchFilters};
use engine_ui::{Application, Window, Layout, Column, Text, Image};

fn main() {
    let config = AssetStoreConfig::default();
    let client = AssetStoreClient::new(config);

    let mut app = Application::new();

    // 创建主窗口
    let window = Window::new()
        .title("Asset Store")
        .size(1200, 800);

    app.run(window, move |ui| {
        // 首页视图
        let home = HomePageView::new(&client);

        Column::new()
            .add(home.render())
            .build(ui);
    });
}

struct HomePageView<'a> {
    client: &'a AssetStoreClient,
}

impl<'a> HomePageView<'a> {
    fn new(client: &'a AssetStoreClient) -> Self {
        Self { client }
    }

    fn render(&self) -> impl View {
        let featured = self.client.featured(10).unwrap_or_default();
        let trending = self.client.trending(10).unwrap_or_default();
        let new_releases = self.client.new_releases(10).unwrap_or_default();

        Column::new()
            .section("Featured", featured.iter().map(asset_card))
            .section("Trending", trending.iter().map(asset_card))
            .section("New Releases", new_releases.iter().map(asset_card))
    }
}

fn asset_card(asset: &AssetSummary) -> impl View {
    // 资源卡片组件
    Column::new()
        .add(Image::new(&asset.thumbnail_url))
        .add(Text::new(&asset.name))
        .add(Text::new(&asset.author))
        .add(rating_stars(asset.rating.stars()))
}
```

**运行方式**:

```bash
cargo run --example store_browse
```

---

### examples/store_purchase

购物车与购买流程示例。

**功能**:
- 添加资源到购物车
- 购物车管理
- 结账流程
- 订单查看

**核心代码**:

```rust
// src/main.rs
use engine_asset_store::{
    AssetStoreClient, Cart, PaymentMethod, Money,
    AssetStoreConfig, PriceModel,
};

fn main() {
    let config = AssetStoreConfig::default();
    let client = AssetStoreClient::new(config);

    // 确保已登录
    client.login("user@example.com", "password").unwrap();

    let mut cart = Cart::new();

    // 搜索并添加到购物车
    let results = client.search("platformer assets", SearchFilters::default()).unwrap();

    for asset in results.iter().take(3) {
        cart.add(asset.id).unwrap();
    }

    // 查看购物车
    println!("Cart items:");
    for item in cart.items() {
        println!("  - {} (${})", item.name, item.price);
    }
    println!("Total: ${}", cart.total());

    // 结账
    let order = cart.checkout(PaymentMethod::CreditCard).unwrap();
    println!("Order created: {}", order.id);

    // 查看订单
    let orders = client.orders(1).unwrap();
    for order in orders.items {
        println!("Order {} - {}", order.id, order.status);
    }
}
```

**运行方式**:

```bash
cargo run --example store_purchase
```

---

### examples/store_install

下载、安装与回滚示例。

**功能**:
- 资源下载（带进度条）
- 资源安装
- 版本更新
- 回滚到指定版本
- 离线模式

**核心代码**:

```rust
// src/main.rs
use engine_asset_store::{
    AssetStoreClient, AssetStoreConfig, InstalledAsset,
    RollbackManager, OfflineMode,
};
use std::path::PathBuf;

fn main() {
    let config = AssetStoreConfig::default();
    let client = AssetStoreClient::new(config);

    // 下载资源
    let asset_id = AssetId::parse("550e8400-e29b-41d4-a716-446655440000").unwrap();

    println!("Downloading asset...");
    let path = client.download(asset_id, |progress| {
        println!(
            "Progress: {:.1}% ({} / {} KB, {} KB/s)",
            progress.bytes_downloaded as f64 / progress.bytes_total as f64 * 100.0,
            progress.bytes_downloaded / 1024,
            progress.bytes_total / 1024,
            progress.speed_kbps
        );
    }).unwrap();

    // 安装
    println!("Installing...");
    let installed = client.install(&path, &PathBuf::from("./assets")).unwrap();
    println!("Installed: {} v{}", installed.name, installed.version);

    // 检查更新
    if let Some(update) = installed.check_update(&client).unwrap() {
        println!("Update available: {}", update);

        // 更新
        let updated = client.update(installed.id).unwrap();
        println!("Updated to: {}", updated.version);
    }

    // 创建快照用于回滚
    let rollback_mgr = RollbackManager::new(PathBuf::from("./snapshots"));
    let snapshot_id = rollback_mgr.create_snapshot(installed.id);
    println!("Snapshot created: {}", snapshot_id);

    // 离线模式
    OfflineMode::enable();
    println!("Offline mode enabled");

    // 列出已安装资源
    let installed_list = client.list_installed();
    for asset in installed_list {
        println!("  - {} v{}", asset.name, asset.version);
    }

    // 回滚
    rollback_mgr.rollback(snapshot_id).unwrap();
    println!("Rolled back successfully");
}
```

**运行方式**:

```bash
cargo run --example store_install
```

---

## 模板管理示例

### examples/template_new

从模板创建 FPS 项目。

**功能**:
- 列出所有内置模板
- 选择 FPS 模板
- 创建新项目
- 验证项目结构

**核心代码**:

```rust
// src/main.rs
use engine_template::{
    TemplateManager, BuiltInTemplates, Project,
    CreateProjectOptions, TemplateId,
};
use std::path::PathBuf;

fn main() {
    let manager = TemplateManager::new();

    // 列出所有内置模板
    println!("Available templates:");
    for template in BuiltInTemplates::all() {
        println!("  - {} ({:?})", template.name, template.game_type);
    }

    // 获取 FPS 模板
    let fps_template = BuiltInTemplates::fps();
    println!("\nCreating project from: {}", fps_template.name);

    // 创建项目
    let output_dir = PathBuf::from("./projects");
    let options = CreateProjectOptions {
        project_name: "my_fps_game".to_string(),
        output_dir: output_dir.clone(),
        overwrite: false,
        init_git: true,
        run_cargo_check: true,
    };

    let project = manager.create_project_with_options(&fps_template.id(), options).unwrap();

    println!("Project created:");
    println!("  Name: {}", project.name());
    println!("  Path: {:?}", project.path());
    println!("  Cargo.toml: {:?}", project.cargo_toml());
    println!("  Main scene: {:?}", project.main_scene());

    // 验证文件存在
    assert!(project.cargo_toml().exists(), "Cargo.toml should exist");
    assert!(project.main_scene().exists(), "Main scene should exist");

    // 运行 cargo check 确保配置正确
    println!("\nRunning cargo check...");
    let output = project.run_cargo(&["check"]).unwrap();
    if output.status.success() {
        println!("Cargo check passed!");
    } else {
        println!("Cargo check failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // 构建项目
    println!("\nBuilding project...");
    let output = project.build().unwrap();
    if output.status.success() {
        println!("Build successful!");
    } else {
        println!("Build failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}
```

**运行方式**:

```bash
cargo run --example template_new
```

---

### examples/template_custom

自定义模板创建与导出。

**功能**:
- 使用 TemplateBuilder 构建自定义模板
- 保存为 zip
- 从 zip 加载模板
- 使用自定义模板创建项目

**核心代码**:

```rust
// src/main.rs
use engine_template::{
    TemplateBuilder, TemplateManager, TemplateType,
    TemplateGameType, TemplateVariable, TemplateVariableContext,
};
use std::path::PathBuf;

fn main() {
    // 使用构建器创建自定义模板
    let template = TemplateBuilder::new("My Custom Platformer")
        .category(TemplateType::Template2D)
        .game_type(TemplateGameType::Platformer)
        .description("A custom 2D platformer template with advanced movement")
        .add_file(
            PathBuf::from("./templates/platformer/Cargo.toml.template"),
            PathBuf::from("Cargo.toml"),
        )
        .add_file(
            PathBuf::from("./templates/platformer/src/main.rs.template"),
            PathBuf::from("src/main.rs"),
        )
        .add_file(
            PathBuf::from("./templates/platformer/scenes/level.rs.template"),
            PathBuf::from("scenes/level.rs"),
        )
        .thumbnail(PathBuf::from("./templates/platformer/thumbnail.png"))
        .build()
        .unwrap();

    println!("Template created: {}", template.name);

    // 保存为 zip
    let zip_path = PathBuf::from("./my_platformer_template.zip");
    template.save_zip(&zip_path).unwrap();
    println!("Template saved to: {:?}", zip_path);

    // 从 zip 加载
    let loaded = Template::load_zip(&zip_path).unwrap();
    println!("Template loaded from zip: {}", loaded.name);

    // 验证模板
    loaded.validate().unwrap();
    println!("Template validation passed!");

    // 模板变量替换
    let context = TemplateVariableContext {
        project_name: "my_game".to_string(),
        author: "Game Developer".to_string(),
        engine_version: "1.0.0".to_string(),
        date: "2024-01-01".to_string(),
    };

    let cargo_content = std::fs::read_to_string("Cargo.toml.template").unwrap();
    let replaced = TemplateVariable::replace_in(&cargo_content, &context);
    println!("\nReplaced Cargo.toml:\n{}", replaced);

    // 使用加载的模板创建项目
    let manager = TemplateManager::new();
    manager.register_template(loaded);

    let project = manager
        .create_project(
            &TemplateId::parse("my-platformer").unwrap(),
            &PathBuf::from("./projects"),
            "custom_game",
        )
        .unwrap();

    println!("\nProject created from custom template: {}", project.name());
}
```

**运行方式**:

```bash
cargo run --example template_custom
```

---

## 性能分析器示例

### examples/profiler_window

内置 Profiler 面板窗口示例。

**功能**:
- 多 Tab 面板（CPU/GPU/内存/渲染）
- Flame Graph 可视化
- Timeline 时间轴
- FPS/帧时间折线图
- 帧详情跳转

**核心代码**:

```rust
// src/main.rs
use engine_profiler::{
    Profiler, ProfilerConfig, ProfilerCategory,
    FlameGraph, Timeline, LineChart, Histogram,
    MetricsCollector, FrameMetricsAggregator,
};
use engine_ui::{Application, Window, TabView, Layout};

fn main() {
    // 创建 Profiler
    let config = ProfilerConfig::default()
        .with_sample_rate(1000)  // 1kHz
        .with_max_frames(10000);
    let mut profiler = Profiler::new(config);

    // 启用所有类别
    profiler.toggle(ProfilerCategory::Cpu, true);
    profiler.toggle(ProfilerCategory::Gpu, true);
    profiler.toggle(ProfilerCategory::Memory, true);
    profiler.toggle(ProfilerCategory::Render, true);

    let mut app = Application::new();

    let window = Window::new()
        .title("Profiler")
        .size(1400, 900);

    app.run(window, move |ui| {
        TabView::new()
            .tab("CPU", || cpu_panel(&profiler))
            .tab("GPU", || gpu_panel(&profiler))
            .tab("Memory", || memory_panel(&profiler))
            .tab("Render", || render_panel(&profiler))
            .tab("Flame Graph", || flame_graph_panel(&profiler))
            .tab("Timeline", || timeline_panel(&profiler))
            .build(ui);
    });

    // 模拟帧循环
    for frame_num in 0..1000 {
        profiler.begin_frame();

        // 模拟游戏逻辑
        let _scope = profiler.begin_scope("game_logic");
        simulate_game_logic();
        drop(_scope);

        // 模拟渲染
        let _scope = profiler.begin_scope("rendering");
        simulate_rendering();
        drop(_scope);

        profiler.end_frame();

        // 实时更新 UI
        if frame_num % 60 == 0 {
            update_profiler_window(&profiler);
        }
    }
}

fn cpu_panel(profiler: &Profiler) -> impl View {
    let samples = profiler.cpu_samples();

    Column::new()
        .add(Text::new(format!("CPU Samples: {}", samples.len())))
        .add(LineChart::new()
            .title("Frame Time (ms)")
            .data(samples.iter().map(|s| (s.start_ns, s.duration_ns as f64 / 1_000_000.0)))
        )
}

fn flame_graph_panel(profiler: &Profiler) -> impl View {
    let samples = profiler.cpu_samples();
    let flame_graph = FlameGraph::from_samples(samples);

    Column::new()
        .add(Text::new("Flame Graph"))
        .add(flame_graph.render_svg())
        .add(Text::new(format!("Hot path: {:?}", flame_graph.hot_path())))
}

fn timeline_panel(profiler: &Profiler) -> impl View {
    let mut timeline = Timeline::new();

    // 添加 CPU track
    timeline.add_track(TimelineTrack {
        name: "CPU".to_string(),
        samples: profiler.cpu_samples().iter().map(|s| s.start_ns).collect(),
        color: "#FF6B6B".to_string(),
        row_index: 0,
    });

    // 添加渲染 track
    timeline.add_track(TimelineTrack {
        name: "Render".to_string(),
        samples: profiler.render_samples().iter().map(|s| s.start_ns).collect(),
        color: "#4ECDC4".to_string(),
        row_index: 1,
    });

    Column::new()
        .add(Text::new("Timeline"))
        .add(timeline.render())
}
```

**运行方式**:

```bash
cargo run --example profiler_window
```

---

### examples/profiler_remote

远程采样与桌面查看示例。

**功能**:
- 启动远程采样服务器
- 移动设备连接
- 桌面端接收并显示

**核心代码**:

```rust
// src/server.rs - 远程设备端
use engine_profiler::{
    Profiler, RemoteProfilerServer, ProfilerConfig,
    SampleBatch, MetricsSnapshot,
};
use std::time::{Duration, Instant};

fn main() {
    let config = ProfilerConfig::default();
    let mut profiler = Profiler::new(config);

    // 启动远程服务器
    let server = RemoteProfilerServer::bind("0.0.0.0:8080").unwrap();
    println!("Remote profiler server listening on 0.0.0.0:8080");

    loop {
        match server.accept() {
            Ok(session) => {
                println!("Client connected: {:?}", session.device_info());

                // 开始采样并发送
                loop {
                    profiler.begin_frame();
                    let _scope = profiler.begin_scope("frame");
                    // ... 采样逻辑
                    profiler.end_frame();

                    let batch = SampleBatch {
                        timestamp: Instant::now().elapsed().as_secs(),
                        device_id: "mobile_device_1".to_string(),
                        cpu_samples: profiler.cpu_samples().to_vec(),
                        gpu_samples: profiler.gpu_samples().to_vec(),
                        metrics: MetricsSnapshot::default(),
                    };

                    session.send(batch);
                    std::thread::sleep(Duration::from_millis(16)); // ~60fps
                }
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
            }
        }
    }
}

// src/client.rs - 桌面端
use engine_profiler::{RemoteProfilerClient, FlameGraph, Timeline};

fn main() {
    // 连接远程服务器
    let mut client = RemoteProfilerClient::connect("192.168.1.100:8080").unwrap();

    println!("Connected to: {:?}", client.server_info());

    let mut flame_graph = FlameGraph::new();
    let mut timeline = Timeline::new();

    // 接收样本
    loop {
        match client.receive_samples() {
            Ok(batch) => {
                println!("Received batch from {}: {} CPU samples",
                    batch.device_id, batch.cpu_samples.len());

                // 更新可视化
                flame_graph.update(&batch.cpu_samples);
                timeline.add_batch(&batch);
            }
            Err(e) => {
                eprintln!("Receive error: {}", e);
                break;
            }
        }
    }
}
```

**运行方式**:

```bash
# 服务器端（在移动设备或嵌入式设备上）
cargo run --example profiler_remote --features server

# 客户端（在桌面上）
cargo run --example profiler_remote --features client
```

---

### examples/profiler_bench

基准测试与回归检测示例。

**功能**:
- 创建性能基准
- 与 baseline 比较
- 导出 .rgeprofile
- 检测回归并输出报告

**核心代码**:

```rust
// src/main.rs
use engine_profiler::{
    Profiler, ProfilerConfig, RgeProfile, BaselineManager,
    RegressionReport, BaselineProfile,
};
use std::path::PathBuf;

fn main() {
    let baseline_dir = PathBuf::from("./baselines");
    let manager = BaselineManager::new(baseline_dir.clone());

    // 列出已有基线
    println!("Available baselines:");
    for name in manager.list() {
        println!("  - {}", name);
    }

    // 创建新基准
    println!("\nCollecting new profile...");
    let config = ProfilerConfig::default();
    let mut profiler = Profiler::new(config);

    for _ in 0..1000 {
        profiler.begin_frame();
        let _scope = profiler.begin_scope("benchmark_frame");

        // 模拟工作负载
        do_workload("texture_loading");
        do_workload("physics_simulation");
        do_workload("rendering");

        drop(_scope);
        profiler.end_frame();
    }

    // 导出 profile
    let profile_path = PathBuf::from("./benchmark.rgeprofile");
    profiler.export(&profile_path).unwrap();
    println!("Profile exported to: {:?}", profile_path);

    // 保存为新基线
    manager.save("benchmark_v1", &profiler).unwrap();
    println!("Saved as baseline: benchmark_v1");

    // 加载基线并比较
    if let Ok(baseline) = manager.load("benchmark_v1") {
        // 运行新基准
        let mut new_profiler = Profiler::new(ProfilerConfig::default());
        run_benchmark(&mut new_profiler);

        // 比较
        let report = baseline.compare(&new_profiler);

        println!("\nRegression Report:");
        println!("{}", report.print());

        if report.has_regressions() {
            println!("\n⚠️  Regressions detected!");
            for reg in report.regressions() {
                println!("  - {}: {:.2}% regression (p={:.4})",
                    reg.metric, reg.delta, reg.p_value);
            }
        } else {
            println!("\n✅ No regressions detected");
        }

        // 导出 JSON 报告
        let json_report = report.to_json();
        std::fs::write("./regression_report.json", &json_report).unwrap();
        println!("\nJSON report written to: regression_report.json");
    }

    // 使用 RgeProfile 直接加载和查看
    let profile = RgeProfile::import(&profile_path).unwrap();
    let summary = profile.summary();

    println!("\nProfile Summary:");
    println!("  Total frames: {}", summary.total_frames);
    println!("  Avg FPS: {:.2}", summary.avg_fps);
    println!("  Avg frame time: {:.2} ms", summary.avg_frame_time_ms);
    println!("  File size: {} bytes", summary.file_size_bytes);
}
```

**运行方式**:

```bash
cargo run --example profiler_bench
```

---

## 示例运行矩阵

| 示例 | 命令 | 验证方式 |
|------|------|----------|
| `hello_world` | `cargo run --example hello_world` | 窗口显示，3秒后退出 |
| `2d_platformer` | `cargo run --example 2d_platformer` | headless 启动验证 |
| `3d_mini` | `cargo run --example 3d_mini` | headless 启动验证 |
| `ui_demo` | `cargo run --example ui_demo` | headless 启动验证 |
| `physics_demo` | `cargo run --example physics_demo` | headless 启动验证 |
| `animation_demo` | `cargo run --example animation_demo` | headless 启动验证 |
| `particles_demo` | `cargo run --example particles_demo` | headless 启动验证 |
| `network_demo` | `cargo run --example network_demo` | 客户端+服务器 |
| `blueprint_demo` | `cargo run --example blueprint_demo` | headless 启动验证 |
| `store_browse` | `cargo run --example store_browse` | UI 窗口显示 |
| `store_purchase` | `cargo run --example store_purchase` | 命令行输出 |
| `store_install` | `cargo run --example store_install` | 命令行输出 |
| `template_new` | `cargo run --example template_new` | 项目创建验证 |
| `template_custom` | `cargo run --example template_custom` | zip 创建验证 |
| `profiler_window` | `cargo run --example profiler_window` | UI 窗口显示 |
| `profiler_remote` | `cargo run --example profiler_remote` | 网络连接验证 |
| `profiler_bench` | `cargo run --example profiler_bench` | 基准测试输出 |
