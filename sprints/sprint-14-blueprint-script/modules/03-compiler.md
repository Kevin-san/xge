# 模块三：蓝图编译器（Blueprint Compiler）

## 1. 模块概述

Blueprint Compiler 负责将 `BlueprintGraph` 编译为 `BlueprintIR`（中间表示），再由 `BlueprintInterpreter` 执行。编译器进行语法分析、类型检查、拓扑排序，生成指令序列、常量池、变量槽。

核心组件：
- `BlueprintCompiler`：图到 IR 的编译器
- `BlueprintIR`：编译产物，指令序列 + 常量池 + 变量槽
- `BlueprintInterpreter`：基于栈的解释执行器
- `BlueprintContext`：执行上下文
- `BlueprintMacro` / `BlueprintFunctionLibrary`：宏与函数库
- `BlueprintDebugger`：调试器

---

## 2. 需求清单

### 2.1 BlueprintCompiler（需求 41-42, 91, 214-220）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-001 | `BlueprintCompiler::new() -> Self` 创建编译器 | P0 |
| REQ-CP-002 | `BlueprintCompiler::compile(&self, graph) -> Result<BlueprintIR, CompileError>` 编译图到 IR | P0 |
| REQ-CP-003 | `BlueprintCompiler::emit_const(&mut self, value) -> ConstId` 发射常量 | P1 |
| REQ-CP-004 | `BlueprintCompiler::emit_var(&mut self, name, ty) -> VarSlot` 发射变量槽 | P1 |
| REQ-CP-005 | `BlueprintCompiler::emit_instruction(&mut self, instr) -> InstrOffset` 发射指令 | P1 |
| REQ-CP-006 | `CompileError::CycleDetected / TypeMismatch / UndefinedVariable / InvalidWire` | P0 |
| REQ-CP-007 | 编译器支持 If/For/While/Switch 等控制流节点编译 | P0 |

### 2.2 BlueprintIR（需求 43, 93, 220-243）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-008 | `BlueprintIR::new() -> Self` 创建 IR | P0 |
| REQ-CP-009 | `BlueprintIR::instructions(&self) -> &[BlueprintIRInstruction]` 获取指令序列 | P0 |
| REQ-CP-010 | `BlueprintIR::constants(&self) -> &[PinValue]` 获取常量池 | P0 |
| REQ-CP-011 | `BlueprintIR::variables(&self) -> &[VariableSlot]` 获取变量槽 | P0 |
| REQ-CP-012 | `BlueprintIR::functions(&self) -> &[FunctionEntry]` 获取函数表 | P0 |
| REQ-CP-013 | `BlueprintIR::entry_point(&self) -> InstrOffset` 获取入口偏移 | P0 |
| REQ-CP-014 | `BlueprintIR::serialize(&self) -> Vec<u8>` bytecode 序列化（bincode） | P1 |
| REQ-CP-015 | `BlueprintIR::deserialize(bytes) -> Result<Self, DeserializeError>` 反序列化 | P1 |

### 2.3 BlueprintIRInstruction（需求 44-46, 226-243）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-016 | `Nop` 空操作 | P0 |
| REQ-CP-017 | `PushConst(ConstId)` 推送常量 | P0 |
| REQ-CP-018 | `LoadVar(VarSlot)` 加载变量 | P0 |
| REQ-CP-019 | `StoreVar(VarSlot)` 存储变量 | P0 |
| REQ-CP-020 | `Dup / Swap / Pop` 栈操作 | P1 |
| REQ-CP-021 | `Add / Sub / Mul / Div / Rem` 算术运算 | P0 |
| REQ-CP-022 | `Neg / Abs / Sqrt / Sin / Cos / Tan / Log / Exp` 数学函数 | P1 |
| REQ-CP-023 | `Lt / Le / Gt / Ge / Eq / Ne` 比较运算 | P0 |
| REQ-CP-024 | `And / Or / Not` 逻辑运算 | P0 |
| REQ-CP-025 | `Jump(offset)` 无条件跳转 | P0 |
| REQ-CP-026 | `JumpIf(offset)` 条件为真跳转 | P0 |
| REQ-CP-027 | `JumpIfNot(offset)` 条件为假跳转 | P0 |
| REQ-CP-028 | `Call(fn_id, argc)` 函数调用 | P0 |
| REQ-CP-029 | `Return` 函数返回 | P0 |
| REQ-CP-030 | `Yield(duration)` 挂起（支持 Delay/RetriggerableDelay） | P1 |
| REQ-CP-031 | `ActivateExec(node, pin) / DeactivateExec(node, pin)` 执行流控制 | P0 |
| REQ-CP-032 | `PinWrite / PinRead` 引脚读写 | P1 |

