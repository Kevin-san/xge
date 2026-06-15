# 模块二：平台工具链（Platform Toolchain）

## 1. 模块概述

平台工具链模块负责检测和管理各目标平台的构建工具，包括 Rust 编译器、Android NDK/iOS Xcode、WebAssembly 工具链以及小程序开发工具。该模块为跨平台构建提供统一的工具链抽象接口。

### 核心职责

- 检测各平台构建工具的安装状态和版本
- 提供平台特定的编译、打包、签名操作
- 管理工具链配置（NDK、SDK、Xcode 等路径）
- 生成平台特定的构建产物

### 需求来源

对应原文档需求编号：**34-98, 202-280**

---

## 2. Toolchain 通用接口

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-034 | `Toolchain::rust_version(&self) -> Version` Rust 版本 |
| REQ-035 | `Toolchain::ndk_version(&self) -> Version` Android NDK 版本 |
| REQ-036 | `Toolchain::xcode_version(&self) -> Version` Xcode 版本 |
| REQ-037 | `Toolchain::node_version(&self) -> Version` Node.js 版本 |
| REQ-038 | `Toolchain::detect() -> Result<Self>` 自动检测构建工具 |

### API 签名

```rust
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

### 输入

- 无（检测系统环境）

### 输出

- `Toolchain` 工具链信息

### 验收标准

- [ ] 正确检测 Rust 版本
- [ ] 正确检测 Android NDK 版本（若安装）
- [ ] 正确检测 Xcode 版本（若安装）
- [ ] 正确检测 Node.js 版本（若安装）

### 依赖关系

- 依赖 `rustc --version` 命令
- 依赖外部工具检测

### 优先级

**P0**

---

## 3. Compiler 编译器接口

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-039 | `Compiler` rustc cargo build 针对目标 |
| REQ-040 | `Compiler::compile(&self, config) -> Result<PathBuf>` 编译接口 |
| REQ-041 | `Compiler::target_triple(&self) -> &str` 目标三元组 |
| REQ-042 | `Compiler::features(&self) -> Vec<String>` 启用的 Cargo features |

### API 签名

```rust
pub trait Compiler {
    fn compile(&self, config: &BuildConfig) -> Result<PathBuf>;
    fn target_triple(&self) -> &str;
    fn features(&self) -> Vec<String>;
}

pub struct RustCompiler {
    target: PlatformTarget,
    features: Vec<String>,
}

