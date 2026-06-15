# 测试计划

## 概述

本文档定义 Sprint 08 的测试计划，包括单元测试、集成测试、CI 测试和验收测试。本 Sprint 共有 378 条需求，其中测试相关需求覆盖单测、集成测试和 CI 验证。

### 需求来源

对应原文档需求编号：**152-169, 180-191, 360-378**

---

## 1. 单元测试

### 1.1 BuildConfig 单测

**需求**: REQ-180, REQ-360

> REQ-180: 单测：`BuildConfig` 保存/加载往返
> REQ-360: 单测：`BuildConfig` TOML 往返

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_config_default() {
        let config = BuildConfig::default();
        assert_eq!(config.app_name, "my-app");
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.profile, Profile::Debug);
    }
    
    #[test]
    fn test_build_config_toml_roundtrip() {
        let config = BuildConfig::default()
            .with_assets_dir("./assets")
            .with_output_dir("./target");
        
        let toml = config.to_toml();
        let parsed = BuildConfig::from_toml(&toml).unwrap();
        
        assert_eq!(parsed.app_name, config.app_name);
        assert_eq!(parsed.version, config.version);
        assert_eq!(parsed.assets_dir, config.assets_dir);
        assert_eq!(parsed.output_dir, config.output_dir);
    }
    
    #[test]
    fn test_build_config_save_load() {
        let config = BuildConfig::default();
        let path = tempfile::tempdir().unwrap().path().join("config.toml");
        
        config.save(&path).unwrap();
        let loaded = BuildConfig::from_file(&path).unwrap();
        
        assert_eq!(loaded.app_name, config.app_name);
    }
}
```

**验收标准**

- [ ] `test_build_config_default` 通过
- [ ] `test_build_config_toml_roundtrip` 通过
- [ ] `test_build_config_save_load` 通过

---

### 1.2 AssetPipeline 单测

**需求**: REQ-181, REQ-361

> REQ-181: 单测：`AssetPipeline` 扫描/导入（用临时目录）
> REQ-361: 单测：`AssetPipeline` 扫描导入

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_asset_pipeline_scan() {
        let temp_dir = tempfile::tempdir().unwrap();
        let asset_dir = temp_dir.path();
        
        // 创建测试资源
        fs::create_dir_all(asset_dir.join("textures")).unwrap();
        fs::write(asset_dir.join("textures/player.png"), &[0u8; 1024]).unwrap();
        fs::write(asset_dir.join("textures/bg.png"), &[0u8; 2048]).unwrap();
        
        let mut pipeline = AssetPipeline::new(asset_dir);
        pipeline.scan().unwrap();
        
        let manifest = pipeline.build_manifest();
        assert_eq!(manifest.entries().len(), 2);
    }
    
    #[test]
    fn test_asset_pipeline_import() {
        let temp_dir = tempfile::tempdir().unwrap();
        let asset_dir = temp_dir.path();
        
        fs::create_dir_all(asset_dir.join("audio")).unwrap();
        fs::write(asset_dir.join("audio/bgm.wav"), &[0u8; 4096]).unwrap();
        
        let mut pipeline = AssetPipeline::new(asset_dir);
        pipeline.scan().unwrap();
        pipeline.import_all().unwrap();
        
        let manifest = pipeline.build_manifest();
        assert!(!manifest.entries().is_empty());
    }
    
    #[test]
    fn test_asset_pipeline_reimport_changed() {
        let temp_dir = tempfile::tempdir().unwrap();
        let asset_dir = temp_dir.path();
        
        fs::write(asset_dir.join("test.txt"), b"original").unwrap();
        
        let mut pipeline = AssetPipeline::new(asset_dir);
        pipeline.scan().unwrap();
        let manifest1 = pipeline.build_manifest();
        
        // 修改文件
        std::thread::sleep(std::time::Duration::from_millis(10));
        fs::write(asset_dir.join("test.txt"), b"modified").unwrap();
        
        pipeline.reimport_changed().unwrap();
        let manifest2 = pipeline.build_manifest();
        
        // 验证文件被重新导入
        assert_ne!(manifest1.entries()[0].hash(), manifest2.entries()[0].hash());
    }
}
```

**验收标准**

- [ ] `test_asset_pipeline_scan` 通过
- [ ] `test_asset_pipeline_import` 通过
- [ ] `test_asset_pipeline_reimport_changed` 通过

---

### 1.3 AssetManifest 单测

**需求**: REQ-182, REQ-362

