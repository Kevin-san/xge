# 安全与加密需求文档

## 模块名称与概述

**模块名称**：engine-security（集成于 engine-network 和 engine-hotfix）

**概述**：安全模块提供网络通信加密、数据完整性验证、反作弊保护以及插件沙盒隔离能力。核心功能包括：AES-GCM/ChaCha20-Poly1305 加密、ECDH 密钥协商、数字签名验证、反作弊机制和权限控制。

---

## 需求清单

### 5.1 网络加密（NetEncryptor）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 59 | `NetEncryptor`：AES-GCM 加密 | `NetEncryptor::aes_gcm_new(key, nonce_gen) -> Self` | 输入：密钥、nonce生成器<br>输出：加密器实例 | 创建成功 | - | P0 |
| 60 | `NetEncryptor`：ChaCha20-Poly1305 加密 | `NetEncryptor::chacha20_new(key) -> Self` | 输入：密钥<br>输出：加密器实例 | 创建成功 | - | P0 |
| 446 | AES-GCM 加密器创建 | `fn aes_gcm_new(key: &[u8; 32], nonce_gen: NonceGenerator) -> Self` | 输入：32字节密钥、nonce生成器<br>输出：实例 | 正确初始化 | - | P0 |
| 447 | ChaCha20-Poly1305 加密器创建 | `fn chacha20_new(key: &[u8; 32]) -> Self` | 输入：32字节密钥<br>输出：实例 | 正确初始化 | - | P0 |
| 448 | 加密方法 | `fn encrypt(&mut self, plain: &[u8]) -> Result<Vec<u8>>` | 输入：明文<br>输出：密文 | 加密成功 | - | P0 |
| 449 | 解密方法 | `fn decrypt(&mut self, cipher: &[u8]) -> Result<Vec<u8>>` | 输入：密文<br>输出：明文 | 解密成功 | - | P0 |
| 450 | 随机 nonce | 使用随机 96-bit nonce 并附在密文前 | - | nonce 唯一且不可预测 | - | P0 |
| 451 | ChaCha20-Poly1305 移动端首选 | - | - | 移动端优先使用 | - | P1 |

### 5.2 密钥协商（KeyAgreement）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 452 | ECDH（x25519）密钥协商 | `KeyAgreement::ecdh_x25519() -> Self` | 输入：无<br>输出：协商器实例 | 创建成功 | - | P0 |
| 453 | TLS 风格握手流程 | `Handshake::client_hello -> server_hello -> key_exchange -> finished` | - | 四阶段握手 | - | P0 |

### 5.3 带宽控制（BandwidthController）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 57 | `BandwidthController`：按连接限速 | `BandwidthController::new(up_limit_bps, down_limit_bps) -> Self` | 输入：上传/下载限速（bps）<br>输出：控制器实例 | 创建成功 | - | P1 |
| 436 | 创建带宽控制器 | `fn new(up_limit_bps: u64, down_limit_bps: u64) -> Self` | 输入：上行/下行限速<br>输出：实例 | 正确初始化 | - | P1 |
| 437 | 更新 token bucket | `fn tick(&mut self, now: Instant)` | 输入：当前时间<br>输出：无 | bucket 正确更新 | - | P1 |
| 438 | 尝试发送 | `fn try_send(&mut self, bytes: usize) -> bool` | 输入：字节数<br>输出：是否允许 | 正确限流 | - | P1 |
| 439 | 每连接限速 | `fn per_connection_limit(&mut self, conn_id: ConnId, bps: u64)` | 输入：连接ID、限速<br>输出：无 | 支持细粒度控制 | - | P1 |

### 5.4 网络压缩（NetCompressor）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 58 | `NetCompressor`：zstd/snappy/lz4 压缩 | `NetCompressor::new(kind) -> Self` | 输入：压缩类型<br>输出：压缩器实例 | 创建成功 | - | P1 |
| 440 | 压缩方法 | `fn compress(&self, data: &[u8]) -> Result<Vec<u8>>` | 输入：原始数据<br>输出：压缩后数据 | 压缩成功 | - | P1 |
| 441 | 解压方法 | `fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>` | 输入：压缩数据<br>输出：原始数据 | 解压成功 | - | P1 |
| 442 | zstd 压缩级别配置 | 压缩级别 1..21 可配置 | - | 级别生效 | - | P1 |
| 443 | snappy 无配置 | - | - | 快速压缩 | - | P1 |
| 444 | lz4 无配置 | - | - | 高压缩率 | - | P1 |
| 445 | 压缩头格式 | `0xC0 0x4D 0x50 0x52` + kind byte | - | 正确识别格式 | - | P1 |

