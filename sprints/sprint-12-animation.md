# Sprint 12 · 动画系统 / 骨骼 / 状态机

> 阶段：阶段三 · 3D 管线升级（第 4 个 Sprint）  
> 周期：4 周  
> 核心目标：建立通用动画系统（关键帧 + 骨骼 + 状态机 + 混合树 + 动画事件 + IK 基础）  
> 验收：`examples/animation_state_machine` 与 `examples/animation_ragdoll` 可运行

---

## 一、Sprint 概览

本 Sprint 建立 `engine-animation` crate。核心交付：

- `AnimationClip`：关键帧动画（位置/旋转/缩放/浮点曲线）
- `Skeleton`：骨骼层级 + 绑定姿态
- `SkinnedMesh`：蒙皮 mesh + 顶点权重 + 矩阵调色板（matrix palette）
- `AnimationController`：动画控制器，按状态机切换
- `StateMachine`：状态/过渡/条件/参数
- `BlendTree1D/2D`：基于参数混合多个 clip
- `IK`：两 bone IK + CCD IK（基本实现）
- `AnimationEvent`：时间戳触发事件（脚步声、粒子等）
- `AnimationCurve` / `Track` / `Keyframe`
- `AnimationDebugRenderer`：可视化骨骼、轨迹
- `examples/animation_basic` / `..._state_machine` / `..._ragdoll` / `..._ik` / `..._blend_2d`

---

## 二、项目需求清单