impl Compiler for RustCompiler {
    fn compile(&self, config: &BuildConfig) -> Result<PathBuf>;
    fn target_triple(&self) -> &str;
    fn features(&self) -> Vec<String>;
}
```

### 目标三元组对照表

| PlatformTarget | target_triple |
|----------------|---------------|
| Windows | x86_64-pc-windows-gnu |
| macOS | x86_64-apple-darwin |
| Linux | x86_64-unknown-linux-gnu |
| Android | aarch64-linux-android |
| iOS | arm64-apple-ios |
| Web | wasm32-unknown-unknown |

### 输入

- `BuildConfig` 构建配置

### 输出

- 编译产物路径

### 验收标准

- [ ] 正确生成目标三元组
- [ ] 正确启用 Cargo features
- [ ] 编译成功生成产物

### 依赖关系

- 依赖 Rust 工具链
- 依赖 `cargo build` 命令

### 优先级

**P0**

---

## 4. AndroidToolchain Android 工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-043 | `AndroidToolchain` NDK/SDK/JDK 路径检测 |
| REQ-044 | `AndroidToolchain::sign_apk(unsigned_apk, keystore, alias, password) -> Result<PathBuf>` APK 签名 |
| REQ-045 | `AndroidToolchain::zipalign(apk) -> Result<PathBuf>` APK 对齐 |
| REQ-046 | `AndroidToolchain::aapt2_package(resources, manifest) -> Result<PathBuf>` 资源打包 |
| REQ-047 | `AndroidManifest::new(config) -> Self` 创建清单 |
| REQ-048 | `AndroidManifest::to_xml(&self) -> String` 序列化为 XML |
| REQ-049 | `AndroidManifest::permissions(&self) -> Vec<String>` 获取权限列表 |
| REQ-050 | `AndroidManifest::min_sdk(&self) -> u32` 最小 SDK 版本 |
| REQ-051 | `AndroidManifest::target_sdk(&self) -> u32` 目标 SDK 版本 |
| REQ-052 | `AndroidManifest::orientation(&self) -> &str` 屏幕方向 |
| REQ-053 | `AndroidManifest::activity_name(&self) -> &str` Activity 名称 |
| REQ-202 | `AndroidToolchain::detect() -> Result<Self>` 检测 Android 工具链 |
| REQ-203 | `AndroidToolchain::build(&self, config) -> Result<PathBuf>` 构建 APK |
| REQ-204 | `AndroidToolchain::abi(&self) -> Vec<String>` 支持的 ABI 列表 |
| REQ-205 | `AndroidToolchain::min_sdk(&self) -> u32` 最小 SDK 版本 |
| REQ-206 | `AndroidToolchain::sign_apk(&self, apk, keystore) -> Result<PathBuf>` 签名 APK |
| REQ-207 | `AndroidToolchain::zipalign(&self, apk) -> Result<PathBuf>` 对齐 APK |
| REQ-208 | `AndroidManifest::new(config) -> Self` 创建清单（重复） |
| REQ-209 | `AndroidManifest::to_xml(&self) -> String` 序列化（重复） |
| REQ-210 | `AndroidManifest::activity(&self) -> &str` 获取 Activity |
| REQ-211 | `AndroidManifest::intent_filters(&self) -> Vec<String>` Intent 过滤器 |

### API 签名

```rust
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

### 支持的 ABI

| ABI | target_triple |
|-----|---------------|
| arm64-v8a | aarch64-linux-android |
| armeabi-v7a | armv7-linux-androideabi |
| x86_64 | x86_64-linux-android |

### 输入

- `BuildConfig` 配置
- APK 签名材料

### 输出

- 签名后的 APK 路径

### 验收标准

- [ ] 正确检测 NDK/SDK/JDK 路径
- [ ] 正确签名 APK
- [ ] 正确对齐 APK
- [ ] AndroidManifest.xml 正确生成
- [ ] 支持多 ABI 构建

### 依赖关系

- 依赖 Android SDK build-tools
- 依赖 Android NDK
- 依赖 `apksigner` / `zipalign` / `aapt2` 工具

### 优先级

**P0**

---

## 5. IosToolchain iOS 工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-054 | `IosToolchain::xcodebuild(proj) -> Result<PathBuf>` Xcode 构建（仅 macOS） |
| REQ-055 | `IosToolchain::codesign(app, identity) -> Result<()>` 代码签名 |
| REQ-056 | `InfoPlist::new(config) -> Self` 创建 Info.plist |
| REQ-057 | `InfoPlist::to_plist(&self) -> Value` 序列化为 plist 格式 |
| REQ-058 | `InfoPlist::bundle_id(&self) -> &str` 获取 Bundle ID |
| REQ-059 | `InfoPlist::version(&self) -> &str` 获取版本 |
| REQ-060 | `InfoPlist::required_device_capabilities(&self) -> Vec<String>` 设备能力 |
| REQ-222 | `IOSToolchain::detect() -> Result<Self>` 检测 iOS 工具链（仅 macOS） |
| REQ-223 | `IOSToolchain::build(&self, config) -> Result<PathBuf>` 构建 iOS 应用 |
| REQ-224 | `IOSToolchain::codesign(&self, app, profile) -> Result<()>` 代码签名 |
| REQ-225 | `InfoPlist::new(config) -> Self` 创建清单（重复） |
| REQ-226 | `InfoPlist::to_string(&self) -> String` 序列化为字符串 |

### API 签名

