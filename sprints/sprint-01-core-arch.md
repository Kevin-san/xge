# Sprint 01 · 核心架构骨架与模块抽象层

> 阶段：阶段一 · 基础内核 MVP  
> 周期：4 周  
> 核心目标：建立 Cargo workspace、L3-L5 分层契约初版、模块注册与生命周期管理  
> 验收：能以 `cargo run --example hello_engine` 打印引擎版本并触发一次空帧

---

## 一、Sprint 概览

本 Sprint 不做实际的渲染 / 物理 / UI，只聚焦「引擎会呼吸」。所有后续 Sprint 将在本 Sprint 建立的骨架上生长。核心交付：

- `engine-core`（核心 crate — 提供 `Engine`、`Module`、`App` 三大结构体；
- `engine-math` — 面向引擎的数学原语（Vec2/Vec3/Mat4/Quat/Transform）；
- `engine-platform` — 平台抽象 trait（Window / FileSystem / Time / ThreadPool）；
- `engine-log` — 结构化日志；
- `workspace root` — 统一的依赖版本、构建标志 (`feature flags`)、CI 脚手架。

---

## 二、项目需求清单（≥ 100 条）

1. 建立 Cargo workspace，包含至少 5 个成员 crate：core / math / platform / log / utils。
2. 统一 MSRV（Minimum Supported Rust Version）策略与 CI 检查。
3. `Cargo.toml` 使用统一的 `workspace.dependencies` 声明依赖，避免版本漂移。
4. 引入 `thiserror` + `anyhow` 作为错误处理基础。
5. 引入 `parking_lot` 替换 std Mutex/RwLock，保证跨平台一致性。
6. 设计并实现 `Engine` trait：`init() / start() / stop() / is_running()`。
7. 设计并实现 `Module` trait：`name() / on_init() / on_update(dt) / on_shutdown()`。
8. 实现 `ModuleRegistry` — 按名称注册、按依赖顺序初始化、逆序关闭。
9. 实现 `Module` 的依赖声明机制，保证初始化顺序可确定性。
10. 实现 `Module` 的启用/禁用开关，支持 feature-gate。
11. 实现 `App` trait — 开发者入口抽象，游戏项目实现 `App` 即可接入引擎。
12. 实现 `AppBuilder` — Fluent API 注册模块和配置。
13. 实现 `Time` 模块 — 提供 `delta_time()` / `elapsed()` / `frame_count()`。
14. 实现 `DeltaTime` 的精度为毫秒级（f64）。
15. 实现 `FixedTimestep` 子模块（固定步长）。
16. 实现 `Stopwatch` 工具类型用于局部计时。
17. 实现 `FileSystem` 抽象 — `read() / write() / exists() / list_dir()`。
18. `FileSystem` 在 Windows/macOS/Linux 上默认走原生 std::fs。
19. `FileSystem` 在 Web 上走 `fetch` / `IndexedDB` 的接口契约（先留 trait）。
20. 实现 `Path` 的规范化（统一 `/` 分隔符，忽略大小写策略可配置）。
21. 实现统一的引擎配置结构体 `EngineConfig`。
22. `EngineConfig` 支持从 JSON/TOML 文件加载。
23. `EngineConfig` 支持命令行覆盖。
24. 实现 `ThreadPool` 任务调度 — `spawn() / block_on() / try_spawn()`。
25. `ThreadPool` 线程数量 = CPU 逻辑核心数 - 1（可配置）。
26. `ThreadPool` 支持任务优先级队列。
27. `ThreadPool` 支持 future-aware（至少引入 `futures-lite`）。
28. `engine-log` 提供 `debug! / info! / warn! / error!` 宏。
29. 日志支持按模块过滤（按 target 前缀匹配）。
30. 日志支持控制台输出 + 文件输出（滚动日志）。
31. 日志等级可在 `EngineConfig` 中配置。
32. `engine-math` 提供 `Vec2 / Vec3 / Vec4`。
33. `engine-math` 提供 `Mat2 / Mat3 / Mat4`。
34. `engine-math` 提供 `Quat` 四元数。
35. `engine-math` 提供 `Transform`（位置 + 旋转 + 缩放）。
36. `engine-math` 提供 `Rect / AABB / OBB` 基础几何原语。
37. `engine-math` 提供 `Euler` 与 `Quat` 互转。
38. `engine-math` 提供 `lerp / slerp / nlerp` 插值函数。
39. 数学库保证 `no_std + alloc`，便于嵌入式主机。
40. `engine-utils` 提供通用工具：Uuid、HashMap 别名、类型安全句柄（Handle<T>）。
41. 实现 `Handle<T>` — 强类型句柄，索引 + 代际号，避免悬挂引用。
42. 实现 `Arena<T>` — 以句柄为键的对象池。
43. `Arena<T>` 支持增删改查，O(1) 平均复杂度。
44. `Arena<T>` 提供迭代器与借用检查。
45. 实现 `EventBus<T>` — 主题式事件总线。
46. `EventBus` 支持订阅者注册、事件派发、订阅者取消。
47. `EventBus` 线程安全：跨线程派发。
48. 实现 `ResourceManager<T>` — 通用资源管理器骨架。
49. `ResourceManager` 支持 load / get / unload / reload。
50. 实现 `AssetId` — 资源 ID（Uuid + 路径哈希）。
51. 建立引擎版本常量 `ENGINE_VERSION` 与构建信息。
52. 构建信息（commit hash / 时间戳）通过 `build.rs` 注入。
53. `examples/hello_engine` 示例：打印引擎版本、初始化、触发一次空帧。
54. `examples/module_order` 示例：演示模块注册与依赖顺序初始化。
55. `examples/event_bus_demo` 示例：演示事件总线订阅与派发。
56. `examples/arena_bench` 示例：演示句柄系统性能基准（可选）。
57. 单元测试覆盖率：每个核心结构体至少 3 个测试。
58. 引入 `criterion` 基准测试框架（可选）。
59. CI：引入 `cargo fmt --check` 在 CI 中强制检查。
60. CI：引入 `cargo clippy -- -D warnings` 强制检查。
61. CI：引入 `cargo test --workspace` 全量测试。
62. CI：引入 `cargo build --release --workspace` 验证 Release 构建。
63. CI：Linux x86_64 / macOS aarch64 / Windows x64 三平台矩阵。
64. CI：缓存 target 目录，缩短构建时间。
65. CI：测试覆盖率收集（可选 `tarpaulin`）。
66. Git `.gitignore`：忽略 `target / .vscode / .idea` 等。
67. `README.md`：说明如何 `cargo run --example hello_engine`。
68. `rust-toolchain.toml`：固定 Rust 工具链版本。
69. `rustfmt.toml`：统一格式化规则。
70. `clippy.toml`：统一 clippy 配置。
71. Feature flags 规划：`render-vulkan` / `render-gl` / `render-webgpu` / `audio` / `network` / `editor`（本 Sprint 先留位）。
72. Feature flags 默认打开 `render-gl + audio`，其余关闭。
73. 实现 `Feature` 结构体 — 在运行时查询 feature 是否启用。
74. `Feature` 在 WebAssembly 下自动禁用 host-only feature。
75. 平台检测 `Platform` 枚举：Windows / MacOS / Linux / Android / Ios / Web / Unknown。
76. 实现 `current_platform()` 函数 — 在编译期/运行期均可用。
77. 实现 `target_os_cfg` 宏 — 便于按平台分发代码。
78. 实现 `thread_local!` 包装器，保证单例对象安全访问。
79. 实现 `SpinLock<T>` — 轻量无栈锁（用于高频短持有场景）。
80. 引入 `bytemuck` + `serde` — 为后续资源序列化做准备。
81. `engine-serde`：二进制与 JSON 双模式（trait 定义，本 Sprint 不做实现）。
82. 引入 `parking_lot::Once` — 用于一次性初始化。
83. 设计引擎的主循环骨架：`Engine::run()` — `poll events → update → render → present`。
84. 主循环支持可变时间步（默认）与固定时间步（可选）。
85. 主循环支持 `pause()` / `resume()`。
86. 主循环支持 `request_quit()` — 请求安全退出。
87. 实现 `FrameStats` — 每帧统计：帧号、dt、CPU 耗时、GPU 耗时（本 Sprint 仅 CPU 可统计）。
88. 实现 `EngineStats` — 全局统计：运行时长、总帧数、平均 FPS。
89. 实现 `Plugin` trait — 插件系统接口（与 Module 的简化版，便于后续生态）。
90. `PluginGroup` — 成组安装插件。
91. `DefaultPlugins` — 默认插件组（后续逐步补充）。
92. 实现 `Schedule` — 调度器骨架：Startup / Update / Render / Shutdown 四阶段。
93. `Schedule` 支持 `add_system(fn)`。
94. `Schedule` 支持阶段顺序保证。
95. `Schedule` 支持系统依赖声明（本 Sprint 仅线性，后续并行化）。
96. 引入 `ahash` 作为默认 HashMap 哈希器，性能优先。
97. 引入 `log` + `env_logger`（或自研轻量日志）做实现层。
98. 实现 `Profiler` 轻量版 — 支持作用域计时 `scope!("name")`。
99. 实现 `Deref` 宏 — 用于 newtype 自动解引用。
100. `examples/minimal_app` — 最小可运行 Demo，演示 `App::default().run()`。
101. 文档注释覆盖率：每个公开项至少有一条 doc comment。
102. `unsafe` 代码必须有 SAFETY 注释。
103. `engine-core 公开 API 稳定度控制在 20~30 个公开函数以内（后续扩展）。
104. 首次 `cargo doc --open` 能正常生成文档并通过。
105. 首次 `cargo deny check`（如引入）用于依赖审计。
106. 约定代码命名规范：snake_case、CamelCase、SCREAMING_SNAKE_CASE。
107. 首次 CHANGELOG 初始化。
108. 提交钩子（可选：pre-commit 运行 `cargo fmt --check`）。

---

## 三、细分需求与验收（≥ 100 条）

### 3.1 `Engine` / `App` / `Module`

109. `Engine::new(config)` — 构造函数。
110. `Engine::run()` — 启动主循环。
111. `Engine::request_quit()` — 请求退出（线程安全）。
112. `Engine::is_running()` — 返回布尔。
113. `Engine::module<T: Module>()` — 获取模块引用。
114. `Engine::module_mut<T: Module>()` — 获取模块可变引用。
115. `Engine::world()` / `world_mut()` — 返回 ECS World 引用（本 Sprint 返回占位）。
116. `Engine::time()` — 返回时间引用。
117. `Engine::filesystem()` — 返回文件系统引用。
118. `Engine::config()` — 返回配置引用。
119. `Engine::spawn_task(future)` — 向线程池提交任务。
120. `App::setup(engine)` — 用户游戏代码入口。
121. `App::update(engine, dt)` — 用户帧更新。
122. `App::render(engine)` — 用户渲染钩子。
123. `App::shutdown(engine)` — 用户退出钩子。
124. `AppBuilder::new()` — 构造。
125. `AppBuilder::with_config(config)` — 配置。
126. `AppBuilder::add_module<T>()` — 注册模块。
127. `AppBuilder::add_plugin<T>()` — 注册插件。
128. `AppBuilder::run(app)` — 启动引擎并运行应用。
129. `Module::name(&self) -> &str` — 唯一模块名。
130. `Module::dependencies(&self) -> Vec<&str>` — 依赖模块名列表。
131. `Module::on_init(&mut self, engine)` — 初始化。
132. `Module::on_update(&mut self, engine, dt)` — 更新。
133. `Module::on_render(&mut self, engine)` — 渲染前。
134. `Module::on_shutdown(&mut self, engine)` — 关闭。
135. `Module::enabled(&self) -> bool`。
136. `ModuleRegistry::register<T>()`。
137. `ModuleRegistry::get<T>()` — 按类型查找。
138. `ModuleRegistry::get_by_name(name)` — 按名称查找（动态）。
139. `ModuleRegistry::initialize_all(engine)` — 按依赖拓扑排序后依序初始化。
140. `ModuleRegistry::update_all(engine, dt)`。
141. `ModuleRegistry::shutdown_all(engine)`。

### 3.2 `Time` / `FileSystem` / `ThreadPool`

142. `Time::new()` — 构造。
143. `Time::tick(&mut self)` — 每帧调用一次，更新 dt。
144. `Time::delta_seconds(&self) -> f32`。
145. `Time::delta(&self) -> Duration`。
146. `Time::elapsed(&self) -> Duration` — 启动至今。
147. `Time::frame_count(&self) -> u64`。
148. `Time::fps(&self) -> f32` — 近帧。
149. `Time::set_fixed_timestep(&mut self, dt)`。
150. `Time::fixed_timestep(&self) -> f32`。
151. `FixedTimestepSteps` — 累积 `steps` 计数。
152. `Stopwatch::new()` / `start()` / `stop()` / `reset()` / `elapsed()`。
153. `FileSystem::read(&self, path) -> Result<Vec<u8>>`。
154. `FileSystem::read_string(&self, path) -> Result<String>`。
155. `FileSystem::write(&self, path, bytes) -> Result<()>`。
156. `FileSystem::write_string(&self, path, s) -> Result<()>`。
157. `FileSystem::exists(&self, path) -> bool`。
158. `FileSystem::list_dir(&self, path) -> Result<Vec<PathBuf>>`。
159. `FileSystem::create_dir_all(&self, path) -> Result<()>`。
160. `FileSystem::remove_file(&self, path) -> Result<()>`。
161. `FileSystem::is_dir(&self) -> bool`。
162. `FileSystem::canonicalize(&self, path) -> Result<PathBuf>`。
163. `ThreadPool::new(num_threads)`。
164. `ThreadPool::spawn<F>(&self, f)` — 返回 JoinHandle。
165. `ThreadPool::try_spawn` — 失败时返回错误而非阻塞。
166. `ThreadPool::block_on<F>(f)`。
167. `ThreadPool::shutdown(&self)`。
168. `ThreadPool::active_count(&self) -> usize`。

### 3.3 `engine-math` 全部公开 API

169. `Vec2::new(x, y)` / `ZERO` / `ONE` / `X` / `Y` 常量。
170. `Vec2` 的 `Add / Sub / Mul / Div` 运算符。
171. `Vec2::dot()` / `cross()` / `length()` / `length_squared()`。
172. `Vec2::normalize()` / `normalize_or_zero()`。
173. `Vec2::lerp(a, b, t)`。
174. `Vec3` 同上一套。
175. `Vec4` 同上一套。
176. `Mat4::IDENTITY` / `ZERO`。
177. `Mat4::from_translation(v)`。
178. `Mat4::from_scale(v)`。
179. `Mat4::from_rotation_x(angle)`。
180. `Mat4::from_quat(q)`。
181. `Mat4::look_at_rh(eye, target, up)`。
182. `Mat4::perspective_rh(fovy, aspect, near, far)`。
183. `Mat4::orthographic_rh(left, right, bottom, top, near, far)`。
184. `Mat4::inverse(&self)`。
185. `Mat4::transpose(&self)`。
186. `Mat4::mul_vec4(&self, v)`。
187. `Mat4::to_cols_array(&self)`。
188. `Quat::IDENTITY`。
189. `Quat::from_rotation_x(angle)`。
190. `Quat::from_rotation_y(angle)`。
191. `Quat::from_rotation_z(angle)`。
192. `Quat::from_euler(euler)`。
193. `Quat::to_euler(&self)`。
194. `Quat::mul(q1, q2)`。
195. `Quat::inverse(&self)`。
196. `Quat::normalize(&self)`。
197. `Quat::slerp(a, b, t)`。
198. `Quat::nlerp(a, b, t)`。
199. `Transform::new(pos, rot, scale)`。
200. `Transform::from_translation(v)`。
201. `Transform::matrix(&self) -> Mat4`。
202. `Transform::inverse(&self) -> Transform`。
203. `Rect::new(x, y, w, h)`。
204. `Rect::contains(point)`。
205. `Rect::intersects(other)`。
206. `AABB::new(center, half_extents)`。
207. `AABB::min / `AABB::contains()`。