1. `engine-animation` crate 建立。
2. `Keyframe<T>`：时间 + 值 + 插值模式（Const / Linear / Step / Bezier / Hermite / EaseIn / EaseOut / EaseInOut）。
3. `Keyframe<T>::time(&self) -> f32`。
4. `Keyframe<T>::value(&self) -> &T`。
5. `Curve<T>`：一条关键帧曲线，按 `Vec<Keyframe<T>>` + 插值模式。
6. `Curve<T>::sample(&self, t) -> T`。
7. `Curve<T>::duration(&self) -> f32`。
8. `Curve<T>::insert(&mut self, keyframe)`。
9. `Curve<T>::remove(&mut self, idx)`。
10. `Curve<T>::optimize(&mut self, error)`。
11. `Track`：针对单个目标（entity + property）绑定一条曲线。
12. `TrackTarget::Translation / TrackTarget::Rotation / TrackTarget::Scale / TrackTarget::Float(String)`。
13. `AnimationClip::new(name, duration) -> Self`。
14. `AnimationClip::name(&self) -> &str`。
15. `AnimationClip::duration(&self) -> f32`。
16. `AnimationClip::add_track(&mut self, track)`。
17. `AnimationClip::tracks(&self) -> &[Track]`。
18. `AnimationClip::sample(&self, time) -> Pose`。
19. `AnimationClip::wrap_mode(&self) -> WrapMode`。
20. `AnimationClip::set_wrap_mode(&mut self, mode)`。
21. `WrapMode::Once / Loop / PingPong / Clamp`。
22. `Pose`：一个时间点所有骨骼的局部变换数组。
23. `Pose::new(num_bones) -> Self`。
24. `Pose::bones(&self) -> &[(Vec3, Quat, Vec3)]`。
25. `Pose::set_bone(&mut self, idx, pos, rot, scale)`。
26. `Pose::get_bone(&self, idx) -> (Vec3, Quat, Vec3)`。
27. `Pose::blend(a, b, alpha) -> Pose`。
28. `Pose::additive_blend(base, additive, alpha) -> Pose`。
29. `Pose::clone_into(&self, other) -> ()`。
30. `Bone`：骨骼数据（name, parent_index, local_bind_pose, inverse_bind_pose）。
31. `Skeleton::new(bones) -> Self`。
32. `Skeleton::bones(&self) -> &[Bone]`。
33. `Skeleton::bone(&self, idx) -> &Bone`。
34. `Skeleton::bone_count(&self) -> usize`。
35. `Skeleton::bind_pose(&self) -> &Pose`。
36. `Skeleton::inverse_bind_pose(&self, idx) -> Mat4`。
37. `Skeleton::root(&self) -> usize`。
38. `Bone::name(&self) -> &str`。
39. `Bone::parent(&self) -> Option<usize>`。
40. `Bone::local_bind_pose(&self) -> (Vec3, Quat, Vec3)`。
41. `Bone::inverse_bind_pose(&self) -> Mat4`。
42. `SkinnedMesh`：mesh + 骨骼权重（vertex -> bone）。
43. `Skin::new(bones, weights) -> Self`。
44. `Skin::bone_count(&self) -> usize`。
45. `Skin::inverse_bind_matrices(&self) -> &[Mat4]`。
46. `Skin::bone_names(&self) -> &[String]`。
47. `VertexWeight::new(bone, weight)`。
48. `SkinnedMesh::skin(&self) -> &Skin`。
49. `SkinnedMesh::mesh(&self) -> Handle<Mesh3D>`。
50. `SkinnedMesh`：每帧根据 pose 计算 skin matrices（matrix palette）。
51. `SkinnedMeshRenderer`：顶点着色器 skinning 渲染。
52. `Animator` 组件：当前播放的 clip + 播放速度 + 时间。
53. `Animator::play(clip_handle)`。
54. `Animator::stop(&mut self)`。
55. `Animator::set_time(&mut self, t)`。
56. `Animator::time(&self) -> f32`。
57. `Animator::set_speed(&mut self, speed)`。
58. `Animator::speed(&self) -> f32`。
59. `Animator::is_playing(&self) -> bool`。
60. `Animator::events(&self) -> &[AnimationEvent]`。
61. `AnimationController`：基于状态机的动画控制器。
62. `AnimationController::new(machine) -> Self`。
63. `AnimationController::set_parameter(name, value)`。
64. `AnimationController::parameter(&self, name) -> Option<ParameterValue>`。
65. `AnimationController::current_state(&self) -> &str`。
66. `AnimationController::update(&mut self, dt)`。
67. `AnimationController::blend_space(&self) -> &[f32]`。
68. `AnimationController::pose(&self) -> &Pose`。
69. `StateMachine::new(name, entry_action(StateMachine::add_state(name, clip) -> StateHandle`。
70. `StateMachine::add_state(name, node)`。
71. `StateMachine::set_entry_state(name)`。
72. `StateMachine::add_transition(from, to, condition)`。
73. `StateMachine::add_any_state_transition(to, condition)`。
74. `Condition::Parameter(name, op, value)`。
75. `Condition::And(a, b)` / `Or(a, b)` / `Not(a)`。
76. `Condition::True / False`。
77. `Condition::TimeElapsed(seconds)`。
78. `Condition::EventTriggered(event_name)`。
79. `Transition::from_state(&self) -> &str`。
80. `Transition::to_state(&self) -> &str`。
81. `Transition::duration(&self) -> f32`。
82. `Transition::blend_mode(&self) -> BlendMode`。
83. `BlendMode::Linear / Additive / Crossfade`。
84. `StateNode::Clip(clip) / StateNode::Blend1D(tree) / StateNode::Blend2D(tree) / StateNode::BlendTree(tree) / StateNode::Layered(layered) / StateNode::StateMachine(nested)`。
85. `BlendNode1D::new(param, nodes)`。
86. `BlendNode1D::push(&mut self, node, value)`。
87. `BlendNode1D::parameter(&self) -> &str`。
88. `BlendNode2D::new(x_param, y_param, nodes)`。
89. `BlendNode2D::push(&mut self, (x, y, clip)`。
90. `BlendNode2D::interpolate(&self, x, y) -> Pose`。
91. `BlendSpace1D::linear(a, b, alpha) 简单插值。
92. `BlendSpace2D::bilinear 双线性插值。
93. `LayeredBlend::base_layer + additive_layer 层级混合（additive animation 叠加）。
94. `AdditiveClip::new(base, additive, alpha)`。
95. `ParameterValue::Bool / Float / Int / Vec2 / Vec3 / Trigger(一次性事件)`。
96. `AnimationControllerBuilder`：流畅构造状态机。
97. `AnimationEvent`：`name + time + payload`。
98. `AnimationEventSystem`：每帧从 clip 中查询已触发的事件。
99. `IK::two_bone_ik(shoulder, elbow, wrist, target_pos, elbow_dir) -> (shoulder_rot, elbow_rot)`。
100. `IK::ccd_ik(chain, target, tolerance, max_iter)`。
101. `IK::fabrik(chain, target, tolerance, max_iter)`。
102. `FABRIK`：Forward And Backward Reaching Inverse Kinematics。
103. `IKChain::apply(&mut self, pose) -> Pose`。
104. `AimIK`：角色头部/武器瞄准目标点。
105. `LookAtIK`：让某骨骼朝向目标方向。
106. `FootIK`：根据地面贴合（简单版）。
107. `Ragdoll`：把 skeleton 映射为物理 joint 物理驱动动画（ragdoll → 物理）。
108. `Ragdoll::activate(&self, world, entities) -> RagdollHandle`。
109. `Ragdoll::bake(&self, world, pose) -> Pose`。
110. `Ragdoll::sync_ragdoll_to_animation(&self, world, entities) -> Pose`。
111. `RagdollBuilder`：设置每段骨骼的物理 collider/joint。
112. `AnimationDebugRenderer`：绘制骨骼、骨骼连线、权重可视化。
113. `AnimationDebugRenderer::draw_skeleton(&mut self, skeleton, pose, transform, color)`。
114. `AnimationDebugRenderer::draw_bone_weights(&mut self, mesh, weights, color)`。
115. `AnimationDebugRenderer::flush(&self, renderer)`。
116. `AnimationClipLoader`：加载 glTF / FBX / 自定义二进制。
117. `glTF 加载：`gltf::animation` 解析动画并导入为 AnimationClip`。
118. `AnimationClip::from_gltf(path) -> Result<Self>`。
119. `Skeleton::from_gltf(path) -> Result<Self>`。
120. `SkinnedMesh::from_gltf(path) -> Result<Self>`。
121. `AnimationGraph`：可视化节点图（编辑器端）。
122. `AnimationGraph::graph() 定义节点和连接。
123. `AnimationGraph::update(dt)` 推进状态机。
124. `AnimationGraphEditor`：在编辑器中可编辑 state machine（节点式）。
125. `AnimationBlending`：线性插值 / 加性 / 交叉淡入淡出。
126. `AnimationLayer`：多层动画叠加。
127. `AnimationLayer::layer(&self) -> usize`。
128. `AnimationLayer::set_weight(&mut self, w)`。
129. `AnimationLayer::mask(&self) -> Option<&AnimationMask>`。
130. `AnimationMask`：布尔数组，标记哪些骨骼受 layer 影响。
131. `AnimationMask::new(num_bones) -> Self`。
132. `AnimationMask::set(&mut self, idx, b)`。
133. `AnimationMask::get(&self, idx) -> bool`。
134. `AnimationMask::with_bone_name(skeleton, name) -> Self`。
135. `AnimationMask::union(&self, other) -> Self`。
136. `AnimationMask::intersection(&self, other) -> Self`。
137. `PlayBack` 队列系统（delay/queue/jump_to/crossfade）。
138. `PlayBack::queue(clip)`。
139. `PlayBack::crossfade(clip, duration)`。
140. `PlayBack::jump_to(clip, time)`。
141. `PlayBack::set_time(time)`。
142. `PlayBack::update(&mut self, dt)`。
143. `AnimationSampler`：glTF 中 sampler 的抽象。
144. `AnimationSampler::interpolation(&self) -> Interpolation`。
145. `Interpolation::Linear / Step / CubicSpline`。
146. `AnimationSampler::input(&self) -> &[f32]`（时间）。
147. `AnimationSampler::output(&self) -> &[T]`。
148. `AnimationAssetLoader`：统一加载入口 `load(path) -> Handle<AnimationClip>`。
149. `AnimationAssetLoader`：`get(handle) -> &AnimationClip`。
150. `SkeletonAssetLoader`：对应 Skeleton 加载。
151. `SkinAssetLoader`：对应 Skin 加载。
152. `examples/animation_basic`：播放单个动画循环。
153. `examples/animation_state_machine`：Idle/Walk/Run/Jump 状态机。
154. `examples/animation_blend_1d`：基于 speed 参数混合 Idle→Walk→Run。
155. `examples/animation_blend_2d`：2D 混合（XZ 方向混合）。
156. `examples/animation_additive`：叠加呼吸 idle + 上半身攻击。
157. `examples/animation_ragdoll`：物理 ragdoll。
158. `examples/animation_ik`：两 bone IK 瞄准目标。
159. `examples/animation_look_at`：头部看向鼠标。
160. `examples/animation_retarget`：同 clip 复用。
161. `examples/animation_event`：动画事件（脚步声触发音效）。
162. 单测：`Curve<Vec3>` 线性插值正确。
163. 单测：`Curve<Quat>` slerp 正确。
164. 单测：`Pose::blend` 输出中间姿态。
165. 单测：`StateMachine` 条件触发切换。
166. 单测：`Blend1D` 在边界值正确。
167. 单测：`IK::two_bone_ik` 数值正确。
168. 单测：`AnimationClip::wrap_mode_loop` 时间回绕。
169. 单测：`AnimationEvent` 在指定时间触发。
170. 单测：`AnimationSampler::CubicSpline` 采样正确。
171. 单测：`PlayBack::crossfade` 交叉混合输出 pose。
172. 单测：`AnimationMask::union` 正确。
173. `cargo test -p engine-animation` 全部通过。
174. `cargo clippy --workspace -- -D warnings` 通过。
175. `cargo fmt --check --workspace` 通过。
176. `cargo doc --workspace --no-deps` 成功。
177. CI 三平台 green。
178. CHANGELOG 记录版本 0.12.0。
179. README.md 加入「动画系统」章节。
180. README.md 加入「骨骼动画与状态机」章节。
181. README.md 加入「动画混合与 IK」章节。
182. 公开 API doc comment 覆盖率 100%。
183. 本 Sprint `unsafe` 块 <= 3。
184. 新增 example 工程 >= 10 个。
185. `examples/animation_state_machine` 根据按键切换 Idle/Walk/Run/Jump。
186. `examples/animation_ragdoll` 在角色死亡后切换 ragdoll。

