//! Advanced Rendering Example
//!
//! This example demonstrates:
//! - Complete OpenGL rendering pipeline with glow
//! - GLSL shader programs for lighting and post-processing
//! - Framebuffer objects for multi-pass rendering
//! - 3D geometry rendering with transformations
//!
//! Note: This example requires a graphics context to be initialized.
//! For a full windowed example, see the engine-core examples.

mod shaders;
mod lighting;

use glow::{HasContext, NativeFramebuffer, NativeProgram, NativeTexture, VertexArray};
use shaders::ShaderManager;
use lighting::{Light, LightingSystem};

/// Render state for demonstration
struct RenderState {
    time: f32,
    width: u32,
    height: u32,
}

/// Simple renderer demonstrating advanced rendering features
struct Renderer {
    // GL objects
    main_shader: NativeProgram,
    postprocess_shader: NativeProgram,
    cube_vao: VertexArray,
    cube_vbo: glow::Buffer,
    cube_ebo: glow::Buffer,
    plane_vao: VertexArray,
    plane_vbo: glow::Buffer,
    plane_ebo: glow::Buffer,
    quad_vao: VertexArray,
    quad_vbo: glow::Buffer,
    
    // Framebuffer for post-processing
    scene_fbo: NativeFramebuffer,
    scene_texture: NativeTexture,
    depth_buffer: NativeTexture,
    
    // Textures
    texture: NativeTexture,
    
    // Lighting
    lighting_system: LightingSystem,
}

