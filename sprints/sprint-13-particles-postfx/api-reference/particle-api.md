# 附录：粒子与特效 API 清单

> 本文档汇总 Sprint 13 所有公开 API，按模块分类列出。

---

## 一、Particle System Core API（对应模块一）

### 1.1 Particle 数据结构

| API | 签名 | 需求 ID |
|-----|------|---------|
| `Particle::new` | `(position: Vec3, velocity: Vec3, rotation, size: Vec2, color: Rgba, lifetime: f32) -> Self` | 181 |
| `Particle::is_alive` | `(&self) -> bool` | 51 |
| `Particle::position` | `(&self) -> Vec3` | 182 |
| `Particle::velocity` | `(&self) -> Vec3` | 183 |
| `Particle::rotation` | `(&self) -> f32`（2D）/ `Quat`（3D） | 184 |
| `Particle::size` | `(&self) -> Vec2` | 185 |
| `Particle::color` | `(&self) -> Rgba` | 186 |
| `Particle::age` | `(&self) -> f32` | 187 |
| `Particle::lifetime` | `(&self) -> f32` | 188 |
| `Particle::normalized_age` | `(&self) -> f32` | 189 |
| `Particle::update` | `(&mut self, dt: f32)` | 190 |

### 1.2 ParticlePool

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticlePool::new` | `(max: usize) -> Self` | 192 |
| `ParticlePool::spawn` | `(&mut self, particle: Particle) -> bool` | 193 |
| `ParticlePool::kill` | `(&mut self, index: usize)` | 194 |
| `ParticlePool::alive_count` | `(&self) -> usize` | 195 |
| `ParticlePool::dead_count` | `(&self) -> usize` | 196 |
| `ParticlePool::swap_remove` | `(&mut self, index: usize)` | 197 |

### 1.3 ParticleSystem

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleSystem::new` | `() -> Self` | 53 |
| `ParticleSystem::add_emitter` | `(&mut self, emitter: ParticleEmitter) -> EmitterHandle` | 54 |
| `ParticleSystem::remove_emitter` | `(&mut self, handle: EmitterHandle)` | 55 |
| `ParticleSystem::emitters` | `(&self) -> &[ParticleEmitter]` | 56 |
| `ParticleSystem::particle_count` | `(&self) -> usize` | 57 |
| `ParticleSystem::max_particles` | `(&self) -> usize` | 58 |
| `ParticleSystem::set_max_particles` | `(&mut self, n: usize)` | 59 |
| `ParticleSystem::play` | `(&mut self)` | 60 |
| `ParticleSystem::pause` | `(&mut self)` | 61 |
| `ParticleSystem::stop` | `(&mut self)` | 62 |
| `ParticleSystem::clear` | `(&mut self)` | 63 |
| `ParticleSystem::update` | `(&mut self, dt: f32)` | 64 |
| `ParticleSystem::pool` | `(&self) -> &ParticlePool` | 198 |
| `ParticleSystem::time` | `(&self) -> f32` | 199 |
| `ParticleSystem::is_playing` | `(&self) -> bool` | 200 |
| `ParticleSystem::simulation_space` | `(&self) -> SimulationSpace` | 201 |
| `ParticleSystem::set_simulation_space` | `(&mut self, space: SimulationSpace)` | 202 |
| `ParticleSystem::scaling_mode` | `(&self) -> ScalingMode` | 204 |
| `ParticleSystem::gravity_modifier` | `(&self) -> f32` | 206 |
| `ParticleSystem::set_gravity_modifier` | `(&mut self, g: f32)` | 207 |
| `ParticleSystem::prewarm` | `(&self) -> bool` | 208 |
| `ParticleSystem::prewarm_time` | `(&self) -> f32` | 209 |
| `ParticleSystem::random_seed` | `(&self) -> u64` | 210 |
| `ParticleSystem::set_random_seed` | `(&mut self, seed: u64)` | 211 |
| `ParticleSystem::delta_time_scale` | `(&self) -> f32` | 212 |
| `ParticleSystem::set_delta_time_scale` | `(&mut self, s: f32)` | 213 |

### 1.4 GPU 粒子系统（Feature: gpu_particles）

