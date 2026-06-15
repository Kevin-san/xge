# 模块一：蓝图核心（Blueprint Core）

## 1. 模块概述

Blueprint Core 是 `engine-blueprint` crate 的核心数据结构层，提供蓝图可视化编程的基础数据模型，包括：
- `BlueprintGraph`：节点图顶层数据结构，管理节点集合与连线集合
- `BlueprintNode`：单个节点的抽象，包含节点类型、输入/输出引脚
- `Pin` 与 `PinValue`：引脚及其值类型系统
- `BlueprintWire`：连线，连接源引脚与目标引脚

本模块是蓝图编译器、解释器、编辑器 UI 的基础。

---

## 2. 需求清单

### 2.1 BlueprintGraph（需求 3-12, 136-218）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-BC-001 | `BlueprintGraph::new() -> Self` 创建空图 | P0 |
| REQ-BC-002 | `BlueprintGraph::with_capacity(nodes, wires) -> Self` 带容量预分配 | P1 |
| REQ-BC-003 | `BlueprintGraph::add_node(&mut self, node) -> NodeId` 添加节点，返回自增 ID | P0 |
| REQ-BC-004 | `BlueprintGraph::remove_node(&mut self, node_id)` 移除节点及所有相关连线 | P0 |
| REQ-BC-005 | `BlueprintGraph::node(&self, id) -> &BlueprintNode` 按 ID 获取节点引用 | P0 |
| REQ-BC-006 | `BlueprintGraph::nodes(&self) -> &[BlueprintNode]` 获取所有节点 | P0 |
| REQ-BC-007 | `BlueprintGraph::nodes_mut(&mut self) -> &mut [BlueprintNode]` 可变获取所有节点 | P1 |
| REQ-BC-008 | `BlueprintGraph::add_wire(&mut self, from_pin, to_pin) -> WireId` 添加连线 | P0 |
| REQ-BC-009 | `BlueprintGraph::remove_wire(&mut self, wire_id)` 移除连线 | P0 |
| REQ-BC-010 | `BlueprintGraph::wires(&self) -> &[BlueprintWire]` 获取所有连线 | P0 |
| REQ-BC-011 | `BlueprintGraph::wires_mut(&mut self) -> &mut [BlueprintWire]` 可变获取所有连线 | P1 |
| REQ-BC-012 | `BlueprintGraph::validate(&self) -> Result<(), BlueprintError>` 检测循环、类型不匹配等 | P0 |
| REQ-BC-013 | `BlueprintGraph::topological_sort(&self) -> Result<Vec<NodeId>, CycleError>` 拓扑排序 | P1 |
| REQ-BC-014 | `BlueprintGraph::clone() -> Self` 深拷贝 | P2 |
| REQ-BC-015 | `BlueprintGraph::to_json(&self) -> String` 序列化为 JSON | P1 |
| REQ-BC-016 | `BlueprintGraph::from_json(json) -> Result<Self, DeserializeError>` 从 JSON 反序列化 | P1 |
| REQ-BC-017 | `BlueprintGraph::contains_node(&self, id) -> bool` 检查节点是否存在 | P1 |
| REQ-BC-018 | `BlueprintGraph::node_mut(&mut self, id) -> &mut BlueprintNode` 可变获取节点 | P1 |
| REQ-BC-019 | `BlueprintGraph::wires_into_pin(&self, pin) -> Vec<WireId>` 获取连入某引脚的连线 | P2 |
| REQ-BC-020 | `BlueprintGraph::wires_out_of_pin(&self, pin) -> Vec<WireId>` 获取连出某引脚的连线 | P2 |

**验证规则（REQ-BC-012）**：
- 检查所有引脚类型匹配
- 检查 exec 引脚与数据引脚不可互连
- 检查无重复连线
- 检查变量 get/set 名称在 graph 作用域内定义

---

