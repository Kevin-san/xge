# Sprint 14 · 蓝图可视化逻辑 + 脚本虚拟机

> 阶段：阶段四 · 高阶能力与生态（第 2 个 Sprint）
> 周期：4 周
> 核心目标：蓝图节点连线编辑器 + Rust/JS/TS/Python/Lua 脚本双调用 + 脚本 VM 沙盒
> 验收：`examples/blueprint_movement` 与 `examples/script_entity` 可运行

---

## 一、Sprint 概览

本 Sprint 建立两个新 crate，并串联已有 ECS 能力：

- `engine-blueprint` crate：可视化节点 + 连线数据结构、引脚类型、节点类型库、蓝图编译器（Graph → IR）、解释执行/JIT、蓝图宏、蓝图函数库、蓝图调试、蓝图编辑器 UI、蓝图与 Rust 绑定、热重载
- `engine-script` crate：`ScriptVM` trait 抽象、Rust dylib 一等公民脚本、JavaScript/TypeScript 运行时、Python 绑定、Lua 绑定、WASM 绑定、脚本沙盒（内存/指令/超时/IO/网络）、反射 + 绑定自动生成、Entity/Component/Resource/System 的脚本访问、脚本事件、调试器、字节码缓存、profiler

核心模块：

- BlueprintGraph / BlueprintNode / BlueprintPin / BlueprintWire
- BlueprintCompiler / BlueprintIR / BlueprintInterpreter
- BlueprintMacro / BlueprintFunctionLibrary / BlueprintDispatcher
- BlueprintEditorView / NodeDragDrop / WireEditor / ZoomPan / Search / AutoLayout / Reroute / CommentBox
- ScriptVM / ScriptInstance / ScriptValue / ScriptFunction
- RustScriptVM / JsScriptVM / TsCompiler / PyScriptVM / LuaScriptVM / WasmScriptVM
- ScriptSandboxPolicy / ScriptLimits / IOWhitelist / NetworkWhitelist
- ScriptReflect / Bindgen / EntityRef / ComponentRef / ResourceRef
- ScriptEventBus / ScriptDebugger / ScriptBytecodeCache / ScriptProfiler

examples 列表：

- `examples/blueprint_hello`
- `examples/blueprint_movement`
- `examples/blueprint_spawn`
- `examples/blueprint_timer`
- `examples/blueprint_ui`
- `examples/blueprint_animation`
- `examples/blueprint_physics`
- `examples/script_rust`
- `examples/script_js`
- `examples/script_ts`
- `examples/script_py`
- `examples/script_lua`
- `examples/script_wasm`
- `examples/script_sandbox`
- `examples/script_entity`

---

## 二、项目需求清单

