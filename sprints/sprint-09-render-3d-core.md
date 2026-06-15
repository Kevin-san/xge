# Sprint 09 · 3D 渲染核心（网格 / 相机 / 光照 / 变换）

> 阶段：阶段三 · 3D 管线升级（第 1 个 Sprint）  
> 周期：4 周  
> 核心目标：建立 3D 渲染基础（网格、相机、光照、材质占位、基础着色器）  
> 验收：能加载 GLTF/FBX 模型并在 3D 场景中以光照效果显示

---

## 一、Sprint 概览

本 Sprint 在 `engine-render` 中引入 3D 能力，新建 `engine-render-3d` crate。核心交付：

- `Mesh3D`：顶点/索引结构、导入 GLTF/FBX、基本图元（立方体/球体/圆柱/平面/圆锥/圆环）
- `Camera3D`：透视/正交相机、FOV、近远裁剪、屏幕/世界射线转换
- `Light3D`：方向光 / 点光源 / 聚光灯 / 环境光
- `Material3D`（初版）：基础单色 + 简单光照着色器
- `Transform3D`：位置/旋转/缩放 + 矩阵计算（基于 engine-math）
- `Scene3D`：节点树 + 包围盒 + 视锥裁剪
- `RenderPipeline3D`：深度测试 / 背面剔除 / 着色器编译
- `examples/mesh_viewer`：简单 3D 模型查看器

---

## 二、项目需求清单

