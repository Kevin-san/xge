# Sprint 11 · 3D 物理引擎集成

> 阶段：阶段三 · 3D 管线升级（第 3 个 Sprint）  
> 周期：4 周  
> 核心目标：基于 Rapier3D / Bullet / PhysX 风格抽象一个通用 3D 物理层，支持刚体、碰撞体、关节、角色控制器、射线检测、物理调试可视化  
> 验收：`examples/physics_3d_ragdoll` 与 `examples/physics_3d_stack` 可运行；物理性能满足典型场景

---

## 一、Sprint 概览

本 Sprint 在 `engine-physics-3d` crate 中提供统一物理抽象，默认使用 Rust 原生的 Rapier3D 作为后端，保留将来替换（PhysX / Jolt）的接口。核心交付：

- `PhysicsWorld3D`：物理世界（重力、阻尼、substep、CCD）
- `RigidBody3D`：Dynamic / Static / Kinematic / Fixed
- `Collider3D`：球 / 盒 / 圆柱 / 胶囊 / 圆锥 / 凸包 / 三角网格 / 高度场
- `Joint3D`：Fixed / Revolute / Prismatic / Ball / Distance / Rope
- `CharacterController3D`：可控制角色移动、碰撞响应、斜坡滑动
- `Query3D`：Ray / Shape / Point / AABB 各类查询
- `PhysicsDebugRenderer3D`：绘制碰撞体、接触点、关节锚
- `PhysicsModule`：与 ECS/Scene3D 集成，transform 双向同步
- `examples/physics_3d_basic` / `..._stack` / `..._ragdoll` / `..._character` / `..._joints` / `..._ray_cast` / `..._trigger`

---

## 二、项目需求清单

