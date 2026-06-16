//! Shaders module - GLSL shader programs for advanced rendering
//!
//! Contains:
//! - Main rendering shader with PBR-like lighting
//! - Shadow mapping shader
//! - Skybox shader
//! - Post-processing shader

use glow::{HasContext, NativeProgram, NativeShader};

/// Shader manager for creating and compiling GLSL shaders
pub struct ShaderManager;

impl ShaderManager {
    /// Create main rendering shader with Phong/PBR-like lighting
    pub fn create_main_shader(gl: &glow::Context) -> NativeProgram {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, MAIN_VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, MAIN_FRAGMENT_SHADER);
            
            let program = Self::link_program(gl, vertex_shader, fragment_shader);
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            program
        }
    }
    
    /// Create shadow mapping shader
    pub fn create_shadow_shader(gl: &glow::Context) -> NativeProgram {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, SHADOW_VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, SHADOW_FRAGMENT_SHADER);
            
            let program = Self::link_program(gl, vertex_shader, fragment_shader);
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            program
        }
    }
    
    /// Create lighting pass shader
    pub fn create_lighting_shader(gl: &glow::Context) -> NativeProgram {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, QUAD_VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, LIGHTING_FRAGMENT_SHADER);
            
            let program = Self::link_program(gl, vertex_shader, fragment_shader);
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            program
        }
    }
    
    /// Create skybox shader
    pub fn create_skybox_shader(gl: &glow::Context) -> NativeProgram {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, SKYBOX_VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, SKYBOX_FRAGMENT_SHADER);
            
            let program = Self::link_program(gl, vertex_shader, fragment_shader);
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            program
        }
    }
    
    /// Create post-processing shader
    pub fn create_postprocess_shader(gl: &glow::Context) -> NativeProgram {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, QUAD_VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, POSTPROCESS_FRAGMENT_SHADER);
            
            let program = Self::link_program(gl, vertex_shader, fragment_shader);
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            program
        }
    }
    
    unsafe fn compile_shader(gl: &glow::Context, shader_type: u32, source: &str) -> NativeShader {
        let shader = gl.create_shader(shader_type).expect("Failed to create shader");
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        
        if !gl.get_shader_compile_status(shader) {
            let log = gl.get_shader_info_log(shader);
            panic!("Shader compile error: {}", log);
        }
        
        shader
    }
    
    unsafe fn link_program(gl: &glow::Context, vertex: NativeShader, fragment: NativeShader) -> NativeProgram {
        let program = gl.create_program().expect("Failed to create program");
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);
        
        if !gl.get_program_link_status(program) {
            let log = gl.get_program_info_log(program);
            panic!("Program link error: {}", log);
        }
        
        program
    }
}

// Main rendering shader
const MAIN_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

out vec3 vPosition;
out vec3 vNormal;
out vec2 vTexCoord;

uniform mat4 uProjection;
uniform mat4 uView;
uniform mat4 uModel;

void main() {
    vec4 worldPos = uModel * vec4(aPosition, 1.0);
    vPosition = worldPos.xyz;
    vNormal = mat3(transpose(inverse(uModel))) * aNormal;
    vTexCoord = aTexCoord;
    
    gl_Position = uProjection * uView * worldPos;
}
"#;

const MAIN_FRAGMENT_SHADER: &str = r#"
#version 330 core

in vec3 vPosition;
in vec3 vNormal;
in vec2 vTexCoord;

out vec4 fragColor;

uniform vec3 uLightDir;
uniform vec3 uLightColor;
uniform vec3 uAmbientColor;
uniform float uShininess;
uniform sampler2D uTexture;

void main() {
    // Sample texture
    vec4 texColor = texture(uTexture, vTexCoord);
    
    // Normal
    vec3 normal = normalize(vNormal);
    
    // View direction
    vec3 viewDir = normalize(-vPosition);
    
    // Ambient
    vec3 ambient = uAmbientColor * texColor.rgb;
    
    // Diffuse (Lambertian)
    vec3 lightDir = normalize(uLightDir);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * uLightColor * texColor.rgb;
    
    // Specular (Blinn-Phong)
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(normal, halfwayDir), 0.0), uShininess);
    vec3 specular = spec * uLightColor * 0.5;
    
    // Final color
    vec3 color = ambient + diffuse + specular;
    
    // Tone mapping (Reinhard)
    color = color / (color + vec3(1.0));
    
    // Gamma correction
    color = pow(color, vec3(1.0 / 2.2));
    
    fragColor = vec4(color, texColor.a);
}
"#;

// Shadow mapping shader
const SHADOW_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec3 aPosition;

uniform mat4 uLightSpaceMatrix;
uniform mat4 uModel;

void main() {
    gl_Position = uLightSpaceMatrix * uModel * vec4(aPosition, 1.0);
}
"#;

const SHADOW_FRAGMENT_SHADER: &str = r#"
#version 330 core

out vec4 fragColor;

void main() {
    // Just output depth for shadow map
    gl_FragDepth = gl_FragCoord.z;
}
"#;

// Lighting pass shader (for deferred rendering)
const QUAD_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;

