# 模块一：粒子系统核心

## 模块名称与概述

**模块名称**：Particle System Core  
**模块路径**：`engine-particles` crate  
**功能概述**：建立通用 2D/3D 粒子数据结构与系统框架，包括 Particle、ParticlePool、ParticleSystem 组件，统一生命周期管理与粒子渲染抽象。

---

## 需求编号

对应原需求文档编号：**1-6, 50-64, 103-126, 128-130, 181-270, 326-340, 397-412, 504, 611**

---

## 功能描述

### 1.1 Particle 数据结构

| 字段 | 类型 | 说明 |
|------|------|------|
| position | `Vec3` | 世界/本地位置 |
| velocity | `Vec3` | 速度向量 |
| rotation | `f32` / `Quat` | 2D 绕 Z 轴旋转 / 3D 四元数 |
| size | `Vec2` | 粒子尺寸 |
| color | `RgbaLinear` | 线性空间颜色 |
| age | `f32` | 已存活时间（秒） |
| lifetime | `f32` | 总生命周期（秒） |

**方法**：
- `Particle::new(position, velocity, rotation, size, color, lifetime) -> Self`
- `Particle::is_alive(&self) -> bool`
- `Particle::position(&self) -> Vec3`
- `Particle::velocity(&self) -> Vec3`
- `Particle::rotation(&self) -> f32`（2D）或 `Quat`（3D）
- `Particle::size(&self) -> Vec2`
- `Particle::color(&self) -> Rgba`
- `Particle::age(&self) -> f32`
- `Particle::lifetime(&self) -> f32`
- `Particle::normalized_age(&self) -> f32`（age / lifetime）
- `Particle::update(&mut self, dt)`

---

### 1.2 ParticlePool

SoA（Structure of Arrays）结构存放大量粒子数据。

**API 签名**：
```rust
pub struct ParticlePool { /* SoA layout */ }

impl ParticlePool {
    pub fn new(max: usize) -> Self;
    pub fn spawn(&mut self, particle: Particle) -> bool;
    pub fn kill(&mut self, index: usize);
    pub fn alive_count(&self) -> usize;
    pub fn dead_count(&self) -> usize;
    pub fn swap_remove(&mut self, index: usize);
    pub fn particle_at(&self, index: usize) -> Option<&Particle>;
    pub fn particle_at_mut(&mut self, index: usize) -> Option<&mut Particle>;
    pub fn alive_indices(&self) -> &[usize];
}
```

**输入**：Particle 实例、最大容量  
**输出**：spawn 返回 bool（成功/失败），索引操作无输出

---

### 1.3 ParticleSystem 组件

管理一组 Emitter 与全局粒子池。

**API 签名**：
```rust
pub struct ParticleSystem {
    max_particles: usize,
    emitters: Vec<ParticleEmitter>,
    pool: ParticlePool,
    time: f32,
    playing: bool,
    simulation_space: SimulationSpace,
    scaling_mode: ScalingMode,
    gravity_modifier: f32,
    prewarm: bool,
    prewarm_time: f32,
    random_seed: u64,
    delta_time_scale: f32,
}

impl ParticleSystem {
    pub fn new() -> Self;
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) -> EmitterHandle;
    pub fn remove_emitter(&mut self, handle: EmitterHandle);
    pub fn emitters(&self) -> &[ParticleEmitter];
    pub fn particle_count(&self) -> usize;
    pub fn max_particles(&self) -> usize;
    pub fn set_max_particles(&mut self, n: usize);
    pub fn play(&mut self);
    pub fn pause(&mut self);
    pub fn stop(&mut self);
    pub fn clear(&mut self);
    pub fn update(&mut self, dt: f32);
    pub fn pool(&self) -> &ParticlePool;
    pub fn time(&self) -> f32;
    pub fn is_playing(&self) -> bool;
    pub fn simulation_space(&self) -> SimulationSpace;
    pub fn set_simulation_space(&mut self, space: SimulationSpace);
    pub fn scaling_mode(&self) -> ScalingMode;
    pub fn gravity_modifier(&self) -> f32;
    pub fn set_gravity_modifier(&mut self, g: f32);
    pub fn prewarm(&self) -> bool;
    pub fn prewarm_time(&self) -> f32;
    pub fn random_seed(&self) -> u64;
    pub fn set_random_seed(&mut self, seed: u64);
    pub fn delta_time_scale(&self) -> f32;
    pub fn set_delta_time_scale(&mut self, s: f32);
}
```

