# 示例实现指南

## 概述

本文档提供 Sprint 10 各模块的示例实现指南，包括 `examples/pbr_materials`、`examples/pbr_editor`、`examples/pbr_ibl` 等示例项目的实现细节。

**对应原需求编号**：97-107, 416-428

---

## 1. examples/pbr_materials

### 1.1 功能需求

| 需求编号 | 功能 | 说明 |
|----------|------|------|
| 97, 416 | 展示多种材质球 | Metal/Plastic/Ceramic/Wood/Concrete/Fabric/Gold/Copper/Leather/CarPaint/Rubber/Brushed Metal |
| 98, 417 | 鼠标旋转查看 | Orbit controls |
| 99, 418 | 切换环境贴图 | 至少 2 个环境贴图 |

### 1.2 材质球 Demo 实现

```rust
// 示例：创建不同类型的 PBR 材质

// 金属材质
fn create_metal_material(renderer: &Renderer) -> PbrMaterial {
    PbrMaterial::from_albedo(Color::rgb(0.95, 0.93, 0.88))
        .with_metallic(1.0)
        .with_roughness(0.1)
}

// 塑料材质
fn create_plastic_material(renderer: &Renderer) -> PbrMaterial {
    PbrMaterial::from_albedo(Color::rgb(0.8, 0.2, 0.2))
        .with_metallic(0.0)
        .with_roughness(0.4)
}

// 金材质
fn create_gold_material(renderer: &Renderer) -> PbrMaterial {
    PbrMaterial::from_albedo(Color::rgb(1.0, 0.84, 0.0))
        .with_metallic(1.0)
        .with_roughness(0.2)
}

// 车漆材质
fn create_car_paint_material(renderer: &Renderer) -> PbrMaterial {
    PbrMaterial::from_albedo(Color::rgb(0.1, 0.1, 0.8))
        .with_metallic(0.9)
        .with_roughness(0.15)
        .with_clear_coat(1.0)
        .with_clear_coat_roughness(0.1)
}

// 皮革材质
fn create_leather_material(renderer: &Renderer) -> PbrMaterial {
    PbrMaterial::from_albedo(Color::rgb(0.4, 0.25, 0.15))
        .with_metallic(0.0)
        .with_roughness(0.6)
        .with_normal_strength(0.5)
}

// 橡胶材质
fn create_rubber_material(renderer: &Renderer) -> PbrMaterial {
    PbrMaterial::from_albedo(Color::rgb(0.1, 0.1, 0.1))
        .with_metallic(0.0)
        .with_roughness(0.9)
}
```

### 1.3 场景设置

```rust
struct PbrMaterialsScene {
    camera: Camera,
    materials: Vec<PbrMaterial>,
    sphere_mesh: Handle<Mesh>,
    environment: EnvironmentMap,
}

impl PbrMaterialsScene {
    pub fn new(renderer: &Renderer) -> Result<Self> {
        // 创建相机
        let camera = Camera::new_perspective(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            Vec3::Y,
            45.0,
            1.0,
        );

        // 加载环境贴图
        let environment = EnvironmentMap::from_hdr("assets/env/studio.hdr")?;

        // 创建球体网格
        let sphere_mesh = renderer.load_mesh("primitives/sphere.obj");

        // 创建多种材质
        let materials = vec![
            create_metal_material(renderer),
            create_plastic_material(renderer),
            create_gold_material(renderer),
            // ... 更多材质
        ];

        Ok(Self { camera, materials, sphere_mesh, environment })
    }
}
```

### 1.4 渲染循环

```rust
impl Scene for PbrMaterialsScene {
    fn update(&mut self, dt: f32) {
        // 更新 Orbit 相机
        self.camera.orbit(self.mouse_dx, self.mouse_dy);
    }

    fn render(&self, renderer: &mut Renderer) {
        // 设置环境光照
        renderer.set_environment(&self.environment);

        // 渲染每个材质球
        for (i, material) in self.materials.iter().enumerate() {
            let x = (i % 4) as f32 * 2.5 - 5.0;
            let y = (i / 4) as f32 * 2.5 - 2.5;
            let transform = Transform::translation(x, y, 0.0);

            renderer.draw_mesh(&self.sphere_mesh, material, &transform);
        }
    }
}
```

---

## 2. examples/pbr_editor

### 2.1 功能需求

