//! Blueprint visual scripting system with node graph and script VM.
//!
//! This crate provides:
//! - `BlueprintGraph`: Top-level data structure for node graphs
//! - `BlueprintNode`: Individual nodes with input/output pins
//! - `BlueprintPin`: Connection points on nodes
//! - `BlueprintWire`: Connections between pins
//! - `BlueprintCompiler`: Compiles graphs to IR
//! - `BlueprintIR`: Intermediate representation bytecode
//! - `BlueprintInterpreter`: Stack-based interpreter for IR execution
//! - `ScriptVM`: Script virtual machine abstraction

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

// ============================================================================
// Constants
// ============================================================================

/// Maximum stack depth for interpreter
const MAX_STACK_DEPTH: usize = 1024;

/// Maximum instruction count per run to prevent infinite loops
const MAX_INSTRUCTIONS_PER_RUN: u64 = 10_000_000;

// ============================================================================
// Strong Type IDs
// ============================================================================

/// Unique identifier for a node in the graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u32);

/// Unique identifier for a pin on a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PinId(pub u32);

/// Unique identifier for a wire connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WireId(pub u32);

/// Reference to a specific pin: (NodeId, PinId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PinRef(pub NodeId, pub PinId);

/// Constant pool index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstId(pub u32);

/// Variable slot index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VarSlot(pub u32);

/// Instruction offset in IR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstrOffset(pub u32);

/// Script handle for VM instances
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScriptHandle(pub u64);

// ============================================================================
// Pin Types and Values
// ============================================================================

/// Direction of a pin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinDirection {
    Input,
    Output,
}

/// Data type of a pin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl PinType {
    /// Check if this type is numeric (Int or Float)
    pub fn is_numeric(&self) -> bool {
        matches!(self, PinType::Int | PinType::Float)
    }

    /// Check if this type is a vector type
    pub fn is_vector(&self) -> bool {
        matches!(self, PinType::Vec2 | PinType::Vec3 | PinType::Vec4)
    }

    /// Check type compatibility (Any is compatible with everything)
    pub fn is_compatible(&self, other: &PinType) -> bool {
        *self == PinType::Any || *other == PinType::Any || *self == *other
    }
}

/// Value held by a pin
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PinValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    String(String),
    Entity(u64),
    Any(Box<serde_json::Value>),
    Exec,
    #[default]
    Invalid,
}

impl PinValue {
    /// Get the type of this value
    pub fn type_of(&self) -> PinType {
        match self {
            PinValue::Bool(_) => PinType::Bool,
            PinValue::Int(_) => PinType::Int,
            PinValue::Float(_) => PinType::Float,
            PinValue::Vec2(_) => PinType::Vec2,
            PinValue::Vec3(_) => PinType::Vec3,
            PinValue::Vec4(_) => PinType::Vec4,
            PinValue::String(_) => PinType::String,
            PinValue::Entity(_) => PinType::Entity,
            PinValue::Any(_) => PinType::Any,
            PinValue::Exec => PinType::Exec,
            PinValue::Invalid => PinType::Any,
        }
    }

    /// Coerce value to target type (supports number/string conversion)
    pub fn coerce_to(&self, target: PinType) -> Option<PinValue> {
        match (self, target) {
            (PinValue::Int(n), PinType::Float) => Some(PinValue::Float(*n as f64)),
            (PinValue::Float(f), PinType::Int) => Some(PinValue::Int(*f as i64)),
            (PinValue::Int(n), PinType::String) => Some(PinValue::String(n.to_string())),
            (PinValue::Float(f), PinType::String) => Some(PinValue::String(f.to_string())),
            (PinValue::String(s), PinType::Int) => s.parse::<i64>().ok().map(PinValue::Int),
            (PinValue::String(s), PinType::Float) => s.parse::<f64>().ok().map(PinValue::Float),
            (PinValue::Bool(b), PinType::Int) => Some(PinValue::Int(if *b { 1 } else { 0 })),
            _ => None,
        }
    }

    /// Get as bool
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            PinValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as int
    pub fn to_int(&self) -> Option<i64> {
        match self {
            PinValue::Int(n) => Some(*n),
            PinValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    /// Get as float
    pub fn to_float(&self) -> Option<f64> {
        match self {
            PinValue::Float(f) => Some(*f),
            PinValue::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    /// Convert to string lossy
    pub fn to_string_lossy(&self) -> String {
        match self {
            PinValue::Bool(b) => b.to_string(),
            PinValue::Int(n) => n.to_string(),
            PinValue::Float(f) => f.to_string(),
            PinValue::String(s) => s.clone(),
            PinValue::Vec2(v) => format!("{}, {}", v[0], v[1]),
            PinValue::Vec3(v) => format!("{}, {}, {}", v[0], v[1], v[2]),
            PinValue::Vec4(v) => format!("{}, {}, {}, {}", v[0], v[1], v[2], v[3]),
            _ => "<invalid>".to_string(),
        }
    }
}

// ============================================================================
// BlueprintPin
// ============================================================================

/// A connection point on a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintPin {
    id: PinId,
    name: String,
    direction: PinDirection,
    data_type: PinType,
    default_value: Option<PinValue>,
}

impl BlueprintPin {
    /// Create a new pin
    pub fn new(name: &str, dir: PinDirection, ty: PinType, default: Option<PinValue>) -> Self {
        Self {
            id: PinId(0), // Will be assigned by node
            name: name.to_string(),
            direction: dir,
            data_type: ty,
            default_value: default,
        }
    }

    pub fn id(&self) -> PinId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn direction(&self) -> PinDirection {
        self.direction
    }

    pub fn data_type(&self) -> PinType {
        self.data_type
    }

    pub fn default_value(&self) -> Option<&PinValue> {
        self.default_value.as_ref()
    }

    /// Check if this pin can connect to another pin
    pub fn can_connect(&self, other: &BlueprintPin) -> bool {
        // Direction check: input connects to output
        let dir_ok = self.direction != other.direction;
        // Type check: Exec pins only connect to Exec pins
        let type_ok = if self.data_type == PinType::Exec || other.data_type == PinType::Exec {
            self.data_type == other.data_type
        } else {
            self.data_type.is_compatible(&other.data_type)
        };
        dir_ok && type_ok
    }

    /// Set pin ID (used by node when adding pins)
    pub(crate) fn set_id(&mut self, id: PinId) {
        self.id = id;
    }
}

// ============================================================================
// NodeKind
// ============================================================================

/// Type/category of a blueprint node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    // Control flow
    If,
    For,
    While,
    Switch,
    Branch,
    Sequence,
    // Events
    BeginPlay,
    Tick,
    CustomEvent,
    EventDispatcher,
    // Variables
    VariableGet,
    VariableSet,
    // Logic
    And,
    Or,
    Not,
    // Math - basic
    Add,
    Subtract,
    Multiply,
    Divide,
    // Math - functions
    Abs,
    Sin,
    Cos,
    Sqrt,
    Min,
    Max,
    Clamp,
    Lerp,
    // Vector
    MakeVec2,
    MakeVec3,
    MakeVec4,
    BreakVec2,
    BreakVec3,
    BreakVec4,
    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    // Utility
    PrintString,
    Delay,
    Select,
    CastTo,
    IsValid,
    // Actor
    SpawnActor,
    DestroyActor,
    GetLocation,
    SetLocation,
    AddOffset,
    // Timer
    SetTimer,
    ClearTimer,
    // Special
    Function,
    Macro,
    Reroute,
    Comment,
}

impl NodeKind {
    /// Check if this node has exec pins
    pub fn has_exec_flow(&self) -> bool {
        matches!(
            self,
            NodeKind::If
                | NodeKind::For
                | NodeKind::While
                | NodeKind::Switch
                | NodeKind::Branch
                | NodeKind::Sequence
                | NodeKind::BeginPlay
                | NodeKind::Tick
                | NodeKind::CustomEvent
                | NodeKind::EventDispatcher
                | NodeKind::PrintString
                | NodeKind::Delay
                | NodeKind::SpawnActor
                | NodeKind::DestroyActor
                | NodeKind::SetLocation
                | NodeKind::AddOffset
                | NodeKind::SetTimer
                | NodeKind::ClearTimer
                | NodeKind::Function
        )
    }

    /// Get default name for this node kind
    pub fn default_name(&self) -> &'static str {
        match self {
            NodeKind::If => "If",
            NodeKind::For => "For Loop",
            NodeKind::While => "While Loop",
            NodeKind::Switch => "Switch",
            NodeKind::Branch => "Branch",
            NodeKind::Sequence => "Sequence",
            NodeKind::BeginPlay => "Begin Play",
            NodeKind::Tick => "Tick",
            NodeKind::CustomEvent => "Custom Event",
            NodeKind::EventDispatcher => "Event Dispatcher",
            NodeKind::VariableGet => "Get Variable",
            NodeKind::VariableSet => "Set Variable",
            NodeKind::Add => "Add",
            NodeKind::Subtract => "Subtract",
            NodeKind::Multiply => "Multiply",
            NodeKind::Divide => "Divide",
            NodeKind::Abs => "Abs",
            NodeKind::Sin => "Sin",
            NodeKind::Cos => "Cos",
            NodeKind::Sqrt => "Sqrt",
            NodeKind::Min => "Min",
            NodeKind::Max => "Max",
            NodeKind::Clamp => "Clamp",
            NodeKind::Lerp => "Lerp",
            NodeKind::MakeVec2 => "Make Vec2",
            NodeKind::MakeVec3 => "Make Vec3",
            NodeKind::MakeVec4 => "Make Vec4",
            NodeKind::BreakVec2 => "Break Vec2",
            NodeKind::BreakVec3 => "Break Vec3",
            NodeKind::BreakVec4 => "Break Vec4",
            NodeKind::Equal => "Equal",
            NodeKind::NotEqual => "Not Equal",
            NodeKind::Less => "Less",
            NodeKind::Greater => "Greater",
            NodeKind::LessEqual => "Less Equal",
            NodeKind::GreaterEqual => "Greater Equal",
            NodeKind::And => "And",
            NodeKind::Or => "Or",
            NodeKind::Not => "Not",
            NodeKind::PrintString => "Print String",
            NodeKind::Delay => "Delay",
            NodeKind::Select => "Select",
            NodeKind::CastTo => "Cast To",
            NodeKind::IsValid => "Is Valid",
            NodeKind::SpawnActor => "Spawn Actor",
            NodeKind::DestroyActor => "Destroy Actor",
            NodeKind::GetLocation => "Get Location",
            NodeKind::SetLocation => "Set Location",
            NodeKind::AddOffset => "Add Offset",
            NodeKind::SetTimer => "Set Timer",
            NodeKind::ClearTimer => "Clear Timer",
            NodeKind::Function => "Function",
            NodeKind::Macro => "Macro",
            NodeKind::Reroute => "Reroute",
            NodeKind::Comment => "Comment",
        }
    }
}

// ============================================================================
// BlueprintNode
// ============================================================================

/// A node in the blueprint graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintNode {
    id: NodeId,
    kind: NodeKind,
    name: String,
    position: (f32, f32),
    input_pins: Vec<BlueprintPin>,
    output_pins: Vec<BlueprintPin>,
    metadata: HashMap<String, String>,
}

