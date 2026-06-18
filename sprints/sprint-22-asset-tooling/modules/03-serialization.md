# Module 03 — 序列化（场景 / Prefab / Asset Meta）

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)
> 文件位置: `engine-asset/src/serde/`

## 1. 场景序列化

```rust
// engine-asset/src/serde/scene.rs

pub struct SceneSerializer {
    pub format: SerializationFormat,
    pub pretty: bool,
    pub include_defaults: bool,
}

pub enum SerializationFormat {
    Yaml,
    Toml,
    Json,
    Binary,  // MessagePack / Bincode
}

impl SceneSerializer {
    pub fn serialize(&self, scene: &Scene) -> Result<String, Error> {
        // 输出 YAML
        let yaml = serde_yaml::to_string(&SceneYaml::from(scene))?;
        Ok(yaml)
    }
}

pub struct SceneDeserializer;

impl SceneDeserializer {
    pub fn deserialize(&self, content: &str) -> Result<Scene, Error> {
        let yaml: SceneYaml = serde_yaml::from_str(content)?;
        Scene::try_from(yaml)
    }
}
```

## 2. YAML Schema

```yaml
# scene.yaml
scene:
  name: "MyLevel"
  entities:
    - id: 0
      name: "Player"
      components:
        Transform:
          position: [0, 1, 0]
          rotation: [0, 0, 0, 1]
          scale: [1, 1, 1]
        Player:
          speed: 5.0
        Mesh3D:
          mesh: "models/hero.glb"
          material: "materials/hero_pbr.mat"
    - id: 1
      name: "Enemy_01"
      components:
        Transform:
          position: [10, 0, 5]
        Enemy:
          health: 100
          ai_state: "Patrol"
```

## 3. Prefab

```rust
pub struct Prefab {
    pub name: String,
    pub base_entities: Vec<EntityTemplate>,
    pub variants: HashMap<String, PrefabVariant>,
}

pub struct PrefabVariant {
    pub name: String,
    pub parent: String,        // 继承自哪个 prefab
    pub overrides: HashMap<String, ComponentValue>,  // 覆盖字段
}

impl Prefab {
    pub fn instantiate(&self) -> Scene;
    pub fn variant(&self, name: &str) -> Option<&PrefabVariant>;
}
```

```yaml
# prefab.yaml
prefab:
  name: "Goblin"
  base_entities:
    - components:
        Transform: default
        Goblin:
          aggression: 0.7
        Mesh3D: "models/goblin.glb"
  variants:
    GoblinKing:
      parent: "Goblin"
      overrides:
        Goblin:
          health: 500
        Mesh3D: "models/goblin_king.glb"
```

## 4. Asset Meta

```rust
pub struct AssetMeta {
    pub uuid: AssetUuid,
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub import_settings: ImportOptions,
    pub dependencies: Vec<PathBuf>,
    pub last_modified: u64,  // 文件时间戳
}
```

```toml
# model.glb.meta
[meta]
uuid = "0x12345678"
path = "models/hero.glb"
asset_type = "Mesh"
last_modified = 1700000000

[import]
generate_lods = true
lod_levels = [1.0, 0.5, 0.25]
generate_mipmaps = true
compression = "Zstd"

[dependencies]
materials = ["hero_pbr.mat"]
```

## 5. 验收

- [ ] 1000 节点场景 YAML 序列化 < 50 ms
- [ ] YAML diff 友好：单元测试变更检测
- [ ] Prefab 继承深度 5 层不爆炸
- [ ] Asset meta 修改后下次 import 复用
- [ ] 损坏 YAML 友好错误（行号）
