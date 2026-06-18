# Sprint 06 · UI 控件库与布局引擎

> 阶段：阶段二 · 编辑器 + 跨平台（第 2 个 Sprint）  
> 周期：4 周  
> 核心目标：完成 UI 控件库（Button/Label/Image/Input/Slider/Panel/Grid/List/ScrollView）+ 锚点/自适应/流式布局 + UI 事件系统  
> 验收：`examples/ui_demo` 构建一个完整主菜单 + 设置面板 + HUD

---

## 一、Sprint 概览

本 Sprint 建立 `engine-ui` crate。基于 ECS（Entity=控件，Component=UI属性），使用 ECS 进行事件与布局更新。核心交付：

- `UiNode` / `UiRect` / `UiStyle` / `UiText` / `UiImage` / `UiButton` / `UiInput` / `UiSlider` / `UiPanel` / `UiGrid` / `UiList` / `UiScroll`
- 锚点布局（Anchor）、边距、百分比尺寸、自动尺寸
- UI 事件系统（点击/悬停/拖拽/按下/释放/焦点/键盘）
- UI 合批与绘制
- 字体系统（TTF 字体加载、字号、字重、富文本简化版）
- `examples/ui_demo`：主菜单 + 设置 + HUD

---

## 二、项目需求清单

1. `engine-ui` crate 建立。
2. `UiNode` 组件：entity 上的 UI 节点标记。
3. `UiRect` 组件：position / size / anchor / margin / padding。
4. `UiStyle` 组件：背景色 / 边框 / 圆角 / 阴影。
5. `UiText` 组件：文本内容 / 字体 / 字号 / 颜色 / 对齐 / 换行模式。
6. `UiImage` 组件：texture 句柄 / 九宫格切片 / UV。
7. `UiButton` 组件：label / normal / hovered / pressed style / disabled。
8. `UiInput` 组件：文本输入 / 光标位置 / 选中区 / placeholder。
9. `UiSlider` 组件：min/max/value/step/方向。
10. `UiToggle` 组件：bool 值 + 样式。
11. `UiPanel` 组件：容器 + 布局模式。
12. `UiGrid` 组件：行/列/间距/对齐。
13. `UiList` 组件：垂直/水平列表 + 子项。
14. `UiScroll` 组件：内容区域 + 滚动条 + 鼠标滚轮支持。
15. `UiProgressBar`：progress 0~1。
16. `UiTooltip`：悬停显示气泡。
17. `UiWindow`：可拖动/可关闭窗口（用于编辑器后续）。
18. `UiCanvas`：根组件 + DPI + screen_size。
19. `UiLayout`：垂直/水平/网格/绝对。
20. `Anchor`：TopLeft / TopCenter / TopRight / CenterLeft / Center / CenterRight / BottomLeft / BottomCenter / BottomRight / Custom(Vec2)。
21. `UiSize`：像素绝对 / 百分比 / 内容自动。
22. `UiMargin`：left/right/top/bottom。
23. `UiVisibility`：Visible / Hidden / Collapsed。
24. `UiFocus`：当前焦点 entity + Tab 顺序。
25. `UiZIndex`：z 排序。
26. `UiEvent` 枚举：Click/Press/Release/HoverEnter/HoverLeave/DragStart/Drag/DragEnd/Focus/Blur/TextInput/ValueChanged。
27. `UiEventReader<E>`。
28. `UiEventWriter<E>`。
29. `UiStyleSheet`：统一样式（类似 CSS subset）。
30. `UiTheme`：亮色/暗色/自定义主题。
31. `Font`：字体加载 TTF/OTF。
32. `FontAtlas`：字体纹理图集。
33. `TextLayout`：字/行/段落布局。
34. `Text2d`：组件封装（简化版文本绘制）。
35. `Glyph`：字形索引 + 位置 + 大小。
36. `RichText`：样式段（颜色、字号、字重）。
37. `TextSection`：文本段。
38. `FontWeight`：Thin/ExtraLight/Light/Normal/Medium/SemiBold/Bold/ExtraBold/Black。
39. `FontStyle`：Normal / Italic / Oblique。
40. `TextAlign`：Left / Center / Right / Justify。
41. `TextOverflow`：Wrap / Clip / Ellipsis。
42. `UiInput` 支持：光标移动 / 删除 / 选中 / 复制 / 粘贴 / 撤销。
43. `UiInput` 支持：限制长度 / 数字模式 / 密码模式（隐藏字符）。
44. `UiSlider` 支持：可拖动 / 点击跳转 / 步长。
45. `UiButton` 状态：Normal / Hovered / Pressed / Disabled / Selected。
46. `UiScroll`：内容区域 + 滑块 + 步长 + 页跳。
47. `UiGrid`：列自动布局。
48. `UiList`：支持虚拟化（仅渲染可见部分）。
49. `UiLayout` 使用：`layout_system` 每帧重算。
50. 布局系统：先递归计算 desired_size，再 apply 实际 rect。
51. 布局系统：支持 `flex_grow / flex_shrink / flex_basis`（Flexbox 简化版）。
52. 布局系统：支持 `auto_calculate_width / auto_calculate_height`。
53. 布局系统：DPI 缩放感知。
54. 布局系统：SafeArea（刘海屏/异形屏）。
55. `UiNodeBundle`：spawn 一个 UI 节点。
56. `TextBundle`：spawn 一个文本节点。
57. `ImageBundle`：spawn 一个图片节点。
58. `ButtonBundle`：spawn 一个按钮节点。
59. `InputBundle`：spawn 一个输入框。
60. `SliderBundle`：spawn 一个滑块。
61. `PanelBundle`：spawn 一个面板容器。
62. `GridBundle`：spawn 一个网格容器。
63. `ListBundle`：spawn 一个列表容器。
64. `ScrollBundle`：spawn 一个滚动容器。
65. `CanvasBundle`：spawn UI Canvas。
66. `UiCommands`：便捷 API 创建 UI。
67. `ui_node!` 宏（声明式）：`ui_node! { Panel(style) { Text("hello") Button("click") } }`。
68. `UiEvent::Click(entity)` 由点击事件派发。
69. `UiEvent::HoverEnter(entity)` 由鼠标进入派发。
70. `UiEvent::HoverLeave(entity)` 由鼠标离开派发。
71. `UiEvent::DragStart(entity)` 由按下 + 移动派发。
72. `UiEvent::ValueChanged(entity, new_value)`。
73. `UiEvent::Focus(entity)` / `Blur(entity)`。
74. `UiEvent::TextInput(entity, text)`。
75. UI 点击检测：AABB 命中测试 + 子节点排序。
76. UI 点击检测：透明区域穿透检测。
77. UI 输入焦点：Tab 切换。
78. UI 输入焦点：鼠标点击切换。
79. UI 键盘导航：方向键。
80. UI 绘制：合批（按 shader、按纹理）。
81. UI 绘制：九宫格切片支持。
82. UI 绘制：圆角绘制。
83. UI 绘制：描边 / 阴影。
84. UI 绘制：抗锯齿。
85. UI 绘制：裁剪（scissor rect）。
86. UI 绘制：DPI 缩放。
87. UI 字体系统：`Font::from_file(path) -> Result<Font>`。
88. `Font::get_glyph(ch, size)` 获取字形。
89. `Font::measure(text, size) -> Vec2`。
90. `FontAtlasBuilder::new()` + `add(font, size, chars)` + `build() -> FontAtlas`。
91. `FontAtlas::texture() -> TextureHandle`。
92. `FontAtlas::get_uv(ch)` 返回字形 UV。
93. `FontAtlas::get_kerning(a, b)` 返回字距。
94. `TextLayout::new(font, size, text, width, align)`。
95. `TextLayout::glyphs() -> &[Glyph]`。
96. `TextLayout::lines() -> &[Line]`。
97. `TextLayout::size() -> Vec2`。
98. 富文本：`RichText` + `TextSection`。
99. 富文本：颜色段、大小段、字体段。
100. 富文本：支持 \n 换行。
101. 动画系统基础：按钮 hover 缩放过渡。
102. 动画：按钮 press 下沉动画。
103. 动画：渐显/渐隐。
104. 动画：滑入/滑出。
105. UI 调试：`UiDebugPlugin` 显示边框、位置、层级。
106. UI 性能：每帧 UI batch 数量跟踪。
107. `examples/ui_demo`：主菜单（开始/设置/退出）。
108. `examples/ui_settings`：分辨率、全屏、音量滑块、主题切换。
109. `examples/ui_hud`：游戏内 HUD（分数、血条、时间）。
110. `examples/ui_input`：输入框 + 键盘输入。
111. `examples/ui_scroll`：长列表滚动。
112. `examples/ui_richtext`：富文本。
113. `examples/ui_layout`：多种布局展示。
114. `examples/ui_animation`：按钮/面板动画。
115. `examples/ui_theme`：亮色/暗色主题切换。
116. `examples/ui_game_menu`：主菜单 + 游戏内暂停菜单。
117. 单测：`UiRect` 计算。
118. 单测：`Anchor` 计算。
119. 单测：`UiLayout` 垂直布局。
120. 单测：`UiLayout` 水平布局。
121. 单测：`UiLayout` 网格布局。
122. 单测：`Font::measure`。
123. 单测：`TextLayout::glyphs` 位置。
124. 单测：`UiButton` 状态机。
125. 单测：`UiSlider` 值变化。
126. 单测：`UiInput` 光标位置、插入、删除。
127. 单测：`UiEvent` 派发与读取。
128. 单测：`UiFocus` Tab 顺序。
129. 集成测试：`ui_demo` 渲染无崩溃。
130. 集成测试：`ui_input` 输入文本可读回。
131. `cargo test -p engine-ui` 全部通过。
132. `cargo clippy --workspace -- -D warnings` 通过。
133. `cargo fmt --check --workspace` 通过。
134. `cargo doc --workspace --no-deps` 成功。
135. CI 三平台 green。
136. CHANGELOG 记录版本 0.6.0。
137. README.md 加入「UI 系统」章节。
138. 公开 API doc comment 覆盖率 100%。
139. 本 Sprint `unsafe` 块 <= 2。
140. 本 Sprint 新增 example 工程 >= 10 个。