1. `engine-physics-3d` crate 建立。
2. `PhysicsBackend` trait：`new_world() / step() / insert_body() / remove_body() / insert_collider() / remove_collider() / insert_joint() / remove_joint() / cast_ray() / query_aabb() / point_test() / debug_draw()`。
3. 默认后端 `RapierBackend`：基于 `rapier3d` v0.19+。
4. `NullBackend`：占位（无物理），用于 unit test 与最小 demo。
5. `PhysicsWorld3D::new(gravity)`。
6. `PhysicsWorld3D::set_gravity(&mut self, g)`。
7. `PhysicsWorld3D::gravity(&self) -> Vec3`。
8. `PhysicsWorld3D::step(&mut self, dt)`。
9. `PhysicsWorld3D::step_with_substeps(&mut self, dt, substeps)`。
10. `PhysicsWorld3D::paused(&self) -> bool`。
11. `PhysicsWorld3D::set_paused(&mut self, bool)`。
12. `PhysicsWorld3D::num_bodies(&self) -> usize`。
13. `PhysicsWorld3D::num_colliders(&self) -> usize`。
14. `PhysicsWorld3D::num_joints(&self) -> usize`。
15. `PhysicsWorld3D::query_pipeline(&self) -> &QueryPipeline`。
16. `PhysicsWorld3D::ccd_enabled(&self) -> bool`。
17. `PhysicsWorld3D::set_ccd_enabled(&mut self, bool)`。
18. `PhysicsWorld3D::gravity_scale(&self) -> f32`。
19. `PhysicsWorld3D::set_gravity_scale(&mut self, v)`。
20. `PhysicsWorld3D::collision_groups() -> CollisionGroups`。
21. `PhysicsWorld3D::max_velocity_iterations(&self) -> usize`。
22. `PhysicsWorld3D::set_max_velocity_iterations(&mut self, n)`。
23. `PhysicsWorld3D::max_position_iterations(&self) -> usize`。
24. `PhysicsWorld3D::set_max_position_iterations(&mut self, n)`。
25. `RigidBody3D::Dynamic / Static / KinematicPositionBased / KinematicVelocityBased / Fixed`。
26. `RigidBodyBuilder::dynamic() -> Self`。
27. `RigidBodyBuilder::static_() -> Self`。
28. `RigidBodyBuilder::kinematic_position_based() -> Self`。
29. `RigidBodyBuilder::kinematic_velocity_based() -> Self`。
30. `RigidBodyBuilder::fixed() -> Self`。
31. `RigidBodyBuilder::translation(v) -> Self`。
32. `RigidBodyBuilder::rotation(q) -> Self`。
33. `RigidBodyBuilder::linvel(v) -> Self`。
34. `RigidBodyBuilder::angvel(v) -> Self`。
35. `RigidBodyBuilder::mass(v) -> Self`。
36. `RigidBodyBuilder::mass_properties(mp) -> Self`。
37. `RigidBodyBuilder::center_of_mass(v) -> Self`。
38. `RigidBodyBuilder::principal_inertia(v) -> Self`。
39. `RigidBodyBuilder::linear_damping(v) -> Self`。
40. `RigidBodyBuilder::angular_damping(v) -> Self`。
41. `RigidBodyBuilder::gravity_scale(v) -> Self`。
42. `RigidBodyBuilder::ccd_enabled(b) -> Self`。
43. `RigidBodyBuilder::sleeping(b) -> Self`。
44. `RigidBodyBuilder::dominance_group(i8) -> Self`。
45. `RigidBodyBuilder::additional_mass(v) -> Self`。
46. `RigidBodyBuilder::lock_translations() -> Self`。
47. `RigidBodyBuilder::lock_rotations() -> Self`。
48. `RigidBodyBuilder::restrict_rotations(x, y, z) -> Self`。
49. `RigidBodyBuilder::build(&self) -> RigidBody3D`。
50. `RigidBodyHandle`（类型句柄）。
51. `PhysicsWorld3D::insert_body(&mut self, body) -> RigidBodyHandle`。
52. `PhysicsWorld3D::remove_body(&mut self, handle)`。
53. `PhysicsWorld3D::body(&self, handle) -> &RigidBody3D`。
54. `PhysicsWorld3D::body_mut(&mut self, handle) -> &mut RigidBody3D`。
55. `RigidBody3D::type(&self) -> RigidBodyType`。
56. `RigidBody3D::position(&self) -> Vec3`。
57. `RigidBody3D::rotation(&self) -> Quat`。
58. `RigidBody3D::transform(&self) -> (Vec3, Quat)`。
59. `RigidBody3D::set_translation(&mut self, v, wake)`。
60. `RigidBody3D::set_rotation(&mut self, q, wake)`。
61. `RigidBody3D::set_position(&mut self, v, q, wake)`。
62. `RigidBody3D::linvel(&self) -> Vec3`。
63. `RigidBody3D::angvel(&self) -> Vec3`。
64. `RigidBody3D::set_linvel(&mut self, v, wake)`。
65. `RigidBody3D::set_angvel(&mut self, v, wake)`。
66. `RigidBody3D::mass(&self) -> f32`。
67. `RigidBody3D::set_mass(&mut self, mass)`。
68. `RigidBody3D::apply_force(&mut self, force, wake)`。
69. `RigidBody3D::apply_force_at_point(&mut self, force, point, wake)`。
70. `RigidBody3D::apply_torque(&mut self, torque, wake)`。
71. `RigidBody3D::apply_impulse(&mut self, impulse, wake)`。
72. `RigidBody3D::apply_impulse_at_point(&mut self, impulse, point, wake)`。
73. `RigidBody3D::apply_torque_impulse(&mut self, torque_impulse, wake)`。
74. `RigidBody3D::linear_damping(&self) -> f32`。
75. `RigidBody3D::set_linear_damping(&mut self, v)`。
76. `RigidBody3D::angular_damping(&self) -> f32`。
77. `RigidBody3D::set_angular_damping(&mut self, v)`。
78. `RigidBody3D::gravity_scale(&self) -> f32`。
79. `RigidBody3D::set_gravity_scale(&mut self, v)`。
80. `RigidBody3D::is_sleeping(&self) -> bool`。
81. `RigidBody3D::wake_up(&mut self, strong)`。
82. `RigidBody3D::sleep(&mut self)`。
83. `RigidBody3D::ccd_enabled(&self) -> bool`。
84. `RigidBody3D::enable_ccd(&mut self, b)`。
85. `RigidBody3D::is_dynamic(&self) -> bool`。
86. `RigidBody3D::is_static(&self) -> bool`。
87. `RigidBody3D::is_kinematic(&self) -> bool`。
88. `Collider3D`：各种形状。
89. `ColliderBuilder::ball(radius) -> Self`。
90. `ColliderBuilder::cuboid(hx, hy, hz) -> Self`。
91. `ColliderBuilder::capsule(half_h, radius, axis) -> Self`。
92. `ColliderBuilder::cylinder(half_h, radius) -> Self`。
93. `ColliderBuilder::cone(half_h, radius) -> Self`。
94. `ColliderBuilder::convex_hull(points) -> Option<Self>`。
95. `ColliderBuilder::trimesh(vertices, indices) -> Self`。
96. `ColliderBuilder::heightfield(heights, scale) -> Self`。
97. `ColliderBuilder::polyline(vertices) -> Self`。
98. `ColliderBuilder::segment(a, b) -> Self`。
99. `ColliderBuilder::triangle(a, b, c) -> Self`。
100. `ColliderBuilder::halfspace(outward_normal) -> Self`。
101. `ColliderBuilder::translation(v) -> Self`。
102. `ColliderBuilder::rotation(q) -> Self`。
103. `ColliderBuilder::density(v) -> Self`。
104. `ColliderBuilder::mass(v) -> Self`。
105. `ColliderBuilder::mass_properties(mp) -> Self`。
106. `ColliderBuilder::friction(v) -> Self`。
107. `ColliderBuilder::friction_combine_rule(rule) -> Self`。
108. `ColliderBuilder::restitution(v) -> Self`。
109. `ColliderBuilder::restitution_combine_rule(rule) -> Self`。
110. `ColliderBuilder::collision_groups(groups) -> Self`。
111. `ColliderBuilder::solver_groups(groups) -> Self`。
112. `ColliderBuilder::sensor(b) -> Self`。
113. `ColliderBuilder::contact_force_event_threshold(v) -> Self`。
114. `ColliderBuilder::contact_skin(v) -> Self`。
115. `ColliderBuilder::build(&self) -> Collider3D`。
116. `PhysicsWorld3D::insert_collider(&mut self, collider, parent_body) -> ColliderHandle`。
117. `PhysicsWorld3D::remove_collider(&mut self, handle)`。
118. `PhysicsWorld3D::collider(&self, handle) -> &Collider3D`。
119. `PhysicsWorld3D::collider_mut(&mut self, handle) -> &mut Collider3D`。
120. `CollisionGroups::new(memberships, filter)`。
121. `CollisionGroups::ALL`。
122. `CollisionGroups::NONE`。
123. `Collider3D::type(&self) -> ColliderType`。
124. `Collider3D::aabb(&self, body) -> AABB`。
125. `Collider3D::mass(&self) -> f32`。
126. `Collider3D::friction(&self) -> f32`。
127. `Collider3D::set_friction(&mut self, v)`。
128. `Collider3D::restitution(&self) -> f32`。
129. `Collider3D::set_restitution(&mut self, v)`。
130. `Collider3D::is_sensor(&self) -> bool`。
131. `Collider3D::set_sensor(&mut self, b)`。
132. `Joint3D`：关节抽象。
133. `FixedJointBuilder`。
134. `RevoluteJointBuilder`：单轴旋转。
135. `PrismaticJointBuilder`：单轴滑动。
136. `BallJointBuilder`：万向。
137. `DistanceJointBuilder`：两点距离约束。
138. `RopeJointBuilder`：距离上限（软绳）。
139. `SphericalJointBuilder`：带锥限制。
140. `PhysicsWorld3D::insert_joint(&mut self, body1, body2, joint) -> JointHandle`。
141. `PhysicsWorld3D::remove_joint(&mut self, handle)`。
142. `CharacterController3D`：基于 `KinematicCharacterController` 概念。
143. `CharacterController3D::new(offset, up_dir, max_slope_climb_angle, max_slide_angle)`。
144. `CharacterController3D::move_shape(&mut self, dt, desired_translation, body, collider, filter) -> CharacterMovement`。
145. `CharacterController3D::set_apply_impulse_to_dynamic_bodies(&mut self, b)`。
146. `CharacterController3D::set_slope_climb_angle(&mut self, angle)`。
147. `CharacterController3D::set_slide_angle(&mut self, angle)`。
148. `CharacterController3D::set_offset(&mut self, offset)`。
149. `CharacterController3D::set_max_distance_to_ground(&mut self, d)`。
150. `CharacterController3D::set_up(&mut self, v)`。
151. `CharacterMovement::translation(&self) -> Vec3`。
152. `CharacterMovement::grounded(&self) -> bool`。
153. `CharacterMovement::hit_ceil(&self) -> bool`。
154. `CharacterMovement::hit_wall(&self) -> bool`。
155. `Query3D::cast_ray(&self, world, ray, max_toi, solid, filter) -> Option<(Entity, toi, normal)>`。
156. `Query3D::cast_ray_and_get_normal(...)`。
157. `Query3D::cast_shape(&self, world, shape, pos, dir, max_toi, filter) -> Option<(Entity, toi, normal)>`。
158. `Query3D::intersection_with_shape(&self, world, shape, pos, filter) -> Vec<Entity>`。
159. `Query3D::point_intersections(&self, world, point, filter, cb)`。
160. `Query3D::intersections_with_aabb(&self, world, aabb, filter, cb)`。
161. `QueryFilter::new() / only_dynamic() / exclude_sensors() / groups(collision_groups)`。
162. `Ray3::new(origin, dir)`。
163. `Ray3::point_at(&self, t) -> Vec3`。
164. `RayCastHit::entity() / toi() / normal() / point()`。
165. `ContactEvent::Started(a, b) / Stopped(a, b)`。
166. `ContactForceEvent::with_threshold(threshold)`。
167. `PhysicsWorld3D::contact_events(&self) -> &[ContactEvent]`。
168. `PhysicsWorld3D::intersection_events(&self) -> &[IntersectionEvent]`。
169. `PhysicsWorld3D::contact_force_events(&self) -> &[ContactForceEvent]`。
170. `PhysicsWorld3D::contact_pair(&self, a, b) -> Option<ContactPair>`。
171. `PhysicsDebugRenderer3D::new() -> Self`。
172. `PhysicsDebugRenderer3D::draw_world(&mut self, world, renderer, view_proj)`。
173. `PhysicsDebugRenderer3D::set_draw_colliders(&mut self, b)`。
174. `PhysicsDebugRenderer3D::set_draw_joints(&mut self, b)`。
175. `PhysicsDebugRenderer3D::set_draw_contacts(&mut self, b)`。
176. `PhysicsDebugRenderer3D::set_draw_aabb(&mut self, b)`。
177. `PhysicsDebugRenderer3D::set_color(&mut self, color)`。
178. `PhysicsDebugRenderer3D::flush(&self, renderer)`。
179. `PhysicsModule`：ECS 模块，管理 body/collider/joint 组件。
180. `PhysicsModule::step(world, time)` 系统。
181. `RigidBodyComponent`：ECS 组件包裹 `RigidBodyHandle` 与 transform 同步。
182. `ColliderComponent`：ECS 组件包裹 `ColliderHandle`。
183. `JointComponent`：ECS 组件包裹 `JointHandle + (Entity, Entity)`。
184. 系统 `sync_transform_to_physics`：把 scene transform 同步到物理引擎。
185. 系统 `sync_physics_to_transform`：把物理引擎结果写回 scene transform。
186. `PhysicsQuery` system param：从 ECS 系统发起查询。
187. `PhysicsConfig`：gravity / default_friction / default_restitution / max_substeps / ccd。
188. `PhysicsConfig::load(path) -> Result<Self>`（JSON/TOML）。
189. `PhysicsConfig::save(&self, path) -> Result<()>`。
190. `PhysicsStats::num_bodies / num_colliders / num_joints / step_time_ms`。
191. `PhysicsStats::to_string(&self) -> String`。
192. `PhysicsTimestepMode::FixedDelta(dt)`。
193. `PhysicsTimestepMode::Variable`。
194. 编辑器集成：在 Inspector 面板显示 body/collider/joint 参数并可编辑。
195. 编辑器集成：调试开关可切换 collider/aabb/contacts/joints 显示。
196. 编辑器集成：按 `P` 切换物理运行/暂停。
197. `examples/physics_3d_basic`：盒子从空中掉落。
198. `examples/physics_3d_stack`：一堆盒子堆成塔。
199. `examples/physics_3d_ragdoll`：铰接 ragdoll。
200. `examples/physics_3d_character`：角色控制器行走/跳跃/斜坡。
201. `examples/physics_3d_joints`：钟摆 / 弹簧演示。
202. `examples/physics_3d_ray_cast`：点击发射射线。
203. `examples/physics_3d_trigger`：进入/离开传感器事件。
204. `examples/physics_3d_heightfield`：高度场地形碰撞。
205. `examples/physics_3d_compound`：复合碰撞体（子 collider）。
206. 单元测试：创建静态平面 + 动态球体 -> 球会落到平面。
207. 单元测试：重力（step 后 linvel.z == g * dt）。
208. 单元测试：`apply_force` 会改变 linvel。
209. 单元测试：射线命中静态物体，返回 toi > 0。
210. 单元测试：射线错过动态物体（在其外），返回 None。
211. 单元测试：`CollisionGroups` 过滤，命中时正确排除。
212. 单元测试：`Sensor` collider 触发 intersection_event。
213. 单元测试：`Joint` 两体被限制距离。
214. 单元测试：`CharacterController3D` 在斜坡上可站立。
215. 单元测试：`step(dt)` 为 0 时不崩溃。
216. 单元测试：`ccd_enabled` 高速小物体命中薄物体。
217. 单元测试：`AABB` transform 正确。
218. 单元测试：`Ray3` 平行于面不崩。
219. `cargo test -p engine-physics-3d` 全部通过。
220. `cargo clippy --workspace -- -D warnings` 通过。
221. `cargo fmt --check --workspace` 通过。
222. `cargo doc --workspace --no-deps` 成功。
223. CI 三平台 green。
224. CHANGELOG 记录版本 0.11.0。
225. README.md 加入「3D 物理引擎」章节。
226. README.md 加入「角色控制器」章节。
227. README.md 加入「物理查询系统」章节。
228. 公开 API doc comment 覆盖率 100%。
229. 本 Sprint `unsafe` 块 <= 5。
230. 新增 example 工程 >= 7 个。
231. `examples/physics_3d_stack` 100 个盒子稳定堆叠，CPU 稳定（>= 60fps）。
232. `examples/physics_3d_character` 角色可在斜坡上站立、滑行。
233. `examples/physics_3d_ragdoll` ragdoll 崩溃稳定。
234. 调试可视化（collider/aabb/contacts）可在编辑器中切换。
235. 与 ECS 系统双向同步正常。

