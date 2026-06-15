# Sprint 13 · 粒子系统与后期特效栈

> 阶段：阶段四 · 高阶能力与生态（第 1 个 Sprint）  
> 周期：4 周  
> 核心目标：建立通用 2D/3D 粒子管线 + 后期栈（FXAA/DOF/Bloom/SSAO/ToneMapping/ColorGrading/Vignette/ChromaticAberration/MotionBlur）  
> 验收：`examples/particles_3d` 与 `examples/postfx_stack` 可运行

---

## 一、Sprint 概览

本 Sprint 建立两个新 crate：`engine-particles` 与 `engine-postfx`。核心交付：

**engine-particles（粒子系统）**
- `ParticleSystem`：一个 entity 上绑定多个 `ParticleEmitter`，统一生命周期管理。
- `ParticleEmitter`：发射源组件，含 shape / emission / modules / material。
- `Particle` 数据结构：2D/3D 统一字段（position, velocity, rotation, size, color, age, lifetime）。
- `EmitShape`：Point / Box / Sphere / Hemisphere / Cone / Circle / Edge / Mesh / SkinnedMesh。
- 发射模式：Burst（爆发）与 Continuous（持续），支持循环 burst。
- Module 系统：InitialVelocity / VelocityOverLife / ColorOverLife / SizeOverLife / RotationOverLife / Force / Gravity / Drag / Turbulence / Noise / Orbital / Attractor / Collision / SubEmitter / Kill / LifetimeByVelocity / RotationBySpeed。
- 粒子渲染模式：SpriteBillboard / MeshBillboard / StretchedBillboard / HorizontalBillboard / VerticalBillboard。
- 粒子材质：PBR + 自定义 shader（alpha clip / additive / transmittance）。
- GPU 粒子：compute shader 路径（desktop/Vulkan/DX12，可选）。
- 粒子事件系统：collision callback / lifetime end / SubEmitter trigger。
- 粒子烘焙：将模拟结果烘焙到 sprite sheet 纹理，供 2D 使用。
- `examples/particles_2d` / `particles_3d` / `particles_fire` / `particles_smoke` / `particles_rain` / `particles_snow` / `particles_collision` / `particles_gpu`。

**engine-postfx（后期特效栈）**
- `PostProcessStack` 组件：按顺序执行一系列 `PostProcessPass`。
- `IPostProcessPass` 抽象：统一的 render graph 插入点与资源声明。
- FXAA（快速近似反锯齿）。
- TAA（临时抗锯齿，基础实现）。
- Bloom：HDR 亮度提取 → 多尺度高斯模糊 → 合成。
- DepthOfField：散景模糊（bokeh）+ 近/远对焦环。
- SSAO / HBAO：屏幕空间环境光遮蔽基础。
- SSR：屏幕空间反射（基础）。
- ToneMapping：ACES / Reinhard / Linear / Neutral / Filmic。
- ColorGrading：LUT / 曲线 / 白平衡 / 饱和度 / 对比度 / 色调。
- Vignette / ChromaticAberration / MotionBlur / Grain / LensDistortion。
- ScreenSpaceShadows 基础。
- `PostProcessDebugView`：显示深度 / 法线 / 运动向量 / AO。
- `examples/postfx_stack` / `postfx_bloom` / `postfx_dof` / `postfx_ssao` / `postfx_color`。

---

## 二、项目需求清单

