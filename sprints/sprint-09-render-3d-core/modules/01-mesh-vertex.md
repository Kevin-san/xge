# 模块一：Mesh 与顶点数据需求

## 1.1 模块概述

本模块定义了 3D 渲染系统中的网格（Mesh）与顶点数据结构，包括顶点格式、索引缓冲、网格生成器、GLTF 加载器以及网格管理器。这是 3D 渲染的基础构建块，所有可见几何体都通过 Mesh 组件进行描述。

**对应原需求编号**：2-57, 226-316

**核心依赖**：
- `engine-math`：Vec3、Mat4 等数学类型
- `engine-render`：GPU 资源上传抽象

---

## 1.2 Vertex 结构体

### 1.2.1 Vertex 定义

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 3 | 顶点结构体：position + normal + texcoord + tangent | `Vertex::new(pos: Vec3, normal: Vec3, uv: Vec2) -> Vertex` | pos, normal, uv | Vertex | tangent 为可选字段 |
| 226 | 创建 Vertex | `Vertex::new(pos, normal, uv) -> Vertex` | Vec3, Vec3, Vec2 | Vertex | 四个分量正确赋值 |
| 227 | 获取顶点位置 | `Vertex::position(&self) -> Vec3` | - | Vec3 | 返回 position 分量 |
| 228 | 获取顶点法线 | `Vertex::normal(&self) -> Vec3` | - | Vec3 | 返回 normal 分量 |
| 229 | 获取顶点 UV | `Vertex::texcoord(&self) -> Vec2` | - | Vec2 | 返回 texcoord 分量 |
| 230 | 顶点布局字节偏移 | `VertexLayout::POS3F_NORMAL3F_UV2F` | - | struct | 正确计算字节偏移 |

**优先级**：P0

---

## 1.3 VertexBuffer 与 IndexBuffer

### 1.3.1 GPU 缓冲管理

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 4 | VertexBuffer：upload 到 GPU / bind | `VertexBuffer::new(renderer, vertices) -> Self` | Renderer, &[Vertex] | VertexBuffer | GPU 上传完成 |
| 231 | 创建 VertexBuffer | `VertexBuffer::new(renderer, vertices)` | Renderer, vertices slice | Self | 缓冲创建成功 |
| 232 | 绑定 VertexBuffer | `VertexBuffer::bind(&self, renderer)` | Renderer | - | 缓冲绑定到渲染管线 |
| 233 | 获取字节大小 | `VertexBuffer::size_bytes(&self) -> usize` | - | usize | 返回顶点数据字节数 |
| 5 | IndexBuffer：upload 到 GPU / bind（16/32 位） | `IndexBuffer::new(renderer, indices) -> Self` | Renderer, &[u16/u32] | IndexBuffer | 支持 U16/U32 格式 |
| 234 | 创建 IndexBuffer | `IndexBuffer::new(renderer, indices)` | Renderer, indices slice | Self | 缓冲创建成功 |
| 235 | 绑定 IndexBuffer | `IndexBuffer::bind(&self, renderer)` | Renderer | - | 缓冲绑定到渲染管线 |
| 236 | 获取索引数量 | `IndexBuffer::index_count(&self) -> usize` | - | usize | 返回索引数量 |
| 237 | 索引格式枚举 | `IndexFormat::U16 / U32` | - | enum | 两种格式可用 |

**优先级**：P0

---

## 1.4 Mesh3D 结构体

### 1.4.1 核心构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 2 | Mesh3D 结构体 | `Mesh3D` | - | - | 包含顶点/索引/拓扑结构 |
| 238 | 构造 Mesh3D | `Mesh3D::new(vertex_buffer, index_buffer, primitives) -> Self` | VB, IB, primitives | Mesh3D | 成员正确赋值 |
| 239 | 从顶点索引构造 | `Mesh3D::from_vertices(vertices, indices) -> Self` | Vec<Vertex>, Vec<u32> | Mesh3D | 自动创建缓冲 |
| 275 | 创建 MeshManager | `MeshManager::new()` | - | Self | 初始化完成 |
| 276 | 加载网格资源 | `MeshManager::load(path) -> Handle<Mesh3D>` | 路径字符串 | Handle | 返回资源句柄 |
| 312 | 获取网格资源 | `MeshManager::get(handle) -> Option<&Mesh3D>` | Handle | Option<&Mesh3D> | 正确返回资源 |
| 313 | 卸载网格资源 | `MeshManager::unload(handle)` | Handle | - | 资源计数递减 |
| 314 | 热重载变化文件 | `MeshManager::reload_changed(&mut self)` | - | - | 检测并重载变化 |
| 315 | 获取管理数量 | `MeshManager::len() -> usize` | - | usize | 返回已加载数量 |

