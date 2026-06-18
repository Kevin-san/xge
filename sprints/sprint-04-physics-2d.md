# Sprint 04 · 2D 物理 + 节点树 MVP

> 阶段：阶段一 · 基础内核 MVP（最后一个 Sprint）  
> 周期：4 周  
> 核心目标：完成 2D 物理引擎（重力/碰撞/响应）+ 节点式场景树 + Prefab + 可玩 Demo  
> 验收：可构建「横版像素平台小游戏」并演示跳跃、碰撞、拾取、得分

---

## 一、Sprint 概览

本 Sprint 建立 `engine-physics-2d` 与 `engine-scene` 两个 crate，并完成阶段一收尾（MVP 验证可用）。关键交付：

- `World2D`：物理世界（重力、阻尼）
- `RigidBody2D`：动态/静态/运动学三种类型
- `Collider2D`：圆形/矩形/多边形/凸包
- `PhysicsMaterial`：弹力/摩擦/密度
- `Contact2D`：接触点信息
- `Joint2D`：距离/铰链/焊接/滑动关节（初版）
- `SceneGraph` / `Node` / `Node2D` / `Node3D（留位）/ Sprite2D / Camera2DNode / Audio2D / AnimationPlayer
- `Prefab` / `SceneLoader`（JSON/BIN 双序列化）
- `examples/pixel_platformer` 横版像素 Demo

---

## 二、项目需求清单

1. `engine-physics-2d` crate 建立。
2. `World2D::new(gravity)`。
3. `World2D::set_gravity(&mut self, v)`。
4. `World2D::gravity(&self) -> Vec2`。
5. `World2D::step(&mut self, dt)` — 默认 dt = 1/60。
6. `World2D::step_with_iterations(&mut self, dt, velocity_iter, position_iter)`。
7. `World2D::insert_body(body) -> BodyHandle`。
8. `World2D::remove_body(handle)`。
9. `World2D::insert_collider(collider, handle)`。
10. `World2D::remove_collider(handle)`。
11. `World2D::get_body(&self, handle) -> &RigidBody2D`。
12. `World2D::get_body_mut(&mut self, handle) -> &mut RigidBody2D`。
13. `World2D::bodies(&self) -> 迭代器。
14. `World2D::colliders(&self) -> 迭代器。
15. `World2D::contacts(&self) -> Vec<Contact2D>`。
16. `World2D::ray_cast(origin, dir, max_toi, filter) -> Option<RayCastHit2D>`。
17. `World2D::shape_cast(shape, origin, dir, max_toi) -> Option<ShapeCastHit2D>`。
18. `World2D::point_overlap(point, filter) -> Vec<BodyHandle>`。
19. `World2D::aabb_overlap(aabb, filter) -> Vec<BodyHandle>`。
20. `World2D::set_paused(&mut self, bool)`。
21. `RigidBody2D::Dynamic / Static / Kinematic` 三种类型。
22. `RigidBody2DBuilder::new(BodyType)`。
23. `RigidBody2D::translation(&self) -> Vec2`。
24. `RigidBody2D::set_translation(&mut self, v)`。
25. `RigidBody2D::rotation(&self) -> f32`。
26. `RigidBody2D::set_rotation(&mut self, rad)`。
27. `RigidBody2D::linvel(&self) -> Vec2`。
28. `RigidBody2D::set_linvel(&mut self, v)`。
29. `RigidBody2D::angvel(&self) -> f32`。
30. `RigidBody2D::set_angvel(&mut self, v)`。
31. `RigidBody2D::apply_force(&mut self, force, point)`。
32. `RigidBody2D::apply_force_at_center(&mut self, force)`。
33. `RigidBody2D::apply_torque(&mut self, torque)`。
34. `RigidBody2D::apply_impulse(&mut self, impulse, point)`。
35. `RigidBody2D::apply_impulse_at_center(&mut self, impulse)`。
36. `RigidBody2D::mass(&self) -> f32`。
37. `RigidBody2D::inertia(&self) -> f32`。
38. `RigidBody2D::set_mass(&mut self, mass)`。
39. `RigidBody2D::local_center_of_mass(&self) -> Vec2`。
40. `RigidBody2D::set_gravity_scale(&mut self, scale)`。
41. `RigidBody2D::gravity_scale(&self) -> f32`。
42. `RigidBody2D::linear_damping(&self) -> f32`。
43. `RigidBody2D::set_linear_damping(&mut self, v)`。
44. `RigidBody2D::angular_damping(&self) -> f32`。
45. `RigidBody2D::set_angular_damping(&mut self, v)`。
46. `RigidBody2D::can_sleep(&self) -> bool`。
47. `RigidBody2D::set_can_sleep(&mut self, bool)`。
48. `RigidBody2D::sleeping(&self) -> bool`。
49. `RigidBody2D::wake_up(&mut self)`。
50. `RigidBody2D::type(&self) -> BodyType`。
51. `Collider2D`：圆形/矩形/多边形/胶囊/三角形。
52. `Collider2DBuilder::circle(radius)`。
53. `Collider2DBuilder::rect(w, h)`。
54. `Collider2DBuilder::polygon(points)` — 顶点需逆时针。
55. `Collider2DBuilder::capsule(half_h, radius)`。
56. `Collider2DBuilder::triangle(a, b, c)`。
57. `Collider2DBuilder::translation(v)`。
58. `Collider2DBuilder::rotation(rad)`。
59. `Collider2DBuilder::sensor(bool)`。
60. `Collider2DBuilder::material(PhysicsMaterial)`。
61. `Collider2DBuilder::density(density)`。
62. `Collider2DBuilder::friction(friction)`。
63. `Collider2DBuilder::restitution(restitution)`。
64. `Collider2DBuilder::collision_group(group)`。
65. `Collider2DBuilder::solver_groups(groups)`。
66. `Collider2DBuilder::build(&self) -> Collider2D`。
67. `PhysicsMaterial::default()`。
68. `PhysicsMaterial::friction / restitution / density`。
69. `Contact2D` 结构体：body_a / body_b / point / normal / penetration。
70. `ContactEvent::Started(Contact2D) / Ended(Contact2D)`。
71. `ContactManifold`：多点接触信息。
72. `CollisionGroup`：bitmask 分组 + 掩码。
73. `CollisionGroup::new(group_bits, mask_bits)`。
74. `CollisionGroup::with_all()`。
75. `CollisionGroup::with_none()`。
76. `CollisionGroup::memberships(&self)`。
77. `CollisionGroup::filters(&self)`。
78. `CollisionGroup::can_interact_with(a, b) -> bool`。
79. `RayCastHit2D`：handle / point / normal / toi。
80. `QueryFilter`：跳过指定 handle、是否包含传感器。
81. `Joint2D` 抽象 trait。
82. `DistanceJoint`。
83. `RevoluteJoint`（铰链）。
84. `WeldJoint`（焊接）。
85. `PrismaticJoint`（滑动）。
86. `SpringJoint`。
87. `MotorJoint`（驱动）。
88. `PhysicsDebugRenderer`：绘制碰撞体线框（基于 DebugRenderer）。
89. `PhysicsModule`：Engine 中的 Module 封装。
90. `engine-scene` crate 建立。
91. `Node` trait：`name() / on_update(dt) / on_ready() / on_destroy()`。
92. `Node2D` 结构体：position / rotation / scale / z_index。
93. `Node2D::new(name)。
94. `Node2D::local_matrix(&self) -> Mat3`。
95. `Node2D::world_matrix(&self) -> Mat3`。
96. `Node2D::set_position(&mut self, v)`。
97. `Node2D::set_rotation(&mut self, rad)`。
98. `Node2D::set_scale(&mut self, v)`。
99. `Node2D::children(&self) -> &[NodeHandle]`。
100. `Node2D::parent(&self) -> Option<NodeHandle>`。
101. `Node2D::add_child(&mut self, child)`。
102. `Node2D::remove_child(&mut self, child)`。
103. `Node2D::detach(&mut self)`。
104. `SceneTree` 场景树：根节点 + 更新顺序。
105. `SceneTree::new()。
106. `SceneTree::root(&self) -> NodeHandle`。
107. `SceneTree::add_child(parent, child)`。
108. `SceneTree::remove_child(parent, child)`。
109. `SceneTree::destroy_node(handle)`。
110. `SceneTree::get_node(&self, handle) -> &Node2D`。
111. `SceneTree::get_node_mut(&mut self, handle) -> &mut Node2D`。
112. `SceneTree::update(&mut self, dt)` — 先序遍历 on_update。
113. `SceneTree::draw(&self, renderer)` — 后序遍历 draw。
114. `SceneTree::find_by_name(&self, name) -> Option<NodeHandle>`。
115. `SceneTree::iter(&self) -> 迭代器。
116. `Sprite2D`：Node2D 子类，含精灵数据。
117. `Camera2DNode`：Node2D 子类，含相机。
118. `Audio2DNode`：Node2D 子类，含音源（下一阶段）。
119. `AnimationPlayerNode`：Node2D 子类，含动画。
120. `TimerNode`：Node2D 子类，含倒计时。
121. `Area2D`：Node2D 子类，含 sensor collider。
122. `Body2DNode`：Node2D 子类，含 RigidBody2D + collider。
123. `NodeSignal` / `emit("clicked")` — 事件派发（简化的信号系统）。
124. `Node::connect(signal, handler)` — 注册信号处理。
125. `Node::emit(signal, args...)` — 派发信号。
126. `Prefab`：节点树模板，可实例化。
127. `Prefab::from_scene(scene)`。
128. `Prefab::instantiate(&self) -> NodeHandle`。
129. `Prefab::save(&self, path)`。
130. `Prefab::load(path) -> Result<Self>`。
131. `SceneLoader::from_json(json) -> SceneTree`。
132. `SceneLoader::to_json(scene) -> String`。
133. `SceneLoader::from_binary(bytes) -> SceneTree`。
134. `SceneLoader::to_binary(scene) -> Vec<u8>`。
135. `SceneManager`：管理多场景切换、异步加载、预加载。
136. `SceneManager::load(path)`。
137. `SceneManager::switch_to(name)`。
138. `SceneManager::push(name)` — 保留旧场景。
139. `SceneManager::pop()` — 恢复旧场景。
140. `SceneManager::current(&self) -> Option<&SceneTree>`。
141. `Transition`：场景切换动画（淡入淡出 / 滑动 / 十字擦除）。
142. `Tween`：补间动画系统（线性、ease_in, ease_out, bounce, elastic）。
143. `TweenValue::Float / Vec2 / Vec3 / Color / Angle`。
144. `Tween::new(start, end, duration, ease)`。
145. `Tween::update(&mut self, dt)`。
146. `Tween::is_finished(&self) -> bool`。
147. `TweenManager`：管理多个 tweens。
148. `Timer`：通用定时器，interval、oneshot。
149. `Timer::new(duration, mode)`。
150. `Timer::tick(&mut self, dt) -> bool`。
151. `Timer::finished(&self) -> bool`。
152. `Timer::reset(&mut self)`。
153. `Timer::remaining(&self) -> f32`。
154. `Timer::elapsed(&self) -> f32`。
155. `TimerMode::Once / Repeat`。
156. `PhysicsModule` 集成至引擎：按 world step 更新节点。
157. `Body2DNode` 与 World2D 的同步：position/rotation 双向同步。
158. `examples/pixel_platformer` 横版像素 Demo：玩家移动、跳跃、碰撞地面、敌人、拾取硬币、得分。
159. `examples/ball_pit` 物理球掉落 Demo：1000 个圆碰撞。
160. `examples/dominoes` 多米诺 Demo。
161. `examples/ray_cast` 射线检测 Demo。
162. `examples/joints` 关节连接 Demo。
163. `examples/scene_tree` 节点树基础 Demo。
164. `examples/prefab_basic` 预制体基础。
165. `examples/scene_switch` 多场景切换 Demo。
166. `examples/signals` 信号系统 Demo。
167. `examples/tween` 补间 Demo。
168. `examples/hello_engine` 升级为节点树 + 2D 精灵的最小 Demo。
169. 单测：`RigidBody2DBuilder` 构建正常。
170. 单测：重力下球体下落符合物理。
171. 单测：两个圆碰撞反弹速度守恒。
172. 单测：Circle vs AABB 碰撞点正确。
173. 单测：RayCast 命中坐标正确。
174. 单测：SceneTree 遍历顺序正确。
175. 单测：Prefab 实例化不修改模板。
176. 单测：SceneLoader JSON 往返。
177. 单测：Tween ease_in_out 时间曲线。
178. 单测：Timer 重复模式下 tick N 次后 finished 次数正确。
179. 单测：CollisionGroup 分组过滤正确。
180. `cargo test --workspace 全部通过。
181. `cargo clippy --workspace -- -D warnings 通过。
182. `cargo fmt --check --workspace 通过。
183. `cargo doc --workspace --no-deps 成功。
184. CI 三平台 green。
185. CHANGELOG 记录版本 0.4.0（阶段一完成）。
186. README.md 加入「物理世界」章节。
187. README.md 加入「场景与节点」章节。
188. README.md 加入「预制体与场景切换」章节。
189. README.md 加入「信号与 Tween」章节。
190. `examples/pixel_platformer` README 记录玩法与操作。
191. `examples/pixel_platformer` 至少 3 个关卡。
192. `examples/pixel_platformer` 至少 1 种敌人（简单巡逻）。
193. `examples/pixel_platformer` 至少 1 种可收集金币。
194. `examples/pixel_platformer` HUD 显示分数与生命。
195. `examples/pixel_platformer` 支持空格跳跃 + 左右方向键移动。
196. 性能 Bench：100 个球在 1680x720 下稳定 60fps。
197. 性能 Bench：1000 个球在 1680x720 下 >= 30fps。
198. Debug 绘制：按 `键 B 显示/隐藏碰撞体线框。
199. Debug 绘制：按 `键 P 暂停/继续物理。
200. Debug 绘制：按 `键 F 显示/隐藏 FPS / FrameTime。

> 以上 200 条需求构成 Sprint 04 全量清单。

---

## 三、细分需求与验收（延续）

### 3.1 Physics World2D

201. `World2D::new()` 支持默认重力 (0, -9.81)。
202. `World2D::step(dt)` 在 dt 过大时分多步。
203. `World2D::step(dt)` 分 velocity / position 两个阶段。
204. `World2D` 维护 active body 列表。
205. `World2D` 支持 sleep / awake（energy threshold）。
206. `World2D` 支持 broad phase：AABB tree / sweep and prune（任选）。
207. `World2D` 支持 narrow phase：GJK / SAT。
208. `World2D` 支持 contact manifold 解算：sequential impulse。
209. `World2D` 支持 friction 模型：Coulomb friction。
210. `World2D` 支持 restitution 模型：速度阈值以上生效。
211. `World2D::ray_cast` 支持过滤 sensor。
212. `World2D::point_overlap` 找到包含该点的所有传感器。
213. `World2D::aabb_overlap` 找到与 AABB 相交的全部。
214. `World2D::insert_body` 后 handle 可用。
215. `World2D::remove_body` 后 handle 失效。
216. `World2D::events(&self) -> 迭代所有 contact events。
217. `World2D::contact_manifolds(&self) -> 迭代所有 manifold。
218. `World2D::clear(&mut self) — 清空世界。

