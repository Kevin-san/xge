# Sprint 10 · PBR 材质与着色器系统

> 阶段：阶段三 · 3D 管线升级（第 2 个 Sprint）  
> 周期：4 周  
> 核心目标：实现完整 PBR 金属/粗糙度工作流与节点式材质编辑器  
> 验收：PBR 材质球 Demo（金属/非金属/车漆/皮革等）视觉正确；材质编辑器可用

---

## 一、Sprint 概览

本 Sprint 引入 `engine-pbr` crate（feature-gated），并在 editor 中提供材质节点式编辑。核心交付：

- `PbrMaterial`：albedo / metallic / roughness / normal / ao / emissive / height 贴图 + 常量值
- `PbrShader`：Cook-Torrance BRDF + IBL（Image-Based Lighting）初版
- `ShaderGraph`：节点式编辑器，内置 PBR 主节点 + 常用 math/tex/uv 节点
- `TextureCompiler`：生成 mipmap、压缩、sRGB/线性处理
- `IBL Baker`：辐照度贴图 / 预滤波环境贴图 / BRDF LUT
- `Lightmap`：烘焙占位（后续完善）
- `examples/pbr_materials`：多个材质球 Demo
- `examples/pbr_editor`：材质节点编辑器演示

---

## 二、项目需求清单