> 以上 186 条需求构成 Sprint 12 全量清单。

---

## 三、细分需求与验收

### 3.1 关键帧与曲线

187. `Keyframe<T>::new(time, value)`。
188. `Keyframe<T>::with_interpolation(interp)`。
189. `KeyframeInterpolation::Linear / Step / Bezier(c0, c1) / Hermite(tan_in, tan_out) / EaseIn / EaseOut / EaseInOut`。
190. `Curve<T>::new() -> Self`。
191. `Curve<T>::with_interpolation(interp) -> Self`。
192. `Curve<T>::push(&mut self, kf)`。
193. `Curve<T>::insert_sorted(&mut self, kf)`。
194. `Curve<T>::remove(&mut self, idx)`。
195. `Curve<T>::len(&self) -> usize`。
196. `Curve<T>::is_empty(&self) -> bool`。
197. `Curve<T>::keyframes(&self) -> &[Keyframe<T>]`。
198. `Curve<T>::keyframes_mut(&mut self) -> &mut [Keyframe<T>]`。
199. `Curve<T>::sample(&self, time) -> T`。
200. `Curve<T>::sample_with_wrap(&self, time, wrap) -> T`。
201. `Curve<T>::duration(&self) -> f32`。
202. `Curve<Vec3>` 使用线性插值。
203. `Curve<Quat>` 使用 slerp。
204. `Curve<f32>` 使用线性。
205. `Curve<Color>` 使用线性（或 HSL 切换）。
206. `Curve<T>::optimize(&mut self, max_error)` 去除冗余 keyframe。
207. `Curve<T>::wrap_mode(&self) -> WrapMode`。
208. `Curve<T>::set_wrap_mode(&mut self, mode)`。
209. `WrapMode::Once / Loop / PingPong / ClampForever`。
210. `wrap_time(time, duration, mode) -> f32`。

