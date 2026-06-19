//! Shader Graph - Node-based shader editor system

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

/// Unique identifier for a shader graph node
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u32);

/// Unique identifier for a shader graph edge
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeId(pub u32);

/// Shader node type specification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShaderNodeType {
    /// Float scalar
    Float,
    /// 2D vector
    Vec2,
    /// 3D vector
    Vec3,
    /// 4D vector
    Vec4,
    /// Color (RGBA)
    Color,
    /// Boolean
    Bool,
    /// Texture sampler
    Sampler,
    /// Void (no output)
    Void,
}

/// Node kind for shader graph
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    // Input nodes
    /// Input parameter with name and type
    Input { name: String, ty: ShaderNodeType },
    /// Output parameter with name and type
    Output { name: String, ty: ShaderNodeType },

    // Constant nodes
    /// Float constant value
    ConstantFloat(f32),
    /// Vec2 constant value
    ConstantVec2([f32; 2]),
    /// Vec3 constant value
    ConstantVec3([f32; 3]),
    /// Vec4 constant value
    ConstantVec4([f32; 4]),
    /// Color constant value
    ConstantColor([f32; 4]),
    /// Boolean constant
    ConstantBool(bool),

    // Texture nodes
    /// Texture sample with UV input
    TextureSample { name: String },

    // Math binary operations
    /// Add two values
    Add,
    /// Subtract two values
    Sub,
    /// Multiply two values
    Mul,
    /// Divide two values
    Div,
    /// Power operation
    Pow,
    /// Minimum of two values
    Min,
    /// Maximum of two values
    Max,
    /// Dot product
    Dot,
    /// Cross product
    Cross,
    /// Distance between two points
    Distance,

    // Math unary operations
    /// Negate value
    Negate,
    /// Absolute value
    Abs,
    /// Sign function
    Sign,
    /// Square root
    Sqrt,
    /// Natural logarithm
    Log,
    /// Exponential
    Exp,
    /// Sine
    Sin,
    /// Cosine
    Cos,
    /// Tangent
    Tan,
    /// Floor
    Floor,
    /// Ceiling
    Ceil,
    /// Round
    Round,
    /// Normalize vector
    Normalize,
    /// Vector length
    Length,

    // Color operations
    /// Swizzle channels
    Swizzle { pattern: String },
    /// Mix two colors
    Mix,
    /// Convert to sRGB
    ToSrgb,
    /// Convert to linear
    ToLinear,
    /// Apply gamma
    Gamma,

    // UV operations
    /// UV tiling
    Tiling,
    /// UV offset
    Offset,
    /// UV rotation
    Rotate,
    /// UV pan animation
    Pan,

    // Time nodes
    /// Current time
    Time,
    /// Sine of time
    SinTime,
    /// Cosine of time
    CosTime,

    // Normal map
    /// Normal map sampling
    NormalMap { strength: f32 },

    // PBR Master node
    /// PBR master output node
    PbrMaster,

    // Vertex data
    /// Vertex position
    VertexPosition,
    /// Vertex normal
    VertexNormal,
    /// Vertex UV channel 0
    VertexUV0,
    /// Vertex UV channel 1
    VertexUV1,
    /// Vertex color
    VertexColor,
    /// Vertex tangent
    VertexTangent,

    // Fragment data
    /// Fragment world-space normal
    FragmentNormalWS,
    /// Fragment world-space view direction
    FragmentViewDirWS,
    /// Fragment world-space light direction
    FragmentLightDirWS,
    /// Shadow coordinate
    FragmentShadowCoord,

    // Control flow
    /// Conditional branch
    If,
    /// Switch/multi-case
    Switch { cases: u32 },

    // Custom
    /// Custom code node
    Custom { name: String, code: String },
}

/// Shader graph node with metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShaderGraphNode {
    /// Node kind/type
    pub kind: NodeKind,
    /// Node identifier
    pub id: NodeId,
    /// Editor position (x, y) for visualization
    pub position: [f32; 2],
    /// Optional comment/annotation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Node color for editor visualization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<[f32; 4]>,
}

/// Edge connecting two nodes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    /// Edge identifier
    pub id: EdgeId,
    /// Source node (output)
    pub from: NodeId,
    /// Target node (input)
    pub to: NodeId,
    /// Output slot index on source node
    pub from_slot: u32,
    /// Input slot index on target node
    pub to_slot: u32,
}

/// Error when graph contains cycles
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CycleError;

/// Shader graph for node-based shader editing
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShaderGraph {
    /// Graph name
    pub name: String,
    /// All nodes in the graph
    nodes: HashMap<NodeId, ShaderGraphNode>,
    /// All edges in the graph
    edges: HashMap<EdgeId, Edge>,
    /// Next node ID to allocate
    next_node_id: u32,
    /// Next edge ID to allocate
    next_edge_id: u32,
}

