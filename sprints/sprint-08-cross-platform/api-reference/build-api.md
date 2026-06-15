# 构建 API 参考清单

## 概述

本文档列出 `engine-build` crate 的所有公开 API，包括函数签名、结构体、方法等。本文档覆盖 Sprint 08 的全部 378 条需求中的 API 相关部分。

---

## 1. engine-build crate 导出

```rust
// lib.rs
pub use crate::build::{BuildPipeline, BuildConfig, BuildArtifact, Profile};
pub use crate::platform::{PlatformTarget, MiniAppPlatform};
pub use crate::toolchain::*;
pub use crate::asset::{AssetPipeline, AssetManifest, AssetEntry, AssetKind};
pub use crate::compress::{AssetCompress, Compress};
pub use crate::encrypt::{AssetEncrypt, Encrypt};
pub use crate::hotupdate::{HotUpdate, HotUpdatePatch, FileChange};
pub use crate::package::{Package, PackageFormat};
pub use crate::signing::{Signing, SignInfo, AndroidKeystore, ProvisioningProfile};
pub use crate::logger::{BuildLogger, BuildStage, BuildProgress, BuildReport};
pub use crate::error::BuildError;
pub use crate::permission::Permission;
```

---

## 2. BuildPipeline 模块

### 模块路径

```rust
pub mod build;
```

### BuildPipeline

```rust
/// 构建管线主入口，协调编译、资源处理、打包、签名等环节
pub struct BuildPipeline {
    config: BuildConfig,
    logger: BuildLogger,
    progress: BuildProgress,
}

impl BuildPipeline {
    /// 创建新的构建管线实例
    pub fn new(config: BuildConfig) -> Result<Self>;
    
    /// 执行完整构建流程
    pub fn build(&self) -> Result<BuildArtifact>;
    
    /// 异步执行构建
    pub fn build_async(&self, sender: Progress) -> Result<()>;
    
    /// 清理构建产物
    pub fn clean(&self) -> Result<()>;
    
    /// 构建并运行（仅本机目标）
    pub fn run(&self) -> Result<()>;
    
    /// 获取构建配置
    pub fn config(&self) -> &BuildConfig;
    
    /// 获取目标平台
    pub fn platform_target(&self) -> PlatformTarget;
    
    /// 获取构建配置
    pub fn profile(&self) -> Profile;
}
```

### BuildConfig

```rust
/// 构建配置，包含应用信息、平台目标、资源目录等
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildConfig {
    pub app_name: String,
    pub app_id: String,
    pub version: String,
    pub version_code: i32,
    pub icons: Vec<PathBuf>,
    pub splash_screen: Option<PathBuf>,
    pub permissions: Vec<Permission>,
    pub orientation: Orientation,
    pub platform_target: PlatformTarget,
    pub profile: Profile,
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
    pub temp_dir: PathBuf,
}

/// 屏幕方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
    Auto,
}

impl BuildConfig {
    /// 创建默认配置
    pub fn new() -> Self;
    pub fn default() -> Self;
    
    /// 获取应用名称
    pub fn app_name(&self) -> &str;
    
    /// 获取应用标识符
    pub fn app_id(&self) -> &str;
    
    /// 获取版本号
    pub fn version(&self) -> &str;
    
    /// 获取版本代码
    pub fn version_code(&self) -> i32;
    
    /// 获取图标列表
    pub fn icons(&self) -> &[PathBuf];
    
    /// 获取启动画面
    pub fn splash(&self) -> Option<&PathBuf>;
    
    /// 获取权限列表
    pub fn permissions(&self) -> &[Permission];
    
    /// 获取屏幕方向
    pub fn orientation(&self) -> Orientation;
    
    /// 获取输出目录
    pub fn output_dir(&self) -> &Path;
    
    /// 获取临时目录
    pub fn temp_dir(&self) -> &Path;
    
    /// 获取资源目录
    pub fn assets_dir(&self) -> &Path;
    
    /// 获取目标平台
    pub fn platform_target(&self) -> PlatformTarget;
    
    /// 获取构建配置
    pub fn profile(&self) -> Profile;
    
    /// 从 TOML 文件加载
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self>;
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self>;
    
    /// 序列化为 TOML
    pub fn to_toml(&self) -> String;
    
    /// 保存到文件
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()>;
    
    /// 设置资源目录
    pub fn with_assets_dir(mut self, dir: impl AsRef<Path>) -> Self;
    
    /// 设置输出目录
    pub fn with_output_dir(mut self, dir: impl AsRef<Path>) -> Self;
    
    /// 设置临时目录
    pub fn with_temp_dir(mut self, dir: impl AsRef<Path>) -> Self;
}
```

