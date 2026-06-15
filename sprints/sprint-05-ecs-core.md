# Sprint 05 · ECS 实体组件系统内核

> 阶段：阶段二 · 编辑器 + 跨平台（第 1 个 Sprint）  
> 周期：4 周  
> 核心目标：实现完整 ECS 内核（World / Entity / Component / System / Query / Resources / Events）  
> 验收：10 万实体 Demo 正常运行；`cargo bench` 显示 Query/迭代性能达标

---

## 一、Sprint 概览

ECS 是引擎性能的关键骨架。本 Sprint 建立 `engine-ecs` crate，提供数据导向架构。核心交付：

- `World`：组件表 + 资源表 + 实体表
- `Entity` / `EntityId`（索引+代际）
- `Component` trait + `ComponentStorage`（Vec/SparseSet/DenseMap）
- `Query<'w, &A, &mut B, Without<C>>` 迭代器
- `System` / `SystemParam` / `SystemStage` / `Schedule`
- `Resource<T>` / `Event<T>` / `EventReader` / `EventWriter`
- `Bundle`：批量 spawn 组件包
- `Commands`：延迟命令队列（spawn / insert / remove / despawn）
- `SystemParamFunction`：`Res / ResMut / Query / EventReader / Local`
- `ParallelIterator`：多线程系统执行
- `examples/ecs_100k`：10 万粒子在 ECS 下稳定 60fps

---

## 二、项目需求清单