### 3.4 句柄 / 事件总线 / 资源管理

208. `Handle<T>` 结构体，内部：`index: u32` + `generation: u32`。
209. `Handle<T>::is_null(&self)`。
210. `Handle<T>` 实现 `Copy + Eq + Hash`。
211. `Arena<T>::new()`。
212. `Arena<T>::insert(value) -> Handle<T>`。
213. `Arena<T>::remove(handle)`。
214. `Arena<T>::get(handle) -> Option<&T>`。
215. `Arena<T>::get_mut(handle) -> Option<&mut T>`。
216. `Arena<T>::len(&self) -> usize`。
217. `Arena<T>::is_empty(&self) -> bool`。
218. `Arena<T>::clear(&mut self)`。
219. `Arena<T>::iter(&self)` — 遍历存活项。
220. `EventBus<T>::new()`。
221. `EventBus<T>::subscribe(&self, cb)` — 返回订阅者句柄。
222. `EventBus<T>::unsubscribe(&self, handle)`。
223. `EventBus<T>::send(&self, event)`。
224. `EventBus<T>::drain(&mut self)` — 批量消费累积事件。
225. `EventBus<T>::len(&self) -> usize`。
226. `AssetId::new(uuid)`。
227. `AssetId::from_path(path)`。
228. `AssetId::null()`。
229. `AssetId::is_null(&self)`。
230. `ResourceManager<T>::new()`。
231. `ResourceManager<T>::load(id, value)`。
232. `ResourceManager<T>::get(id)`。
233. `ResourceManager<T>::unload(id)`。
234. `ResourceManager<T>::contains(id)`。

