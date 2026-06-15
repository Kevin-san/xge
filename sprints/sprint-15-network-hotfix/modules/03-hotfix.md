# 热更新需求

## 模块概述

热更新模块 (`engine-hotfix`) 提供运行时热更新能力，包括差分补丁、脚本热重载、动态库热加载、WASM 模块热替换、资产热重载等功能。支持多种更新策略（整包/增量/混合）和灰度发布机制。

## 需求清单

### 1. HotfixManager 核心

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 71 | 统一热更入口 | `HotfixManager`：统一热更入口 `check_for_update()` | - | - | 统一入口存在 | - | P0 |
| 72 | 应用 patch | `HotfixManager::apply_patch()`：应用 patch | - | - | 应用 patch | - | P0 |
| 73 | 进度查询 | `HotfixManager::progress()`：返回进度 `(cur, total)` | - | `(u64, u64)` | 返回进度 | - | P0 |
| 143 | HotfixManager 建立 | `HotfixManager` 建立 | - | - | 模块建立 | - | P0 |
| 238 | 状态查询 | `HotfixManager::status() -> HotfixStatus` | - | HotfixStatus | 返回当前状态 | - | P0 |
| 239 | 状态枚举 | `HotfixStatus::Idle / Checking / Downloading / Applying / Ready / Error(String)` | - | - | 状态枚举完整 | - | P0 |
| 363 | 统一入口实现 | `HotfixManager::new(work_dir) -> Self` | work_dir | Self | 创建管理器 | - | P0 |
| 364 | 当前版本 | `HotfixManager::current_version(&self) -> Version` | - | Version | 获取当前版本 | - | P0 |
| 365 | 检查更新 | `HotfixManager::check_for_update(manifest_url) -> Result<Option<UpdateInfo>>` | manifest_url | Result | 检查更新 | - | P0 |
| 366 | 下载更新 | `HotfixManager::download(&mut self, update) -> Result<()>` | update | Result | 下载更新 | - | P0 |
| 367 | 应用更新 | `HotfixManager::apply(&mut self) -> Result<()>` | - | Result | 应用更新 | - | P0 |
| 368 | 进度查询实现 | `HotfixManager::progress(&self) -> (u64, u64)` | - | `(u64, u64)` | 与 73 一致 | - | P0 |
| 369 | 状态查询实现 | `HotfixManager::status(&self) -> HotfixStatus` | - | HotfixStatus | 与 238 一致 | - | P0 |
| 370 | Idle 状态 | `HotfixStatus::Idle` | - | - | 空闲状态 | - | P0 |
| 371 | Checking 状态 | `HotfixStatus::Checking` | - | - | 检查中状态 | - | P0 |
| 372 | Downloading 状态 | `HotfixStatus::Downloading(percent)` | - | - | 下载中状态 | - | P0 |
| 373 | Applying 状态 | `HotfixStatus::Applying(percent)` | - | - | 应用中状态 | - | P0 |
| 374 | Ready 状态 | `HotfixStatus::Ready(restart_required)` | - | - | 就绪状态 | - | P0 |
| 375 | Error 状态 | `HotfixStatus::Error(String)` | - | - | 错误状态 | - | P0 |