### 1.4.2 网格属性查询

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 8 | 本地包围盒 | `Mesh3D::aabb(&self) -> AABB` | - | AABB | 返回本地空间包围盒 |
| 241 | 获取 AABB | `Mesh3D::aabb(&self) -> AABB` | - | AABB | 与上述一致 |
| 9 | 包围球 | `Mesh3D::bounding_sphere(&self) -> Sphere` | - | Sphere | 返回最小包围球 |
| 242 | 获取包围球 | `Mesh3D::bounding_sphere(&self) -> Sphere` | - | Sphere | 与上述一致 |
| 277 | 获取原始网格 | `Mesh3D::primitive_count(&self) -> usize` | - | usize | 返回子网格数量 |
| 244 | 获取三角面数量 | `Mesh3D::triangles(&self) -> usize` | - | usize | 返回总三角面数 |
| 280 | 获取顶点数量 | `Mesh3D::vertices(&self) -> usize` | - | usize | 返回总顶点数 |
| 281 | 检查是否有法线 | `Mesh3D::has_normals(&self) -> bool` | - | bool | 法线数据存在返回 true |
| 282 | 检查是否有切线 | `Mesh3D::has_tangents(&self) -> bool` | - | bool | 切线数据存在返回 true |
| 283 | 检查是否有 UV | `Mesh3D::has_uv(&self) -> bool` | - | bool | UV 数据存在返回 true |

### 1.4.3 网格数据处理

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 212 | 计算法线 | `Mesh3D::compute_normals(&mut self)` | - | - | 自动生成缺失法线 |
| 249 | 计算法线（详细） | `Mesh3D::compute_normals(&mut self)` | - | - | 与上述一致 |
| 213 | 计算切线 | `Mesh3D::compute_tangents(&mut self)` | - | - | 自动生成缺失切线 |
| 250 | 计算切线（详细） | `Mesh3D::compute_tangents(&mut self)` | - | - | 与上述一致 |
| 215 | 重新计算 AABB | `Mesh3D::recalculate_aabb(&mut self)` | - | - | 更新顶点后重新计算 |
| 251 | 重新计算 AABB（详细） | `Mesh3D::recalculate_aabb(&mut self)` | - | - | 与上述一致 |
| 214 | 翻转 V 坐标 | `Mesh3D::invert_v(&mut self)` | - | - | UV V 分量取反 |
| 252 | 翻转 V 坐标（详细） | `Mesh3D::invert_v(&mut self)` | - | - | 与上述一致 |
| 288 | 原地变换顶点 | `Mesh3D::transform(&mut self, mat: Mat4)` | Mat4 | - | 顶点乘以矩阵 |

**优先级**：P0

---

## 1.5 图元生成器

