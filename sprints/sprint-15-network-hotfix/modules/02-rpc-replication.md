# RPC 与状态同步需求

## 模块概述

RPC 与状态同步模块提供远程过程调用能力和实体状态同步机制。RPC 系统支持同步/异步调用、超时重试、流式响应等特性；状态同步系统支持客户端预测、服务端延迟补偿、插值同步等高级功能。

## 需求清单

### 1. NetMessage Trait 与序列化

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 18 | 消息类型 ID | `NetMessage::type_id() -> u32` | - | u32 | 返回 CRC32 类型 ID | - | P0 |
| 19 | 序列化 | `NetMessage::serialize(&self) -> Vec<u8>` | - | `Vec<u8>` | 返回字节数组 | - | P0 |
| 20 | 反序列化 | `NetMessage::deserialize(buf) -> Result<Self>` | buf | Result<Self> | 从字节恢复对象 | - | P0 |
| 90 | 类型 ID | `NetMessage::type_id() -> u32`（crc32 of type_name） | - | u32 | 与 18 一致 | - | P0 |
| 293 | 类型 ID 实现 | `fn type_id() -> u32`（crc32 of type_name） | - | u32 | 自动生成类型 ID | - | P0 |
| 294 | 序列化接口 | `fn serialize(&self, serializer: &mut NetSerializer) -> Result<()>` | serializer | Result | 序列化到 serializer | - | P0 |
| 295 | 反序列化接口 | `fn deserialize(deserializer: &mut NetDeserializer) -> Result<Self>` | deserializer | Result | 从 deserializer 恢复 | - | P0 |
| 296 | 编码 | `fn encode(&self, format: SerializeFormat) -> Result<Vec<u8>>` | SerializeFormat | Result<Vec<u8>> | 按格式编码 | - | P0 |
| 297 | 解码 | `fn decode(buf: &[u8], format: SerializeFormat) -> Result<Self>` | buf, format | Result<Self> | 按格式解码 | - | P0 |
| 377 | type_id 实现 | `fn type_id() -> u32`（crc32 of type_name） | - | u32 | 与 293 一致 | - | P0 |
| 378 | 序列化实现 | `fn serialize(&self, serializer: &mut NetSerializer) -> Result<()>` | serializer | Result | 与 294 一致 | - | P0 |
| 379 | 反序列化实现 | `fn deserialize(deserializer: &mut NetDeserializer) -> Result<Self>` | deserializer | Result | 与 295 一致 | - | P0 |

### 2. NetworkMessage 派生宏

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 21 | 派生宏 | `#[derive(NetworkMessage)]` 过程宏 | - | - | 为 struct 生成实现 | 18-20 | P0 |
| 93 | 派生宏实现 | `#[derive(NetworkMessage)]` 过程宏 | - | - | 与 21 一致 | - | P0 |
| 302 | struct 派生 | 为 struct 生成默认实现 | - | - | struct 派生正确 | - | P0 |
| 303 | enum 派生 | 为 enum 生成默认实现 | - | - | enum 派生正确 | - | P0 |
| 304 | 手动 type_id | `#[net_message(type_id = 42)]` 手动指定 type_id | - | - | 手动指定生效 | - | P1 |
| 305 | 跳过字段 | `#[net_message(skip)]` 跳过字段 | - | - | 字段被跳过 | - | P1 |
| 306 | 指定格式 | `#[net_message(format = "Bincode")]` 指定默认格式 | - | - | 格式指定生效 | - | P1 |