1. `engine-pbr` crate 建立（或 `engine-render` feature `pbr`）。
2. `PbrMaterial`：albedo map + albedo color 常量（相乘）。
3. `PbrMaterial`：metallic map + metallic 常量。
4. `PbrMaterial`：roughness map + roughness 常量。
5. `PbrMaterial`：normal map + normal strength。
6. `PbrMaterial`：ao map + ao strength。
7. `PbrMaterial`：emissive map + emissive color + emissive intensity。
8. `PbrMaterial`：height map + parallax 强度。
9. `PbrMaterial`：clear coat（清漆）+ clear coat roughness。
10. `PbrMaterial`：anisotropy（各向异性）+ tangent map 可选。
11. `PbrMaterial`：sheen（织物光泽）+ sheen color。
12. `PbrMaterial`：subsurface scattering（次表面散射，简化版）。
13. `PbrMaterial`：alpha mode（opaque / mask / blend）。
14. `PbrMaterial`：alpha cutoff（mask 模式）。
15. `PbrMaterial`：two-sided / double-sided。
16. `PbrMaterial`：face culling（front/back/none）。
17. `PbrMaterial`：transparent vs opaque 的 pipeline 区别。
18. `PbrMaterial`：casts_shadow / receives_shadow。
19. `PbrMaterialFlags`：按位标记启用哪些贴图。
20. `PbrMaterial`：`default() -> Self` 与 `from_albedo(color) -> Self`。
21. `PbrMaterial`：资源句柄 + bind group 懒构建（初次 draw 时）。
22. `MaterialSystemPbr`：统一管理所有 PBR 材质。
23. `MaterialSystemPbr::load(toml_or_json) -> Handle<PbrMaterial>`。
24. `MaterialSystemPbr::save(handle, path) -> Result<()>`。
25. `MaterialSystemPbr::recompile(handle) -> Result<()>`（动态重编）。
26. `ShaderStage`：Vertex / Fragment / Compute。
27. `ShaderLanguage`：WGSL / GLSL / HLSL / MSL。
28. `ShaderCompiler`：跨语言交叉编译（通过 Naga，或直接用 WGSL）。
29. `ShaderCompiler::compile_wgsl(src) -> Result<ShaderModule>`。
30. `ShaderCompiler::compile_glsl(src, stage) -> Result<ShaderModule>`。
31. `ShaderCompiler::inspect_errors(src) -> Vec<Diagnostic>`（语法错误定位）。
32. `ShaderSource`：存储源代码与 include 路径。
33. `ShaderLibrary`：常用头文件与工具函数（common/pbr/utils/brdf）。
34. 工具函数：`pow5 / saturate / fresnel_schlick / ggx / smith_g / ndf_ggx / geometry_smith / diffuse_lambert / cook_torrance`。
35. PBR 主着色器：`pbr_lit.vert/frag`。
36. PBR 主着色器：支持 IBL（irradiance + prefilter + brdf_lut）。
37. PBR 主着色器：支持点光源（最多 N 个）、方向光、聚光灯。
38. PBR 主着色器：支持阴影（shadow map）。
39. PBR 主着色器：支持 normal mapping。
40. PBR 主着色器：支持 emissive + bloom 预备。
41. PBR 主着色器：支持 parallax occlusion mapping（POM）。
42. PBR 主着色器：预处理器宏切换 feature（PBR_IBL / PBR_SHADOW / PBR_PARALLAX / PBR_ANISOTROPY / PBR_CLEAR_COAT / PBR_SUBSURFACE）。
43. `ShaderPermutation`：按宏组合编译并缓存。
44. `ShaderKey`：哈希映射，确保相同 permutation 复用已编译 pipeline。
45. `ShaderHotReload`：监听文件变化（开发期），自动重建 pipeline。
46. `ShaderGraph`：节点式着色器编辑器结构。
47. `ShaderGraph::new() -> Self`。
48. `NodeKind`：`Input / Output / Constant / TextureSample / Math(Binary/Unary) / Color / UV / Time / NormalMap / PBRMaster / VertexData / FragmentData / If / Switch`。
49. `ShaderGraph::add_node(kind) -> NodeId`。
50. `ShaderGraph::add_edge(from, to) -> EdgeId`。
51. `ShaderGraph::compile(&self) -> Result<ShaderSource>`（代码生成）。
52. 代码生成器：按拓扑排序生成 WGSL 片段。
53. 代码生成器：对循环/分支进行简单 DAG 展开。
54. 代码生成器：对纹理采样、常量、UV、时间等内置节点生成对应代码。
55. 代码生成器：PBR Master 节点生成完整 BRDF 主函数。
56. ShaderGraph 可保存为 JSON 格式，版本化。
57. ShaderGraph 编辑器支持撤销/重做（与 Editor 协作）。
58. ShaderGraph 节点属性面板（Inspector 扩展）。
59. ShaderGraph 节点分类：Input / Constant / Math / Color / Texture / UV / Utility / Advanced。
60. `IBLBaker`：离线烘焙 irradiance / prefilter / brdf_lut。
61. `IBLBaker::bake_irradiance(env_map) -> CubeMap`。
62. `IBLBaker::bake_prefilter(env_map, levels) -> CubeMap`。
63. `IBLBaker::bake_brdf_lut(size) -> Texture2D`。
64. `IBLBaker` 支持在开发期实时从 HDR env 贴图烘焙。
65. `IBLBaker` 支持在 release 模式下预烘焙二进制缓存。
66. `EnvironmentMap`：HDR 贴图输入（.hdr / .exr 简化版）。
67. `EnvironmentMap::from_hdr(path) -> Result<Self>`。
68. `EnvironmentMap::skybox(&self) -> CubeMap`。
69. `EnvironmentMap::irradiance(&self) -> CubeMap`。
70. `EnvironmentMap::prefilter(&self) -> CubeMap`。
71. `EnvironmentMap::brdf_lut(&self) -> Texture2D`。
72. `Skybox` 渲染：使用 equirectangular 采样或 cube map 采样。
73. `SkyboxRenderer::draw(renderer, camera, env)`。
74. `SkyboxRenderer`：关闭深度写入，绘制在远平面。
75. `TextureCompiler`：自动生成 mipmap。
76. `TextureCompiler`：BCn / ETC2 / ASTC 压缩（按平台选择）。
77. `TextureCompiler`：sRGB / Linear 颜色空间选择。
78. `TextureCompiler`：normal map 使用 BC5 / RG 通道。
79. `TextureCompiler`：HDR 纹理使用 RGBA16F / RGBA32F。
80. `TextureCompiler::compile(path, options) -> Result<Texture>`。
81. `TextureOptions`：`format / filter / wrap / mipmap / srgb / compression / hdr`。
82. `Tonemapper`：ACES / Reinhard / Filmic / None。
83. `Tonemapper::apply(pixel) -> pixel`（着色器端）。
84. `ColorGrading`：曝光、对比度、饱和度、色温、伽马。
85. `ColorGradingLUT`：3D LUT 颜色查找（下一阶段后期特效完善）。
86. `PbrShaderKey`：根据材质特性 + 渲染特性构建 key。
87. `PipelineCache`：持久化到磁盘（可选）以加速后续启动。
88. `PbrPipeline::new(renderer, key) -> Result<PbrPipeline>`。
89. `PbrPipeline::bind(renderer, camera, lights, env)`。
90. `PbrPipeline::draw_mesh(renderer, mesh, material, transform)`。
91. `PbrPass`：`depth_pass -> shadow_map_pass -> opaque_pass -> transparent_pass -> skybox_pass`。
92. `PbrPass::draw(renderer, scene, camera, env)`。
93. `ShadowMapPass`：方向光阴影贴图（2048x2048，可选级联）。
94. `ShadowMapPass`：点光源 cubemap shadow（后续扩展）。
95. `PCF`：百分比更近滤波（Percentage-Closer Filtering）。
96. `ShadowQuality`：Low / Medium / High / Ultra（切换分辨率与 PCF kernel）。
97. `examples/pbr_materials`：展示多种材质球。
98. `examples/pbr_materials`：至少包含 Metal / Plastic / Ceramic / Wood / Concrete / Fabric / Gold / Copper / Leather / CarPaint / Rubber / Brushed Metal。
99. `examples/pbr_materials`：支持鼠标旋转查看。
100. `examples/pbr_editor`：材质节点编辑器演示（创建 ShaderGraph -> 实时预览）。
101. `examples/pbr_ibl`：HDR 环境贴图 + IBL。
102. `examples/pbr_tonemap`：切换色调映射算子。
103. `examples/pbr_shadow`：方向光与软阴影演示。
104. `examples/pbr_normal_map`：法线贴图演示。
105. `examples/pbr_emissive`：自发光演示。
106. `examples/pbr_parallax`：视差贴图演示。
107. `examples/pbr_clear_coat`：清漆效果演示。
108. `PbrMaterial` 序列化/反序列化：JSON / TOML。
109. `PbrMaterial::to_json(&self) -> String`。
110. `PbrMaterial::from_json(json) -> Result<Self>`。
111. `PbrMaterial::save(&self, path) -> Result<()>`。
112. `PbrMaterial::load(path) -> Result<Self>`。
113. `MaterialId` / `ShaderId` / `TextureId` 统一 ID。
114. 材质面板（Inspector 扩展）：支持拖拽贴图、设置常量值。
115. 材质面板：支持切换 Alpha mode / two-sided / cast shadow。
116. 材质面板：支持关键字宏启用/禁用（如启用 clear coat）。
117. 材质面板：支持「保存」按钮保存到磁盘。
118. 材质面板：支持「预览」窗口（小材质球预览）。
119. `PreviewRenderer`：为 Inspector 中的材质预览做实时渲染。
120. `PreviewRenderer::draw_material_ball(material)`。
121. 颜色空间：纹理 sRGB 正确；着色器在线性空间计算；最终 framebuffer sRGB 输出。
122. HDR 渲染管线：渲染到 FP16 framebuffer；后期执行 tonemap + gamma correct。
123. `HdrPipeline`：hdr_render_target + tonemap_pass（下一阶段 PostFx 中完善）。
124. `RenderGraph`：Pass 级别的依赖图（后续扩展，本 Sprint 先留接口）。
125. `RenderGraph::add_pass(name, deps, fn)`。
126. `RenderGraph::compile() -> PassOrder`。
127. `RenderGraph::execute(renderer)`。
128. `RenderPassDescriptor`：颜色/深度/模板附件 + load/store ops。
129. `RenderPassEncoder`：封装 draw/dispatch。
130. `ComputePass`：用于 IBL 烘焙与后续粒子模拟。
131. `ComputeShader::new(src) -> Result<Self>`。
132. `ComputeShader::dispatch(renderer, groups_x, groups_y, groups_z)`。
133. `Buffer::read_back(renderer) -> Result<Vec<u8>>`（CPU 读取）。
134. `Buffer::write(renderer, data)`。
135. `Sampler`：线性 / 三线性 / 各向异性 / 比较模式（shadow PCF）。
136. `Sampler::comparison(CompareFunction::LessEqual)`。
137. `Sampler::anisotropic(level)`。
138. `Sampler::nearest()`。
139. `Sampler::linear()`。
140. `BindingType`：uniform buffer / storage buffer / sampler / texture / texture_storage。
141. `BindGroupLayoutBuilder`：流畅构建 bind group 布局。
142. `BindGroupBuilder`：流畅构建 bind group。
143. `BindGroupCache`：按 (layout, resources) 复用已构建 bind group。
144. `RenderError`：枚举错误类型（shader 编译错误 / pipeline 错误 / resource 错误）。
145. `RenderError::to_string(&self) -> String`。
146. `RenderError::location(&self) -> Option<LineCol>`。
147. 单元测试：`PbrMaterial` JSON 往返。
148. 单元测试：`ShaderGraph` 拓扑排序。
149. 单元测试：`ShaderGraph` 编译简单图（Constant + Output）。
150. 单元测试：`ShaderGraph` 含 PBR Master 的代码生成。
151. 单元测试：BRDF math 函数（fresnel_schlick 值范围 [0,1]）。
152. 单元测试：Cook-Torrance BRDF 在 albedo=(1,1,1) 时不过饱和。
153. 单元测试：`Tonemapper::aces` 对 HDR 值正常压缩。
154. 单元测试：`IBLBaker::bake_brdf_lut` 生成非空贴图。
155. 单元测试：`TextureCompiler` mipmap 级数正确。
156. 单元测试：`PbrPipeline::new` 构建成功。
157. 单元测试：`MaterialSystemPbr::load` 简单 JSON 加载。
158. 单元测试：`ShaderKey` 不同 feature 产生不同 hash。
159. 单元测试：`PbrMaterialFlags` 位运算正确。
160. 集成测试：`examples/pbr_materials` 运行 10 帧无错误。
161. `cargo test -p engine-pbr` 全部通过。
162. `cargo clippy --workspace -- -D warnings` 通过。
163. `cargo fmt --check --workspace` 通过。
164. `cargo doc --workspace --no-deps` 成功。
165. CI 三平台 green。
166. CHANGELOG 记录版本 0.10.0。
167. README.md 加入「PBR 材质系统」章节。
168. README.md 加入「着色器系统与 ShaderGraph」章节。
169. README.md 加入「IBL 与环境光照」章节。
170. README.md 加入「色调映射与颜色分级」章节。
171. 公开 API doc comment 覆盖率 100%。
172. 本 Sprint `unsafe` 块 <= 10（主要在 GPU 绑定层）。
173. 新增 example 工程 >= 10 个。
174. `examples/pbr_materials` 渲染结果与主流 PBR 参考图大致一致（主观验收）。
175. 性能基准：典型场景（30 个 PBR mesh）稳定 60fps（GTX 1660 级 GPU）。
176. PBR 材质 inspector 预览在编辑器中可用。