### 5.5 反作弊（AntiCheat）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 69 | `AntiCheat::crc_check()`：关键状态 CRC 校验 | `AntiCheat::crc_compare(client_hash, server_hash) -> bool` | 输入：客户端hash、服务端hash<br>输出：是否匹配 | 正确比较 | - | P0 |
| 70 | `AntiCheat::sign_input()`：关键输入签名 | `AntiCheat::sign_input(input, private_key) -> Signature` | 输入：输入数据、私钥<br>输出：签名 | 签名成功 | - | P0 |
| 71 | 权威服务器：物理与关键状态仅在服务端计算 | - | - | 客户端不执行关键逻辑 | - | P0 |
| 218 | `AntiCheat::state_hash(world) -> u64` | `fn state_hash(world: &World) -> u64` | 输入：World引用<br>输出：哈希值 | 状态变更时哈希变化 | - | P0 |
| 398 | `state_hash` 方法 | `fn state_hash(world: &World) -> u64` | 输入：World<br>输出：u64哈希 | 正确计算 | - | P0 |
| 399 | CRC 比较 | `fn crc_compare(client_hash: u64, server_hash: u64) -> bool` | 输入：两个哈希<br>输出：是否相等 | 正确比较 | - | P0 |
| 400 | 签名输入 | `fn sign_input(input: &[u8], private_key: &PrivateKey) -> Signature` | 输入：输入、私钥<br>输出：签名 | 签名有效 | - | P0 |
| 401 | 验证输入 | `fn verify_input(input: &[u8], signature: &Signature, public_key: &PublicKey) -> bool` | 输入：输入、签名、公钥<br>输出：是否有效 | 验证成功 | - | P0 |
| 402 | 权威服务器：物理模拟仅在服务端 | - | - | 客户端不运行物理 | - | P0 |
| 403 | 权威服务器：关键数值仅服务端写入 | - | - | 血量/金币等仅服务端控制 | - | P0 |
| 404 | 权威服务器：客户端输入仅转发 | - | - | 输入作为事件转发 | - | P0 |
| 405 | `Authority` 资源 | 在 server 角色存在 | - | 服务端独有资源 | - | P0 |

### 5.6 PatchBundle 签名验证

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 80 | `PatchBundle::sign(keypair)`：RSA 签名 | `PatchBundle::sign_rsa(private_key_pem) -> Result<Signature>` | 输入：RSA私钥PEM<br>输出：签名 | 签名成功 | - | P0 |
| 81 | `PatchBundle::sign_ed25519(keypair)`：Ed25519 签名 | `PatchBundle::sign_ed25519(private_key) -> Result<Signature>` | 输入：Ed25519私钥<br>输出：签名 | 签名成功 | - | P0 |
| 82 | `PatchBundle::verify(pubkey) -> bool` | `PatchBundle::verify_rsa(public_key_pem, sig) -> bool` | 输入：公钥、签名<br>输出：是否有效 | 验证成功 | - | P0 |
| 477 | RSA 签名 | `fn sign_rsa(&mut self, private_key_pem: &str) -> Result<Signature>` | 输入：私钥PEM<br>输出：签名 | 正确签名 | - | P0 |
| 478 | Ed25519 签名 | `fn sign_ed25519(&mut self, private_key: &Ed25519PrivateKey) -> Result<Signature>` | 输入：私钥<br>输出：签名 | 正确签名 | - | P0 |
| 479 | RSA 验证 | `fn verify_rsa(&self, public_key_pem: &str, sig: &Signature) -> bool` | 输入：公钥、签名<br>输出：是否有效 | 验证正确 | - | P0 |
| 480 | Ed25519 验证 | `fn verify_ed25519(&self, public_key: &Ed25519PublicKey, sig: &Signature) -> bool` | 输入：公钥、签名<br>输出：是否有效 | 验证正确 | - | P0 |
| 483 | SHA-256 merkle root | PatchBundle 包含 merkle root | - | 完整性校验 | - | P0 |