1. `engine-ecs` crate 建立。
2. `World::new()`。
3. `World::spawn() -> Entity`。
4. `World::spawn_bundle(bundle) -> Entity`。
5. `World::spawn_batch(bundles)` — 批量生成。
6. `World::despawn(entity)`。
7. `World::clear_entities(&mut self)`。
8. `World::insert(entity, component)`。
9. `World::insert_bundle(entity, bundle)`。
10. `World::remove::<C>(entity)`。
11. `World::remove_bundle::<Bundle>(entity)`。
12. `World::entity(entity) -> EntityRef`。
13. `World::entity_mut(entity) -> EntityMut`。
14. `World::contains(entity) -> bool`。
15. `World::get_component::<C>(entity) -> Option<&C>`。
16. `World::get_component_mut::<C>(entity) -> Option<&mut C>`。
17. `World::entities(&self) -> &Entities`。
18. `World::components(&self) -> &Components`。
19. `World::resource::<R>() -> &R`。
20. `World::resource_mut::<R>() -> &mut R`。
21. `World::insert_resource(resource)`。
22. `World::remove_resource::<R>()`。
23. `World::contains_resource::<R>() -> bool`。
24. `World::send_event::<E>(event)`。
25. `World::events::<E>() -> EventReader<E>`。
26. `World::run_system(system_fn)`。
27. `World::run_system_catched(system_fn) -> Result<()>`。
28. `World::schedule(name) -> &mut Schedule`。
29. `World::add_system_to_stage(stage, system)`。
30. `World::add_system_set(set)`。
31. `World::insert_resource(resource)`。
32. `World::add_stage_after(existing, name, stage)`。
33. `World::add_stage_before(existing, name, stage)`。
34. `Entity` 结构体：`id: u32 + generation: u32`。
35. `Entity::id(&self) -> u32`。
36. `Entity::generation(&self) -> u32`。
37. `Entity::null() -> Entity`。
38. `Entity::is_null(&self) -> bool`。
39. `Component` trait：`type Storage = SparseSet<C>`。
40. `Component` 派生宏 `#[derive(Component)]`。
41. `ComponentStorage<C>` 抽象 trait。
42. `SparseSet<T>`：稀疏索引 + 密集数组。
43. `DenseVec<T>`：索引即数组下标。
44. `HashMapStorage<T>`：散列存储稀疏组件。
45. `Bundle` trait + `#[derive(Bundle)]` 宏。
46. `Bundle::bundle_components(&self, func)`。
47. `Bundle::from_components(func) -> Self`。
48. `Bundle::bundle_id() -> BundleId`。
49. `Query<'w, Q, F>`：泛型查询。
50. `Query::iter(&self)` — 只读迭代。
51. `Query::iter_mut(&mut self)` — 可写迭代。
52. `Query::get(&self, entity)` — 单实体查询。
53. `Query::get_mut(&mut self, entity)` — 单实体可变查询。
54. `Query::single(&self)` — 期望只有一个。
55. `Query::single_mut(&mut self)`。
56. `Query::is_empty(&self) -> bool`。
57. `Query::len(&self) -> usize`。
58. `Query::for_each(func)`。
59. `Query::par_for_each(batch_size, func)` — 并行。
60. `QueryState<Q, F>`：可缓存的查询状态。
61. `QueryFilter`：`With / Without / Added / Changed / Mutated`。
62. `Or<(A, B, C)>` / `And<(A, B, C)>` 组合过滤。
63. `Changed<T>` 过滤组件变化。
64. `Added<T>` 过滤组件被添加。
65. `RemovedComponents<T>` 迭代被移除的组件。
66. `SystemParam` trait：`Res / ResMut / Query / EventReader / EventWriter / Local / Commands`。
67. `System` trait：`run(&mut self, world)`。
68. `System::name(&self) -> &str`。
69. `System::is_exclusive(&self) -> bool`。
70. `IntoSystem` 转换：`fn system(...) -> impl IntoSystem`。
71. `SystemStage::single_threaded()` — 单线程调度。
72. `SystemStage::parallel()` — 多线程调度（基于系统依赖关系图）。
73. `Schedule::new() / add_stage / run(world)`。
74. `Stage` trait：`run(&mut self, world)`。
75. `SystemSet`：一组系统集合，可整体启用/禁用。
76. `RunCriteria`：基于条件决定是否执行阶段。
77. `Label`：阶段/系统标签。
78. `Commands`：延迟命令队列。
79. `Commands::spawn(bundle) -> EntityCommands`。
80. `Commands::spawn_batch(bundles)`。
81. `Commands::insert(entity, bundle)`。
82. `Commands::remove::<C>(entity)`。
83. `Commands::despawn(entity)`。
84. `Commands::insert_resource(resource)`。
85. `Commands::remove_resource::<R>()`。
86. `Commands::add(command)` — 自定义命令。
87. `Commands::apply(world)` — 应用到 world。
88. `EntityCommands`：insert / remove / despawn / insert_resource / remove_resource / id()。
89. `Resource<T>`：全局资源（单件）。
90. `Resource` trait（可用 `#[derive(Resource)]`）。
91. `Event<T>`：事件队列。
92. `EventReader<T>::iter(&self)` 读取事件。
93. `EventWriter<T>::send(event)` 发送事件。
94. `EventWriter<T>::send_batch(events)` 批量发送。
95. `Events<T>::update(&mut self)` — 旧事件清理（双缓冲）。
96. `Local<T>`：系统本地状态。
97. `ParamSet`：同一系统中同时存在多个冲突 Query。
98. `Archetype`：相同组件集合的实体分组。
99. `Archetype::id(&self) -> ArchetypeId`。
100. `Archetype::entities(&self) -> &[Entity]`。
101. `Archetype::component_ids(&self) -> &[ComponentId]`。
102. `Archetype::get::<C>(&self) -> Option<&[C]>`。
103. `ArchetypeGraph`：archetype 迁移边。
104. `ChangeTrackers`：检测组件变更。
105. `Tick`：帧计数代际。
106. `Ticks<T>`：`added / changed / last_changed`。
107. `Ref<T>`：组件引用（含变更检测）。
108. `Reflect`：动态反射（trait 定义，后续实现）。
109. `Name` 组件：便于调试。
110. `Hierarchy`：`Parent / Children` 组件（父子关系）。
111. `Parent(entity)` 组件。
112. `Children(Vec<Entity>)` 组件。
113. `BuildChildren` trait：`push_children / insert_children / remove_children`。
114. `WorldQuery` trait：`&T / &mut T / Option<&T> / Changed<T> / Added<T> / Without<T> / With<T>`。
115. `WorldQuery::ReadOnly` 标记。
116. `QueryBorrowState<Q>`：查询借用状态跟踪。
117. `BorrowError`：多次可变借用错误。
118. `ExclusiveSystem`：直接获取 `&mut World`。
119. `NonSend`：不在线程间发送的资源标记。
120. `NonSendMut`。
121. `SystemLabel / SystemSet / RunCriteriaLabel` 标记。
122. `examples/ecs_hello` — 最小 ECS：spawn + query + update。
123. `examples/ecs_100k` — 10 万粒子 + 位置/速度更新 + 绘制。
124. `examples/ecs_events` — 事件读写。
125. `examples/ecs_hierarchy` — 父子层级与 transform 传播。
126. `examples/ecs_parallel` — 并行系统对比单线程性能。
127. `examples/ecs_commands` — Commands 延迟命令。
128. `examples/ecs_change_tracking` — Changed/Added 过滤。
129. `examples/ecs_resources` — 资源单件。
130. `examples/ecs_bundle` — Bundle 简化 spawn。
131. `examples/ecs_schedule` — 多阶段 Schedule。
132. `examples/ecs_ray_cast` — ECS + 物理 raycast。
133. `examples/ecs_pong` — ECS 简化 pong。
134. `criterion` 基准：`Query::iter 100k` / `Query::par_for_each 100k` / `spawn_batch 100k` / `insert bundle`。
135. 文档章节：ECS 入门 / 组件 / 系统 / 查询 / 资源 / 事件 / Commands / Bundle / Schedule / 并行 / 性能提示。
136. `World::dump_stats(&self)` — 打印 archetype 信息。
137. `World::validate(&self) -> Result<()>` — 基本一致性校验。
138. `#[derive(Component)]` 宏定义（使用 proc-macro）。
139. `#[derive(Bundle)]` 宏。
140. `#[derive(Resource)]` 宏。
141. `#[derive(Event)]` 宏。
142. `#[derive(SystemLabel)]` 宏。
143. `#[derive(StageLabel)]` 宏。
144. `#[derive(SystemSet)]` 宏。
145. `#[derive(RunCriteriaLabel)]` 宏。
146. 单元测试：World spawn/despawn 不泄漏。
147. 单元测试：Query With/Without 过滤正确。
148. 单元测试：Changed/Added 正确。
149. 单元测试：Bundle 往返。
150. 单元测试：Event 双缓冲清理。
151. 单元测试：Commands 应用正确。
152. 单元测试：Hierarchy Children 跟随 Parent。
153. 单元测试：Schedule 阶段顺序正确。
154. 单元测试：System 并行无数据竞争。
155. 单元测试：Query::single 在有多个实体时 panic。
156. 单元测试：Query::iter_mut 不与其他借用冲突。
157. 单测：`SparseSet` 插入/删除/查找。
158. 单测：`DenseVec` 索引。
159. 单测：`Archetype` 组件数组对齐正确。
160. `cargo test -p engine-ecs 全部通过。
161. `cargo clippy --workspace -- -D warnings 通过。
162. `cargo fmt --check --workspace 通过。
163. `cargo doc --workspace --no-deps 成功。
164. CI 三平台 green。
165. CHANGELOG 记录版本 0.5.0。
166. README.md 加入「ECS 系统」章节。
167. 公开 API 数量 <= 100。
168. 公开 API doc comment 覆盖率 100%。
169. 本 Sprint `unsafe` 块数量 <= 3。
170. Benchmark：10 万实体移动 + 绘制 >= 60fps。

