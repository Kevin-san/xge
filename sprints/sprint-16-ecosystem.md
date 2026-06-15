# Sprint 16 · 资源商店 / 生态 / 性能调优

> 阶段：阶段四 · 高阶能力与生态（第 4 个 Sprint）
> 周期：4 周
> 核心目标：Asset Store、工程模板、Profiler 性能面板、文档与教程生态
> 验收：`examples/store_browse` 与 `examples/profiler_window` 可运行

---

## 一、Sprint 概览

本 Sprint 建立 `engine-asset-store`、`engine-template`、`engine-profiler`、`engine-docs` 四个 crate，并完善整个引擎的生态能力。核心交付：

- `AssetStoreClient`：资源商店客户端（登录 / 搜索 / 浏览 / 下载 / 安装 / 更新 / 卸载）
- `.rgepkg` 资源打包格式（zip + 签名 + manifest）
- `TemplateManager`：工程模板管理（2D / 3D / VR / AR / 多种游戏类型模板）
- `Profiler`：多维度性能采样面板（CPU / GPU / 内存 / 渲染 / 网络 / 脚本）
- `FlameGraph`、`Timeline`、直方图等可视化面板
- `engine-docs`：自动 API 文档 + 教程文档 + 示例项目 + 教学视频大纲
- 发布流程（alpha / beta / rc / stable / LTS）+ CI/CD 矩阵（三平台 + 移动 + WebAssembly）
- 社区工具与生态（Discord / Matrix / 微信 / QQ / 知乎 / B 站 / Reddit）
- 示例：`examples/store_browse` / `examples/store_install` / `examples/store_purchase` / `examples/template_new` / `examples/template_custom` / `examples/profiler_window` / `examples/profiler_remote` / `examples/profiler_bench`

---

## 二、项目需求清单