| API | 签名 | 需求 ID |
|-----|------|---------|
| `GpuParticleSystem::new` | `(max_count: u32) -> Self` | 327 |
| `GpuParticleSystem::count` | `(&self) -> u32` | 328 |
| `GpuParticleSystem::max_count` | `(&self) -> u32` | 83 |
| `GpuParticleSystem::storage_buffer` | `(&self) -> BufferHandle` | 329 |
| `GpuParticleSystem::dispatch` | `(&self, cmd_encoder: &mut CommandEncoder, dt: f32)` | 330 |

---

## 二、Emitter and Forces API（对应模块二）

### 2.1 ParticleEmitter

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleEmitter::new` | `(shape: EmitShape, emission_mode: EmissionMode, material: Handle<Material>) -> Self` | 19, 214 |
| `ParticleEmitter::with_rate` | `(rate: f32) -> Self` | 215 |
| `ParticleEmitter::with_duration` | `(duration: f32, looping: bool) -> Self` | 216 |
| `ParticleEmitter::with_max_particles` | `(max: usize) -> Self` | 217 |
| `ParticleEmitter::with_burst` | `(burst: BurstConfig) -> Self` | 218 |
| `ParticleEmitter::with_render_mode` | `(mode: ParticleRenderMode) -> Self` | 219 |
| `ParticleEmitter::with_material` | `(material: Handle<Material>) -> Self` | 220 |
| `ParticleEmitter::shape` | `(&self) -> &EmitShape` | 20 |
| `ParticleEmitter::set_shape` | `(&mut self, shape: EmitShape)` | 21 |
| `ParticleEmitter::rate` | `(&self) -> f32` | 22 |
| `ParticleEmitter::set_rate` | `(&mut self, rate: f32)` | 23 |
| `ParticleEmitter::burst` | `(&self) -> Option<BurstConfig>` | 24 |
| `ParticleEmitter::set_burst` | `(&mut self, burst: Option<BurstConfig>)` | 25 |
| `ParticleEmitter::duration` | `(&self) -> f32` | 26 |
| `ParticleEmitter::set_duration` | `(&mut self, duration: f32)` | 27 |
| `ParticleEmitter::is_looping` | `(&self) -> bool` | 28 |
| `ParticleEmitter::set_looping` | `(&mut self, b: bool)` | 29 |
| `ParticleEmitter::is_emitting` | `(&self) -> bool` | 30 |
| `ParticleEmitter::play` | `(&mut self)` | 31 |
| `ParticleEmitter::stop` | `(&mut self)` | 32 |
| `ParticleEmitter::modules` | `(&self) -> &[ParticleModule]` | 33 |
| `ParticleEmitter::add_module` | `(&mut self, module: Box<dyn ParticleModule>)` | 34 |
| `ParticleEmitter::render_mode` | `(&self) -> ParticleRenderMode` | 35 |
| `ParticleEmitter::set_render_mode` | `(&mut self, mode: ParticleRenderMode)` | 36 |
| `ParticleEmitter::material` | `(&self) -> Handle<Material>` | 37 |
| `ParticleEmitter::set_material` | `(&mut self, material: Handle<Material>)` | 38 |
| `ParticleEmitter::time` | `(&self) -> f32` | 221 |
| `ParticleEmitter::emitted_count` | `(&self) -> u64` | 222 |
| `ParticleEmitter::alive_count` | `(&self) -> usize` | 223 |
| `ParticleEmitter::active_particles` | `(&self) -> &[Particle]` | 224 |
| `ParticleEmitter::spawn` | `(&mut self, count: usize)` | 225 |
| `ParticleEmitter::emit_one` | `(&mut self, ctx: &ModuleContext) -> Option<Particle>` | 226 |
| `ParticleEmitter::update` | `(&mut self, dt: f32, ctx: &ModuleContext) -> Vec<Particle>` | 227 |
| `ParticleEmitter::burst_list` | `(&self) -> &[BurstConfig]` | 234 |
| `ParticleEmitter::add_burst` | `(&mut self, burst: BurstConfig)` | 235 |
| `ParticleEmitter::remove_burst` | `(&mut self, index: usize)` | 236 |
| `ParticleEmitter::emission_rate_over_time` | `(&self) -> Option<&Curve<f32>>` | 237 |
| `ParticleEmitter::set_emission_rate_over_time` | `(&mut self, curve: Option<Curve<f32>>)` | 238 |
| `ParticleEmitter::delay` | `(&self) -> f32` | 239 |
| `ParticleEmitter::set_delay` | `(&mut self, seconds: f32)` | 240 |

### 2.2 EmitShape

| API | 签名 | 需求 ID |
|-----|------|---------|
| `EmitShape::Point` | `() -> Self` | 39, 241 |
| `EmitShape::Box` | `(size: Vec3) -> Self` | 40, 242 |
| `EmitShape::Sphere` | `(radius: f32, mode: SphereEmitMode) -> Self` | 41, 243 |
| `EmitShape::Hemisphere` | `(radius: f32) -> Self` | 42, 245 |
| `EmitShape::Cone` | `(angle, base_radius, length, mode) -> Self` | 43, 246 |
| `EmitShape::Circle` | `(radius: f32, axis) -> Self` | 44, 248 |
| `EmitShape::Edge` | `(start: Vec3, end: Vec3) -> Self` | 45, 249 |
| `EmitShape::Mesh` | `(mesh: Handle<Mesh3D>, mode: MeshEmitMode) -> Self` | 46, 250 |
| `EmitShape::SkinnedMesh` | `(skin: Handle<SkinnedMesh>, mode: MeshEmitMode) -> Self` | 47, 252 |
| `EmitShape::sample_position` | `(&self, rng: &mut dyn Rng) -> Vec3` | 48, 253 |
| `EmitShape::sample_direction` | `(&self, rng: &mut dyn Rng, position: Vec3) -> Vec3` | 49, 253 |
| `EmitShape::sample` | `(&self, rng: &mut dyn Rng) -> (Vec3, Vec3)` | 253 |
| `EmitShape::surface_area` | `(&self) -> f32` | 254 |
| `EmitShape::aabb` | `(&self, transform: &Transform) -> Aabb` | 255 |
| `EmitShape::scale` | `(&self, s: f32) -> EmitShape` | 256 |
| `EmitShape::transform` | `(&self, mat: &Mat4) -> EmitShape` | 257 |
| `EmitShape::rotate` | `(&self, quat: &Quat) -> EmitShape` | 258 |

### 2.3 EmissionMode / BurstConfig

| API | 签名 | 需求 ID |
|-----|------|---------|
| `EmissionMode::Continuous` | `(rate: f32) -> Self` | 51, 231 |
| `EmissionMode::Burst` | `(bursts: Vec<BurstConfig>) -> Self` | 52, 232 |
| `EmissionMode::Mixed` | `{ rate: f32, bursts: Vec<BurstConfig> }` | 53, 233 |
| `BurstConfig::new` | `(time: f32, count: u32, cycles: u32, interval: f32) -> Self` | 50, 228 |
| `BurstConfig::should_fire` | `(&self, current_time: f32) -> bool` | 229 |
| `BurstConfig::reset` | `(&mut self)` | 230 |

### 2.4 ParticleModule

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleModule::priority` | `(&self) -> i32` | 259 |
| `ParticleModule::apply` | `(&self, particle: &mut Particle, dt: f32, ctx: &ModuleContext)` | 260 |
| `InitialVelocityModule::new` | `(min: Vec3, max: Vec3) -> Self` | 55, 261 |
| `InitialVelocityModule::speed` | `(&self) -> (Vec3, Vec3)` | 262 |
| `VelocityOverLifeModule::new` | `(curve: Curve<Vec3>) -> Self` | 56, 263 |
| `VelocityOverLifeModule::curve` | `(&self) -> &Curve<Vec3>` | 264 |
| `ColorOverLifeModule::new` | `(gradient: ColorGradient) -> Self` | 57, 265 |
| `ColorGradient::new` | `(stops: Vec<(f32, Rgba)>) -> Self` | 266 |
| `ColorGradient::sample` | `(&self, t: f32) -> Rgba` | 267 |
| `SizeOverLifeModule::new` | `(curve: Curve<f32>) -> Self` | 58, 268 |
| `RotationOverLifeModule::new` | `(curve: Curve<f32>) -> Self` | 59, 269 |
| `ForceModule::new` | `(force: Vec3) -> Self` | 60, 270 |
| `ForceModule::force` | `(&self) -> Vec3` | 271 |
| `GravityModule::new` | `(g: Vec3) -> Self` | 61, 272 |
| `GravityModule::default` | `() -> Self` | 61, 273 |
| `DragModule::new` | `(drag: f32) -> Self` | 62, 274 |
| `DragModule::drag` | `(&self) -> f32` | 275 |
| `TurbulenceModule::new` | `(intensity, frequency, speed) -> Self` | 63, 276 |
| `NoiseModule::new` | `(frequency, octaves, seed, amplitude) -> Self` | 64, 277 |
| `NoiseModule::sample` | `(&self, position: Vec3, time: f32) -> Vec3` | 278 |
| `OrbitalModule::new` | `(center: Vec3, axis: Vec3, angular_speed: f32) -> Self` | 65, 279 |
| `AttractorModule::new` | `(center: Vec3, strength: f32, falloff_radius: f32) -> Self` | 66, 280 |
| `AttractorModule::attract` | `(&self, position: Vec3) -> Vec3` | 281 |
| `CollisionModule::new` | `(colliders: Vec<ParticleCollider>) -> Self` | 67, 282 |
| `CollisionModule::colliders` | `(&self) -> &[ParticleCollider]` | 283 |
| `CollisionModule::bounce` | `(&self) -> f32` | 289 |
| `CollisionModule::friction` | `(&self) -> f32` | 290 |
| `CollisionModule::kill_threshold` | `(&self) -> f32` | 291 |
| `SubEmitterModule::new` | `(emitter_handle: EmitterHandle, trigger: SubEmitterTrigger) -> Self` | 68, 292 |
| `SubEmitterTrigger::OnBirth` | `()` | 293 |
| `SubEmitterTrigger::OnDeath` | `()` | 293 |
| `SubEmitterTrigger::OnCollision` | `()` | 293 |
| `SubEmitterTrigger::OnTime` | `(f32)` | 293 |
| `SubEmitterModule::inherit_position` | `(&self) -> bool` | 352 |
| `SubEmitterModule::inherit_rotation` | `(&self) -> bool` | 353 |
| `SubEmitterModule::inherit_velocity` | `(&self) -> f32` | 354 |
| `KillModule::by_outside_aabb` | `(aabb: Aabb) -> Self` | 294, 359 |
| `KillModule::by_min_speed` | `(v: f32) -> Self` | 295, 360 |
| `KillModule::by_max_distance` | `(origin: Vec3, d: f32) -> Self` | 296, 361 |
| `LifetimeByVelocityModule::new` | `(min: f32, max: f32) -> Self` | 69, 297 |
| `RotationBySpeedModule::new` | `(scale: f32) -> Self` | 70, 298 |

