# Sprint 08 · 跨平台打包与资源管线

> 阶段：阶段二 · 编辑器 + 跨平台（第 4 个 Sprint）  
> 周期：4 周  
> 核心目标：实现 PC / Mobile / Web / 小程序 统一打包管线与资源处理  
> 验收：一键打包到 Windows/macOS/Linux/Android/iOS/Web，真机/浏览器可运行

---

## 一、Sprint 概览

本 Sprint 建立 `engine-build` crate。核心交付：

- `BuildPipeline`：编译+资源处理+签名+输出安装包
- `AssetPipeline`：资源扫描/导入/合批/压缩/加密/增量差分
- `PlatformTarget`：Windows/macOS/Linux/Android/iOS/Web/微信小程序
- `BuildProfile`：Debug / Release / Ship
- `BuildConfig`：构建配置（app ID、版本、图标、启动页、权限）
- `Package`：最终产物（exe / app / apk / ipa / wasm / miniapp）
- `HotUpdate`：差分热更新（后续 Sprint 完善，本阶段提供基础）
- `examples/build_cli`：`engine build --target android --profile release`
- `examples/wasm_demo`：Web 版 hello world

---

## 二、项目需求清单

1. `engine-build` crate 建立。
2. `BuildPipeline::new(config)`。
3. `BuildPipeline::build(&self) -> Result<BuildArtifact>`。
4. `BuildPipeline::clean(&self)`。
5. `BuildPipeline::run(&self)` — 构建并运行（仅本机目标）。
6. `PlatformTarget::Windows / MacOS / Linux / Android / Ios / Web / MiniApp(WeChat/ByteDance/QQ)`。
7. `PlatformTarget::current() -> PlatformTarget`（当前主机平台）。
8. `PlatformTarget::supported(&self) -> bool`（当前主机可构建此目标）。
9. `Profile::Debug / Release / Ship`。
10. `Profile::optimization_level(&self) -> u8`。
11. `Profile::debug_info(&self) -> bool`。
12. `Profile::strip_symbols(&self) -> bool`。
13. `Profile::lto(&self) -> bool`。
14. `BuildConfig::app_name`。
15. `BuildConfig::app_id`（如 `com.example.myapp`）。
16. `BuildConfig::version`（如 `1.0.0`）。
17. `BuildConfig::version_code(i32)`。
18. `BuildConfig::icons(Vec<PathBuf>)`。
19. `BuildConfig::splash_screen`。
20. `BuildConfig::permissions(Vec<Permission>)`（网络/存储/相机/麦克风/定位等）。
21. `BuildConfig::orientation(Portrait / Landscape / Auto)`。
22. `BuildConfig::platform_target(&self) -> PlatformTarget`。
23. `BuildConfig::profile(&self) -> Profile`。
24. `BuildConfig::from_file(path) -> Result<Self>`。
25. `BuildConfig::save(&self, path)`。
26. `BuildConfig::with_assets_dir(dir)`。
27. `BuildConfig::with_output_dir(dir)`。
28. `BuildConfig::with_temp_dir(dir)`。
29. `BuildArtifact::path(&self) -> &Path`。
30. `BuildArtifact::size(&self) -> u64`。
31. `BuildArtifact::platform(&self) -> PlatformTarget`。
32. `BuildArtifact::version(&self) -> &str`。
33. `BuildArtifact::sign_info(&self) -> Option<&SignInfo>`。
34. `Toolchain::rust_version(&self) -> Version`。
35. `Toolchain::ndk_version(&self) -> Version`（Android）。
36. `Toolchain::xcode_version(&self) -> Version`（iOS）。
37. `Toolchain::node_version(&self) -> Version`（小程序）。
38. `Toolchain::detect() -> Result<Self>`（自动检测构建工具）。
39. `Compiler`：rustc cargo build 针对目标。
40. `Compiler::compile(&self, config) -> Result<PathBuf>`。
41. `Compiler::target_triple(&self) -> &str`（如 `aarch64-linux-android`）。
42. `Compiler::features(&self) -> Vec<String>`（启用的 Cargo features）。
43. `AndroidToolchain`：NDK/SDK/JDK 路径检测。
44. `AndroidToolchain::sign_apk(unsigned_apk, keystore, alias, password) -> Result<PathBuf>`。
45. `AndroidToolchain::zipalign(apk) -> Result<PathBuf>`。
46. `AndroidToolchain::aapt2_package(resources, manifest) -> Result<PathBuf>`。
47. `AndroidManifest::new(config) -> Self`。
48. `AndroidManifest::to_xml(&self) -> String`。
49. `AndroidManifest::permissions(&self) -> Vec<String>`。
50. `AndroidManifest::min_sdk(&self) -> u32`。
51. `AndroidManifest::target_sdk(&self) -> u32`。
52. `AndroidManifest::orientation(&self) -> &str`。
53. `AndroidManifest::activity_name(&self) -> &str`。
54. `IosToolchain::xcodebuild(proj) -> Result<PathBuf>`（仅 macOS）。
55. `IosToolchain::codesign(app, identity) -> Result<()>`。
56. `InfoPlist::new(config) -> Self`。
57. `InfoPlist::to_plist(&self) -> Value`。
58. `InfoPlist::bundle_id(&self) -> &str`。
59. `InfoPlist::version(&self) -> &str`。
60. `InfoPlist::required_device_capabilities(&self) -> Vec<String>`。
61. `WebToolchain::rustc_to_wasm(&self, src) -> Result<PathBuf>`。
62. `WebToolchain::wasm_bindgen(wasm, out_dir) -> Result<PathBuf>`。
63. `WebToolchain::wasm_opt(wasm) -> Result<PathBuf>`。
64. `WebToolchain::generate_html(&self, js, wasm) -> Result<PathBuf>`。
65. `WebToolchain::generate_service_worker(&self) -> Result<PathBuf>`（PWA）。
66. `MiniAppToolchain::pack(src_dir, out_file) -> Result<PathBuf>`（微信/抖音/QQ）。
67. `MiniAppToolchain::minify(js) -> Result<String>`。
68. `MiniAppToolchain::generate_manifest(config) -> Result<String>`。
69. `MiniAppToolchain::generate_app_js() -> Result<String>`。
70. `MiniAppToolchain::generate_project_config() -> Result<String>`。
71. `AssetPipeline::new(asset_dir)`。
72. `AssetPipeline::scan(&mut self)`。
73. `AssetPipeline::import_all(&mut self)`。
74. `AssetPipeline::reimport_changed(&mut self)`（基于 mtime 或 hash）。
75. `AssetPipeline::process_all(&mut self)` 合批/压缩/加密。
76. `AssetPipeline::package(&self, out_dir) -> Result<PathBuf>`。
77. `AssetPipeline::build_manifest(&self) -> AssetManifest`。
78. `AssetPipeline::incremental_hash(&self) -> String`。
79. `AssetPipeline::diff(from_manifest, to_manifest) -> DiffResult`。
80. `AssetManifest::to_json(&self) -> String`。
81. `AssetManifest::load_json(path) -> Result<Self>`。
82. `AssetManifest::entries(&self) -> &[AssetEntry]`。
83. `AssetEntry::path / hash / size / kind`。
84. `AssetKind::Texture / Audio / Model / Scene / Prefab / Font / Custom`。
85. `AssetCompress::Zstd / Gzip / Brotli / LZ4 / None`。
86. `AssetEncrypt::AesGcm128 / AesGcm256 / XorChaCha20 / None`。
87. `TextureProcessor::compress_bc(img) -> Result<Vec<u8>>`。
88. `TextureProcessor::compress_etc(img) -> Result<Vec<u8>>`。
89. `TextureProcessor::mipmap(img) -> Result<Vec<u8>>`。
90. `TextureProcessor::resize(img, size) -> Result<Image>`。
91. `TextureProcessor::pack_atlas(images, padding) -> Result<(Image, Vec<Rect>)>`。
92. `AudioProcessor::convert(src, format) -> Result<Vec<u8>>`。
93. `AudioProcessor::compress_ogg(src) -> Result<Vec<u8>>`。
94. `AudioProcessor::compress_mp3(src) -> Result<Vec<u8>>`。
95. `ModelProcessor::optimize(src) -> Result<Vec<u8>>`。
96. `SceneProcessor::bake(scene) -> Result<Vec<u8>>`。
97. `Package::new(output_dir, config, assets, binary) -> Result<Self>`。
98. `Package::add_file(&mut self, path_in_pkg, bytes)`。
99. `Package::add_directory(&mut self, prefix, dir)`。
100. `Package::build(&self) -> Result<BuildArtifact>`。
101. `Package::format(&self) -> PackageFormat`。
102. `PackageFormat::Dir / Zip / Apk / Ipa / Wasm / MiniApp`。
103. `HotUpdate::diff(old_manifest, new_manifest) -> HotUpdatePatch`。
104. `HotUpdate::apply(current_dir, patch) -> Result<()>`。
105. `HotUpdatePatch::version / new_manifest / file_changes / size_bytes`。
106. `FileChange::Added(path, size, hash) / Modified(path, diff, size) / Removed(path)`。
107. `Signing`：Windows 签名 / Android 签名 / iOS 签名 / 小程序可不。
108. `SignInfo::signature / certificate / timestamp`。
109. `ProvisioningProfile`（iOS）。
110. `AndroidKeystore`：路径、别名、密码。
111. `BuildLogger`：构建日志，彩色输出 + 进度。
112. `BuildProgress`：阶段（1/10 初始化/编译/处理资源/打包/签名）。
113. `BuildError`：错误码定位到阶段与文件。
114. `BuildWarning`：可忽略提示。
115. `BuildReport`：耗时 / 产物大小 / 警告数 / 错误数。
116. `BuildReport::save_html(&self, path)` 生成可视化报表。
117. `build_cli` 子命令：`engine new` / `engine build` / `engine run` / `engine clean` / `engine package` / `engine hot-update`。
118. `engine build --target <target> --profile <profile> --config <toml>`。
119. `engine run --profile <profile>`。
120. `engine package --target <target> --output <dir>`。
121. `engine hot-update --from <v1> --to <v2> --output <patch>`。
122. `wasm-bindgen` 支持（Web 目标）。
123. `wasm-opt` 支持（可选）。
124. Web 构建产物：`index.html` + `*.js` + `*.wasm` + 资源。
125. Web 构建支持 service worker（离线 PWA）。
126. Android 构建产物：`app-debug.apk` / `app-release.apk`。
127. Android 构建支持 `aarch64-linux-android` + `armv7-linux-androideabi` + `x86_64-linux-android`。
128. Android 构建支持动态权限弹窗（运行时权限）。
129. iOS 构建产物：`.app` 或 `.ipa`（仅 macOS 支持）。
130. iOS 构建支持设备/模拟器双 target。
131. Windows 构建产物：`.exe` + 依赖 DLL + 资源目录。
132. Windows 构建支持 console / window subsystem。
133. Windows 构建支持 icon 嵌入。
134. macOS 构建产物：`.app`。
135. macOS 构建支持 Info.plist 与签名。
136. Linux 构建产物：可执行二进制 + 资源目录（或 `.AppImage`，可选）。
137. 微信/抖音/QQ 小程序构建：把 Web 产物适配为小程序 canvas + JS。
138. 小程序构建输出 `miniapp.zip` 或直接输出工程目录。
139. 资源清单 `assets.manifest`：包含所有资源路径、hash、大小、依赖。
140. 资源清单可在运行时读取，用于加载校验与差分更新。
141. 资源加密：开发期可选关闭，发布期默认开启。
142. 资源解密密钥：内嵌于二进制，可外部配置。
143. `BuildCache`：缓存编译产物加速增量构建。
144. `BuildCache::hash(key) -> String`。
145. `BuildCache::get(&self, key) -> Option<PathBuf>`。
146. `BuildCache::put(&mut self, key, path)`。
147. `BuildCache::clean(&mut self)`。
148. `examples/build_cli`：CLI 示例。
149. `examples/wasm_demo`：Web 版 hello world。
150. `examples/miniapp_demo`：微信小游戏示例（需开发者工具导入）。
151. `examples/android_demo`：Android 示例（需 Android Studio / adb）。
152. 单测：`BuildConfig` 保存/加载往返。
153. 单测：`AssetPipeline` 扫描/导入（用临时目录）。
154. 单测：`AssetManifest` JSON 往返。
155. 单测：`HotUpdate::diff` 正确识别新增/修改/删除。
156. 单测：`BuildCache` hash。
157. 单测：`BuildPipeline` 在当前平台可构建（Linux）。
158. 集成测试：CLI `engine build --target linux --profile debug` 成功。
159. `cargo test -p engine-build` 全部通过。
160. `cargo clippy --workspace -- -D warnings` 通过。
161. `cargo fmt --check --workspace` 通过。
162. `cargo doc --workspace --no-deps` 成功。
163. CI：新增 `cargo build --target wasm32-unknown-unknown` 测试。
164. CI：新增 `cargo build --target x86_64-pc-windows-gnu` 测试（交叉编译）。
165. CI：三平台 green。
166. CHANGELOG 记录版本 0.8.0（阶段二完成）。
167. README.md 加入「跨平台打包」章节。
168. README.md 加入「资源管线」章节。
169. README.md 加入「CLI 使用指南」章节。
170. 公开 API doc comment 覆盖率 100%。
171. 本 Sprint `unsafe` 块 <= 5（主要调用外部工具）。
172. 新增 example 工程 >= 4 个。