1. `engine-render-3d` crate 建立（或在 engine-render 中 feature-gated `3d`）。
2. `Mesh3D` 结构体：顶点数组 + 索引数组 + primitive_topology。
3. `Vertex` 结构体：position(Vec3) + normal(Vec3) + texcoord(Vec2) + tangent(Vec3)（可选）。
4. `VertexBuffer`：upload 到 GPU / bind。
5. `IndexBuffer`：upload 到 GPU / bind（16/32 位）。
6. `Mesh3D::from_vertices(vertices, indices)`。
7. `Mesh3D::from_file(path) -> Result<Self>`（支持 GLTF/GLB，FBX 为后续扩展）。
8. `Mesh3D::aabb(&self) -> AABB` 本地包围盒。
9. `Mesh3D::bounding_sphere(&self) -> Sphere`。
10. `Mesh3D::primitives(&self) -> &[Primitive]`（多子网格）。
11. `Primitive`：顶点范围 + 材质索引。
12. `Mesh3D::upload(&mut self, renderer)` GPU 上传。
13. `Mesh3D::draw(&self, renderer, pipeline, bind_groups)`。
14. 图元生成：`Mesh3D::cube(size) -> Self`。
15. 图元生成：`Mesh3D::sphere(radius, segments, rings) -> Self`。
16. 图元生成：`Mesh3D::plane(size, segments) -> Self`。
17. 图元生成：`Mesh3D::cylinder(radius, height, segments) -> Self`。
18. 图元生成：`Mesh3D::cone(radius, height, segments) -> Self`。
19. 图元生成：`Mesh3D::torus(major_r, minor_r, major_seg, minor_seg) -> Self`。
20. 图元生成：`Mesh3D::capsule(radius, height, segments) -> Self`。
21. GLTF 加载：通过 `gltf` crate（或自研解析器）。
22. GLTF 顶点属性：POSITION / NORMAL / TEXCOORD_0 / TANGENT / COLOR_0。
23. GLTF 材质映射到 `Material3D`（初版简化）。
24. GLTF 动画：暂不解析（后续动画 Sprint 实现）。
25. GLTF 骨骼：暂不解析（后续骨骼动画）。
26. FBX 加载：先不支持，留接口。
27. OBJ 加载：简化支持（可选）。
28. Mesh 管理器：`MeshManager::load(path) -> Handle<Mesh3D>`。
29. Mesh 管理器：LRU 缓存与资源计数。
30. Mesh 管理器：reload 变化文件（开发期）。
31. `Camera3D::perspective(fovy, aspect, near, far)`。
32. `Camera3D::orthographic(left, right, bottom, top, near, far)`。
33. `Camera3D::view_matrix(&self) -> Mat4`。
34. `Camera3D::projection_matrix(&self) -> Mat4`。
35. `Camera3D::view_projection(&self) -> Mat4`。
36. `Camera3D::inverse_view(&self) -> Mat4`。
37. `Camera3D::inverse_projection(&self) -> Mat4`。
38. `Camera3D::position(&self) -> Vec3`。
39. `Camera3D::forward(&self) -> Vec3`。
40. `Camera3D::right(&self) -> Vec3`。
41. `Camera3D::up(&self) -> Vec3`。
42. `Camera3D::fovy(&self) -> f32`。
43. `Camera3D::aspect(&self) -> f32`。
44. `Camera3D::near(&self) -> f32`。
45. `Camera3D::far(&self) -> f32`。
46. `Camera3D::set_fovy(&mut self, f)`。
47. `Camera3D::set_aspect(&mut self, a)`。
48. `Camera3D::set_near(&mut self, n)`。
49. `Camera3D::set_far(&mut self, f)`。
50. `Camera3D::screen_to_world_ray(screen_pos, screen_size) -> Ray3`。
51. `Camera3D::world_to_screen(world_pos, screen_size) -> Vec2`。
52. `Camera3D::look_at(&mut self, target)`。
53. `Camera3D::look_to(&mut self, dir, up)`。
54. `Frustum`：由相机 VP 矩阵提取 6 个平面。
55. `Frustum::from_view_projection(vp) -> Self`。
56. `Frustum::contains_aabb(&self, aabb) -> bool`。
57. `Frustum::contains_sphere(&self, sphere) -> bool`。
58. `Frustum::contains_point(&self, p) -> bool`。
59. `Frustum::planes(&self) -> &[Plane; 6]`。
60. `Light3D` trait：`contribution(world_pos) -> LightSample`。
61. `DirectionalLight`：方向 + 颜色 + 强度 + 阴影开关。
62. `PointLight`：位置 + 颜色 + 强度 + 半径 + 衰减。
63. `SpotLight`：位置 + 方向 + 内/外圆锥角 + 颜色 + 强度。
64. `AmbientLight`：颜色 + 强度。
65. `HemisphereLight`：天/地颜色 + 强度。
66. `LightManager`：管理场景光源，上限（如 16 个方向光 + 64 个点光）。
67. `LightUniformBuffer`：UBO 上传到着色器。
68. 每帧自动排序光源（按距离摄像机远近）。
69. `Transform3D`：位置/旋转/缩放。
70. `Transform3D::new() -> Self`。
71. `Transform3D::from_translation(v) -> Self`。
72. `Transform3D::from_rotation(q) -> Self`。
73. `Transform3D::from_scale(v) -> Self`。
74. `Transform3D::matrix(&self) -> Mat4`。
75. `Transform3D::inverse_matrix(&self) -> Mat4`。
76. `Transform3D::translation(&self) -> Vec3`。
77. `Transform3D::rotation(&self) -> Quat`。
78. `Transform3D::scale(&self) -> Vec3`。
79. `Transform3D::set_translation(&mut self, v)`。
80. `Transform3D::set_rotation(&mut self, q)`。
81. `Transform3D::set_scale(&mut self, v)`。
82. `Transform3D::translate(&mut self, v)`。
83. `Transform3D::rotate(&mut self, q)`。
84. `Transform3D::scale_by(&mut self, v)`。
85. `Transform3D::look_at(&mut self, target, up)`。
86. `Transform3D::lerp(a, b, t)`。
87. `Transform3D::transform_point(&self, p) -> Vec3`。
88. `Transform3D::transform_vector(&self, v) -> Vec3`。
89. `Transform3D::transform_direction(&self, v) -> Vec3`。
90. `Scene3D`：节点树 + 渲染实体列表。
91. `Scene3D::new() -> Self`。
92. `Scene3D::add_node(&mut self, node) -> NodeHandle`。
93. `Scene3D::remove_node(&mut self, handle)`。
94. `Scene3D::node(&self, handle) -> &Node3D`。
95. `Scene3D::node_mut(&mut self, handle) -> &mut Node3D`。
96. `Scene3D::nodes(&self) -> &[Node3D]`。
97. `Scene3D::main_camera(&self) -> Option<Camera3D>`。
98. `Scene3D::set_main_camera(&mut self, handle)`。
99. `Scene3D::update_world_transforms(&mut self)` — 从父到子传播。
100. `Scene3D::cull(&mut self, frustum)` — 视锥裁剪。
101. `Scene3D::visible_entities(&self) -> &[RenderEntity3D]`。
102. `Node3D`：`name / parent / children / transform / mesh / material / visible`。
103. `Node3D::world_transform(&self) -> Transform3D`。
104. `Node3D::aabb(&self) -> AABB`（世界空间）。
105. `RenderEntity3D`：mesh handle + material handle + world matrix。
106. `Material3D`：基础单色 + 简单光照（Phong/Blinn-Phong 初版）。
107. `Material3D::color(&self) -> Color`。
108. `Material3D::set_color(&mut self, color)`。
109. `Material3D::shader(&self) -> ShaderHandle`。
110. `Material3D::main_texture(&self) -> Option<TextureHandle>`。
111. `Material3D::shininess(&self) -> f32`（Phong 高光）。
112. `Material3D::ambient(&self) -> Color`。
113. `MaterialManager3D::load(path) -> Handle<Material3D>`。
114. `Shader3D::default_pbr_lit() -> Handle<Shader>`（占位，后续 Sprint 替换）。
115. `Shader3D::default_unlit() -> Handle<Shader>`。
116. `Shader3D::default_skinned() -> Handle<Shader>`（占位）。
117. `RenderPipeline3D`：`init / begin_frame / end_frame`。
118. `RenderPipeline3D::draw_scene(renderer, scene)`。
119. `RenderPipeline3D::depth_test(enabled)`。
120. `RenderPipeline3D::depth_write(enabled)`。
121. `RenderPipeline3D::face_culling(enabled, winding)`。
122. `RenderPipeline3D::blend_mode(mode)`。
123. `RenderPipeline3D::clear_color(color)`。
124. `RenderPipeline3D::wireframe(enabled)`。
125. `RenderPipeline3D::msaa(samples)`。
126. `RenderPipeline3D::recompile_shaders()` 热重载。
127. 内建着色器（WGSL/GLSL 双份）：
128. - `lit.vert/frag`（Phong/Blinn-Phong + 方向光 + 点光）
129. - `unlit.vert/frag`（仅采样贴图，不光照）
130. - `skinned.vert/frag`（骨骼动画占位，后续补）
131. - `normal.vert/frag`（法线可视化）
132. - `wireframe.vert/frag`（线框）
133. - `shadow.vert/frag`（阴影贴图，下一阶段实现）
134. 渲染流程：
135. 1. 清屏（颜色 + 深度 + 模板）
136. 2. 更新 world transform
137. 3. 视锥裁剪
138. 4. 收集可见 RenderEntity3D
139. 5. 按 material / mesh 排序以减少状态切换
140. 6. 绑定 VP uniform
141. 7. 逐实体绑定 transform / material / mesh 绘制
142. 8. 绘制调试 gizmo（可选）
143. 渲染状态管理：`PipelineStateCache` 缓存已编译 pipeline。
144. 错误处理：shader 编译失败时回退到 unlit 并输出错误。
145. `RenderStats3D`：draw_calls / triangles / vertices / entities_rendered / entities_culled。
146. `RenderStats3D::reset(&mut self)` 每帧重置。
147. 调试功能：渲染 AABB 包围盒 / 渲染相机视锥 / 渲染法线。
148. 调试功能：渲染 wireframe 模式。
149. 调试功能：逐帧打印 RenderStats3D。
150. `examples/mesh_viewer`：命令行参数加载 GLTF，相机漫游。
151. `examples/mesh_viewer` 支持 WASD 飞行 + 鼠标右键旋转。
152. `examples/mesh_viewer` 支持数字键 1-6 切换：实体 / 线框 / 法线 / AABB / 光照 / 合成。
153. `examples/primitives_demo`：展示所有基本图元。
154. `examples/3d_scene_simple`：简单场景（立方体 + 平面 + 方向光）。
155. `examples/3d_lights`：多光源演示（点光 / 方向光 / 聚光）。
156. `examples/3d_frustum_cull`：视锥裁剪演示，显示 Cull Stats。
157. `examples/3d_picker`：鼠标点击选中 mesh（ray-mesh intersection）。
158. `Ray3`：origin + direction。
159. `Ray3::at(&self, t) -> Vec3`。
160. `Ray3::hit_aabb(&self, aabb) -> Option<f32>`。
161. `Ray3::hit_sphere(&self, sphere) -> Option<f32>`。
162. `Ray3::hit_triangle(&self, v0, v1, v2) -> Option<f32>`（Möller–Trumbore）。
163. `Ray3::hit_mesh(&self, mesh, transform) -> Option<HitResult>`。
164. `HitResult`：t, point, normal, uv, mesh, primitive_index。
165. `PickResult`：实体集合，可按 t 排序取最近。
166. `MeshAssetLoader3D`：编辑器侧资源加载器。
167. 编辑器场景视图支持 3D 模式（与 Sprint 07 衔接）。
168. 编辑器 Gizmo 3D：平移/旋转/缩放手柄。
169. 编辑器 3D 相机：WASD + 鼠标右键旋转 + 滚轮缩放。
170. `AABB`：`min/max`、`merge(other)`、`transform_by(mat)`。
171. `Sphere`：`center/radius`、`merge(other)`。
172. `Plane`：`normal + d`。
173. `Plane::distance(&self, p) -> f32`。
174. `Plane::normalize(&mut self)`。
175. `Frustum::intersects_aabb(&self, aabb)` 粗测试。
176. `RenderResources3D`：GPU 资源槽（MVP uniform、光源 UBO、材质 slots）。
177. `UniformBuffer`：泛型上传。
178. `BindGroup`：layout + resources。
179. 顶点布局常量：`POS_3F / NORMAL_3F / UV_2F / COLOR_4F / TANGENT_4F`。
180. `MeshBuilder3D`：流式构建 mesh。
181. `MeshBuilder3D::new() -> Self`。
182. `MeshBuilder3D::vertex(&mut self, vertex)`。
183. `MeshBuilder3D::index(&mut self, idx)`。
184. `MeshBuilder3D::triangle(&mut self, a, b, c)`。
185. `MeshBuilder3D::build(&self) -> Mesh3D`。
186. `Mesh3D::compute_normals(&mut self)` — 若缺失则自动生成。
187. `Mesh3D::compute_tangents(&mut self)` — 若缺失则自动生成。
188. `Mesh3D::invert_v(&mut self)` — 翻转 V 坐标（某些格式）。
189. `Mesh3D::recalculate_aabb(&mut self)`。
190. `Mesh3D::triangles(&self) -> usize`。
191. `Mesh3D::vertices(&self) -> usize`。
192. `TextureManager3D`：重用 engine-render 2D 纹理管理。
193. `Sampler3D`：三线性 / 各向异性 / mipmap。
194. `RenderTarget3D`：离屏渲染（后续 PBR 需要）。
195. `DepthTarget`：24/32 位深度缓冲。
196. `Framebuffer3D`：color + depth + stencil。
197. 支持 `RenderPipeline3D::render_to_texture(...)`（后续）。
198. 渲染后端抽象：`trait RenderBackend3D`，wgpu / GL 两种实现。
199. 切换后端：`RUSTENGINE_RENDERER=wgpu` 环境变量或 `EngineConfig`。
200. 错误容错：加载失败时用 `default_error_mesh()` 占位（红立方体）。
201. 单元测试：`Camera3D` 矩阵乘积正确性。
202. 单元测试：`Frustum::contains_aabb` 与 `contains_sphere`。
203. 单元测试：`Ray3::hit_aabb`、`hit_sphere`、`hit_triangle`。
204. 单元测试：`Transform3D` 矩阵与 inverse 乘积 ~= I。
205. 单元测试：`AABB::transform_by` 正确性。
206. 单元测试：`Mesh3D::cube` 三角面数量正确。
207. 单元测试：`Mesh3D::sphere` 顶点与索引数量匹配。
208. 单元测试：`MeshBuilder3D` 构建成功。
209. 单元测试：`Scene3D::update_world_transforms` 子节点世界矩阵 = 父×本地。
210. 集成测试：加载 cube.gltf 无 panic。
211. 集成测试：渲染一帧无 GPU error（通过 backend validation）。
212. `cargo test -p engine-render-3d`（或相应 feature 下）全部通过。
213. `cargo clippy --workspace -- -D warnings` 通过。
214. `cargo fmt --check --workspace` 通过。
215. `cargo doc --workspace --no-deps` 成功。
216. CI 三平台 green。
217. CHANGELOG 记录版本 0.9.0。
218. README.md 加入「3D 渲染」章节。
219. README.md 加入「加载 3D 模型」章节。
220. 公开 API doc comment 覆盖率 100%。
221. 本 Sprint `unsafe` 块 <= 8（GPU 绑定层需要少量 unsafe）。
222. 新增 example 工程 >= 7 个。
223. 每帧 RenderStats3D 可在调试面板显示。
224. 默认 RenderPipeline3D 在空场景下也能输出（清屏色）。
225. RenderStats3D 在 `examples/mesh_viewer` 中通过 `键 ~` 切换显示。

