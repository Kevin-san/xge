# 插件系统需求文档

## 模块名称与概述

**模块名称**：engine-plugin

**概述**：插件系统提供了一个灵活的扩展机制，允许在运行时动态加载、卸载和升级功能模块。核心能力包括：插件生命周期管理、沙盒安全隔离、依赖解析、资源配额管理以及插件市场集成。

---

## 需求清单

### 4.1 Plugin Trait

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 114 | `Plugin` trait：`fn on_load(&mut self, world)` | `fn on_load(&mut self, world: &mut World)` | 输入：`World` 引用<br>输出：无 | 插件加载时正确初始化 | - | P0 |
| 115 | `Plugin` trait：`fn on_unload(&mut self, world)` | `fn on_unload(&mut self, world: &mut World)` | 输入：`World` 引用<br>输出：无 | 插件卸载时正确清理资源 | - | P0 |
| 116 | `Plugin` trait：`fn on_tick(&mut self, world, dt)` | `fn on_tick(&mut self, world: &mut World, dt: f32)` | 输入：`World` 引用、时间增量<br>输出：无 | 每帧正确调用 | - | P0 |
| 117 | `Plugin` trait：`fn register_types(&mut self, registry)` | `fn register_types(&mut self, registry: &mut TypeRegistry)` | 输入：`TypeRegistry` 引用<br>输出：无 | 正确注册自定义类型 | - | P0 |
| 555 | `Plugin` trait：`fn name(&self) -> &str` | `fn name(&self) -> &str` | 输入：无<br>输出：插件名称字符串 | 返回非空字符串 | - | P0 |
| 556 | `Plugin` trait：`fn version(&self) -> Version` | `fn version(&self) -> Version` | 输入：无<br>输出：`Version` 对象 | 返回有效的 semver 版本 | - | P0 |
| 557 | `Plugin` trait：`fn on_load` 带 registry | `fn on_load(&mut self, world: &mut World, registry: &mut PluginRegistry)` | 输入：`World`、`PluginRegistry`<br>输出：无 | 加载时可注册组件/系统 | - | P0 |

### 4.2 PluginKind 类型

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 118 | `PluginKind::RustDylib` | `enum PluginKind { RustDylib, ... }` | - | 正确识别 Rust 动态库 | - | P0 |
| 119 | `PluginKind::Wasm` | `enum PluginKind { Wasm, ... }` | - | 正确识别 WASM 模块 | - | P0 |
| 120 | `PluginKind::Script(js/py/lua)` | `enum PluginKind { Script(ScriptLang), ... }` | - | 正确识别脚本类型 | - | P1 |
| 121 | `PluginKind::CAbi` | `enum PluginKind { CAbi, ... }` | - | 正确识别 C ABI 插件 | - | P1 |
| 561 | `PluginKind::RustDylib` | `PluginKind::RustDylib` | - | 匹配 dylib 文件扩展名 | - | P0 |
| 562 | `PluginKind::Wasm` | `PluginKind::Wasm` | - | 匹配 `.wasm` 文件 | - | P0 |
| 563 | `PluginKind::Script(ScriptLang)` | `PluginKind::Script(ScriptLang)` | - | 支持 JS/TS/Py/Lua | - | P1 |
| 564 | `PluginKind::CAbi` | `PluginKind::CAbi` | - | 匹配 `.so/.dll/.dylib` | - | P1 |

### 4.3 PluginManifest

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 122 | `PluginManifest`：从 `manifest.toml` 解析 | `PluginManifest::from_toml(path) -> Result<Self>` | 输入：文件路径<br>输出：`PluginManifest` | 正确解析 toml 文件 | - | P0 |
| 123 | `PluginManifest::name` | `pub name: String` | - | 非空字符串 | - | P0 |
| 124 | `PluginManifest::version` | `pub version: String` | - | 符合 semver 格式 | - | P0 |
| 125 | `PluginManifest::dependencies` | `pub dependencies: HashMap<String, VersionReq>` | - | 正确解析依赖列表 | - | P0 |
| 126 | `PluginManifest::permissions` | `pub permissions: Vec<PluginPermission>` | - | 权限列表非空 | - | P0 |
| 127 | `PluginManifest::entry_point` | `pub entry_point: PathBuf` | - | 有效路径 | - | P0 |
| 566 | `PluginManifest::from_toml` | `fn from_toml(path: impl AsRef<Path>) -> Result<Self>` | 输入：路径<br>输出：`Result<PluginManifest>` | 解析失败返回错误 | - | P0 |
| 568 | `PluginManifest::description` | `pub description: Option<String>` | - | 可选描述字段 | - | P2 |
| 569 | `PluginManifest::authors` | `pub authors: Vec<String>` | - | 作者列表 | - | P2 |
| 573 | `PluginManifest::kind` | `pub kind: PluginKind` | - | 插件类型 | - | P0 |