1. `engine-particles` crate 建立。
2. `engine-postfx` crate 建立。
3. `Particle` 数据结构定义：position(Vec3), velocity(Vec3), rotation(f32/Quat), size(Vec2), color(RgbaLinear), age(f32), lifetime(f32)。
4. `Particle::is_alive(&self) -> bool`。
5. `ParticleSystem` 组件：管理一组 emitter 与全局粒子池。
6. `ParticleSystem::new() -> Self`。
7. `ParticleSystem::add_emitter(&mut self, emitter) -> EmitterHandle`。
8. `ParticleSystem::remove_emitter(&mut self, handle)`。
9. `ParticleSystem::emitters(&self) -> &[ParticleEmitter]`。
10. `ParticleSystem::particle_count(&self) -> usize`。
11. `ParticleSystem::max_particles(&self) -> usize`。
12. `ParticleSystem::set_max_particles(&mut self, n)`。
13. `ParticleSystem::play(&mut self)`。
14. `ParticleSystem::pause(&mut self)`。
15. `ParticleSystem::stop(&mut self)`。
16. `ParticleSystem::clear(&mut self)`。
17. `ParticleSystem::update(&mut self, dt)`。
18. `ParticleEmitter` 组件：单个发射源。
19. `ParticleEmitter::new(shape, emission_mode, material) -> Self`。
20. `ParticleEmitter::shape(&self) -> &EmitShape`。
21. `ParticleEmitter::set_shape(&mut self, shape)`。
22. `ParticleEmitter::rate(&self) -> f32`（每秒发射数）。
23. `ParticleEmitter::set_rate(&mut self, rate)`。
24. `ParticleEmitter::burst(&self) -> Option<BurstConfig>`。
25. `ParticleEmitter::set_burst(&mut self, burst)`。
26. `ParticleEmitter::duration(&self) -> f32`。
27. `ParticleEmitter::set_duration(&mut self, duration)`。
28. `ParticleEmitter::is_looping(&self) -> bool`。
29. `ParticleEmitter::set_looping(&mut self, b)`。
30. `ParticleEmitter::is_emitting(&self) -> bool`。
31. `ParticleEmitter::play(&mut self)`。
32. `ParticleEmitter::stop(&mut self)`。
33. `ParticleEmitter::modules(&self) -> &[ParticleModule]`。
34. `ParticleEmitter::add_module(&mut self, module)`。
35. `ParticleEmitter::render_mode(&self) -> ParticleRenderMode`。
36. `ParticleEmitter::set_render_mode(&mut self, mode)`。
37. `ParticleEmitter::material(&self) -> Handle<Material>`。
38. `ParticleEmitter::set_material(&mut self, material)`。
39. `EmitShape::Point`：从单点发射。
40. `EmitShape::Box(size)`：在长方体体积内发射。
41. `EmitShape::Sphere(radius, emit_from)`：球体体积/表面。
42. `EmitShape::Hemisphere(radius)`：半球体。
43. `EmitShape::Cone(angle, radius, length)`：圆锥体（含体/壳）。
44. `EmitShape::Circle(radius, axis)`：2D 圆形。
45. `EmitShape::Edge(start, end)`：线段。
46. `EmitShape::Mesh(mesh_handle, emit_mode)`：从 mesh 顶点/边/面发射。
47. `EmitShape::SkinnedMesh(skin_handle, emit_mode)`：从蒙皮 mesh 发射。
48. `EmitShape::sample_position(&self, rng) -> Vec3`。
49. `EmitShape::sample_direction(&self, rng, position) -> Vec3`。
50. `BurstConfig`：time, count, cycles, interval。
51. `EmissionMode::Continuous(rate)`。
52. `EmissionMode::Burst(bursts)`。
53. `EmissionMode::Mixed(rate, bursts)`。
54. `ParticleModule` trait：`apply(&self, particle, dt, ctx)`。
55. `InitialVelocityModule::new(min, max)`。
56. `VelocityOverLifeModule::new(curve)`。
57. `ColorOverLifeModule::new(gradient)`。
58. `SizeOverLifeModule::new(curve)`。
59. `RotationOverLifeModule::new(curve)`。
60. `ForceModule::new(force_vec)`。
61. `GravityModule::new(gravity)`（默认 `(0,-9.8,0)`）。
62. `DragModule::new(drag_coefficient)`。
63. `TurbulenceModule::new(noise_params)`。
64. `NoiseModule::new(frequency, octaves, seed)`。
65. `OrbitalModule::new(center, axis, angular_speed)`。
66. `AttractorModule::new(point, strength, falloff)`。
67. `CollisionModule::new(planes/spheres/boxes/meshes)`。
68. `SubEmitterModule::new(event, child_emitter)`。
69. `KillModule::new(condition)`：超出 AABB / age 超限 / 速度 < 阈值 时 kill。
70. `LifetimeByVelocityModule::new(min, max)`。
71. `RotationBySpeedModule::new(scale)`。
72. `ParticleRenderMode::SpriteBillboard`：始终面向相机的 2D sprite。
73. `ParticleRenderMode::MeshBillboard(mesh_handle)`：用 mesh 代替 sprite，整体朝向相机。
74. `ParticleRenderMode::StretchedBillboard`：沿速度方向拉伸，用于雨滴/光束。
75. `ParticleRenderMode::HorizontalBillboard`：绕 Y 轴对齐水平。
76. `ParticleRenderMode::VerticalBillboard`：绕相机 up 对齐。
77. 粒子材质：PBR 基础 + alpha clip / additive / transmittance。
78. `ParticleMaterial::PBR(albedo, metallic, roughness, blending)`。
79. `ParticleMaterial::Custom(shader_handle)`。
80. 粒子 blend mode：Opaque / Masked / Translucent / Additive。
81. GPU 粒子：compute shader 路径（desktop 可选 feature flag）。
82. `GpuParticleSystem`：storage buffer + compute dispatch。
83. `GpuParticleSystem::max_count(&self) -> u32`。
84. GPU 粒子：Cull/Sort/Soft particle 支持。
85. 粒子系统事件：`ParticleEvent::Born / Died / Collided / Triggered`。
86. `ParticleEventQueue::push(event)`。
87. `ParticleEventQueue::pop() -> Option<ParticleEvent>`。
88. 粒子碰撞回调：`on_collide(particle, normal, depth)`。
89. SubEmitter 触发：在 particle death / collision / time 时 spawn 子 emitter。
90. `SubEmitter::trigger_on(&self) -> SubEmitterTrigger`。
91. 粒子烘焙到 sprite sheet 纹理缓存。
92. `ParticleBaker::bake(system, frames, fps, sheet_size) -> Handle<Texture>`。
93. `SpriteSheet::frame_count(&self) -> u32`。
94. `SpriteSheet::sample(&self, time) -> (u32, u32)`（列/行索引）。
95. `examples/particles_2d`：2D 粒子（sprite billboard）。
96. `examples/particles_3d`：3D 场景中的多种发射器组合。
97. `examples/particles_fire`：火焰 + 烟雾子发射。
98. `examples/particles_smoke`：体积烟。
99. `examples/particles_rain`：锥形雨 + stretched billboard。
100. `examples/particles_snow`：球体雪 + turbulence。
101. `examples/particles_collision`：粒子与地面/球体碰撞。
102. `examples/particles_gpu`：GPU compute 10w 粒子。
103. `PostProcessStack` 组件：按顺序执行多个 Pass。
104. `PostProcessStack::new() -> Self`。
105. `PostProcessStack::add_pass(&mut self, pass)`。
106. `PostProcessStack::insert_pass(&mut self, index, pass)`。
107. `PostProcessStack::remove_pass(&mut self, index)`。
108. `PostProcessStack::passes(&self) -> &[Box<dyn IPostProcessPass>]`。
109. `PostProcessStack::enabled(&self) -> bool`。
110. `PostProcessStack::set_enabled(&mut self, b)`。
111. `IPostProcessPass` trait：`name / enabled / apply / declare_resources`。
112. `PostProcessPass::FXAA`。
113. `PostProcessPass::TAA`。
114. `PostProcessPass::Bloom`。
115. `PostProcessPass::DepthOfField`。
116. `PostProcessPass::SSAO`。
117. `PostProcessPass::HBAO`。
118. `PostProcessPass::SSR`。
119. `PostProcessPass::ToneMapping`。
120. `PostProcessPass::ColorGrading`。
121. `PostProcessPass::Vignette`。
122. `PostProcessPass::ChromaticAberration`。
123. `PostProcessPass::MotionBlur`。
124. `PostProcessPass::Grain`。
125. `PostProcessPass::LensDistortion`。
126. `PostProcessPass::ScreenSpaceShadows`。
127. `FXAAPass::new(threshold, edge_threshold)`。
128. `BloomPass::new(intensity, threshold, radius)`。
129. `BloomPass::extract_bright(src, dst)`。
130. `BloomPass::gaussian_blur(src, dst, iterations)`。
131. `BloomPass::composite(src, bloom, output)`。
132. `DOFPass::new(focus_distance, focal_length, aperture, max_blur)`。
133. `DOFPass::bokeh_shape(&self) -> BokehShape`。
134. `SSAOPass::new(radius, bias, power, kernel_size)`。
135. `SSAOPass::noise_texture(&self) -> Handle<Texture>`。
136. `SSAOPass::kernel(&self) -> &[Vec3]`。
137. `ToneMappingPass::new(mode)`。
138. `ToneMappingMode::ACES / Reinhard / Linear / Neutral / Filmic`。
139. `ColorGradingPass::new(lut, white_balance, saturation, contrast, hue)`。
140. `ColorGradingPass::global_tone_curve(&self) -> Curve<f32>`。
141. `VignettePass::new(intensity, smoothness, roundness, center)`。
142. `ChromaticAberrationPass::new(offset, strength)`。
143. `MotionBlurPass::new(sample_count, shutter, velocity_scale)`。
144. `GrainPass::new(intensity, size, seed)`。
145. `LensDistortionPass::new(k1, k2, p1, p2, k3)`。
146. `ScreenSpaceShadowsPass::new(step_count, thickness)`。
147. `PostProcessDebugView`：切换显示 Depth / Normal / MotionVector / AO。
148. `PostProcessDebugView::mode(&self) -> DebugViewMode`。
149. `examples/postfx_stack`：完整后期链可切换。
150. `examples/postfx_bloom`：HDR 场景发光。
151. `examples/postfx_dof`：前景/背景散景。
152. `examples/postfx_ssao`：AO 强度对比。
153. `examples/postfx_color`：不同 LUT 切换。
154. `cargo test -p engine-particles` 全部通过。
155. `cargo test -p engine-postfx` 全部通过。
156. `cargo clippy --workspace -- -D warnings` 通过。
157. `cargo fmt --check --workspace` 通过。
158. `cargo doc --workspace --no-deps` 成功。
159. 基准测试：10w 粒子 CPU 预算 <= 8ms/帧（release）。
160. 单测覆盖：Emitter / 各 Module / Pass 抽象。
161. CHANGELOG 记录版本 0.13.0。
162. README.md 加入「粒子系统」章节。
163. README.md 加入「后期特效栈」章节。
164. 公开 API doc comment 覆盖率 >= 95%。
165. 本 Sprint `unsafe` 块 <= 5。
166. 新增 example 工程 >= 13 个（8 粒子 + 5 后期）。
167. `examples/particles_3d` 可运行并展示多个 3D 粒子效果。
168. `examples/postfx_stack` 可运行并可切换各个 Pass。
169. 三平台 CI green（Windows/macOS/Linux）。
170. WASM 编译通过（feature `no_gpu_particles`）。
171. 粒子系统 render graph 阶段声明为 `PostTransparent`。
172. 后期栈 render graph 阶段声明为 `PostProcessing`。
173. 粒子支持 sort by depth（back-to-front 透明排序）。
174. 粒子支持 soft particle fade（与 scene depth 融合）。
175. 粒子支持 frustum culling per emitter。
176. 粒子支持 LOD（距离降级粒子数量）。
177. 后期栈 Pass 之间 ping-pong RT 管理自动回收。
178. 后期栈支持 HDR/LDR pipeline 双路径。
179. 后期栈支持单 Pass 跳过（enabled=false）。
180. `PostProcessStack::apply(&mut self, ctx)`：按顺序串联所有 Pass。