### 2.5 CollisionModule Colliders

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleCollider::Plane` | `{ normal: Vec3, offset: f32 }` | 284 |
| `ParticleCollider::Sphere` | `{ center: Vec3, radius: f32 }` | 285 |
| `ParticleCollider::Box` | `{ center: Vec3, half_size: Vec3 }` | 286 |
| `ParticleCollider::Mesh` | `{ mesh: Handle<Mesh3D>, mode: MeshCollisionMode }` | 287 |
| `MeshCollisionMode::DistanceFieldApprox` | `()` | 288 |
| `MeshCollisionMode::TriangleTest` | `()` | 288 |

---

## 三、Post-Processing API（对应模块三）

### 3.1 PostProcessStack

| API | 签名 | 需求 ID |
|-----|------|---------|
| `PostProcessStack::new` | `() -> Self` | 104 |
| `PostProcessStack::add_pass` | `(&mut self, pass: Box<dyn IPostProcessPass>)` | 105 |
| `PostProcessStack::insert_pass` | `(&mut self, index: usize, pass: Box<dyn IPostProcessPass>)` | 106 |
| `PostProcessStack::remove_pass` | `(&mut self, index: usize)` | 107 |
| `PostProcessStack::passes` | `(&self) -> &[Box<dyn IPostProcessPass>]` | 108 |
| `PostProcessStack::order` | `(&self) -> Vec<usize>` | 371 |
| `PostProcessStack::reorder` | `(&mut self, new_order: Vec<usize>)` | 372 |
| `PostProcessStack::enabled` | `(&self) -> bool` | 109 |
| `PostProcessStack::set_enabled` | `(&mut self, b: bool)` | 110 |
| `PostProcessStack::hdr` | `(&self) -> bool` | 516 |
| `PostProcessStack::set_hdr` | `(&mut self, b: bool)` | 517 |
| `PostProcessStack::apply` | `(&mut self, ctx: PostProcessContext, scene_color: &Texture, output: &mut Texture)` | 227, 373 |

### 3.2 IPostProcessPass Trait

| API | 签名 | 需求 ID |
|-----|------|---------|
| `IPostProcessPass::name` | `(&self) -> &str` | 365 |
| `IPostProcessPass::enabled` | `(&self) -> bool` | 366 |
| `IPostProcessPass::set_enabled` | `(&mut self, b: bool)` | 367 |
| `IPostProcessPass::declare_resources` | `(&self, graph_builder: &mut RenderGraphBuilder)` | 368 |
| `IPostProcessPass::apply` | `(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture)` | 369 |

### 3.3 PostProcessPipeline

| API | 签名 | 需求 ID |
|-----|------|---------|
| `PostProcessPipeline::build` | `(graph_builder: &mut RenderGraphBuilder, stack: PostProcessStack, scene_input: NodeHandle) -> NodeHandle` | 377 |
| `PostProcessPipeline::requires_depth` | `(&self) -> bool` | 378 |
| `PostProcessPipeline::requires_normal` | `(&self) -> bool` | 379 |
| `PostProcessPipeline::requires_motion` | `(&self) -> bool` | 380 |

### 3.4 RenderTargetPool

| API | 签名 | 需求 ID |
|-----|------|---------|
| `RenderTargetPool::acquire` | `(&mut self, size: UVec2, format: TextureFormat) -> Handle<Texture>` | 506 |
| `RenderTargetPool::release` | `(&mut self, handle: Handle<Texture>)` | 507 |
| `RenderTargetPool::on_frame_end` | `(&mut self)` | 508 |

---

## 四、Bloom and DOF API（对应模块四）

### 4.1 BloomPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `BloomPass::new` | `() -> Self` | 129, 393 |
| `BloomPass::with_intensity` | `(f: f32) -> Self` | 394 |
| `BloomPass::intensity` | `(&self) -> f32` | 130, 395 |
| `BloomPass::threshold` | `(&self) -> f32` | 131, 396 |
| `BloomPass::soft_knee` | `(&self) -> f32` | 397 |
| `BloomPass::radius` | `(&self) -> f32` | 132, 398 |
| `BloomPass::mip_count` | `(&self) -> u32` | 399 |
| `BloomPass::extract_bright` | `(&self, src: &Texture, dst: &mut Texture)` | 133 |
| `BloomPass::downsample` | `(&self, src: &Texture, dst: &mut Texture)` | 134 |
| `BloomPass::upsample` | `(&self, src: &Texture, dst: &mut Texture)` | 135 |
| `BloomPass::gaussian_blur` | `(&self, src: &Texture, dst: &mut Texture, iterations: u32)` | 136 |
| `BloomPass::composite` | `(&self, scene: &Texture, bloom_chain: &[&Texture], output: &mut Texture, intensity: f32)` | 137, 403 |
| `BloomPass::dirt_texture` | `(&self) -> Option<Handle<Texture>>` | 404 |
| `BloomPass::dirt_intensity` | `(&self) -> f32` | 405 |

### 4.2 DOFPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `DOFPass::new` | `() -> Self` | 139, 406 |
| `DOFPass::focus_distance` | `(&self) -> f32` | 140, 407 |
| `DOFPass::focal_length` | `(&self) -> f32` | 408 |
| `DOFPass::aperture` | `(&self) -> f32` | 409 |
| `DOFPass::max_blur` | `(&self) -> f32` | 410 |
| `DOFPass::near_blur` | `(&self, scene: &Texture, depth: &Texture, output: &mut Texture)` | 141, 411 |
| `DOFPass::far_blur` | `(&self, scene: &Texture, depth: &Texture, output: &mut Texture)` | 142, 412 |
| `DOFPass::circle_of_confusion` | `(&self, depth: f32) -> f32` | 143, 413 |
| `DOFPass::bokeh_shape` | `(&self) -> BokehShape` | 144 |
| `DOFPass::bokeh` | `(&self, coc_map: &Texture, scene: &Texture, output: &mut Texture)` | 415 |
| `DOFPass::composite` | `(&self, scene: &Texture, near: &Texture, far: &Texture, coc: &Texture, output: &mut Texture)` | 416 |

### 4.3 BokehShape

| API | 签名 | 需求 ID |
|-----|------|---------|
| `BokehShape::Hexagon` | `()` | 145 |
| `BokehShape::Disk` | `()` | 145 |
| `BokehShape::Polygon` | `(side_count: u32)` | 145 |

---

## 五、SSAO and SSR API（对应模块五）

### 5.1 SSAOPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `SSAOPass::new` | `() -> Self` | 141, 417 |
| `SSAOPass::radius` | `(&self) -> f32` | 142, 418 |
| `SSAOPass::bias` | `(&self) -> f32` | 143, 419 |
| `SSAOPass::power` | `(&self) -> f32` | 144, 420 |
| `SSAOPass::kernel_size` | `(&self) -> u32` | 145, 421 |
| `SSAOPass::generate_kernel` | `(&self, seed: u32) -> Vec<Vec3>` | 146, 422 |
| `SSAOPass::generate_noise_texture` | `(&self) -> Handle<Texture>` | 147, 423 |
| `SSAOPass::noise_texture` | `(&self) -> Handle<Texture>` | 148 |
| `SSAOPass::kernel` | `(&self) -> &[Vec3]` | 149 |
| `SSAOPass::apply` | `(&self, scene_color: &Texture, depth: &Texture, normal: &Texture, output: &mut Texture)` | 150, 424 |
| `SSAOPass::blur` | `(&self, input: &Texture, output: &mut Texture)` | 151, 425 |
| `SSAOPass::blend_with_scene` | `(&self, ao_map: &Texture, scene: &Texture, output: &mut Texture)` | 426 |

### 5.2 SSRPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `SSRPass::new` | `() -> Self` | 428 |
| `SSRPass::step_count` | `(&self) -> u32` | 429 |
| `SSRPass::thickness` | `(&self) -> f32` | 430 |
| `SSRPass::binary_search_steps` | `(&self) -> u32` | 431 |
| `SSRPass::max_distance` | `(&self) -> f32` | 432 |
| `SSRPass::apply` | `(&self, depth: &Texture, normal: &Texture, scene_color: &Texture, output: &mut Texture, history_opt: Option<&Texture>)` | 433 |

---

## 六、ToneMapping and ColorGrading API（对应模块六）

### 6.1 ToneMappingPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ToneMappingPass::new` | `(mode: ToneMappingMode) -> Self` | 138, 434 |
| `ToneMappingPass::exposure` | `(&self) -> f32` | 139, 441 |
| `ToneMappingPass::set_exposure` | `(&mut self, e: f32)` | 140, 442 |
| `ToneMappingPass::apply` | `(&self, hdr_input: &Texture, ldr_output: &mut Texture)` | 443 |