1. `engine-blueprint` crate 建立，加入 workspace。
2. `engine-script` crate 建立，加入 workspace。
3. `BlueprintGraph`：节点集合 + 连线集合 + 宏/函数索引的顶层数据结构。
4. `BlueprintGraph::new() -> Self`。
5. `BlueprintGraph::add_node(&mut self, node) -> NodeId`。
6. `BlueprintGraph::remove_node(&mut self, node_id)`。
7. `BlueprintGraph::node(&self, id) -> &BlueprintNode`。
8. `BlueprintGraph::nodes(&self) -> &[BlueprintNode]`。
9. `BlueprintGraph::add_wire(&mut self, from_pin, to_pin) -> WireId`。
10. `BlueprintGraph::remove_wire(&mut self, wire_id)`。
11. `BlueprintGraph::wires(&self) -> &[BlueprintWire]`。
12. `BlueprintGraph::validate(&self) -> Result<(), BlueprintError>`（检测循环、类型不匹配等）。
13. `BlueprintNode`：节点 id + 节点类型 + 输入引脚 + 输出引脚 + 元数据。
14. `BlueprintNode::id(&self) -> NodeId`。
15. `BlueprintNode::kind(&self) -> NodeKind`。
16. `BlueprintNode::input_pins(&self) -> &[Pin]`。
17. `BlueprintNode::output_pins(&self) -> &[Pin]`。
18. `BlueprintNode::pin_by_name(&self, name) -> Option<&Pin>`。
19. `Pin`：引脚 id + 名称 + 方向 + 类型 + 默认值。
20. `PinDirection::Input / Output`。
21. `PinType::Bool / Int / Float / Vec2 / Vec3 / Vec4 / String / Entity / Any / Exec`。
22. `PinValue::Bool / Int / Float / Vec2 / Vec3 / Vec4 / String / Entity / Any / Exec / Invalid`。
23. `Pin::default_value(&self) -> Option<&PinValue>`。
24. `BlueprintWire`：源引脚 + 目标引脚。
25. `BlueprintWire::from(&self) -> PinRef`。
26. `BlueprintWire::to(&self) -> PinRef`。
27. `NodeId / PinId / WireId` 类型别名，强类型 `newtype`。
28. `NodeKind::Function / Macro / Event / VariableGet / VariableSet / If / For / While / Switch / Branch / Sequence`。
29. `NodeKind::Timeline / Gate / DoN / DoOnce / RetriggerableDelay / Delay / PrintString`。
30. 数学节点：`Add / Subtract / Multiply / Divide / Dot / Cross / Normalize / Length`。
31. 数学节点：`Lerp / Clamp / Min / Max / Abs / Sin / Cos / Tan / Log / Sqrt`。
32. 向量节点：`BreakVec2 / BreakVec3 / BreakVec4 / MakeVec2 / MakeVec3 / MakeVec4`。
33. 计时节点：`SetTimer / ClearTimer / IsTimerActive`。
34. Actor 节点：`SpawnActor / DestroyActor / GetActorLocation / SetActorLocation`。
35. Actor 节点：`GetActorRotation / SetActorRotation / GetActorScale / SetActorScale`。
36. Actor 节点：`AddActorWorldOffset / AddActorLocalOffset`。
37. 碰撞与检测：`LineTraceByChannel / MultiSphereTrace / OverlapAll`。
38. 事件节点：`BeginPlay / Tick / OnComponentHit / OnComponentBeginOverlap / OnClicked`。
39. 事件节点：`CustomEvent / EventDispatcher / AddCustomEventListener / CallEventDispatcher`。
40. 类型与工具节点：`CastTo / IsValid / Select / StructSetMember / StructGetMember`。
41. `BlueprintCompiler`：将 `BlueprintGraph` 编译为 `BlueprintIR`。
42. `BlueprintCompiler::compile(&self, graph) -> Result<BlueprintIR, CompileError>`。
43. `BlueprintIR`：指令序列 + 常量池 + 变量槽 + 函数表。
44. `BlueprintIRInstruction::Nop / PushConst / LoadVar / StoreVar / Add / Sub / Mul / Div`。
45. `BlueprintIRInstruction::Jump / JumpIf / JumpIfNot / Call / Return / Yield`。
46. `BlueprintIRInstruction::PinWrite / PinRead / ExecActivate / ExecDeactivate`。
47. `BlueprintIR::serialize(&self) -> Vec<u8>`（bytecode 序列化）。
48. `BlueprintIR::deserialize(bytes) -> Result<Self, DeserializeError>`。
49. `BlueprintInterpreter`：基于栈的 IR 解释执行器。
50. `BlueprintInterpreter::new(ir) -> Self`。
51. `BlueprintInterpreter::run(&mut self, context) -> Result<(), RuntimeError>`。
52. `BlueprintInterpreter::step(&mut self, context) -> Result<(), RuntimeError>`（单步）。
53. `BlueprintJIT`（可选）：将热点 IR 编译为 Rust dynlib 或 native call（第一阶段仅解释）。
54. `BlueprintMacro`：Macro Graph + 输入/输出参数 + 展开为节点组。
55. `BlueprintMacro::expand(&self, context) -> Vec<BlueprintNode>`。
56. `BlueprintFunctionLibrary`：一组可复用的函数蓝图，跨 graph 调用。
57. `BlueprintFunctionLibrary::add_function(&mut self, name, graph)`。
58. `BlueprintFunctionLibrary::get_function(&self, name) -> Option<&BlueprintGraph>`。
59. `BlueprintDebugger`：断点 / 单步 / 变量监视 / 调用栈。
60. `BlueprintDebugger::set_breakpoint(&mut self, node_id, pin_id)`。
61. `BlueprintDebugger::call_stack(&self) -> Vec<CallFrame>`。
62. `BlueprintDebugger::watch(&self, variable_name) -> Option<&PinValue>`。
63. `BlueprintEditorView`：编辑器 UI，支持节点拖拽 / 连线 / 缩放 / 平移。
64. `NodeDragDropController`：节点位置更新、网格吸附、重叠检测。
65. `WireEditor`：鼠标拖拽连线，检测源/目标引脚类型匹配。
66. `ZoomPanController`：缩放比例范围 `[0.2, 4.0]`，平移使用 Canvas 坐标。
67. `NodeSearch`：按名称/类型模糊搜索节点，插入到光标处。
68. `AutoWire`：根据类型兼容的引脚自动连线（批量插入工具）。
69. `AutoLayout`：节点布局优化（层次化 / 力导向 / 左到右拓扑）。
70. `CommentBox`：矩形注释框，覆盖一组节点，支持颜色/文本。
71. `RerouteNode`：连线中继节点，可平移以美化连线路径。
72. `BlueprintBindings`：将 Rust 函数/struct 反射为蓝图可调用节点。
73. `#[blueprint_function]` 过程宏（第一阶段用 `inventory` 手动注册）。
74. `BlueprintHotReload`：检测 `.bp.json` 文件变更，重新编译 IR 并热替换实例。
75. `examples/blueprint_hello`：BeginPrint + PrintString hello world。
76. `examples/blueprint_movement`：按键 + WASD 控制角色位移（本 Sprint 验收主例）。
77. `examples/blueprint_spawn`：定时生成 Actor。
78. `examples/blueprint_timer`：SetTimer + ClearTimer。
79. `examples/blueprint_ui`：按钮点击驱动蓝图节点。
80. `examples/blueprint_animation`：蓝图触发 AnimationController。
81. `examples/blueprint_physics`：LineTrace + AddImpulse。
82. `ScriptVM` trait：统一脚本虚拟机抽象（load / call / set_global / get_global / update）。
83. `ScriptVM::load(&mut self, source) -> Result<ScriptHandle, ScriptError>`。
84. `ScriptVM::call(&mut self, handle, fn_name, args) -> Result<ScriptValue, ScriptError>`。
85. `ScriptVM::set_global(&mut self, name, value)`。
86. `ScriptVM::get_global(&self, name) -> Option<ScriptValue>`。
87. `ScriptVM::update(&mut self, dt)`。
88. `ScriptValue::Null / Bool / Int / Float / String / Array / Map / Entity / UserData`。
89. `ScriptInstance`：单个脚本的运行实例（函数 + 闭包 + 生命周期）。
90. `RustScriptVM`：将 Rust crate 以 dylib 形式热加载，`libloading` 调用约定统一。
91. `RustScriptVM::load_dylib(&mut self, path) -> Result<ScriptHandle, ScriptError>`。
92. `RustScriptVM::reload(&mut self, handle)`（热替换）。
93. `JsScriptVM`：基于 RustyV8（或 quickjs/boa 作为 fallback）的 JS 运行时。
94. `JsScriptVM::evaluate(&mut self, code) -> Result<ScriptValue, ScriptError>`。
95. `TsCompiler`：将 TypeScript 编译为 JS，生成 `.d.ts` 绑定声明。
96. `PyScriptVM`：基于 PyO3 的 Python 嵌入解释器。
97. `LuaScriptVM`：基于 mlua 的 Lua 嵌入解释器。
98. `WasmScriptVM`：基于 wasmtime 的 WASM 运行时（`#[wasm_bindgen]` 风格 API）。
99. `ScriptSandboxPolicy`：每个 VM 可配置的安全策略。
100. `ScriptLimits::max_memory_bytes / max_instructions_per_tick / max_execution_time_ms`。
101. `ScriptLimits::max_stack_depth / max_string_length / max_array_length`。
102. `IOWhitelist`：允许的文件路径前缀；默认全部禁止。
103. `NetworkWhitelist`：允许的 host + 端口白名单；默认全部禁止。
104. `ScriptSandbox::enforce(&self, vm)` 在每次调用前检查策略。
105. 脚本 API 自动生成：从 Rust 模块/函数/struct 反射出脚本侧 binding。
106. `#[script_expose]` 过程宏：把函数/struct 标记为脚本可见（第一阶段手动绑定表）。
107. `Bindgen`：生成 `.d.ts` / `.pyi` / Lua `*.lua.d.ts` / WASM `*.js` 的类型声明。
108. `EntityRef`：脚本对 Entity 的句柄（opaque，带版本号）。
109. `ComponentRef`：脚本对 Component 的读写访问（以类型擦除 + schema 描述）。
110. `ResourceRef`：脚本对 Resource 的读写访问。
111. `ScriptSystem`：每帧 tick 所有脚本实例，调用 `OnUpdate(dt)`。
112. `ScriptEventBus`：`OnEnter / OnExit / OnTrigger / OnCollision / OnInput / OnTick` 路由到脚本回调。
113. `ScriptDebugger`：V8 Inspector / Python pdb 桥接 / Lua debug hook。
114. `ScriptBytecodeCache`：保存已编译字节码（JS/V8 snapshot、Python pyc、Lua bytecode）。
115. `ScriptProfiler`：每个函数的调用次数、耗时、分配统计（hook + 采样）。
116. `examples/script_rust`：dylib hot-reload，更新一个数字显示。
117. `examples/script_js`：JS 控制实体移动。
118. `examples/script_ts`：TS + 类型声明，编译后运行。
119. `examples/script_py`：Python 控制实体。
120. `examples/script_lua`：Lua 控制实体。
121. `examples/script_wasm`：WASM 执行数字/物理运算。
122. `examples/script_sandbox`：尝试无限循环、访问非法文件、访问非法网络，均被拦截。
123. `examples/script_entity`：脚本访问 Entity/Component/Resource（本 Sprint 验收主例）。
124. `cargo test -p engine-blueprint` 全部通过。
125. `cargo test -p engine-script` 全部通过。
126. `cargo clippy --workspace -- -D warnings` 通过。
127. `cargo fmt --check --workspace` 通过。
128. `cargo doc --workspace --no-deps` 成功。
129. 脚本沙盒安全性测试：死循环被指令计数 + 超时拦截。
130. 脚本沙盒安全性测试：访问未列入白名单的文件路径返回 Err。
131. 脚本沙盒安全性测试：访问未列入白名单的网络地址返回 Err。
132. 蓝图编译正确性测试：If/For/While/Switch 等价于手写代码输出。
133. CHANGELOG 记录 v0.14.0。
134. README 加入「蓝图系统」章节。
135. README 加入「脚本虚拟机与沙盒」章节。

