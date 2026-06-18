# 3D 渲染 API 参考清单

## 概述

本文档列出 Sprint 09 中 `engine-render-3d` crate 的所有公开 API，按模块分组。

---

## 1. Mesh 与顶点数据 API

### 1.1 Vertex

```rust
/// 创建顶点
pub fn Vertex::new(pos: Vec3, normal: Vec3, uv: Vec2) -> Vertex

/// 获取位置
pub fn Vertex::position(&self) -> Vec3

/// 获取法线
pub fn Vertex::normal(&self) -> Vec3

/// 获取 UV
pub fn Vertex::texcoord(&self) -> Vec2

/// 顶点布局常量
pub struct VertexLayout;
pub const VertexLayout::POS3F_NORMAL3F_UV2F: VertexLayout
```

### 1.2 VertexBuffer / IndexBuffer

```rust
/// 创建顶点缓冲
pub fn VertexBuffer::new(renderer: &Renderer, vertices: &[Vertex]) -> Self

/// 绑定顶点缓冲
pub fn VertexBuffer::bind(&self, renderer: &Renderer)

/// 获取字节大小
pub fn VertexBuffer::size_bytes(&self) -> usize

/// 创建索引缓冲
pub fn IndexBuffer::new(renderer: &Renderer, indices: &[u16]) -> Self
pub fn IndexBuffer::new(renderer: &Renderer, indices: &[u32]) -> Self

/// 绑定索引缓冲
pub fn IndexBuffer::bind(&self, renderer: &Renderer)

/// 获取索引数量
pub fn IndexBuffer::index_count(&self) -> usize

/// 索引格式
pub enum IndexFormat {
    U16,
    U32,
}
```

### 1.3 Mesh3D

```rust
/// 创建网格
pub fn Mesh3D::new(vertex_buffer: VertexBuffer, index_buffer: IndexBuffer, primitives: Vec<Primitive>) -> Self

/// 从顶点索引创建
pub fn Mesh3D::from_vertices(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self

/// 从文件加载（GLTF/GLB）
pub fn Mesh3D::from_file(path: &str) -> Result<Self>

/// 获取本地包围盒
pub fn Mesh3D::aabb(&self) -> AABB

/// 获取包围球
pub fn Mesh3D::bounding_sphere(&self) -> Sphere

/// 获取子网格列表
pub fn Mesh3D::primitives(&self) -> &[Primitive]

/// 获取三角面数量
pub fn Mesh3D::triangles(&self) -> usize

/// 获取顶点数量
pub fn Mesh3D::vertices(&self) -> usize

/// 检查是否有法线
pub fn Mesh3D::has_normals(&self) -> bool

/// 检查是否有切线
pub fn Mesh3D::has_tangents(&self) -> bool

/// 检查是否有 UV
pub fn Mesh3D::has_uv(&self) -> bool

/// 计算法线
pub fn Mesh3D::compute_normals(&mut self)

/// 计算切线
pub fn Mesh3D::compute_tangents(&mut self)

/// 重新计算 AABB
pub fn Mesh3D::recalculate_aabb(&mut self)

/// 翻转 V 坐标
pub fn Mesh3D::invert_v(&mut self)

/// 原地变换顶点
pub fn Mesh3D::transform(&mut self, mat: Mat4)

/// GPU 上传
pub fn Mesh3D::upload(&mut self, renderer: &Renderer)

/// 绘制
pub fn Mesh3D::draw(&self, renderer: &Renderer, pipeline: &RenderPipeline3D, bind_groups: &BindGroups)

// === 图元生成 ===
pub fn Mesh3D::cube(size: f32) -> Self
pub fn Mesh3D::sphere(radius: f32, segments: u32, rings: u32) -> Self
pub fn Mesh3D::plane(size: Vec2, segments: u32) -> Self
pub fn Mesh3D::cylinder(radius: f32, height: f32, segments: u32) -> Self
pub fn Mesh3D::cone(radius: f32, height: f32, segments: u32) -> Self
pub fn Mesh3D::torus(major_r: f32, minor_r: f32, major_seg: u32, minor_seg: u32) -> Self
pub fn Mesh3D::capsule(radius: f32, height: f32, segments: u32) -> Self
```

### 1.4 MeshBuilder3D

```rust
pub fn MeshBuilder3D::new() -> Self
pub fn MeshBuilder3D::vertex(&mut self, v: Vertex)
pub fn MeshBuilder3D::index(&mut self, i: u32)
pub fn MeshBuilder3D::triangle(&mut self, a: u32, b: u32, c: u32)
pub fn MeshBuilder3D::quad(&mut self, a: u32, b: u32, c: u32, d: u32)
pub fn MeshBuilder3D::build(&self) -> Mesh3D
```

