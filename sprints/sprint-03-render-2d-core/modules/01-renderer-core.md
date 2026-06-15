# 模块一：渲染器核心需求

## 1. 模块概述

渲染器核心是 `engine-render` crate 的基础，提供统一的渲染抽象接口，支持多后端（gl/wgpu/metal）切换。本模块建立 `Renderer` trait、 `RenderContext` 全局上下文、基础绘制 API 以及调试渲染能力。

**核心目标**：在窗口系统之上接入图形 API，提供统一的 2D 渲染接口。

---

## 2. 需求清单

### 2.1 基础架构（需求 1-9）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 1 | engine-render crate 建立 | `mod engine_render` | - | crate 创建 | Cargo.toml 正确配置 | - | P0 |
| 2 | Renderer trait 定义 | `trait Renderer { fn init(window) -> Result<Self>; fn begin_frame() -> Result<()>; fn end_frame() -> Result<()>; fn draw_sprite(...); fn draw_rect(...); }` | Window | Renderer 实例 | trait 完整定义 | - | P0 |
| 3 | 多后端支持 | `enum Backend { Gl, Wgpu, Metal }` | feature 切换 | 后端实例 | gl/wgpu/metal 可切换 | 需求 2 | P0 |
| 4 | 默认后端 gl 打开 | `[features] default = ["render-gl"]` | - | gl 后端 | gl 默认启用 | 需求 3 | P0 |
| 5 | RenderContext 全局上下文 | `struct RenderContext` | - | 帧状态、缓存、管线 | 全局可访问 | 需求 2 | P0 |
| 6 | RenderQueue 绘制指令队列 | `struct RenderQueue` | Draw 指令 | 队列 | 指令正确排队 | 需求 5 | P1 |
| 7 | ClearColor 清屏色可配置 | `fn set_clear_color(&mut self, color: Color)` | Color | - | 颜色可设置 | 需求 2 | P1 |
| 8 | Swapchain 抽象 | `trait Swapchain { fn present(&mut self); }` | - | 呈现 | present() 可调用 | 需求 2 | P1 |
| 9 | Framebuffer 抽象 | `struct Framebuffer` | - | 帧缓冲 | 可创建和使用 | 需求 2 | P1 |

### 2.2 Renderer Trait 详细 API（需求 121-160, 192-194）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 121 | 创建 Renderer | `Renderer::new(window) -> Result<Self>` | Window | Renderer | 成功创建 | 需求 1-2 | P0 |
| 122 | 获取默认后端 | `Renderer::default_backend() -> &str` | - | 后端名称字符串 | 返回 "gl"/"wgpu" | 需求 4 | P1 |
| 123 | 后端信息 | `Renderer::backend_info(&self) -> String` | - | 后端详情 | 返回版本等信息 | 需求 3 | P1 |
| 157 | 清屏颜色设置 | `Renderer::set_clear_color(&mut self, color: Color)` | Color | - | 颜色生效 | 需求 7 | P0 |
| 158 | 垂直同步设置 | `Renderer::set_vsync(&mut self, mode: VsyncMode)` | VsyncMode | - | 模式切换 | 需求 2 | P1 |
| 159 | 分辨率设置 | `Renderer::set_resolution(&mut self, w: u32, h: u32)` | w, h | - | 分辨率改变 | 需求 2 | P0 |
| 160 | 窗口缩放 | `Renderer::resize(&mut self, w: u32, h: u32)` | w, h | - | 重新设置 | 需求 2 | P0 |
| 161 | 开始帧 | `Renderer::begin_frame(&mut self) -> Result<()>` | - | Result | 帧开始 | 需求 2 | P0 |
| 162 | 结束帧 | `Renderer::end_frame(&mut self) -> Result<()>` | - | Result | 帧结束 | 需求 2 | P0 |
| 163 | 呈现 | `Renderer::present(&mut self)` | - | - | 呈现到屏幕 | 需求 8 | P0 |
| 164 | 推送裁剪矩形 | `Renderer::push_scissor_rect(&mut self, rect: Rect)` | Rect | - | 裁剪生效 | 需求 2 | P1 |
| 165 | 弹出裁剪矩形 | `Renderer::pop_scissor_rect(&mut self)` | - | - | 恢复上一级 | 需求 164 | P1 |
| 166 | 推送变换矩阵 | `Renderer::push_transform(&mut self, mat: Mat4)` | Mat4 | - | 矩阵压栈 | 需求 2 | P1 |
| 167 | 弹出变换矩阵 | `Renderer::pop_transform(&mut self)` | - | - | 矩阵出栈 | 需求 166 | P1 |
| 168 | 设置混合模式 | `Renderer::set_blend_mode(&mut self, mode: BlendMode)` | BlendMode | - | 混合模式生效 | 需求 69 | P0 |
| 169 | 重置混合模式 | `Renderer::reset_blend_mode(&mut self)` | - | - | 恢复默认 | 需求 168 | P0 |
| 170 | 刷新队列 | `Renderer::flush(&mut self)` | - | - | 所有指令执行 | 需求 6 | P0 |
| 171 | 渲染统计 | `Renderer::stats(&self) -> RenderStats` | - | RenderStats | 返回统计数据 | 需求 119 | P1 |
| 172 | 获取相机 | `Renderer::camera(&self) -> Option<&Camera2D>` | - | Option<Camera2D> | 返回当前相机 | 需求 62 | P1 |
| 173 | 设置相机 | `Renderer::set_camera(&mut self, camera: Camera2D)` | Camera2D | - | 相机生效 | 需求 62 | P1 |