1. `engine-asset-store` crate 建立。
2. `engine-template` crate 建立。
3. `engine-profiler` crate 建立。
4. `engine-docs` crate / 工作目录建立。
5. `AssetStoreClient::new(config) -> Self`。
6. `AssetStoreClient::login(username, password) -> Result<()>`。
7. `AssetStoreClient::login_with_token(token) -> Result<()>`。
8. `AssetStoreClient::logout(&mut self) -> ()`。
9. `AssetStoreClient::is_logged_in(&self) -> bool`。
10. `AssetStoreClient::search(keyword, filters) -> Result<Vec<AssetSummary>>`。
11. `AssetStoreClient::browse(category, page, page_size) -> Result<Vec<AssetSummary>>`。
12. `AssetStoreClient::get_asset(id) -> Result<AssetDetail>`。
13. `AssetStoreClient::download(id, progress_cb) -> Result<PathBuf>`。
14. `AssetStoreClient::install(downloaded_path, target_dir) -> Result<InstalledAsset>`。
15. `AssetStoreClient::update(id) -> Result<InstalledAsset>`。
16. `AssetStoreClient::uninstall(id) -> Result<()>`。
17. `AssetStoreClient::list_installed(&self) -> Vec<InstalledAsset>`。
18. `AssetStoreClient::has_updates(&self) -> Vec<AssetId>`。
19. `AssetStoreClient::rollback(id, version) -> Result<()>`。
20. `AssetType::Texture2D / Texture3D / CubeMap / Model3D / Material / Sound / Scene / Script / Plugin / FullProject / Shader / UIKit / ParticlePack / PostFXPack`。
21. `AssetSummary`：名称 / 作者 / 版本 / 标签 / 分类 / 评级 / 下载量 / 价格 / 许可证 / 缩略图 / 依赖摘要。
22. `AssetDetail`：AssetSummary + 完整描述 / 截图列表 / 视频列表 / 依赖图 / 评论列表 / 更新日志。
23. `AssetMetadata`：名称 / 作者 / 版本 / 标签 / 分类 / 评级 / 下载量 / 价格 / 许可证 / 截图 / 视频 / 依赖 / 兼容性 / 最小引擎版本。
24. `AssetLicense::MIT / Apache2 / GPLv3 / Proprietary / CreativeCommons / Custom(String)`。
25. `PriceModel::Free / Paid(amount, currency) / Subscription(amount, currency, period)`。
26. `AssetRating::stars(&self) -> f32`。
27. `AssetRating::review_count(&self) -> usize`。
28. `AssetId::new(uuid) -> Self`。
29. `AssetId::parse(string) -> Result<Self>`。
30. `.rgepkg` 格式：zip 压缩包 + manifest.yaml + 文件列表 + 签名 + 校验和。
31. `RgePkg::pack(source_dir, output_path, signing_key) -> Result<()>`。
32. `RgePkg::unpack(pkg_path, target_dir, verify_signature) -> Result<AssetMetadata>`。
33. `RgePkg::verify(pkg_path, public_key) -> Result<bool>`。
34. `RgePkgManifest::name / version / author / engine_version / asset_type / files / dependencies / checksums / signature`。
35. `ChecksumAlgorithm::SHA256 / SHA512 / BLAKE3`。
36. `DependencyResolver::resolve(deps) -> Result<ResolutionGraph>`。
37. `DependencyResolver::detect_conflicts(graph) -> Vec<Conflict>`。
38. `Conflict::asset_a / asset_b / reason`。
39. `InstalledAsset::id / name / version / install_path / install_time / files`。
40. `AssetVersion::semver 语义化版本`。
41. `AssetVersion::parse(string) -> Result<Self>`。
42. `AssetVersion::cmp(&self, other) -> Ordering`。
43. `RollbackManager::snapshots(&self) -> Vec<Snapshot>`。
44. `RollbackManager::rollback(snapshot_id) -> Result<()>`。
45. `RollbackManager::clean_old(max_count) -> ()`。
46. `AssetStoreUI::home_page() -> HomePageView`（首页）。
47. `AssetStoreUI::category_page(category) -> CategoryView`（分类页）。
48. `AssetStoreUI::search_page(keyword) -> SearchView`（搜索页）。
49. `AssetStoreUI::detail_page(id) -> DetailView`（详情页）。
50. `AssetStoreUI::cart_page() -> CartView`（购物车）。
51. `AssetStoreUI::orders_page() -> OrdersView`（订单）。
52. `AssetStoreUI::my_assets_page() -> MyAssetsView`（我的资源）。
53. `DeveloperCenter::publish_asset(draft) -> Result<PublishedAsset>`。
54. `DeveloperCenter::submit_for_review(asset_id) -> Result<()>`。
55. `DeveloperCenter::set_price(asset_id, price_model) -> Result<()>`。
56. `DeveloperCenter::revenue_report(asset_id, range) -> RevenueReport`。
57. `DeveloperCenter::download_stats(asset_id, range) -> DownloadStats`。
58. `RevenueSplit::default_70_30() -> Self`（开发者 70% / 平台 30%）。
59. `Comment::author / content / rating / timestamp / helpful_votes`。
60. `CommentSystem::post(asset_id, comment) -> Result<CommentId>`。
61. `CommentSystem::list(asset_id, page) -> Vec<Comment>`。
62. `CommentSystem::vote(comment_id, helpful) -> Result<()>`。
63. `FreeAsset`：零元资源，可直接下载安装。
64. `PaidAsset`：需结账后下载。
65. `SubscriptionAsset`：按月 / 季度 / 年度订阅。
66. `OfflineMode::enable() / disable() / is_enabled()`。
67. `LocalLibrary::list(&self) -> Vec<InstalledAsset>`。
68. `LocalLibrary::import_manual(pkg_path) -> Result<InstalledAsset>`。
69. `examples/store_browse`：浏览与搜索资源。
70. `examples/store_purchase`：购物车 + 结账流程。
71. `examples/store_install`：下载 + 安装 + 回滚演示。
72. `TemplateManager::new() -> Self`。
73. `TemplateManager::list_templates(&self) -> Vec<Template>`。
74. `TemplateManager::list_templates_by_category(&self, cat) -> Vec<Template>`。
75. `TemplateManager::get_template(&self, id) -> Option<&Template>`。
76. `TemplateManager::create_project(&self, template_id, output_dir, project_name) -> Result<Project>`。
77. `TemplateType::Template2D / Template3D / TemplateVR / TemplateAR / TemplateEmpty / TemplateTutorial`。
78. `TemplateGameType::FPS / TPS / RPG / RTS / MOBA / Racing / Platformer / Puzzle / Card / Roguelike / VisualNovel / TowerDefense`。
79. `Template::id / name / description / category / game_type / thumbnail / engine_version / files`。
80. `Template::from_zip(path) -> Result<Self>`。
81. `Template::to_zip(&self, output_path) -> Result<()>`。
82. `Project::name / path / cargo_toml_path / main_scene_path`。
83. `Project::open(path) -> Result<Self>`。
84. `Project::run_cargo(&self, args) -> Result<Output>`。
85. `TemplateContent::CargoToml / MainScene / MainScript / README / EngineConfig / Gitignore`。
86. `TemplateMarketplace::publish(template) -> Result<TemplateId>`。
87. `TemplateMarketplace::install(template_id) -> Result<Template>`。
88. `TemplateMarketplace::uninstall(template_id) -> Result<()>`。
89. `TemplateMarketplace::search(keyword) -> Vec<Template>`。
90. `examples/template_new`：从模板创建 FPS 项目。
91. `examples/template_custom`：自定义模板并导出。
92. `Profiler::new(config) -> Self`。
93. `Profiler::begin_frame(&mut self) -> ()`。
94. `Profiler::end_frame(&mut self) -> ()`。
95. `Profiler::begin_scope(&mut self, name) -> ScopeGuard`。
96. `Profiler::end_scope(&mut self) -> ()`。
97. `Profiler::record_event(&mut self, name, data) -> ()`。
98. `Profiler::cpu_samples(&self) -> &[CpuSample]`。
99. `Profiler::gpu_samples(&self) -> &[GpuSample]`。
100. `Profiler::memory_samples(&self) -> &[MemorySample]`。
101. `Profiler::render_samples(&self) -> &[RenderSample]`。
102. `Profiler::network_samples(&self) -> &[NetworkSample]`。
103. `Profiler::script_samples(&self) -> &[ScriptSample]`。
104. `SampleGranularity::Frame / Tick / System / Function / DrawCall`。
105. `FlameGraph::from_samples(samples) -> Self`。
106. `FlameGraph::render_svg(&self, path) -> Result<()>`。
107. `FlameGraph::nodes(&self) -> &[FlameNode]`。
108. `FlameNode::name / start / duration / children`。
109. `Timeline::new() -> Self`。
110. `Timeline::add_track(&mut self, track) -> ()`。
111. `Timeline::tracks(&self) -> &[TimelineTrack]`。
112. `Timeline::render(&self) -> TimelineView`。
113. `Histogram::from_values(values, bucket_count) -> Self`。
114. `Histogram::mean(&self) -> f64`。
115. `Histogram::median(&self) -> f64`。
116. `Histogram::p95(&self) -> f64`。
117. `Histogram::p99(&self) -> f64`。
118. `PieChart::from_segments(segments) -> Self`。
119. `LineChart::push(timestamp, value) -> ()`。
120. `Metrics::frame_time_ms / fps / cpu_usage_percent / gpu_usage_percent / rss_kb / vss_kb / network_in_kbs / network_out_kbs / disk_read_kbs / disk_write_kbs`。
121. `FrameMetrics::frame_number / frame_time / gpu_time / cpu_time / draw_calls / triangles / vertices`。
122. `CallStackSample::addresses / symbols / thread_id`。
123. `SymbolResolver::resolve(address) -> Result<Symbol>`。
124. `Symbol::name / file / line / column`。
125. `HardwareCounter::CpuCycles / CacheMisses / BranchMisses / Instructions / GpuVertexTime / GpuFragmentTime / GpuComputeTime`。
126. `HardwareCounter::read(&self) -> Result<u64>`。
127. `profile_scope!("name")` 宏：自动 RAII scope。
128. `profile_event!("name", data)` 宏：一次性事件。
129. `RemoteProfilerServer::bind(addr) -> Result<Self>`。
130. `RemoteProfilerServer::accept(&mut self) -> Result<RemoteSession>`。
131. `RemoteProfilerServer::stream_samples(&mut self) -> ()`。
132. `RemoteProfilerClient::connect(addr) -> Result<Self>`。
133. `RemoteProfilerClient::receive_samples(&mut self) -> Result<SampleBatch>`。
134. `RemoteSession::device_info(&self) -> DeviceInfo`。
135. `RemoteSession::disconnect(&mut self) -> ()`。
136. `PerformanceDiagnostic::detect(profile) -> Vec<Warning>`。
137. `PerformanceWarning::category / message / severity / suggestion`。
138. `WarningSeverity::Info / Warning / Critical`。
139. `DiagnosticRule::excessive_draw_calls / high_frame_time_variance / memory_leak_suspect / gpu_bound / cpu_bound / script_gc_stall`。
140. `BaselineProfile::new(name, samples) -> Self`。
141. `BaselineProfile::compare(&self, new_samples) -> RegressionReport`。
142. `RegressionReport::regressions(&self) -> Vec<Regression>`。
143. `Regression::metric / delta / p_value / statistically_significant`。
144. `.rgeprofile` 格式：样本数据 + 元数据 + 符号表 + 压缩。
145. `RgeProfile::export(path, profile) -> Result<()>`。
146. `RgeProfile::import(path) -> Result<ProfileData>`。
147. `RgeProfile::summary(&self) -> ProfileSummary`。
148. `examples/profiler_window`：内置 Profiler 面板窗口。
149. `examples/profiler_remote`：远程采样 + 桌面查看。
150. `examples/profiler_bench`：基准测试 + 回归检测。
151. `engine-docs` 自动 API 文档（基于 cargo doc）。
152. `engine-docs` 主题（自定义 CSS / 侧边栏 / 搜索）。
153. `docs/src/getting_started/` 教程目录。
154. `docs/src/concepts/` 概念章节。
155. `docs/src/guides/` 指南章节。
156. `docs/src/examples/` 示例章节。
157. `docs/src/api_reference/` API 参考章节。
158. `docs/src/faq/` FAQ 章节。
159. `examples/hello_world`：引擎初始化 + 空窗口。
160. `examples/2d_platformer`：2D 平台跳跃示例。
161. `examples/3d_mini`：3D 小场景 + 角色控制。
162. `examples/ui_demo`：UI 组件演示。
163. `examples/physics_demo`：物理碰撞演示。
164. `examples/animation_demo`：动画系统演示。
165. `examples/particles_demo`：粒子系统演示。
166. `examples/network_demo`：多人联网演示。
167. `examples/blueprint_demo`：蓝图 / 可视化脚本演示。
168. 教学视频脚本大纲至少 20 讲。
169. 教学视频第 01 讲：引擎介绍与环境搭建。
170. 教学视频第 02 讲：第一个 Hello World 项目。
171. 教学视频第 03 讲：ECS 概念入门。
172. 教学视频第 04 讲：Entity / Component / System 详解。
173. 教学视频第 05 讲：资源管理与 Asset 系统。
174. 教学视频第 06 讲：2D 渲染与精灵。
175. 教学视频第 07 讲：2D 物理与碰撞。
176. 教学视频第 08 讲：3D 渲染基础。
177. 教学视频第 09 讲：PBR 材质与光照。
178. 教学视频第 10 讲：3D 物理与碰撞。
179. 教学视频第 11 讲：UI 系统与布局。
180. 教学视频第 12 讲：输入系统与事件。
181. 教学视频第 13 讲：动画系统入门。
182. 教学视频第 14 讲：状态机与混合树。
183. 教学视频第 15 讲：粒子与后处理。
184. 教学视频第 16 讲：音频系统。
185. 教学视频第 17 讲：脚本与蓝图。
186. 教学视频第 18 讲：多人联网与同步。
187. 教学视频第 19 讲：性能调优与 Profiler。
188. 教学视频第 20 讲：打包发布与跨平台。
189. 教学视频第 21 讲：资源商店与插件。
190. 教学视频第 22 讲：编辑器扩展。
191. 最佳实践指南（Best Practices）。
192. 性能调优指南（Performance Tuning Guide）。
193. 安全指南（Safety Guide）。
194. 迁移指南（Migration Guide）——从旧版本迁移。
195. 中文 / 英文双语文档。
196. 文档搜索功能（基于 tantivy / lunr）。
197. 文档版本切换（v0.9 / v1.0 / v1.1 / latest）。
198. 离线文档（docs.tar.gz / PDF 导出）。
199. docs 站点使用 mdbook 构建。
200. docs 站点备用 zola 构建。
201. `CHANGELOG.md` 按 Keep a Changelog 格式。
202. `ROADMAP.md` 路线图文档。
203. `CONTRIBUTING.md` 贡献指南。
204. `CODE_OF_CONDUCT.md` 行为准则。
205. Rust API Guidelines 合规（RFC 1574）。
206. 发布流程：alpha → beta → rc → stable → LTS。
207. `alpha` 版本：功能不稳定，API 可能变化。
208. `beta` 版本：功能冻结，仅修 bug。
209. `rc` 版本：发布候选，若两周无严重 bug 即 stable。
210. `stable` 版本：正式发布。
211. `LTS` 版本：长期支持 3 年。
212. CI/CD：GitHub Actions + Gitea + 自建 Runner。
213. 构建矩阵：Windows x86_64。
214. 构建矩阵：Linux x86_64。
215. 构建矩阵：macOS x86_64。
216. 构建矩阵：macOS arm64（Apple Silicon）。
217. 构建矩阵：Android armv7。
218. 构建矩阵：Android arm64。
219. 构建矩阵：iOS arm64。
220. 构建矩阵：WebAssembly（wasm32-unknown-unknown）。
221. `cargo publish` 自动发布到 crates.io。
222. GitHub Release 自动上传二进制。
223. Docker 镜像构建与推送。
224. Homebrew 包发布。
225. Scoop 包发布。
226. Chocolatey 包发布。
227. Flatpak 包发布。
228. Snap 包发布。
229. Issue 模板（Bug Report / Feature Request / Question）。
230. PR 模板（Description / Testing / Checklist）。
231. 自动标签 bot（label bot）。
232. Stale bot 标记与关闭旧 issue。
233. Discord 社区服务器。
234. Matrix 社区频道。
235. 微信社区群。
236. QQ 社区群。
237. 知乎专栏。
238. B 站官方账号。
239. Reddit 社区。
240. 邮件列表。
241. 官方博客（blog.engine.example.com）。
242. 更新日志发布（每 Sprint 一次）。
243. 路线图公开（每季更新）。
244. SDK / API 稳定性：semver 2.0。
245. 弃用周期：deprecate → 2 个 minor 版本后移除。
246. 迁移工具（cargo rge-migrate）。
247. 安全披露政策（Security Disclosure Policy）。
248. CVE 追踪（security@engine.example.com + GitHub Security Advisory）。
249. 官方 Logo（矢量 SVG）。
250. 视觉识别系统（色板 / 字体 / logo 使用规范）。
251. 品牌指南（Brand Guide）。
252. `cargo test --workspace` 全部通过。
253. `cargo clippy --workspace -- -D warnings` 通过。
254. `cargo fmt --check --workspace` 通过。
255. `cargo doc --workspace --no-deps` 成功。
256. 全量集成测试（examples 全部能跑）。
257. 性能基准测试（criterion.rs）。
258. fuzz 测试（cargo fuzz）。
259. Miri 检测未定义行为（Miri）。
260. ASan 检测内存错误（AddressSanitizer）。
261. TSan 检测数据竞争（ThreadSanitizer）。
262. MSan 检测未初始化读取（MemorySanitizer）。
263. CHANGELOG 记录 v1.0.0。
264. README 更新到 v1.0 发布状态。
265. 正式发布到 crates.io（v1.0.0）。

