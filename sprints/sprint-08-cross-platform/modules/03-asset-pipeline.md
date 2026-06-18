# 模块三：资源处理管线（Asset Pipeline）

## 1. 模块概述

资源处理管线是游戏引擎中负责资源扫描、导入、处理、打包的核心模块。它支持纹理、音频、模型、场景等各类资源的处理，提供增量构建和差分更新能力。

### 核心职责

- 扫描资源目录，发现所有资源文件
- 导入资源，生成中间格式
- 处理资源（压缩、加密、合批）
- 打包资源生成资源清单
- 支持增量构建和差分更新

### 需求来源

对应原文档需求编号：**98-114, 241-330**

---

## 2. AssetPipeline 核心接口

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-071 | `AssetPipeline::new(asset_dir)` 创建资源管线 |
| REQ-072 | `AssetPipeline::scan(&mut self)` 扫描资源目录 |
| REQ-073 | `AssetPipeline::import_all(&mut self)` 导入所有资源 |
| REQ-074 | `AssetPipeline::reimport_changed(&mut self)` 增量导入（基于 mtime 或 hash） |
| REQ-075 | `AssetPipeline::process_all(&mut self)` 合批/压缩/加密处理 |
| REQ-076 | `AssetPipeline::package(&self, out_dir) -> Result<PathBuf>` 打包资源 |
| REQ-077 | `AssetPipeline::build_manifest(&self) -> AssetManifest` 生成资源清单 |
| REQ-078 | `AssetPipeline::incremental_hash(&self) -> String` 增量构建哈希 |
| REQ-079 | `AssetPipeline::diff(from_manifest, to_manifest) -> DiffResult` 资源差异计算 |
| REQ-241 | `AssetPipeline::new(asset_dir)` 创建资源管线（重复） |
| REQ-242 | `AssetPipeline::scan(&mut self)` 扫描资源（重复） |
| REQ-243 | `AssetPipeline::import(&mut self)` 导入资源 |
| REQ-244 | `AssetPipeline::process(&mut self)` 处理资源 |
| REQ-245 | `AssetPipeline::package(&mut self, out) -> Result<PathBuf>` 打包资源 |
| REQ-246 | `AssetPipeline::build_manifest(&self) -> AssetManifest` 生成清单 |
| REQ-247 | `AssetPipeline::changed_files(&self, since: SystemTime) -> Vec<PathBuf>` 获取变更文件 |
| REQ-248 | `AssetPipeline::incremental_build(&mut self, since) -> Result<AssetManifest>` 增量构建 |

### API 签名

```rust
pub struct AssetPipeline {
    asset_dir: PathBuf,
    entries: Vec<AssetEntry>,
    cache: BuildCache,
}

impl AssetPipeline {
    pub fn new(asset_dir: impl AsRef<Path>) -> Self;
    pub fn scan(&mut self) -> Result<()>;
    pub fn import_all(&mut self) -> Result<()>;
    pub fn import(&mut self) -> Result<()>;
    pub fn reimport_changed(&mut self) -> Result<()>;
    pub fn process_all(&mut self) -> Result<()>;
    pub fn process(&mut self) -> Result<()>;
    pub fn package(&mut self, out_dir: impl AsRef<Path>) -> Result<PathBuf>;
    pub fn build_manifest(&self) -> AssetManifest;
    pub fn incremental_hash(&self) -> String;
    pub fn diff(&self, from_manifest: &AssetManifest, to_manifest: &AssetManifest) -> DiffResult;
    pub fn changed_files(&self, since: SystemTime) -> Vec<PathBuf>;
    pub fn incremental_build(&mut self, since: SystemTime) -> Result<AssetManifest>;
}
```

### 输入

- 资源目录路径

### 输出

- 打包后的资源包路径
- 资源清单文件

### 验收标准

- [ ] 正确扫描资源目录
- [ ] 正确导入所有资源
- [ ] 增量导入仅处理变更文件
- [ ] 正确打包资源

### 依赖关系

- 依赖 `TextureProcessor`
- 依赖 `AudioProcessor`
- 依赖 `ModelProcessor`
- 依赖 `Compress` / `Encrypt`

### 优先级

**P0**

---

