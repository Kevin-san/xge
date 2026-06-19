//! Shader module system for 3D rendering
//!
//! Provides shader compilation, linking, uniform binding, and vertex attribute management.
//! Supports GLSL/WGSL source formats with CPU-side validation and uniform tracking.

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt;
#[cfg(feature = "std")]
use engine_math::{Mat4, Vec2, Vec3, Vec4};

/// Shader stage in the rendering pipeline.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
    Compute,
    TessellationControl,
    TessellationEvaluation,
}

/// Shader source format.
#[derive(Clone, Debug)]
pub enum ShaderSource {
    /// GLSL source text.
    Glsl(String),
    /// WGSL source text.
    Wgsl(String),
    /// SPIR-V bytecode as 32-bit words.
    SpirV(Vec<u32>),
}

/// Shader compilation or linking error.
#[derive(Debug)]
pub enum ShaderError {
    /// Shader compilation failed.
    CompilationFailed(String),
    /// Shader program linking failed.
    LinkingFailed(String),
    /// The provided shader source was invalid (e.g. empty).
    InvalidSource,
    /// A shader stage did not match the expected pipeline stage.
    StageMismatch,
    /// Shader validation failed.
    ValidationError(String),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderError::CompilationFailed(msg) => {
                write!(f, "shader compilation failed: {}", msg)
            }
            ShaderError::LinkingFailed(msg) => write!(f, "shader linking failed: {}", msg),
            ShaderError::InvalidSource => write!(f, "invalid shader source"),
            ShaderError::StageMismatch => write!(f, "shader stage mismatch"),
            ShaderError::ValidationError(msg) => write!(f, "shader validation error: {}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ShaderError {}

/// Type of a uniform variable in a shader.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UniformType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat4,
    Sampler2D,
    SamplerCube,
    Int,
    Bool,
}

impl UniformType {
    /// Returns the byte size of a single element of this uniform type.
    pub fn byte_size(&self) -> usize {
        match self {
            UniformType::Float | UniformType::Int | UniformType::Bool | UniformType::Sampler2D
            | UniformType::SamplerCube => 4,
            UniformType::Vec2 => 8,
            UniformType::Vec3 => 12,
            UniformType::Vec4 => 16,
            UniformType::Mat4 => 64,
        }
    }
}

/// Metadata describing a single uniform variable.
#[derive(Clone, Debug)]
pub struct UniformInfo {
    pub name: String,
    pub uniform_type: UniformType,
    pub location: i32,
    pub byte_size: usize,
    pub array_size: usize,
}

impl UniformInfo {
    /// Creates a new `UniformInfo` with `array_size` defaulting to 1 and
    /// `byte_size` derived from the uniform type.
    pub fn new(name: String, uniform_type: UniformType, location: i32) -> Self {
        Self {
            byte_size: uniform_type.byte_size(),
            name,
            uniform_type,
            location,
            array_size: 1,
        }
    }

    /// Builder-style method to set the array size.
    pub fn with_array_size(mut self, array_size: usize) -> Self {
        self.array_size = array_size;
        self
    }
}

/// Format of a single vertex attribute.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VertexAttributeFormat {
    Float,
    Vec2,
    Vec3,
    Vec4,
}

impl VertexAttributeFormat {
    /// Returns the number of float components in this attribute format.
    pub fn component_count(&self) -> usize {
        match self {
            VertexAttributeFormat::Float => 1,
            VertexAttributeFormat::Vec2 => 2,
            VertexAttributeFormat::Vec3 => 3,
            VertexAttributeFormat::Vec4 => 4,
        }
    }

    /// Returns the total byte size of this attribute format.
    pub fn byte_size(&self) -> usize {
        self.component_count() * core::mem::size_of::<f32>()
    }
}

/// Metadata describing a single vertex attribute.
#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub name: String,
    pub location: u32,
    pub format: VertexAttributeFormat,
    pub size: usize,
}

impl VertexAttribute {
    /// Creates a new `VertexAttribute` with `size` derived from the format.
    pub fn new(name: String, location: u32, format: VertexAttributeFormat) -> Self {
        Self {
            size: format.byte_size(),
            name,
            location,
            format,
        }
    }
}

/// Computes a simple FNV-1a 64-bit hash of the given source string.
fn hash_source(source: &str) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET_BASIS;
    for byte in source.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Computes an FNV-1a 64-bit hash over a slice of bytes.
