# 示例实现指南

## 概述

本文档提供 Sprint 13 新增示例工程的实现指导，包括粒子系统示例（8个）和后期特效示例（5个），共 13 个示例。

---

## 一、粒子系统示例（engine-particles）

### 1.1 particles_2d

**目标**：展示 2D 场景中的 sprite billboard 粒子

**场景设置**：
```rust
// 创建正交相机（2D 视角）
let camera = Camera::orthographic();

// 创建粒子系统
let mut particle_system = ParticleSystem::new();

// 创建精灵材质
let sprite_material = load("particle_sprite.png");
let material = ParticleMaterial::PBR {
    albedo: Rgba::WHITE,
    metallic: 0.0,
    roughness: 1.0,
    blending: Blending::Additive,
};

// 创建圆形发射器
let circle_shape = EmitShape::Circle(1.0, Axis::Z);
let emitter = ParticleEmitter::new(circle_shape, EmissionMode::Continuous(100.0), material);
particle_system.add_emitter(emitter);
```

**关键配置**：
- EmitShape：Circle（2D 发射）
- RenderMode：SpriteBillboard
- Blending：Additive（火焰/发光效果常用）

---

### 1.2 particles_3d

**目标**：3D 场景中的多种发射器组合

**场景设置**：
```rust
// 场景包含：
// 1. 火球效果 - Point emitter + fire material
// 2. 烟雾效果 - Box emitter + smoke material  
// 3. 火花效果 - Sphere emitter + spark material

// 火球发射器
let fire_emitter = ParticleEmitter::new(
    EmitShape::Sphere(0.5, SphereEmitMode::Shell),
    EmissionMode::Continuous(200.0),
    fire_material,
);

// 烟雾发射器（sub-emitter）
let smoke_emitter = ParticleEmitter::new(
    EmitShape::Box(Vec3::new(0.3, 0.3, 0.3)),
    EmissionMode::Continuous(50.0),
    smoke_material,
);

// 火花发射器
let spark_emitter = ParticleEmitter::new(
    EmitShape::Point,
    EmissionMode::Burst(vec![BurstConfig::new(0.0, 50, u32::MAX, 0.5)]),
    spark_material,
);
```

**关键配置**：
- 多种 EmitShape 组合
- Burst + Continuous 混合发射
- 不同渲染模式切换

---

### 1.3 particles_fire

**目标**：火焰 + 烟雾子发射器级联

**场景设置**：
```rust
// 火焰发射器（主）
let fire_emitter = ParticleEmitter::new(
    EmitShape::Cone(45.0_f32.to_radians(), 0.5, 2.0, ConeEmitMode::Volume),
    EmissionMode::Continuous(100.0),
    fire_material,
);

// 烟雾子发射器（绑定到火焰粒子死亡时触发）
let smoke_sub = SubEmitterModule::new(smoke_emitter_handle, SubEmitterTrigger::OnDeath);
fire_emitter.add_module(Box::new(smoke_sub));

// 颜色梯度：黄色→橙色→红色→黑色
let fire_gradient = ColorGradient::new(vec![
    (0.0, Rgba::new(1.0, 1.0, 0.0, 1.0)),   // 黄色
    (0.3, Rgba::new(1.0, 0.5, 0.0, 1.0)),   // 橙色
    (0.7, Rgba::new(1.0, 0.1, 0.0, 1.0)),   // 红色
    (1.0, Rgba::new(0.0, 0.0, 0.0, 0.0)),  // 黑色（淡出）
]);
fire_emitter.add_module(Box::new(ColorOverLifeModule::new(fire_gradient)));

// 初始向上速度
fire_emitter.add_module(Box::new(InitialVelocityModule::new(
    Vec3::new(0.0, 5.0, 0.0),
    Vec3::new(0.0, 8.0, 0.0),
)));

// 重力（轻微，模拟热气流上升）
fire_emitter.add_module(Box::new(GravityModule::new(Vec3::new(0.0, 2.0, 0.0))));
```

---

### 1.4 particles_smoke

**目标**：体积烟雾 + turbulence 扰动