### Profile

```rust
/// 构建配置（Debug/Release/Ship）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Profile {
    #[default]
    Debug,
    Release,
    Ship,
}

impl Profile {
    /// 获取优化级别
    pub fn optimization_level(&self) -> u8;
    
    /// 获取优化级别字符串
    pub fn opt_level(&self) -> String;
    
    /// 是否包含调试信息
    pub fn debug_info(&self) -> bool;
    pub fn debug(&self) -> bool;
    
    /// 是否剥离符号
    pub fn strip_symbols(&self) -> bool;
    pub fn strip(&self) -> bool;
    
    /// 是否启用 LTO
    pub fn lto(&self) -> bool;
    
    /// 获取 Cargo 参数
    pub fn cargo_args(&self) -> Vec<String>;
}
```

### BuildArtifact

```rust
/// 构建产物信息
#[derive(Debug, Clone)]
pub struct BuildArtifact {
    pub path: PathBuf,
    pub size: u64,
    pub platform: PlatformTarget,
    pub version: String,
    pub sign_info: Option<SignInfo>,
}

impl BuildArtifact {
    /// 获取产物路径
    pub fn path(&self) -> &Path;
    
    /// 获取产物大小
    pub fn size(&self) -> u64;
    
    /// 获取目标平台
    pub fn platform(&self) -> PlatformTarget;
    
    /// 获取版本
    pub fn version(&self) -> &str;
    
    /// 获取签名信息
    pub fn sign_info(&self) -> Option<&SignInfo>;
}
```

---

## 3. PlatformTarget 平台目标

### 模块路径

```rust
pub mod platform;
```

### PlatformTarget

```rust
/// 目标平台枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformTarget {
    Windows,
    MacOS,
    Linux,
    Android,
    Ios,
    Web,
    MiniApp(MiniAppPlatform),
}

/// 小程序平台
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MiniAppPlatform {
    WeChat,
    ByteDance,
    QQ,
}

impl PlatformTarget {
    /// 获取当前主机平台
    pub fn current() -> PlatformTarget;
    
    /// 判断当前主机是否支持构建此目标
    pub fn supported(&self) -> bool;
}
```

---

## 4. Toolchain 工具链

### 模块路径

```rust
pub mod toolchain;
```

### Toolchain

```rust
/// 工具链信息
#[derive(Debug, Clone)]
pub struct Toolchain {
    pub rust_version: Version,
    pub ndk_version: Option<Version>,
    pub xcode_version: Option<Version>,
    pub node_version: Option<Version>,
}

impl Toolchain {
    pub fn rust_version(&self) -> &Version;
    pub fn ndk_version(&self) -> Option<&Version>;
    pub fn xcode_version(&self) -> Option<&Version>;
    pub fn node_version(&self) -> Option<&Version>;
    pub fn detect() -> Result<Self>;
}
```

### AndroidToolchain

```rust
/// Android 构建工具链
pub struct AndroidToolchain {
    ndk_path: PathBuf,
    sdk_path: PathBuf,
    jdk_path: PathBuf,
}

impl AndroidToolchain {
    pub fn detect() -> Result<Self>;
    pub fn build(&self, config: &BuildConfig) -> Result<PathBuf>;
    pub fn abi(&self) -> Vec<String>;
    pub fn min_sdk(&self) -> u32;
    pub fn sign_apk(&self, apk: impl AsRef<Path>, keystore: &AndroidKeystore) -> Result<PathBuf>;
    pub fn zipalign(&self, apk: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn aapt2_package(&self, resources: impl AsRef<Path>, manifest: impl AsRef<Path>) -> Result<PathBuf>;
}
```

### AndroidManifest

```rust
/// Android AndroidManifest.xml 生成器
pub struct AndroidManifest {
    config: BuildConfig,
}

impl AndroidManifest {
    pub fn new(config: &BuildConfig) -> Self;
    pub fn to_xml(&self) -> String;
    pub fn permissions(&self) -> Vec<String>;
    pub fn min_sdk(&self) -> u32;
    pub fn target_sdk(&self) -> u32;
    pub fn orientation(&self) -> &str;
    pub fn activity_name(&self) -> &str;
    pub fn activity(&self) -> &str;
    pub fn intent_filters(&self) -> Vec<String>;
}
```

### IosToolchain

