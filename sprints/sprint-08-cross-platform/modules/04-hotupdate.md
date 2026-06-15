# 模块四：热更新（HotUpdate）

## 1. 模块概述

热更新模块提供差分更新能力，支持增量下载和更新游戏资源，无需完整重新下载客户端。该模块通过比较新旧资源清单生成差分补丁，实现高效的增量更新。

### 核心职责

- 计算新旧资源清单的差异
- 生成热更新补丁
- 应用补丁更新到当前版本
- 支持文件级别的增量更新

### 需求来源

对应原文档需求编号：**103-106, 131-133, 300-356**

---

## 2. HotUpdate 核心接口

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-103 | `HotUpdate::diff(old_manifest, new_manifest) -> HotUpdatePatch` 计算差异生成补丁 |
| REQ-104 | `HotUpdate::apply(current_dir, patch) -> Result<()>` 应用补丁 |
| REQ-300 | `HotUpdate::diff(old, new) -> HotUpdatePatch` 计算差异（重复） |
| REQ-301 | `HotUpdate::apply(dir, patch) -> Result<()>` 应用补丁（重复） |

### API 签名

```rust
pub struct HotUpdate;

impl HotUpdate {
    pub fn diff(old_manifest: &AssetManifest, new_manifest: &AssetManifest) -> HotUpdatePatch;
    pub fn apply(current_dir: impl AsRef<Path>, patch: &HotUpdatePatch) -> Result<()>;
}
```

### 输入

- 旧版本资源清单
- 新版本资源清单

### 输出

- `HotUpdatePatch` 热更新补丁

### 验收标准

- [ ] 正确计算清单差异
- [ ] 正确生成补丁
- [ ] 正确应用补丁
- [ ] 补丁可序列化/反序列化

### 依赖关系

- 依赖 `AssetManifest`
- 依赖 `HotUpdatePatch`
- 依赖 `FileChange`

### 优先级

**P0**

---

## 3. HotUpdatePatch 热更新补丁

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-105 | `HotUpdatePatch::version / new_manifest / file_changes / size_bytes` 补丁字段 |
| REQ-302 | `HotUpdatePatch::version(&self) -> &str` 获取版本 |
| REQ-303 | `HotUpdatePatch::new_manifest(&self) -> &AssetManifest` 获取新清单 |
| REQ-304 | `HotUpdatePatch::changes(&self) -> &[FileChange]` 获取变更列表 |
| REQ-305 | `HotUpdatePatch::size_bytes(&self) -> u64` 获取补丁大小 |
| REQ-306 | `HotUpdatePatch::to_bytes(&self) -> Result<Vec<u8>>` 序列化为字节 |
| REQ-307 | `HotUpdatePatch::from_bytes(bytes) -> Result<Self>` 从字节反序列化 |

### API 签名

```rust
#[derive(Debug, Clone)]
pub struct HotUpdatePatch {
    pub version: String,
    pub new_manifest: AssetManifest,
    pub file_changes: Vec<FileChange>,
    pub size_bytes: u64,
}

impl HotUpdatePatch {
    pub fn new(version: String, new_manifest: AssetManifest, file_changes: Vec<FileChange>) -> Self;
    pub fn version(&self) -> &str;
    pub fn new_manifest(&self) -> &AssetManifest;
    pub fn changes(&self) -> &[FileChange];
    pub fn size_bytes(&self) -> u64;
    pub fn to_bytes(&self) -> Result<Vec<u8>>;
    pub fn from_bytes(bytes: &[u8]) -> Result<Self>;
}
```

### HotUpdatePatch JSON 格式

```json
{
  "version": "1.1.0",
  "new_manifest": { ... },
  "file_changes": [
    { "type": "Added", "path": "textures/new.png", "size": 10240, "hash": "..." },
    { "type": "Modified", "path": "textures/player.png", "size": 8192, "hash": "...", "diff": "..." },
    { "type": "Removed", "path": "textures/old.png" }
  ],
  "size_bytes": 18432
}
```

### 输入

- 版本号
- 新资源清单
- 文件变更列表

### 输出

- 可序列化的补丁数据

### 验收标准

- [ ] 版本信息正确
- [ ] 新清单正确包含
- [ ] 变更列表完整
- [ ] 大小计算正确
- [ ] 可序列化/反序列化往返

### 依赖关系

- 依赖 `AssetManifest`
- 依赖 `FileChange`
- 依赖 `serde`

### 优先级

**P0**

---