**场景设置**：
```rust
// 烟雾发射器
let smoke_emitter = ParticleEmitter::new(
    EmitShape::Sphere(1.0, SphereEmitMode::Volume),
    EmissionMode::Continuous(30.0),
    smoke_material,
);

// Turbulence 湍流扰动
let turbulence = TurbulenceModule::new(
    intensity: 2.0,
    frequency: 0.5,
    speed: 1.0,
);
smoke_emitter.add_module(Box::new(turbulence));

// 尺寸随生命周期增大（体积感）
let size_curve = Curve::linear((0.0, 1.0), (1.0, 5.0));
smoke_emitter.add_module(Box::new(SizeOverLifeModule::new(size_curve)));

// 颜色渐变：深灰→浅灰→透明
let smoke_gradient = ColorGradient::new(vec![
    (0.0, Rgba::new(0.3, 0.3, 0.3, 0.5)),
    (0.5, Rgba::new(0.5, 0.5, 0.5, 0.3)),
    (1.0, Rgba::new(0.7, 0.7, 0.7, 0.0)),
]);
smoke_emitter.add_module(Box::new(ColorOverLifeModule::new(smoke_gradient)));

// 长时间生命周期
smoke_emitter.set_duration(10.0);
```

---

### 1.5 particles_rain

**目标**：锥形雨 + StretchedBillboard

**场景设置**：
```rust
// 雨滴发射器
let rain_emitter = ParticleEmitter::new(
    EmitShape::Cone(30.0_f32.to_radians(), 10.0, 50.0, ConeEmitMode::Volume),
    EmissionMode::Continuous(5000.0),
    rain_material,
);

// StretchedBillboard（沿速度方向拉伸）
rain_emitter.set_render_mode(ParticleRenderMode::StretchedBillboard {
    length_scale: 3.0,
    speed_scale: 0.5,
});

// 高速向下
rain_emitter.add_module(Box::new(InitialVelocityModule::new(
    Vec3::new(0.0, -30.0, 0.0),
    Vec3::new(0.0, -40.0, 0.0),
)));

// 小尺寸
let size_curve = Curve::constant(0.05);
rain_emitter.add_module(Box::new(SizeOverLifeModule::new(size_curve)));

// 短生命周期（落地消失）
rain_emitter.set_duration(2.0);
```

---

### 1.6 particles_snow

**目标**：球体雪 + turbulence

**场景设置**：
```rust
// 雪发射器（半球）
let snow_emitter = ParticleEmitter::new(
    EmitShape::Hemisphere(20.0),
    EmissionMode::Continuous(1000.0),
    snow_material,
);

// Turbulence（飘落过程中的随机摆动）
let turbulence = TurbulenceModule::new(
    intensity: 1.0,
    frequency: 0.2,
    speed: 0.5,
);
snow_emitter.add_module(Box::new(turbulence));

// 缓慢下落
snow_emitter.add_module(Box::new(InitialVelocityModule::new(
    Vec3::new(0.0, -2.0, 0.0),
    Vec3::new(0.0, -5.0, 0.0),
)));

// 雪花旋转
let rotation_curve = Curve::linear((0.0, 0.0), (1.0, 360.0_f32.to_radians()));
snow_emitter.add_module(Box::new(RotationOverLifeModule::new(rotation_curve)));

// 白色渐变
let snow_gradient = ColorGradient::new(vec![
    (0.0, Rgba::WHITE),
    (1.0, Rgba::new(1.0, 1.0, 1.0, 0.5)),
]);
snow_emitter.add_module(Box::new(ColorOverLifeModule::new(snow_gradient)));
```

---

### 1.7 particles_collision

**目标**：粒子与地面/球体/盒子碰撞

**场景设置**：
```rust
// 碰撞配置
let colliders = vec![
    // 地面（Plane）
    ParticleCollider::Plane {
        normal: Vec3::Y,
        offset: 0.0,
    },
    // 球体障碍
    ParticleCollider::Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 1.0,
    },
    // 盒子障碍
    ParticleCollider::Box {
        center: Vec3::new(3.0, 1.0, 0.0),
        half_size: Vec3::new(0.5, 0.5, 0.5),
    },
];

let collision_module = CollisionModule::new(colliders);
collision_module.set_bounce(0.5);      // 弹力
collision_module.set_friction(0.2);    // 摩擦力
collision_module.set_kill_threshold(0.1);  // 速度低于阈值则消亡

let mut emitter = ParticleEmitter::new(
    EmitShape::Box(Vec3::new(2.0, 5.0, 2.0)),
    EmissionMode::Continuous(100.0),
    particle_material,
);
emitter.add_module(Box::new(collision_module));
```

---