> 以上 176 条需求构成 Sprint 10 全量清单。

---

## 三、细分需求与验收

### 3.1 PBR Material

177. `PbrMaterial::default() -> Self`（全 1.0 白色 + 无贴图）。
178. `PbrMaterial::from_albedo(color) -> Self`。
179. `PbrMaterial::albedo_map(&self) -> Option<Handle<Texture>>`。
180. `PbrMaterial::set_albedo_map(&mut self, tex)`。
181. `PbrMaterial::albedo(&self) -> Color`。
182. `PbrMaterial::set_albedo(&mut self, color)`。
183. `PbrMaterial::metallic_map(&self) -> Option<Handle<Texture>>`。
184. `PbrMaterial::set_metallic_map(&mut self, tex)`。
185. `PbrMaterial::metallic(&self) -> f32`。
186. `PbrMaterial::set_metallic(&mut self, v)`。
187. `PbrMaterial::roughness_map(&self) -> Option<Handle<Texture>>`。
188. `PbrMaterial::set_roughness_map(&mut self, tex)`。
189. `PbrMaterial::roughness(&self) -> f32`。
190. `PbrMaterial::set_roughness(&mut self, v)`。
191. `PbrMaterial::normal_map(&self) -> Option<Handle<Texture>>`。
192. `PbrMaterial::set_normal_map(&mut self, tex)`。
193. `PbrMaterial::normal_strength(&self) -> f32`。
194. `PbrMaterial::set_normal_strength(&mut self, v)`。
195. `PbrMaterial::ao_map(&self) -> Option<Handle<Texture>>`。
196. `PbrMaterial::set_ao_map(&mut self, tex)`。
197. `PbrMaterial::ao_strength(&self) -> f32`。
198. `PbrMaterial::set_ao_strength(&mut self, v)`。
199. `PbrMaterial::emissive_map(&self) -> Option<Handle<Texture>>`。
200. `PbrMaterial::set_emissive_map(&mut self, tex)`。
201. `PbrMaterial::emissive(&self) -> Color`。
202. `PbrMaterial::set_emissive(&mut self, color)`。
203. `PbrMaterial::emissive_intensity(&self) -> f32`。
204. `PbrMaterial::set_emissive_intensity(&mut self, v)`。
205. `PbrMaterial::height_map(&self) -> Option<Handle<Texture>>`。
206. `PbrMaterial::set_height_map(&mut self, tex)`。
207. `PbrMaterial::parallax_strength(&self) -> f32`。
208. `PbrMaterial::set_parallax_strength(&mut self, v)`。
209. `PbrMaterial::clear_coat(&self) -> f32`。
210. `PbrMaterial::set_clear_coat(&mut self, v)`。
211. `PbrMaterial::clear_coat_roughness(&self) -> f32`。
212. `PbrMaterial::set_clear_coat_roughness(&mut self, v)`。
213. `PbrMaterial::anisotropy(&self) -> f32`。
214. `PbrMaterial::set_anisotropy(&mut self, v)`。
215. `PbrMaterial::sheen(&self) -> Color`。
216. `PbrMaterial::set_sheen(&mut self, color)`。
217. `PbrMaterial::sheen_roughness(&self) -> f32`。
218. `PbrMaterial::set_sheen_roughness(&mut self, v)`。
219. `PbrMaterial::subsurface(&self) -> f32`。
220. `PbrMaterial::set_subsurface(&mut self, v)`。
221. `PbrMaterial::alpha_mode(&self) -> AlphaMode`。
222. `PbrMaterial::set_alpha_mode(&mut self, mode)`。
223. `PbrMaterial::alpha_cutoff(&self) -> f32`。
224. `PbrMaterial::set_alpha_cutoff(&mut self, v)`。
225. `PbrMaterial::double_sided(&self) -> bool`。
226. `PbrMaterial::set_double_sided(&mut self, bool)`。
227. `PbrMaterial::casts_shadow(&self) -> bool`。
228. `PbrMaterial::set_casts_shadow(&mut self, bool)`。
229. `PbrMaterial::receives_shadow(&self) -> bool`。
230. `PbrMaterial::set_receives_shadow(&mut self, bool)`。
231. `PbrMaterial::flags(&self) -> PbrMaterialFlags`。
232. `PbrMaterial::bind_group_layout(renderer) -> BindGroupLayout`。
233. `PbrMaterial::bind_group(&self, renderer) -> BindGroup`。
234. `PbrMaterial::to_json(&self) -> String`。
235. `PbrMaterial::from_json(json) -> Result<Self>`。
236. `PbrMaterial::save(&self, path) -> Result<()>`。
237. `PbrMaterial::load(path) -> Result<Self>`。
238. `AlphaMode::Opaque / Mask / Blend`。
239. `PbrMaterialFlags` 按位：HAS_ALBEDO_MAP / HAS_NORMAL_MAP / HAS_METALLIC_MAP / HAS_ROUGHNESS_MAP / HAS_AO_MAP / HAS_EMISSIVE_MAP / HAS_HEIGHT_MAP / USE_IBL / USE_CLEAR_COAT / USE_ANISOTROPY / USE_SHEEN / USE_SUBSURFACE / USE_PARALLAX。
240. `PbrMaterialFlags::contains(self, flag) -> bool`。