> 以上 265 条需求构成 Sprint 16 全量清单。

---

## 三、细分需求与验收

### 3.1 资源商店（engine-asset-store）

266. `AssetStoreConfig::default() -> Self`。
267. `AssetStoreConfig::server_url(&self) -> &str`。
268. `AssetStoreConfig::cache_dir(&self) -> &Path`。
269. `AssetStoreConfig::install_dir(&self) -> &Path`。
270. `AssetStoreConfig::with_server_url(url) -> Self`。
271. `AssetStoreClient::register(username, email, password) -> Result<()>`。
272. `AssetStoreClient::request_password_reset(email) -> Result<()>`。
273. `AssetStoreClient::reset_password(token, new_password) -> Result<()>`。
274. `AuthToken::from_string(s) -> Result<Self>`。
275. `AuthToken::to_string(&self) -> String`。
276. `AuthToken::expired(&self) -> bool`。
277. `AssetStoreClient::refresh_token(&mut self) -> Result<()>`。
278. `UserProfile::username / email / display_name / avatar_url / joined_at`。
279. `AssetStoreClient::me(&self) -> Result<UserProfile>`。
280. `SearchQuery::keyword / categories / tags / min_rating / price_range / license_types / sort_by`。
281. `SortOrder::Relevance / Rating / Downloads / Newest / PriceAsc / PriceDesc`。
282. `AssetStoreClient::search_with_pagination(query, page, page_size) -> Result<Paged<AssetSummary>>`。
283. `Paged<T>::items / page / page_size / total_pages / total_items`。
284. `AssetCategory::All / Art2D / Models3D / Materials / Audio / Scenes / Scripts / Plugins / Templates / Shaders / UIKits / ParticlePacks / PostFX / FullProjects`。
285. `AssetStoreClient::browse_category(cat, sort, page) -> Result<Paged<AssetSummary>>`。
286. `AssetStoreClient::trending(limit) -> Vec<AssetSummary>`。
287. `AssetStoreClient::featured(limit) -> Vec<AssetSummary>`。
288. `AssetStoreClient::new_releases(limit) -> Vec<AssetSummary>`。
289. `AssetStoreClient::top_rated(limit) -> Vec<AssetSummary>`。
290. `AssetStoreClient::most_downloaded(limit) -> Vec<AssetSummary>`。
291. `AssetStoreClient::related_assets(id, limit) -> Vec<AssetSummary>`。
292. `AssetScreenshot::url / caption / width / height`。
293. `AssetVideo::url / thumbnail_url / duration_seconds`。
294. `AssetDependency::id / name / version_range`。
295. `AssetCompatibility::min_engine_version / max_engine_version / platforms`。
296. `PlatformFlag::WINDOWS / LINUX / MACOS / ANDROID / IOS / WEBASSEMBLY`。
297. `AssetStoreClient::is_compatible(asset, current_engine_version, current_platform) -> bool`。
298. `DownloadState::Idle / Queued / Downloading(progress) / Completed / Failed(error)`。
299. `DownloadProgress::bytes_downloaded / bytes_total / speed_kbps / eta_seconds`。
300. `AssetStoreClient::download_async(id, on_progress) -> JoinHandle<Result<PathBuf>>`。
301. `AssetStoreClient::cancel_download(id) -> bool`。
302. `AssetStoreClient::pause_download(id) -> bool`。
303. `AssetStoreClient::resume_download(id) -> bool`。
304. `DownloadManager::queue(&self) -> Vec<DownloadTask>`。
305. `DownloadTask::asset_id / state / start_time`。
306. `InstallResult::installed / already_installed / version_skipped / conflicts`。
307. `Installer::install(pkg_path, target) -> Result<InstallReport>`。
308. `InstallReport::install_path / files_installed / dependencies_installed / time_elapsed_ms`。
309. `Installer::uninstall(asset_id) -> Result<()>`。
310. `Installer::verify_install(asset_id) -> Result<bool>`。
311. `DependencyResolution::resolved_graph / conflicts / missing_packages`。
312. `ConflictResolver::automatic_resolve_option(conflict) -> Option<Resolution>`。
313. `ConflictResolver::ask_user_resolve(conflict) -> Resolution`。
314. `VersionRange::parse(string) -> Result<Self>`。
315. `VersionRange::contains(&self, version) -> bool`。
316. `InstalledAsset::check_update(&self, client) -> Result<Option<AssetVersion>>`。
317. `InstalledAsset::installed_files(&self) -> &[PathBuf]`。
318. `InstalledAsset::total_size_bytes(&self) -> u64`。
319. `UpdatePolicy::Auto / Notify / Manual`。
320. `AssetStoreClient::set_update_policy(policy) -> ()`。
321. `AssetStoreClient::update_all(&mut self) -> Vec<Result<InstalledAsset>>`。
322. `VersionHistory::versions(&self) -> Vec<AssetVersion>`。
323. `VersionHistory::changelog_for(version) -> Option<&str>`。
324. `RollbackManager::create_snapshot(asset_id) -> SnapshotId`。
325. `RollbackManager::list_snapshots(asset_id) -> Vec<Snapshot>`。
326. `RollbackManager::delete_snapshot(id) -> Result<()>`。
327. `Snapshot::id / asset_id / from_version / to_version / created_at / size_bytes`。
328. `.rgepkg` 文件结构：`manifest.yaml` / `files/` / `signature.bin` / `checksums.txt`。
329. `RgePkg::manifest(&self) -> &RgePkgManifest`。
330. `RgePkg::file_entries(&self) -> Vec<FileEntry>`。
331. `FileEntry::path / size / sha256 / permissions`。
332. `RgePkg::extract_file(&self, member_path, output) -> Result<()>`。
333. `SigningKey::generate() -> Self`。
334. `SigningKey::from_pem(path) -> Result<Self>`。
335. `SigningKey::public_key(&self) -> PublicKey`。
336. `PublicKey::verify(&self, signature, message) -> bool`。
337. `RgePkg::sign(&mut self, key) -> Result<()>`。
338. `RgePkg::has_signature(&self) -> bool`。
339. `RgePkg::signer_key_id(&self) -> Option<String>`。
340. `Cart::add(asset_id) -> Result<()>`。
341. `Cart::remove(asset_id) -> Result<()>`。
342. `Cart::items(&self) -> Vec<CartItem>`。
343. `Cart::total(&self) -> Money`。
344. `Cart::checkout(&self, payment_method) -> Result<Order>`。
345. `CartItem::asset_id / name / price / quantity`。
346. `Money::amount / currency`。
347. `Money::to_string(&self) -> String`。
348. `PaymentMethod::CreditCard / PayPal / Alipay / WeChatPay / StoreCredit`。
349. `Order::id / items / total / status / created_at / payment_id`。
350. `OrderStatus::Pending / Paid / Shipped / Completed / Refunded / Cancelled`。
351. `AssetStoreClient::orders(&self, page) -> Result<Paged<Order>>`。
352. `AssetStoreClient::order_detail(id) -> Result<Order>`。
353. `MyAssets::owned(&self) -> Vec<OwnedAsset>`。
354. `OwnedAsset::asset_id / purchase_date / license_key / download_count`。
355. `MyAssets::download(asset_id) -> Result<PathBuf>`。
356. `DeveloperDraft::title / description / category / tags / price_model / files_dir / screenshots / videos`。
357. `DeveloperCenter::save_draft(draft) -> DraftId`。
358. `DeveloperCenter::update_draft(id, patch) -> Result<()>`。
359. `DeveloperCenter::delete_draft(id) -> Result<()>`。
360. `DeveloperCenter::drafts(&self) -> Vec<Draft>`。
361. `ReviewStatus::Draft / Submitted / UnderReview / Approved / Rejected(reason)`。
362. `ReviewChecklist::description_ok / screenshots_ok / metadata_ok / license_ok / no_malware / engine_version_ok`。
363. `DeveloperCenter::review_status(asset_id) -> Result<ReviewStatus>`。
364. `DeveloperCenter::withdraw_from_review(asset_id) -> Result<()>`。
365. `RevenueReport::period / gross_revenue / net_revenue / downloads / refunds`。
366. `DownloadStats::period / total_downloads / by_country / by_platform`。
367. `DeveloperCenter::payout_settings(&self) -> Result<PayoutSettings>`。
368. `DeveloperCenter::set_payout_settings(settings) -> Result<()>`。
369. `RevenueSplit::custom(dev_percent, platform_percent) -> Self`。
370. `RevenueSplit::premium() -> Self`（开发者 80% / 平台 20%）。
371. `CommentModeration::flag(comment_id, reason) -> Result<()>`。
372. `CommentModeration::delete(comment_id) -> Result<()>`（作者/管理员）。
373. `CommentModeration::edit(comment_id, new_content) -> Result<()>`。
374. `RatingDistribution::one_star / two_star / three_star / four_star / five_star`。
375. `AssetRating::distribution(&self) -> RatingDistribution`。
376. `Subscription::tier / price / period / start_date / next_billing_date / status`。
377. `SubscriptionStatus::Active / Cancelled / Expired / PastDue`。
378. `AssetStoreClient::subscribe(asset_id, tier) -> Result<Subscription>`。
379. `AssetStoreClient::cancel_subscription(id) -> Result<()>`。
380. `AssetStoreClient::active_subscriptions() -> Vec<Subscription>`。
381. `OfflineCache::prefetch(&self, asset_ids) -> Result<()>`。
382. `OfflineCache::available_offline(&self) -> Vec<InstalledAsset>`。
383. `OfflineCache::last_sync_time(&self) -> Option<DateTime>`。
384. `OfflineCache::sync(&mut self) -> Result<()>`。
385. `LocalLibrary::scan(path) -> Result<Vec<InstalledAsset>>`。
386. `LocalLibrary::add(installed) -> Result<()>`。
387. `LocalLibrary::remove(asset_id) -> Result<()>`。
388. `LocalLibrary::export_collection(path) -> Result<()>`。
389. `LocalLibrary::import_collection(path) -> Result<Vec<InstalledAsset>>`。
390. `AssetStoreClient::rate_limit_status(&self) -> RateLimitStatus`。
391. `RateLimitStatus::requests_remaining / reset_timestamp`。
392. `AssetStoreError::Network / Auth / NotFound / RateLimit / Payment / Conflict / Signature / Unknown`。
393. `AssetStoreError::message(&self) -> String`。
394. `AssetStoreClient::set_retry_policy(max_attempts, backoff) -> ()`。
395. `AssetStoreClient::set_timeout(seconds) -> ()`。
396. `AssetStoreClient::user_agent(&self) -> String`。
397. `examples/store_browse`：首页 + 分类 + 搜索 + 详情页 UI。
398. `examples/store_purchase`：购物车 + 支付模拟 + 订单查看。
399. `examples/store_install`：下载进度条 + 安装 + 更新 + 回滚演示。
400. `examples/store_install`：离线模式演示。

