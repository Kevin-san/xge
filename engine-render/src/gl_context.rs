//! OpenGL 渲染上下文模块
//!
//! 提供与 winit 窗口集成的 OpenGL 上下文管理。

use std::error::Error;

/// OpenGL 渲染上下文
///
/// 管理 OpenGL 上下文的创建、配置和生命周期。
pub struct RenderContext {
    /// Glow OpenGL 上下文
    pub gl: glow::Context,
    /// 窗口宽度
    pub width: u32,
    /// 窗口高度
    pub height: u32,
}

impl RenderContext {
    /// 从 winit 窗口创建渲染上下文
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    pub fn from_winit_window(window: &winit::window::Window) -> Result<Self, Box<dyn Error>> {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        
        // 获取原生窗口句柄
        let handle = window.window_handle()?;
        let raw_handle = handle.as_raw();
        
        // 创建 OpenGL 上下文
        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                #[cfg(target_os = "windows")]
                {
                    windows_opengl::get_proc_address(s)
                }
                #[cfg(target_os = "linux")]
                {
                    gl_loader::get_proc_address(s)
                }
                #[cfg(target_os = "macos")]
                {
                    macos_opengl::get_proc_address(s)
                }
                #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
                {
                    panic!("Unsupported platform for OpenGL")
                }
            })
        };
        
        Ok(Self {
            gl,
            width,
            height,
        })
    }

    /// 创建 headless 渲染上下文（无窗口）
    pub fn headless(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        // 在 headless 模式下，我们无法创建真正的 OpenGL 上下文
        // 但我们可以创建一个模拟实现用于测试
        Err("Headless OpenGL context not supported. Requires display server.".into())
    }

    /// 获取上下文信息
    pub fn info(&self) -> RenderContextInfo {
        unsafe {
            RenderContextInfo {
                vendor: self.gl.get_parameter_string(glow::VENDOR),
                renderer: self.gl.get_parameter_string(glow::RENDERER),
                version: self.gl.get_parameter_string(glow::VERSION),
                shading_language_version: self.gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION),
            }
        }
    }
}

/// 渲染上下文信息
#[derive(Debug, Clone)]
pub struct RenderContextInfo {
    /// GPU 厂商
    pub vendor: String,
    /// GPU 渲染器
    pub renderer: String,
    /// OpenGL 版本
    pub version: String,
    /// GLSL 版本
    pub shading_language_version: String,
}

/// OpenGL 配置
#[derive(Debug, Clone)]
pub struct GlConfig {
    /// 抗锯齿样本数
    pub samples: u8,
    /// 垂直同步
    pub vsync: bool,
    /// 颜色位数
    pub color_bits: u8,
    /// 深度位数
    pub depth_bits: u8,
    /// 模板位数
    pub stencil_bits: u8,
    /// 双缓冲
    pub double_buffer: bool,
}

impl Default for GlConfig {
    fn default() -> Self {
        Self {
            samples: 4,
            vsync: true,
            color_bits: 32,
            depth_bits: 24,
            stencil_bits: 8,
            double_buffer: true,
        }
    }
}