> 以上 180 条需求构成 Sprint 13 项目需求清单（第二部分）。

---

## 三、细分需求与验收

### 3.1 粒子核心数据与系统

181. `Particle::new(position, velocity, rotation, size, color, lifetime) -> Self`。
182. `Particle::position(&self) -> Vec3`。
183. `Particle::velocity(&self) -> Vec3`。
184. `Particle::rotation(&self) -> f32`（2D z-rotation）或 `Quat`（3D）。
185. `Particle::size(&self) -> Vec2`。
186. `Particle::color(&self) -> Rgba`。
187. `Particle::age(&self) -> f32`。
188. `Particle::lifetime(&self) -> f32`。
189. `Particle::normalized_age(&self) -> f32`（age / lifetime）。
190. `Particle::update(&mut self, dt)`。
191. `ParticlePool`：SoA 结构存放大量粒子数据。
192. `ParticlePool::new(max) -> Self`。
193. `ParticlePool::spawn(&mut self, particle) -> bool`。
194. `ParticlePool::kill(&mut self, index)`。
195. `ParticlePool::alive_count(&self) -> usize`。
196. `ParticlePool::dead_count(&self) -> usize`。
197. `ParticlePool::swap_remove(&mut self, index)`。
198. `ParticleSystem::pool(&self) -> &ParticlePool`。
199. `ParticleSystem::time(&self) -> f32`。
200. `ParticleSystem::is_playing(&self) -> bool`。
201. `ParticleSystem::simulation_space(&self) -> SimulationSpace`。
202. `ParticleSystem::set_simulation_space(&mut self, space)`。
203. `SimulationSpace::Local / World`。
204. `ParticleSystem::scaling_mode(&self) -> ScalingMode`。
205. `ScalingMode::Hierarchy / Local / ShapeOnly`。
206. `ParticleSystem::gravity_modifier(&self) -> f32`。
207. `ParticleSystem::set_gravity_modifier(&mut self, g)`。
208. `ParticleSystem::prewarm(&self) -> bool`：启动时先模拟几秒填充粒子。
209. `ParticleSystem::prewarm_time(&self) -> f32`。
210. `ParticleSystem::random_seed(&self) -> u64`。
211. `ParticleSystem::set_random_seed(&mut self, seed)`。
212. `ParticleSystem::delta_time_scale(&self) -> f32`（慢放 / 倍速）。
213. `ParticleSystem::set_delta_time_scale(&mut self, s)`。

