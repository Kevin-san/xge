# Sprint 15 · 网络 / 热更新 / 插件系统

> 阶段：阶段四 · 高阶能力与生态（第 3 个 Sprint）
> 周期：4 周
> 核心目标：TCP/UDP/WebSocket/RakNet 网络通道 + 差分热更 + 插件沙盒 API
> 验收：`examples/network_chat` 与 `examples/hotfix_patch` 与 `examples/plugin_hello` 可运行

---

## 一、Sprint 概览

本 Sprint 建立三个 crate：`engine-network`、`engine-hotfix`、`engine-plugin`。核心交付：

- `engine-network`：
  - `NetChannel` trait：统一抽象 Send/Recv
  - `TcpChannel` / `UdpChannel` / `WsChannel` / `QuicChannel` / `RaknetChannel` 通道实现
  - `NetworkMessage` trait + `#[derive(NetworkMessage)]` 派生宏
  - `NetSerializer`：MessagePack / JSON / Protobuf / Bincode / Cap'n Proto 统一接口
  - `RpcSystem` + `#[rpc]` 过程宏生成 client/server 桩
  - `Replication` / `NetworkSync`：entity 同步（插值 / 预测 / 延迟补偿）
  - `NetRole`：Server / Client / ListenServer / Standalone
  - `NetAttr`：Replicate / OnlyServer / OnlyOwner / OwnerAuto / Predict / Interpolate
  - `Lobby` / `Matchmaker` / `Room` 大厅与匹配系统
  - `NatTraversal`：STUN / TURN / ICE 穿透
  - `BandwidthController` / `NetCompressor` / `NetEncryptor`（AES-GCM / ChaCha20-Poly1305）
  - `NetDebugPanel`：实时流量 / 延迟 / 丢包 / RPC 计数 / 同步对象数
  - `ReplayRecorder` / `ReplayPlayer`：回放记录与播放
  - `AntiCheat` 基础：CRC 校验 / 关键输入签名 / 权威服务器
- `engine-hotfix`：
  - `HotfixManager`：统一热更入口
  - `DiffEngine`：bsdiff / bspatch / zstd / courgette-like 差分
  - `PatchBundle`：patch 文件格式与签名校验（RSA / Ed25519）
  - `UpdateStrategy`：整包更新 / 增量更新
  - `ScriptHotreload`：JS / TS / Py / Lua
  - `DylibHotload`：Rust dylib 热加载（desktop 平台）
  - `WasmHotswap`：WASM 模块热替换
  - `AssetHotreload`：纹理 / 模型 / 音频 / 场景热重载
  - `GreyRelease`：渠道 / 版本 / 设备 / 地区白名单
  - `Downloader`：CDN / HTTP(S) / FTP 多源 + 断点续传 + 多线程下载
  - `UpdateUI`：进度条 / 变更日志 / 强制更新 / 可选更新
  - `VersionCompat`：semver / 兼容性检查
- `engine-plugin`：
  - `Plugin` trait：`on_load` / `on_unload` / `on_tick` / `register_types`
  - `PluginKind`：Rust dylib / WASM / 脚本 / C ABI FFI
  - `PluginManifest`：`manifest.toml` 解析（名称 / 版本 / 依赖 / 权限 / 入口）
  - `PluginSandbox`：权限声明（文件 / 网络 / 内存 / CPU 时间）
  - `PluginRegistry`：组件 / 系统 / 资源 / 窗口 / UI Widget / 渲染 Pass 注册
  - `PluginResolver`：依赖解析（DAG）+ 版本冲突检测
  - `PluginLifecycle`：热卸载与热升级
  - `PluginQuota`：沙盒资源计费（内存 / CPU / 句柄）
  - `PluginDebug`：日志 / hook / 崩溃恢复
  - `PluginStoreClient`：连接 Sprint 16 Asset Store
- examples：
  - `examples/network_chat`
  - `examples/network_echo`
  - `examples/network_rpc`
  - `examples/network_replication`
  - `examples/network_lobby`
  - `examples/network_replay`
  - `examples/hotfix_patch`
  - `examples/hotfix_script`
  - `examples/hotfix_asset`
  - `examples/hotfix_grey`
  - `examples/plugin_hello`
  - `examples/plugin_ui_widget`
  - `examples/plugin_render_pass`
  - `examples/plugin_ffi`

---

## 二、项目需求清单