### 6.2 ToneMappingMode

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ToneMappingMode::Linear` | `()` | 435 |
| `ToneMappingMode::Reinhard` | `()` | 436 |
| `ToneMappingMode::ReinhardExtended` | `{ white: f32 }` | 437 |
| `ToneMappingMode::ACES` | `()` | 438 |
| `ToneMappingMode::Neutral` | `()` | 439 |
| `ToneMappingMode::Filmic` | `()` | 440 |

### 6.3 ColorGradingPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ColorGradingPass::new` | `() -> Self` | 146, 444 |
| `ColorGradingPass::lut` | `(&self) -> Option<Handle<Texture3D>>` | 147, 445 |
| `ColorGradingPass::set_lut` | `(&mut self, lut: Option<Handle<Texture3D>>)` | 148, 446 |
| `ColorGradingPass::white_balance` | `(&self) -> (f32, f32)` | 149, 447 |
| `ColorGradingPass::saturation` | `(&self) -> f32` | 150, 448 |
| `ColorGradingPass::contrast` | `(&self) -> f32` | 151, 449 |
| `ColorGradingPass::hue_shift` | `(&self) -> f32` | 152, 450 |
| `ColorGradingPass::lift` | `(&self) -> Vec3` | 153, 451 |
| `ColorGradingPass::gamma` | `(&self) -> Vec3` | 154, 452 |
| `ColorGradingPass::gain` | `(&self) -> Vec3` | 155, 453 |
| `ColorGradingPass::tone_curve` | `(&self) -> Option<&Curve<f32>>` | 156, 454 |
| `ColorGradingPass::global_tone_curve` | `(&self) -> Curve<f32>` | 157 |
| `ColorGradingPass::apply` | `(&self, input: &Texture, output: &mut Texture)` | 158, 455 |
| `Lut3D::generate_neutral` | `(size: u32) -> Handle<Texture3D>` | 456 |

