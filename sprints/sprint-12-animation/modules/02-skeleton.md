# 骨骼系统模块 (Skeleton / Skin / SkinnedMesh)

## 模块概述

骨骼系统负责管理骨骼层级结构、绑定姿态、蒙皮网格和顶点权重。本模块是动画系统的核心基础设施，支持蒙皮渲染和骨骼动画计算。

## 需求编号与功能描述

### Bone 骨骼数据

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 30 | Bone 数据结构 | `Bone` | name, parent_index, local_bind_pose, inverse_bind_pose | - |
| 38 | 获取骨骼名称 | `Bone::name(&self) -> &str` | - | &str |
| 39 | 获取父骨骼索引 | `Bone::parent(&self) -> Option<usize>` | - | Option<usize> |
| 40 | 获取局部绑定姿态 | `Bone::local_bind_pose(&self) -> (Vec3, Quat, Vec3)` | - | (Vec3, Quat, Vec3) |
| 41 | 获取逆绑定矩阵 | `Bone::inverse_bind_pose(&self) -> Mat4` | - | Mat4 |
| 245 | 构造函数 | `Bone::new(name, parent, local_bind_pose, inverse_bind_pose) -> Self` | String, Option<usize>, (Vec3, Quat, Vec3), Mat4 | Self |
| 246 | 获取名称 | `Bone::name(&self) -> &str` | - | &str |
| 247 | 获取父索引 | `Bone::parent(&self) -> Option<usize>` | - | Option<usize> |
| 248 | 获取局部绑定姿态 | `Bone::local_bind_pose(&self) -> (Vec3, Quat, Vec3)` | - | (Vec3, Quat, Vec3) |
| 249 | 获取逆绑定矩阵 | `Bone::inverse_bind_pose(&self) -> Mat4` | - | Mat4 |

### Skeleton 骨骼骨架

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 31 | 创建 Skeleton | `Skeleton::new(bones) -> Self` | Vec<Bone> | Self |
| 32 | 获取骨骼数组 | `Skeleton::bones(&self) -> &[Bone]` | - | &[Bone] |
| 33 | 获取单个骨骼 | `Skeleton::bone(&self, idx) -> &Bone` | usize | &Bone |
| 34 | 获取骨骼数量 | `Skeleton::bone_count(&self) -> usize` | - | usize |
| 35 | 获取绑定姿态 | `Skeleton::bind_pose(&self) -> &Pose` | - | &Pose |
| 36 | 获取逆绑定矩阵 | `Skeleton::inverse_bind_pose(&self, idx) -> Mat4` | usize | Mat4 |
| 37 | 获取根骨骼索引 | `Skeleton::root(&self) -> usize` | - | usize |
| 250 | 构造函数 | `Skeleton::new(bones) -> Self` | Vec<Bone> | Self |
| 251 | 获取骨骼数组 | `Skeleton::bones(&self) -> &[Bone]` | - | &[Bone] |
| 252 | 获取单个骨骼 | `Skeleton::bone(&self, idx) -> &Bone` | usize | &Bone |
| 253 | 获取骨骼数量 | `Skeleton::bone_count(&self) -> usize` | - | usize |
| 254 | 获取根骨骼 | `Skeleton::root(&self) -> usize` | - | usize |
| 255 | 获取子骨骼列表 | `Skeleton::children(&self, parent) -> Vec<usize>` | usize | Vec<usize> |
| 256 | 获取绑定姿态 | `Skeleton::bind_pose(&self) -> &Pose` | - | &Pose |
| 257 | 获取逆绑定矩阵数组 | `Skeleton::inverse_bind_matrices(&self) -> &[Mat4]` | - | &[Mat4] |
| 258 | 按名称查找骨骼 | `Skeleton::find_bone_by_name(&self, name) -> Option<usize>` | &str | Option<usize> |
| 406 | 从 glTF 加载 | `Skeleton::from_gltf(path) -> Result<Self>` | &str | Result<Self> |