### 2.4 BlueprintInterpreter（需求 49-52, 100-102, 244-252）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-033 | `BlueprintInterpreter::new(ir) -> Self` 创建解释器 | P0 |
| REQ-CP-034 | `BlueprintInterpreter::run(&mut self, context) -> Result<(), RuntimeError>` 执行 | P0 |
| REQ-CP-035 | `BlueprintInterpreter::step(&mut self, context) -> Result<(), RuntimeError>` 单步执行 | P0 |
| REQ-CP-036 | `BlueprintInterpreter::reset(&mut self)` 重置解释器 | P1 |
| REQ-CP-037 | `BlueprintInterpreter::stack(&self) -> &[PinValue]` 获取栈快照 | P1 |
| REQ-CP-038 | `BlueprintInterpreter::variables(&self) -> &[PinValue]` 获取变量快照 | P1 |
| REQ-CP-039 | `BlueprintInterpreter::pc(&self) -> InstrOffset` 获取程序计数器 | P1 |
| REQ-CP-040 | `RuntimeError::StackOverflow / StackUnderflow / DivisionByZero / InvalidCast` | P0 |
| REQ-CP-041 | `RuntimeError::TypeMismatch / BadInstruction / UnknownFunction` | P0 |

### 2.5 BlueprintContext（需求 253-255）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-042 | `BlueprintContext` 执行上下文（world、entity、delta_time、事件总线引用） | P0 |
| REQ-CP-043 | `BlueprintContext::world(&self) -> &World` 获取 World 引用 | P0 |
| REQ-CP-044 | `BlueprintContext::entity(&self) -> Option<Entity>` 获取当前实体 | P0 |
| REQ-CP-045 | `BlueprintContext::delta_time(&self) -> f32` 获取帧时间 | P0 |

### 2.6 BlueprintMacro 与 FunctionLibrary（需求 54-58, 104-108）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-046 | `BlueprintMacro::expand(&self, context) -> Vec<BlueprintNode>` 宏展开 | P1 |
| REQ-CP-047 | `BlueprintFunctionLibrary` 可复用函数蓝图的集合 | P1 |
| REQ-CP-048 | `BlueprintFunctionLibrary::add_function(&mut self, name, graph)` 添加函数 | P1 |
| REQ-CP-049 | `BlueprintFunctionLibrary::get_function(&self, name) -> Option<&BlueprintGraph>` 获取函数 | P1 |

### 2.7 BlueprintDebugger（需求 59-62, 109-112）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-050 | `BlueprintDebugger::set_breakpoint(&mut self, node_id, pin_id)` 设置断点 | P1 |
| REQ-CP-051 | `BlueprintDebugger::remove_breakpoint(&mut self, id)` 移除断点 | P1 |
| REQ-CP-052 | `BlueprintDebugger::call_stack(&self) -> Vec<CallFrame>` 获取调用栈 | P1 |
| REQ-CP-053 | `BlueprintDebugger::watch(&self, variable_name) -> Option<&PinValue>` 变量监视 | P1 |

### 2.8 蓝图宏与热重载（需求 54, 73-74, 123-124）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-CP-054 | `#[blueprint_function]` 过程宏（第一阶段用 `inventory` 手动注册） | P1 |
| REQ-CP-055 | `BlueprintHotReload` 检测 `.bp.json` 文件变更重新编译 IR | P2 |
| REQ-CP-056 | `BlueprintBindings` 将 Rust 函数/struct 反射为蓝图可调用节点 | P1 |

---

## 3. API 签名

### 3.1 BlueprintCompiler

```rust
pub struct BlueprintCompiler {
    constants: Vec<PinValue>,
    variables: Vec<VariableSlot>,
    functions: HashMap<String, FunctionEntry>,
}

impl BlueprintCompiler {
    pub fn new() -> Self;
    pub fn compile(&self, graph: &BlueprintGraph) -> Result<BlueprintIR, CompileError>;
    pub fn emit_const(&mut self, value: PinValue) -> ConstId;
    pub fn emit_var(&mut self, name: &str, ty: PinType) -> VarSlot;
    pub fn emit_instruction(&mut self, instr: BlueprintIRInstruction) -> InstrOffset;
}

pub enum CompileError {
    CycleDetected(NodeId),
    TypeMismatch { expected: PinType, found: PinType },
    UndefinedVariable(String),
    InvalidWire { wire_id: WireId, reason: String },
}
```

### 3.2 BlueprintIR

```rust
pub struct BlueprintIR {
    instructions: Vec<BlueprintIRInstruction>,
    constants: Vec<PinValue>,
    variables: Vec<VariableSlot>,
    functions: Vec<FunctionEntry>,
    entry_point: InstrOffset,
}

impl BlueprintIR {
    pub fn new() -> Self;
    pub fn instructions(&self) -> &[BlueprintIRInstruction];
    pub fn constants(&self) -> &[PinValue];
    pub fn variables(&self) -> &[VariableSlot];
    pub fn functions(&self) -> &[FunctionEntry];
    pub fn entry_point(&self) -> InstrOffset;
    pub fn serialize(&self) -> Vec<u8>;
    pub fn deserialize(bytes: &[u8]) -> Result<Self, DeserializeError>;
}

pub enum BlueprintIRInstruction {
    Nop,
    PushConst(ConstId),
    LoadVar(VarSlot),
    StoreVar(VarSlot),
    Dup,
    Swap,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Log,
    Exp,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,
    Not,
    Jump(InstrOffset),
    JumpIf(InstrOffset),
    JumpIfNot(InstrOffset),
    Call(u32, u32),
    Return,
    Yield(f32),
    ActivateExec(NodeId, PinId),
    DeactivateExec(NodeId, PinId),
    PinWrite,
    PinRead,
}
```