### 3.2 Track / Clip / Pose

211. `Track::new(bone, translation_curve, rotation_curve, scale_curve) -> Self`。
212. `Track::bone(&self) -> usize`（骨骼索引）。
213. `Track::translation(&self) -> &Curve<Vec3>`。
214. `Track::rotation(&self) -> &Curve<Quat>`。
215. `Track::scale(&self) -> &Curve<Vec3>`。
216. `Track::custom_curves(&self) -> &HashMap<String, Curve<f32>>`。
217. `Track::sample_local_pose(&self, time) -> (Vec3, Quat, Vec3)`。
218. `AnimationClip::new(name, duration) -> Self`。
219. `AnimationClip::with_warp_mode(mode) -> Self`。
220. `AnimationClip::name(&self) -> &str`。
221. `AnimationClip::duration(&self) -> f32`。
222. `AnimationClip::tracks(&self) -> &[Track]`。
223. `AnimationClip::tracks_mut(&mut self) -> &mut Vec<Track>`。
224. `AnimationClip::add_track(&mut self, track)`。
225. `AnimationClip::sample(&self, time) -> Pose`。
226. `AnimationClip::sample_into(&self, time, pose)`。
227. `AnimationClip::events(&self) -> &[AnimationEvent]`。
228. `AnimationClip::add_event(&mut self, event)`。
229. `AnimationClip::wrap_mode(&self) -> WrapMode`。
230. `AnimationClip::set_wrap_mode(&mut self, mode)`。
231. `AnimationClip::is_looping(&self) -> bool`。
232. `Pose::new(num_bones) -> Self`。
233. `Pose::with_default_bind(skeleton) -> Self`。
234. `Pose::len(&self) -> usize`。
235. `Pose::bones(&self) -> &[(Vec3, Quat, Vec3)]`。
236. `Pose::bones_mut(&mut self) -> &mut [(Vec3, Quat, Vec3)]`。
237. `Pose::set_bone(&mut self, idx, pos, rot, scale)`。
238. `Pose::get_bone(&self, idx) -> (Vec3, Quat, Vec3)`。
239. `Pose::blend(a, b, alpha) -> Pose`。
240. `Pose::blend_into(&mut self, other, alpha)`。
241. `Pose::additive_blend(base, additive, alpha) -> Pose`。
242. `Pose::lerp(a, b, alpha) -> Pose`。
243. `Pose::identity(num_bones) -> Pose`（单位姿态）。
244. `Pose::local_to_world(&self, skeleton) -> Vec<Mat4>`（计算世界空间 matrix palette）。