## 3. AssetManifest 资源清单

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-080 | `AssetManifest::to_json(&self) -> String` 序列化为 JSON |
| REQ-081 | `AssetManifest::load_json(path) -> Result<Self>` 从 JSON 加载 |
| REQ-082 | `AssetManifest::entries(&self) -> &[AssetEntry]` 获取资源条目列表 |
| REQ-083 | `AssetEntry::path / hash / size / kind` 资源条目字段 |
| REQ-249 | `AssetManifest::new() -> Self` 创建清单 |
| REQ-250 | `AssetManifest::add(&mut self, entry)` 添加资源条目 |
| REQ-251 | `AssetManifest::entries(&self) -> &[AssetEntry]` 获取条目（重复） |
| REQ-252 | `AssetManifest::to_json(&self) -> String` 序列化（重复） |
| REQ-253 | `AssetManifest::from_json(json) -> Result<Self>` 反序列化 |
| REQ-254 | `AssetManifest::save(&self, path) -> Result<()>` 保存清单 |
| REQ-255 | `AssetManifest::load(path) -> Result<Self>` 加载清单 |
| REQ-256 | `AssetManifest::diff(&self, other) -> DiffResult` 计算差异 |
| REQ-257 | `AssetEntry::path(&self) -> &Path` 获取路径 |
| REQ-258 | `AssetEntry::hash(&self) -> &str` 获取哈希 |
| REQ-259 | `AssetEntry::size(&self) -> u64` 获取大小 |
| REQ-260 | `AssetEntry::kind(&self) -> AssetKind` 获取类型 |

### API 签名

```rust
#[derive(Debug, Clone)]
pub struct AssetManifest {
    version: String,
    entries: Vec<AssetEntry>,
}

#[derive(Debug, Clone)]
pub struct AssetEntry {
    pub path: PathBuf,
    pub hash: String,
    pub size: u64,
    pub kind: AssetKind,
    pub dependencies: Vec<PathBuf>,
}

impl AssetManifest {
    pub fn new() -> Self;
    pub fn add(&mut self, entry: AssetEntry);
    pub fn entries(&self) -> &[AssetEntry];
    pub fn to_json(&self) -> String;
    pub fn from_json(json: &str) -> Result<Self>;
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()>;
    pub fn load(path: impl AsRef<Path>) -> Result<Self>;
    pub fn diff(&self, other: &AssetManifest) -> DiffResult;
}

impl AssetEntry {
    pub fn path(&self) -> &Path;
    pub fn hash(&self) -> &str;
    pub fn size(&self) -> u64;
    pub fn kind(&self) -> AssetKind;
}
```

### AssetManifest JSON 格式

```json
{
  "version": "1.0.0",
  "entries": [
    {
      "path": "textures/player.png",
      "hash": "a1b2c3d4...",
      "size": 102400,
      "kind": "Texture",
      "dependencies": []
    }
  ]
}
```

### 输入

- 资源处理结果

### 输出

- JSON 格式资源清单

### 验收标准

- [ ] JSON 序列化/反序列化往返正确
- [ ] 资源条目信息完整
- [ ] 差异计算正确

### 依赖关系

- 依赖 `serde_json`

### 优先级

**P0**

---

## 4. AssetKind 资源类型

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-084 | `AssetKind::Texture / Audio / Model / Scene / Prefab / Font / Custom` 资源类型枚举 |
| REQ-261 | `AssetKind::Texture / Audio / Model / Scene / Prefab / Font / Custom(&str)` 资源类型枚举（重复） |

### API 签名

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetKind {
    Texture,
    Audio,
    Model,
    Scene,
    Prefab,
    Font,
    Custom(String),
}