```rust
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

### Info.plist 关键字段

| 字段 | 值来源 |
|------|--------|
| CFBundleIdentifier | BuildConfig.app_id |
| CFBundleShortVersionString | BuildConfig.version |
| CFBundleVersion | BuildConfig.version_code |
| UILaunchStoryboardName | BuildConfig.splash_screen |
| UISupportedInterfaceOrientations | BuildConfig.orientation |

### 输入

- `BuildConfig` 配置
- `ProvisioningProfile` 配置文件

### 输出

- `.app` 或 `.ipa` 路径

### 验收标准

- [ ] 正确检测 Xcode
- [ ] Info.plist 正确生成
- [ ] 代码签名成功
- [ ] 支持设备和模拟器双目标

### 依赖关系

- 依赖 Xcode Command Line Tools
- 依赖 `xcodebuild` / `codesign` 命令

### 优先级

**P0**（仅 macOS）

---

## 6. WindowsToolchain Windows 工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-202 | `WindowsToolchain::detect() -> Result<Self>` 检测 Windows 工具链 |
| REQ-203 | `WindowsToolchain::build(&self, config) -> Result<PathBuf>` 构建 Windows 应用 |
| REQ-204 | `WindowsToolchain::embed_icon(exe, icon) -> Result<()>` 嵌入图标（可选） |
| REQ-205 | `WindowsToolchain::sign(exe, cert) -> Result<()>` 代码签名（可选） |

### API 签名

```rust
pub struct WindowsToolchain {
    vswhere_path: Option<PathBuf>,
}

impl WindowsToolchain {
    pub fn detect() -> Result<Self>;
    pub fn build(&self, config: &BuildConfig) -> Result<PathBuf>;
    pub fn embed_icon(&self, exe: impl AsRef<Path>, icon: impl AsRef<Path>) -> Result<()>;
    pub fn sign(&self, exe: impl AsRef<Path>, cert: &Certificate) -> Result<()>;
}
```

### Windows 构建配置

| 配置项 | 说明 |
|--------|------|
| subsystem | console / windows |
| target | x86_64-pc-windows-gnu |

### 输入

- `BuildConfig` 配置
- 图标文件（可选）
- 证书（可选）

### 输出

- `.exe` 可执行文件路径

### 验收标准

- [ ] 正确检测 Visual Studio / MinGW
- [ ] 图标正确嵌入
- [ ] 代码签名成功（如提供证书）

### 依赖关系

- 依赖 Rust 工具链
- 依赖 `cargo build` 命令

### 优先级

**P1**

---

## 7. MacOSToolchain macOS 工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-206 | `MacOSToolchain::detect() -> Result<Self>` 检测 macOS 工具链 |
| REQ-207 | `MacOSToolchain::build(&self, config) -> Result<PathBuf>` 构建 macOS 应用 |
| REQ-208 | `MacOSToolchain::codesign(app, identity) -> Result<()>` 代码签名 |

### API 签名

```rust
pub struct MacOSToolchain {
    xcode_path: PathBuf,
}

impl MacOSToolchain {
    pub fn detect() -> Result<Self>;
    pub fn build(&self, config: &BuildConfig) -> Result<PathBuf>;
    pub fn codesign(&self, app: impl AsRef<Path>, identity: &str) -> Result<()>;
}
```

### macOS 构建产物

- `.app` 应用程序包
- 包含 `Info.plist`

### 输入

- `BuildConfig` 配置
- 代码签名身份

### 输出

- `.app` 路径

### 验收标准

- [ ] 正确检测 Xcode
- [ ] `.app` 包正确生成
- [ ] 代码签名成功

### 依赖关系

- 依赖 Xcode Command Line Tools

### 优先级

**P1**

---

## 8. LinuxToolchain Linux 工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-209 | `LinuxToolchain::detect() -> Result<Self>` 检测 Linux 工具链 |
| REQ-210 | `LinuxToolchain::build(&self, config) -> Result<PathBuf>` 构建 Linux 应用 |
| REQ-211 | `LinuxToolchain::appimage(src_dir, out) -> Result<PathBuf>` 生成 AppImage（可选） |

### API 签名

```rust
pub struct LinuxToolchain {
    gcc_path: PathBuf,
}