### 1.5 MeshManager

```rust
pub fn MeshManager::new() -> Self
pub fn MeshManager::load(&mut self, path: &str) -> Handle<Mesh3D>
pub fn MeshManager::get(&self, handle: Handle<Mesh3D>) -> Option<&Mesh3D>
pub fn MeshManager::unload(&mut self, handle: Handle<Mesh3D>)
pub fn MeshManager::reload_changed(&mut self)
pub fn MeshManager::len(&self) -> usize
```

### 1.6 Primitive

```rust
pub struct Primitive {
    pub start: u32,           // 起始索引
    pub count: u32,           // 索引数量
    pub material_index: u32, // 材质索引
}
```

---

## 2. 相机与视锥 API

### 2.1 Camera3D

```rust
// === 构造 ===
pub fn Camera3D::perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self
pub fn Camera3D::orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self

// === 矩阵 ===
pub fn Camera3D::view_matrix(&self) -> Mat4
pub fn Camera3D::projection_matrix(&self) -> Mat4
pub fn Camera3D::view_projection(&self) -> Mat4
pub fn Camera3D::inverse_view(&self) -> Mat4
pub fn Camera3D::inverse_projection(&self) -> Mat4
pub fn Camera3D::inverse_view_projection(&self) -> Mat4

// === 属性访问 ===
pub fn Camera3D::position(&self) -> Vec3
pub fn Camera3D::forward(&self) -> Vec3
pub fn Camera3D::right(&self) -> Vec3
pub fn Camera3D::up(&self) -> Vec3
pub fn Camera3D::fovy(&self) -> f32
pub fn Camera3D::aspect(&self) -> f32
pub fn Camera3D::near(&self) -> f32
pub fn Camera3D::far(&self) -> f32

// === 设置器 ===
pub fn Camera3D::set_fovy(&mut self, f: f32)
pub fn Camera3D::set_aspect(&mut self, a: f32)
pub fn Camera3D::set_near(&mut self, n: f32)
pub fn Camera3D::set_far(&mut self, f: f32)

// === 控制 ===
pub fn Camera3D::look_at(&mut self, target: Vec3)
pub fn Camera3D::look_to(&mut self, dir: Vec3, up: Vec3)

// === 坐标转换 ===
pub fn Camera3D::screen_to_world_ray(screen_pos: Vec2, screen_size: Vec2) -> Ray3
pub fn Camera3D::world_to_screen(world_pos: Vec3, screen_size: Vec2) -> Vec2
```

### 2.2 Frustum

```rust
pub fn Frustum::from_view_projection(vp: Mat4) -> Self
pub fn Frustum::planes(&self) -> &[Plane; 6]
pub fn Frustum::contains_point(&self, p: Vec3) -> bool
pub fn Frustum::contains_aabb(&self, aabb: &AABB) -> bool
pub fn Frustum::contains_sphere(&self, sphere: &Sphere) -> bool
pub fn Frustum::intersects_aabb(&self, aabb: &AABB) -> bool
```

---

## 3. 光照 API

### 3.1 Plane（光照计算用）

```rust
pub fn Plane::from_normal_and_point(normal: Vec3, point: Vec3) -> Self
pub fn Plane::distance(&self, p: Vec3) -> f32
pub fn Plane::normalize(&mut self)
```

### 3.2 Light Trait

```rust
pub trait Light3D {
    fn contribution(&self, world_pos: Vec3) -> LightSample;
}
```

### 3.3 DirectionalLight

```rust
pub fn DirectionalLight::new(dir: Vec3, color: Color, intensity: f32) -> Self
pub fn DirectionalLight::direction(&self) -> Vec3
pub fn DirectionalLight::color(&self) -> Color
pub fn DirectionalLight::intensity(&self) -> f32
pub fn DirectionalLight::casts_shadow(&self) -> bool
```

### 3.4 PointLight

```rust
pub fn PointLight::new(pos: Vec3, color: Color, intensity: f32, radius: f32) -> Self
pub fn PointLight::position(&self) -> Vec3
pub fn PointLight::color(&self) -> Color
pub fn PointLight::intensity(&self) -> f32
pub fn PointLight::radius(&self) -> f32
pub fn PointLight::attenuation(&self, distance: f32) -> f32
```

### 3.5 SpotLight