### 2. DiffEngine 差分引擎

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 74 | bsdiff | `DiffEngine::bsdiff(old, new) -> patch` | old, new | patch | 生成差分 | - | P0 |
| 75 | bspatch | `DiffEngine::bspatch(old, patch) -> new` | old, patch | new | 应用差分 | - | P0 |
| 76 | zstd 压缩 | `DiffEngine::zstd_compress(data) -> Vec<u8>` | data | Vec<u8> | Zstd 压缩 | - | P0 |
| 77 | 块差分 | `DiffEngine::chunk_diff`：基于内容分片的差分（类似 courgette） | - | - | 内容分片差分 | - | P1 |
| 146 | bsdiff 实现 | `DiffEngine::bsdiff(old_path, new_path, patch_path) -> Result<()>` | old_path, new_path, patch_path | Result | 与 74 一致 | - | P0 |
| 147 | bspatch 实现 | `DiffEngine::bspatch(old_path, patch_path, new_path) -> Result<()>` | old_path, patch_path, new_path | Result | 与 75 一致 | - | P0 |
| 148 | zstd 压缩实现 | `DiffEngine::zstd_compress(data) -> Result<Vec<u8>>` | data | Result<Vec<u8>> | 与 76 一致 | - | P0 |
| 149 | zstd 解压 | `DiffEngine::zstd_decompress(data) -> Result<Vec<u8>>` | data | Result<Vec<u8>> | Zstd 解压 | - | P0 |
| 150 | 块差分实现 | `DiffEngine::chunk_diff(old, new) -> Vec<ChunkDiff>`（基于内容哈希滚动匹配） | old, new | Vec<ChunkDiff> | 与 77 一致 | - | P1 |
| 151 | Same 块 | `ChunkDiff::Same(offset, len)` | - | - | 相同块 | - | P1 |
| 152 | Diff 块 | `ChunkDiff::Diff(bytes)` | - | - | 不同块 | - | P1 |
| 153 | 块应用 | `DiffEngine::chunk_apply(old, diff) -> Vec<u8>` | old, diff | Vec<u8> | 应用块差分 | - | P1 |
| 479 | bsdiff 小文件 patch | 单测：bsdiff 小文件 patch | - | - | 测试通过 | - | P0 |
| 480 | zstd 压缩解压 | 单测：zstd 压缩 / 解压 | - | - | 测试通过 | - | P0 |
| 481 | bsdiff/bspatch 测试 | 单测：bsdiff/bspatch 对 1KB 文件 patch 正确 | - | - | 测试通过 | - | P0 |
| 482 | zstd 流压缩解压 | 单测：zstd 流压缩/解压 | - | - | 测试通过 | - | P0 |

### 3. PatchBundle 补丁包

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 78 | PatchBundle 格式 | `PatchBundle`：patch 文件格式与签名校验（RSA / Ed25519） | - | - | 格式定义完整 | - | P0 |
| 79 | PatchBundle 结构 | `PatchBundle`：patch 文件封装格式（头部 + meta + 签名 + 数据） | - | - | 封装格式正确 | - | P0 |
| 154 | RSA 签名 | `PatchBundle::sign(keypair)`：RSA 签名 | - | - | RSA 签名 | - | P0 |
| 155 | Ed25519 签名 | `PatchBundle::sign_ed25519(keypair)`：Ed25519 签名 | - | - | Ed25519 签名 | - | P0 |
| 156 | 签名校验 | `PatchBundle::verify(pubkey) -> bool` | pubkey | bool | 验证签名 | - | P0 |
| 384 | PatchBundle 构造 | `PatchBundle::new(version_from, version_to) -> Self` | version_from, version_to | Self | 创建 Bundle | - | P0 |
| 385 | 添加文件 | `PatchBundle::add_file(relative_path, diff_data)` | relative_path, diff_data | - | 添加文件 | - | P0 |
| 386 | RSA 签名方法 | `PatchBundle::sign_rsa(private_key_pem) -> Result<Signature>` | private_key_pem | Result | RSA 签名 | - | P0 |
| 387 | Ed25519 签名方法 | `PatchBundle::sign_ed25519(private_key) -> Result<Signature>` | private_key | Result | Ed25519 签名 | - | P0 |
| 388 | RSA 验证 | `PatchBundle::verify_rsa(public_key_pem, sig) -> bool` | public_key_pem, sig | bool | RSA 验证 | - | P0 |
| 389 | Ed25519 验证 | `PatchBundle::verify_ed25519(public_key, sig) -> bool` | public_key, sig | bool | Ed25519 验证 | - | P0 |
| 390 | 转字节 | `PatchBundle::to_bytes(&self) -> Result<Vec<u8>>` | - | Result<Vec<u8>> | 序列化 | - | P0 |
| 391 | 从字节 | `PatchBundle::from_bytes(&[u8]) -> Result<Self>` | &[u8] | Result<Self> | 反序列化 | - | P0 |
| 392 | Merkle 根 | `PatchBundle` 包含 SHA-256 merkle root 用于完整性 | - | - | 包含 merkle root | - | P0 |
| 491 | RSA 签名验证测试 | 单测：`PatchBundle` RSA 签名/验证 | - | - | 测试通过 | - | P0 |
| 492 | Ed25519 签名验证测试 | 单测：`PatchBundle` Ed25519 签名/验证 | - | - | 测试通过 | - | P0 |