impl BlueprintNode {
    /// Create a new node with the given kind and pins
    pub fn new(
        kind: NodeKind,
        inputs: Vec<BlueprintPin>,
        outputs: Vec<BlueprintPin>,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            id: NodeId(0), // Will be assigned by graph
            kind,
            name: kind.default_name().to_string(),
            position: (0.0, 0.0),
            input_pins: inputs,
            output_pins: outputs,
            metadata,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }

    pub fn input_pins(&self) -> &[BlueprintPin] {
        &self.input_pins
    }

    pub fn output_pins(&self) -> &[BlueprintPin] {
        &self.output_pins
    }

    pub fn pin(&self, id: PinId) -> Option<&BlueprintPin> {
        self.input_pins
            .iter()
            .find(|p| p.id() == id)
            .or_else(|| self.output_pins.iter().find(|p| p.id() == id))
    }

    pub fn pin_by_name(&self, name: &str) -> Option<&BlueprintPin> {
        self.input_pins
            .iter()
            .find(|p| p.name() == name)
            .or_else(|| self.output_pins.iter().find(|p| p.name() == name))
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Set node ID (used by graph when adding nodes)
    pub(crate) fn set_id(&mut self, id: NodeId) {
        self.id = id;
    }

    /// Assign pin IDs (used by graph when adding nodes)
    pub(crate) fn assign_pin_ids(&mut self) {
        let mut pin_counter = 0u32;
        for pin in &mut self.input_pins {
            pin.set_id(PinId(pin_counter));
            pin_counter += 1;
        }
        for pin in &mut self.output_pins {
            pin.set_id(PinId(pin_counter));
            pin_counter += 1;
        }
    }
}

// ============================================================================
// BlueprintWire
// ============================================================================

/// A connection between two pins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintWire {
    id: WireId,
    from: PinRef,
    to: PinRef,
}

impl BlueprintWire {
    pub fn new(from: PinRef, to: PinRef) -> Self {
        Self {
            id: WireId(0), // Will be assigned by graph
            from,
            to,
        }
    }

    pub fn id(&self) -> WireId {
        self.id
    }

    pub fn from(&self) -> PinRef {
        self.from
    }

    pub fn to(&self) -> PinRef {
        self.to
    }