### 2.3 绘制方法（需求 114-118, 174-189）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 114 | 绘制四边形 | `Renderer::draw_quad(&mut self, quad: Quad)` | Quad | - | 绘制成功 | 需求 70 | P0 |
| 115 | 绘制纹理 | `Renderer::draw_texture(&mut self, tex: TextureHandle, x: f32, y: f32, color: Color)` | tex, x, y, color | - | 绘制成功 | 需求 16 | P0 |
| 116 | 高级纹理绘制 | `Renderer::draw_texture_ex(&mut self, tex: TextureHandle, x: f32, y: f32, params: DrawParams)` | tex, x, y, params | - | 按参数绘制 | 需求 68 | P0 |
| 117 | 纹理绘制（source/dest） | `Renderer::draw_texture_pro(&mut self, tex, source, dest, origin, rot, color)` | tex, source, dest, origin, rot, color | - | 高级绘制 | 需求 115 | P1 |
| 175 | 旋转纹理绘制 | `Renderer::draw_texture_rotated(&mut self, tex, x, y, angle, color)` | tex, x, y, angle, color | - | 旋转绘制 | 需求 115 | P1 |
| 176 | 纹理矩形绘制 | `Renderer::draw_texture_rect(&mut self, tex, source, dest, color)` | tex, source, dest, color | - | 区域绘制 | 需求 115 | P1 |
| 177 | 绘制矩形 | `Renderer::draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color)` | x, y, w, h, color | - | 矩形显示 | 需求 70 | P0 |
| 178 | 绘制矩形边框 | `Renderer::draw_rectangle_lines(&mut self, x, y, w, h, thickness, color)` | x, y, w, h, thickness, color | - | 边框显示 | 需求 177 | P0 |
| 179 | 旋转矩形绘制 | `Renderer::draw_rectangle_rotated(&mut self, x, y, w, h, angle, color)` | x, y, w, h, angle, color | - | 旋转矩形 | 需求 177 | P1 |
| 180 | 绘制圆形 | `Renderer::draw_circle(&mut self, x: f32, y: f32, r: f32, color: Color)` | x, y, r, color | - | 圆形显示 | 需求 70 | P0 |
| 181 | 绘制圆形边框 | `Renderer::draw_circle_lines(&mut self, x, y, r, thickness, color)` | x, y, r, thickness, color | - | 圆形边框 | 需求 180 | P0 |
| 182 | 绘制线条 | `Renderer::draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color)` | x1, y1, x2, y2, thickness, color | - | 线条显示 | 需求 70 | P0 |
| 183 | 绘制三角形 | `Renderer::draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color)` | p1, p2, p3, color | - | 三角形显示 | 需求 70 | P0 |
| 184 | 绘制三角形边框 | `Renderer::draw_triangle_lines(&mut self, p1, p2, p3, thickness, color)` | p1, p2, p3, thickness, color | - | 三角形边框 | 需求 183 | P0 |
| 185 | 绘制多边形 | `Renderer::draw_poly(&mut self, x: f32, y: f32, sides: u32, radius: f32, rotation: f32, color: Color)` | x, y, sides, radius, rotation, color | - | 多边形显示 | 需求 70 | P0 |
| 186 | 绘制多边形边框 | `Renderer::draw_poly_lines(&mut self, x, y, sides, radius, rotation, thickness, color)` | x, y, sides, radius, rotation, thickness, color | - | 多边形边框 | 需求 185 | P0 |
| 187 | 绘制文本（留位） | `Renderer::draw_text(&mut self, text: &str, x: f32, y: f32, font_size: u32, color: Color)` | text, x, y, font_size, color | - | 留位接口 | 需求 86 | P2 |
| 118 | 变换矩阵栈操作 | `Renderer::push_transform_matrix / pop_transform_matrix` | Mat4 | - | 栈操作正常 | 需求 166-167 | P1 |
| 119 | 裁剪矩形栈操作 | `Renderer::push_scissor_rect / pop_scissor_rect` | Rect | - | 栈操作正常 | 需求 164-165 | P1 |
| 120 | 混合模式操作 | `Renderer::set_blend_mode / reset_blend_mode` | BlendMode | - | 模式切换正常 | 需求 168-169 | P0 |
| 121 | 渲染统计 | `Renderer::stats() -> RenderStats` | - | RenderStats | 返回正确统计 | 需求 193 | P1 |
| 122 | RenderStats 默认 | `RenderStats::default()` | - | RenderStats | 返回默认统计 | 需求 119 | P1 |
| 123 | RenderStats 重置 | `RenderStats::reset(&mut self)` | - | - | 统计清零 | 需求 193 | P1 |
| 124 | RenderStats 字段 | `draw_calls, vertices, indices, batches, texture_switches` | - | 各计数 | 字段正确 | 需求 119 | P1 |

