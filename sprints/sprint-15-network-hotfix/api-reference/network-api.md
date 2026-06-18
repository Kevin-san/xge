# 网络 API 清单

## 概述

本文档列出 `engine-network` crate 提供的所有公开 API，包括通道抽象、序列化、RPC、同步、大厅匹配等核心功能。

---

## 1. NetChannel Trait

### 核心接口

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `send` | `async fn send(&self, payload: &[u8]) -> Result<()>` | 异步发送数据 |
| `recv` | `async fn recv(&mut self) -> Result<Option<Vec<u8>>>` | 异步接收数据 |
| `is_connected` | `fn is_connected(&self) -> bool` | 检查连接状态 |
| `peer_addr` | `fn peer_addr(&self) -> Option<SocketAddr>` | 获取远端地址 |
| `local_addr` | `fn local_addr(&self) -> Option<SocketAddr>` | 获取本地地址 |
| `close` | `fn close(&mut self) -> Result<()>` | 关闭连接 |
| `stats` | `fn stats(&self) -> NetStats` | 获取统计信息 |

### NetStats 结构

| 字段 | 类型 | 说明 |
| :--- | :--- | :--- |
| `bytes_in` | `u64` | 累计接收字节数 |
| `bytes_out` | `u64` | 累计发送字节数 |
| `msg_in` | `u64` | 累计接收消息数 |
| `msg_out` | `u64` | 累计发送消息数 |
| `rtt_ms` | `u32` | 当前往返时间（毫秒） |

---

## 2. 通道实现

### TcpChannel

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `connect` | `fn connect(addr: SocketAddr) -> Result<Self>` | 客户端连接 |
| `bind` | `fn bind(addr: SocketAddr) -> Result<TcpListener>` | 服务端绑定 |

### TcpListener

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `accept` | `async fn accept(&mut self) -> Result<TcpChannel>` | 接受连接 |
| `incoming` | `fn incoming(&mut self) -> impl Stream<Item = Result<TcpChannel>>` | 连接流 |

### UdpChannel

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `bind` | `fn bind(addr: SocketAddr) -> Result<Self>` | 绑定地址 |
| `connect` | `fn connect(addr: SocketAddr) -> Result<Self>` | 连接到地址 |
| `send_to` | `async fn send_to(&mut self, addr: SocketAddr, payload: &[u8], reliability: Reliability) -> Result<()>` | 发送到指定地址 |
| `recv_from` | `async fn recv_from(&mut self) -> Result<Option<(SocketAddr, Vec<u8>)>>` | 接收数据 |

### Reliability 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Unreliable` | 不可靠传输 |
| `UnreliableSequenced` | 不可靠但有序 |
| `Reliable` | 可靠传输 |
| `ReliableOrdered` | 可靠且有序 |

### WsChannel

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `connect` | `fn connect(url: &str) -> Result<Self>` | 客户端连接 |
| `bind` | `fn bind(addr: SocketAddr) -> Result<WsListener>` | 服务端绑定 |
| `send_text` | `async fn send_text(&mut self, s: &str) -> Result<()>` | 发送文本 |
| `send_binary` | `async fn send_binary(&mut self, buf: &[u8]) -> Result<()>` | 发送二进制 |
| `recv_text_or_binary` | `async fn recv_text_or_binary(&mut self) -> Result<WsMessage>` | 接收消息 |

### WsMessage 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Text(String)` | 文本消息 |
| `Binary(Vec<u8>)` | 二进制消息 |
| `Close` | 关闭消息 |
| `Ping(Vec<u8>)` | Ping 消息 |
| `Pong(Vec<u8>)` | Pong 消息 |

### QuicChannel

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `connect` | `fn connect(addr: SocketAddr, server_name: &str) -> Result<Self>` | 客户端连接 |
| `bind` | `fn bind(addr: SocketAddr, cert: Certificate) -> Result<QuicListener>` | 服务端绑定 |
| `open_bi_stream` | `async fn open_bi_stream(&mut self) -> Result<(QuicSend, QuicRecv)>` | 打开双向流 |

### RaknetChannel

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `connect` | `fn connect(addr: SocketAddr) -> Result<Self>` | 连接到服务器 |

---

## 3. NetMessage Trait

### 核心接口

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `type_id` | `fn type_id() -> u32` | 获取消息类型 ID（crc32） |
| `serialize` | `fn serialize(&self, serializer: &mut NetSerializer) -> Result<()>` | 序列化到串行器 |
| `deserialize` | `fn deserialize(deserializer: &mut NetDeserializer) -> Result<Self>` | 从反串行器解析 |
| `encode` | `fn encode(&self, format: SerializeFormat) -> Result<Vec<u8>>` | 编码为字节 |
| `decode` | `fn decode(buf: &[u8], format: SerializeFormat) -> Result<Self>` | 从字节解码 |

### SerializeFormat 枚举

| 变体 | 说明 |
| :--- | :--- |
| `MessagePack` | MessagePack 格式 |
| `Json` | JSON 格式 |
| `Protobuf` | Protobuf 格式 |
| `Bincode` | Bincode 格式 |
| `CapnProto` | Cap'n Proto 格式 |