    /// Set wire ID (used by graph when adding wires)
    pub(crate) fn set_id(&mut self, id: WireId) {
        self.id = id;
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Error during blueprint operations
#[derive(Debug, Error)]
pub enum BlueprintError {
    #[error("Node not found: {0:?}")]
    NodeNotFound(NodeId),

    #[error("Invalid pin reference: {pin:?}")]
    InvalidPinRef { pin: PinRef },

    #[error("Invalid connection from {from:?} to {to:?}: {reason}")]
    InvalidConnection {
        from: PinRef,
        to: PinRef,
        reason: String,
    },

    #[error("Duplicate wire from {from:?} to {to:?}")]
    DuplicateWire { from: PinRef, to: PinRef },

    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },

    #[error("Cycle detected in graph")]
    CycleDetected,
}

/// Error indicating a cycle in the graph
#[derive(Debug, Error)]
#[error("Cycle detected in blueprint graph")]
pub struct CycleError;

impl From<CycleError> for BlueprintError {
    fn from(_: CycleError) -> Self {
        BlueprintError::CycleDetected
    }
}

impl From<CycleError> for CompileError {
    fn from(e: CycleError) -> Self {
        CompileError::Blueprint(BlueprintError::from(e))
    }
}

// ============================================================================
// BlueprintGraph
// ============================================================================

/// Top-level blueprint graph containing nodes and wires
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintGraph {
    nodes: Vec<BlueprintNode>,
    wires: Vec<BlueprintWire>,
    next_node_id: u32,
    next_wire_id: u32,
    variables: HashMap<String, PinType>,
}

impl BlueprintGraph {
    /// Create an empty graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            wires: Vec::new(),
            next_node_id: 0,
            next_wire_id: 0,
            variables: HashMap::new(),
        }
    }

    /// Create a graph with pre-allocated capacity
    pub fn with_capacity(nodes: usize, wires: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(nodes),
            wires: Vec::with_capacity(wires),
            next_node_id: 0,
            next_wire_id: 0,
            variables: HashMap::new(),
        }
    }

    /// Add a node to the graph, returns its ID
    pub fn add_node(&mut self, mut node: BlueprintNode) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        node.set_id(id);
        node.assign_pin_ids();
        self.nodes.push(node);
        id
    }

    /// Remove a node and all connected wires
    pub fn remove_node(&mut self, node_id: NodeId) {
        // Remove all wires connected to this node
        self.wires
            .retain(|w| w.from.0 != node_id && w.to.0 != node_id);
        // Remove the node
        self.nodes.retain(|n| n.id() != node_id);
    }

    /// Check if a node exists
    pub fn contains_node(&self, id: NodeId) -> bool {
        self.nodes.iter().any(|n| n.id() == id)
    }

    /// Get a node by ID
    pub fn node(&self, id: NodeId) -> Option<&BlueprintNode> {
        self.nodes.iter().find(|n| n.id() == id)
    }

    /// Get a mutable node by ID
    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut BlueprintNode> {
        self.nodes.iter_mut().find(|n| n.id() == id)
    }

    /// Get all nodes
    pub fn nodes(&self) -> &[BlueprintNode] {
        &self.nodes
    }

    /// Get all nodes mutably
    pub fn nodes_mut(&mut self) -> &mut [BlueprintNode] {
        &mut self.nodes
    }

    /// Add a wire between two pins
    pub fn add_wire(&mut self, from: PinRef, to: PinRef) -> Result<WireId, BlueprintError> {
        // Validate connection
        let from_node = self
            .node(from.0)
            .ok_or(BlueprintError::InvalidPinRef { pin: from })?;
        let to_node = self
            .node(to.0)
            .ok_or(BlueprintError::InvalidPinRef { pin: to })?;
        let from_pin = from_node
            .pin(from.1)
            .ok_or(BlueprintError::InvalidPinRef { pin: from })?;
        let to_pin = to_node
            .pin(to.1)
            .ok_or(BlueprintError::InvalidPinRef { pin: to })?;

        if !from_pin.can_connect(to_pin) {
            return Err(BlueprintError::InvalidConnection {
                from,
                to,
                reason: "Type or direction mismatch".to_string(),
            });
        }

        // Check for duplicate wire
        if self.wires.iter().any(|w| w.from == from && w.to == to) {
            return Err(BlueprintError::DuplicateWire { from, to });
        }

        let id = WireId(self.next_wire_id);
        self.next_wire_id += 1;
        let mut wire = BlueprintWire::new(from, to);
        wire.set_id(id);
        self.wires.push(wire);
        Ok(id)
    }

    /// Remove a wire by ID
    pub fn remove_wire(&mut self, wire_id: WireId) {
        self.wires.retain(|w| w.id() != wire_id);
    }

    /// Get all wires
    pub fn wires(&self) -> &[BlueprintWire] {
        &self.wires
    }

    /// Get all wires mutably
    pub fn wires_mut(&mut self) -> &mut [BlueprintWire] {
        &mut self.wires
    }

    /// Get wires connected to a specific pin (incoming)
    pub fn wires_into_pin(&self, pin: PinRef) -> Vec<WireId> {
        self.wires
            .iter()
            .filter(|w| w.to == pin)
            .map(|w| w.id())
            .collect()
    }

    /// Get wires connected from a specific pin (outgoing)
    pub fn wires_out_of_pin(&self, pin: PinRef) -> Vec<WireId> {
        self.wires
            .iter()
            .filter(|w| w.from == pin)
            .map(|w| w.id())
            .collect()
    }

    /// Define a variable in the graph scope
    pub fn define_variable(&mut self, name: &str, ty: PinType) {
        self.variables.insert(name.to_string(), ty);
    }

    /// Get variable type
    pub fn variable_type(&self, name: &str) -> Option<PinType> {
        self.variables.get(name).copied()
    }

    /// Topological sort of nodes (for exec flow)
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, CycleError> {
        // Build adjacency list for exec flow
        let mut adj: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut in_degree: HashMap<NodeId, u32> = HashMap::new();

        for node in &self.nodes {
            in_degree.insert(node.id(), 0);
            adj.insert(node.id(), Vec::new());
        }

        // Find exec wire connections
        for wire in &self.wires {
            let from_node = self.node(wire.from.0);
            let to_node = self.node(wire.to.0);
            if let (Some(from), Some(to)) = (from_node, to_node) {
                let from_pin = from.pin(wire.from.1);
                let to_pin = to.pin(wire.to.1);
                if let (Some(fp), Some(tp)) = (from_pin, to_pin) {
                    if fp.data_type() == PinType::Exec && tp.data_type() == PinType::Exec {
                        adj.get_mut(&wire.from.0).unwrap().push(wire.to.0);
                        *in_degree.get_mut(&wire.to.0).unwrap() += 1;
                    }
                }
            }
        }

        // Kahn's algorithm
        let mut queue: Vec<NodeId> = in_degree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(n, _)| *n)
            .collect();

        let mut result = Vec::new();

        while let Some(node_id) = queue.pop() {
            result.push(node_id);
            for neighbor in adj.get(&node_id).unwrap_or(&Vec::new()) {
                let deg = in_degree.get_mut(neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(*neighbor);
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err(CycleError);
        }

        Ok(result)
    }

    /// Validate the graph
    pub fn validate(&self) -> Result<(), BlueprintError> {
        // Check for cycles in exec flow
        self.topological_sort()?;

        // Check all wires have valid pin references
        for wire in &self.wires {
            let from_node = self
                .node(wire.from.0)
                .ok_or(BlueprintError::InvalidPinRef { pin: wire.from })?;
            let to_node = self
                .node(wire.to.0)
                .ok_or(BlueprintError::InvalidPinRef { pin: wire.to })?;

            from_node
                .pin(wire.from.1)
                .ok_or(BlueprintError::InvalidPinRef { pin: wire.from })?;
            to_node
                .pin(wire.to.1)
                .ok_or(BlueprintError::InvalidPinRef { pin: wire.to })?;
        }

        // Check variable references
        for node in &self.nodes {
            if node.kind() == NodeKind::VariableGet || node.kind() == NodeKind::VariableSet {
                let var_name = node.metadata().get("variable_name").ok_or_else(|| {
                    BlueprintError::UndefinedVariable {
                        name: "unknown".to_string(),
                    }
                })?;
                if !self.variables.contains_key(var_name) {
                    return Err(BlueprintError::UndefinedVariable {
                        name: var_name.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for BlueprintGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Blueprint IR
// ============================================================================

/// Intermediate representation instruction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    Call(u32, u32), // (function_id, arg_count)
    Return,
    Yield(f32), // duration in seconds
    ActivateExec(NodeId, PinId),
    DeactivateExec(NodeId, PinId),
    Print,
    Halt,
}

/// Variable slot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSlot {
    name: String,
    ty: PinType,
}

impl VariableSlot {
    pub fn new(name: &str, ty: PinType) -> Self {
        Self {
            name: name.to_string(),
            ty,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> PinType {
        self.ty
    }
}

/// Function entry in IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionEntry {
    name: String,
    entry_point: InstrOffset,
    arg_count: u32,
}

impl FunctionEntry {
    pub fn new(name: &str, entry: InstrOffset, args: u32) -> Self {
        Self {
            name: name.to_string(),
            entry_point: entry,
            arg_count: args,
        }
    }
}

/// Compiled blueprint IR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintIR {
    instructions: Vec<BlueprintIRInstruction>,
    constants: Vec<PinValue>,
    variables: Vec<VariableSlot>,
    functions: Vec<FunctionEntry>,
    entry_point: InstrOffset,
}

impl BlueprintIR {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
            functions: Vec::new(),
            entry_point: InstrOffset(0),
        }
    }

    pub fn instructions(&self) -> &[BlueprintIRInstruction] {
        &self.instructions
    }

    pub fn constants(&self) -> &[PinValue] {
        &self.constants
    }

    pub fn variables(&self) -> &[VariableSlot] {
        &self.variables
    }

    pub fn functions(&self) -> &[FunctionEntry] {
        &self.functions
    }

    pub fn entry_point(&self) -> InstrOffset {
        self.entry_point
    }

    /// Serialize to binary (bincode)
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serde::encode_to_vec(self, bincode::config::standard())?)
    }

    /// Deserialize from binary with size limit to prevent DoS attacks
    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // 限制大小为 10MB
        const MAX_IR_SIZE: u64 = 10 * 1024 * 1024;
        if bytes.len() as u64 > MAX_IR_SIZE {
            return Err("IR too large".into());
        }
        Ok(bincode::serde::decode_from_slice(bytes, bincode::config::standard())?.0)
    }
}