> 以上 140 条需求构成 Sprint 06 全量清单。

---

## 三、细分需求与验收

### 3.1 UI 核心组件

141. `UiNode` 组件 + Bundle：默认值、公开字段。
142. `UiRect::position`：相对父节点的偏移。
143. `UiRect::size`：宽高。
144. `UiRect::anchor`：锚点。
145. `UiRect::margin`：外边距。
146. `UiRect::padding`：内边距。
147. `UiRect::z_index`：绘制顺序。
148. `UiRect::desired_size(&self) -> Vec2`。
149. `UiRect::final_rect(&self, parent_rect) -> Rect`。
150. `UiRect::visible(&self) -> bool`。
151. `UiStyle::background_color`。
152. `UiStyle::border_color`。
153. `UiStyle::border_radius`。
154. `UiStyle::border_width`。
155. `UiStyle::box_shadow`（偏移 + 模糊 + 颜色）。
156. `UiStyle::opacity`（0~1）。
157. `UiStyle::merge(&self, other)` 主题合并。
158. `UiTheme::default_light() / default_dark()`。
159. `UiTheme::apply(&self, ui_node)` 应用样式。
160. `UiTheme::get_style(&self, class) -> &UiStyle`。

### 3.2 文本/字体

161. `Font::load(path)` 从 TTF 加载。
162. `Font::load_from_bytes(bytes)` 从内存加载。
163. `Font::name(&self) -> &str`。
164. `Font::has_glyph(ch)`。
165. `Font::line_height(size)`。
166. `Font::get_glyph(ch, size) -> Glyph`。
167. `Font::get_kerning(a, b)`。
168. `FontAtlasBuilder::new()`。
169. `FontAtlasBuilder::add_font(font, size, charset)`。
170. `FontAtlasBuilder::build(&self, ctx) -> Result<FontAtlas>`。
171. `FontAtlas::texture(&self) -> TextureHandle`。
172. `FontAtlas::get_uv(ch) -> Option<Rect>`。
173. `FontAtlas::get_glyph(ch) -> Option<Glyph>`。
174. `FontAtlas::font_size(&self) -> f32`。
175. `TextLayout::new(font, size, text, max_width, align)`。
176. `TextLayout::glyphs(&self) -> &[Glyph]`。
177. `TextLayout::lines(&self) -> &[Line]`。
178. `TextLayout::size(&self) -> Vec2`。
179. `TextLayout::char_index_at(pos) -> usize`。
180. `TextLayout::line_wrap: Wrap / Clip / Ellipsis`。
181. `RichText::new()`。
182. `RichText::push(section)`。
183. `TextSection::new(text, style)`。
184. `TextSection::with_color(color)`。
185. `TextSection::with_size(size)`。
186. `TextSection::with_bold()`。
187. `TextSection::with_italic()`。
188. `TextSection::with_font(font_handle)`。
189. `RichTextLayout`：计算富文本。
190. `Glyph::uv_rect / position / size` 公开字段。