### 3.2 Shader 系统

241. `ShaderModule::from_wgsl(src) -> Result<Self>`。
242. `ShaderModule::from_glsl(src, stage) -> Result<Self>`。
243. `ShaderModule::entry_points(&self) -> Vec<&str>`。
244. `ShaderModule::stage(&self) -> ShaderStage`。
245. `ShaderCompiler::new()`。
246. `ShaderCompiler::compile(&self, source, lang, stage) -> Result<ShaderModule>`。
247. `ShaderCompiler::diagnostics(&self) -> Vec<Diagnostic>`。
248. `Diagnostic::level(&self) -> Level`（Error/Warning/Info）。
249. `Diagnostic::message(&self) -> &str`。
250. `Diagnostic::span(&self) -> Option<Range<usize>>`（源码位置）。
251. `ShaderPermutation::new(base_source, &[(&str, bool)]) -> Self`。
252. `ShaderPermutation::compile(&self, compiler) -> Result<ShaderModule>`。
253. `ShaderPermutation::key(&self) -> ShaderKey`。
254. `ShaderKey::hash(&self) -> u64`。
255. `ShaderHotReload::watch(path, callback)`。
256. `ShaderHotReload::tick(&mut self)`。
257. `ShaderLibrary::include(name) -> &str`。
258. `ShaderLibrary::brdf_functions() -> &str`。
259. `ShaderLibrary::pbr_common() -> &str`。
260. `ShaderLibrary::utils() -> &str`。
261. 工具函数：`pow5(x)`。
262. 工具函数：`saturate(x)`。
263. 工具函数：`fresnel_schlick(cos_theta, f0)`。
264. 工具函数：`fresnel_schlick_roughness(cos_theta, f0, roughness)`。
265. 工具函数：`ndf_ggx(n_dot_h, roughness)`。
266. 工具函数：`geometry_schlick_ggx(n_dot_v, roughness)`。
267. 工具函数：`geometry_smith(n_dot_v, n_dot_l, roughness)`。
268. 工具函数：`diffuse_lambert()`。
269. 工具函数：`cook_torrance(n_dot_v, n_dot_l, n_dot_h, v_dot_h, roughness, f0)`。
270. 工具函数：`tangent_bitangent(normal, uv)`（TBN 构造）。
271. 工具函数：`normal_sample(normal_map, uv, tbn)`。
272. `PbrShader::main_vs() -> ShaderSource`。
273. `PbrShader::main_fs() -> ShaderSource`。
274. `PbrShader::skybox_vs() -> ShaderSource`。
275. `PbrShader::skybox_fs() -> ShaderSource`。
276. `PbrShader::shadow_vs() -> ShaderSource`。
277. `PbrShader::shadow_fs() -> ShaderSource`。
278. `PbrShader::ibl_irradiance_cs() -> ShaderSource`（compute）。
279. `PbrShader::ibl_prefilter_cs() -> ShaderSource`。
280. `PbrShader::ibl_brdf_lut_cs() -> ShaderSource`。