> REQ-182: 单测：`AssetManifest` JSON 往返
> REQ-362: 单测：`AssetManifest` JSON 往返

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_asset_manifest_new() {
        let manifest = AssetManifest::new();
        assert!(manifest.entries().is_empty());
    }
    
    #[test]
    fn test_asset_manifest_add() {
        let mut manifest = AssetManifest::new();
        manifest.add(AssetEntry {
            path: PathBuf::from("test.png"),
            hash: "abc123".to_string(),
            size: 1024,
            kind: AssetKind::Texture,
            dependencies: vec![],
        });
        
        assert_eq!(manifest.entries().len(), 1);
        assert_eq!(manifest.entries()[0].path, PathBuf::from("test.png"));
    }
    
    #[test]
    fn test_asset_manifest_json_roundtrip() {
        let mut manifest = AssetManifest::new();
        manifest.add(AssetEntry {
            path: PathBuf::from("textures/player.png"),
            hash: "def456".to_string(),
            size: 2048,
            kind: AssetKind::Texture,
            dependencies: vec![],
        });
        
        let json = manifest.to_json();
        let parsed = AssetManifest::from_json(&json).unwrap();
        
        assert_eq!(parsed.entries().len(), manifest.entries().len());
        assert_eq!(parsed.entries()[0].path, manifest.entries()[0].path);
        assert_eq!(parsed.entries()[0].hash, manifest.entries()[0].hash);
    }
    
    #[test]
    fn test_asset_manifest_save_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("manifest.json");
        
        let mut manifest = AssetManifest::new();
        manifest.add(AssetEntry {
            path: PathBuf::from("test.glb"),
            hash: "ghi789".to_string(),
            size: 4096,
            kind: AssetKind::Model,
            dependencies: vec![],
        });
        
        manifest.save(&path).unwrap();
        let loaded = AssetManifest::load(&path).unwrap();
        
        assert_eq!(loaded.entries().len(), manifest.entries().len());
    }
}
```

**验收标准**

- [ ] `test_asset_manifest_new` 通过
- [ ] `test_asset_manifest_add` 通过
- [ ] `test_asset_manifest_json_roundtrip` 通过
- [ ] `test_asset_manifest_save_load` 通过

---

### 1.4 HotUpdate 单测

**需求**: REQ-183, REQ-363

> REQ-183: 单测：`HotUpdate::diff` 正确识别新增/修改/删除
> REQ-363: 单测：`HotUpdate::diff`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_manifest(entries: Vec<(&str, &str)>) -> AssetManifest {
        let mut manifest = AssetManifest::new();
        for (path, hash) in entries {
            manifest.add(AssetEntry {
                path: PathBuf::from(path),
                hash: hash.to_string(),
                size: 1024,
                kind: AssetKind::Custom("test".to_string()),
                dependencies: vec![],
            });
        }
        manifest
    }
    
    #[test]
    fn test_hotupdate_diff_added() {
        let old = create_test_manifest(vec![
            ("a.txt", "hash_a"),
            ("b.txt", "hash_b"),
        ]);
        
        let new = create_test_manifest(vec![
            ("a.txt", "hash_a"),
            ("b.txt", "hash_b"),
            ("c.txt", "hash_c"),
        ]);
        
        let diff = HotUpdate::diff(&old, &new);
        
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.modified.len(), 0);
        assert_eq!(diff.removed.len(), 0);
        assert_eq!(diff.added[0].path, PathBuf::from("c.txt"));
    }
    
    #[test]
    fn test_hotupdate_diff_modified() {
        let old = create_test_manifest(vec![
            ("a.txt", "hash_a"),
            ("b.txt", "hash_b"),
        ]);
        
        let new = create_test_manifest(vec![
            ("a.txt", "hash_a_modified"),
            ("b.txt", "hash_b"),
        ]);
        
        let diff = HotUpdate::diff(&old, &new);
        
        assert_eq!(diff.added.len(), 0);
        assert_eq!(diff.modified.len(), 1);
        assert_eq!(diff.removed.len(), 0);
        assert_eq!(diff.modified[0].hash, "hash_a_modified");
    }
    
    #[test]
    fn test_hotupdate_diff_removed() {
        let old = create_test_manifest(vec![
            ("a.txt", "hash_a"),
            ("b.txt", "hash_b"),
        ]);
        
        let new = create_test_manifest(vec![
            ("b.txt", "hash_b"),
        ]);
        
        let diff = HotUpdate::diff(&old, &new);
        
        assert_eq!(diff.added.len(), 0);
        assert_eq!(diff.modified.len(), 0);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.removed[0], PathBuf::from("a.txt"));
    }
    
    #[test]
    fn test_hotupdate_diff_mixed() {
        let old = create_test_manifest(vec![
            ("a.txt", "hash_a"),
            ("b.txt", "hash_b"),
            ("c.txt", "hash_c"),
        ]);
        
        let new = create_test_manifest(vec![
            ("a.txt", "hash_a_modified"),  // modified
            ("b.txt", "hash_b"),            // unchanged
            ("d.txt", "hash_d"),            // added
        ]);
        
        let diff = HotUpdate::diff(&old, &new);
        
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.modified.len(), 1);
        assert_eq!(diff.removed.len(), 1);
        
        assert_eq!(diff.added[0].path, PathBuf::from("d.txt"));
        assert_eq!(diff.modified[0].path, PathBuf::from("a.txt"));
        assert!(diff.removed.contains(&PathBuf::from("c.txt")));
    }
    
    #[test]
    fn test_hotupdate_diff_empty() {
        let manifest = create_test_manifest(vec![
            ("a.txt", "hash_a"),
        ]);
        
        let diff = HotUpdate::diff(&manifest, &manifest);
        
        assert!(diff.is_empty());
        assert_eq!(diff.total_changes(), 0);
    }
}
```

