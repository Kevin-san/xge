# Sprint 07 · 可视化编辑器基础框架

> 阶段：阶段二 · 编辑器 + 跨平台（第 3 个 Sprint）  
> 周期：4 周  
> 核心目标：在引擎内部提供可视化编辑器 UI（场景视图 / 层级 / 属性 / 资源 / 控制台 / 动画预览 / 调试面板）  
> 验收：可拖拽创建节点、编辑属性、保存/加载场景、插件系统雏形

---

## 一、Sprint 概览

本 Sprint 建立 `engine-editor` crate。核心交付：

- 编辑器主窗口布局（Dock 式）
- `SceneView`：2D / 3D 场景渲染 + 交互（选择/平移/旋转/缩放）
- `HierarchyPanel`：场景树视图，选中/拖放/重命名
- `InspectorPanel`：实体/组件属性编辑面板（数字/文本/颜色/纹理/对象引用）
- `AssetPanel`：资源浏览器（文件树 + 缩略图）
- `ConsolePanel`：日志与错误面板
- `AnimationPreviewPanel`：简单时间轴动画预览（下一阶段完善）
- `DebugPanel`：性能/ECS 统计信息
- `EditorCommands`：撤销/重做（undo/redo）
- `EditorAction`：action 系统
- `EditorPrefab`：编辑器侧预制体编辑
- `EditorSettings`：编辑器配置（主题/键位/外部工具路径）
- `examples/editor_app`：运行可直接弹出编辑器

---

## 二、项目需求清单

