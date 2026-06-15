# 模块六：Shader 与渲染管线需求

## 1. 模块概述

本模块提供图形渲染管线抽象，包括 `Shader` 着色器、`Pipeline` 渲染管线、`Buffer` 缓冲、`BindGroup` 绑定组、`VertexLayout` 顶点布局和 `Mesh2D` 2D 网格。

**核心目标**：提供灵活的渲染管线构建能力，支持 shader 热重载，内置 2D 着色器。

---

## 2. 需求清单

### 2.1 Shader / ShaderModule（需求 76-78, 102-106, 307-311）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 76 | Shader 源码/预编译 | `struct Shader { source: String, compiled: Option<CompiledShader> }` | - | - | 结构完整 | 需求 2 | P0 |
| 77 | Shader::from_source | `Shader::from_source(stage: ShaderStage, src: &str) -> Result<Self>` | stage, src | Result<Self> | 编译成功 | 需求 76 | P0 |
| 78 | ShaderModule | `struct ShaderModule { vertex, fragment, compute }` | - | - | 模块完整 | 需求 76 | P0 |
| 102 | 内置 sprite 着色器 | `sprite.vert / sprite.frag` | - | - | 2D 精灵着色 | 需求 84 | P0 |
| 103 | 内置纯色着色器 | `color.vert / color.frag` | - | - | 纯色绘制 | 需求 84 | P0 |
| 104 | 内置文本着色器 | `text.vert / text.frag` (留位) | - | - | 留位接口 | 需求 86 | P2 |
| 105 | Shader hot-reload | debug 模式监视文件修改 | - | - | 热重载 | 需求 77 | P1 |
| 307 | Shader::from_source | `Shader::from_source(stage: ShaderStage, source: &str) -> Result<Self>` | stage, source | Result<Self> | 同需求 77 | 需求 77 | P0 |
| 308 | Shader::from_file | `Shader::from_file(stage: ShaderStage, path: &str) -> Result<Self>` | stage, path | Result<Self> | 从文件加载 | 需求 307 | P0 |
| 309 | ShaderStage 枚举 | `enum ShaderStage { Vertex, Fragment, Compute }` | - | - | 正确枚举 | 需求 76 | P0 |
| 310 | ShaderModule::compile | `ShaderModule::compile(&self, entry: &str) -> Result<CompiledShader>` | entry | Result | 编译成功 | 需求 78 | P0 |
| 311 | ShaderModule::hot_reload | `ShaderModule::hot_reload(&mut self)` | - | - | 重新加载 | 需求 105 | P1 |

### 2.2 Pipeline / PipelineBuilder（需求 74-75, 100-101, 312-319, 360-368）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 74 | Pipeline 结构 | `struct Pipeline { shader, layout, blend, depth, stencil, rasterizer }` | - | - | 结构完整 | 需求 2 | P0 |
| 75 | PipelineBuilder | `struct PipelineBuilder` | - | - | 构建器完整 | 需求 74 | P0 |
| 100 | Pipeline 着色器 layout | shader + layout + blend + depth + stencil + rasterizer state | - | - | 状态完整 | 需求 74 | P0 |
| 312 | PipelineBuilder::new | `PipelineBuilder::new(shader_module: ShaderModule) -> Self` | shader_module | Self | 创建成功 | 需求 75 | P0 |
| 313 | 设置顶点布局 | `PipelineBuilder::with_vertex_layout(layout: VertexLayout) -> Self` | layout | Self | 链式调用 | 需求 312 | P0 |
| 314 | 设置混合模式 | `PipelineBuilder::with_blend_mode(mode: BlendMode) -> Self` | mode | Self | 链式调用 | 需求 69 | P0 |
| 315 | 设置深度测试 | `PipelineBuilder::with_depth_test(enabled: bool) -> Self` | enabled | Self | 链式调用 | 需求 74 | P0 |
| 316 | 设置剔除模式 | `PipelineBuilder::with_cull_mode(mode: CullMode) -> Self` | mode | Self | 链式调用 | 需求 74 | P0 |
| 317 | 设置环绕顺序 | `PipelineBuilder::with_winding_order(order: WindingOrder) -> Self` | order | Self | 链式调用 | 需求 74 | P0 |
| 318 | 构建 Pipeline | `PipelineBuilder::build(ctx: &RenderContext) -> Result<Pipeline>` | ctx | Result<Pipeline> | 构建成功 | 需求 312 | P0 |
| 319 | Pipeline::bind | `Pipeline::bind(&self, ctx: &mut RenderContext)` | ctx | - | 绑定管线 | 需求 318 | P0 |
| 360 | PipelineBuilder::new | `PipelineBuilder::new(shader_module)` | shader_module | Self | 同需求 312 | 需求 312 | P0 |
| 361 | with_vertex_layout | `PipelineBuilder::with_vertex_layout(layout)` | layout | Self | 同需求 313 | 需求 313 | P0 |
| 362 | with_blend_mode | `PipelineBuilder::with_blend_mode(mode)` | mode | Self | 同需求 314 | 需求 314 | P0 |
| 363 | with_depth_test | `PipelineBuilder::with_depth_test(enabled)` | enabled | Self | 同需求 315 | 需求 315 | P0 |
| 364 | with_cull_mode | `PipelineBuilder::with_cull_mode(mode)` | mode | Self | 同需求 316 | 需求 316 | P0 |
| 365 | with_winding_order | `PipelineBuilder::with_winding_order(order)` | order | Self | 同需求 317 | 需求 317 | P0 |
| 366 | build | `PipelineBuilder::build(ctx) -> Result<Pipeline>` | ctx | Result | 同需求 318 | 需求 318 | P0 |
| 367 | Pipeline::bind | `Pipeline::bind(ctx)` | ctx | - | 同需求 319 | 需求 319 | P0 |