> 以上 235 条需求构成 Sprint 11 全量清单。

---

## 三、细分需求与验收

### 3.1 PhysicsWorld

236. `PhysicsWorld3D::new(gravity)` 初始化所有内部状态。
237. `PhysicsWorld3D::step(dt)` 执行一次模拟。
238. `PhysicsWorld3D::step_with_substeps(dt, substeps)` 细分模拟。
239. `PhysicsWorld3D::gravity(&self) -> Vec3`。
240. `PhysicsWorld3D::set_gravity(&mut self, g)`。
241. `PhysicsWorld3D::set_paused(paused)` 暂停后 step 不更新位置。
242. `PhysicsWorld3D::paused(&self) -> bool`。
243. `PhysicsWorld3D::insert_body(body) -> RigidBodyHandle`。
244. `PhysicsWorld3D::remove_body(handle)`。
245. `PhysicsWorld3D::body(&self, handle) -> &RigidBody3D`。
246. `PhysicsWorld3D::body_mut(&mut self, handle) -> &mut RigidBody3D`。
247. `PhysicsWorld3D::insert_collider(collider, parent) -> ColliderHandle`。
248. `PhysicsWorld3D::remove_collider(handle)`。
249. `PhysicsWorld3D::collider(&self, handle) -> &Collider3D`。
250. `PhysicsWorld3D::collider_mut(&mut self, handle) -> &mut Collider3D`。
251. `PhysicsWorld3D::insert_joint(body1, body2, joint) -> JointHandle`。
252. `PhysicsWorld3D::remove_joint(handle)`。
253. `PhysicsWorld3D::num_bodies(&self) -> usize`。
254. `PhysicsWorld3D::num_colliders(&self) -> usize`。
255. `PhysicsWorld3D::num_joints(&self) -> usize`。
256. `PhysicsWorld3D::ccd_enabled(&self) -> bool`。
257. `PhysicsWorld3D::set_ccd_enabled(&mut self, bool)`。
258. `PhysicsWorld3D::gravity_scale(&self) -> f32`。
259. `PhysicsWorld3D::set_gravity_scale(&mut self, v)`。
260. `PhysicsWorld3D::contact_events(&self) -> &[ContactEvent]`。
261. `PhysicsWorld3D::intersection_events(&self) -> &[IntersectionEvent]`。
262. `PhysicsWorld3D::contact_force_events(&self) -> &[ContactForceEvent]`。
263. `PhysicsWorld3D::contact_pair(&self, a, b) -> Option<ContactPair>`。
264. `PhysicsWorld3D::bodies_iter(&self) -> impl Iterator<Item=(RigidBodyHandle, &RigidBody3D)>`。
265. `PhysicsWorld3D::colliders_iter(&self) -> impl Iterator<Item=(ColliderHandle, &Collider3D)>`。
266. `PhysicsWorld3D::query_pipeline(&self) -> &QueryPipeline`。
267. `PhysicsWorld3D::clear(&mut self)` 清空所有 body/collider/joint。

