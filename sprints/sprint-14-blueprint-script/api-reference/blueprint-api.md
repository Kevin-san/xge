# 蓝图 API 清单（Blueprint API Reference）

## 1. 概述

本文档列出 `engine-blueprint` crate 的公开 API，包括所有 public struct、enum、trait、function。

---

## 2. Crate 入口

```rust
// engine-blueprint crate
pub mod blueprint;
pub mod compiler;
pub mod interpreter;
pub mod editor;
pub mod debugger;
```

---

## 3. 核心数据类型（01-blueprint-core）

### 3.1 BlueprintGraph

```rust
// 位置: src/blueprint/graph.rs

pub struct BlueprintGraph { /* ... */ }

impl BlueprintGraph {
    pub fn new() -> Self;
    pub fn with_capacity(nodes: usize, wires: usize) -> Self;
    pub fn add_node(&mut self, node: BlueprintNode) -> NodeId;
    pub fn remove_node(&mut self, node_id: NodeId);
    pub fn contains_node(&self, id: NodeId) -> bool;
    pub fn node(&self, id: NodeId) -> &BlueprintNode;
    pub fn node_mut(&mut self, id: NodeId) -> &mut BlueprintNode;
    pub fn nodes(&self) -> &[BlueprintNode];
    pub fn nodes_mut(&mut self) -> &mut [BlueprintNode];
    pub fn add_wire(&mut self, from: PinRef, to: PinRef) -> WireId;
    pub fn remove_wire(&mut self, wire_id: WireId);
    pub fn wires(&self) -> &[BlueprintWire];
    pub fn wires_mut(&mut self) -> &mut [BlueprintWire];
    pub fn wires_into_pin(&self, pin: PinRef) -> Vec<WireId>;
    pub fn wires_out_of_pin(&self, pin: PinRef) -> Vec<WireId>;
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, CycleError>;
    pub fn validate(&self) -> Result<(), BlueprintError>;
    pub fn clone(&self) -> Self;
    pub fn to_json(&self) -> String;
    pub fn from_json(json: &str) -> Result<Self, DeserializeError>;
}
```

### 3.2 BlueprintNode

```rust
pub struct BlueprintNode {
    pub id: NodeId,
    pub kind: NodeKind,
    pub name: String,
    pub position: (f32, f32),
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
    pub metadata: HashMap<String, String>,
}

impl BlueprintNode {
    pub fn new(kind: NodeKind, inputs: Vec<Pin>, outputs: Vec<Pin>, meta: HashMap<String, String>) -> Self;
    pub fn id(&self) -> NodeId;
    pub fn kind(&self) -> NodeKind;
    pub fn name(&self) -> &str;
    pub fn position(&self) -> (f32, f32);
    pub fn set_position(&mut self, x: f32, y: f32);
    pub fn input_pins(&self) -> &[Pin];
    pub fn output_pins(&self) -> &[Pin];
    pub fn pin(&self, id: PinId) -> Option<&Pin>;
    pub fn pin_by_name(&self, name: &str) -> Option<&Pin>;
}
```

### 3.3 Pin 与类型

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct WireId(pub u64);

pub struct PinRef { /* ... */ }

pub enum PinDirection { Input, Output }

pub enum PinType {
    Bool,
    Int,
    Float,
    Vec2,
    Vec3,
    Vec4,
    String,
    Entity,
    Any,
    Exec,
}

pub enum PinValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    String(String),
    Entity(EntityRef),
    Any(Box<dyn Any>),
    Exec,
    Invalid,
}

pub struct Pin {
    pub id: PinId,
    pub name: String,
    pub direction: PinDirection,
    pub data_type: PinType,
    pub default_value: Option<PinValue>,
}

impl Pin {
    pub fn new(name: &str, dir: PinDirection, ty: PinType, default: Option<PinValue>) -> Self;
    pub fn id(&self) -> PinId;
    pub fn name(&self) -> &str;
    pub fn direction(&self) -> PinDirection;
    pub fn data_type(&self) -> PinType;
    pub fn default_value(&self) -> Option<&PinValue>;
    pub fn can_connect(&self, other: &Pin) -> bool;
}

impl PinType {
    pub fn is_numeric(&self) -> bool;
    pub fn is_vector(&self) -> bool;
    pub fn is_compatible(&self, other: &PinType) -> bool;
}

impl PinValue {
    pub fn type_of(&self) -> PinType;
    pub fn coerce_to(&self, target: PinType) -> Option<PinValue>;
}
```

### 3.4 BlueprintWire

```rust
pub struct BlueprintWire {
    pub id: WireId,
    pub from: PinRef,
    pub to: PinRef,
}