> 以上 172 条需求构成 Sprint 08 全量清单。

---

## 三、细分需求与验收

### 3.1 BuildPipeline / BuildConfig

173. `BuildPipeline::new(config)`。
174. `BuildPipeline::config(&self) -> &BuildConfig`。
175. `BuildPipeline::build(&self) -> Result<BuildArtifact>`。
176. `BuildPipeline::build_async(&self, sender: Progress)`。
177. `BuildPipeline::clean(&self)`。
178. `BuildPipeline::run(&self)`。
179. `BuildPipeline::platform_target(&self) -> PlatformTarget`。
180. `BuildPipeline::profile(&self) -> Profile`。
181. `BuildConfig::default()`。
182. `BuildConfig::app_name(&self) -> &str`。
183. `BuildConfig::app_id(&self) -> &str`。
184. `BuildConfig::version(&self) -> &str`。
185. `BuildConfig::version_code(&self) -> i32`。
186. `BuildConfig::icons(&self) -> &[PathBuf]`。
187. `BuildConfig::splash(&self) -> Option<&PathBuf>`。
188. `BuildConfig::permissions(&self) -> &[Permission]`。
189. `BuildConfig::orientation(&self) -> Orientation`。
190. `BuildConfig::output_dir(&self) -> &Path`。
191. `BuildConfig::temp_dir(&self) -> &Path`。
192. `BuildConfig::assets_dir(&self) -> &Path`。
193. `BuildConfig::from_toml(path) -> Result<Self>`。
194. `BuildConfig::to_toml(&self) -> String`。
195. `BuildConfig::save(&self, path) -> Result<()>`。
196. `Profile::Debug / Release / Ship`。
197. `Profile::opt_level(&self) -> String`。
198. `Profile::debug(&self) -> bool`。
199. `Profile::strip(&self) -> bool`。
200. `Profile::lto(&self) -> bool`。
201. `Profile::cargo_args(&self) -> Vec<String>`。