> 以上 135 条需求构成 Sprint 14 第二部分前导清单。

---

## 三、细分需求与验收

### 3.1 蓝图图编辑内核

136. `BlueprintGraph::new() -> Self`。
137. `BlueprintGraph::with_capacity(nodes, wires) -> Self`。
138. `BlueprintGraph::nodes(&self) -> &[BlueprintNode]`。
139. `BlueprintGraph::nodes_mut(&mut self) -> &mut [BlueprintNode]`。
140. `BlueprintGraph::wires(&self) -> &[BlueprintWire]`。
141. `BlueprintGraph::wires_mut(&mut self) -> &mut [BlueprintWire]`。
142. `BlueprintGraph::add_node(&mut self, node) -> NodeId`（内部维护自增 id）。
143. `BlueprintGraph::remove_node(&mut self, id)`：移除节点及所有相关连线。
144. `BlueprintGraph::contains_node(&self, id) -> bool`。
145. `BlueprintGraph::node(&self, id) -> &BlueprintNode`。
146. `BlueprintGraph::node_mut(&mut self, id) -> &mut BlueprintNode`。
147. `BlueprintGraph::add_wire(&mut self, from, to) -> WireId`。
148. `BlueprintGraph::remove_wire(&mut self, id)`。
149. `BlueprintGraph::wires_into_pin(&self, pin) -> Vec<WireId>`。
150. `BlueprintGraph::wires_out_of_pin(&self, pin) -> Vec<WireId>`。
151. `BlueprintGraph::topological_sort(&self) -> Result<Vec<NodeId>, CycleError>`。
152. `BlueprintGraph::validate(&self) -> Result<(), BlueprintError>`：
     - 检查所有引脚类型匹配
     - 检查 exec 引脚与数据引脚不可互连
     - 检查无重复连线
     - 检查变量 get/set 名称在 graph 作用域内定义