### 2.2 BlueprintNode（需求 13-18, 156-175）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-BC-021 | `BlueprintNode::new(kind, inputs, outputs, meta) -> Self` 创建节点 | P0 |
| REQ-BC-022 | `BlueprintNode::id(&self) -> NodeId` 获取节点 ID | P0 |
| REQ-BC-023 | `BlueprintNode::kind(&self) -> NodeKind` 获取节点类型 | P0 |
| REQ-BC-024 | `BlueprintNode::name(&self) -> &str` 获取节点名称 | P1 |
| REQ-BC-025 | `BlueprintNode::position(&self) -> (f32, f32)` 获取编辑器坐标 | P1 |
| REQ-BC-026 | `BlueprintNode::set_position(&mut self, x, y)` 设置编辑器坐标 | P1 |
| REQ-BC-027 | `BlueprintNode::input_pins(&self) -> &[Pin]` 获取输入引脚列表 | P0 |
| REQ-BC-028 | `BlueprintNode::output_pins(&self) -> &[Pin]` 获取输出引脚列表 | P0 |
| REQ-BC-029 | `BlueprintNode::pin(&self, id) -> Option<&Pin>` 按 ID 获取引脚 | P1 |
| REQ-BC-030 | `BlueprintNode::pin_by_name(&self, name) -> Option<&Pin>` 按名称获取引脚 | P1 |

---

### 2.3 Pin 与类型系统（需求 19-26, 166-248）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-BC-031 | `Pin::new(name, dir, ty, default) -> Self` 创建引脚 | P0 |
| REQ-BC-032 | `Pin::id(&self) -> PinId` 获取引脚 ID | P0 |
| REQ-BC-033 | `Pin::name(&self) -> &str` 获取引脚名称 | P0 |
| REQ-BC-034 | `Pin::direction(&self) -> PinDirection` 获取引脚方向 | P0 |
| REQ-BC-035 | `Pin::data_type(&self) -> PinType` 获取数据类型 | P0 |
| REQ-BC-036 | `Pin::default_value(&self) -> Option<&PinValue>` 获取默认值 | P1 |
| REQ-BC-037 | `Pin::can_connect(&self, other) -> bool` 检查是否可连接到另一引脚 | P0 |
| REQ-BC-038 | `PinDirection::Input / Output` 枚举定义 | P0 |
| REQ-BC-039 | `PinType::Bool / Int / Float / Vec2 / Vec3 / Vec4 / String / Entity / Any / Exec` | P0 |
| REQ-BC-040 | `PinValue::Bool / Int / Float / Vec2 / Vec3 / Vec4 / String / Entity / Any / Exec / Invalid` | P0 |
| REQ-BC-041 | `PinType::is_numeric(&self) -> bool` 判断是否为数值类型 | P1 |
| REQ-BC-042 | `PinType::is_vector(&self) -> bool` 判断是否为向量类型 | P1 |
| REQ-BC-043 | `PinType::is_compatible(&self, other) -> bool` 检查类型兼容性（Any 兼容一切） | P1 |
| REQ-BC-044 | `PinValue::type_of(&self) -> PinType` 获取值的类型 | P1 |
| REQ-BC-045 | `PinValue::coerce_to(&self, target) -> Option<PinValue>` 类型强制转换（数字/字符串互转） | P2 |

---

### 2.4 BlueprintWire（需求 24-26, 244-248）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-BC-046 | `BlueprintWire::new(from, to) -> Self` 创建连线 | P0 |
| REQ-BC-047 | `BlueprintWire::from(&self) -> PinRef` 获取源引脚引用 | P0 |
| REQ-BC-048 | `BlueprintWire::to(&self) -> PinRef` 获取目标引脚引用 | P0 |
| REQ-BC-049 | `BlueprintWire::color(&self) -> Color` 根据 PinType 返回连线颜色 | P2 |

---

### 2.5 强类型 ID（需求 27）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-BC-050 | `NodeId / PinId / WireId` 类型别名，采用强类型 `newtype` 模式 | P0 |

---

### 2.6 节点类型枚举（需求 28-40）

