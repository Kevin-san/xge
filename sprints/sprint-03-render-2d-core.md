# Sprint 03 · 2D 渲染核心（精灵 / 纹理 / 批处理 / 图集）

> 阶段：阶段一 · 基础内核 MVP  
> 周期：4 周  
> 核心目标：在窗口系统之上接入图形 API，完成精灵绘制 / 纹理 / 图集 / 批处理 / 正交相机  
> 验收：可在窗口中绘制彩色矩形、精灵、图集动画，并能进行合批

---

## 一、Sprint 概览

本 Sprint 建立 `engine-render crate。对外暴露统一 `Renderer` trait，默认使用 `wgpu` / `glow` 二选一。核心交付：

- `Renderer` trait 与 `RenderContext` 统一抽象
- `Texture` / `Image` / `Sampler`
- `Sprite` / `SpriteBatch`
- `TextureAtlas` / 图集打包
- `OrthographicCamera` / `View`
- `Color` / `DrawParams` / `BlendMode`
- `Shader` / `Pipeline` / `Buffer` / `BindGroup`
- `examples/sprite_draw` / `examples/atlas_animation`

---

## 二、项目需求清单

1. `engine-render crate 建立。
2. `Renderer` trait：`init(window) / begin_frame() / end_frame() / draw_sprite(...) / draw_rect(...)`。
3. `Renderer` 支持多个后端（`gl` / `wgpu` / `metal`），可通过 feature 切换。
4. 默认后端 `gl` 打开；`wgpu` 作为 feature `render-wgpu`。
5. `RenderContext` 作为全局渲染上下文（帧状态、缓存、管线）。
6. `RenderQueue` 用于排队 Draw 指令。
7. `ClearColor` 清屏色可配置。
8. `Swapchain` 抽象：支持 `present()`。
9. `Framebuffer` 抽象。
10. `Texture2D`：创建、销毁、上传像素数据、下载像素数据（可选）。
11. `Texture2D` 支持：RGBA8 / RGBA16 / R8 / BGRA8。
12. `Texture2D` 支持从 PNG / JPG / BMP / GIF 加载（通过 `image` crate）。
13. `Texture2D::width / height / size`。
14. `Texture2D::update(region, data)`。
15. `Texture2DBuilder` Fluent API：format、filter、wrap、mipmap。
16. `Sampler`：Linear / Nearest、WrapClamp / WrapRepeat / WrapMirror。
17. `SamplerBuilder`。
18. `Image`：未上传 GPU 的 CPU 端像素数据，提供 upload() 转为 Texture。
19. `Image::from_file(path) -> Result<Image>。
20. `Image::from_bytes(bytes) -> Result<Image>。
21. `Image::from_rgba(width, height, data)`。
22. `Image::save(path)`。
23. `Image::region(x, y, w, h) -> Image`。
24. `Sprite` 结构体：texture 句柄、source rect、color tint、flip_x/flip_y、anchor、origin。
25. `Sprite::from_texture(handle)`。
26. `Sprite::with_source_rect(rect)`。
27. `Sprite::with_color(color)`。
28. `Sprite::with_flip_x(bool)`。
29. `Sprite::with_flip_y(bool)`。
30. `Sprite::with_anchor(Vec2)`。
31. `Sprite::draw(&self, transform, ctx)`。
32. `AnimatedSprite`：基于图集与帧序列。
33. `AnimatedSprite::play / pause / stop / is_playing`。
34. `AnimatedSprite::current_frame`。
35. `AnimatedSprite::fps` / `total_frames`。
36. `SpriteBatch`：多个同纹理 Sprite 合并绘制，减少 draw call。
37. `SpriteBatch::new(texture)`。
38. `SpriteBatch::add(sprite, transform)`。
39. `SpriteBatch::draw(ctx)`。
40. `SpriteBatch::clear()`。
41. `SpriteBatch::capacity()` / `len()`。
42. `SpriteBatch` 内部使用顶点缓冲 + 索引缓冲（Triangles 2x3 per sprite）。
43. `BatchRenderer`：按 texture 自动合批。
44. `Quad` 结构体（x,y,w,h,u0,v0,u1,v1,color）。
45. `TextureAtlas`：将多张纹理合成为一张大纹理，输出 UV 列表。
46. `TextureAtlasBuilder`：按 max_size / padding / algorithm 合并。
47. `TextureAtlas::from_images(images) -> (Atlas, Vec<(index, Rect)>)`。
48. `TextureAtlas::get_uv(index)`。
49. `TextureAtlas::get_rect(index)`。
50. `TextureAtlas::num_textures()`。
51. `TextureAtlas::size()`。
52. 算法 `Skyline` / `Guillotine` 两种 bin packing。
53. `OrthographicCamera`：left/right/bottom/top/near/far。
54. `OrthographicCamera::from_window(window, zoom)`。
55. `OrthographicCamera::projection(&self) -> Mat4`。
56. `OrthographicCamera::view(&self) -> Mat4`。
57. `OrthographicCamera::view_projection(&self) -> Mat4`。
58. `OrthographicCamera::screen_to_world(screen_pos)`。
59. `OrthographicCamera::world_to_screen(world_pos)`。
60. `OrthographicCamera::zoom(factor)`。
61. `OrthographicCamera::move_by(delta)`。
62. `Camera2D`：带位置/旋转/缩放的相机。
63. `View` / `Viewport`。
64. `Color`：RGBA f32。
65. `Color::new(r, g, b, a) / from_u8(r,g,b,a) / from_hex(&str) / to_hex(&self)`。
66. `Color` 常量：RED / GREEN / BLUE / WHITE / BLACK / TRANSPARENT / YELLOW / CYAN / MAGENTA / ORANGE / GRAY / LIME / PINK / PURPLE / TEAL（15+ 常用色）。
67. `Color::lerp(a, b, t)`。
68. `DrawParams`：color tint、blend_mode、z_order。
69. `BlendMode`：Alpha / Add / Subtract / Multiply / Replace / Invert / PreMultiplied。
70. `Buffer`：顶点/索引/Uniform buffer 抽象。
71. `Buffer::new(usage, size_bytes, data)`。
72. `Buffer::update(offset, data)`。
73. `Buffer::size(&self) -> usize`。
74. `Pipeline`：shader + layout + blend + depth + stencil + rasterizer state。
75. `PipelineBuilder`。
76. `Shader`：源码字符串 + 预编译支持。
77. `Shader::from_source(type, src)`。
78. `ShaderModule`：vertex / fragment / compute。
79. `BindGroup`：uniform + sampler 分组。
80. `BindGroupLayout`。
81. `VertexLayout`：pos/uv/color/normal 等 attribute。
82. `Mesh2D`：顶点数组 + 索引数组。
83. `Mesh2D::draw(ctx, pipeline, bindgroups)`。
84. 2D 正交着色器：内置 `sprite.vert / sprite.frag`。
85. 2D 纯色着色器：`color.vert / color.frag`。
86. 2D 文本着色器：留位。
87. 内置 shader 提供 hot-reload 支持（debug 模式下监视文件修改）。
88. `Renderer` 提供 `draw_quad / draw_texture / draw_texture_ex / draw_rectangle / draw_rectangle_lines / draw_circle / draw_circle_lines / draw_line / draw_triangle / draw_poly`。
89. `Renderer` 提供 `push_transform_matrix / pop_transform_matrix`。
90. `Renderer` 提供 `push_scissor_rect / pop_scissor_rect`。
91. `Renderer` 提供 `set_blend_mode / reset_blend_mode`。
92. `Renderer` 提供 `stats() -> RenderStats (draw_calls, vertices, indices, batches)`。
93. `RenderStats`：每帧 reset。
94. `DebugRenderer`：绘制调试图形（线框、包围盒、坐标轴）。
95. `DebugRenderer::line(a, b, color)`。
96. `DebugRenderer::rect(rect, color)`。
97. `DebugRenderer::circle(c, r, color)`。
98. `DebugRenderer::text(text, pos, color)`。
99. `examples/sprite_draw`：绘制一个精灵。
100. `examples/multi_sprite`：1000 个随机精灵 + FPS 统计。
101. `examples/batch_draw`：相同纹理 10k 精灵合批。
102. `examples/atlas_animation`：图集帧动画。
103. `examples/camera_follow`：相机跟随示例。
104. `examples/shape_draw`：基本图形绘制。
105. `examples/debug_draw`：绘制调试信息。
106. `examples/blend_mode`：演示不同 blend mode。
107. `examples/scissor`：演示剪刀矩形。
108. `examples/transform_stack`：矩阵栈。
109. `examples/hot_shader`：shader 热重载（debug 模式）。
110. WebAssembly：`cargo build --target wasm32-unknown-unknown` 可运行。
111. `cargo test -p engine-render 单元测试 20+ 条。
112. `criterion` 性能测试：10k / 100k 精灵帧率。
113. 纹理压缩：ETC1 / BCn 支持（留位，后续）。
114. 多线程渲染：RenderThread 通道（留位）。
115. CHANGELOG 记录 0.3.0。
116. README 加入「如何绘制精灵」章节。
117. clippy 无 warning。
118. fmt 检查通过。
119. `cargo doc --workspace 正常。
120. CI 三平台 green。