### 3.2 平台工具链

202. `WindowsToolchain::detect() -> Result<Self>`。
203. `WindowsToolchain::build(&self, config) -> Result<PathBuf>`。
204. `WindowsToolchain::embed_icon(exe, icon) -> Result<()>`（可选）。
205. `WindowsToolchain::sign(exe, cert) -> Result<()>`（可选）。
206. `MacOSToolchain::detect() -> Result<Self>`。
207. `MacOSToolchain::build(&self, config) -> Result<PathBuf>`。
208. `MacOSToolchain::codesign(app, identity) -> Result<()>`。
209. `LinuxToolchain::detect() -> Result<Self>`。
210. `LinuxToolchain::build(&self, config) -> Result<PathBuf>`。
211. `LinuxToolchain::appimage(src_dir, out) -> Result<PathBuf>`（可选）。
212. `AndroidToolchain::detect() -> Result<Self>`。
213. `AndroidToolchain::build(&self, config) -> Result<PathBuf>`。
214. `AndroidToolchain::abi(&self) -> Vec<String>`。
215. `AndroidToolchain::min_sdk(&self) -> u32`。
216. `AndroidToolchain::sign_apk(&self, apk, keystore) -> Result<PathBuf>`。
217. `AndroidToolchain::zipalign(&self, apk) -> Result<PathBuf>`。
218. `AndroidManifest::new(config) -> Self`。
219. `AndroidManifest::to_xml(&self) -> String`。
220. `AndroidManifest::activity(&self) -> &str`。
221. `AndroidManifest::intent_filters(&self) -> Vec<String>`。
222. `IOSToolchain::detect() -> Result<Self>`（仅 macOS）。
223. `IOSToolchain::build(&self, config) -> Result<PathBuf>`。
224. `IOSToolchain::codesign(&self, app, profile) -> Result<()>`。
225. `InfoPlist::new(config) -> Self`。
226. `InfoPlist::to_string(&self) -> String`。
227. `WebToolchain::detect() -> Result<Self>`。
228. `WebToolchain::build_wasm(&self, config) -> Result<PathBuf>`。
229. `WebToolchain::wasm_bindgen(&self, src) -> Result<Vec<PathBuf>>`。
230. `WebToolchain::wasm_opt(&self, src) -> Result<PathBuf>`。
231. `WebToolchain::generate_html(&self) -> Result<PathBuf>`。
232. `WebToolchain::generate_sw(&self) -> Result<PathBuf>`（PWA）。
233. `WebToolchain::generate_manifest(&self) -> Result<PathBuf>`（PWA）。
234. `MiniAppToolchain::build(&self, config, wasm_or_js) -> Result<PathBuf>`。
235. `MiniAppToolchain::minify_js(&self, src) -> Result<String>`。
236. `MiniAppToolchain::generate_app_json(&self) -> String`。
237. `MiniAppToolchain::generate_project_config(&self) -> String`。
238. `MiniAppToolchain::generate_game_js(&self) -> String`。
239. `MiniAppToolchain::miniapp_platform(&self) -> MiniAppPlatform`。
240. `MiniAppPlatform::WeChat / ByteDance / QQ`。