### 4. UpdateStrategy 更新策略

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 80 | 整包更新 | `UpdateStrategy::Full`：整包更新 | - | - | 整包策略 | - | P0 |
| 81 | 增量更新 | `UpdateStrategy::Incremental`：增量差分 | - | - | 增量策略 | - | P0 |
| 82 | 混合策略 | `UpdateStrategy::Hybrid`：混合策略 | - | - | 混合策略 | - | P0 |
| 393 | 整包更新实现 | `UpdateStrategy::Full`：下载完整替换包 | - | - | 与 80 一致 | - | P0 |
| 394 | 增量更新实现 | `UpdateStrategy::Incremental`：下载增量 patch | - | - | 与 81 一致 | - | P0 |
| 395 | 混合策略实现 | `UpdateStrategy::Hybrid`：优先增量，失败回退整包 | - | - | 与 82 一致 | - | P0 |
| 396 | 策略选择 | `UpdateStrategy::choose(strategy_list, platform, bandwidth_est)` | strategy_list, platform, bandwidth_est | - | 智能选择 | - | P1 |

### 5. ScriptHotreload 脚本热重载

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 83 | JS/TS 支持 | `ScriptHotreload`：支持 JS / TS | - | - | 支持 JavaScript | - | P0 |
| 84 | Python 支持 | `ScriptHotreload`：支持 Python | - | - | 支持 Python | - | P0 |
| 85 | Lua 支持 | `ScriptHotreload`：支持 Lua | - | - | 支持 Lua | - | P0 |
| 86 | 文件监视 | `ScriptHotreload`：文件 watcher 自动 reload | - | - | 自动热重载 | - | P0 |
| 407 | ScriptRuntime 构造 | `ScriptRuntime::new(lang) -> Self` | lang | Self | 创建运行时 | - | P0 |
| 408 | JS 语言 | `ScriptLang::Js` | - | - | JavaScript | - | P0 |
| 409 | TS 语言 | `ScriptLang::Ts`（先转 JS） | - | - | TypeScript | - | P0 |
| 410 | Python 语言 | `ScriptLang::Py` | - | - | Python | - | P0 |
| 411 | Lua 语言 | `ScriptLang::Lua` | - | - | Lua | - | P0 |
| 412 | 加载脚本 | `ScriptRuntime::load(&mut self, path) -> Result<ScriptHandle>` | path | Result | 加载脚本 | - | P0 |
| 413 | 重载脚本 | `ScriptRuntime::reload(&mut self, handle) -> Result<()>` | handle | Result | 重载脚本 | - | P0 |
| 414 | 调用脚本 | `ScriptRuntime::call(&mut self, handle, fn_name, args) -> Result<Value>` | handle, fn_name, args | Result | 调用函数 | - | P0 |
| 415 | 文件监视器 | `ScriptFileWatcher::new(runtime, dir)` 监听文件变化自动 reload | runtime, dir | - | 自动重载 | - | P0 |
| 500 | 脚本热重载测试 | `examples/hotfix_script`：JS/Py/Lua 脚本热重载（修改源码后自动生效） | - | - | 测试通过 | - | P0 |