```rust
/// iOS 构建工具链
pub struct IosToolchain {
    xcode_path: PathBuf,
    developer_dir: PathBuf,
}

impl IosToolchain {
    pub fn detect() -> Result<Self>;
    pub fn build(&self, config: &BuildConfig) -> Result<PathBuf>;
    pub fn xcodebuild(&self, proj: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn codesign(&self, app: impl AsRef<Path>, profile: &ProvisioningProfile) -> Result<()>;
}
```

### InfoPlist

```rust
/// iOS Info.plist 生成器
pub struct InfoPlist {
    config: BuildConfig,
}

impl InfoPlist {
    pub fn new(config: &BuildConfig) -> Self;
    pub fn to_plist(&self) -> Value;
    pub fn to_string(&self) -> String;
    pub fn bundle_id(&self) -> &str;
    pub fn version(&self) -> &str;
    pub fn required_device_capabilities(&self) -> Vec<String>;
}
```

### WebToolchain

```rust
/// Web/WASM 构建工具链
pub struct WebToolchain {
    wasm_bindgen_path: PathBuf,
    wasm_opt_path: Option<PathBuf>,
}

impl WebToolchain {
    pub fn detect() -> Result<Self>;
    pub fn build_wasm(&self, config: &BuildConfig) -> Result<PathBuf>;
    pub fn rustc_to_wasm(&self, src: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn wasm_bindgen(&self, wasm: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>>;
    pub fn wasm_opt(&self, src: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn generate_html(&self) -> Result<PathBuf>;
    pub fn generate_sw(&self) -> Result<PathBuf>;
    pub fn generate_manifest(&self) -> Result<PathBuf>;
}
```

### MiniAppToolchain

```rust
/// 小程序构建工具链
pub struct MiniAppToolchain {
    platform: MiniAppPlatform,
    node_path: PathBuf,
}

impl MiniAppToolchain {
    pub fn new(platform: MiniAppPlatform) -> Self;
    pub fn build(&self, config: &BuildConfig, wasm_or_js: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn pack(&self, src_dir: impl AsRef<Path>, out_file: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn minify_js(&self, src: impl AsRef<Path>) -> Result<String>;
    pub fn minify(&self, js: &str) -> Result<String>;
    pub fn generate_manifest(&self, config: &BuildConfig) -> Result<String>;
    pub fn generate_app_json(&self) -> String;
    pub fn generate_project_config(&self) -> String;
    pub fn generate_game_js(&self) -> String;
    pub fn generate_app_js(&self) -> Result<String>;
    pub fn miniapp_platform(&self) -> MiniAppPlatform;
}
```

---

## 5. AssetPipeline 资源管线

### 模块路径

```rust
pub mod asset;
```

### AssetPipeline

```rust
/// 资源处理管线
pub struct AssetPipeline {
    asset_dir: PathBuf,
    entries: Vec<AssetEntry>,
    cache: BuildCache,
}

impl AssetPipeline {
    pub fn new(asset_dir: impl AsRef<Path>) -> Self;
    pub fn scan(&mut self) -> Result<()>;
    pub fn import_all(&mut self) -> Result<()>;
    pub fn import(&mut self) -> Result<()>;
    pub fn reimport_changed(&mut self) -> Result<()>;
    pub fn process_all(&mut self) -> Result<()>;
    pub fn process(&mut self) -> Result<()>;
    pub fn package(&mut self, out_dir: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn build_manifest(&self) -> AssetManifest;
    pub fn incremental_hash(&self) -> String;
    pub fn diff(&self, from_manifest: &AssetManifest, to_manifest: &AssetManifest) -> DiffResult;
    pub fn changed_files(&self, since: SystemTime) -> Vec<PathBuf>;
    pub fn incremental_build(&mut self, since: SystemTime) -> Result<AssetManifest>;
}
```

### AssetManifest

```rust
/// 资源清单
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssetManifest {
    version: String,
    entries: Vec<AssetEntry>,
}

impl AssetManifest {
    pub fn new() -> Self;
    pub fn add(&mut self, entry: AssetEntry);
    pub fn entries(&self) -> &[AssetEntry];
    pub fn to_json(&self) -> String;
    pub fn from_json(json: &str) -> Result<Self>;
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()>;
    pub fn load(path: impl AsRef<Path>) -> Result<Self>;
    pub fn diff(&self, other: &AssetManifest) -> DiffResult;
}
```

### AssetEntry