**验收标准**

- [ ] `test_hotupdate_diff_added` 通过
- [ ] `test_hotupdate_diff_modified` 通过
- [ ] `test_hotupdate_diff_removed` 通过
- [ ] `test_hotupdate_diff_mixed` 通过
- [ ] `test_hotupdate_diff_empty` 通过

---

### 1.5 BuildCache 单测

**需求**: REQ-184, REQ-364

> REQ-184: 单测：`BuildCache` hash
> REQ-364: 单测：`BuildCache` 缓存命中

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_cache_new() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = BuildCache::new(temp_dir.path()).unwrap();
        
        assert!(cache.get("nonexistent").is_none());
    }
    
    #[test]
    fn test_build_cache_hash() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = BuildCache::new(temp_dir.path()).unwrap();
        
        let hash1 = cache.hash("key1");
        let hash2 = cache.hash("key2");
        let hash3 = cache.hash("key1");  // Same key
        
        assert_ne!(hash1, hash2);
        assert_eq!(hash1, hash3);
    }
    
    #[test]
    fn test_build_cache_put_get() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache = BuildCache::new(temp_dir.path()).unwrap();
        
        // 创建测试文件
        let file_path = temp_dir.path().join("test.bin");
        std::fs::write(&file_path, b"test data").unwrap();
        
        cache.put("test_key", &file_path);
        
        let retrieved = cache.get("test_key");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), file_path);
    }
    
    #[test]
    fn test_build_cache_clean() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut cache = BuildCache::new(temp_dir.path()).unwrap();
        
        let file_path = temp_dir.path().join("test.bin");
        std::fs::write(&file_path, b"test data").unwrap();
        
        cache.put("test_key", &file_path);
        assert!(cache.get("test_key").is_some());
        
        cache.clean();
        assert!(cache.get("test_key").is_none());
    }
}
```

**验收标准**

- [ ] `test_build_cache_new` 通过
- [ ] `test_build_cache_hash` 通过
- [ ] `test_build_cache_put_get` 通过
- [ ] `test_build_cache_clean` 通过

---

### 1.6 BuildPipeline 单测

**需求**: REQ-185, REQ-365

> REQ-185: 单测：`BuildPipeline` 在当前平台可构建（Linux）
> REQ-365: 集成测试：`engine build --target linux --profile debug`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]  // 需要较长时间，仅在集成测试时运行
    fn test_build_pipeline_current_platform() {
        let config = BuildConfig::default()
            .with_output_dir("./target/test")
            .with_temp_dir("./target/test/.tmp");
        
        let pipeline = BuildPipeline::new(config).unwrap();
        
        // 清理后构建
        pipeline.clean().unwrap();
        let result = pipeline.build();
        
        assert!(result.is_ok());
        let artifact = result.unwrap();
        assert!(artifact.path().exists());
    }
    
    #[test]
    fn test_build_pipeline_config() {
        let config = BuildConfig::default();
        let pipeline = BuildPipeline::new(config.clone()).unwrap();
        
        assert_eq!(pipeline.config().app_name, config.app_name);
        assert_eq!(pipeline.platform_target(), PlatformTarget::current());
    }
    
    #[test]
    fn test_build_pipeline_profile() {
        let config = BuildConfig::default();
        let pipeline = BuildPipeline::new(config).unwrap();
        
        assert_eq!(pipeline.profile(), Profile::Debug);
    }
}
```