### 4.4 PluginPermission 权限

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 223 | `PluginPermission::FileRead(path_glob)` | `PluginPermission::FileRead(Glob)` | - | 读取权限 | - | P0 |
| 224 | `PluginPermission::FileWrite(path_glob)` | `PluginPermission::FileWrite(Glob)` | - | 写入权限 | - | P0 |
| 225 | `PluginPermission::Net(host, port)` | `PluginPermission::Net(Glob, PortRange)` | - | 网络权限 | - | P0 |
| 226 | `PluginPermission::Memory(bytes)` | `PluginPermission::MemoryLimit(usize)` | - | 内存配额 | - | P0 |
| 227 | `PluginPermission::CpuTime(seconds)` | `PluginPermission::CpuLimit(Duration)` | - | CPU 时间配额 | - | P0 |
| 574 | `PluginPermission::FileRead(glob)` | `FileRead(Glob)` | - | 路径 glob 匹配 | - | P0 |
| 575 | `PluginPermission::FileWrite(glob)` | `FileWrite(Glob)` | - | 路径 glob 匹配 | - | P0 |
| 576 | `PluginPermission::FileDelete(glob)` | `FileDelete(Glob)` | - | 删除权限 | - | P1 |
| 577 | `PluginPermission::NetConnect` | `NetConnect(Glob, PortRange)` | - | 出站连接 | - | P0 |
| 578 | `PluginPermission::NetListen` | `NetListen(PortRange)` | - | 监听端口 | - | P1 |
| 579 | `PluginPermission::MemoryLimit` | `MemoryLimit(usize)` | - | 字节数限制 | - | P0 |
| 580 | `PluginPermission::CpuLimit` | `CpuLimit(f32)` | - | 每分钟秒数 | - | P0 |
| 581 | `PluginPermission::EnvRead` | `EnvRead(Glob)` | - | 环境变量读取 | - | P2 |
| 582 | `PluginPermission::All` | `All` | - | 仅开发模式可用 | - | P2 |

### 4.5 PluginSandbox 沙盒

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 128 | `PluginSandbox`：文件权限 | `fn wrap_file_open(path, mode) -> Result<File>` | 输入：路径、打开模式<br>输出：`Result<File>` | 未授权路径返回拒绝 | - | P0 |
| 129 | `PluginSandbox`：网络权限 | `fn wrap_net_connect(addr) -> Result<TcpStream>` | 输入：地址<br>输出：`Result<TcpStream>` | 未授权地址返回拒绝 | - | P0 |
| 130 | `PluginSandbox`：内存配额 | `fn wrap_alloc(bytes) -> Result<()>` | 输入：字节数<br>输出：`Result<()>` | 超限返回错误 | - | P0 |
| 131 | `PluginSandbox`：CPU 时间配额 | `fn check_cpu(time) -> bool` | 输入：时间<br>输出：是否允许 | 超限返回 false | - | P0 |
| 228 | `PluginSandbox::check(&self, perm) -> bool` | `fn check(&self, perm: &PluginPermission) -> bool` | 输入：权限<br>输出：是否允许 | 正确判断权限 | - | P0 |
| 583 | `PluginSandbox::new(manifest)` | `fn new(manifest: &PluginManifest) -> Self` | 输入：manifest<br>输出：沙盒实例 | 创建成功 | - | P0 |
| 584 | `PluginSandbox::check` | `fn check(&self, perm: &PluginPermission) -> bool` | 输入：权限<br>输出：布尔值 | 正确验证权限 | - | P0 |
| 585 | `PluginSandbox::deny` | `fn deny(&self, perm: &PluginPermission) -> bool` | 输入：权限<br>输出：是否拒绝 | 正确拒绝未授权 | - | P0 |
| 586 | `PluginSandbox::wrap_file_open` | `fn wrap_file_open(&self, path: &Path, mode: OpenOptions) -> Result<File>` | 输入：路径、选项<br>输出：文件句柄 | 权限检查通过 | - | P0 |
| 587 | `PluginSandbox::wrap_net_connect` | `fn wrap_net_connect(&self, addr: SocketAddr) -> Result<TcpStream>` | 输入：地址<br>输出：连接 | 权限检查通过 | - | P0 |
| 588 | `PluginSandbox::wrap_alloc` | `fn wrap_alloc(&self, bytes: usize) -> Result<()>` | 输入：字节数<br>输出：结果 | 配额检查通过 | - | P0 |
| 589 | 所有 I/O 经过 hook | - | - | 未授权操作被拒绝 | - | P0 |

