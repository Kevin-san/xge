# 模块四：Bloom 与景深（Depth of Field）

## 模块名称与概述

**模块名称**：Bloom and Depth of Field  
**模块路径**：`engine-postfx` crate  
**功能概述**：实现 HDR Bloom（亮度提取 → 多尺度高斯模糊 → 合成）与散景景深（Circle of Confusion + Bokeh Kernel 卷积），支持 lens dirt 特效。

---

## 需求编号

对应原需求文档编号：**115, 129-135, 175-182, 393-416, 484-492**

---

## 功能描述

---

## Part A：Bloom Pass

### A.1 核心结构

```rust
pub struct BloomPass {
    intensity: f32,          // 发光强度
    threshold: f32,         // HDR 亮度阈值（默认 1.0）
    soft_knee: f32,         // 软阈值过渡（默认 0.5）
    radius: f32,            // 模糊半径
    mip_count: u32,         // Mip 链级别（默认 6）
    dirt_texture: Option<Handle<Texture>>,  // lens dirt 遮罩
    dirt_intensity: f32,
}
```

### A.2 API 签名

```rust
impl BloomPass {
    pub fn new() -> Self;
    pub fn with_intensity(f: f32) -> Self;
    pub fn intensity(&self) -> f32;
    pub fn threshold(&self) -> f32;      // 默认 1.0
    pub fn soft_knee(&self) -> f32;      // 默认 0.5
    pub fn radius(&self) -> f32;
    pub fn mip_count(&self) -> u32;      // 默认 6
    pub fn dirt_texture(&self) -> Option<Handle<Texture>>;
    pub fn dirt_intensity(&self) -> f32;
    
    // 核心 Pass
    pub fn extract_bright(&self, src: &Texture, dst: &mut Texture);
    pub fn downsample(&self, src: &Texture, dst: &mut Texture);  // 2x2 盒式降采样
    pub fn upsample(&self, src: &Texture, dst: &mut Texture);    // 双线性插值升采样
    pub fn gaussian_blur(&self, src: &Texture, dst: &mut Texture, iterations: u32);
    pub fn composite(&self, scene: &Texture, bloom_chain: &[&Texture], output: &mut Texture, intensity: f32);
    pub fn apply(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture);
}
```

### A.3 算法流程

```
输入 HDR 场景纹理
    ↓
extract_bright：亮度提取
    ↓
downsample × N（构建 Mip 链）
    ↓
gaussian_blur（每级）
    ↓
upsample × N（升采样回原尺寸）
    ↓
composite：与原场景叠加 + dirt mask
    ↓
输出
```

### A.4 Bloom 亮度提取

```rust
fn extract_bright(&self, src: &Texture, dst: &mut Texture) {
    // 使用 soft knee 软化阈值过渡
    // threshold - soft_knee 到 threshold + soft_knee 平滑插值
    // luminance = dot(color, vec3(0.2126, 0.7152, 0.0722))
}
```

**输入**：HDR 场景纹理  
**输出**：只保留亮部的纹理

### A.5 Downsample / Upsample

```rust
fn downsample(&self, src: &Texture, dst: &mut Texture) {
    // 2x2 盒式滤波降采样
    // dst.size = src.size / 2
}

fn upsample(&self, src: &Texture, dst: &mut Texture) {
    // 双线性插值升采样
    // dst.size = src.size * 2
}
```

### A.6 Gaussian Blur

```rust
fn gaussian_blur(&self, src: &Texture, dst: &mut Texture, iterations: u32) {
    // 两遍分离高斯（水平 + 垂直）
    // sigma = radius / 3.0
    // iterations = 1 时为单级模糊
}
```

### A.7 Composite 合成

```rust
fn composite(&self, scene: &Texture, bloom_chain: &[&Texture], output: &mut Texture, intensity: f32) {
    // bloom_chain[0] 是最高分辨率的模糊结果
    // output = scene + bloom * intensity
    // 可选：dirt_texture * dirt_intensity 应用 lens dirt
}
```

---

## Part B：Depth of Field Pass

### B.1 核心结构

```rust
pub struct DOFPass {
    focus_distance: f32,    // 对焦距离（米），默认相机到目标
    focal_length: f32,      // 焦距（mm），默认相机设置
    aperture: f32,         // 光圈（f-stop），控制散景大小
    max_blur: f32,          // 最大模糊半径（像素）
    bokeh_shape: BokehShape,
}
```

### B.2 API 签名

```rust
impl DOFPass {
    pub fn new() -> Self;
    pub fn focus_distance(&self) -> f32;  // 米
    pub fn focal_length(&self) -> f32;    // mm
    pub fn aperture(&self) -> f32;       // f-stop
    pub fn max_blur(&self) -> f32;
    pub fn bokeh_shape(&self) -> BokehShape;
    
    // CoC 计算
    pub fn circle_of_confusion(&self, depth: f32) -> f32;
    
    // 分离模糊
    pub fn near_blur(&self, scene: &Texture, depth: &Texture, output: &mut Texture);
    pub fn far_blur(&self, scene: &Texture, depth: &Texture, output: &mut Texture);
    
    // Bokeh 卷积
    pub fn bokeh(&self, coc_map: &Texture, scene: &Texture, output: &mut Texture);
    
    // 合成
    pub fn composite(&self, scene: &Texture, near: &Texture, far: &Texture, coc: &Texture, output: &mut Texture);
    pub fn apply(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture);
}
```