impl Default for BlueprintIR {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Blueprint Compiler
// ============================================================================

/// Compile error
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Cycle detected at node {0:?}")]
    CycleDetected(NodeId),

    #[error("Type mismatch: expected {expected:?}, found {found:?}")]
    TypeMismatch { expected: PinType, found: PinType },

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Invalid wire {wire_id:?}: {reason}")]
    InvalidWire { wire_id: WireId, reason: String },

    #[error("Blueprint error: {0}")]
    Blueprint(#[from] BlueprintError),
}

/// Blueprint compiler
pub struct BlueprintCompiler {
    constants: Vec<PinValue>,
    variables: Vec<VariableSlot>,
    var_map: HashMap<String, VarSlot>,
    instructions: Vec<BlueprintIRInstruction>,
}

impl BlueprintCompiler {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            variables: Vec::new(),
            var_map: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    /// Compile a graph to IR
    pub fn compile(&mut self, graph: &BlueprintGraph) -> Result<BlueprintIR, CompileError> {
        // Validate first
        graph.validate()?;

        // Reset compiler state
        self.constants.clear();
        self.variables.clear();
        self.var_map.clear();
        self.instructions.clear();

        // Emit variables
        for (name, ty) in &graph.variables {
            self.emit_var(name, *ty);
        }

        // Get topological order for exec flow
        let order = graph.topological_sort()?;

        // Compile nodes in order
        for node_id in order {
            if let Some(node) = graph.node(node_id) {
                self.compile_node(node, graph)?;
            }
        }

        // Add halt instruction
        self.emit_instruction(BlueprintIRInstruction::Halt);

        Ok(BlueprintIR {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
            variables: self.variables.clone(),
            functions: Vec::new(),
            entry_point: InstrOffset(0),
        })
    }

    /// Emit a constant
    pub fn emit_const(&mut self, value: PinValue) -> ConstId {
        let id = ConstId(self.constants.len() as u32);
        self.constants.push(value);
        id
    }

    /// Emit a variable slot
    pub fn emit_var(&mut self, name: &str, ty: PinType) -> VarSlot {
        if let Some(slot) = self.var_map.get(name) {
            return *slot;
        }
        let slot = VarSlot(self.variables.len() as u32);
        self.variables.push(VariableSlot::new(name, ty));
        self.var_map.insert(name.to_string(), slot);
        slot
    }

    /// Emit an instruction
    pub fn emit_instruction(&mut self, instr: BlueprintIRInstruction) -> InstrOffset {
        let offset = InstrOffset(self.instructions.len() as u32);
        self.instructions.push(instr);
        offset
    }

    /// Compile a single node
    fn compile_node(
        &mut self,
        node: &BlueprintNode,
        graph: &BlueprintGraph,
    ) -> Result<(), CompileError> {
        match node.kind() {
            NodeKind::BeginPlay => {
                // Entry point - no action needed
            }

            NodeKind::PrintString => {
                // Get input value and print
                self.compile_data_inputs(node, graph)?;
                self.emit_instruction(BlueprintIRInstruction::Print);
                self.emit_instruction(BlueprintIRInstruction::Pop);
            }

            NodeKind::Add | NodeKind::Subtract | NodeKind::Multiply | NodeKind::Divide => {
                self.compile_data_inputs(node, graph)?;
                let op = match node.kind() {
                    NodeKind::Add => BlueprintIRInstruction::Add,
                    NodeKind::Subtract => BlueprintIRInstruction::Sub,
                    NodeKind::Multiply => BlueprintIRInstruction::Mul,
                    NodeKind::Divide => BlueprintIRInstruction::Div,
                    _ => BlueprintIRInstruction::Add,
                };
                self.emit_instruction(op);
            }

            NodeKind::VariableGet => {
                let var_name = node
                    .metadata()
                    .get("variable_name")
                    .ok_or_else(|| CompileError::UndefinedVariable("unknown".to_string()))?;
                let slot = self.emit_var(var_name, PinType::Any);
                self.emit_instruction(BlueprintIRInstruction::LoadVar(slot));
            }

            NodeKind::VariableSet => {
                let var_name = node
                    .metadata()
                    .get("variable_name")
                    .ok_or_else(|| CompileError::UndefinedVariable("unknown".to_string()))?;
                self.compile_data_inputs(node, graph)?;
                let slot = self.emit_var(var_name, PinType::Any);
                self.emit_instruction(BlueprintIRInstruction::StoreVar(slot));
            }

            NodeKind::If => {
                self.compile_data_inputs(node, graph)?;
                // JumpIfNot to skip the true branch
                let jump_offset = self.instructions.len() as u32;
                self.emit_instruction(BlueprintIRInstruction::JumpIfNot(InstrOffset(0))); // Placeholder
                                                                                          // Compile true branch (find connected nodes)
                                                                                          // ... (simplified for now)
                                                                                          // Fix jump target
                let end_offset = InstrOffset(self.instructions.len() as u32);
                self.instructions[jump_offset as usize] =
                    BlueprintIRInstruction::JumpIfNot(end_offset);
            }

            NodeKind::Delay => {
                // Get delay duration from input
                self.compile_data_inputs(node, graph)?;
                // Pop the duration and emit yield
                // For simplicity, use default 1.0 second
                self.emit_instruction(BlueprintIRInstruction::Yield(1.0));
            }

            _ => {
                // Default: compile inputs
                self.compile_data_inputs(node, graph)?;
            }
        }

        Ok(())
    }

    /// Compile data input connections for a node
    fn compile_data_inputs(
        &mut self,
        node: &BlueprintNode,
        graph: &BlueprintGraph,
    ) -> Result<(), CompileError> {
        for pin in node.input_pins() {
            if pin.data_type() == PinType::Exec {
                continue; // Skip exec pins
            }

            // Find wire connected to this pin
            let wire = graph
                .wires()
                .iter()
                .find(|w| w.to == PinRef(node.id(), pin.id()));

            if let Some(wire) = wire {
                // Get source node and pin
                if let Some(src_node) = graph.node(wire.from.0) {
                    if let Some(_src_pin) = src_node.pin(wire.from.1) {
                        // The source node should have already been compiled
                        // Its output value is on the stack or in a variable
                        // For simplicity, we assume it's on the stack
                    }
                }
            } else if let Some(default) = pin.default_value() {
                // Use default value
                let const_id = self.emit_const(default.clone());
                self.emit_instruction(BlueprintIRInstruction::PushConst(const_id));
            }
        }

        Ok(())
    }
}

impl Default for BlueprintCompiler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Blueprint Interpreter
// ============================================================================

/// Runtime error
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Stack overflow")]
    StackOverflow,

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid cast")]
    InvalidCast,

