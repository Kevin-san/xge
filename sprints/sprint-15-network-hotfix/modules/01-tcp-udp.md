# TCP/UDP 网络需求

## 模块概述

TCP/UDP 网络模块 (`engine-network`) 提供统一抽象的网络通道实现，包括 TCP、UDP、WebSocket、QUIC、RakNet 等多种传输层协议的支持。本模块是整个网络系统的基础，为上层 RPC、状态同步、大厅系统等提供可靠的传输服务。

## 需求清单

### 1. NetChannel Trait 抽象

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|--------|
| 4 | 发送消息 | `async fn send(&self, msg: &[u8]) -> Result<()>` | `&[u8]` 字节数组 | `Result<()>` | 返回 `Ok(())` 表示发送成功 | - | P0 |
| 5 | 接收消息 | `async fn recv(&mut self) -> Result<Option<Vec<u8>>>` | - | `Result<Option<Vec<u8>>>` | 返回 `Ok(None)` 表示连接关闭 | - | P0 |
| 6 | 连接状态 | `fn is_connected(&self) -> bool` | - | `bool` | 正确反映当前连接状态 | - | P0 |
| 7 | 对端地址 | `fn peer_addr(&self) -> Option<SocketAddr>` | - | `Option<SocketAddr>` | 正确返回对端 SocketAddr | - | P0 |
| 8 | 关闭连接 | `fn close(&mut self) -> Result<()>` | - | `Result<()>` | 优雅关闭连接 | - | P0 |
| 241 | 异步发送 | `async fn send(&self, payload: &[u8]) -> Result<()>` | `&[u8]` | `Result<()>` | 异步发送不阻塞 | - | P0 |
| 242 | 异步接收 | `async fn recv(&mut self) -> Result<Option<Vec<u8>>>` | - | `Result<Option<Vec<u8>>>` | 异步接收消息 | - | P0 |
| 243 | 连接状态 | `fn is_connected(&self) -> bool` | - | `bool` | 与 6 一致 | - | P0 |
| 244 | 对端地址 | `fn peer_addr(&self) -> Option<std::net::SocketAddr>` | - | `Option<SocketAddr>` | 与 7 一致 | - | P0 |
| 245 | 本地地址 | `fn local_addr(&self) -> Option<std::net::SocketAddr>` | - | `Option<SocketAddr>` | 返回本地绑定地址 | - | P0 |
| 246 | 关闭连接 | `fn close(&mut self) -> Result<()>` | - | `Result<()>` | 与 8 一致 | - | P0 |
| 247 | 统计信息 | `fn stats(&self) -> NetStats` | - | `NetStats` | 返回网络统计 | - | P0 |
| 248 | 统计数据结构 | `NetStats: bytes_in / bytes_out / msg_in / msg_out / rtt_ms` | - | 结构体 | 包含所有统计字段 | - | P0 |

### 2. TCP Channel 实现

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 9 | TCP 客户端 | `TcpChannel::connect(addr) -> Result<Self>` | SocketAddr | `Result<TcpChannel>` | 成功建立 TCP 连接 | 4-8 | P0 |
| 81 | TCP 客户端实现 | `TcpChannel` 客户端实现 | - | - | 支持 connect | - | P0 |
| 10 | TCP 服务端 | `TcpChannel` 服务端实现（accept） | - | - | 支持接收连接 | - | P0 |
| 82 | TCP 服务端实现 | `TcpListener::accept() -> Result<TcpChannel>` | - | `Result<TcpChannel>` | 返回已连接的 channel | - | P0 |
| 249 | TCP 客户端连接 | `TcpChannel::connect(addr) -> Result<Self>` | SocketAddr | `Result<Self>` | 与 9 一致 | - | P0 |
| 250 | TCP 服务端绑定 | `TcpChannel::bind(addr) -> Result<TcpListener>` | SocketAddr | `Result<TcpListener>` | 绑定并监听 | - | P0 |
| 251 | TCP 接受连接 | `TcpListener::accept() -> Result<TcpChannel>` | - | `Result<TcpChannel>` | 与 82 一致 | - | P0 |
| 252 | TCP 迭代器 | `TcpListener::incoming() -> impl Stream<Item = Result<TcpChannel>>` | - | Stream | 返回异步流 | - | P0 |
| 284 | 压缩配置 | `NetChannel::set_compression(bool)` | bool | - | 设置压缩开关 | - | P1 |
| 285 | 加密配置 | `NetChannel::set_encryption(bool)` | bool | - | 设置加密开关 | - | P1 |
| 286 | 统计接口 | `NetChannel::stats() -> NetStats` | - | NetStats | 与 287 一致 | - | P0 |

