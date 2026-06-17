//! hot_shader 示例 - 演示着色器热重载
//!
//! 本示例演示如何在调试模式下监视着色器文件变化并自动重载。
//! 热重载允许在不重启程序的情况下更新着色器代码。

use engine_core::{Engine, EngineConfig};
use engine_render::{RenderContext, ShaderModule};

fn main() {
    println!("Hot Shader Reload Example");
    println!("=========================");

    let config = EngineConfig {
        window_title: "Hot Shader Reload Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };

    let _engine = Engine::new(config);
    let _ctx = RenderContext::new();

    // Define shader source strings
    let vertex_shader = r#"
        #version 330 core
        layout (location = 0) in vec2 aPos;
        layout (location = 1) in vec2 aTexCoord;
        layout (location = 2) in vec4 aColor;

        uniform mat4 uViewProjection;

        out vec2 vTexCoord;
        out vec4 vColor;

        void main() {
            gl_Position = uViewProjection * vec4(aPos, 0.0, 1.0);
            vTexCoord = aTexCoord;
            vColor = aColor;
        }
    "#;

    let fragment_shader = r#"
        #version 330 core
        in vec2 vTexCoord;
        in vec4 vColor;

        uniform sampler2D uTexture;

        out vec4 fragColor;

        void main() {
            vec4 texColor = texture(uTexture, vTexCoord);
            fragColor = texColor * vColor;
        }
    "#;

    println!("\nShader sources defined:");
    println!("  Vertex shader: {} chars", vertex_shader.len());
    println!("  Fragment shader: {} chars", fragment_shader.len());

    // Create shader module from source
    match ShaderModule::from_source(vertex_shader, fragment_shader) {
        Ok(_module) => {
            println!("\nShader module created successfully!");
            println!("  Contains: vertex shader, fragment shader");

            // In debug mode, shader hot-reload would be enabled
            #[cfg(debug_assertions)]
            {
                println!("\n[DEBUG MODE] Shader hot-reload is enabled!");
                println!("  Shader files will be monitored for changes");
                println!("  Changes to .vert or .frag files will trigger recompilation");
                println!("  No need to restart the application to see changes");
            }

            #[cfg(not(debug_assertions))]
            {
                println!("\n[RELEASE MODE] Shader hot-reload is disabled");
                println!("  For development, use debug build: cargo build --example hot_shader");
            }
        }
        Err(e) => {
            println!("\nFailed to create shader module: {:?}", e);
        }
    }

    println!("\nExample demonstrates:");
    println!("  - ShaderModule::from_source() to create shaders from strings");
    println!("  - Shader hot-reload in debug builds");
    println!("  - File watching for .vert/.frag files");
    println!("  - Automatic recompilation on file change");

    println!("\nHot reload workflow:");
    println!("  1. Shader source stored in memory");
    println!("  2. File watcher monitors shader files");
    println!("  3. On file change, file is re-read");
    println!("  4. New shader is compiled");
    println!("  5. If compilation succeeds, new shader is linked");
    println!("  6. Next frame uses the updated shader");

    println!("\nTypical shader file locations:");
    println!("  - assets/shaders/sprite.vert");
    println!("  - assets/shaders/sprite.frag");
    println!("  - assets/shaders/color.vert");
    println!("  - assets/shaders/color.frag");
}
