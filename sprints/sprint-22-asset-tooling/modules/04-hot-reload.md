# Module 04 — 热更新

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)
> 文件位置: `engine-asset/src/hot_reload/`

## 1. FileWatcher

```rust
pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    tx: Sender<WatchEvent>,
    rx: Receiver<WatchEvent>,
    debouncer: Debouncer,
}

pub struct Debouncer {
    pending: HashMap<PathBuf, Instant>,
    delay: Duration,    // 100ms
}

pub enum WatchEvent {
    Modified(PathBuf),
    Created(PathBuf),
    Removed(PathBuf),
}

impl FileWatcher {
    pub fn new() -> Self;
    
    pub fn watch(&mut self, path: &Path) -> Result<(), Error>;
    pub fn unwatch(&mut self, path: &Path);
    
    pub fn poll(&mut self) -> Vec<WatchEvent> {
        // 应用防抖
        let mut result = Vec::new();
        let now = Instant::now();
        let ready: Vec<PathBuf> = self.debouncer.pending.iter()
            .filter(|(_, t)| now.duration_since(**t) >= self.debouncer.delay)
            .map(|(p, _)| p.clone())
            .collect();
        for path in ready {
            self.debouncer.pending.remove(&path);
            result.push(WatchEvent::Modified(path));
        }
        result
    }
}
```

## 2. AssetReloader

```rust
pub struct AssetReloader {
    registry: Arc<AssetRegistry>,
    shader_reloader: ShaderReloader,
    scene_reloader: SceneReloader,
    cache: HashMap<PathBuf, ReloadEntry>,
}

struct ReloadEntry {
    uuid: AssetUuid,
    last_reload: Instant,
}

impl AssetReloader {
    pub fn on_file_changed(&mut self, path: &Path, world: &mut World) {
        match path.extension().and_then(|e| e.to_str()) {
            Some("glb") | Some("gltf") => {
                self.reload_mesh(path, world);
            }
            Some("frag") | Some("vert") | Some("glsl") | Some("hlsl") | Some("wgsl") => {
                self.shader_reloader.reload(path, world);
            }
            Some("yaml") | Some("scene") => {
                self.scene_reloader.reload(path, world);
            }
            Some("png") | Some("jpg") | Some("ktx2") => {
                self.reload_texture(path, world);
            }
            _ => {}
        }
    }
    
    fn reload_mesh(&mut self, path: &Path, world: &mut World) {
        let new_mesh = GltfImporter.import(path).unwrap();
        // 找到使用此 mesh 的所有 entity，更新组件
        let mut q = world.query::<&mut MeshComponent>();
        for mut comp in q.iter_mut() {
            if comp.path == path {
                comp.mesh = new_mesh.clone();
            }
        }
    }
}
```

## 3. Shader 热重载

```rust
pub struct ShaderReloader {
    pipelines: HashMap<PathBuf, GpuPipelineState>,
}

impl ShaderReloader {
    pub fn reload(&mut self, path: &Path, world: &mut World) {
        let source = std::fs::read_to_string(path).unwrap();
        // 重新编译 shader
        let new_pipeline = world.resource_mut::<GpuDevice>().create_pipeline_state(&PipelineDesc {
            vertex_shader: source.clone(),
            /* ... */
        });
        // 替换旧 pipeline
        if let Some(old) = self.pipelines.insert(path.to_path_buf(), new_pipeline) {
            // 通知所有使用此 pipeline 的材质
        }
    }
}
```

## 4. 场景热重载

```rust
pub struct SceneReloader;

impl SceneReloader {
    pub fn reload(&mut self, path: &Path, world: &mut World) {
        // 1. 备份当前 entity ID 映射
        let entity_map: HashMap<String, Entity> = collect_named_entities(world);
        
        // 2. 加载新场景
        let new_scene = SceneDeserializer.deserialize(&std::fs::read_to_string(path).unwrap()).unwrap();
        
        // 3. 保留 entity ID，更新组件
        for entity_def in &new_scene.entities {
            let entity = entity_map.get(&entity_def.name)
                .copied()
                .unwrap_or_else(|| world.spawn());
            // 应用组件（如果存在则更新，否则插入）
            apply_components(world, entity, &entity_def.components);
        }
    }
}
```

## 5. 验收

- [ ] 着色器修改 → 100 ms 内生效
- [ ] 场景保存 → 1 s 内运行时刷新
- [ ] 资产包版本切换无内存泄漏
- [ ] 防抖：100ms 窗口内多次修改合并
- [ ] 删除文件友好处理