---

## 三、细分需求与验收

### 3.1 Renderer / RenderContext

121. `Renderer::new(window) -> Result<Self>。
122. `Renderer::default_backend() -> &str。
123. `Renderer::backend_info(&self) -> String。
124. `Renderer::set_clear_color(&mut self, color)。
125. `Renderer::set_vsync(&mut self, mode)。
126. `Renderer::set_resolution(&mut self, w, h)。
127. `Renderer::resize(&mut self, w, h)。
128. `Renderer::begin_frame(&mut self) -> Result<()>。
129. `Renderer::end_frame(&mut self) -> Result<()>。
130. `Renderer::present(&mut self)。
131. `Renderer::push_scissor_rect(&mut self, rect)。
132. `Renderer::pop_scissor_rect(&mut self)。
133. `Renderer::push_transform(&mut self, mat)。
134. `Renderer::pop_transform(&mut self)。
135. `Renderer::set_blend_mode(&mut self, mode)。
136. `Renderer::reset_blend_mode(&mut self)。
137. `Renderer::flush(&mut self)。
138. `Renderer::stats(&self) -> RenderStats。
139. `Renderer::camera(&self) -> Option<&Camera2D>。
140. `Renderer::set_camera(&mut self, camera)。
141. `Renderer::draw_quad(&mut self, quad)。
142. `Renderer::draw_texture(&mut self, tex, x, y, color)。
143. `Renderer::draw_texture_ex(&mut self, tex, x, y, params)。
144. `Renderer::draw_texture_pro(&mut self, tex, source, dest, origin, rot, color)。
145. `Renderer::draw_texture_rotated(&mut self, tex, x, y, angle, color)。
146. `Renderer::draw_texture_rect(&mut self, tex, source, dest, color)。
147. `Renderer::draw_rectangle(&mut self, x, y, w, h, color)。
148. `Renderer::draw_rectangle_lines(&mut self, x, y, w, h, thickness, color)。
149. `Renderer::draw_rectangle_rotated(&mut self, x, y, w, h, angle, color)。
150. `Renderer::draw_circle(&mut self, x, y, r, color)。
151. `Renderer::draw_circle_lines(&mut self, x, y, r, thickness, color)。
152. `Renderer::draw_line(&mut self, x1, y1, x2, y2, thickness, color)。
153. `Renderer::draw_triangle(&mut self, p1, p2, p3, color)。
154. `Renderer::draw_triangle_lines(&mut self, p1, p2, p3, thickness, color)。
155. `Renderer::draw_poly(&mut self, x, y, sides, radius, rotation, color)。
156. `Renderer::draw_poly_lines(&mut self, x, y, sides, radius, rotation, thickness, color)。
157. `Renderer::draw_text(&mut self, text, x, y, font_size, color) — 留位。
158. `RenderStats::default()。
159. `RenderStats::reset(&mut self)。
160. `RenderStats::draw_calls & vertices & indices & batches & texture_switches`。

### 3.2 Texture / Image / Sampler

161. `Texture2D::from_image(ctx, image) -> Result<Self>。
162. `Texture2D::empty(ctx, w, h, format)。
163. `Texture2D::from_file(ctx, path) -> Result<Self>。
164. `Texture2D::from_bytes(ctx, bytes) -> Result<Self>。
165. `Texture2D::width(&self) -> u32。
166. `Texture2D::height(&self) -> u32。
167. `Texture2D::size(&self) -> (u32, u32)。
168. `Texture2D::format(&self) -> TextureFormat。
169. `Texture2D::set_filter(&mut self, filter)。
170. `Texture2D::set_wrap(&mut self, wrap)。
171. `Texture2D::update(&mut self, rect, data)。
172. `Texture2D::generate_mipmaps(&mut self)。
173. `Texture2D::handle(&self) -> TextureHandle。
174. `TextureFormat::RGBA8 / RGBA16F / R8 / BGRA8`。
175. `FilterMode::Linear / Nearest`。
176. `WrapMode::Clamp / Repeat / MirrorRepeat`。
177. `Image::from_pixels(width, height, data) -> Self。
178. `Image::from_file(path) -> Result<Self>。
179. `Image::from_bytes(bytes) -> Result<Self>。
180. `Image::width(&self) -> u32。
181. `Image::height(&self) -> u32。
182. `Image::size(&self) -> (u32, u32)。
183. `Image::save(&self, path) -> Result<()>。
184. `Image::crop(&self, rect) -> Image。
185. `Image::flip_horizontal(&mut self)。
186. `Image::flip_vertical(&mut self)。
187. `Image::rotate_90(&mut self)。
188. `Image::rotate_180(&mut self)。
189. `Image::rotate_270(&mut self)。
190. `Image::resize(&mut self, new_w, new_h)。
191. `Image::pixels(&self) -> &[u8]。
192. `Image::pixels_mut(&mut self) -> &mut [u8]。
193. `SamplerBuilder::new()。
194. `SamplerBuilder::with_filter(mag, min)。
195. `SamplerBuilder::with_wrap(s, t)。
196. `SamplerBuilder::with_mipmap_filter(mode)。
197. `SamplerBuilder::with_anisotropy(level)。
198. `SamplerBuilder::build(&self, ctx) -> Sampler。
199. `Sampler::handle(&self) -> SamplerHandle。
200. `TextureManager::new()。
201. `TextureManager::load(path) -> Result<TextureHandle>。
202. `TextureManager::get(handle) -> Option<&Texture2D>。
203. `TextureManager::unload(handle)。
204. `TextureManager::reload(handle)。
205. `TextureManager::iter(&self) -> impl Iterator。

### 3.3 Sprite / SpriteBatch / AnimatedSprite

206. `Sprite::new(texture) -> Self。
207. `Sprite::from_texture_rect(texture, rect) -> Self。
208. `Sprite::source_rect(&self) -> Option<Rect>。
209. `Sprite::set_source_rect(&mut self, rect)。
210. `Sprite::color(&self) -> Color。
211. `Sprite::set_color(&mut self, color)。
212. `Sprite::flip_x(&self) -> bool。
213. `Sprite::set_flip_x(&mut self, bool)。
214. `Sprite::flip_y(&self) -> bool。
215. `Sprite::set_flip_y(&mut self, bool)。
216. `Sprite::anchor(&self) -> Vec2。
217. `Sprite::set_anchor(&mut self, anchor)。
218. `Sprite::size(&self) -> Vec2。
219. `Sprite::draw(&self, ctx, position)。
220. `Sprite::draw_ex(&self, ctx, params)。
221. `AnimatedSprite::new(atlas, fps, frames) -> Self。
222. `AnimatedSprite::play(&mut self)。
223. `AnimatedSprite::pause(&mut self)。
224. `AnimatedSprite::stop(&mut self)。
225. `AnimatedSprite::is_playing(&self) -> bool。
226. `AnimatedSprite::current_frame(&self) -> usize。
227. `AnimatedSprite::set_frame(&mut self, idx)。
228. `AnimatedSprite::total_frames(&self) -> usize。
229. `AnimatedSprite::fps(&self) -> f32。
230. `AnimatedSprite::set_fps(&mut self, fps)。
231. `AnimatedSprite::set_loop(&mut self, mode)。
232. `AnimatedSprite::loop_mode(&self) -> LoopMode。
233. `AnimatedSprite::update(&mut self, dt)。
234. `AnimatedSprite::draw(&self, ctx, position)。
235. `LoopMode::Once / Loop / PingPong`。
236. `SpriteBatch::new(texture) -> Self。
237. `SpriteBatch::with_capacity(texture, cap) -> Self。
238. `SpriteBatch::add(&mut self, sprite, position) -> usize。
239. `SpriteBatch::add_ex(&mut self, sprite, params) -> usize。
240. `SpriteBatch::set(&mut self, idx, sprite, position)。
241. `SpriteBatch::remove(&mut self, idx)。
242. `SpriteBatch::clear(&mut self)。
243. `SpriteBatch::len(&self) -> usize。
244. `SpriteBatch::is_empty(&self) -> bool。
245. `SpriteBatch::capacity(&self) -> usize。
246. `SpriteBatch::draw(&self, ctx)。
247. `SpriteBatch::draw_at(&self, ctx, x, y)。
248. `SpriteBatch::texture(&self) -> TextureHandle。
249. `SpriteBatch::set_texture(&mut self, tex)。
250. `BatchRenderer::new()。
251. `BatchRenderer::begin(&mut self)。
252. `BatchRenderer::draw(&mut self, sprite, transform)。
253. `BatchRenderer::end(&mut self, ctx)。
254. `BatchRenderer::flush(&mut self, ctx)。
255. `BatchRenderer::batches(&self) -> usize。