**输入**：dt（delta time）  
**输出**：无（内部状态更新）

---

### 1.4 SimulationSpace 与 ScalingMode

```rust
pub enum SimulationSpace {
    Local,
    World,
}

pub enum ScalingMode {
    Hierarchy,
    Local,
    ShapeOnly,
}
```

---

### 1.5 GPU 粒子系统（可选 Feature）

```rust
#[cfg(feature = "gpu_particles")]
pub struct GpuParticleSystem {
    max_count: u32,
    storage_buffer: BufferHandle,
}

#[cfg(feature = "gpu_particles")]
impl GpuParticleSystem {
    pub fn new(max_count: u32) -> Self;
    pub fn count(&self) -> u32;
    pub fn max_count(&self) -> u32;
    pub fn storage_buffer(&self) -> BufferHandle;
    pub fn dispatch(&self, cmd_encoder: &mut CommandEncoder, dt: f32);
}
```

**GPU 粒子数据结构**（128B 对齐）：
```rust
struct GpuParticle {
    pos: Vec3,
    vel: Vec3,
    rot: Quat,
    size: Vec2,
    color: Vec4,
    age: f32,
    life: f32,
    pad: f32, // padding to 128B
}
```

---

## 验收标准

| ID | 描述 |
|----|------|
| V1 | `Particle::is_alive` 在 age < lifetime 时返回 true |
| V2 | `ParticlePool::spawn` 成功返回 true，池满返回 false |
| V3 | `ParticlePool::kill` 将粒子标记为死亡 |
| V4 | `ParticleSystem::add_emitter` 返回有效 Handle |
| V5 | `ParticleSystem::update` 推进所有活跃粒子 age |
| V6 | `ParticleSystem::play/pause/stop` 正确控制状态 |
| V7 | `ParticleSystem::max_particles` 可调整 |
| V8 | GPU 粒子 feature gate 正确隔离 platform-specific 代码 |
| V9 | SoA 布局支持批量 SIMD 更新 |

---

## 依赖关系

**前置依赖**：
- `engine-core`：基础类型（Vec3, Quat, Rgba, Handle）
- `engine-render`：渲染抽象（CommandEncoder, BufferHandle）
- `engine-ecs`：Component trait 实现

**被依赖**：
- `engine-particles` 被 `02-emitters-forces` 模块依赖（Emitter 管理粒子）
- 渲染系统依赖 `ParticleRenderList`

---

## 优先级

**P0**（核心必需）：
- Particle/ParticlePool 数据结构
- ParticleSystem 生命周期管理
- play/pause/stop/clear/update

**P1**（重要功能）：
- simulation_space / scaling_mode
- prewarm / random_seed
- GPU 粒子框架

**P2**（优化/可选）：
- GPU compute dispatch 实现
- 性能基准测试

---

## 渲染管线集成

| 属性 | 值 |
|------|-----|
| render graph 阶段 | `PostTransparent` |
| 深度写入 | 默认关闭 |
| 透明排序 | 支持 sort by depth（back-to-front） |

---

## 性能目标

- 100,000 CPU 粒子单帧更新 <= 8ms（release / x86_64）
- 支持 soft particle fade（与 scene depth 融合）
- 支持 frustum culling per emitter
- 支持 LOD（距离降级粒子数量）