1. `engine-network` crate 建立。
2. `engine-hotfix` crate 建立。
3. `engine-plugin` crate 建立。
4. `NetChannel` trait：`send(&self, msg) -> Result<()>`。
5. `NetChannel` trait：`recv(&self) -> Result<Option<NetMessage>>`。
6. `NetChannel` trait：`is_connected(&self) -> bool`。
7. `NetChannel` trait：`peer_addr(&self) -> Option<SocketAddr>`。
8. `NetChannel` trait：`close(&mut self) -> Result<()>`。
9. `TcpChannel` 客户端实现。
10. `TcpChannel` 服务端实现（accept）。
11. `UdpChannel` 实现（支持可靠与不可靠标记）。
12. `UdpChannel` 可靠模式：滑动窗口 / ACK / 重传。
13. `WsChannel` 客户端（基于 tokio-tungstenite）。
14. `WsChannel` 服务端（基于 tokio-tungstenite）。
15. `WsChannel` text / binary frame 区分。
16. `QuicChannel`（基于 quinn）。
17. `RaknetChannel`：RakNet 兼容协议层（或自研可靠 UDP）。
18. `NetMessage` trait：`type_id() -> u32`。
19. `NetMessage` trait：`serialize(&self) -> Vec<u8>`。
20. `NetMessage` trait：`deserialize(buf) -> Result<Self>`。
21. `#[derive(NetworkMessage)]` 过程宏。
22. `NetSerializer` trait 抽象。
23. MessagePack 序列化实现。
24. JSON 序列化实现。
25. Protobuf 序列化实现（基于 prost）。
26. Bincode 序列化实现。
27. Cap'n Proto 序列化实现。
28. `RpcService` trait：`#[rpc]` 宏生成 client stub。
29. `#[rpc]` 宏生成 server dispatcher。
30. RPC 请求 / 响应 ID 匹配。
31. RPC 超时 / 重试机制。
32. RPC 异步 `Future` 支持。
33. `Replication` 组件：标记 entity 需要网络同步。
34. `NetworkSync`：位置 / 旋转 / 速度同步。
35. 客户端插值（interpolation）。
36. 客户端预测（prediction）。
37. 服务端延迟补偿（lag compensation）。
38. `NetRole` 枚举：`Server / Client / ListenServer / Standalone`。
39. `NetAttr::Replicate` 全量同步。
40. `NetAttr::OnlyServer` 仅服务器同步。
41. `NetAttr::OnlyOwner` 仅 Owner 同步。
42. `NetAttr::OwnerAuto` 自动推断 Owner。
43. `NetAttr::Predict` 启用客户端预测。
44. `NetAttr::Interpolate` 启用客户端插值。
45. `OnSpawn` 网络事件：实体在远端生成。
46. `OnDespawn` 网络事件：实体在远端销毁。
47. `OnSync` 网络事件：每次同步数据到达。
48. `OnRPC` 网络事件：RPC 调用。
49. `OnConnect` 事件：玩家连接。
50. `OnDisconnect` 事件：玩家断开。
51. `Lobby` 大厅系统：创建 / 加入 / 列表。
52. `Room` 房间系统：最大人数 / 密码 / 私有标记。
53. `Matchmaker` 匹配：按段位 / 延迟 / 队伍匹配。
54. `NatTraversal::stun(server)` 公网 IP 发现。
55. `NatTraversal::turn` TURN 中继。
56. `NatTraversal::ice` ICE 候选收集与连通性检查。
57. `BandwidthController`：按连接限速。
58. `NetCompressor`：zstd / snappy / lz4 可选压缩。
59. `NetEncryptor`：AES-GCM 加密。
60. `NetEncryptor`：ChaCha20-Poly1305 加密。
61. `NetDebugPanel`：实时上行 / 下行带宽显示。
62. `NetDebugPanel`：RTT（往返时间）显示。
63. `NetDebugPanel`：丢包率显示。
64. `NetDebugPanel`：RPC 调用计数。
65. `NetDebugPanel`：同步对象总数。
66. `ReplayRecorder`：记录全部网络消息到文件。
67. `ReplayPlayer`：从文件回放并驱动世界。
68. `Replay` 文件格式：头部 + 帧 + 消息流。
69. `AntiCheat::crc_check()`：关键状态 CRC 校验。
70. `AntiCheat::sign_input()`：关键输入签名。
71. 权威服务器（authoritative server）：物理与关键状态仅在服务端计算。
72. `HotfixManager`：统一热更入口 `check_for_update()`。
73. `HotfixManager::apply_patch()`：应用 patch。
74. `HotfixManager::progress()`：返回进度 `(cur, total)`。
75. `DiffEngine::bsdiff(old, new) -> patch`。
76. `DiffEngine::bspatch(old, patch) -> new`。
77. `DiffEngine::zstd_compress(data) -> Vec<u8>`。
78. `DiffEngine::chunk_diff`：基于内容分片的差分（类似 courgette）。
79. `PatchBundle`：patch 文件封装格式（头部 + meta + 签名 + 数据）。
80. `PatchBundle::sign(keypair)`：RSA 签名。
81. `PatchBundle::sign_ed25519(keypair)`：Ed25519 签名。
82. `PatchBundle::verify(pubkey) -> bool`。
83. `UpdateStrategy::Full`：整包替换。
84. `UpdateStrategy::Incremental`：增量差分。
85. `UpdateStrategy::Hybrid`：混合策略。
86. `ScriptHotreload`：支持 JS / TS。
87. `ScriptHotreload`：支持 Python。
88. `ScriptHotreload`：支持 Lua。
89. `ScriptHotreload`：文件 watcher 自动 reload。
90. `DylibHotload`：加载 `*.dylib / *.so / *.dll`。
91. `DylibHotload`：符号解析 `plugin_init / plugin_update / plugin_shutdown`。
92. `DylibHotload`：热卸载 + 重新加载。
93. `WasmHotswap`：加载 WASM module。
94. `WasmHotswap`：swap 新 module 保留 host 状态。
95. `AssetHotreload`：纹理热重载。
96. `AssetHotreload`：模型热重载。
97. `AssetHotreload`：音频热重载。
98. `AssetHotreload`：场景（scene JSON）热重载。
99. `GreyRelease`：渠道白名单过滤。
100. `GreyRelease`：版本区间过滤。
101. `GreyRelease`：设备型号过滤。
102. `GreyRelease`：地区过滤（IP / GPS）。
103. `GreyRelease`：按比例灰度（AB 测试）。
104. `Downloader`：CDN / HTTP(S) / FTP 多源配置。
105. `Downloader`：断点续传（ETag / Range）。
106. `Downloader`：多线程分片下载。
107. `Downloader`：校验 hash（SHA-256）。
108. `UpdateUI`：进度条组件。
109. `UpdateUI`：变更日志展示。
110. `UpdateUI`：强制更新对话框。
111. `UpdateUI`：可选更新对话框。
112. `VersionCompat::check(current, target) -> CompatResult`。
113. `VersionCompat` semver 解析：`1.2.3` / `^1.2` / `~1.2.3`。
114. `Plugin` trait：`fn on_load(&mut self, world)`。
115. `Plugin` trait：`fn on_unload(&mut self, world)`。
116. `Plugin` trait：`fn on_tick(&mut self, world, dt)`。
117. `Plugin` trait：`fn register_types(&mut self, registry)`。
118. `PluginKind::RustDylib`。
119. `PluginKind::Wasm`。
120. `PluginKind::Script(js/py/lua)`。
121. `PluginKind::CAbi`。
122. `PluginManifest`：从 `manifest.toml` 解析。
123. `PluginManifest::name`。
124. `PluginManifest::version`。
125. `PluginManifest::dependencies`。
126. `PluginManifest::permissions`。
127. `PluginManifest::entry_point`。
128. `PluginSandbox`：文件权限（read/write/path 白名单）。
129. `PluginSandbox`：网络权限（host/port 白名单）。
130. `PluginSandbox`：内存配额上限。
131. `PluginSandbox`：CPU 时间配额。
132. `PluginRegistry::register_component::<T>()`。
133. `PluginRegistry::register_system(system)`。
134. `PluginRegistry::register_resource::<T>()`。
135. `PluginRegistry::register_window_builder(builder)`。
136. `PluginRegistry::register_ui_widget(widget)`。
137. `PluginRegistry::register_render_pass(pass)`。
138. `PluginResolver`：DAG 依赖拓扑排序。
139. `PluginResolver`：循环依赖检测。
140. `PluginResolver`：版本冲突检测（semver）。
141. `PluginLifecycle::load(path) -> PluginHandle`。
142. `PluginLifecycle::unload(handle)`。
143. `PluginLifecycle::upgrade(handle, new_version)`。
144. `PluginQuota::memory_usage(&self) -> usize`。
145. `PluginQuota::cpu_time(&self) -> Duration`。
146. `PluginQuota::handle_count(&self) -> usize`。
147. `PluginQuota::over_quota(&self) -> bool`。
148. `PluginDebug::logger`：独立命名空间日志。
149. `PluginDebug::hook(fn)`：函数 hook 调试。
150. `PluginDebug::crash_recovery`：捕获 panic 并卸载。
151. `PluginStoreClient`：连接 Sprint 16 的 Asset Store。
152. `PluginStoreClient::list_plugins()`。
153. `PluginStoreClient::download(name)`。
154. `PluginStoreClient::install(path)`。
155. `examples/network_chat`：TCP 文本聊天室。
156. `examples/network_echo`：UDP echo 服务器。
157. `examples/network_rpc`：RPC 调用示例。
158. `examples/network_replication`：entity 位置同步。
159. `examples/network_lobby`：大厅 + 房间匹配。
160. `examples/network_replay`：回放记录的对战。
161. `examples/hotfix_patch`：生成并应用差分 patch。
162. `examples/hotfix_script`：脚本热重载。
163. `examples/hotfix_asset`：资源热重载。
164. `examples/hotfix_grey`：灰度发布示例。
165. `examples/plugin_hello`：最小插件示例。
166. `examples/plugin_ui_widget`：自定义 UI Widget 插件。
167. `examples/plugin_render_pass`：自定义渲染 Pass 插件。
168. `examples/plugin_ffi`：C ABI FFI 插件示例。
169. 单测：`TcpChannel` 回环消息。
170. 单测：`UdpChannel` 可靠模式 ACK。
171. 单测：`WsChannel` 文本帧。
172. 单测：`#[derive(NetworkMessage)]` 序列化往返。
173. 单测：MessagePack round-trip。
174. 单测：Bincode round-trip。
175. 单测：RPC 请求-响应匹配。
176. 单测：Replication 属性过滤。
177. 单测：Lobby 创建 / 加入。
178. 单测：bsdiff 小文件 patch。
179. 单测：zstd 压缩 / 解压。
180. 单测：RSA 签名 / 验证。
181. 单测：Ed25519 签名 / 验证。
182. 单测：semver 解析。
183. 单测：PluginManifest 解析。
184. 单测：PluginSandbox 权限拒绝。
185. 单测：PluginResolver 循环依赖。
186. 单测：PluginResolver 版本冲突。
187. `cargo test -p engine-network` 全部通过。
188. `cargo test -p engine-hotfix` 全部通过。
189. `cargo test -p engine-plugin` 全部通过。
190. `cargo clippy --workspace -- -D warnings` 通过。
191. `cargo fmt --check --workspace` 通过。
192. `cargo doc --workspace --no-deps` 成功。
193. 网络基准：1000 并发连接稳定。
194. 网络基准：平均延迟 < 50ms。
195. 网络基准：丢包率 < 1%。
196. 热更基准：1GB 资源差分 < 30s。
197. 热更基准：patch 文件大小 < 原始 10%。
198. 插件安全：沙盒逃逸负例测试通过。
199. 插件安全：未授权网络访问被拒绝。
200. 插件安全：超限内存使用被 kill。
201. CI 三平台 green。
202. CHANGELOG 记录版本 0.15.0。
203. README.md 加入「网络系统」章节。
204. README.md 加入「热更新系统」章节。
205. README.md 加入「插件系统与生态」章节。
206. 公开 API doc comment 覆盖率 100%。
207. 本 Sprint `unsafe` 块 <= 5。
208. 新增 example 工程 >= 14 个。
209. `examples/network_chat` 可多人聊天。
210. `examples/hotfix_patch` 可生成和应用 patch。
211. `examples/plugin_hello` 可加载并 tick 插件。
212. `NetChannel::set_compression(bool)` 接口。
213. `NetChannel::set_encryption(bool)` 接口。
214. `NetChannel::stats() -> NetStats`（bytes in/out / msg count）。
215. `RpcRequestId` 类型 u64。
216. `RpcTimeout`（默认 5 秒，可配置）。
217. `ReplayFrame::tick_id` + `payload`。
218. `AntiCheat::state_hash(world) -> u64`。
219. `GreyRelease::match(user_profile) -> bool`。
220. `Downloader::register_mirror(url)`。
221. `UpdateUI::on_confirm(fn)` 回调。
222. `PluginHandle = u64`。
223. `PluginPermission::FileRead(path_glob)`。
224. `PluginPermission::FileWrite(path_glob)`。
225. `PluginPermission::Net(host, port)`。
226. `PluginPermission::Memory(bytes)`。
227. `PluginPermission::CpuTime(seconds)`。
228. `PluginSandbox::check(&self, perm) -> bool`。
229. `PluginStoreClient::search(keyword)`。
230. `PluginStoreClient::uninstall(name)`。
231. `examples/network_chat` 命令行输入发送消息。
232. `examples/network_chat` 显示在线用户列表。
233. `examples/hotfix_patch` CLI 参数：`--old / --new / --patch`。
234. `examples/plugin_hello` 打印 plugin tick 日志。
235. `NetRole::current() -> NetRole` 全局访问。
236. `NetAttr::from_str(s) -> Result<NetAttr>`。
237. `ReplayRecorder::flush()` 写入磁盘。
238. `ReplayPlayer::seek_to(tick)` 跳帧。
239. `HotfixManager::status() -> HotfixStatus`。
240. `HotfixStatus::Idle / Checking / Downloading / Applying / Ready / Error(String)`。