### 3. NetSerializer 序列化器

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 22 | 序列化器 trait | `NetSerializer` trait 抽象 | - | - | 定义序列化接口 | - | P0 |
| 94 | MessagePack | MessagePack 序列化实现 | - | - | 支持 MessagePack | - | P0 |
| 95 | JSON | JSON 序列化实现 | - | - | 支持 JSON | - | P0 |
| 96 | Protobuf | Protobuf 序列化实现（基于 prost） | - | - | 支持 Protobuf | - | P0 |
| 97 | Bincode | Bincode 序列化实现 | - | - | 支持 Bincode | - | P0 |
| 98 | Cap'n Proto | Cap'n Proto 序列化实现 | - | - | 支持 Cap'n Proto | - | P2 |
| 297 | SerializeFormat | `SerializeFormat::MessagePack` | - | - | 序列化格式枚举 | - | P0 |
| 298 | JSON 格式 | `SerializeFormat::Json` | - | - | JSON 格式 | - | P0 |
| 299 | Protobuf 格式 | `SerializeFormat::Protobuf` | - | - | Protobuf 格式 | - | P0 |
| 300 | Bincode 格式 | `SerializeFormat::Bincode` | - | - | Bincode 格式 | - | P0 |
| 301 | Cap'n Proto 格式 | `SerializeFormat::CapnProto` | - | - | Cap'n Proto 格式 | - | P2 |
| 307 | MessagePack 序列化器 | `MessagePackSerializer::new() -> Self` | - | Self | 创建序列化器 | - | P0 |
| 308 | MessagePack 完成 | `MessagePackSerializer::finish(&mut self) -> Vec<u8>` | - | Vec<u8> | 获取序列化结果 | - | P0 |
| 309 | JSON 序列化器 | `JsonSerializer::to_string(&self) -> String` | - | String | 返回 JSON 字符串 | - | P0 |
| 310 | Protobuf 序列化器 | `ProtobufSerializer`（基于 prost） | - | - | Protobuf 支持 | - | P0 |
| 311 | Bincode 序列化器 | `BincodeSerializer::new() -> Self` | - | Self | 创建 Bincode | - | P0 |
| 312 | Cap'n Proto 序列化器 | `CapnpSerializer`（基于 capnp） | - | - | Cap'n Proto 支持 | - | P2 |
| 313 | 序列化性能 | 序列化性能基准：100KB < 1ms | - | - | 满足性能要求 | - | P1 |
| 314 | 反序列化性能 | 反序列化性能基准：100KB < 1ms | - | - | 满足性能要求 | - | P1 |
| 315 | 嵌套消息 | `NetMessage` 支持嵌套（递归 Message） | - | - | 递归序列化正确 | - | P0 |
| 316 | 集合类型 | `NetMessage` 支持 `Vec<T>` / `HashMap<K, V>` / `Option<T>` | - | - | 集合序列化正确 | - | P0 |
| 317 | 基础类型 | `NetMessage` 支持 `String` / `u8..u64` / `i8..i64` / `f32/f64` / `bool` | - | - | 基础类型正确 | - | P0 |
| 318 | 数学类型 | `NetMessage` 支持 `Vec2/3/4` / `Quat` / `Mat4` | - | - | 数学类型正确 | - | P1 |