### 3.2 模板系统（engine-template）

401. `TemplateManager::register_template(template) -> TemplateId`。
402. `TemplateManager::unregister_template(id) -> Result<()>`。
403. `TemplateManager::template_count(&self) -> usize`。
404. `TemplateManager::reload() -> Result<()>`。
405. `TemplateId::new(uuid) -> Self`。
406. `TemplateId::parse(s) -> Result<Self>`。
407. `Template::version(&self) -> &str`。
408. `Template::engine_version_required(&self) -> &str`。
409. `Template::is_compatible(&self, engine_version) -> bool`。
410. `Template::files_count(&self) -> usize`。
411. `Template::thumbnail_path(&self) -> Option<&Path>`。
412. `Template::readme_content(&self) -> Option<&str>`。
413. `Template::tags(&self) -> &[String]`。
414. `TemplateFilter::category / game_type / engine_version / tags`。
415. `TemplateManager::filter(filter) -> Vec<Template>`。
416. `TemplateManager::search(keyword) -> Vec<Template>`。
417. `TemplateManager::featured() -> Vec<Template>`。
418. `TemplateManager::recent() -> Vec<Template>`。
419. `CreateProjectOptions::project_name / output_dir / overwrite / init_git / run_cargo_check`。
420. `TemplateManager::create_project_with_options(id, options) -> Result<Project>`。
421. `Project::name(&self) -> &str`。
422. `Project::path(&self) -> &Path`。
423. `Project::cargo_toml(&self) -> &Path`。
424. `Project::main_scene(&self) -> &Path`。
425. `Project::exists(&self) -> bool`。
426. `Project::is_initialized(&self) -> bool`。
427. `Project::build(&self) -> Result<Output>`。
428. `Project::run(&self) -> Result<Output>`。
429. `Project::test(&self) -> Result<Output>`。
430. `Project::read_cargo_toml(&self) -> Result<CargoToml>`。
431. `CargoToml::package_name / version / edition / authors / description / dependencies`。
432. `CargoTomlDependency::name / version / path / git / features`。
433. `TemplateContent::files(&self) -> Vec<TemplateFile>`。
434. `TemplateFile::source_path / target_path / is_binary / content_hash`。
435. `TemplateBuilder::new(name) -> Self`。
436. `TemplateBuilder::category(cat) -> Self`。
437. `TemplateBuilder::game_type(gt) -> Self`。
438. `TemplateBuilder::description(s) -> Self`。
439. `TemplateBuilder::add_file(source, target) -> Self`。
440. `TemplateBuilder::add_directory(dir) -> Self`。
441. `TemplateBuilder::thumbnail(path) -> Self`。
442. `TemplateBuilder::build(&self) -> Result<Template>`。
443. `Template::save_zip(&self, path) -> Result<()>`。
444. `Template::load_zip(path) -> Result<Self>`。
445. `TemplateCache::get(id) -> Option<&Template>`。
446. `TemplateCache::insert(template) -> ()`。
447. `TemplateCache::invalidate(id) -> ()`。
448. `TemplateCache::clear() -> ()`。
449. `ProjectInitializer::init_git_repo(project) -> Result<()>`。
450. `ProjectInitializer::write_default_config(project) -> Result<()>`。
451. `ProjectInitializer::write_readme(project) -> Result<()>`。
452. `ProjectInitializer::write_gitignore(project) -> Result<()>`。
453. `Template::validate(&self) -> Result<()>`。
454. `Template::validate_required_files(&self) -> Result<()>`。
455. `Template::validate_engine_version(&self) -> Result<()>`。
456. `Template::validate_manifest(&self) -> Result<()>`。
457. `TemplateVariable::PROJECT_NAME / AUTHOR / ENGINE_VERSION / CREATION_DATE`。
458. `TemplateVariable::replace_in(content, context) -> String`。
459. `TemplateVariableContext::project_name / author / engine_version / date`。
460. `TemplateTypeDisplay::label(&self) -> &str`。
461. `TemplateTypeDisplay::icon(&self) -> &str`。
462. `TemplateTypeDisplay::description(&self) -> &str`。
463. `TemplateGameTypeDisplay::label(&self) -> &str`。
464. `TemplateGameTypeDisplay::icon(&self) -> &str`。
465. `BuiltInTemplates::all() -> Vec<Template>`。
466. `BuiltInTemplates::empty_2d() -> Template`。
467. `BuiltInTemplates::empty_3d() -> Template`。
468. `BuiltInTemplates::empty_vr() -> Template`。
469. `BuiltInTemplates::empty_ar() -> Template`。
470. `BuiltInTemplates::fps() -> Template`。
471. `BuiltInTemplates::tps() -> Template`。
472. `BuiltInTemplates::rpg() -> Template`。
473. `BuiltInTemplates::racing() -> Template`。
474. `BuiltInTemplates::platformer_2d() -> Template`。
475. `BuiltInTemplates::puzzle() -> Template`。
476. `BuiltInTemplates::card_game() -> Template`。
477. `BuiltInTemplates::roguelike() -> Template`。
478. `BuiltInTemplates::visual_novel() -> Template`。
479. `BuiltInTemplates::tower_defense() -> Template`。
480. `BuiltInTemplates::tutorial_first_project() -> Template`。
481. `examples/template_new`：从 BuiltInTemplates::fps() 创建项目并编译运行。
482. `examples/template_new`：验证 Cargo.toml 正确生成。
483. `examples/template_new`：验证主场景文件存在。
484. `examples/template_custom`：构造自定义模板 → 保存 zip → 加载 zip → 从 zip 创建项目。
485. `examples/template_custom`：验证模板变量替换正确。