### 3.5 调度 / 统计 / 日志

235. `Schedule::new()`。
236. `Schedule::add_stage(name)` — 阶段注册。
237. `Schedule::add_system_to_stage(stage, system)`。
238. `Schedule::run(&mut self, engine)`。
239. `Schedule::stage_order(&self)` — 阶段顺序。
240. `Schedule::set_run_criteria(...)` — 留接口。
241. `FrameStats::frame_number` / `dt` / `cpu_time_us`。
242. `EngineStats::uptime_seconds` / `total_frames` / `avg_fps`。
243. `Profiler::new()`。
244. `Profiler::begin_scope(name)`。
245. `Profiler::end_scope()`。
246. `Profiler::scope(name)` — RAII 守卫。
247. `Profiler::dump(&self)` — 输出作用域耗时汇总。
248. `log::init(level)`。
249. `log::set_level(level)`。
250. `log::enabled(level) -> bool`。
251. `log::debug!(...)`。
252. `log::info!(...)`。
253. `log::warn!(...)`。
254. `log::error!(...)`。
255. `log` 输出支持 `{target}` 字段过滤。
256. `log` 文件日志按大小滚动。
257. `log` 文件日志按天滚动。

### 3.6 平台抽象 trait

258. `Platform::current() -> Platform`。
259. `Platform::is_windows()` / `is_macos()` / `is_linux()` / `is_web()`。
260. `Platform::name(&self) -> &str`。
261. `target_os_cfg!` 宏 — 在编译期分发。
262. `Feature::enabled(name) -> bool`。
263. `Feature::list() -> Vec<&str>` — 返回所有 feature 列表。
264. `Feature::render_backend() -> &'static str`。

