# 模块一：构建管线（BuildPipeline）

## 1. 模块概述

构建管线模块是 `engine-build` crate 的核心组件，负责协调整个构建流程，包括编译、资源处理、打包、签名和输出等环节。该模块提供统一的构建接口，支持多平台目标的构建需求。

### 核心职责

- 提供 `BuildPipeline` 作为构建流程的统一入口
- 管理 `BuildConfig` 配置，支持 TOML 文件格式的持久化
- 定义 `Profile` 构建配置（Debug/Release/Ship）
- 生成 `BuildArtifact` 构建产物信息
- 提供构建日志、进度和报告功能

### 需求来源

对应原文档需求编号：**1-56, 173-238**

---

## 2. engine-build crate 建立

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-001 | `engine-build` crate 建立 |

### API 签名

```toml
# Cargo.toml
[package]
name = "engine-build"
version = "0.8.0"
edition = "2021"
```

### 验收标准

- [ ] `cargo build -p engine-build` 成功
- [ ] 模块结构清晰，包含合理的子模块划分

### 依赖关系

- 依赖 `engine-core` crate
- 依赖标准库 `toml` / `serde` / `sha2` 等

### 优先级

**P0**

---

## 3. BuildPipeline 核心接口

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-002 | `BuildPipeline::new(config)` 创建构建管线实例 |
| REQ-003 | `BuildPipeline::build(&self) -> Result<BuildArtifact>` 执行完整构建流程 |
| REQ-004 | `BuildPipeline::clean(&self)` 清理构建产物 |
| REQ-005 | `BuildPipeline::run(&self)` 构建并运行（仅本机目标） |
| REQ-173 | `BuildPipeline::config(&self) -> &BuildConfig` 获取构建配置 |
| REQ-174 | `BuildPipeline::build_async(&self, sender: Progress)` 异步执行构建 |
| REQ-175 | `BuildPipeline::platform_target(&self) -> PlatformTarget` 获取目标平台 |
| REQ-176 | `BuildPipeline::profile(&self) -> Profile` 获取构建配置 |

### API 签名

```rust
pub struct BuildPipeline {
    config: BuildConfig,
    logger: BuildLogger,
    progress: BuildProgress,
}

impl BuildPipeline {
    pub fn new(config: BuildConfig) -> Result<Self>;
    pub fn build(&self) -> Result<BuildArtifact>;
    pub fn build_async(&self, sender: Progress) -> Result<()>;
    pub fn clean(&self) -> Result<()>;
    pub fn run(&self) -> Result<()>;
    pub fn config(&self) -> &BuildConfig;
    pub fn platform_target(&self) -> PlatformTarget;
    pub fn profile(&self) -> Profile;
}
```

### 输入

- `config: BuildConfig` - 构建配置对象

### 输出

- `Result<BuildArtifact>` - 构建成功返回产物信息

### 验收标准

- [ ] `BuildPipeline::new(config)` 正确初始化
- [ ] `build()` 执行完整构建流程
- [ ] `clean()` 正确清理产物
- [ ] `run()` 在本机目标可执行
- [ ] 异步构建接口正常工作

### 依赖关系

- 依赖 `Compiler` 编译模块
- 依赖 `AssetPipeline` 资源管线
- 依赖 `Package` 打包模块
- 依赖 `Signing` 签名模块

### 优先级

**P0**

---

## 4. PlatformTarget 平台目标

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-006 | `PlatformTarget::Windows / MacOS / Linux / Android / Ios / Web / MiniApp` 枚举 |
| REQ-007 | `PlatformTarget::current() -> PlatformTarget` 获取当前主机平台 |
| REQ-008 | `PlatformTarget::supported(&self) -> bool` 判断是否支持构建此目标 |
| REQ-279 | `MiniAppPlatform::WeChat / ByteDance / QQ` 小程序平台枚举 |

### API 签名

```rust
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MiniAppPlatform {
    WeChat,
    ByteDance,
    QQ,
}

impl PlatformTarget {
    pub fn current() -> PlatformTarget;
    pub fn supported(&self) -> bool;
}
```

### 输入

- 无（静态方法）

### 输出

- `PlatformTarget` 平台枚举值

### 验收标准

- [ ] 正确识别当前主机平台
- [ ] 跨平台编译时正确判断支持状态
- [ ] 小程序平台枚举正确

### 依赖关系

- 依赖标准库 `std::env`

### 优先级

**P0**

---

