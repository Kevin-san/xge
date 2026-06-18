# Module 01 — 资产系统

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)
> 文件位置: `engine-asset/src/`

## 1. AssetHandle

```rust
// engine-asset/src/handle.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetUuid(pub u64);

pub struct AssetHandle<T: Asset> {
    uuid: AssetUuid,
    state: Arc<AssetState<T>>,
}

struct AssetState<T> {
    asset: RwLock<Option<Arc<T>>>,
    ref_count: AtomicUsize,
    load_status: AtomicU8,  // 0=loading, 1=loaded, 2=error
    loader: AssetTypeErasedLoader,
}

impl<T: Asset> AssetHandle<T> {
    pub fn is_loaded(&self) -> bool { self.state.load_status.load(Acquire) == 1 }
    pub fn get(&self) -> Option<Arc<T>> { self.state.asset.read().as_ref().cloned() }
    pub fn wait_load(&self) -> Arc<T> { /* 阻塞等待 */ }
}

pub struct WeakAssetHandle<T: Asset> {
    state: Weak<AssetState<T>>,
    uuid: AssetUuid,
}

impl<T: Asset> WeakAssetHandle<T> {
    pub fn upgrade(&self) -> Option<AssetHandle<T>>;
}
```

## 2. AssetRegistry

```rust
pub struct AssetRegistry {
    by_uuid: RwLock<HashMap<AssetUuid, Arc<dyn AnyAssetState>>>,
    by_path: RwLock<HashMap<PathBuf, AssetUuid>>,
    loaders: HashMap<&'static str, Box<dyn AssetLoaderFactory>>,
    loading_tasks: Mutex<HashMap<AssetUuid, JoinHandle<()>>>,
    next_uuid: AtomicU64,
}

impl AssetRegistry {
    pub fn new() -> Self;
    
    pub fn load<T: Asset>(&self, path: &Path) -> AssetHandle<T>;
    pub fn load_async<T: Asset>(&self, path: &Path) -> AssetHandle<T>;
    
    pub fn get<T: Asset>(&self, uuid: AssetUuid) -> Option<AssetHandle<T>>;
    pub fn release(&self, uuid: AssetUuid);
    
    pub fn register_loader(&mut self, ext: &'static str, loader: Box<dyn AssetLoaderFactory>);
    
    pub fn tick(&self) {
        // 检查异步加载完成
        let mut tasks = self.loading_tasks.lock();
        let mut done = Vec::new();
        for (uuid, handle) in tasks.iter() {
            if handle.is_finished() { done.push(*uuid); }
        }
        for uuid in done {
            tasks.remove(&uuid);
        }
    }
}
```

## 3. AssetLoader

```rust
pub trait Asset: Send + Sync + 'static {
    type Loader: AssetLoader;
    fn loader() -> Self::Loader;
}

pub trait AssetLoader: Send + Sync + 'static {
    type Asset: Asset;
    type Options: Default + Send + Sync;
    type Error: std::error::Error + Send + Sync;
    
    fn load(&self, reader: &mut dyn Read, options: &Self::Options) -> Result<Self::Asset, Self::Error>;
    fn extensions(&self) -> &[&'static str];
}

pub trait AssetLoaderFactory: Send + Sync {
    fn extensions(&self) -> &[&'static str];
    fn load(&self, path: &Path, reader: &mut dyn Read) -> Result<Box<dyn AnyAsset>, Box<dyn Error>>;
}
```

## 4. AssetBundle（.pak 格式）

```rust
pub struct AssetBundle {
    path: PathBuf,
    index: BundleIndex,
    file: Mutex<File>,
}

pub struct BundleIndex {
    entries: HashMap<AssetUuid, BundleEntry>,
    pub compression: CompressionType,
}

pub struct BundleEntry {
    pub offset: u64,
    pub size: u64,
    pub compressed_size: u64,
    pub asset_type: String,
    pub path: String,
}

#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    None,
    Zstd,
    Lz4,
}

impl AssetBundle {
    pub fn open(path: &Path) -> Result<Self, Error>;
    pub fn create(path: &Path, compression: CompressionType) -> Result<Self, Error>;
    
    pub fn get<T: Asset>(&self, uuid: AssetUuid) -> Result<T, Error>;
    pub fn contains(&self, uuid: AssetUuid) -> bool;
    
    pub fn add(&mut self, path: &Path, data: &[u8]) -> Result<AssetUuid, Error>;
    pub fn save(&self) -> Result<(), Error>;
}
```

## 5. 异步加载

```rust
impl AssetRegistry {
    pub fn load_async<T: Asset>(&self, path: &Path) -> AssetHandle<T> {
        let uuid = self.next_uuid.fetch_add(1, Ordering::Relaxed);
        let state = Arc::new(AssetState::new_loading());
        let registry = self.clone();
        let path = path.to_path_buf();
        
        let handle = tokio::spawn(async move {
            let asset = T::loader().load_async(&path).await;
            match asset {
                Ok(asset) => registry.mark_loaded(uuid, Arc::new(asset)),
                Err(e) => registry.mark_error(uuid),
            }
        });
        
        self.loading_tasks.lock().insert(uuid, handle);
        AssetHandle { uuid, state }
    }
}
```

## 6. 缓存策略

```rust
pub struct LruCache<T> {
    map: HashMap<AssetUuid, Arc<T>>,
    list: LruList<AssetUuid>,
    max_size: usize,
}

impl<T> LruCache<T> {
    pub fn get(&mut self, uuid: AssetUuid) -> Option<Arc<T>>;
    pub fn put(&mut self, uuid: AssetUuid, value: Arc<T>);
    pub fn evict_if_needed(&mut self);
}
```

## 7. 验收

- [ ] 1 GB 资产包随机访问 < 10 ms
- [ ] 引用计数正确：卸载无泄漏
- [ ] 异步加载 UI 不卡顿
- [ ] LRU 内存限制生效
- [ ] Zstd 压缩 100MB → 50MB
- [ ] 与 ECS Resource 集成