fn hash_bytes(bytes: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET_BASIS;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Computes an FNV-1a 64-bit hash over SPIR-V words.
fn hash_spirv(words: &[u32]) -> u64 {
    let mut bytes: Vec<u8> = Vec::with_capacity(words.len() * 4);
    for &word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    hash_bytes(&bytes)
}

/// A compiled shader stage module.
///
/// Since there is no real GPU backend, compilation only validates that the
/// source is non-empty and records a hash of the source for caching purposes.
pub struct ShaderModule {
    stage: ShaderStage,
    source: ShaderSource,
    uniforms: Vec<UniformInfo>,
    attributes: Vec<VertexAttribute>,
    is_compiled: bool,
    source_hash: u64,
}

impl ShaderModule {
    /// Creates a new, uncompiled shader module for the given stage and source.
    pub fn new(stage: ShaderStage, source: ShaderSource) -> Self {
        let source_hash = match &source {
            ShaderSource::Glsl(s) | ShaderSource::Wgsl(s) => hash_source(s),
            ShaderSource::SpirV(words) => hash_spirv(words),
        };
        Self {
            stage,
            source,
            uniforms: Vec::new(),
            attributes: Vec::new(),
            is_compiled: false,
            source_hash,
        }
    }

    /// Validates the source and marks the module as compiled.
    ///
    /// Returns `InvalidSource` if the source is empty.
    pub fn compile(&mut self) -> Result<(), ShaderError> {
        let non_empty = match &self.source {
            ShaderSource::Glsl(s) | ShaderSource::Wgsl(s) => !s.is_empty(),
            ShaderSource::SpirV(words) => !words.is_empty(),
        };
        if !non_empty {
            return Err(ShaderError::InvalidSource);
        }
        self.is_compiled = true;
        Ok(())
    }

    /// Registers a uniform with this module.
    pub fn add_uniform(&mut self, info: UniformInfo) {
        self.uniforms.push(info);
    }

    /// Registers a vertex attribute with this module.
    pub fn add_attribute(&mut self, attr: VertexAttribute) {
        self.attributes.push(attr);
    }

    /// Finds a uniform by name.
    pub fn find_uniform(&self, name: &str) -> Option<&UniformInfo> {
        self.uniforms.iter().find(|u| u.name == name)
    }

    /// Returns whether this module has been successfully compiled.
    pub fn is_compiled(&self) -> bool {
        self.is_compiled
    }

    /// Returns the shader stage of this module.
    pub fn stage(&self) -> ShaderStage {
        self.stage
    }

    /// Returns the hash of the shader source.
    pub fn source_hash(&self) -> u64 {
        self.source_hash
    }

    /// Returns the uniforms registered with this module.
    pub fn uniforms(&self) -> &[UniformInfo] {
        &self.uniforms
    }

    /// Returns the vertex attributes registered with this module.
    pub fn attributes(&self) -> &[VertexAttribute] {
        &self.attributes
    }
}

/// A linked shader program composed of a vertex and fragment stage.
pub struct ShaderProgram {
    vertex: Option<ShaderModule>,
    fragment: Option<ShaderModule>,
    uniforms: Vec<UniformInfo>,
    attributes: Vec<VertexAttribute>,
    is_linked: bool,
    program_hash: u64,
}

impl ShaderProgram {
    /// Creates a new, empty shader program.
    pub fn new() -> Self {
        Self {
            vertex: None,
            fragment: None,
            uniforms: Vec::new(),
            attributes: Vec::new(),
            is_linked: false,
            program_hash: 0,
        }
    }

    /// Attaches a vertex shader module.
    pub fn attach_vertex(&mut self, module: ShaderModule) {
        self.vertex = Some(module);
    }

    /// Attaches a fragment shader module.
    pub fn attach_fragment(&mut self, module: ShaderModule) {
        self.fragment = Some(module);
    }

    /// Links the attached vertex and fragment stages.
    ///
    /// Validates that both stages are present and compiled, that they are of
    /// the correct stage type, then merges their uniforms and attributes.
    pub fn link(&mut self) -> Result<(), ShaderError> {
        let vertex = self
            .vertex
            .as_ref()
            .ok_or_else(|| ShaderError::LinkingFailed("no vertex shader attached".to_string()))?;
        if !vertex.is_compiled {
            return Err(ShaderError::LinkingFailed(
                "vertex shader is not compiled".to_string(),
            ));
        }
        if vertex.stage != ShaderStage::Vertex {
            return Err(ShaderError::StageMismatch);
        }

        let fragment = self
            .fragment
            .as_ref()
            .ok_or_else(|| ShaderError::LinkingFailed("no fragment shader attached".to_string()))?;
        if !fragment.is_compiled {
            return Err(ShaderError::LinkingFailed(
                "fragment shader is not compiled".to_string(),
            ));
        }
        if fragment.stage != ShaderStage::Fragment {
            return Err(ShaderError::StageMismatch);
        }

        // Merge uniforms and attributes from the vertex stage.
        self.uniforms.clear();
        self.attributes.clear();
        for u in &vertex.uniforms {
            self.uniforms.push(u.clone());
        }
        for a in &vertex.attributes {
            self.attributes.push(a.clone());
        }
        // Merge fragment uniforms, skipping duplicates by name.
        for u in &fragment.uniforms {
            if !self.uniforms.iter().any(|x| x.name == u.name) {
                self.uniforms.push(u.clone());
            }
        }

        // Combine the two source hashes into a program hash.
        self.program_hash = vertex
            .source_hash
            .rotate_left(32)
            .wrapping_add(fragment.source_hash);

        self.is_linked = true;
        Ok(())
    }

    /// Finds the location of a uniform by name.
    pub fn find_uniform_location(&self, name: &str) -> Option<i32> {
        self.uniforms
            .iter()
            .find(|u| u.name == name)
            .map(|u| u.location)
    }

    /// Returns the merged uniforms of the linked program.
    pub fn uniforms(&self) -> &[UniformInfo] {
        &self.uniforms
    }

    /// Returns the merged vertex attributes of the linked program.
    pub fn attributes(&self) -> &[VertexAttribute] {
        &self.attributes
    }

    /// Returns whether this program has been successfully linked.
    pub fn is_linked(&self) -> bool {
        self.is_linked
    }

    /// Returns the hash of the linked program.
    pub fn program_hash(&self) -> u64 {
        self.program_hash
    }
}

impl Default for ShaderProgram {
    fn default() -> Self {
        Self::new()
    }
}

/// A stored uniform value.
#[derive(Clone, Debug, PartialEq)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
    Int(i32),
    Bool(bool),
}