impl LinuxToolchain {
    pub fn detect() -> Result<Self>;
    pub fn build(&self, config: &BuildConfig) -> Result<PathBuf>;
    pub fn appimage(&self, src_dir: impl AsRef<Path>, out: impl AsRef<Path>) -> Result<PathBuf>;
}
```

### Linux 构建产物

- 可执行二进制文件
- 资源目录
- 可选 `.AppImage`

### 输入

- `BuildConfig` 配置

### 输出

- 可执行文件路径

### 验收标准

- [ ] 正确检测 GCC/Clang
- [ ] 可执行文件正确生成
- [ ] AppImage 可选生成

### 依赖关系

- 依赖 Rust 工具链
- 依赖 `cargo build` 命令

### 优先级

**P1**

---

## 9. WebToolchain Web 工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-061 | `WebToolchain::rustc_to_wasm(&self, src) -> Result<PathBuf>` Rust 编译为 WASM |
| REQ-062 | `WebToolchain::wasm_bindgen(wasm, out_dir) -> Result<PathBuf>` wasm-bindgen 处理 |
| REQ-063 | `WebToolchain::wasm_opt(wasm) -> Result<PathBuf>` wasm-opt 优化 |
| REQ-064 | `WebToolchain::generate_html(&self, js, wasm) -> Result<PathBuf>` 生成 HTML |
| REQ-065 | `WebToolchain::generate_service_worker(&self) -> Result<PathBuf>` 生成 Service Worker（PWA） |
| REQ-227 | `WebToolchain::detect() -> Result<Self>` 检测 Web 工具链 |
| REQ-228 | `WebToolchain::build_wasm(&self, config) -> Result<PathBuf>` 构建 Web 版本 |
| REQ-229 | `WebToolchain::wasm_bindgen(&self, src) -> Result<Vec<PathBuf>>` wasm-bindgen 处理 |
| REQ-230 | `WebToolchain::wasm_opt(&self, src) -> Result<PathBuf>` wasm-opt 优化 |
| REQ-231 | `WebToolchain::generate_html(&self) -> Result<PathBuf>` 生成 HTML |
| REQ-232 | `WebToolchain::generate_sw(&self) -> Result<PathBuf>` 生成 Service Worker |
| REQ-233 | `WebToolchain::generate_manifest(&self) -> Result<PathBuf>` 生成 Web Manifest |

### API 签名

```rust
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

### Web 构建产物

| 文件 | 说明 |
|------|------|
| index.html | 入口 HTML |
| *.js | JavaScript 绑定代码 |
| *.wasm | WebAssembly 二进制 |
| sw.js | Service Worker（PWA） |
| manifest.webmanifest | Web App Manifest（PWA） |

### PWA 配置

```json
{
  "name": "App Name",
  "short_name": "App",
  "start_url": ".",
  "display": "standalone",
  "icons": [...]
}
```

### 输入

- `BuildConfig` 配置
- 编译后的 WASM 文件

### 输出

- HTML/JS/WASM 文件路径

### 验收标准

- [ ] wasm-bindgen 正确处理
- [ ] wasm-opt 正确优化（release）
- [ ] HTML 正确生成
- [ ] Service Worker 正确生成（PWA）
- [ ] Web Manifest 正确生成（PWA）
- [ ] WASM 产物 < 5MB（release）
- [ ] 启动时间 < 1s

### 依赖关系

- 依赖 `wasm-bindgen-cli`
- 依赖 `wasm-opt`（可选）

### 优先级

**P0**

---