1. `engine-editor` crate 建立。
2. `EditorApp` 结构体：持有引擎、当前场景、选择集。
3. `EditorApp::run()`：启动编辑器主循环。
4. 编辑器 Docking 布局：顶部菜单、左侧场景树 / 资源、中间场景视图、右侧属性、底部控制台。
5. 顶部菜单：File / Edit / View / Project / Build / Tools / Help。
6. File：New Scene / Open / Save / Save As / Import Asset / Export / Exit。
7. Edit：Undo / Redo / Cut / Copy / Paste / Delete / Duplicate / Select All / Preferences。
8. View：Toggle 各面板显示、重置布局、全屏。
9. Project：Project Settings / Build Settings / Plugins。
10. Build：Run / Build Windows / Build macOS / Build Linux / Build Android / Build iOS / Build Web / Rebuild。
11. Tools：Options 调试选项 / 性能分析器 / 插件管理器。
12. Help：About / Documentation / Report Bug。
13. 键盘快捷键：Ctrl+S 保存 / Ctrl+Z 撤销 / Ctrl+Y 重做 / Ctrl+N 新场景 / Ctrl+O 打开 / F2 重命名 / Del 删除。
14. Dock 布局可拖拽重新布局（简化版）。
15. Dock 布局可重置为默认。
16. `SceneView` 面板：场景绘制窗口。
17. `SceneView`：显示网格（2D / 3D）。
18. `SceneView`：显示参考线。
19. `SceneView`：显示 gizmos（位置箭头 / 旋转 / 缩放手柄）。
20. `SceneView`：鼠标选择实体（点击命中测试）。
21. `SceneView`：框选（Lasso/Rectangle）。
22. `SceneView`：多选（按住 Shift/Click 或 Ctrl/Click）。
23. `SceneView`：移动/旋转/缩放工具（W/E/R 键切换）。
24. `SceneView`：场景平移中键拖拽 / 右键旋转 / 滚轮缩放。
25. `SceneView`：像素对齐（2D 场景）。
26. `SceneView`：正交 / 透视切换。
27. `SceneView`：2D / 3D 模式切换。
28. `SceneView`：Play 按钮（运行场景）/ Stop / Step。
29. `SceneView`：运行时不可编辑（只读视图）。
30. `HierarchyPanel`：显示场景树（Node 或 Entity）。
31. `HierarchyPanel`：可展开/折叠节点。
32. `HierarchyPanel`：点击选择节点；双击重命名。
33. `HierarchyPanel`：右键菜单（添加子节点/重命名/删除/复制/粘贴/保存为 Prefab）。
34. `HierarchyPanel`：拖拽改变父子关系。
35. `HierarchyPanel`：支持搜索过滤（按名称/类型）。
36. `InspectorPanel`：显示当前选中节点的组件列表。
37. `InspectorPanel`：每个组件显示为可折叠 section。
38. `InspectorPanel`：可编辑基础类型：i32 / f32 / f64 / u32 / bool / String / Vec2 / Vec3 / Vec4 / Mat4 / Color / Rect。
39. `InspectorPanel`：可编辑资源引用（Texture / Material / Prefab / Scene / Font）。
40. `InspectorPanel`：资源引用下拉选择/拖拽引用。
41. `InspectorPanel`：组件新增/删除菜单（按类型）。
42. `InspectorPanel`：禁用/启用组件。
43. `InspectorPanel`：实时显示组件变化（ECS 变更检测）。
44. `InspectorPanel`：批量编辑多选节点的公共属性。
45. `AssetPanel`：左侧项目资源文件树（基于 `assets/` 目录）。
46. `AssetPanel`：右侧资源网格视图（缩略图）。
47. `AssetPanel`：支持拖拽资源到场景视图创建节点。
48. `AssetPanel`：支持双击打开（场景文件在编辑器中切换）。
49. `AssetPanel`：右键菜单（新建文件夹/删除/重命名/导入/导出）。
50. `AssetPanel`：支持常见扩展名过滤（png/jpg/ttf/otf/fbx/glb/json/bin）。
51. `AssetPanel`：显示资源大小、修改时间。
52. `AssetPanel`：搜索框按名称过滤。
53. `AssetPanel`：支持创建资源元数据 `.meta`。
54. `ConsolePanel`：日志分级显示（Info / Warn / Error / Debug）。
55. `ConsolePanel`：过滤级别切换。
56. `ConsolePanel`：过滤关键字。
57. `ConsolePanel`：清空按钮。
58. `ConsolePanel`：点击日志显示详情（堆栈、文件、行号）。
59. `ConsolePanel`：可复制。
60. `ConsolePanel`：支持颜色标记（Info = 绿 / Warn = 黄 / Error = 红）。
61. `AnimationPreviewPanel`：时间轴显示（初版）。
62. `AnimationPreviewPanel`：播放/暂停/停止/循环。
63. `AnimationPreviewPanel`：关键帧列表显示。
64. `AnimationPreviewPanel`：缩放时间轴。
65. `DebugPanel`：FPS / FrameTime / DrawCalls / Vertices / Entities / Components / Memory 统计。
66. `DebugPanel`：性能图表（折线图）。
67. `DebugPanel`：ECS archetype 统计（与 Sprint 05 的 `World::dump_stats` 联动）。
68. `DebugPanel`：内存使用（系统资源 / GPU 资源）。
69. `EditorSettings`：编辑器配置。
70. `EditorSettings`：主题（Light / Dark / Custom）。
71. `EditorSettings`：键位（Key Bindings）。
72. `EditorSettings`：字体字号。
73. `EditorSettings`：外部工具路径（VS Code / git / cargo）。
74. `EditorSettings`：默认创建路径。
75. `EditorSettings`：自动保存（On/Off + 时间间隔）。
76. `EditorSettings`：可保存到 `editor_settings.json`。
77. `EditorSettings`：可加载。
78. `EditorSettings` UI：可配置界面。
79. `EditorAction`：Action 系统（抽象操作：CreateEntity / Delete / SetProperty / ...）。
80. `EditorActionStack`：undo/redo 栈。
81. `EditorAction` 可序列化，便于回放/录制。
82. `EditorSelection`：选择集（多选 entity / node）。
83. `EditorSelection::clear()` / `select(entity)` / `toggle(entity)` / `contains(entity)` / `iter()` / `len()`.
84. `EditorSelectionChanged` 事件。
85. `EditorClipboard`：复制/粘贴节点 / 组件 / 资源路径。
86. `EditorGizmos`：绘制辅助图形（箭头 / 网格 / 选择框 / 工具手柄）。
87. `Gizmo2d`：线 / 圆 / 矩形 / 文本 / 箭头。
88. `Gizmo3d`：变换手柄、相机视锥体、光照 gizmo。
89. `EditorTools`：Select / Move / Rotate / Scale 工具切换。
90. `Snap`：像素吸附 / 网格吸附 / 旋转吸附（15° 等）。
91. `EditorPlugin` trait：第三方扩展可接入编辑器。
92. `EditorPlugin::register(&mut editor)` 注册菜单 / 面板 / 自定义窗口。
93. `EditorPlugin::update(&mut self, editor, dt)` 插件更新。
94. `EditorPluginRegistry`：插件管理器。
95. `EditorMode`：EditMode / PlayMode / PausedMode。
96. `EditorMode` 切换按钮在工具栏。
97. PlayMode 下禁止编辑，但可以修改值观察效果，退出时还原。
98. `SceneSaver::save(scene, path)` 保存场景为 JSON 或二进制。
99. `SceneLoader::load(path) -> Scene` 从文件加载场景。
100. 场景文件格式：`*.scene.json` / `*.scene.bin`。
101. `PrefabSaver::save(prefab, path)`。
102. `PrefabLoader::load(path) -> Prefab`。
103. `AssetMeta`：资源元数据（importer 设置 / GUID）。
104. `AssetImporter`：PNG/JPG/TTF/FBX/GLB/自定义。
105. `AssetPipeline::import_all()` 扫描资源并导入。
106. `AssetPipeline::reimport_changed()` 增量导入。
107. 缩略图生成：为模型/场景生成缩略图（初版占位，后续完善）。
108. `EditorUndo`：至少保存 50 步 undo。
109. `EditorUndo`：保存/加载时清空。
110. 编辑器 UI 主题：默认 Dark 主题。
111. 编辑器 UI 主题：Light 主题。
112. 编辑器 UI 可访问性：Tab 顺序 / 键盘导航。
113. 编辑器 UI 性能：每帧增量刷新，不重建全部 UI。
114. 编辑器 UI 国际化：支持多语言（初版中文/英文）。
115. 编辑器帮助菜单：文档链接。
116. 编辑器关于对话框：引擎版本、构建信息、依赖列表。
117. `examples/editor_app` — 启动编辑器示例。
118. `examples/editor_custom_panel` — 自定义面板示例。
119. `examples/editor_plugin` — 自定义插件示例。
120. `examples/editor_game` — 使用编辑器开发简单游戏示例。
121. 单测：`EditorActionStack` undo/redo 正确性。
122. 单测：`EditorSelection` 行为。
123. 单测：`SceneSaver/Loader` 往返。
124. 单测：`Prefab` 保存/加载。
125. 单测：`Gizmo` 绘制 API。
126. 单测：`AssetPipeline` 导入流程。
127. `cargo test -p engine-editor` 全部通过。
128. `cargo clippy --workspace -- -D warnings` 通过。
129. `cargo fmt --check --workspace` 通过。
130. `cargo doc --workspace --no-deps` 成功。
131. CI 三平台 green。
132. CHANGELOG 记录版本 0.7.0。
133. README.md 加入「可视化编辑器」章节。
134. README.md 加入「编辑器使用指南」章节。
135. 公开 API doc comment 覆盖率 100%。
136. 本 Sprint `unsafe` 块 <= 5。
137. 新增 example 工程 >= 4 个。
138. 编辑器至少支持 10 个基础组件属性编辑。
139. 编辑器至少支持 2 种资源类型（PNG 与场景）。
140. 编辑器 2D 场景下基本可用。

