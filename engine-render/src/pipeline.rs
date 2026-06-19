//! Pipeline 模块 - 通用图形管线与资源绑定组

use std::collections::HashMap;

/// 管线类型
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PipelineType {
    /// 渲染管线（Vertex + Fragment 组合）
    #[default]
    Render,
    /// 计算管线
    Compute,
}

/// 基元拓扑
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PrimitiveTopology {
    /// 点列表
    PointList,
    /// 线条列表
    LineList,
    /// 线条带
    LineStrip,
    /// 三角形列表
    #[default]
    TriangleList,
    /// 三角形带
    TriangleStrip,
    /// 线条列表（带邻接信息）
    LineListAdjacency,
    /// 三角形列表（带邻接信息）
    TriangleListAdjacency,
}

/// 多边形填充模式
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PolygonMode {
    /// 填充
    #[default]
    Fill,
    /// 线框
    Line,
    /// 点
    Point,
}

/// 比较函数
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    #[default]
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

/// 混合因子
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BlendFactor {
    Zero,
    #[default]
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
}

/// 混合操作
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BlendOperation {
    /// 相加
    #[default]
    Add,
    /// 相减（src - dst）
    Subtract,
    /// 反向相减（dst - src）
    ReverseSubtract,
    /// 取最小
    Min,
    /// 取最大
    Max,
}