> 以上 225 条需求构成 Sprint 09 全量清单。

---

## 三、细分需求与验收

### 3.1 Mesh / Vertex / Index

226. `Vertex::new(pos, normal, uv) -> Vertex`。
227. `Vertex::position(&self) -> Vec3`。
228. `Vertex::normal(&self) -> Vec3`。
229. `Vertex::texcoord(&self) -> Vec2`。
230. `VertexLayout::POS3F_NORMAL3F_UV2F` 字节偏移。
231. `VertexBuffer::new(renderer, vertices)` — GPU 上传。
232. `VertexBuffer::bind(&self, renderer)`。
233. `VertexBuffer::size_bytes(&self) -> usize`。
234. `IndexBuffer::new(renderer, indices)`。
235. `IndexBuffer::bind(&self, renderer)`。
236. `IndexBuffer::index_count(&self) -> usize`。
237. `IndexFormat::U16 / U32`。
238. `Mesh3D::new(vertex_buffer, index_buffer, primitives)`。
239. `Mesh3D::from_vertices(vertices, indices) -> Self`。
240. `Mesh3D::from_file(path) -> Result<Self>`（GLTF）。
241. `Mesh3D::aabb(&self) -> AABB`。
242. `Mesh3D::bounding_sphere(&self) -> Sphere`。
243. `Mesh3D::primitive_count(&self) -> usize`。
244. `Mesh3D::triangles(&self) -> usize`。
245. `Mesh3D::vertices(&self) -> usize`。
246. `Mesh3D::has_normals(&self) -> bool`。
247. `Mesh3D::has_tangents(&self) -> bool`。
248. `Mesh3D::has_uv(&self) -> bool`。
249. `Mesh3D::compute_normals(&mut self)`。
250. `Mesh3D::compute_tangents(&mut self)`。
251. `Mesh3D::recalculate_aabb(&mut self)`。
252. `Mesh3D::invert_v(&mut self)`。
253. `Mesh3D::transform(&mut self, mat)` — 原地变换顶点。
254. `MeshBuilder3D::new()`。
255. `MeshBuilder3D::vertex(v)`。
256. `MeshBuilder3D::index(i)`。
257. `MeshBuilder3D::triangle(a, b, c)`。
258. `MeshBuilder3D::quad(a, b, c, d)` — 拆成两个三角。
259. `MeshBuilder3D::build() -> Mesh3D`。
260. `Mesh3D::cube(size)` 生成立方体。
261. `Mesh3D::sphere(radius, segments, rings)` 生成球体。
262. `Mesh3D::plane(size, segments)` 生成平面。
263. `Mesh3D::cylinder(radius, height, segments)` 生成圆柱。
264. `Mesh3D::cone(radius, height, segments)` 生成圆锥。
265. `Mesh3D::torus(major_r, minor_r, major_seg, minor_seg)` 生成圆环。
266. `Mesh3D::capsule(radius, height, segments)` 生成胶囊。
267. GLTF 加载：vertices 正确读取。
268. GLTF 加载：indices 正确读取。
269. GLTF 加载：normal 属性可选。
270. GLTF 加载：texcoord 属性可选。
271. GLTF 加载：tangent 属性可选。
272. GLTF 加载：多 primitive 支持。
273. GLTF 加载：材质信息提取（简化版）。
274. GLTF 加载：失败时返回错误而非 panic。
275. `MeshManager::new()`。
276. `MeshManager::load(path) -> Handle<Mesh3D>`。
277. `MeshManager::get(handle) -> Option<&Mesh3D>`。
278. `MeshManager::unload(handle)`。
279. `MeshManager::reload_changed(&mut self)`。
280. `MeshManager::len() -> usize`。