### 4.6 PluginRegistry 注册

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 132 | `PluginRegistry::register_component::<T>()` | `fn register_component<T: Component>(&mut self)` | 输入：组件类型<br>输出：无 | 组件可被查询 | - | P0 |
| 133 | `PluginRegistry::register_system(system)` | `fn register_system<T: System>(&mut self, stage, system)` | 输入：阶段、系统<br>输出：无 | 系统被添加到正确阶段 | - | P0 |
| 134 | `PluginRegistry::register_resource::<T>()` | `fn register_resource<T: Resource>(&mut self, init)` | 输入：初始化函数<br>输出：无 | 资源可被访问 | - | P0 |
| 135 | `PluginRegistry::register_window_builder` | `fn register_window(&mut self, builder)` | 输入：窗口构建器<br>输出：无 | 窗口可被创建 | - | P1 |
| 136 | `PluginRegistry::register_ui_widget` | `fn register_ui_widget(&mut self, widget)` | 输入：widget<br>输出：无 | widget 可被渲染 | - | P1 |
| 137 | `PluginRegistry::register_render_pass` | `fn register_render_pass(&mut self, pass)` | 输入：渲染 pass<br>输出：无 | pass 被添加到管线 | - | P1 |
| 590 | `PluginRegistry::new()` | `fn new() -> Self` | 输入：无<br>输出：实例 | 创建成功 | - | P0 |
| 591 | `register_component` | `fn register_component<T: Component>(&mut self)` | 输入：类型参数<br>输出：无 | 注册成功 | - | P0 |
| 592 | `register_system` | `fn register_system<T: System>(&mut self, stage: Stage, system: T)` | 输入：阶段、系统<br>输出：无 | 系统注册到阶段 | - | P0 |
| 593 | `register_resource` | `fn register_resource<T: Resource>(&mut self, init: impl FnOnce() -> T)` | 输入：初始化函数<br>输出：无 | 资源注册成功 | - | P0 |
| 594 | `register_event` | `fn register_event<T: Event>(&mut self)` | 输入：事件类型<br>输出：无 | 事件可被发送/接收 | - | P0 |
| 595 | `register_window` | `fn register_window(&mut self, builder: WindowBuilder)` | 输入：构建器<br>输出：无 | 窗口注册成功 | - | P1 |
| 596 | `register_ui_widget` | `fn register_ui_widget(&mut self, widget: UiWidget)` | 输入：widget<br>输出：无 | widget 注册成功 | - | P1 |
| 597 | `register_render_pass` | `fn register_render_pass(&mut self, pass: RenderPass)` | 输入：pass<br>输出：无 | pass 注册成功 | - | P1 |
| 598 | `register_asset_loader` | `fn register_asset_loader(&mut self, loader: AssetLoader)` | 输入：加载器<br>输出：无 | 加载器注册成功 | - | P1 |
| 599 | `entries` | `fn entries(&self) -> &[RegistryEntry]` | 输入：无<br>输出：注册条目列表 | 返回所有注册项 | - | P2 |