### 3.7 工具与构建

265. `ENGINE_VERSION` / `ENGINE_VERSION_MAJOR` / `MINOR` / `PATCH`。
266. `BUILD_COMMIT_HASH`（来自 `build.rs`）。
267. `BUILD_TIMESTAMP`。
268. `build.rs` 输出 `cargo:rerun-if-changed=build.rs`。
269. `build.rs` 通过 `git` 命令取 commit hash。
270. `examples/hello_engine` 运行成功退出码 0。
271. `examples/minimal_app` 运行成功退出码 0。
272. CI 三平台矩阵全部 green。
273. `cargo doc` 无 warning。
274. `cargo clippy` 无 warning。
275. 本 Sprint 公开 API 数量 <= 30。
276. 本 Sprint 单元测试数量 >= 30。
277. 本 Sprint Bench 数量 >= 3（可选）。
278. CHANGELOG 首条写版本 0.1.0-dev。
279. `README` 至少包含「运行方法、架构图、MSRV」。
280. 首次架构图（Mermaid ASCII）写入 README。

> 以上 280 条需求构成 Sprint 01 的完整可交付清单。

---

## 四、验收标准

- [ ] `cargo build --workspace` 成功
- [ ] `cargo test --workspace` 全部通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo run --example hello_engine` 输出引擎版本号
- [ ] `cargo run --example minimal_app` 触发一次空帧后退出
- [ ] CI Linux / macOS / Windows 三平台矩阵 green
- [ ] `cargo doc --workspace --no-deps` 成功生成
- [ ] `examples/module_order` 按依赖顺序初始化验证通过
- [ ] `Arena` 单元测试 10+ 条
- [ ] `EventBus` 单元测试 10+ 条
- [ ] `Time` 单元测试 10+ 条
- [ ] `engine-math` 单元测试 30+ 条
- [ ] 本 Sprint 结束时 `unsafe` 块数量 <= 5（且全部带 SAFETY 注释

---

## 五、风险与缓解

- **风险 1**：数学库从零实现工作量大  
  → 缓解：可先基于 `glam` 做一层包装，保留后续替换为自研。
- **风险 2**：feature flags 设计过度  
  → 缓解：本 Sprint 只定义 feature 位，不实现具体渲染后端。
- **风险 3**：平台差异（Windows/macOS/Linux）  
  → 缓解：统一用 `std` 做文件/时间/线程，不引入平台专用 API。

---

## 六、下一个 Sprint

Sprint 02 将在本 Sprint 骨架上接入窗口系统、事件循环与输入原语。