```rust
pub fn SpotLight::new(pos: Vec3, dir: Vec3, inner_angle: f32, outer_angle: f32, color: Color, intensity: f32) -> Self
pub fn SpotLight::inner_angle(&self) -> f32
pub fn SpotLight::outer_angle(&self) -> f32
pub fn SpotLight::cone_attenuation(&self, dir_to_point: Vec3) -> f32
```

### 3.6 AmbientLight

```rust
pub fn AmbientLight::new(color: Color, intensity: f32) -> Self
```

### 3.7 HemisphereLight

```rust
pub fn HemisphereLight::new(sky: Color, ground: Color, intensity: f32) -> Self
```

### 3.8 LightManager

```rust
pub fn LightManager::new() -> Self
pub fn LightManager::add_directional(&mut self, l: DirectionalLight)
pub fn LightManager::add_point(&mut self, l: PointLight)
pub fn LightManager::add_spot(&mut self, l: SpotLight)
pub fn LightManager::set_ambient(&mut self, l: AmbientLight)
pub fn LightManager::lights_ubo(&self) -> &UniformBuffer
pub fn LightManager::directional_count(&self) -> usize
pub fn LightManager::point_count(&self) -> usize
pub fn LightManager::spot_count(&self) -> usize
```

---

## 4. Transform 与场景 API

### 4.1 Transform3D

```rust
// === 构造 ===
pub fn Transform3D::new() -> Self
pub fn Transform3D::from_translation(v: Vec3) -> Self
pub fn Transform3D::from_rotation(q: Quat) -> Self
pub fn Transform3D::from_scale(v: Vec3) -> Self
pub const Transform3D::IDENTITY: Transform3D

// === 矩阵 ===
pub fn Transform3D::matrix(&self) -> Mat4
pub fn Transform3D::inverse_matrix(&self) -> Mat4

// === 属性访问 ===
pub fn Transform3D::translation(&self) -> Vec3
pub fn Transform3D::rotation(&self) -> Quat
pub fn Transform3D::scale(&self) -> Vec3

// === 设置器 ===
pub fn Transform3D::set_translation(&mut self, v: Vec3)
pub fn Transform3D::set_rotation(&mut self, q: Quat)
pub fn Transform3D::set_scale(&mut self, v: Vec3)

// === 增量操作 ===
pub fn Transform3D::translate(&mut self, v: Vec3)
pub fn Transform3D::rotate(&mut self, q: Quat)
pub fn Transform3D::scale_by(&mut self, v: Vec3)

// === 操作 ===
pub fn Transform3D::look_at(&mut self, target: Vec3, up: Vec3)
pub fn Transform3D::lerp(a: &Transform3D, b: &Transform3D, t: f32) -> Transform3D
pub fn Transform3D::transform_point(&self, p: Vec3) -> Vec3
pub fn Transform3D::transform_vector(&self, v: Vec3) -> Vec3
pub fn Transform3D::transform_direction(&self, v: Vec3) -> Vec3
```

### 4.2 Node3D

```rust
// === 构造 ===
pub fn Node3D::new() -> Self
pub fn Node3D::with_name(name: String) -> Self
pub fn Node3D::with_mesh(handle: Handle<Mesh3D>) -> Self

// === 属性访问 ===
pub fn Node3D::name(&self) -> &str
pub fn Node3D::parent(&self) -> Option<NodeHandle>
pub fn Node3D::children(&self) -> &[NodeHandle]
pub fn Node3D::local_transform(&self) -> &Transform3D
pub fn Node3D::world_transform(&self) -> &Transform3D
pub fn Node3D::aabb(&self) -> AABB
pub fn Node3D::visible(&self) -> bool
pub fn Node3D::mesh(&self) -> Option<Handle<Mesh3D>>
pub fn Node3D::material(&self) -> Option<Handle<Material3D>>

// === 设置器 ===
pub fn Node3D::set_visible(&mut self, visible: bool)
```

### 4.3 Scene3D

```rust
// === 构造 ===
pub fn Scene3D::new() -> Self

// === 节点管理 ===
pub fn Scene3D::add_node(&mut self, node: Node3D) -> NodeHandle
pub fn Scene3D::remove_node(&mut self, handle: NodeHandle)
pub fn Scene3D::node(&self, handle: NodeHandle) -> &Node3D
pub fn Scene3D::node_mut(&mut self, handle: NodeHandle) -> &mut Node3D
pub fn Scene3D::nodes(&self) -> &[Node3D]
pub fn Scene3D::root_nodes(&self) -> Vec<NodeHandle>

// === 相机管理 ===
pub fn Scene3D::main_camera(&self) -> Option<&Camera3D>
pub fn Scene3D::set_main_camera(&mut self, handle: NodeHandle)

// === 场景更新 ===
pub fn Scene3D::update_world_transforms(&mut self)
pub fn Scene3D::cull(&mut self, frustum: &Frustum)
pub fn Scene3D::visible_entities(&self) -> &[RenderEntity3D]
pub fn Scene3D::stats(&self) -> &SceneStats3D
```

