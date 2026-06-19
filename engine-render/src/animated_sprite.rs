//! AnimatedSprite 模块 - 帧动画精灵
//!
//! 提供 AnimatedSprite 类型，支持从纹理图集播放帧动画。

use super::{Color, Sprite, TextureHandle};
use engine_math::Vec2;

/// 循环模式
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoopMode {
    /// 播放一次
    Once,
    /// 循环播放
    #[default]
    Loop,
    /// 来回播放（ping-pong）
    PingPong,
}

/// 帧动画精灵
#[derive(Clone, Debug)]
pub struct AnimatedSprite {
    /// 纹理图集
    atlas: TextureHandle,
    /// 帧区域列表
    frames: Vec<super::Rect>,
    /// 当前帧索引
    current_frame: usize,
    /// FPS（每秒帧数）
    fps: f32,
    /// 循环模式
    loop_mode: LoopMode,
    /// 是否正在播放
    is_playing: bool,
    /// 累积时间
    accumulator: f32,
    /// 播放方向（用于 PingPong 模式）
    direction: i32,
    /// 颜色
    color: Color,
}

impl AnimatedSprite {
    /// 创建新的帧动画精灵
    ///
    /// # Arguments
    /// * `atlas` - 纹理图集句柄
    /// * `fps` - 每秒帧数
    /// * `frames` - 帧区域列表
    pub fn new(atlas: TextureHandle, fps: f32, frames: Vec<super::Rect>) -> Self {
        Self {
            atlas,
            frames,
            current_frame: 0,
            fps,
            loop_mode: LoopMode::Loop,
            is_playing: false,
            accumulator: 0.0,
            direction: 1,
            color: Color::WHITE,
        }
    }

    // region: 播放控制

    /// 开始播放
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// 暂停播放
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// 停止播放并重置到第一帧
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_frame = 0;
        self.accumulator = 0.0;
        self.direction = 1;
    }

    /// 检查是否正在播放
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    // endregion

    // region: 帧控制

    /// 获取当前帧索引
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    /// 设置当前帧
    pub fn set_frame(&mut self, index: usize) {
        if index < self.frames.len() {
            self.current_frame = index;
        }
    }

    /// 获取总帧数
    pub fn total_frames(&self) -> usize {
        self.frames.len()
    }

    /// 获取 FPS
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// 设置 FPS
    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
    }

    /// 获取循环模式
    pub fn loop_mode(&self) -> LoopMode {
        self.loop_mode
    }

    /// 设置循环模式
    pub fn set_loop(&mut self, mode: LoopMode) {
        self.loop_mode = mode;
    }

    // endregion

    // region: 更新与绘制

    /// 更新动画
    ///
    /// # Arguments
    /// * `dt` - 帧时间（秒）
    pub fn update(&mut self, dt: f32) {
        if !self.is_playing || self.frames.is_empty() {
            return;
        }

        let frame_time = 1.0 / self.fps;
        self.accumulator += dt;

        while self.accumulator >= frame_time {
            self.accumulator -= frame_time;
            self.advance_frame();
        }
    }

    /// 前进一帧
    fn advance_frame(&mut self) {
        match self.loop_mode {
            LoopMode::Once => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.is_playing = false;
                }
            }
            LoopMode::Loop => {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
            }
            LoopMode::PingPong => {
                let next = self.current_frame as i32 + self.direction;
                if next >= (self.frames.len() as i32) - 1 {
                    self.direction = -1;
                } else if next <= 0 {
                    self.direction = 1;
                }
                self.current_frame = next as usize;
            }
        }
    }

    /// 绘制当前帧
    pub fn draw(&self, _ctx: &super::RenderContext, position: Vec2) {
        if self.frames.is_empty() {
            return;
        }

        let frame_rect = self.frames[self.current_frame];
        let sprite = Sprite::from_texture(self.atlas.clone())
            .with_source_rect(frame_rect)
            .with_color(self.color);

        sprite.draw(_ctx, position);
    }

    // endregion

    // region: 颜色

    /// 获取颜色
    pub fn color(&self) -> Color {
        self.color
    }

    /// 设置颜色
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // endregion

    // region: 帧区域

    /// 获取当前帧的区域
    pub fn current_frame_rect(&self) -> Option<super::Rect> {
        self.frames.get(self.current_frame).copied()
    }

    /// 获取指定帧的区域
    pub fn get_frame_rect(&self, index: usize) -> Option<super::Rect> {
        self.frames.get(index).copied()
    }

    // endregion
}