### 3.3 Skeleton / Skin / SkinnedMesh

245. `Bone::new(name, parent, local_bind_pose, inverse_bind_pose) -> Self`。
246. `Bone::name(&self) -> &str`。
247. `Bone::parent(&self) -> Option<usize>`。
248. `Bone::local_bind_pose(&self) -> (Vec3, Quat, Vec3)`。
249. `Bone::inverse_bind_pose(&self) -> Mat4`。
250. `Skeleton::new(bones) -> Self`。
251. `Skeleton::bones(&self) -> &[Bone]`。
252. `Skeleton::bone(&self, idx) -> &Bone`。
253. `Skeleton::bone_count(&self) -> usize`。
254. `Skeleton::root(&self) -> usize`。
255. `Skeleton::children(&self, parent) -> Vec<usize>`。
256. `Skeleton::bind_pose(&self) -> &Pose`。
257. `Skeleton::inverse_bind_matrices(&self) -> &[Mat4]`。
258. `Skeleton::find_bone_by_name(&self, name) -> Option<usize>`。
259. `Skin::new(bone_names, inverse_bind_matrices) -> Self`。
260. `Skin::bone_count(&self) -> usize`。
261. `Skin::bone_names(&self) -> &[String]`。
262. `Skin::inverse_bind_matrices(&self) -> &[Mat4]`。
263. `VertexWeight::new(bone, weight)`。
264. `VertexWeight::bone(&self) -> u32`。
265. `VertexWeight::weight(&self) -> f32`。
266. `VertexWeightArray`：每个顶点最多 4 权重（标准实现）。
267. `SkinnedMesh::new(mesh, skin) -> Self`。
268. `SkinnedMesh::mesh(&self) -> Handle<Mesh3D>`。
269. `SkinnedMesh::skin(&self) -> &Skin`。
270. `SkinnedMesh::vertex_weights(&self) -> &[Vec<VertexWeight>]`。
271. `SkinnedMesh::compute_matrix_palette(&self, pose) -> Vec<Mat4>`。
272. `SkinnedMeshRenderer::new(renderer) -> Result<Self>`。
273. `SkinnedMeshRenderer::draw(&self, renderer, mesh, skeleton, pose, material, camera)`。

### 3.4 Animator 与状态机