### 3.2 Camera / Frustum

281. `Camera3D::perspective(fovy, aspect, near, far)`。
282. `Camera3D::orthographic(left, right, bottom, top, near, far)`。
283. `Camera3D::view_matrix(&self) -> Mat4`。
284. `Camera3D::projection_matrix(&self) -> Mat4`。
285. `Camera3D::view_projection(&self) -> Mat4`。
286. `Camera3D::inverse_view(&self) -> Mat4`。
287. `Camera3D::inverse_projection(&self) -> Mat4`。
288. `Camera3D::inverse_view_projection(&self) -> Mat4`。
289. `Camera3D::position(&self) -> Vec3`。
290. `Camera3D::forward(&self) -> Vec3`。
291. `Camera3D::right(&self) -> Vec3`。
292. `Camera3D::up(&self) -> Vec3`。
293. `Camera3D::fovy(&self) -> f32`。
294. `Camera3D::aspect(&self) -> f32`。
295. `Camera3D::near(&self) -> f32`。
296. `Camera3D::far(&self) -> f32`。
297. `Camera3D::set_fovy(&mut self, f)`。
298. `Camera3D::set_aspect(&mut self, a)`。
299. `Camera3D::set_near(&mut self, n)`。
300. `Camera3D::set_far(&mut self, f)`。
301. `Camera3D::look_at(&mut self, target)`。
302. `Camera3D::look_to(&mut self, dir, up)`。
303. `Camera3D::screen_to_world_ray(screen_pos, screen_size) -> Ray3`。
304. `Camera3D::world_to_screen(world_pos, screen_size) -> Vec2`。
305. `Frustum::from_view_projection(vp)` 正确提取 6 平面。
306. `Frustum::planes(&self) -> &[Plane; 6]`。
307. `Frustum::contains_point(&self, p) -> bool`。
308. `Frustum::contains_aabb(&self, aabb) -> bool`。
309. `Frustum::contains_sphere(&self, sphere) -> bool`。
310. `Frustum::intersects_aabb(&self, aabb) -> bool`（粗测）。

