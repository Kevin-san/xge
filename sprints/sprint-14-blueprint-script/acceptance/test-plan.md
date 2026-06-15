# 测试计划（Test Plan）

## 1. 概述

本文档定义 Sprint 14 的测试策略、测试用例清单与验收标准。

---

## 2. 测试分层

| 层级 | 范围 | 工具 |
|------|------|------|
| 单元测试 | 核心数据结构、编译器、解释器、VM | `cargo test` |
| 集成测试 | 模块间交互、example 编译运行 | `cargo test --test *` |
| 沙盒安全测试 | 资源限制、IO/网络拦截 | 手动测试 + 自动化 |
| CI 测试 | 三平台（Linux/macOS/Windows）| GitHub Actions |

---

## 3. 单元测试清单

### 3.1 BlueprintGraph（需求 381）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-BC-001 | `add_node` + `remove_node` 保持 ID 唯一 | 连续 add/remove | 再次 add 的 ID 不重复 |
| UT-BC-002 | `validate` 对类型不匹配返回 `Err` | Bool 输出 → Int 输入 | `Err(TypeMismatch)` |
| UT-BC-003 | `topological_sort` 对含环图返回 `CycleError` | A→B→C→A 的图 | `Err(CycleError)` |
| UT-BC-004 | `clone` 深拷贝后修改不相互影响 | graph.clone() | 两个 graph 独立 |
| UT-BC-005 | `to_json` / `from_json` 往返一致 | 任意 graph | `from_json(to_json(g)) == g` |

### 3.2 Pin 与类型系统（需求 382-383）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-BC-006 | `Pin::can_connect` 方向检查 | Input vs Input | `false` |
| UT-BC-007 | `Pin::can_connect` 类型检查 | Float vs Bool | `false` |
| UT-BC-008 | `Pin::can_connect` Any 兼容一切 | Any vs Bool | `true` |
| UT-BC-009 | `PinValue::coerce_to` 字符串→数字 | `"123"` → Int | `Some(Int(123))` |
| UT-BC-010 | `PinValue::coerce_to` 数字→字符串 | `Float(3.14)` → String | `Some(String("3.14"))` |

### 3.3 编译器（需求 384）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-CP-001 | `compile` 对 hello graph 生成 ≥ 5 条指令 | BeginPlay→PrintString | `ir.instructions().len() >= 5` |
| UT-CP-002 | If 节点编译为 JumpIf | `If(cond, a, b)` | IR 含 `JumpIf` |
| UT-CP-003 | For 节点编译为 Jump 循环 | `For(i, 0, 10)` | IR 含 `Jump` 回跳 |
| UT-CP-004 | While 节点编译为条件跳转 | `While(i<3)` | 正确终止 |
| UT-CP-005 | Switch 节点编译为多分支 | 4 个分支 | 4 个 `Jump` |

### 3.4 解释器（需求 385-388）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-CP-006 | If 执行 true 分支 | `If(true, a=1, a=2)` | `a == 1` |
| UT-CP-007 | If 执行 false 分支 | `If(false, a=1, a=2)` | `a == 2` |
| UT-CP-008 | For 求和 | `For(i, 0, 10) { sum += i }` | `sum == 45` |
| UT-CP-009 | While 正确终止 | `While(i<3) { i+=1 }` | `i == 3` |
| UT-CP-010 | Switch 分支选择 | `Switch(value=2)` | 执行第 2 分支 |

### 3.5 脚本 VM（需求 389-398）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-VM-001 | `RustScriptVM::load_dylib` 成功加载 mock .so | 有效 dylib 路径 | `Ok(handle)` |
| UT-VM-002 | `JsScriptVM::evaluate` 算术运算 | `"1+2"` | `ScriptValue::Float(3.0)` |
| UT-VM-003 | `TsCompiler::compile` TS→JS 可运行 | `let x: number = 1;` | 生成的 JS 可执行 |
| UT-VM-004 | `PyScriptVM` 表达式求值 | `"x = 1; x + 2"` | `ScriptValue::Int(3)` |
| UT-VM-005 | `LuaScriptVM` 表达式求值 | `"return 1 + 2"` | `ScriptValue::Int(3)` |
| UT-VM-006 | `WasmScriptVM` 调用 add 函数 | add(i32, i32) | `ScriptValue::Int(5)` |
| UT-VM-007 | `ScriptValue` 类型转换 Bool | `ScriptValue::Bool(true).to_int()` | `Some(1)` |
| UT-VM-008 | `ScriptValue` 类型转换 Float | `ScriptValue::Int(42).to_float()` | `Some(42.0)` |