### 3.2 Emitter 与发射模式

214. `ParticleEmitter::new(shape) -> Self`。
215. `ParticleEmitter::with_rate(rate) -> Self`。
216. `ParticleEmitter::with_duration(duration, looping) -> Self`。
217. `ParticleEmitter::with_max_particles(max) -> Self`。
218. `ParticleEmitter::with_burst(burst) -> Self`。
219. `ParticleEmitter::with_render_mode(mode) -> Self`。
220. `ParticleEmitter::with_material(material) -> Self`。
221. `ParticleEmitter::time(&self) -> f32`。
222. `ParticleEmitter::emitted_count(&self) -> u64`。
223. `ParticleEmitter::alive_count(&self) -> usize`。
224. `ParticleEmitter::active_particles(&self) -> &[Particle]`。
225. `ParticleEmitter::spawn(&mut self, count)`。
226. `ParticleEmitter::emit_one(&mut self, ctx) -> Option<Particle>`。
227. `ParticleEmitter::update(&mut self, dt, ctx) -> Vec<Particle>`。
228. `BurstConfig::new(time, count, cycles, interval) -> Self`。
229. `BurstConfig::should_fire(&self, current_time) -> bool`。
230. `BurstConfig::reset(&mut self)`。
231. `EmissionMode::Continuous`。
232. `EmissionMode::Burst(bursts: Vec<BurstConfig>)`。
233. `EmissionMode::Mixed`。
234. `ParticleEmitter::burst_list(&self) -> &[BurstConfig]`。
235. `ParticleEmitter::add_burst(&mut self, burst)`。
236. `ParticleEmitter::remove_burst(&mut self, index)`。
237. `ParticleEmitter::emission_rate_over_time(&self) -> Option<&Curve<f32>>`。
238. `ParticleEmitter::set_emission_rate_over_time(&mut self, curve)`。
239. `ParticleEmitter::delay(&self) -> f32`。
240. `ParticleEmitter::set_delay(&mut self, seconds)`。

### 3.3 EmitShape

241. `EmitShape::Point`。
242. `EmitShape::Box(Vec3)`。
243. `EmitShape::Sphere(f32, SphereEmitMode)`。
244. `SphereEmitMode::Volume / Shell`。
245. `EmitShape::Hemisphere(f32)`。
246. `EmitShape::Cone(angle, base_radius, length, ConeEmitMode)`。
247. `ConeEmitMode::Base / Volume / Shell`。
248. `EmitShape::Circle(f32, axis)`。
249. `EmitShape::Edge(Vec3, Vec3)`。
250. `EmitShape::Mesh(Handle<Mesh3D>, MeshEmitMode)`。
251. `MeshEmitMode::Vertex / Edge / Triangle / Volume(approx)`。
252. `EmitShape::SkinnedMesh(Handle<SkinnedMesh>, MeshEmitMode)`。
253. `EmitShape::sample(&self, rng) -> (Vec3, Vec3)`（位置 + 初始方向）。
254. `EmitShape::surface_area(&self) -> f32`（用于 mesh 概率采样）。
255. `EmitShape::aabb(&self, transform) -> Aabb`。
256. `EmitShape::scale(&self, s) -> EmitShape`。
257. `EmitShape::transform(&self, mat) -> EmitShape`。
258. `EmitShape::rotate(&self, quat) -> EmitShape`。

### 3.4 Module 系统

259. `ParticleModule::priority(&self) -> i32`（执行顺序）。
260. `ParticleModule::apply(&self, particle, dt, ctx)`。
261. `InitialVelocityModule::new(min, max) -> Self`。
262. `InitialVelocityModule::speed(&self) -> (f32, f32)`。
263. `VelocityOverLifeModule::new(curve) -> Self`。
264. `VelocityOverLifeModule::curve(&self) -> &Curve<Vec3>`。
265. `ColorOverLifeModule::new(gradient) -> Self`。
266. `ColorGradient::new(stops) -> Self`。
267. `ColorGradient::sample(&self, t) -> Rgba`。
268. `SizeOverLifeModule::new(curve) -> Self`。
269. `RotationOverLifeModule::new(curve) -> Self`。
270. `ForceModule::new(force) -> Self`。
271. `ForceModule::force(&self) -> Vec3`。
272. `GravityModule::new(g) -> Self`。
273. `GravityModule::default() -> Self`（(0,-9.8,0)）。
274. `DragModule::new(drag) -> Self`。
275. `DragModule::drag(&self) -> f32`。
276. `TurbulenceModule::new(intensity, frequency, speed) -> Self`。
277. `NoiseModule::new(frequency, octaves, seed, amplitude) -> Self`。
278. `NoiseModule::sample(&self, position, time) -> Vec3`。
279. `OrbitalModule::new(center, axis, angular_speed) -> Self`。
280. `AttractorModule::new(center, strength, falloff_radius) -> Self`。
281. `AttractorModule::attract(&self, position) -> Vec3`。
282. `CollisionModule::new(colliders) -> Self`。
283. `CollisionModule::colliders(&self) -> &[ParticleCollider]`。
284. `ParticleCollider::Plane(normal, offset)`。
285. `ParticleCollider::Sphere(center, radius)`。
286. `ParticleCollider::Box(center, half_size)`。
287. `ParticleCollider::Mesh(Handle<Mesh3D>, MeshCollisionMode)`。
288. `MeshCollisionMode::DistanceFieldApprox / TriangleTest`。
289. `CollisionModule::bounce(&self) -> f32`。
290. `CollisionModule::friction(&self) -> f32`。
291. `CollisionModule::kill_threshold(&self) -> f32`。
292. `SubEmitterModule::new(emitter_handle, trigger) -> Self`。
293. `SubEmitterTrigger::OnBirth / OnDeath / OnCollision / OnTime(f32)`。
294. `KillModule::by_outside_aabb(aabb) -> Self`。
295. `KillModule::by_min_speed(v) -> Self`。
296. `KillModule::by_max_distance(origin, d) -> Self`。
297. `LifetimeByVelocityModule::new(min, max) -> Self`。
298. `RotationBySpeedModule::new(scale) -> Self`。
299. `ModuleContext`：dt / total_time / emitter_transform / rng / event_queue。
300. `ModuleContext::emit(&mut self, event)`。