### 3.3 Light

311. `DirectionalLight::new(dir, color, intensity)`。
312. `DirectionalLight::direction(&self) -> Vec3`。
313. `DirectionalLight::color(&self) -> Color`。
314. `DirectionalLight::intensity(&self) -> f32`。
315. `DirectionalLight::casts_shadow(&self) -> bool`。
316. `PointLight::new(pos, color, intensity, radius)`。
317. `PointLight::position(&self) -> Vec3`。
318. `PointLight::color(&self) -> Color`。
319. `PointLight::intensity(&self) -> f32`。
320. `PointLight::radius(&self) -> f32`。
321. `PointLight::attenuation(&self, distance) -> f32`。
322. `SpotLight::new(pos, dir, inner_angle, outer_angle, color, intensity)`。
323. `SpotLight::inner_angle(&self) -> f32`。
324. `SpotLight::outer_angle(&self) -> f32`。
325. `SpotLight::cone_attenuation(&self, dir_to_point) -> f32`。
326. `AmbientLight::new(color, intensity)`。
327. `HemisphereLight::new(sky, ground, intensity)`。
328. `LightManager::new()`。
329. `LightManager::add_directional(l)`。
330. `LightManager::add_point(l)`。
331. `LightManager::add_spot(l)`。
332. `LightManager::set_ambient(l)`。
333. `LightManager::lights_ubo(&self) -> &UniformBuffer`。
334. `LightManager::directional_count(&self) -> usize`。
335. `LightManager::point_count(&self) -> usize`。
336. `LightManager::spot_count(&self) -> usize`。