### 2.3 Buffer / BufferUsage（需求 70-73, 96-99, 320-325, 368-373）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 70 | Buffer 顶点/索引/Uniform | `struct Buffer` | - | - | 缓冲抽象 | 需求 2 | P0 |
| 71 | Buffer::new | `Buffer::new(usage: BufferUsage, size_bytes: usize, data: Option<&[u8]>) -> Self` | usage, size, data | Self | 创建成功 | 需求 70 | P0 |
| 72 | Buffer::update | `Buffer::update(&mut self, offset: usize, data: &[u8])` | offset, data | - | 更新数据 | 需求 70 | P0 |
| 73 | Buffer::size | `Buffer::size(&self) -> usize` | - | usize | 返回大小 | 需求 70 | P0 |
| 96 | Buffer 顶点缓冲 | `Buffer::new_vertex(ctx, data) -> Result<Self>` | ctx, data | Self | 同需求 71 | 需求 71 | P0 |
| 97 | Buffer 索引缓冲 | `Buffer::new_index(ctx, data) -> Result<Self>` | ctx, data | Self | 同需求 71 | 需求 71 | P0 |
| 98 | Buffer Uniform 缓冲 | `Buffer::new_uniform(ctx, data) -> Result<Self>` | ctx, data | Self | 同需求 71 | 需求 71 | P0 |
| 99 | Buffer 更新 | `Buffer::update(offset, data)` | offset, data | - | 同需求 72 | 需求 72 | P0 |
| 320 | Buffer::new_vertex | `Buffer::new_vertex(ctx: &RenderContext, data: &[u8]) -> Result<Self>` | ctx, data | Result<Self> | 同需求 96 | 需求 96 | P0 |
| 321 | Buffer::new_index | `Buffer::new_index(ctx: &RenderContext, data: &[u8]) -> Result<Self>` | ctx, data | Result<Self> | 同需求 97 | 需求 97 | P0 |
| 322 | Buffer::new_uniform | `Buffer::new_uniform(ctx: &RenderContext, data: &[u8]) -> Result<Self>` | ctx, data | Result<Self> | 同需求 98 | 需求 98 | P0 |
| 323 | Buffer::update | `Buffer::update(&mut self, offset: usize, data: &[u8])` | offset, data | - | 同需求 99 | 需求 99 | P0 |
| 324 | Buffer::size | `Buffer::size(&self) -> usize` | - | usize | 同需求 73 | 需求 73 | P0 |
| 325 | BufferUsage 枚举 | `enum BufferUsage { Vertex, Index, Uniform, CopyDst }` | - | - | 正确枚举 | 需求 70 | P0 |
| 368 | Buffer::new_vertex | `Buffer::new_vertex(ctx, data) -> Result<Self>` | ctx, data | Result<Self> | 同需求 320 | 需求 320 | P0 |
| 369 | Buffer::new_index | `Buffer::new_index(ctx, data) -> Result<Self>` | ctx, data | Result<Self> | 同需求 321 | 需求 321 | P0 |
| 370 | Buffer::new_uniform | `Buffer::new_uniform(ctx, data) -> Result<Self>` | ctx, data | Result<Self> | 同需求 322 | 需求 322 | P0 |
| 371 | Buffer::update | `Buffer::update(offset, data)` | offset, data | - | 同需求 323 | 需求 323 | P0 |
| 372 | Buffer::size | `Buffer::size() -> usize` | - | usize | 同需求 324 | 需求 324 | P0 |
| 373 | BufferUsage | `enum BufferUsage { Vertex, Index, Uniform, CopyDst }` | - | - | 同需求 325 | 需求 325 | P0 |