> 以上 140 条需求构成 Sprint 07 全量清单。

---

## 三、细分需求与验收

### 3.1 编辑器核心

141. `EditorApp::new(window, engine)` 构造。
142. `EditorApp::run(&mut self)` 启动。
143. `EditorApp::update(&mut self, dt)`。
144. `EditorApp::render(&mut self)`。
145. `EditorApp::handle_event(&mut self, event)`。
146. `EditorApp::mode(&self) -> EditorMode`。
147. `EditorApp::set_mode(&mut self, mode)`。
148. `EditorApp::selection(&self) -> &EditorSelection`。
149. `EditorApp::selection_mut(&mut self) -> &mut EditorSelection`。
150. `EditorApp::action_stack(&self) -> &EditorActionStack`。
151. `EditorApp::action_stack_mut(&mut self) -> &mut EditorActionStack`。
152. `EditorApp::scene(&self) -> &SceneTree`。
153. `EditorApp::scene_mut(&mut self) -> &mut SceneTree`。
154. `EditorApp::new_scene(&mut self)`。
155. `EditorApp::save_scene(&self, path)`。
156. `EditorApp::load_scene(&mut self, path)`。
157. `EditorApp::settings(&self) -> &EditorSettings`。
158. `EditorApp::settings_mut(&mut self) -> &mut EditorSettings`。
159. `EditorApp::save_settings(&self, path)`。
160. `EditorApp::load_settings(&mut self, path)`。
161. `EditorApp::open_menu(&mut self, menu_name)`。
162. `EditorApp::show_panel(&mut self, panel_id)`。
163. `EditorApp::hide_panel(&mut self, panel_id)`。
164. `EditorApp::toggle_panel(&mut self, panel_id)`。
165. `EditorApp::reset_layout(&mut self)`。
166. `EditorApp::register_plugin<P: EditorPlugin>(&mut self, plugin)`。
167. `EditorMode::Edit / Play / Paused`。
168. `EditorState` 结构体：场景、选择、撤销栈、剪贴板、设置、插件。
169. `EditorState::tick(&mut self, dt)`。