## 5. Profile 构建配置

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-009 | `Profile::Debug / Release / Ship` 配置文件枚举 |
| REQ-010 | `Profile::optimization_level(&self) -> u8` 获取优化级别 |
| REQ-011 | `Profile::debug_info(&self) -> bool` 是否包含调试信息 |
| REQ-012 | `Profile::strip_symbols(&self) -> bool` 是否剥离符号 |
| REQ-013 | `Profile::lto(&self) -> bool` 是否启用 LTO 链接时优化 |
| REQ-196 | `Profile::Debug / Release / Ship` 配置文件枚举（重复） |
| REQ-197 | `Profile::opt_level(&self) -> String` 获取优化级别字符串 |
| REQ-198 | `Profile::debug(&self) -> bool` 是否调试构建 |
| REQ-199 | `Profile::strip(&self) -> bool` 是否剥离符号 |
| REQ-200 | `Profile::lto(&self) -> bool` 是否启用 LTO |
| REQ-201 | `Profile::cargo_args(&self) -> Vec<String>` 获取 Cargo 参数 |

### API 签名

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Profile {
    #[default]
    Debug,
    Release,
    Ship,
}

impl Profile {
    pub fn optimization_level(&self) -> u8;
    pub fn debug_info(&self) -> bool;
    pub fn strip_symbols(&self) -> bool;
    pub fn lto(&self) -> bool;
    pub fn opt_level(&self) -> String;
    pub fn debug(&self) -> bool;
    pub fn strip(&self) -> bool;
    pub fn cargo_args(&self) -> Vec<String>;
}
```

### Profile 参数对照表

| Profile | opt_level | debug | strip | lto |
|---------|-----------|-------|-------|-----|
| Debug | "0" | true | false | false |
| Release | "2" | false | true | false |
| Ship | "3" | false | true | true |

### 输入

- 无（基于枚举值）

### 输出

- 各类配置参数

### 验收标准

- [ ] Debug 配置正确设置
- [ ] Release 配置正确设置
- [ ] Ship 配置正确设置
- [ ] Cargo 参数正确生成

### 依赖关系

- 无

### 优先级

**P0**

---

## 6. BuildConfig 构建配置

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-014 | `BuildConfig::app_name` 应用名称字段 |
| REQ-015 | `BuildConfig::app_id` 应用标识符字段 |
| REQ-016 | `BuildConfig::version` 版本号字段 |
| REQ-017 | `BuildConfig::version_code(i32)` 版本代码字段 |
| REQ-018 | `BuildConfig::icons(Vec<PathBuf>)` 图标路径列表 |
| REQ-019 | `BuildConfig::splash_screen` 启动画面路径 |
| REQ-020 | `BuildConfig::permissions(Vec<Permission>)` 权限列表 |
| REQ-021 | `BuildConfig::orientation(Portrait / Landscape / Auto)` 屏幕方向 |
| REQ-022 | `BuildConfig::platform_target(&self) -> PlatformTarget` 获取目标平台 |
| REQ-023 | `BuildConfig::profile(&self) -> Profile` 获取构建配置 |
| REQ-024 | `BuildConfig::from_file(path) -> Result<Self>` 从文件加载 |
| REQ-025 | `BuildConfig::save(&self, path)` 保存配置到文件 |
| REQ-026 | `BuildConfig::with_assets_dir(dir)` 设置资源目录 |
| REQ-027 | `BuildConfig::with_output_dir(dir)` 设置输出目录 |
| REQ-028 | `BuildConfig::with_temp_dir(dir)` 设置临时目录 |
| REQ-173 | `BuildConfig::default()` 默认配置 |
| REQ-174 | `BuildConfig::app_name(&self) -> &str` 获取应用名称 |
| REQ-175 | `BuildConfig::app_id(&self) -> &str` 获取应用标识符 |
| REQ-176 | `BuildConfig::version(&self) -> &str` 获取版本号 |
| REQ-177 | `BuildConfig::version_code(&self) -> i32` 获取版本代码 |
| REQ-178 | `BuildConfig::icons(&self) -> &[PathBuf]` 获取图标列表 |
| REQ-179 | `BuildConfig::splash(&self) -> Option<&PathBuf>` 获取启动画面 |
| REQ-180 | `BuildConfig::permissions(&self) -> &[Permission]` 获取权限列表 |
| REQ-181 | `BuildConfig::orientation(&self) -> Orientation` 获取屏幕方向 |
| REQ-182 | `BuildConfig::output_dir(&self) -> &Path` 获取输出目录 |
| REQ-183 | `BuildConfig::temp_dir(&self) -> &Path` 获取临时目录 |
| REQ-184 | `BuildConfig::assets_dir(&self) -> &Path` 获取资源目录 |
| REQ-185 | `BuildConfig::from_toml(path) -> Result<Self>` 从 TOML 加载 |
| REQ-186 | `BuildConfig::to_toml(&self) -> String` 序列化为 TOML |
| REQ-187 | `BuildConfig::save(&self, path) -> Result<()>` 保存到文件 |

### API 签名

```rust
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
    Auto,
}