## 10. MiniAppToolchain 小程序工具链

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-066 | `MiniAppToolchain::pack(src_dir, out_file) -> Result<PathBuf>` 打包小程序 |
| REQ-067 | `MiniAppToolchain::minify(js) -> Result<String>` 压缩 JS |
| REQ-068 | `MiniAppToolchain::generate_manifest(config) -> Result<String>` 生成清单 |
| REQ-069 | `MiniAppToolchain::generate_app_js() -> Result<String>` 生成 app.js |
| REQ-070 | `MiniAppToolchain::generate_project_config() -> Result<String>` 生成项目配置 |
| REQ-234 | `MiniAppToolchain::build(&self, config, wasm_or_js) -> Result<PathBuf>` 构建小程序 |
| REQ-235 | `MiniAppToolchain::minify_js(&self, src) -> Result<String>` 压缩 JS |
| REQ-236 | `MiniAppToolchain::generate_app_json(&self) -> String` 生成 app.json |
| REQ-237 | `MiniAppToolchain::generate_project_config(&self) -> String` 生成项目配置 |
| REQ-238 | `MiniAppToolchain::generate_game_js(&self) -> String` 生成 game.js |
| REQ-239 | `MiniAppToolchain::miniapp_platform(&self) -> MiniAppPlatform` 获取小程序平台 |
| REQ-240 | `MiniAppPlatform::WeChat / ByteDance / QQ` 小程序平台枚举 |

### API 签名

```rust
pub enum MiniAppPlatform {
    WeChat,
    ByteDance,
    QQ,
}

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

### 小程序平台配置

| 平台 | 项目配置 | 入口文件 |
|------|---------|---------|
| 微信 | project.config.json | app.js |
| 抖音 | app.json | app.js |
| QQ | game.json | game.js |

### 小程序构建产物

- `miniapp.zip` 或工程目录
- `app.js` / `game.js` 入口
- `app.json` / `game.json` 配置
- 适配后的资源文件

### 输入

- `BuildConfig` 配置
- Web 产物路径

### 输出

- 小程序工程目录或 ZIP

### 验收标准

- [ ] 微信小程序正确生成
- [ ] 抖音小程序正确生成
- [ ] QQ 小程序正确生成
- [ ] JS 正确压缩
- [ ] 工程配置正确

### 依赖关系

- 依赖 Node.js
- 依赖小程序开发者工具（用于预览）

### 优先级

**P1**

---

## 11. 优先级汇总

| 优先级 | 需求编号 | 模块 |
|-------|---------|------|
| P0 | REQ-034~042, REQ-202~211, REQ-227~233 | Toolchain, Compiler, Android, Web |
| P1 | REQ-043~053, REQ-054~060, REQ-212~226, REQ-234~240 | Ios, MiniApp |
| P2 | REQ-061~066 | Windows, MacOS, Linux |

---

## 12. 工具检测矩阵

| 工具链 | 检测命令 | 必需 |
|--------|---------|------|
| Rust | rustc --version | 是 |
| NDK | $ANDROID_NDK_HOME/source.properties | Android 目标 |
| SDK | $ANDROID_HOME/platform-tools | Android 目标 |
| Xcode | xcodebuild -version | iOS/macOS 目标 |
| wasm-bindgen | wasm-bindgen --version | Web 目标 |
| wasm-opt | wasm-opt --version | Web 目标（可选） |
| Node | node --version | 小程序目标 |

---

## 13. 依赖关系图

```
Toolchain
├── RustCompiler
├── AndroidToolchain
│   ├── AndroidManifest
│   └── AndroidKeystore
├── IosToolchain
│   ├── InfoPlist
│   └── ProvisioningProfile
├── WindowsToolchain
├── MacOSToolchain
├── LinuxToolchain
├── WebToolchain
└── MiniAppToolchain
```

---

## 14. 验收清单

- [ ] `Toolchain::detect()` 正确检测所有工具
- [ ] `AndroidToolchain` 签名和对齐正确
- [ ] `WebToolchain` 生成有效的 WASM + HTML
- [ ] `MiniAppToolchain` 生成各平台小程序
- [ ] `cargo build --target wasm32-unknown-unknown` 成功
- [ ] `cargo build --target x86_64-pc-windows-gnu` 成功