### 3.4 图集（TextureAtlas）

256. `TextureAtlasBuilder::new(max_size)。
257. `TextureAtlasBuilder::with_padding(pixels)。
258. `TextureAtlasBuilder::with_algorithm(algorithm)。
259. `TextureAtlasBuilder::add(image) -> usize。
260. `TextureAtlasBuilder::add_from_file(path) -> Result<usize>。
261. `TextureAtlasBuilder::build(&self, ctx) -> Result<(TextureAtlas, Vec<Rect>)>。
262. `TextureAtlas::texture(&self) -> TextureHandle。
263. `TextureAtlas::size(&self) -> (u32, u32)。
264. `TextureAtlas::len(&self) -> usize。
265. `TextureAtlas::is_empty(&self) -> bool。
266. `TextureAtlas::get(&self, idx) -> Option<Rect>。
267. `TextureAtlas::get_uv(&self, idx) -> Option<(Vec2, Vec2)>。
268. `TextureAtlas::get_sprite(&self, idx) -> Sprite。
269. `PackAlgorithm::Skyline / Guillotine`。
270. `PackResult::contains_collisions(&self) -> bool。

### 3.5 相机 / View / 颜色

271. `Camera2D::new()。
272. `Camera2D::from_window(window, zoom)。
273. `Camera2D::position(&self) -> Vec2。
274. `Camera2D::set_position(&mut self, pos)。
275. `Camera2D::rotation(&self) -> f32。
276. `Camera2D::set_rotation(&mut self, angle)。
277. `Camera2D::zoom(&self) -> f32。
278. `Camera2D::set_zoom(&mut self, zoom)。
279. `Camera2D::target(&self) -> Option<Vec2>。
280. `Camera2D::set_target(&mut self, target)。
281. `Camera2D::offset(&self) -> Vec2。
282. `Camera2D::set_offset(&mut self, offset)。
283. `Camera2D::projection(&self) -> Mat4。
284. `Camera2D::view(&self) -> Mat4。
285. `Camera2D::view_projection(&self) -> Mat4。
286. `Camera2D::screen_to_world(&self, screen_pos) -> Vec2。
287. `Camera2D::world_to_screen(&self, world_pos) -> Vec2。
288. `Camera2D::update(&mut self, dt) — 平滑跟随。
289. `OrthographicCamera::new(left, right, bottom, top, near, far)。
290. `OrthographicCamera::from_size(w, h)。
291. `OrthographicCamera::view_projection(&self) -> Mat4。
292. `View::new(camera, viewport)。
293. `Viewport::new(x, y, w, h)。
294. `Viewport::x(&self) / y(&self) / w(&self) / h(&self)。
295. `Color::new(r, g, b, a)。
296. `Color::from_rgb(r, g, b)。
297. `Color::from_rgba(r, g, b, a)。
298. `Color::from_u8(r, g, b, a)。
299. `Color::from_hex(hex) -> Result<Self>。
300. `Color::to_hex(&self) -> String。
301. `Color::to_vec4(&self) -> [f32; 4]。
302. `Color::to_array(&self) -> [f32; 4]。
303. `Color::lerp(a, b, t) -> Color。
304. `Color::RED / GREEN / BLUE / WHITE / BLACK / TRANSPARENT / YELLOW / CYAN / MAGENTA / ORANGE / GRAY / LIGHTGRAY / DARKGRAY / GOLD / LIME / PINK / PURPLE / TEAL / MAROON / NAVY / OLIVE / BROWN`（24 种常用色）。
305. `BlendMode::Alpha / Additive / Subtract / Multiply / Replace / Invert / PreMultiplied`。
306. `BlendMode::to_gl_enum(&self) -> u32（内部）。