153. `BlueprintGraph::clone() -> Self`（深拷贝）。
154. `BlueprintGraph::to_json(&self) -> String`。
155. `BlueprintGraph::from_json(json) -> Result<Self, DeserializeError>`。

### 3.2 节点系统

156. `BlueprintNode::new(kind, inputs, outputs, meta) -> Self`。
157. `BlueprintNode::id(&self) -> NodeId`。
158. `BlueprintNode::kind(&self) -> NodeKind`。
159. `BlueprintNode::name(&self) -> &str`。
160. `BlueprintNode::position(&self) -> (f32, f32)`（编辑器坐标）。
161. `BlueprintNode::set_position(&mut self, x, y)`。
162. `BlueprintNode::input_pins(&self) -> &[Pin]`。
163. `BlueprintNode::output_pins(&self) -> &[Pin]`。
164. `BlueprintNode::pin(&self, id) -> Option<&Pin>`。
165. `BlueprintNode::pin_by_name(&self, name) -> Option<&Pin>`。
166. `Pin::new(name, dir, ty, default) -> Self`。
167. `Pin::id(&self) -> PinId`。
168. `Pin::name(&self) -> &str`。
169. `Pin::direction(&self) -> PinDirection`。
170. `Pin::data_type(&self) -> PinType`。
171. `Pin::default_value(&self) -> Option<&PinValue>`。
172. `Pin::can_connect(&self, other) -> bool`（方向 + 类型检查）。
173. `PinType::is_numeric(&self) -> bool`。
174. `PinType::is_vector(&self) -> bool`。
175. `PinType::is_compatible(&self, other) -> bool`（Any 兼容一切）。
176. `PinValue::type_of(&self) -> PinType`。
177. `PinValue::coerce_to(&self, target) -> Option<PinValue>`（数字/字符串互转）。
178. `BlueprintWire::new(from, to) -> Self`。
179. `BlueprintWire::from(&self) -> PinRef`。
180. `BlueprintWire::to(&self) -> PinRef`。
181. `BlueprintWire::color(&self) -> Color`（按 PinType 着色）。

### 3.3 可视化编辑 UI

182. `BlueprintEditorView::new(graph_arc) -> Self`。
183. `BlueprintEditorView::show(&mut self, ui)`：渲染节点 + 连线 + 工具栏。
184. `NodeWidget`：渲染单个节点（标题栏 + 输入引脚列 + 输出引脚列 + 主体）。
185. `PinWidget`：圆形锚点 + 名称标签 + 颜色。
186. `NodeDragDropController::begin_drag(node_id, mouse)`。
187. `NodeDragDropController::drag_to(&mut self, mouse)`。
188. `NodeDragDropController::end_drag(&mut self)`。
189. `NodeDragDropController::snap_to_grid(&self, x, y) -> (f32, y)`（网格 `20` 像素）。
190. `WireEditor::begin_wire(from_pin, mouse)`。
191. `WireEditor::preview(&self, mouse)`：绘制贝塞尔预览线。
192. `WireEditor::end_wire(&mut self, target_pin)`：类型兼容则真正连线。
193. `WireEditor::cancel(&mut self)`。
194. `ZoomPanController::zoom(&self) -> f32`。
195. `ZoomPanController::set_zoom(&mut self, z, anchor)`：以鼠标为中心缩放。
196. `ZoomPanController::pan(&mut self, delta)`。
197. `ZoomPanController::screen_to_world(&self, p) -> (f32, f32)`。
198. `ZoomPanController::world_to_screen(&self, p) -> (f32, f32)`。
199. `NodeSearch::open(&mut self)`：弹出搜索菜单。
200. `NodeSearch::filter(&mut self, query) -> Vec<NodeKind>`：模糊匹配。
201. `NodeSearch::insert_at_cursor(&mut self, kind, cursor)`：插入并自动连输入。
202. `AutoWire::wire_compatible(graph, from_node, to_node)`：按名称+类型自动连。
203. `AutoLayout::apply(graph, algo)`：把图重排为层次化布局。
204. `AutoLayout::force_directed(graph)`：力导向布局（第一阶段仅层次化）。
205. `CommentBox::new(rect, text, color) -> Self`。
206. `CommentBox::contains(&self, node) -> bool`。
207. `CommentBox::render(&self, ui)`。
208. `RerouteNode`：一个特殊节点，接收与发射连线（作为连线路由点）。
209. `RerouteNode::split_wire(&mut self, wire_id, point)`：把一根 wire 拆为两根 + reroute。
210. 编辑器工具栏：保存 / 加载 / 自动布局 / 缩放 100% / 编译 IR。
211. 编辑器右键菜单：删除节点 / 断开连线 / 插入 reroute / 添加注释框。
212. 编辑器快捷键：Ctrl+S 保存、Ctrl+Z 撤销、Ctrl+Y 重做、Delete 删除、Ctrl+F 搜索。
213. 编辑器撤销栈：`UndoStack` 记录节点/连线的增删改操作（上限 200）。