### 4.4 RenderEntity3D

```rust
pub struct RenderEntity3D {
    pub mesh: Handle<Mesh3D>,
    pub material: Handle<Material3D>,
    pub world_matrix: Mat4,
}
```

### 4.5 SceneStats3D

```rust
pub struct SceneStats3D {
    pub nodes: usize,
    pub visible_nodes: usize,
    pub total_triangles: usize,
}
```

---

## 5. 渲染管线 API

### 5.1 RenderPipeline3D

```rust
// === 构造 ===
pub fn RenderPipeline3D::new(renderer: &Renderer, config: PipelineConfig) -> Result<Self>

// === 帧控制 ===
pub fn RenderPipeline3D::begin_frame(&mut self, renderer: &mut Renderer)
pub fn RenderPipeline3D::end_frame(&mut self, renderer: &mut Renderer)

// === 场景绘制 ===
pub fn RenderPipeline3D::draw_scene(&mut self, scene: &Scene3D, camera: &Camera3D, lights: &LightManager)

// === 渲染状态 ===
pub fn RenderPipeline3D::set_clear_color(&mut self, color: Color)
pub fn RenderPipeline3D::set_wireframe(&mut self, enabled: bool)
pub fn RenderPipeline3D::set_depth_test(&mut self, enabled: bool)
pub fn RenderPipeline3D::set_depth_write(&mut self, enabled: bool)
pub fn RenderPipeline3D::set_face_culling(&mut self, enabled: bool, winding: Winding)
pub fn RenderPipeline3D::set_blend_mode(&mut self, mode: BlendMode)
pub fn RenderPipeline3D::set_msaa(&mut self, samples: u32)

// === 着色器 ===
pub fn RenderPipeline3D::recompile_shaders(&mut self)

// === 统计 ===
pub fn RenderPipeline3D::stats(&self) -> &RenderStats3D
```

### 5.2 Material3D

```rust
// === 构造 ===
pub fn Material3D::from_color(color: Color) -> Self
pub fn Material3D::from_texture(tex: Handle<Texture>) -> Self

// === 属性访问 ===
pub fn Material3D::color(&self) -> Color
pub fn Material3D::main_texture(&self) -> Option<Handle<Texture>>
pub fn Material3D::shininess(&self) -> f32
pub fn Material3D::ambient(&self) -> Color

// === 设置器 ===
pub fn Material3D::set_color(&mut self, color: Color)
pub fn Material3D::set_main_texture(&mut self, tex: Handle<Texture>)
pub fn Material3D::set_shininess(&mut self, f: f32)
```

### 5.3 MaterialManager3D

```rust
pub fn MaterialManager3D::load(path: &str) -> Handle<Material3D>
```

### 5.4 Shader3D

```rust
pub fn Shader3D::default_unlit() -> Handle<Shader>
pub fn Shader3D::default_lit() -> Handle<Shader>
pub fn Shader3D::default_wireframe() -> Handle<Shader>
pub fn Shader3D::default_normal() -> Handle<Shader>
pub fn Shader3D::default_pbr_lit() -> Handle<Shader>  // 占位
pub fn Shader3D::default_skinned() -> Handle<Shader>   // 占位

pub fn ShaderModule::compile(src: &str, stage: ShaderStage) -> Result<Handle<Shader>>
```

### 5.5 PipelineStateCache

```rust
pub fn PipelineStateCache::get(&self, key: &PipelineKey) -> Option<Handle<Pipeline>>
pub fn PipelineStateCache::insert(&mut self, key: PipelineKey, pipeline: Handle<Pipeline>)
```

### 5.6 RenderStats3D

```rust
pub struct RenderStats3D {
    pub draw_calls: usize,
    pub triangles: usize,
    pub vertices: usize,
    pub entities_rendered: usize,
    pub entities_culled: usize,
    pub point_lights: usize,
    pub spot_lights: usize,
}

pub fn RenderStats3D::reset(&mut self)
pub fn RenderStats3D::to_string(&self) -> String
```

---

## 6. Ray 拾取与几何 API

### 6.1 Ray3

