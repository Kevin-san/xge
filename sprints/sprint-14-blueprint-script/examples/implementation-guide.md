# 示例实现指南（Implementation Guide）

## 1. 概述

本文档为 Sprint 14 的示例实现提供指导，包括 14 个 blueprint 示例和 8 个 script 示例。

---

## 2. Blueprint 示例

### 2.1 blueprint_hello

**目的**：BeginPlay → PrintString hello world（入门示例）

**实现步骤**：
1. 创建 `BlueprintGraph`
2. 添加 `BeginPlay` 事件节点（输出 Exec）
3. 添加 `PrintString` 节点（输入 Exec + 输入 String）
4. 用 `BlueprintWire` 连接 `BeginPlay` → `PrintString`
5. 为 `PrintString` 的 String 引脚设置默认值 `"Hello Blueprint"`
6. 编译为 IR
7. 在 `OnBeginPlay` 回调中执行 `BlueprintInterpreter::run`

**验收**：控制台输出 "Hello Blueprint"

---

### 2.2 blueprint_movement（验收主例）

**目的**：Tick → GetInputAxis → AddActorWorldOffset（WASD 控制角色）

**实现步骤**：
1. 创建蓝图图，包含以下节点：
   - `Tick` 事件节点（输出 Exec + DeltaTime）
   - `GetInputAxis` 节点（输入 String "Horizontal"/"Vertical"，输出 Float）
   - `Multiply` 节点（输入 Float × Float，输出 Float）
   - `GetActorLocation` 节点（输出 Vec3）
   - `BreakVec3` 节点（输入 Vec3，输出 Float x/y/z）
   - `MakeVec3` 节点（输入 x/y/z，输出 Vec3）
   - `AddActorWorldOffset` 节点（输入 Vec3）
2. 连接：`Tick → GetInputAxis → Multiply(x2) → ... → AddActorWorldOffset`
3. 编译执行
4. 绑定键盘输入到 InputAxis

**验收**：`cargo run --example blueprint_movement` 可通过 WASD 控制角色位移

---

### 2.3 blueprint_spawn

**目的**：定时器每隔 1 秒 SpawnActor

**实现步骤**：
1. 创建蓝图：
   - `Tick` → `SetTimer`（输出 Exec + TimerHandle）
   - Timer 触发 → `SpawnActor`（输入 Class，输出 Actor）
   - `IsTimerActive` 检查循环
2. 设置定时器间隔为 1.0 秒，循环为 true

---

### 2.4 blueprint_timer

**目的**：SetTimer + ClearTimer 按钮切换

**实现步骤**：
1. UI 按钮节点 → `CallEventDispatcher` 或直接 → `SetTimer`
2. 另一个按钮 → `ClearTimer`
3. TimerHandle 作为变量在两者之间共享

---

### 2.5 blueprint_ui

**目的**：UI 按钮点击驱动蓝图节点

**实现步骤**：
1. UI 按钮 `OnClicked` 事件
2. 连接到 `CallEventDispatcher` 或 `PrintString`
3. 通过 `BlueprintEventBus` 路由 UI 事件到蓝图

---

### 2.6 blueprint_animation

**目的**：按键 → AnimationController SetParameter

**实现步骤**：
1. `OnKeyDown` 事件（检测特定键）
2. `SetParameter` 节点（输入参数名 + 值）
3. 连接到 `AnimationController`

---

### 2.7 blueprint_physics

**目的**：LineTraceByChannel 命中 → AddImpulse

**实现步骤**：
1. `Tick` → `LineTraceByChannel`（输入起点、方向、长度，输出 Hit Actor）
2. `Branch` 判断是否命中
3. 命中时 → `AddImpulse`（输入 Actor + Vec3）

---

## 3. Script 示例

### 3.1 script_rust

**目的**：Rust dylib 热重载，更新一个数字显示

**实现步骤**：
1. 创建 `rust-plugin` crate，导出 `#[no_mangle] pub extern "C" fn on_update(world: *mut World, dt: f32)`
2. 在主程序中 `RustScriptVM::load_dylib("target/release/librust_plugin.so")`
3. 每帧调用 `vm.call(handle, "on_update", &[ScriptValue::Float(dt)])`
4. 修改插件代码后 `vm.reload(handle)` 重新加载

**验收**：修改 `.so` 后无需重启程序即可观察到数值变化

---

### 3.2 script_js

**目的**：JS 控制实体移动

**实现步骤**：
1. 创建 `JsScriptVM`
2. 注册 host 函数：
   ```javascript
   engine.setPosition(entity, x, y, z);
   engine.getInputAxis(name);
   ```
3. 执行 JS 脚本：
   ```javascript
   let axis = engine.getInputAxis("Horizontal");
   let pos = engine.getPosition(player);
   pos.x += axis * 5.0 * dt;
   engine.setPosition(player, pos.x, pos.y, pos.z);
   ```

**验收**：`cargo run --example script_js` 实体响应 JS 脚本移动

---

### 3.3 script_ts

**目的**：TS 源码 + 自动生成 `.d.ts` + tsconfig.json

**实现步骤**：
1. `TsCompiler::compile` 将 TS 转为 JS
2. `TsCompiler::emit_declarations` 生成 `.d.ts`
3. 使用 Bindgen 生成 `engine.d.ts`
4. 运行 JS

**验收**：`cargo run --example script_ts` 自动编译 TS 并运行