---

## 七、其他 PostFX Pass API

### 7.1 FXAAPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `FXAAPass::new` | `() -> Self` | 128, 381 |
| `FXAAPass::with_quality` | `(quality: FXAAQuality) -> Self` | 382 |
| `FXAAPass::with_threshold` | `(threshold: f32) -> Self` | 383 |
| `FXAAPass::threshold` | `(&self) -> f32` | 384 |
| `FXAAPass::edge_threshold` | `(&self) -> f32` | 385 |
| `FXAAPass::apply` | `(&self, ctx: &PostProcessContext, input: &Texture, output: &mut Texture)` | 386 |

### 7.2 TAAPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `TAAPass::new` | `() -> Self` | 387 |
| `TAAPass::history_buffer` | `(&self) -> Handle<Texture>` | 388 |
| `TAAPass::jitter` | `(&self) -> Vec<Vec2>` | 389 |
| `TAAPass::feedback` | `(&self) -> f32` | 390 |
| `TAAPass::clamp_neighborhood` | `(&self, color_tex: &Texture, position_tex: &Texture)` | 391 |

### 7.3 VignettePass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `VignettePass::new` | `() -> Self` | 188, 458 |
| `VignettePass::intensity` | `(&self) -> f32` | 189, 459 |
| `VignettePass::smoothness` | `(&self) -> f32` | 460 |
| `VignettePass::roundness` | `(&self) -> f32` | 461 |
| `VignettePass::center` | `(&self) -> Vec2` | 462 |
| `VignettePass::color` | `(&self) -> Rgba` | 463 |
| `VignettePass::apply` | `(&self, input: &Texture, output: &mut Texture)` | 464 |