impl ShaderGraph {
    /// Create a new empty shader graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Get graph name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set graph name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get all nodes
    pub fn nodes(&self) -> &HashMap<NodeId, ShaderGraphNode> {
        &self.nodes
    }

    /// Get all edges
    pub fn edges(&self) -> &HashMap<EdgeId, Edge> {
        &self.edges
    }

    /// Add a new node to the graph
    pub fn add_node(&mut self, kind: NodeKind) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = ShaderGraphNode {
            kind,
            id,
            position: [0.0, 0.0],
            comment: None,
            color: None,
        };

        self.nodes.insert(id, node);
        id
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        // Remove all edges connected to this node
        self.edges.retain(|_, e| e.from != id && e.to != id);
    }

    /// Add an edge connecting two nodes
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, from_slot: u32, to_slot: u32) -> EdgeId {
        let id = EdgeId(self.next_edge_id);
        self.next_edge_id += 1;

        let edge = Edge {
            id,
            from,
            to,
            from_slot,
            to_slot,
        };

        self.edges.insert(id, edge);
        id
    }

    /// Remove an edge from the graph
    pub fn remove_edge(&mut self, id: EdgeId) {
        self.edges.remove(&id);
    }

    /// Compute topological ordering of nodes
    ///
    /// Returns error if graph contains cycles
    pub fn topological_order(&self) -> Result<Vec<NodeId>, CycleError> {
        let mut visited: HashSet<NodeId> = HashSet::new();
        let mut result = Vec::new();

        // Build adjacency list (reverse: from -> list of nodes that depend on it)
        let mut dependents: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut dependencies: HashMap<NodeId, u32> = HashMap::new();

        for node in self.nodes().values() {
            dependencies.insert(node.id, 0);
            dependents.insert(node.id, Vec::new());
        }

        for edge in self.edges().values() {
            if let Some(dep_count) = dependencies.get_mut(&edge.to) {
                *dep_count += 1;
            }
            if let Some(deps) = dependents.get_mut(&edge.from) {
                deps.push(edge.to);
            }
        }

        // Kahn's algorithm
        let mut queue: Vec<NodeId> = dependencies
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(id, _)| *id)
            .collect();

        while let Some(node_id) = queue.pop() {
            result.push(node_id);
            visited.insert(node_id);

            if let Some(deps) = dependents.get(&node_id) {
                for dep_id in deps {
                    if let Some(count) = dependencies.get_mut(dep_id) {
                        *count -= 1;
                        if *count == 0 && !visited.contains(dep_id) {
                            queue.push(*dep_id);
                        }
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err(CycleError);
        }

        Ok(result)
    }

    /// Validate the graph structure
    pub fn validate(&self) -> Result<(), CycleError> {
        self.topological_order()?;
        Ok(())
    }

    /// Serialize graph to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize graph from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Generate WGSL shader code from the graph
    pub fn generate_wgsl(&self) -> Result<String, CycleError> {
        let order = self.topological_order()?;
        let mut code = String::new();

        // Generate shader header
        code.push_str("// Generated shader from ShaderGraph\n\n");

        // Generate each node's code in topological order
        for node_id in order {
            if let Some(node) = self.nodes.get(&node_id) {
                let node_code = self.generate_node_code(node);
                code.push_str(&node_code);
                code.push('\n');
            }
        }

        Ok(code)
    }

    /// Generate code for a single node
    fn generate_node_code(&self, node: &ShaderGraphNode) -> String {
        match &node.kind {
            NodeKind::Input { name, ty } => {
                let ty_str = self.type_to_wgsl(*ty);
                format!("var {}: {};", name, ty_str)
            }
            NodeKind::Output { name, ty } => {
                let ty_str = self.type_to_wgsl(*ty);
                format!("let {}: {};", name, ty_str)
            }
            NodeKind::ConstantFloat(v) => format!("let n{}: f32 = {}f;", node.id.0, v),
            NodeKind::ConstantVec2(v) => {
                format!("let n{}: vec2<f32> = vec2({}, {});", node.id.0, v[0], v[1])
            }
            NodeKind::ConstantVec3(v) => format!(
                "let n{}: vec3<f32> = vec3({}, {}, {});",
                node.id.0, v[0], v[1], v[2]
            ),
            NodeKind::ConstantVec4(v) => format!(
                "let n{}: vec4<f32> = vec4({}, {}, {}, {});",
                node.id.0, v[0], v[1], v[2], v[3]
            ),
            NodeKind::ConstantBool(v) => format!("let n{}: bool = {};", node.id.0, v),
            NodeKind::Add => format!("let n{} = a{} + b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Sub => format!("let n{} = a{} - b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Mul => format!("let n{} = a{} * b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Div => format!("let n{} = a{} / b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Normalize => format!("let n{} = normalize(v{});", node.id.0, node.id.0),
            NodeKind::Length => format!("let n{} = length(v{});", node.id.0, node.id.0),
            NodeKind::Time => "let time = time_uniform;".to_string(),
            NodeKind::PbrMaster => "// PBR Master node - outputs to fragment shader".to_string(),
            _ => format!("// Node: {:?}", node.kind),
        }
    }

    /// Convert shader node type to WGSL type string
    fn type_to_wgsl(&self, ty: ShaderNodeType) -> &'static str {
        match ty {
            ShaderNodeType::Float => "f32",
            ShaderNodeType::Vec2 => "vec2<f32>",
            ShaderNodeType::Vec3 => "vec3<f32>",
            ShaderNodeType::Vec4 => "vec4<f32>",
            ShaderNodeType::Color => "vec4<f32>",
            ShaderNodeType::Bool => "bool",
            ShaderNodeType::Sampler => "sampler",
            ShaderNodeType::Void => "()",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let graph = ShaderGraph::new();
        assert_eq!(graph.name(), "");
        assert!(graph.nodes().is_empty());
        assert!(graph.edges().is_empty());
    }

    #[test]
    fn test_graph_add_node() {
        let mut graph = ShaderGraph::new();
        let id = graph.add_node(NodeKind::ConstantFloat(1.0));
        assert!(graph.nodes().contains_key(&id));
    }

    #[test]
    fn test_graph_remove_node() {
        let mut graph = ShaderGraph::new();
        let id = graph.add_node(NodeKind::ConstantFloat(1.0));
        graph.remove_node(id);
        assert!(!graph.nodes().contains_key(&id));
    }

    #[test]
    fn test_graph_add_edge() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::ConstantFloat(1.0));
        let b = graph.add_node(NodeKind::Add);
        let edge_id = graph.add_edge(a, b, 0, 0);
        assert!(graph.edges().contains_key(&edge_id));
    }

    #[test]
    fn test_graph_remove_edge() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::ConstantFloat(1.0));
        let b = graph.add_node(NodeKind::Add);
        let edge_id = graph.add_edge(a, b, 0, 0);
        graph.remove_edge(edge_id);
        assert!(!graph.edges().contains_key(&edge_id));
    }

    #[test]
    fn test_graph_topological_order_simple() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::ConstantFloat(1.0));
        let b = graph.add_node(NodeKind::ConstantFloat(2.0));
        let c = graph.add_node(NodeKind::Add);
        graph.add_edge(a, c, 0, 0);
        graph.add_edge(b, c, 0, 1);

        let order = graph.topological_order().unwrap();
        // a and b should come before c
        let a_pos = order.iter().position(|&id| id == a).unwrap();
        let b_pos = order.iter().position(|&id| id == b).unwrap();
        let c_pos = order.iter().position(|&id| id == c).unwrap();
        assert!(a_pos < c_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_graph_cycle_detection() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::Add);
        let b = graph.add_node(NodeKind::Add);
        graph.add_edge(a, b, 0, 0);
        graph.add_edge(b, a, 0, 0); // Creates cycle

        let result = graph.topological_order();
        assert!(result.is_err());
    }

    #[test]
    fn test_graph_json_roundtrip() {
        let mut graph = ShaderGraph::new();
        graph.set_name("test_graph".to_string());
        graph.add_node(NodeKind::ConstantFloat(1.0));

        let json = graph.to_json().unwrap();
        let parsed = ShaderGraph::from_json(&json).unwrap();
        assert_eq!(graph.name, parsed.name);
        assert_eq!(graph.nodes.len(), parsed.nodes.len());
    }

    #[test]
    fn test_graph_generate_wgsl() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantFloat(1.0));
        graph.add_node(NodeKind::Time);

        let code = graph.generate_wgsl().unwrap();
        assert!(code.contains("f32"));
    }

    #[test]
    fn test_graph_node_kind_input() {
        let kind = NodeKind::Input {
            name: "albedo".to_string(),
            ty: ShaderNodeType::Vec3,
        };
        if let NodeKind::Input { name, ty } = kind {
            assert_eq!(name, "albedo");
            assert_eq!(ty, ShaderNodeType::Vec3);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_node_kind_output() {
        let kind = NodeKind::Output {
            name: "out".to_string(),
            ty: ShaderNodeType::Color,
        };
        if let NodeKind::Output { name, ty } = kind {
            assert_eq!(name, "out");
            assert_eq!(ty, ShaderNodeType::Color);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_constant_vec2() {
        let kind = NodeKind::ConstantVec2([0.5, 0.75]);
        if let NodeKind::ConstantVec2(v) = kind {
            assert_eq!(v, [0.5, 0.75]);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_constant_vec3() {
        let kind = NodeKind::ConstantVec3([1.0, 0.5, 0.2]);
        if let NodeKind::ConstantVec3(v) = kind {
            assert_eq!(v, [1.0, 0.5, 0.2]);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_constant_vec4() {
        let kind = NodeKind::ConstantVec4([1.0, 0.5, 0.2, 0.8]);
        if let NodeKind::ConstantVec4(v) = kind {
            assert_eq!(v, [1.0, 0.5, 0.2, 0.8]);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_constant_color() {
        let kind = NodeKind::ConstantColor([1.0, 0.0, 0.0, 1.0]);
        if let NodeKind::ConstantColor(v) = kind {
            assert_eq!(v, [1.0, 0.0, 0.0, 1.0]);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_constant_bool() {
        let kind = NodeKind::ConstantBool(true);
        if let NodeKind::ConstantBool(v) = kind {
            assert!(v);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_node_kind_texture_sample() {
        let kind = NodeKind::TextureSample {
            name: "tex".to_string(),
        };
        if let NodeKind::TextureSample { name } = kind {
            assert_eq!(name, "tex");
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_node_kind_normal_map() {
        let kind = NodeKind::NormalMap { strength: 2.0 };
        if let NodeKind::NormalMap { strength } = kind {
            assert_eq!(strength, 2.0);
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_node_kind_custom() {
        let kind = NodeKind::Custom {
            name: "func".to_string(),
            code: "fn func() {}".to_string(),
        };
        if let NodeKind::Custom { name, code } = kind {
            assert_eq!(name, "func");
            assert_eq!(code, "fn func() {}");
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_node_kind_swizzle() {
        let kind = NodeKind::Swizzle {
            pattern: "rgb".to_string(),
        };
        if let NodeKind::Swizzle { pattern } = kind {
            assert_eq!(pattern, "rgb");
        } else {
            panic!("Wrong node kind");
        }
    }

    #[test]
    fn test_graph_add_multiple_nodes() {
        let mut graph = ShaderGraph::new();
        let id1 = graph.add_node(NodeKind::ConstantFloat(1.0));
        let id2 = graph.add_node(NodeKind::ConstantFloat(2.0));
        let id3 = graph.add_node(NodeKind::Add);
        let _edge1 = graph.add_edge(id1, id3, 0, 0);
        let _edge2 = graph.add_edge(id2, id3, 0, 1);

        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 2);
    }

    #[test]
    fn test_graph_remove_node_removes_edges() {
        let mut graph = ShaderGraph::new();
        let id1 = graph.add_node(NodeKind::ConstantFloat(1.0));
        let id2 = graph.add_node(NodeKind::Add);
        let _edge = graph.add_edge(id1, id2, 0, 0);

        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.edges().len(), 1);

        graph.remove_node(id1);
        assert_eq!(graph.nodes().len(), 1);
        assert_eq!(graph.edges().len(), 0);
    }

    #[test]
    fn test_graph_validate_empty() {
        let graph = ShaderGraph::new();
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn test_graph_validate_with_cycle_fails() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::Add);
        let b = graph.add_node(NodeKind::Add);
        graph.add_edge(a, b, 0, 0);
        graph.add_edge(b, a, 0, 0);

        assert!(graph.validate().is_err());
    }

    #[test]
    fn test_graph_validate_with_valid_graph() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::ConstantFloat(1.0));
        let b = graph.add_node(NodeKind::ConstantFloat(2.0));
        let c = graph.add_node(NodeKind::Add);
        graph.add_edge(a, c, 0, 0);
        graph.add_edge(b, c, 0, 1);

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn test_graph_topological_order_long_chain() {
        let mut graph = ShaderGraph::new();
        let nodes: Vec<NodeId> = (0..5)
            .map(|_| graph.add_node(NodeKind::ConstantFloat(1.0)))
            .collect();
        for i in 0..4 {
            graph.add_edge(nodes[i], nodes[i + 1], 0, 0);
        }
        let _order = graph.topological_order().unwrap();
        // Just verify topological order succeeds for a chain graph
    }

    #[test]
    fn test_graph_node_id_type() {
        let nid = NodeId(42);
        assert_eq!(nid.0, 42);
    }

    #[test]
    fn test_graph_edge_id_type() {
        let eid = EdgeId(7);
        assert_eq!(eid.0, 7);
    }

    #[test]
    fn test_graph_json_empty_roundtrip() {
        let graph = ShaderGraph::new();
        let json = graph.to_json().unwrap();
        let parsed = ShaderGraph::from_json(&json).unwrap();
        assert_eq!(parsed.nodes().len(), 0);
        assert_eq!(parsed.edges().len(), 0);
    }

    #[test]
    fn test_graph_node_default() {
        let n: NodeId = NodeId::default();
        assert_eq!(n.0, 0);
    }

    #[test]
    fn test_graph_edge_default() {
        let e: EdgeId = EdgeId::default();
        assert_eq!(e.0, 0);
    }
}