### 3.2 面板与 UI

170. `Panel` trait：`title(&self)` / `ui(&mut self, editor, ui)`。
171. `HierarchyPanel` 显示场景树。
172. `HierarchyPanel` 节点显示图标（sprite / model / camera）。
173. `HierarchyPanel` 节点可拖放。
174. `HierarchyPanel` 右键菜单。
175. `HierarchyPanel` 搜索框。
176. `InspectorPanel` 显示 entity 的组件。
177. `InspectorPanel` 组件 section 折叠。
178. `InspectorPanel` 基础类型字段 UI：`label_for(value)` / `drag_value(value)` / `color_picker(value)` / `slider(value, min, max)`。
179. `InspectorPanel` 字符串字段：单行输入框。
180. `InspectorPanel` bool 字段：checkbox。
181. `InspectorPanel` 资源引用：下拉 + 拖拽。
182. `InspectorPanel` 对象引用（entity ref）：选择。
183. `InspectorPanel` 组合字段：`Vec2/3/4` 显示 x/y/z/w。
184. `InspectorPanel` 颜色：RGBA + 颜色选择器。
185. `InspectorPanel` 枚举：下拉选择。
186. `InspectorPanel` 数组：可展开列表。
187. `InspectorPanel` 结构体：递归 section。
188. `InspectorPanel` Add Component 按钮。
189. `InspectorPanel` Remove Component 按钮。
190. `InspectorPanel` Enable/Disable Component 开关。
191. `AssetPanel` 左侧目录树。
192. `AssetPanel` 右侧网格。
193. `AssetPanel` 文件过滤器。
194. `AssetPanel` 右键菜单。
195. `AssetPanel` 缩略图（初版占位）。
196. `AssetPanel` 支持拖动到场景。
197. `ConsolePanel` 日志显示行。
198. `ConsolePanel` 级别过滤。
199. `ConsolePanel` 搜索过滤。
200. `ConsolePanel` 行点击详情。
201. `AnimationPreviewPanel` 时间轴 UI。
202. `AnimationPreviewPanel` 播放控件。
203. `AnimationPreviewPanel` 缩放时间轴。
204. `DebugPanel` FPS / FrameTime 折线图（简易实现）。
205. `DebugPanel` 性能统计。
206. `DebugPanel` ECS 统计。
207. `DebugPanel` GPU 内存（估算）。
208. `EditorSettingsPanel` 主题切换 UI。
209. `EditorSettingsPanel` 键位配置 UI。
210. `EditorSettingsPanel` 自动保存设置。

### 3.3 场景视图与交互