### 3.6 Shader / Pipeline / Buffer / BindGroup

307. `Shader::from_source(stage, source) -> Result<Self>。
308. `Shader::from_file(stage, path) -> Result<Self>。
309. `ShaderStage::Vertex / Fragment / Compute`。
310. `ShaderModule::compile(&self, entry)。
311. `ShaderModule::hot_reload(&mut self)。
312. `PipelineBuilder::new(shader_module)。
313. `PipelineBuilder::with_vertex_layout(layout)。
314. `PipelineBuilder::with_blend_mode(mode)。
315. `PipelineBuilder::with_depth_test(enabled)。
316. `PipelineBuilder::with_cull_mode(mode)。
317. `PipelineBuilder::with_winding_order(order)。
318. `PipelineBuilder::build(ctx) -> Result<Pipeline>。
319. `Pipeline::bind(&self, ctx)。
320. `Buffer::new_vertex(ctx, data) -> Result<Self>。
321. `Buffer::new_index(ctx, data) -> Result<Self>。
322. `Buffer::new_uniform(ctx, data) -> Result<Self>。
323. `Buffer::update(&mut self, offset, data)。
324. `Buffer::size(&self) -> usize。
325. `BufferUsage::Vertex / Index / Uniform / CopyDst`。
326. `BindGroup::new(ctx, layout, resources) -> Result<Self>。
327. `BindGroupLayout::new(ctx, entries) -> Result<Self>。
328. `VertexLayout::new()。
329. `VertexLayout::push::<Vec2>("aPos")。
330. `VertexLayout::push::<Vec2>("aUv")。
331. `VertexLayout::push::<Color>("aColor")。
332. `VertexLayout::stride(&self) -> usize。
333. `VertexLayout::attributes(&self) -> &[VertexAttr]。
334. `Mesh2D::new(vertices, indices) -> Self。
335. `Mesh2D::quad(w, h, color) -> Self。
336. `Mesh2D::draw(&self, ctx, pipeline, bind_groups)。