> 以上 240 条需求构成 Sprint 15 全量清单第一部分。

---

## 三、细分需求与验收

### 3.1 网络通道抽象（NetChannel）

241. `NetChannel` trait：`async fn send(&self, payload: &[u8]) -> Result<()>`。
242. `NetChannel` trait：`async fn recv(&mut self) -> Result<Option<Vec<u8>>>`。
243. `NetChannel` trait：`fn is_connected(&self) -> bool`。
244. `NetChannel` trait：`fn peer_addr(&self) -> Option<std::net::SocketAddr>`。
245. `NetChannel` trait：`fn local_addr(&self) -> Option<std::net::SocketAddr>`。
246. `NetChannel` trait：`fn close(&mut self) -> Result<()>`。
247. `NetChannel` trait：`fn stats(&self) -> NetStats`。
248. `NetStats`：`bytes_in / bytes_out / msg_in / msg_out / rtt_ms`。
249. `TcpChannel::connect(addr) -> Result<Self>`。
250. `TcpChannel::bind(addr) -> Result<TcpListener>`。
251. `TcpListener::accept() -> Result<TcpChannel>`。
252. `TcpListener::incoming() -> impl Stream<Item = Result<TcpChannel>>`。
253. `UdpChannel::bind(addr) -> Result<Self>`。
254. `UdpChannel::connect(addr) -> Result<Self>`。
255. `UdpChannel::send_to(addr, payload, reliability) -> Result<()>`。
256. `UdpChannel::recv_from() -> Result<Option<(SocketAddr, Vec<u8>)>>`。
257. `Reliability::Unreliable`。
258. `Reliability::UnreliableSequenced`。
259. `Reliability::Reliable`。
260. `Reliability::ReliableOrdered`。
261. `UdpChannel` 可靠模式：滑动窗口大小 32。
262. `UdpChannel` 可靠模式：ACK 位图。
263. `UdpChannel` 可靠模式：重传超时 200ms（可配置）。
264. `WsChannel::connect(url) -> Result<Self>`。
265. `WsChannel::bind(addr) -> Result<WsListener>`。
266. `WsChannel::send_text(s: &str) -> Result<()>`。
267. `WsChannel::send_binary(buf: &[u8]) -> Result<()>`。
268. `WsChannel::recv_text_or_binary() -> Result<WsMessage>`。
269. `WsMessage::Text(String)`。
270. `WsMessage::Binary(Vec<u8>)`。
271. `WsMessage::Close`。
272. `WsMessage::Ping(Vec<u8>)`。
273. `WsMessage::Pong(Vec<u8>)`。
274. `QuicChannel::connect(addr, server_name) -> Result<Self>`。
275. `QuicChannel::bind(addr, cert) -> Result<QuicListener>`。
276. `QuicChannel` 复用 QUIC stream。
277. `QuicChannel::open_bi_stream() -> Result<(QuicSend, QuicRecv)>`。
278. `RaknetChannel::connect(addr) -> Result<Self>`。
279. `RaknetChannel` 使用 RakNet 兼容握手协议（或自研等价）。
280. `RaknetChannel`：MTU 探测。
281. `RaknetChannel`：拥塞控制（类似 CUBIC 简化版）。
282. `ChannelBuilder`：配置通道（超时 / 加密 / 压缩）。
283. `ChannelBuilder::timeout(Duration)`。
284. `ChannelBuilder::encryption(Cipher)`。
285. `ChannelBuilder::compression(CompressionKind)`。
286. `Cipher::Aes256Gcm`。
287. `Cipher::ChaCha20Poly1305`。
288. `Cipher::None`。
289. `CompressionKind::Zstd(level)`。
290. `CompressionKind::Snappy`。
291. `CompressionKind::Lz4`。
292. `CompressionKind::None`。

