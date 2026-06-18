//! OpenGL 渲染后端实现
//!
//! 提供基于 glow crate 的完整 OpenGL 渲染实现。

use glow::{Buffer, HasContext, NativeFramebuffer, NativeProgram, NativeShader, NativeTexture};
use std::collections::HashMap;
use std::ptr;

use crate::sprite::Rect;
use crate::{
    BlendMode, Camera2D, Color, DrawParams, Image, OrthographicCamera, RenderStats, Renderer,
    Texture2D, TextureHandle,
};
use engine_math::{Mat4, Vec2, Vec3};

/// OpenGL 着色器错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderError {
    /// 顶点着色器编译失败
    VertexCompile(String),
    /// 片段着色器编译失败
    FragmentCompile(String),
    /// 程序链接失败
    ProgramLink(String),
    /// 资源创建失败
    ResourceCreation(String),
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderError::VertexCompile(msg) => write!(f, "Vertex shader compile error: {}", msg),
            ShaderError::FragmentCompile(msg) => write!(f, "Fragment shader compile error: {}", msg),
            ShaderError::ProgramLink(msg) => write!(f, "Program link error: {}", msg),
            ShaderError::ResourceCreation(msg) => write!(f, "Resource creation error: {}", msg),
        }
    }
}

impl std::error::Error for ShaderError {}

/// 精灵顶点格式
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct SpriteVertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
    color: [f32; 4],
}

impl SpriteVertex {
    fn new(x: f32, y: f32, z: f32, u: f32, v: f32, color: Color) -> Self {
        Self {
            position: [x, y, z],
            tex_coord: [u, v],
            color: [color.r, color.g, color.b, color.a],
        }
    }
}

/// OpenGL 渲染器
pub struct GlRenderer {
    /// glow OpenGL 上下文
    gl: glow::Context,
    /// 窗口尺寸
    window_size: (u32, u32),
    /// 清除颜色
    clear_color: Color,
    /// 垂直同步
    vsync: bool,
    /// 统计信息
    stats: RenderStats,
    /// 当前混合模式
    current_blend_mode: BlendMode,
    /// 相机
    camera: Option<Camera2D>,
    /// 正交相机
    ortho_camera: OrthographicCamera,
    /// 变换矩阵栈
    transform_stack: Vec<Mat4>,
    /// 当前变换
    current_transform: Mat4,
    /// 裁剪矩形栈
    scissor_stack: Vec<Rect>,
    /// 默认精灵着色器程序
    sprite_program: NativeProgram,
    /// 纯色着色器程序
    color_program: NativeProgram,
    /// 纹理缓存
    textures: HashMap<u32, NativeTexture>,
    /// 当前绑定的纹理
    current_texture: Option<NativeTexture>,
    /// 顶点缓冲区
    vertex_buffer: Buffer,
    /// 索引缓冲区
    index_buffer: Buffer,
    /// 当前批次的顶点数据
    batch_vertices: Vec<SpriteVertex>,
    /// 当前批次的索引数据
    batch_indices: Vec<u32>,
    /// 当前批次的纹理
    batch_texture: Option<u32>,
    /// 着色器 uniform 位置缓存
    sprite_uniforms: SpriteUniforms,
    /// 是否启用深度测试
    depth_test: bool,
    /// 是否启用裁剪测试
    scissor_test: bool,
}

/// 精灵着色器 uniform 位置
struct SpriteUniforms {
    projection: Option<glow::UniformLocation>,
    view: Option<glow::UniformLocation>,
    transform: Option<glow::UniformLocation>,
    texture: Option<glow::UniformLocation>,
    has_texture: Option<glow::UniformLocation>,
    tint: Option<glow::UniformLocation>,
}

impl GlRenderer {
    /// 创建 OpenGL 渲染器
    /// 
    /// # Panics
    /// 如果着色器编译失败则 panic，推荐使用 `try_new()` 替代
    pub fn new(gl: glow::Context, width: u32, height: u32) -> Self {
        Self::try_new(gl, width, height).expect("Failed to create OpenGL renderer")
    }