### 3.2 RigidBody / Collider

268. `RigidBodyBuilder::dynamic()`。
269. `RigidBodyBuilder::static_()`。
270. `RigidBodyBuilder::kinematic_position_based()`。
271. `RigidBodyBuilder::kinematic_velocity_based()`。
272. `RigidBodyBuilder::fixed()`。
273. `RigidBodyBuilder::translation(v) -> Self`。
274. `RigidBodyBuilder::rotation(q) -> Self`。
275. `RigidBodyBuilder::linvel(v) -> Self`。
276. `RigidBodyBuilder::angvel(v) -> Self`。
277. `RigidBodyBuilder::mass(v) -> Self`。
278. `RigidBodyBuilder::mass_properties(mp) -> Self`。
279. `RigidBodyBuilder::center_of_mass(v) -> Self`。
280. `RigidBodyBuilder::principal_inertia(v) -> Self`。
281. `RigidBodyBuilder::linear_damping(v) -> Self`。
282. `RigidBodyBuilder::angular_damping(v) -> Self`。
283. `RigidBodyBuilder::gravity_scale(v) -> Self`。
284. `RigidBodyBuilder::ccd_enabled(b) -> Self`。
285. `RigidBodyBuilder::sleeping(b) -> Self`。
286. `RigidBodyBuilder::dominance_group(g) -> Self`。
287. `RigidBodyBuilder::additional_mass(v) -> Self`。
288. `RigidBodyBuilder::lock_translations() -> Self`。
289. `RigidBodyBuilder::lock_rotations() -> Self`。
290. `RigidBodyBuilder::restrict_rotations(x, y, z) -> Self`。
291. `RigidBodyBuilder::build(&self) -> RigidBody3D`。
292. `RigidBodyType::Dynamic / Static / KinematicPositionBased / KinematicVelocityBased / Fixed`。
293. `RigidBody3D::type(&self) -> RigidBodyType`。
294. `RigidBody3D::position(&self) -> Vec3`。
295. `RigidBody3D::rotation(&self) -> Quat`。
296. `RigidBody3D::set_translation(&mut self, v, wake)`。
297. `RigidBody3D::set_rotation(&mut self, q, wake)`。
298. `RigidBody3D::linvel(&self) -> Vec3`。
299. `RigidBody3D::set_linvel(&mut self, v, wake)`。
300. `RigidBody3D::angvel(&self) -> Vec3`。
301. `RigidBody3D::set_angvel(&mut self, v, wake)`。
302. `RigidBody3D::mass(&self) -> f32`。
303. `RigidBody3D::apply_force(&mut self, f, wake)`。
304. `RigidBody3D::apply_force_at_point(&mut self, f, p, wake)`。
305. `RigidBody3D::apply_torque(&mut self, t, wake)`。
306. `RigidBody3D::apply_impulse(&mut self, i, wake)`。
307. `RigidBody3D::apply_impulse_at_point(&mut self, i, p, wake)`。
308. `RigidBody3D::apply_torque_impulse(&mut self, t, wake)`。
309. `RigidBody3D::linear_damping(&self) -> f32`。
310. `RigidBody3D::set_linear_damping(&mut self, v)`。
311. `RigidBody3D::angular_damping(&self) -> f32`。
312. `RigidBody3D::set_angular_damping(&mut self, v)`。
313. `RigidBody3D::gravity_scale(&self) -> f32`。
314. `RigidBody3D::set_gravity_scale(&mut self, v)`。
315. `RigidBody3D::is_sleeping(&self) -> bool`。
316. `RigidBody3D::wake_up(&mut self, strong)`。
317. `RigidBody3D::sleep(&mut self)`。
318. `RigidBody3D::ccd_enabled(&self) -> bool`。
319. `RigidBody3D::enable_ccd(&mut self, b)`。
320. `ColliderBuilder::ball(r)`。
321. `ColliderBuilder::cuboid(hx, hy, hz)`。
322. `ColliderBuilder::capsule(half_h, r, axis)`。
323. `ColliderBuilder::cylinder(half_h, r)`。
324. `ColliderBuilder::cone(half_h, r)`。
325. `ColliderBuilder::convex_hull(points)`。
326. `ColliderBuilder::trimesh(vertices, indices)`。
327. `ColliderBuilder::heightfield(heights, scale)`。
328. `ColliderBuilder::segment(a, b)`。
329. `ColliderBuilder::triangle(a, b, c)`。
330. `ColliderBuilder::halfspace(n)`。
331. `ColliderBuilder::translation(v) -> Self`。
332. `ColliderBuilder::rotation(q) -> Self`。
333. `ColliderBuilder::density(v) -> Self`。
334. `ColliderBuilder::mass(v) -> Self`。
335. `ColliderBuilder::friction(v) -> Self`。
336. `ColliderBuilder::restitution(v) -> Self`。
337. `ColliderBuilder::collision_groups(g) -> Self`。
338. `ColliderBuilder::sensor(b) -> Self`。
339. `ColliderBuilder::build(&self) -> Collider3D`。
340. `Collider3D::aabb(&self, body_transform) -> AABB`。
341. `Collider3D::mass(&self) -> f32`。
342. `Collider3D::friction(&self) -> f32`。
343. `Collider3D::set_friction(&mut self, v)`。
344. `Collider3D::restitution(&self) -> f32`。
345. `Collider3D::set_restitution(&mut self, v)`。
346. `Collider3D::is_sensor(&self) -> bool`。
347. `Collider3D::set_sensor(&mut self, b)`。
348. `Collider3D::shape(&self) -> ColliderShape`。
349. `ColliderShape::Ball(r) / Cuboid(hx, hy, hz) / Capsule(half_h, r) / Cylinder(half_h, r) / Cone(half_h, r) / Trimesh(verts, idx) / Heightfield(heights, scale) / ConvexHull(points) / Segment(a, b) / Triangle(a, b, c) / Halfspace(n)`。