### 3.2 序列化与 NetworkMessage

293. `NetMessage` trait：`fn type_id() -> u32`（crc32 of type_name）。
294. `NetMessage` trait：`fn serialize(&self, serializer: &mut NetSerializer) -> Result<()>`。
295. `NetMessage` trait：`fn deserialize(deserializer: &mut NetDeserializer) -> Result<Self>`。
296. `NetMessage` trait：`fn encode(&self, format: SerializeFormat) -> Result<Vec<u8>>`。
297. `NetMessage` trait：`fn decode(buf: &[u8], format: SerializeFormat) -> Result<Self>`。
298. `SerializeFormat::MessagePack`。
299. `SerializeFormat::Json`。
300. `SerializeFormat::Protobuf`。
301. `SerializeFormat::Bincode`。
302. `SerializeFormat::CapnProto`。
303. `#[derive(NetworkMessage)]` 为 struct 生成默认实现。
304. `#[derive(NetworkMessage)]` 为 enum 生成默认实现。
305. `#[net_message(type_id = 42)]` 手动指定 type_id。
306. `#[net_message(skip)]` 跳过字段。
307. `#[net_message(format = "Bincode")]` 指定默认格式。
308. `MessagePackSerializer::new() -> Self`。
309. `MessagePackSerializer::finish(&mut self) -> Vec<u8>`。
310. `JsonSerializer::to_string(&self) -> String`。
311. `ProtobufSerializer`（基于 prost）。
312. `BincodeSerializer::new() -> Self`。
313. `CapnpSerializer`（基于 capnp）。
314. 序列化性能基准：100KB < 1ms。
315. 反序列化性能基准：100KB < 1ms。
316. `NetMessage` 支持嵌套（递归 Message）。
317. `NetMessage` 支持 `Vec<T>` / `HashMap<K, V>` / `Option<T>`。
318. `NetMessage` 支持 `String` / `u8..u64` / `i8..i64` / `f32/f64` / `bool`。
319. `NetMessage` 支持 `Vec2/3/4` / `Quat` / `Mat4`。

### 3.3 Entity 同步（Replication / Prediction / Interpolation）

320. `Replication` 组件：空标记组件。
321. `NetworkSync` 组件：`last_authority_pos / last_authority_rot / last_authority_vel / tick`。
322. `NetworkOwner` 组件：`client_id`。
323. `NetworkId` 组件：`u64` 全局唯一。
324. `NetAttr::Replicate` 全量同步所有字段。
325. `NetAttr::OnlyServer` 字段只在服务端存在。
326. `NetAttr::OnlyOwner` 字段仅向 owner 客户端同步。
327. `NetAttr::OwnerAuto`：根据 `NetworkOwner` 推断。
328. `NetAttr::Predict` 字段启用客户端预测。
329. `NetAttr::Interpolate` 字段启用客户端插值。
330. 服务端 replication 系统：每固定 tick 生成 snapshot。
331. 服务端 delta 压缩：仅发送变化字段。
332. 客户端 interpolation：在两个权威 snapshot 之间线性 / 样条插值。
333. 客户端 prediction：根据 last input 预测位置。
334. 客户端 reconciliation：权威 snapshot 到达时校正预测。
335. 服务端 lag compensation：rewind 到客户端输入时间做碰撞检测。
336. `OnSpawn(client_id, entity, snapshot)` 事件。
337. `OnDespawn(client_id, entity)` 事件。
338. `OnSync(client_id, entity, snapshot)` 事件。
339. `OnRpc(from, to, rpc_id, payload)` 事件。
340. `OnConnect(client_id)` 事件。
341. `OnDisconnect(client_id, reason)` 事件。
342. `Snapshot::encode(&self) -> Vec<u8>`。
343. `Snapshot::decode(buf) -> Result<Self>`。
344. `Snapshot` 包含 tick 序号用于去重。
345. `Snapshot` 包含时间戳用于插值。
346. `Prediction::reset()` 清空预测 buffer。
347. `PredictionBuffer::push(tick, input, state)`。
348. `PredictionBuffer::replay_from(tick, authoritative_state)`。
349. `InterpolationBuffer::push(tick, state)`。
350. `InterpolationBuffer::sample(now) -> state`（线性插值）。

### 3.4 RPC 系统

351. `#[rpc]` 属性宏应用在 impl block 上。
352. `#[rpc(server)]` 生成服务端 dispatcher。
353. `#[rpc(client)]` 生成客户端 stub。
354. `#[rpc(bidirectional)]` 双向。
355. `#[rpc_method]` 标记单个 RPC 方法。
356. `#[rpc_method(one_way)]` 无需返回。
357. `#[rpc_method(timeout = "3s")]` 指定超时。
358. 生成的 client: `RpcClient::new(channel) -> Self`。
359. 生成的 client: `async fn method_name(&self, args...) -> Result<Ret>`。
360. 生成的 server: `RpcServer::dispatch(&mut self, msg) -> Result<Option<Vec<u8>>>`。
361. `RpcRequestId` 全局自增 u64。
362. `RpcRequest`：`{ id, method, args }`。
363. `RpcResponse`：`{ id, result }`。
364. `RpcError::Timeout`。
365. `RpcError::NotFound`。
366. `RpcError::Deserialize`。
367. `RpcError::Serialize`。
368. `RpcError::Application(String)`。
369. RPC 支持流式 `impl Stream<Item = T>`（SSE 风格）。
370. RPC 支持取消（基于 request_id cancel）。
371. RPC 支持批处理 `batch(Vec<Request>) -> Vec<Response>`。
372. RPC 中间件：`RpcMiddleware::before/after`。
373. `RpcMiddleware` 用于日志 / 鉴权 / 限流。
374. `RpcRateLimiter`：qps 限制。
375. `RpcAuth`：基于 token 的简单鉴权。

### 3.5 网络调试 / 回放 / 反作弊