### 3.5 粒子渲染

301. `ParticleRenderMode::SpriteBillboard`。
302. `ParticleRenderMode::MeshBillboard(mesh_handle)`。
303. `ParticleRenderMode::StretchedBillboard(length_scale, speed_scale)`。
304. `ParticleRenderMode::HorizontalBillboard`。
305. `ParticleRenderMode::VerticalBillboard`。
306. `ParticleRenderer::new(renderer) -> Result<Self>`。
307. `ParticleRenderer::draw(&self, renderer, system, camera)`。
308. `ParticleRenderer::sort_mode(&self) -> SortMode`。
309. `SortMode::None / ViewDepth / YoungestInFront / OldestInFront`。
310. 粒子 shader：position + size → 屏幕空间四边形。
311. 粒子 shader：颜色纹理采样 + alpha clip。
312. `ParticleMaterial::Blending::Opaque / Masked / Translucent / Additive`。
313. 粒子 PBR shader：albedo + metallic + roughness + emissive。
314. Soft particles：基于 scene depth 边缘淡入淡出。
315. `ParticleMaterial::soft_particle_fade(&self) -> f32`。
316. GPU instancing 粒子渲染。
317. 粒子 UV 动画：flipbook 图集采样。
318. `FlipbookParams::new(columns, rows, frame_rate, blend)`。
319. 粒子深度写入开关（透明粒子默认关）。
320. 粒子接收阴影开关。
321. 粒子投射阴影开关。
322. `ParticleRenderList`：收集当前帧所有可见粒子。
323. `ParticleRenderList::push(&mut self, particle, material)`。
324. `ParticleRenderList::sort(&mut self, camera)`。
325. `ParticleRenderList::batch(&self) -> Vec<DrawBatch>`。

### 3.6 GPU 粒子（Compute Shader 路径）

326. `#[cfg(feature = "gpu_particles")]` feature gate。
327. `GpuParticleSystem::new(max_count) -> Self`。
328. `GpuParticleSystem::count(&self) -> u32`。
329. `GpuParticleSystem::storage_buffer(&self) -> BufferHandle`。
330. `GpuParticleSystem::dispatch(&self, cmd_encoder, dt)`。
331. GPU 粒子数据结构：`GpuParticle { pos, vel, rot, size, color, age, life, pad }` 128B 对齐。
332. GPU 粒子 spawn buffer：indirect count。
333. GPU 粒子 dead index stack buffer：append-consume。
334. GPU 粒子 sort：bitonic sort（可选）。
335. GPU 粒子 cull：视锥剔除 compute。
336. GPU 粒子 vs CPU fallback：feature 关闭时走 CPU 路径。
337. `GpuParticleBenchmark`：100,000 particles benchmark。
338. GPU 粒子：turbulence noise 在 shader 中实现。
339. GPU 粒子：plane/sphere collision 在 shader 中实现。
340. GPU 粒子：readback 到 CPU 用于碰撞回调（可选）。

### 3.7 粒子事件与 SubEmitter

341. `ParticleEvent::Born { index }`。
342. `ParticleEvent::Died { index, age, position }`。
343. `ParticleEvent::Collided { index, normal, depth, collider_id }`。
344. `ParticleEvent::Triggered { index, name }`。
345. `ParticleEventQueue::with_capacity(n) -> Self`。
346. `ParticleEventQueue::len(&self) -> usize`。
347. `ParticleEventQueue::drain(&mut self) -> impl Iterator<Item = ParticleEvent>`。
348. `ParticleEventSystem::update(world)`：分发事件。
349. SubEmitter 在粒子死亡时触发另一个 emitter。
350. SubEmitter 在粒子碰撞时触发。
351. SubEmitter 级联（fire → smoke → sparks）。
352. `SubEmitter::inherit_position(&self) -> bool`。
353. `SubEmitter::inherit_rotation(&self) -> bool`。
354. `SubEmitter::inherit_velocity(&self) -> f32`（0~1 传递系数）。

### 3.8 粒子烘焙与 Sprite Sheet

355. `ParticleBaker::new() -> Self`。
356. `ParticleBaker::bake(&self, system, frames, fps, sheet_size) -> Handle<Texture>`。
357. `ParticleBaker::offscreen_render_target(&self) -> RenderTarget`。
358. `SpriteSheet::new(columns, rows, fps, texture) -> Self`。
359. `SpriteSheet::uv(&self, time) -> (Vec2, Vec2)`（uv 偏移与尺寸）。
360. `SpriteSheet::frame_at(&self, frame_index) -> (u32, u32)`。
361. `SpriteSheet::total_frames(&self) -> u32`。
362. 烘焙输出 sRGB / linear 两种。
363. 烘焙输出带 alpha。
364. 烘焙可导出 PNG / KTX2。

