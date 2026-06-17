# Sprint 19 · API 参考

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)

## RenderPipeline

```rust
pub trait RenderPipeline: Send + Sync {
    fn name(&self) -> &str;
    fn setup(&mut self, ctx: &mut RenderContext);
    fn execute(&mut self, ctx: &mut RenderContext, commands: &mut CommandList);
    fn cleanup(&mut self, ctx: &mut RenderContext);
}
```

## RenderGraph

```rust
pub struct RenderGraph;
impl RenderGraph {
    pub fn new() -> Self;
    pub fn add_pass(&mut self, name: &str, pipeline: Box<dyn RenderPipeline>) -> PassHandle;
    pub fn connect(&mut self, pass: PassHandle, slot: ResourceSlot, resource: ResourceHandle);
    pub fn compile(&mut self);
    pub fn execute(&mut self, ctx: &mut RenderContext);
}
```

## CommandList

```rust
pub struct CommandList;
impl CommandList {
    pub fn new() -> Self;
    pub fn begin_pass(&mut self, name: &str, color: Vec<ColorTarget>, depth: Option<DepthTarget>);
    pub fn end_pass(&mut self);
    pub fn draw(&mut self, mesh: MeshHandle, material: MaterialHandle, transform: Mat4);
    pub fn draw_indexed(&mut self, mesh: MeshHandle, material: MaterialHandle, transform: Mat4, count: u32);
    pub fn dispatch(&mut self, shader: ComputeHandle, groups: (u32, u32, u32));
    pub fn set_pipeline(&mut self, state: PipelineStateHandle);
    pub fn set_bind_group(&mut self, slot: u32, group: BindGroupHandle);
    pub fn barrier(&mut self, resource: ResourceHandle);
    pub fn blit(&mut self, src: TextureHandle, dst: TextureHandle);
    pub fn submit(self, ctx: &mut RenderContext);
}
```

## GPU 设备与资源

```rust
pub struct GpuDevice;
impl GpuDevice {
    pub fn create_buffer(&self, desc: &BufferDesc) -> GpuBuffer;
    pub fn create_texture(&self, desc: &TextureDesc) -> GpuTexture;
    pub fn create_shader(&self, source: &ShaderSource) -> GpuShader;
    pub fn create_pipeline_state(&self, desc: &PipelineDesc) -> GpuPipelineState;
    pub fn create_sampler(&self, desc: &SamplerDesc) -> GpuSampler;
}

pub struct GpuBuffer;
pub struct GpuTexture;
pub struct GpuShader;
pub struct GpuPipelineState;
pub struct GpuSampler;
```

## Forward+ Cluster

```rust
pub struct ClusterGrid { pub screen_x: u32, pub screen_y: u32, pub depth_z: u32, pub near: f32, pub far: f32 }
pub struct ClusterBuffer;
pub struct DepthPyramid;
```

## PBR 材质

```rust
pub struct PbrMaterial { /* 详见 module 03 */ }
pub enum AlphaMode { Opaque, Mask { cutoff: f32 }, Blend }
pub struct MaterialManager3D;
pub struct IblProbe;
```

## 阴影

```rust
pub struct CascadedShadowMap;
pub enum CascadeSplitScheme { Linear, Logarithmic, PSSM { lambda: f32 }, Manual(Vec<f32>) }
```

## 后处理

```rust
pub struct HdrTarget;
pub struct FxaaPass;
pub struct TaaPass;
pub struct Exposure;
pub struct HaltonSequence;
```

## 顶层 Re-export

```rust
pub use pipeline::{RenderPipeline, RenderContext, RenderGraph, RenderPass};
pub use gpu::{GpuDevice, GpuBuffer, GpuTexture, GpuShader, GpuPipelineState};
pub use material::pbr::PbrMaterial;
pub use lighting::cluster::{ClusterGrid, ClusteredLighting};
pub use shadow::csm::CascadedShadowMap;
pub use postprocess::{HdrTarget, ToneMap, FxaaPass, TaaPass};
```