### 3.3 ShaderGraph

281. `ShaderGraph::new()`。
282. `ShaderGraph::name(&self) -> &str`。
283. `ShaderGraph::set_name(&mut self, name)`。
284. `ShaderGraph::nodes(&self) -> &[Node]`。
285. `ShaderGraph::edges(&self) -> &[Edge]`。
286. `ShaderGraph::add_node(&mut self, kind) -> NodeId`。
287. `ShaderGraph::remove_node(&mut self, id)`。
288. `ShaderGraph::add_edge(&mut self, from, to)`。
289. `ShaderGraph::remove_edge(&mut self, id)`。
290. `ShaderGraph::topological_order(&self) -> Result<Vec<NodeId>, CycleError>`。
291. `ShaderGraph::compile(&self) -> Result<ShaderSource>`。
292. `ShaderGraph::validate(&self) -> Result<()>`。
293. `ShaderGraph::to_json(&self) -> String`。
294. `ShaderGraph::from_json(json) -> Result<Self>`。
295. `NodeId` 类型（u32 包装）。
296. `EdgeId` 类型。
297. `NodeKind::Input(name, type)`。
298. `NodeKind::Output(name, type)`。
299. `NodeKind::Constant(value)`。
300. `NodeKind::TextureSample(texture, uv)`。
301. `NodeKind::MathBinary(op, a, b)`（Add / Sub / Mul / Div / Pow / Min / Max / Dot / Cross / Distance）。
302. `NodeKind::MathUnary(op, a)`（Negate / Abs / Sign / Sqrt / Log / Exp / Sin / Cos / Tan / Floor / Ceil / Round / Normalize / Length）。
303. `NodeKind::Color(Swizzle / Mix / ToSrgb / ToLinear / Gamma)`。
304. `NodeKind::UV(Tiling / Offset / Rotate / Pan)`。
305. `NodeKind::Time(Time / SinTime / CosTime)`。
306. `NodeKind::NormalMap(texture, uv, strength)`。
307. `NodeKind::PbrMaster(albedo, normal, metallic, roughness, ao, emissive, alpha, clear_coat, clear_coat_rough, sheen, sheen_rough, subsurface, anisotropy)`。
308. `NodeKind::VertexData(Position / Normal / UV0 / UV1 / Color / Tangent)`。
309. `NodeKind::FragmentData(NormalWS / ViewDirWS / LightDirWS / ShadowCoord)`。
310. `NodeKind::If(cond, then, else_)`。
311. `NodeKind::Switch(selector, cases)`。
312. `NodeKind::Custom(name, code)`。
313. `ShaderGraphNode::position`（编辑器位置，不影响编译）。
314. `ShaderGraphNode::comment`（节点注释）。
315. `ShaderGraphNode::color`（节点颜色）。
316. `ShaderGraph::generate_wgsl(&self) -> String`。
317. `ShaderGraph::generate_glsl(&self) -> String`。
318. `ShaderGraphEditor`（与 editor 集成）。
319. `ShaderGraphEditor::open(&mut self, graph)`。
320. `ShaderGraphEditor::close(&mut self)`。
321. `ShaderGraphEditor::select(&mut self, node_id)`。
322. `ShaderGraphEditor::draw(&mut self, ui)`。
323. `ShaderGraphEditor::preview(&mut self, renderer)` — 预览材质球。