### 3.3 控件库

191. `UiButton::label(&self) -> &str`。
192. `UiButton::state(&self) -> ButtonState`。
193. `UiButton::set_state(&mut self, state)`。
194. `UiButton::is_disabled(&self) -> bool`。
195. `UiButton::set_disabled(&mut self, bool)`。
196. `ButtonState::Normal / Hovered / Pressed / Disabled / Selected`。
197. `UiInput::text(&self) -> &str`。
198. `UiInput::set_text(&mut self, text)`。
199. `UiInput::cursor(&self) -> usize`。
200. `UiInput::set_cursor(&mut self, pos)`。
201. `UiInput::select_range(&self) -> Range<usize>`。
202. `UiInput::insert_char(&mut self, ch)`。
203. `UiInput::delete_backward(&mut self)`。
204. `UiInput::delete_forward(&mut self)`。
205. `UiInput::max_length(&self) -> Option<usize>`。
206. `UiInput::set_max_length(&mut self, opt_len)`。
207. `UiInput::is_password(&self) -> bool`。
208. `UiInput::password_char(&self) -> char`。
209. `UiInput::placeholder(&self) -> &str`。
210. `UiInput::set_placeholder(&mut self, text)`。
211. `UiInput::numeric_mode(&self) -> bool`。
212. `UiInput::set_numeric_mode(&mut self, bool)`。
213. `UiSlider::value(&self) -> f32`。
214. `UiSlider::set_value(&mut self, v)` — clamp 到 [min, max]。
215. `UiSlider::min(&self) / max(&self) / step(&self)`。
216. `UiSlider::orientation(&self) -> H / V`。
217. `UiToggle::value(&self) -> bool`。
218. `UiToggle::set_value(&mut self, bool)`。
219. `UiProgressBar::progress(&self) -> f32`。
220. `UiProgressBar::set_progress(&mut self, v)`。
221. `UiPanel::layout_mode(&self) -> LayoutMode`。
222. `UiGrid::rows(&self) / cols(&self) / gap(&self)`。
223. `UiGrid::set_rows(&mut self, rows)`。
224. `UiGrid::set_cols(&mut self, cols)`。
225. `UiGrid::set_gap(&mut self, gap)`。
226. `UiList::items(&self) -> &[Entity]`。
227. `UiList::add_item(&mut self, item)`。
228. `UiList::remove_item(&mut self, index)`。
229. `UiList::direction(&self) -> V / H`。
230. `UiScroll::content(&self) -> Entity`。
231. `UiScroll::scroll_offset(&self) -> Vec2`。
232. `UiScroll::set_scroll_offset(&mut self, v)`。
233. `UiScroll::scrollbar_visible(&self) -> bool`。
234. `UiTooltip::text(&self) -> &str`。
235. `UiTooltip::delay(&self) -> f32`。
236. `UiWindow::title(&self) -> &str`。
237. `UiWindow::draggable(&self) -> bool`。
238. `UiWindow::resizable(&self) -> bool`。
239. `UiWindow::closable(&self) -> bool`。
240. `UiCanvas::size(&self) -> Vec2`。
241. `UiCanvas::dpi(&self) -> f64`。
242. `UiCanvas::safe_area(&self) -> Rect`。