274. `Animator::new(skeleton_handle) -> Self`。
275. `Animator::play(&mut self, clip)`。
276. `Animator::play_with_speed(&mut self, clip, speed)`。
277. `Animator::stop(&mut self)`。
278. `Animator::is_playing(&self) -> bool`。
279. `Animator::time(&self) -> f32`。
280. `Animator::set_time(&mut self, t)`。
281. `Animator::speed(&self) -> f32`。
282. `Animator::set_speed(&mut self, speed)`。
283. `Animator::wrap_mode(&self) -> WrapMode`。
284. `Animator::set_wrap_mode(&mut self, mode)`。
285. `Animator::current_clip(&self) -> Option<Handle<AnimationClip>>`。
286. `Animator::pose(&self) -> &Pose`。
287. `Animator::update(&mut self, dt)`。
288. `Animator::events_triggered(&self) -> &[AnimationEvent]`。
289. `AnimationController::new(state_machine) -> Self`。
290. `AnimationController::machine(&self) -> &StateMachine`。
291. `AnimationController::set_parameter_float(&mut self, name, value)`。
292. `AnimationController::set_parameter_bool(&mut self, name, value)`。
293. `AnimationController::set_parameter_int(&mut self, name, value)`。
294. `AnimationController::trigger(&mut self, name)`。
295. `AnimationController::current_state(&self) -> &str`。
296. `AnimationController::current_time(&self) -> f32`。
297. `AnimationController::update(&mut self, dt)`。
298. `AnimationController::pose(&self) -> &Pose`。
299. `StateMachine::new() -> Self`。
300. `StateMachine::add_state(&mut self, name, node) -> StateHandle`。
301. `StateMachine::set_entry_state(&mut self, name)`。
302. `StateMachine::add_transition(&mut self, from, to, duration, condition)`。
303. `StateMachine::add_any_state_transition(&mut self, to, condition)`。
304. `StateMachine::states(&self) -> &[StateNode]`。
305. `StateMachine::transitions(&self) -> &[Transition]`。
306. `StateMachine::parameters(&self) -> &ParameterMap`。
307. `StateHandle = usize`。
308. `Condition::True / False / Parameter(name, CompareOp, value) / And(a, b) / Or(a, b) / Not(a) / TimeElapsed(seconds) / EventTriggered(name)`。
309. `CompareOp::Equal / NotEqual / Less / LessEqual / Greater / GreaterEqual`。
310. `ParameterValue::Bool / Float / Int / Vec2 / Vec3 / Trigger`。
311. `ParameterMap::get(&self, name) -> Option<ParameterValue>`。
312. `ParameterMap::set(&mut self, name, value)`。
313. `ParameterMap::trigger(&mut self, name)`。
314. `Transition::from(&self) -> &str`。
315. `Transition::to(&self) -> &str`。
316. `Transition::duration(&self) -> f32`。
317. `Transition::blend_mode(&self) -> BlendMode`。
318. `Transition::exit_time(&self) -> f32`。
319. `Transition::has_exit_time(&self) -> bool`。
320. `BlendMode::Linear / Additive / Crossfade`。
321. `StateNode::Clip(clip_handle)`。
322. `StateNode::Blend1D(tree)`。
323. `StateNode::Blend2D(tree)`。
324. `StateNode::BlendTree(node)`。
325. `StateNode::Layered(layered)`。
326. `StateNode::StateMachine(nested)`。
327. `StateNode::duration(&self) -> f32`。
328. `StateNode::is_looping(&self) -> bool`。
329. `StateNode::wrap_mode(&self) -> WrapMode`。
330. `StateNode::speed(&self) -> f32`。
331. `StateNode::events(&self, time) -> Vec<AnimationEvent>`。
332. `BlendNode1D::new(param_name) -> Self`。
333. `BlendNode1D::with_nodes(nodes: Vec<(f32, Handle<AnimationClip>>) -> Self`。
334. `BlendNode1D::push(&mut self, value, clip)`。
335. `BlendNode1D::parameter(&self) -> &str`。
336. `BlendNode1D::interpolate(&self, param_value, clips) -> Pose`。
337. `BlendNode2D::new(x_param, y_param) -> Self`。
338. `BlendNode2D::push(&mut self, (x, y), clip)`。
339. `BlendNode2D::interpolate(&self, x, y) -> Pose`。
340. `BlendTree`：复合 node（自定义树状）。
341. `BlendTree::new(root) -> Self`。
342. `BlendTree::node(&self) -> &BlendTreeNode`。
343. `BlendTreeNode::Leaf(clip)`。
344. `BlendTreeNode::Blend1D(n, nodes) / Blend2D(n, nodes) / Additive(a, b, alpha) / Layered(layers) / Masked(layer, mask)`。
345. `LayeredBlend::new(base, layers) -> Self`。
346. `LayeredBlend::base(&self) -> &StateNode`。
347. `LayeredBlend::layers(&self) -> &[Layer]`。
348. `Layer::new(node, weight, mask)`。
349. `Layer::node(&self) -> &StateNode`。
350. `Layer::weight(&self) -> f32`。
351. `Layer::mask(&self) -> Option<&AnimationMask>`。
352. `Layer::set_weight(&mut self, w)`。
353. `AnimationMask::new(num_bones) -> Self`。
354. `AnimationMask::set(&mut self, idx, b)`。
355. `AnimationMask::get(&self, idx) -> bool`。
356. `AnimationMask::invert(&mut self)`。
357. `AnimationMask::union(&self, other) -> Self`。
358. `AnimationMask::intersection(&self, other) -> Self`。
359. `AnimationMask::with_bone_name(skeleton, name, b) -> Self`。
360. `PlayBack::new() -> Self`。
361. `PlayBack::play(&mut self, clip)`。
362. `PlayBack::stop(&mut self)`。
363. `PlayBack::queue(&mut self, clip)`。
364. `PlayBack::crossfade(&mut self, clip, duration)`。
365. `PlayBack::jump_to(&mut self, clip, time)`。
366. `PlayBack::set_time(&mut self, time)`。
367. `PlayBack::set_speed(&mut self, speed)`。
368. `PlayBack::is_playing(&self) -> bool`。
369. `PlayBack::time(&self) -> f32`。
370. `PlayBack::update(&mut self, dt)`。
371. `PlayBack::pose(&self) -> &Pose`。
372. `PlayBack::events(&self) -> &[AnimationEvent]`。
373. `AnimationControllerBuilder::new() -> Self`。
374. `AnimationControllerBuilder::with_state(name, node) -> Self`。
375. `AnimationControllerBuilder::with_entry(name) -> Self`。
376. `AnimationControllerBuilder::with_transition(from, to, duration, condition) -> Self`。
377. `AnimationControllerBuilder::build(&self) -> AnimationController`。
378. `AnimationEvent::new(name, time) -> Self`。
379. `AnimationEvent::with_payload(name, time, payload) -> Self`。
380. `AnimationEvent::name(&self) -> &str`。
381. `AnimationEvent::time(&self) -> f32`。
382. `AnimationEvent::payload(&self) -> Option<&str>`。
383. `AnimationEventSystem::pop(&self) -> String`。