211. `SceneView::ui(&mut self, editor, ui)`。
212. `SceneView::render(&self, engine, renderer)`。
213. `SceneView::handle_mouse(&mut self, editor, event)`。
214. `SceneView::draw_gizmos(&self, gizmos)`。
215. `SceneView::hit_test(&self, pos) -> Option<Entity>`。
216. `SceneView::tool(&self) -> EditorTool`。
217. `SceneView::set_tool(&mut self, tool)`。
218. `SceneView::snap_enabled(&self) -> bool`。
219. `SceneView::snap_value(&self) -> f32`。
220. `SceneView::camera_pan(&mut self, delta)`。
221. `SceneView::camera_zoom(&mut self, factor)`。
222. `SceneView::camera_rotate(&mut self, delta)`。
223. `SceneView::grid_visible(&self) -> bool`。
224. `SceneView::gizmos_visible(&self) -> bool`。
225. `SceneView::mode_2d(&self) -> bool`。
226. `SceneView::toggle_2d(&mut self)`。
227. `EditorTool::Select / Move / Rotate / Scale`。
228. `EditorTool::switch(tool)` 快捷键 W/E/R。
229. `GizmoSystem::draw_transform_gizmo(transform, selected, tool)` 绘制手柄。
230. `GizmoSystem::draw_gizmo_circle(pos, r, color)`。
231. `GizmoSystem::draw_gizmo_rect(rect, color)`。
232. `GizmoSystem::draw_gizmo_grid(spacing, size, color)`。
233. `GizmoSystem::draw_gizmo_arrow(from, to, color)`。
234. `GizmoSystem::draw_gizmo_text(text, pos, color)`。
235. `SelectionRect`：矩形框选区域。
236. `LassoSelect`：自由曲线选择（后续完善，留接口）。

### 3.4 Undo/Redo 与命令

237. `EditorAction` trait：`apply(&mut self, editor)` / `undo(&mut self, editor)` / `mergeable(&self) -> bool`。
238. `CreateNodeAction`：新建节点。
239. `DeleteNodeAction`：删除节点。
240. `RenameNodeAction`：重命名。
241. `SetParentAction`：改变父子关系。
242. `SetPropertyAction`：设置单字段。
243. `AddComponentAction`：新增组件。
244. `RemoveComponentAction`：移除组件。
245. `MoveNodesAction`：位移。
246. `DuplicateAction`：复制（含 apply）。
247. `PasteAction`：粘贴。
248. `BatchAction`：多 action 合并。
249. `EditorActionStack::push(&mut self, action)`。
250. `EditorActionStack::undo(&mut self, editor)`。
251. `EditorActionStack::redo(&mut self, editor)`。
252. `EditorActionStack::clear(&mut self)`。
253. `EditorActionStack::can_undo(&self) -> bool`。
254. `EditorActionStack::can_redo(&self) -> bool`。
255. `EditorActionStack::len(&self) -> usize`。
256. `EditorActionStack::max_len(&self) -> usize`（默认 50）。
257. `EditorClipboard::copy_entities(&mut self, entities)`。
258. `EditorClipboard::paste_entities(&self, editor)`。
259. `EditorClipboard::copy_component(&mut self, component)`。
260. `EditorClipboard::paste_component(&self, editor, entity)`。

### 3.5 选择集与事件

261. `EditorSelection::new()`。
262. `EditorSelection::clear(&mut self)`。
263. `EditorSelection::select(&mut self, entity)`。
264. `EditorSelection::toggle(&mut self, entity)`。
265. `EditorSelection::add(&mut self, entity)`。
266. `EditorSelection::remove(&mut self, entity)`。
267. `EditorSelection::contains(&self, entity) -> bool`。
268. `EditorSelection::is_empty(&self) -> bool`。
269. `EditorSelection::len(&self) -> usize`。
270. `EditorSelection::iter(&self) -> impl Iterator`。
271. `EditorSelection::first(&self) -> Option<Entity>`。
272. `EditorSelection::last(&self) -> Option<Entity>`。
273. `EditorSelectionChanged` 事件。
274. `EditorSelectionChanged::old() / new()`。

### 3.6 资源管线（编辑器侧）