### 3.4 布局系统

243. `LayoutMode::None / Vertical / Horizontal / Grid / Flex`。
244. `LayoutDirection::LeftToRight / RightToLeft / TopToBottom / BottomToTop`。
245. `LayoutAlignment::Start / Center / End / Fill`。
246. `LayoutConstraints::min_size / max_size / available`。
247. `FlexLayout::gap / flex_direction / justify / align_items / align_self`。
248. `Justify::Start / Center / End / SpaceBetween / SpaceAround / SpaceEvenly`。
249. `AlignItems::Start / Center / End / Stretch / Baseline`。
250. `UiNode::flex_grow / flex_shrink / flex_basis`。
251. `layout_system(world)` PreUpdate 执行。
252. `layout_system` O(n) 复杂度。
253. `layout_system` DPI 缩放感知。
254. `layout_system` 支持 SafeArea。
255. `layout_system` 支持自动尺寸（根据内容）。
256. `layout_system` 支持百分比尺寸。
257. `layout_system` 支持 anchor 相对父节点。
258. `layout_system` 支持 margin/padding。
259. `layout_system` 支持 z_index 绘制顺序。
260. `layout_system` 支持裁剪 scissor。
261. `LayoutDebugPlugin` 可视化布局边框。

### 3.5 UI 事件与交互

