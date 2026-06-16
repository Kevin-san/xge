//! engine-render crate - 2D 渲染核心（精灵 / 纹理 / 批处理 / 图集）
//!
//! 提供 2D 渲染所需的核心类型，包括 Renderer trait、Texture、Image、Sprite、SpriteBatch、
//! TextureAtlas、正交相机等。本 crate 默认使用 OpenGL (glow) 后端。

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::collapsible_if)]
//!
//! # 核心模块
//!
//! - [`renderer`] - Renderer trait 与 RenderContext 全局渲染上下文
//! - [`texture`] - Texture2D 纹理与 Sampler 采样器
//! - [`image`] - Image CPU 端像素数据
//! - [`sprite`] - Sprite 精灵结构
//! - [`sprite_batch`] - SpriteBatch 高效批渲染
//! - [`animated_sprite`] - AnimatedSprite 帧动画
//! - [`texture_atlas`] - TextureAtlas 图集打包（Skyline/Guillotine）
//! - [`camera`] - OrthographicCamera / Camera2D 正交相机
//! - [`color`] - Color RGBA 颜色与 BlendMode 混合模式
//! - [`draw_params`] - DrawParams 绘制参数
//! - [`shader`] - Shader / Pipeline / Buffer / BindGroup 着色器抽象
//! - [`debug_renderer`] - DebugRenderer 调试图形
//! - [`render_stats`] - RenderStats 渲染统计

// Re-export commonly used types
pub use animated_sprite::{AnimatedSprite, LoopMode};
pub use camera::{Camera2D, OrthographicCamera, View, Viewport};
pub use color::Color;
pub use draw_params::{BlendMode, DrawParams};
pub use image::Image;
pub use render_stats::RenderStats;
pub use renderer::{RenderContext, Renderer};
pub use sprite::Rect;
pub use sprite::Sprite;
pub use sprite_batch::SpriteBatch;
pub use texture::{FilterMode, Sampler, Texture2D, TextureFormat, TextureHandle, WrapMode};
pub use texture_atlas::{PackAlgorithm, PackResult, TextureAtlas, TextureAtlasBuilder};

// Module declarations
mod animated_sprite;
mod camera;
mod color;
mod debug_renderer;
mod draw_params;
mod image;
mod render_stats;
mod renderer;
mod shader;
mod sprite;
mod sprite_batch;
mod texture;
mod texture_atlas;

// Optional OpenGL backend
#[cfg(feature = "gl")]
mod gl_backend;

// Full OpenGL implementation
#[cfg(feature = "gl")]
pub mod opengl;