### 3. UDP Channel 实现

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 11 | UDP 通道 | `UdpChannel` 实现（支持可靠与不可靠标记） | - | - | 支持两种模式 | 4-8 | P0 |
| 12 | UDP 可靠模式 | 可靠模式：滑动窗口 / ACK / 重传 | - | - | 可靠传输 | - | P0 |
| 83 | UDP 可靠模式 | `UdpChannel` 可靠模式：滑动窗口 / ACK / 重传 | - | - | 与 12 一致 | - | P0 |
| 253 | UDP 绑定 | `UdpChannel::bind(addr) -> Result<Self>` | SocketAddr | `Result<Self>` | 绑定 UDP socket | - | P0 |
| 254 | UDP 连接 | `UdpChannel::connect(addr) -> Result<Self>` | SocketAddr | `Result<Self>` | 设置默认对端 | - | P0 |
| 255 | UDP 发送 | `UdpChannel::send_to(addr, payload, reliability) -> Result<()>` | SocketAddr, payload, Reliability | `Result<()>` | 支持指定可靠性 | - | P0 |
| 256 | UDP 接收 | `UdpChannel::recv_from() -> Result<Option<(SocketAddr, Vec<u8>)>>` | - | `Result<Option<(SocketAddr, Vec<u8>)>>` | 返回地址和数据 | - | P0 |
| 257 | 不可靠传输 | `Reliability::Unreliable` | - | - | 不保证送达 | - | P0 |
| 258 | 顺序不可靠 | `Reliability::UnreliableSequenced` | - | - | 顺序但不重传 | - | P0 |
| 259 | 可靠传输 | `Reliability::Reliable` | - | - | 保证送达 | - | P0 |
| 260 | 可靠有序 | `Reliability::ReliableOrdered` | - | - | 保证顺序 | - | P0 |
| 261 | 滑动窗口 | 可靠模式：滑动窗口大小 32 | - | - | 窗口大小可配置 | - | P1 |
| 262 | ACK 位图 | 可靠模式：ACK 位图 | - | - | 确认收到消息 | - | P0 |
| 263 | 重传超时 | 可靠模式：重传超时 200ms（可配置） | - | Duration | 默认 200ms | - | P1 |

### 4. WebSocket Channel 实现

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 13 | WS 客户端 | `WsChannel` 客户端（基于 tokio-tungstenite） | - | - | 支持连接 | - | P0 |
| 14 | WS 服务端 | `WsChannel` 服务端（基于 tokio-tungstenite） | - | - | 支持监听 | - | P0 |
| 15 | WS 帧区分 | text / binary frame 区分 | - | - | 正确区分帧类型 | - | P0 |
| 84 | WS 客户端实现 | `WsChannel::connect(url) -> Result<Self>` | URL | `Result<Self>` | 连接 WebSocket | - | P0 |
| 85 | WS 服务端实现 | `WsChannel::bind(addr) -> Result<WsListener>` | SocketAddr | `Result<WsListener>` | 绑定 WebSocket | - | P0 |
| 264 | WS 客户端连接 | `WsChannel::connect(url) -> Result<Self>` | URL | `Result<Self>` | 与 84 一致 | - | P0 |
| 265 | WS 服务端绑定 | `WsChannel::bind(addr) -> Result<WsListener>` | SocketAddr | `Result<WsListener>` | 与 85 一致 | - | P0 |
| 266 | WS 发送文本 | `WsChannel::send_text(s: &str) -> Result<()>` | &str | `Result<()>` | 发送文本帧 | - | P0 |
| 267 | WS 发送二进制 | `WsChannel::send_binary(buf: &[u8]) -> Result<()>` | &[u8] | `Result<()>` | 发送二进制帧 | - | P0 |
| 268 | WS 接收消息 | `WsChannel::recv_text_or_binary() -> Result<WsMessage>` | - | `Result<WsMessage>` | 返回消息类型 | - | P0 |
| 269 | WS 文本消息 | `WsMessage::Text(String)` | - | - | 文本消息变体 | - | P0 |
| 270 | WS 二进制消息 | `WsMessage::Binary(Vec<u8>)` | - | - | 二进制消息变体 | - | P0 |
| 271 | WS 关闭消息 | `WsMessage::Close` | - | - | 关闭连接消息 | - | P0 |
| 272 | WS Ping | `WsMessage::Ping(Vec<u8>)` | - | - | Ping 消息 | - | P0 |
| 273 | WS Pong | `WsMessage::Pong(Vec<u8>)` | - | - | Pong 消息 | - | P0 |