### 3.3 资源管线

241. `AssetPipeline::new(asset_dir)`。
242. `AssetPipeline::scan(&mut self)`。
243. `AssetPipeline::import(&mut self)`。
244. `AssetPipeline::process(&mut self)`。
245. `AssetPipeline::package(&mut self, out) -> Result<PathBuf>`。
246. `AssetPipeline::build_manifest(&self) -> AssetManifest`。
247. `AssetPipeline::changed_files(&self, since: SystemTime) -> Vec<PathBuf>`。
248. `AssetPipeline::incremental_build(&mut self, since) -> Result<AssetManifest>`。
249. `AssetManifest::new() -> Self`。
250. `AssetManifest::add(&mut self, entry)`。
251. `AssetManifest::entries(&self) -> &[AssetEntry]`。
252. `AssetManifest::to_json(&self) -> String`。
253. `AssetManifest::from_json(json) -> Result<Self>`。
254. `AssetManifest::save(&self, path) -> Result<()>`。
255. `AssetManifest::load(path) -> Result<Self>`。
256. `AssetManifest::diff(&self, other) -> DiffResult`。
257. `AssetEntry::path(&self) -> &Path`。
258. `AssetEntry::hash(&self) -> &str`。
259. `AssetEntry::size(&self) -> u64`。
260. `AssetEntry::kind(&self) -> AssetKind`。
261. `AssetKind::Texture / Audio / Model / Scene / Prefab / Font / Custom(&str)`。
262. `DiffResult::added / modified / removed`。
263. `TextureProcessor::compress_bc(img) -> Result<Vec<u8>>`。
264. `TextureProcessor::compress_etc(img) -> Result<Vec<u8>>`。
265. `TextureProcessor::compress_astc(img) -> Result<Vec<u8>>`（可选）。
266. `TextureProcessor::generate_mipmaps(img) -> Result<Vec<u8>>`。
267. `TextureProcessor::resize(img, w, h) -> Result<Image>`。
268. `TextureProcessor::pack_atlas(images, padding) -> Result<(Image, Vec<Rect>)>`。
269. `AudioProcessor::to_ogg(src) -> Result<Vec<u8>>`。
270. `AudioProcessor::to_mp3(src) -> Result<Vec<u8>>`。
271. `AudioProcessor::to_wav(src) -> Result<Vec<u8>>`。
272. `AudioProcessor::to_flac(src) -> Result<Vec<u8>>`。
273. `ModelProcessor::glb_to_optimized(src) -> Result<Vec<u8>>`。
274. `ModelProcessor::gltf_to_optimized(src_dir) -> Result<Vec<u8>>`。
275. `SceneProcessor::bake(scene) -> Result<Vec<u8>>`。
276. `Compress::zstd(bytes, level) -> Result<Vec<u8>>`。
277. `Compress::gzip(bytes, level) -> Result<Vec<u8>>`。
278. `Compress::brotli(bytes, level) -> Result<Vec<u8>>`。
279. `Compress::lz4(bytes) -> Result<Vec<u8>>`。
280. `Compress::decompress(bytes, algo) -> Result<Vec<u8>>`。
281. `Encrypt::aes_gcm_128(bytes, key, nonce) -> Result<Vec<u8>>`。
282. `Encrypt::aes_gcm_256(bytes, key, nonce) -> Result<Vec<u8>>`。
283. `Encrypt::chacha20(bytes, key, nonce) -> Result<Vec<u8>>`。
284. `Encrypt::decrypt(bytes, key, nonce, algo) -> Result<Vec<u8>>`。
285. `Hash::sha256(bytes) -> String`。
286. `Hash::hash_file(path) -> Result<String>`。
287. `Hash::hash_dir(path) -> Result<String>`。

