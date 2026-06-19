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
                let node_code = self.generate_node_code_wgsl(node);
                code.push_str(&node_code);
                code.push('\n');
            }
        }

        Ok(code)
    }

    /// Generate GLSL shader code from the graph
    ///
    /// Produces GLSL 330+ compatible code. Supports all node types
    /// that the WGSL generator supports, with GLSL-specific syntax.
    pub fn generate_glsl(&self) -> Result<String, CycleError> {
        let order = self.topological_order()?;
        let mut code = String::new();

        // Generate shader header
        code.push_str("#version 330\n");
        code.push_str("// Generated shader from ShaderGraph\n\n");

        // Generate each node's code in topological order
        for node_id in order {
            if let Some(node) = self.nodes.get(&node_id) {
                let node_code = self.generate_node_code_glsl(node);
                code.push_str(&node_code);
                code.push('\n');
            }
        }

        Ok(code)
    }

    /// Generate code for a single node (WGSL)
    fn generate_node_code_wgsl(&self, node: &ShaderGraphNode) -> String {
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

    /// Generate code for a single node (GLSL)
    fn generate_node_code_glsl(&self, node: &ShaderGraphNode) -> String {
        match &node.kind {
            NodeKind::Input { name, ty } => {
                let ty_str = self.type_to_glsl(*ty);
                format!("{} {};", ty_str, name)
            }
            NodeKind::Output { name, ty } => {
                let ty_str = self.type_to_glsl(*ty);
                format!("{} {};", ty_str, name)
            }
            NodeKind::ConstantFloat(v) => format!("float n{} = {};", node.id.0, Self::fmt_float(*v)),
            NodeKind::ConstantVec2(v) => {
                format!(
                    "vec2 n{} = vec2({}, {});",
                    node.id.0,
                    Self::fmt_float(v[0]),
                    Self::fmt_float(v[1])
                )
            }
            NodeKind::ConstantVec3(v) => format!(
                "vec3 n{} = vec3({}, {}, {});",
                node.id.0,
                Self::fmt_float(v[0]),
                Self::fmt_float(v[1]),
                Self::fmt_float(v[2])
            ),
            NodeKind::ConstantVec4(v) => format!(
                "vec4 n{} = vec4({}, {}, {}, {});",
                node.id.0,
                Self::fmt_float(v[0]),
                Self::fmt_float(v[1]),
                Self::fmt_float(v[2]),
                Self::fmt_float(v[3])
            ),
            NodeKind::ConstantColor(v) => format!(
                "vec4 n{} = vec4({}, {}, {}, {});",
                node.id.0, v[0], v[1], v[2], v[3]
            ),
            NodeKind::ConstantBool(v) => format!("bool n{} = {};", node.id.0, v),
            // Binary operations
            NodeKind::Add => format!("n{} = a{} + b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Sub => format!("n{} = a{} - b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Mul => format!("n{} = a{} * b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Div => format!("n{} = a{} / b{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Pow => format!("n{} = pow(a{}, b{});", node.id.0, node.id.0, node.id.0),
            NodeKind::Min => format!("n{} = min(a{}, b{});", node.id.0, node.id.0, node.id.0),
            NodeKind::Max => format!("n{} = max(a{}, b{});", node.id.0, node.id.0, node.id.0),
            NodeKind::Dot => format!("n{} = dot(a{}, b{});", node.id.0, node.id.0, node.id.0),
            NodeKind::Cross => format!("n{} = cross(a{}, b{});", node.id.0, node.id.0, node.id.0),
            NodeKind::Distance => format!("n{} = distance(a{}, b{});", node.id.0, node.id.0, node.id.0),
            // Unary operations
            NodeKind::Negate => format!("n{} = -v{};", node.id.0, node.id.0),
            NodeKind::Abs => format!("n{} = abs(v{});", node.id.0, node.id.0),
            NodeKind::Sign => format!("n{} = sign(v{});", node.id.0, node.id.0),
            NodeKind::Sqrt => format!("n{} = sqrt(v{});", node.id.0, node.id.0),
            NodeKind::Log => format!("n{} = log(v{});", node.id.0, node.id.0),
            NodeKind::Exp => format!("n{} = exp(v{});", node.id.0, node.id.0),
            NodeKind::Sin => format!("n{} = sin(v{});", node.id.0, node.id.0),
            NodeKind::Cos => format!("n{} = cos(v{});", node.id.0, node.id.0),
            NodeKind::Tan => format!("n{} = tan(v{});", node.id.0, node.id.0),
            NodeKind::Floor => format!("n{} = floor(v{});", node.id.0, node.id.0),
            NodeKind::Ceil => format!("n{} = ceil(v{});", node.id.0, node.id.0),
            NodeKind::Round => format!("n{} = round(v{});", node.id.0, node.id.0),
            NodeKind::Normalize => format!("n{} = normalize(v{});", node.id.0, node.id.0),
            NodeKind::Length => format!("n{} = length(v{});", node.id.0, node.id.0),
            // Color operations
            NodeKind::Swizzle { pattern } => {
                format!("n{} = v{}.{};", node.id.0, node.id.0, pattern)
            }
            NodeKind::Mix => format!("n{} = mix(a{}, b{}, t{});", node.id.0, node.id.0, node.id.0, node.id.0),
            NodeKind::ToSrgb => format!("n{} = pow(v{}, vec3(1.0/2.2));", node.id.0, node.id.0),
            NodeKind::ToLinear => format!("n{} = pow(v{}, vec3(2.2));", node.id.0, node.id.0),
            NodeKind::Gamma => format!("n{} = pow(v{}, vec3(g{}));", node.id.0, node.id.0, node.id.0),
            // UV operations
            NodeKind::Tiling => format!("n{} = uv{} * t{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Offset => format!("n{} = uv{} + o{};", node.id.0, node.id.0, node.id.0),
            NodeKind::Rotate => format!("n{} = rotateUV(uv{}, a{});", node.id.0, node.id.0, node.id.0),
            NodeKind::Pan => format!("n{} = uv{} + time * s{};", node.id.0, node.id.0, node.id.0),
            // Time nodes
            NodeKind::Time => "float time = time_uniform;".to_string(),
            NodeKind::SinTime => "float sin_time = sin(time_uniform);".to_string(),
            NodeKind::CosTime => "float cos_time = cos(time_uniform);".to_string(),
            // Normal map
            NodeKind::NormalMap { strength } => {
                format!("vec3 n{} = normalize(texture(normalMap, uv{}).xyz * 2.0 - 1.0 * {});",
                    node.id.0, node.id.0, strength)
            }
            // PBR Master
            NodeKind::PbrMaster => "// PBR Master node - outputs to fragment shader".to_string(),
            // Vertex data
            NodeKind::VertexPosition => "vec3 vertex_pos = in_position;".to_string(),
            NodeKind::VertexNormal => "vec3 vertex_normal = in_normal;".to_string(),
            NodeKind::VertexUV0 => "vec2 vertex_uv0 = in_uv0;".to_string(),
            NodeKind::VertexUV1 => "vec2 vertex_uv1 = in_uv1;".to_string(),
            NodeKind::VertexColor => "vec4 vertex_color = in_color;".to_string(),
            NodeKind::VertexTangent => "vec3 vertex_tangent = in_tangent;".to_string(),
            // Fragment data
            NodeKind::FragmentNormalWS => "vec3 frag_normal_ws = normal_ws;".to_string(),
            NodeKind::FragmentViewDirWS => "vec3 frag_view_dir_ws = view_dir_ws;".to_string(),
            NodeKind::FragmentLightDirWS => "vec3 frag_light_dir_ws = light_dir_ws;".to_string(),
            NodeKind::FragmentShadowCoord => "vec4 frag_shadow_coord = shadow_coord;".to_string(),
            // Texture sample
            NodeKind::TextureSample { name } => {
                format!("vec4 n{} = texture({}, uv{});", node.id.0, name, node.id.0)
            }
            // Control flow
            NodeKind::If => format!("n{} = (c{}) ? a{} : b{};", node.id.0, node.id.0, node.id.0, node.id.0),
            NodeKind::Switch { cases } => {
                let mut s = format!("switch(v{}) {{\n", node.id.0);
                for i in 0..*cases {
                    s.push_str(&format!("  case {}: r{} = v{}_{};\n", i, node.id.0, node.id.0, i));
                }
                s.push_str("}\n");
                s.push_str(&format!("n{} = r{};", node.id.0, node.id.0));
                s
            }
            // Custom
            NodeKind::Custom { name, code } => {
                format!("// Custom node: {}\n{}", name, code)
            }
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

    /// Convert shader node type to GLSL type string
    fn type_to_glsl(&self, ty: ShaderNodeType) -> &'static str {
        match ty {
            ShaderNodeType::Float => "float",
            ShaderNodeType::Vec2 => "vec2",
            ShaderNodeType::Vec3 => "vec3",
            ShaderNodeType::Vec4 => "vec4",
            ShaderNodeType::Color => "vec4",
            ShaderNodeType::Bool => "bool",
            ShaderNodeType::Sampler => "sampler2D",
            ShaderNodeType::Void => "void",
        }
    }

    /// Format a float for GLSL output, ensuring it always has a decimal point
    fn fmt_float(v: f32) -> String {
        let s = format!("{}", v);
        if s.contains('.') || s.contains("e") || s.contains("E") || s.contains("inf") || s.contains("NaN") {
            s
        } else {
            format!("{}.0", s)
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
    fn test_graph_generate_glsl() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantFloat(1.0));
        graph.add_node(NodeKind::Time);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("#version 330"));
        assert!(code.contains("float"));
    }

    #[test]
    fn test_graph_generate_glsl_vec3() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantVec3([1.0, 0.5, 0.2]));

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("vec3"));
        assert!(code.contains("1.0"));
    }

    #[test]
    fn test_graph_generate_glsl_vec4() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantVec4([1.0, 0.5, 0.2, 0.8]));

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("vec4"));
    }

    #[test]
    fn test_graph_generate_glsl_color() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantColor([1.0, 0.0, 0.0, 1.0]));

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("vec4"));
    }

    #[test]
    fn test_graph_generate_glsl_bool() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantBool(true));

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("bool"));
    }

    #[test]
    fn test_graph_generate_glsl_add() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::ConstantFloat(1.0));
        let b = graph.add_node(NodeKind::ConstantFloat(2.0));
        let c = graph.add_node(NodeKind::Add);
        graph.add_edge(a, c, 0, 0);
        graph.add_edge(b, c, 0, 1);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("+"));
    }

    #[test]
    fn test_graph_generate_glsl_sub() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Sub);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("-"));
    }

    #[test]
    fn test_graph_generate_glsl_mul() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Mul);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("*"));
    }

    #[test]
    fn test_graph_generate_glsl_div() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Div);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("/"));
    }

    #[test]
    fn test_graph_generate_glsl_math_functions() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Sin);
        graph.add_node(NodeKind::Cos);
        graph.add_node(NodeKind::Abs);
        graph.add_node(NodeKind::Sqrt);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("sin("));
        assert!(code.contains("cos("));
        assert!(code.contains("abs("));
        assert!(code.contains("sqrt("));
    }

    #[test]
    fn test_graph_generate_glsl_normalize_length() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Normalize);
        graph.add_node(NodeKind::Length);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("normalize("));
        assert!(code.contains("length("));
    }

    #[test]
    fn test_graph_generate_glsl_dot_cross() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Dot);
        graph.add_node(NodeKind::Cross);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("dot("));
        assert!(code.contains("cross("));
    }

    #[test]
    fn test_graph_generate_glsl_texture_sample() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::TextureSample {
            name: "albedo_tex".to_string(),
        });

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("texture("));
        assert!(code.contains("albedo_tex"));
    }

    #[test]
    fn test_graph_generate_glsl_vertex_data() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::VertexPosition);
        graph.add_node(NodeKind::VertexNormal);
        graph.add_node(NodeKind::VertexUV0);
        graph.add_node(NodeKind::VertexColor);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("in_position"));
        assert!(code.contains("in_normal"));
        assert!(code.contains("in_uv0"));
        assert!(code.contains("in_color"));
    }

    #[test]
    fn test_graph_generate_glsl_fragment_data() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::FragmentNormalWS);
        graph.add_node(NodeKind::FragmentViewDirWS);
        graph.add_node(NodeKind::FragmentLightDirWS);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("normal_ws"));
        assert!(code.contains("view_dir_ws"));
        assert!(code.contains("light_dir_ws"));
    }

    #[test]
    fn test_graph_generate_glsl_time_nodes() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Time);
        graph.add_node(NodeKind::SinTime);
        graph.add_node(NodeKind::CosTime);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("time_uniform"));
        assert!(code.contains("sin("));
        assert!(code.contains("cos("));
    }

    #[test]
    fn test_graph_generate_glsl_normal_map() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::NormalMap { strength: 1.5 });

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("normalMap"));
        assert!(code.contains("1.5"));
    }

    #[test]
    fn test_graph_generate_glsl_pbr_master() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::PbrMaster);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("PBR Master"));
    }

    #[test]
    fn test_graph_generate_glsl_custom() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Custom {
            name: "my_func".to_string(),
            code: "float my_func() { return 1.0; }".to_string(),
        });

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("my_func"));
        assert!(code.contains("return 1.0"));
    }

    #[test]
    fn test_graph_generate_glsl_swizzle() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Swizzle {
            pattern: "xyz".to_string(),
        });

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains(".xyz"));
    }

    #[test]
    fn test_graph_generate_glsl_mix() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Mix);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("mix("));
    }

    #[test]
    fn test_graph_generate_glsl_input_output() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Input {
            name: "albedo".to_string(),
            ty: ShaderNodeType::Vec3,
        });
        graph.add_node(NodeKind::Output {
            name: "frag_color".to_string(),
            ty: ShaderNodeType::Color,
        });

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("vec3 albedo"));
        assert!(code.contains("vec4 frag_color"));
    }

    #[test]
    fn test_graph_generate_glsl_if() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::If);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("?"));
        assert!(code.contains(":"));
    }

    #[test]
    fn test_graph_generate_glsl_switch() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Switch { cases: 3 });

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("switch"));
        assert!(code.contains("case"));
    }

    #[test]
    fn test_graph_generate_glsl_cycle_error() {
        let mut graph = ShaderGraph::new();
        let a = graph.add_node(NodeKind::Add);
        let b = graph.add_node(NodeKind::Add);
        graph.add_edge(a, b, 0, 0);
        graph.add_edge(b, a, 0, 0);

        let result = graph.generate_glsl();
        assert!(result.is_err());
    }

    #[test]
    fn test_graph_generate_glsl_empty() {
        let graph = ShaderGraph::new();
        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("#version 330"));
    }

    #[test]
    fn test_graph_generate_glsl_pow_min_max() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Pow);
        graph.add_node(NodeKind::Min);
        graph.add_node(NodeKind::Max);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("pow("));
        assert!(code.contains("min("));
        assert!(code.contains("max("));
    }

    #[test]
    fn test_graph_generate_glsl_floor_ceil_round() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Floor);
        graph.add_node(NodeKind::Ceil);
        graph.add_node(NodeKind::Round);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("floor("));
        assert!(code.contains("ceil("));
        assert!(code.contains("round("));
    }

    #[test]
    fn test_graph_generate_glsl_distance() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Distance);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("distance("));
    }

    #[test]
    fn test_graph_generate_glsl_negate() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Negate);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("-v"));
    }

    #[test]
    fn test_graph_generate_glsl_sign() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Sign);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("sign("));
    }

    #[test]
    fn test_graph_generate_glsl_log_exp() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Log);
        graph.add_node(NodeKind::Exp);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("log("));
        assert!(code.contains("exp("));
    }

    #[test]
    fn test_graph_generate_glsl_tan() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Tan);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("tan("));
    }

    #[test]
    fn test_graph_generate_glsl_to_srgb() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ToSrgb);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("pow("));
        assert!(code.contains("2.2"));
    }

    #[test]
    fn test_graph_generate_glsl_to_linear() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ToLinear);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("pow("));
        assert!(code.contains("2.2"));
    }

    #[test]
    fn test_graph_generate_glsl_uv_operations() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Tiling);
        graph.add_node(NodeKind::Offset);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("uv"));
    }

    #[test]
    fn test_graph_generate_glsl_pan() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Pan);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("time"));
    }

    #[test]
    fn test_graph_generate_both_wgsl_and_glsl() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::ConstantFloat(1.0));
        graph.add_node(NodeKind::ConstantVec3([1.0, 0.0, 0.0]));
        graph.add_node(NodeKind::Add);

        let wgsl = graph.generate_wgsl().unwrap();
        let glsl = graph.generate_glsl().unwrap();

        // WGSL uses f32, GLSL uses float
        assert!(wgsl.contains("f32"));
        assert!(glsl.contains("float"));
        // WGSL uses vec3<f32>, GLSL uses vec3
        assert!(wgsl.contains("vec3<f32>"));
        assert!(!glsl.contains("vec3<f32>"));
    }

    #[test]
    fn test_graph_generate_glsl_shadow_coord() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::FragmentShadowCoord);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("shadow_coord"));
    }

    #[test]
    fn test_graph_generate_glsl_vertex_tangent_uv1() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::VertexTangent);
        graph.add_node(NodeKind::VertexUV1);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("in_tangent"));
        assert!(code.contains("in_uv1"));
    }

    #[test]
    fn test_graph_generate_glsl_gamma() {
        let mut graph = ShaderGraph::new();
        graph.add_node(NodeKind::Gamma);

        let code = graph.generate_glsl().unwrap();
        assert!(code.contains("pow("));
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