### 6. DylibHotload 动态库热加载

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 87 | dylib 加载 | `DylibHotload`：加载 `*.dylib / *.so / *.dll` | - | - | 加载动态库 | - | P0 |
| 88 | 符号解析 | `DylibHotload`：符号解析 `plugin_init / plugin_update / plugin_shutdown` | - | - | 符号解析 | - | P0 |
| 89 | 热卸载重载 | `DylibHotload`：热卸载 + 重新加载 | - | - | 热卸载重载 | - | P0 |
| 416 | 加载 dylib | `DylibHotload::load(path) -> Result<DylibHandle>` | path | Result | 加载动态库 | - | P0 |
| 417 | 卸载 dylib | `DylibHotload::unload(handle) -> Result<()>` | handle | Result | 卸载动态库 | - | P0 |
| 418 | 重新加载 | `DylibHotload::reload(handle, new_path) -> Result<()>` | handle, new_path | Result | 重新加载 | - | P0 |
| 419 | 符号解析 | `DylibHotload::symbol<T>(handle, name) -> Result<*const T>` | handle, name | Result | 获取符号地址 | - | P0 |
| 420 | 导出约定 | `DylibHotload` 约定导出：`plugin_init / plugin_update / plugin_shutdown` | - | - | 约定正确 | - | P0 |
| 421 | macOS 平台 | macOS `.dylib` | - | - | macOS 支持 | - | P0 |
| 422 | Linux 平台 | Linux `.so` | - | - | Linux 支持 | - | P0 |
| 423 | Windows 平台 | Windows `.dll` | - | - | Windows 支持 | - | P0 |

### 7. WasmHotswap WASM 热替换

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 90 | WASM 加载 | `WasmHotswap`：加载 WASM module | - | - | 加载 WASM | - | P0 |
| 91 | WASM 替换 | `WasmHotswap`：swap 新 module 保留 host 状态 | - | - | 热替换 | - | P0 |
| 424 | 加载 WASM | `WasmHotswap::load(bytes) -> Result<WasmHandle>` | bytes | Result | 加载 WASM | - | P0 |
| 425 | 替换 WASM | `WasmHotswap::swap(handle, new_bytes) -> Result<()>` | handle, new_bytes | Result | 替换模块 | - | P0 |
| 426 | 调用 WASM | `WasmHotswap::call(handle, fn_name, args) -> Result<Value>` | handle, fn_name, args | Result | 调用函数 | - | P0 |
| 427 | 保留 Host 状态 | `WasmHotswap` 保留 host 侧 `World` 引用，热换 wasm 模块不重建 world | - | - | 状态保留 | - | P0 |

### 8. AssetHotreload 资产热重载

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 92 | 纹理热重载 | `AssetHotreload`：纹理热重载 | - | - | 纹理热重载 | - | P0 |
| 93 | 模型热重载 | `AssetHotreload`：模型热重载 | - | - | 模型热重载 | - | P0 |
| 94 | 音频热重载 | `AssetHotreload`：音频热重载 | - | - | 音频热重载 | - | P0 |
| 95 | 场景热重载 | `AssetHotreload`：场景（scene JSON）热重载 | - | - | 场景热重载 | - | P0 |
| 428 | 注册纹理 | `AssetHotreload::register_texture(path, handle)` | path, handle | - | 注册纹理 | - | P0 |
| 429 | 注册模型 | `AssetHotreload::register_mesh(path, handle)` | path, handle | - | 注册模型 | - | P0 |
| 430 | 注册音频 | `AssetHotreload::register_audio(path, handle)` | path, handle | - | 注册音频 | - | P0 |
| 431 | 注册场景 | `AssetHotreload::register_scene(path, handle)` | path, handle | - | 注册场景 | - | P0 |
| 432 | tick 函数 | `AssetHotreload::tick(&mut self)` 轮询 mtime | - | - | 轮询文件变化 | - | P0 |
| 433 | 变化回调 | `AssetHotreload::on_change(path, cb)` 回调 | path, cb | - | 注册回调 | - | P0 |
| 434 | 纹理重传 | `AssetHotreload` 纹理重新上传 GPU | - | - | GPU 更新 | - | P0 |
| 435 | 场景更新 | `AssetHotreload` 场景重新解析并 update entity | - | - | 场景更新 | - | P0 |