### 3.7 调试 / Profiler

337. `DebugRenderer::new()。
338. `DebugRenderer::line(a, b, color)。
339. `DebugRenderer::rect(rect, color)。
340. `DebugRenderer::rect_lines(rect, color)。
341. `DebugRenderer::circle(c, r, color)。
342. `DebugRenderer::circle_lines(c, r, color)。
343. `DebugRenderer::text(text, pos, color) — 后续完善。
344. `DebugRenderer::cross(pos, size, color)。
345. `DebugRenderer::grid(origin, cell_size, cols, rows, color)。
346. `DebugRenderer::flush(ctx)。
347. `DebugRenderer::clear()。
348. `Profiler::push_scope(name)。
349. `Profiler::pop_scope()。
350. `Profiler::dump() -> String。

### 3.8 示例与测试

351. `examples/sprite_draw` 可运行。
352. `examples/multi_sprite` 可运行。
353. `examples/batch_draw` 可运行。
354. `examples/atlas_animation` 可运行。
355. `examples/camera_follow` 可运行。
356. `examples/shape_draw` 可运行。
357. `examples/debug_draw` 可运行。
358. `examples/blend_mode` 可运行。
359. `examples/scissor` 可运行。
360. `examples/transform_stack` 可运行。
361. `examples/hot_shader` 可运行（debug 模式）。
362. 单测 Image::from_bytes。
363. 单测 Texture::update。
364. 单测 SpriteBatch::add/draw。
365. 单测 Camera::screen_to_world。
366. 单测 Color::from_hex / to_hex 往返。
367. 单测 TextureAtlas 打包不越界。
368. 单测 OrthographicCamera 投影。
369. 单测 VertexLayout stride。
370. 单测 BlendMode::Alpha 的 gl enum。
371. `cargo fmt --check 全部通过。
372. `cargo clippy -- -D warnings 全部通过。
373. `cargo test --workspace 全部通过。
374. `cargo doc --workspace --no-deps 生成成功。
375. CHANGELOG 记录版本 0.3.0。
376. README.md 包含「快速开始」章节。
377. README.md 含有「绘制精灵」章节。
378. README.md 含有「相机与视图」章节。
379. README.md 含有「图集打包」章节。
380. README.md 含有「着色器与 Pipeline」章节。
381. 本 Sprint 新增 example 工程 >= 11 个。
382. 本 Sprint 公开 API 数量 <= 80。
383. 本 Sprint 公开 API doc comment 覆盖率 100%。
384. WebAssembly 目标能成功 `cargo build --target wasm32-unknown-unknown`（可选）。
385. WASM demo.html 能在浏览器中显示精灵（可选）。

> 以上 385 条需求构成 Sprint 03 全部清单。

---

## 四、验收标准

- [ ] `cargo run --example sprite_draw` 显示精灵
- [ ] `cargo run --example batch_draw` 批量绘制无闪烁
- [ ] `cargo run --example atlas_animation` 可播放动画
- [ ] `cargo run --example camera_follow` 相机可跟随对象移动
- [ ] `cargo test -p engine-render 全部通过
- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功
- [ ] 三平台 CI green
- [ ] README 至少 5 章节

---

## 五、下一个 Sprint

Sprint 04 将引入 2D 物理引擎（碰撞体 / 刚体 / 关节）+ 节点树 MVP，完成阶段一。