### 1.8 particles_gpu（Feature: gpu_particles）

**目标**：GPU compute 10万粒子

**场景设置**：
```rust
#[cfg(feature = "gpu_particles")]
{
    let gpu_system = GpuParticleSystem::new(100_000);
    
    // GPU 粒子不需要 CPU 侧的 update
    // 使用 compute shader 模拟
    
    // 配置 GPU 粒子参数
    gpu_system.set_emitter_config(EmitShape::Sphere(5.0, SphereEmitMode::Volume));
    gpu_system.set_velocity_range(Vec3::ONE * -5.0, Vec3::ONE * 5.0);
    
    // 添加到场景
    scene.add_particle_system(gpu_system);
}
```

**性能基准**：
- 100,000 CPU 粒子：<= 8ms/帧（release）
- GPU 粒子：<= 2ms/帧

---

## 二、后期特效示例（engine-postfx）

### 2.1 postfx_stack

**目标**：完整后期链，可切换各个 Pass

**场景设置**：
```rust
// 创建后期栈
let mut stack = PostProcessStack::new();
stack.set_hdr(true);

// 添加各个 Pass
stack.add_pass(Box::new(FXAAPass::new()));
stack.add_pass(Box::new(TAAPass::new()));
stack.add_pass(Box::new(BloomPass::new()));
stack.add_pass(Box::new(DOFPass::new()));
stack.add_pass(Box::new(SSAOPass::new()));
stack.add_pass(Box::new(ToneMappingPass::new(ToneMappingMode::ACES)));
stack.add_pass(Box::new(ColorGradingPass::new()));

// UI 控制
ui.add_checkbox("FXAA", stack.pass::<FXAAPass>().enabled());
ui.add_slider("Bloom Intensity", bloom_pass.intensity());
ui.add_slider("DOF Focus", dof_pass.focus_distance());
ui.add_slider("SSAO Radius", ssao_pass.radius());
```

---

### 2.2 postfx_bloom

**目标**：HDR 场景发光

**场景设置**：
```rust
// 创建 HDR 场景
let mut stack = PostProcessStack::new();
stack.set_hdr(true);

// Bloom Pass
let mut bloom = BloomPass::new();
bloom.set_intensity(1.5);
bloom.set_threshold(1.0);
bloom.set_radius(1.0);
bloom.set_mip_count(6);
stack.add_pass(Box::new(bloom));

// ToneMapping（Bloom 后必须 tone map）
stack.add_pass(Box::new(ToneMappingPass::new(ToneMappingMode::ACES)));

// 场景物体
let emissive_sphere = MeshBuilder::sphere()
    .material(emissive_material)  // 高 emissive 值
    .build();
```

**UI 控制**：
- Bloom Intensity：0.0 ~ 3.0
- Bloom Threshold：0.0 ~ 2.0
- Bloom Radius：0.5 ~ 2.0

---

### 2.3 postfx_dof

**目标**：前景/背景散景

**场景设置**：
```rust
let mut stack = PostProcessStack::new();
stack.set_hdr(true);

// DOF Pass
let mut dof = DOFPass::new();
dof.set_focus_distance(10.0);      // 10米对焦
dof.set_focal_length(50.0);       // 50mm 焦距
dof.set_aperture(2.8);            // f/2.8 大光圈
dof.set_max_blur(20.0);           // 20像素最大模糊
stack.add_pass(Box::new(dof));

// ToneMapping
stack.add_pass(Box::new(ToneMappingPass::new(ToneMappingMode::Filmic)));

// 场景：多个物体在不同距离
let near_cube = MeshBuilder::cube().at(Vec3::new(0.0, 0.0, 3.0)).build();
let focus_sphere = MeshBuilder::sphere().at(Vec3::new(0.0, 0.0, 10.0)).build();
let far_pyramid = MeshBuilder::pyramid().at(Vec3::new(0.0, 0.0, 20.0)).build();
```

**UI 控制**：
- Focus Distance：1.0 ~ 50.0 米
- Aperture（f-stop）：1.0 ~ 16.0
- Bokeh Shape：Hexagon / Disk / Polygon

---

### 2.4 postfx_ssao

**目标**：屏幕空间环境光遮蔽强度对比