262. `UiEvent::Click(entity)`。
263. `UiEvent::Press(entity)`。
264. `UiEvent::Release(entity)`。
265. `UiEvent::HoverEnter(entity)`。
266. `UiEvent::HoverLeave(entity)`。
267. `UiEvent::DragStart(entity)`。
268. `UiEvent::Drag(entity, delta)`。
269. `UiEvent::DragEnd(entity)`。
270. `UiEvent::Focus(entity)`。
271. `UiEvent::Blur(entity)`。
272. `UiEvent::TextInput(entity, text)`。
273. `UiEvent::ValueChanged(entity, value)`。
274. `UiEvent::ValueChangedSlider(entity, value)`。
275. `UiEvent::Toggled(entity, bool)`。
276. `UiEvent::Scroll(entity, offset)`。
277. `hit_test_system` 将鼠标事件转换为 UI 事件。
278. `hit_test_system` 支持嵌套：返回最上层命中节点。
279. `hit_test_system` 支持透明穿透。
280. `focus_system` 管理 Tab 顺序。
281. `focus_system` 管理鼠标点击切换焦点。
282. `focus_system` 方向键切换焦点。
283. `ui_event_dispatch_system` 派发事件。
284. `ui_event_reader_system` 读取并消费事件。
285. `UiEventQueue` 双缓冲。

### 3.6 UI 渲染

286. `ui_render_system` 生成 draw call。
287. `ui_render_system` 按 texture 合批。
288. `ui_render_system` 按 shader 合批。
289. `ui_render_system` 支持 scissor 裁剪。
290. `ui_render_system` 支持圆角矩形。
291. `ui_render_system` 支持边框。
292. `ui_render_system` 支持阴影。
293. `ui_render_system` 支持透明度。
294. `ui_render_system` 抗锯齿（MSAA 或软边缘）。
295. `ui_render_system` DPI 缩放。
296. `nine_slice` 九宫格绘制。
297. `text_batch` 文本合批绘制。
298. `font_atlas_system` 字体图集构建。
299. `ui_batch_stats` 每帧 batch 数量。
300. `ui_batch_stats` 每帧顶点数量。

### 3.7 示例与测试

301. `examples/ui_demo` 主菜单。
302. `examples/ui_settings` 设置面板。
303. `examples/ui_hud` HUD。
304. `examples/ui_input` 输入框。
305. `examples/ui_scroll` 滚动面板。
306. `examples/ui_richtext` 富文本。
307. `examples/ui_layout` 布局展示。
308. `examples/ui_animation` 过渡动画。
309. `examples/ui_theme` 主题切换。
310. `examples/ui_game_menu` 游戏菜单。
311. 单测 `UiRect` 计算。
312. 单测 `Anchor` 计算。
313. 单测 `VerticalLayout`。
314. 单测 `HorizontalLayout`。
315. 单测 `GridLayout`。
316. 单测 `Font` measure。
317. 单测 `TextLayout` glyph 位置。
318. 单测 `TextLayout` word wrap。
319. 单测 `UiButton` 状态切换。
320. 单测 `UiSlider` 值 clamp。
321. 单测 `UiInput` 光标 + 输入。
322. 单测 `UiInput` Tab 切换。
323. 单测 `UiEvent` 派发。
324. 单测 `UiFocus` 顺序。
325. 集成测试 UI 渲染无崩溃。
326. 集成测试 UI 输入。
327. `cargo test -p engine-ui` 全部通过。
328. `cargo clippy --workspace -- -D warnings` 通过。
329. `cargo fmt --check --workspace` 通过。
330. `cargo doc --workspace --no-deps` 成功。
331. CI 三平台 green。
332. CHANGELOG 记录 0.6.0。
333. README.md 加入「UI 系统」章节。
334. README.md 加入「主题与样式」章节。
335. README.md 加入「布局引擎」章节。
336. README.md 加入「控件库」章节。
337. README.md 加入「文本与字体」章节。
338. 公开 API doc comment 覆盖率 100%。
339. `unsafe` 块 <= 2。
340. 新增 example 工程 >= 10 个。

---

## 四、验收标准

- [ ] `cargo run --example ui_demo` 正常显示主菜单
- [ ] `cargo run --example ui_settings` 可切换全屏/分辨率
- [ ] `cargo run --example ui_hud` 正常显示 HUD
- [ ] `cargo run --example ui_input` 可输入文本并显示
- [ ] `cargo run --example ui_scroll` 可滚动
- [ ] `cargo run --example ui_theme` 可切换亮/暗色主题
- [ ] `cargo test -p engine-ui` 全部通过
- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 0.6.0

---

## 五、下一个 Sprint

Sprint 07 将在引擎 UI 基础上实现可视化编辑器：场景视图 / 属性面板 / 资源面板 / 控制台。