### 1.5.1 内置几何体

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 14 | 立方体 | `Mesh3D::cube(size: f32) -> Self` | size | Mesh3D | 生成 12 三角面立方体 |
| 260 | 立方体（详细） | `Mesh3D::cube(size)` | f32 | Self | 与上述一致 |
| 15 | 球体 | `Mesh3D::sphere(radius, segments, rings) -> Self` | f32, u32, u32 | Mesh3D | 生成光滑球体 |
| 261 | 球体（详细） | `Mesh3D::sphere(radius, segments, rings)` | f32, u32, u32 | Self | 与上述一致 |
| 16 | 平面 | `Mesh3D::plane(size, segments) -> Self` | Vec2, u32 | Mesh3D | 生成平面网格 |
| 262 | 平面（详细） | `Mesh3D::plane(size, segments)` | Vec2, u32 | Self | 与上述一致 |
| 17 | 圆柱 | `Mesh3D::cylinder(radius, height, segments) -> Self` | f32, f32, u32 | Mesh3D | 生成圆柱体 |
| 263 | 圆柱（详细） | `Mesh3D::cylinder(radius, height, segments)` | f32, f32, u32 | Self | 与上述一致 |
| 18 | 圆锥 | `Mesh3D::cone(radius, height, segments) -> Self` | f32, f32, u32 | Mesh3D | 生成圆锥体 |
| 264 | 圆锥（详细） | `Mesh3D::cone(radius, height, segments)` | f32, f32, u32 | Self | 与上述一致 |
| 19 | 圆环 | `Mesh3D::torus(major_r, minor_r, major_seg, minor_seg) -> Self` | f32, f32, u32, u32 | Mesh3D | 生成圆环体 |
| 265 | 圆环（详细） | `Mesh3D::torus(major_r, minor_r, major_seg, minor_seg)` | f32, f32, u32, u32 | Self | 与上述一致 |
| 20 | 胶囊 | `Mesh3D::capsule(radius, height, segments) -> Self` | f32, f32, u32 | Mesh3D | 生成胶囊体 |
| 266 | 胶囊（详细） | `Mesh3D::capsule(radius, height, segments)` | f32, f32, u32 | Self | 与上述一致 |

### 1.5.2 MeshBuilder 流式构建器

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 180 | MeshBuilder3D 声明 | `MeshBuilder3D` | - | - | 流式构建网格 |
| 181 | 创建 MeshBuilder | `MeshBuilder3D::new() -> Self` | - | Self | 初始化空构建器 |
| 208 | 添加顶点 | `MeshBuilder3D::vertex(&mut self, vertex: Vertex)` | Vertex | - | 添加到顶点列表 |
| 209 | 添加索引 | `MeshBuilder3D::index(&mut self, idx: u32)` | u32 | - | 添加到索引列表 |
| 210 | 添加三角面 | `MeshBuilder3D::triangle(&mut self, a, b, c: u32)` | u32, u32, u32 | - | 添加三个索引 |
| 289 | 添加顶点（详细） | `MeshBuilder3D::vertex(v: Vertex)` | Vertex | - | 与上述一致 |
| 290 | 添加索引（详细） | `MeshBuilder3D::index(i: u32)` | u32 | - | 与上述一致 |
| 292 | 添加三角面（详细） | `MeshBuilder3D::triangle(a, b, c)` | u32, u32, u32 | - | 与上述一致 |
| 293 | 添加四边形 | `MeshBuilder3D::quad(a, b, c, d: u32)` | u32, u32, u32, u32 | - | 拆分为两个三角 |
| 294 | 构建网格 | `MeshBuilder3D::build(&self) -> Mesh3D` | - | Mesh3D | 生成最终网格 |
| 211 | 构建网格（详细） | `MeshBuilder3D::build(&self) -> Mesh3D` | - | Mesh3D | 与上述一致 |

**优先级**：P0

---

## 1.6 GLTF 加载器

### 1.6.1 GLTF/GLB 格式支持

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 7 | 从文件加载 | `Mesh3D::from_file(path) -> Result<Self>` | 路径字符串 | Result<Mesh3D> | 支持 GLTF/GLB |
| 21 | GLTF crate 使用 | GLTF 加载通过 `gltf` crate | - | - | 使用成熟的 gltf 库 |
| 22 | 顶点属性映射 | GLTF 顶点属性：POSITION / NORMAL / TEXCOORD_0 / TANGENT / COLOR_0 | - | - | 完整属性映射 |
| 267 | 加载顶点数据 | GLTF 加载：vertices 正确读取 | - | - | 位置数据正确 |
| 268 | 加载索引数据 | GLTF 加载：indices 正确读取 | - | - | 索引数据正确 |
| 269 | 法线属性可选 | GLTF 加载：normal 属性可选 | - | - | 缺失时自动生成 |
| 270 | UV 属性可选 | GLTF 加载：texcoord 属性可选 | - | - | 缺失时标记 has_uv=false |
| 271 | 切线属性可选 | GLTF 加载：tangent 属性可选 | - | - | 缺失时自动计算 |
| 272 | 多 primitive 支持 | GLTF 加载：多 primitive 支持 | - | - | 每个 primitive 生成子网格 |
| 273 | 材质映射 | GLTF 加载：材质信息提取（简化版） | - | - | 映射到 Material3D |
| 274 | 错误处理 | GLTF 加载：失败时返回错误而非 panic | - | - | Result 正确返回错误 |