### 4. RPC 系统

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 23 | RPC Service trait | `RpcService` trait：`#[rpc]` 宏生成 client stub | - | - | 定义 RPC 接口 | - | P0 |
| 24 | RPC 宏生成 | `#[rpc]` 宏生成 server dispatcher | - | - | 生成服务端调度 | - | P0 |
| 25 | RPC ID 匹配 | RPC 请求 / 响应 ID 匹配 | - | - | 正确匹配请求响应 | - | P0 |
| 26 | RPC 超时重试 | RPC 超时 / 重试机制 | - | - | 支持超时配置 | - | P0 |
| 27 | RPC 异步 | RPC 异步 `Future` 支持 | - | - | 异步调用 | - | P0 |
| 99 | RPC client stub | `#[rpc]` 宏生成 client stub | - | - | 与 23 一致 | - | P0 |
| 100 | RPC server dispatcher | `#[rpc]` 宏生成 server dispatcher | - | - | 与 24 一致 | - | P0 |
| 101 | RPC 请求响应 | RPC 请求 / 响应 ID 匹配 | - | - | 与 25 一致 | - | P0 |
| 102 | RPC 超时重试 | RPC 超时 / 重试机制 | - | - | 与 26 一致 | - | P0 |
| 103 | RPC 异步 Future | RPC 异步 `Future` 支持 | - | - | 与 27 一致 | - | P0 |
| 350 | RPC 属性宏 | `#[rpc]` 属性宏应用在 impl block 上 | - | - | 宏正确应用 | - | P0 |
| 351 | RPC 服务端 | `#[rpc(server)]` 生成服务端 dispatcher | - | - | 生成服务端 | - | P0 |
| 352 | RPC 客户端 | `#[rpc(client)]` 生成客户端 stub | - | - | 生成客户端 | - | P0 |
| 353 | RPC 双向 | `#[rpc(bidirectional)]` 双向 | - | - | 支持双向调用 | - | P0 |
| 354 | RPC 方法标记 | `#[rpc_method]` 标记单个 RPC 方法 | - | - | 方法正确标记 | - | P0 |
| 355 | RPC 单向 | `#[rpc_method(one_way)]` 无需返回 | - | - | 不等待响应 | - | P1 |
| 356 | RPC 超时标记 | `#[rpc_method(timeout = "3s")]` 指定超时 | - | - | 超时生效 | - | P1 |
| 357 | RPC Client 构造 | `RpcClient::new(channel) -> Self` | channel | Self | 创建 RPC 客户端 | - | P0 |
| 358 | RPC Client 方法 | `async fn method_name(&self, args...) -> Result<Ret>` | args | Result<Ret> | 异步调用方法 | - | P0 |
| 359 | RPC Server 调度 | `RpcServer::dispatch(&mut self, msg) -> Result<Option<Vec<u8>>>` | msg | Result | 调度 RPC 调用 | - | P0 |
| 360 | RPC 请求 ID | `RpcRequestId` 全局自增 u64 | - | - | 全局唯一 | - | P0 |
| 361 | RPC 请求结构 | `RpcRequest: { id, method, args }` | - | - | 请求结构正确 | - | P0 |
| 362 | RPC 响应结构 | `RpcResponse: { id, result }` | - | - | 响应结构正确 | - | P0 |
| 363 | RPC 超时错误 | `RpcError::Timeout` | - | - | 超时错误类型 | - | P0 |
| 364 | RPC 未找到错误 | `RpcError::NotFound` | - | - | 方法未找到错误 | - | P0 |
| 365 | RPC 反序列化错误 | `RpcError::Deserialize` | - | - | 反序列化错误 | - | P0 |
| 366 | RPC 序列化错误 | `RpcError::Serialize` | - | - | 序列化错误 | - | P0 |
| 367 | RPC 应用错误 | `RpcError::Application(String)` | - | - | 应用级错误 | - | P0 |
| 368 | RPC 流式支持 | RPC 支持流式 `impl Stream<Item = T>`（SSE 风格） | - | - | 流式响应 | - | P2 |
| 369 | RPC 取消支持 | RPC 支持取消（基于 request_id cancel） | - | - | 支持取消调用 | - | P1 |
| 370 | RPC 批处理 | RPC 支持批处理 `batch(Vec<Request>) -> Vec<Response>` | - | - | 批量调用 | - | P1 |
| 371 | RPC 中间件 | `RpcMiddleware::before/after` | - | - | 中间件接口 | - | P1 |
| 372 | 中间件应用 | `RpcMiddleware` 用于日志 / 鉴权 / 限流 | - | - | 日志、鉴权、限流 | - | P1 |
| 373 | RPC 限流 | `RpcRateLimiter`：qps 限制 | - | - | QPS 限制 | - | P1 |
| 374 | RPC 鉴权 | `RpcAuth`：基于 token 的简单鉴权 | - | - | Token 鉴权 | - | P1 |

