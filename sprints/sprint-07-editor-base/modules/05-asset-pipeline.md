# 模块五：资源管线需求

## 1. 模块概述

资源管线负责编辑器侧资源的导入、管理、元数据维护，包括 AssetMeta 资源元数据、AssetImporter 导入器 trait、各种资源类型导入器（Texture/Font/Scene/Prefab/Model）、AssetPipeline 管线调度、AssetDB 数据库管理。

## 2. 功能需求

### 2.1 AssetMeta 资源元数据

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 103 | `AssetMeta`：资源元数据（importer 设置/GUID） | P0 |
| 275 | `AssetMeta::new(guid, path, importer_settings)` | P0 |
| 276 | `AssetMeta::save(&self, path)` | P0 |
| 277 | `AssetMeta::load(path) -> Result<Self>` | P0 |

### 2.2 AssetImporter 导入器

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 104 | `AssetImporter`：PNG/JPG/TTF/FBX/GLB/自定义 | P0 |
| 278 | `AssetImporter` trait：`can_import(&self, ext) -> bool` / `import(&self, path) -> Result<Asset>` | P0 |
| 279 | `TextureImporter`：PNG/JPG | P0 |
| 280 | `FontImporter`：TTF/OTF | P0 |
| 281 | `SceneImporter`：自定义 JSON | P0 |
| 282 | `PrefabImporter`：自定义 JSON | P0 |
| 283 | `ModelImporter`：GLB/GLTF（后续完善） | P1 |

### 2.3 AssetPipeline 管线

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 105 | `AssetPipeline::import_all()` 扫描资源并导入 | P0 |
| 106 | `AssetPipeline::reimport_changed()` 增量导入 | P0 |
| 284 | `AssetPipeline::new(asset_dir)` | P0 |
| 285 | `AssetPipeline::scan(&mut self)` 扫描目录 | P0 |
| 286 | `AssetPipeline::import_all(&mut self)` | P0 |
| 287 | `AssetPipeline::reimport_changed(&mut self)` 基于 mtime | P0 |
| 288 | `AssetPipeline::assets(&self) -> &[AssetInfo]` | P0 |
| 289 | `AssetInfo::path / meta / size / mtime` | P0 |

### 2.4 AssetDB 数据库

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 290 | `AssetDB`：全局数据库 | P0 |

### 2.5 缩略图

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 107 | 缩略图生成：为模型/场景生成缩略图（初版占位，后续完善） | P2 |

### 2.6 场景/预制体导入导出

| 需求编号 | 功能描述 | 优先级 |
|---------|---------|--------|
| 130 | 场景文件格式：`*.scene.json` / `*.scene.bin` | P0 |
| 131 | `PrefabSaver::save(prefab, path)` | P0 |
| 132 | `PrefabLoader::load(path) -> Prefab` | P0 |
| 303 | `SceneSaver::save_json(scene, path)` | P0 |
| 304 | `SceneSaver::save_bin(scene, path)` | P0 |
| 305 | `SceneLoader::load_json(path) -> Result<SceneTree>` | P0 |
| 306 | `SceneLoader::load_bin(path) -> Result<SceneTree>` | P0 |
| 307 | `PrefabSaver::save_json(prefab, path)` | P0 |
| 308 | `PrefabSaver::save_bin(prefab, path)` | P0 |
| 309 | `PrefabLoader::load_json(path) -> Result<Prefab>` | P0 |
| 310 | `PrefabLoader::load_bin(path) -> Result<Prefab>` | P0 |
| 311 | 文件格式包含：version / nodes / components / assets_ref | P0 |
| 312 | 序列化兼容：旧版本可打开 | P1 |

## 3. API 签名

### 3.1 AssetMeta

```rust
pub struct AssetMeta {
    pub guid: Guid,
    pub path: PathBuf,
    pub importer_type: String,
    pub importer_settings: HashMap<String, Value>,
    pub imported_at: DateTime<Utc>,
}

impl AssetMeta {
    pub fn new(guid: Guid, path: PathBuf, importer_settings: HashMap<String, Value>) -> Self;
    pub fn save(&self, path: &Path) -> Result<()>;
    pub fn load(path: &Path) -> Result<Self>;
}
```

### 3.2 AssetImporter Trait

```rust
pub trait AssetImporter: Send + Sync {
    fn can_import(&self, ext: &str) -> bool;
    fn import(&self, path: &Path) -> Result<Asset>;
    fn name(&self) -> &str;
}
```