### 9. AssetDiff 资源差分

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 397 | 目录差分 | `AssetDiff::diff_dir(old_dir, new_dir) -> Vec<FilePatch>` | old_dir, new_dir | Vec<FilePatch> | 生成目录差分 | - | P0 |
| 398 | 应用差分 | `AssetDiff::apply_dir(base_dir, patches) -> Result<()>` | base_dir, patches | Result | 应用目录差分 | - | P0 |
| 399 | Add 文件补丁 | `FilePatch::Add(path, bytes)` | - | - | 新增文件 | - | P0 |
| 400 | Modify 文件补丁 | `FilePatch::Modify(path, bsdiff_bytes)` | - | - | 修改文件 | - | P0 |
| 401 | Remove 文件补丁 | `FilePatch::Remove(path)` | - | - | 删除文件 | - | P0 |
| 501 | AssetDiff 测试 | 单测：`AssetDiff::diff_dir` 生成正确 Add/Modify/Remove | - | - | 测试通过 | - | P0 |

### 10. GreyRelease 灰度发布

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 96 | 渠道白名单 | `GreyRelease`：渠道白名单过滤 | - | - | 渠道过滤 | - | P0 |
| 97 | 版本区间 | `GreyRelease`：版本区间过滤 | - | - | 版本过滤 | - | P0 |
| 98 | 设备型号 | `GreyRelease`：设备型号过滤 | - | - | 设备过滤 | - | P0 |
| 99 | 地区过滤 | `GreyRelease`：地区过滤（IP / GPS） | - | - | 地区过滤 | - | P0 |
| 100 | 比例灰度 | `GreyRelease`：按比例灰度（AB 测试） | - | - | AB 测试 | - | P0 |
| 218 | GreyRelease 匹配 | `GreyRelease::match(user_profile) -> bool` | user_profile | bool | 用户匹配 | - | P0 |
| 436 | GreyRelease 构造 | `GreyRelease::new() -> Self` | - | Self | 创建灰度 | - | P0 |
| 437 | 渠道过滤 | `GreyRelease::by_channel(channels)` 设置渠道白名单 | channels | - | 渠道白名单 | - | P0 |
| 438 | 版本过滤 | `GreyRelease::by_version(range)` 设置版本区间 | range | - | 版本区间 | - | P0 |
| 439 | 设备过滤 | `GreyRelease::by_device(models)` 设置设备型号 | models | - | 设备型号 | - | P0 |
| 440 | 地区过滤 | `GreyRelease::by_region(regions)` 设置地区 | regions | - | 地区 | - | P0 |
| 441 | 比例灰度 | `GreyRelease::by_ratio(0.0..1.0)` 按比例灰度 | ratio | - | 比例灰度 | - | P0 |
| 442 | 用户匹配 | `GreyRelease::match_user(&self, user: &UserProfile) -> bool` | user | bool | 匹配用户 | - | P0 |
| 443 | UserProfile | `UserProfile::channel / os_version / device_model / region / user_id_hash` | - | - | 用户画像 | - | P0 |
| 498 | 灰度测试 | `examples/hotfix_grey`：灰度按比例分布稳定 | - | - | 测试通过 | - | P0 |
| 499 | 灰度匹配测试 | 单测：`GreyRelease::match_user` 按比例灰度分布稳定 | - | - | 测试通过 | - | P0 |