### Skin 蒙皮数据

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 43 | 创建 Skin | `Skin::new(bones, weights) -> Self` | Vec<String>, Vec<Mat4> | Self |
| 44 | 获取骨骼数量 | `Skin::bone_count(&self) -> usize` | - | usize |
| 45 | 获取逆绑定矩阵 | `Skin::inverse_bind_matrices(&self) -> &[Mat4]` | - | &[Mat4] |
| 46 | 获取骨骼名称 | `Skin::bone_names(&self) -> &[String]` | - | &[String] |
| 259 | 构造函数 | `Skin::new(bone_names, inverse_bind_matrices) -> Self` | Vec<String>, Vec<Mat4> | Self |
| 260 | 骨骼数量 | `Skin::bone_count(&self) -> usize` | - | usize |
| 261 | 骨骼名称 | `Skin::bone_names(&self) -> &[String]` | - | &[String] |
| 262 | 逆绑定矩阵 | `Skin::inverse_bind_matrices(&self) -> &[Mat4]` | - | &[Mat4> |

### VertexWeight 顶点权重

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 47 | 创建顶点权重 | `VertexWeight::new(bone, weight)` | u32, f32 | Self |
| 76 | VertexWeight 数据结构 | `VertexWeight` | bone index + weight | - |
| 263 | 构造函数 | `VertexWeight::new(bone, weight)` | u32, f32 | Self |
| 264 | 获取骨骼索引 | `VertexWeight::bone(&self) -> u32` | - | u32 |
| 265 | 获取权重值 | `VertexWeight::weight(&self) -> f32` | - | f32 |
| 266 | VertexWeightArray | 每个顶点最多 4 权重（标准实现） | - | - |

### SkinnedMesh 蒙皮网格

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 42 | SkinnedMesh 数据结构 | `SkinnedMesh` | mesh + 骨骼权重 | - |
| 48 | 获取 Skin | `SkinnedMesh::skin(&self) -> &Skin` | - | &Skin |
| 49 | 获取 Mesh Handle | `SkinnedMesh::mesh(&self) -> Handle<Mesh3D>` | - | Handle<Mesh3D> |
| 50 | 计算矩阵调色板 | 每帧根据 pose 计算 skin matrices | Pose | Vec<Mat4> |
| 267 | 构造函数 | `SkinnedMesh::new(mesh, skin) -> Self` | Handle<Mesh3D>, Skin | Self |
| 268 | 获取 Mesh | `SkinnedMesh::mesh(&self) -> Handle<Mesh3D>` | - | Handle<Mesh3D> |
| 269 | 获取 Skin | `SkinnedMesh::skin(&self) -> &Skin` | - | &Skin |
| 270 | 获取顶点权重 | `SkinnedMesh::vertex_weights(&self) -> &[Vec<VertexWeight>]` | - | &[Vec<VertexWeight>] |
| 271 | 计算矩阵调色板 | `SkinnedMesh::compute_matrix_palette(&self, pose) -> Vec<Mat4>` | &Pose | Vec<Mat4> |
| 407 | 从 glTF 加载 | `SkinnedMesh::from_gltf(path) -> Result<Self>` | &str | Result<Self> |

### SkinnedMeshRenderer 蒙皮渲染器

| 编号 | 功能描述 | API 签名 | 输入 | 输出 |
|------|----------|----------|------|------|
| 51 | 蒙皮渲染器 | `SkinnedMeshRenderer` | 顶点着色器 skinning | - |
| 272 | 创建渲染器 | `SkinnedMeshRenderer::new(renderer) -> Result<Self>` | &Renderer | Result<Self> |
| 273 | 绘制蒙皮网格 | `SkinnedMeshRenderer::draw(&self, renderer, mesh, skeleton, pose, material, camera)` | Renderer, Mesh, Skeleton, Pose, Material, Camera | - |

## 验收标准

- [ ] Skeleton 正确维护骨骼层级关系
- [ ] Skeleton::find_bone_by_name 正确查找（需求 258）
- [ ] Pose::local_to_world 正确计算世界空间矩阵（需求 278）
- [ ] SkinnedMesh::compute_matrix_palette 正确计算矩阵调色板（需求 271）
- [ ] glTF 加载正确解析 skeleton/skin/node（需求 406-407, 418-420）
- [ ] 单测 Skeleton::find_bone_by_name 通过（需求 487）
- [ ] 单测 Pose::local_to_world 通过（需求 488）
- [ ] 单测 SkinnedMesh::compute_matrix_palette 通过（需求 489）

## 依赖关系

- 依赖 `Pose` 类型（01-animation-clip.md）
- 依赖 `Mat4` / `Vec3` / `Quat` 数学类型
- 被 Animator、IK、Ragdoll 等模块使用

## 优先级

**P0（核心）：**
- Bone 数据结构
- Skeleton 层级管理
- Skin / VertexWeight 数据结构
- SkinnedMesh::compute_matrix_palette

**P1（重要）：**
- SkinnedMeshRenderer GPU skinning
- Skeleton::children 遍历
- 按名称查找骨骼

**P2（增强）：**
- glTF 完整加载流程
- 骨骼可视化
- 权重编辑功能