### 3.4 IBL / Environment

324. `IBLBaker::new(renderer) -> Result<Self>`。
325. `IBLBaker::bake_irradiance(&self, env_map) -> CubeMap`。
326. `IBLBaker::bake_prefilter(&self, env_map, levels) -> CubeMap`。
327. `IBLBaker::bake_brdf_lut(&self, size) -> Texture2D`。
328. `IBLBaker::save_cache(&self, dir)`。
329. `IBLBaker::load_cache(&self, dir)`。
330. `EnvironmentMap::from_hdr(path) -> Result<Self>`。
331. `EnvironmentMap::from_equirectangular(texture) -> Result<Self>`。
332. `EnvironmentMap::skybox(&self) -> CubeMap`。
333. `EnvironmentMap::irradiance(&self) -> CubeMap`。
334. `EnvironmentMap::prefilter(&self) -> CubeMap`。
335. `EnvironmentMap::brdf_lut(&self) -> Texture2D`。
336. `EnvironmentMap::intensity(&self) -> f32`。
337. `EnvironmentMap::set_intensity(&mut self, v)`。
338. `SkyboxRenderer::new(renderer) -> Result<Self>`。
339. `SkyboxRenderer::draw(&self, renderer, camera, env)`。
340. `SkyboxRenderer::set_skybox_texture(&mut self, cube_map)`。