### 5.7 插件沙盒安全

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 128 | `PluginSandbox`：文件权限 | `fn wrap_file_open(path, mode) -> Result<File>` | 输入：路径、模式<br>输出：文件句柄或错误 | 未授权被拒绝 | - | P0 |
| 129 | `PluginSandbox`：网络权限 | `fn wrap_net_connect(addr) -> Result<TcpStream>` | 输入：地址<br>输出：连接或错误 | 未授权被拒绝 | - | P0 |
| 130 | `PluginSandbox`：内存配额 | `fn wrap_alloc(bytes) -> Result<()>` | 输入：字节数<br>输出：结果 | 超限被拒绝 | - | P0 |
| 131 | `PluginSandbox`：CPU 时间配额 | `fn check_cpu(time) -> bool` | 输入：时间<br>输出：是否允许 | 超限被拒绝 | - | P0 |
| 583 | 创建沙盒 | `fn new(manifest: &PluginManifest) -> Self` | 输入：manifest<br>输出：沙盒实例 | 创建成功 | PluginManifest | P0 |
| 584 | 权限检查 | `fn check(&self, perm: &PluginPermission) -> bool` | 输入：权限<br>输出：是否允许 | 正确判断 | - | P0 |
| 585 | 权限拒绝 | `fn deny(&self, perm: &PluginPermission) -> bool` | 输入：权限<br>输出：是否拒绝 | 正确拒绝 | - | P0 |
| 586 | 文件操作包装 | `fn wrap_file_open(&self, path: &Path, mode: OpenOptions) -> Result<File>` | 输入：路径、选项<br>输出：文件或错误 | 权限检查 | - | P0 |
| 587 | 网络操作包装 | `fn wrap_net_connect(&self, addr: SocketAddr) -> Result<TcpStream>` | 输入：地址<br>输出：连接或错误 | 权限检查 | - | P0 |
| 588 | 内存分配包装 | `fn wrap_alloc(&self, bytes: usize) -> Result<()>` | 输入：字节数<br>输出：结果 | 配额检查 | - | P0 |
| 589 | 所有 I/O 经过 hook | - | - | 未授权操作被拒绝 | - | P0 |

### 5.8 加密配置（ChannelBuilder）

| 需求编号 | 功能描述 | API 签名 | 输入/输出 | 验收标准 | 依赖关系 | 优先级 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 283 | `ChannelBuilder`：配置通道 | `ChannelBuilder::new() -> Self` | 输入：无<br>输出：构建器实例 | 创建成功 | - | P1 |
| 284 | 设置超时 | `ChannelBuilder::timeout(Duration)` | 输入：持续时间<br>输出：构建器 | 配置生效 | - | P1 |
| 285 | 设置加密 | `ChannelBuilder::encryption(Cipher)` | 输入：加密算法<br>输出：构建器 | 配置生效 | - | P0 |
| 286 | `Cipher::Aes256Gcm` | `Cipher::Aes256Gcm` | - | AES-256-GCM | - | P0 |
| 287 | `Cipher::ChaCha20Poly1305` | `Cipher::ChaCha20Poly1305` | - | ChaCha20-Poly1305 | - | P0 |
| 288 | `Cipher::None` | `Cipher::None` | - | 无加密 | - | P2 |
| 289 | `CompressionKind::Zstd(level)` | `CompressionKind::Zstd(u32)` | - | zstd 压缩 | - | P1 |
| 290 | `CompressionKind::Snappy` | `CompressionKind::Snappy` | - | snappy 压缩 | - | P1 |
| 291 | `CompressionKind::Lz4` | `CompressionKind::Lz4` | - | lz4 压缩 | - | P1 |
| 292 | `CompressionKind::None` | `CompressionKind::None` | - | 无压缩 | - | P2 |

---

## 依赖关系总览

```
NetEncryptor
    └── KeyAgreement (密钥协商)

KeyAgreement
    └── Handshake (握手流程)

BandwidthController
    └── NetChannel (应用于通道)

NetCompressor
    └── NetChannel (应用于通道)

AntiCheat
    ├── Authority (服务端资源)
    └── World (状态哈希)

PatchBundle
    ├── DiffEngine (差分数据)
    └── Signature (签名验证)

PluginSandbox
    ├── PluginManifest (权限配置)
    └── PluginQuota (资源配额)

ChannelBuilder
    ├── Cipher (加密配置)
    └── CompressionKind (压缩配置)
```

---

## 优先级分布

| 优先级 | 数量 | 说明 |
| :--- | :--- | :--- |
| P0 | 核心安全 | 加密器、反作弊、签名验证、沙盒权限 |
| P1 | 重要功能 | 带宽控制、压缩、通道配置 |
| P2 | 辅助功能 | 无加密/无压缩选项 |

---

## 安全边界

### 加密边界
- 所有网络传输数据必须经过加密
- 密钥通过安全通道协商（ECDH x25519）
- 使用 AEAD 模式（AES-GCM / ChaCha20-Poly1305）提供机密性和完整性

### 权限边界
- 插件默认无任何权限
- 权限需在 manifest.toml 中显式声明
- 沙盒 hook 所有 I/O 操作，未授权直接拒绝

### 反作弊边界
- 关键状态仅服务端计算
- 客户端输入必须签名
- 定期状态哈希校验

### 完整性边界
- PatchBundle 使用数字签名验证
- 使用 SHA-256 merkle root 确保完整性
- 下载文件校验 SHA-256