### 3.3 Profiler 性能面板（engine-profiler）

486. `ProfilerConfig::default() -> Self`。
487. `ProfilerConfig::sample_rate_hz(&self) -> u32`。
488. `ProfilerConfig::max_frames(&self) -> usize`。
489. `ProfilerConfig::enabled_categories(&self) -> ProfilerCategories`。
490. `ProfilerCategories::cpu / gpu / memory / render / network / script`。
491. `Profiler::with_config(config) -> Self`。
492. `Profiler::toggle(&mut self, category) -> ()`。
493. `Profiler::is_enabled(&self, category) -> bool`。
494. `Profiler::clear(&mut self) -> ()`。
495. `Profiler::frame_count(&self) -> usize`。
496. `Profiler::current_frame_number(&self) -> u64`。
497. `FrameScope::new(profiler) -> Self`。
498. `FrameScope::drop(&mut self) -> ()`（自动调用 end_frame）。
499. `CpuSample::scope_name / thread_id / start_ns / duration_ns / parent_index`。
500. `CpuSample::exclusive_duration(&self) -> u64`。
501. `GpuSample::scope_name / queue_index / start_ns / duration_ns / gpu_timer_id`。
502. `MemorySample::event_type / bytes / address / timestamp_ns / thread_id`。
503. `MemoryEventType::Alloc / Dealloc / Realloc / GarbageCollect`。
504. `RenderSample::draw_call_index / pipeline / vertices / indices / textures_bound / shader_name`。
505. `NetworkSample::direction / bytes / protocol / remote_addr / timestamp_ns`。
506. `ScriptSample::function_name / file / line / duration_ns / invocations`。
507. `Profiler::add_cpu_sample(&mut self, sample) -> ()`。
508. `Profiler::add_gpu_sample(&mut self, sample) -> ()`。
509. `Profiler::add_memory_sample(&mut self, sample) -> ()`。
510. `Profiler::add_render_sample(&mut self, sample) -> ()`。
511. `Profiler::add_network_sample(&mut self, sample) -> ()`。
512. `Profiler::add_script_sample(&mut self, sample) -> ()`。
513. `Profiler::samples_for_frame(&self, frame_idx) -> FrameSamples`。
514. `FrameSamples::frame_number / cpu / gpu / memory / render / network / script`。
515. `ScopeGuard::new(profiler, name) -> Self`。
516. `ScopeGuard::drop(&mut self) -> ()`（自动 end_scope）。
517. `profile_scope!` 宏捕获函数名与行号。
518. `profile_scope_data!(tag, key=value)` 支持附加上下文。
519. `profile_event!` 宏记录一次性事件。
520. `profile_event_data!("name", { "key": value })`。
521. `FlameGraphNode::total_duration(&self) -> u64`。
522. `FlameGraphNode::self_duration(&self) -> u64`。
523. `FlameGraphNode::percent_of_parent(&self) -> f64`。
524. `FlameGraphNode::percent_of_total(&self) -> f64`。
525. `FlameGraph::root(&self) -> &FlameNode`。
526. `FlameGraph::search(&self, keyword) -> Vec<&FlameNode>`。
527. `FlameGraph::hot_path(&self) -> Vec<&FlameNode>`。
528. `FlameGraph::render_text(&self, width, height) -> String`。
529. `FlameGraph::render_json(&self) -> String`。
530. `TimelineTrack::name / samples / color / row_index`。
531. `Timeline::total_duration(&self) -> u64`。
532. `Timeline::track_count(&self) -> usize`。
533. `Timeline::zoom(&mut self, start_ratio, end_ratio) -> ()`。
534. `Timeline::pan(&mut self, offset_ratio) -> ()`。
535. `Timeline::cursor_time(&self) -> u64`。
536. `Timeline::set_cursor(&mut self, time_ns) -> ()`。
537. `HistogramBucket::start / end / count`。
538. `Histogram::buckets(&self) -> &[HistogramBucket]`。
539. `Histogram::min(&self) -> f64`。
540. `Histogram::max(&self) -> f64`。
541. `Histogram::std_dev(&self) -> f64`。
542. `Histogram::render_text(&self, width, height) -> String`。
543. `LineChartPoint::timestamp / value`。
544. `LineChart::points(&self) -> &[LineChartPoint]`。
545. `LineChart::min_value(&self) -> f64`。
546. `LineChart::max_value(&self) -> f64`。
547. `LineChart::trend_slope(&self) -> f64`。
548. `MetricsSnapshot::timestamp / frame_time_ms / fps / cpu / gpu / memory_rss_mb / memory_vss_mb / net_in_kbs / net_out_kbs / disk_read_kbs / disk_write_kbs`。
549. `MetricsCollector::snapshot(&mut self) -> MetricsSnapshot`。
550. `MetricsCollector::history(&self, window) -> Vec<MetricsSnapshot>`。
551. `MetricsCollector::moving_average(window_seconds) -> MetricsSnapshot`。
552. `FrameMetricsAggregator::push(&mut self, frame) -> ()`。
553. `FrameMetricsAggregator::average_frame_time(&self) -> f64`。
554. `FrameMetricsAggregator::average_fps(&self) -> f64`。
555. `FrameMetricsAggregator::average_draw_calls(&self) -> f64`。
556. `FrameMetricsAggregator::total_triangles(&self) -> u64`。
557. `FrameMetricsAggregator::total_vertices(&self) -> u64`。
558. `CallStack::frames(&self) -> &[StackFrame]`。
559. `CallStack::depth(&self) -> usize`。
560. `StackFrame::function / file / line / column / address`。
561. `StackFrame::display(&self) -> String`。
562. `SymbolResolver::load_symbols(executable) -> Result<Self>`。
563. `SymbolResolver::cache(&self) -> &SymbolCache`。
564. `SymbolCache::get(address) -> Option<&Symbol>`。
565. `SymbolCache::insert(address, symbol) -> ()`。
566. `SymbolCache::invalidate() -> ()`。
567. `HardwareCounterSet::available_counters() -> Vec<HardwareCounter>`。
568. `HardwareCounterSet::start_all(&mut self) -> ()`。
569. `HardwareCounterSet::stop_all(&mut self) -> ()`。
570. `HardwareCounterSet::read_all(&self) -> HashMap<HardwareCounter, u64>`。
571. `CpuCounter::Cycles / Instructions / CacheReferences / CacheMisses / BranchInstructions / BranchMisses / BusCycles`。
572. `GpuCounter::VertexShaderNs / FragmentShaderNs / ComputeShaderNs / TessControlNs / TessEvalNs / GeometryShaderNs / TransferNs`。
573. `PerfData::cpu_counters(&self) -> HashMap<CpuCounter, u64>`。
574. `PerfData::gpu_counters(&self) -> HashMap<GpuCounter, u64>`。
575. `RemoteProfilerProtocol::VERSION -> u32`。
576. `RemoteProfilerProtocol::HELLO / SAMPLE_BATCH / DEVICE_INFO / DISCONNECT`。
577. `RemoteMessage::serialize(&self) -> Vec<u8>`。
578. `RemoteMessage::deserialize(buf) -> Result<Self>`。
579. `DeviceInfo::name / os / cpu_brand / gpu_brand / ram_gb / screen_resolution`。
580. `SampleBatch::timestamp / device_id / cpu_samples / gpu_samples / metrics`。
581. `RemoteProfilerServer::connected_clients(&self) -> usize`。
582. `RemoteProfilerServer::broadcast(&mut self, message) -> Result<usize>`。
583. `RemoteProfilerServer::sessions(&self) -> Vec<RemoteSession>`。
584. `RemoteProfilerClient::server_info(&self) -> ServerInfo`。
585. `RemoteProfilerClient::stream(&mut self, callback) -> Result<()>`。
586. `PerformanceDiagnosticEngine::new(rules) -> Self`。
587. `PerformanceDiagnosticEngine::add_rule(&mut self, rule) -> ()`。
588. `PerformanceDiagnosticEngine::run(&self, profile) -> Vec<PerformanceWarning>`。
589. `DiagnosticRuleSet::default() -> Self`。
590. `DiagnosticRuleSet::all_rules() -> Vec<DiagnosticRule>`。
591. `DiagnosticRule::excessive_gc_pauses / high_gpu_latency / texture_upload_stall / asset_load_on_critical_path / unbatched_draw_calls`。
592. `PerformanceWarning::suggestion_code(&self) -> Option<&str>`。
593. `PerformanceWarning::documentation_url(&self) -> Option<&str>`。
594. `PerformanceWarning::severity_rank(&self) -> u8`。
595. `DiagnosticReport::warnings(&self) -> &[PerformanceWarning]`。
596. `DiagnosticReport::summary(&self) -> String`。
597. `DiagnosticReport::critical_count(&self) -> usize`。
598. `DiagnosticReport::warning_count(&self) -> usize`。
599. `DiagnosticReport::info_count(&self) -> usize`。
600. `BaselineManager::new(dir) -> Self`。
601. `BaselineManager::save(&self, name, profile) -> Result<()>`。
602. `BaselineManager::load(&self, name) -> Result<BaselineProfile>`。
603. `BaselineManager::list(&self) -> Vec<String>`。
604. `BaselineManager::delete(&self, name) -> Result<()>`。
605. `BaselineComparison::metric_name / baseline_mean / new_mean / delta_percent / is_regression`。
606. `RegressionDetector::t_test(&self, baseline, samples, alpha) -> bool`（统计显著性）。
607. `RegressionDetector::threshold_check(baseline, samples, percent_threshold) -> bool`。
608. `RegressionReport::print(&self) -> String`。
609. `RegressionReport::to_json(&self) -> String`。
610. `RegressionReport::has_regressions(&self) -> bool`。
611. `RegressionReport::regression_count(&self) -> usize`。
612. `.rgeprofile` 头部：magic / version / compression / encryption_flag / metadata_len`。
613. `.rgeprofile` 压缩：zstd / gzip / none`。
614. `.rgeprofile` 可选 AES-256-GCM 加密。
615. `RgeProfile::set_encryption_key(key) -> ()`。
616. `RgeProfile::has_encryption(&self) -> bool`。
617. `RgeProfile::file_size_bytes(&self) -> u64`。
618. `RgeProfile::metadata(&self) -> ProfileMetadata`。
619. `ProfileMetadata::engine_version / captured_at / device_info / total_samples / duration_seconds`。
620. `ProfileData::frames / cpu_samples / gpu_samples / memory_samples / events / symbols`。
621. `ProfileViewer::open(path) -> Result<Self>`。
622. `ProfileViewer::frame_summary(&self, frame_idx) -> FrameSummary`。
623. `ProfileViewer::flame_graph(&self, frame_range) -> FlameGraph`。
624. `ProfileViewer::timeline(&self, frame_range) -> Timeline`。
625. `ProfileViewer::export_csv(&self, path) -> Result<()>`。
626. `ProfileViewer::export_json(&self, path) -> Result<()>`。
627. `ProfileSummary::total_frames / avg_fps / avg_frame_time_ms / total_samples / file_size_bytes`。
628. `examples/profiler_window`：打开窗口显示 CPU/GPU/内存/渲染多 Tab。
629. `examples/profiler_window`：Flame Graph 可视化展示。
630. `examples/profiler_window`：Timeline 时间轴展示。
631. `examples/profiler_window`：FPS/帧时间折线图实时刷新。
632. `examples/profiler_window`：点击帧可跳转到具体帧详情。
633. `examples/profiler_remote`：启动远程采样服务器，移动设备连接。
634. `examples/profiler_remote`：桌面端接收远程样本并显示。
635. `examples/profiler_bench`：创建基准并与 baseline 比较。
636. `examples/profiler_bench`：导出 .rgeprofile 文件。
637. `examples/profiler_bench`：检测回归并输出报告。