### 3.3 Joints

350. `FixedJointBuilder::new()`。
351. `FixedJointBuilder::local_anchor1(v) -> Self`。
352. `FixedJointBuilder::local_anchor2(v) -> Self`。
353. `FixedJointBuilder::local_basis1(q) -> Self`。
354. `FixedJointBuilder::local_basis2(q) -> Self`。
355. `FixedJointBuilder::build(&self) -> Joint3D`。
356. `RevoluteJointBuilder::new(axis)`。
357. `RevoluteJointBuilder::local_anchor1(v) -> Self`。
358. `RevoluteJointBuilder::local_anchor2(v) -> Self`。
359. `RevoluteJointBuilder::motor_model(model) -> Self`。
360. `RevoluteJointBuilder::limits(min, max) -> Self`。
361. `RevoluteJointBuilder::motor_velocity(vel, factor) -> Self`。
362. `RevoluteJointBuilder::motor_position(pos, stiffness, damping) -> Self`。
363. `PrismaticJointBuilder::new(axis)`。
364. `PrismaticJointBuilder::limits(min, max) -> Self`。
365. `BallJointBuilder::new(anchor1, anchor2)`。
366. `BallJointBuilder::limits(max_angle) -> Self`。
367. `DistanceJointBuilder::new(anchor1, anchor2)`。
368. `DistanceJointBuilder::length(l) -> Self`。
369. `RopeJointBuilder::new(anchor1, anchor2, max_length)`。
370. `SphericalJointBuilder::with_cone_limit(axis, angle)`。