### 派生宏属性

| 属性 | 说明 |
| :--- | :--- |
| `#[derive(NetworkMessage)]` | 自动生成 trait 实现 |
| `#[net_message(type_id = 42)]` | 手动指定类型 ID |
| `#[net_message(skip)]` | 跳过字段 |
| `#[net_message(format = "Bincode")]` | 指定默认格式 |

---

## 4. RPC 系统

### 属性宏

| 宏 | 说明 |
| :--- | :--- |
| `#[rpc]` | 应用在 impl block 上 |
| `#[rpc(server)]` | 生成服务端 dispatcher |
| `#[rpc(client)]` | 生成客户端 stub |
| `#[rpc(bidirectional)]` | 双向 RPC |
| `#[rpc_method]` | 标记单个方法 |
| `#[rpc_method(one_way)]` | 无需返回 |
| `#[rpc_method(timeout = "3s")]` | 指定超时 |

### 生成的客户端 API

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `RpcClient::new(channel) -> Self` | 创建客户端 |
| `method_name` | `async fn method_name(&self, args...) -> Result<Ret>` | 调用 RPC 方法 |

### 生成的服务端 API

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `dispatch` | `RpcServer::dispatch(&mut self, msg) -> Result<Option<Vec<u8>>>` | 分发 RPC 请求 |

### RpcError 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Timeout` | 请求超时 |
| `NotFound` | 方法不存在 |
| `Deserialize` | 反序列化失败 |
| `Serialize` | 序列化失败 |
| `Application(String)` | 应用层错误 |

### 中间件

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `RpcMiddleware::before` | 在请求前执行 |
| `RpcMiddleware::after` | 在响应后执行 |
| `RpcRateLimiter` | QPS 限流 |
| `RpcAuth` | Token 鉴权 |

---

## 5. Entity 同步

### 组件

| 组件 | 字段 | 说明 |
| :--- | :--- | :--- |
| `Replication` | 空标记 | 标记需要同步的 entity |
| `NetworkSync` | `last_authority_pos`, `last_authority_rot`, `last_authority_vel`, `tick` | 同步状态 |
| `NetworkOwner` | `client_id` | 拥有者客户端 ID |
| `NetworkId` | `u64` | 全局唯一网络 ID |

### NetAttr 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Replicate` | 全量同步 |
| `OnlyServer` | 仅服务端 |
| `OnlyOwner` | 仅 Owner |
| `OwnerAuto` | 自动推断 Owner |
| `Predict` | 启用预测 |
| `Interpolate` | 启用插值 |

### 系统 API

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `PredictionBuffer::push` | `push(tick, input, state)` | 记录预测状态 |
| `PredictionBuffer::replay_from` | `replay_from(tick, authoritative_state)` | 重放并校正 |
| `InterpolationBuffer::push` | `push(tick, state)` | 添加插值状态 |
| `InterpolationBuffer::sample` | `sample(now) -> state` | 线性插值采样 |

### 事件

| 事件 | 字段 | 说明 |
| :--- | :--- | :--- |
| `OnSpawn` | `client_id`, `entity`, `snapshot` | 实体在远端生成 |
| `OnDespawn` | `client_id`, `entity` | 实体在远端销毁 |
| `OnSync` | `client_id`, `entity`, `snapshot` | 同步数据到达 |
| `OnRpc` | `from`, `to`, `rpc_id`, `payload` | RPC 调用 |
| `OnConnect` | `client_id` | 玩家连接 |
| `OnDisconnect` | `client_id`, `reason` | 玩家断开 |

---

## 6. 大厅与匹配

### Lobby

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `fn new(max_rooms: usize) -> Self` | 创建大厅 |
| `create_room` | `fn create_room(&mut self, creator: ClientId, config: RoomConfig) -> RoomId` | 创建房间 |
| `join_room` | `fn join_room(&mut self, client: ClientId, room_id: RoomId) -> Result<()>` | 加入房间 |
| `leave_room` | `fn leave_room(&mut self, client: ClientId, room_id: RoomId) -> Result<()>` | 离开房间 |
| `list_rooms` | `fn list_rooms(&self) -> Vec<RoomInfo>` | 获取房间列表 |
| `set_ready` | `fn set_ready(&mut self, client: ClientId, room_id: RoomId, ready: bool) -> Result<()>` | 设置准备状态 |
| `start_game` | `fn start_game(&mut self, room_id: RoomId) -> Result<GameHandle>` | 开始游戏 |

### RoomConfig

| 字段 | 类型 | 说明 |
| :--- | :--- | :--- |
| `name` | `String` | 房间名称 |
| `max_players` | `usize` | 最大人数 |
| `password` | `Option<String>` | 密码 |
| `is_private` | `bool` | 是否私有 |
| `game_mode` | `String` | 游戏模式 |

### RoomInfo

| 字段 | 类型 | 说明 |
| :--- | :--- | :--- |
| `id` | `RoomId` | 房间 ID |
| `name` | `String` | 房间名称 |
| `current` | `usize` | 当前人数 |
| `max` | `usize` | 最大人数 |
| `is_private` | `bool` | 是否私有 |
| `ping_ms` | `u32` | 延迟 |