### 3.9 后期栈 Pass 抽象

365. `IPostProcessPass::name(&self) -> &str`。
366. `IPostProcessPass::enabled(&self) -> bool`。
367. `IPostProcessPass::set_enabled(&mut self, b)`。
368. `IPostProcessPass::declare_resources(&self, graph_builder)`。
369. `IPostProcessPass::apply(&self, ctx, input, output)`。
370. `PostProcessContext`：renderer / camera / time / viewport / depth_rt / normal_rt / motion_rt。
371. `PostProcessStack::order(&self) -> Vec<usize>`（Pass 索引）。
372. `PostProcessStack::reorder(&mut self, new_order)`。
373. `PostProcessStack::apply(&mut self, ctx, scene_color, output)`。
374. 后期栈资源：RT pool 自动回收复用。
375. 后期栈 HDR pipeline 与 LDR pipeline 双路径。
376. 后期栈支持 render graph 资源声明与 pass 依赖。
377. `PostProcessPipeline::build(graph_builder, stack, scene_input) -> NodeHandle`。
378. `PostProcessPipeline::requires_depth(&self) -> bool`。
379. `PostProcessPipeline::requires_normal(&self) -> bool`。
380. `PostProcessPipeline::requires_motion(&self) -> bool`。

### 3.10 FXAA / TAA

381. `FXAAPass::new() -> Self`。
382. `FXAAPass::with_quality(quality) -> Self`。
383. `FXAAQuality::Low / Medium / High / Ultra`。
384. `FXAAPass::threshold(&self) -> f32`（默认 0.063）。
385. `FXAAPass::edge_threshold(&self) -> f32`（默认 0.0312）。
386. `FXAAPass::apply(&self, ctx, input, output)`：luma-based edge detect + subpixel AA。
387. `TAAPass::new() -> Self`。
388. `TAAPass::history_buffer(&self) -> Handle<Texture>`。
389. `TAAPass::jitter(&self) -> Vec2`（Halton 序列）。
390. `TAAPass::feedback(&self) -> f32`（默认 0.9）。
391. `TAAPass::clamp_neighborhood(&self, color_tex, position_tex)`：reprojection + clamp。
392. TAA 历史缓冲 double-buffered。

### 3.11 Bloom

393. `BloomPass::new() -> Self`。
394. `BloomPass::with_intensity(f) -> Self`。
395. `BloomPass::intensity(&self) -> f32`。
396. `BloomPass::threshold(&self) -> f32`（默认 1.0，HDR）。
397. `BloomPass::soft_knee(&self) -> f32`（默认 0.5）。
398. `BloomPass::radius(&self) -> f32`。
399. `BloomPass::mip_count(&self) -> u32`（默认 6）。
400. `BloomPass::extract(input, output, threshold, knee)`：亮度提取。
401. `BloomPass::downsample(input, output)`：2x2 盒式降采样。
402. `BloomPass::upsample(input, output)`：双线性插值升采样。
403. `BloomPass::composite(scene, bloom_chain, output, intensity)`：叠加合成。
404. `BloomPass::dirt_texture(&self) -> Option<Handle<Texture>>`（lens dirt）。
405. `BloomPass::dirt_intensity(&self) -> f32`。

### 3.12 DepthOfField

406. `DOFPass::new() -> Self`。
407. `DOFPass::focus_distance(&self) -> f32`（米）。
408. `DOFPass::focal_length(&self) -> f32`（mm）。
409. `DOFPass::aperture(&self) -> f32`（f-stop）。
410. `DOFPass::max_blur(&self) -> f32`。
411. `DOFPass::near_blur(&self, scene, depth, output)`。
412. `DOFPass::far_blur(&self, scene, depth, output)`。
413. `DOFPass::circle_of_confusion(&self, depth) -> f32`。
414. `BokehShape::Hexagon / Disk / Polygon(side_count)`。
415. `DOFPass::bokeh(&self, coc_map, output)`：散景 kernel 卷积。
416. `DOFPass::composite(scene, near, far, coc, output)`。

### 3.13 SSAO / HBAO / SSR

417. `SSAOPass::new() -> Self`。
418. `SSAOPass::radius(&self) -> f32`（默认 0.5）。
419. `SSAOPass::bias(&self) -> f32`（默认 0.025）。
420. `SSAOPass::power(&self) -> f32`（默认 2.0）。
421. `SSAOPass::kernel_size(&self) -> u32`（默认 32）。
422. `SSAOPass::generate_kernel(&self, seed) -> Vec<Vec3>`。
423. `SSAOPass::generate_noise_texture(&self) -> Handle<Texture>`（4x4 随机方向）。
424. `SSAOPass::apply(scene_color, depth, normal, output)`。
425. `SSAOPass::blur(input, output)`：双边滤波去噪。
426. `HBAOPass`：基于半球角度积分，更准确（慢）。
427. `SSAOPass::blend_with_scene(ao_map, scene, output)`。
428. `SSRPass::new() -> Self`。
429. `SSRPass::step_count(&self) -> u32`（默认 64）。
430. `SSRPass::thickness(&self) -> f32`（默认 0.5）。
431. `SSRPass::binary_search_steps(&self) -> u32`（默认 8）。
432. `SSRPass::max_distance(&self) -> f32`。
433. `SSRPass::apply(depth, normal, scene_color, output, history_opt)`。

### 3.14 ToneMapping