impl AssetKind {
    pub fn as_str(&self) -> &str;
}
```

### 资源类型对照表

| AssetKind | 文件扩展名示例 |
|------------|---------------|
| Texture | .png, .jpg, .tga, .bmp |
| Audio | .wav, .ogg, .mp3, .flac |
| Model | .glb, .gltf, .obj, .fbx |
| Scene | .scene, .json |
| Prefab | .prefab, .json |
| Font | .ttf, .otf, .fnt |
| Custom | 其他自定义格式 |

### 输入

- 无（枚举类型）

### 输出

- 资源类型字符串

### 验收标准

- [ ] 枚举变体完整
- [ ] Custom 类型支持自定义名称

### 依赖关系

- 无

### 优先级

**P0**

---

## 5. AssetCompress 压缩算法

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-085 | `AssetCompress::Zstd / Gzip / Brotli / LZ4 / None` 压缩算法枚举 |
| REQ-318 | `Compress::zstd(bytes, level) -> Result<Vec<u8>>` Zstd 压缩 |
| REQ-319 | `Compress::gzip(bytes, level) -> Result<Vec<u8>>` Gzip 压缩 |
| REQ-320 | `Compress::brotli(bytes, level) -> Result<Vec<u8>>` Brotli 压缩 |
| REQ-321 | `Compress::lz4(bytes) -> Result<Vec<u8>>` LZ4 压缩 |
| REQ-322 | `Compress::decompress(bytes, algo) -> Result<Vec<u8>>` 解压缩 |

### API 签名

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AssetCompress {
    #[default]
    None,
    Zstd,
    Gzip,
    Brotli,
    LZ4,
}

pub struct Compress;

impl Compress {
    pub fn zstd(bytes: &[u8], level: i32) -> Result<Vec<u8>>;
    pub fn gzip(bytes: &[u8], level: i32) -> Result<Vec<u8>>;
    pub fn brotli(bytes: &[u8], level: u32) -> Result<Vec<u8>>;
    pub fn lz4(bytes: &[u8]) -> Result<Vec<u8>>;
    pub fn decompress(bytes: &[u8], algo: AssetCompress) -> Result<Vec<u8>>;
}
```

### 压缩算法特性

| 算法 | 压缩率 | 速度 | 典型使用场景 |
|------|--------|------|-------------|
| Zstd | 高 | 中 | 资源包 |
| Gzip | 中 | 中 | Web 传输 |
| Brotli | 最高 | 慢 | Web 静态资源 |
| LZ4 | 低 | 最快 | 运行时加载 |
| None | 无 | 无 | 开发调试 |

### 压缩级别对照

| 算法 | 最低 | 默认 | 最高 |
|------|------|------|------|
| Zstd | 1 | 3 | 22 |
| Gzip | 1 | 6 | 9 |
| Brotli | 0 | 11 | 11 |
| LZ4 | - | - | - |

### 输入

- 原始字节数据
- 压缩级别

### 输出

- 压缩后字节数据

### 验收标准

- [ ] Zstd 压缩/解压缩正确
- [ ] Gzip 压缩/解压缩正确
- [ ] Brotli 压缩/解压缩正确
- [ ] LZ4 压缩/解压缩正确

### 依赖关系

- 依赖 `zstd` crate
- 依赖 `flate2` crate
- 依赖 `brotli` crate
- 依赖 `lz4` crate

### 优先级

**P1**

---

## 6. AssetEncrypt 加密算法

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-086 | `AssetEncrypt::AesGcm128 / AesGcm256 / XorChaCha20 / None` 加密算法枚举 |
| REQ-323 | `Encrypt::aes_gcm_128(bytes, key, nonce) -> Result<Vec<u8>>` AES-GCM 128 加密 |
| REQ-324 | `Encrypt::aes_gcm_256(bytes, key, nonce) -> Result<Vec<u8>>` AES-GCM 256 加密 |
| REQ-325 | `Encrypt::chacha20(bytes, key, nonce) -> Result<Vec<u8>>` ChaCha20 加密 |
| REQ-326 | `Encrypt::decrypt(bytes, key, nonce, algo) -> Result<Vec<u8>>` 解密 |

### API 签名

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AssetEncrypt {
    #[default]
    None,
    AesGcm128,
    AesGcm256,
    XorChaCha20,
}

pub struct Encrypt;