### 4.7 PluginResolver 依赖解析

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 138 | `PluginResolver`：DAG 依赖拓扑排序 | `fn resolve(&self) -> Result<Vec<PluginId>>` | 输入：无<br>输出：有序插件 ID | 正确排序 | - | P0 |
| 139 | `PluginResolver`：循环依赖检测 | `fn detect_cycles() -> Result<(), CycleError>` | 输入：无<br>输出：结果 | 检测到循环返回错误 | - | P0 |
| 140 | `PluginResolver`：版本冲突检测 | `fn detect_conflicts() -> Result<(), ConflictError>` | 输入：无<br>输出：结果 | 检测到冲突返回错误 | - | P0 |
| 601 | `PluginResolver::new()` | `fn new() -> Self` | 输入：无<br>输出：实例 | 创建成功 | - | P0 |
| 602 | `PluginResolver::add` | `fn add(&mut self, manifest: PluginManifest) -> Result<()>` | 输入：manifest<br>输出：结果 | 添加成功 | - | P0 |
| 603 | `PluginResolver::resolve` | `fn resolve(&self) -> Result<Vec<PluginId>>` | 输入：无<br>输出：有序列表 | 拓扑排序正确 | - | P0 |
| 604 | `detect_cycles` | `fn detect_cycles(&self) -> Result<(), CycleError>` | 输入：无<br>输出：结果 | 正确检测循环 | - | P0 |
| 605 | `detect_conflicts` | `fn detect_conflicts(&self) -> Result<(), ConflictError>` | 输入：无<br>输出：结果 | 正确检测冲突 | - | P0 |
| 606 | semver 冲突检测 | - | - | 不兼容版本被检测 | - | P0 |
| 607 | `CycleError::cycle_path` | `pub cycle_path: Vec<PluginId>` | - | 包含循环路径 | - | P0 |
| 608 | `ConflictError` 字段 | `pub plugin: String, version_a: Version, version_b: Version` | - | 包含冲突信息 | - | P0 |

### 4.8 PluginLifecycle 生命周期

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 141 | `PluginLifecycle::load(path) -> PluginHandle` | `fn load(&mut self, dir: &Path) -> Result<PluginHandle>` | 输入：目录路径<br>输出：句柄 | 加载成功 | - | P0 |
| 142 | `PluginLifecycle::unload(handle)` | `fn unload(&mut self, handle: PluginHandle) -> Result<()>` | 输入：句柄<br>输出：结果 | 卸载成功 | - | P0 |
| 143 | `PluginLifecycle::upgrade(handle, new_version)` | `fn upgrade(&mut self, handle: PluginHandle, new_dir: &Path) -> Result<()>` | 输入：句柄、新目录<br>输出：结果 | 升级成功 | - | P0 |
| 609 | `PluginLifecycle::new` | `fn new(world: &mut World, registry: &PluginRegistry) -> Self` | 输入：world、registry<br>输出：实例 | 创建成功 | - | P0 |
| 610 | `PluginLifecycle::load` | `fn load(&mut self, dir: &Path) -> Result<PluginHandle>` | 输入：目录<br>输出：句柄 | 插件加载成功 | - | P0 |
| 611 | `PluginLifecycle::unload` | `fn unload(&mut self, handle: PluginHandle) -> Result<()>` | 输入：句柄<br>输出：结果 | 插件卸载成功 | - | P0 |
| 612 | `PluginLifecycle::upgrade` | `fn upgrade(&mut self, handle: PluginHandle, new_dir: &Path) -> Result<()>` | 输入：句柄、新目录<br>输出：结果 | 插件升级成功 | - | P0 |
| 613 | `PluginLifecycle::reload` | `fn reload(&mut self, handle: PluginHandle) -> Result<()>` | 输入：句柄<br>输出：结果 | 插件重载成功 | - | P0 |
| 614 | `PluginLifecycle::tick` | `fn tick(&mut self, world: &mut World, dt: f32)` | 输入：world、dt<br>输出：无 | 调用所有插件 on_tick | - | P0 |

