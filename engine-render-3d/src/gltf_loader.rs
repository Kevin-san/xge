//! GLTF/GLB model loader
//!
//! Provides parsing of GLTF JSON and GLB binary formats, with accessor decoding
//! for vertex positions, normals, texcoords, and indices. Outputs Mesh3D instances.

use crate::mesh::{Mesh3D, Primitive};
use crate::vertex::Vertex;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use engine_math::{Vec2, Vec3};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors produced by the GLTF/GLB loader.
#[derive(Debug)]
pub enum GltfError {
    ParseFailed(String),
    InvalidJson(String),
    MissingBuffer,
    MissingAccessor,
    InvalidGlbHeader,
    ChunkTooSmall,
    UnsupportedVersion(String),
    BufferDecodeFailed(String),
}

impl fmt::Display for GltfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GltfError::ParseFailed(s) => write!(f, "GLTF parse failed: {}", s),
            GltfError::InvalidJson(s) => write!(f, "Invalid JSON: {}", s),
            GltfError::MissingBuffer => write!(f, "Missing buffer"),
            GltfError::MissingAccessor => write!(f, "Missing accessor"),
            GltfError::InvalidGlbHeader => write!(f, "Invalid GLB header"),
            GltfError::ChunkTooSmall => write!(f, "GLB chunk too small"),
            GltfError::UnsupportedVersion(s) => write!(f, "Unsupported GLB version: {}", s),
            GltfError::BufferDecodeFailed(s) => write!(f, "Buffer decode failed: {}", s),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GltfError {}

// ---------------------------------------------------------------------------
// Minimal JSON parser
// ---------------------------------------------------------------------------

/// A minimal JSON value representation used for parsing GLTF JSON.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

#[allow(dead_code)]
impl JsonValue {
    pub fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            JsonValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        self.as_number().map(|n| n as u32)
    }

    pub fn as_usize(&self) -> Option<usize> {
        self.as_number().map(|n| n as usize)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.as_object().and_then(|o| o.get(key))
    }
}

fn skip_whitespace(s: &str) -> &str {
    s.trim_start()
}

/// Parse a single JSON value from the start of `s`, returning the value and the
/// remaining unparsed slice.
pub fn parse_json_value(s: &str) -> Result<(JsonValue, &str), GltfError> {
    let s = skip_whitespace(s);
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return Err(GltfError::InvalidJson(
            "Unexpected end of input".to_string(),
        ));
    }
    match bytes[0] {
        b'n' => parse_null(s),
        b't' | b'f' => parse_bool(s),
        b'"' => {
            let (val, rest) = parse_string_raw(s)?;
            Ok((JsonValue::String(val), rest))
        }
        b'[' => parse_array(s),
        b'{' => parse_object(s),
        b'-' | b'0'..=b'9' => parse_number(s),
        _ => Err(GltfError::InvalidJson(format!(
            "Unexpected character: {}",
            bytes[0] as char
        ))),
    }
}

fn parse_null(s: &str) -> Result<(JsonValue, &str), GltfError> {
    if let Some(rest) = s.strip_prefix("null") {
        Ok((JsonValue::Null, rest))
    } else {
        Err(GltfError::InvalidJson("Expected `null`".to_string()))
    }
}

fn parse_bool(s: &str) -> Result<(JsonValue, &str), GltfError> {
    if let Some(rest) = s.strip_prefix("true") {
        Ok((JsonValue::Bool(true), rest))
    } else if let Some(rest) = s.strip_prefix("false") {
        Ok((JsonValue::Bool(false), rest))
    } else {
        Err(GltfError::InvalidJson(
            "Expected `true` or `false`".to_string(),
        ))
    }
}

fn parse_number(s: &str) -> Result<(JsonValue, &str), GltfError> {
    let bytes = s.as_bytes();
    let mut i = 0;
    if i < bytes.len() && bytes[i] == b'-' {
        i += 1;
    }
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_digit() || c == b'.' || c == b'e' || c == b'E' || c == b'+' || c == b'-' {
            i += 1;
        } else {
            break;
        }
    }
    let num_str = &s[..i];
    num_str
        .parse::<f64>()
        .map(|n| (JsonValue::Number(n), &s[i..]))
        .map_err(|_| GltfError::InvalidJson(format!("Invalid number: {}", num_str)))
}

fn parse_string_raw(s: &str) -> Result<(String, &str), GltfError> {
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes[0] != b'"' {
        return Err(GltfError::InvalidJson("Expected string".to_string()));
    }
    let mut result = String::new();
    let mut chars = s.char_indices();
    chars.next(); // consume opening quote

    loop {
        let (i, c) = chars
            .next()
            .ok_or_else(|| GltfError::InvalidJson("Unterminated string".to_string()))?;
        if c == '"' {
            return Ok((result, &s[i + c.len_utf8()..]));
        } else if c == '\\' {
            let (_, escaped) = chars
                .next()
                .ok_or_else(|| GltfError::InvalidJson("Unterminated escape".to_string()))?;
            match escaped {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                '/' => result.push('/'),
                'n' => result.push('\n'),
                't' => result.push('\t'),
                'r' => result.push('\r'),
                'b' => result.push('\u{0008}'),
                'f' => result.push('\u{000C}'),
                'u' => {
                    let mut code = 0u32;
                    for _ in 0..4 {
                        let (_, h) = chars.next().ok_or_else(|| {
                            GltfError::InvalidJson("Invalid unicode escape".to_string())
                        })?;
                        let d = h.to_digit(16).ok_or_else(|| {
                            GltfError::InvalidJson("Invalid unicode escape".to_string())
                        })?;
                        code = code * 16 + d;
                    }
                    if let Some(ch) = char::from_u32(code) {
                        result.push(ch);
                    }
                }
                _ => {
                    return Err(GltfError::InvalidJson(format!(
                        "Invalid escape: \\{}",
                        escaped
                    )))
                }
            }
        } else {
            result.push(c);
        }
    }
}

fn parse_array(s: &str) -> Result<(JsonValue, &str), GltfError> {
    let mut s = skip_whitespace(&s[1..]); // consume '['
    let mut arr = Vec::new();

    s = skip_whitespace(s);
    if let Some(rest) = s.strip_prefix(']') {
        return Ok((JsonValue::Array(arr), rest));
    }

    loop {
        let (val, rest) = parse_json_value(s)?;
        arr.push(val);
        s = skip_whitespace(rest);
        if let Some(rest) = s.strip_prefix(',') {
            s = skip_whitespace(rest);
        } else if let Some(rest) = s.strip_prefix(']') {
            return Ok((JsonValue::Array(arr), rest));
        } else {
            return Err(GltfError::InvalidJson(
                "Expected `,` or `]` in array".to_string(),
            ));
        }
    }
}