### 3.3 BlueprintInterpreter

```rust
pub struct BlueprintInterpreter {
    ir: Arc<BlueprintIR>,
    stack: Vec<PinValue>,
    variables: Vec<PinValue>,
    pc: InstrOffset,
    state: InterpreterState,
}

impl BlueprintInterpreter {
    pub fn new(ir: Arc<BlueprintIR>) -> Self;
    pub fn run(&mut self, context: &mut BlueprintContext) -> Result<(), RuntimeError>;
    pub fn step(&mut self, context: &mut BlueprintContext) -> Result<(), RuntimeError>;
    pub fn reset(&mut self);
    pub fn stack(&self) -> &[PinValue];
    pub fn variables(&self) -> &[PinValue];
    pub fn pc(&self) -> InstrOffset;
}

pub enum RuntimeError {
    StackOverflow,
    StackUnderflow,
    DivisionByZero,
    InvalidCast,
    TypeMismatch,
    BadInstruction,
    UnknownFunction,
}
```

### 3.4 BlueprintContext

```rust
pub struct BlueprintContext<'w> {
    world: &'w World,
    entity: Option<Entity>,
    delta_time: f32,
    event_bus: &'w EventBus,
}

impl<'w> BlueprintContext<'w> {
    pub fn world(&self) -> &World;
    pub fn entity(&self) -> Option<Entity>;
    pub fn delta_time(&self) -> f32;
}
```

### 3.5 BlueprintDebugger

```rust
pub struct BlueprintDebugger {
    breakpoints: HashSet<(NodeId, PinId)>,
    call_stack: Vec<CallFrame>,
    watches: HashMap<String, PinValue>,
}

impl BlueprintDebugger {
    pub fn set_breakpoint(&mut self, node_id: NodeId, pin_id: PinId);
    pub fn remove_breakpoint(&mut self, id: (NodeId, PinId));
    pub fn call_stack(&self) -> Vec<CallFrame>;
    pub fn watch(&self, variable_name: &str) -> Option<&PinValue>;
}

pub struct CallFrame {
    node_id: NodeId,
    pc: InstrOffset,
}
```

---

## 4. 输入与输出

| 组件 | 输入 | 输出 |
|------|------|------|
| `BlueprintCompiler::compile` | `&BlueprintGraph` | `Result<BlueprintIR, CompileError>` |
| `BlueprintCompiler::emit_const` | `PinValue` | `ConstId` |
| `BlueprintCompiler::emit_instruction` | `BlueprintIRInstruction` | `InstrOffset` |
| `BlueprintIR::serialize` | - | `Vec<u8>` |
| `BlueprintIR::deserialize` | `&[u8]` | `Result<Self, DeserializeError>` |
| `BlueprintInterpreter::run` | `&mut BlueprintContext` | `Result<(), RuntimeError>` |
| `BlueprintInterpreter::step` | `&mut BlueprintContext` | `Result<(), RuntimeError>` |

---

## 5. 验收标准

- [ ] `BlueprintCompiler::compile` 对 hello graph 生成 ≥ 5 条指令
- [ ] 编译器对 If/For/While/Switch 节点生成等价的 IR 指令
- [ ] `BlueprintInterpreter` 对 `If(cond, a=1, a=2)` 后 `a==cond?1:2` 正确执行
- [ ] `BlueprintInterpreter` 对 `For(i, 0, 10) { sum += i }`，`sum == 45`
- [ ] `BlueprintInterpreter` 对 `While(i<3) { i+=1 }` 正确终止
- [ ] `BlueprintInterpreter` 对 `Switch(value)` 的分支选择正确
- [ ] IR 序列化/反序列化保持语义一致
- [ ] 运行时错误（StackOverflow、DivisionByZero 等）正确抛出

---

## 6. 依赖关系

- 依赖 `01-blueprint-core` 模块的 `BlueprintGraph`、`BlueprintNode`、`Pin`、`BlueprintWire`
- 被 `BlueprintEditorView` 调用（编译按钮）
- 为 `BlueprintInterpreter` 提供编译产物
- 与 `engine-ecs` 的 `World`、`Entity`、`EventBus` 交互

---

## 7. 优先级汇总

| 优先级 | 需求数量 | 说明 |
|-------|---------|------|
| P0 | 31 | 核心编译与执行，必须完成 |
| P1 | 20 | 重要功能，应完成 |
| P2 | 1 | 增强功能（热重载） |