    /// 初始化 OpenGL 状态
    fn init_gl_state(&mut self) {
        unsafe {
            // 启用混合
            self.gl.enable(glow::BLEND);
            self.apply_blend_mode(self.current_blend_mode);

            // 禁用深度测试（2D渲染不需要）
            self.gl.disable(glow::DEPTH_TEST);

            // 设置视口
            self.gl
                .viewport(0, 0, self.window_size.0 as i32, self.window_size.1 as i32);

            // 设置清除颜色
            self.set_clear_color(self.clear_color);
        }
    }

    /// 尝试创建 OpenGL 渲染器（推荐使用）
    /// 
    /// 返回 `Result<GlRenderer, ShaderError>` 如果初始化成功，否则返回着色器编译/链接错误
    pub fn try_new(gl: glow::Context, width: u32, height: u32) -> Result<Self, ShaderError> {
        unsafe {
            // 创建着色器程序
            let sprite_program = Self::create_sprite_program(&gl)?;
            let color_program = Self::create_color_program(&gl)?;

            // 创建缓冲区
            let vertex_buffer = gl.create_buffer()
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;
            let index_buffer = gl.create_buffer()
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            // 获取 uniform 位置
            let sprite_uniforms = SpriteUniforms {
                projection: gl.get_uniform_location(sprite_program, "uProjection"),
                view: gl.get_uniform_location(sprite_program, "uView"),
                transform: gl.get_uniform_location(sprite_program, "uTransform"),
                texture: gl.get_uniform_location(sprite_program, "uTexture"),
                has_texture: gl.get_uniform_location(sprite_program, "uHasTexture"),
                tint: gl.get_uniform_location(sprite_program, "uTint"),
            };

            let mut renderer = Self {
                gl,
                window_size: (width, height),
                clear_color: Color::BLACK,
                vsync: true,
                stats: RenderStats::new(),
                current_blend_mode: BlendMode::Alpha,
                camera: None,
                ortho_camera: OrthographicCamera::from_window(width, height, 1.0),
                transform_stack: Vec::new(),
                current_transform: Mat4::IDENTITY,
                scissor_stack: Vec::new(),
                sprite_program,
                color_program,
                textures: HashMap::new(),
                current_texture: None,
                vertex_buffer,
                index_buffer,
                batch_vertices: Vec::with_capacity(4096),
                batch_indices: Vec::with_capacity(6144),
                batch_texture: None,
                sprite_uniforms,
                depth_test: false,
                scissor_test: false,
            };

            // 初始化 OpenGL 状态
            renderer.init_gl_state();

            Ok(renderer)
        }
    }