### 3.4 蓝图编译到 IR

214. `BlueprintCompiler::new() -> Self`。
215. `BlueprintCompiler::compile(&self, graph) -> Result<BlueprintIR, CompileError>`。
216. `BlueprintCompiler::emit_const(&mut self, value) -> ConstId`。
217. `BlueprintCompiler::emit_var(&mut self, name, ty) -> VarSlot`。
218. `BlueprintCompiler::emit_instruction(&mut self, instr) -> InstrOffset`。
219. `CompileError::CycleDetected / TypeMismatch / UndefinedVariable / InvalidWire`。
220. `BlueprintIR::new() -> Self`。
221. `BlueprintIR::instructions(&self) -> &[BlueprintIRInstruction]`。
222. `BlueprintIR::constants(&self) -> &[PinValue]`。
223. `BlueprintIR::variables(&self) -> &[VariableSlot]`。
224. `BlueprintIR::functions(&self) -> &[FunctionEntry]`。
225. `BlueprintIR::entry_point(&self) -> InstrOffset`。
226. `BlueprintIRInstruction::Nop`。
227. `BlueprintIRInstruction::PushConst(ConstId)`。
228. `BlueprintIRInstruction::LoadVar(VarSlot)`。
229. `BlueprintIRInstruction::StoreVar(VarSlot)`。
230. `BlueprintIRInstruction::Dup / Swap / Pop`（栈操作）。
231. `BlueprintIRInstruction::Add / Sub / Mul / Div / Rem`。
232. `BlueprintIRInstruction::Neg / Abs / Sqrt / Sin / Cos / Tan / Log / Exp`。
233. `BlueprintIRInstruction::Lt / Le / Gt / Ge / Eq / Ne`。
234. `BlueprintIRInstruction::And / Or / Not`。
235. `BlueprintIRInstruction::Jump(offset)`。
236. `BlueprintIRInstruction::JumpIf(offset)`。
237. `BlueprintIRInstruction::JumpIfNot(offset)`。
238. `BlueprintIRInstruction::Call(fn_id, argc)`。
239. `BlueprintIRInstruction::Return`。
240. `BlueprintIRInstruction::Yield(duration)`（支持 Delay / RetriggerableDelay 的挂起）。
241. `BlueprintIRInstruction::ActivateExec(node, pin)` / `DeactivateExec(node, pin)`。
242. `BlueprintIR::serialize(&self) -> Vec<u8>`（bincode/serde+binary）。
243. `BlueprintIR::deserialize(bytes) -> Result<Self, DeserializeError>`。
244. `BlueprintInterpreter::new(ir) -> Self`。
245. `BlueprintInterpreter::stack(&self) -> &[PinValue]`。
246. `BlueprintInterpreter::variables(&self) -> &[PinValue]`。
247. `BlueprintInterpreter::pc(&self) -> InstrOffset`。
248. `BlueprintInterpreter::run(&mut self, ctx) -> Result<(), RuntimeError>`。
249. `BlueprintInterpreter::step(&mut self, ctx) -> Result<(), RuntimeError>`。
250. `BlueprintInterpreter::reset(&mut self)`。
251. `RuntimeError::StackOverflow / StackUnderflow / DivisionByZero / InvalidCast`。
252. `RuntimeError::TypeMismatch / BadInstruction / UnknownFunction`。
253. `BlueprintContext`：执行上下文（当前 world、当前 entity、delta_time、事件总线引用）。
254. `BlueprintContext::world(&self) -> &World`。
255. `BlueprintContext::entity(&self) -> Option<Entity>`。
256. `BlueprintContext::delta_time(&self) -> f32`。

### 3.5 脚本 VM 抽象