| 需求编号 | 功能 | 说明 |
|----------|------|------|
| 100, 419 | 材质节点编辑器 | 创建 ShaderGraph -> 实时预览 |

### 2.2 编辑器实现

```rust
struct PbrEditorScene {
    graph: Handle<ShaderGraph>,
    editor: ShaderGraphEditor,
    preview_renderer: PreviewRenderer,
    material_ball: Handle<Mesh>,
}

impl PbrEditorScene {
    pub fn new(renderer: &Renderer) -> Result<Self> {
        // 创建空 ShaderGraph
        let graph = ShaderGraph::new();
        let graph = renderer.register(graph);

        // 创建编辑器
        let editor = ShaderGraphEditor::new();

        // 创建预览渲染器
        let preview_renderer = PreviewRenderer::new(renderer);

        // 创建材质球网格
        let material_ball = renderer.load_mesh("primitives/material_ball.obj");

        Ok(Self { graph, editor, preview_renderer, material_ball })
    }
}
```

### 2.3 添加默认 PBR 节点

```rust
fn setup_default_graph(graph: &mut ShaderGraph) {
    // 添加输入节点
    let albedo_input = graph.add_node(NodeKind::Input("Albedo".into(), Type::Color3));
    let metallic_input = graph.add_node(NodeKind::Input("Metallic".into(), Type::Scalar));
    let roughness_input = graph.add_node(NodeKind::Input("Roughness".into(), Type::Scalar));

    // 添加 PBR Master 节点
    let pbr_master = graph.add_node(NodeKind::PbrMaster(PbrMasterInputs {
        albedo: albedo_input,
        normal: None,
        metallic: metallic_input,
        roughness: roughness_input,
        ao: None,
        emissive: None,
        alpha: None,
        clear_coat: None,
        clear_coat_rough: None,
        sheen: None,
        sheen_rough: None,
        subsurface: None,
        anisotropy: None,
    }));

    // 连接节点
    graph.add_edge(albedo_input, pbr_master, "albedo");
    graph.add_edge(metallic_input, pbr_master, "metallic");
    graph.add_edge(roughness_input, pbr_master, "roughness");
}
```

### 2.4 预览更新

```rust
impl Scene for PbrEditorScene {
    fn render(&self, renderer: &mut Renderer) {
        // 编译 ShaderGraph
        if let Ok(source) = self.graph.compile() {
            // 编译 Shader
            let shader = ShaderCompiler::new().compile(&source, ShaderLanguage::WGSL, ShaderStage::Fragment)?;
            
            // 更新预览材质
            self.preview_renderer.update_shader(shader);
        }

        // 渲染材质球预览
        self.preview_renderer.draw_material_ball(renderer, &self.material_ball);
    }
}
```

---

## 3. examples/pbr_ibl

### 3.1 功能需求

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 101, 420 | HDR 环境贴图 + IBL | 展示 IBL 效果 |

### 3.2 实现

```rust
struct PbrIblScene {
    camera: Camera,
    environment: EnvironmentMap,
    baker: IBLBaker,
    skybox_renderer: SkyboxRenderer,
    scene: Scene,
}

impl PbrIblScene {
    pub fn new(renderer: &Renderer) -> Result<Self> {
        // 从 HDR 加载环境贴图
        let environment = EnvironmentMap::from_hdr("assets/env/kloofendal.hdr")?;

        // 创建 IBL Baker
        let baker = IBLBaker::new(renderer);

        // 烘焙 IBL 贴图（开发期实时烘焙）
        let irradiance = baker.bake_irradiance(&environment);
        let prefilter = baker.bake_prefilter(&environment, 8);
        let brdf_lut = baker.bake_brdf_lut(512);

        environment.set_irradiance(irradiance);
        environment.set_prefilter(prefilter);
        environment.set_brdf_lut(brdf_lut);

        // 创建天空盒渲染器
        let skybox_renderer = SkyboxRenderer::new(renderer)?;
        skybox_renderer.set_skybox_texture(environment.skybox());

        Ok(Self { camera, environment, baker, skybox_renderer, scene })
    }
}
```

---

## 4. examples/pbr_tonemap

### 4.1 功能需求

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 102, 421 | 切换色调映射算子 | ACES / Reinhard / Filmic / None |

### 4.2 实现