```rust
// === 构造 ===
pub fn Ray3::new(origin: Vec3, direction: Vec3) -> Self

// === 操作 ===
pub fn Ray3::at(&self, t: f32) -> Vec3

// === 命中检测 ===
pub fn Ray3::hit_aabb(&self, aabb: &AABB) -> Option<f32>
pub fn Ray3::hit_sphere(&self, sphere: &Sphere) -> Option<f32>
pub fn Ray3::hit_triangle(&self, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32>
pub fn Ray3::hit_plane(&self, plane: &Plane) -> Option<f32>
pub fn Ray3::hit_mesh(&self, mesh: &Mesh3D, transform: &Transform3D) -> Option<HitResult>
```

### 6.2 HitResult

```rust
pub struct HitResult {
    pub t: f32,              // 命中距离
    pub point: Vec3,         // 命中点
    pub normal: Vec3,        // 命中法线
    pub uv: Vec2,            // 命中 UV
    pub primitive_index: u32, // 子网格索引
}
```

### 6.3 PickResult

```rust
pub struct PickResult {
    pub hits: Vec<HitResult>,
}

impl PickResult {
    pub fn sorted(&self) -> &[HitResult]  // 按 t 排序
    pub fn closest(&self) -> Option<&HitResult>  // 最近的命中
}
```

### 6.4 AABB

```rust
// === 构造 ===
pub fn AABB::new(min: Vec3, max: Vec3) -> Self
pub fn AABB::from_points(points: &[Vec3]) -> Self

// === 属性 ===
pub fn AABB::min(&self) -> Vec3
pub fn AABB::max(&self) -> Vec3
pub fn AABB::center(&self) -> Vec3
pub fn AABB::half_extents(&self) -> Vec3
pub fn AABB::size(&self) -> Vec3

// === 检测 ===
pub fn AABB::contains_point(&self, p: Vec3) -> bool
pub fn AABB::intersects_aabb(&self, other: &AABB) -> bool

// === 操作 ===
pub fn AABB::merge(&self, other: &AABB) -> AABB
pub fn AABB::transform_by(&self, mat: Mat4) -> AABB
```

### 6.5 Sphere

```rust
// === 构造 ===
pub fn Sphere::new(center: Vec3, radius: f32) -> Self

// === 检测 ===
pub fn Sphere::contains_point(&self, p: Vec3) -> bool
pub fn Sphere::intersects_sphere(&self, other: &Sphere) -> bool

// === 操作 ===
pub fn Sphere::merge(&self, other: &Sphere) -> Sphere
```

### 6.6 Plane

```rust
pub fn Plane::from_normal_and_point(normal: Vec3, point: Vec3) -> Self
pub fn Plane::distance(&self, p: Vec3) -> f32
pub fn Plane::normalize(&mut self)
```

---

## 7. 调试渲染 API

### 7.1 DebugRenderer3D

```rust
pub fn DebugRenderer3D::line(&mut self, a: Vec3, b: Vec3, color: Color)
pub fn DebugRenderer3D::lines(&mut self, points: &[Vec3], color: Color)
pub fn DebugRenderer3D::aabb(&mut self, aabb: &AABB, color: Color)
pub fn DebugRenderer3D::sphere(&mut self, sphere: &Sphere, color: Color, segments: u32)
pub fn DebugRenderer3D::arrow(&mut self, from: Vec3, to: Vec3, color: Color)
pub fn DebugRenderer3D::frustum(&mut self, camera: &Camera3D, color: Color)
pub fn DebugRenderer3D::axis(&mut self, transform: &Transform3D, length: f32)
pub fn DebugRenderer3D::flush(&mut self, renderer: &Renderer)
pub fn DebugRenderer3D::clear(&mut self)
```

---

## 8. 示例入口

```rust
// === 示例程序 ===
// examples/mesh_viewer/main.rs
// examples/primitives_demo/main.rs
// examples/3d_scene_simple/main.rs
// examples/3d_lights/main.rs
// examples/3d_frustum_cull/main.rs
// examples/3d_picker/main.rs
// examples/3d_shader_hot_reload/main.rs
```

---

## 9. 配置与常量

### 9.1 环境变量

| 变量名 | 说明 | 可选值 |
|--------|------|--------|
| `RUSTENGINE_RENDERER` | 渲染后端选择 | `wgpu`, `gl` |

### 9.2 资源限制

| 资源类型 | 上限 |
|----------|------|
| 方向光数量 | 16 |
| 点光源数量 | 64 |
| 聚光灯数量 | 32 |
| `unsafe` 块数量 | <= 8 |

### 9.3 版本信息

| 项目 | 版本 |
|------|------|
| engine-render-3d | 0.9.0 |
| CHANGELOG | 记录 0.9.0 |