257. `ScriptVM` trait 定义：`load / call / set_global / get_global / update / gc`。
258. `ScriptVM::load(&mut self, source: ScriptSource) -> Result<ScriptHandle, ScriptError>`。
259. `ScriptVM::call(&mut self, handle, fn_name: &str, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>`。
260. `ScriptVM::call_mut(&mut self, handle, fn_name, args) -> Result<ScriptValue, ScriptError>`。
261. `ScriptVM::set_global(&mut self, name: &str, value: ScriptValue)`。
262. `ScriptVM::get_global(&self, name: &str) -> Option<ScriptValue>`。
263. `ScriptVM::update(&mut self, dt: f32)`。
264. `ScriptVM::gc(&mut self)`（垃圾回收）。
265. `ScriptSource::Path(PathBuf) / Code(String) / Bytes(Vec<u8>)`。
266. `ScriptValue::Null / Bool(bool) / Int(i64) / Float(f64) / String(String)`。
267. `ScriptValue::Array(Vec<ScriptValue>)`。
268. `ScriptValue::Map(HashMap<String, ScriptValue>)`。
269. `ScriptValue::Entity(EntityRef)`。
270. `ScriptValue::UserData(TypeId, Arc<dyn Any>)`。
271. `ScriptValue::type_name(&self) -> &str`。
272. `ScriptValue::to_bool(&self) -> Option<bool>`。
273. `ScriptValue::to_int(&self) -> Option<i64>`。
274. `ScriptValue::to_float(&self) -> Option<f64>`。
275. `ScriptValue::to_string_lossy(&self) -> String`。
276. `ScriptError::CompileError(String) / RuntimeError(String) / Timeout / MemoryLimit`。
277. `ScriptError::NotFound(String) / InvalidArg(String) / IoError(io::Error)`。
278. `ScriptHandle = u64`（强类型 newtype）。
279. `ScriptInstance`：`vm_handle + fn_table + state + last_error`。
280. `ScriptInstance::new(handle) -> Self`。
281. `ScriptInstance::has(&self, name) -> bool`。
282. `ScriptInstance::last_error(&self) -> Option<&ScriptError>`。

### 3.6 各语言绑定

283. `RustScriptVM`：`libloading::Library` 装载 `.so/.dll/.dylib`。
284. `RustScriptVM::new() -> Self`。
285. `RustScriptVM::load_dylib(&mut self, path) -> Result<ScriptHandle, ScriptError>`。
286. `RustScriptVM::reload(&mut self, handle)`：重新加载并保留状态迁移函数。
287. `RustScriptVM::export(&self, handle, name) -> Option<unsafe extern "C" fn(...)>`。
288. `JsScriptVM::new() -> Self`（默认 quickjs，可选 RustyV8 feature）。
289. `JsScriptVM::evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>`。
290. `JsScriptVM::register_fn(&mut self, name, f)`：把 Rust 闭包注入为 JS 函数。
291. `JsScriptVM::register_class(&mut self, name, methods)`：把 Rust struct 注入 JS 对象。
292. `TsCompiler::compile(&self, source) -> Result<JsCode, String>`（内部调用 `swc` 或 tsc CLI）。
293. `TsCompiler::emit_declarations(&self, source) -> Result<String, String>`。
294. `PyScriptVM::new() -> Self`（`pyo3::Python::with_gil`）。
295. `PyScriptVM::evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>`。
296. `PyScriptVM::register_module(&mut self, name, module_fn)`：`#[pymodule]` 导出。
297. `LuaScriptVM::new() -> Self`（`mlua::Lua::new()`）。
298. `LuaScriptVM::evaluate(&mut self, code: &str) -> Result<ScriptValue, ScriptError>`。
299. `LuaScriptVM::create_table(&mut self) -> ScriptValue`。
300. `WasmScriptVM::new() -> Self`（`wasmtime::Engine` + `Store`）。
301. `WasmScriptVM::load_bytes(&mut self, bytes: &[u8]) -> Result<ScriptHandle, ScriptError>`。
302. `WasmScriptVM::call_export(&mut self, handle, name, args) -> Result<ScriptValue, ScriptError>`。
303. `WasmScriptVM::link_host_fn(&mut self, module, name, f)`：把 host 函数暴露给 WASM。

### 3.7 沙盒安全

304. `ScriptLimits::default()`：max_memory=64MB, max_instructions=1M/tick, max_time=5ms。
305. `ScriptLimits::with_memory(mb)` / `with_instructions(n)` / `with_time_ms(ms)`。
306. `ScriptSandboxPolicy::new(limits, io, net)`。
307. `IOWhitelist::allow_path_prefix(&mut self, path)`。
308. `IOWhitelist::is_allowed(&self, path) -> bool`。
309. `NetworkWhitelist::allow_host(&mut self, host, port)`。
310. `NetworkWhitelist::is_allowed(&self, host, port) -> bool`。
311. `NetworkWhitelist::block_all(&mut self)`（默认状态）。
312. `InstructionCounter`：在 Js/Lua/Py 解释器中以 hook 或 bytecode patch 方式计数。
313. `InstructionCounter::inc(&mut self, n)`，超出上限触发 `ScriptError::InstructionLimit`。
314. `TimeoutGuard`：跨线程定时器，到时主动中断 VM 执行。
315. `MemoryGuard`：以分配器 hook 统计当前 VM 字节数（WASM 可用线性内存限制，其他语言估算）。
316. `FileIoHook`：在 `std::fs::*` 之外再实现脚本侧文件接口，强制走 `IOWhitelist`。
317. `NetworkHook`：脚本侧网络 API 强制走 `NetworkWhitelist`。
318. 沙盒默认禁用一切 IO/网络；必须在 `ScriptSandboxPolicy` 显式开启。
319. `examples/script_sandbox`：
     - 无限循环脚本在 `max_instructions` 后被终止。
     - 尝试打开 `/etc/passwd` 失败。
     - 尝试连接 `8.8.8.8:53` 失败。
320. 沙盒审计：每次被拦截的调用在日志输出，并可订阅事件 `SandboxViolationEvent`。

### 3.8 反射与绑定生成