```rust
/// 资源条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssetEntry {
    pub path: PathBuf,
    pub hash: String,
    pub size: u64,
    pub kind: AssetKind,
    pub dependencies: Vec<PathBuf>,
}

impl AssetEntry {
    pub fn path(&self) -> &Path;
    pub fn hash(&self) -> &str;
    pub fn size(&self) -> u64;
    pub fn kind(&self) -> AssetKind;
}
```

### AssetKind

```rust
/// 资源类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AssetKind {
    Texture,
    Audio,
    Model,
    Scene,
    Prefab,
    Font,
    Custom(String),
}

impl AssetKind {
    pub fn as_str(&self) -> &str;
}
```

### DiffResult

```rust
/// 差异结果
#[derive(Debug, Clone, Default)]
pub struct DiffResult {
    pub added: Vec<AssetEntry>,
    pub modified: Vec<AssetEntry>,
    pub removed: Vec<PathBuf>,
}

impl DiffResult {
    pub fn is_empty(&self) -> bool;
    pub fn total_changes(&self) -> usize;
}
```

---

## 6. Compress 压缩

### 模块路径

```rust
pub mod compress;
```

### AssetCompress

```rust
/// 压缩算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AssetCompress {
    #[default]
    None,
    Zstd,
    Gzip,
    Brotli,
    LZ4,
}
```

### Compress

```rust
/// 压缩工具
pub struct Compress;

impl Compress {
    pub fn zstd(bytes: &[u8], level: i32) -> Result<Vec<u8>>;
    pub fn gzip(bytes: &[u8], level: i32) -> Result<Vec<u8>>;
    pub fn brotli(bytes: &[u8], level: u32) -> Result<Vec<u8>>;
    pub fn lz4(bytes: &[u8]) -> Result<Vec<u8>>;
    pub fn decompress(bytes: &[u8], algo: AssetCompress) -> Result<Vec<u8>>;
}
```

---

## 7. Encrypt 加密

### 模块路径

```rust
pub mod encrypt;
```

### AssetEncrypt

```rust
/// 加密算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AssetEncrypt {
    #[default]
    None,
    AesGcm128,
    AesGcm256,
    XorChaCha20,
}
```

### Encrypt

```rust
/// 加密工具
pub struct Encrypt;

impl Encrypt {
    pub fn aes_gcm_128(bytes: &[u8], key: &[u8; 16], nonce: &[u8; 12]) -> Result<Vec<u8>>;
    pub fn aes_gcm_256(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Result<Vec<u8>>;
    pub fn chacha20(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 24]) -> Result<Vec<u8>>;
    pub fn decrypt(bytes: &[u8], key: &[u8], nonce: &[u8], algo: AssetEncrypt) -> Result<Vec<u8>>;
}
```

---

## 8. HotUpdate 热更新

### 模块路径

```rust
pub mod hotupdate;
```

### HotUpdate

```rust
/// 热更新工具
pub struct HotUpdate;

impl HotUpdate {
    pub fn diff(old_manifest: &AssetManifest, new_manifest: &AssetManifest) -> HotUpdatePatch;
    pub fn apply(current_dir: impl AsRef<Path>, patch: &HotUpdatePatch) -> Result<()>;
}
```

### HotUpdatePatch

```rust
/// 热更新补丁
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HotUpdatePatch {
    pub version: String,
    pub new_manifest: AssetManifest,
    pub file_changes: Vec<FileChange>,
    pub size_bytes: u64,
}

impl HotUpdatePatch {
    pub fn new(version: String, new_manifest: AssetManifest, file_changes: Vec<FileChange>) -> Self;
    pub fn version(&self) -> &str;
    pub fn new_manifest(&self) -> &AssetManifest;
    pub fn changes(&self) -> &[FileChange];
    pub fn size_bytes(&self) -> u64;
    pub fn to_bytes(&self) -> Result<Vec<u8>>;
    pub fn from_bytes(bytes: &[u8]) -> Result<Self>;
}
```

### FileChange

```rust
/// 文件变更
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FileChange {
    Added {
        path: PathBuf,
        size: u64,
        hash: String,
    },
    Modified {
        path: PathBuf,
        diff: Vec<u8>,
        size: u64,
    },
    Removed {
        path: PathBuf,
    },
}

impl FileChange {
    pub fn path(&self) -> &Path;
    pub fn size(&self) -> u64;
    pub fn is_added(&self) -> bool;
    pub fn is_modified(&self) -> bool;
    pub fn is_removed(&self) -> bool;
}
```

---

## 9. Package 打包