### 11. Downloader 下载器

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 101 | CDN/HTTP(S)/FTP | `Downloader`：CDN / HTTP(S) / FTP 多源配置 | - | - | 多源下载 | - | P0 |
| 102 | 断点续传 | `Downloader`：断点续传（ETag / Range） | - | - | 断点续传 | - | P0 |
| 103 | 多线程下载 | `Downloader`：多线程分片下载 | - | - | 分片下载 | - | P0 |
| 104 | Hash 校验 | `Downloader`：校验 hash（SHA-256） | - | - | SHA-256 校验 | - | P0 |
| 219 | 注册镜像 | `Downloader::register_mirror(url)` | url | - | 注册镜像 | - | P1 |
| 444 | Downloader 构造 | `Downloader::new(work_dir) -> Self` | work_dir | Self | 创建下载器 | - | P0 |
| 445 | 注册镜像方法 | `Downloader::register_mirror(url, priority)` | url, priority | - | 注册镜像 | - | P1 |
| 446 | 下载文件 | `Downloader::download(url, dest, expected_sha256) -> Result<()>` | url, dest, sha256 | Result | 下载文件 | - | P0 |
| 447 | 异步下载 | `Downloader::download_async(url, dest) -> JoinHandle<Result<()>>` | url, dest | JoinHandle | 异步下载 | - | P0 |
| 448 | Range 下载 | HTTP Range 断点续传 | - | - | Range 请求 | - | P0 |
| 449 | ETag 验证 | 使用 `ETag` 验证资源未变 | - | - | ETag 验证 | - | P0 |
| 450 | 镜像切换 | 失败自动切换 mirror | - | - | 自动切换 | - | P0 |
| 451 | 分片下载 | 多线程分片（8 线程默认） | - | - | 8 线程分片 | - | P0 |
| 452 | SHA-256 校验 | 下载完成校验 SHA-256 | - | - | 校验完整性 | - | P0 |
| 453 | 下载进度 | `Downloader::progress(&self, url) -> Option<(u64, u64)>` | url | Option<(u64, u64)> | 获取进度 | - | P0 |
| 500 | Downloader 测试 | 单测：`Downloader` mock HTTP 服务器 + 断点续传 | - | - | 测试通过 | - | P0 |

### 12. UpdateUI 更新界面

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 105 | 进度条 | `UpdateUI`：进度条组件 | - | - | 显示进度 | - | P0 |
| 106 | 变更日志 | `UpdateUI`：变更日志展示 | - | - | 显示日志 | - | P0 |
| 107 | 强制更新 | `UpdateUI`：强制更新对话框 | - | - | 强制更新 | - | P0 |
| 108 | 可选更新 | `UpdateUI`：可选更新对话框 | - | - | 可选更新 | - | P0 |
| 220 | 确认回调 | `UpdateUI::on_confirm(fn)` 回调 | fn | - | 确认回调 | - | P0 |
| 454 | UpdateUI 构造 | `UpdateUI::new() -> Self` | - | Self | 创建 UI | - | P0 |
| 455 | 显示进度 | `UpdateUI::show_progress(title, cur, total)` | title, cur, total | - | 显示进度 | - | P0 |
| 456 | 显示日志 | `UpdateUI::show_changelog(text)` | text | - | 显示变更日志 | - | P0 |
| 457 | 强制更新对话框 | `UpdateUI::show_mandatory_dialog(new_version, on_confirm)` | new_version, on_confirm | - | 强制更新 | - | P0 |
| 458 | 可选更新对话框 | `UpdateUI::show_optional_dialog(new_version, on_confirm, on_cancel)` | new_version, on_confirm, on_cancel | - | 可选更新 | - | P0 |
| 459 | 隐藏 UI | `UpdateUI::hide()` | - | - | 隐藏界面 | - | P0 |