| 需求编号 | 描述 | 优先级 |
|---------|------|--------|
| REQ-BC-051 | `NodeKind::Function / Macro / Event / VariableGet / VariableSet / If / For / While / Switch / Branch / Sequence` | P0 |
| REQ-BC-052 | `NodeKind::Timeline / Gate / DoN / DoOnce / RetriggerableDelay / Delay / PrintString` | P0 |
| REQ-BC-053 | 数学节点：`Add / Subtract / Multiply / Divide / Dot / Cross / Normalize / Length` | P1 |
| REQ-BC-054 | 数学节点：`Lerp / Clamp / Min / Max / Abs / Sin / Cos / Tan / Log / Sqrt` | P1 |
| REQ-BC-055 | 向量节点：`BreakVec2 / BreakVec3 / BreakVec4 / MakeVec2 / MakeVec3 / MakeVec4` | P1 |
| REQ-BC-056 | 计时节点：`SetTimer / ClearTimer / IsTimerActive` | P1 |
| REQ-BC-057 | Actor 节点：`SpawnActor / DestroyActor / GetActorLocation / SetActorLocation` | P1 |
| REQ-BC-058 | Actor 节点：`GetActorRotation / SetActorRotation / GetActorScale / SetActorScale` | P1 |
| REQ-BC-059 | Actor 节点：`AddActorWorldOffset / AddActorLocalOffset` | P1 |
| REQ-BC-060 | 碰撞与检测：`LineTraceByChannel / MultiSphereTrace / OverlapAll` | P1 |
| REQ-BC-061 | 事件节点：`BeginPlay / Tick / OnComponentHit / OnComponentBeginOverlap / OnClicked` | P0 |
| REQ-BC-062 | 事件节点：`CustomEvent / EventDispatcher / AddCustomEventListener / CallEventDispatcher` | P1 |
| REQ-BC-063 | 类型与工具节点：`CastTo / IsValid / Select / StructSetMember / StructGetMember` | P1 |

---

## 3. API 签名

### 3.1 BlueprintGraph

```rust
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
    id: NodeId,
    kind: NodeKind,
    name: String,
    position: (f32, f32),
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
    metadata: HashMap<String, String>,
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
pub enum PinDirection { Input, Output }

pub enum PinType {
    Bool, Int, Float, Vec2, Vec3, Vec4, String, Entity, Any, Exec
}

pub enum PinValue {
    Bool(bool), Int(i64), Float(f64), Vec2([f32; 2]), Vec3([f32; 3]), Vec4([f32; 4]),
    String(String), Entity(EntityRef), Any(Box<dyn Any>), Exec, Invalid
}

pub struct Pin {
    id: PinId,
    name: String,
    direction: PinDirection,
    data_type: PinType,
    default_value: Option<PinValue>,
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
    id: WireId,
    from: PinRef,
    to: PinRef,
}

impl BlueprintWire {
    pub fn new(from: PinRef, to: PinRef) -> Self;
    pub fn from(&self) -> PinRef;
    pub fn to(&self) -> PinRef;
    pub fn color(&self) -> Color;
}
```

---

## 4. 输入与输出

| 数据结构 | 输入 | 输出 |
|---------|------|------|
| `BlueprintGraph::add_node` | `BlueprintNode` | `NodeId` |
| `BlueprintGraph::remove_node` | `NodeId` | `()` |
| `BlueprintGraph::add_wire` | `PinRef, PinRef` | `WireId` |
| `BlueprintGraph::validate` | - | `Result<(), BlueprintError>` |
| `BlueprintGraph::topological_sort` | - | `Result<Vec<NodeId>, CycleError>` |
| `BlueprintNode::new` | `NodeKind, Vec<Pin>, Vec<Pin>, HashMap` | `Self` |
| `Pin::can_connect` | `&Pin` | `bool` |
| `PinValue::coerce_to` | `PinType` | `Option<PinValue>` |

---

## 5. 验收标准

- [ ] `BlueprintGraph::add_node` + `remove_node` 保持 ID 唯一性
- [ ] `BlueprintGraph::validate` 对类型不匹配返回 `Err`
- [ ] `BlueprintGraph::topological_sort` 对含环图返回 `CycleError`
- [ ] `Pin::can_connect` 正确检查方向与类型兼容性
- [ ] `PinValue::coerce_to` 支持字符串↔数字互转
- [ ] 所有 ID 类型（NodeId/PinId/WireId）为强类型 newtype
- [ ] 节点类型枚举覆盖所有指定类型

---

## 6. 依赖关系

- 依赖 `engine-ecs` crate 中的 `EntityRef` 类型
- 被 `BlueprintCompiler`、`BlueprintInterpreter`、`BlueprintEditorView` 模块使用
- 被 `examples/blueprint_*` 示例使用

---

## 7. 优先级汇总

| 优先级 | 需求数量 | 说明 |
|-------|---------|------|
| P0 | 32 | 核心数据结构，必须完成 |
| P1 | 26 | 重要功能，应完成 |
| P2 | 5 | 增强功能，可延后 |