fn parse_object(s: &str) -> Result<(JsonValue, &str), GltfError> {
    let mut s = skip_whitespace(&s[1..]); // consume '{'
    let mut obj = BTreeMap::new();

    s = skip_whitespace(s);
    if let Some(rest) = s.strip_prefix('}') {
        return Ok((JsonValue::Object(obj), rest));
    }

    loop {
        s = skip_whitespace(s);
        let (key, rest) = parse_string_raw(s)?;
        s = skip_whitespace(rest);
        if let Some(rest) = s.strip_prefix(':') {
            s = skip_whitespace(rest);
        } else {
            return Err(GltfError::InvalidJson("Expected `:` in object".to_string()));
        }
        let (val, rest2) = parse_json_value(s)?;
        obj.insert(key, val);
        s = skip_whitespace(rest2);
        if let Some(rest) = s.strip_prefix(',') {
            s = skip_whitespace(rest);
        } else if let Some(rest) = s.strip_prefix('}') {
            return Ok((JsonValue::Object(obj), rest));
        } else {
            return Err(GltfError::InvalidJson(
                "Expected `,` or `}` in object".to_string(),
            ));
        }
    }
}

fn json_to_f32_array(val: &JsonValue) -> Vec<f32> {
    match val {
        JsonValue::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_number())
            .map(|n| n as f32)
            .collect(),
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// GLTF data structures
// ---------------------------------------------------------------------------

/// A GLTF accessor describing how to read typed data from a buffer view.
#[derive(Debug, Clone)]
pub struct GltfAccessor {
    pub buffer_view: usize,
    pub byte_offset: usize,
    pub component_type: u32,
    pub count: usize,
    pub type_str: String,
}

/// A GLTF buffer view: a slice of a buffer.
#[derive(Debug, Clone)]
pub struct GltfBufferView {
    pub buffer: usize,
    pub byte_offset: usize,
    pub byte_length: usize,
    pub stride: Option<usize>,
}

/// A GLTF buffer: a block of binary data.
#[derive(Debug, Clone)]
pub struct GltfBuffer {
    pub byte_length: usize,
    pub uri: Option<String>,
}

/// A GLTF primitive: a single drawable submesh.
#[derive(Debug, Clone)]
pub struct GltfPrimitive {
    pub attributes: BTreeMap<String, usize>,
    pub indices: Option<usize>,
    pub material: Option<usize>,
}

/// A GLTF mesh: a collection of primitives.
#[derive(Debug, Clone)]
pub struct GltfMesh {
    pub name: Option<String>,
    pub primitives: Vec<GltfPrimitive>,
}

/// A GLTF node in the scene graph.
#[derive(Debug, Clone)]
pub struct GltfNode {
    pub name: Option<String>,
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for GltfNode {
    fn default() -> Self {
        Self {
            name: None,
            mesh: None,
            children: Vec::new(),
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

/// A GLTF scene: a list of root node indices.
#[derive(Debug, Clone)]
pub struct GltfScene {
    pub nodes: Vec<usize>,
}

/// A parsed GLTF document.
#[derive(Debug, Clone)]
pub struct GltfDocument {
    pub accessors: Vec<GltfAccessor>,
    pub buffer_views: Vec<GltfBufferView>,
    pub buffers: Vec<GltfBuffer>,
    pub meshes: Vec<GltfMesh>,
    pub nodes: Vec<GltfNode>,
    pub scenes: Vec<GltfScene>,
    pub default_scene: Option<usize>,
}

impl GltfDocument {
    /// Parse a GLTF JSON document string.
    pub fn parse_json(json: &str) -> Result<Self, GltfError> {
        let (value, rest) = parse_json_value(json)?;
        let rest = skip_whitespace(rest);
        if !rest.is_empty() {
            return Err(GltfError::InvalidJson(
                "Trailing content after JSON".to_string(),
            ));
        }
        let obj = value
            .as_object()
            .ok_or_else(|| GltfError::ParseFailed("GLTF root is not an object".to_string()))?;

        // Accessors
        let mut accessors = Vec::new();
        if let Some(JsonValue::Array(arr)) = obj.get("accessors") {
            for v in arr {
                let o = v.as_object().ok_or_else(|| {
                    GltfError::ParseFailed("Accessor is not an object".to_string())
                })?;
                accessors.push(GltfAccessor {
                    buffer_view: o.get("bufferView").and_then(|v| v.as_usize()).unwrap_or(0),
                    byte_offset: o.get("byteOffset").and_then(|v| v.as_usize()).unwrap_or(0),
                    component_type: o.get("componentType").and_then(|v| v.as_u32()).unwrap_or(0),
                    count: o.get("count").and_then(|v| v.as_usize()).unwrap_or(0),
                    type_str: o
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("SCALAR")
                        .to_string(),
                });
            }
        }

        // Buffer views
        let mut buffer_views = Vec::new();
        if let Some(JsonValue::Array(arr)) = obj.get("bufferViews") {
            for v in arr {
                let o = v.as_object().ok_or_else(|| {
                    GltfError::ParseFailed("BufferView is not an object".to_string())
                })?;
                buffer_views.push(GltfBufferView {
                    buffer: o.get("buffer").and_then(|v| v.as_usize()).unwrap_or(0),
                    byte_offset: o.get("byteOffset").and_then(|v| v.as_usize()).unwrap_or(0),
                    byte_length: o.get("byteLength").and_then(|v| v.as_usize()).unwrap_or(0),
                    stride: o.get("byteStride").and_then(|v| v.as_usize()),
                });
            }
        }

        // Buffers
        let mut buffers = Vec::new();
        if let Some(JsonValue::Array(arr)) = obj.get("buffers") {
            for v in arr {
                let o = v
                    .as_object()
                    .ok_or_else(|| GltfError::ParseFailed("Buffer is not an object".to_string()))?;
                buffers.push(GltfBuffer {
                    byte_length: o.get("byteLength").and_then(|v| v.as_usize()).unwrap_or(0),
                    uri: o.get("uri").and_then(|v| v.as_str()).map(|s| s.to_string()),
                });
            }
        }

        // Meshes
        let mut meshes = Vec::new();
        if let Some(JsonValue::Array(arr)) = obj.get("meshes") {
            for v in arr {
                let o = v
                    .as_object()
                    .ok_or_else(|| GltfError::ParseFailed("Mesh is not an object".to_string()))?;
                let name = o
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let mut prims = Vec::new();
                if let Some(JsonValue::Array(parr)) = o.get("primitives") {
                    for pv in parr {
                        let po = pv.as_object().ok_or_else(|| {
                            GltfError::ParseFailed("Primitive is not an object".to_string())
                        })?;
                        let mut attributes = BTreeMap::new();
                        if let Some(JsonValue::Object(attrs)) = po.get("attributes") {
                            for (k, av) in attrs {
                                if let Some(idx) = av.as_usize() {
                                    attributes.insert(k.clone(), idx);
                                }
                            }
                        }
                        prims.push(GltfPrimitive {
                            attributes,
                            indices: po.get("indices").and_then(|v| v.as_usize()),
                            material: po.get("material").and_then(|v| v.as_usize()),
                        });
                    }
                }
                meshes.push(GltfMesh {
                    name,
                    primitives: prims,
                });
            }
        }

        // Nodes
        let mut nodes = Vec::new();
        if let Some(JsonValue::Array(arr)) = obj.get("nodes") {
            for v in arr {
                let o = v
                    .as_object()
                    .ok_or_else(|| GltfError::ParseFailed("Node is not an object".to_string()))?;
                let mut node = GltfNode::default();
                if let Some(n) = o.get("name").and_then(|v| v.as_str()) {
                    node.name = Some(n.to_string());
                }
                node.mesh = o.get("mesh").and_then(|v| v.as_usize());
                if let Some(JsonValue::Array(children)) = o.get("children") {
                    node.children = children.iter().filter_map(|v| v.as_usize()).collect();
                }
                if let Some(t) = o.get("translation") {
                    let a = json_to_f32_array(t);
                    node.translation = [
                        a.first().copied().unwrap_or(0.0),
                        a.get(1).copied().unwrap_or(0.0),
                        a.get(2).copied().unwrap_or(0.0),
                    ];
                }
                if let Some(r) = o.get("rotation") {
                    let a = json_to_f32_array(r);
                    node.rotation = [
                        a.first().copied().unwrap_or(0.0),
                        a.get(1).copied().unwrap_or(0.0),
                        a.get(2).copied().unwrap_or(0.0),
                        a.get(3).copied().unwrap_or(1.0),
                    ];
                }
                if let Some(sc) = o.get("scale") {
                    let a = json_to_f32_array(sc);
                    node.scale = [
                        a.first().copied().unwrap_or(1.0),
                        a.get(1).copied().unwrap_or(1.0),
                        a.get(2).copied().unwrap_or(1.0),
                    ];
                }
                nodes.push(node);
            }
        }

        // Scenes
        let mut scenes = Vec::new();
        if let Some(JsonValue::Array(arr)) = obj.get("scenes") {
            for v in arr {
                let o = v
                    .as_object()
                    .ok_or_else(|| GltfError::ParseFailed("Scene is not an object".to_string()))?;
                let mut scene_nodes = Vec::new();
                if let Some(JsonValue::Array(narr)) = o.get("nodes") {
                    scene_nodes = narr.iter().filter_map(|v| v.as_usize()).collect();
                }
                scenes.push(GltfScene { nodes: scene_nodes });
            }
        }

        let default_scene = obj.get("scene").and_then(|v| v.as_usize());

        Ok(Self {
            accessors,
            buffer_views,
            buffers,
            meshes,
            nodes,
            scenes,
            default_scene,
        })
    }
}

// ---------------------------------------------------------------------------
// GLB binary parser
// ---------------------------------------------------------------------------

const GLB_MAGIC: u32 = 0x46546C67; // "glTF"
const GLB_VERSION: u32 = 2;
const GLB_CHUNK_JSON: u32 = 0x4E4F534A; // "JSON"
const GLB_CHUNK_BIN: u32 = 0x004E4942; // "BIN\0"

/// A parsed GLB binary file.
#[derive(Debug, Clone)]
pub struct GlbFile {
    pub magic: u32,
    pub version: u32,
    pub length: u32,
    pub json_chunk: String,
    pub bin_chunk: Vec<u8>,
}

impl GlbFile {
    /// Parse a GLB binary file from raw bytes.
    pub fn parse(data: &[u8]) -> Result<Self, GltfError> {
        if data.len() < 12 {
            return Err(GltfError::InvalidGlbHeader);
        }
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != GLB_MAGIC {
            return Err(GltfError::InvalidGlbHeader);
        }
        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if version != GLB_VERSION {
            return Err(GltfError::UnsupportedVersion(version.to_string()));
        }
        let length = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);

        let mut offset = 12usize;
        let mut json_chunk = String::new();
        let mut bin_chunk = Vec::new();

        while offset < data.len() {
            if offset + 8 > data.len() {
                return Err(GltfError::ChunkTooSmall);
            }
            let chunk_length = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            let chunk_type = u32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);
            offset += 8;
            if offset + chunk_length > data.len() {
                return Err(GltfError::ChunkTooSmall);
            }
            let chunk_data = &data[offset..offset + chunk_length];
            offset += chunk_length;

            match chunk_type {
                GLB_CHUNK_JSON => {
                    json_chunk = core::str::from_utf8(chunk_data)
                        .map_err(|_| {
                            GltfError::InvalidJson("Invalid UTF-8 in JSON chunk".to_string())
                        })?
                        .to_string();
                }
                GLB_CHUNK_BIN => {
                    bin_chunk = chunk_data.to_vec();
                }
                _ => {
                    // Unknown chunk type: ignore.
                }
            }
        }

        Ok(Self {
            magic,
            version,
            length,
            json_chunk,
            bin_chunk,
        })
    }
}

// ---------------------------------------------------------------------------
// Accessor decoding helpers
// ---------------------------------------------------------------------------

/// Returns the byte size of a GLTF component type.
pub fn component_type_size(component_type: u32) -> usize {
    match component_type {
        5120 => 1, // BYTE
        5121 => 1, // UNSIGNED_BYTE
        5122 => 2, // SHORT
        5123 => 2, // UNSIGNED_SHORT
        5125 => 4, // UNSIGNED_INT
        5126 => 4, // FLOAT
        _ => 0,
    }
}

/// Returns the number of scalar components for a GLTF accessor type string.
pub fn type_component_count(type_str: &str) -> usize {
    match type_str {
        "SCALAR" => 1,
        "VEC2" => 2,
        "VEC3" => 3,
        "VEC4" => 4,
        "MAT2" => 4,
        "MAT3" => 9,
        "MAT4" => 16,
        _ => 0,
    }
}

fn read_component_f32(buf: &[u8], offset: usize, component_type: u32) -> Result<f32, GltfError> {
    let size = component_type_size(component_type);
    if offset + size > buf.len() {
        return Err(GltfError::BufferDecodeFailed(
            "Accessor out of bounds".to_string(),
        ));
    }
    Ok(match component_type {
        5126 => {
            let mut b = [0u8; 4];
            b.copy_from_slice(&buf[offset..offset + 4]);
            f32::from_le_bytes(b)
        }
        5120 => buf[offset] as i8 as f32,
        5121 => buf[offset] as f32,
        5122 => {
            let mut b = [0u8; 2];
            b.copy_from_slice(&buf[offset..offset + 2]);
            i16::from_le_bytes(b) as f32
        }
        5123 => {
            let mut b = [0u8; 2];
            b.copy_from_slice(&buf[offset..offset + 2]);
            u16::from_le_bytes(b) as f32
        }
        5125 => {
            let mut b = [0u8; 4];
            b.copy_from_slice(&buf[offset..offset + 4]);
            u32::from_le_bytes(b) as f32
        }
        _ => {
            return Err(GltfError::BufferDecodeFailed(format!(
                "Unknown component type: {}",
                component_type
            )))
        }
    })
}

fn read_component_u32(buf: &[u8], offset: usize, component_type: u32) -> Result<u32, GltfError> {
    let size = component_type_size(component_type);
    if offset + size > buf.len() {
        return Err(GltfError::BufferDecodeFailed(
            "Accessor out of bounds".to_string(),
        ));
    }
    Ok(match component_type {
        5120 => buf[offset] as i8 as u32,
        5121 => buf[offset] as u32,
        5122 => {
            let mut b = [0u8; 2];
            b.copy_from_slice(&buf[offset..offset + 2]);
            i16::from_le_bytes(b) as u32
        }
        5123 => {
            let mut b = [0u8; 2];
            b.copy_from_slice(&buf[offset..offset + 2]);
            u16::from_le_bytes(b) as u32
        }
        5125 => {
            let mut b = [0u8; 4];
            b.copy_from_slice(&buf[offset..offset + 4]);
            u32::from_le_bytes(b)
        }
        5126 => {
            let mut b = [0u8; 4];
            b.copy_from_slice(&buf[offset..offset + 4]);
            f32::from_le_bytes(b) as u32
        }
        _ => {
            return Err(GltfError::BufferDecodeFailed(format!(
                "Unknown component type: {}",
                component_type
            )))
        }
    })
}

/// Decode an accessor as a flat `Vec<f32>`.
pub fn decode_accessor_f32(
    accessor: &GltfAccessor,
    doc: &GltfDocument,
    buffers: &[&[u8]],
) -> Result<Vec<f32>, GltfError> {
    let buffer_view = doc
        .buffer_views
        .get(accessor.buffer_view)
        .ok_or(GltfError::MissingAccessor)?;
    let buffer = buffers
        .get(buffer_view.buffer)
        .ok_or(GltfError::MissingBuffer)?;

    let component_size = component_type_size(accessor.component_type);
    if component_size == 0 {
        return Err(GltfError::BufferDecodeFailed(format!(
            "Unknown component type: {}",
            accessor.component_type
        )));
    }
    let component_count = type_component_count(&accessor.type_str);
    if component_count == 0 {
        return Err(GltfError::BufferDecodeFailed(format!(
            "Unknown accessor type: {}",
            accessor.type_str
        )));
    }
    let stride = buffer_view
        .stride
        .unwrap_or(component_size * component_count);

    let total_values = accessor.count * component_count;
    let mut result = Vec::with_capacity(total_values);

    let base_offset = buffer_view.byte_offset + accessor.byte_offset;
    for i in 0..accessor.count {
        let element_offset = base_offset + i * stride;
        for j in 0..component_count {
            let value_offset = element_offset + j * component_size;
            result.push(read_component_f32(
                buffer,
                value_offset,
                accessor.component_type,
            )?);
        }
    }

    Ok(result)
}

/// Decode an accessor as a `Vec<u32>` (used for index data).
pub fn decode_accessor_u32(
    accessor: &GltfAccessor,
    doc: &GltfDocument,
    buffers: &[&[u8]],
) -> Result<Vec<u32>, GltfError> {
    let buffer_view = doc
        .buffer_views
        .get(accessor.buffer_view)
        .ok_or(GltfError::MissingAccessor)?;
    let buffer = buffers
        .get(buffer_view.buffer)
        .ok_or(GltfError::MissingBuffer)?;

    let component_size = component_type_size(accessor.component_type);
    if component_size == 0 {
        return Err(GltfError::BufferDecodeFailed(format!(
            "Unknown component type: {}",
            accessor.component_type
        )));
    }
    let stride = buffer_view.stride.unwrap_or(component_size);

    let mut result = Vec::with_capacity(accessor.count);
    let base_offset = buffer_view.byte_offset + accessor.byte_offset;
    for i in 0..accessor.count {
        let value_offset = base_offset + i * stride;
        result.push(read_component_u32(
            buffer,
            value_offset,
            accessor.component_type,
        )?);
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Main GLTF/GLB loader.
pub struct GltfLoader;

impl GltfLoader {
    /// Load meshes from a GLTF JSON document and a set of binary buffers.
    pub fn load_from_json(json: &str, buffers: &[&[u8]]) -> Result<Vec<Mesh3D>, GltfError> {
        let doc = GltfDocument::parse_json(json)?;
        let mut meshes = Vec::new();
        for mesh in &doc.meshes {
            for prim in &mesh.primitives {
                let m = Self::parse_gltf_primitive(prim, &doc, buffers)?;
                meshes.push(m);
            }
        }
        Ok(meshes)
    }

    /// Load meshes from a GLB binary file.
    pub fn load_from_glb(data: &[u8]) -> Result<Vec<Mesh3D>, GltfError> {
        let glb = GlbFile::parse(data)?;
        let buffers: Vec<&[u8]> = if glb.bin_chunk.is_empty() {
            Vec::new()
        } else {
            vec![glb.bin_chunk.as_slice()]
        };
        Self::load_from_json(&glb.json_chunk, &buffers)
    }

    /// Parse a single GLTF primitive into a `Mesh3D`.
    pub fn parse_gltf_primitive(
        prim: &GltfPrimitive,
        doc: &GltfDocument,
        buffers: &[&[u8]],
    ) -> Result<Mesh3D, GltfError> {
        let pos_accessor_idx = prim
            .attributes
            .get("POSITION")
            .ok_or_else(|| GltfError::ParseFailed("Missing POSITION attribute".to_string()))?;
        let pos_accessor = doc
            .accessors
            .get(*pos_accessor_idx)
            .ok_or(GltfError::MissingAccessor)?;

        let positions = decode_accessor_f32(pos_accessor, doc, buffers)?;
        let vertex_count = pos_accessor.count;

        let normals = if let Some(&idx) = prim.attributes.get("NORMAL") {
            let accessor = doc.accessors.get(idx).ok_or(GltfError::MissingAccessor)?;
            Some(decode_accessor_f32(accessor, doc, buffers)?)
        } else {
            None
        };

        let texcoords = if let Some(&idx) = prim.attributes.get("TEXCOORD_0") {
            let accessor = doc.accessors.get(idx).ok_or(GltfError::MissingAccessor)?;
            Some(decode_accessor_f32(accessor, doc, buffers)?)
        } else {
            None
        };

        let indices = if let Some(idx) = prim.indices {
            let accessor = doc.accessors.get(idx).ok_or(GltfError::MissingAccessor)?;
            decode_accessor_u32(accessor, doc, buffers)?
        } else {
            (0..vertex_count as u32).collect()
        };

        let mut vertices = Vec::with_capacity(vertex_count);
        for i in 0..vertex_count {
            let pos = Vec3::new(positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]);
            let normal = if let Some(ref n) = normals {
                Vec3::new(n[i * 3], n[i * 3 + 1], n[i * 3 + 2])
            } else {
                Vec3::ZERO
            };
            let texcoord = if let Some(ref t) = texcoords {
                Vec2::new(t[i * 2], t[i * 2 + 1])
            } else {
                Vec2::ZERO
            };
            vertices.push(Vertex::new(pos, normal, texcoord));
        }

        let mut primitive = Primitive::new(indices);
        primitive.material_index = prim.material;
        primitive.vertex_count = vertex_count;

        Ok(Mesh3D::with_primitives(vertices, vec![primitive]))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- JSON value parser tests ---

    #[test]
    fn test_json_parse_null() {
        let (val, rest) = parse_json_value("null").unwrap();
        assert!(matches!(val, JsonValue::Null));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_json_parse_bool_true() {
        let (val, rest) = parse_json_value("true").unwrap();
        assert_eq!(val.as_bool(), Some(true));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_json_parse_bool_false() {
        let (val, rest) = parse_json_value("false").unwrap();
        assert_eq!(val.as_bool(), Some(false));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_json_parse_number() {
        let (val, rest) = parse_json_value("3.5").unwrap();
        assert_eq!(val.as_number(), Some(3.5));
        assert!(rest.is_empty());

        let (val2, _) = parse_json_value("-42").unwrap();
        assert_eq!(val2.as_number(), Some(-42.0));

        let (val3, _) = parse_json_value("1e3").unwrap();
        assert_eq!(val3.as_number(), Some(1000.0));
    }

    #[test]
    fn test_json_parse_string() {
        let (val, rest) = parse_json_value("\"hello world\"").unwrap();
        assert_eq!(val.as_str(), Some("hello world"));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_json_parse_string_escapes() {
        let (val, _) = parse_json_value("\"line1\\nline2\\ttab\"").unwrap();
        assert_eq!(val.as_str(), Some("line1\nline2\ttab"));

        let (val2, _) = parse_json_value("\"quote: \\\"q\\\"\"").unwrap();
        assert_eq!(val2.as_str(), Some("quote: \"q\""));
    }

    #[test]
    fn test_json_parse_array() {
        let (val, rest) = parse_json_value("[1, 2, 3]").unwrap();
        let arr = val.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_number(), Some(1.0));
        assert_eq!(arr[2].as_number(), Some(3.0));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_json_parse_array_empty() {
        let (val, _) = parse_json_value("[]").unwrap();
        assert!(val.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_json_parse_object() {
        let (val, rest) = parse_json_value("{\"a\": 1, \"b\": \"x\"}").unwrap();
        let obj = val.as_object().unwrap();
        assert_eq!(obj.get("a").unwrap().as_number(), Some(1.0));
        assert_eq!(obj.get("b").unwrap().as_str(), Some("x"));
        assert!(rest.is_empty());
    }

    #[test]
    fn test_json_parse_nested() {
        let (val, _) = parse_json_value("{\"outer\": {\"inner\": [1, 2]}}").unwrap();
        let outer = val.get("outer").unwrap().as_object().unwrap();
        let inner = outer.get("inner").unwrap().as_array().unwrap();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[1].as_number(), Some(2.0));
    }

    #[test]
    fn test_json_parse_with_whitespace() {
        let (val, _) = parse_json_value("  {  \"k\"  :  7  }  ").unwrap();
        assert_eq!(val.get("k").unwrap().as_number(), Some(7.0));
    }

    // --- GLTF document parsing tests ---

    #[test]
    fn test_gltf_document_empty() {
        let doc = GltfDocument::parse_json("{}").unwrap();
        assert!(doc.accessors.is_empty());
        assert!(doc.buffer_views.is_empty());
        assert!(doc.buffers.is_empty());
        assert!(doc.meshes.is_empty());
        assert!(doc.nodes.is_empty());
        assert!(doc.scenes.is_empty());
        assert_eq!(doc.default_scene, None);
    }

    #[test]
    fn test_gltf_document_with_accessors() {
        let json = r#"{
            "accessors": [
                {"bufferView": 0, "byteOffset": 0, "componentType": 5126, "count": 3, "type": "VEC3"},
                {"bufferView": 1, "componentType": 5123, "count": 3, "type": "SCALAR"}
            ]
        }"#;
        let doc = GltfDocument::parse_json(json).unwrap();
        assert_eq!(doc.accessors.len(), 2);
        assert_eq!(doc.accessors[0].buffer_view, 0);
        assert_eq!(doc.accessors[0].component_type, 5126);
        assert_eq!(doc.accessors[0].count, 3);
        assert_eq!(doc.accessors[0].type_str, "VEC3");
        assert_eq!(doc.accessors[1].byte_offset, 0);
        assert_eq!(doc.accessors[1].type_str, "SCALAR");
    }

    #[test]
    fn test_gltf_document_with_buffer_views() {
        let json = r#"{
            "bufferViews": [
                {"buffer": 0, "byteOffset": 0, "byteLength": 36, "byteStride": 12},
                {"buffer": 0, "byteOffset": 36, "byteLength": 6}
            ]
        }"#;
        let doc = GltfDocument::parse_json(json).unwrap();
        assert_eq!(doc.buffer_views.len(), 2);
        assert_eq!(doc.buffer_views[0].stride, Some(12));
        assert_eq!(doc.buffer_views[1].stride, None);
        assert_eq!(doc.buffer_views[1].byte_offset, 36);
    }

    #[test]
    fn test_gltf_document_with_meshes() {
        let json = r#"{
            "meshes": [
                {"name": "Triangle", "primitives": [
                    {"attributes": {"POSITION": 0, "NORMAL": 1}, "indices": 2, "material": 0}
                ]}
            ]
        }"#;
        let doc = GltfDocument::parse_json(json).unwrap();
        assert_eq!(doc.meshes.len(), 1);
        assert_eq!(doc.meshes[0].name, Some("Triangle".to_string()));
        assert_eq!(doc.meshes[0].primitives.len(), 1);
        let prim = &doc.meshes[0].primitives[0];
        assert_eq!(prim.attributes.get("POSITION"), Some(&0));
        assert_eq!(prim.attributes.get("NORMAL"), Some(&1));
        assert_eq!(prim.indices, Some(2));
        assert_eq!(prim.material, Some(0));
    }

    #[test]
    fn test_gltf_document_with_nodes() {
        let json = r#"{
            "nodes": [
                {"name": "Root", "mesh": 0, "children": [1, 2],
                 "translation": [1.0, 2.0, 3.0], "rotation": [0.0, 0.0, 0.0, 1.0],
                 "scale": [2.0, 2.0, 2.0]},
                {"mesh": 1}
            ],
            "scenes": [{"nodes": [0]}],
            "scene": 0
        }"#;
        let doc = GltfDocument::parse_json(json).unwrap();
        assert_eq!(doc.nodes.len(), 2);
        assert_eq!(doc.nodes[0].name, Some("Root".to_string()));
        assert_eq!(doc.nodes[0].mesh, Some(0));
        assert_eq!(doc.nodes[0].children, vec![1, 2]);
        assert_eq!(doc.nodes[0].translation, [1.0, 2.0, 3.0]);
        assert_eq!(doc.nodes[0].rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(doc.nodes[0].scale, [2.0, 2.0, 2.0]);
        assert_eq!(doc.nodes[1].mesh, Some(1));
        // Default rotation/scale when omitted
        assert_eq!(doc.nodes[1].rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(doc.nodes[1].scale, [1.0, 1.0, 1.0]);
        assert_eq!(doc.scenes.len(), 1);
        assert_eq!(doc.scenes[0].nodes, vec![0]);
        assert_eq!(doc.default_scene, Some(0));
    }

    // --- GLB parsing tests ---

    fn build_glb(json: &str, bin: &[u8]) -> Vec<u8> {
        let mut json_padded = json.as_bytes().to_vec();
        while json_padded.len() % 4 != 0 {
            json_padded.push(b' ');
        }
        let mut bin_padded = bin.to_vec();
        while bin_padded.len() % 4 != 0 {
            bin_padded.push(0);
        }
        let total_length = 12 + 8 + json_padded.len() + 8 + bin_padded.len();
        let mut data = Vec::new();
        data.extend_from_slice(&0x46546C67u32.to_le_bytes());
        data.extend_from_slice(&2u32.to_le_bytes());
        data.extend_from_slice(&(total_length as u32).to_le_bytes());
        data.extend_from_slice(&(json_padded.len() as u32).to_le_bytes());
        data.extend_from_slice(&0x4E4F534Au32.to_le_bytes());
        data.extend_from_slice(&json_padded);
        data.extend_from_slice(&(bin_padded.len() as u32).to_le_bytes());
        data.extend_from_slice(&0x004E4942u32.to_le_bytes());
        data.extend_from_slice(&bin_padded);
        data
    }

    #[test]
    fn test_glb_valid_header() {
        let json = r#"{"asset": {"version": "2.0"}}"#;
        let data = build_glb(json, &[]);
        let glb = GlbFile::parse(&data).unwrap();
        assert_eq!(glb.magic, 0x46546C67);
        assert_eq!(glb.version, 2);
        assert!(!glb.json_chunk.is_empty());
        assert!(glb.bin_chunk.is_empty());
    }

    #[test]
    fn test_glb_invalid_magic() {
        let mut data = vec![0u8; 20];
        // wrong magic
        data[0..4].copy_from_slice(&0x12345678u32.to_le_bytes());
        data[4..8].copy_from_slice(&2u32.to_le_bytes());
        data[8..12].copy_from_slice(&20u32.to_le_bytes());
        let err = GlbFile::parse(&data).unwrap_err();
        assert!(matches!(err, GltfError::InvalidGlbHeader));
    }

    #[test]
    fn test_glb_unsupported_version() {
        let mut data = vec![0u8; 20];
        data[0..4].copy_from_slice(&0x46546C67u32.to_le_bytes());
        data[4..8].copy_from_slice(&1u32.to_le_bytes()); // version 1
        data[8..12].copy_from_slice(&20u32.to_le_bytes());
        let err = GlbFile::parse(&data).unwrap_err();
        assert!(matches!(err, GltfError::UnsupportedVersion(_)));
    }

    #[test]
    fn test_glb_too_small_for_header() {
        let data = vec![0u8; 5];
        let err = GlbFile::parse(&data).unwrap_err();
        assert!(matches!(err, GltfError::InvalidGlbHeader));
    }

    #[test]
    fn test_glb_chunk_extraction() {
        let json = r#"{"asset": {"version": "2.0"}}"#;
        let bin = [0xDE, 0xAD, 0xBE, 0xEF];
        let data = build_glb(json, &bin);
        let glb = GlbFile::parse(&data).unwrap();
        assert!(glb.json_chunk.contains("asset"));
        assert_eq!(glb.bin_chunk, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    // --- Accessor decoding tests ---

    #[test]
    fn test_decode_accessor_f32() {
        // 3 VEC3 floats = 9 floats
        let positions: [f32; 9] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let mut buffer = Vec::new();
        for p in &positions {
            buffer.extend_from_slice(&p.to_le_bytes());
        }

        let doc = GltfDocument {
            accessors: vec![GltfAccessor {
                buffer_view: 0,
                byte_offset: 0,
                component_type: 5126,
                count: 3,
                type_str: "VEC3".to_string(),
            }],
            buffer_views: vec![GltfBufferView {
                buffer: 0,
                byte_offset: 0,
                byte_length: buffer.len(),
                stride: None,
            }],
            buffers: vec![GltfBuffer {
                byte_length: buffer.len(),
                uri: None,
            }],
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
            default_scene: None,
        };

        let buffers: Vec<&[u8]> = vec![buffer.as_slice()];
        let result = decode_accessor_f32(&doc.accessors[0], &doc, &buffers).unwrap();
        assert_eq!(result.len(), 9);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[3], 1.0);
        assert_eq!(result[7], 1.0);
    }

    #[test]
    fn test_decode_accessor_u32() {
        // 3 uint indices
        let indices: [u32; 3] = [0, 1, 2];
        let mut buffer = Vec::new();
        for i in &indices {
            buffer.extend_from_slice(&i.to_le_bytes());
        }

        let doc = GltfDocument {
            accessors: vec![GltfAccessor {
                buffer_view: 0,
                byte_offset: 0,
                component_type: 5125,
                count: 3,
                type_str: "SCALAR".to_string(),
            }],
            buffer_views: vec![GltfBufferView {
                buffer: 0,
                byte_offset: 0,
                byte_length: buffer.len(),
                stride: None,
            }],
            buffers: vec![GltfBuffer {
                byte_length: buffer.len(),
                uri: None,
            }],
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
            default_scene: None,
        };

        let buffers: Vec<&[u8]> = vec![buffer.as_slice()];
        let result = decode_accessor_u32(&doc.accessors[0], &doc, &buffers).unwrap();
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_decode_accessor_u16_indices() {
        // 3 ushort indices
        let indices: [u16; 3] = [10, 20, 30];
        let mut buffer = Vec::new();
        for i in &indices {
            buffer.extend_from_slice(&i.to_le_bytes());
        }

        let doc = GltfDocument {
            accessors: vec![GltfAccessor {
                buffer_view: 0,
                byte_offset: 0,
                component_type: 5123,
                count: 3,
                type_str: "SCALAR".to_string(),
            }],
            buffer_views: vec![GltfBufferView {
                buffer: 0,
                byte_offset: 0,
                byte_length: buffer.len(),
                stride: None,
            }],
            buffers: vec![GltfBuffer {
                byte_length: buffer.len(),
                uri: None,
            }],
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
            default_scene: None,
        };

        let buffers: Vec<&[u8]> = vec![buffer.as_slice()];
        let result = decode_accessor_u32(&doc.accessors[0], &doc, &buffers).unwrap();
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_decode_accessor_missing_buffer() {
        let doc = GltfDocument {
            accessors: vec![GltfAccessor {
                buffer_view: 0,
                byte_offset: 0,
                component_type: 5126,
                count: 3,
                type_str: "VEC3".to_string(),
            }],
            buffer_views: vec![GltfBufferView {
                buffer: 0,
                byte_offset: 0,
                byte_length: 36,
                stride: None,
            }],
            buffers: vec![],
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
            default_scene: None,
        };

        let buffers: Vec<&[u8]> = vec![];
        let err = decode_accessor_f32(&doc.accessors[0], &doc, &buffers).unwrap_err();
        assert!(matches!(err, GltfError::MissingBuffer));
    }

    // --- Component type / type mapping tests ---

    #[test]
    fn test_component_type_size() {
        assert_eq!(component_type_size(5120), 1);
        assert_eq!(component_type_size(5121), 1);
        assert_eq!(component_type_size(5122), 2);
        assert_eq!(component_type_size(5123), 2);
        assert_eq!(component_type_size(5125), 4);
        assert_eq!(component_type_size(5126), 4);
        assert_eq!(component_type_size(9999), 0);
    }

    #[test]
    fn test_type_component_count() {
        assert_eq!(type_component_count("SCALAR"), 1);
        assert_eq!(type_component_count("VEC2"), 2);
        assert_eq!(type_component_count("VEC3"), 3);
        assert_eq!(type_component_count("VEC4"), 4);
        assert_eq!(type_component_count("MAT4"), 16);
        assert_eq!(type_component_count("UNKNOWN"), 0);
    }

    // --- GltfLoader tests ---

    #[test]
    fn test_loader_load_from_json_triangle() {
        // Triangle: 3 positions (VEC3 float), 3 indices (uint)
        let positions: [f32; 9] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let indices: [u32; 3] = [0, 1, 2];
        let mut buffer = Vec::new();
        for p in &positions {
            buffer.extend_from_slice(&p.to_le_bytes());
        }
        let pos_len = buffer.len();
        for i in &indices {
            buffer.extend_from_slice(&i.to_le_bytes());
        }

        let json = format!(
            r#"{{
                "asset": {{"version": "2.0"}},
                "accessors": [
                    {{"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3"}},
                    {{"bufferView": 1, "componentType": 5125, "count": 3, "type": "SCALAR"}}
                ],
                "bufferViews": [
                    {{"buffer": 0, "byteLength": {}}},
                    {{"buffer": 0, "byteOffset": {}, "byteLength": 12}}
                ],
                "buffers": [{{"byteLength": {}}}],
                "meshes": [{{"primitives": [{{"attributes": {{"POSITION": 0}}, "indices": 1}}]}}],
                "nodes": [{{"mesh": 0}}],
                "scenes": [{{"nodes": [0]}}],
                "scene": 0
            }}"#,
            pos_len,
            pos_len,
            buffer.len()
        );

        let buffers: Vec<&[u8]> = vec![buffer.as_slice()];
        let meshes = GltfLoader::load_from_json(&json, &buffers).unwrap();
        assert_eq!(meshes.len(), 1);
        let mesh = &meshes[0];
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.primitive_count(), 1);
        let verts = mesh.vertices();
        assert_eq!(verts[0].position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(verts[1].position, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(verts[2].position, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_loader_load_from_json_with_normals_and_uv() {
        let positions: [f32; 9] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let normals: [f32; 9] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let texcoords: [f32; 6] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let mut buffer = Vec::new();
        for p in &positions {
            buffer.extend_from_slice(&p.to_le_bytes());
        }
        for n in &normals {
            buffer.extend_from_slice(&n.to_le_bytes());
        }
        for t in &texcoords {
            buffer.extend_from_slice(&t.to_le_bytes());
        }

        let pos_len = positions.len() * 4;
        let norm_len = normals.len() * 4;
        let uv_len = texcoords.len() * 4;

        let json = format!(
            r#"{{
                "asset": {{"version": "2.0"}},
                "accessors": [
                    {{"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3"}},
                    {{"bufferView": 1, "componentType": 5126, "count": 3, "type": "VEC3"}},
                    {{"bufferView": 2, "componentType": 5126, "count": 3, "type": "VEC2"}}
                ],
                "bufferViews": [
                    {{"buffer": 0, "byteLength": {}}},
                    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
                    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
                ],
                "buffers": [{{"byteLength": {}}}],
                "meshes": [{{"primitives": [{{"attributes": {{"POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2}}}}]}}]
            }}"#,
            pos_len,
            pos_len,
            norm_len,
            pos_len + norm_len,
            uv_len,
            buffer.len()
        );

        let buffers: Vec<&[u8]> = vec![buffer.as_slice()];
        let meshes = GltfLoader::load_from_json(&json, &buffers).unwrap();
        let mesh = &meshes[0];
        assert_eq!(mesh.vertex_count(), 3);
        assert!(mesh.has_normals());
        assert!(mesh.has_uv());
        // No indices accessor -> sequential indices generated
        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.vertices()[0].normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(mesh.vertices()[1].texcoord, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_loader_load_from_glb() {
        let positions: [f32; 9] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let indices: [u32; 3] = [0, 1, 2];
        let mut bin = Vec::new();
        for p in &positions {
            bin.extend_from_slice(&p.to_le_bytes());
        }
        let pos_len = bin.len();
        for i in &indices {
            bin.extend_from_slice(&i.to_le_bytes());
        }

        let json = format!(
            r#"{{
                "asset": {{"version": "2.0"}},
                "accessors": [
                    {{"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3"}},
                    {{"bufferView": 1, "componentType": 5125, "count": 3, "type": "SCALAR"}}
                ],
                "bufferViews": [
                    {{"buffer": 0, "byteLength": {}}},
                    {{"buffer": 0, "byteOffset": {}, "byteLength": 12}}
                ],
                "buffers": [{{"byteLength": {}}}],
                "meshes": [{{"primitives": [{{"attributes": {{"POSITION": 0}}, "indices": 1}}]}}]
            }}"#,
            pos_len,
            pos_len,
            bin.len()
        );

        let glb_data = build_glb(&json, &bin);
        let meshes = GltfLoader::load_from_glb(&glb_data).unwrap();
        assert_eq!(meshes.len(), 1);
        let mesh = &meshes[0];
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
        assert_eq!(mesh.vertices()[1].position, Vec3::new(1.0, 0.0, 0.0));
    }

    // --- Error case tests ---

    #[test]
    fn test_error_invalid_json() {
        let err = GltfDocument::parse_json("{invalid}").unwrap_err();
        assert!(matches!(err, GltfError::InvalidJson(_)));
    }

    #[test]
    fn test_error_root_not_object() {
        let err = GltfDocument::parse_json("[1, 2, 3]").unwrap_err();
        assert!(matches!(err, GltfError::ParseFailed(_)));
    }

    #[test]
    fn test_error_missing_position_attribute() {
        let json = r#"{
            "asset": {"version": "2.0"},
            "meshes": [{"primitives": [{"attributes": {"NORMAL": 0}}]}]
        }"#;
        let buffers: Vec<&[u8]> = vec![];
        let err = GltfLoader::load_from_json(json, &buffers).unwrap_err();
        assert!(matches!(err, GltfError::ParseFailed(_)));
    }

    #[test]
    fn test_error_missing_accessor_index() {
        let json = r#"{
            "asset": {"version": "2.0"},
            "meshes": [{"primitives": [{"attributes": {"POSITION": 5}}]}]
        }"#;
        let buffers: Vec<&[u8]> = vec![&[]];
        let err = GltfLoader::load_from_json(json, &buffers).unwrap_err();
        assert!(matches!(err, GltfError::MissingAccessor));
    }

    #[test]
    fn test_error_glb_invalid_header() {
        let data = [0u8; 12];
        let err = GlbFile::parse(&data).unwrap_err();
        assert!(matches!(err, GltfError::InvalidGlbHeader));
    }

    #[test]
    fn test_error_display_messages() {
        assert_eq!(format!("{}", GltfError::MissingBuffer), "Missing buffer");
        assert_eq!(
            format!("{}", GltfError::InvalidGlbHeader),
            "Invalid GLB header"
        );
        assert_eq!(
            format!("{}", GltfError::UnsupportedVersion("1".to_string())),
            "Unsupported GLB version: 1"
        );
        assert_eq!(
            format!("{}", GltfError::ChunkTooSmall),
            "GLB chunk too small"
        );
    }

    #[test]
    fn test_gltf_node_default() {
        let node = GltfNode::default();
        assert_eq!(node.translation, [0.0, 0.0, 0.0]);
        assert_eq!(node.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(node.scale, [1.0, 1.0, 1.0]);
        assert!(node.children.is_empty());
        assert_eq!(node.mesh, None);
    }

    #[test]
    fn test_decode_accessor_with_byte_offset_and_stride() {
        // 2 VEC3 floats with a stride of 16 (extra padding per vertex)
        let positions: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut buffer = Vec::new();
        // Prepend 8 bytes of padding to test byte_offset
        buffer.extend_from_slice(&[0u8; 8]);
        // Vertex 0: 3 floats + 1 float padding (stride 16)
        buffer.extend_from_slice(&positions[0].to_le_bytes());
        buffer.extend_from_slice(&positions[1].to_le_bytes());
        buffer.extend_from_slice(&positions[2].to_le_bytes());
        buffer.extend_from_slice(&99.0f32.to_le_bytes()); // padding
                                                          // Vertex 1: 3 floats + 1 float padding
        buffer.extend_from_slice(&positions[3].to_le_bytes());
        buffer.extend_from_slice(&positions[4].to_le_bytes());
        buffer.extend_from_slice(&positions[5].to_le_bytes());
        buffer.extend_from_slice(&99.0f32.to_le_bytes()); // padding

        let doc = GltfDocument {
            accessors: vec![GltfAccessor {
                buffer_view: 0,
                byte_offset: 0,
                component_type: 5126,
                count: 2,
                type_str: "VEC3".to_string(),
            }],
            buffer_views: vec![GltfBufferView {
                buffer: 0,
                byte_offset: 8, // skip leading padding
                byte_length: buffer.len() - 8,
                stride: Some(16),
            }],
            buffers: vec![GltfBuffer {
                byte_length: buffer.len(),
                uri: None,
            }],
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
            default_scene: None,
        };

        let buffers: Vec<&[u8]> = vec![buffer.as_slice()];
        let result = decode_accessor_f32(&doc.accessors[0], &doc, &buffers).unwrap();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 1.0);
        assert_eq!(result[2], 3.0);
        assert_eq!(result[3], 4.0);
        assert_eq!(result[5], 6.0);
    }
}