### 3.5 IK / Ragdoll

384. `IK::two_bone_ik(shoulder, elbow, wrist, target_pos, elbow_dir) -> (shoulder_rot, elbow_rot)`。
385. `IK::ccd_ik(chain, target, tolerance, max_iter) -> Vec<Quat>`。
386. `IK::fabrik(chain, target, tolerance, max_iter) -> Vec<Vec3>`。
387. `IKChain::new(bones) -> Self`。
388. `IKChain::push(&mut self, bone_idx)`。
389. `IKChain::bones(&self) -> &[usize]`。
390. `IKChain::root(&self) -> usize`。
391. `AimIK::new(skeleton, head_bone, aim_axis)`。
392. `AimIK::apply(&self, pose, target_pos) -> Pose`。
393. `LookAtIK::new(skeleton, head_bone, up)`。
394. `LookAtIK::apply(&self, pose, target_pos) -> Pose`。
395. `FootIK::new(skeleton, left_foot_bone, right_foot_bone)`。
396. `FootIK::apply(&self, world, pose, ground_height_fn) -> Pose`。
397. `Ragdoll::new(skeleton) -> RagdollBuilder`。
398. `RagdollBuilder::bone(&mut self, idx, collider, joint_type)`。
399. `RagdollBuilder::build(&self, world) -> Ragdoll`。
400. `Ragdoll::activate(&mut self, world)`。
401. `Ragdoll::deactivate(&mut self, world)`。
402. `Ragdoll::bake_pose(&self, world) -> Pose`。
403. `Ragdoll::is_active(&self) -> bool`。
404. `RagdollJointType::Ball / Revolute / Fixed`。

### 3.6 资源加载与 glTF

