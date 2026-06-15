# 模块五：SSAO 与 SSR

## 模块名称与概述

**模块名称**：Screen Space Ambient Occlusion & Screen Space Reflections  
**模块路径**：`engine-postfx` crate  
**功能概述**：实现屏幕空间环境光遮蔽（SSAO/HBAO）与屏幕空间反射（SSR）基础，为场景添加深度感和真实感。

---

## 需求编号

对应原需求文档编号：**116-117, 136-140, 417-426, 509-526**

---

## 功能描述

---

## Part A：SSAO Pass（Screen Space Ambient Occlusion）

### A.1 核心结构

```rust
pub struct SSAOPass {
    radius: f32,          // 采样半径（默认 0.5）
    bias: f32,            // 深度偏移（默认 0.025）
    power: f32,           // AO 强度（默认 2.0）
    kernel_size: u32,    // 采样点数（默认 32）
    kernel: Vec<Vec3>,   // hemisphere kernel
    noise_texture: Handle<Texture>,  // 4x4 随机旋转噪声
}
```

### A.2 API 签名

```rust
impl SSAOPass {
    pub fn new() -> Self;
    pub fn radius(&self) -> f32;        // 默认 0.5
    pub fn bias(&self) -> f32;           // 默认 0.025
    pub fn power(&self) -> f32;          // 默认 2.0
    pub fn kernel_size(&self) -> u32;   // 默认 32
    pub fn noise_texture(&self) -> Handle<Texture>;
    pub fn kernel(&self) -> &[Vec3];
    
    // 生成
    pub fn generate_kernel(&self, seed: u32) -> Vec<Vec3>;
    pub fn generate_noise_texture(&self) -> Handle<Texture>;
    
    // 核心 Pass
    pub fn apply(&self, scene_color: &Texture, depth: &Texture, normal: &Texture, output: &mut Texture);
    
    // 后处理
    pub fn blur(&self, input: &Texture, output: &mut Texture);  // 双边滤波去噪
    pub fn blend_with_scene(&self, ao_map: &Texture, scene: &Texture, output: &mut Texture);
}
```

### A.3 算法流程

```
输入：depth (D32) + normal (RGB10A2) + scene_color
    ↓
计算 view-space position（从 depth 反投影）
    ↓
对每个像素：
    - 读取随机旋转噪声向量
    - 在 hemisphere 上采样 kernel
    - 对每个样本：project + range check + occlusion test
    ↓
AO = 1 - (occluded_samples / total_samples)
AO = pow(AO, power)  // 调制强度
    ↓
blur（可选）：双边滤波保边去噪
    ↓
blend_with_scene：AO * scene_color 或单独输出 AO
    ↓
输出：AO map 或调制后场景
```

### A.4 Kernel 生成

```rust
fn generate_kernel(&self, seed: u32) -> Vec<Vec3> {
    // 生成 32 个 hemisphere 方向向量
    // 使用 spherical cap 分布（更多近处样本）
    // 长度分布在 [0, 1]，后期缩放 * radius
}
```

### A.5 Noise Texture

```rust
fn generate_noise_texture(&self) -> Handle<Texture> {
    // 4x4 RGBA 纹理
    // 每个像素：随机旋转角度的切比雪夫方向向量
    // 用于打破采样 pattern 的规律性
}
```

### A.6 Blur（双边滤波）

```rust
fn blur(&self, input: &Texture, output: &mut Texture) {
    // 双边滤波：边缘保持滤波
    // 权重 = spatial_gaussian * range_gaussian
    // 5x5 或 7x7 kernel
}
```

---

## Part B：HBAO Pass（Hemisphere-based Ambient Occlusion）

```rust
pub struct HBAOPass {
    radius: f32,
    bias: f32,
    power: f32,
    kernel_size: u32,
    // HBAO 特有：基于半球角度积分
    max_distance: f32,
    step_count: u32,
}

impl HBAOPass {
    pub fn new() -> Self;
    pub fn apply(&self, depth: &Texture, normal: &Texture, output: &mut Texture);
}
```

**HBAO vs SSAO**：
- SSAO：单一 hemisphere 方向（+Y 或 +Z）
- HBAO：基于法线的半球积分（更准确但更慢）

---

## Part C：SSR Pass（Screen Space Reflections）

### C.1 核心结构

```rust
pub struct SSRPass {
    step_count: u32,              // 射线步进数（默认 64）
    thickness: f32,               // 物体厚度（默认 0.5）
    binary_search_steps: u32,     // 二分搜索步数（默认 8）
    max_distance: f32,            // 最大反射距离
}
```