434. `ToneMappingPass::new(mode) -> Self`。
435. `ToneMappingMode::Linear`。
436. `ToneMappingMode::Reinhard`：L / (1 + L)。
437. `ToneMappingMode::ReinhardExtended(white)`：扩展版。
438. `ToneMappingMode::ACES`：Academy Color Encoding System。
439. `ToneMappingMode::Neutral`：中性调。
440. `ToneMappingMode::Filmic`：Uncharted 2 风格。
441. `ToneMappingPass::exposure(&self) -> f32`（默认 1.0）。
442. `ToneMappingPass::set_exposure(&mut self, e)`。
443. `ToneMappingPass::apply(hdr_input, ldr_output)`。

### 3.15 ColorGrading

444. `ColorGradingPass::new() -> Self`。
445. `ColorGradingPass::lut(&self) -> Option<Handle<Texture3D>>`（32x32x32 LUT）。
446. `ColorGradingPass::set_lut(&mut self, lut)`。
447. `ColorGradingPass::white_balance(&self) -> (f32, f32)`（温度 + 色调）。
448. `ColorGradingPass::saturation(&self) -> f32`（默认 1.0）。
449. `ColorGradingPass::contrast(&self) -> f32`（默认 1.0）。
450. `ColorGradingPass::hue_shift(&self) -> f32`（弧度）。
451. `ColorGradingPass::lift(&self) -> Vec3`（阴影色）。
452. `ColorGradingPass::gamma(&self) -> Vec3`（中灰）。
453. `ColorGradingPass::gain(&self) -> Vec3`（高光）。
454. `ColorGradingPass::tone_curve(&self) -> Option<&Curve<f32>>`。
455. `ColorGradingPass::apply(input, output)`。
456. `Lut3D::generate_neutral(size) -> Handle<Texture3D>`。

### 3.16 Vignette / CA / MotionBlur / Grain / LensDistortion

457. `VignettePass::new() -> Self`。
458. `VignettePass::intensity(&self) -> f32`（默认 0.45）。
459. `VignettePass::smoothness(&self) -> f32`（默认 0.2）。
460. `VignettePass::roundness(&self) -> f32`（默认 1.0，0 为方形）。
461. `VignettePass::center(&self) -> Vec2`（默认 (0.5, 0.5)）。
462. `VignettePass::color(&self) -> Rgba`（默认纯黑）。
463. `VignettePass::apply(input, output)`。
464. `ChromaticAberrationPass::new() -> Self`。
465. `ChromaticAberrationPass::strength(&self) -> f32`（默认 0.3）。
466. `ChromaticAberrationPass::max_offset(&self) -> f32`（默认 0.02）。
467. `ChromaticAberrationPass::apply(input, output)`：RGB 通道径向偏移。
468. `MotionBlurPass::new() -> Self`。
469. `MotionBlurPass::sample_count(&self) -> u32`（默认 12）。
470. `MotionBlurPass::shutter_angle(&self) -> f32`（默认 270°）。
471. `MotionBlurPass::velocity_scale(&self) -> f32`。
472. `MotionBlurPass::max_velocity(&self) -> f32`。
473. `MotionBlurPass::apply(input, motion_vector, output)`。
474. `MotionVectorTexture`：由前帧 VP 矩阵反推屏幕空间运动向量。
475. `GrainPass::new() -> Self`。
476. `GrainPass::intensity(&self) -> f32`（默认 0.15）。
477. `GrainPass::size(&self) -> f32`（默认 1.0）。
478. `GrainPass::luminance_contribution(&self) -> f32`（默认 0.8，暗部更明显）。
479. `GrainPass::seed(&self) -> f32`。
480. `LensDistortionPass::new() -> Self`。
481. `LensDistortionPass::k1(&self) -> f32`（径向畸变一阶）。
482. `LensDistortionPass::k2(&self) -> f32`（径向畸变二阶）。
483. `LensDistortionPass::p1(&self) -> f32`（切向一阶）。
484. `LensDistortionPass::p2(&self) -> f32`（切向二阶）。
485. `LensDistortionPass::k3(&self) -> f32`（径向三阶）。
486. `LensDistortionPass::center(&self) -> Vec2`。
487. `LensDistortionPass::apply(input, output)`：Brown–Conrady 模型。

### 3.17 ScreenSpaceShadows / DebugView

488. `ScreenSpaceShadowsPass::new() -> Self`。
489. `ScreenSpaceShadowsPass::step_count(&self) -> u32`。
490. `ScreenSpaceShadowsPass::thickness(&self) -> f32`。
491. `ScreenSpaceShadowsPass::max_distance(&self) -> f32`。
492. `ScreenSpaceShadowsPass::apply(depth, normal, light_dir, output)`。
493. `PostProcessDebugView::new(mode) -> Self`。
494. `DebugViewMode::None`。
495. `DebugViewMode::Depth`。
496. `DebugViewMode::Normal`。
497. `DebugViewMode::MotionVector`。
498. `DebugViewMode::AmbientOcclusion`。
499. `DebugViewMode::BloomOnly`。
500. `DebugViewMode::ColorGradingOnly`。
501. `PostProcessDebugView::mode(&self) -> DebugViewMode`。
502. `PostProcessDebugView::set_mode(&mut self, mode)`。
503. `PostProcessDebugView::apply(input, output, ctx)`。

### 3.18 渲染管线与资源管理

504. `engine-particles` 在 render graph 中声明 `OpaquePass` 后 `TransparentPass` 前节点。
505. `engine-postfx` 在 `ForwardLighting` 后声明 `PostProcessing` 阶段。
506. 后期 RT pool：`RenderTargetPool::acquire(size, format) -> Handle`。
507. `RenderTargetPool::release(handle)`。
508. `RenderTargetPool::on_frame_end(&mut self)`：回收上一帧资源。
509. HDR pipeline 格式：R11G11B10 / RGBA16F。
510. LDR pipeline 格式：RGBA8 sRGB。
511. 深度缓冲格式：D32S8 / D24S8。
512. 运动向量格式：RG16F。
513. 法线图格式：RGB10A2。
514. 后期 Pass 之间 ping-pong 切换纹理。
515. `PostProcessStack` 自动决定最小纹理尺寸（如 bloom 降采样 1/4）。
516. `PostProcessStack::hdr(&self) -> bool`。
517. `PostProcessStack::set_hdr(&mut self, b)`。
518. 支持半分辨率 AO 提升性能。