> 以上 170 条需求构成 Sprint 05 全量清单。

---

## 三、细分需求与验收

### 3.1 World / Entity

171. `World::new()` 创建空世界。
172. `World::spawn()` 生成唯一 Entity。
173. `World::spawn_bundle()` 在一次调用中插入多个组件。
174. `World::spawn_batch()` 批量 spawn，性能符合 benchmark。
175. `World::despawn()` 回收实体并清理组件。
176. `World::despawn()` 在空实体上安全调用。
177. `World::contains()` 正确反映存活状态。
178. `World::clear_entities()` 清空世界并可继续工作。
179. `Entity::id()` 唯一且稳定。
180. `Entity::generation()` 防止 ABA 问题。
181. `Entity` 实现 `Copy + Eq + Hash + Send + Sync`。
182. `Entity` 可作为 HashMap 键。

### 3.2 Component / Storage / Archetype

183. `Component` trait 提供 `type Storage` 关联类型。
184. `#[derive(Component)]` 默认使用 `SparseSet`。
185. `#[component(storage = "DenseVec")]` 支持切换。
186. `SparseSet::insert(entity, value)`。
187. `SparseSet::get(entity)`。
188. `SparseSet::remove(entity)`。
189. `SparseSet::iter()`。
190. `SparseSet::iter_mut()`。
191. `SparseSet::contains(entity)`。
192. `SparseSet::len()`。
193. `DenseVec` 同上一套。
194. `HashMapStorage` 同上一套。
195. `Archetype` 按组件集合自动创建。
196. `Archetype` 组件按 ComponentId 排序。
197. `Archetype` 内部组件数组 `SoA` 对齐。
198. 实体 spawn 时选择正确 archetype。
199. insert 新组件时迁移到新 archetype。
200. remove 组件时迁移到新 archetype。
201. `ArchetypeGraph` 缓存迁移路径。