### 3.4 Character Controller

371. `CharacterController3D::new(offset, up, max_slope_climb_angle, max_slide_angle)`。
372. `CharacterController3D::move_shape(dt, desired, body, collider, filter) -> CharacterMovement`。
373. `CharacterController3D::set_apply_impulse_to_dynamic_bodies(&mut self, b)`。
374. `CharacterController3D::set_slope_climb_angle(&mut self, angle)`。
375. `CharacterController3D::set_slide_angle(&mut self, angle)`。
376. `CharacterController3D::set_offset(&mut self, offset)`。
377. `CharacterController3D::set_max_distance_to_ground(&mut self, d)`。
378. `CharacterController3D::set_up(&mut self, v)`。
379. `CharacterMovement::translation(&self) -> Vec3`。
380. `CharacterMovement::grounded(&self) -> bool`。
381. `CharacterMovement::hit_ceil(&self) -> bool`。
382. `CharacterMovement::hit_wall(&self) -> bool`。
383. `CharacterMovement::ground_normal(&self) -> Vec3`。
384. `CharacterControllerPlugin`：在 ECS 中提供 `character_controller_system`。
385. 系统在 update 阶段运行，读取 input/velocity 并更新 transform。
386. 系统自动在 grounded=true 时取消重力（可选配置）。
387. 系统支持「跳跃」逻辑（基于 grounded 触发）。
388. 支持可调步高（step_offset）。
389. 支持动态物体在角色脚下，产生推挤。
390. 支持斜坡滑动（高于 slide_angle 时不爬升）。

