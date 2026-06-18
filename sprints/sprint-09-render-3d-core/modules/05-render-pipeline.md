# 模块五：渲染管线需求

## 5.1 模块概述

本模块定义了 3D 渲染系统的渲染管线（RenderPipeline3D）、材质（Material3D）和着色器（Shader3D）。渲染管线负责场景的最终绘制，包括深度测试、背面剔除、混合模式等渲染状态管理，以及着色器编译和 UniformBuffer 管理。

**对应原需求编号**：106-155, 386-461

**核心依赖**：
- `engine-render`：渲染后端抽象
- `Scene3D`：场景数据
- `Camera3D`：相机矩阵

---

## 5.2 RenderPipeline3D 结构体

### 5.2.1 管线构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 117 | 管线结构 | RenderPipeline3D：`init / begin_frame / end_frame` | - | - | 渲染管线主入口 |
| 386 | 创建渲染管线 | `RenderPipeline3D::new(renderer, config) -> Result<Self>` | Renderer, Config | Result<Self> | 成功创建管线 |
| 433 | 创建渲染管线（详细） | `RenderPipeline3D::new(renderer, config)` | Renderer, Config | Result<Self> | 与上述一致 |

### 5.2.2 帧控制

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 118 | 开始帧 | `RenderPipeline3D::begin_frame(&mut self, renderer: &mut Renderer)` | Renderer | - | 清空上帧状态 |
| 387 | 开始帧（详细） | `RenderPipeline3D::begin_frame(&mut self, renderer)` | Renderer | - | 与上述一致 |
| 434 | 开始帧（详细） | `RenderPipeline3D::begin_frame(&mut self, renderer)` | Renderer | - | 与上述一致 |
| 119 | 结束帧 | `RenderPipeline3D::end_frame(&mut self, renderer: &mut Renderer)` | Renderer | - | 提交渲染指令 |
| 388 | 结束帧（详细） | `RenderPipeline3D::end_frame(&mut self, renderer)` | Renderer | - | 与上述一致 |
| 435 | 结束帧（详细） | `RenderPipeline3D::end_frame(&mut self, renderer)` | Renderer | - | 与上述一致 |

### 5.2.3 场景绘制

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 118 | 绘制场景 | `RenderPipeline3D::draw_scene(&mut self, renderer, scene, camera, lights)` | Renderer, Scene3D, Camera3D, LightManager | - | 执行完整渲染流程 |
| 389 | 绘制场景（详细） | `RenderPipeline3D::draw_scene(&mut self, scene, camera, lights)` | Scene3D, Camera3D, LightManager | - | 与上述一致 |
| 436 | 绘制场景（详细） | `RenderPipeline3D::draw_scene(&mut self, scene, camera, lights)` | Scene3D, Camera3D, LightManager | - | 与上述一致 |

**优先级**：P0

---

## 5.3 渲染状态控制

### 5.3.1 深度测试

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 119 | 深度测试开关 | `RenderPipeline3D::depth_test(enabled: bool)` | bool | - | 启用/禁用深度测试 |
| 145 | 深度测试（详细） | `RenderPipeline3D::depth_test(enabled)` | bool | - | 与上述一致 |
| 392 | 深度测试（详细） | `RenderPipeline3D::set_depth_test(&mut self, enabled)` | bool | - | 与上述一致 |
| 439 | 设置深度测试（详细） | `RenderPipeline3D::set_depth_test(&mut self, enabled)` | bool | - | 与上述一致 |
| 120 | 深度写入开关 | `RenderPipeline3D::depth_write(enabled: bool)` | bool | - | 启用/禁用深度写入 |
| 146 | 深度写入（详细） | `RenderPipeline3D::depth_write(enabled)` | bool | - | 与上述一致 |

### 5.3.2 背面剔除

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 121 | 背面剔除 | `RenderPipeline3D::face_culling(enabled: bool, winding: Winding)` | bool, Winding | - | 设置剔除模式和面的绕序 |
| 147 | 背面剔除（详细） | `RenderPipeline3D::face_culling(enabled, winding)` | bool, Winding | - | 与上述一致 |
| 393 | 设置背面剔除（详细） | `RenderPipeline3D::set_face_culling(&mut self, enabled, winding)` | bool, Winding | - | 与上述一致 |
| 440 | 设置背面剔除（详细） | `RenderPipeline3D::set_face_culling(&mut self, enabled, winding)` | bool, Winding | - | 与上述一致 |

