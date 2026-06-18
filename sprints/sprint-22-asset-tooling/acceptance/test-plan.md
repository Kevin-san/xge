# Sprint 22 · 验收测试计划

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)

## 1. 单元测试清单

| 模块 | 测试数 | 覆盖 |
|------|--------|------|
| AssetHandle | 15+ | 引用计数 / Weak upgrade |
| AssetRegistry | 25+ | 同步 / 异步 / 缓存 |
| AssetLoader | 20+ | 4 种 loader |
| AssetBundle | 25+ | 创建 / 读取 / 压缩 |
| Importer | 30+ | glTF / 纹理 / 音频 |
| Serializer | 20+ | YAML / Prefab / Meta |
| HotReload | 15+ | 4 种文件类型 |
| Profiler | 25+ | CPU / GPU / Memory / Frame |
| CLI | 15+ | 命令解析 / Cook |

**总计：** 200+ 单元测试

## 2. 关键测试

### 2.1 引用计数

```rust
#[test]
fn test_ref_count() {
    let registry = AssetRegistry::new();
    let h1 = registry.load::<Mesh3D>("test.glb");
    let h2 = h1.clone();
    assert_eq!(registry.ref_count(h1.uuid), 2);
    
    drop(h1);
    assert_eq!(registry.ref_count(h2.uuid), 1);
    
    drop(h2);
    assert_eq!(registry.ref_count(h2.uuid), 0);
    assert!(registry.get::<Mesh3D>(h2.uuid).is_none());
}
```

### 2.2 glTF 导入

```rust
#[test]
fn test_gltf_import() {
    let importer = GltfImporter;
    let result = importer.import_gltf("fixtures/Box.glb").unwrap();
    
    assert_eq!(result.meshes.len(), 1);
    assert!(result.meshes[0].vertex_count() > 0);
}
```

### 2.3 Bundle 压缩

```rust
#[test]
fn test_bundle_compression() {
    let data = vec![0u8; 1_000_000];  // 1 MB
    let mut bundle = AssetBundle::create("test.pak", CompressionType::Zstd).unwrap();
    let uuid = bundle.add("test.bin", &data).unwrap();
    bundle.save().unwrap();
    
    let bundle = AssetBundle::open("test.pak").unwrap();
    let loaded = bundle.get::<Vec<u8>>(uuid).unwrap();
    assert_eq!(loaded, data);
    
    let metadata = std::fs::metadata("test.pak").unwrap();
    assert!(metadata.len() < 100_000);  // 应 < 100KB（压缩）
}
```

### 2.4 Profiler 火焰图导出

```rust
#[test]
fn test_chrome_trace_export() {
    let mut profiler = CpuProfiler::new();
    {
        let _a = profiler.begin("update");
        std::thread::sleep(Duration::from_millis(10));
        {
            let _b = profiler.begin("physics");
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    
    profiler.export_chrome("trace.json").unwrap();
    let content = std::fs::read_to_string("trace.json").unwrap();
    assert!(content.contains("update"));
    assert!(content.contains("physics"));
}
```

### 2.5 热重载

```rust
#[test]
fn test_hot_reload_shader() {
    let mut world = World::new();
    let mut watcher = FileWatcher::new();
    let mut reloader = AssetReloader::new();
    
    watcher.watch("test.glsl").unwrap();
    
    // 模拟文件修改
    std::fs::write("test.glsl", "// new content").unwrap();
    std::thread::sleep(Duration::from_millis(200));  // 超过防抖
    
    let events = watcher.poll();
    for event in events {
        reloader.on_file_changed(&event.path(), &mut world);
    }
    
    // 验证 shader 已重新加载
}
```

## 3. 性能基准

| 基准 | 目标 |
|------|------|
| 1 GB 资产包随机访问 | < 10 ms |
| 100 MB PNG → KTX2 | < 1 s |
| 1000 节点场景 YAML | < 50 ms |
| 着色器热重载 | < 100 ms |
| CPU profiler 开销 | < 5% |
| GPU 时间戳精度 | 1 ms |
| `engine build --release` | < 5 min (CI) |
| `engine cook` 100MB | < 30 s |

## 4. CI 集成

- [ ] GitHub Actions 模板
- [ ] GitLab CI 模板
- [ ] 多平台矩阵（ubuntu / macos / windows）
- [ ] cargo test 100% pass
- [ ] lint / clippy 通过
- [ ] 资产 cook 输出
- [ ] 基准结果存档

## 5. 文档

- [ ] CLI 命令帮助
- [ ] 配置文件示例
- [ ] 导入器使用指南
- [ ] 热重载教程
- [ ] Profiler 火焰图阅读