impl BuildConfig {
    pub fn new() -> Self;
    pub fn default() -> Self;
    pub fn app_name(&self) -> &str;
    pub fn app_id(&self) -> &str;
    pub fn version(&self) -> &str;
    pub fn version_code(&self) -> i32;
    pub fn icons(&self) -> &[PathBuf];
    pub fn splash(&self) -> Option<&PathBuf>;
    pub fn permissions(&self) -> &[Permission];
    pub fn orientation(&self) -> Orientation;
    pub fn output_dir(&self) -> &Path;
    pub fn temp_dir(&self) -> &Path;
    pub fn assets_dir(&self) -> &Path;
    pub fn platform_target(&self) -> PlatformTarget;
    pub fn profile(&self) -> Profile;
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self>;
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self>;
    pub fn to_toml(&self) -> String;
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()>;
    pub fn with_assets_dir(mut self, dir: impl AsRef<Path>) -> Self;
    pub fn with_output_dir(mut self, dir: impl AsRef<Path>) -> Self;
    pub fn with_temp_dir(mut self, dir: impl AsRef<Path>) -> Self;
}
```

### 输入

- TOML 配置文件路径或手动设置

### 输出

- `BuildConfig` 配置对象

### 验收标准

- [ ] 配置文件可正确解析
- [ ] 配置可序列化/反序列化往返
- [ ] Builder 模式链式调用正常

### 依赖关系

- 依赖 `toml` crate
- 依赖 `serde` crate

### 优先级

**P0**

---

## 7. BuildArtifact 构建产物

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-029 | `BuildArtifact::path(&self) -> &Path` 产物路径 |
| REQ-030 | `BuildArtifact::size(&self) -> u64` 产物大小 |
| REQ-031 | `BuildArtifact::platform(&self) -> PlatformTarget` 目标平台 |
| REQ-032 | `BuildArtifact::version(&self) -> &str` 版本信息 |
| REQ-033 | `BuildArtifact::sign_info(&self) -> Option<&SignInfo>` 签名信息 |
| REQ-290 | `BuildArtifact::path(&self) -> &Path` 产物路径（重复） |
| REQ-291 | `BuildArtifact::size(&self) -> u64` 产物大小（重复） |
| REQ-292 | `BuildArtifact::platform(&self) -> PlatformTarget` 目标平台（重复） |
| REQ-293 | `BuildArtifact::version(&self) -> &str` 版本信息（重复） |
| REQ-294 | `BuildArtifact::sign(&self) -> Option<&SignInfo>` 签名信息（重复） |

### API 签名

```rust
#[derive(Debug, Clone)]
pub struct BuildArtifact {
    pub path: PathBuf,
    pub size: u64,
    pub platform: PlatformTarget,
    pub version: String,
    pub sign_info: Option<SignInfo>,
}

impl BuildArtifact {
    pub fn path(&self) -> &Path;
    pub fn size(&self) -> u64;
    pub fn platform(&self) -> PlatformTarget;
    pub fn version(&self) -> &str;
    pub fn sign_info(&self) -> Option<&SignInfo>;
}
```

### 输入

- 无（从构建结果生成）

### 输出

- 产物元数据信息

### 验收标准

- [ ] 产物信息正确记录
- [ ] 签名信息正确关联

### 依赖关系

- 依赖 `SignInfo` 结构体

### 优先级

**P0**

---

## 8. Permission 权限定义

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-020 | `BuildConfig::permissions(Vec<Permission>)` 权限列表 |
| REQ-316 | `Permission::Internet / Storage / Camera / Microphone / Location / Bluetooth / NFC` |
| REQ-317 | `Permission::to_android_string(&self) -> &str` 转换为 Android 权限字符串 |
| REQ-318 | `Permission::to_ios_string(&self) -> &str` 转换为 iOS 权限字符串 |

### API 签名

```rust
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

### 权限映射表

