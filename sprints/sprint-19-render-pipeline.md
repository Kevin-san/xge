# Sprint 19 · 渲染管线 v2（Forward+ / PBR / 延迟路径 / 阴影 / 后处理）

> 文档编号: `sprint-19-render-pipeline.md / v1.0
> 周期: 5 周 (25 个工作日)
> 上游依赖: Sprint 17 (Math SIMD), Sprint 18 (ECS)
> 下游交付: Sprint 21 (Particle / PostFX), Sprint 22 (Editor)

---

## 1. 目标与范围

**目标：** 将 `engine-render` (2D) 与 `engine-render-3d` (3D) 整合为 **现代 GPU 驱动渲染管线**，实现 Forward+ 集群剔除、PBR 主材质、级联阴影、HDR 渲染目标、通用 CommandList 提交。

**范围：**
- ✅ Forward+ 集群光照（Clustered Forward）
- ✅ PBR 材质（金属/粗糙度/IBL）
- ✅ 级联阴影映射（CSM，Cascaded Shadow Maps）
- ✅ HDR + Tone Mapping（ACES Filmic）
- ✅ MSAA / FXAA / TAA
- ✅ 渲染管线 CommandList 抽象（替代 GL Backend 拼装）
- ⛔ 不含：Lumen 全局光照、Nanite 虚拟几何（研究项）、光线追踪、Path Tracer

**核心参考：** Unity SRP / Unreal Forward Shading / Godot RenderingDevice / bgfx。

---

## 2. 上游需求对接

| 来源 | 关联章节 | 承接 |
|------|---------|------|
| [NEXT_PHASE_REQUIREMENTS.md § 6.1](../NEXT_PHASE_REQUIREMENTS.md) | 渲染管线总架构 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 6.2](../NEXT_PHASE_REQUIREMENTS.md) | Forward+ Cluster | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 6.3](../NEXT_PHASE_REQUIREMENTS.md) | PBR + IBL | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 6.4](../NEXT_PHASE_REQUIREMENTS.md) | 阴影系统 | ✅ 本 sprint |
| [NEXT_PHASE_REQUIREMENTS.md § 5 M3](../NEXT_PHASE_REQUIREMENTS.md) | Milestone M3 | ✅ 本 sprint |

---

## 3. 子模块拆分

### 3.1 [01-pipeline-architecture.md](modules/01-pipeline-architecture.md) — 管线架构

**核心交付：**
- `engine-render-v2/src/pipeline/mod.rs`
  - `RenderPipeline` trait
  - `RenderContext` 渲染上下文（GPU 设备、CommandEncoder、Swapchain）
  - `RenderGraph` 渲染图（pass 节点 + 资源依赖）
  - `RenderPass` 各种 pass 接口：`Forward`, `Shadow`, `PostFx`, `Present`
- `engine-render-v2/src/pipeline/command.rs`
  - `CommandList` 跨后端命令列表（OpenGL / Vulkan / WebGPU）
  - `CommandEncoder` 编码器
  - `DrawCall`, `DispatchCall`, `BlitCall`
- `engine-render-v2/src/pipeline/gpu.rs`
  - `GpuDevice` 抽象
  - `GpuBuffer`, `GpuTexture`, `GpuSampler`, `GpuShader`, `GpuPipelineState`
  - 资源生命周期（Create/Submit/Drop）

**Bug 修复对应：** `engine-render-3d/src/{pipeline,gl_backend}.rs` RenderPipeline3D 空壳

**验收：**
- CommandList 跨后端可移植（GL / Vulkan 编译期分支）
- 渲染图 pass 拓扑自动排序
- `cargo test` 100% pass

---

### 3.2 [02-forward-plus-cluster.md](modules/02-forward-plus-cluster.md) — Forward+ 集群光照

**核心交付：**
- `engine-render-v2/src/lighting/cluster.rs`
  - `ClusterGrid { sx: u32, sy: u32, sz: u32 }` 3D 网格（如 16×16×32）
  - `Cluster { min, max: Vec3, light_indices: Vec<u32> }`
  - `build_clusters(frustum, lights, depth_pyramid)` CPU 端构建
  - ClusteredLightingPass GPU 端 shader
- `engine-render-v2/src/lighting/light_culling.rs`
  - 光源 AABB 计算（point/spot/directional）
  - Frustum 与光源相交测试
- `engine-render-v2/src/lighting/light_buffer.rs`
  - 帧间光源 uniform buffer
  - 光源数量上限：4 方向光 + 256 点光 + 64 聚光

**验收：**
- 1024 光源场景（512 动态） 60 FPS @ 1080p
- Cluster 数量 16×16×32 = 8192，光源归属开销 < 1 ms CPU
- GPU cluster 评估 < 0.5 ms

---

### 3.3 [03-pbr-material-ibl.md](modules/03-pbr-material-ibl.md) — PBR + IBL

**核心交付：**
- `engine-render-v2/src/material/pbr.rs`
  - `PbrMaterial { albedo, metallic, roughness, ao, normal, emissive, occlusion }`
  - PBR Shader（GGX BRDF，UE4 / Unity 标准模型）
  - Alpha Mode：Opaque / Mask / Blend
  - 双面渲染标志
- `engine-render-v2/src/material/ibl.rs`
  - 立方贴图 IBL：辐射率 + 辐照度预过滤
  - SH（球谐）环境光（9 阶系数 / 27 阶 L2）
  - IBL Prefilter Pass：mipmap 链
- `engine-render-v2/src/material/texture.rs`
  - 多纹理槽位绑定（绑定表）
  - 纹理流式加载

**验收：**
- Disney BRDF 视觉对比参考：差异 < 5%（sRGB 像素）
- IBL 烘焙示例：HDR EXR → 立方贴图
- 材质编辑：实时 uniform 更新 < 16 µs

---

### 3.4 [04-cascaded-shadows.md](modules/04-cascaded-shadows.md) — CSM 阴影

**核心交付：**
- `engine-render-v2/src/shadow/csm.rs`
  - `CascadeSplitter` 分割算法：logarithmic / PSSM / manual
  - 4 级联（默认 0.1 / 1 / 10 / 50 米）
  - 视锥拟合 + 稳定化
- `engine-render-v2/src/shadow/shadow_map.rs`
  - 阴影贴图 Atlas（4 层级联打包到一张 4096×4096 贴图）
  - Shadow Pass：depth-only 渲染
  - PCF 软阴影 / VSM 方差阴影
- `engine-render-v2/src/shadow/shader_integration.rs`
  - PBR Shader 集成：阴影级联采样、cascade 切换、edge softening
  - `shadow_normal_offset` 自阴影缓解

**验收：**
- 4 级联阴影 @ 4096² 总贴图 < 2 ms GPU
- 自阴影瑕疵 < 0.1% 像素
- 远距离阴影过渡平滑

---

### 3.5 [05-hdr-tone-mapping.md](modules/05-hdr-tone-mapping.md) — HDR 渲染目标

**核心交付：**
- `engine-render-v2/src/postprocess/hdr.rs`
  - `HdrTarget { r16g16b16a16_float }` HDR 渲染目标
  - 多重渲染目标 (MRT)：albedo / normal / material / depth
- `engine-render-v2/src/postprocess/tonemap.rs`
  - ACES Filmic Tone Mapping
  - 暴露控制（exposure）
  - Gamma 校正（sRGB 输出）
- `engine-render-v2/src/postprocess/fxaa.rs`
  - FXAA 1.x 抗锯齿
- `engine-render-v2/src/postprocess/taa.rs` (optional)
  - Temporal AA：jitter + history buffer
  - Halton sequence jitter

**验收：**
- HDR 渲染目标线性空间，gamma 2.2 输出
- ACES Filmic 视觉对比
- FXAA 边缘平滑无 ghosting

---

## 4. 验收清单（acceptance/test-plan.md）

- [ ] Forward+ 集群：1024 光源 60 FPS @ 1080p
- [ ] PBR 球体展示 5 个金属/粗糙度组合视觉正确
- [ ] IBL 立方贴图加载与预过滤 < 100 ms
- [ ] CSM 4 级联阴影：稳定 + 软过渡
- [ ] HDR + ACES 视觉对比：参考图匹配
- [ ] FXAA 无 ghosting
- [ ] CommandList 跨后端编译通过
- [ ] `cargo test` 全通过
- [ ] `cargo bench` 基准存档
- [ ] 示例程序：`pbr_showcase`, `csm_demo`, `cluster_demo`, `hdr_demo`

---

## 5. API 稳定承诺

```rust
pub use pipeline::{RenderPipeline, RenderContext, RenderGraph, RenderPass};
pub use gpu::{GpuDevice, GpuBuffer, GpuTexture, GpuShader, GpuPipelineState};
pub use material::pbr::PbrMaterial;
pub use lighting::cluster::{ClusterGrid, ClusteredLighting};
pub use shadow::csm::CascadedShadowMap;
pub use postprocess::{HdrTarget, ToneMap, FxaaPass, TaaPass};
```

---

## 6. 与上下游依赖

| 依赖 | 来自 | 用途 |
|------|------|------|
| `Mat4::inverse_transpose` | sprint-17 | 法线矩阵 |
| `Frustum::classify_aabb_batch` | sprint-17 | 集群构建 |
| `World`, `Query<(&Mesh3D, &PbrMaterial)>` | sprint-18 | 实体遍历 |
| `Resource<GpuDevice>` | sprint-18 | 注入 GPU 设备 |

---

## 7. 风险与缓解

| 风险 | 等级 | 缓解 |
|------|------|------|
| WebGPU 后端复杂度 | 高 | 第一阶段只完成 OpenGL，第二阶段 Vulkan/WebGPU |
| 阴影瑕疵（Peter Panning / acne） | 中 | NORMAL_OFFSET + 稳定化 |
| PBR 视觉与参考差异 | 中 | 参考 Unity/UE 截图对比 |
| IBL 烘焙时间 | 中 | 异步预过滤 + 缓存 |