### 4.9 PluginQuota 配额管理

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 144 | `PluginQuota::memory_usage(&self) -> usize` | `fn memory_used(&self) -> usize` | 输入：无<br>输出：字节数 | 返回当前使用量 | - | P0 |
| 145 | `PluginQuota::cpu_time(&self) -> Duration` | `fn cpu_time(&self) -> Duration` | 输入：无<br>输出：持续时间 | 返回累计 CPU 时间 | - | P0 |
| 146 | `PluginQuota::handle_count(&self) -> usize` | `fn handle_count(&self) -> usize` | 输入：无<br>输出：数量 | 返回句柄数 | - | P1 |
| 147 | `PluginQuota::over_quota(&self) -> bool` | `fn over_quota(&self) -> bool` | 输入：无<br>输出：布尔值 | 超限返回 true | - | P0 |
| 615 | `PluginQuota::new` | `fn new(mem_limit: usize, cpu_limit: Duration) -> Self` | 输入：限制值<br>输出：实例 | 创建成功 | - | P0 |
| 616 | `memory_used` | `fn memory_used(&self) -> usize` | 输入：无<br>输出：字节数 | 正确返回 | - | P0 |
| 617 | `cpu_time` | `fn cpu_time(&self) -> Duration` | 输入：无<br>输出：时间 | 正确返回 | - | P0 |
| 618 | `handle_count` | `fn handle_count(&self) -> usize` | 输入：无<br>输出：数量 | 正确返回 | - | P1 |
| 619 | `record_alloc` | `fn record_alloc(&mut self, bytes: usize)` | 输入：字节数<br>输出：无 | 记录分配 | - | P0 |
| 620 | `record_dealloc` | `fn record_dealloc(&mut self, bytes: usize)` | 输入：字节数<br>输出：无 | 记录释放 | - | P0 |
| 621 | `record_cpu` | `fn record_cpu(&mut self, duration: Duration)` | 输入：时间<br>输出：无 | 记录 CPU 时间 | - | P0 |
| 622 | `over_quota` | `fn over_quota(&self) -> bool` | 输入：无<br>输出：布尔值 | 正确判断 | - | P0 |
| 623 | `kill_if_over` | `fn kill_if_over(&mut self, handle: PluginHandle)` | 输入：句柄<br>输出：无 | 超限插件被卸载 | - | P0 |

### 4.10 PluginDebug 调试

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 148 | `PluginDebug::logger`：独立命名空间日志 | `fn log(&self, level: Level, msg: &str)` | 输入：级别、消息<br>输出：无 | 日志写入独立文件 | - | P1 |
| 149 | `PluginDebug::hook(fn)`：函数 hook 调试 | `fn hook_fn(&self, target, before, after)` | 输入：目标、回调<br>输出：无 | 正确 hook | - | P2 |
| 150 | `PluginDebug::crash_recovery`：捕获 panic | `fn crash_recovery(&self, handle, world)` | 输入：句柄、world<br>输出：无 | panic 被捕获 | - | P0 |
| 624 | `PluginDebug::new` | `fn new(plugin_name: &str) -> Self` | 输入：名称<br>输出：实例 | 创建成功 | - | P1 |
| 625 | `PluginDebug::log` | `fn log(&self, level: Level, msg: &str)` | 输入：级别、消息<br>输出：无 | 写入独立日志 | - | P1 |
| 626 | `PluginDebug::hook_fn` | `fn hook_fn(&self, target: &str, before: Option<HookFn>, after: Option<HookFn>)` | 输入：目标、回调<br>输出：无 | hook 生效 | - | P2 |
| 627 | `set_crash_handler` | `fn set_crash_handler(&self, handler: CrashHandler)` | 输入：处理器<br>输出：无 | 注册成功 | - | P0 |
| 628 | `crash_recovery` | `fn crash_recovery(&self, handle: PluginHandle, world: &mut World)` | 输入：句柄、world<br>输出：无 | 卸载并报告 | - | P0 |
| 629 | `profile` | `fn profile(&self) -> PluginProfile` | 输入：无<br>输出：配置文件 | 返回性能数据 | - | P2 |
| 630 | `top_functions` | `fn top_functions(n: usize) -> Vec<(String, Duration)>` | 输入：数量<br>输出：函数列表 | 返回耗时最多的函数 | - | P2 |

