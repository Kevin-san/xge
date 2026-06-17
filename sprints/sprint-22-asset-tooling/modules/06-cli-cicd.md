# Module 06 — 命令行工具与 CI

> 上游 sprint: [Sprint 22](../sprint-22-asset-tooling.md)
> 文件位置: `engine-cli/src/`

## 1. CLI 命令

```rust
// engine-cli/src/main.rs

#[derive(Parser)]
#[command(name = "engine")]
enum Command {
    /// 构建项目
    Build {
        #[arg(long, default_value = "release")]
        profile: String,
        
        #[arg(long)]
        target: Option<String>,  // x86_64-unknown-linux-gnu
        
        #[arg(long)]
        features: Vec<String>,
    },
    
    /// 运行项目
    Run {
        #[arg(long, default_value = "dev")]
        profile: String,
        
        #[arg(long)]
        example: Option<String>,
    },
    
    /// 性能分析
    Profile {
        #[arg(long, default_value = "60")]
        seconds: u32,
        
        #[arg(long, default_value = "chrome")]
        format: ProfileFormat,  // chrome / tracy / flamegraph
    },
    
    /// 资产 cook
    Cook {
        #[arg(long)]
        input: PathBuf,
        
        #[arg(long)]
        output: PathBuf,
        
        #[arg(long, default_value = "zstd")]
        compression: String,
    },
    
    /// 运行测试
    Test {
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 清理
    Clean,
}

fn main() {
    let cmd = Command::parse();
    match cmd {
        Command::Build { profile, target, features } => build(profile, target, features),
        Command::Run { profile, example } => run(profile, example),
        Command::Profile { seconds, format } => profile(seconds, format),
        Command::Cook { input, output, compression } => cook(input, output, compression),
        Command::Test { args } => test(args),
        Command::Clean => clean(),
    }
}
```

## 2. 配置文件

```toml
# engine.toml
[project]
name = "my_game"
version = "0.1.0"

[build]
target = "x86_64-unknown-linux-gnu"
features = ["vulkan", "fsr"]

[assets]
source = "assets"
output = "dist/assets.pak"
compression = "zstd"

[profile]
chrome_trace = true
tracy = false
flamegraph = true

[bundles]
windows = { ext = ".exe", target = "x86_64-pc-windows-msvc" }
linux = { ext = "", target = "x86_64-unknown-linux-gnu" }
macos = { ext = ".app", target = "x86_64-apple-darwin" }
web = { ext = ".wasm", target = "wasm32-unknown-unknown" }
```

## 3. Cook 命令

```rust
// engine-cli/src/commands/cook.rs

pub fn cook(input: PathBuf, output: PathBuf, compression: String) -> Result<(), Error> {
    let mut bundle = AssetBundle::create(&output, parse_compression(&compression))?;
    
    // 递归遍历 input
    for entry in walkdir::WalkDir::new(&input) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            // 选择 importer
            let imported = match path.extension().and_then(|e| e.to_str()) {
                Some("gltf") | Some("glb") => GltfImporter.import(path)?,
                Some("png") | Some("jpg") | Some("exr") => TextureImporter.import(path)?,
                Some("ogg") | Some("wav") | Some("mp3") => AudioImporter.import(path)?,
                Some("yaml") => SceneImporter.import(path)?,
                _ => continue,
            };
            // 写入 bundle
            let uuid = bundle.add(path.strip_prefix(&input)?, &imported.bytes())?;
            // 写 meta
            bundle.add_meta(&AssetMeta {
                uuid,
                path: path.to_path_buf(),
                asset_type: imported.asset_type(),
                ..Default::default()
            })?;
        }
    }
    
    bundle.save()?;
    Ok(())
}
```

## 4. CI 模板

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    name: ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Lint
        run: cargo clippy --workspace --all-features -- -D warnings
      - name: Test
        run: cargo test --workspace
      - name: Bench
        run: cargo bench --workspace -- --output-format bencher
      - name: Cook
        run: cargo run --release --bin engine -- cook --input assets --output dist/assets.pak
      - name: Build
        run: cargo build --release --workspace
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: assets-${{ matrix.os }}
          path: dist/
```

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  release:
    name: Build & Publish
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build (linux)
        run: cargo build --release
      - name: Build (windows)
        run: cargo build --release --target x86_64-pc-windows-msvc
      - name: Build (macos)
        run: cargo build --release --target x86_64-apple-darwin
      - name: Cook
        run: cargo run --release --bin engine -- cook --input assets --output release/assets.pak
      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            target/release/my_game
            target/x86_64-pc-windows-msvc/release/my_game.exe
            target/x86_64-apple-darwin/release/my_game
            release/assets.pak
```

## 5. 验收

- [ ] `engine build --release` CI < 5 min
- [ ] `engine cook` 输出独立运行 .pak
- [ ] CI 矩阵：ubuntu / macos / windows 全绿
- [ ] `engine profile` 火焰图 SVG
- [ ] `engine run` 一键运行
- [ ] `engine test` 透传 cargo test 参数