### 3.4 文档生成（engine-docs）

638. `docs/book.toml` mdbook 配置。
639. `docs/src/SUMMARY.md` 目录。
640. `docs/src/introduction.md` 引言。
641. `docs/src/getting_started/installation.md` 安装指南。
642. `docs/src/getting_started/quick_start.md` 快速入门。
643. `docs/src/getting_started/first_project.md` 第一个项目。
644. `docs/src/concepts/ecs_overview.md` ECS 概览。
645. `docs/src/concepts/entities_components_systems.md` Entity/Component/System。
646. `docs/src/concepts/asset_system.md` 资源系统。
647. `docs/src/concepts/scene_graph.md` 场景图。
648. `docs/src/concepts/rendering_pipeline.md` 渲染管线。
649. `docs/src/concepts/physics_engine.md` 物理引擎。
650. `docs/src/concepts/event_system.md` 事件系统。
651. `docs/src/guides/rendering_2d.md` 2D 渲染指南。
652. `docs/src/guides/rendering_3d.md` 3D 渲染指南。
653. `docs/src/guides/physics.md` 物理指南。
654. `docs/src/guides/ui_guide.md` UI 指南。
655. `docs/src/guides/audio_guide.md` 音频指南。
656. `docs/src/guides/animation_guide.md` 动画指南。
657. `docs/src/guides/networking_guide.md` 联网指南。
658. `docs/src/guides/scripting_guide.md` 脚本指南。
659. `docs/src/guides/deployment_guide.md` 部署指南。
660. `docs/src/examples/hello_world.md` hello_world 示例说明。
661. `docs/src/examples/platformer_2d.md` 2D 平台跳跃说明。
662. `docs/src/examples/mini_3d.md` 3D 迷你场景说明。
663. `docs/src/examples/ui_demo.md` UI 演示说明。
664. `docs/src/examples/physics_demo.md` 物理演示说明。
665. `docs/src/examples/animation_demo.md` 动画演示说明。
666. `docs/src/examples/particles_demo.md` 粒子演示说明。
667. `docs/src/examples/network_demo.md` 联网演示说明。
668. `docs/src/examples/blueprint_demo.md` 蓝图演示说明。
669. `docs/src/api_reference/index.md` API 索引。
670. `docs/src/faq/general.md` 常见问题（通用）。
671. `docs/src/faq/build_issues.md` 构建问题。
672. `docs/src/faq/performance.md` 性能问题。
673. `docs/theme/index.hbs` 主题模板。
674. `docs/theme/css/general.css` 主题样式。
675. `docs/theme/css/chrome.css` 主题 Chrome。
676. `docs/theme/css/variables.css` 主题变量。
677. `docs/theme/favicon.svg` Favicon。
678. `docs/theme/logo.svg` Logo。
679. `docs/book.toml` 的 `output.html` 配置搜索功能。
680. `docs/book.toml` 的 `output.html` 自定义主题。
681. `docs/book.toml` 的 `output.html` Git 仓库链接。
682. `docs/book.toml` 的 `output.html` 语言选择。
683. `docs/zola.toml` 备用 zola 配置。
684. `docs/content/_index.md` zola 首页。
685. `docs/templates/base.html` zola 基础模板。
686. `docs/static/css/style.css` zola 样式。
687. `docs/zh/src/SUMMARY.md` 中文文档目录。
688. `docs/zh/src/introduction.md` 中文引言。
689. `docs/zh/src/getting_started/installation.md` 中文安装指南。
690. `docs/zh/src/getting_started/quick_start.md` 中文快速入门。
691. `docs/zh/src/getting_started/first_project.md` 中文第一个项目。
692. `docs/zh/src/concepts/ecs_overview.md` 中文 ECS 概览。
693. `docs/zh/src/guides/rendering_2d.md` 中文 2D 渲染指南。
694. `docs/zh/src/guides/deployment_guide.md` 中文部署指南。
695. `docs/en/src/SUMMARY.md` 英文文档目录。
696. `docs/en/src/introduction.md` 英文引言。
697. `docs/en/src/getting_started/installation.md` 英文安装指南。
698. `docs/en/src/concepts/ecs_overview.md` 英文 ECS 概览。
699. `docs/en/src/guides/rendering_2d.md` 英文 2D 渲染指南。
700. `docs/src/best_practices.md` 最佳实践。
701. `docs/src/performance_tuning.md` 性能调优指南。
702. `docs/src/safety_guide.md` 安全指南。
703. `docs/src/migration_guide.md` 迁移指南。
704. `docs/src/migration/from_0_9_to_1_0.md` v0.9 → v1.0 迁移。
705. `docs/src/migration/from_0_8_to_0_9.md` v0.8 → v0.9 迁移。
706. `docs/src/CHANGELOG.md` 变更日志（同根目录软链接）。
707. `docs/src/ROADMAP.md` 路线图。
708. `docs/src/CONTRIBUTING.md` 贡献指南。
709. `docs/src/CODE_OF_CONDUCT.md` 行为准则。
710. `docs/src/SECURITY.md` 安全披露政策。
711. `docs/search_index.json` 预构建搜索索引。
712. `docs/versions.json` 版本列表（供切换器使用）。
713. `docs/build.sh` 构建脚本。
714. `docs/watch.sh` 本地开发脚本（mdbook watch）。
715. `docs/serve.sh` 本地预览脚本（mdbook serve）。
716. `docs/export_pdf.sh` PDF 导出脚本（基于 wkhtmltopdf）。
717. `docs/export_tarball.sh` tar.gz 导出脚本。
718. `docs/version_selector.js` 版本切换器脚本。
719. `docs/language_selector.js` 语言切换器脚本。
720. `cargo doc --workspace --no-deps --document-private-items` 成功。
721. `cargo rustdoc -- --html-in-header custom.html` 自定义头部。
722. `rustdoc` 主题 CSS 自定义（与 mdbook 色板一致）。
723. 所有公开函数文档注释覆盖率 >= 95%。
724. 所有公开结构体文档注释覆盖率 100%。
725. 所有公开枚举文档注释覆盖率 100%。
726. 文档示例代码块均通过 `cargo test --doc`。
727. API 参考 `docs/src/api_reference/engine_asset_store.md`。
728. API 参考 `docs/src/api_reference/engine_template.md`。
729. API 参考 `docs/src/api_reference/engine_profiler.md`。
730. API 参考 `docs/src/api_reference/core_modules.md`。