## 4. FileChange 文件变更

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-106 | `FileChange::Added(path, size, hash) / Modified(path, diff, size) / Removed(path)` 变更类型 |
| REQ-308 | `FileChange::Added / Modified / Removed` 变更类型（重复） |
| REQ-309 | `FileChange::path(&self) -> &Path` 获取文件路径 |
| REQ-310 | `FileChange::size(&self) -> u64` 获取文件大小 |

### API 签名

```rust
#[derive(Debug, Clone)]
pub enum FileChange {
    Added {
        path: PathBuf,
        size: u64,
        hash: String,
    },
    Modified {
        path: PathBuf,
        diff: Vec<u8>,
        size: u64,
    },
    Removed {
        path: PathBuf,
    },
}

impl FileChange {
    pub fn path(&self) -> &Path;
    pub fn size(&self) -> u64;
    pub fn is_added(&self) -> bool;
    pub fn is_modified(&self) -> bool;
    pub fn is_removed(&self) -> bool;
}
```

### FileChange 变体说明

| 变体 | 说明 | 字段 |
|------|------|------|
| Added | 新增文件 | path, size, hash |
| Modified | 修改文件 | path, diff, size |
| Removed | 删除文件 | path |

### Modified 的 diff 格式

- 增量差分（bsdiff/bspatch）
- 或完整新文件内容（简化实现）

### 输入

- 变更类型和相关信息

### 输出

- 变更信息

### 验收标准

- [ ] Added 变体正确
- [ ] Modified 变体正确
- [ ] Removed 变体正确
- [ ] 路径和大小获取正确

### 依赖关系

- 无

### 优先级

**P0**

---

## 5. 差分算法

### 增量差分 vs 完整下载

| 方式 | 优点 | 缺点 | 适用场景 |
|------|------|------|---------|
| 增量差分 | 流量省 | 需要额外计算 | 小更新 |
| 完整下载 | 简单 | 流量大 | 大更新 |

### 简化实现

本 Sprint 采用简化实现：Modified 变体存储完整新文件内容，而非差分。

后续 Sprint 可升级为 bsdiff/bspatch 增量差分。

### DiffResult 与 FileChange 映射

```rust
impl DiffResult {
    fn into_file_changes(self) -> Vec<FileChange> {
        let mut changes = Vec::new();
        
        for entry in self.added {
            changes.push(FileChange::Added {
                path: entry.path,
                size: entry.size,
                hash: entry.hash,
            });
        }
        
        for entry in self.modified {
            changes.push(FileChange::Modified {
                path: entry.path,
                diff: vec![],  // 简化：存储完整内容
                size: entry.size,
            });
        }
        
        for path in self.removed {
            changes.push(FileChange::Removed { path });
        }
        
        changes
    }
}
```

### 输入

- `DiffResult` 差异结果

### 输出

- `Vec<FileChange>` 文件变更列表

### 验收标准

- [ ] Added 正确映射
- [ ] Modified 正确映射
- [ ] Removed 正确映射

### 依赖关系

- 依赖 `DiffResult`
- 依赖 `AssetEntry`

### 优先级

**P0**

---

## 6. 补丁应用流程

### 热更新流程

```
1. 检测更新
   └── 比对当前版本与远程版本

2. 下载补丁
   └── 下载 HotUpdatePatch

3. 应用补丁
   ├── 新增文件：下载并写入
   ├── 修改文件：下载并覆盖
   └── 删除文件：删除文件

4. 更新清单
   └── 用 new_manifest 替换当前清单

5. 重启应用
   └── 加载新资源
```

### 补丁应用实现

```rust
impl HotUpdate {
    pub fn apply(current_dir: impl AsRef<Path>, patch: &HotUpdatePatch) -> Result<()> {
        let dir = current_dir.as_ref();
        
        for change in &patch.file_changes {
            match change {
                FileChange::Added { path, size, hash } => {
                    // 下载新文件并验证
                    let dest = dir.join(path);
                    // ...
                }
                FileChange::Modified { path, diff, size } => {
                    // 应用修改
                    let dest = dir.join(path);
                    // ...
                }
                FileChange::Removed { path } => {
                    // 删除文件
                    let dest = dir.join(path);
                    if dest.exists() {
                        std::fs::remove_file(dest)?;
                    }
                }
            }
        }
        
        // 更新清单
        patch.new_manifest().save(dir.join("assets.manifest"))?;
        
        Ok(())
    }
}
```

### 输入

- 当前资源目录
- 热更新补丁

### 输出

- 无（修改文件系统）

### 验收标准