### 3.2 RigidBody2D / Collider2D

219. `BodyType::Dynamic / Static / Kinematic / Sensor`。
220. `RigidBody2DBuilder::dynamic() / static() / kinematic() / sensor()`。
221. `RigidBody2DBuilder::translation(v)`。
222. `RigidBody2DBuilder::rotation(rad)`。
223. `RigidBody2DBuilder::linvel(v)`。
224. `RigidBody2DBuilder::angvel(v)`。
225. `RigidBody2DBuilder::gravity_scale(f)`。
226. `RigidBody2DBuilder::linear_damping(f)`。
227. `RigidBody2DBuilder::angular_damping(f)`。
228. `RigidBody2DBuilder::can_sleep(bool)`。
229. `RigidBody2DBuilder::ccd_enabled(bool)`。
230. `RigidBody2DBuilder::build(&self) -> RigidBody2D`。
231. `RigidBody2D::handle(&self) -> BodyHandle`。
232. `RigidBody2D::is_dynamic(&self) -> bool`。
233. `RigidBody2D::is_static(&self) -> bool`。
234. `RigidBody2D::is_kinematic(&self) -> bool`。
235. `RigidBody2D::ccd_enabled(&self) -> bool`。
236. `RigidBody2D::mass_properties(&self) -> MassProperties2D`。
237. `MassProperties2D::mass / local_center / inertia`。
238. `Collider2D::aabb(&self) -> AABB`。
239. `Collider2D::mass_properties(&self, density) -> MassProperties2D`。
240. `Collider2D::handle(&self) -> ColliderHandle`。
241. `Collider2D::body(&self) -> Option<BodyHandle>`。
242. `Collider2D::is_sensor(&self) -> bool`。
243. `Collider2D::material(&self) -> &PhysicsMaterial`。
244. `Collider2D::collision_groups(&self) -> CollisionGroup`。
245. `Collider2D::solver_groups(&self) -> CollisionGroup`。
246. `Shape` trait：`aabb(&self, transform) -> AABB`。
247. `Circle::aabb(transform)`。
248. `Rect::aabb(transform)`。
249. `Polygon::aabb(transform)`。
250. `Capsule::aabb(transform)`。