    /// 创建精灵着色器程序
    fn create_sprite_program(gl: &glow::Context) -> Result<NativeProgram, ShaderError> {
        unsafe {
            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            gl.shader_source(
                vertex_shader,
                r#"
                #version 330 core
                layout (location = 0) in vec3 aPosition;
                layout (location = 1) in vec2 aTexCoord;
                layout (location = 2) in vec4 aColor;
                
                uniform mat4 uProjection;
                uniform mat4 uView;
                uniform mat4 uTransform;
                
                out vec2 vTexCoord;
                out vec4 vColor;
                
                void main() {
                    gl_Position = uProjection * uView * uTransform * vec4(aPosition, 1.0);
                    vTexCoord = aTexCoord;
                    vColor = aColor;
                }
                "#,
            );
            gl.compile_shader(vertex_shader);

            if !gl.get_shader_compile_status(vertex_shader) {
                let msg = gl.get_shader_info_log(vertex_shader);
                return Err(ShaderError::VertexCompile(msg));
            }

            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            gl.shader_source(
                fragment_shader,
                r#"
                #version 330 core
                in vec2 vTexCoord;
                in vec4 vColor;
                
                uniform sampler2D uTexture;
                uniform bool uHasTexture;
                uniform vec4 uTint;
                
                out vec4 fragColor;
                
                void main() {
                    if (uHasTexture) {
                        vec4 texColor = texture(uTexture, vTexCoord);
                        fragColor = texColor * vColor * uTint;
                    } else {
                        fragColor = vColor * uTint;
                    }
                }
                "#,
            );
            gl.compile_shader(fragment_shader);

            if !gl.get_shader_compile_status(fragment_shader) {
                let msg = gl.get_shader_info_log(fragment_shader);
                return Err(ShaderError::FragmentCompile(msg));
            }

            let program = gl.create_program()
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let msg = gl.get_program_info_log(program);
                return Err(ShaderError::ProgramLink(msg));
            }

            // 删除着色器（已链接到程序，不再需要）
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(program)
        }
    }

    /// 创建纯色着色器程序
    fn create_color_program(gl: &glow::Context) -> Result<NativeProgram, ShaderError> {
        unsafe {
            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            gl.shader_source(
                vertex_shader,
                r#"
                #version 330 core
                layout (location = 0) in vec3 aPosition;
                layout (location = 2) in vec4 aColor;
                
                uniform mat4 uProjection;
                uniform mat4 uView;
                uniform mat4 uTransform;
                
                out vec4 vColor;
                
                void main() {
                    gl_Position = uProjection * uView * uTransform * vec4(aPosition, 1.0);
                    vColor = aColor;
                }
                "#,
            );
            gl.compile_shader(vertex_shader);

            if !gl.get_shader_compile_status(vertex_shader) {
                let msg = gl.get_shader_info_log(vertex_shader);
                return Err(ShaderError::VertexCompile(msg));
            }

            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            gl.shader_source(
                fragment_shader,
                r#"
                #version 330 core
                in vec4 vColor;
                out vec4 fragColor;
                
                void main() {
                    fragColor = vColor;
                }
                "#,
            );
            gl.compile_shader(fragment_shader);

            if !gl.get_shader_compile_status(fragment_shader) {
                let msg = gl.get_shader_info_log(fragment_shader);
                return Err(ShaderError::FragmentCompile(msg));
            }

            let program = gl.create_program()
                .map_err(|e| ShaderError::ResourceCreation(e.to_string()))?;

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let msg = gl.get_program_info_log(program);
                return Err(ShaderError::ProgramLink(msg));
            }

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(program)
        }
    }

    /// 应用混合模式
    fn apply_blend_mode(&self, mode: BlendMode) {
        unsafe {
            match mode {
                BlendMode::Alpha => {
                    self.gl
                        .blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
                }
                BlendMode::Additive => {
                    self.gl.blend_func(glow::SRC_ALPHA, glow::ONE);
                }
                BlendMode::Subtract => {
                    self.gl.blend_func(glow::ZERO, glow::ONE_MINUS_SRC_COLOR);
                }
                BlendMode::Multiply => {
                    self.gl.blend_func(glow::DST_COLOR, glow::ZERO);
                }
                BlendMode::Replace => {
                    self.gl.blend_func(glow::ONE, glow::ZERO);
                }
                BlendMode::Invert => {
                    self.gl
                        .blend_func(glow::ONE_MINUS_DST_COLOR, glow::ONE_MINUS_SRC_COLOR);
                }
                BlendMode::PreMultiplied => {
                    self.gl.blend_func(glow::ONE, glow::ONE_MINUS_SRC_ALPHA);
                }
                BlendMode::None => {
                    self.gl.blend_func(glow::ONE, glow::ZERO);
                }
            }
        }
    }

    /// 添加精灵到批处理
    fn add_sprite_to_batch(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        color: Color,
        texture_id: Option<u32>,
    ) {
        // 检查是否需要切换纹理并刷新批处理
        if self.batch_texture.is_some() && self.batch_texture != texture_id {
            self.flush_batch();
        }
        self.batch_texture = texture_id;

        let base_index = self.batch_vertices.len() as u32;

        // 添加4个顶点
        self.batch_vertices.extend_from_slice(&[
            SpriteVertex::new(x, y, 0.0, u0, v0, color),
            SpriteVertex::new(x + w, y, 0.0, u1, v0, color),
            SpriteVertex::new(x + w, y + h, 0.0, u1, v1, color),
            SpriteVertex::new(x, y + h, 0.0, u0, v1, color),
        ]);

        // 添加6个索引（两个三角形）
        self.batch_indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    /// 刷新批处理
    fn flush_batch(&mut self) {
        if self.batch_vertices.is_empty() {
            return;
        }

        unsafe {
            // 绑定着色器
            self.gl.use_program(Some(self.sprite_program));

            // 上传顶点数据
            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.batch_vertices),
                glow::DYNAMIC_DRAW,
            );

            // 上传索引数据
            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.index_buffer));
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.batch_indices),
                glow::DYNAMIC_DRAW,
            );

            // 设置顶点属性
            let stride = std::mem::size_of::<SpriteVertex>() as i32;

            // 位置属性 (location = 0)
            self.gl.enable_vertex_attrib_array(0);
            self.gl
                .vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);

            // UV 属性 (location = 1)
            self.gl.enable_vertex_attrib_array(1);
            self.gl
                .vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, stride, 12);

            // 颜色属性 (location = 2)
            self.gl.enable_vertex_attrib_array(2);
            self.gl
                .vertex_attrib_pointer_f32(2, 4, glow::FLOAT, false, stride, 20);

            // 设置相机矩阵
            let proj = self.ortho_camera.projection();
            let view = Mat4::IDENTITY;
            let transform = self.current_transform;

            if let Some(loc) = self.sprite_uniforms.projection {
                let proj_slice = std::slice::from_raw_parts(proj.cols.as_ptr() as *const f32, 16);
                self.gl
                    .uniform_matrix_4_f32_slice(Some(&loc), false, proj_slice);
            }
            if let Some(loc) = self.sprite_uniforms.view {
                let view_slice = std::slice::from_raw_parts(view.cols.as_ptr() as *const f32, 16);
                self.gl
                    .uniform_matrix_4_f32_slice(Some(&loc), false, view_slice);
            }
            if let Some(loc) = self.sprite_uniforms.transform {
                let trans_slice =
                    std::slice::from_raw_parts(transform.cols.as_ptr() as *const f32, 16);
                self.gl
                    .uniform_matrix_4_f32_slice(Some(&loc), false, trans_slice);
            }

            // 绑定纹理
            if let Some(tex_id) = self.batch_texture {
                if let Some(tex) = self.textures.get(&tex_id) {
                    self.gl.active_texture(glow::TEXTURE0);
                    self.gl.bind_texture(glow::TEXTURE_2D, Some(*tex));
                    self.current_texture = Some(*tex);
                }
            } else {
                self.gl.bind_texture(glow::TEXTURE_2D, None);
                self.current_texture = None;
            }

            // 设置纹理 uniform
            if let Some(loc) = self.sprite_uniforms.texture {
                self.gl.uniform_1_i32(Some(&loc), 0);
            }
            if let Some(loc) = self.sprite_uniforms.has_texture {
                self.gl
                    .uniform_1_i32(Some(&loc), if self.batch_texture.is_some() { 1 } else { 0 });
            }
            if let Some(loc) = self.sprite_uniforms.tint {
                self.gl.uniform_4_f32(Some(&loc), 1.0, 1.0, 1.0, 1.0);
            }

            // 绘制
            self.gl.draw_elements(
                glow::TRIANGLES,
                self.batch_indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );

            // 更新统计
            self.stats.add_draw_call(1);
            self.stats.add_vertices(self.batch_vertices.len() as u32);
            self.stats.add_indices(self.batch_indices.len() as u32);
            self.stats.add_batch(1);
        }

        // 清空批处理
        self.batch_vertices.clear();
        self.batch_indices.clear();
        self.batch_texture = None;
    }

    /// 上传纹理到 GPU
    pub fn upload_texture(&mut self, handle: TextureHandle, image: &Image) -> NativeTexture {
        unsafe {
            let texture = self.gl.create_texture().expect("Failed to create texture");
            self.gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            let (format, internal_format) = match image.channels() {
                4 => (glow::RGBA, glow::RGBA8),
                3 => (glow::RGB, glow::RGB8),
                1 => (glow::RED, glow::R8),
                _ => (glow::RGBA, glow::RGBA8),
            };

            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                internal_format as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                Some(image.pixels()),
            );

            // 设置纹理参数
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            self.textures.insert(handle.index(), texture);
            texture
        }
    }

    /// 获取纹理
    pub fn get_texture(&self, handle: &TextureHandle) -> Option<NativeTexture> {
        self.textures.get(&handle.index()).copied()
    }

    /// 检查是否需要创建新的上下文
    pub fn needs_context() -> bool {
        true
    }
}