### 5.3.3 混合模式

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 122 | 混合模式 | `RenderPipeline3D::blend_mode(mode: BlendMode)` | BlendMode | - | 设置颜色混合模式 |
| 148 | 混合模式（详细） | `RenderPipeline3D::blend_mode(mode)` | BlendMode | - | 与上述一致 |
| 394 | 设置混合模式（详细） | `RenderPipeline3D::set_blend_mode(&mut self, mode)` | BlendMode | - | 与上述一致 |
| 441 | 设置混合模式（详细） | `RenderPipeline3D::set_blend_mode(&mut self, mode)` | BlendMode | - | 与上述一致 |

### 5.3.4 清屏与线框

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 123 | 清屏颜色 | `RenderPipeline3D::clear_color(color: Color)` | Color | - | 设置清屏颜色 |
| 149 | 清屏颜色（详细） | `RenderPipeline3D::clear_color(color)` | Color | - | 与上述一致 |
| 390 | 设置清屏颜色（详细） | `RenderPipeline3D::set_clear_color(&mut self, color)` | Color | - | 与上述一致 |
| 437 | 设置清屏颜色（详细） | `RenderPipeline3D::set_clear_color(&mut self, color)` | Color | - | 与上述一致 |
| 124 | 线框模式 | `RenderPipeline3D::wireframe(enabled: bool)` | bool | - | 启用/禁用线框渲染 |
| 150 | 线框模式（详细） | `RenderPipeline3D::wireframe(enabled)` | bool | - | 与上述一致 |
| 391 | 设置线框模式（详细） | `RenderPipeline3D::set_wireframe(&mut self, enabled)` | bool | - | 与上述一致 |
| 438 | 设置线框模式（详细） | `RenderPipeline3D::set_wireframe(&mut self, enabled)` | bool | - | 与上述一致 |

### 5.3.5 MSAA

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 125 | MSAA 采样数 | `RenderPipeline3D::msaa(samples: u32)` | u32 | - | 设置多重采样数（1/2/4/8） |
| 151 | MSAA（详细） | `RenderPipeline3D::msaa(samples)` | u32 | - | 与上述一致 |

### 5.3.6 着色器热重载

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 126 | 重新编译着色器 | `RenderPipeline3D::recompile_shaders(&mut self)` | - | - | 热重载所有着色器 |
| 152 | 重新编译着色器（详细） | `RenderPipeline3D::recompile_shaders()` | - | - | 与上述一致 |
| 395 | 重新编译着色器（详细） | `RenderPipeline3D::recompile_shaders(&mut self)` | - | - | 与上述一致 |
| 442 | 重新编译着色器（详细） | `RenderPipeline3D::recompile_shaders(&mut self)` | - | - | 与上述一致 |

**优先级**：P0

---

## 5.4 Material3D 材质

### 5.4.1 材质构造

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 106 | 材质结构 | Material3D：基础单色 + 简单光照（Phong/Blinn-Phong 初版） | - | - | 简化版材质 |
| 397 | 从颜色创建 | `Material3D::from_color(color: Color) -> Self` | Color | Self | 创建纯色材质 |
| 444 | 从颜色创建（详细） | `Material3D::from_color(color)` | Color | Self | 与上述一致 |
| 398 | 从纹理创建 | `Material3D::from_texture(tex: Handle<Texture>) -> Self` | Handle<Texture> | Self | 创建纹理材质 |
| 445 | 从纹理创建（详细） | `Material3D::from_texture(tex)` | Handle<Texture> | Self | 与上述一致 |