### 1.6.2 扩展格式接口

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 26 | FBX 加载接口预留 | FBX 加载：先不支持，留接口 | - | - | 留出扩展接口 |
| 27 | OBJ 简化支持 | OBJ 加载：简化支持（可选） | - | - | 基础 OBJ 解析 |

**优先级**：P1

---

## 1.7 Primitive 子网格

### 1.7.1 多材质支持

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 10 | 获取图元列表 | `Mesh3D::primitives(&self) -> &[Primitive]` | - | &[Primitive] | 返回所有子网格 |
| 37 | Primitive 结构 | Primitive：顶点范围 + 材质索引 | - | - | 包含 start/count/material_idx |
| 279 | 获取子网格数量 | `Mesh3D::primitive_count(&self) -> usize` | - | usize | 返回 primitive 数量 |

**优先级**：P0

---

## 1.8 GPU 上传与绘制

### 1.8.1 渲染调用

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 12 | GPU 上传 | `Mesh3D::upload(&mut self, renderer)` | Renderer | - | 顶点/索引缓冲上传 GPU |
| 13 | 绘制调用 | `Mesh3D::draw(&self, renderer, pipeline, bind_groups)` | Renderer, Pipeline, BindGroups | - | 发起绘制指令 |

**优先级**：P0

---

## 1.9 依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                     engine-math                         │
│            (Vec3, Vec2, Mat4, Quat, AABB)               │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                     engine-render                        │
│            (Renderer, Buffer, Texture, Shader)           │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                   engine-render-3d                       │
│                   (本模块: Mesh/Vertex)                   │
└─────────────────────────────────────────────────────────┘
```

**上游依赖**：
- `engine-math`：数学原语
- `engine-render`：渲染后端抽象

**下游依赖**：
- `Scene3D`：网格挂载到场景节点
- `RenderPipeline3D`：网格绘制调用
- `Ray3`：射线检测网格

---

## 1.10 验收标准

### 1.10.1 功能验收

- [ ] `Mesh3D::cube()` 生成正确的 12 三角面立方体
- [ ] `Mesh3D::sphere()` 顶点和索引数量匹配公式：`vertices = (rings+1) * (segments+1)`
- [ ] `Mesh3D::from_file("cube.gltf")` 能正确加载 GLTF 文件
- [ ] `MeshBuilder3D` 能正确构建自定义网格
- [ ] `Mesh3D::compute_normals()` 能为无法线网格生成法线
- [ ] `MeshManager::load()` 返回有效 Handle
- [ ] `MeshManager::reload_changed()` 能检测文件变化并重载

### 1.10.2 单元测试

| 测试项 | 需求ID | 验证内容 |
|--------|--------|----------|
| `Mesh3D::cube` 三角面数量 | 232 | 立方体应生成 12 个三角面 |
| `Mesh3D::sphere` 顶索引匹配 | 233 | 顶点与索引数量符合公式 |
| `MeshBuilder3D` 构建成功 | 234 | 成功构建并生成 Mesh3D |

### 1.10.3 集成测试

- [ ] 加载 `cube.gltf` 无 panic
- [ ] 渲染一帧无 GPU error

---

## 1.11 优先级汇总

| 优先级 | 需求ID | 占比 |
|--------|--------|------|
| P0 | 2-9, 12-20, 212-215, 226-294, 312-315 | 85% |
| P1 | 21-27, 295-311 | 10% |
| P2 | - | 5% |

**P0 核心**：Vertex/IndexBuffer、Mesh3D 核心、图元生成器、MeshBuilder、GPU 上传/绘制
**P1 重要**：GLTF 加载、FBX/OBJ 接口预留
**P2 可选**：高级格式支持