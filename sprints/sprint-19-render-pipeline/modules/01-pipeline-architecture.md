# Module 01 — 渲染管线架构与 CommandList 抽象

> 上游 sprint: [Sprint 19](../sprint-19-render-pipeline.md)
> 文件位置: `engine-render-v2/src/pipeline/mod.rs`

---

## 1. 目标

建立 **跨后端** 渲染管线抽象：
- `RenderPipeline` trait
- `RenderGraph` 渲染图（pass 节点 + 资源依赖）
- `CommandList` 跨后端命令列表（OpenGL / Vulkan / WebGPU）

## 2. RenderPipeline Trait

```rust
pub trait RenderPipeline: Send + Sync {
    fn name(&self) -> &str;
    fn setup(&mut self, ctx: &mut RenderContext);
    fn execute(&mut self, ctx: &mut RenderContext, commands: &mut CommandList);
    fn cleanup(&mut self, ctx: &mut RenderContext);
}
```

## 3. RenderGraph

```rust
pub struct RenderGraph {
    pub passes: Vec<RenderPassNode>,
    pub resources: HashMap<ResourceHandle, ResourceNode>,
}

pub struct RenderPassNode {
    pub name: String,
    pub inputs: Vec<ResourceHandle>,
    pub outputs: Vec<ResourceHandle>,
    pub pipeline: Box<dyn RenderPipeline>,
}

impl RenderGraph {
    pub fn add_pass(&mut self, name: &str, pipeline: Box<dyn RenderPipeline>) -> PassHandle;
    pub fn connect(&mut self, pass: PassHandle, input: ResourceHandle, resource: ResourceHandle);
    pub fn compile(&mut self) -> Vec<CompiledPass>;
    pub fn execute(&mut self, ctx: &mut RenderContext);
}
```

## 4. CommandList

```rust
pub struct CommandList {
    pub commands: Vec<Command>,
}

pub enum Command {
    BeginPass { name: String, color: Vec<ColorTarget>, depth: Option<DepthTarget> },
    EndPass,
    Draw { mesh: MeshHandle, material: MaterialHandle, transform: Mat4 },
    DrawIndexed { mesh: MeshHandle, material: MaterialHandle, transform: Mat4, count: u32 },
    DispatchCompute { shader: ComputeHandle, groups: (u32, u32, u32) },
    SetPipeline(PipelineStateHandle),
    SetBindGroup { slot: u32, group: BindGroupHandle },
    SetUniform { name: String, data: Vec<u8> },
    Blit { src: TextureHandle, dst: TextureHandle },
    Barrier(ResourceHandle),
}

impl CommandList {
    pub fn new() -> Self;
    pub fn draw(&mut self, mesh: MeshHandle, material: MaterialHandle, transform: Mat4);
    pub fn dispatch(&mut self, shader: ComputeHandle, groups: (u32, u32, u32));
    pub fn barrier(&mut self, resource: ResourceHandle);
    pub fn submit(self, ctx: &mut RenderContext);
}
```

## 5. GPU 资源

```rust
pub struct GpuDevice { /* 平台分发 */ }

impl GpuDevice {
    pub fn create_buffer(&self, desc: &BufferDesc) -> GpuBuffer;
    pub fn create_texture(&self, desc: &TextureDesc) -> GpuTexture;
    pub fn create_shader(&self, source: &ShaderSource) -> GpuShader;
    pub fn create_pipeline_state(&self, desc: &PipelineDesc) -> GpuPipelineState;
    pub fn create_sampler(&self, desc: &SamplerDesc) -> GpuSampler;
}
```

## 6. 跨后端实现

- `gl_backend.rs` — OpenGL 4.5
- `vk_backend.rs` — Vulkan 1.3
- `wgpu_backend.rs` — WebGPU（wasm + native）

## 7. 验收

- [ ] CommandList 跨后端可移植
- [ ] 渲染图 pass 拓扑自动排序
- [ ] 资源生命周期：Create / Submit / Drop
- [ ] cargo test 100% pass
- [ ] cargo bench：1000 draw call < 5 ms CPU