### 3.4 Transform3D / Scene3D

337. `Transform3D::new()`。
338. `Transform3D::from_translation(v)`。
339. `Transform3D::from_rotation(q)`。
340. `Transform3D::from_scale(v)`。
341. `Transform3D::matrix(&self) -> Mat4`。
342. `Transform3D::inverse_matrix(&self) -> Mat4`。
343. `Transform3D::translation(&self) -> Vec3`。
344. `Transform3D::rotation(&self) -> Quat`。
345. `Transform3D::scale(&self) -> Vec3`。
346. `Transform3D::set_translation(&mut self, v)`。
347. `Transform3D::set_rotation(&mut self, q)`。
348. `Transform3D::set_scale(&mut self, v)`。
349. `Transform3D::translate(&mut self, v)`。
350. `Transform3D::rotate(&mut self, q)`。
351. `Transform3D::scale_by(&mut self, v)`。
352. `Transform3D::look_at(&mut self, target, up)`。
353. `Transform3D::lerp(a, b, t)`。
354. `Transform3D::transform_point(&self, p) -> Vec3`。
355. `Transform3D::transform_vector(&self, v) -> Vec3`。
356. `Transform3D::transform_direction(&self, v) -> Vec3`。
357. `Transform3D::IDENTITY` 常量。
358. `Node3D::new() -> Self`。
359. `Node3D::with_name(name) -> Self`。
360. `Node3D::with_mesh(handle) -> Self`。
361. `Node3D::name(&self) -> &str`。
362. `Node3D::parent(&self) -> Option<NodeHandle>`。
363. `Node3D::children(&self) -> &[NodeHandle]`。
364. `Node3D::local_transform(&self) -> &Transform3D`。
365. `Node3D::world_transform(&self) -> &Transform3D`。
366. `Node3D::aabb(&self) -> AABB`。
367. `Node3D::visible(&self) -> bool`。
368. `Node3D::set_visible(&mut self, bool)`。
369. `Node3D::mesh(&self) -> Option<Handle<Mesh3D>>`。
370. `Node3D::material(&self) -> Option<Handle<Material3D>>`。
371. `Scene3D::new()`。
372. `Scene3D::add_node(&mut self, node) -> NodeHandle`。
373. `Scene3D::remove_node(&mut self, handle)`。
374. `Scene3D::node(&self, handle) -> &Node3D`。
375. `Scene3D::node_mut(&mut self, handle) -> &mut Node3D`。
376. `Scene3D::nodes(&self) -> &[Node3D]`。
377. `Scene3D::root_nodes(&self) -> Vec<NodeHandle>`。
378. `Scene3D::main_camera(&self) -> Option<&Camera3D>`。
379. `Scene3D::set_main_camera(&mut self, handle)`。
380. `Scene3D::update_world_transforms(&mut self)`。
381. `Scene3D::cull(&mut self, frustum)`。
382. `Scene3D::visible_entities(&self) -> &[RenderEntity3D]`。
383. `Scene3D::stats(&self) -> &SceneStats3D`。
384. `SceneStats3D::nodes / visible_nodes / total_triangles`。
385. `RenderEntity3D::mesh / material / world_matrix`。