/// CPU-side helper for storing uniform values keyed by location.
///
/// Requires the `std` feature because it uses `std::collections::HashMap`.
#[cfg(feature = "std")]
pub struct UniformBinder {
    values: std::collections::HashMap<i32, UniformValue>,
}

#[cfg(feature = "std")]
impl UniformBinder {
    /// Creates a new, empty uniform binder.
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }

    /// Sets a `float` uniform value at the given location.
    pub fn set_float(&mut self, location: i32, value: f32) {
        self.values.insert(location, UniformValue::Float(value));
    }

    /// Sets a `vec2` uniform value at the given location.
    pub fn set_vec2(&mut self, location: i32, value: Vec2) {
        self.values
            .insert(location, UniformValue::Vec2([value.x, value.y]));
    }

    /// Sets a `vec3` uniform value at the given location.
    pub fn set_vec3(&mut self, location: i32, value: Vec3) {
        self.values
            .insert(location, UniformValue::Vec3([value.x, value.y, value.z]));
    }

    /// Sets a `vec4` uniform value at the given location.
    pub fn set_vec4(&mut self, location: i32, value: Vec4) {
        self.values.insert(
            location,
            UniformValue::Vec4([value.x, value.y, value.z, value.w]),
        );
    }

    /// Sets a `mat4` uniform value at the given location.
    pub fn set_mat4(&mut self, location: i32, value: Mat4) {
        let mut flat = [0.0f32; 16];
        for (col, col_vals) in value.cols.iter().enumerate() {
            for (row, &v) in col_vals.iter().enumerate() {
                flat[col * 4 + row] = v;
            }
        }
        self.values.insert(location, UniformValue::Mat4(flat));
    }

    /// Sets an `int` uniform value at the given location.
    pub fn set_int(&mut self, location: i32, value: i32) {
        self.values.insert(location, UniformValue::Int(value));
    }

    /// Sets a `bool` uniform value at the given location.
    pub fn set_bool(&mut self, location: i32, value: bool) {
        self.values.insert(location, UniformValue::Bool(value));
    }

    /// Returns the uniform value stored at the given location, if any.
    pub fn get_value(&self, location: i32) -> Option<&UniformValue> {
        self.values.get(&location)
    }
}