### 7.4 ChromaticAberrationPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ChromaticAberrationPass::new` | `() -> Self` | 190, 465 |
| `ChromaticAberrationPass::strength` | `(&self) -> f32` | 466 |
| `ChromaticAberrationPass::max_offset` | `(&self) -> f32` | 467 |
| `ChromaticAberrationPass::apply` | `(&self, input: &Texture, output: &mut Texture)` | 468 |

### 7.5 MotionBlurPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `MotionBlurPass::new` | `() -> Self` | 191, 469 |
| `MotionBlurPass::sample_count` | `(&self) -> u32` | 470 |
| `MotionBlurPass::shutter_angle` | `(&self) -> f32` | 471 |
| `MotionBlurPass::velocity_scale` | `(&self) -> f32` | 472 |
| `MotionBlurPass::max_velocity` | `(&self) -> f32` | 473 |
| `MotionBlurPass::apply` | `(&self, input: &Texture, motion_vector: &Texture, output: &mut Texture)` | 474 |

### 7.6 GrainPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `GrainPass::new` | `() -> Self` | 192, 476 |
| `GrainPass::intensity` | `(&self) -> f32` | 477 |
| `GrainPass::size` | `(&self) -> f32` | 478 |
| `GrainPass::luminance_contribution` | `(&self) -> f32` | 479 |
| `GrainPass::seed` | `(&self) -> f32` | 480 |
| `GrainPass::apply` | `(&self, input: &Texture, output: &mut Texture)` | 481 |