### 3.5 Render Pipeline / Material / Shader

386. `RenderPipeline3D::new(renderer, config) -> Result<Self>`。
387. `RenderPipeline3D::begin_frame(&mut self, renderer)`。
388. `RenderPipeline3D::end_frame(&mut self, renderer)`。
389. `RenderPipeline3D::draw_scene(&mut self, scene, camera, lights)`。
390. `RenderPipeline3D::set_clear_color(&mut self, color)`。
391. `RenderPipeline3D::set_wireframe(&mut self, enabled)`。
392. `RenderPipeline3D::set_depth_test(&mut self, enabled)`。
393. `RenderPipeline3D::set_face_culling(&mut self, enabled, winding)`。
394. `RenderPipeline3D::set_blend_mode(&mut self, mode)`。
395. `RenderPipeline3D::recompile_shaders(&mut self)`。
396. `RenderPipeline3D::stats(&self) -> &RenderStats3D`。
397. `Material3D::from_color(color) -> Self`。
398. `Material3D::from_texture(tex) -> Self`。
399. `Material3D::color(&self) -> Color`。
400. `Material3D::set_color(&mut self, color)`。
401. `Material3D::main_texture(&self) -> Option<Handle<Texture>>`。
402. `Material3D::set_main_texture(&mut self, tex)`。
403. `Material3D::shader(&self) -> Handle<Shader>`。
404. `Material3D::shininess(&self) -> f32`。
405. `Material3D::set_shininess(&mut self, f)`。
406. `MaterialManager3D::load(path) -> Handle<Material3D>`。
407. `ShaderModule::compile(src, stage) -> Result<Handle<Shader>>`。
408. `Shader3D::default_unlit() -> Handle<Shader>`。
409. `Shader3D::default_lit() -> Handle<Shader>`。
410. `Shader3D::default_wireframe() -> Handle<Shader>`。
411. `Shader3D::default_normal() -> Handle<Shader>`。
412. `PipelineStateCache::get(&self, key) -> Option<Handle<Pipeline>>`。
413. `PipelineStateCache::insert(&mut self, key, pipeline)`。

### 3.6 Ray / Picking / Geometry