### 3.6 沙盒（需求 398-400）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-SB-001 | `InstructionCounter` 超限返回错误 | 计数超过 limit | `Err(InstructionLimit)` |
| UT-SB-002 | `IOWhitelist` 非法路径返回 false | `"/etc/passwd"` 不在白名单 | `false` |
| UT-SB-003 | `NetworkWhitelist` 非法 host 返回 false | `"8.8.8.8:53"` 不在白名单 | `false` |
| UT-SB-004 | 无限循环被指令计数拦截 | `while(true){}` | 约 1M 指令后终止 |
| UT-SB-005 | 非法文件访问被拦截 | `fs.readFile("/etc/passwd")` | `Err(IoError)` |
| UT-SB-006 | 非法网络访问被拦截 | `fetch("http://evil.com")` | `Err(NetworkError)` |

### 3.7 编辑器 UI（需求 401-402）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-NG-001 | `AutoLayout` 拓扑排序左到右 | 任意 graph | 节点 x 坐标递增 |
| UT-NG-002 | `RerouteNode::split_wire` 拆分连线 | wire_id + point | 生成两根新 wire |

### 3.8 热重载与缓存（需求 403）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-HR-001 | `BlueprintHotReload` 文件变更后重新编译 | 修改 .bp.json | 重新生成 IR |
| UT-BC-001 | `ScriptBytecodeCache::get/insert/clear` | key + bytes | 正确存取 |

### 3.9 Profiler（需求 405）

| 用例 ID | 描述 | 输入 | 预期输出 |
|---------|------|------|----------|
| UT-PR-001 | `ScriptProfiler` begin/end 计数 | 两次 begin/end | `count == 2` |

---

## 4. 集成测试清单

### 4.1 Example 编译测试

| 用例 ID | 命令 | 预期结果 |
|---------|------|----------|
| IT-EX-001 | `cargo build --example blueprint_hello` | 编译成功 |
| IT-EX-002 | `cargo build --example blueprint_movement` | 编译成功 |
| IT-EX-003 | `cargo build --example script_js` | 编译成功 |
| IT-EX-004 | `cargo build --example script_entity` | 编译成功 |
| IT-EX-005 | `cargo build --example script_sandbox` | 编译成功 |

### 4.2 Example 运行测试

| 用例 ID | 命令 | 预期结果 |
|---------|------|----------|
| IT-RUN-001 | `cargo run --example blueprint_hello` | 输出 "Hello Blueprint" |
| IT-RUN-002 | `cargo run --example blueprint_movement` | WASD 控制角色移动 |
| IT-RUN-003 | `cargo run --example script_sandbox` | 展示沙盒拦截日志 |
| IT-RUN-004 | `cargo run --example script_entity` | 脚本读写 entity/component/resource |

---

## 5. 代码质量检查

| 检查项 | 命令 | 验收标准 |
|--------|------|----------|
| 单元测试 | `cargo test -p engine-blueprint` | 全部通过 |
| 单元测试 | `cargo test -p engine-script` | 全部通过 |
| Clippy | `cargo clippy --workspace -- -D warnings` | 通过 |
|Fmt | `cargo fmt --check --workspace` | 通过 |
| Doc | `cargo doc --workspace --no-deps` | 成功生成 |

---

## 6. CI 测试矩阵

| 平台 | 工具链 | 架构 |
|------|--------|------|
| Linux | stable | x86_64 |
| macOS | stable | x86_64, aarch64 |
| Windows | stable | x86_64 |

---

## 7. 验收标准汇总

### 7.1 必须通过

- [ ] `cargo test -p engine-blueprint` 全部通过（≥ 50 个测试）
- [ ] `cargo test -p engine-script` 全部通过（≥ 40 个测试）
- [ ] `cargo clippy --workspace -- -D warnings` 通过
- [ ] `cargo fmt --check --workspace` 通过
- [ ] `cargo doc --workspace --no-deps` 成功
- [ ] 三平台 CI green

### 7.2 示例验收

- [ ] `examples/blueprint_movement` 可通过 WASD 控制角色移动
- [ ] `examples/script_entity` 可在脚本侧读写 entity/component/resource
- [ ] `examples/script_sandbox` 展示沙盒拦截日志

### 7.3 文档

- [ ] CHANGELOG 记录 v0.14.0
- [ ] README.md 加入「蓝图系统」章节
- [ ] README.md 加入「脚本虚拟机与沙盒」章节
- [ ] 公开 API doc comment 覆盖率 100%

---

## 8. 性能基准

| 指标 | 目标 |
|------|------|
| 蓝图编译（100 节点）| < 100ms |
| IR 解释执行（1000 指令）| < 1ms |
| JS 引擎初始化 | < 50ms |
| 沙盒指令计数开销 | < 5% CPU |

---

## 9. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 多语言 VM 依赖项安装复杂 | 高 | 使用 feature flag，默认仅启用 JS |
| WASM 线性内存限制实现 | 中 | 使用 wasmtime 内置限制 |
| 沙盒安全性验证困难 | 中 | 手动测试 + 自动化边界测试 |
| 热重载状态迁移 | 低 | 第一阶段不实现状态迁移 |
