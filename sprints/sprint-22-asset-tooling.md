# Sprint 22 · 资产 / 工具链 / 热更新 / 性能分析

> 文档编号: `sprint-22-asset-tooling.md / v1.0
> 周期: 3 周 (15 个工作日)
> 上游依赖: Sprint 17-21 全部
> 下游交付: 引擎 1.0 正式版 / 编辑器（后续 sprint）

---

## 1. 目标与范围

**目标：** 补齐 **生产级引擎** 缺失的工具链：资产管道（glTF / FBX / 纹理导入）、场景序列化（文本格式、版本控制友好）、热更新机制、Profiler 性能分析、CI/CD 集成、资产包格式（pak）。

**范围：**
- ✅ 资产导入器：glTF / FBX / PNG / JPG / KTX2 / EXR
- ✅ 资产包格式（pak / zip / 目录）
- ✅ 场景序列化：YAML / TOML（人类可读、diff 友好）
- ✅ 热更新：增量资源重载
- ✅ 性能分析器：CPU / GPU / 内存 / 帧时间
- ✅ 命令行工具：`engine-cli` (build / run / profile / cook)
- ✅ CI/CD 模板：GitHub Actions / GitLab CI
- ⛔ 不含：完整编辑器（独立项目）、Steam 集成、PS5/Xbox 平台构建（远期）

**核心参考：** Unreal Pak / Unity AssetBundle / Godot ResourceFormat / Bevy Asset。

---

## 2. 上游需求对接

| 来源 | 关联章节 | 承接 |
|------|---------|------|
| [NEXT_PHASE_REQUIREMENTS.md § 9.1](../NEXT_PHASE_REQUIREMENTS.md) | 资产系统 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 9.2](../NEXT_PHASE_REQUIREMENTS.md) | 序列化 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 9.3](../NEXT_PHASE_REQUIREMENTS.md) | 热更新 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 9.4](../NEXT_PHASE_REQUIREMENTS.md) | Profiler | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 9.5](../NEXT_PHASE_REQUIREMENTS.md) | CLI / CI | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 5 M5](../NEXT_PHASE_REQUIREMENTS.md) | Milestone M5 部分 | ✅ 本 sprint |

---

## 3. 子模块拆分

### 3.1 [01-asset-system.md](modules/01-asset-system.md) — 资产系统

**核心交付：**
- `engine-asset/src/handle.rs` — 强 / 弱资产句柄
  - `AssetHandle<T>` (强引用，Rc-like)
  - `WeakAssetHandle<T>` (弱引用)
  - 类型擦除：UUID + 类型 ID
- `engine-asset/src/registry.rs`
  - `AssetRegistry` 全局资产表
  - 加载 / 卸载 / 引用计数
- `engine-asset/src/loader/mod.rs`
  - `AssetLoader` trait
  - 实现：`GltfLoader`（已有，扩展）, `FbxLoader`, `TextureLoader`, `AudioLoader`
  - 异步加载（tokio / smol）
- `engine-asset/src/bundle.rs`
  - `AssetBundle` 资产包（.pak 格式）
  - LRU 内存缓存 + 磁盘回写
  - 压缩（zstd / lz4）

**验收：**
- 1 GB 资产包随机访问 < 10 ms
- 引用计数正确：卸载无泄漏
- 异步加载 UI 不卡顿

---

### 3.2 [02-asset-import.md](modules/02-asset-import.md) — 资产导入器

**核心交付：**
- `engine-asset/src/import/mod.rs`
  - `AssetImporter` trait
  - `gltf_importer`：网格 + 蒙皮 + 动画 + 材质 + 纹理
  - `fbx_importer`：同上（autodesk SDK 或自定义）
  - `texture_importer`：PNG / JPG / KTX2 / EXR
  - `audio_importer`：WAV / OGG / MP3
- `engine-asset/src/import/processor.rs`
  - 压缩：纹理 → KTX2 (BC7/ASTC)
  - LOD 生成：网格简化
  - Mipmap 自动生成
  - 立方贴图阵列化

**验收：**
- glTF 2.0 全特性支持（动画、蒙皮、PBR、IBL）
- 纹理导入：100 MB PNG → 10 MB KTX2 < 1 s
- 网格简化：100k 三角 → 1k 三角 < 500 ms（QEM 算法）

---

### 3.3 [03-serialization.md](modules/03-serialization.md) — 序列化

**核心交付：**
- `engine-asset/src/serde/scene.rs`
  - `SceneSerializer` 场景 → YAML
  - `SceneDeserializer` YAML → 场景
  - 人类可读：节点层级、组件字段、注释
  - 增量序列化（patch 模式）
- `engine-asset/src/serde/prefab.rs`
  - `Prefab` 预制件
  - `Variant` 重写系统（继承 + 覆盖）
- `engine-asset/src/serde/asset_meta.rs`
  - 资产 .meta 文件：UUID / 导入设置 / 依赖

**验收：**
- 1000 节点场景 YAML 序列化 < 50 ms
- YAML diff 友好：单元测试变更检测
- Prefab 继承深度 5 层不爆炸

---

### 3.4 [04-hot-reload.md](modules/04-hot-reload.md) — 热更新

**核心交付：**
- `engine-asset/src/hot_reload/watcher.rs`
  - `FileWatcher` 监听（notify crate）
  - 防抖：100 ms 窗口
- `engine-asset/src/hot_reload/reloader.rs`
  - `AssetReloader` 资产热重载
  - 场景热重载：保留实体 ID，刷新组件值
  - 着色器热重载：实时编译
- `engine-asset/src/hot_reload/scripting.rs`
  - 脚本热重载（仅在 dev 模式）

**验收：**
- 着色器修改 → 100 ms 内生效
- 场景保存 → 1 s 内运行时刷新
- 资产包版本切换无内存泄漏

---

### 3.5 [05-profiler.md](modules/05-profiler.md) — 性能分析器

**核心交付：**
- `engine-profiler/src/cpu.rs`
  - CPU 采样（rdtsc / chrono）
  - 函数级 profile 树
  - 火焰图（.folded 格式导出）
- `engine-profiler/src/gpu.rs`
  - GPU 时间戳查询（OpenGL / Vulkan）
  - Pass 级别时间统计
- `engine-profiler/src/memory.rs`
  - 分配器钩子
  - 泄漏检测
- `engine-profiler/src/frame.rs`
  - 帧时间直方图
  - 99 / 95 / 50 百分位
- `engine-profiler/src/export.rs`
  - Chrome Tracing / Tracy 格式导出

**验收：**
- CPU 采样开销 < 5%
- GPU 时间戳精度：1 ms
- 内存分配跟踪：零开销 fallback
- Chrome 浏览器打开火焰图

---

### 3.6 [06-cli-cicd.md](modules/06-cli-cicd.md) — 命令行工具与 CI

**核心交付：**
- `engine-cli/src/main.rs`
  - 命令：`engine build`, `engine run`, `engine profile`, `engine cook`, `engine test`
  - 配置文件：`engine.toml`
- `engine-cli/src/commands/build.rs`
  - 跨平台构建（windows / linux / macos / web）
  - Profile：dev / release / dist
- `engine-cli/src/commands/cook.rs`
  - 资产 cook：源资产 → 运行时资产包
  - 内容分发网络 (CDN) 友好的 pak 索引
- `engine-cli/src/commands/profile.rs`
  - 启动 profiler 录制
  - 输出 flamegraph.svg

**CI 模板：**
- `.github/workflows/ci.yml`
  - 矩阵：ubuntu / macos / windows
  - 步骤：lint / test / build / cook / bench
- `.github/workflows/release.yml`
  - 触发：tag push
  - 发布：GitHub Release + 资产包

**验收：**
- `engine build --release` < 5 min (CI)
- `engine cook` 输出可独立运行的 .pak
- CI 全绿：lint + 单元测试 + 集成测试 + 资产 cook + 基准

---

## 4. 验收清单（acceptance/test-plan.md）

- [ ] 1 GB 资产包随机访问 < 10 ms
- [ ] glTF 2.0 全特性导入（动画 / 蒙皮 / PBR / IBL）
- [ ] 100 MB PNG → KTX2 < 1 s
- [ ] 1000 节点场景 YAML 序列化 < 50 ms
- [ ] 着色器热重载 < 100 ms
- [ ] CPU profiler 开销 < 5%
- [ ] GPU 时间戳精度 1 ms
- [ ] `engine build --release` CI < 5 min
- [ ] `engine cook` 输出独立运行 .pak
- [ ] CI 矩阵：ubuntu / macos / windows 全绿
- [ ] `cargo test --workspace` 全通过
- [ ] `cargo bench --workspace` 基准存档
- [ ] 示例：`asset_pipeline_demo`, `hot_reload_demo`, `profiler_demo`

---

## 5. API 稳定承诺

```rust
// engine-asset
pub use handle::{AssetHandle, WeakAssetHandle};
pub use registry::AssetRegistry;
pub use loader::AssetLoader;
pub use bundle::AssetBundle;
pub use import::AssetImporter;
pub use serde::{SceneSerializer, SceneDeserializer, Prefab};

// engine-profiler
pub use cpu::CpuProfiler;
pub use gpu::GpuProfiler;
pub use memory::MemoryProfiler;
pub use frame::FrameProfiler;

// engine-cli
pub use main::run; // CLI 入口
```

---

## 6. 与上下游依赖

| 依赖 | 来自 | 用途 |
|------|------|------|
| `Mesh3D` / `Material3D` / `AnimationClip` | sprint-19, 21 | 资产导入产物 |
| `GpuDevice` | sprint-19 | GPU profiler 时间戳 |
| `Resource` / `Res` | sprint-18 | 资产注册到 ECS |
| 全部 | sprint-17-21 | 工具链围绕主引擎 |

---

## 7. 风险与缓解

| 风险 | 等级 | 缓解 |
|------|------|------|
| glTF/FBX 双格式维护 | 中 | 优先 glTF，FBX 走第三方 SDK |
| 资产包版本管理 | 高 | UUID 稳定 + 内容寻址 |
| 热重载运行时崩溃 | 中 | dev 模式 only，release 关闭 |
| Profiler 开销 | 中 | 关闭采样 fallback 到 0 开销 |
| CI 跨平台编译时间 | 中 | cargo cache + 增量 |