### 3.5 教程生态

731. `videos/outline.md` 视频大纲文档。
732. `videos/episode_01_setup.md` 第 01 讲脚本：引擎介绍 + 环境安装 + IDE 配置。
733. `videos/episode_02_hello_world.md` 第 02 讲脚本：创建项目 + 运行空窗口。
734. `videos/episode_03_ecs_concepts.md` 第 03 讲脚本：ECS 介绍 + 数据驱动。
735. `videos/episode_04_ecs_api.md` 第 04 讲脚本：Entity/Component/System API 使用。
736. `videos/episode_05_assets.md` 第 05 讲脚本：资源导入 + 资源生命周期。
737. `videos/episode_06_render_2d.md` 第 06 讲脚本：Sprite + 摄像机 + 动画。
738. `videos/episode_07_physics_2d.md` 第 07 讲脚本：RigidBody + Collider + 事件。
739. `videos/episode_08_render_3d.md` 第 08 讲脚本：3D Mesh + 摄像机 + 光照。
740. `videos/episode_09_pbr.md` 第 09 讲脚本：PBR 材质 + IBL + 阴影。
741. `videos/episode_10_physics_3d.md` 第 10 讲脚本：3D 物理 + 碰撞层 + 关节。
742. `videos/episode_11_ui.md` 第 11 讲脚本：Widget + 布局 + 主题。
743. `videos/episode_12_input.md` 第 12 讲脚本：键盘 / 鼠标 / 手柄 / 触摸。
744. `videos/episode_13_animation.md` 第 13 讲脚本：Skeleton + Animator + Clip。
745. `videos/episode_14_state_machine.md` 第 14 讲脚本：状态机 + 混合树。
746. `videos/episode_15_particles.md` 第 15 讲脚本：粒子系统 + 后处理。
747. `videos/episode_16_audio.md` 第 16 讲脚本：音效 + 背景音乐 + 3D 音频。
748. `videos/episode_17_scripting.md` 第 17 讲脚本：脚本 API + 蓝图节点。
749. `videos/episode_18_networking.md` 第 18 讲脚本：客户端 / 服务器 + 状态同步。
750. `videos/episode_19_profiler.md` 第 19 讲脚本：Profiler 使用 + 诊断报告。
751. `videos/episode_20_deploy.md` 第 20 讲脚本：Windows / Linux / macOS / Android / iOS。
752. `videos/episode_21_asset_store.md` 第 21 讲脚本：资源商店 + 开发者中心。
753. `videos/episode_22_extension.md` 第 22 讲脚本：编辑器插件 + 自定义系统。
754. `videos/assets/slides/` 幻灯片素材目录。
755. `videos/assets/code/` 每讲示例代码目录。
756. 每讲脚本包含：`title / duration / prerequisites / outline / key_points / commands / demo_steps / summary / homework`。
757. 每讲时长约 20-40 分钟。
758. `videos/README.md` 视频总索引。
759. `videos/zh/` 中文字幕脚本目录。
760. `videos/en/` 英文字幕脚本目录。
761. `examples/hello_world`：`main.rs` + `Cargo.toml` + `README.md`。
762. `examples/hello_world` 窗口打开 + 标题正确。
763. `examples/hello_world` 事件循环正常退出。
764. `examples/2d_platformer`：玩家 + 平台 + 跳跃 + 敌人。
765. `examples/2d_platformer`：瓦片地图 + 碰撞检测。
766. `examples/2d_platformer`：分数 HUD。
767. `examples/3d_mini`：PBR 材质的立方体 + 方向光 + 摄像机轨道控制。
768. `examples/3d_mini`：天空盒 + 阴影。
769. `examples/ui_demo`：按钮 / 文本框 / 滑块 / 下拉框 / 弹窗。
770. `examples/ui_demo`：布局（水平 / 垂直 / 网格 / flex）。
771. `examples/physics_demo`：一堆碰撞物体 + 交互投放。
772. `examples/animation_demo`：行走循环 + IK 瞄准。
773. `examples/particles_demo`：火焰 / 烟雾 / 爆炸粒子效果。
774. `examples/network_demo`：客户端 / 服务器双向通信 + 玩家位置同步。
775. `examples/blueprint_demo`：蓝图节点图 + 执行。
776. `examples/README.md` 所有示例列表 + 运行说明。
777. 每个示例 `README.md` 包含简介 + 运行命令 + 截图链接。

### 3.6 发布流程

778. `RELEASES.md` 文档：版本生命周期说明。
779. `RELEASE_TRAIN.md` 文档：发布时间表与节奏。
780. 每 6 周一个 minor 版本。
781. 每 12 个月一个 major 版本。
782. 每个 LTS 版本 36 个月支持期。
783. 版本号 `MAJOR.MINOR.PATCH`（semver 2.0）。
784. `PRE-RELEASE TAG`：`-alpha.N` / `-beta.N` / `-rc.N`。
785. 构建发布候选版本：`cargo release --level rc`。
786. 构建正式版本：`cargo release --level patch|minor|major`。
787. 版本冻结期（beta 起）：仅修 bug，不引入新功能。
788. 发布前检查清单：`cargo test --workspace` 通过。
789. 发布前检查清单：`cargo clippy --workspace -- -D warnings` 通过。
790. 发布前检查清单：`cargo fmt --check --workspace` 通过。
791. 发布前检查清单：`cargo audit` 无严重漏洞。
792. 发布前检查清单：`cargo deny check licenses` 通过。
793. 发布前检查清单：所有 examples 至少一次 `cargo build --workspace --examples` 通过。
794. 发布前检查清单：CHANGELOG 更新。
795. 发布前检查清单：README 版本号同步。
796. 发布前检查清单：docs 版本切换器新增版本。
797. 发布前检查清单：Git tag 签名。
798. 发布前检查清单：GitHub Release Draft 准备。
799. 发布步骤脚本 `scripts/release.sh`。
800. 发布后检查：crates.io 包存在且可下载。
801. 发布后检查：GitHub Release Assets 存在。
802. 发布后检查：docs 站点新版本链接可访问。
803. 发布后检查：Docker Hub 新 tag 存在。
804. Hotfix 流程：从 `stable` 分支切 `hotfix/X.Y.Z`。
805. Hotfix 合并：回合并入 `stable` + `main`。
806. LTS 分支：`lts-1.x`，每季度回溯安全补丁。
807. 版本弃用警告：`#[deprecated(since = "1.1", note = "use X instead")]`。
808. 版本移除：弃用后两个 minor 版本移除。
809. 发布公告：博客 + Discord + 邮件列表 + 知乎 + B 站 + Reddit。
810. 版本兼容性表格：`docs/src/compatibility.md`。

### 3.7 CI/CD

811. `.github/workflows/ci.yml` 主 CI 工作流。
812. `.github/workflows/release.yml` Release 工作流。
813. `.github/workflows/docs.yml` 文档部署工作流。
814. `.gitea/workflows/ci.yml` Gitea 备用 CI。
815. 自建 Runner：Linux x86_64 / macOS arm64 / Windows x86_64。
816. CI 触发：push 到 `main` / PR / tag `v*`。
817. CI 步骤：checkout → toolchain → cache → test → clippy → fmt → doc → audit → deny → build examples → benchmark（可选）。
818. CI 矩阵 OS：`ubuntu-latest` / `macos-latest` / `windows-latest`。
819. CI 矩阵 Rust：`stable` / `beta` / `nightly`（nightly allow-failure）。
820. CI 构建 target：`x86_64-unknown-linux-gnu`。
821. CI 构建 target：`x86_64-pc-windows-msvc`。
822. CI 构建 target：`x86_64-apple-darwin`。
823. CI 构建 target：`aarch64-apple-darwin`。
824. CI 构建 target：`armv7-linux-androideabi`。
825. CI 构建 target：`aarch64-linux-android`。
826. CI 构建 target：`aarch64-apple-ios`。
827. CI 构建 target：`wasm32-unknown-unknown`。
828. CI 构建工具：`cross`（跨平台交叉编译）。
829. CI 缓存：`cargo cache` / `sccache`。
830. CI 缓存 key：`os` + `rust-version` + `hash(Cargo.lock)`。
831. CI 产物保留 30 天。
832. CI Android 构建：NDK r26b。
833. CI iOS 构建：Xcode 15。
834. CI WebAssembly 构建：`wasm-bindgen` + `wasm-opt`。
835. CI 测试覆盖率：`cargo llvm-cov` 上传到 codecov.io。
836. CI 基准测试：`criterion` 每次 main 分支 commit 跑基准。
837. CI 基准回归检测：基准较基线下降 >= 10% 标记警告。
838. CI fuzz 测试：`cargo fuzz` 10 分钟定时任务。
839. CI Miri：`cargo miri test` 每周一次。
840. CI ASan：`-Z sanitizer=address` 每日一次。
841. CI TSan：`-Z sanitizer=thread` 每日一次。
842. CI MSan：`-Z sanitizer=memory` 每日一次（nightly）。
843. CI 文档部署：`mdbook build` → `gh-pages` 分支。
844. crates.io 发布：`cargo publish --token $CARGO_REGISTRY_TOKEN`。
845. GitHub Release：`softprops/action-gh-release` 上传二进制。
846. Docker Hub：`docker/build-push-action` 推送镜像。
847. Homebrew Tap：推送到 `homebrew-tap` 仓库。
848. Scoop Bucket：推送 JSON 到 `scoop-bucket`。
849. Chocolatey：`choco push` 上传 nupkg。
850. Flatpak：`flatpak build-commit-from` 推送。
851. Snap：`snapcraft push` 上传 snap。
852. CI Badges：README 显示 `build` / `crates.io` / `docs` / `coverage` / `license`。
853. CI 通知失败到 Discord / Matrix。