### 3.5 Query（Ray / Shape / Point / AABB）

391. `Ray3::new(origin, dir)`。
392. `Ray3::point_at(&self, t) -> Vec3`。
393. `Query3D::cast_ray(world, ray, max_toi, solid, filter) -> Option<(Entity, f32, Vec3)>`。
394. `Query3D::cast_ray_and_get_normal(world, ray, max_toi, solid, filter) -> Option<RayCastHit>`。
395. `Query3D::cast_shape(world, shape, pos, dir, max_toi, filter) -> Option<ShapeCastHit>`。
396. `Query3D::intersection_with_shape(world, shape, pos, filter) -> Vec<Entity>`。
397. `Query3D::point_intersections(world, point, filter, cb)`。
398. `Query3D::intersections_with_aabb(world, aabb, filter, cb)`。
399. `QueryFilter::new()`。
400. `QueryFilter::only_dynamic()`。
401. `QueryFilter::exclude_sensors()`。
402. `QueryFilter::groups(collision_groups)`。
403. `QueryFilter::exclude_fixed()`。
404. `QueryFilter::exclude(body)`。
405. `RayCastHit::entity() -> Entity`。
406. `RayCastHit::toi(&self) -> f32`。
407. `RayCastHit::point(&self) -> Vec3`。
408. `RayCastHit::normal(&self) -> Vec3`。
409. `ContactEvent::Started(a, b) / Stopped(a, b)`。
410. `IntersectionEvent::Started(a, b) / Stopped(a, b)`。
411. `ContactForceEvent::total_force(&self) -> Vec3`。
412. `ContactForceEvent::total_magnitude(&self) -> f32`。
413. `PhysicsWorld3D::contact_pair(&self, a, b) -> Option<ContactPair>`。
414. `ContactPair::normal(&self) -> Vec3`。
415. `ContactPair::points(&self) -> &[ContactPoint]`。
416. `ContactPoint::point(&self) -> Vec3`。
417. `ContactPoint::penetration(&self) -> f32`。

### 3.6 调试可视化

418. `PhysicsDebugRenderer3D::new() -> Self`。
419. `PhysicsDebugRenderer3D::draw_world(&mut self, world, renderer, view_proj, debug_flags)`。
420. `PhysicsDebugRenderer3D::draw_collider_lines(&mut self, collider, color)`。
421. `PhysicsDebugRenderer3D::draw_contact(&mut self, point, normal, magnitude, color)`。
422. `PhysicsDebugRenderer3D::draw_aabb(&mut self, aabb, color)`。
423. `PhysicsDebugRenderer3D::draw_joint_anchor(&mut self, anchor, color)`。
424. `PhysicsDebugRenderer3D::flush(&self, renderer)`。
425. `DebugPhysicsFlags::DRAW_COLLIDERS / DRAW_AABB / DRAW_CONTACTS / DRAW_JOINTS / DRAW_RIGID_BODIES`。
426. `DebugPhysicsFlags::default() -> Self`。
427. 调试绘制管线使用独立 pass，不干扰主 PBR 渲染。

### 3.7 ECS 集成