**验收标准**

- [ ] `test_build_pipeline_config` 通过
- [ ] `test_build_pipeline_profile` 通过
- [ ] `test_build_pipeline_current_platform` 通过（CI 环境）

---

## 2. 集成测试

### 2.1 CLI 集成测试

**需求**: REQ-186, REQ-365

> REQ-186: 集成测试：CLI `engine build --target linux --profile debug` 成功

```bash
# tests/integration_test.sh

#!/bin/bash
set -e

echo "=== CLI Integration Tests ==="

# Test 1: Build CLI help
echo "Test 1: build_cli --help"
cargo run --example build_cli -- help | grep -q "Usage"
echo "PASS"

# Test 2: Build for Linux
echo "Test 2: engine build --target linux --profile debug"
cargo run --example build_cli -- build --target linux --profile debug
echo "PASS"

# Test 3: Clean
echo "Test 3: engine clean"
cargo run --example build_cli -- clean
echo "PASS"

echo "=== All CLI Integration Tests Passed ==="
```

### 2.2 WebAssembly 集成测试

**需求**: REQ-190, REQ-357

> REQ-190: CI：新增 `cargo build --target wasm32-unknown-unknown` 测试
> REQ-357: `examples/wasm_demo` 生成 WASM 产物

```bash
# tests/wasm_test.sh

#!/bin/bash
set -e

echo "=== WebAssembly Integration Tests ==="

# Test: Build WASM
echo "Test: cargo build --target wasm32-unknown-unknown"
cargo build --target wasm32-unknown-unknown --release -p wasm_demo
echo "PASS"

# Verify output
test -f target/wasm32-unknown-unknown/release/wasm_demo.wasm
echo "WASM file generated: PASS"

echo "=== WebAssembly Integration Tests Passed ==="
```

### 2.3 Cross-compile 集成测试

**需求**: REQ-191

> REQ-191: CI：新增 `cargo build --target x86_64-pc-windows-gnu` 测试（交叉编译）

```bash
# tests/cross_compile_test.sh

#!/bin/bash
set -e

echo "=== Cross-compile Integration Tests ==="

# Test: Cross-compile to Windows
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Test: cargo build --target x86_64-pc-windows-gnu"
    cargo build --target x86_64-pc-windows-gnu
    echo "PASS"
else
    echo "SKIP: mingw not installed"
fi

echo "=== Cross-compile Integration Tests Passed ==="
```

---

## 3. CI 测试

### 3.1 CI 环境要求

**需求**: REQ-192

> REQ-192: CI：三平台 green

| 平台 | 触发条件 | 测试项 |
|------|---------|--------|
| Linux | 始终 | 所有测试 |
| macOS | PR/Push | 所有测试 |
| Windows | PR/Push | 交叉编译测试 |

### 3.2 CI 测试命令

**需求**: REQ-186, REQ-359~369

```yaml
# .github/workflows/ci.yml

name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    name: Test on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: wasm32-unknown-unknown, x86_64-pc-windows-gnu
      
      - name: Run tests
        run: |
          cargo test -p engine-build --all-features
          cargo test -p engine-build --doc
      
      - name: Check formatting
        run: |
          cargo fmt --check --workspace
      
      - name: Run Clippy
        run: |
          cargo clippy --workspace -- -D warnings
      
      - name: Build documentation
        run: |
          cargo doc --workspace --no-deps
      
      - name: WASM build
        run: |
          cargo build --target wasm32-unknown-unknown -p wasm_demo
      
      - name: Cross-compile to Windows
        if: matrix.os == 'ubuntu-latest'
        run: |
          cargo build --target x86_64-pc-windows-gnu

  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
      
      - name: CLI integration test
        run: |
          cargo run --example build_cli -- build --target linux --profile debug
      
      - name: WASM demo test
        run: |
          cd examples/wasm_demo
          wasm-pack build --target web --out-dir pkg
          test -f pkg/wasm_demo.js
          test -f pkg/wasm_demo_bg.wasm
```

---

## 4. 代码质量检查

### 4.1 Formatting Check

**需求**: REQ-368

> REQ-368: `cargo fmt --check --workspace` 通过