| Permission | Android | iOS |
|------------|---------|-----|
| Internet | android.permission.INTERNET | NSInternetPermission |
| Storage | android.permission.READ_EXTERNAL_STORAGE | NSPhotoLibraryUsageDescription |
| Camera | android.permission.CAMERA | NSCameraUsageDescription |
| Microphone | android.permission.RECORD_AUDIO | NSMicrophoneUsageDescription |
| Location | android.permission.ACCESS_FINE_LOCATION | NSLocationWhenInUseUsageDescription |
| Bluetooth | android.permission.BLUETOOTH | NSBluetoothAlwaysUsageDescription |
| NFC | android.permission.NFC | NSNFCUsageDescription |

### 输入

- 权限枚举列表

### 输出

- 平台特定权限字符串

### 验收标准

- [ ] Android 权限字符串正确
- [ ] iOS 权限字符串正确

### 依赖关系

- 无

### 优先级

**P1**

---

## 9. 构建日志与报告

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-111 | `BuildLogger` 构建日志，彩色输出 + 进度 |
| REQ-112 | `BuildProgress` 构建进度，阶段指示 |
| REQ-113 | `BuildError` 错误码定位到阶段与文件 |
| REQ-114 | `BuildWarning` 可忽略提示 |
| REQ-115 | `BuildReport` 耗时 / 产物大小 / 警告数 / 错误数 |
| REQ-116 | `BuildReport::save_html(&self, path)` 生成可视化报表 |
| REQ-320 | `BuildLogger::new(verbose)` 创建日志器 |
| REQ-321 | `BuildLogger::info(msg)` 输出信息 |
| REQ-322 | `BuildLogger::warn(msg)` 输出警告 |
| REQ-323 | `BuildLogger::error(msg)` 输出错误 |
| REQ-324 | `BuildLogger::progress(percent, msg)` 输出进度 |
| REQ-325 | `BuildStage::Init / Compile / ProcessAssets / Package / Sign / Done` 阶段枚举 |
| REQ-326 | `BuildReport::new() -> Self` 创建报告 |
| REQ-327 | `BuildReport::add_stage(name, duration, size)` 添加阶段 |
| REQ-328 | `BuildReport::to_html(&self) -> String` 序列化为 HTML |
| REQ-329 | `BuildReport::save_html(&self, path) -> Result<()>` 保存 HTML |
| REQ-330 | `BuildReport::total_duration(&self) -> Duration` 总耗时 |
| REQ-331 | `BuildReport::total_size(&self) -> u64` 总大小 |
| REQ-332 | `BuildReport::warnings(&self) -> u32` 警告数 |
| REQ-333 | `BuildReport::errors(&self) -> u32` 错误数 |

### API 签名

```rust
pub struct BuildLogger {
    verbose: bool,
}

pub enum BuildStage {
    Init,
    Compile,
    ProcessAssets,
    Package,
    Sign,
    Done,
}

pub struct BuildProgress {
    stage: BuildStage,
    percent: u8,
}

impl BuildLogger {
    pub fn new(verbose: bool) -> Self;
    pub fn info(&self, msg: &str);
    pub fn warn(&self, msg: &str);
    pub fn error(&self, msg: &str);
    pub fn progress(&self, percent: u8, msg: &str);
}

impl BuildProgress {
    pub fn new() -> Self;
    pub fn set_stage(&mut self, stage: BuildStage);
    pub fn set_percent(&mut self, percent: u8);
}

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

### 输入

- 构建过程日志和阶段信息

### 输出

- HTML 格式构建报告

### 验收标准

- [ ] 日志彩色输出正常
- [ ] 进度指示正确
- [ ] HTML 报告生成正确

### 依赖关系

- 依赖 `colored` crate（可选）

### 优先级

**P1**

---

## 10. 产物打包格式

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-097 | `Package::new(output_dir, config, assets, binary) -> Result<Self>` 创建包 |
| REQ-098 | `Package::add_file(&mut self, path_in_pkg, bytes)` 添加文件 |
| REQ-099 | `Package::add_directory(&mut self, prefix, dir)` 添加目录 |
| REQ-100 | `Package::build(&self) -> Result<BuildArtifact>` 构建包 |
| REQ-101 | `Package::format(&self) -> PackageFormat` 获取包格式 |
| REQ-102 | `PackageFormat::Dir / Zip / Apk / Ipa / Wasm / MiniApp` 格式枚举 |
| REQ-288 | `Package::new(output_dir) -> Self` 创建包（重复） |
| REQ-289 | `Package::add_file(&mut self, pkg_path, bytes)` 添加文件（重复） |
| REQ-290 | `Package::add_directory(&mut self, prefix, dir)` 添加目录（重复） |
| REQ-291 | `Package::add_manifest(&mut self, manifest)` 添加清单 |
| REQ-292 | `Package::build(&self, format) -> Result<BuildArtifact>` 构建包 |
| REQ-293 | `Package::build_zip(&self, out) -> Result<BuildArtifact>` 构建 ZIP |
| REQ-294 | `Package::build_dir(&self, out) -> Result<BuildArtifact>` 构建目录 |

### API 签名

```rust
pub enum PackageFormat {
    Dir,
    Zip,
    Apk,
    Ipa,
    Wasm,
    MiniApp,
}

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