### 13. VersionCompat 版本兼容

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 109 | 版本检查 | `VersionCompat::check(current, target) -> CompatResult` | current, target | CompatResult | 版本检查 | - | P0 |
| 110 | semver 解析 | `VersionCompat` semver 解析：`1.2.3` / `^1.2` / `~1.2.3` | - | - | semver 解析 | - | P0 |
| 461 | Version 解析 | `Version::parse("1.2.3") -> Result<Self>` | - | Result<Self> | 解析版本 | - | P0 |
| 462 | 版本字段 | `Version::major() / minor() / patch() / pre() / build()` | - | - | 获取版本字段 | - | P0 |
| 463 | VersionReq 解析 | `VersionReq::parse("^1.2") -> Result<Self>` | - | Result<Self> | 解析要求 | - | P0 |
| 464 | 版本匹配 | `VersionReq::matches(&self, v) -> bool` | v | bool | 匹配版本 | - | P0 |
| 465 | 兼容性检查 | `VersionCompat::check(current, min_required) -> CompatResult` | current, min_required | CompatResult | 与 109 一致 | - | P0 |
| 466 | Ok 结果 | `CompatResult::Ok` | - | - | 兼容 | - | P0 |
| 467 | Breaking 结果 | `CompatResult::Breaking(notes)` | - | - | 不兼容 | - | P0 |
| 468 | 升级要求结果 | `CompatResult::UpgradeRequired(min_version)` | - | - | 需要升级 | - | P0 |
| 469 | Breaking notes | `VersionCompat::breaking_notes(from, to) -> Vec<String>`（从 CHANGELOG 生成） | from, to | Vec<String> | 生成 notes | - | P1 |
| 493 | 版本解析测试 | 单测：`Version::parse` 与 `VersionReq::matches` | - | - | 测试通过 | - | P0 |

### 14. Examples 示例

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 156 | hotfix_patch 示例 | `examples/hotfix_patch`：生成并应用差分 patch | - | - | 示例可运行 | - | P0 |
| 157 | hotfix_script 示例 | `examples/hotfix_script`：脚本热重载 | - | - | 示例可运行 | - | P0 |
| 158 | hotfix_asset 示例 | `examples/hotfix_asset`：资源热重载 | - | - | 示例可运行 | - | P0 |
| 159 | hotfix_grey 示例 | `examples/hotfix_grey`：灰度发布示例 | - | - | 示例可运行 | - | P0 |
| 454 | hotfix_patch CLI | CLI 工具 `diff old new -> patch` 与 `patch old patch -> new` | - | - | CLI 可用 | - | P0 |
| 455 | patch 验证回滚 | 验证签名后应用 patch，失败回滚 | - | - | 签名验证 | - | P0 |
| 496 | hotfix_patch 运行 | `examples/hotfix_patch` 可生成和应用 patch | - | - | 可运行 | - | P0 |
| 497 | hotfix_asset 运行 | `examples/hotfix_asset` 纹理热重载可见 | - | - | 可运行 | - | P0 |
| 498 | hotfix_grey 运行 | `examples/hotfix_grey` 灰度按比例分布稳定 | - | - | 可运行 | - | P0 |

### 15. crate 建立

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 2 | crate 建立 | `engine-hotfix` crate 建立 | - | - | Cargo.toml 存在且可编译 | - | P0 |

## 验收标准

- `cargo test -p engine-hotfix` 全部通过
- `cargo clippy --workspace -- -D warnings` 通过
- `cargo fmt --check --workspace` 通过
- 1GB 资源差分 < 30s（CPU 8 核）
- patch 大小 < 原始 10%（对于差异 < 5% 的文件）
- 100MB patch 应用 < 10s

## 依赖关系

```
engine-hotfix
├── bsdiff (差分算法)
├── zstd (压缩)
├── sha2 (SHA-256)
├── rsa (RSA 签名)
├── ed25519-dalek (Ed25519 签名)
├── notify (文件监视)
├── wasmer (WASM 运行时)
└── tokio (异步)
```

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 内完成
- **P1**: 重要功能，应尽量完成
- **P2**: 优化功能，可延后到下一 Sprint