### 3.5 PBR Pipeline / Passes

341. `PbrPipeline::new(renderer, key) -> Result<Self>`。
342. `PbrPipeline::bind(&self, renderer, camera, lights, env)`。
343. `PbrPipeline::draw_mesh(&self, renderer, mesh, material, transform)`。
344. `PbrPass::new(renderer) -> Result<Self>`。
345. `PbrPass::draw(&self, renderer, scene, camera, lights, env)`。
346. `ShadowMapPass::new(renderer, size) -> Result<Self>`。
347. `ShadowMapPass::draw_directional(&self, renderer, light, scene)`。
348. `ShadowMapPass::draw_point(&self, renderer, light, scene)`。
349. `ShadowMapPass::texture(&self) -> &Texture`。
350. `ShadowQuality::Low(512) / Medium(1024) / High(2048) / Ultra(4096)`。
351. `CascadedShadowMap`：4 级联（后续扩展）。
352. `Tonemapper::Aces / Reinhard / Filmic / None`。
353. `Tonemapper::apply(&self, hdr_color) -> LdrColor`。
354. `ColorGrading::exposure(&self) -> f32`。
355. `ColorGrading::set_exposure(&mut self, v)`。
356. `ColorGrading::contrast(&self) -> f32`。
357. `ColorGrading::set_contrast(&mut self, v)`。
358. `ColorGrading::saturation(&self) -> f32`。
359. `ColorGrading::set_saturation(&mut self, v)`。
360. `ColorGrading::temperature(&self) -> f32`。
361. `ColorGrading::set_temperature(&mut self, v)`。
362. `HdrPipeline::new(renderer, size) -> Result<Self>`。
363. `HdrPipeline::render_target(&self) -> &Texture`。
364. `HdrPipeline::tonemap(&self, renderer, tonemapper, color_grading)`。

### 3.6 纹理 / 采样器 / 缓冲

365. `TextureCompiler::new(renderer)`。
366. `TextureCompiler::compile_file(&self, path, options) -> Result<Texture>`。
367. `TextureCompiler::compile_bytes(&self, bytes, options) -> Result<Texture>`。
368. `TextureCompiler::compile_image(&self, image, options) -> Result<Texture>`。
369. `TextureOptions::default()`。
370. `TextureOptions::with_srgb(b: bool)`。
371. `TextureOptions::with_mipmap(b: bool)`。
372. `TextureOptions::with_compression(c: Compression)`。
373. `TextureOptions::with_filter(f: FilterMode)`。
374. `TextureOptions::with_wrap(w: WrapMode)`。
375. `TextureOptions::with_hdr(b: bool)`。
376. `Compression::None / BCn / ETC2 / ASTC`。
377. `Texture::width(&self) -> u32`。
378. `Texture::height(&self) -> u32`。
379. `Texture::depth_or_layers(&self) -> u32`。
380. `Texture::mip_levels(&self) -> u32`。
381. `Texture::format(&self) -> TextureFormat`。
382. `Texture::is_srgb(&self) -> bool`。
383. `Texture::generate_mipmaps(&mut self, renderer)`。
384. `Sampler::nearest(renderer) -> Handle<Sampler>`。
385. `Sampler::linear(renderer) -> Handle<Sampler>`。
386. `Sampler::trilinear(renderer) -> Handle<Sampler>`。
387. `Sampler::anisotropic(renderer, level) -> Handle<Sampler>`。
388. `Sampler::comparison(renderer, func) -> Handle<Sampler>`。
389. `Sampler::repeat(renderer) -> Handle<Sampler>`。
390. `Buffer::new(renderer, usage, size_bytes, data_opt) -> Result<Self>`。
391. `Buffer::write(&self, renderer, offset, data)`。
392. `Buffer::read_back(&self, renderer) -> Result<Vec<u8>>`。
393. `Buffer::size(&self) -> u64`。
394. `BindGroupLayoutBuilder::new()`。
395. `BindGroupLayoutBuilder::add(binding, type)`。
396. `BindGroupLayoutBuilder::build(&self, renderer) -> BindGroupLayout`。
397. `BindGroupBuilder::new()`。
398. `BindGroupBuilder::add_buffer(binding, buffer)`。
399. `BindGroupBuilder::add_sampler(binding, sampler)`。
400. `BindGroupBuilder::add_texture(binding, texture)`。
401. `BindGroupBuilder::build(&self, renderer, layout) -> BindGroup`。
402. `BindGroupCache::get(&self, key) -> Option<BindGroup>`。
403. `BindGroupCache::insert(&mut self, key, bg)`。
404. `PipelineCache::save(&self, dir) -> Result<()>`。
405. `PipelineCache::load(&mut self, dir) -> Result<()>`。