#[cfg(feature = "std")]
impl Default for UniformBinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_stage_variants() {
        assert_eq!(ShaderStage::Vertex, ShaderStage::Vertex);
        assert_ne!(ShaderStage::Vertex, ShaderStage::Fragment);
        let stages = [
            ShaderStage::Vertex,
            ShaderStage::Fragment,
            ShaderStage::Geometry,
            ShaderStage::Compute,
            ShaderStage::TessellationControl,
            ShaderStage::TessellationEvaluation,
        ];
        // All variants are distinct.
        for (i, a) in stages.iter().enumerate() {
            for (j, b) in stages.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_shader_stage_copy_clone() {
        let a = ShaderStage::Compute;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_shader_source_glsl() {
        let src = ShaderSource::Glsl("#version 450\nvoid main(){}".to_string());
        if let ShaderSource::Glsl(s) = &src {
            assert!(s.starts_with("#version"));
        } else {
            panic!("expected Glsl variant");
        }
    }

    #[test]
    fn test_shader_source_wgsl() {
        let src = ShaderSource::Wgsl("@vertex fn vs() {}".to_string());
        if let ShaderSource::Wgsl(s) = &src {
            assert!(s.contains("@vertex"));
        } else {
            panic!("expected Wgsl variant");
        }
    }

    #[test]
    fn test_shader_source_spirv() {
        let src = ShaderSource::SpirV(alloc::vec![0x07230203, 0x00010000]);
        if let ShaderSource::SpirV(words) = &src {
            assert_eq!(words.len(), 2);
            assert_eq!(words[0], 0x07230203);
        } else {
            panic!("expected SpirV variant");
        }
    }

    #[test]
    fn test_shader_error_display() {
        assert_eq!(
            ShaderError::InvalidSource.to_string(),
            "invalid shader source"
        );
        assert_eq!(
            ShaderError::StageMismatch.to_string(),
            "shader stage mismatch"
        );
        assert_eq!(
            ShaderError::CompilationFailed("boom".to_string()).to_string(),
            "shader compilation failed: boom"
        );
        assert_eq!(
            ShaderError::LinkingFailed("nope".to_string()).to_string(),
            "shader linking failed: nope"
        );
        assert_eq!(
            ShaderError::ValidationError("bad".to_string()).to_string(),
            "shader validation error: bad"
        );
    }

    #[test]
    fn test_uniform_type_byte_size() {
        assert_eq!(UniformType::Float.byte_size(), 4);
        assert_eq!(UniformType::Int.byte_size(), 4);
        assert_eq!(UniformType::Bool.byte_size(), 4);
        assert_eq!(UniformType::Vec2.byte_size(), 8);
        assert_eq!(UniformType::Vec3.byte_size(), 12);
        assert_eq!(UniformType::Vec4.byte_size(), 16);
        assert_eq!(UniformType::Mat4.byte_size(), 64);
        assert_eq!(UniformType::Sampler2D.byte_size(), 4);
        assert_eq!(UniformType::SamplerCube.byte_size(), 4);
    }

    #[test]
    fn test_uniform_info_new_defaults() {
        let info = UniformInfo::new("u_color".to_string(), UniformType::Vec4, 3);
        assert_eq!(info.name, "u_color");
        assert_eq!(info.uniform_type, UniformType::Vec4);
        assert_eq!(info.location, 3);
        assert_eq!(info.byte_size, 16);
        assert_eq!(info.array_size, 1);
    }

    #[test]
    fn test_uniform_info_with_array_size() {
        let info = UniformInfo::new("u_bones".to_string(), UniformType::Mat4, 7)
            .with_array_size(32);
        assert_eq!(info.array_size, 32);
        assert_eq!(info.byte_size, 64);
    }

    #[test]
    fn test_vertex_attribute_format_component_count() {
        assert_eq!(VertexAttributeFormat::Float.component_count(), 1);
        assert_eq!(VertexAttributeFormat::Vec2.component_count(), 2);
        assert_eq!(VertexAttributeFormat::Vec3.component_count(), 3);
        assert_eq!(VertexAttributeFormat::Vec4.component_count(), 4);
    }

    #[test]
    fn test_vertex_attribute_format_byte_size() {
        assert_eq!(VertexAttributeFormat::Float.byte_size(), 4);
        assert_eq!(VertexAttributeFormat::Vec2.byte_size(), 8);
        assert_eq!(VertexAttributeFormat::Vec3.byte_size(), 12);
        assert_eq!(VertexAttributeFormat::Vec4.byte_size(), 16);
    }

    #[test]
    fn test_vertex_attribute_new() {
        let attr = VertexAttribute::new(
            "a_position".to_string(),
            0,
            VertexAttributeFormat::Vec3,
        );
        assert_eq!(attr.name, "a_position");
        assert_eq!(attr.location, 0);
        assert_eq!(attr.format, VertexAttributeFormat::Vec3);
        assert_eq!(attr.size, 12);
    }

    #[test]
    fn test_shader_module_new_initial_state() {
        let module = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450".to_string()),
        );
        assert!(!module.is_compiled());
        assert_eq!(module.stage(), ShaderStage::Vertex);
        assert_ne!(module.source_hash(), 0);
        assert!(module.uniforms().is_empty());
        assert!(module.attributes().is_empty());
    }

    #[test]
    fn test_shader_module_compile_success_glsl() {
        let mut module = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450\nvoid main(){}".to_string()),
        );
        assert!(!module.is_compiled());
        assert!(module.compile().is_ok());
        assert!(module.is_compiled());
    }