### 7.7 LensDistortionPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `LensDistortionPass::new` | `() -> Self` | 193, 482 |
| `LensDistortionPass::k1` | `(&self) -> f32` | 483 |
| `LensDistortionPass::k2` | `(&self) -> f32` | 484 |
| `LensDistortionPass::p1` | `(&self) -> f32` | 485 |
| `LensDistortionPass::p2` | `(&self) -> f32` | 486 |
| `LensDistortionPass::k3` | `(&self) -> f32` | 487 |
| `LensDistortionPass::center` | `(&self) -> Vec2` | 488 |
| `LensDistortionPass::apply` | `(&self, input: &Texture, output: &mut Texture)` | 489 |

### 7.8 ScreenSpaceShadowsPass

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ScreenSpaceShadowsPass::new` | `() -> Self` | 194, 493 |
| `ScreenSpaceShadowsPass::step_count` | `(&self) -> u32` | 494 |
| `ScreenSpaceShadowsPass::thickness` | `(&self) -> f32` | 495 |
| `ScreenSpaceShadowsPass::max_distance` | `(&self) -> f32` | 496 |
| `ScreenSpaceShadowsPass::apply` | `(&self, depth: &Texture, normal: &Texture, light_dir: Vec3, output: &mut Texture)` | 497 |

### 7.9 PostProcessDebugView

| API | 签名 | 需求 ID |
|-----|------|---------|
| `PostProcessDebugView::new` | `(mode: DebugViewMode) -> Self` | 195, 503 |
| `PostProcessDebugView::mode` | `(&self) -> DebugViewMode` | 196, 504 |
| `PostProcessDebugView::set_mode` | `(&mut self, mode: DebugViewMode)` | 505 |
| `PostProcessDebugView::apply` | `(&self, input: &Texture, output: &mut Texture, ctx: &PostProcessContext)` | 506 |

---

## 八、粒子事件与烘焙 API

### 8.1 ParticleEvent

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleEvent::Born` | `{ index: usize }` | 341 |
| `ParticleEvent::Died` | `{ index: usize, age: f32, position: Vec3 }` | 342 |
| `ParticleEvent::Collided` | `{ index: usize, normal: Vec3, depth: f32, collider_id: u64 }` | 343 |
| `ParticleEvent::Triggered` | `{ index: usize, name: String }` | 344 |

