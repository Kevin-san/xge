# Sprint 02 · 事件循环 / 窗口 / 输入原语

> 阶段：阶段一 · 基础内核 MVP  
> 周期：3 周  
> 核心目标：在 Sprint 01 基础上接入窗口系统与事件循环  
> 验收：能弹出窗口并显示精灵/三角形，键鼠输入可被日志记录

---

## 一、Sprint 概览

本 Sprint 引入 `engine-window crate，默认依赖 `winit`（或自研轻量实现），并在引擎层面抽象事件循环。不关心渲染。关键交付：

- `Window` 抽象 trait + 原生实现；
- `EventLoop` 包装；
- `InputDevice` 键盘/鼠标/触摸（尚未接入；
- `WindowEvent` / `DeviceEvent` / `WindowState`；
- `examples/window_basic` — 弹出一个可关闭窗口并实时打印 FPS。

---

## 二、项目需求清单

1. `engine-window` crate 建立。
2. `WindowBuilder` Fluent API — 设置标题、尺寸、可调整大小、vsync、全屏模式。
3. `WindowBuilder` 设置最小/最大尺寸。
4. `WindowBuilder` 设置图标（可选）。
5. `WindowBuilder` 设置是否装饰（有无标题栏）。
6. `WindowBuilder` 设置透明度。
7. `WindowBuilder` 设置显示器（多显示器支持）。
8. `Window::inner_size(&self) -> PhysicalSize。
9. `Window::outer_size(&self) -> PhysicalSize。
10. `Window::set_title(&self, title)。
11. `Window::set_inner_size(&self, size)。
12. `Window::set_min_size(&self, size)。
13. `Window::set_max_size(&self, size)。
14. `Window::set_resizable(&self, bool)。
15. `Window::set_fullscreen(&self, mode)。
16. `Window::set_decorations(&self, bool)。
17. `Window::set_always_on_top(&self, bool)。
18. `Window::request_redraw(&self)。
19. `Window::set_visible(&self, bool)。
20. `Window::scale_factor(&self) -> f64。
21. `Window::current_monitor(&self) -> Option<MonitorHandle>。
22. `Window::raw_display_handle(&self) — 返回底层 handle（便于接入图形 API）。
23. `Window::raw_window_handle(&self)。
24. `EventLoop` 包装。
25. `EventLoop::run(handler) — 启动并阻塞。
26. `EventLoop::proxy() -> EventLoopProxy 用于跨线程唤醒。
27. `EventLoopBuilder::with_user_event() 支持自定义事件。
28. `Engine::window(&self) -> &Window — 返回主窗口引用。
29. `Engine::set_window_mode(...) — 切换窗口模式。
30. `Engine::request_close(&self) — 请求退出。
31. `Engine::show_cursor(&self, bool)。
32. `Engine::set_cursor_grab(&self, bool)。
33. `Engine::set_cursor_icon(CursorIcon)。
34. `Engine::set_ime_allowed(bool)。
35. `Engine::set_ime_position(position)。
36. `Engine::set_cursor_position(position)。
37. 实现 `Fullscreen` 枚举：Borderless / Exclusive / None。
38. 实现 `Monitor` 抽象：分辨率列表、当前分辨率、刷新率。
39. 实现 `VideoMode` 枚举。
40. 实现 `dpi::PhysicalSize / LogicalSize / Position`。
41. 实现 DPI 缩放感知。
42. 实现 DPI 变化事件。
43. 实现窗口最小化/最大化/恢复/关闭事件。
44. 实现窗口获得/失去焦点事件。
45. 实现窗口悬停/离开事件。
46. 实现 RedrawRequested 事件。
47. 实现 `KeyboardInput` 事件 — keycode + scancode + state + repeat。
48. 实现 `ModifiersState` — shift/ctrl/alt/meta/win/cmd。
49. 实现 `ReceivedCharacter(char)` 事件 — 用于文本输入。
50. 实现 `MouseButton` 枚举：Left / Middle / Right / Other(u16)。
51. 实现 `MouseInput` 事件。
52. 实现 `CursorMoved` 事件。
53. 实现 `CursorEntered` / `CursorLeft` 事件。
54. 实现 `MouseWheel` 事件（LineDelta / PixelDelta）。
55. 实现 `Touch` 事件（phase + location + force + id）。
56. 实现 `TouchPhase` 枚举：Started / Moved / Ended / Cancelled。
57. 实现 `GamepadEvent` 留位（后续实现）。
58. 实现 `Input` 状态快照：键盘/鼠标/触摸。
59. `Input::key_pressed(KeyCode) -> bool。
60. `Input::key_just_pressed(KeyCode) -> bool。
61. `Input::key_just_released(KeyCode) -> bool。
62. `Input::mouse_button_pressed(button) -> bool。
63. `Input::mouse_button_just_pressed(button) -> bool。
64. `Input::mouse_button_just_released(button) -> bool。
65. `Input::mouse_position(&self) -> Vec2。
66. `Input::mouse_delta(&self) -> Vec2。
67. `Input::wheel_delta(&self) -> Vec2。
68. `Input::touch_count(&self) -> usize。
69. `Input::touches(&self) -> 迭代器。
70. `Input::modifiers(&self) -> ModifiersState。
71. `Input::clear(&mut self) — 每帧重置瞬时状态。
72. `Input::reset(&mut self) — 完全重置。
73. `InputModule` 作为 Module：on_event 订阅窗口事件并更新 `Input`。
74. 实现 `KeyCode` 枚举：覆盖常用键位（A-Z, 0-9, F1-F12, 方向键, 空格, 回车, 退格, Esc, Tab, 修饰键）。
75. 实现 `ScanCode` 原始扫描码类型。
76. 实现 `ElementState`：Pressed / Released。
77. 实现 `CursorIcon` 枚举：Default / Crosshair / Hand / Move / Text / Wait / Help / Progress / NResize / EResize / …（12 方向）。
78. 实现 `WindowMode` 枚举。
79. 实现 `WindowEvent` 与 `EngineEvent` 分离 — EngineEvent 作为用户自定义事件。
80. 实现 `Event` trait — 泛型事件类型。
81. 事件派发使用 `EventReader<T>` / `EventWriter<T>` 模式。
82. `Engine::events<T>() -> EventReader<T>。
83. `Engine::send_event<T>(event)。
84. 主循环：poll -> update -> render。
85. 主循环固定时间步支持：`update(dt)` 多次调用以补偿。
86. 主循环对大 dt 钳制（max_dt 防止尖峰）。
87. vsync 开启/关闭可配置。
88. 帧率限制可配置（max_fps）。
89. `WindowConfig` 结构体并可从配置文件。
90. `WindowConfig::default()` 给出默认：1280x720，可调整，标题为 engine 名称。
91. `examples/window_basic` 示例：创建窗口并在 3 秒后自动退出。
92. `examples/input_show_keys` 示例：实时打印按键事件。
93. `examples/input_mouse` 示例：实时打印鼠标位置和按钮。
94. `examples/fullscreen` 示例：按键 F 切换全屏。
95. `examples/dpi` 示例：打印 DPI 缩放变化。
96. WebAssembly 平台：通过 `wasm-bindgen` 留位（trait 空实现）。
97. 窗口事件抽象：trait WindowBackend，默认实现 `WinitWindow（winit）。
98. 引入 `raw-window-handle 0.5` 接口统一。
99. CI 增加 headless 测试：`--no-window 模式。
100. `cargo test -p engine-window 单元测试 10+ 条。
101. `engine-window 公开 API 数量控制在 30~40 之间。
102. 中文注释与英文注释并存。
103. `Engine::is_focused(&self) -> bool。
104. `Engine::is_minimized(&self) -> bool。
105. `Engine::is_maximized(&self) -> bool。
106. `Engine::is_visible(&self) -> bool。
107. 实现 `Clipboard` 支持（预留接口留位。
108. 实现 `Clipboard::get_text(&self) -> Option<String>` 留位。
109. 实现 `Clipboard::set_text(&self, text) -> Result<()>` 留位。
110. 文档注释全部补齐。

---

## 三、细分需求与验收

### 3.1 窗口系统（Window API）

111. `WindowBuilder::new()` — 构造。
112. `WindowBuilder::with_title(title)`。
113. `WindowBuilder::with_inner_size(size)`。
114. `WindowBuilder::with_min_inner_size(size)`。
115. `WindowBuilder::with_max_inner_size(size)`。
116. `WindowBuilder::with_resizable(bool)`。
117. `WindowBuilder::with_fullscreen(mode)`。
118. `WindowBuilder::with_decorations(bool)`。
119. `WindowBuilder::with_transparent(bool)`。
120. `WindowBuilder::with_always_on_top(bool)`。
121. `WindowBuilder::with_visible(bool)`。
122. `WindowBuilder::with_maximized(bool)`。
123. `WindowBuilder::with_minimized(bool)`。
124. `WindowBuilder::with_content_protected(bool)`。
125. `WindowBuilder::with_window_icon(icon)`。
126. `WindowBuilder::with_prevent_defocus(bool)`。
127. `WindowBuilder::with_ime(bool)`。
128. `WindowBuilder::with_cursor_hittest(bool)`。
129. `WindowBuilder::build(&self) -> Result<Window>`。
130. `Window::id(&self) -> WindowId`。
131. `Window::title(&self) -> String`。
132. `Window::inner_size(&self) -> PhysicalSize<u32>`。
133. `Window::outer_size(&self) -> PhysicalSize<u32>`。
134. `Window::position(&self) -> PhysicalPosition<i32>`。
135. `Window::inner_position(&self) -> PhysicalPosition<i32>`。
136. `Window::outer_position(&self) -> PhysicalPosition<i32>`。
137. `Window::request_redraw(&self)`。
138. `Window::set_title(&self, title)`。
139. `Window::set_dimensions(&self, size)`。
140. `Window::set_min_dimensions(&self, size)`。
141. `Window::set_max_dimensions(&self, size)`。
142. `Window::set_resizable(&self, bool)`。
143. `Window::set_minimized(&self, bool)`。
144. `Window::set_maximized(&self, bool)`。
145. `Window::set_visible(&self, bool)`。
146. `Window::set_always_on_top(&self, bool)`。
147. `Window::set_fullscreen(&self, mode)`。
148. `Window::set_decorations(&self, bool)`。
149. `Window::set_window_level(level)`。
150. `Window::set_window_icon(icon)`。
151. `Window::set_ime_allowed(&self, bool)`。
152. `Window::set_ime_cursor_area(&self, pos)`。
153. `Window::set_ime_purpose(purpose)`。
154. `Window::set_cursor_icon(cursor)`。
155. `Window::set_cursor_position(pos) -> Result<()>`。
156. `Window::set_cursor_grab(mode)`。
157. `Window::set_cursor_hittest(bool)`。
158. `Window::set_cursor_visible(&self, bool)`。
159. `Window::drag_window(&self) -> Result<()>`。
160. `Window::drag_resize_window(edge)`。
161. `Window::focus_window(&self)`。
162. `Window::show_window_menu(&self, pos)`。
163. `Window::scale_factor(&self) -> f64`。
164. `Window::raw_display_handle(&self)`。
165. `Window::raw_window_handle(&self)`。
166. `Window::current_monitor(&self)`。
167. `Window::available_monitors(&self)`。
168. `Window::primary_monitor(&self)`。
169. `Window::theme(&self) -> Theme`。
170. `Window::scale_factor_changed_event_notifier(&self)`。

### 3.2 事件循环（EventLoop）

171. `EventLoop::new()`。
172. `EventLoopBuilder::new()`。
173. `EventLoopBuilder::with_user_event()`。
174. `EventLoop::run(event_handler)`。
175. `EventLoop::run_return(event_handler)`（非阻塞模式）。
176. `EventLoopProxy::create_proxy(&self)`。
177. `EventLoopProxy::send_event(user_event)`。
178. `EventLoopProxy::wake_up()`。
179. `Event::WindowEvent { window_id, event }`。
180. `Event::DeviceEvent { device_id, event }`。
181. `Event::NewEvents(cause)`。
182. `Event::AboutToWait`。
183. `Event::LoopDestroyed`。
184. `Event::UserEvent(T)`。
185. `Event::Suspended`。
186. `Event::Resumed`。
187. `EventLoopControlFlow` 枚举：Poll / Wait / WaitUntil(Instant) / Exit。
188. `EventLoop::set_control_flow(control_flow)`。
189. `EventLoop::control_flow(&self)`。
190. `Engine::event_loop_proxy(&self) -> EventLoopProxy`。

### 3.3 输入状态（Input）

191. `Input::new()`。
192. `Input::clear(&mut self)`。
193. `Input::reset(&mut self)`。
194. `Input::key_pressed(&self, keycode)`。
195. `Input::key_just_pressed(&self, keycode)`。
196. `Input::key_just_released(&self, keycode)`。
197. `Input::mouse_pressed(&self, button)`。
198. `Input::mouse_just_pressed(&self, button)`。
199. `Input::mouse_just_released(&self, button)`。
200. `Input::mouse_position(&self)`。
201. `Input::mouse_delta(&self)`。
202. `Input::wheel_delta(&self)`。
203. `Input::modifiers(&self)`。
204. `Input::set_cursor_in_window(&self, bool)`。
205. `Input::text(&self) -> &str`。
206. `Input::events(&self) -> impl Iterator`。
207. `Input::pressed_keys(&self)`。
208. `Input::released_keys(&self)`。
209. `Input::pressed_buttons(&self)`。
210. `Input::released_buttons(&self)`。
211. `Input::touches(&self)`。
212. `Input::touch(id)`。
213. `KeyCode::KeyA-Z` 全部字母键 26 个。
214. `KeyCode::Digit0-9` 数字键 10 个。
215. `KeyCode::F1-F12` 功能键 12 个。
216. `KeyCode::ArrowUp/Down/Left/Right` 4 个。
217. `KeyCode::Escape` / `Tab / Space / Enter / Backspace / Delete / Insert / Home / End / PageUp / PageDown 11 个。
218. `KeyCode::ShiftLeft / ShiftRight / ControlLeft / ControlRight / AltLeft / AltRight / SuperLeft / SuperRight 8 个修饰键。
219. `KeyCode::Numpad0-9 + Add + Subtract + Multiply + Divide + Decimal + Enter 15 个。
220. `KeyCode::Plus / Minus / Equal / BracketLeft / BracketRight / Semicolon / Apostrophe / Comma / Period / Slash / Backslash / Grave 12 个符号键。
221. `MouseButton::Left / Middle / Right` + Other(u16)。
222. `MouseScrollDelta::LineDelta(x, y) / PixelDelta(x, y)。
223. `TouchPhase::Started / Moved / Ended / Cancelled`。
224. `Touch` 结构体：id, position, force, phase。
225. `ModifiersState SHIFT / CTRL / ALT / SUPER / 组合操作（bitflags）。
226. `InputModule::new()`。
227. `InputModule::process_event(&mut self, event)`。
228. `InputModule::input(&self) -> &Input`。
229. `InputModule::input_mut(&mut self) -> &mut Input`。

### 3.4 示例工程

230. `examples/window_basic` — 创建默认窗口。
231. `examples/window_fullscreen` — F 切换全屏。
232. `examples/window_custom` — 1280x720 + 自定义标题。
233. `examples/input_keys` — 打印键盘事件。
234. `examples/input_mouse` — 打印鼠标事件。
235. `examples/input_touch` — Web/移动端触摸。
236. `examples/input_gamepad` — 留位。
237. `examples/input_text` — 文本输入留位。
238. `examples/dpi` — DPI 变化感知。
239. `examples/multi_window` — 留位（后续）。
240. `examples/event_loop_proxy` — 跨线程发事件。

### 3.5 测试与文档

241. `cargo test --workspace 全部通过。
242. 单元测试覆盖 WindowBuilder 测试：设置/最小/最大 尺寸。
243. 单元测试 Input clear/reset 的区别。
244. 单元测试 key_pressed / key_just_pressed。
245. 单元测试 mouse_button_pressed / mouse_button_just_pressed。
246. 单元测试 mouse_position / delta。
247. 单元测试 EventLoopProxy send_event。
248. 单元测试 EventBus<WindowEvent>。
249. 文档：`cargo doc --open 正常生成。
250. 文档：Window API 所有公开项都有 doc comment。
251. 文档：Input API 所有公开项都有 doc comment。
252. 文档：示例 10 个以上。
253. 文档：README.md 包含快速上手。
254. 文档：README.md 含有「如何创建窗口」章节。
255. 文档：README.md 含有「如何处理输入」章节。
256. clippy 无 warning。
257. fmt 检查通过。
258. CI 三平台 green。
259. 本 Sprint 新增 unsafe 数 <= 3。
260. CHANGELOG 记录版本 0.2.0。
261. 新增 examples 全部可运行。
262. 本 Sprint 公开 API <= 50。
263. 本 Sprint 公开函数 doc comment 覆盖率 100%。
264. 本 Sprint 新增文档 >= 200 行。
265. 本 Sprint 新增 example 工程 >= 10 个。

> 以上 265 条需求为 Sprint 02 全量清单。

---

## 四、验收标准

- [ ] `cargo run --example window_basic` 弹出窗口并正常退出
- [ ] `cargo run --example input_keys` 实时打印按键
- [ ] `cargo run --example fullscreen` F 键切换全屏
- [ ] 所有 example 在 Windows/macOS/Linux 运行成功
- [ ] 单元测试 `cargo test -p engine-window 全部通过
- [ ] clippy 无 warning
- [ ] fmt check 通过
- [ ] cargo doc 成功
- [ ] 本 Sprint 结束 `unsafe` <= 3
- [ ] CHANGELOG 已更新

---

## 五、下一个 Sprint

Sprint 03 将在窗口之上接入 2D 渲染核心（精灵/纹理/批处理/图集）。