### 3.19 示例工程与测试

519. `examples/particles_2d`：sprite billboard 粒子 + 2D 场景。
520. `examples/particles_3d`：3D 场景多种 emitter / render mode。
521. `examples/particles_fire`：fire + smoke sub emitter。
522. `examples/particles_smoke`：volume smoke + turbulence。
523. `examples/particles_rain`：stretched billboard rain + cone emitter。
524. `examples/particles_snow`：sphere emitter + turbulence。
525. `examples/particles_collision`：粒子与 plane / sphere / box 碰撞。
526. `examples/particles_gpu`：GPU compute 10w 粒子。
527. `examples/postfx_stack`：完整后期链，可开关各个 Pass。
528. `examples/postfx_bloom`：emissive 球体 + bloom 阈值可调。
529. `examples/postfx_dof`：对焦距离 + 散景演示。
530. `examples/postfx_ssao`：Sponza 风格建筑 + AO 强度可调。
531. `examples/postfx_color`：多种 LUT 切换 + white balance。
532. 单测：`Particle::update` age/lifetime 正确。
533. 单测：`ParticlePool::spawn/kill` 数量与 alive 索引正确。
534. 单测：`EmitShape::sample` 在 Point 返回 (origin, forward)。
535. 单测：`EmitShape::Sphere` sample 长度接近 radius。
536. 单测：`EmitShape::Box` sample 在 AABB 内部。
537. 单测：`BurstConfig::should_fire` 在给定时间触发。
538. 单测：`InitialVelocityModule` 使用速度区间。
539. 单测：`GravityModule` 在 dt 后速度变化正确。
540. 单测：`DragModule` 衰减速度收敛到 0。
541. 单测：`ColorGradient::sample` 线性插值正确。
542. 单测：`ColorGradient::sample` 边界 t<0 / t>1 clamp。
543. 单测：`AttractorModule` 在 center 处无吸引（稳定点）。
544. 单测：`CollisionModule::Sphere` 反射法向量单位化。
545. 单测：`CollisionModule::Plane` 位置 + 法向 + 弹回。
546. 单测：`KillModule` kill 超龄粒子。
547. 单测：`PostProcessStack::add_pass/remove_pass/reorder`。
548. 单测：`PostProcessStack::apply` 顺序调用每个 Pass。
549. 单测：`FXAAPass` enabled=false 时 apply no-op。
550. 单测：`BloomPass::extract` threshold 为 0 时返回原样。
551. 单测：`SSAOPass::generate_kernel` 生成指定数量向量。
552. 单测：`ToneMappingMode::Reinhard` 对 L=1 返回 0.5。
553. 单测：`ColorGradingPass::white_balance` 纯灰 (0.5,0.5,0.5) 不变。
554. 单测：`VignettePass` 中心像素输出 = 输入。
555. 单测：`MotionVectorTexture::generate` 前向单位向量。
556. `cargo test -p engine-particles` 全部通过。
557. `cargo test -p engine-postfx` 全部通过。
558. `cargo clippy --workspace -- -D warnings` 通过。
559. `cargo fmt --check --workspace` 通过。
560. `cargo doc --workspace --no-deps` 成功。
561. 三平台 CI green（Windows / macOS / Linux）。
562. WASM 目标 `wasm32-unknown-unknown` 编译通过（gpu_particles feature off）。
563. 基准测试：`cargo bench -p engine-particles`。
564. 100,000 CPU 粒子一帧更新 <= 8ms（release / x86_64）。
565. 后期栈默认链 <= 6ms（1080p / integrated GPU）。
566. CHANGELOG.md 记录 0.13.0 条目：粒子系统 + 后期栈。
567. README.md 加入「粒子系统」章节。
568. README.md 加入「后期特效栈」章节。
569. 公开 API doc comment 覆盖率 >= 95%。
570. 本 Sprint `unsafe` 块 <= 5。
571. `examples/particles_3d` 可 `cargo run --example particles_3d` 运行成功。
572. `examples/postfx_stack` 可 `cargo run --example postfx_stack` 运行成功。
573. 新增 example 工程 >= 13 个。
574. 示例工程 UI 可切换各个 Pass 开关。
575. 示例工程 UI 可调整关键参数（intensity, threshold 等）。

---

## 四、验收标准

- [ ] `cargo run --example particles_2d` 展示 2D 粒子
- [ ] `cargo run --example particles_3d` 可运行并展示多种 3D 粒子效果
- [ ] `cargo run --example particles_fire` 火焰 + 烟雾 sub emitter
- [ ] `cargo run --example particles_gpu` 10w 粒子 GPU 模拟
- [ ] `cargo run --example postfx_stack` 完整后期链可开关
- [ ] `cargo run --example postfx_bloom` HDR 发光
- [ ] `cargo run --example postfx_dof` 散景演示
- [ ] `cargo run --example postfx_ssao` AO 强度可调
- [ ] `cargo run --example postfx_color` LUT 可切换
- [ ] `cargo test -p engine-particles` 全部通过
- [ ] `cargo test -p engine-postfx` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] 三平台 CI green
- [ ] WASM 编译通过
- [ ] CHANGELOG 记录 0.13.0
- [ ] README.md 加入「粒子系统」与「后期特效栈」章节

---

## 五、下一个 Sprint

Sprint 14 将引入音频系统（engine-audio）、输入系统 v2（多平台手柄 + 手势 + 输入映射）与本地化（i18n / L10n）基础。