### 模块路径

```rust
pub mod package;
```

### PackageFormat

```rust
/// 打包格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Dir,
    Zip,
    Apk,
    Ipa,
    Wasm,
    MiniApp,
}
```

### Package

```rust
/// 打包工具
pub struct Package {
    output_dir: PathBuf,
    files: HashMap<PathBuf, Vec<u8>>,
    manifest: Option<AssetManifest>,
}

impl Package {
    pub fn new(output_dir: impl AsRef<Path>) -> Result<Self>;
    pub fn add_file(&mut self, pkg_path: impl AsRef<Path>, bytes: impl Into<Vec<u8>>);
    pub fn add_directory(&mut self, prefix: impl AsRef<Path>, dir: impl AsRef<Path>) -> Result<()>;
    pub fn add_manifest(&mut self, manifest: AssetManifest);
    pub fn build(&self, format: PackageFormat) -> Result<BuildArtifact>;
    pub fn build_zip(&self, out: impl AsRef<Path>) -> Result<BuildArtifact>;
    pub fn build_dir(&self, out: impl AsRef<Path>) -> Result<BuildArtifact>;
    pub fn format(&self) -> PackageFormat;
}
```

---

## 10. Signing 签名

### 模块路径

```rust
pub mod signing;
```

### SignInfo

```rust
/// 签名信息
#[derive(Debug, Clone)]
pub struct SignInfo {
    pub signature: Vec<u8>,
    pub certificate: Vec<u8>,
    pub timestamp: Option<String>,
}
```

### AndroidKeystore

```rust
/// Android 密钥库
pub struct AndroidKeystore {
    pub path: PathBuf,
    pub alias: String,
    pub password: String,
}
```

### ProvisioningProfile

```rust
/// iOS 配置文件
pub struct ProvisioningProfile {
    pub name: String,
    pub team_id: String,
    pub app_id: String,
    pub entitlements: HashMap<String, String>,
}
```

### Signing

```rust
/// 签名工具
pub struct Signing;

impl Signing {
    pub fn android_sign(unsigned_apk: impl AsRef<Path>, keystore: &AndroidKeystore) -> Result<PathBuf>;
    pub fn android_verify(apk: impl AsRef<Path>) -> bool;
}
```

---

## 11. Logger 日志

### 模块路径

```rust
pub mod logger;
```

### BuildLogger

```rust
/// 构建日志
pub struct BuildLogger {
    verbose: bool,
}

impl BuildLogger {
    pub fn new(verbose: bool) -> Self;
    pub fn info(&self, msg: &str);
    pub fn warn(&self, msg: &str);
    pub fn error(&self, msg: &str);
    pub fn progress(&self, percent: u8, msg: &str);
}
```

### BuildStage

```rust
/// 构建阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildStage {
    Init,
    Compile,
    ProcessAssets,
    Package,
    Sign,
    Done,
}
```

### BuildProgress

```rust
/// 构建进度
pub struct BuildProgress {
    stage: BuildStage,
    percent: u8,
}

impl BuildProgress {
    pub fn new() -> Self;
    pub fn set_stage(&mut self, stage: BuildStage);
    pub fn set_percent(&mut self, percent: u8);
}
```

### BuildReport

```rust
/// 构建报告
pub struct BuildReport {
    stages: Vec<StageReport>,
    warnings: u32,
    errors: u32,
}

impl BuildReport {
    pub fn new() -> Self;
    pub fn add_stage(&mut self, name: &str, duration: Duration, size: u64);
    pub fn to_html(&self) -> String;
    pub fn save_html(&self, path: impl AsRef<Path>) -> Result<()>;
    pub fn total_duration(&self) -> Duration;
    pub fn total_size(&self) -> u64;
    pub fn warnings(&self) -> u32;
    pub fn errors(&self) -> u32;
}
```

---

## 12. Permission 权限

### 模块路径

```rust
pub mod permission;
```

### Permission

```rust
/// 运行时权限
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Internet,
    Storage,
    Camera,
    Microphone,
    Location,
    Bluetooth,
    NFC,
}

impl Permission {
    pub fn to_android_string(&self) -> &'static str;
    pub fn to_ios_string(&self) -> &'static str;
}
```

---

## 13. BuildCache 缓存

### 模块路径

```rust
pub mod cache;
```

### BuildCache