### 3.3 Query / Filter

202. `Query<(&A, &mut B)>` 合法 borrow。
203. `Query<(&mut A, &mut B)>` 合法 borrow。
204. `Query<(&A, &A)>` 非法 -> 编译期错误。
205. `Query<(&mut A, &mut A)>` 非法 -> 编译期错误。
206. `Query` 支持 `With<T>` 过滤。
207. `Query` 支持 `Without<T>` 过滤。
208. `Query` 支持 `Added<T>` 过滤。
209. `Query` 支持 `Changed<T>` 过滤。
210. `Query` 支持 `Or<(A, B)>` 组合过滤。
211. `Query::iter` 返回正确个数。
212. `Query::iter_mut` 返回正确个数。
213. `Query::get` 未找到时返回 `None`。
214. `Query::get_mut` 未找到时返回 `None`。
215. `Query::single` 在 0 或 >1 时 panic。
216. `QueryState` 缓存查询状态，连续调用快速。
217. `Query::par_for_each` 拆分任务到线程池。
218. `Query::par_for_each` 线程安全：无 data race。

### 3.4 System / Schedule

219. `fn system(query: Query<(&mut Pos, &Vel)>)` 合法定义。
220. `fn system(res: Res<Time>)` 合法定义。
221. `fn system(mut res: ResMut<Score>)` 合法定义。
222. `fn system(ev: EventReader<E>)` 合法定义。
223. `fn system(mut ev: EventWriter<E>)` 合法定义。
224. `fn system(mut cmds: Commands)` 合法定义。
225. `fn system(local: Local<u32>)` 合法定义。
226. `fn exclusive(world: &mut World)` 合法定义（ExclusiveSystem）。
227. `SystemStage::parallel()` 按资源访问图并行化。
228. `Schedule` 按 stage 顺序执行。
229. `Schedule::run(world)` 不崩溃。
230. `SystemSet` 可整体 disable。
231. `RunCriteria::Paused` 可暂停阶段。

### 3.5 Commands / Resources / Events

232. `Commands::spawn` 在 apply 后才实际插入世界。
233. `Commands::insert` 在 apply 后才实际插入组件。
234. `Commands::remove` 在 apply 后才实际移除组件。
235. `Commands::despawn` 在 apply 后才实际移除实体。
236. `Commands::insert_resource` / `remove_resource` 延迟生效。
237. `World::resource::<Time>()` 不可用资源时 panic。
238. `World::get_resource::<R>()` 返回 Option。
239. `Events<T>` 双缓冲：`update()` 丢弃上一帧之前的事件。
240. `EventReader` `iter` 仅返回新事件。
241. `EventReader` 支持多次读取同一条事件（多 reader 独立）。
242. `EventWriter` `send` 发送立即入队。
243. `EventWriter` `send_batch` 批量。
244. `Local<T>` 默认值 T: Default。
245. `Local<T>::default` 支持 `#[local]` 自定义初始值。