405. `AnimationClip::from_gltf(path) -> Result<Vec<AnimationClip>>`。
406. `Skeleton::from_gltf(path) -> Result<Self>`。
407. `SkinnedMesh::from_gltf(path) -> Result<Self>`。
408. `AnimationClip::to_json(&self) -> String`。
409. `AnimationClip::from_json(json) -> Result<Self>`。
410. `AnimationAssetLoader::new()`。
411. `AnimationAssetLoader::load(path) -> Handle<AnimationClip>`。
412. `AnimationAssetLoader::get(handle) -> &AnimationClip`。
413. `AnimationAssetLoader::contains(handle) -> bool`。
414. `AnimationAssetLoader::unload(handle)`。
415. `SkeletonAssetLoader` 同上。
416. `SkinAssetLoader` 同上。
417. `gltf animation 解析：sampler input/output/interpolation。
418. `gltf skin` 解析：joints / matrices。
419. `gltf node` 解析：skin / mesh / joint。
420. `AnimationSampler::from_gltf(sampler) -> Self`。
421. `AnimationSampler::interpolation(&self) -> Interpolation`。
422. `AnimationSampler::input(&self) -> &[f32]`。
423. `AnimationSampler::output_vec3(&self) -> &[Vec3]`。
424. `AnimationSampler::output_quat(&self) -> &[Quat]`。
425. `AnimationSampler::sample_vec3(&self, t) -> Vec3`。
426. `AnimationSampler::sample_quat(&self, t) -> Quat`。
427. `Interpolation::Linear / Step / CubicSpline`。
428. `CubicSpline` 切线采样实现正确。

### 3.7 调试可视化与编辑器集成

429. `AnimationDebugRenderer::new() -> Self`。
430. `AnimationDebugRenderer::draw_skeleton(&mut self, skeleton, pose, transform, color)`。
431. `AnimationDebugRenderer::draw_joints(&mut self, skeleton, pose, color)`。
432. `AnimationDebugRenderer::draw_bone_weights(&mut self, mesh, weights, color)`。
433. `AnimationDebugRenderer::flush(&self, renderer)`。
434. `AnimationDebugRenderer::clear(&mut self)`。
435. 编辑器在 Inspector 中显示骨骼列表。
436. 编辑器在 3D 视图中绘制骨骼。
437. 编辑器支持播放/暂停/步进动画。
438. 编辑器动画时间轴：拖拽时间滑块。
439. 编辑器状态机可视化编辑。
440. 编辑器动画状态机节点可拖放编辑。
441. 编辑器状态机节点可设置参数。
442. 编辑器状态机 transition 可拖动连线。
443. 编辑器动画 clip 资源可预览。
444. 编辑器 blend 1D/2D 参数可视化。
445. 编辑器支持保存与加载状态机 JSON。

### 3.8 动画系统核心（系统函数

446. `animation_clip_sample_system(world)`。
447. `animation_controller_update_system(world, dt)`。
448. `animation_event_system(world)`。
449. `animation_pose_apply_system(world)`。
450. `animation_skinning_system(world)`。
451. `animation_skinning_render_system(world, renderer, camera)`。
452. `animation_debug_draw_system(world, renderer)`。
453. `animation_ragdoll_system(world)`。
454. `animation_ik_system(world)`。
455. `animation_look_at_system(world)`。
456. `animation_additive_blend_system(world)`。
457. `animation_crossfade_system(world)`。
458. `animation_event_dispatch_system(world)` 把动画事件转发为 ECS 事件。
459. `AnimationClipSet` 资源：Handle<AnimationClip> 集合。
460. `AnimationClipMapping`：clip 名称到 handle。
461. `SkinMatrixPalette`：skinning 矩阵数组。
462. `SkinMatrixBuffer`：GPU 端 buffer。
463. `SkinUniform`：shader 中 uniform。

### 3.9 示例与测试

464. `examples/animation_basic`。
465. `examples/animation_state_machine`。
466. `examples/animation_blend_1d`。
467. `examples/animation_blend_2d`。
468. `examples/animation_additive`。
469. `examples/animation_ragdoll`。
470. `examples/animation_ik`。
471. `examples/animation_look_at`。
472. `examples/animation_retarget`。
473. `examples/animation_event`。
474. 单测 `Curve::sample`。
475. 单测 `Curve::blend`。
476. 单测 `Pose::blend`。
477. 单测 `StateMachine` transition。
478. 单测 `Blend1D`。
479. 单测 `Blend2D`。
480. 单测 `IK::two_bone_ik`。
481. 单测 `IK::ccd_ik`。
482. 单测 `IK::fabrik`。
483. 单测 `AnimationClip wrap loop`。
484. 单测 `AnimationEvent` 触发。
485. 单测 `PlayBack::crossfade`。
486. 单测 `AnimationMask`。
487. 单测 `Skeleton::find_bone_by_name`。
488. 单测 `Pose::local_to_world`。
489. 单测 `SkinnedMesh::compute_matrix_palette`。
490. `cargo test -p engine-animation` 全部通过。
491. `cargo clippy --workspace -- -D warnings` 通过。
492. `cargo fmt --check --workspace` 通过。
493. `cargo doc --workspace --no-deps` 成功。
494. CI 三平台 green。
495. CHANGELOG 记录 0.12.0。
496. README.md 加入「动画系统」章节。
497. README.md 加入「骨骼动画与状态机」章节。
498. README.md 加入「动画混合与 IK」章节。
499. 公开 API doc comment 覆盖率 100%。
500. `unsafe` 块 <= 3。
501. 新增 example 工程 >= 10 个。
502. `examples/animation_state_machine` 可切换动画状态。

---

## 四、验收标准

- [ ] `cargo run --example animation_basic` 循环播放动画
- [ ] `cargo run --example animation_state_machine` 根据输入切换状态
- [ ] `cargo run --example animation_blend_1d` 线性平滑过渡
- [ ] `cargo run --example animation_ragdoll` 角色死亡后 ragdoll
- [ ] `cargo run --example animation_ik` 两 bone IK 瞄准目标
- [ ] `cargo test -p engine-animation` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.12.0

---

## 五、下一个 Sprint

Sprint 13 将引入粒子系统与后期特效栈。
