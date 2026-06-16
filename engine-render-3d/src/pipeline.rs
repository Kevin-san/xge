//! 渲染管线模块
//!
//! 提供 RenderPipeline3D、PipelineStateCache、RenderStats3D。

use engine_math::Vec4;
use std::collections::HashMap;

use crate::camera::Camera3D;
use crate::light::LightManager;
use crate::material::Material3D;
use crate::scene::Scene3D;

/// 混合模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// 不透明
    Opaque,
    /// 透明混合
    Alpha,
    /// 加法
    Additive,
    /// 乘
    Multiply,
}

/// 面剔除
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceCulling {
    /// 关闭
    None,
    /// 背面剔除（顺时针）
    Back,
    /// 正面剔除
    Front,
}

/// 渲染状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderState {
    /// 深度测试
    pub depth_test: bool,
    /// 深度写入
    pub depth_write: bool,
    /// 面剔除
    pub face_culling: FaceCulling,
    /// 混合模式
    pub blend_mode: BlendMode,
    /// 线框模式
    pub wireframe: bool,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            depth_test: true,
            depth_write: true,
            face_culling: FaceCulling::Back,
            blend_mode: BlendMode::Opaque,
            wireframe: false,
        }
    }
}

/// 渲染统计
#[derive(Debug, Clone, Default)]
pub struct RenderStats3D {
    /// 绘制调用次数
    pub draw_calls: u32,
    /// 三角面数量
    pub triangles: u32,
    /// 顶点数
    pub vertices: u32,
    /// 渲染实体数
    pub entities_rendered: u32,
    /// 裁剪实体数
    pub entities_culled: u32,
}

impl RenderStats3D {
    /// 重置统计
    pub fn reset(&mut self) {
        self.draw_calls = 0;
        self.triangles = 0;
        self.vertices = 0;
        self.entities_rendered = 0;
        self.entities_culled = 0;
    }
}

/// 渲染管线状态缓存
#[derive(Debug, Clone, Default)]
pub struct PipelineStateCache {
    states: HashMap<String, RenderState>,
}

impl PipelineStateCache {
    /// 创建新缓存
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// 获取或创建状态
    pub fn get_or_create(&mut self, key: &str) -> &mut RenderState {
        self.states.entry(key.to_string()).or_insert_with(RenderState::default)
    }

    /// 缓存命中数
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

/// 渲染管线 3D
pub struct RenderPipeline3D {
    /// 清屏颜色
    clear_color: Vec4,
    /// 渲染状态
    state: RenderState,
    /// MSAA 采样数
    msaa_samples: u32,
    /// 状态缓存
    state_cache: PipelineStateCache,
    /// 渲染统计
    stats: RenderStats3D,
    /// 是否已初始化
    initialized: bool,
}

impl RenderPipeline3D {
    /// 创建新管线
    pub fn new() -> Self {
        Self {
            clear_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
            state: RenderState::default(),
            msaa_samples: 1,
            state_cache: PipelineStateCache::new(),
            stats: RenderStats3D::default(),
            initialized: false,
        }
    }

    /// 初始化
    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// 是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 开始一帧
    pub fn begin_frame(&mut self) {
        self.stats.reset();
    }

    /// 结束一帧
    pub fn end_frame(&mut self) {
        // 帧结束处理
    }

    /// 清屏颜色
    pub fn clear_color(&self) -> Vec4 {
        self.clear_color
    }

    /// 设置清屏颜色
    pub fn set_clear_color(&mut self, color: Vec4) {
        self.clear_color = color;
    }

    /// 启用/禁用深度测试
    pub fn set_depth_test(&mut self, enabled: bool) {
        self.state.depth_test = enabled;
    }

    /// 启用/禁用深度写入
    pub fn set_depth_write(&mut self, enabled: bool) {
        self.state.depth_write = enabled;
    }

    /// 设置面剔除
    pub fn set_face_culling(&mut self, mode: FaceCulling) {
        self.state.face_culling = mode;
    }

    /// 设置混合模式
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.state.blend_mode = mode;
    }

    /// 启用/禁用线框模式
    pub fn set_wireframe(&mut self, enabled: bool) {
        self.state.wireframe = enabled;
    }

    /// 设置 MSAA 采样数
    pub fn set_msaa(&mut self, samples: u32) {
        self.msaa_samples = samples.clamp(1, 16);
    }

    /// 获取当前状态
    pub fn state(&self) -> &RenderState {
        &self.state
    }

    /// 获取状态缓存
    pub fn state_cache(&mut self) -> &mut PipelineStateCache {
        &mut self.state_cache
    }

    /// 获取渲染统计
    pub fn stats(&self) -> &RenderStats3D {
        &self.stats
    }

    /// 记录一次 draw call
    pub fn record_draw_call(&mut self, triangles: u32, vertices: u32) {
        self.stats.draw_calls += 1;
        self.stats.triangles += triangles;
        self.stats.vertices += vertices;
    }

    /// 记录渲染实体
    pub fn record_entity_rendered(&mut self) {
        self.stats.entities_rendered += 1;
    }

    /// 记录被裁剪实体
    pub fn record_entity_culled(&mut self) {
        self.stats.entities_culled += 1;
    }

    /// 重新编译所有着色器（占位 API）
    pub fn recompile_shaders(&mut self) {
        // 实际实现中会清空着色器缓存
    }

    /// 准备场景绘制（更新统计、应用视锥裁剪等）
    pub fn prepare_scene(&mut self, scene: &Scene3D, _camera: &Camera3D, _lights: &LightManager) {
        let visible = scene.visible_entities();
        for entity in &visible {
            self.stats.entities_rendered += 1;
            let _ = entity; // 占位
        }
    }

    /// 检查材质与当前状态是否匹配（用于优化状态切换）
    pub fn is_material_compatible(&self, material: &Material3D) -> bool {
        // 简化：默认不透明和启用了光照
        material.lit() && !material.double_sided()
    }
}

impl Default for RenderPipeline3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_default() {
        let p = RenderPipeline3D::new();
        assert!(!p.is_initialized());
    }

    #[test]
    fn test_pipeline_state() {
        let mut p = RenderPipeline3D::new();
        p.set_depth_test(false);
        assert!(!p.state().depth_test);

        p.set_blend_mode(BlendMode::Alpha);
        assert_eq!(p.state().blend_mode, BlendMode::Alpha);
    }

    #[test]
    fn test_pipeline_stats() {
        let mut p = RenderPipeline3D::new();
        p.begin_frame();
        p.record_draw_call(12, 36);
        p.record_entity_rendered();
        p.record_entity_culled();

        assert_eq!(p.stats().draw_calls, 1);
        assert_eq!(p.stats().triangles, 12);
        assert_eq!(p.stats().entities_rendered, 1);
        assert_eq!(p.stats().entities_culled, 1);
    }
}