### 2.4 DebugRenderer（需求 94-98, 337-349）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 94 | DebugRenderer 创建 | `DebugRenderer::new()` | - | DebugRenderer | 创建成功 | 需求 2 | P1 |
| 95 | 绘制线段 | `DebugRenderer::line(&mut self, a: Vec2, b: Vec2, color: Color)` | a, b, color | - | 线段显示 | 需求 94 | P1 |
| 96 | 绘制矩形 | `DebugRenderer::rect(&mut self, rect: Rect, color: Color)` | rect, color | - | 矩形显示 | 需求 94 | P1 |
| 97 | 绘制圆形 | `DebugRenderer::circle(&mut self, c: Vec2, r: f32, color: Color)` | c, r, color | - | 圆形显示 | 需求 94 | P1 |
| 98 | 绘制文本 | `DebugRenderer::text(&mut self, text: &str, pos: Vec2, color: Color)` | text, pos, color | - | 留位 | 需求 94 | P2 |
| 337 | DebugRenderer 新实例 | `DebugRenderer::new()` | - | Self | 同需求 94 | 需求 94 | P1 |
| 338 | 绘制线段 | `DebugRenderer::line(a: Vec2, b: Vec2, color: Color)` | Vec2, Vec2, Color | - | 显示线段 | 需求 337 | P1 |
| 339 | 绘制矩形填充 | `DebugRenderer::rect(rect: Rect, color: Color)` | Rect, Color | - | 填充矩形 | 需求 337 | P1 |
| 340 | 绘制矩形边框 | `DebugRenderer::rect_lines(rect: Rect, color: Color)` | Rect, Color | - | 矩形边框 | 需求 339 | P1 |
| 341 | 绘制圆形填充 | `DebugRenderer::circle(c: Vec2, r: f32, color: Color)` | Vec2, f32, Color | - | 填充圆 | 需求 337 | P1 |
| 342 | 绘制圆形边框 | `DebugRenderer::circle_lines(c: Vec2, r: f32, color: Color)` | Vec2, f32, Color | - | 圆形边框 | 需求 341 | P1 |
| 343 | 绘制文本 | `DebugRenderer::text(text: &str, pos: Vec2, color: Color)` | &str, Vec2, Color | - | 留位实现 | 需求 337 | P2 |
| 344 | 绘制十字 | `DebugRenderer::cross(pos: Vec2, size: f32, color: Color)` | Vec2, f32, Color | - | 十字显示 | 需求 337 | P1 |
| 345 | 绘制网格 | `DebugRenderer::grid(origin: Vec2, cell_size: f32, cols: u32, rows: u32, color: Color)` | Vec2, f32, u32, u32, Color | - | 网格显示 | 需求 337 | P1 |
| 346 | 刷新调试绘制 | `DebugRenderer::flush(ctx: &mut RenderContext)` | ctx | - | 绘制到屏幕 | 需求 337 | P1 |
| 347 | 清空调试绘制 | `DebugRenderer::clear(&mut self)` | - | - | 清空队列 | 需求 337 | P1 |