impl Encrypt {
    pub fn aes_gcm_128(bytes: &[u8], key: &[u8; 16], nonce: &[u8; 12]) -> Result<Vec<u8>>;
    pub fn aes_gcm_256(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Result<Vec<u8>>;
    pub fn chacha20(bytes: &[u8], key: &[u8; 32], nonce: &[u8; 24]) -> Result<Vec<u8>>;
    pub fn decrypt(bytes: &[u8], key: &[u8], nonce: &[u8], algo: AssetEncrypt) -> Result<Vec<u8>>;
}
```

### 加密算法特性

| 算法 | 密钥长度 | Nonce 长度 | 安全性 |
|------|----------|------------|--------|
| AesGcm128 | 16 字节 | 12 字节 | 高 |
| AesGcm256 | 32 字节 | 12 字节 | 最高 |
| XorChaCha20 | 32 字节 | 24 字节 | 高 |

### 密钥管理

- 开发期：密钥可外部配置或关闭加密
- 发布期：默认开启加密，密钥内嵌二进制或外部配置

### 输入

- 原始字节数据
- 密钥
- Nonce

### 输出

- 加密后字节数据

### 验收标准

- [ ] AES-GCM-128 加密/解密正确
- [ ] AES-GCM-256 加密/解密正确
- [ ] ChaCha20 加密/解密正确
- [ ] 开发期可关闭加密

### 依赖关系

- 依赖 `aes-gcm` crate
- 依赖 `chacha20poly1305` crate

### 优先级

**P1**

---

## 7. TextureProcessor 纹理处理器

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-087 | `TextureProcessor::compress_bc(img) -> Result<Vec<u8>>` BC 压缩（DXT） |
| REQ-088 | `TextureProcessor::compress_etc(img) -> Result<Vec<u8>>` ETC 压缩 |
| REQ-089 | `TextureProcessor::mipmap(img) -> Result<Vec<u8>>` 生成 mipmap |
| REQ-090 | `TextureProcessor::resize(img, size) -> Result<Image>` 调整大小 |
| REQ-091 | `TextureProcessor::pack_atlas(images, padding) -> Result<(Image, Vec<Rect>)>` 图集合批 |
| REQ-263 | `TextureProcessor::compress_etc(img) -> Result<Vec<u8>>` ETC 压缩（重复） |
| REQ-264 | `TextureProcessor::compress_astc(img) -> Result<Vec<u8>>` ASTC 压缩（可选） |
| REQ-265 | `TextureProcessor::generate_mipmaps(img) -> Result<Vec<u8>>` 生成 mipmap |
| REQ-266 | `TextureProcessor::resize(img, w, h) -> Result<Image>` 调整大小 |
| REQ-267 | `TextureProcessor::pack_atlas(images, padding) -> Result<(Image, Vec<Rect>)>` 图集合批 |

### API 签名

```rust
pub struct TextureProcessor;

impl TextureProcessor {
    pub fn compress_bc(img: &Image) -> Result<Vec<u8>>;
    pub fn compress_etc(img: &Image) -> Result<Vec<u8>>;
    pub fn compress_astc(img: &Image) -> Result<Vec<u8>>;
    pub fn mipmap(img: &Image) -> Result<Vec<u8>>;
    pub fn generate_mipmaps(img: &Image) -> Result<Vec<u8>>;
    pub fn resize(img: &Image, w: u32, h: u32) -> Result<Image>;
    pub fn pack_atlas(images: &[Image], padding: u32) -> Result<(Image, Vec<Rect>)>;
}
```

### 纹理压缩格式适用平台

| 格式 | 平台 | 扩展名 |
|------|------|--------|
| BC (DXT) | Windows | .dds |
| ETC | Android (旧) | .ktx |
| ASTC | Android (新), iOS | .ktx2 |
| ETC2 | 跨平台 | .ktx2 |

### 输入

- 原始图像数据

### 输出

- 压缩后图像数据

### 验收标准

- [ ] BC 压缩正确生成
- [ ] ETC 压缩正确生成
- [ ] ASTC 压缩正确生成（可选）
- [ ] Mipmap 正确生成
- [ ] 图集合批正确

### 依赖关系

- 依赖 `image` crate
- 依赖 `-basis-universal`（BC）
- 依赖 `astc-encoder`（ASTC，可选）

### 优先级

**P1**

---

## 8. AudioProcessor 音频处理器

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-092 | `AudioProcessor::convert(src, format) -> Result<Vec<u8>>` 格式转换 |
| REQ-093 | `AudioProcessor::compress_ogg(src) -> Result<Vec<u8>>` OGG 压缩 |
| REQ-094 | `AudioProcessor::compress_mp3(src) -> Result<Vec<u8>>` MP3 压缩 |
| REQ-269 | `AudioProcessor::to_ogg(src) -> Result<Vec<u8>>` 转换为 OGG |
| REQ-270 | `AudioProcessor::to_mp3(src) -> Result<Vec<u8>>` 转换为 MP3 |
| REQ-271 | `AudioProcessor::to_wav(src) -> Result<Vec<u8>>` 转换为 WAV |
| REQ-272 | `AudioProcessor::to_flac(src) -> Result<Vec<u8>>` 转换为 FLAC |

### API 签名

```rust
pub struct AudioProcessor;

impl AudioProcessor {
    pub fn convert(src: &[u8], format: AudioFormat) -> Result<Vec<u8>>;
    pub fn compress_ogg(src: &[u8]) -> Result<Vec<u8>>;
    pub fn compress_mp3(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_ogg(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_mp3(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_wav(src: &[u8]) -> Result<Vec<u8>>;
    pub fn to_flac(src: &[u8]) -> Result<Vec<u8>>;
}
```

### 音频格式特性

| 格式 | 压缩率 | 质量 | 适用场景 |
|------|--------|------|---------|
| WAV | 无 | 最高 | 原始音频 |
| MP3 | 中 | 高 | 音乐 |
| OGG | 高 | 高 | 音效 |
| FLAC | 中 | 无损 | 音乐存档 |

### 输入

- 原始音频数据
- 目标格式

### 输出

- 转换后音频数据

### 验收标准

- [ ] OGG 压缩正确
- [ ] MP3 压缩正确
- [ ] WAV 转换正确
- [ ] FLAC 转换正确

### 依赖关系

- 依赖 `symphonia` crate
- 依赖外部编解码器

### 优先级

**P2**

---

## 9. ModelProcessor 模型处理器

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-095 | `ModelProcessor::optimize(src) -> Result<Vec<u8>>` 模型优化 |
| REQ-123 | `SceneProcessor::bake(scene) -> Result<Vec<u8>>` 场景烘焙 |
| REQ-273 | `ModelProcessor::glb_to_optimized(src) -> Result<Vec<u8>>` GLB 优化 |
| REQ-274 | `ModelProcessor::gltf_to_optimized(src_dir) -> Result<Vec<u8>>` GLTF 优化 |
| REQ-275 | `SceneProcessor::bake(scene) -> Result<Vec<u8>>` 场景烘焙（重复） |

### API 签名

```rust
pub struct ModelProcessor;

impl ModelProcessor {
    pub fn optimize(src: &[u8]) -> Result<Vec<u8>>;
    pub fn glb_to_optimized(src: &[u8]) -> Result<Vec<u8>>;
    pub fn gltf_to_optimized(src_dir: impl AsRef<Path>) -> Result<Vec<u8>>;
}

pub struct SceneProcessor;

impl SceneProcessor {
    pub fn bake(scene: &Scene) -> Result<Vec<u8>>;
}
```

### 模型优化操作

- 顶点数据重排
- 索引优化
- 去除重复纹理
- LOD 生成

### 输入

- 原始模型数据（GLB/GLTF）

### 输出

- 优化后模型数据

### 验收标准

- [ ] GLB 优化正确
- [ ] GLTF 优化正确
- [ ] 场景烘焙正确

### 依赖关系

- 依赖 `gltf` crate
- 依赖 `FBX`/`OBJ` 解析库

### 优先级

**P2**

---

## 10. DiffResult 差异结果

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-256 | `DiffResult::added / modified / removed` 差异类型 |
| REQ-304 | `DiffResult::added / modified / removed` 差异类型（重复） |

### API 签名

```rust
#[derive(Debug, Clone)]
pub struct DiffResult {
    pub added: Vec<AssetEntry>,
    pub modified: Vec<AssetEntry>,
    pub removed: Vec<PathBuf>,
}

impl DiffResult {
    pub fn is_empty(&self) -> bool;
    pub fn total_changes(&self) -> usize;
}
```

### 输入

- 旧资源清单
- 新资源清单

### 输出

- 差异结果（新增/修改/删除列表）

### 验收标准

- [ ] 正确识别新增资源
- [ ] 正确识别修改资源
- [ ] 正确识别删除资源

### 依赖关系

- 依赖 `AssetManifest`

### 优先级

**P0**

---

## 11. Hash 哈希计算

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-285 | `Hash::sha256(bytes) -> String` SHA256 哈希 |
| REQ-286 | `Hash::hash_file(path) -> Result<String>` 文件哈希 |
| REQ-287 | `Hash::hash_dir(path) -> Result<String>` 目录哈希 |

### API 签名

```rust
pub struct Hash;

impl Hash {
    pub fn sha256(bytes: &[u8]) -> String;
    pub fn hash_file(path: impl AsRef<Path>) -> Result<String>;
    pub fn hash_dir(path: impl AsRef<Path>) -> Result<String>;
}
```

### 输入

- 字节数据或文件/目录路径

### 输出

- SHA256 哈希字符串（十六进制）

### 验收标准

- [ ] SHA256 计算正确
- [ ] 文件哈希计算正确
- [ ] 目录哈希计算正确

### 依赖关系

- 依赖 `sha2` crate

### 优先级

**P0**

---

## 12. BuildCache 构建缓存

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-143 | `BuildCache` 缓存编译产物加速增量构建 |
| REQ-144 | `BuildCache::hash(key) -> String` 计算缓存键哈希 |
| REQ-145 | `BuildCache::get(&self, key) -> Option<PathBuf>` 获取缓存 |
| REQ-146 | `BuildCache::put(&mut self, key, path)` 存储缓存 |
| REQ-147 | `BuildCache::clean(&mut self)` 清理缓存 |

### API 签名

```rust
pub struct BuildCache {
    cache_dir: PathBuf,
    index: HashMap<String, PathBuf>,
}

impl BuildCache {
    pub fn new(cache_dir: impl AsRef<Path>) -> Result<Self>;
    pub fn hash(&self, key: &str) -> String;
    pub fn get(&self, key: &str) -> Option<PathBuf>;
    pub fn put(&mut self, key: &str, path: impl AsRef<Path>);
    pub fn clean(&mut self);
}
```

### 缓存键格式

```
hash(config + source_files + assets + timestamp)
```

### 输入

- 缓存键
- 产物路径

### 输出

- 缓存产物路径（若命中）

### 验收标准

- [ ] 缓存命中返回正确路径
- [ ] 缓存未命中返回 None
- [ ] 缓存正确存储
- [ ] 缓存正确清理

### 依赖关系

- 依赖文件系统

### 优先级

**P1**

---

## 13. 资源清单与运行时

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-139 | 资源清单 `assets.manifest`：包含所有资源路径、hash、大小、依赖 |
| REQ-140 | 资源清单可在运行时读取，用于加载校验与差分更新 |
| REQ-141 | 资源加密：开发期可选关闭，发布期默认开启 |
| REQ-142 | 资源解密密钥：内嵌于二进制，可外部配置 |

### 资源清单文件

- 文件名：`assets.manifest`
- 格式：JSON
- 位置：资源包根目录

### 运行时加载流程

1. 读取 `assets.manifest`
2. 验证资源 hash
3. 按需加载资源
4. 支持差分更新

### 输入

- 资源处理结果

### 输出

- `assets.manifest` 文件

### 验收标准

- [ ] 清单包含所有资源
- [ ] 清单包含 hash 和大小
- [ ] 清单包含依赖关系
- [ ] 运行时可正确读取

### 依赖关系

- 依赖 `AssetManifest`

### 优先级

**P0**

---

## 14. 优先级汇总

| 优先级 | 需求编号 | 模块 |
|-------|---------|------|
| P0 | REQ-071~084, REQ-241~260 | AssetPipeline, AssetManifest, AssetKind, Hash |
| P1 | REQ-085~086, REQ-143~147, REQ-263~272 | Compress, Encrypt, BuildCache, Audio |
| P2 | REQ-087~094, REQ-273~275 | Texture, Model, Scene |

---

## 15. 依赖关系图

```
AssetPipeline
├── TextureProcessor
│   └── image crate
├── AudioProcessor
│   └── symphonia crate
├── ModelProcessor
│   └── gltf crate
├── SceneProcessor
├── Compress
│   ├── zstd
│   ├── flate2
│   ├── brotli
│   └── lz4
├── Encrypt
│   ├── aes-gcm
│   └── chacha20poly1305
├── Hash
│   └── sha2
└── BuildCache
    └──文件系统
```

---

## 16. 验收清单

- [ ] `AssetPipeline::scan()` 正确扫描所有资源
- [ ] `AssetPipeline::package()` 正确打包资源
- [ ] `AssetManifest` JSON 往返正确
- [ ] `DiffResult` 正确识别新增/修改/删除
- [ ] `BuildCache` 缓存命中正确
- [ ] 资源加密/解密正确
- [ ] 资源压缩/解压缩正确