impl Renderer for GlRenderer {
    fn init(_window: &crate::RenderContext) -> anyhow::Result<Self> {
        Err(anyhow::anyhow!(
            "GlRenderer requires a valid glow::Context. Use GlRenderer::new() instead."
        ))
    }

    fn default_backend() -> &'static str
    where
        Self: Sized,
    {
        "OpenGL (glow)"
    }

    fn backend_info(&self) -> String {
        unsafe {
            let vendor = self.gl.get_parameter_string(glow::VENDOR);
            let renderer = self.gl.get_parameter_string(glow::RENDERER);
            let version = self.gl.get_parameter_string(glow::VERSION);
            format!("OpenGL - {} {} (API: {})", vendor, renderer, version)
        }
    }

    fn begin_frame(&mut self) -> anyhow::Result<()> {
        self.stats.reset();

        unsafe {
            // 清除颜色缓冲
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // 重置批处理
        self.batch_vertices.clear();
        self.batch_indices.clear();
        self.batch_texture = None;

        Ok(())
    }

    fn end_frame(&mut self) -> anyhow::Result<()> {
        // 刷新剩余的批处理
        self.flush_batch();
        Ok(())
    }

    fn present(&mut self) {
        // 交换缓冲区由窗口系统处理
    }

    fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
        unsafe {
            self.gl.clear_color(color.r, color.g, color.b, color.a);
        }
    }

    fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled;
    }

    fn set_resolution(&mut self, width: u32, height: u32) {
        self.window_size = (width, height);
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
        self.ortho_camera = OrthographicCamera::from_window(width, height, 1.0);
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.set_resolution(width, height);
    }

    fn push_transform(&mut self, matrix: Mat4) {
        self.transform_stack.push(self.current_transform);
        self.current_transform = self.current_transform * matrix;
    }

    fn pop_transform(&mut self) {
        if let Some(prev) = self.transform_stack.pop() {
            self.current_transform = prev;
        }
    }

    fn push_scissor_rect(&mut self, rect: Rect) {
        self.scissor_stack.push(rect);
        if !self.scissor_test {
            unsafe {
                self.gl.enable(glow::SCISSOR_TEST);
            }
            self.scissor_test = true;
        }
        unsafe {
            self.gl.scissor(
                rect.x as i32,
                (self.window_size.1 as f32 - rect.y - rect.height) as i32,
                rect.width as i32,
                rect.height as i32,
            );
        }
    }

    fn pop_scissor_rect(&mut self) {
        self.scissor_stack.pop();
        if self.scissor_stack.is_empty() {
            unsafe {
                self.gl.disable(glow::SCISSOR_TEST);
            }
            self.scissor_test = false;
        } else {
            let rect = *self.scissor_stack.last().unwrap();
            unsafe {
                self.gl.scissor(
                    rect.x as i32,
                    (self.window_size.1 as f32 - rect.y - rect.height) as i32,
                    rect.width as i32,
                    rect.height as i32,
                );
            }
        }
    }

    fn set_blend_mode(&mut self, mode: BlendMode) {
        self.current_blend_mode = mode;
        self.apply_blend_mode(mode);
    }

    fn reset_blend_mode(&mut self) {
        self.current_blend_mode = BlendMode::Alpha;
        self.apply_blend_mode(BlendMode::Alpha);
    }

    fn camera(&self) -> Option<&Camera2D> {
        self.camera.as_ref()
    }

    fn set_camera(&mut self, camera: Camera2D) {
        self.camera = Some(camera);
    }

    fn draw_quad(&mut self, _quad: &crate::shader::Mesh2D) {
        // 简化实现
        self.draw_rectangle(0.0, 0.0, 100.0, 100.0, Color::WHITE);
    }

    fn draw_texture(&mut self, texture: TextureHandle, x: f32, y: f32, color: Color) {
        let tex_index = texture.index();
        if let Some(_tex) = self.get_texture(&texture) {
            self.add_sprite_to_batch(x, y, 64.0, 64.0, 0.0, 0.0, 1.0, 1.0, color, Some(tex_index));
        } else {
            self.draw_rectangle(x, y, 64.0, 64.0, color);
        }
    }

    fn draw_texture_ex(&mut self, texture: TextureHandle, x: f32, y: f32, params: DrawParams) {
        self.draw_texture(texture, x, y, params.color);
    }

    fn draw_texture_pro(
        &mut self,
        texture: TextureHandle,
        source: Option<Rect>,
        dest: Rect,
        _origin: Vec2,
        _rotation: f32,
        color: Color,
    ) {
        let tex_index = texture.index();
        let (u0, v0, u1, v1) = if let Some(src) = source {
            (src.x, src.y, src.x + src.width, src.y + src.height)
        } else {
            (0.0, 0.0, 1.0, 1.0)
        };

        if let Some(_tex) = self.get_texture(&texture) {
            self.add_sprite_to_batch(
                dest.x,
                dest.y,
                dest.width,
                dest.height,
                u0,
                v0,
                u1,
                v1,
                color,
                Some(tex_index),
            );
        } else {
            self.draw_rectangle(dest.x, dest.y, dest.width, dest.height, color);
        }
    }

    fn draw_texture_rotated(
        &mut self,
        texture: TextureHandle,
        x: f32,
        y: f32,
        _angle: f32,
        color: Color,
    ) {
        self.draw_texture(texture, x, y, color);
    }

    fn draw_texture_rect(
        &mut self,
        texture: TextureHandle,
        source: Rect,
        dest: Rect,
        color: Color,
    ) {
        let tex_index = texture.index();
        if let Some(_tex) = self.get_texture(&texture) {
            self.add_sprite_to_batch(
                dest.x,
                dest.y,
                dest.width,
                dest.height,
                source.x,
                source.y,
                source.x + source.width,
                source.y + source.height,
                color,
                Some(tex_index),
            );
        } else {
            self.draw_rectangle(dest.x, dest.y, dest.width, dest.height, color);
        }
    }

    fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.add_sprite_to_batch(x, y, w, h, 0.0, 0.0, 1.0, 1.0, color, None);
    }

    fn draw_rectangle_lines(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        thickness: f32,
        color: Color,
    ) {
        let t = thickness.max(1.0);
        // 顶部
        self.draw_rectangle(x, y, w, t, color);
        // 底部
        self.draw_rectangle(x, y + h - t, w, t, color);
        // 左边
        self.draw_rectangle(x, y, t, h, color);
        // 右边
        self.draw_rectangle(x + w - t, y, t, h, color);
    }

    fn draw_rectangle_rotated(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        _angle: f32,
        color: Color,
    ) {
        self.draw_rectangle(x, y, w, h, color);
    }

    fn draw_circle(&mut self, x: f32, y: f32, r: f32, color: Color) {
        // 使用多边形近似圆形
        self.draw_poly(x, y, 32, r, 0.0, color);
    }

    fn draw_circle_lines(&mut self, x: f32, y: f32, r: f32, thickness: f32, color: Color) {
        self.draw_poly_lines(x, y, 32, r, 0.0, thickness, color);
    }

    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();
        if length < 0.001 {
            return;
        }

        // 简化实现：使用矩形
        let angle = dy.atan2(dx);
        let t = thickness.max(1.0);

        // 保存当前变换
        self.push_transform(Mat4::from_translation(Vec3::new(x1, y1, 0.0)));
        self.push_transform(Mat4::from_rotation_z(angle));

        // 绘制线段（矩形）
        self.draw_rectangle(-t / 2.0, 0.0, length, t, color);

        // 恢复变换
        self.pop_transform();
        self.pop_transform();
    }

    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        let base_index = self.batch_vertices.len() as u32;

        self.batch_vertices.extend_from_slice(&[
            SpriteVertex::new(p1.x, p1.y, 0.0, 0.0, 0.0, color),
            SpriteVertex::new(p2.x, p2.y, 0.0, 1.0, 0.0, color),
            SpriteVertex::new(p3.x, p3.y, 0.0, 0.5, 1.0, color),
        ]);

        self.batch_indices
            .extend_from_slice(&[base_index, base_index + 1, base_index + 2]);

        self.stats.add_vertices(3);
        self.stats.add_indices(3);
    }

    fn draw_triangle_lines(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, thickness: f32, color: Color) {
        self.draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color);
        self.draw_line(p2.x, p2.y, p3.x, p3.y, thickness, color);
        self.draw_line(p3.x, p3.y, p1.x, p1.y, thickness, color);
    }

    fn draw_poly(&mut self, x: f32, y: f32, sides: u32, radius: f32, _rotation: f32, color: Color) {
        let base_index = self.batch_vertices.len() as u32;

        // 中心点（用于三角形扇）
        self.batch_vertices
            .push(SpriteVertex::new(x, y, 0.0, 0.5, 0.5, color));

        for i in 0..=sides {
            let angle = (i as f32 / sides as f32) * std::f32::consts::PI * 2.0;
            let px = x + radius * angle.cos();
            let py = y + radius * angle.sin();
            let u = 0.5 + 0.5 * angle.cos();
            let v = 0.5 + 0.5 * angle.sin();
            self.batch_vertices
                .push(SpriteVertex::new(px, py, 0.0, u, v, color));
        }

        // 创建三角形扇索引
        for i in 1..=sides {
            self.batch_indices
                .extend_from_slice(&[base_index, base_index + i, base_index + i + 1]);
        }

        self.stats.add_vertices(sides + 2);
        self.stats.add_indices(sides * 3);
    }

    fn draw_poly_lines(
        &mut self,
        x: f32,
        y: f32,
        sides: u32,
        radius: f32,
        rotation: f32,
        thickness: f32,
        color: Color,
    ) {
        let mut prev_x = x + radius;
        let mut prev_y = y;

        for i in 1..=sides {
            let angle = (i as f32 / sides as f32) * std::f32::consts::PI * 2.0 + rotation;
            let curr_x = x + radius * angle.cos();
            let curr_y = y + radius * angle.sin();
            self.draw_line(prev_x, prev_y, curr_x, curr_y, thickness, color);
            prev_x = curr_x;
            prev_y = curr_y;
        }
    }

    fn flush(&mut self) {
        self.flush_batch();
    }

    fn stats(&self) -> RenderStats {
        self.stats.clone()
    }
}

impl Drop for GlRenderer {
    fn drop(&mut self) {
        unsafe {
            // 清理资源
            self.gl.delete_program(self.sprite_program);
            self.gl.delete_program(self.color_program);
            self.gl.delete_buffer(self.vertex_buffer);
            self.gl.delete_buffer(self.index_buffer);

            // 删除所有纹理
            for (&_, &tex) in self.textures.iter() {
                self.gl.delete_texture(tex);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_vertex_size() {
        // 验证顶点结构大小（用于缓冲区计算）
        assert_eq!(std::mem::size_of::<SpriteVertex>(), 36); // 3*4 + 2*4 + 4*4 = 12 + 8 + 16 = 36
    }

    #[test]
    fn test_sprite_vertex_creation() {
        let vertex = SpriteVertex::new(1.0, 2.0, 0.0, 0.5, 0.5, Color::RED);
        assert_eq!(vertex.position, [1.0, 2.0, 0.0]);
        assert_eq!(vertex.tex_coord, [0.5, 0.5]);
        assert_eq!(vertex.color, [1.0, 0.0, 0.0, 1.0]);
    }
}