impl Renderer {
    /// Create renderer with given GL context
    pub fn new(gl: &glow::Context, width: u32, height: u32) -> Self {
        unsafe {
            // Initialize OpenGL state
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::BLEND);
            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
            gl.depth_func(glow::LEQUAL);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.clear_color(0.05, 0.05, 0.1, 1.0);
            
            // Create shaders
            let main_shader = ShaderManager::create_main_shader(gl);
            let postprocess_shader = ShaderManager::create_postprocess_shader(gl);
            
            // Create geometry
            let (cube_vao, cube_vbo, cube_ebo) = Self::create_cube(gl);
            let (plane_vao, plane_vbo, plane_ebo) = Self::create_plane(gl);
            let (quad_vao, quad_vbo) = Self::create_quad(gl);
            
            // Create framebuffer
            let (scene_fbo, scene_texture, depth_buffer) = Self::create_framebuffer(gl, width, height);
            
            // Create texture
            let texture = Self::create_checker_texture(gl);
            
            // Setup lighting
            let mut lighting_system = LightingSystem::new(8);
            lighting_system.add_light(Light::directional_3d(
                engine_math::Vec3::new(-0.5, -1.0, 0.3).normalize(),
                engine_math::Vec4::new(1.0, 0.98, 0.95, 1.0),
                1.0,
            ));
            
            Self {
                main_shader,
                postprocess_shader,
                cube_vao,
                cube_vbo,
                cube_ebo,
                plane_vao,
                plane_vbo,
                plane_ebo,
                quad_vao,
                quad_vbo,
                scene_fbo,
                scene_texture,
                depth_buffer,
                texture,
                lighting_system,
            }
        }
    }
    
    /// Create cube geometry
    unsafe fn create_cube(gl: &glow::Context) -> (VertexArray, glow::Buffer, glow::Buffer) {
        // Vertices: position(3) + normal(3) + texcoord(2) = 8 floats per vertex
        let vertices: [f32; 192] = [
            // Front face
            -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   0.0, 0.0,
             0.5, -0.5,  0.5,   0.0,  0.0,  1.0,   1.0, 0.0,
             0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   1.0, 1.0,
            -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,   0.0, 1.0,
            // Back face
            -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   1.0, 0.0,
            -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   1.0, 1.0,
             0.5,  0.5, -0.5,   0.0,  0.0, -1.0,   0.0, 1.0,
             0.5, -0.5, -0.5,   0.0,  0.0, -1.0,   0.0, 0.0,
            // Top face
            -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   0.0, 0.0,
            -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   0.0, 1.0,
             0.5,  0.5,  0.5,   0.0,  1.0,  0.0,   1.0, 1.0,
             0.5,  0.5, -0.5,   0.0,  1.0,  0.0,   1.0, 0.0,
            // Bottom face
            -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   0.0, 1.0,
             0.5, -0.5, -0.5,   0.0, -1.0,  0.0,   1.0, 1.0,
             0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   1.0, 0.0,
            -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,   0.0, 0.0,
            // Right face
             0.5, -0.5, -0.5,   1.0,  0.0,  0.0,   0.0, 0.0,
             0.5,  0.5, -0.5,   1.0,  0.0,  0.0,   0.0, 1.0,
             0.5,  0.5,  0.5,   1.0,  0.0,  0.0,   1.0, 1.0,
             0.5, -0.5,  0.5,   1.0,  0.0,  0.0,   1.0, 0.0,
            // Left face
            -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,   1.0, 0.0,
            -0.5, -0.5,  0.5,  -1.0,  0.0,  0.0,   1.0, 1.0,
            -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,   0.0, 1.0,
            -0.5,  0.5, -0.5,  -1.0,  0.0,  0.0,   0.0, 0.0,
        ];
        
        let indices: [u32; 36] = [
            0,  1,  2,   0,  2,  3,   // Front
            4,  5,  6,   4,  6,  7,   // Back
            8,  9,  10,  8,  10, 11,  // Top
            12, 13, 14,  12, 14, 15,  // Bottom
            16, 17, 18,  16, 18, 19,  // Right
            20, 21, 22,  20, 22, 23,  // Left
        ];
        
        let vao = gl.create_vertex_array().expect("Failed to create VAO");
        let vbo = gl.create_buffer().expect("Failed to create VBO");
        let ebo = gl.create_buffer().expect("Failed to create EBO");
        
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&vertices), glow::STATIC_DRAW);
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), glow::STATIC_DRAW);
        
        // Position attribute
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 32, 0);
        // Normal attribute
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 32, 12);
        // TexCoord attribute
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 32, 24);
        
        gl.bind_vertex_array(None);
        
        (vao, vbo, ebo)
    }
    
    /// Create plane geometry
    unsafe fn create_plane(gl: &glow::Context) -> (VertexArray, glow::Buffer, glow::Buffer) {
        let vertices: [f32; 32] = [
            -5.0, 0.0, -5.0,   0.0, 1.0, 0.0,   0.0, 5.0,
             5.0, 0.0, -5.0,   0.0, 1.0, 0.0,   5.0, 5.0,
             5.0, 0.0,  5.0,   0.0, 1.0, 0.0,   5.0, 0.0,
            -5.0, 0.0,  5.0,   0.0, 1.0, 0.0,   0.0, 0.0,
        ];
        let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
        
        let vao = gl.create_vertex_array().expect("Failed to create VAO");
        let vbo = gl.create_buffer().expect("Failed to create VBO");
        let ebo = gl.create_buffer().expect("Failed to create EBO");
        
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&vertices), glow::STATIC_DRAW);
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices), glow::STATIC_DRAW);
        
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 32, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 32, 12);
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 32, 24);
        
        gl.bind_vertex_array(None);
        
        (vao, vbo, ebo)
    }
    
    /// Create fullscreen quad
    unsafe fn create_quad(gl: &glow::Context) -> (VertexArray, glow::Buffer) {
        let vertices: [f32; 16] = [
            -1.0, -1.0, 0.0, 0.0,
             1.0, -1.0, 1.0, 0.0,
             1.0,  1.0, 1.0, 1.0,
            -1.0,  1.0, 0.0, 1.0,
        ];
        
        let vao = gl.create_vertex_array().expect("Failed to create VAO");
        let vbo = gl.create_buffer().expect("Failed to create VBO");
        
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&vertices), glow::STATIC_DRAW);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 16, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 16, 8);
        gl.bind_vertex_array(None);
        
        (vao, vbo)
    }
    
    /// Create framebuffer with color and depth textures
    unsafe fn create_framebuffer(gl: &glow::Context, width: u32, height: u32) -> (NativeFramebuffer, NativeTexture, NativeTexture) {
        // Color texture
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA8 as i32, width as i32, height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, None);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        
        // Depth texture
        let depth = gl.create_texture().expect("Failed to create depth texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(depth));
        gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::DEPTH_COMPONENT24 as i32, width as i32, height as i32, 0, glow::DEPTH_COMPONENT, glow::UNSIGNED_INT, None);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        
        // Framebuffer
        let fbo = gl.create_framebuffer().expect("Failed to create FBO");
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
        gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(texture), 0);
        gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::DEPTH_ATTACHMENT, glow::TEXTURE_2D, Some(depth), 0);
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        
        (fbo, texture, depth)
    }
    
    /// Create checker pattern texture
    unsafe fn create_checker_texture(gl: &glow::Context) -> NativeTexture {
        let size = 256;
        let mut data = Vec::with_capacity((size * size * 4) as usize);
        
        for y in 0..size {
            for x in 0..size {
                let checker = ((x / 32) + (y / 32)) % 2 == 0;
                let color = if checker { [180u8, 180u8, 180u8, 255] } else { [80u8, 80u8, 80u8, 255] };
                data.extend_from_slice(&color);
            }
        }
        
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA8 as i32, size as i32, size as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(&data));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
        gl.generate_mipmap(glow::TEXTURE_2D);
        
        texture
    }
    
    /// Render a frame
    pub fn render(&mut self, gl: &glow::Context, state: &RenderState) {
        unsafe {
            // Render scene to framebuffer
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.scene_fbo));
            gl.viewport(0, 0, state.width as i32, state.height as i32);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            
            // Setup camera
            let aspect = state.width as f32 / state.height as f32;
            let proj = Self::perspective(45.0, aspect, 0.1, 100.0);
            let cam_angle = state.time * 0.5;
            let cam_dist = 10.0;
            let view = Self::look_at(
                engine_math::Vec3::new(cam_dist * cam_angle.sin(), 4.0, cam_dist * cam_angle.cos()),
                engine_math::Vec3::new(0.0, 0.0, 0.0),
                engine_math::Vec3::new(0.0, 1.0, 0.0),
            );
            
            // Draw ground plane
            self.draw_plane(gl, &proj, &view);
            
            // Draw animated cubes
            self.draw_cubes(gl, &proj, &view, state.time);
            
            // Post-processing pass
            self.apply_postprocess(gl, state.time);
            
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }
    
    unsafe fn draw_plane(&self, gl: &glow::Context, proj: &engine_math::Mat4, view: &engine_math::Mat4) {
        gl.use_program(Some(self.main_shader));
        
        let model = engine_math::Mat4::IDENTITY;
        Self::set_mat4(gl, self.main_shader, "uProjection", proj);
        Self::set_mat4(gl, self.main_shader, "uView", view);
        Self::set_mat4(gl, self.main_shader, "uModel", &model);
        
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        Self::set_int(gl, self.main_shader, "uTexture", 0);
        
        gl.bind_vertex_array(Some(self.plane_vao));
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.plane_ebo));
        gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    }
    
    unsafe fn draw_cubes(&self, gl: &glow::Context, proj: &engine_math::Mat4, view: &engine_math::Mat4, time: f32) {
        gl.use_program(Some(self.main_shader));
        
        for i in 0..4 {
            for j in 0..4 {
                let x = (i as f32 - 1.5) * 1.8;
                let z = (j as f32 - 1.5) * 1.8;
                let y = (time * 0.5 + (i + j) as f32 * 0.3).sin() * 0.5 + 1.2;
                
                let scale = 0.5 + (time + (i * 3 + j * 7) as f32 * 0.2 * 0.3).sin() * 0.3;
                let rot_x = (time + i as f32) * 0.5;
                let rot_y = (time + j as f32) * 0.7;
                let rot_z = (time + (i + j) as f32) * 0.3;
                
                let mut model = engine_math::Mat4::IDENTITY;
                model = model * engine_math::Mat4::from_translation(engine_math::Vec3::new(x, y, z));
                model = model * engine_math::Mat4::from_rotation_x(rot_x);
                model = model * engine_math::Mat4::from_rotation_y(rot_y);
                model = model * engine_math::Mat4::from_rotation_z(rot_z);
                model = model * engine_math::Mat4::from_scale(engine_math::Vec3::new(scale, scale, scale));
                
                Self::set_mat4(gl, self.main_shader, "uProjection", proj);
                Self::set_mat4(gl, self.main_shader, "uView", view);
                Self::set_mat4(gl, self.main_shader, "uModel", &model);
                
                gl.active_texture(glow::TEXTURE0);
                gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
                Self::set_int(gl, self.main_shader, "uTexture", 0);
                
                gl.bind_vertex_array(Some(self.cube_vao));
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.cube_ebo));
                gl.draw_elements(glow::TRIANGLES, 36, glow::UNSIGNED_INT, 0);
            }
        }
    }
    
    unsafe fn apply_postprocess(&self, gl: &glow::Context, time: f32) {
        gl.use_program(Some(self.postprocess_shader));
        
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.scene_texture));
        Self::set_int(gl, self.postprocess_shader, "uScene", 0);
        Self::set_float(gl, self.postprocess_shader, "uTime", time);
        
        gl.bind_vertex_array(Some(self.quad_vao));
        gl.draw_arrays(glow::TRIANGLE_FAN, 0, 4);
    }
    
    /// Create perspective projection matrix
    fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> engine_math::Mat4 {
        let f = 1.0 / (fov * std::f32::consts::PI / 360.0).tan();
        let nf = 1.0 / (near - far);
        
        engine_math::Mat4 {
            cols: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) * nf, -1.0],
                [0.0, 0.0, 2.0 * far * near * nf, 0.0],
            ],
        }
    }
    
    /// Create look-at view matrix
    fn look_at(eye: engine_math::Vec3, target: engine_math::Vec3, up: engine_math::Vec3) -> engine_math::Mat4 {
        let z = (eye - target).normalize();
        let x = up.cross(z).normalize();
        let y = z.cross(x);
        
        engine_math::Mat4 {
            cols: [
                [x.x, y.x, z.x, 0.0],
                [x.y, y.y, z.y, 0.0],
                [x.z, y.z, z.z, 0.0],
                [-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0],
            ],
        }
    }
    
    unsafe fn set_mat4(gl: &glow::Context, program: NativeProgram, name: &str, mat: &engine_math::Mat4) {
        if let Some(loc) = gl.get_uniform_location(program, name) {
            let data = mat.to_cols_array();
            gl.uniform_matrix_4_f32_slice(Some(&loc), false, &data);
        }
    }
    
    unsafe fn set_int(gl: &glow::Context, program: NativeProgram, name: &str, val: i32) {
        if let Some(loc) = gl.get_uniform_location(program, name) {
            gl.uniform_1_i32(Some(&loc), val);
        }
    }
    
    unsafe fn set_float(gl: &glow::Context, program: NativeProgram, name: &str, val: f32) {
        if let Some(loc) = gl.get_uniform_location(program, name) {
            gl.uniform_1_f32(Some(&loc), val);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            // Note: In real code, you would delete all GL objects here
            // Skipping for brevity as the GL context lifetime is managed externally
        }
    }
}

fn main() {
    println!("Advanced Rendering Example");
    println!("========================");
    println!();
    println!("This example demonstrates:");
    println!("- GLSL shaders with Phong lighting and post-processing effects");
    println!("- Framebuffer objects for multi-pass rendering");
    println!("- 3D geometry with transformations");
    println!("- Animated cubes with dynamic lighting");
    println!();
    println!("To run this example with a window, see engine-core examples.");
    println!("The Renderer struct can be initialized with an OpenGL context.");
}