/// 颜色写入掩码
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct ColorWriteMask {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

impl ColorWriteMask {
    pub const ALL: ColorWriteMask = ColorWriteMask {
        red: true,
        green: true,
        blue: true,
        alpha: true,
    };

    pub const NONE: ColorWriteMask = ColorWriteMask {
        red: false,
        green: false,
        blue: false,
        alpha: false,
    };

    pub fn new(red: bool, green: bool, blue: bool, alpha: bool) -> Self {
        Self { red, green, blue, alpha }
    }

    pub fn all() -> Self { Self::ALL }

    pub fn none() -> Self { Self::NONE }
}

/// 混合状态
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BlendState {
    pub blend_enabled: bool,
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub operation: BlendOperation,
    pub write_mask: ColorWriteMask,
}

impl Default for BlendState {
    fn default() -> Self {
        Self {
            blend_enabled: true,
            src_factor: BlendFactor::SrcAlpha,
            dst_factor: BlendFactor::OneMinusSrcAlpha,
            operation: BlendOperation::Add,
            write_mask: ColorWriteMask::ALL,
        }
    }
}

impl BlendState {
    /// 标准 alpha 混合
    pub fn alpha_blend() -> Self {
        Self::default()
    }

    /// 加法混合（用于发光效果）
    pub fn additive() -> Self {
        Self {
            blend_enabled: true,
            src_factor: BlendFactor::SrcAlpha,
            dst_factor: BlendFactor::One,
            operation: BlendOperation::Add,
            write_mask: ColorWriteMask::ALL,
        }
    }

    /// 无混合（直接写入）
    pub fn none() -> Self {
        Self {
            blend_enabled: false,
            ..Default::default()
        }
    }
}

/// 深度/模板测试状态
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct DepthStencilState {
    pub depth_enabled: bool,
    pub depth_write: bool,
    pub compare: CompareFunction,
}

impl DepthStencilState {
    pub fn disabled() -> Self {
        Self {
            depth_enabled: false,
            depth_write: false,
            compare: CompareFunction::Always,
        }
    }
}

/// 着色器参数绑定槽位
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BindingType {
    /// 采样器 + 纹理（只读）
    SampledTexture,
    /// 只读存储纹理
    StorageTexture,
    /// 统一缓冲区
    UniformBuffer,
    /// 存储缓冲区
    StorageBuffer,
}

/// 绑定槽位（BindGroup 中的单个绑定项）
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BindingEntry {
    /// 槽位索引（location / binding）
    pub slot: u32,
    /// 数据类型
    pub kind: BindingType,
    /// 可写
    pub writable: bool,
}

impl BindingEntry {
    pub fn new(slot: u32, kind: BindingType) -> Self {
        Self { slot, kind, writable: false }
    }
}

/// 绑定组布局（描述 BindGroup 的结构）
#[derive(Clone, Debug, Default)]
pub struct BindGroupLayout {
    entries: Vec<BindingEntry>,
}

impl BindGroupLayout {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn with_entries(entries: Vec<BindingEntry>) -> Self {
        Self { entries }
    }

    pub fn add(&mut self, entry: BindingEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[BindingEntry] {
        &self.entries
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

/// 绑定组（运行时绑定到管线的资源组）
///
/// BindGroup 保存一组与特定槽位的资源绑定。例如精灵渲染时
/// 同一纹理只需要绑定一次，避免重复绑定。
#[derive(Clone, Debug, Default)]
pub struct BindGroup {
    layout: BindGroupLayout,
    /// 槽位 -> 资源句柄（仅以简单索引标识）
    bindings: HashMap<u32, u64>,
    /// 绑定名称映射（slot -> name for 可读标签）
    labels: HashMap<u32, String>,
    dirty: bool,
}

impl BindGroup {
    /// 创建空绑定组
    pub fn new(layout: BindGroupLayout) -> Self {
        Self {
            layout,
            bindings: HashMap::new(),
            labels: HashMap::new(),
            dirty: true,
        }
    }

    /// 按槽位绑定一个资源句柄
    pub fn bind_handle(&mut self, slot: u32, handle: u64) {
        self.bindings.insert(slot, handle);
        self.dirty = true;
    }

    /// 按槽位绑定一个具名资源（便于调试与管理）
    pub fn bind_named(&mut self, slot: u32, handle: u64, label: &str) {
        self.bindings.insert(slot, handle);
        self.labels.insert(slot, label.to_string());
        self.dirty = true;
    }

    /// 解除绑定
    pub fn unbind(&mut self, slot: u32) {
        self.bindings.remove(&slot);
        self.labels.remove(&slot);
        self.dirty = true;
    }

    /// 是否已绑定
    pub fn has_binding(&self, slot: u32) -> bool {
        self.bindings.contains_key(&slot)
    }

    /// 获取某一绑定句柄
    pub fn get_binding(&self, slot: u32) -> Option<u64> {
        self.bindings.get(&slot).copied()
    }

    /// 获取绑定组布局引用
    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    /// 标记已应用（清除 dirty）
    pub fn mark_applied(&mut self) {
        self.dirty = false;
    }

    /// 是否需要重新应用
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 当前已绑定的槽位数
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }
}

/// 管线描述符
#[derive(Clone, Debug)]
pub struct PipelineDescriptor {
    pub name: String,
    pub pipeline_type: PipelineType,
    pub topology: PrimitiveTopology,
    pub polygon_mode: PolygonMode,
    pub blend_state: BlendState,
    pub depth_stencil: DepthStencilState,
    pub group_layouts: Vec<BindGroupLayout>,
}

impl Default for PipelineDescriptor {
    fn default() -> Self {
        Self {
            name: "anonymous".to_string(),
            pipeline_type: PipelineType::Render,
            topology: PrimitiveTopology::TriangleList,
            polygon_mode: PolygonMode::Fill,
            blend_state: BlendState::default(),
            depth_stencil: DepthStencilState::disabled(),
            group_layouts: Vec::new(),
        }
    }
}

impl PipelineDescriptor {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn with_polygon_mode(mut self, mode: PolygonMode) -> Self {
        self.polygon_mode = mode;
        self
    }

    pub fn with_blend(mut self, state: BlendState) -> Self {
        self.blend_state = state;
        self
    }

    pub fn with_depth(mut self, state: DepthStencilState) -> Self {
        self.depth_stencil = state;
        self
    }

    pub fn add_group_layout(mut self, layout: BindGroupLayout) -> Self {
        self.group_layouts.push(layout);
        self
    }
}

/// 通用管线对象
///
/// 封装管线的 CPU 端信息，保存描述符与可变更的 uniform 数据。
/// 真正的着色器程序编译、VAO 等 GPU 资源由后端 Renderer 创建。
#[derive(Debug)]
pub struct Pipeline {
    descriptor: PipelineDescriptor,
    compiled: bool,
    dirty: bool,
    uniforms: HashMap<String, Vec<f32>>,
}

impl Pipeline {
    pub fn new(descriptor: PipelineDescriptor) -> Self {
        Self {
            descriptor,
            compiled: false,
            dirty: true,
            uniforms: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.descriptor.name
    }

    pub fn descriptor(&self) -> &PipelineDescriptor {
        &self.descriptor
    }

    pub fn is_compiled(&self) -> bool {
        self.compiled
    }

    pub fn mark_compiled(&mut self) {
        self.compiled = true;
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 设置 uniform 数据（按名称）
    pub fn set_uniform(&mut self, name: &str, values: Vec<f32>) {
        self.uniforms.insert(name.to_string(), values);
        self.dirty = true;
    }

    /// 获取 uniform 数据
    pub fn get_uniform(&self, name: &str) -> Option<&[f32]> {
        self.uniforms.get(name).map(|v| v.as_slice())
    }

    pub fn uniform_count(&self) -> usize {
        self.uniforms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_descriptor_defaults() {
        let desc = PipelineDescriptor::default();
        assert_eq!(desc.name, "anonymous");
        assert_eq!(desc.pipeline_type, PipelineType::Render);
        assert_eq!(desc.topology, PrimitiveTopology::TriangleList);
        assert_eq!(desc.polygon_mode, PolygonMode::Fill);
    }

    #[test]
    fn test_pipeline_descriptor_builder() {
        let desc = PipelineDescriptor::default()
            .with_name("sprite")
            .with_topology(PrimitiveTopology::TriangleStrip);
        assert_eq!(desc.name, "sprite");
        assert_eq!(desc.topology, PrimitiveTopology::TriangleStrip);
    }

    #[test]
    fn test_blend_state_presets() {
        let alpha = BlendState::alpha_blend();
        assert!(alpha.blend_enabled);
        let additive = BlendState::additive();
        assert_eq!(additive.dst_factor, BlendFactor::One);
        let none = BlendState::none();
        assert!(!none.blend_enabled);
    }

    #[test]
    fn test_color_write_mask() {
        let all = ColorWriteMask::all();
        assert!(all.red && all.green && all.blue && all.alpha);
        let none = ColorWriteMask::none();
        assert!(!none.red && !none.green && !none.blue && !none.alpha);
        let custom = ColorWriteMask::new(true, false, true, false);
        assert!(custom.red && custom.blue);
        assert!(!custom.green && !custom.alpha);
    }

    #[test]
    fn test_depth_stencil_disabled() {
        let d = DepthStencilState::disabled();
        assert!(!d.depth_enabled);
        assert_eq!(d.compare, CompareFunction::Always);
    }

    #[test]
    fn test_bind_group_layout() {
        let mut layout = BindGroupLayout::new();
        let entry = BindingEntry::new(0, BindingType::SampledTexture);
        layout.add(entry);
        assert_eq!(layout.count(), 1);
        assert_eq!(layout.entries().len(), 1);
    }

    #[test]
    fn test_bind_group() {
        let layout = BindGroupLayout::with_entries(vec![
            BindingEntry::new(0, BindingType::SampledTexture),
        ]);
        let mut bg = BindGroup::new(layout);
        bg.bind_handle(0, 42);
        assert!(bg.has_binding(0));
        assert!(bg.is_dirty()); // bind_handle sets dirty
        assert_eq!(bg.get_binding(0), Some(42));
        bg.mark_applied();
        assert!(!bg.is_dirty());
        bg.unbind(0);
        assert!(!bg.has_binding(0));
    }

    #[test]
    fn test_bind_group_named() {
        let layout = BindGroupLayout::new();
        let mut bg = BindGroup::new(layout);
        bg.bind_named(0, 123, "texture0");
        assert_eq!(bg.get_binding(0), Some(123));
        assert_eq!(bg.binding_count(), 1);
    }

    #[test]
    fn test_pipeline_new() {
        let desc = PipelineDescriptor::default().with_name("test");
        let mut pipeline = Pipeline::new(desc);
        assert_eq!(pipeline.name(), "test");
        assert!(!pipeline.is_compiled());
        assert!(pipeline.is_dirty());
        pipeline.mark_compiled();
        assert!(pipeline.is_compiled());
        assert!(!pipeline.is_dirty());
    }

    #[test]
    fn test_pipeline_uniforms() {
        let desc = PipelineDescriptor::default();
        let mut pipeline = Pipeline::new(desc);
        pipeline.set_uniform("mvp", vec![1.0, 2.0, 3.0, 4.0]);
        assert!(pipeline.get_uniform("mvp").is_some());
        assert!(pipeline.get_uniform("missing").is_none());
        assert_eq!(pipeline.uniform_count(), 1);
        assert!(pipeline.is_dirty());
    }

    #[test]
    fn test_binding_type_variants() {
        let _ = BindingType::SampledTexture;
        let _ = BindingType::StorageTexture;
        let _ = BindingType::UniformBuffer;
        let _ = BindingType::StorageBuffer;
    }

    #[test]
    fn test_pipeline_type_default() {
        assert_eq!(PipelineType::default(), PipelineType::Render);
    }

    #[test]
    fn test_polygon_mode_default() {
        assert_eq!(PolygonMode::default(), PolygonMode::Fill);
    }

    #[test]
    fn test_compare_function_default() {
        assert_eq!(CompareFunction::default(), CompareFunction::LessEqual);
    }

    #[test]
    fn test_blend_factor_default() {
        assert_eq!(BlendFactor::default(), BlendFactor::One);
    }

    #[test]
    fn test_blend_operation_default() {
        assert_eq!(BlendOperation::default(), BlendOperation::Add);
    }

    #[test]
    fn test_primitive_topology_default() {
        assert_eq!(PrimitiveTopology::default(), PrimitiveTopology::TriangleList);
    }
}