### 3.4 打包与热更新

288. `Package::new(output_dir) -> Self`。
289. `Package::add_file(&mut self, pkg_path, bytes)`。
290. `Package::add_directory(&mut self, prefix, dir)`。
291. `Package::add_manifest(&mut self, manifest)`。
292. `Package::build(&self, format) -> Result<BuildArtifact>`。
293. `Package::build_zip(&self, out) -> Result<BuildArtifact>`。
294. `Package::build_dir(&self, out) -> Result<BuildArtifact>`。
295. `BuildArtifact::path(&self) -> &Path`。
296. `BuildArtifact::size(&self) -> u64`。
297. `BuildArtifact::platform(&self) -> PlatformTarget`。
298. `BuildArtifact::version(&self) -> &str`。
299. `BuildArtifact::sign(&self) -> Option<&SignInfo>`。
300. `HotUpdate::diff(old, new) -> HotUpdatePatch`。
301. `HotUpdate::apply(dir, patch) -> Result<()>`。
302. `HotUpdatePatch::version(&self) -> &str`。
303. `HotUpdatePatch::new_manifest(&self) -> &AssetManifest`。
304. `HotUpdatePatch::changes(&self) -> &[FileChange]`。
305. `HotUpdatePatch::size_bytes(&self) -> u64`。
306. `HotUpdatePatch::to_bytes(&self) -> Result<Vec<u8>>`。
307. `HotUpdatePatch::from_bytes(bytes) -> Result<Self>`。
308. `FileChange::Added / Modified / Removed`。
309. `FileChange::path(&self) -> &Path`。
310. `FileChange::size(&self) -> u64`。