275. `AssetMeta::new(guid, path, importer_settings)`。
276. `AssetMeta::save(&self, path)`。
277. `AssetMeta::load(path) -> Result<Self>`。
278. `AssetImporter` trait：`can_import(&self, ext) -> bool` / `import(&self, path) -> Result<Asset>`。
279. `TextureImporter`：PNG/JPG。
280. `FontImporter`：TTF/OTF。
281. `SceneImporter`：自定义 JSON。
282. `PrefabImporter`：自定义 JSON。
283. `ModelImporter`：GLB/GLTF（后续完善）。
284. `AssetPipeline::new(asset_dir)`。
285. `AssetPipeline::scan(&mut self)` 扫描目录。
286. `AssetPipeline::import_all(&mut self)`。
287. `AssetPipeline::reimport_changed(&mut self)` 基于 mtime。
288. `AssetPipeline::assets(&self) -> &[AssetInfo]`。
289. `AssetInfo::path / meta / size / mtime`。
290. `AssetDB`：全局数据库。

### 3.7 设置与插件

291. `EditorSettings::default()`。
292. `EditorSettings::theme(&self)`。
293. `EditorSettings::set_theme(&mut self, theme)`。
294. `EditorSettings::auto_save(&self) -> bool`。
295. `EditorSettings::auto_save_interval(&self) -> Duration`。
296. `EditorSettings::key_bindings(&self) -> &KeyBindings`。
297. `EditorSettings::set_key_binding(&mut self, action, keys)`。
298. `KeyBindings`：按 action -> Vec<Key>。
299. `EditorPlugin` trait：`register(&mut editor)` / `update(&mut editor, dt)` / `ui(&mut editor, ui)`。
300. `EditorPluginRegistry::register(&mut self, plugin)`。
301. `EditorPluginRegistry::update_all(&mut self, editor, dt)`。
302. `EditorPluginRegistry::ui_all(&mut self, editor, ui)`。

### 3.8 场景/预制体保存与加载

303. `SceneSaver::save_json(scene, path)`。
304. `SceneSaver::save_bin(scene, path)`。
305. `SceneLoader::load_json(path) -> Result<SceneTree>`。
306. `SceneLoader::load_bin(path) -> Result<SceneTree>`。
307. `PrefabSaver::save_json(prefab, path)`。
308. `PrefabSaver::save_bin(prefab, path)`。
309. `PrefabLoader::load_json(path) -> Result<Prefab>`。
310. `PrefabLoader::load_bin(path) -> Result<Prefab>`。
311. 文件格式包含：version / nodes / components / assets_ref。
312. 序列化兼容：旧版本可打开。

### 3.9 示例与测试

313. `examples/editor_app` 启动编辑器。
314. `examples/editor_custom_panel` 自定义面板。
315. `examples/editor_plugin` 自定义插件。
316. `examples/editor_game` 使用编辑器开发简单游戏。
317. 单测：`EditorActionStack` undo/redo 循环。
318. 单测：`EditorSelection` 增删。
319. 单测：`SceneSaver/Loader` 往返一致。
320. 单测：`Prefab` 往返一致。
321. 单测：`AssetPipeline` 扫描与导入（无实际文件时 mock）。
322. `cargo test -p engine-editor` 全部通过。
323. `cargo clippy --workspace -- -D warnings` 通过。
324. `cargo fmt --check --workspace` 通过。
325. `cargo doc --workspace --no-deps` 成功。
326. CI 三平台 green。
327. CHANGELOG 记录 0.7.0。
328. README.md 加入「可视化编辑器」章节。
329. README.md 加入「编辑器使用指南」章节。
330. README.md 加入「插件开发指南」章节。
331. 公开 API doc comment 覆盖率 100%。
332. `unsafe` 块 <= 5。
333. 新增 example 工程 >= 4 个。
334. 编辑器面板数量 >= 5 个。
335. 编辑器菜单数量 >= 7 个。

---

## 四、验收标准

- [ ] `cargo run --example editor_app` 可启动编辑器
- [ ] `examples/editor_app` 中能：新建节点、移动节点、保存场景、加载场景
- [ ] 场景树/属性/资源/控制台/调试 5 个面板可用
- [ ] 撤销/重做至少可工作
- [ ] 可以选择/移动/旋转/缩放 2D 节点
- [ ] 能保存/加载 `*.scene.json`
- [ ] `cargo test -p engine-editor` 全部通过
- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.7.0

---

## 五、下一个 Sprint

Sprint 08 将完善跨平台打包与资源管线（Android / iOS / Web / H5 / 小程序兼容）。