---

### 3.4 script_py

**目的**：Python 控制实体

**实现步骤**：
1. 创建 `PyScriptVM`
2. 注册 host 函数为 Python 模块
3. 执行 Python 脚本：
   ```python
   from engine import get_entity, set_position
   entity = get_entity("player")
   x, y, z = get_position(entity)
   set_position(entity, x + 1, y, z)
   ```

**验收**：`cargo run --example script_py` Python 脚本控制实体

---

### 3.5 script_lua

**目的**：Lua 脚本驱动 entity 按正弦曲线移动

**实现步骤**：
1. 创建 `LuaScriptVM`
2. 注册 host 函数
3. 执行 Lua 脚本：
   ```lua
   local entity = engine.get_entity("ball")
   local x = engine.get_x(entity)
   local y = math.sin(x * 0.1) * 10
   engine.set_position(entity, x + 0.1, y, 0)
   ```

---

### 3.6 script_wasm

**目的**：WASM 计算物理积分并通过 host 函数设置 entity 位置

**实现步骤**：
1. 编写 Rust 并编译为 WASM：
   ```rust
   #[no_mangle]
   pub extern "C" fn physics_step(pos: f32, vel: f32, dt: f32) -> (f32, f32) {
       let new_pos = pos + vel * dt;
       let new_vel = vel - 9.8 * dt;
       (new_pos, new_vel)
   }
   ```
2. `WasmScriptVM::load_bytes` 加载 WASM
3. 链接 host 函数 `engine.set_position`
4. 每帧调用 WASM 的 `physics_step`

---

### 3.7 script_sandbox（沙盒演示）

**目的**：展示沙盒拦截（无限循环、非法文件、非法网络）

**实现步骤**：
1. 创建 `ScriptSandboxPolicy`：
   - `ScriptLimits::with_instructions(1000)` // 小限制
   - `IOWhitelist::new()` // 空的
   - `NetworkWhitelist::block_all()`
2. 执行 JS：
   ```javascript
   // 无限循环
   while(true) {} // 应被 instruction limit 中断

   // 非法文件
   fs.readFile("/etc/passwd") // 应返回错误

   // 非法网络
   fetch("http://evil.com") // 应返回错误
   ```
3. 检查日志或捕获 `ScriptError`

**验收**：
- 无限循环在 1000 指令后被终止
- 尝试打开 `/etc/passwd` 返回 Err
- 尝试连接 `8.8.8.8:53` 返回 Err

---

### 3.8 script_entity（验收主例）

**目的**：脚本访问 Entity/Component/Resource

**实现步骤**：
1. 在 Rust 侧注册 `EntityRef`、`ComponentRef`、`ResourceRef`
2. JS 脚本示例：
   ```javascript
   let player = engine.spawn_entity("Player");
   engine.set_component(player, "Position", { x: 0, y: 0, z: 0 });
   engine.set_component(player, "Velocity", { x: 1, y: 0, z: 0 });

   let pos = engine.get_component(player, "Position");
   let vel = engine.get_component(player, "Velocity");
   pos.x += vel.x * dt;
   engine.set_component(player, "Position", pos);

   let gravity = engine.get_resource("Gravity");
   vel.y -= gravity.value * dt;
   ```

**验收**：`cargo run --example script_entity` 可在脚本侧读写 entity/component/resource

---

## 4. 示例运行命令

| 示例 | 命令 |
|------|------|
| blueprint_hello | `cargo run --example blueprint_hello` |
| blueprint_movement | `cargo run --example blueprint_movement` |
| blueprint_spawn | `cargo run --example blueprint_spawn` |
| blueprint_timer | `cargo run --example blueprint_timer` |
| blueprint_ui | `cargo run --example blueprint_ui` |
| blueprint_animation | `cargo run --example blueprint_animation` |
| blueprint_physics | `cargo run --example blueprint_physics` |
| script_rust | `cargo run --example script_rust` |
| script_js | `cargo run --example script_js` |
| script_ts | `cargo run --example script_ts` |
| script_py | `cargo run --example script_py` |
| script_lua | `cargo run --example script_lua` |
| script_wasm | `cargo run --example script_wasm` |
| script_sandbox | `cargo run --example script_sandbox` |
| script_entity | `cargo run --example script_entity` |

---

## 5. 依赖项配置

### 5.1 Cargo.toml

```toml
[dependencies]
engine-blueprint = { path = "../engine-blueprint" }
engine-script = { path = "../engine-script" }
engine-ecs = { path = "../engine-ecs" }

# JS
rquickjs = { version = "1", optional = true }

# Python
pyo3 = { version = "0.20", optional = true }

# Lua
mlua = { version = "0.9", optional = true }

# WASM
wasmtime = { version = "20", optional = true }

[features]
default = []
sandbox = ["engine-script/sandbox"]
script-js = ["rquickjs"]
script-py = ["pyo3"]
script-lua = ["mlua"]
script-wasm = ["wasmtime"]
```

### 5.2 条件编译

```rust
// 示例使用
#[cfg(feature = "script-js")]
let mut vm = JsScriptVM::new();

#[cfg(feature = "script-py")]
let mut vm = PyScriptVM::new();

// 沙盒示例
#[cfg(feature = "sandbox")]
let policy = ScriptSandboxPolicy::new(limits, io_whitelist, net_whitelist);
```