### 3.8 社区工具

854. `.github/ISSUE_TEMPLATE/bug_report.md` Bug 报告模板。
855. `.github/ISSUE_TEMPLATE/feature_request.md` 功能请求模板。
856. `.github/ISSUE_TEMPLATE/question.md` 问题模板。
857. `.github/ISSUE_TEMPLATE/config.yml` 模板配置。
858. `.github/PULL_REQUEST_TEMPLATE.md` PR 模板。
859. PR 模板字段：`Description / Motivation / Changes / Testing / Checklist / Screenshots`。
860. PR Checklist：`cargo test / clippy / fmt / doc / examples build`。
861. Label bot：根据路径自动打 `A-asset-store / A-template / A-profiler / A-docs / C-bug / C-feature` 标签。
862. Label bot：根据 PR size 打 `S-small / S-medium / S-large` 标签。
863. Label bot：根据 title 前缀打 `bug / feat / docs / ci / refactor / perf` 标签。
864. Stale bot：60 天无活动 issue → `stale`，再 7 天关闭。
865. Stale bot：PR 45 天无活动 → `stale-pr`。
866. Dependabot：`dependabot.yml` 每周检查依赖更新。
867. Discord：#announcements / #general / #help / #development / #showcase 频道。
868. Matrix：#engine:matrix.org 桥接 Discord。
869. 微信群：二维码 + 机器人助手。
870. QQ 群：群号 + 管理员列表。
871. 知乎专栏：每月一篇深度文章。
872. B 站：教学视频发布 + 直播答疑。
873. Reddit：r/rustgameengine 子版块。
874. 邮件列表：`engine-announce@` / `engine-users@` / `engine-dev@`。
875. 博客 `blog.engine.example.com`：静态站点。
876. 博客首页：最新文章 + 精选 + 分类。
877. 博客 RSS：`feed.xml`。
878. 更新日志推送：每个 Sprint 结束发布一篇更新博客。
879. 路线图公开：`ROADMAP.md` 季度更新。
880. 路线图章节：`已发布 / 进行中 / 规划中 / 远期 / 想法`。
881. `cargo rge-migrate` 子命令：自动迁移旧项目配置。
882. `cargo rge-migrate --from 0.9 --to 1.0`。
883. `cargo rge-migrate --dry-run`。
884. 弃用 API 列表 `docs/src/deprecated.md`。
885. 安全披露政策：`SECURITY.md`。
886. 安全披露邮箱：`security@engine.example.com`。
887. 安全披露响应时间承诺：48 小时内确认，90 天内修复。
888. GitHub Security Advisory：公开前 embargo。
889. CVE 流程：发现 → 内部修复 → 预披露期 → 公开发布 + CVE-ID。
890. `assets/logo/engine-logo.svg` 矢量 Logo。
891. `assets/logo/engine-logo-black.svg`。
892. `assets/logo/engine-logo-white.svg`。
893. `assets/logo/favicon.svg`。
894. `assets/logo/engine-logo-128.png`。
895. `assets/logo/engine-logo-256.png`。
896. `assets/logo/engine-logo-512.png`。
897. `assets/brand/palette.md` 色板（主色 / 辅色 / 中性色）。
898. `assets/brand/typography.md` 字体规范。
899. `assets/brand/guide.md` 品牌使用指南（Do / Don't）。
900. `assets/brand/social-preview.png` 社交分享预览图。

### 3.9 示例与测试

901. `cargo test --workspace` 全部通过。
902. 单元测试覆盖率目标 >= 70%。
903. 单元测试覆盖率目标 core crate >= 80%。
904. `cargo test -p engine-asset-store` 通过。
905. `cargo test -p engine-template` 通过。
906. `cargo test -p engine-profiler` 通过。
907. `cargo test -p engine-docs-meta` 通过。
908. 集成测试 `tests/integration/` 目录。
909. 集成测试 `tests/integration/asset_store.rs`。
910. 集成测试 `tests/integration/template_creation.rs`。
911. 集成测试 `tests/integration/profiler_sampling.rs`。
912. 集成测试 `tests/integration/profile_roundtrip.rs`（.rgeprofile 写读）。
913. `cargo clippy --workspace --all-targets -- -D warnings`。
914. `cargo fmt --check --all --manifest-path .`。
915. `cargo doc --workspace --no-deps --document-private-items`。
916. `cargo doc --workspace --all-features`。
917. `cargo doc --workspace --no-default-features`。
918. 文档内代码块可运行测试：`cargo test --doc`。
919. `cargo build --workspace --examples` 成功。
920. `cargo run --example hello_world` 3 秒后正常退出。
921. `cargo run --example 2d_platformer`（headless 模式验证启动）。
922. `cargo run --example 3d_mini`（headless 模式验证启动）。
923. `cargo run --example ui_demo`（headless 模式验证启动）。
924. `cargo run --example physics_demo`（headless 模式验证启动）。
925. `cargo run --example animation_demo`（headless 模式验证启动）。
926. `cargo run --example particles_demo`（headless 模式验证启动）。
927. `cargo run --example network_demo`（客户端 + 服务器）。
928. `cargo run --example blueprint_demo`（headless 模式验证启动）。
929. `cargo run --example store_browse`（headless 模式验证启动）。
930. `cargo run --example store_purchase`（headless 模式验证启动）。
931. `cargo run --example store_install`（headless 模式验证启动）。
932. `cargo run --example template_new`（headless 模式验证启动）。
933. `cargo run --example template_custom`（headless 模式验证启动）。
934. `cargo run --example profiler_window`（headless 模式验证启动）。
935. `cargo run --example profiler_remote`（headless 模式验证启动）。
936. `cargo run --example profiler_bench`（headless 模式验证启动）。
937. criterion 基准 `benches/asset_store.rs`。
938. criterion 基准 `benches/profiler.rs`。
939. criterion 基准 `benches/ecs.rs`。
940. criterion 基准 `benches/render.rs`。
941. criterion 基准报告 `target/criterion/`。
942. `cargo bench --workspace` 成功。
943. `cargo fuzz init` fuzz 目录。
944. `fuzz/fuzz_targets/asset_pkg.rs` 打包 fuzz。
945. `fuzz/fuzz_targets/profile_import.rs` .rgeprofile 导入 fuzz。
946. `fuzz/fuzz_targets/json_parser.rs` JSON 解析 fuzz。
947. `cargo fuzz run asset_pkg -- -max_total_time=60`。
948. `cargo miri test --workspace`（weekly on nightly）。
949. `RUSTFLAGS="-Z sanitizer=address" cargo test --workspace --target x86_64-unknown-linux-gnu`。
950. `RUSTFLAGS="-Z sanitizer=thread" cargo test --workspace --target x86_64-unknown-linux-gnu`。
951. `RUSTFLAGS="-Z sanitizer=memory" cargo test --workspace --target x86_64-unknown-linux-gnu`（nightly）。
952. `CHANGELOG.md` 更新至 v1.0.0。
953. `CHANGELOG.md` 包含 Added / Changed / Deprecated / Removed / Fixed / Security 六部分。
954. `README.md` 版本号改为 v1.0.0。
955. `README.md` 包含 Badges / 简介 / 特性 / 快速开始 / 示例 / 文档 / 生态 / 社区 / 许可证。
956. 各 crate `README.md` 更新到 v1.0.0。
957. 正式发布到 crates.io：`cargo publish`（各 crate 按依赖顺序）。
958. crates.io 页面描述 / 关键字 / 分类 / 仓库链接正确。
959. crates.io `categories`：`game-engines / rendering / simulation / wasm`。
960. crates.io `keywords`：`engine / gfx / ecs / 3d / 2d`。

> 以上 960 条细分需求构成 Sprint 16 完整验收标准。核心验收示例 `examples/store_browse` 与 `examples/profiler_window` 必须可运行并通过集成测试。