```rust
/// 构建缓存
pub struct BuildCache {
    cache_dir: PathBuf,
    index: HashMap<String, PathBuf>,
}

impl BuildCache {
    pub fn new(cache_dir: impl AsRef<Path>) -> Result<Self>;
    pub fn hash(&self, key: &str) -> String;
    pub fn get(&self, key: &str) -> Option<PathBuf>;
    pub fn put(&mut self, key: &str, path: impl AsRef<Path>);
    pub fn clean(&mut self);
}
```

---

## 14. Hash 哈希

### 模块路径

```rust
pub mod hash;
```

### Hash

```rust
/// 哈希计算工具
pub struct Hash;

impl Hash {
    pub fn sha256(bytes: &[u8]) -> String;
    pub fn hash_file(path: impl AsRef<Path>) -> Result<String>;
    pub fn hash_dir(path: impl AsRef<Path>) -> Result<String>;
}
```

---

## 15. Error 错误

### 模块路径

```rust
pub mod error;
```

### BuildError

```rust
/// 构建错误
#[derive(Debug)]
pub struct BuildError {
    pub code: String,
    pub message: String,
    pub stage: Option<BuildStage>,
    pub file: Option<PathBuf>,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl std::error::Error for BuildError {}
```

### BuildWarning

```rust
/// 构建警告
#[derive(Debug)]
pub struct BuildWarning {
    pub code: String,
    pub message: String,
    pub file: Option<PathBuf>,
}
```

---

## 16. TextureProcessor 纹理处理

### 模块路径

```rust
pub mod texture;
```

### TextureProcessor

```rust
/// 纹理处理器
pub struct TextureProcessor;

impl TextureProcessor {
    pub fn compress_bc(img: &Image) -> Result<Vec<u8>>;
    pub fn compress_etc(img: &Image) -> Result<Vec<u8>>;
    pub fn compress_astc(img: &Image) -> Result<Vec<u8>>;
    pub fn mipmap(img: &Image) -> Result<Vec<u8>>;
    pub fn generate_mipmaps(img: &Image) -> Result<Vec<u8>>;
    pub fn resize(img: &Image, w: u32, h: u32) -> Result<Image>;
    pub fn pack_atlas(images: &[Image], padding: u32) -> Result<(Image, Vec<Rect>)>;
}
```

---

## 17. AudioProcessor 音频处理

### 模块路径

```rust
pub mod audio;
```

### AudioProcessor

```rust
/// 音频处理器
pub struct AudioProcessor;

impl AudioProcessor {
    pub fn convert(src: &[u8], format: AudioFormat) -> Result<Vec<u8>>;
    pub fn compress_ogg(src: &[u8]) -> Result<Vec<u8>>;
    pub fn compress_mp3(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_ogg(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_mp3(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_wav(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_flac(src: &[u8]) -> Result<Vec<u8>>;
}
```

---

## 18. ModelProcessor 模型处理

### 模块路径

```rust
pub mod model;
```

### ModelProcessor

```rust
/// 模型处理器
pub struct ModelProcessor;

impl ModelProcessor {
    pub fn optimize(src: &[u8]) -> Result<Vec<u8>>;
    pub fn glb_to_optimized(src: &[u8]) -> Result<Vec<u8>>;
    pub fn gltf_to_optimized(src_dir: impl AsRef<Path>) -> Result<Vec<u8>>;
}
```

---

## 19. SceneProcessor 场景处理

### 模块路径

```rust
pub mod scene;
```

### SceneProcessor

```rust
/// 场景处理器
pub struct SceneProcessor;

impl SceneProcessor {
    pub fn bake(scene: &Scene) -> Result<Vec<u8>>;
}
```

---

## 20. CLI

### 模块路径

```rust
pub mod cli;
```

### EngineCLI

```rust
/// Engine CLI 主入口
pub struct EngineCLI {
    name: String,
    version: String,
}

impl EngineCLI {
    pub fn new() -> Self;
    pub fn run(&self, args: Vec<String>) -> Result<()>;
}
```

---

## 21. 公开 API 覆盖率

根据需求编号 REQ-197：

> 公开 API doc comment 覆盖率 100%

所有公开的函数、结构体、枚举都必须包含完整的文档注释。

### 文档注释示例

```rust
/// 创建新的构建管线实例
///
/// # Arguments
///
/// * `config` - 构建配置对象
///
/// # Example
///
/// ```
/// let config = BuildConfig::default();
/// let pipeline = BuildPipeline::new(config).unwrap();
/// ```
///
/// # Errors
///
/// 如果配置无效或初始化失败，返回错误。
pub fn new(config: BuildConfig) -> Result<Self>;
```