**场景设置**：
```rust
let mut stack = PostProcessStack::new();
stack.set_hdr(true);

// SSAO Pass
let mut ssao = SSAOPass::new();
ssao.set_radius(0.5);
ssao.set_bias(0.025);
ssao.set_power(2.0);
ssao.set_kernel_size(32);
stack.add_pass(Box::new(ssao));

// 可选：Blur
let mut ssao_blur = SSAOPass::new();
// blur only mode
stack.add_pass(Box::new(ssao_blur));

// ToneMapping + ColorGrading
stack.add_pass(Box::new(ToneMappingPass::new(ToneMappingMode::ACES)));
let mut color_grading = ColorGradingPass::new();
color_grading.set_saturation(0.9);  // 略微降低饱和度
stack.add_pass(Box::new(color_grading));

// Sponza 风格场景（大量角落/缝隙，AO 效果明显）
```

**UI 控制**：
- SSAO Radius：0.1 ~ 2.0
- SSAO Bias：0.001 ~ 0.1
- SSAO Power：1.0 ~ 4.0
- AO Intensity：0.0 ~ 2.0

---

### 2.5 postfx_color

**目标**：LUT 切换 + 白平衡

**场景设置**：
```rust
let mut stack = PostProcessStack::new();
stack.set_hdr(true);

// ToneMapping
stack.add_pass(Box::new(ToneMappingPass::new(ToneMappingMode::ACES)));

// ColorGrading with LUT
let mut color_grading = ColorGradingPass::new();

// 预置 LUT
let neutral_lut = Lut3D::generate_neutral(32);
let cinematic_lut = load("lut_cinematic.png");
let vintage_lut = load("lut_vintage.png");

// UI 选择 LUT
match selected_lut {
    "Neutral" => color_grading.set_lut(Some(neutral_lut)),
    "Cinematic" => color_grading.set_lut(Some(cinematic_lut)),
    "Vintage" => color_grading.set_lut(Some(vintage_lut)),
}

color_grading.set_white_balance((6500.0, 0.0));  // 6500K 日光
color_grading.set_saturation(1.1);
color_grading.set_contrast(1.05);
stack.add_pass(Box::new(color_grading));

// 场景：色彩丰富的物体（便于观察 LUT 效果）
```

**UI 控制**：
- LUT 选择：Neutral / Cinematic / Vintage / Custom
- White Balance Temperature：2000K ~ 10000K
- Saturation：0.0 ~ 2.0
- Contrast：0.5 ~ 2.0
- Hue Shift：0.0 ~ 360°
- Lift/Gamma/Gain 独立控制

---

## 三、实现检查清单

### 3.1 粒子系统检查

- [ ] `examples/particles_2d` 可 cargo run 成功
- [ ] `examples/particles_3d` 可 cargo run 成功
- [ ] `examples/particles_fire` 子发射器级联工作
- [ ] `examples/particles_smoke` turbulence 可见
- [ ] `examples/particles_rain` stretched billboard 工作
- [ ] `examples/particles_snow` 雪花飘落效果
- [ ] `examples/particles_collision` 碰撞反弹正确
- [ ] `examples/particles_gpu` 10万粒子性能达标

### 3.2 后期特效检查

- [ ] `examples/postfx_stack` 可切换各个 Pass
- [ ] `examples/postfx_bloom` HDR 发光效果明显
- [ ] `examples/postfx_dof` 散景效果正确
- [ ] `examples/postfx_ssao` AO 增加深度感
- [ ] `examples/postfx_color` LUT 正确应用

### 3.3 UI 要求

- [ ] 所有示例 UI 可切换 Pass 开关
- [ ] 所有示例 UI 可调整关键参数（intensity, threshold 等）
- [ ] UI 布局清晰，参数范围合理

---

## 四、常见问题解决

### 4.1 粒子不显示

1. 检查 emitter 是否在 playing 状态
2. 检查 material 是否正确加载
3. 检查 blend mode 是否正确（Additive 用于发光）
4. 检查 camera 是否正交/透视匹配场景

### 4.2 后期效果不明显

1. 确认 HDR pipeline 开启（PostProcessStack::set_hdr(true)）
2. Bloom 需要 emissive 物体或高亮度区域
3. DOF 需要大光圈（低 f-stop 值）
4. SSAO 需要场景有 depth 变化（角落/缝隙）

### 4.3 性能问题

1. 降低 particle count（粒子数量过多）
2. 减少 module 数量（每个 module 有开销）
3. DOF/Bloom 使用半分辨率
4. SSAO 使用降采样