- [ ] 新增文件正确创建
- [ ] 修改文件正确更新
- [ ] 删除文件正确移除
- [ ] 清单正确更新

### 依赖关系

- 依赖文件系统
- 依赖网络下载（外部实现）

### 优先级

**P0**

---

## 7. 版本管理

### 版本号格式

- 语义化版本：`MAJOR.MINOR.PATCH`
- 示例：`1.0.0` -> `1.1.0`

### 版本比较

```rust
pub fn needs_update(current: &str, target: &str) -> bool {
    let current = Version::parse(current).unwrap();
    let target = Version::parse(target).unwrap();
    target > current
}
```

### 补丁大小估算

- 记录在 `HotUpdatePatch::size_bytes`
- 用于显示下载进度

### 输入

- 当前版本
- 目标版本

### 输出

- 是否需要更新

### 验收标准

- [ ] 版本比较正确
- [ ] 补丁大小计算正确

### 依赖关系

- 依赖 `semver` crate

### 优先级

**P1**

---

## 8. CLI 支持

| 需求编号 | 功能描述 |
|---------|---------|
| REQ-121 | `engine hot-update --from <v1> --to <v2> --output <patch>` 热更新命令 |

### CLI 用法

```bash
# 生成补丁
engine hot-update --from v1.0.0 --to v1.1.0 --output ./patch.zip

# 或指定清单文件
engine hot-update --from-manifest old.manifest --to-manifest new.manifest --output ./patch.zip
```

### API 签名

```rust
pub struct HotUpdateCLI;

impl HotUpdateCLI {
    pub fn generate_patch(from: &str, to: &str, output: impl AsRef<Path>) -> Result<()>;
    pub fn apply_patch(dir: impl AsRef<Path>, patch: impl AsRef<Path>) -> Result<()>;
}
```

### 输入

- 旧版本标识
- 新版本标识
- 输出路径

### 输出

- 补丁文件

### 验收标准

- [ ] CLI 命令正确生成补丁
- [ ] 补丁可被应用

### 依赖关系

- 依赖 `HotUpdate`
- 依赖 CLI 框架

### 优先级

**P1**

---

## 9. 安全性

### 文件校验

- 新增/修改文件需验证 hash
- 校验失败拒绝应用

### 签名验证（可选）

```rust
pub struct SignedPatch {
    patch: HotUpdatePatch,
    signature: Vec<u8>,
}

impl SignedPatch {
    pub fn verify(&self, public_key: &PublicKey) -> bool;
}
```

### 输入

- 补丁数据
- 签名

### 输出

- 验证结果

### 验收标准

- [ ] Hash 校验正确
- [ ] 签名验证正确（如实现）

### 依赖关系

- 依赖 `ed25519-dalek`（可选）

### 优先级

**P2**

---

## 10. 错误处理

### 错误类型

| 错误码 | 说明 |
|--------|------|
| HOT_001 | 旧清单不存在 |
| HOT_002 | 新清单不存在 |
| HOT_003 | 补丁文件损坏 |
| HOT_004 | Hash 校验失败 |
| HOT_005 | 文件写入失败 |
| HOT_006 | 磁盘空间不足 |

### BuildError 格式

```rust
pub struct BuildError {
    pub code: String,
    pub message: String,
    pub stage: BuildStage,
    pub file: Option<PathBuf>,
}
```

### 输入

- 错误上下文

### 输出

- 错误信息

### 验收标准

- [ ] 错误码正确
- [ ] 错误信息清晰
- [ ] 阶段定位正确

### 依赖关系

- 无

### 优先级

**P1**

---

## 11. 优先级汇总

| 优先级 | 需求编号 | 模块 |
|-------|---------|------|
| P0 | REQ-103~106, REQ-300~310 | HotUpdate, HotUpdatePatch, FileChange |
| P1 | REQ-121, REQ-302~307 | CLI, Version |
| P2 | - | Security |

---

## 12. 依赖关系图

```
HotUpdate
├── HotUpdatePatch
│   └── AssetManifest
├── FileChange
│   └── (无依赖)
├── DiffResult
│   └── AssetManifest
└── AssetEntry
    └── AssetKind
```

---

## 13. 验收清单

- [ ] `HotUpdate::diff()` 正确计算差异
- [ ] `HotUpdatePatch` 正确序列化/反序列化
- [ ] `FileChange` 三种变体正确
- [ ] `HotUpdate::apply()` 正确应用补丁
- [ ] `engine hot-update` CLI 命令正常工作
- [ ] 单测：`HotUpdate::diff` 正确识别新增/修改/删除