### B.3 BokehShape 散景形状

```rust
pub enum BokehShape {
    Hexagon,
    Disk,
    Polygon { side_count: u32 },  // 可配置多边形
}
```

### B.4 Circle of Confusion 计算

```rust
fn circle_of_confusion(&self, depth: f32) -> f32 {
    // CoC = |depth - focus_distance| * (focal_length / aperture) / depth
    // CoC 受 max_blur 钳制
}
```

**输入**：深度（米）  
**输出**：模糊半径（像素）

### B.5 近/远模糊分离

```rust
fn near_blur(&self, scene: &Texture, depth: &Texture, output: &mut Texture) {
    // depth < focus_distance 的像素
    // 应用 bokeh kernel
}

fn far_blur(&self, scene: &Texture, depth: &Texture, output: &mut Texture) {
    // depth > focus_distance 的像素
    // 应用 bokeh kernel
}
```

### B.6 Bokeh Kernel 卷积

```rust
fn bokeh(&self, coc_map: &Texture, scene: &Texture, output: &mut Texture) {
    // 根据 CoC 大小选择 kernel 半径
    // Hexagon/Disk/Polygon 形状加权采样
    // 大 CoC = 大散景 = 更多样本
}
```

### B.7 DOF Composite

```rust
fn composite(&self, scene: &Texture, near: &Texture, far: &Texture, coc: &Texture, output: &mut Texture) {
    // 基于 CoC alpha 混合 near/far 模糊结果
    // 焦平面内 = 清晰（scene 原样）
}
```

---

## Part C：Bloom 辅助函数

### C.1 Lens Dirt（镜头污渍）

```rust
// dirt_texture 是黑白遮罩图
// 应用到 bloom 合成阶段
// 用于模拟镜头脏污产生的发光效果

pub fn with_dirt_texture(mut self, tex: Handle<Texture>, intensity: f32) -> Self {
    self.dirt_texture = Some(tex);
    self.dirt_intensity = intensity;
    self
}
```

---

## 输入/输出规格

### Bloom

| 阶段 | 输入 | 输出 |
|------|------|------|
| extract_bright | `scene_color (RGBA16F)` | `bright (RGBA16F)` |
| downsample | `mip[n]` | `mip[n+1]` |
| gaussian_blur | `mip[n]` | `mip_blurred[n]` |
| upsample | `mip[n+1]` | `mip[n]` |
| composite | `scene_color` + `bloom_chain` | `output (RGBA16F)` |

### DOF

| 阶段 | 输入 | 输出 |
|------|------|------|
| CoC 计算 | `depth (D32)` | `coc_map (R16F)` |
| near_blur | `scene_color` + `depth` | `near_blur` |
| far_blur | `scene_color` + `depth` | `far_blur` |
| bokeh | `coc_map` + `near_blur/far_blur` | `blurred` |
| composite | `scene` + `near` + `far` + `coc` | `output` |

---

## 验收标准

| ID | 描述 |
|----|------|
| V1 | BloomPass::extract threshold=0 时返回原样 |
| V2 | BloomPass::composite 正确叠加 bloom * intensity |
| V3 | Bloom dirt_texture 正确调制发光区域 |
| V4 | DOFPass circle_of_confusion 计算正确 |
| V5 | 焦平面（depth == focus_distance）CoC = 0 |
| V6 | near_blur 只处理近景（depth < focus） |
| V7 | far_blur 只处理远景（depth > focus） |
| V8 | BokehShape::Polygon(n) 生成 n 边形状 |
| V9 | DOF composite 正确 alpha 混合 |
| V10 | Bloom Mip 链级数 = mip_count |

---

## 依赖关系

**前置依赖**：
- `03-post-processing`：PostProcessContext / IPostProcessPass

**被依赖**：
- `examples/postfx_bloom`：Bloom 演示
- `examples/postfx_dof`：DOF 演示

---

## 优先级

**P0**：
- Bloom 亮度提取 + 降采样 + 升采样 + 合成
- DOF CoC 计算 + 近/远模糊分离

**P1**：
- Bokeh kernel 实现（Hexagon/Disk/Polygon）
- Lens dirt

**P2**：
- 多边形 bokeh side_count 可配置
- 性能优化（半分辨率模糊）

---

## 性能目标

- 后期栈默认链（包括 Bloom/DOF）<= 6ms（1080p / integrated GPU）
- Bloom mip_count 默认 6 级
- DOF max_blur 限制防止过度采样