### 5. Replication 组件与同步

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 28 | Replication 组件 | `Replication` 组件：标记 entity 需要网络同步 | - | - | 标记组件存在 | - | P0 |
| 29 | NetworkSync 组件 | `NetworkSync`：位置 / 旋转 / 速度同步 | - | - | 同步组件存在 | - | P0 |
| 104 | Replication 组件实现 | `Replication` 组件：标记 entity 需要网络同步 | - | - | 与 28 一致 | - | P0 |
| 105 | NetworkSync 实现 | `NetworkSync`：位置 / 旋转 / 速度同步 | - | - | 与 29 一致 | - | P0 |
| 319 | Replication 空标记 | `Replication` 组件：空标记组件 | - | - | 空组件正确 | - | P0 |
| 320 | NetworkSync 数据 | `NetworkSync` 组件：`last_authority_pos / last_authority_rot / last_authority_vel / tick` | - | - | 数据完整 | - | P0 |
| 321 | NetworkOwner | `NetworkOwner` 组件：`client_id` | - | - | 拥有者组件 | - | P0 |
| 322 | NetworkId | `NetworkId` 组件：`u64` 全局唯一 | - | - | 全局唯一 ID | - | P0 |
| 329 | Replicate 属性 | `NetAttr::Replicate` 全量同步所有字段 | - | - | 全量同步 | - | P0 |
| 330 | OnlyServer 属性 | `NetAttr::OnlyServer` 字段只在服务端存在 | - | - | 服务端私有 | - | P0 |
| 331 | OnlyOwner 属性 | `NetAttr::OnlyOwner` 字段仅向 owner 客户端同步 | - | - | 仅拥有者可见 | - | P0 |
| 332 | OwnerAuto 属性 | `NetAttr::OwnerAuto`：根据 `NetworkOwner` 推断 | - | - | 自动推断 | - | P0 |
| 333 | Predict 属性 | `NetAttr::Predict` 字段启用客户端预测 | - | - | 启用预测 | - | P0 |
| 334 | Interpolate 属性 | `NetAttr::Interpolate` 字段启用客户端插值 | - | - | 启用插值 | - | P0 |

### 6. 客户端预测与插值

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 30 | 客户端插值 | 客户端插值（interpolation） | - | - | 插值平滑 | - | P0 |
| 31 | 客户端预测 | 客户端预测（prediction） | - | - | 预测准确 | - | P0 |
| 32 | 延迟补偿 | 服务端延迟补偿（lag compensation） | - | - | 服务端补偿正确 | - | P0 |
| 106 | 插值实现 | 客户端 interpolation：在两个权威 snapshot 之间线性 / 样条插值 | - | - | 插值平滑 | - | P0 |
| 107 | 预测实现 | 客户端 prediction：根据 last input 预测位置 | - | - | 预测准确 | - | P0 |
| 108 | 延迟补偿实现 | 服务端延迟补偿（lag compensation） | - | - | 与 32 一致 | - | P0 |
| 331 | 线性插值 | `InterpolationBuffer::sample(now) -> state`（线性插值） | now | state | 线性插值 | - | P0 |
| 332 | 预测 buffer | `PredictionBuffer::push(tick, input, state)` | tick, input, state | - | 记录预测状态 | - | P0 |
| 333 | 预测重放 | `PredictionBuffer::replay_from(tick, authoritative_state)` | tick, authoritative_state | - | 从权威状态重放 | - | P0 |
| 334 | 插值 buffer | `InterpolationBuffer::push(tick, state)` | tick, state | - | 记录插值状态 | - | P0 |
| 335 | 插值采样 | `InterpolationBuffer::sample(now) -> state`（线性插值） | now | state | 与 331 一致 | - | P0 |
| 336 | 预测重置 | `Prediction::reset()` 清空预测 buffer | - | - | 清空预测 | - | P0 |

### 7. 服务端 replication 系统

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 325 | 服务端 replication 系统 | 服务端 replication 系统：每固定 tick 生成 snapshot | - | - | 定期生成 snapshot | - | P0 |
| 326 | 差分压缩 | 服务端 delta 压缩：仅发送变化字段 | - | - | 减少带宽 | - | P1 |
| 327 | 客户端校正 | 客户端 reconciliation：权威 snapshot 到达时校正预测 | - | - | 校正预测误差 | - | P0 |
| 337 | 服务端 lag compensation | rewinding 到客户端输入时间做碰撞检测 | - | - | 正确 lag comp | - | P0 |
| 338 | Snapshot 编码 | `Snapshot::encode(&self) -> Vec<u8>` | - | Vec<u8> | 编码 snapshot | - | P0 |
| 339 | Snapshot 解码 | `Snapshot::decode(buf) -> Result<Self>` | buf | Result<Self> | 解码 snapshot | - | P0 |
| 340 | Snapshot tick | `Snapshot` 包含 tick 序号用于去重 | - | - | 包含 tick | - | P0 |
| 341 | Snapshot 时间戳 | `Snapshot` 包含时间戳用于插值 | - | - | 包含 timestamp | - | P0 |