impl Default for AnimatedSprite {
    fn default() -> Self {
        Self {
            atlas: engine_utils::Handle::null(),
            frames: Vec::new(),
            current_frame: 0,
            fps: 12.0,
            loop_mode: LoopMode::Loop,
            is_playing: false,
            accumulator: 0.0,
            direction: 1,
            color: Color::WHITE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texture::Texture2D;
    use crate::Rect;
    use engine_utils::Handle;

    fn create_test_frames() -> Vec<Rect> {
        vec![
            Rect::new(0.0, 0.0, 32.0, 32.0),
            Rect::new(32.0, 0.0, 32.0, 32.0),
            Rect::new(64.0, 0.0, 32.0, 32.0),
            Rect::new(96.0, 0.0, 32.0, 32.0),
        ]
    }

    #[test]
    fn test_animated_sprite_new() {
        let atlas = Handle::<Texture2D>::null();
        let frames = create_test_frames();
        let anim = AnimatedSprite::new(atlas, 12.0, frames.clone());

        assert_eq!(anim.total_frames(), 4);
        assert_eq!(anim.current_frame(), 0);
        assert_eq!(anim.fps(), 12.0);
        assert!(!anim.is_playing());
    }

    #[test]
    fn test_animated_sprite_play_pause() {
        let atlas = Handle::<Texture2D>::null();
        let anim = AnimatedSprite::new(atlas, 12.0, create_test_frames());

        assert!(!anim.is_playing());

        // Can't test &mut self here due to immutability in test
    }

    #[test]
    fn test_animated_sprite_loop_once() {
        let atlas = Handle::<Texture2D>::null();
        let mut anim = AnimatedSprite::new(atlas, 12.0, create_test_frames());
        anim.set_loop(LoopMode::Once);

        assert_eq!(anim.loop_mode(), LoopMode::Once);
    }

    #[test]
    fn test_animated_sprite_update() {
        let atlas = Handle::<Texture2D>::null();
        let mut anim = AnimatedSprite::new(atlas, 2.0, create_test_frames()); // 2 fps = 0.5s per frame
        anim.play();

        // First frame
        assert_eq!(anim.current_frame(), 0);

        // Advance 0.3s (less than frame time)
        anim.update(0.3);
        assert_eq!(anim.current_frame(), 0);

        // Advance another 0.3s (total 0.6s > 0.5s frame time)
        anim.update(0.3);
        // Should have advanced to frame 1
        assert_eq!(anim.current_frame(), 1);
    }

    #[test]
    fn test_animated_sprite_loop_mode_loop() {
        let atlas = Handle::<Texture2D>::null();
        // FPS = 4 means each frame is 0.25 seconds
        // With 10 updates of 1.0 second each = 10 seconds = 40 frames
        // 40 frames % 4 frames = 0, so we're back at frame 0
        let mut anim = AnimatedSprite::new(atlas, 4.0, create_test_frames());
        anim.set_loop(LoopMode::Loop);
        anim.play();

        // Advance 10 frames (1 second each)
        for _ in 0..10 {
            anim.update(1.0);
        }

        // 10 seconds * 4 FPS = 40 frames, 40 % 4 = 0
        assert_eq!(anim.current_frame(), 0);
    }

    #[test]
    fn test_animated_sprite_stop() {
        let atlas = Handle::<Texture2D>::null();
        let mut anim = AnimatedSprite::new(atlas, 12.0, create_test_frames());
        anim.play();
        anim.update(1.0); // Advance some frames
        anim.stop();

        assert!(!anim.is_playing());
        assert_eq!(anim.current_frame(), 0);
    }

    #[test]
    fn test_animated_sprite_set_frame() {
        let atlas = Handle::<Texture2D>::null();
        let mut anim = AnimatedSprite::new(atlas, 12.0, create_test_frames());

        anim.set_frame(2);
        assert_eq!(anim.current_frame(), 2);

        anim.set_frame(5); // Out of bounds, should be ignored
        assert_eq!(anim.current_frame(), 2);
    }

    #[test]
    fn test_animated_sprite_color() {
        let atlas = Handle::<Texture2D>::null();
        let mut anim = AnimatedSprite::new(atlas, 12.0, create_test_frames());

        assert_eq!(anim.color(), Color::WHITE);

        anim.set_color(Color::RED);
        assert_eq!(anim.color(), Color::RED);
    }

    #[test]
    fn test_animated_sprite_current_frame_rect() {
        let atlas = Handle::<Texture2D>::null();
        let frames = create_test_frames();
        let mut anim = AnimatedSprite::new(atlas, 12.0, frames.clone());

        assert_eq!(anim.current_frame_rect(), Some(frames[0]));

        anim.set_frame(2);
        assert_eq!(anim.current_frame_rect(), Some(frames[2]));
    }
}