### 3.6 Hierarchy / Transform

246. `Parent` 组件指向父实体。
247. `Children(Vec<Entity>)` 反向索引。
248. `push_child(parent, child)` 建立双向关系。
249. `remove_child(parent, child)` 解除双向关系。
250. `despawn_recursive(entity)` 递归销毁子树。
251. `Transform2D`（位置/旋转/缩放）组件。
252. `GlobalTransform2D` 组件。
253. `transform_propagate_system` 计算 GlobalTransform2D。
254. Transform 传播在 PreUpdate 阶段执行。

### 3.7 变更检测 / Tick

255. `Tick` 每帧递增。
256. `Ref<T>` 暴露 `is_added()` / `is_changed()`。
257. `Changed<T>` 过滤仅匹配本帧变化的。
258. `Added<T>` 过滤仅匹配本帧新增的。
259. `RemovedComponents<T>` 迭代被移除的实体。
260. `World::clear_trackers(&mut self)` 手动重置。

### 3.8 示例 / Bench

261. `examples/ecs_hello` 打印 10 个实体位置。
262. `examples/ecs_100k` 10 万粒子移动 + 绘制稳定 >= 60fps。
263. `examples/ecs_events` 键盘事件触发得分变化。
264. `examples/ecs_hierarchy` 父子 Transform 传播。
265. `examples/ecs_parallel` 并行系统比单线程快 >= 1.5x。
266. `examples/ecs_commands` Commands 演示。
267. `examples/ecs_change_tracking` Changed/Added 过滤。
268. `examples/ecs_resources` 资源单件。
269. `examples/ecs_bundle` Bundle 简化 spawn。
270. `examples/ecs_schedule` 多阶段 Schedule。
271. `examples/ecs_ray_cast` 射线检测 ECS。
272. `examples/ecs_pong` 简化 pong 游戏。
273. criterion bench `ecs_query_iter_100k`。
274. criterion bench `ecs_query_par_100k`。
275. criterion bench `ecs_spawn_100k`。
276. criterion bench `ecs_insert_bundle`。
277. `cargo bench` 可运行。

### 3.9 测试

278. 单元测试 `World::spawn / despawn`。
279. 单元测试 `Query` With/Without。
280. 单元测试 `Changed/Added`。
281. 单元测试 `Bundle`。
282. 单元测试 `Events`。
283. 单元测试 `Commands`。
284. 单元测试 `Hierarchy`。
285. 单元测试 `Schedule`。
286. 单元测试 `Parallel System`。
287. 单元测试 `Query::single`。
288. 单元测试 `Query::iter_mut`。
289. 单元测试 `SparseSet`。
290. 单元测试 `DenseVec`。
291. 单元测试 `Archetype` 迁移。
292. `cargo test --workspace` 全部通过。
293. `cargo fmt --check --workspace 通过。
294. `cargo clippy --workspace -- -D warnings 通过。
295. `cargo doc --workspace --no-deps 成功。
296. CI 三平台 green。
297. CHANGELOG 记录 0.5.0。
298. README.md 加入「ECS 系统」章节。
299. 公开 API doc comment 覆盖率 100%。
300. `unsafe` 块 <= 3。

---

## 四、验收标准

- [ ] `cargo run --example ecs_100k` 10 万粒子稳定运行
- [ ] `cargo run --example ecs_pong` 可玩
- [ ] `cargo test -p engine-ecs` 全部通过
- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.5.0

---

## 五、下一个 Sprint

Sprint 06 将在 ECS 之上构建 UI 控件库与布局引擎（UI 系统）。