### 2.4 BindGroup / BindGroupLayout（需求 79-80, 105, 326-327, 374-375）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 79 | BindGroup | `struct BindGroup { layout, resources }` | - | - | uniform + sampler | 需求 2 | P0 |
| 80 | BindGroupLayout | `struct BindGroupLayout` | - | - | 布局描述 | 需求 79 | P0 |
| 105 | BindGroup uniform + sampler | uniform + sampler 分组 | - | - | 分组正确 | 需求 79 | P0 |
| 326 | BindGroup::new | `BindGroup::new(ctx: &RenderContext, layout: &BindGroupLayout, resources: &[BindingResource]) -> Result<Self>` | ctx, layout, resources | Result<Self> | 创建成功 | 需求 79 | P0 |
| 327 | BindGroupLayout::new | `BindGroupLayout::new(ctx: &RenderContext, entries: &[BindingEntry]) -> Result<Self>` | ctx, entries | Result<Self> | 创建成功 | 需求 80 | P0 |
| 374 | BindGroup::new | `BindGroup::new(ctx, layout, resources) -> Result<Self>` | ctx, layout, resources | Result<Self> | 同需求 326 | 需求 326 | P0 |
| 375 | BindGroupLayout::new | `BindGroupLayout::new(ctx, entries) -> Result<Self>` | ctx, entries | Result<Self> | 同需求 327 | 需求 327 | P0 |

### 2.5 VertexLayout / VertexAttr / Mesh2D（需求 81-83, 107-109, 328-384, 376-385）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 81 | VertexLayout | `struct VertexLayout` | - | - | pos/uv/color/normal | 需求 2 | P0 |
| 82 | VertexAttr | `struct VertexAttr` | - | - | 属性描述 | 需求 81 | P0 |
| 83 | Mesh2D | `struct Mesh2D { vertices, indices }` | - | - | 顶点+索引 | 需求 2 | P0 |
| 107 | Mesh2D::draw | `Mesh2D::draw(&self, ctx: &mut RenderContext, pipeline: &Pipeline, bind_groups: &[&BindGroup])` | ctx, pipeline, bind_groups | - | 绘制调用 | 需求 83 | P0 |
| 328 | VertexLayout::new | `VertexLayout::new() -> Self` | - | Self | 创建成功 | 需求 81 | P0 |
| 329 | VertexLayout::push Vec2 | `VertexLayout::push::<Vec2>(&mut self, name: &str)` | name | &mut Self | 添加位置属性 | 需求 328 | P0 |
| 330 | VertexLayout::push Vec2 UV | `VertexLayout::push::<Vec2>(&mut self, name: &str)` (UV) | name | &mut Self | 添加 UV 属性 | 需求 328 | P0 |
| 331 | VertexLayout::push Color | `VertexLayout::push::<Color>(&mut self, name: &str)` | name | &mut Self | 添加颜色属性 | 需求 328 | P0 |
| 332 | VertexLayout::stride | `VertexLayout::stride(&self) -> usize` | - | usize | 返回步长 | 需求 328 | P0 |
| 333 | VertexLayout::attributes | `VertexLayout::attributes(&self) -> &[VertexAttr]` | - | &[VertexAttr] | 返回属性列表 | 需求 328 | P0 |
| 334 | Mesh2D::new | `Mesh2D::new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self` | vertices, indices | Self | 创建成功 | 需求 83 | P0 |
| 335 | Mesh2D::quad | `Mesh2D::quad(w: f32, h: f32, color: Color) -> Self` | w, h, color | Self | 创建四边形 | 需求 334 | P0 |
| 336 | Mesh2D::draw | `Mesh2D::draw(&self, ctx: &mut RenderContext, pipeline: &Pipeline, bind_groups: &[&BindGroup])` | ctx, pipeline, bind_groups | - | 同需求 107 | 需求 107 | P0 |
| 376 | VertexLayout::new | `VertexLayout::new()` | - | Self | 同需求 328 | 需求 328 | P0 |
| 377 | push::<Vec2> "aPos" | `VertexLayout::push::<Vec2>("aPos")` | - | &mut Self | 同需求 329 | 需求 329 | P0 |
| 378 | push::<Vec2> "aUv" | `VertexLayout::push::<Vec2>("aUv")` | - | &mut Self | 同需求 330 | 需求 330 | P0 |
| 379 | push::<Color> "aColor" | `VertexLayout::push::<Color>("aColor")` | - | &mut Self | 同需求 331 | 需求 331 | P0 |
| 380 | stride | `VertexLayout::stride() -> usize` | - | usize | 同需求 332 | 需求 332 | P0 |
| 381 | attributes | `VertexLayout::attributes() -> &[VertexAttr]` | - | &[VertexAttr] | 同需求 333 | 需求 333 | P0 |
| 382 | Mesh2D::new | `Mesh2D::new(vertices, indices) -> Self` | vertices, indices | Self | 同需求 334 | 需求 334 | P0 |
| 383 | Mesh2D::quad | `Mesh2D::quad(w, h, color) -> Self` | w, h, color | Self | 同需求 335 | 需求 335 | P0 |
| 384 | Mesh2D::draw | `Mesh2D::draw(ctx, pipeline, bind_groups)` | ctx, pipeline, bind_groups | - | 同需求 336 | 需求 336 | P0 |
| 385 | BlendMode::Alpha GL | `BlendMode::to_gl_enum(&self) -> u32` (Alpha) | - | u32 | GL 混合枚举 | 需求 306 | P1 |