376. `NetDebugPanel::new() -> Self`。
377. `NetDebugPanel::set_visible(bool)`。
378. `NetDebugPanel::draw(ui, net)`。
379. `NetDebugPanel` 显示：上行带宽 `KB/s`。
380. `NetDebugPanel` 显示：下行带宽 `KB/s`。
381. `NetDebugPanel` 显示：延迟 RTT（ms，滑动平均）。
382. `NetDebugPanel` 显示：丢包率 %。
383. `NetDebugPanel` 显示：RPC 计数（in / out）。
384. `NetDebugPanel` 显示：同步对象数（entities with `Replication`）。
385. `NetDebugPanel` 显示：churn 速率（spawn/despawn 每秒）。
386. `NetDebugPanel` 支持过滤器：按 peer 筛选。
387. `ReplayRecorder::new(path) -> Result<Self>`。
388. `ReplayRecorder::record(tick, msg)`。
389. `ReplayRecorder::flush() -> Result<()>`。
390. `ReplayRecorder::close() -> Result<()>`。
391. `ReplayFile` header：magic + version + meta JSON。
392. `ReplayFile` frame：tick_id + length + payload。
393. `ReplayPlayer::new(path) -> Result<Self>`。
394. `ReplayPlayer::next_frame() -> Result<Option<ReplayFrame>>`。
395. `ReplayPlayer::seek_to(tick) -> Result<()>`。
396. `ReplayPlayer::speed(factor)` 1x / 2x / 0.5x。
397. `ReplayPlayer::is_finished() -> bool`。
398. `AntiCheat::state_hash(world) -> u64`。
399. `AntiCheat::crc_compare(client_hash, server_hash) -> bool`。
400. `AntiCheat::sign_input(input, private_key) -> Signature`。
401. `AntiCheat::verify_input(input, signature, public_key) -> bool`。
402. 权威服务器：物理模拟仅在服务端。
403. 权威服务器：血量 / 金币等关键数值只由服务端写入。
404. 权威服务器：客户端输入仅作为 `InputEvent` 转发。
405. `Authority` 资源：在 server 角色存在。

### 3.6 大厅 / 匹配 / NAT 穿透

406. `Lobby::new(max_rooms: usize) -> Self`。
407. `Lobby::create_room(creator, config) -> RoomId`。
408. `Lobby::join_room(client, room_id) -> Result<()>`。
409. `Lobby::leave_room(client, room_id) -> Result<()>`。
410. `Lobby::list_rooms(&self) -> Vec<RoomInfo>`。
411. `Lobby::set_ready(client, room_id, ready) -> Result<()>`。
412. `Lobby::start_game(room_id) -> Result<GameHandle>`。
413. `RoomConfig::name: String`。
414. `RoomConfig::max_players: usize`。
415. `RoomConfig::password: Option<String>`。
416. `RoomConfig::is_private: bool`。
417. `RoomConfig::game_mode: String`。
418. `RoomInfo::id / name / current / max / is_private / ping_ms`。
419. `Matchmaker::new(lobby) -> Self`。
420. `Matchmaker::enqueue(client, profile) -> QueueId`。
421. `Matchmaker::dequeue(queue_id)`。
422. `Matchmaker::tick()` 执行匹配算法。
423. 匹配算法：按 `skill`（elo 分）区间贪心匹配。
424. 匹配算法：按 `ping` 偏好（<50ms 优先）。
425. 匹配算法：按 `team_balance` 均衡组队。
426. 匹配超时：60s 后放宽区间。
427. `NatTraversal::stun_request(stun_server) -> Result<SocketAddr>`。
428. `NatTraversal::turn_allocate(turn_server, cred) -> Result<RelayAddr>`。
429. `NatTraversal::ice_gather() -> Vec<IceCandidate>`。
430. `IceCandidate::Host`。
431. `IceCandidate::ServerReflexive`。
432. `IceCandidate::Relay`。
433. `NatTraversal::ice_connect(a_candidates, b_candidates) -> Result<SocketPair>`。
434. `SocketPair::local / remote`。
435. NAT 穿透失败时自动回退到 TURN 中继。

### 3.7 带宽控制 / 压缩 / 加密

436. `BandwidthController::new(up_limit_bps, down_limit_bps) -> Self`。
437. `BandwidthController::tick(&mut self, now)` 更新 token bucket。
438. `BandwidthController::try_send(bytes) -> bool` 是否允许发送。
439. `BandwidthController::per_connection_limit(conn_id, bps)`。
440. `NetCompressor::compress(&self, data) -> Result<Vec<u8>>`。
441. `NetCompressor::decompress(&self, data) -> Result<Vec<u8>>`。
442. zstd 压缩级别 1..21 可配置。
443. snappy 无配置。
444. lz4 无配置。
445. 压缩头 magic `0xC0 0x4D 0x50 0x52` + kind byte。
446. `NetEncryptor::aes_gcm_new(key, nonce_gen) -> Self`。
447. `NetEncryptor::chacha20_new(key) -> Self`。
448. `NetEncryptor::encrypt(&mut self, plain) -> Result<Vec<u8>>`。
449. `NetEncryptor::decrypt(&mut self, cipher) -> Result<Vec<u8>>`。
450. `NetEncryptor` 使用随机 nonce（96-bit）并附在密文前。
451. `NetEncryptor` 使用 ChaCha20-Poly1305 作为移动端首选。
452. `KeyAgreement`：ECDH（x25519）用于握手密钥协商。
453. `Handshake::client_hello -> server_hello -> key_exchange -> finished`。

### 3.8 热更新方案（HotfixManager / Diff / Patch）

454. `HotfixManager::new(work_dir) -> Self`。
455. `HotfixManager::current_version(&self) -> Version`。
456. `HotfixManager::check_for_update(manifest_url) -> Result<Option<UpdateInfo>>`。
457. `HotfixManager::download(&mut self, update) -> Result<()>`。
458. `HotfixManager::apply(&mut self) -> Result<()>`。
459. `HotfixManager::progress(&self) -> (u64, u64)`。
460. `HotfixManager::status(&self) -> HotfixStatus`。
461. `HotfixStatus::Idle`。
462. `HotfixStatus::Checking`。
463. `HotfixStatus::Downloading(percent)`。
464. `HotfixStatus::Applying(percent)`。
465. `HotfixStatus::Ready(restart_required)`。
466. `HotfixStatus::Error(String)`。
467. `DiffEngine::bsdiff(old_path, new_path, patch_path) -> Result<()>`。
468. `DiffEngine::bspatch(old_path, patch_path, new_path) -> Result<()>`。
469. `DiffEngine::zstd_compress(data) -> Result<Vec<u8>>`。
470. `DiffEngine::zstd_decompress(data) -> Result<Vec<u8>>`。
471. `DiffEngine::chunk_diff(old, new) -> Vec<ChunkDiff>`（基于内容哈希滚动匹配）。
472. `ChunkDiff::Same(offset, len)`。
473. `ChunkDiff::Diff(bytes)`。
474. `DiffEngine::chunk_apply(old, diff) -> Vec<u8>`。
475. `PatchBundle::new(version_from, version_to) -> Self`。
476. `PatchBundle::add_file(relative_path, diff_data)`。
477. `PatchBundle::sign_rsa(private_key_pem) -> Result<Signature>`。
478. `PatchBundle::sign_ed25519(private_key) -> Result<Signature>`。
479. `PatchBundle::verify_rsa(public_key_pem, sig) -> bool`。
480. `PatchBundle::verify_ed25519(public_key, sig) -> bool`。
481. `PatchBundle::to_bytes(&self) -> Result<Vec<u8>>`。
482. `PatchBundle::from_bytes(&[u8]) -> Result<Self>`。
483. `PatchBundle` 包含 SHA-256 merkle root 用于完整性。
484. `UpdateStrategy::Full`：下载完整替换包。
485. `UpdateStrategy::Incremental`：下载增量 patch。
486. `UpdateStrategy::Hybrid`：优先增量，失败回退整包。
487. `UpdateStrategy::choose(strategy_list, platform, bandwidth_est)`。