### 8. 网络事件

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 40 | OnSpawn 事件 | `OnSpawn` 网络事件：实体在远端生成 | - | - | 事件触发 | - | P0 |
| 41 | OnDespawn 事件 | `OnDespawn` 网络事件：实体在远端销毁 | - | - | 事件触发 | - | P0 |
| 42 | OnSync 事件 | `OnSync` 网络事件：每次同步数据到达 | - | - | 事件触发 | - | P0 |
| 43 | OnRPC 事件 | `OnRPC` 网络事件：RPC 调用 | - | - | 事件触发 | - | P0 |
| 44 | OnConnect 事件 | `OnConnect` 事件：玩家连接 | - | - | 事件触发 | - | P0 |
| 45 | OnDisconnect 事件 | `OnDisconnect` 事件：玩家断开 | - | - | 事件触发 | - | P0 |
| 322 | OnSpawn 事件签名 | `OnSpawn(client_id, entity, snapshot)` 事件 | - | - | 事件结构正确 | - | P0 |
| 323 | OnDespawn 事件签名 | `OnDespawn(client_id, entity)` 事件 | - | - | 事件结构正确 | - | P0 |
| 324 | OnSync 事件签名 | `OnSync(client_id, entity, snapshot)` 事件 | - | - | 事件结构正确 | - | P0 |
| 325 | OnRpc 事件签名 | `OnRpc(from, to, rpc_id, payload)` 事件 | - | - | 事件结构正确 | - | P0 |
| 326 | OnConnect 事件签名 | `OnConnect(client_id)` 事件 | - | - | 事件结构正确 | - | P0 |
| 327 | OnDisconnect 事件签名 | `OnDisconnect(client_id, reason)` 事件 | - | - | 事件结构正确 | - | P0 |

### 9. NetRole 与 NetAttr

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 33 | NetRole 枚举 | `NetRole` 枚举：`Server / Client / ListenServer / Standalone` | - | - | 枚举定义正确 | - | P0 |
| 34 | NetAttr Replicate | `NetAttr::Replicate` 全量同步 | - | - | 全量同步 | - | P0 |
| 35 | NetAttr OnlyServer | `NetAttr::OnlyServer` 仅服务器同步 | - | - | 仅服务端可见 | - | P0 |
| 36 | NetAttr OnlyOwner | `NetAttr::OnlyOwner` 仅 Owner 同步 | - | - | 仅拥有者可见 | - | P0 |
| 37 | NetAttr OwnerAuto | `NetAttr::OwnerAuto` 自动推断 Owner | - | - | 自动推断 | - | P0 |
| 38 | NetAttr Predict | `NetAttr::Predict` 启用客户端预测 | - | - | 启用预测 | - | P0 |
| 39 | NetAttr Interpolate | `NetAttr::Interpolate` 启用客户端插值 | - | - | 启用插值 | - | P0 |
| 109 | NetRole 枚举实现 | `NetRole::Server / Client / ListenServer / Standalone` | - | - | 与 33 一致 | - | P0 |
| 110 | NetRole 全局访问 | `NetRole::current() -> NetRole` 全局访问 | - | NetRole | 全局获取角色 | - | P1 |
| 111 | NetAttr 解析 | `NetAttr::from_str(s) -> Result<NetAttr>` | s | Result<NetAttr> | 字符串解析 | - | P1 |

### 10. RpcRequestId 与 RpcTimeout

| 需求编号 | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|---------|---------|----------|------|------|----------|------|-------- |
| 214 | RpcRequestId 类型 | `RpcRequestId` 类型 u64 | - | - | u64 类型 | - | P0 |
| 215 | RpcTimeout | `RpcTimeout`（默认 5 秒，可配置） | - | - | 默认 5 秒超时 | - | P0 |

## 验收标准

- RPC 请求-响应 ID 匹配正确
- RPC 超时机制正常工作
- Replication 属性过滤正确
- 客户端预测与插值平滑
- `cargo test -p engine-network` 全部通过

## 依赖关系

```
engine-network
├── engine-core (World, Entity, Component)
├── tokio (async runtime)
└── prost (protobuf)
```

## 优先级说明

- **P0**: 核心功能，必须在 Sprint 内完成
- **P1**: 重要功能，应尽量完成
- **P2**: 优化功能，可延后到下一 Sprint