321. `ScriptReflect`：对脚本暴露的 Rust 类型/函数的描述。
322. `ScriptReflect::type_name(&self) -> &str`。
323. `ScriptReflect::fields(&self) -> &[FieldDesc]`。
324. `ScriptReflect::methods(&self) -> &[MethodDesc]`。
325. `FieldDesc::new(name, ty, readonly)`。
326. `MethodDesc::new(name, params, ret)`。
327. `ScriptType::Null / Bool / Int / Float / String / Array / Map / Entity / UserType(String)`。
328. `Bindgen::new() -> Self`。
329. `Bindgen::add_type(&mut self, reflect)`。
330. `Bindgen::add_function(&mut self, name, signature)`。
331. `Bindgen::emit_d_ts(&self) -> String`（TypeScript 声明）。
332. `Bindgen::emit_pyi(&self) -> String`（Python stub）。
333. `Bindgen::emit_lua_defs(&self) -> String`（Lua 注释/EmmyLua）。
334. `Bindgen::emit_wasm_js(&self) -> String`（WASM JS 粘合层）。
335. `#[script_expose]` 过程宏：把 `fn` 加入到一个全局 `inventory::collect!` 注册表。
336. `#[script_expose]` 对 `struct` 生成 `ScriptReflect` 实现（第一阶段用手动 table）。
337. `EntityRef`：`Entity(index, generation)`，脚本不可直接修改字段。
338. `EntityRef::id(&self) -> u64`。
339. `EntityRef::is_alive(&self, world) -> bool`。
340. `ComponentRef::get(&self, entity) -> Option<ScriptValue>`。
341. `ComponentRef::set(&self, entity, value) -> Result<(), ScriptError>`。
342. `ResourceRef::get(&self) -> ScriptValue`。
343. `ResourceRef::set(&self, value)`。
344. `ScriptSystem::new(world) -> Self`。
345. `ScriptSystem::update(&mut self, world, dt)`：每个实例调用 `OnUpdate(dt)`。
346. `ScriptEventBus::emit(&mut self, event)`：事件派发到 `OnEnter/OnExit/OnTrigger/OnCollision/OnInput`。

### 3.9 脚本调试器 / 字节码缓存 / Profiler

347. `ScriptDebugger`：为 `JsScriptVM` 接入 V8 Inspector；第一阶段以断点回调为主。
348. `ScriptDebugger::set_breakpoint(&mut self, source, line)`。
349. `ScriptDebugger::remove_breakpoint(&mut self, id)`。
350. `ScriptDebugger::pause(&mut self)` / `resume(&mut self)` / `step(&mut self)`。
351. `ScriptDebugger::frame(&self) -> Option<StackFrame>`。
352. `ScriptDebugger::locals(&self) -> Vec<(String, ScriptValue)>`。
353. Python 调试桥：允许在脚本侧 `breakpoint()`，路由到 IDE 协议。
354. Lua 调试：`debug.sethook` 作为行/调用 hook。
355. `ScriptBytecodeCache::new(dir) -> Self`。
356. `ScriptBytecodeCache::get(&self, key) -> Option<Vec<u8>>`。
357. `ScriptBytecodeCache::insert(&mut self, key, bytes)`。
358. `ScriptBytecodeCache::clear(&mut self)`。
359. `ScriptBytecodeCache` 以源文件 hash 作为 key，修改时自动失效。
360. `ScriptProfiler`：记录每个 `call` 的开始/结束时间、调用次数、分配字节数估算。
361. `ScriptProfiler::begin(&mut self, fn_name)` / `end(&mut self, fn_name)`。
362. `ScriptProfiler::report(&self) -> ProfileReport`。
363. `ProfileReport::top_hot(&self, n) -> Vec<ProfileEntry>`。
364. `ProfileEntry::name / count / total_ms / avg_ms / alloc_bytes`。
365. `ProfilerHook`：在每次 `ScriptVM::call` 前后插入计时事件。

### 3.10 示例与测试

