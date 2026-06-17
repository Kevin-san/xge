# Module 05 — GPU 粒子系统

> 上游 sprint: [Sprint 21](../sprint-21-animation-particle-postfx.md)
> 文件位置: `engine-particle/src/`

## 1. ParticleSystem

```rust
pub struct ParticleSystem {
    pub emitters: Vec<Emitter>,
    pub modules: Vec<Box<dyn ParticleModule>>,
    pub max_particles: u32,
    pub gpu_buffer: GpuBuffer,  // 粒子缓冲
    pub compute_pipeline: GpuPipelineState,
}

pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub age: f32,
    pub lifetime: f32,
    pub color: Vec4,
    pub size: f32,
    pub rotation: f32,
}
```

## 2. Emitter

```rust
pub struct Emitter {
    pub shape: EmitterShape,
    pub rate: f32,           // 每秒发射数量
    pub duration: f32,       // 持续时间
    pub looping: bool,
    pub start_lifetime: Range<f32>,
    pub start_speed: Range<f32>,
    pub start_size: Range<f32>,
    pub start_color: Range<Vec4>,
    pub start_rotation: Range<f32>,
    pub gravity_scale: f32,
}

pub enum EmitterShape {
    Point,
    Sphere { radius: f32 },
    Cone { angle: f32, height: f32 },
    Box { half_extents: Vec3 },
    Mesh { mesh: MeshHandle },
}
```

## 3. Module（数据驱动）

```rust
pub trait ParticleModule: Send + Sync {
    fn name(&self) -> &str;
    fn update(&self, particles: &mut [Particle], dt: f32);
}

pub struct ColorOverLife {
    pub gradient: Gradient<Vec4>,
}
impl ParticleModule for ColorOverLife {
    fn update(&self, particles: &mut [Particle], dt: f32) {
        for p in particles {
            let t = p.age / p.lifetime;
            p.color = self.gradient.sample(t);
        }
    }
}

pub struct SizeOverLife {
    pub curve: Curve<f32>,
}

pub struct ForceField {
    pub force: Vec3,
    pub falloff: f32,
}

// Niagara 风格模块
pub struct Gravity { pub acceleration: Vec3 }
pub struct Wind { pub direction: Vec3, pub strength: f32 }
pub struct Vortex { pub center: Vec3, pub axis: Vec3, pub strength: f32 }
pub struct Turbulence { pub frequency: f32, pub amplitude: f32 }
```

## 4. GPU Compute 模拟

```glsl
// compute shader
layout(set = 0, binding = 0) buffer Particles {
    ParticleData particles[];
};

layout(set = 0, binding = 1) uniform EmitterData {
    float dt;
    float time;
    uint emit_count;
    uint max_particles;
};

void main() {
    uint id = gl_GlobalInvocationID.x;
    if (id >= max_particles) return;
    
    ParticleData p = particles[id];
    if (p.lifetime <= 0.0) {
        // 死亡：从死亡列表取出 / 随机
        return;
    }
    
    p.age += dt;
    if (p.age >= p.lifetime) {
        p.lifetime = -1.0;  // 标记死亡
        return;
    }
    
    // 应用力场
    p.velocity += gravity * dt;
    p.velocity += wind * dt;
    p.velocity += vortex_force(p.position, p.velocity) * dt;
    p.velocity += turbulence(p.position, time) * dt;
    
    p.position += p.velocity * dt;
    
    particles[id] = p;
}
```

## 5. Soft Particle（深度融合）

```glsl
// 渲染 vertex shader
out float gl_FragDepth;

void main() {
    vec4 clip = projection * view * vec4(position, 1.0);
    gl_Position = clip;
    gl_FragDepth = clip.z / clip.w;  // 用于软深度
}

// fragment shader
in float vAlpha;
in float vDepth;

void main() {
    float scene_depth = texture(depth_map, screen_uv).r;
    float fade = clamp((scene_depth - vDepth) * softness, 0.0, 1.0);
    gl_FragColor = vec4(color.rgb, color.a * fade);
}
```

## 6. 验收

- [ ] 10000 GPU 粒子 60 FPS
- [ ] 100 发射器混合 < 0.5 ms CPU
- [ ] 4 种力场同时作用视觉正确
- [ ] Soft particle 边缘无硬切
- [ ] Sprite / Mesh 粒子支持
- [ ] Niagara 风格模块组合