### 3.9 资源差分 / 脚本 / dylib / wasm / 资产热重载

488. `AssetDiff::diff_dir(old_dir, new_dir) -> Vec<FilePatch>`。
489. `AssetDiff::apply_dir(base_dir, patches) -> Result<()>`。
490. `FilePatch::Add(path, bytes)`。
491. `FilePatch::Modify(path, bsdiff_bytes)`。
492. `FilePatch::Remove(path)`。
493. `ScriptRuntime::new(lang) -> Self`。
494. `ScriptLang::Js`。
495. `ScriptLang::Ts`（先转 JS）。
496. `ScriptLang::Py`。
497. `ScriptLang::Lua`。
498. `ScriptRuntime::load(&mut self, path) -> Result<ScriptHandle>`。
499. `ScriptRuntime::reload(&mut self, handle) -> Result<()>`。
500. `ScriptRuntime::call(&mut self, handle, fn_name, args) -> Result<Value>`。
501. `ScriptFileWatcher::new(runtime, dir)` 监听文件变化自动 reload。
502. `DylibHotload::load(path) -> Result<DylibHandle>`。
503. `DylibHotload::unload(handle) -> Result<()>`。
504. `DylibHotload::reload(handle, new_path) -> Result<()>`。
505. `DylibHotload::symbol<T>(handle, name) -> Result<*const T>`。
506. `DylibHotload` 约定导出：`plugin_init / plugin_update / plugin_shutdown`。
507. `DylibHotload` 平台：macOS `.dylib`。
508. `DylibHotload` 平台：Linux `.so`。
509. `DylibHotload` 平台：Windows `.dll`。
510. `WasmHotswap::load(bytes) -> Result<WasmHandle>`。
511. `WasmHotswap::swap(handle, new_bytes) -> Result<()>`。
512. `WasmHotswap::call(handle, fn_name, args) -> Result<Value>`。
513. `WasmHotswap` 保留 host 侧 `World` 引用，热换 wasm 模块不重建 world。
514. `AssetHotreload::register_texture(path, handle)`。
515. `AssetHotreload::register_mesh(path, handle)`。
516. `AssetHotreload::register_audio(path, handle)`。
517. `AssetHotreload::register_scene(path, handle)`。
518. `AssetHotreload::tick(&mut self)` 轮询 mtime。
519. `AssetHotreload::on_change(path, cb)` 回调。
520. `AssetHotreload` 纹理重新上传 GPU。
521. `AssetHotreload` 场景重新解析并 update entity。

### 3.10 灰度发布 / 多源下载 / 更新 UI / 版本兼容

522. `GreyRelease::new() -> Self`。
523. `GreyRelease::by_channel(channels)` 设置渠道白名单。
524. `GreyRelease::by_version(range)` 设置版本区间。
525. `GreyRelease::by_device(models)` 设置设备型号。
526. `GreyRelease::by_region(regions)` 设置地区。
527. `GreyRelease::by_ratio(0.0..1.0)` 按比例灰度。
528. `GreyRelease::match_user(&self, user: &UserProfile) -> bool`。
529. `UserProfile::channel / os_version / device_model / region / user_id_hash`。
530. `Downloader::new(work_dir) -> Self`。
531. `Downloader::register_mirror(url, priority)`。
532. `Downloader::download(url, dest, expected_sha256) -> Result<()>`。
533. `Downloader::download_async(url, dest) -> JoinHandle<Result<()>>`。
534. `Downloader` 使用 HTTP Range 断点续传。
535. `Downloader` 使用 `ETag` 验证资源未变。
536. `Downloader` 失败自动切换 mirror。
537. `Downloader` 多线程分片（8 线程默认）。
538. `Downloader` 下载完成校验 SHA-256。
539. `Downloader::progress(&self, url) -> Option<(u64, u64)>`。
540. `UpdateUI::new() -> Self`。
541. `UpdateUI::show_progress(title, cur, total)`。
542. `UpdateUI::show_changelog(text)`。
543. `UpdateUI::show_mandatory_dialog(new_version, on_confirm)`。
544. `UpdateUI::show_optional_dialog(new_version, on_confirm, on_cancel)`。
545. `UpdateUI::hide()`。
546. `Version::parse("1.2.3") -> Result<Self>`。
547. `Version::major() / minor() / patch() / pre() / build()`。
548. `VersionReq::parse("^1.2") -> Result<Self>`。
549. `VersionReq::matches(&self, v) -> bool`。
550. `VersionCompat::check(current, min_required) -> CompatResult`。
551. `CompatResult::Ok`。
552. `CompatResult::Breaking(notes)`。
553. `CompatResult::UpgradeRequired(min_version)`。
554. `VersionCompat::breaking_notes(from, to) -> Vec<String>`（从 CHANGELOG 生成）。

### 3.11 插件系统核心（Plugin trait / 类型 / manifest / 沙盒）

555. `Plugin` trait：`fn name(&self) -> &str`。
556. `Plugin` trait：`fn version(&self) -> Version`。
557. `Plugin` trait：`fn on_load(&mut self, world: &mut World, registry: &mut PluginRegistry)`。
558. `Plugin` trait：`fn on_unload(&mut self, world: &mut World)`。
559. `Plugin` trait：`fn on_tick(&mut self, world: &mut World, dt: f32)`。
560. `Plugin` trait：`fn register_types(&mut self, registry: &mut TypeRegistry)`。
561. `PluginKind::RustDylib`。
562. `PluginKind::Wasm`。
563. `PluginKind::Script(ScriptLang)`。
564. `PluginKind::CAbi`。
565. `PluginManifest::from_toml(path) -> Result<Self>`。
566. `PluginManifest::name: String`。
567. `PluginManifest::version: String`。
568. `PluginManifest::description: Option<String>`。
569. `PluginManifest::authors: Vec<String>`。
570. `PluginManifest::dependencies: HashMap<String, VersionReq>`。
571. `PluginManifest::permissions: Vec<PluginPermission>`。
572. `PluginManifest::entry_point: PathBuf`。
573. `PluginManifest::kind: PluginKind`。
574. `PluginPermission::FileRead(glob)`。
575. `PluginPermission::FileWrite(glob)`。
576. `PluginPermission::FileDelete(glob)`。
577. `PluginPermission::NetConnect(host_glob, port_range)`。
578. `PluginPermission::NetListen(port_range)`。
579. `PluginPermission::MemoryLimit(bytes)`。
580. `PluginPermission::CpuLimit(seconds_per_min)`。
581. `PluginPermission::EnvRead(key_glob)`。
582. `PluginPermission::All`（仅开发模式）。
583. `PluginSandbox::new(manifest) -> Self`。
584. `PluginSandbox::check(&self, perm: &PluginPermission) -> bool`。
585. `PluginSandbox::deny(&self, perm: &PluginPermission) -> bool`。
586. `PluginSandbox::wrap_file_open(path, mode) -> Result<File>`。
587. `PluginSandbox::wrap_net_connect(addr) -> Result<TcpStream>`。
588. `PluginSandbox::wrap_alloc(bytes) -> Result<()>`（超过 quota 返回 Err）。
589. `PluginSandbox` 所有 I/O 经过 hook，未授权直接拒绝。