### 5.4.2 材质属性

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 107 | 获取颜色 | `Material3D::color(&self) -> Color` | - | Color | 返回材质颜色 |
| 133 | 颜色（详细） | `Material3D::color(&self) -> Color` | - | Color | 与上述一致 |
| 399 | 颜色（详细） | `Material3D::color(&self) -> Color` | - | Color | 与上述一致 |
| 108 | 设置颜色 | `Material3D::set_color(&mut self, color: Color)` | Color | - | 设置材质颜色 |
| 134 | 设置颜色（详细） | `Material3D::set_color(&mut self, color)` | Color | - | 与上述一致 |
| 400 | 设置颜色（详细） | `Material3D::set_color(&mut self, color)` | Color | - | 与上述一致 |
| 109 | 获取主纹理 | `Material3D::main_texture(&self) -> Option<Handle<Texture>>` | - | Option<Handle<Texture>> | 返回主纹理 |
| 136 | 主纹理（详细） | `Material3D::main_texture(&self) -> Option<Handle<Texture>>` | - | Option<Handle<Texture>> | 与上述一致 |
| 401 | 主纹理（详细） | `Material3D::main_texture(&self) -> Option<Handle<Texture>>` | - | Option<Handle<Texture>> | 与上述一致 |
| 110 | 设置主纹理 | `Material3D::set_main_texture(&mut self, tex: Handle<Texture>)` | Handle<Texture> | - | 设置主纹理 |
| 137 | 主纹理设置（详细） | `Material3D::set_main_texture(&mut self, tex)` | Handle<Texture> | - | 与上述一致 |
| 402 | 设置主纹理（详细） | `Material3D::set_main_texture(&mut self, tex)` | Handle<Texture> | - | 与上述一致 |
| 111 | 获取高光系数 | `Material3D::shininess(&self) -> f32` | - | f32 | 返回 Phong 高光系数 |
| 138 | 高光系数（详细） | `Material3D::shininess(&self) -> f32` | - | f32 | 与上述一致 |
| 404 | 高光系数（详细） | `Material3D::shininess(&self) -> f32` | - | f32 | 与上述一致 |
| 112 | 获取环境光颜色 | `Material3D::ambient(&self) -> Color` | - | Color | 返回环境光颜色 |
| 139 | 环境光颜色（详细） | `Material3D::ambient(&self) -> Color` | - | Color | 与上述一致 |
| 451 | 高光系数设置 | `Material3D::set_shininess(&mut self, f: f32)` | f32 | - | 设置高光系数 |
| 452 | 高光系数设置（详细） | `Material3D::set_shininess(&mut self, f)` | f32 | - | 与上述一致 |

### 5.4.3 材质管理器

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 113 | 材质管理器 | MaterialManager3D：`load(path) -> Handle<Material3D>` | - | - | 管理材质资源 |
| 139 | 材质管理器（详细） | `MaterialManager3D::load(path) -> Handle<Material3D>` | path | Handle<Material3D> | 加载材质文件 |
| 453 | 材质管理器（详细） | `MaterialManager3D::load(path)` | path | Handle<Material3D> | 与上述一致 |
| 406 | 材质加载（详细） | `MaterialManager3D::load(path) -> Handle<Material3D>` | path | Handle<Material3D> | 与上述一致 |

**优先级**：P0

---

## 5.5 Shader3D 着色器

### 5.5.1 内建着色器

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 114 | 默认 PBR 着色器 | `Shader3D::default_pbr_lit() -> Handle<Shader>`（占位） | - | Handle<Shader> | PBR 占位实现 |
| 140 | 默认无光着色器 | `Shader3D::default_unlit() -> Handle<Shader>` | - | Handle<Shader> | 不进行光照计算 |
| 115 | 默认无光着色器（详细） | `Shader3D::default_unlit() -> Handle<Shader>` | - | Handle<Shader> | 与上述一致 |
| 408 | 默认无光着色器（详细） | `Shader3D::default_unlit()` | - | Handle<Shader> | 与上述一致 |
| 455 | 默认无光着色器（详细） | `Shader3D::default_unlit() -> Handle<Shader>` | - | Handle<Shader> | 与上述一致 |
| 116 | 骨骼着色器占位 | `Shader3D::default_skinned() -> Handle<Shader>`（占位） | - | Handle<Shader> | 骨骼动画占位 |
| 141 | 默认线框着色器 | `Shader3D::default_wireframe() -> Handle<Shader>` | - | Handle<Shader> | 线框渲染 |
| 142 | 默认法线着色器 | `Shader3D::default_normal() -> Handle<Shader>` | - | Handle<Shader> | 法线可视化 |
| 410 | 默认线框着色器（详细） | `Shader3D::default_wireframe() -> Handle<Shader>` | - | Handle<Shader> | 与上述一致 |
| 411 | 默认法线着色器（详细） | `Shader3D::default_normal() -> Handle<Shader>` | - | Handle<Shader> | 与上述一致 |
| 456 | 默认线框着色器（详细） | `Shader3D::default_wireframe()` | - | Handle<Shader> | 与上述一致 |
| 457 | 默认法线着色器（详细） | `Shader3D::default_normal()` | - | Handle<Shader> | 与上述一致 |
| 409 | 默认光照着色器 | `Shader3D::default_lit() -> Handle<Shader>` | - | Handle<Shader> | 基础光照着色 |
| 456 | 默认光照着色器（详细） | `Shader3D::default_lit() -> Handle<Shader>` | - | Handle<Shader> | 与上述一致 |