### 4.11 PluginStoreClient 插件市场

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 151 | `PluginStoreClient`：连接 Asset Store | `fn new(base_url: &str) -> Self` | 输入：URL<br>输出：实例 | 创建成功 | - | P1 |
| 152 | `PluginStoreClient::list_plugins()` | `fn list(&self) -> Result<Vec<PluginInfo>>` | 输入：无<br>输出：插件列表 | 返回非空列表 | - | P1 |
| 153 | `PluginStoreClient::download(name)` | `fn download(&self, name: &str, version: Option<&str>) -> Result<PathBuf>` | 输入：名称、版本<br>输出：路径 | 下载成功 | - | P1 |
| 154 | `PluginStoreClient::install(path)` | `fn install(&self, path: &Path) -> Result<PluginHandle>` | 输入：路径<br>输出：句柄 | 安装成功 | - | P1 |
| 229 | `PluginStoreClient::search(keyword)` | `fn search(&self, keyword: &str) -> Result<Vec<PluginInfo>>` | 输入：关键词<br>输出：列表 | 搜索结果正确 | - | P1 |
| 230 | `PluginStoreClient::uninstall(name)` | `fn uninstall(&self, name: &str) -> Result<()>` | 输入：名称<br>输出：结果 | 卸载成功 | - | P1 |
| 631 | `PluginStoreClient::new` | `fn new(base_url: &str) -> Self` | 输入：URL<br>输出：实例 | 创建成功 | - | P1 |
| 632 | `list` | `fn list(&self) -> Result<Vec<PluginInfo>>` | 输入：无<br>输出：列表 | 正确返回 | - | P1 |
| 633 | `search` | `fn search(&self, keyword: &str) -> Result<Vec<PluginInfo>>` | 输入：关键词<br>输出：列表 | 搜索正确 | - | P1 |
| 634 | `info` | `fn info(&self, name: &str) -> Result<PluginInfo>` | 输入：名称<br>输出：详情 | 返回正确信息 | - | P1 |
| 635 | `download` | `fn download(&self, name: &str, version: Option<&str>) -> Result<PathBuf>` | 输入：名称、版本<br>输出：路径 | 下载成功 | - | P1 |
| 636 | `install` | `fn install(&self, path: &Path) -> Result<PluginHandle>` | 输入：路径<br>输出：句柄 | 安装成功 | - | P1 |
| 637 | `uninstall` | `fn uninstall(&self, name: &str) -> Result<()>` | 输入：名称<br>输出：结果 | 卸载成功 | - | P1 |
| 638 | `update` | `fn update(&self, name: &str) -> Result<PluginHandle>` | 输入：名称<br>输出：句柄 | 更新成功 | - | P1 |
| 639 | `auth` | `fn auth(&mut self, token: &str)` | 输入：token<br>输出：无 | 设置成功 | - | P1 |
| 640 | `PluginInfo` 字段 | `pub name, version, author, rating, downloads, manifest_url` | - | 包含必要信息 | - | P1 |

---

## 依赖关系总览

```
PluginKind
    └── PluginManifest (解析时使用)

PluginManifest
    ├── PluginKind
    ├── PluginPermission
    └── PluginRegistry (注册类型)

PluginSandbox
    ├── PluginManifest (读取权限配置)
    └── PluginQuota (检查资源配额)

PluginRegistry
    └── PluginLifecycle (加载时注册)

PluginResolver
    └── PluginManifest (解析依赖)

PluginLifecycle
    ├── PluginRegistry
    ├── PluginResolver
    ├── PluginSandbox
    └── PluginDebug

PluginQuota
    └── PluginSandbox

PluginDebug
    └── PluginLifecycle

PluginStoreClient
    └── PluginLifecycle (安装/卸载)
```

---

## 优先级分布

| 优先级 | 数量 | 说明 |
| :--- | :--- | :--- |
| P0 | 核心功能 | Plugin trait、Manifest、Sandbox、Registry、Resolver、Lifecycle |
| P1 | 重要功能 | PluginStoreClient、Debug、Quota 高级功能 |
| P2 | 辅助功能 | 额外权限类型、详细调试信息 |