### 3.12 插件注册 / 依赖解析 / 生命周期 / 配额

590. `PluginRegistry::new() -> Self`。
591. `PluginRegistry::register_component<T: Component>(&mut self)`。
592. `PluginRegistry::register_system<T: System>(&mut self, stage, system)`。
593. `PluginRegistry::register_resource<T: Resource>(&mut self, init)`。
594. `PluginRegistry::register_event<T: Event>(&mut self)`。
595. `PluginRegistry::register_window(&mut self, builder)`。
596. `PluginRegistry::register_ui_widget(&mut self, widget)`。
597. `PluginRegistry::register_render_pass(&mut self, pass)`。
598. `PluginRegistry::register_asset_loader(&mut self, loader)`。
599. `PluginRegistry::entries(&self) -> &[RegistryEntry]`。
600. `Stage::PreUpdate / Update / PostUpdate / Render`。
601. `PluginResolver::new() -> Self`。
602. `PluginResolver::add(&mut self, manifest) -> Result<()>`。
603. `PluginResolver::resolve(&self) -> Result<Vec<PluginId>>`（拓扑排序）。
604. `PluginResolver::detect_cycles() -> Result<(), CycleError>`。
605. `PluginResolver::detect_conflicts() -> Result<(), ConflictError>`。
606. `PluginResolver` semver 冲突：同一 name 多个不兼容版本。
607. `CycleError::cycle_path: Vec<PluginId>`。
608. `ConflictError::plugin / version_a / version_b`。
609. `PluginLifecycle::new(world, registry) -> Self`。
610. `PluginLifecycle::load(&mut self, dir) -> Result<PluginHandle>`。
611. `PluginLifecycle::unload(&mut self, handle) -> Result<()>`。
612. `PluginLifecycle::upgrade(&mut self, handle, new_dir) -> Result<()>`。
613. `PluginLifecycle::reload(&mut self, handle) -> Result<()>`。
614. `PluginLifecycle::tick(&mut self, world, dt)` 调用所有 plugin `on_tick`。
615. `PluginQuota::new(mem_limit, cpu_limit) -> Self`。
616. `PluginQuota::memory_used(&self) -> usize`。
617. `PluginQuota::cpu_time(&self) -> Duration`。
618. `PluginQuota::handle_count(&self) -> usize`。
619. `PluginQuota::record_alloc(bytes)`。
620. `PluginQuota::record_dealloc(bytes)`。
621. `PluginQuota::record_cpu(duration)`。
622. `PluginQuota::over_quota(&self) -> bool`。
623. `PluginQuota::kill_if_over(&mut self, handle)` 超限卸载。

### 3.13 插件调试 / 插件市场

624. `PluginDebug::new(plugin_name) -> Self`。
625. `PluginDebug::log(&self, level, msg)` 输出到独立日志文件。
626. `PluginDebug::hook_fn(&self, target, before, after)` 函数 hook。
627. `PluginDebug::set_crash_handler(&self, handler)` 注册 panic hook。
628. `PluginDebug::crash_recovery(&self, handle, world)` 尝试卸载并报告。
629. `PluginDebug::profile(&self) -> PluginProfile`（内存 / CPU / 调用次数）。
630. `PluginProfile::top_functions(n) -> Vec<(String, Duration)>`。
631. `PluginStoreClient::new(base_url) -> Self`。
632. `PluginStoreClient::list(&self) -> Result<Vec<PluginInfo>>`。
633. `PluginStoreClient::search(&self, keyword) -> Result<Vec<PluginInfo>>`。
634. `PluginStoreClient::info(&self, name) -> Result<PluginInfo>`。
635. `PluginStoreClient::download(&self, name, version) -> Result<PathBuf>`。
636. `PluginStoreClient::install(&self, downloaded_path) -> Result<PluginHandle>`。
637. `PluginStoreClient::uninstall(&self, name) -> Result<()>`。
638. `PluginStoreClient::update(&self, name) -> Result<PluginHandle>`。
639. `PluginStoreClient::auth(token)` 设置鉴权 token。
640. `PluginInfo::name / version / author / rating / downloads / manifest_url`。

### 3.14 示例工程

641. `examples/network_chat`：TCP 聊天室，支持昵称 / 房间 / 广播。
642. `examples/network_chat`：服务端 `cargo run --example network_chat -- --server 0.0.0.0:8080`。
643. `examples/network_chat`：客户端 `cargo run --example network_chat -- --client 127.0.0.1:8080 --name Alice`。
644. `examples/network_echo`：UDP echo 回显（可靠 / 不可靠切换）。
645. `examples/network_rpc`：展示 `#[rpc]` 宏的客户端调用与服务端响应。
646. `examples/network_replication`：2D 方块在多客户端间位置同步（预测 + 插值）。
647. `examples/network_lobby`：大厅 + 房间创建 / 加入 + 匹配算法展示。
648. `examples/network_replay`：记录一场对战并可回放、跳转、倍速播放。
649. `examples/hotfix_patch`：CLI 工具 `diff old new -> patch` 与 `patch old patch -> new`。
650. `examples/hotfix_patch`：验证签名后应用 patch，失败回滚。
651. `examples/hotfix_script`：JS/Py/Lua 脚本热重载（修改源码后自动生效）。
652. `examples/hotfix_asset`：运行时修改纹理，观察画面立即刷新。
653. `examples/hotfix_grey`：构造 user profile 并展示灰度匹配逻辑。
654. `examples/plugin_hello`：加载一个 Rust dylib 插件并 tick。
655. `examples/plugin_ui_widget`：插件注册一个自定义 UI Widget（如心形按钮）。
656. `examples/plugin_render_pass`：插件注册一个后处理渲染 Pass（如 bloom）。
657. `examples/plugin_ffi`：从 C 侧调用 engine 的 FFI 插件示例。
658. 所有 example `cargo run --example <name>` 正常退出（非 0 异常）。

### 3.15 测试与质量