### 5.5.2 着色器编译

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 407 | 着色器编译 | `ShaderModule::compile(src: &str, stage: ShaderStage) -> Result<Handle<Shader>>` | src, stage | Handle<Shader> | 编译着色器源码 |

**优先级**：P0

---

## 5.6 内建着色器列表

### 5.6.1 着色器清单

| 需求ID | 功能描述 | 文件名 | 验收标准 |
|--------|----------|--------|----------|
| 127 | Lit 着色器 | `lit.vert/frag` | Phong/Blinn-Phong + 方向光 + 点光 |
| 128 | Unlit 着色器 | `unlit.vert/frag` | 仅采样贴图，不光照 |
| 129 | Skinned 着色器 | `skinned.vert/frag` | 骨骼动画占位 |
| 130 | Normal 着色器 | `normal.vert/frag` | 法线可视化 |
| 131 | Wireframe 着色器 | `wireframe.vert/frag` | 线框渲染 |
| 132 | Shadow 着色器 | `shadow.vert/frag` | 阴影贴图（下一阶段） |

**优先级**：P0

---

## 5.7 PipelineStateCache 管线状态缓存

### 5.7.1 状态缓存

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 143 | 状态缓存 | PipelineStateCache：缓存已编译 pipeline | - | - | 减少重复编译 |
| 459 | 获取管线状态 | `PipelineStateCache::get(&self, key: &PipelineKey) -> Option<Handle<Pipeline>>` | &PipelineKey | Option<Handle<Pipeline>> | 查找缓存 |
| 412 | 获取管线状态（详细） | `PipelineStateCache::get(&self, key) -> Option<Handle<Pipeline>>` | &PipelineKey | Option<Handle<Pipeline>> | 与上述一致 |
| 460 | 插入管线状态 | `PipelineStateCache::insert(&mut self, key: PipelineKey, pipeline: Handle<Pipeline>)` | PipelineKey, Handle<Pipeline> | - | 插入缓存 |
| 413 | 插入管线状态（详细） | `PipelineStateCache::insert(&mut self, key, pipeline)` | PipelineKey, Handle<Pipeline> | - | 与上述一致 |

**优先级**：P1

---

## 5.8 RenderStats3D 渲染统计

### 5.8.1 统计信息

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 145 | 渲染统计 | RenderStats3D：draw_calls / triangles / vertices / entities_rendered / entities_culled | - | - | 渲染性能统计 |
| 171 | 渲染统计（详细） | `RenderStats3D::draw_calls / triangles / vertices / entities_rendered / entities_culled` | - | - | 各项统计值 |
| 172 | 重置统计 | `RenderStats3D::reset(&mut self)` | - | - | 每帧重置 |
| 146 | 重置统计（详细） | `RenderStats3D::reset(&mut self)` | - | - | 与上述一致 |
| 396 | 获取统计 | `RenderPipeline3D::stats(&self) -> &RenderStats3D` | - | &RenderStats3D | 返回统计引用 |
| 443 | 获取统计（详细） | `RenderPipeline3D::stats(&self) -> &RenderStats3D` | - | &RenderStats3D | 与上述一致 |
| 506 | 重置统计（详细） | `RenderStats3D::reset(&mut self)` | - | - | 与上述一致 |
| 449 | Draw Calls 计数 | `RenderStats3D::draw_calls` | - | usize | 绘制调用次数 |
| 450 | 三角面计数 | `RenderStats3D::triangles` | - | usize | 三角形数量 |
| 451 | 顶点数计数 | `RenderStats3D::vertices` | - | usize | 顶点数量 |
| 452 | 渲染实体数 | `RenderStats3D::entities_rendered` | - | usize | 实际渲染实体数 |
| 453 | 裁剪实体数 | `RenderStats3D::entities_culled` | - | usize | 被裁剪实体数 |
| 454 | 点光源数量 | `RenderStats3D::point_lights` | - | usize | 场景点光源数 |
| 455 | 聚光灯数量 | `RenderStats3D::spot_lights` | - | usize | 场景聚光灯数 |
| 456 | 重置（详细） | `RenderStats3D::reset(&mut self)` | - | - | 与上述一致 |
| 457 | 转换为字符串 | `RenderStats3D::to_string(&self) -> String` | - | String | 格式化统计输出 |