366. `examples/blueprint_hello`：BeginPlay → PrintString("Hello Blueprint")。
367. `examples/blueprint_movement`：Tick → GetInputAxis → AddActorWorldOffset（验收主例）。
368. `examples/blueprint_spawn`：定时器每隔 1 秒 SpawnActor。
369. `examples/blueprint_timer`：SetTimer + ClearTimer 按钮切换。
370. `examples/blueprint_ui`：UI 按钮点击 → CallEventDispatcher → PrintString。
371. `examples/blueprint_animation`：按键 → AnimationController SetParameter。
372. `examples/blueprint_physics`：LineTraceByChannel 命中 → AddImpulse。
373. `examples/script_rust`：Rust dylib 暴露 `on_update(world, dt)`，动态更新 UI 数字。
374. `examples/script_js`：JS 读取 `Input.axis("Horizontal")`，设置 `Entity.position`。
375. `examples/script_ts`：TS 源码 + 自动生成 `.d.ts` + tsconfig.json，`cargo run` 自动编译。
376. `examples/script_py`：Python 脚本控制实体跳跃与移动。
377. `examples/script_lua`：Lua 脚本驱动 entity 按正弦曲线移动。
378. `examples/script_wasm`：WASM 计算物理积分并通过 host 函数设置 entity 位置。
379. `examples/script_sandbox`：演示沙盒拦截（无限循环、非法文件、非法网络）。
380. `examples/script_entity`：脚本访问 Entity/Component/Resource 的完整示例（验收主例）。
381. 单测 `BlueprintGraph::add_node` + `remove_node` 保持 id 唯一。
382. 单测 `BlueprintGraph::validate` 对类型不匹配返回 `Err`。
383. 单测 `BlueprintGraph::topological_sort` 对含环图返回 `CycleError`。
384. 单测 `BlueprintCompiler::compile` 对 hello graph 生成 ≥ 5 条指令。
385. 单测 `BlueprintInterpreter` 对 `If(cond, a=1, a=2)` 后 `a==cond?1:2`。
386. 单测 `BlueprintInterpreter` 对 `For(i, 0, 10) { sum += i }`，`sum == 45`。
387. 单测 `BlueprintInterpreter` 对 `While(i<3) { i+=1 }` 正确终止。
388. 单测 `BlueprintInterpreter` 对 `Switch(value)` 的分支选择。
389. 单测 `Pin::can_connect` 类型方向检查。
390. 单测 `PinValue::coerce_to` 字符串↔数字。
391. 单测 `ScriptValue::from/to` 各种类型。
392. 单测 `RustScriptVM::load_dylib` 对 mock `.so` 成功加载。
393. 单测 `JsScriptVM::evaluate` 对 `1+2` 返回 `ScriptValue::Float(3.0)`。
394. 单测 `TsCompiler::compile` 对 TS 源码生成可被 JS 运行代码。
395. 单测 `PyScriptVM` 对 `x = 1; x + 2` 返回 3。
396. 单测 `LuaScriptVM` 对 `return 1 + 2` 返回 3。
397. 单测 `WasmScriptVM` 对 `add(i32, i32) -> i32` 模块调用正确。
398. 单测 `ScriptLimits` + `InstructionCounter` 超出上限返回 `ScriptError::InstructionLimit`。
399. 单测 `IOWhitelist` 对不在白名单的路径返回 false。
400. 单测 `NetworkWhitelist` 对不在白名单的 host 返回 false。
401. 单测 `AutoLayout` 把 graph 按拓扑序左到右排布。
402. 单测 `RerouteNode::split_wire` 把 wire 拆为两段并记录新 id。
403. 单测 `BlueprintHotReload`：模拟文件变更 → 重新编译 IR。
404. 单测 `ScriptBytecodeCache::get/insert/clear`。
405. 单测 `ScriptProfiler` 两次 `begin/end` 后 `count == 2`。
406. `cargo test -p engine-blueprint` 全部通过。
407. `cargo test -p engine-script` 全部通过。
408. `cargo clippy --workspace -- -D warnings` 通过。
409. `cargo fmt --check --workspace` 通过。
410. `cargo doc --workspace --no-deps` 成功。
411. CI 三平台 green。
412. CHANGELOG 记录 v0.14.0（列出：蓝图 graph、IR、解释器、节点库、编辑器 UI、脚本 VM、各语言绑定、沙盒、profiler、examples）。
413. README.md 加入「蓝图系统」章节。
414. README.md 加入「脚本虚拟机与沙盒」章节。
415. README.md 加入「脚本绑定与反射」章节。
416. 公开 API doc comment 覆盖率 100%。
417. `unsafe` 块控制：`engine-blueprint <= 3`，`engine-script <= 5`（集中在 FFI/VM 底层）。
418. 新增 example 工程 >= 14 个。
419. `examples/blueprint_movement` 可通过 WASD 控制角色移动。
420. `examples/script_entity` 可在脚本侧读写 entity 的组件与资源。

---

## 四、验收标准

- [ ] `cargo run --example blueprint_hello` 正常运行，控制台输出 "Hello Blueprint"
- [ ] `cargo run --example blueprint_movement` 可通过 WASD 控制角色位移
- [ ] `cargo run --example blueprint_spawn` 周期性生成 Actor
- [ ] `cargo run --example blueprint_timer` SetTimer/ClearTimer 切换按钮可响应
- [ ] `cargo run --example blueprint_ui` 按钮点击触发蓝图事件
- [ ] `cargo run --example blueprint_animation` 按键切换动画状态机参数
- [ ] `cargo run --example blueprint_physics` LineTrace 命中后给对象加冲量
- [ ] `cargo run --example script_rust` dylib 热重载可观察到运行态变化
- [ ] `cargo run --example script_js` JS 脚本控制实体移动
- [ ] `cargo run --example script_ts` TS 自动编译并运行
- [ ] `cargo run --example script_py` Python 控制实体
- [ ] `cargo run --example script_lua` Lua 控制实体
- [ ] `cargo run --example script_wasm` WASM 参与数值计算
- [ ] `cargo run --example script_sandbox` 展示沙盒拦截日志
- [ ] `cargo run --example script_entity` 脚本读写 entity/component/resource
- [ ] `cargo test -p engine-blueprint` 全部通过
- [ ] `cargo test -p engine-script` 全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] 三平台 CI green
- [ ] CHANGELOG 记录 v0.14.0
- [ ] README 加入「蓝图系统」「脚本虚拟机与沙盒」章节

---

## 五、下一个 Sprint

Sprint 15 将引入编辑器资产流水线（导入/导出/Inspector 可视化/SceneGraph/序列化）与网络多人雏形（replication + 快照），打通蓝图 + 脚本 + 资源 + 网络的四象限能力。