### 3.3 Joints

251. `DistanceJointBuilder::new(body_a, body_b, local_a, local_b)`。
252. `DistanceJointBuilder::length(f)`。
253. `DistanceJointBuilder::stiffness(f)`。
254. `DistanceJointBuilder::damping(f)`。
255. `RevoluteJointBuilder::new(body_a, body_b, anchor)`。
256. `RevoluteJointBuilder::limits(min, max)`。
257. `RevoluteJointBuilder::motor(velocity, max_torque)`。
258. `PrismaticJointBuilder::new(body_a, body_b, anchor, axis)`。
259. `PrismaticJointBuilder::limits(min, max)`。
260. `PrismaticJointBuilder::motor(velocity, max_force)`。
261. `WeldJointBuilder::new(body_a, body_b, local_a, local_b)`。
262. `SpringJointBuilder::new(body_a, body_b, anchor_a, anchor_b)`。
263. `SpringJointBuilder::stiffness(f) / damping(f) / rest_length(f)`。
264. `Joint::body_a(&self) -> BodyHandle`。
265. `Joint::body_b(&self) -> BodyHandle`。
266. `World2D::insert_joint(joint) -> JointHandle`。
267. `World2D::remove_joint(handle)`。
268. `World2D::joints(&self) -> 迭代。

### 3.4 SceneTree / Node

269. `Node::on_ready(&mut self)` — 首次创建后调用。
270. `Node::on_update(&mut self, dt)`。
271. `Node::on_draw(&self, renderer)`。
272. `Node::on_destroy(&mut self)`。
273. `Node::name(&self) -> &str`。
274. `Node::parent(&self) -> Option<NodeHandle>`。
275. `Node::children(&self) -> &[NodeHandle]`。
276. `Node::add_child(&mut self, child)`。
277. `Node::remove_child(&mut self, child)`。
278. `Node::set_parent(&mut self, parent)`。
279. `Node::detach(&mut self)`。
280. `Node::visible(&self) -> bool`。
281. `Node::set_visible(&mut self, bool)`。
282. `Node::paused(&self) -> bool`。
283. `Node::set_paused(&mut self, bool)`。
284. `Node2D::position(&self) -> Vec2`。
285. `Node2D::set_position(&mut self, v)`。
286. `Node2D::translate(&mut self, delta)`。
287. `Node2D::rotation(&self) -> f32`。
288. `Node2D::set_rotation(&mut self, rad)`。
289. `Node2D::rotate(&mut self, delta)`。
290. `Node2D::scale(&self) -> Vec2`。
291. `Node2D::set_scale(&mut self, v)`。
292. `Node2D::z_index(&self) -> i32`。
293. `Node2D::set_z_index(&mut self, v)`。
294. `Node2D::world_position(&self) -> Vec2`。
295. `Node2D::world_rotation(&self) -> f32`。
296. `Node2D::world_scale(&self) -> Vec2`。
297. `Node2D::local_matrix(&self) -> Mat3`。
298. `Node2D::world_matrix(&self) -> Mat3`。
299. `Node2D::local_transform(&self) -> Transform`。
300. `Sprite2D::sprite(&self) -> &Sprite`。
301. `Sprite2D::set_sprite(&mut self, sprite)`。
302. `Camera2DNode::camera(&self) -> &Camera2D`。
303. `Camera2DNode::set_camera(&mut self, camera)`。
304. `Area2D::collider(&self) -> Collider2D`。
305. `Area2D::on_entered(&self) -> &[BodyHandle]`。
306. `Body2DNode::body(&self) -> BodyHandle`。
307. `Body2DNode::sync_from_world(&mut self, world)`。

### 3.5 Prefab / SceneLoader

308. `Prefab::save_json(&self, path)`。
309. `Prefab::load_json(path) -> Result<Self>`。
310. `Prefab::save_bin(&self, path)`。
311. `Prefab::load_bin(path) -> Result<Self>`。
312. `Prefab::instantiate_in(&self, scene) -> NodeHandle`。
313. `SceneLoader::save_json(scene, path)`。
314. `SceneLoader::load_json(path) -> Result<SceneTree>`。
315. `SceneLoader::save_bin(scene, path)`。
316. `SceneLoader::load_bin(path) -> Result<SceneTree>`。
317. `SceneFile::version` 字段。
318. `SceneFile::nodes` 数组。
319. `SceneFile::resources` 引用表。
320. `SceneFile::signals` 信号连接表。

### 3.6 Tween / Signal

321. `Tween::new(start, end, duration, ease)`。
322. `Tween::with_repeat(times, mode)`。
323. `Tween::with_yoyo(bool)`。
324. `Tween::with_delay(delay)`。
325. `Tween::on_complete(callback)`。
326. `Tween::value(&self) -> TweenValue`。
327. `Tween::progress(&self) -> f32`。
328. `Tween::update(&mut self, dt)`。
329. `Tween::is_finished(&self) -> bool`。
330. `Tween::reset(&mut self)`。
331. `TweenManager::new()`。
332. `TweenManager::add(&mut self, tween) -> TweenHandle`。
333. `TweenManager::remove(&mut self, handle)`。
334. `TweenManager::update(&mut self, dt)`。
335. `TweenManager::clear(&mut self)`。
336. `Ease::Linear / InQuad / OutQuad / InOutQuad / InCubic / OutCubic / InOutCubic / InQuart / OutQuart / InOutQuart / InQuint / OutQuint / InOutQuint / InSine / OutSine / InOutSine / InExpo / OutExpo / InOutExpo / InCirc / OutCirc / InOutCirc / InBack / OutBack / InOutBack / InElastic / OutElastic / InOutElastic / InBounce / OutBounce / InOutBounce`（30+ 缓动曲线）。
337. `Signal::new(name)`。
338. `Signal::connect(&mut self, handler)`。
339. `Signal::disconnect(&mut self, handler_id)`。
340. `Signal::emit(&self, args...)`。
341. `Node::get_signal(&self, name) -> &Signal`。
342. `Node::signal_mut(&mut self, name) -> &mut Signal`。

### 3.7 示例工程（续）

343. `examples/pixel_platformer` 包含 Title 场景。
344. `examples/pixel_platformer` 包含 Game 场景。
345. `examples/pixel_platformer` 包含 GameOver 场景。
346. `examples/pixel_platformer` 使用 Prefab 创建玩家、敌人、金币。
347. `examples/pixel_platformer` 使用信号连接点击事件。
348. `examples/pixel_platformer` 使用 Tween 做过渡动画。
349. `examples/pixel_platformer` HUD 分数使用 UI 系统（Sprint 06 后升级）。
350. `examples/ball_pit` 1000 个彩色球随机生成并下落。
351. `examples/dominoes` 多米诺骨牌倒下。
352. `examples/ray_cast` 鼠标射线检测。
353. `examples/joints` 钟摆 / 弹簧演示。
354. `examples/scene_tree` 节点层级演示。
355. `examples/prefab_basic` 实例化多个 Prefab。
356. `examples/scene_switch` 按 1/2/3 键切换场景。
357. `examples/signals` 点击按钮派发信号。
358. `examples/tween` 多种缓动演示。
359. `examples/timer` 定时器演示。
360. `examples/physics_perf` 1000 球体性能测试。

### 3.8 测试 / 文档

361. 单元测试 World2D 双球碰撞速度守恒。
362. 单元测试 RayCastHit2D 命中点坐标。
363. 单元测试 CollisionGroup 互相过滤。
364. 单元测试 SceneTree 层级遍历。
365. 单元测试 Prefab 实例化与原模板独立。
366. 单元测试 SceneLoader JSON 往返。
367. 单元测试 Tween 线性缓动。
368. 单元测试 Ease::InOutCubic 在 t=0 / 0.5 / 1 处的输出。
369. 单元测试 Timer Once 模式的 finished 行为。
370. 单元测试 Signal emit 被所有 handler 接收。
371. 集成测试 `cargo test --workspace 全部通过。
372. `cargo fmt --check --workspace 通过。
373. `cargo clippy --workspace -- -D warnings 通过。
374. `cargo doc --workspace --no-deps 成功。
375. CI Windows / macOS / Linux green。
376. CHANGELOG 记录版本 0.4.0「阶段一完成」。
377. README.md 加入「物理引擎」章节。
378. README.md 加入「场景与节点」章节。
379. README.md 加入「预制体」章节。
380. README.md 加入「信号系统」章节。
381. README.md 加入「Tween 与定时器」章节。
382. 公开 API 数量 <= 120。
383. 公开 API doc comment 覆盖率 100%。
384. 本 Sprint `unsafe` <= 5。
385. 本 Sprint 新增示例工程 >= 12 个。

---

## 四、验收标准

- [ ] `cargo run --example pixel_platformer 可玩（跳跃、碰撞、得分）
- [ ] `cargo run --example ball_pit 1000 个球稳定 60fps
- [ ] `cargo run --example joints 关节演示正常
- [ ] `cargo run --example prefab_basic 实例化无崩溃
- [ ] `cargo run --example scene_switch 多场景切换正常
- [ ] `cargo run --example signals 信号派发正常
- [ ] `cargo run --example tween 补间正常
- [ ] `cargo test --workspace 全部通过
- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.4.0
- [ ] README 新增 5 章节

---

## 五、阶段一完成总结

**阶段一（MVP）至此结束。**  
已完成：核心架构 / 窗口系统 / 输入 / 2D 渲染 / 2D 物理 / 节点场景树 / Prefab / SceneManager / Tween / Signal / 12+ 示例 / 可玩平台 Demo。  
下一步进入 **阶段二（Sprint 05–08）**：ECS 系统、UI 控件库、可视化编辑器、跨平台打包管线。