**优先级**：P1

---

## 5.9 渲染流程

### 5.9.1 渲染步骤

| 需求ID | 功能描述 | 步骤描述 | 验收标准 |
|--------|----------|----------|----------|
| 134 | 步骤 1 | 清屏（颜色 + 深度 + 模板） | 正确清除帧缓冲 |
| 135 | 步骤 2 | 更新 world transform | 从根到叶传播变换 |
| 136 | 步骤 3 | 视锥裁剪 | 标记可见节点 |
| 137 | 步骤 4 | 收集可见 RenderEntity3D | 生成渲染实体列表 |
| 138 | 步骤 5 | 按 material / mesh 排序 | 减少渲染状态切换 |
| 139 | 步骤 6 | 绑定 VP uniform | 上传视投影矩阵 |
| 140 | 步骤 7 | 逐实体绑定 transform / material / mesh 绘制 | 执行绘制调用 |
| 141 | 步骤 8 | 绘制调试 gizmo（可选） | 渲染调试图形 |

### 5.9.2 错误处理

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 |
|--------|----------|----------|------|------|----------|
| 144 | 着色器编译失败处理 | 错误处理：shader 编译失败时回退到 unlit 并输出错误 | - | - | 降级但不崩溃 |
| 170 | 着色器错误处理（详细） | shader 编译失败时回退到 unlit | - | - | 回退到 unlit 着色器 |

### 5.9.3 空场景处理

| 需求ID | 功能描述 | 验收标准 |
|--------|----------|----------|
| 224 | 默认 RenderPipeline3D 在空场景下也能输出（清屏色） | 渲染管线可在无场景时输出清屏色 |
| 250 | 空场景输出（详细） | 与上述一致 |

**优先级**：P0

---

## 5.10 依赖关系

```
┌─────────────────────────────────────────────────────────┐
│                    engine-render                         │
│              (Renderer, Buffer, Shader, Texture)        │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                  engine-render-3d                        │
│                  (RenderPipeline3D)                     │
└─────────────────────────────────────────────────────────┘
          │                       │
          ▼                       ▼
┌─────────────────┐     ┌─────────────────┐
│    Scene3D      │     │   Camera3D       │
│  场景数据        │     │   VP 矩阵        │
└─────────────────┘     └─────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────┐
│                    Mesh3D / Material3D                   │
│                  网格和材质数据                           │
└─────────────────────────────────────────────────────────┘
```

**上游依赖**：
- `engine-render`：渲染后端抽象

**下游依赖**：
- `examples/`：使用渲染管线
- `editor`：编辑器视口渲染

---

## 5.11 验收标准

### 5.11.1 功能验收

- [ ] `RenderPipeline3D::draw_scene()` 正确执行 8 步渲染流程
- [ ] 着色器编译失败时回退到 unlit 并输出错误日志
- [ ] `RenderStats3D` 正确统计 draw_calls、triangles、vertices
- [ ] 空场景下渲染管线输出清屏色

### 5.11.2 示例验收

- [ ] `examples/3d_scene_simple` 立方体 + 平面 + 方向光正确渲染

---

## 5.12 优先级汇总

| 优先级 | 需求ID | 占比 |
|--------|--------|------|
| P0 | 106-142, 386-461, 134-141 | 80% |
| P1 | 143-144, 171-172, 449-457 | 15% |
| P2 | 145-155 | 5% |

**P0 核心**：RenderPipeline3D 核心、Material3D、Shader3D、渲染流程
**P1 重要**：RenderStats3D、PipelineStateCache
**P2 可选**：高级渲染选项