### Matchmaker

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `fn new(lobby: &Lobby) -> Self` | 创建匹配器 |
| `enqueue` | `fn enqueue(&mut self, client: ClientId, profile: PlayerProfile) -> QueueId` | 加入队列 |
| `dequeue` | `fn dequeue(&mut self, queue_id: QueueId)` | 退出队列 |
| `tick` | `fn tick(&mut self)` | 执行匹配算法 |

---

## 7. NAT 穿透

### NatTraversal

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `stun_request` | `async fn stun_request(stun_server: &str) -> Result<SocketAddr>` | 获取公网地址 |
| `turn_allocate` | `async fn turn_allocate(turn_server: &str, cred: Credentials) -> Result<RelayAddr>` | 分配 TURN 中继 |
| `ice_gather` | `async fn ice_gather() -> Vec<IceCandidate>` | 收集 ICE 候选 |
| `ice_connect` | `async fn ice_connect(a_candidates: &[IceCandidate], b_candidates: &[IceCandidate]) -> Result<SocketPair>` | 建立连接 |

### IceCandidate 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Host` | 本地地址 |
| `ServerReflexive` | 服务器反射地址 |
| `Relay` | 中继地址 |

---

## 8. 调试与回放

### NetDebugPanel

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `fn new() -> Self` | 创建调试面板 |
| `set_visible` | `fn set_visible(&mut self, visible: bool)` | 设置可见性 |
| `draw` | `fn draw(&mut self, ui: &mut Ui, net: &Network)` | 绘制面板 |

### ReplayRecorder

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `fn new(path: &Path) -> Result<Self>` | 创建记录器 |
| `record` | `fn record(&mut self, tick: u64, msg: &[u8])` | 记录消息 |
| `flush` | `async fn flush(&mut self) -> Result<()>` | 刷新到磁盘 |
| `close` | `async fn close(&mut self) -> Result<()>` | 关闭 |

### ReplayPlayer

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `fn new(path: &Path) -> Result<Self>` | 创建播放器 |
| `next_frame` | `async fn next_frame(&mut self) -> Result<Option<ReplayFrame>>` | 获取下一帧 |
| `seek_to` | `async fn seek_to(&mut self, tick: u64) -> Result<()>` | 跳转到指定帧 |
| `speed` | `fn speed(&mut self, factor: f32)` | 设置播放速度 |
| `is_finished` | `fn is_finished(&self) -> bool` | 是否播放完毕 |

---

## 9. 反作弊

### AntiCheat

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `state_hash` | `fn state_hash(world: &World) -> u64` | 计算世界状态哈希 |
| `crc_compare` | `fn crc_compare(client_hash: u64, server_hash: u64) -> bool` | 比较 CRC |
| `sign_input` | `fn sign_input(input: &[u8], private_key: &PrivateKey) -> Signature` | 签名输入 |
| `verify_input` | `fn verify_input(input: &[u8], signature: &Signature, public_key: &PublicKey) -> bool` | 验证输入 |

---

## 10. 构建器

### ChannelBuilder

| API | 签名 | 说明 |
| :--- | :--- | :--- |
| `new` | `fn new() -> Self` | 创建构建器 |
| `timeout` | `fn timeout(self, duration: Duration) -> Self` | 设置超时 |
| `encryption` | `fn encryption(self, cipher: Cipher) -> Self` | 设置加密 |
| `compression` | `fn compression(self, kind: CompressionKind) -> Self` | 设置压缩 |

### Cipher 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Aes256Gcm` | AES-256-GCM |
| `ChaCha20Poly1305` | ChaCha20-Poly1305 |
| `None` | 无加密 |

### CompressionKind 枚举

| 变体 | 说明 |
| :--- | :--- |
| `Zstd(level)` | zstd 压缩 |
| `Snappy` | snappy 压缩 |
| `Lz4` | lz4 压缩 |
| `None` | 无压缩 |

---

## 11. 枚举与常量

### NetRole

| 变体 | 说明 |
| :--- | :--- |
| `Server` | 纯服务端 |
| `Client` | 纯客户端 |
| `ListenServer` | 监听服务器（主机） |
| `Standalone` | 单机模式 |

### 常量

| 常量 | 值 | 说明 |
| :--- | :--- | :--- |
| `DEFAULT_RPC_TIMEOUT` | `5s` | 默认 RPC 超时 |
| `UDP_WINDOW_SIZE` | `32` | UDP 滑动窗口大小 |
| `UDP_RETRANSMIT_TIMEOUT` | `200ms` | 重传超时 |
| `MATCHMAKING_TIMEOUT` | `60s` | 匹配超时 |

---

## 12. 错误类型

| 错误类型 | 说明 |
| :--- | :--- |
| `NetError` | 网络错误 |
| `SerializeError` | 序列化错误 |
| `RpcError` | RPC 错误 |
| `ReplicationError` | 同步错误 |
| `LobbyError` | 大厅错误 |
| `MatchmakerError` | 匹配错误 |
| `NatTraversalError` | NAT 穿透错误 |