impl BlueprintWire {
    pub fn new(from: PinRef, to: PinRef) -> Self;
    pub fn from(&self) -> PinRef;
    pub fn to(&self) -> PinRef;
    pub fn color(&self) -> Color;
}
```

### 3.5 NodeKind

```rust
pub enum NodeKind {
    // 控制流
    Function,
    Macro,
    Event,
    VariableGet,
    VariableSet,
    If,
    For,
    While,
    Switch,
    Branch,
    Sequence,
    Timeline,
    Gate,
    DoN,
    DoOnce,
    RetriggerableDelay,
    Delay,
    PrintString,
    // 数学
    Add,
    Subtract,
    Multiply,
    Divide,
    Dot,
    Cross,
    Normalize,
    Length,
    Lerp,
    Clamp,
    Min,
    Max,
    Abs,
    Sin,
    Cos,
    Tan,
    Log,
    Sqrt,
    // 向量
    BreakVec2,
    BreakVec3,
    BreakVec4,
    MakeVec2,
    MakeVec3,
    MakeVec4,
    // 计时
    SetTimer,
    ClearTimer,
    IsTimerActive,
    // Actor
    SpawnActor,
    DestroyActor,
    GetActorLocation,
    SetActorLocation,
    GetActorRotation,
    SetActorRotation,
    GetActorScale,
    SetActorScale,
    AddActorWorldOffset,
    AddActorLocalOffset,
    // 碰撞
    LineTraceByChannel,
    MultiSphereTrace,
    OverlapAll,
    // 事件
    BeginPlay,
    Tick,
    OnComponentHit,
    OnComponentBeginOverlap,
    OnClicked,
    CustomEvent,
    EventDispatcher,
    AddCustomEventEventListener,
    CallEventDispatcher,
    // 类型与工具
    CastTo,
    IsValid,
    Select,
    StructSetMember,
    StructGetMember,
    // 动画
    AnimationParameter,
}
```

### 3.6 错误类型

```rust
pub enum BlueprintError {
    CycleDetected(NodeId),
    TypeMismatch { node: NodeId, pin: PinId, expected: PinType, found: PinType },
    UndefinedVariable(String),
    InvalidWire { wire_id: WireId, reason: String },
}

pub enum CycleError {
    Detected(Vec<NodeId>),
}
```

---

## 4. 编辑器 UI（02-node-graph）

```rust
pub struct BlueprintEditorView { /* ... */ }

impl BlueprintEditorView {
    pub fn new(graph_arc: Arc<RwLock<BlueprintGraph>>) -> Self;
    pub fn show(&mut self, ui: &mut Ui);
}

pub struct NodeDragDropController { /* ... */ }

impl NodeDragDropController {
    pub fn begin_drag(&mut self, node_id: NodeId, mouse: (f32, f32));
    pub fn drag_to(&mut self, mouse: (f32, f32));
    pub fn end_drag(&mut self);
    pub fn snap_to_grid(&self, x: f32, y: f32) -> (f32, f32);
}

pub struct WireEditor { /* ... */ }

impl WireEditor {
    pub fn begin_wire(&mut self, from_pin: PinRef, mouse: (f32, f32));
    pub fn preview(&self, mouse: (f32, f32));
    pub fn end_wire(&mut self, target_pin: PinRef) -> Option<WireId>;
    pub fn cancel(&mut self);
}

pub struct ZoomPanController { /* ... */ }

impl ZoomPanController {
    pub fn zoom(&self) -> f32;
    pub fn set_zoom(&mut self, z: f32, anchor: (f32, f32));
    pub fn pan(&mut self, delta: (f32, f32));
    pub fn screen_to_world(&self, p: (f32, f32)) -> (f32, f32);
    pub fn world_to_screen(&self, p: (f32, f32)) -> (f32, f32);
}

pub struct NodeSearch { /* ... */ }

impl NodeSearch {
    pub fn open(&mut self);
    pub fn filter(&mut self, query: &str) -> Vec<NodeKind>;
    pub fn insert_at_cursor(&mut self, kind: NodeKind, cursor: (f32, f32));
}

pub enum LayoutAlgorithm { Hierarchical, ForceDirected }

pub struct AutoLayout;

impl AutoLayout {
    pub fn apply(graph: &mut BlueprintGraph, algo: LayoutAlgorithm);
}

pub struct CommentBox { /* ... */ }

impl CommentBox {
    pub fn new(rect: Rect, text: &str, color: Color) -> Self;
    pub fn contains(&self, node: &BlueprintNode) -> bool;
    pub fn render(&self, ui: &mut Ui);
}

pub struct RerouteNode { /* ... */ }

impl RerouteNode {
    pub fn split_wire(graph: &mut BlueprintGraph, wire_id: WireId, point: (f32, f32)) -> (WireId, WireId);
}

pub struct UndoStack { /* ... */ }
```

---

## 5. 编译器与解释器（03-compiler）

```rust
pub struct BlueprintCompiler { /* ... */ }

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

#[derive(Clone, Copy)]
pub struct ConstId(pub u32);

#[derive(Clone, Copy)]
pub struct VarSlot(pub u32);

#[derive(Clone, Copy)]
pub struct InstrOffset(pub usize);

pub struct BlueprintIR { /* ... */ }

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

pub struct BlueprintInterpreter { /* ... */ }

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

pub struct BlueprintDebugger { /* ... */ }

impl BlueprintDebugger {
    pub fn set_breakpoint(&mut self, node_id: NodeId, pin_id: PinId);
    pub fn remove_breakpoint(&mut self, id: (NodeId, PinId));
    pub fn call_stack(&self) -> Vec<CallFrame>;
    pub fn watch(&self, variable_name: &str) -> Option<&PinValue>;
}

pub struct CallFrame {
    pub node_id: NodeId,
    pub pc: InstrOffset,
}
```

---

## 6. 过程宏

```rust
// engine-blueprint-macros crate

#[proc_macro_attribute]
pub fn blueprint_function(attr: TokenStream, item: TokenStream) -> TokenStream;
```

---

## 7. 变更日志

| 版本 | 日期 | 变更 |
|------|------|------|
| v0.14.0 | TBD | 初始版本，包含 BlueprintGraph、Compiler、Interpreter、Editor UI |