    #[test]
    fn test_shader_module_compile_success_wgsl() {
        let mut module = ShaderModule::new(
            ShaderStage::Fragment,
            ShaderSource::Wgsl("@fragment fn fs() {}".to_string()),
        );
        assert!(module.compile().is_ok());
        assert!(module.is_compiled());
    }

    #[test]
    fn test_shader_module_compile_success_spirv() {
        let mut module = ShaderModule::new(
            ShaderStage::Compute,
            ShaderSource::SpirV(alloc::vec![0x07230203, 0x00010000]),
        );
        assert!(module.compile().is_ok());
        assert!(module.is_compiled());
    }

    #[test]
    fn test_shader_module_compile_empty_glsl_fails() {
        let mut module = ShaderModule::new(ShaderStage::Vertex, ShaderSource::Glsl(String::new()));
        let err = module.compile().unwrap_err();
        assert!(matches!(err, ShaderError::InvalidSource));
        assert!(!module.is_compiled());
    }

    #[test]
    fn test_shader_module_compile_empty_wgsl_fails() {
        let mut module = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Wgsl(String::new()),
        );
        assert!(matches!(module.compile().unwrap_err(), ShaderError::InvalidSource));
    }

    #[test]
    fn test_shader_module_compile_empty_spirv_fails() {
        let mut module = ShaderModule::new(ShaderStage::Vertex, ShaderSource::SpirV(Vec::new()));
        assert!(matches!(module.compile().unwrap_err(), ShaderError::InvalidSource));
    }

    #[test]
    fn test_shader_module_add_and_find_uniform() {
        let mut module = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450".to_string()),
        );
        module.add_uniform(UniformInfo::new("u_mvp".to_string(), UniformType::Mat4, 0));
        module.add_uniform(UniformInfo::new("u_color".to_string(), UniformType::Vec4, 1));

        assert_eq!(module.uniforms().len(), 2);
        let found = module.find_uniform("u_mvp").expect("uniform should exist");
        assert_eq!(found.location, 0);
        assert_eq!(found.uniform_type, UniformType::Mat4);
        assert!(module.find_uniform("missing").is_none());
    }

    #[test]
    fn test_shader_module_add_attribute() {
        let mut module = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450".to_string()),
        );
        module.add_attribute(VertexAttribute::new(
            "a_pos".to_string(),
            0,
            VertexAttributeFormat::Vec3,
        ));
        module.add_attribute(VertexAttribute::new(
            "a_uv".to_string(),
            1,
            VertexAttributeFormat::Vec2,
        ));
        assert_eq!(module.attributes().len(), 2);
        assert_eq!(module.attributes()[1].location, 1);
    }

    #[test]
    fn test_shader_module_source_hash_consistency() {
        let a = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450".to_string()),
        );
        let b = ShaderModule::new(
            ShaderStage::Fragment,
            ShaderSource::Glsl("#version 450".to_string()),
        );
        // Same source text yields the same hash regardless of stage.
        assert_eq!(a.source_hash(), b.source_hash());
    }

    #[test]
    fn test_shader_module_source_hash_differs() {
        let a = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450".to_string()),
        );
        let b = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 460".to_string()),
        );
        assert_ne!(a.source_hash(), b.source_hash());
    }

    #[test]
    fn test_shader_program_new_initial_state() {
        let program = ShaderProgram::new();
        assert!(!program.is_linked());
        assert_eq!(program.program_hash(), 0);
        assert!(program.uniforms().is_empty());
        assert!(program.attributes().is_empty());
    }

    fn make_compiled_vertex() -> ShaderModule {
        let mut m = ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450\nvoid main(){}".to_string()),
        );
        m.add_attribute(VertexAttribute::new(
            "a_pos".to_string(),
            0,
            VertexAttributeFormat::Vec3,
        ));
        m.add_uniform(UniformInfo::new("u_mvp".to_string(), UniformType::Mat4, 0));
        m.compile().unwrap();
        m
    }

    fn make_compiled_fragment() -> ShaderModule {
        let mut m = ShaderModule::new(
            ShaderStage::Fragment,
            ShaderSource::Glsl("#version 450\nvoid main(){}".to_string()),
        );
        m.add_uniform(UniformInfo::new("u_color".to_string(), UniformType::Vec4, 1));
        m.compile().unwrap();
        m
    }

    #[test]
    fn test_shader_program_link_success() {
        let mut program = ShaderProgram::new();
        program.attach_vertex(make_compiled_vertex());
        program.attach_fragment(make_compiled_fragment());

        assert!(program.link().is_ok());
        assert!(program.is_linked());
        assert_ne!(program.program_hash(), 0);
        // Vertex attribute merged.
        assert_eq!(program.attributes().len(), 1);
        // Both vertex and fragment uniforms merged.
        assert_eq!(program.uniforms().len(), 2);
    }

    #[test]
    fn test_shader_program_link_no_vertex_fails() {
        let mut program = ShaderProgram::new();
        program.attach_fragment(make_compiled_fragment());
        let err = program.link().unwrap_err();
        assert!(matches!(err, ShaderError::LinkingFailed(_)));
        assert!(!program.is_linked());
    }

    #[test]
    fn test_shader_program_link_no_fragment_fails() {
        let mut program = ShaderProgram::new();
        program.attach_vertex(make_compiled_vertex());
        let err = program.link().unwrap_err();
        assert!(matches!(err, ShaderError::LinkingFailed(_)));
        assert!(!program.is_linked());
    }

    #[test]
    fn test_shader_program_link_not_compiled_fails() {
        let mut program = ShaderProgram::new();
        // Attach an uncompiled vertex shader.
        program.attach_vertex(ShaderModule::new(
            ShaderStage::Vertex,
            ShaderSource::Glsl("#version 450".to_string()),
        ));
        program.attach_fragment(make_compiled_fragment());
        let err = program.link().unwrap_err();
        assert!(matches!(err, ShaderError::LinkingFailed(_)));
        assert!(!program.is_linked());
    }

    #[test]
    fn test_shader_program_link_stage_mismatch_fails() {
        let mut program = ShaderProgram::new();
        // Attach a compiled fragment shader in the vertex slot.
        program.attach_vertex(make_compiled_fragment());
        program.attach_fragment(make_compiled_fragment());
        let err = program.link().unwrap_err();
        assert!(matches!(err, ShaderError::StageMismatch));
        assert!(!program.is_linked());
    }

    #[test]
    fn test_shader_program_find_uniform_location() {
        let mut program = ShaderProgram::new();
        program.attach_vertex(make_compiled_vertex());
        program.attach_fragment(make_compiled_fragment());
        program.link().unwrap();

        assert_eq!(program.find_uniform_location("u_mvp"), Some(0));
        assert_eq!(program.find_uniform_location("u_color"), Some(1));
        assert_eq!(program.find_uniform_location("missing"), None);
    }

    #[test]
    fn test_shader_program_dedup_uniforms() {
        let mut vertex = make_compiled_vertex();
        // Add a uniform that also exists in the fragment shader.
        vertex.add_uniform(UniformInfo::new("u_color".to_string(), UniformType::Vec4, 1));
        let mut program = ShaderProgram::new();
        program.attach_vertex(vertex);
        program.attach_fragment(make_compiled_fragment());
        program.link().unwrap();
        // u_mvp (vertex), u_color (vertex), u_color should not be duplicated from fragment.
        let color_count = program
            .uniforms()
            .iter()
            .filter(|u| u.name == "u_color")
            .count();
        assert_eq!(color_count, 1);
    }

    #[test]
    fn test_shader_program_program_hash_stable() {
        let mut program1 = ShaderProgram::new();
        program1.attach_vertex(make_compiled_vertex());
        program1.attach_fragment(make_compiled_fragment());
        program1.link().unwrap();

        let mut program2 = ShaderProgram::new();
        program2.attach_vertex(make_compiled_vertex());
        program2.attach_fragment(make_compiled_fragment());
        program2.link().unwrap();

        assert_eq!(program1.program_hash(), program2.program_hash());
    }

    #[test]
    fn test_hash_source_consistency() {
        assert_eq!(hash_source("hello"), hash_source("hello"));
        assert_ne!(hash_source("hello"), hash_source("world"));
        assert_eq!(hash_source(""), hash_source(""));
    }

    #[test]
    fn test_hash_source_known_value() {
        // FNV-1a of empty string is the offset basis.
        assert_eq!(hash_source(""), 0xcbf29ce484222325);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_float() {
        let mut binder = UniformBinder::new();
        binder.set_float(0, 1.5);
        match binder.get_value(0) {
            Some(UniformValue::Float(v)) => assert!((*v - 1.5).abs() < 1e-6),
            other => panic!("expected Float, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_vec2() {
        let mut binder = UniformBinder::new();
        binder.set_vec2(1, Vec2::new(1.0, 2.0));
        match binder.get_value(1) {
            Some(UniformValue::Vec2(v)) => {
                assert_eq!(*v, [1.0, 2.0]);
            }
            other => panic!("expected Vec2, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_vec3() {
        let mut binder = UniformBinder::new();
        binder.set_vec3(2, Vec3::new(1.0, 2.0, 3.0));
        match binder.get_value(2) {
            Some(UniformValue::Vec3(v)) => {
                assert_eq!(*v, [1.0, 2.0, 3.0]);
            }
            other => panic!("expected Vec3, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_vec4() {
        let mut binder = UniformBinder::new();
        binder.set_vec4(3, Vec4::new(1.0, 2.0, 3.0, 4.0));
        match binder.get_value(3) {
            Some(UniformValue::Vec4(v)) => {
                assert_eq!(*v, [1.0, 2.0, 3.0, 4.0]);
            }
            other => panic!("expected Vec4, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_mat4() {
        let mut binder = UniformBinder::new();
        binder.set_mat4(4, Mat4::IDENTITY);
        match binder.get_value(4) {
            Some(UniformValue::Mat4(v)) => {
                // Identity matrix flattened column-major.
                assert_eq!(*v, [
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                ]);
            }
            other => panic!("expected Mat4, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_int() {
        let mut binder = UniformBinder::new();
        binder.set_int(5, 42);
        match binder.get_value(5) {
            Some(UniformValue::Int(v)) => assert_eq!(*v, 42),
            other => panic!("expected Int, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_set_bool() {
        let mut binder = UniformBinder::new();
        binder.set_bool(6, true);
        match binder.get_value(6) {
            Some(UniformValue::Bool(v)) => assert!(*v),
            other => panic!("expected Bool, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_get_missing() {
        let binder = UniformBinder::new();
        assert!(binder.get_value(99).is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_binder_overwrite() {
        let mut binder = UniformBinder::new();
        binder.set_float(0, 1.0);
        binder.set_float(0, 2.0);
        match binder.get_value(0) {
            Some(UniformValue::Float(v)) => assert!((*v - 2.0).abs() < 1e-6),
            other => panic!("expected Float, got {:?}", other),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_uniform_value_equality() {
        assert_eq!(UniformValue::Float(1.0), UniformValue::Float(1.0));
        assert_ne!(UniformValue::Float(1.0), UniformValue::Float(2.0));
        assert_eq!(UniformValue::Vec3([1.0, 2.0, 3.0]), UniformValue::Vec3([1.0, 2.0, 3.0]));
        assert_eq!(UniformValue::Int(5), UniformValue::Int(5));
        assert_eq!(UniformValue::Bool(true), UniformValue::Bool(true));
    }

    #[test]
    fn test_shader_program_default() {
        let program = ShaderProgram::default();
        assert!(!program.is_linked());
    }
}