414. `Ray3::new(origin, direction)`。
415. `Ray3::at(&self, t) -> Vec3`。
416. `Ray3::hit_aabb(&self, aabb) -> Option<f32>`（slab method）。
417. `Ray3::hit_sphere(&self, sphere) -> Option<f32>`。
418. `Ray3::hit_triangle(&self, v0, v1, v2) -> Option<f32>`（Möller-Trumbore）。
419. `Ray3::hit_plane(&self, plane) -> Option<f32>`。
420. `Ray3::hit_mesh(&self, mesh, transform) -> Option<HitResult>`。
421. `HitResult::t / point / normal / uv / primitive_index`。
422. `AABB::new(min, max)`。
423. `AABB::from_points(points)`。
424. `AABB::min(&self) -> Vec3`。
425. `AABB::max(&self) -> Vec3`。
426. `AABB::center(&self) -> Vec3`。
427. `AABB::half_extents(&self) -> Vec3`。
428. `AABB::size(&self) -> Vec3`。
429. `AABB::contains_point(&self, p) -> bool`。
430. `AABB::intersects_aabb(&self, other) -> bool`。
431. `AABB::merge(&self, other) -> AABB`。
432. `AABB::transform_by(&self, mat) -> AABB`。
433. `Sphere::new(center, radius)`。
434. `Sphere::contains_point(&self, p) -> bool`。
435. `Sphere::intersects_sphere(&self, other) -> bool`。
436. `Sphere::merge(&self, other) -> Sphere`。
437. `Plane::from_normal_and_point(normal, point)`。
438. `Plane::distance(&self, p) -> f32`。
439. `Plane::normalize(&mut self)`。

### 3.7 调试 / 统计

440. `DebugRenderer3D::line(a, b, color)`。
441. `DebugRenderer3D::lines(points, color)`。
442. `DebugRenderer3D::aabb(aabb, color)`。
443. `DebugRenderer3D::sphere(sphere, color, segments)`。
444. `DebugRenderer3D::arrow(from, to, color)`。
445. `DebugRenderer3D::frustum(camera, color)`。
446. `DebugRenderer3D::axis(transform, length)`。
447. `DebugRenderer3D::flush(renderer)`。
448. `DebugRenderer3D::clear()`。
449. `RenderStats3D::draw_calls`。
450. `RenderStats3D::triangles`。
451. `RenderStats3D::vertices`。
452. `RenderStats3D::entities_rendered`。
453. `RenderStats3D::entities_culled`。
454. `RenderStats3D::point_lights`。
455. `RenderStats3D::spot_lights`。
456. `RenderStats3D::reset(&mut self)`。
457. `RenderStats3D::to_string(&self) -> String`。

### 3.8 示例与测试

458. `examples/mesh_viewer` 能加载 GLTF。
459. `examples/mesh_viewer` WASD 飞行。
460. `examples/mesh_viewer` 数字键切换渲染模式。
461. `examples/primitives_demo` 展示所有图元。
462. `examples/3d_scene_simple` 立方体+平面+方向光。
463. `examples/3d_lights` 点光/方向光/聚光。
464. `examples/3d_frustum_cull` 视锥裁剪演示。
465. `examples/3d_picker` 点击选中 mesh。
466. `examples/3d_shader_hot_reload`（与 2D 复用）。
467. 单测 `Camera3D` 矩阵。
468. 单测 `Frustum` 裁剪。
469. 单测 `Ray3` 命中。
470. 单测 `Transform3D` inverse。
471. 单测 `AABB` transform。
472. 单测 `Mesh3D` 图元生成。
473. 单测 `MeshBuilder3D` 构建。
474. 单测 `Scene3D` 世界变换。
475. 集成测试：渲染一帧无错误。
476. `cargo test -p engine-render-3d` 通过。
477. `cargo clippy --workspace -- -D warnings` 通过。
478. `cargo fmt --check --workspace` 通过。
479. `cargo doc --workspace --no-deps` 成功。
480. CI 三平台 green。
481. CHANGELOG 记录 0.9.0。
482. README.md 加入「3D 渲染」章节。
483. README.md 加入「加载 3D 模型」章节。
484. README.md 加入「相机与视锥裁剪」章节。
485. 公开 API doc comment 覆盖率 100%。
486. `unsafe` 块 <= 8。
487. 新增 example 工程 >= 7 个。

---

## 四、验收标准

- [ ] `cargo run --example mesh_viewer` 能加载 GLTF 并显示
- [ ] `cargo run --example 3d_scene_simple` 立方体 + 平面 + 方向光正确渲染
- [ ] `cargo run --example 3d_lights` 多光源有效
- [ ] `cargo run --example 3d_picker` 点击可命中实体
- [ ] `cargo run --example 3d_frustum_cull` 视锥裁剪生效，统计可见
- [ ] `cargo test -p engine-render-3d` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.9.0

---

## 五、下一个 Sprint

Sprint 10 将基于本 Sprint 的基础，引入 PBR 金属/粗糙度工作流与完整材质系统。
