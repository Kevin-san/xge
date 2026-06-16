//! OpenGL 后端模块
//!
//! 提供 engine-render-3d 的 OpenGL (glow) 后端实现。

#[cfg(feature = "gl")]
use glow::HasContext;

#[cfg(feature = "gl")]
use crate::mesh::Vertex3D;
#[cfg(feature = "gl")]
use crate::geometry::AABB as GeoAABB;
#[cfg(feature = "gl")]
use engine_math::Vec3;

/// GPU 顶点缓冲
#[cfg(feature = "gl")]
pub struct GLVertexBuffer {
    vbo: glow::Buffer,
    vertex_count: usize,
}

/// GPU 索引缓冲
#[cfg(feature = "gl")]
pub struct GLIndexBuffer {
    ebo: glow::Buffer,
    index_count: usize,
}

/// GPU 网格
#[cfg(feature = "gl")]
pub struct GLMesh3D {
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,
    vertex_count: usize,
    index_count: usize,
    aabb: GeoAABB,
}

#[cfg(feature = "gl")]
impl GLMesh3D {
    /// 从 Mesh3D 创建 GL 网格
    pub unsafe fn from_mesh3d(gl: &glow::Context, mesh: &crate::mesh::Mesh3D) -> Self {
        let vertices = mesh.vertices_array();
        let indices = mesh.indices_array();

        let vao = gl.create_vertex_array().expect("Failed to create VAO");
        let vbo = gl.create_buffer().expect("Failed to create VBO");
        let ebo = gl.create_buffer().expect("Failed to create EBO");

        gl.bind_vertex_array(Some(vao));

        // 上传顶点数据
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        let vertex_ptr = vertices.as_ptr() as *const u8;
        let vertex_len = vertices.len() * std::mem::size_of::<Vertex3D>();
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, std::slice::from_raw_parts(vertex_ptr, vertex_len), glow::STATIC_DRAW);

        // 上传索引数据
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        let index_ptr = indices.as_ptr() as *const u8;
        let index_len = indices.len() * std::mem::size_of::<u32>();
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, std::slice::from_raw_parts(index_ptr, index_len), glow::STATIC_DRAW);

        // 设置顶点属性
        // Position (3 floats, offset 0)
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 32, 0);

        // Normal (3 floats, offset 12)
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 32, 12);

        // TexCoord (2 floats, offset 24)
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 32, 24);

        // Tangent (4 floats, offset 32) - 注意 glow 不直接支持 i32/offset，需要处理
        // 这里简化处理，实际应该检查顶点格式

        gl.bind_vertex_array(None);

        Self {
            vao,
            vbo,
            ebo,
            vertex_count: vertices.len(),
            index_count: indices.len(),
            aabb: mesh.aabb(),
        }
    }

    /// 绑定网格
    pub unsafe fn bind(&self, gl: &glow::Context) {
        gl.bind_vertex_array(Some(self.vao));
    }

    /// 绘制网格
    pub unsafe fn draw(&self, gl: &glow::Context) {
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_elements(glow::TRIANGLES, self.index_count as i32, glow::UNSIGNED_INT, 0);
    }

    /// 获取包围盒
    pub fn aabb(&self) -> GeoAABB {
        self.aabb
    }

    /// 销毁网格
    pub unsafe fn destroy(&self, gl: &glow::Context) {
        gl.delete_vertex_array(self.vao);
        gl.delete_buffer(self.vbo);
        gl.delete_buffer(self.ebo);
    }
}

/// OpenGL 材质
#[cfg(feature = "gl")]
pub struct GLMaterial3D {
    program: glow::Program,
}

/// OpenGL 着色器编译器
#[cfg(feature = "gl")]
pub struct GLShaderCompiler;

#[cfg(feature = "gl")]
impl GLShaderCompiler {
    /// 编译着色器
    pub unsafe fn compile_shader(gl: &glow::Context, source: &str, shader_type: u32) -> Result<glow::Shader, String> {
        let shader = gl.create_shader(shader_type)
            .map_err(|e| format!("Failed to create shader: {:?}", e))?;

        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            let error = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);
            return Err(error);
        }

        Ok(shader)
    }

    /// 链接程序
    pub unsafe fn link_program(gl: &glow::Context, vertex_shader: glow::Shader, fragment_shader: glow::Shader) -> Result<glow::Program, String> {
        let program = gl.create_program()
            .map_err(|e| format!("Failed to create program: {:?}", e))?;

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            let error = gl.get_program_info_log(program);
            gl.delete_program(program);
            return Err(error);
        }

        Ok(program)
    }

    /// 创建完整着色器程序
    pub unsafe fn create_program(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Result<glow::Program, String> {
        let vertex_shader = Self::compile_shader(gl, vertex_src, glow::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(gl, fragment_src, glow::FRAGMENT_SHADER)?;
        Self::link_program(gl, vertex_shader, fragment_shader)
    }
}

/// 默认 3D 着色器
#[cfg(feature = "gl")]
pub mod shaders {
    /// 默认顶点着色器
    pub const BASIC_VERT: &str = r#"
        #version 330 core

        layout(location = 0) in vec3 aPosition;
        layout(location = 1) in vec3 aNormal;
        layout(location = 2) in vec2 aTexCoord;

        uniform mat4 uModel;
        uniform mat4 uView;
        uniform mat4 uProjection;

        out vec3 vPosition;
        out vec3 vNormal;
        out vec2 vTexCoord;

        void main() {
            vPosition = (uModel * vec4(aPosition, 1.0)).xyz;
            vNormal = mat3(uModel) * aNormal;
            vTexCoord = aTexCoord;
            gl_Position = uProjection * uView * uModel * vec4(aPosition, 1.0);
        }
    "#;

    /// 默认片段着色器（Phong 光照）
    pub const BASIC_FRAG: &str = r#"
        #version 330 core

        in vec3 vPosition;
        in vec3 vNormal;
        in vec2 vTexCoord;

        uniform vec3 uViewPos;
        uniform vec4 uColor;
        uniform sampler2D uTexture;
        uniform float uShininess;

        // Light parameters
        uniform vec3 uLightDir;
        uniform vec3 uLightColor;
        uniform float uLightIntensity;

        out vec4 fragColor;

        void main() {
            vec3 normal = normalize(vNormal);
            vec3 lightDir = normalize(uLightDir);
            vec3 viewDir = normalize(uViewPos - vPosition);

            // Ambient
            vec3 ambient = 0.1 * uLightColor;

            // Diffuse
            float diff = max(dot(normal, lightDir), 0.0);
            vec3 diffuse = diff * uLightColor;

            // Specular (Blinn-Phong)
            vec3 halfwayDir = normalize(lightDir + viewDir);
            float spec = pow(max(dot(normal, halfwayDir), 0.0), uShininess);
            vec3 specular = spec * uLightColor;

            vec3 result = (ambient + diffuse + specular) * uColor.rgb;
            fragColor = vec4(result, uColor.a);
        }
    "#;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "gl")]
    #[test]
    fn test_vertex_size() {
        use std::mem::size_of;
        use crate::mesh::Vertex3D;
        // Vertex3D should be 32 bytes: 12 (pos) + 12 (normal) + 8 (texcoord) + 16 (tangent) = 48
        // But we only use position + normal + texcoord in basic shader (32 bytes)
        assert_eq!(size_of::<Vertex3D>(), 48);
    }
}