### 2.5 Profiler（需求 98-107）

| 需求ID | 功能描述 | API 签名 | 输入 | 输出 | 验收标准 | 依赖 | 优先级 |
|--------|----------|----------|------|------|----------|------|--------|
| 98 | 性能剖析器推送 | `Profiler::push_scope(name: &str)` | name | - | 作用域开始 | 需求 2 | P2 |
| 99 | 性能剖析器弹出 | `Profiler::pop_scope()` | - | - | 作用域结束 | 需求 98 | P2 |
| 100 | 性能剖析器导出 | `Profiler::dump() -> String` | - | String | 返回统计信息 | 需求 98 | P2 |

---

## 3. 验收标准

### 3.1 功能验收

- [ ] `Renderer::new(window)` 成功创建渲染器实例
- [ ] `Renderer::begin_frame() / end_frame()` 正确管理帧生命周期
- [ ] `Renderer::draw_*` 系列方法正确绘制图形
- [ ] 多后端切换（gl/wgpu）正常工作
- [ ] RenderStats 正确统计 draw_calls, vertices, indices, batches
- [ ] DebugRenderer 可绘制调试图形

### 3.2 质量验收

- [ ] clippy 无 warning
- [ ] fmt 检查通过
- [ ] cargo doc 生成成功
- [ ] 单元测试覆盖核心 API

---

## 4. 依赖关系图

```
┌─────────────────────────────────────┐
│         engine-render crate         │
├─────────────────────────────────────┤
│  Renderer Trait                     │
│  ├── RenderContext (全局上下文)       │
│  ├── RenderQueue (指令队列)           │
│  ├── RenderStats (统计)              │
│  ├── DebugRenderer (调试)            │
│  └── Profiler (性能)                 │
└─────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│     Backend Implementations         │
│  ├── GlRenderer (默认)              │
│  ├── WgpuRenderer (render-wgpu)    │
│  └── MetalRenderer (预留)           │
└─────────────────────────────────────┘
```

---

## 5. 备注

- 需求 113（纹理压缩）和 114（多线程渲染）标记为留位，不在本 Sprint 实现
- WebAssembly 支持（需求 110, 384-385）为可选目标
- 需求 116-119 关于 clippy/fmt/doc/CI 属于工程化验收标准