---

## 3. 验收标准

### 3.1 功能验收

- [ ] `Shader::from_source / from_file` 正确编译 GLSL/HLSL
- [ ] `PipelineBuilder` 可构建完整渲染管线
- [ ] `Buffer` 支持顶点/索引/Uniform 缓冲创建和更新
- [ ] `BindGroup` 正确绑定 uniform 和 sampler
- [ ] `VertexLayout` 正确描述顶点格式
- [ ] `Mesh2D` 支持创建和绘制

### 3.2 质量验收

- [ ] `VertexLayout::stride` 单元测试通过（需求 369）
- [ ] `BlendMode::Alpha::to_gl_enum` 单元测试通过（需求 370）
- [ ] clippy 无 warning
- [ ] fmt 检查通过

---

## 4. 依赖关系图

```
┌─────────────────────────────────────┐
│           ShaderModule               │
│  ├── vertex / fragment / compute    │
│  ├── compile / hot_reload          │
│  └── from_source / from_file       │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│          PipelineBuilder             │
│  ├── with_vertex_layout            │
│  ├── with_blend_mode               │
│  ├── with_depth_test               │
│  ├── with_cull_mode                │
│  └── build -> Pipeline              │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│            Pipeline                  │
│  ├── shader / layout               │
│  ├── blend / depth / stencil       │
│  └── rasterizer state              │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│      BindGroup / BindGroupLayout    │
│  ├── uniform bindings              │
│  └── sampler bindings              │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│             Buffer                  │
│  ├── new_vertex / new_index        │
│  ├── new_uniform                   │
│  └── update / size                 │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│          VertexLayout                │
│  ├── push::<Vec2>("aPos")          │
│  ├── push::<Vec2>("aUv")           │
│  ├── push::<Color>("aColor")       │
│  └── stride / attributes           │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│             Mesh2D                   │
│  ├── new(vertices, indices)       │
│  ├── quad(w, h, color)             │
│  └── draw(ctx, pipeline, groups)  │
└─────────────────────────────────────┘
```

---

## 5. 备注

- Shader 热重载仅在 debug 模式下启用
- 内置 shader 包括 `sprite.vert/sprite.frag` 和 `color.vert/color.frag`
- 2D 渲染使用简单的顶点格式（位置 + UV + 颜色）
- Mesh2D 内部使用三角形绘制，quad 方法创建 2 三角形矩形