### 3.3 AssetPipeline

```rust
pub struct AssetPipeline {
    asset_dir: PathBuf,
    importers: Vec<Box<dyn AssetImporter>>,
    assets: Vec<AssetInfo>,
}

pub struct AssetInfo {
    pub path: PathBuf,
    pub meta: Option<AssetMeta>,
    pub size: u64,
    pub mtime: DateTime<Utc>,
}

impl AssetPipeline {
    pub fn new(asset_dir: PathBuf) -> Self;
    pub fn register_importer(&mut self, importer: Box<dyn AssetImporter>);
    pub fn scan(&mut self) -> Result<()>;
    pub fn import_all(&mut self) -> Result<()>;
    pub fn reimport_changed(&mut self) -> Result<()>;
    pub fn assets(&self) -> &[AssetInfo];
}
```

### 3.4 AssetDB

```rust
pub struct AssetDB {
    assets: HashMap<Guid, Asset>,
    path_to_guid: HashMap<PathBuf, Guid>,
}

impl AssetDB {
    pub fn instance() -> &'static mut Self;
    pub fn get(&self, guid: Guid) -> Option<&Asset>;
    pub fn get_by_path(&self, path: &Path) -> Option<Guid>;
    pub fn register(&mut self, guid: Guid, asset: Asset, path: PathBuf);
    pub fn load_asset(&mut self, path: &Path) -> Result<Guid>;
}
```

### 3.5 SceneSaver/SceneLoader

```rust
pub struct SceneSaver;

impl SceneSaver {
    pub fn save_json(scene: &SceneTree, path: &Path) -> Result<()>;
    pub fn save_bin(scene: &SceneTree, path: &Path) -> Result<()>;
}

pub struct SceneLoader;

impl SceneLoader {
    pub fn load_json(path: &Path) -> Result<SceneTree>;
    pub fn load_bin(path: &Path) -> Result<SceneTree>;
}
```

### 3.6 PrefabSaver/PrefabLoader

```rust
pub struct PrefabSaver;

impl PrefabSaver {
    pub fn save_json(prefab: &Prefab, path: &Path) -> Result<()>;
    pub fn save_bin(prefab: &Prefab, path: &Path) -> Result<()>;
}

pub struct PrefabLoader;

impl PrefabLoader {
    pub fn load_json(path: &Path) -> Result<Prefab>;
    pub fn load_bin(path: &Path) -> Result<Prefab>;
}
```

## 4. 输入/输出

| 操作 | 输入 | 输出 |
|-----|-----|-----|
| AssetMeta::new | guid, path, settings | AssetMeta 实例 |
| AssetImporter::import | file path | Asset 实例 |
| AssetPipeline::scan | - | 扫描 asset_dir |
| AssetPipeline::import_all | - | 导入所有资源 |
| AssetPipeline::reimport_changed | - | 增量导入变更资源 |
| SceneSaver::save_json | scene, path | JSON 文件 |
| SceneLoader::load_json | path | SceneTree |
| PrefabSaver::save_json | prefab, path | JSON 文件 |
| PrefabLoader::load_json | path | Prefab |

## 5. 验收标准

- [ ] AssetMeta 可保存/加载
- [ ] TextureImporter 可导入 PNG/JPG
- [ ] FontImporter 可导入 TTF/OTF
- [ ] SceneImporter 可导入自定义 JSON 场景
- [ ] PrefabImporter 可导入自定义 JSON 预制体
- [ ] AssetPipeline::scan 可扫描 assets/ 目录
- [ ] AssetPipeline::import_all 导入所有资源
- [ ] AssetPipeline::reimport_changed 增量导入
- [ ] AssetDB 全局单例可用
- [ ] 场景可保存为 JSON/BIN 格式
- [ ] 场景可从 JSON/BIN 格式加载
- [ ] 预制体可保存为 JSON/BIN 格式
- [ ] 预制体可从 JSON/BIN 格式加载
- [ ] 支持资源元数据 .meta 文件
- [ ] 文件格式包含 version/nodes/components/assets_ref

## 6. 依赖关系

- 依赖引擎资源系统
- 依赖引擎序列化系统
- 依赖文件系统

## 7. 优先级

| 优先级 | 说明 |
|-------|------|
| P0 | 核心功能，必须完成 |
| P1 | 重要功能，应完成 |
| P2 | 增强功能，可后续完善 |