### 5. QUIC Channel 实现

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 16 | QUIC 通道 | `QuicChannel`（基于 quinn） | - | - | 支持 QUIC 协议 | - | P1 |
| 86 | QUIC 实现 | `QuicChannel::connect(addr, server_name) -> Result<Self>` | SocketAddr, server_name | `Result<Self>` | 建立 QUIC 连接 | - | P1 |
| 274 | QUIC 客户端连接 | `QuicChannel::connect(addr, server_name) -> Result<Self>` | SocketAddr, server_name | `Result<Self>` | 与 86 一致 | - | P1 |
| 275 | QUIC 服务端绑定 | `QuicChannel::bind(addr, cert) -> Result<QuicListener>` | SocketAddr, cert | `Result<QuicListener>` | 绑定并监听 | - | P1 |
| 276 | QUIC Stream | 复用 QUIC stream | - | - | 多流复用 | - | P1 |
| 277 | QUIC 双向流 | `QuicChannel::open_bi_stream() -> Result<(QuicSend, QuicRecv)>` | - | `(QuicSend, QuicRecv)` | 打开双向流 | - | P1 |

### 6. RakNet Channel 实现

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 17 | RakNet 通道 | `RaknetChannel`：RakNet 兼容协议层（或自研可靠 UDP） | - | - | 兼容 RakNet 协议 | - | P1 |
| 87 | RakNet 实现 | `RaknetChannel::connect(addr) -> Result<Self>` | SocketAddr | `Result<Self>` | 建立 RakNet 连接 | - | P1 |
| 278 | RakNet 客户端连接 | `RaknetChannel::connect(addr) -> Result<Self>` | SocketAddr | `Result<Self>` | 与 87 一致 | - | P1 |
| 279 | RakNet 握手 | 使用 RakNet 兼容握手协议 | - | - | 协议兼容 | - | P1 |
| 280 | MTU 探测 | MTU 探测 | - | - | 发现最佳 MTU | - | P1 |
| 281 | 拥塞控制 | 拥塞控制（类似 CUBIC 简化版） | - | - | 拥塞避免 | - | P1 |

### 7. Channel Builder 与配置

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 282 | 通道构建器 | `ChannelBuilder`：配置通道（超时 / 加密 / 压缩） | - | - | 统一配置接口 | - | P0 |
| 283 | 超时配置 | `ChannelBuilder::timeout(Duration)` | Duration | Self | 设置超时时间 | - | P0 |
| 284 | 加密配置 | `ChannelBuilder::encryption(Cipher)` | Cipher | Self | 设置加密算法 | - | P0 |
| 285 | 压缩配置 | `ChannelBuilder::compression(CompressionKind)` | CompressionKind | Self | 设置压缩算法 | - | P0 |
| 286 | AES256-GCM | `Cipher::Aes256Gcm` | - | - | AES-256-GCM 加密 | - | P0 |
| 287 | ChaCha20 | `Cipher::ChaCha20Poly1305` | - | - | ChaCha20-Poly1305 加密 | - | P0 |
| 288 | 无加密 | `Cipher::None` | - | - | 不加密 | - | P0 |
| 289 | Zstd 压缩 | `CompressionKind::Zstd(level)` | level: i32 | - | Zstd 压缩 | - | P1 |
| 290 | Snappy 压缩 | `CompressionKind::Snappy` | - | - | Snappy 压缩 | - | P1 |
| 291 | LZ4 压缩 | `CompressionKind::Lz4` | - | - | LZ4 压缩 | - | P1 |
| 292 | 无压缩 | `CompressionKind::None` | - | - | 不压缩 | - | P0 |

### 8. crate 建立

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 1 | crate 建立 | `engine-network` crate 建立 | - | - | Cargo.toml 存在且可编译 | - | P0 |

## 验收标准

- `cargo test -p engine-network` 全部通过
- `cargo clippy --workspace -- -D warnings` 通过
- `cargo fmt --check --workspace` 通过
- 1000 并发连接稳定 >= 60 秒
- 平均延迟 < 50ms
- 丢包率 < 1%（模拟 0.1% 丢包网络）
- 吞吐量 >= 5000 msg/s
- 所有 NetChannel trait 方法实现完整

## 依赖关系

```
engine-network
├── tokio (async runtime)
├── tokio-tungstenite (WebSocket)
├── quinn (QUIC)
├── bytes (bytes handling)
└── socket2 (socket configuration)
```

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 内完成
- **P1**: 重要功能，应尽量完成
- **P2**: 优化功能，可延后到下一 Sprint