### 3.7 错误处理与诊断

406. `RenderError` 枚举：ShaderCompile / Pipeline / Resource / Other。
407. `RenderError::shader(msg: String, line: Option<u32>) -> Self`。
408. `RenderError::to_string(&self) -> String`。
409. `RenderError::span(&self) -> Option<Range<usize>>`。
410. 当 shader 编译失败时，回退到 unlit 并在屏幕上显示错误（开发模式）。
411. 渲染调试层：`DebugMessageCallback`（GL）或 `device.on_uncaptured_error`（wgpu）。
412. 渲染验证层：启用 validation feature，在 CI 中 fail on error。
413. `gpu_debug_marker(group, name)` — 标记 GPU 工作负载，方便 RenderDoc 调试。
414. `gpu_timestamp_query` — 测量每个 pass 耗时（后续 Profiler 扩展）。
415. 日志级别：渲染内部错误使用 error，降级使用 warn。

### 3.8 示例与测试

416. `examples/pbr_materials` 能显示所有材质球。
417. `examples/pbr_materials` 支持拖动旋转相机。
418. `examples/pbr_materials` 支持切环境贴图（至少 2 个）。
419. `examples/pbr_editor` 能编辑节点、实时预览。
420. `examples/pbr_ibl` 显示 IBL 效果。
421. `examples/pbr_tonemap` 切换 tonemapper。
422. `examples/pbr_shadow` 显示阴影。
423. `examples/pbr_normal_map` 显示法线贴图。
424. `examples/pbr_emissive` 显示自发光。
425. `examples/pbr_parallax` 显示视差。
426. `examples/pbr_clear_coat` 显示清漆。
427. `examples/pbr_subsurface` 显示次表面散射。
428. `examples/pbr_anisotropy` 显示各向异性。
429. 单测 `PbrMaterial` JSON 往返。
430. 单测 `ShaderGraph` 拓扑排序。
431. 单测 `ShaderGraph` 代码生成（简单样例）。
432. 单测 `ShaderGraph` PBR master 代码生成。
433. 单测 `fresnel_schlick` 对极端输入（0/1）保持 [0,1]。
434. 单测 `cook_torrance` 对常见输入不 NaN。
435. 单测 `Tonemapper::aces` 对正输入输出有限。
436. 单测 `IBLBaker` brdf_lut 非空。
437. 单测 `TextureCompiler` mipmap 级数 = log2(max(w,h)) + 1。
438. 单测 `PbrPipeline` 构建。
439. 单测 `ShaderKey` hash。
440. 单测 `PbrMaterialFlags` 位运算。
441. 集成测试：运行 `examples/pbr_materials` 10 帧无错误。
442. `cargo test -p engine-pbr` 全部通过。
443. `cargo clippy --workspace -- -D warnings` 通过。
444. `cargo fmt --check --workspace` 通过。
445. `cargo doc --workspace --no-deps` 成功。
446. CI 三平台 green。
447. CHANGELOG 记录 0.10.0。
448. README.md 加入「PBR 材质系统」章节。
449. README.md 加入「ShaderGraph 节点着色器」章节。
450. README.md 加入「IBL 与环境光照」章节。
451. README.md 加入「色调映射与颜色分级」章节。
452. 公开 API doc comment 覆盖率 100%。
453. `unsafe` 块 <= 10。
454. 新增 example 工程 >= 10 个。
455. 材质 inspector 预览可用。

---

## 四、验收标准

- [ ] `cargo run --example pbr_materials` 展示多种 PBR 材质球
- [ ] `cargo run --example pbr_editor` 节点编辑器可用
- [ ] `cargo run --example pbr_ibl` 展示 HDR 环境光照
- [ ] `cargo run --example pbr_shadow` 方向光阴影正常
- [ ] `cargo run --example pbr_normal_map` 法线贴图效果明显
- [ ] `cargo run --example pbr_parallax` 视差效果明显
- [ ] `cargo test -p engine-pbr` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.10.0

---

## 五、下一个 Sprint

Sprint 11 将引入 3D 物理引擎（刚体/关节/角色控制器/射线检测）并与 ECS/场景系统集成。