    #[error("Type mismatch")]
    TypeMismatch,

    #[error("Bad instruction at offset {0:?}")]
    BadInstruction(InstrOffset),

    #[error("Unknown function: {0}")]
    UnknownFunction(String),

    #[error("Execution timeout: exceeded maximum instruction count")]
    InstructionLimitExceeded,
}

/// Interpreter state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpreterState {
    Running,
    Paused,
    Yielding,
    Halted,
    Error,
}

/// Blueprint execution context
pub struct BlueprintContext {
    delta_time: f32,
    current_entity: Option<u64>,
}

impl BlueprintContext {
    pub fn new(delta_time: f32) -> Self {
        Self {
            delta_time,
            current_entity: None,
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn set_entity(&mut self, entity: u64) {
        self.current_entity = Some(entity);
    }

    pub fn entity(&self) -> Option<u64> {
        self.current_entity
    }
}

/// Stack-based blueprint interpreter
pub struct BlueprintInterpreter {
    ir: Arc<BlueprintIR>,
    stack: Vec<PinValue>,
    variables: Vec<PinValue>,
    pc: InstrOffset,
    state: InterpreterState,
    yield_timer: f32,
}

impl BlueprintInterpreter {
    pub fn new(ir: Arc<BlueprintIR>) -> Self {
        let vars = ir.variables.iter().map(|_| PinValue::Invalid).collect();
        Self {
            ir,
            stack: Vec::with_capacity(MAX_STACK_DEPTH),
            variables: vars,
            pc: InstrOffset(0),
            state: InterpreterState::Halted,
            yield_timer: 0.0,
        }
    }

    pub fn stack(&self) -> &[PinValue] {
        &self.stack
    }

    pub fn variables(&self) -> &[PinValue] {
        &self.variables
    }

    pub fn pc(&self) -> InstrOffset {
        self.pc
    }

    pub fn state(&self) -> InterpreterState {
        self.state
    }

    /// Reset interpreter to initial state
    pub fn reset(&mut self) {
        self.stack.clear();
        for var in &mut self.variables {
            *var = PinValue::Invalid;
        }
        self.pc = self.ir.entry_point;
        self.state = InterpreterState::Running;
        self.yield_timer = 0.0;
    }

    /// Run until halt or error
    pub fn run(&mut self, ctx: &mut BlueprintContext) -> Result<(), RuntimeError> {
        self.state = InterpreterState::Running;
        let mut instruction_count: u64 = 0;
        while self.state == InterpreterState::Running {
            self.step(ctx)?;
            instruction_count += 1;
            if instruction_count > MAX_INSTRUCTIONS_PER_RUN {
                return Err(RuntimeError::InstructionLimitExceeded);
            }
        }
        if self.state == InterpreterState::Error {
            return Err(RuntimeError::BadInstruction(self.pc));
        }
        Ok(())
    }

    /// Execute one instruction
    pub fn step(&mut self, ctx: &mut BlueprintContext) -> Result<(), RuntimeError> {
        if self.state == InterpreterState::Yielding {
            self.yield_timer -= ctx.delta_time;
            if self.yield_timer <= 0.0 {
                self.state = InterpreterState::Running;
            }
            return Ok(());
        }

        // Get instruction offset first
        let offset = self.pc.0 as usize;
        if offset >= self.ir.instructions.len() {
            self.state = InterpreterState::Halted;
            return Ok(());
        }

        // Clone instruction to avoid borrow conflict
        let instr = self.ir.instructions[offset].clone();
        self.execute_instruction(&instr, ctx)?;

        self.pc = InstrOffset(self.pc.0 + 1);
        Ok(())
    }

    /// Execute a single instruction
    fn execute_instruction(
        &mut self,
        instr: &BlueprintIRInstruction,
        _ctx: &mut BlueprintContext,
    ) -> Result<(), RuntimeError> {
        match instr {
            BlueprintIRInstruction::Nop => {}

            BlueprintIRInstruction::PushConst(id) => {
                let value = self
                    .ir
                    .constants
                    .get(id.0 as usize)
                    .cloned()
                    .unwrap_or(PinValue::Invalid);
                self.push(value)?;
            }

            BlueprintIRInstruction::LoadVar(slot) => {
                let value = self
                    .variables
                    .get(slot.0 as usize)
                    .cloned()
                    .unwrap_or(PinValue::Invalid);
                self.push(value)?;
            }

            BlueprintIRInstruction::StoreVar(slot) => {
                let value = self.pop()?;
                if (slot.0 as usize) < self.variables.len() {
                    self.variables[slot.0 as usize] = value;
                }
            }

            BlueprintIRInstruction::Dup => {
                let value = self.stack.last().cloned().unwrap_or(PinValue::Invalid);
                self.push(value)?;
            }

            BlueprintIRInstruction::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                }
            }

            BlueprintIRInstruction::Pop => {
                self.pop()?;
            }

            BlueprintIRInstruction::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Int(x + y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Float(x + y),
                    (PinValue::Vec2(x), PinValue::Vec2(y)) => {
                        PinValue::Vec2([x[0] + y[0], x[1] + y[1]])
                    }
                    (PinValue::Vec3(x), PinValue::Vec3(y)) => {
                        PinValue::Vec3([x[0] + y[0], x[1] + y[1], x[2] + y[2]])
                    }
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Sub => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Int(x - y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Float(x - y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Mul => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Int(x * y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Float(x * y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Div => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => {
                        if *y == 0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        PinValue::Int(x / y)
                    }
                    (PinValue::Float(x), PinValue::Float(y)) => {
                        if *y == 0.0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        PinValue::Float(x / y)
                    }
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Neg => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Int(x) => PinValue::Int(-x),
                    PinValue::Float(x) => PinValue::Float(-x),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Abs => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Int(x) => PinValue::Int(x.abs()),
                    PinValue::Float(x) => PinValue::Float(x.abs()),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Sin => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Float(x) => PinValue::Float(x.sin()),
                    PinValue::Int(x) => PinValue::Float((x as f64).sin()),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Cos => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Float(x) => PinValue::Float(x.cos()),
                    PinValue::Int(x) => PinValue::Float((x as f64).cos()),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Sqrt => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Float(x) if x >= 0.0 => PinValue::Float(x.sqrt()),
                    PinValue::Int(x) if x >= 0 => PinValue::Float((x as f64).sqrt()),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Lt => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Bool(x < y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Bool(x < y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Le => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Bool(x <= y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Bool(x <= y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Gt => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Bool(x > y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Bool(x > y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Ge => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => PinValue::Bool(x >= y),
                    (PinValue::Float(x), PinValue::Float(y)) => PinValue::Bool(x >= y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Eq => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(PinValue::Bool(a == b))?;
            }

            BlueprintIRInstruction::Ne => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(PinValue::Bool(a != b))?;
            }

            BlueprintIRInstruction::And => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Bool(x), PinValue::Bool(y)) => PinValue::Bool(*x && *y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Or => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Bool(x), PinValue::Bool(y)) => PinValue::Bool(*x || *y),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Not => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Bool(x) => PinValue::Bool(!x),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Jump(offset) => {
                self.pc = *offset;
                // Decrement PC because step() will increment it
                self.pc = InstrOffset(self.pc.0.saturating_sub(1));
            }

            BlueprintIRInstruction::JumpIf(offset) => {
                let cond = self.pop()?;
                if cond.to_bool() == Some(true) {
                    self.pc = *offset;
                    self.pc = InstrOffset(self.pc.0.saturating_sub(1));
                }
            }

            BlueprintIRInstruction::JumpIfNot(offset) => {
                let cond = self.pop()?;
                if cond.to_bool() != Some(true) {
                    self.pc = *offset;
                    self.pc = InstrOffset(self.pc.0.saturating_sub(1));
                }
            }

            BlueprintIRInstruction::Yield(duration) => {
                self.yield_timer = *duration;
                self.state = InterpreterState::Yielding;
            }

            BlueprintIRInstruction::Print => {
                let value = self.pop()?;
                println!("Blueprint: {}", value.to_string_lossy());
            }

            BlueprintIRInstruction::Halt => {
                self.state = InterpreterState::Halted;
            }

            BlueprintIRInstruction::Return => {
                self.state = InterpreterState::Halted;
            }

            BlueprintIRInstruction::Call(fn_id, _argc) => {
                // Look up function
                if let Some(func) = self.ir.functions.get(*fn_id as usize) {
                    self.pc = func.entry_point;
                    self.pc = InstrOffset(self.pc.0.saturating_sub(1));
                } else {
                    return Err(RuntimeError::UnknownFunction(format!("fn_{}", fn_id)));
                }
            }

            BlueprintIRInstruction::ActivateExec(_, _)
            | BlueprintIRInstruction::DeactivateExec(_, _) => {
                // Exec flow control - no runtime action needed
            }

            BlueprintIRInstruction::Rem => {
                let b = self.pop()?;
                let a = self.pop()?;
                let result = match (&a, &b) {
                    (PinValue::Int(x), PinValue::Int(y)) => {
                        if *y == 0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        PinValue::Int(x % y)
                    }
                    (PinValue::Float(x), PinValue::Float(y)) => {
                        if *y == 0.0 {
                            return Err(RuntimeError::DivisionByZero);
                        }
                        PinValue::Float(x % y)
                    }
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Tan => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Float(x) => PinValue::Float(x.tan()),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Log => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Float(x) => PinValue::Float(x.log(std::f64::consts::E)),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }

            BlueprintIRInstruction::Exp => {
                let a = self.pop()?;
                let result = match a {
                    PinValue::Float(x) => PinValue::Float(x.exp()),
                    _ => PinValue::Invalid,
                };
                self.push(result)?;
            }
        }

        Ok(())
    }

    /// Push value onto stack
    pub fn push(&mut self, value: PinValue) -> Result<(), RuntimeError> {
        if self.stack.len() >= MAX_STACK_DEPTH {
            return Err(RuntimeError::StackOverflow);
        }
        self.stack.push(value);
        Ok(())
    }

    /// Pop value from stack
    pub fn pop(&mut self) -> Result<PinValue, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }
}

// ============================================================================
// Script VM
// ============================================================================

/// Script source type
#[derive(Debug, Clone)]
pub enum ScriptSource {
    Path(std::path::PathBuf),
    Code(String),
    Bytes(Vec<u8>),
}

/// Script value type
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<ScriptValue>),
    Map(HashMap<String, ScriptValue>),
    Entity(u64),
}

impl ScriptValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            ScriptValue::Null => "null",
            ScriptValue::Bool(_) => "bool",
            ScriptValue::Int(_) => "int",
            ScriptValue::Float(_) => "float",
            ScriptValue::String(_) => "string",
            ScriptValue::Array(_) => "array",
            ScriptValue::Map(_) => "map",
            ScriptValue::Entity(_) => "entity",
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            ScriptValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn to_int(&self) -> Option<i64> {
        match self {
            ScriptValue::Int(n) => Some(*n),
            ScriptValue::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        match self {
            ScriptValue::Float(f) => Some(*f),
            ScriptValue::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn to_string_lossy(&self) -> String {
        match self {
            ScriptValue::Null => "null".to_string(),
            ScriptValue::Bool(b) => b.to_string(),
            ScriptValue::Int(n) => n.to_string(),
            ScriptValue::Float(f) => f.to_string(),
            ScriptValue::String(s) => s.clone(),
            ScriptValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_lossy()).collect();
                format!("[{}]", items.join(", "))
            }
            ScriptValue::Map(map) => {
                let items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_lossy()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            ScriptValue::Entity(e) => format!("Entity({})", e),
        }
    }
}

impl From<PinValue> for ScriptValue {
    fn from(value: PinValue) -> Self {
        match value {
            PinValue::Bool(b) => ScriptValue::Bool(b),
            PinValue::Int(n) => ScriptValue::Int(n),
            PinValue::Float(f) => ScriptValue::Float(f),
            PinValue::String(s) => ScriptValue::String(s),
            PinValue::Entity(e) => ScriptValue::Entity(e),
            PinValue::Vec2(v) => ScriptValue::Array(vec![
                ScriptValue::Float(v[0] as f64),
                ScriptValue::Float(v[1] as f64),
            ]),
            PinValue::Vec3(v) => ScriptValue::Array(vec![
                ScriptValue::Float(v[0] as f64),
                ScriptValue::Float(v[1] as f64),
                ScriptValue::Float(v[2] as f64),
            ]),
            PinValue::Vec4(v) => ScriptValue::Array(vec![
                ScriptValue::Float(v[0] as f64),
                ScriptValue::Float(v[1] as f64),
                ScriptValue::Float(v[2] as f64),
                ScriptValue::Float(v[3] as f64),
            ]),
            _ => ScriptValue::Null,
        }
    }
}

impl From<ScriptValue> for PinValue {
    fn from(value: ScriptValue) -> Self {
        match value {
            ScriptValue::Bool(b) => PinValue::Bool(b),
            ScriptValue::Int(n) => PinValue::Int(n),
            ScriptValue::Float(f) => PinValue::Float(f),
            ScriptValue::String(s) => PinValue::String(s),
            ScriptValue::Entity(e) => PinValue::Entity(e),
            ScriptValue::Array(arr) if arr.len() == 2 => {
                let x = arr[0].to_float().unwrap_or(0.0) as f32;
                let y = arr[1].to_float().unwrap_or(0.0) as f32;
                PinValue::Vec2([x, y])
            }
            ScriptValue::Array(arr) if arr.len() == 3 => {
                let x = arr[0].to_float().unwrap_or(0.0) as f32;
                let y = arr[1].to_float().unwrap_or(0.0) as f32;
                let z = arr[2].to_float().unwrap_or(0.0) as f32;
                PinValue::Vec3([x, y, z])
            }
            ScriptValue::Array(arr) if arr.len() == 4 => {
                let x = arr[0].to_float().unwrap_or(0.0) as f32;
                let y = arr[1].to_float().unwrap_or(0.0) as f32;
                let z = arr[2].to_float().unwrap_or(0.0) as f32;
                let w = arr[3].to_float().unwrap_or(0.0) as f32;
                PinValue::Vec4([x, y, z, w])
            }
            _ => PinValue::Invalid,
        }
    }
}

/// Script error
#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Compile error: {0}")]
    CompileError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Timeout")]
    Timeout,

    #[error("Memory limit exceeded")]
    MemoryLimit,

    #[error("Function not found: {0}")]
    NotFound(String),

    #[error("Invalid argument: {0}")]
    InvalidArg(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Script instance
#[derive(Debug)]
pub struct ScriptInstance {
    handle: ScriptHandle,
    functions: HashMap<String, u32>,
    last_error: Option<ScriptError>,
}

impl ScriptInstance {
    pub fn new(handle: ScriptHandle) -> Self {
        Self {
            handle,
            functions: HashMap::new(),
            last_error: None,
        }
    }

    pub fn handle(&self) -> ScriptHandle {
        self.handle
    }

    pub fn has(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn last_error(&self) -> Option<&ScriptError> {
        self.last_error.as_ref()
    }

    pub fn set_error(&mut self, err: ScriptError) {
        self.last_error = Some(err);
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
}

/// Script VM trait
pub trait ScriptVM {
    /// Load a script from source
    fn load(&mut self, source: ScriptSource) -> Result<ScriptHandle, ScriptError>;

    /// Call a function in a script
    fn call(
        &mut self,
        handle: ScriptHandle,
        fn_name: &str,
        args: &[ScriptValue],
    ) -> Result<ScriptValue, ScriptError>;

    /// Set a global variable
    fn set_global(&mut self, name: &str, value: ScriptValue);

    /// Get a global variable
    fn get_global(&self, name: &str) -> Option<ScriptValue>;

    /// Update the VM (called each frame)
    fn update(&mut self, dt: f32);

    /// Run garbage collection
    fn gc(&mut self);
}

/// Simple blueprint-based script VM
pub struct BlueprintScriptVM {
    instances: HashMap<ScriptHandle, (Arc<BlueprintIR>, BlueprintInterpreter)>,
    globals: HashMap<String, ScriptValue>,
    next_handle: u64,
}

impl BlueprintScriptVM {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            globals: HashMap::new(),
            next_handle: 0,
        }
    }

    /// Load a blueprint IR as a script
    pub fn load_ir(&mut self, ir: BlueprintIR) -> Result<ScriptHandle, ScriptError> {
        let handle = ScriptHandle(self.next_handle);
        self.next_handle += 1;
        let arc_ir = Arc::new(ir);
        let interpreter = BlueprintInterpreter::new(arc_ir.clone());
        self.instances.insert(handle, (arc_ir, interpreter));
        Ok(handle)
    }
}

impl ScriptVM for BlueprintScriptVM {
    fn load(&mut self, source: ScriptSource) -> Result<ScriptHandle, ScriptError> {
        match source {
            ScriptSource::Code(code) => {
                // Parse as JSON blueprint graph
                let graph = BlueprintGraph::from_json(&code)
                    .map_err(|e| ScriptError::CompileError(e.to_string()))?;

                let mut compiler = BlueprintCompiler::new();
                let ir = compiler
                    .compile(&graph)
                    .map_err(|e| ScriptError::CompileError(e.to_string()))?;

                self.load_ir(ir)
            }
            ScriptSource::Path(path) => {
                let code = std::fs::read_to_string(&path)?;
                self.load(ScriptSource::Code(code))
            }
            ScriptSource::Bytes(bytes) => {
                // Try to deserialize as IR
                let ir = BlueprintIR::deserialize(&bytes)
                    .map_err(|e| ScriptError::CompileError(e.to_string()))?;
                self.load_ir(ir)
            }
        }
    }

    fn call(
        &mut self,
        handle: ScriptHandle,
        _fn_name: &str,
        args: &[ScriptValue],
    ) -> Result<ScriptValue, ScriptError> {
        let (_ir, interpreter) = self
            .instances
            .get_mut(&handle)
            .ok_or_else(|| ScriptError::NotFound(format!("handle {}", handle.0)))?;

        interpreter.reset();

        // Push args onto stack
        for arg in args {
            interpreter
                .push(PinValue::from(arg.clone()))
                .map_err(|_| ScriptError::RuntimeError("Stack overflow".to_string()))?;
        }

        let mut ctx = BlueprintContext::new(0.0);
        interpreter
            .run(&mut ctx)
            .map_err(|e| ScriptError::RuntimeError(e.to_string()))?;

        // Return top of stack
        let result = interpreter
            .stack()
            .last()
            .cloned()
            .map(ScriptValue::from)
            .unwrap_or(ScriptValue::Null);

        Ok(result)
    }

    fn set_global(&mut self, name: &str, value: ScriptValue) {
        self.globals.insert(name.to_string(), value);
    }

    fn get_global(&self, name: &str) -> Option<ScriptValue> {
        self.globals.get(name).cloned()
    }

    fn update(&mut self, dt: f32) {
        for (_, interpreter) in self.instances.values_mut() {
            if interpreter.state() == InterpreterState::Yielding {
                let mut ctx = BlueprintContext::new(dt);
                // Continue execution
                while interpreter.state() == InterpreterState::Running
                    || interpreter.state() == InterpreterState::Yielding
                {
                    interpreter.step(&mut ctx).ok();
                }
            }
        }
    }

    fn gc(&mut self) {
        // Blueprint VM doesn't need GC
    }
}

impl Default for BlueprintScriptVM {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_type_compatibility() {
        assert!(PinType::Int.is_compatible(&PinType::Int));
        assert!(PinType::Any.is_compatible(&PinType::Int));
        assert!(PinType::Int.is_compatible(&PinType::Any));
        assert!(!PinType::Int.is_compatible(&PinType::Bool));
        assert!(!PinType::Exec.is_compatible(&PinType::Int));
    }

    #[test]
    fn test_pin_value_coerce() {
        let int_val = PinValue::Int(42);
        let float_val = int_val.coerce_to(PinType::Float);
        assert_eq!(float_val, Some(PinValue::Float(42.0)));

        let str_val = PinValue::String("123".to_string());
        let parsed = str_val.coerce_to(PinType::Int);
        assert_eq!(parsed, Some(PinValue::Int(123)));
    }

    #[test]
    fn test_pin_can_connect() {
        let input_int = BlueprintPin::new("a", PinDirection::Input, PinType::Int, None);
        let output_int = BlueprintPin::new("b", PinDirection::Output, PinType::Int, None);
        assert!(input_int.can_connect(&output_int));

        let input_exec = BlueprintPin::new("exec", PinDirection::Input, PinType::Exec, None);
        assert!(!input_int.can_connect(&input_exec));
    }

    #[test]
    fn test_graph_add_remove_node() {
        let mut graph = BlueprintGraph::new();
        let node = BlueprintNode::new(NodeKind::Add, vec![], vec![], HashMap::new());
        let id = graph.add_node(node);
        assert!(graph.contains_node(id));

        graph.remove_node(id);
        assert!(!graph.contains_node(id));
    }

    #[test]
    fn test_graph_add_wire() {
        let mut graph = BlueprintGraph::with_capacity(2, 1);

        let node1 = BlueprintNode::new(
            NodeKind::Add,
            vec![BlueprintPin::new(
                "a",
                PinDirection::Input,
                PinType::Int,
                None,
            )],
            vec![BlueprintPin::new(
                "result",
                PinDirection::Output,
                PinType::Int,
                None,
            )],
            HashMap::new(),
        );
        let id1 = graph.add_node(node1);

        let node2 = BlueprintNode::new(
            NodeKind::PrintString,
            vec![BlueprintPin::new(
                "value",
                PinDirection::Input,
                PinType::Int,
                None,
            )],
            vec![],
            HashMap::new(),
        );
        let id2 = graph.add_node(node2);

        // Get pin IDs
        let pin1 = graph.node(id1).unwrap().output_pins()[0].id();
        let pin2 = graph.node(id2).unwrap().input_pins()[0].id();

        let wire_id = graph.add_wire(PinRef(id1, pin1), PinRef(id2, pin2));
        assert!(wire_id.is_ok());
    }

    #[test]
    fn test_topological_sort_cycle() {
        let mut graph = BlueprintGraph::new();

        // Create nodes with exec pins
        let node1 = BlueprintNode::new(
            NodeKind::Function,
            vec![BlueprintPin::new(
                "exec_in",
                PinDirection::Input,
                PinType::Exec,
                None,
            )],
            vec![BlueprintPin::new(
                "exec_out",
                PinDirection::Output,
                PinType::Exec,
                None,
            )],
            HashMap::new(),
        );
        let node2 = BlueprintNode::new(
            NodeKind::Function,
            vec![BlueprintPin::new(
                "exec_in",
                PinDirection::Input,
                PinType::Exec,
                None,
            )],
            vec![BlueprintPin::new(
                "exec_out",
                PinDirection::Output,
                PinType::Exec,
                None,
            )],
            HashMap::new(),
        );

        let id1 = graph.add_node(node1);
        let id2 = graph.add_node(node2);

        // Create cycle: 1 -> 2 -> 1
        let pin1_out = graph.node(id1).unwrap().output_pins()[0].id();
        let pin2_in = graph.node(id2).unwrap().input_pins()[0].id();
        let pin2_out = graph.node(id2).unwrap().output_pins()[0].id();
        let pin1_in = graph.node(id1).unwrap().input_pins()[0].id();

        graph
            .add_wire(PinRef(id1, pin1_out), PinRef(id2, pin2_in))
            .unwrap();
        graph
            .add_wire(PinRef(id2, pin2_out), PinRef(id1, pin1_in))
            .unwrap();

        let result = graph.topological_sort();
        assert!(result.is_err());
    }

    #[test]
    fn test_compiler_hello_world() {
        let mut graph = BlueprintGraph::new();
        graph.define_variable("message", PinType::String);

        let begin = BlueprintNode::new(
            NodeKind::BeginPlay,
            vec![BlueprintPin::new(
                "exec",
                PinDirection::Input,
                PinType::Exec,
                None,
            )],
            vec![BlueprintPin::new(
                "exec",
                PinDirection::Output,
                PinType::Exec,
                None,
            )],
            HashMap::new(),
        );
        graph.add_node(begin);

        let print = BlueprintNode::new(
            NodeKind::PrintString,
            vec![
                BlueprintPin::new("exec", PinDirection::Input, PinType::Exec, None),
                BlueprintPin::new(
                    "value",
                    PinDirection::Input,
                    PinType::String,
                    Some(PinValue::String("Hello Blueprint".to_string())),
                ),
            ],
            vec![BlueprintPin::new(
                "exec",
                PinDirection::Output,
                PinType::Exec,
                None,
            )],
            HashMap::new(),
        );
        graph.add_node(print);

        let mut compiler = BlueprintCompiler::new();
        let ir = compiler.compile(&graph).unwrap();

        assert!(ir.instructions().len() >= 3);
    }

    #[test]
    fn test_interpreter_add() {
        let mut ir = BlueprintIR::new();
        ir.constants.push(PinValue::Int(10));
        ir.constants.push(PinValue::Int(20));
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(0)));
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(1)));
        ir.instructions.push(BlueprintIRInstruction::Add);
        ir.instructions.push(BlueprintIRInstruction::Halt);

        let arc_ir = Arc::new(ir);
        let mut interpreter = BlueprintInterpreter::new(arc_ir.clone());
        let mut ctx = BlueprintContext::new(0.0);

        interpreter.reset();
        interpreter.run(&mut ctx).unwrap();

        assert_eq!(interpreter.stack().len(), 1);
        assert_eq!(interpreter.stack()[0], PinValue::Int(30));
    }

    #[test]
    fn test_script_value_conversion() {
        let pin_val = PinValue::Int(42);
        let script_val: ScriptValue = pin_val.clone().into();
        assert_eq!(script_val, ScriptValue::Int(42));

        let back: PinValue = script_val.into();
        assert_eq!(back, PinValue::Int(42));
    }

    #[test]
    fn test_script_vm_load() {
        let mut vm = BlueprintScriptVM::new();

        // Create simple graph
        let mut graph = BlueprintGraph::new();
        let node = BlueprintNode::new(
            NodeKind::Add,
            vec![],
            vec![BlueprintPin::new(
                "result",
                PinDirection::Output,
                PinType::Int,
                None,
            )],
            HashMap::new(),
        );
        graph.add_node(node);

        let json = graph.to_json().unwrap();
        let handle = vm.load(ScriptSource::Code(json)).unwrap();

        assert!(vm.instances.contains_key(&handle));
    }

    #[test]
    fn test_ir_serialize_deserialize() {
        let mut ir = BlueprintIR::new();
        ir.constants.push(PinValue::Int(100));
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(0)));
        ir.instructions.push(BlueprintIRInstruction::Halt);

        let bytes = ir.serialize().unwrap();
        let decoded = BlueprintIR::deserialize(&bytes).unwrap();

        assert_eq!(decoded.instructions().len(), 2);
        assert_eq!(decoded.constants().len(), 1);
    }

    #[test]
    fn test_interpreter_division_by_zero() {
        let mut ir = BlueprintIR::new();
        ir.constants.push(PinValue::Int(10));
        ir.constants.push(PinValue::Int(0));
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(0)));
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(1)));
        ir.instructions.push(BlueprintIRInstruction::Div);
        ir.instructions.push(BlueprintIRInstruction::Halt);