```rust
struct PbrTonemapScene {
    tonemapper: Tonemapper,
    color_grading: ColorGrading,
    hdr_pipeline: HdrPipeline,
}

impl PbrTonemapScene {
    pub fn new(renderer: &Renderer) -> Result<Self> {
        let tonemapper = Tonemapper::Aces;
        let color_grading = ColorGrading::default();
        let hdr_pipeline = HdrPipeline::new(renderer, [1920, 1080])?;

        Ok(Self { tonemapper, color_grading, hdr_pipeline })
    }

    fn switch_tonemapper(&mut self, index: usize) {
        self.tonemapper = match index {
            0 => Tonemapper::Aces,
            1 => Tonemapper::Reinhard,
            2 => Tonemapper::Filmic,
            3 => Tonemapper::None,
            _ => return,
        };
    }
}
```

---

## 5. examples/pbr_shadow

### 5.1 功能需求

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 103, 422 | 方向光与软阴影演示 | PCF 阴影 |

### 5.2 实现

```rust
struct PbrShadowScene {
    shadow_pass: ShadowMapPass,
    quality: ShadowQuality,
    light: Light,
}

impl PbrShadowScene {
    pub fn new(renderer: &Renderer) -> Result<Self> {
        let shadow_pass = ShadowMapPass::new(renderer, 2048)?;
        let quality = ShadowQuality::High;
        let light = Light::directional(Vec3::new(1.0, 1.0, 1.0), Color::WHITE, 1.0);

        Ok(Self { shadow_pass, quality, light })
    }
}
```

---

## 6. examples/pbr_normal_map

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 104, 423 | 法线贴图演示 | 砖墙/皮肤等法线贴图效果 |

---

## 7. examples/pbr_emissive

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 105, 424 | 自发光演示 | 霓虹灯/发光文字效果 |

---

## 8. examples/pbr_parallax

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 106, 425 | 视差贴图演示 | POM 效果 |

---

## 9. examples/pbr_clear_coat

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 107, 426 | 清漆效果演示 | 车漆/清漆层 |

---

## 10. examples/pbr_subsurface

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 427 | 次表面散射 | 皮肤/蜡烛等 SSS 效果 |

---

## 11. examples/pbr_anisotropy

| 需求编号 | 功能 | 说明 |
|----------|------|----------|
| 428 | 各向异性 | 拉丝金属/头发等 |

---

## 12. 材质 JSON 格式示例

```json
{
  "version": "1.0",
  "name": "GoldMaterial",
  "albedo": [1.0, 0.84, 0.0],
  "albedo_map": null,
  "metallic": 1.0,
  "metallic_map": null,
  "roughness": 0.2,
  "roughness_map": null,
  "normal_map": "textures/gold_normal.png",
  "normal_strength": 1.0,
  "ao_map": null,
  "ao_strength": 1.0,
  "emissive": [0.0, 0.0, 0.0],
  "emissive_map": null,
  "emissive_intensity": 1.0,
  "height_map": null,
  "parallax_strength": 0.05,
  "clear_coat": 0.0,
  "clear_coat_roughness": 0.0,
  "anisotropy": 0.0,
  "sheen": [1.0, 1.0, 1.0],
  "sheen_roughness": 0.0,
  "subsurface": 0.0,
  "alpha_mode": "Opaque",
  "alpha_cutoff": 0.5,
  "double_sided": false,
  "casts_shadow": true,
  "receives_shadow": true
}
```

---

## 13. 性能基准

| 需求编号 | 指标 | 说明 |
|----------|------|------|
| 175 | 60fps | 典型场景（30 个 PBR mesh），GTX 1660 级 GPU |

```rust
// 性能测试场景设置
struct PerformanceBenchmark {
    mesh_count: usize,
    target_fps: f32,
}

impl PerformanceBenchmark {
    fn run(renderer: &Renderer) -> bool {
        let scene = Self::create_scene(30);
        let mut frame_times = Vec::new();

        for _ in 0..120 {
            let start = std::time::Instant::now();
            renderer.render(&scene);
            let elapsed = start.elapsed().as_secs_f32();
            frame_times.push(elapsed);
        }

        let avg_frame_time = frame_times.iter().sum::<f32>() / frame_times.len() as f32;
        let avg_fps = 1.0 / avg_frame_time;

        avg_fps >= 60.0
    }
}
```