### 8.2 ParticleEventQueue

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleEventQueue::with_capacity` | `(n: usize) -> Self` | 345 |
| `ParticleEventQueue::push` | `(&mut self, event: ParticleEvent)` | 86 |
| `ParticleEventQueue::pop` | `(&mut self) -> Option<ParticleEvent>` | 87 |
| `ParticleEventQueue::len` | `(&self) -> usize` | 346 |
| `ParticleEventQueue::drain` | `(&mut self) -> impl Iterator<Item = ParticleEvent>` | 347 |

### 8.3 ParticleBaker / SpriteSheet

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleBaker::new` | `() -> Self` | 355 |
| `ParticleBaker::bake` | `(&self, system: &ParticleSystem, frames: u32, fps: f32, sheet_size: UVec2) -> Handle<Texture>` | 139, 356 |
| `ParticleBaker::offscreen_render_target` | `(&self) -> RenderTarget` | 357 |
| `SpriteSheet::new` | `(columns: u32, rows: u32, fps: f32, texture: Handle<Texture>) -> Self` | 358 |
| `SpriteSheet::uv` | `(&self, time: f32) -> (Vec2, Vec2)` | 359 |
| `SpriteSheet::frame_at` | `(&self, frame_index: u32) -> (u32, u32)` | 360 |
| `SpriteSheet::frame_count` | `(&self) -> u32` | 94 |
| `SpriteSheet::sample` | `(&self, time: f32) -> (u32, u32)`（列/行索引） | 141 |
| `SpriteSheet::total_frames` | `(&self) -> u32` | 361 |

---

## 九、渲染模式与材质 API

### 9.1 ParticleRenderMode

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleRenderMode::SpriteBillboard` | `()` | 72, 301 |
| `ParticleRenderMode::MeshBillboard` | `{ mesh: Handle<Mesh3D> }` | 73, 302 |
| `ParticleRenderMode::StretchedBillboard` | `{ length_scale: f32, speed_scale: f32 }` | 74, 303 |
| `ParticleRenderMode::HorizontalBillboard` | `()` | 75, 304 |
| `ParticleRenderMode::VerticalBillboard` | `()` | 76, 305 |

### 9.2 ParticleMaterial / Blending

| API | 签名 | 需求 ID |
|-----|------|---------|
| `ParticleMaterial::PBR` | `{ albedo: Rgba, metallic: f32, roughness: f32, blending: Blending }` | 78, 313 |
| `ParticleMaterial::Custom` | `{ shader: Handle<Shader> }` | 79 |
| `Blending::Opaque` | `()` | 80 |
| `Blending::Masked` | `()` | 80 |
| `Blending::Translucent` | `()` | 80 |
| `Blending::Additive` | `()` | 80 |

### 9.3 SortMode / ParticleRenderList

| API | 签名 | 需求 ID |
|-----|------|---------|
| `SortMode::None` | `()` | 308 |
| `SortMode::ViewDepth` | `()` | 308 |
| `SortMode::YoungestInFront` | `()` | 308 |
| `SortMode::OldestInFront` | `()` | 308 |
| `ParticleRenderer::sort_mode` | `(&self) -> SortMode` | 308 |
| `ParticleRenderList::sort` | `(&mut self, camera: &Camera)` | 324 |
| `ParticleRenderList::batch` | `(&self) -> Vec<DrawBatch>` | 325 |
