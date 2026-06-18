# Module 02 — 资产导入器

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)
> 文件位置: `engine-asset/src/import/`

## 1. AssetImporter Trait

```rust
pub trait AssetImporter: Send + Sync {
    type Asset: Asset;
    type Source;
    type Error: std::error::Error + Send + Sync;
    
    fn import(&self, source: Self::Source, options: &ImportOptions) -> Result<ImportedAsset<Self::Asset>, Self::Error>;
    fn supported_extensions(&self) -> &[&'static str];
}

pub struct ImportOptions {
    pub target_platform: Platform,
    pub compression: CompressionType,
    pub generate_mipmaps: bool,
    pub generate_lods: bool,
    pub optimize_for: OptimizationTarget,
}

pub enum OptimizationTarget {
    Quality,
    Balanced,
    Performance,
    Size,
}
```

## 2. glTF 导入器

```rust
pub struct GltfImporter;

impl GltfImporter {
    pub fn import_gltf(&self, path: &Path) -> Result<GltfImportResult, Error> {
        let (doc, buffers, images) = gltf::import(path)?;
        
        // 1. 提取 mesh
        let mut meshes = Vec::new();
        for mesh in doc.meshes() {
            let engine_mesh = convert_mesh(&mesh, &buffers)?;
            meshes.push(engine_mesh);
        }
        
        // 2. 提取材质
        let mut materials = Vec::new();
        for material in doc.materials() {
            let pbr = convert_material(&material, &images)?;
            materials.push(pbr);
        }
        
        // 3. 提取节点
        let mut nodes = Vec::new();
        for node in doc.nodes() {
            let engine_node = convert_node(&node)?;
            nodes.push(engine_node);
        }
        
        // 4. 提取动画
        let mut animations = Vec::new();
        for anim in doc.animations() {
            let clip = convert_animation(&anim, &buffers)?;
            animations.push(clip);
        }
        
        // 5. 提取蒙皮
        let mut skins = Vec::new();
        for skin in doc.skins() {
            let skinned = convert_skin(&skin, &buffers)?;
            skins.push(skinned);
        }
        
        Ok(GltfImportResult { meshes, materials, nodes, animations, skins })
    }
}
```

## 3. 纹理导入器

```rust
pub struct TextureImporter;

impl TextureImporter {
    pub fn import(&self, path: &Path, options: &TextureImportOptions) -> Result<TextureAsset, Error> {
        // 1. 加载源图
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        
        // 2. 生成 mipmap
        let mipmaps = if options.generate_mipmaps {
            generate_mipmaps(&rgba)
        } else {
            vec![rgba.clone()]
        };
        
        // 3. 压缩到 KTX2 / BC7 / ASTC
        let compressed = match options.target_format {
            TargetFormat::BC7 => compress_bc7(&mipmaps)?,
            TargetFormat::ASTC_4x4 => compress_astc(&mipmaps, 4, 4)?,
            TargetFormat::KTX2 => compress_ktx2(&mipmaps)?,
            _ => mipmaps,
        };
        
        Ok(TextureAsset {
            width: rgba.width(),
            height: rgba.height(),
            mipmaps: compressed,
            format: options.target_format,
        })
    }
}
```

## 4. 网格简化（QEM）

```rust
pub fn simplify_mesh(
    mesh: &Mesh3D,
    target_triangle_count: usize,
) -> Mesh3D {
    // Quadric Error Metrics 算法
    // 1. 计算每个顶点的 QEM 矩阵
    // 2. 边折叠优先级队列
    // 3. 迭代折叠到目标三角形数
    
    let mut simplifier = QemSimplifier::new(mesh);
    simplifier.simplify_to(target_triangle_count);
    simplifier.into_mesh()
}
```

## 5. 验收

- [ ] glTF 2.0 全特性支持（动画 / 蒙皮 / PBR / IBL）
- [ ] 100 MB PNG → KTX2 < 1 s
- [ ] 网格简化：100k 三角 → 1k 三角 < 500 ms
- [ ] 立方贴图 IBL 预过滤
- [ ] 错误恢复：损坏文件友好错误
