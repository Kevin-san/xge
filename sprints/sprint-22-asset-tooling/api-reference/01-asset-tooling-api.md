# Sprint 22 · API 参考

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)

## Asset

```rust
// engine-asset
pub use handle::{AssetHandle, WeakAssetHandle, AssetUuid};
pub use registry::AssetRegistry;
pub use loader::AssetLoader;
pub use bundle::{AssetBundle, BundleIndex, CompressionType};
pub use import::{AssetImporter, ImportOptions, OptimizationTarget};
pub use serde::{SceneSerializer, SceneDeserializer, SerializationFormat, Prefab, AssetMeta};
pub use hot_reload::{FileWatcher, AssetReloader, ShaderReloader, SceneReloader};
```

## Profiler

```rust
// engine-profiler
pub use cpu::{CpuProfiler, Sample, ScopeGuard, profile_scope};
pub use gpu::{GpuProfiler, TimeQuery};
pub use memory::{MemoryProfiler, SiteStats, MemorySnapshot, LeakInfo};
pub use frame::{FrameProfiler, FrameStats};
pub use export::{export_chrome_tracing, ProfileFormat};
pub use tracy::TracyClient;
```

## CLI

```rust
// engine-cli
pub use main::run;  // CLI 入口
// 命令：build / run / profile / cook / test / clean
```

## 关键 API

```rust
impl AssetRegistry {
    pub fn load<T: Asset>(&self, path: &Path) -> AssetHandle<T>;
    pub fn load_async<T: Asset>(&self, path: &Path) -> AssetHandle<T>;
    pub fn get<T: Asset>(&self, uuid: AssetUuid) -> Option<AssetHandle<T>>;
    pub fn tick(&self);  // 每帧调用，更新异步加载
}

impl CpuProfiler {
    pub fn begin(&mut self, name: &str) -> ScopeGuard;
    pub fn end(&mut self, scope: ScopeGuard);
    pub fn export_chrome(&self, path: &Path) -> Result<(), Error>;
}

impl FileWatcher {
    pub fn watch(&mut self, path: &Path) -> Result<(), Error>;
    pub fn poll(&mut self) -> Vec<WatchEvent>;
}
```