### 3.5 签名与权限

311. `AndroidKeystore::path / alias / password`。
312. `Signing::android_sign(unsigned_apk, keystore) -> Result<PathBuf>`。
313. `Signing::android_verify(apk) -> bool`。
314. `WindowsCodeSign::sign(exe, cert) -> Result<()>`。
315. `AppleSign::codesign(app) -> Result<()>`。
316. `Permission::Internet / Storage / Camera / Microphone / Location / Bluetooth / NFC`。
317. `Permission::to_android_string(&self) -> &str`。
318. `Permission::to_ios_string(&self) -> &str`。
319. `ProvisioningProfile::name / team_id / app_id / entitlements`。

### 3.6 日志与报告

320. `BuildLogger::new(verbose)`。
321. `BuildLogger::info(msg)`。
322. `BuildLogger::warn(msg)`。
323. `BuildLogger::error(msg)`。
324. `BuildLogger::progress(percent, msg)`。
325. `BuildStage::Init / Compile / ProcessAssets / Package / Sign / Done`。
326. `BuildReport::new() -> Self`。
327. `BuildReport::add_stage(name, duration, size)`。
328. `BuildReport::to_html(&self) -> String`。
329. `BuildReport::save_html(&self, path) -> Result<()>`。
330. `BuildReport::total_duration(&self) -> Duration`。
331. `BuildReport::total_size(&self) -> u64`。
332. `BuildReport::warnings(&self) -> u32`。
333. `BuildReport::errors(&self) -> u32`。