659. 单测：`TcpChannel` 回环消息发送/接收。
660. 单测：`UdpChannel` 可靠模式 ACK 去重。
661. 单测：`WsChannel` text frame 往返。
662. 单测：`QuicChannel` 单端握手。
663. 单测：`#[derive(NetworkMessage)]` 对 struct/enum 序列化往返。
664. 单测：MessagePack round-trip。
665. 单测：Bincode round-trip。
666. 单测：JSON round-trip。
667. 单测：Protobuf round-trip。
668. 单测：RPC request_id 匹配与超时。
669. 单测：Replication 属性过滤（OnlyServer 字段在客户端不可见）。
670. 单测：`Lobby::create_room / join_room / list_rooms`。
671. 单测：Matchmaker 基本匹配（2 人组队）。
672. 单测：`NatTraversal::stun_request`（mock STUN 服务器）。
673. 单测：`BandwidthController` token bucket 精确。
674. 单测：`NetCompressor::zstd` 压缩/解压。
675. 单测：`NetEncryptor::aes_gcm` 加密/解密。
676. 单测：`NetEncryptor::chacha20` 加密/解密。
677. 单测：`ReplayRecorder` / `ReplayPlayer` 文件往返。
678. 单测：bsdiff/bspatch 对 1KB 文件 patch 正确。
679. 单测：zstd 流压缩/解压。
680. 单测：`PatchBundle` RSA 签名/验证。
681. 单测：`PatchBundle` Ed25519 签名/验证。
682. 单测：`AssetDiff::diff_dir` 生成正确 Add/Modify/Remove。
683. 单测：`Version::parse` 与 `VersionReq::matches`。
684. 单测：`PluginManifest::from_toml` 正确解析。
685. 单测：`PluginSandbox::check` 拒绝未声明权限。
686. 单测：`PluginResolver` 检测循环依赖。
687. 单测：`PluginResolver` 检测版本冲突。
688. 单测：`PluginLifecycle::load / unload` 状态机正确。
689. 单测：`PluginQuota` 记录正确并触发 `over_quota`。
690. 单测：`GreyRelease::match_user` 按比例灰度分布稳定。
691. 单测：`Downloader` mock HTTP 服务器 + 断点续传。
692. 负例：沙盒插件访问未声明的绝对路径被拒绝。
693. 负例：沙盒插件访问未声明的网络地址被拒绝。
694. 负例：沙盒插件超限内存被 kill。
695. 负例：沙盒插件超限 CPU 被 kill。
696. 负例：RPC 不存在的 method 返回 `NotFound`。
697. 负例：RPC 超时返回 `Timeout`。
698. 负例：PatchBundle 被篡改 hash 不通过验证。
699. 负例：Lobby 加入密码错误的房间失败。
700. 负例：PluginResolver 循环依赖抛错。
701. `cargo test -p engine-network` 全部通过。
702. `cargo test -p engine-hotfix` 全部通过。
703. `cargo test -p engine-plugin` 全部通过。
704. `cargo test --workspace` 全部通过。
705. `cargo clippy --workspace -- -D warnings` 通过。
706. `cargo fmt --check --workspace` 通过。
707. `cargo doc --workspace --no-deps` 成功且无 warning。
708. 网络基准：1000 并发连接稳定 >= 60 秒。
709. 网络基准：平均延迟 < 50ms。
710. 网络基准：丢包率 < 1%（模拟 0.1% 丢包网络）。
711. 网络基准：吞吐量 >= 5000 msg/s。
712. 热更基准：1GB 资源差分 < 30s（CPU 8 核）。
713. 热更基准：patch 大小 < 原始 10%（对于差异 < 5% 的文件）。
714. 热更基准：100MB patch 应用 < 10s。
715. 插件安全：沙盒逃逸负例测试全部通过。
716. 插件安全：未授权网络访问被拒绝。
717. 插件安全：超限内存使用被 kill。
718. 插件安全：超限 CPU 使用被 kill。
719. CI：Linux x86_64 green。
720. CI：macOS x86_64 / arm64 green。
721. CI：Windows x86_64 green。
722. CHANGELOG 记录 0.15.0：`engine-network` / `engine-hotfix` / `engine-plugin` 概览。
723. README.md 加入「网络系统」章节：通道 / RPC / Replication。
724. README.md 加入「热更新系统」章节：差分 / 脚本 / 灰度。
725. README.md 加入「插件系统与生态」章节：manifest / sandbox / store。
726. 公开 API doc comment 覆盖率 100%（三个 crate 合计）。
727. 本 Sprint `unsafe` 块 <= 5（主要用于 dylib 符号加载 / FFI）。
728. 新增 example 工程 >= 14 个。
729. `examples/network_chat` 多人聊天可运行。
730. `examples/hotfix_patch` CLI 可运行。
731. `examples/plugin_hello` 可加载并 tick 插件。
732. `examples/network_replication` 预测 + 插值平滑展示。
733. `examples/plugin_render_pass` bloom 效果可见。
734. `examples/hotfix_asset` 纹理热重载可见。
735. `examples/plugin_ui_widget` 自定义 UI 可见。

> 以上 735 条需求（编号 1..735）构成 Sprint 15 全量清单。

---

## 四、验收标准

- [ ] `cargo run --example network_chat -- --server 0.0.0.0:8080` 可启动服务端
- [ ] `cargo run --example network_chat -- --client 127.0.0.1:8080 --name Alice` 可连接并收发消息
- [ ] `cargo run --example hotfix_patch -- diff old.bin new.bin out.patch` 可生成 patch
- [ ] `cargo run --example hotfix_patch -- patch old.bin out.patch new.bin` 可还原 new.bin
- [ ] `cargo run --example plugin_hello` 可加载 Rust dylib 插件并 tick
- [ ] `cargo run --example network_replication` 多客户端位置同步平滑
- [ ] `cargo run --example network_rpc` 客户端发起 RPC 并获得响应
- [ ] `cargo run --example network_lobby` 创建/加入/列出房间正常
- [ ] `cargo run --example network_replay` 记录并回放对战
- [ ] `cargo run --example hotfix_script` 修改脚本后运行时热重载生效
- [ ] `cargo run --example hotfix_asset` 纹理热重载画面刷新
- [ ] `cargo run --example hotfix_grey` 灰度按比例分布稳定
- [ ] `cargo run --example plugin_ui_widget` 自定义 UI Widget 可见
- [ ] `cargo run --example plugin_render_pass` bloom 后处理可见
- [ ] `cargo run --example plugin_ffi` C ABI 插件可调用 engine 接口
- [ ] `cargo test -p engine-network` 全部通过
- [ ] `cargo test -p engine-hotfix` 全部通过
- [ ] `cargo test -p engine-plugin` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] CI 三平台 green
- [ ] CHANGELOG 记录 0.15.0
- [ ] 公开 API doc comment 覆盖率 100%
- [ ] 本 Sprint `unsafe` 块 <= 5
- [ ] 网络基准：1000 并发连接稳定，延迟 < 50ms，丢包 < 1%
- [ ] 热更基准：1GB 资源差分 < 30s，patch < 原始 10%
- [ ] 插件安全：沙盒逃逸负例测试全部通过

---

## 五、下一个 Sprint

Sprint 16 将引入 Asset Store（插件市场）、Mod 工具链、以及跨平台发布管线（桌面 / Web / 移动端打包器）。