428. `PhysicsModule`：注册系统 `sync_transform_to_physics / step_physics / sync_physics_to_transform / update_query_pipeline`。
429. `RigidBodyComponent` 组件：handle + dirty flag。
430. `ColliderComponent` 组件：handle + optional parent body entity。
431. `JointComponent` 组件：handle + (entity_a, entity_b)。
432. `PhysicsPendingSpawn`：延迟创建（避免在系统中立即创建）。
433. `PhysicsPendingRemove`：延迟销毁。
434. `sync_transform_to_physics_system(world)` 在 PreUpdate 阶段运行。
435. `step_physics_system(world)` 在 Update 阶段运行。
436. `sync_physics_to_transform_system(world)` 在 PostUpdate 阶段运行。
437. `contact_event_system(world)` 把物理事件转发到 ECS Event。
438. `PhysicsConfig` 资源：`gravity / default_friction / default_restitution / max_substeps / ccd / fixed_dt / debug_draw_enabled`。
439. `PhysicsStats` 资源：每帧更新 `step_time_ms / bodies / colliders / joints`。
440. `PhysicsQuery` SystemParam：`fn system(q: PhysicsQuery)` 查询 world。

### 3.8 编辑器集成

441. Inspector 中 `RigidBodyComponent`：显示类型 / 质量 / linvel / angvel / ccd / 编辑 position/rotation。
442. Inspector 中 `ColliderComponent`：显示形状类型 / friction / restitution / sensor / aabb。
443. Inspector 中 `JointComponent`：显示 joint 类型与锚点。
444. Inspector 中 `CharacterController3D`：显示 slope / slide / offset / grounded。
445. 编辑器菜单「Physics > Toggle Debug Draw」。
446. 编辑器菜单「Physics > Pause/Step」。
447. 编辑器集成显示 PhysicsStats（FPS 面板扩展）。
448. 编辑器支持在 3D 视图中点击 object 并高亮其 collider。
449. 编辑器支持 drag 调整 collider 参数，实时反馈。
450. 编辑器支持快速创建 Box Collider / Sphere Collider。

### 3.9 示例与测试

451. `examples/physics_3d_basic`：盒子下落到地面。
452. `examples/physics_3d_stack`：堆叠盒子。
453. `examples/physics_3d_ragdoll`：铰接 ragdoll。
454. `examples/physics_3d_character`：角色控制器。
455. `examples/physics_3d_joints`：钟摆与弹簧。
456. `examples/physics_3d_ray_cast`：点击发射射线，选中物体。
457. `examples/physics_3d_trigger`：sensor 进入/离开事件。
458. `examples/physics_3d_heightfield`：高度场地形。
459. `examples/physics_3d_compound`：复合碰撞体。
460. `examples/physics_3d_billiard`：撞球（可选）。
461. 单测：重力会改变 linvel。
462. 单测：动态球落至静态平面。
463. 单测：`apply_impulse` 改变 linvel。
464. 单测：`CollisionGroups` 过滤命中。
465. 单测：Sensor collider 触发 intersection_event。
466. 单测：Joint 两体距离被限制。
467. 单测：CharacterController 在斜坡站立。
468. 单测：`step(dt)` 为 0 不崩溃。
469. 单测：高速小物体启用 CCD 命中薄物体。
470. 单测：AABB 变换正确。
471. 单测：`Ray3` 平行于平面时返回 `None` 不崩溃。
472. `cargo test -p engine-physics-3d` 全部通过。
473. `cargo clippy --workspace -- -D warnings` 通过。
474. `cargo fmt --check --workspace` 通过。
475. `cargo doc --workspace --no-deps` 成功。
476. CI 三平台 green。
477. CHANGELOG 记录 0.11.0。
478. README.md 加入「3D 物理引擎」章节。
479. README.md 加入「角色控制器」章节。
480. README.md 加入「物理查询系统」章节。
481. 公开 API doc comment 覆盖率 100%。
482. `unsafe` 块 <= 5。
483. 新增 example 工程 >= 7 个。
484. `examples/physics_3d_stack` 100 盒子稳定 >= 60fps。
485. 调试可视化可在编辑器中切换。
486. 与 ECS 系统双向同步正常。

---

## 四、验收标准

- [ ] `cargo run --example physics_3d_basic` 正常模拟
- [ ] `cargo run --example physics_3d_stack` 堆叠稳定
- [ ] `cargo run --example physics_3d_ragdoll` 铰接 ragdoll 正常
- [ ] `cargo run --example physics_3d_character` 角色可移动/跳跃
- [ ] `cargo run --example physics_3d_ray_cast` 点击命中物体高亮
- [ ] `cargo run --example physics_3d_trigger` 触发事件正确
- [ ] `cargo test -p engine-physics-3d` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.11.0

---

## 五、下一个 Sprint

Sprint 12 将引入完整的动画系统（关键帧动画、骨骼动画、状态机、混合树、动画事件）。