        let arc_ir = Arc::new(ir);
        let mut interpreter = BlueprintInterpreter::new(arc_ir);
        let mut ctx = BlueprintContext::new(0.0);

        interpreter.reset();
        let result = interpreter.run(&mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_interpreter_loop() {
        // Simulate For loop: sum = 0; for i in 0..10 { sum += i }
        let mut ir = BlueprintIR::new();
        ir.variables.push(VariableSlot::new("sum", PinType::Int));
        ir.variables.push(VariableSlot::new("i", PinType::Int));

        // Constants: 0=0, 1=10, 2=1
        ir.constants.push(PinValue::Int(0));
        ir.constants.push(PinValue::Int(10));
        ir.constants.push(PinValue::Int(1));

        // sum = 0 (offset 0-1)
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(0))); // 0
        ir.instructions
            .push(BlueprintIRInstruction::StoreVar(VarSlot(0))); // 1

        // i = 0 (offset 2-3)
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(0))); // 2
        ir.instructions
            .push(BlueprintIRInstruction::StoreVar(VarSlot(1))); // 3

        // Loop start at offset 4
        ir.instructions
            .push(BlueprintIRInstruction::LoadVar(VarSlot(1))); // 4: push i
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(1))); // 5: push 10
        ir.instructions.push(BlueprintIRInstruction::Lt); // 6: i < 10
        ir.instructions
            .push(BlueprintIRInstruction::JumpIfNot(InstrOffset(17))); // 7: if not, jump to halt

        // sum += i (offset 8-11)
        ir.instructions
            .push(BlueprintIRInstruction::LoadVar(VarSlot(0))); // 8: push sum
        ir.instructions
            .push(BlueprintIRInstruction::LoadVar(VarSlot(1))); // 9: push i
        ir.instructions.push(BlueprintIRInstruction::Add); // 10: sum + i
        ir.instructions
            .push(BlueprintIRInstruction::StoreVar(VarSlot(0))); // 11: store sum

        // i += 1 (offset 12-15)
        ir.instructions
            .push(BlueprintIRInstruction::LoadVar(VarSlot(1))); // 12: push i
        ir.instructions
            .push(BlueprintIRInstruction::PushConst(ConstId(2))); // 13: push 1
        ir.instructions.push(BlueprintIRInstruction::Add); // 14: i + 1
        ir.instructions
            .push(BlueprintIRInstruction::StoreVar(VarSlot(1))); // 15: store i

        // Jump back to loop start (offset 16)
        ir.instructions
            .push(BlueprintIRInstruction::Jump(InstrOffset(4))); // 16: jump to offset 4

        // End (offset 17)
        ir.instructions.push(BlueprintIRInstruction::Halt); // 17

        let arc_ir = Arc::new(ir);
        let mut interpreter = BlueprintInterpreter::new(arc_ir);
        let mut ctx = BlueprintContext::new(0.0);

        interpreter.reset();
        interpreter.run(&mut ctx).unwrap();

        // sum should be 45 (0+1+2+...+9)
        assert_eq!(interpreter.variables()[0], PinValue::Int(45));
    }
}