### 3.7 CLI 子命令

334. `engine build --target <target> --profile <profile> --config <toml>`。
335. `engine run --profile <profile>`。
336. `engine clean`。
337. `engine package --target <target> --output <dir>`。
338. `engine hot-update --from <v1_manifest> --to <v2_manifest> --output <patch>`。
339. `engine new <name>`：创建工程模板。
340. `engine doctor`：检测构建环境。
341. `engine info`：打印引擎版本与配置。
342. `engine --help` / `-h`。
343. `engine --version`。

### 3.8 WebAssembly 支持

344. 依赖 `wasm-bindgen-cli`。
345. 依赖 `wasm-opt`（可选，release 生效）。
346. 生成 `index.html` 引用 JS/WASM。
347. 资源通过 fetch 加载（相对路径）。
348. 生成 `manifest.webmanifest`（PWA）。
349. 生成 `sw.js`（PWA 离线缓存）。
350. 生成 `assets.manifest`。
351. WASM 产物大小 < 5MB（release）。
352. WASM 启动时间 < 1s（网络良好）。
353. `examples/wasm_demo` 正常运行。
354. `examples/wasm_demo` 中 WebGL 能绘制精灵。

### 3.9 示例与测试

355. `examples/build_cli` 打印帮助。
356. `examples/build_cli` 在本机构建 debug。
357. `examples/wasm_demo` 生成 WASM 产物。
358. `examples/miniapp_demo` 生成小程序工程。
359. `examples/android_demo` 生成 APK。
360. 单测：`BuildConfig` TOML 往返。
361. 单测：`AssetPipeline` 扫描导入。
362. 单测：`AssetManifest` JSON 往返。
363. 单测：`HotUpdate::diff`。
364. 单测：`BuildCache` 缓存命中。
365. 集成测试：`engine build --target linux --profile debug`。
366. `cargo test -p engine-build` 全部通过。
367. `cargo clippy --workspace -- -D warnings` 通过。
368. `cargo fmt --check --workspace` 通过。
369. `cargo doc --workspace --no-deps` 成功。
370. CI 三平台 green。
371. CHANGELOG 记录 0.8.0。
372. README.md 加入「跨平台打包」章节。
373. README.md 加入「资源管线」章节。
374. README.md 加入「CLI 使用指南」章节。
375. README.md 加入「构建产物部署」章节。
376. 公开 API doc comment 覆盖率 100%。
377. `unsafe` 块 <= 5。
378. 新增 example 工程 >= 4 个。

---

## 四、验收标准

- [ ] `cargo run --example build_cli -- build --target linux --profile debug` 成功
- [ ] `cargo run --example build_cli -- build --target web --profile release` 成功生成 WASM + HTML
- [ ] `cargo run --example wasm_demo` 可在浏览器打开运行
- [ ] Android 构建：在 Linux CI 上能生成 unsigned APK（可选，需 NDK）
- [ ] `cargo test -p engine-build` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] `cargo doc --workspace --no-deps` 成功
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.8.0

---

## 五、阶段二完成总结

**阶段二（编辑器 + 跨平台）至此完成。**  
已完成：UI 控件库 / 可视化编辑器（≥5 面板 / 七核心区域）/ 跨平台打包 / 资源管线 / WASM 支持。  
下一步进入 **阶段三（Sprint 09–12）**：3D 渲染管线 / 材质系统 / 动画 / 物理。