out vec2 vTexCoord;

void main() {
    vTexCoord = aTexCoord;
    gl_Position = vec4(aPosition, 0.0, 1.0);
}
"#;

const LIGHTING_FRAGMENT_SHADER: &str = r#"
#version 330 core

in vec2 vTexCoord;
out vec4 fragColor;

uniform sampler2D uPositionTex;
uniform sampler2D uNormalTex;
uniform sampler2D uAlbedoSpecTex;
uniform vec3 uCameraPos;
uniform vec3 uLightPos;
uniform vec3 uLightColor;

const float uShininess = 32.0;

void main() {
    // Sample G-buffer
    vec3 position = texture(uPositionTex, vTexCoord).rgb;
    vec3 normal = texture(uNormalTex, vTexCoord).rgb;
    vec4 albedoSpec = texture(uAlbedoSpecTex, vTexCoord);
    
    vec3 albedo = albedoSpec.rgb;
    float specular = albedoSpec.a;
    
    // View direction
    vec3 viewDir = normalize(uCameraPos - position);
    
    // Ambient
    vec3 ambient = vec3(0.03) * albedo;
    
    // Diffuse
    vec3 lightDir = normalize(uLightPos - position);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * uLightColor * albedo;
    
    // Specular (Blinn-Phong)
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(normal, halfwayDir), 0.0), uShininess);
    vec3 specular = spec * uLightColor * specular;
    
    // Final
    vec3 color = ambient + diffuse + specular;
    
    // HDR tonemapping
    color = color / (color + vec3(1.0));
    
    // Gamma
    color = pow(color, vec3(1.0 / 2.2));
    
    fragColor = vec4(color, 1.0);
}
"#;

// Skybox shader
const SKYBOX_VERTEX_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec3 aPosition;

out vec3 vPosition;

uniform mat4 uView;
uniform mat4 uProjection;

void main() {
    // Remove translation from view matrix for skybox
    mat4 viewNoTranslation = mat4(mat3(uView));
    vec4 pos = uProjection * viewNoTranslation * vec4(aPosition, 1.0);
    gl_Position = pos.xyww;
    vPosition = aPosition;
}
"#;

const SKYBOX_FRAGMENT_SHADER: &str = r#"
#version 330 core

in vec3 vPosition;
out vec4 fragColor;

uniform samplerCube uSkybox;
uniform float uTime;

vec3 calcSkyColor(vec3 dir) {
    // Gradient sky
    vec3 horizon = vec3(0.4, 0.5, 0.7);
    vec3 zenith = vec3(0.1, 0.2, 0.5);
    vec3 ground = vec3(0.15, 0.12, 0.1);
    
    float y = dir.y;
    vec3 skyColor;
    
    if (y > 0.0) {
        skyColor = mix(horizon, zenith, pow(y, 0.5));
    } else {
        skyColor = mix(horizon, ground, pow(-y, 0.5));
    }
    
    // Add sun
    vec3 sunDir = normalize(vec3(sin(uTime * 0.1), 0.3, cos(uTime * 0.1)));
    float sunIntensity = pow(max(dot(dir, sunDir), 0.0), 256.0);
    vec3 sunColor = vec3(1.0, 0.9, 0.7);
    
    skyColor += sunIntensity * sunColor * 2.0;
    
    return skyColor;
}

void main() {
    vec3 color = calcSkyColor(normalize(vPosition));
    fragColor = vec4(color, 1.0);
}
"#;

// Post-processing shader
const POSTPROCESS_FRAGMENT_SHADER: &str = r#"
#version 330 core

in vec2 vTexCoord;
out vec4 fragColor;

uniform sampler2D uScene;
uniform float uTime;

// Vignette effect
float vignette(vec2 uv, float intensity, float radius) {
    vec2 center = uv - 0.5;
    float dist = length(center);
    return 1.0 - smoothstep(radius * 0.5, radius, dist) * intensity;
}

// Chromatic aberration
vec3 chromaticAberration(sampler2D tex, vec2 uv, float offset) {
    vec2 dir = (uv - 0.5) * offset;
    float r = texture(tex, uv + dir).r;
    float g = texture(tex, uv).g;
    float b = texture(tex, uv - dir).b;
    return vec3(r, g, b);
}

// Simple bloom extraction
vec3 bloomExtract(vec3 color, float threshold) {
    float brightness = dot(color, vec3(0.2126, 0.7152, 0.0722));
    if (brightness > threshold) {
        return color;
    }
    return vec3(0.0);
}

void main() {
    vec2 uv = vTexCoord;
    
    // Chromatic aberration
    vec3 color = chromaticAberration(uScene, uv, 0.003);
    
    // Apply vignette
    float vig = vignette(uv, 0.4, 0.9);
    color *= vig;
    
    // Subtle bloom glow
    vec3 bloom = bloomExtract(color, 0.8) * 0.3;
    color += bloom;
    
    // Color grading (warm tint)
    color.r *= 1.05;
    color.b *= 0.95;
    
    // Clamp
    color = clamp(color, 0.0, 1.0);
    
    fragColor = vec4(color, 1.0);
}
"#;