```bash
# 验证格式
cargo fmt --check --workspace

# 自动格式化（如需要）
cargo fmt --workspace
```

### 4.2 Clippy Lints

**需求**: REQ-367

> REQ-367: `cargo clippy --workspace -- -D warnings` 通过

```bash
cargo clippy --workspace -- -D warnings
```

### 4.3 Documentation

**需求**: REQ-369

> REQ-369: `cargo doc --workspace --no-deps` 成功

```bash
cargo doc --workspace --no-deps
```

### 4.4 Unsafe Code Audit

**需求**: REQ-198

> REQ-198: 本 Sprint `unsafe` 块 <= 5

```bash
# 统计 unsafe 块数量
rg 'unsafe\s*\{' --type rust crates/engine-build/src -c
```

---

## 5. 验收测试

### 5.1 构建验收

**需求**: REQ-444~453

> 验收标准清单

| 测试项 | 命令 | 预期结果 |
|--------|------|---------|
| Linux 构建 | `cargo run --example build_cli -- build --target linux --profile debug` | 成功 |
| Web 构建 | `cargo run --example build_cli -- build --target web --profile release` | 成功 |
| WASM 产物 | `cargo run --example wasm_demo` | 浏览器可运行 |
| 单测 | `cargo test -p engine-build` | 全部通过 |
| Clippy | `cargo clippy --workspace -- -D warnings` | 通过 |
| Format | `cargo fmt --check --workspace` | 通过 |
| Docs | `cargo doc --workspace --no-deps` | 成功 |

### 5.2 测试执行脚本

```bash
#!/bin/bash
# tests/acceptance_test.sh

set -e

echo "=== Acceptance Tests ==="

echo "1. Running unit tests..."
cargo test -p engine-build

echo "2. Running clippy..."
cargo clippy --workspace -- -D warnings

echo "3. Checking formatting..."
cargo fmt --check --workspace

echo "4. Building documentation..."
cargo doc --workspace --no-deps

echo "5. Building for Linux..."
cargo run --example build_cli -- build --target linux --profile debug

echo "6. Building for Web..."
cargo run --example build_cli -- build --target web --profile release

echo "7. Building WASM demo..."
cargo build --target wasm32-unknown-unknown -p wasm_demo

echo "=== All Acceptance Tests Passed ==="
```

---

## 6. CHANGELOG 和文档

### 6.1 CHANGELOG 更新

**需求**: REQ-193, REQ-431

> REQ-193: CHANGELOG 记录版本 0.8.0（阶段二完成）
> REQ-431: CHANGELOG 记录 0.8.0

```markdown
# Changelog

## [0.8.0] - 2024-XX-XX

### Added
- `engine-build` crate 实现
- `BuildPipeline` 构建管线
- `PlatformTarget` 平台目标支持
- `AssetPipeline` 资源处理管线
- `HotUpdate` 热更新功能
- CLI 工具 `engine`
- WebAssembly 构建支持
- 小程序构建支持
- Android 构建支持
- iOS 构建支持

### Changed
- 阶段二完成

### Fixed
- (none)

### Security
- (none)
```

### 6.2 README 更新

**需求**: REQ-194~196, REQ-372~375

> README.md 加入「跨平台打包」章节
> README.md 加入「资源管线」章节
> README.md 加入「CLI 使用指南」章节
> README.md 加入「构建产物部署」章节

---

## 7. 测试覆盖率目标

| 模块 | 覆盖率目标 |
|------|-----------|
| build.rs | 90%+ |
| asset.rs | 85%+ |
| hotupdate.rs | 90%+ |
| toolchain/*.rs | 70%+ |
| 总计 | 80%+ |

---

## 8. 测试执行计划

| 阶段 | 测试类型 | 执行频率 | 负责人 |
|------|---------|---------|--------|
| 开发中 | 单测 | 每次提交 | 开发者 |
| PR | 单测 + 集成 | 每次 PR | CI |
| 合并 | 全部测试 | 每次合并 | CI |
| 验收 | 验收测试 | Sprint 结束 | 团队 |

---

## 9. 验收清单

- [ ] `cargo test -p engine-build` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] `cargo doc --workspace --no-deps` 成功
- [ ] `cargo build --target wasm32-unknown-unknown` 成功
- [ ] `cargo build --target x86_64-pc-windows-gnu` 成功
- [ ] `examples/build_cli` 集成测试通过
- [ ] CHANGELOG 记录 0.8.0
- [ ] README 包含所有必需章节
- [ ] `unsafe` 块数量 <= 5