### C.2 API 签名

```rust
impl SSRPass {
    pub fn new() -> Self;
    pub fn step_count(&self) -> u32;              // 默认 64
    pub fn thickness(&self) -> f32;                // 默认 0.5
    pub fn binary_search_steps(&self) -> u32;      // 默认 8
    pub fn max_distance(&self) -> f32;
    
    pub fn apply(
        &self,
        depth: &Texture,
        normal: &Texture,
        scene_color: &Texture,
        output: &mut Texture,
        history_opt: Option<&Texture>,  // 可选时序滤波
    );
}
```

### C.3 算法流程

```
输入：depth (D32) + normal (RGB10A2) + scene_color
    ↓
计算 view-space position + view-space normal
    ↓
反射方向 = reflect(-view_dir, normal)
    ↓
Ray marching：
    for i in 0..step_count:
        ray_pos += reflect_dir * step_size
        projected_pos = project_to_screen(ray_pos)
        ray_depth = sample_depth(projected_pos)
        if ray_depth < ray_pos.z:
            # 找到交点
            binary_search()
            break
    ↓
若无交点：使用环境/天空盒
    ↓
混合：reflection * fresnel + scene * (1 - fresnel)
    ↓
输出
```

### C.4 射线步进

```rust
fn ray_march(&self, origin: Vec3, direction: Vec3, depth: &Texture) -> Option<Vec3> {
    let step_size = thickness / step_count as f32;
    let mut t = 0.0;
    
    for _ in 0..step_count {
        let pos = origin + direction * t;
        let proj = project(pos);
        let scene_depth = depth.sample(proj);
        
        if scene_depth < pos.z {
            // 命中
            return Some(binary_search(origin, direction, t, t - step_size));
        }
        t += step_size;
    }
    None
}
```

### C.5 二分搜索

```rust
fn binary_search(&self, origin: Vec3, dir: Vec3, near: f32, far: f32) -> Vec3 {
    let mut t = (near + far) / 2.0;
    for _ in 0..binary_search_steps {
        let pos = origin + dir * t;
        let proj = project(pos);
        let scene_depth = depth.sample(proj);
        
        if (scene_depth - pos.z).abs() < epsilon {
            return pos;
        } else if scene_depth < pos.z {
            far = t;
        } else {
            near = t;
        }
        t = (near + far) / 2.0;
    }
    t
}
```

### C.6 历史缓冲（可选时序滤波）

```rust
// SSR 会产生噪点，时序滤波可以平滑
// 使用 TAA 类似的 history buffer
// 当前位置的反射与历史缓冲混合
```

---

## 输入/输出规格

### SSAO

| 阶段 | 输入 | 输出 |
|------|------|------|
| AO generate | depth + normal + noise | `ao_map (R8 or R16F)` |
| blur | `ao_map` | `ao_blurred` |
| blend | `ao_blurred` + `scene` | `output` |

### SSR

| 阶段 | 输入 | 输出 |
|------|------|------|
| ray march | depth + normal | `reflect_map (RGBA8)` |
| composite | `reflect_map` + `scene` + fresnel | `output` |

---

## 验收标准

| ID | 描述 |
|----|------|
| V1 | SSAOPass::generate_kernel 生成指定数量向量 |
| V2 | SSAOPass blur 保持边缘清晰 |
| V3 | SSAOPass blend_with_scene 正确调制 |
| V4 | SSR step_count=1 时退化为简单射线 |
| V5 | SSR 无交点时返回天空盒/环境色 |
| V6 | SSR thickness 影响 ray termination 灵敏度 |
| V7 | SSR binary_search_steps 影响精度 |
| V8 | HBAO 基于法线的 hemisphere 积分更准确 |
| V9 | 半分辨率 AO 提升性能（结果降采样） |

---

## 依赖关系

**前置依赖**：
- `03-post-processing`：PostProcessContext
- 深度缓冲：depth (D32S8)
- 法线缓冲：normal (RGB10A2)

**被依赖**：
- `examples/postfx_ssao`：SSAO 演示

---

## 优先级

**P0**：
- SSAO 基础实现（kernel + noise + ray march）
- SSAO blur

**P1**：
- SSR 基础实现
- HBAO

**P2**：
- SSR 历史缓冲时序滤波
- SSR 粗糙度调制反射强度

---

## 性能目标

- SSAO kernel_size=32，半分辨率 <= 2ms
- SSR step_count=64，full 分辨率 <= 3ms
- HBAO 比 SSAO 慢 30-50%