### 输入

- 输出目录、文件列表、清单

### 输出

- `BuildArtifact` 构建产物

### 验收标准

- [ ] ZIP 包正确生成
- [ ] 目录结构正确
- [ ] 清单正确包含

### 依赖关系

- 依赖 `zip` crate
- 依赖 `AssetManifest`

### 优先级

**P0**

---

## 11. 签名信息

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-107 | `Signing` Windows/Android/iOS 签名 |
| REQ-108 | `SignInfo::signature / certificate / timestamp` 签名信息字段 |
| REQ-109 | `ProvisioningProfile` iOS 配置文件 |
| REQ-110 | `AndroidKeystore` Android 密钥库 |
| REQ-311 | `AndroidKeystore::path / alias / password` 密钥库字段 |
| REQ-312 | `Signing::android_sign(unsigned_apk, keystore) -> Result<PathBuf>` Android 签名 |
| REQ-313 | `Signing::android_verify(apk) -> bool` Android 验签 |
| REQ-314 | `WindowsCodeSign::sign(exe, cert) -> Result<()>` Windows 签名 |
| REQ-315 | `AppleSign::codesign(app) -> Result<()>` Apple 签名 |
| REQ-319 | `ProvisioningProfile::name / team_id / app_id / entitlements` 配置文件字段 |

### API 签名

```rust
#[derive(Debug, Clone)]
pub struct SignInfo {
    pub signature: Vec<u8>,
    pub certificate: Vec<u8>,
    pub timestamp: Option<String>,
}

pub struct AndroidKeystore {
    pub path: PathBuf,
    pub alias: String,
    pub password: String,
}

pub struct ProvisioningProfile {
    pub name: String,
    pub team_id: String,
    pub app_id: String,
    pub entitlements: HashMap<String, String>,
}

impl Signing {
    pub fn android_sign(unsigned_apk: impl AsRef<Path>, keystore: &AndroidKeystore) -> Result<PathBuf>;
    pub fn android_verify(apk: impl AsRef<Path>) -> bool;
}

impl WindowsCodeSign {
    pub fn sign(exe: impl AsRef<Path>, cert: &Certificate) -> Result<()>;
}

impl AppleSign {
    pub fn codesign(app: impl AsRef<Path>, profile: &ProvisioningProfile) -> Result<()>;
}
```

### 输入

- 未签名包、证书/密钥库

### 输出

- 已签名包路径

### 验收标准

- [ ] Android 签名正确
- [ ] Windows 签名正确
- [ ] iOS 签名正确

### 依赖关系

- 依赖外部签名工具

### 优先级

**P1**

---

## 12. 优先级汇总

| 优先级 | 需求编号 |
|-------|---------|
| P0 | REQ-001~033, REQ-173~201, REQ-288~294 |
| P1 | REQ-107~116, REQ-311~333 |
| P2 | - |

---

## 13. 依赖关系图

```
BuildPipeline
├── Compiler
│   └── PlatformToolchain (Windows/MacOS/Linux/Android/IOS/Web/MiniApp)
├── AssetPipeline
│   ├── TextureProcessor
│   ├── AudioProcessor
│   ├── ModelProcessor
│   └── SceneProcessor
├── Package
│   └── PackageFormat
├── Signing
│   ├── AndroidKeystore
│   ├── WindowsCodeSign
│   └── AppleSign
├── BuildConfig
│   └── Profile
├── BuildLogger
├── BuildProgress
└── BuildReport
```

---

## 14. 验收清单

- [ ] `BuildPipeline::build()` 成功生成产物
- [ ] `BuildConfig` TOML 序列化/反序列化往返
- [ ] 所有 `PlatformTarget` 枚举正确
- [ ] 所有 `Profile` 配置正确
- [ ] `BuildReport::save_html()` 生成有效 HTML
- [ ] 日志输出正常工作
