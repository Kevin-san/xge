//! AnimatedSprite 模块 - 帧动画精灵
//!
//! 提供 `AnimatedSprite` 类型，支持从纹理图集播放帧动画。
//!
//! 核心特性：
//! - `play()` / `pause()` / `stop()` — 播放控制
//! - `update(dt)` — 每帧调用，推进时间
//! - `set_frame(n)` / `jump_to_frame(n)` / `seek(t)` — 跳帧 / 时间跳转
//! - `restart()` — 从头播放
//! - `set_loop_mode(LoopMode)` — Once / Loop / PingPong
//! - `is_playing()` / `is_finished()` — 状态检测
//! - `set_fps(fps)` — 自定义帧率
//! - `frame_count()` / `current_frame_rect()` — 查询信息

use engine_utils::Handle;
use super::{Color, DrawParams, Sprite, TextureHandle};
use engine_math::Vec2;

/// 循环模式
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LoopMode {
    Once,
    #[default]
    Loop,
    PingPong,
}

#[derive(Clone, Debug)]
pub struct AnimatedSprite {
    atlas: TextureHandle,
    frames: Vec<super::Rect>,
    current_frame: usize,
    fps: f32,
    loop_mode: LoopMode,
    is_playing: bool,
    accumulator: f32,
    elapsed: f32,
    direction: i32,
    color: Color,
    /// 是否已结束（仅 Once 模式有意义）
    finished: bool,
}

impl AnimatedSprite {
    pub fn new(atlas: TextureHandle, fps: f32, frames: Vec<super::Rect>) -> Self {
        Self {
            atlas,
            frames,
            current_frame: 0,
            fps,
            loop_mode: LoopMode::Loop,
            is_playing: false,
            accumulator: 0.0,
            elapsed: 0.0,
            direction: 1,
            color: Color::WHITE,
            finished: false,
        }
    }

    // region: 播放控制

    pub fn play(&mut self) {
        self.is_playing = true;
        self.finished = false;
        if self.frames.is_empty() {
            self.is_playing = false;
        }
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_frame = 0;
        self.accumulator = 0.0;
        self.elapsed = 0.0;
        self.direction = 1;
        self.finished = false;
    }

    pub fn restart(&mut self) {
        self.stop();
        self.play();
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_paused(&self) -> bool {
        !self.is_playing
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    // endregion

    // region: 帧控制

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn set_frame(&mut self, index: usize) {
        if self.frames.is_empty() {
            return;
        }
        self.current_frame = index.min(self.frames.len() - 1);
        self.accumulator = 0.0;
        self.finished = false;
    }

    pub fn jump_to_frame(&mut self, index: usize) {
        self.set_frame(index);
    }

    pub fn total_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// 根据总时间跳转到某一秒（秒）
    pub fn seek(&mut self, seconds: f32) {
        if self.frames.is_empty() {
            return;
        }
        let frame_duration = 1.0 / self.fps.max(f32::EPSILON);
        let clamped = seconds.max(0.0);
        let frame_index = match self.loop_mode {
            LoopMode::Loop => (clamped / frame_duration) as usize % self.frames.len(),
            LoopMode::Once => {
                let idx = (clamped / frame_duration) as usize;
                if idx >= self.frames.len() {
                    self.finished = true;
                    self.is_playing = false;
                    self.frames.len() - 1
                } else {
                    idx
                }
            }
            LoopMode::PingPong => {
                let cycle_frames = self.frames.len().saturating_sub(1) * 2;
                if cycle_frames == 0 {
                    0
                } else {
                    let mod_idx = (clamped / frame_duration) as usize % cycle_frames;
                    if mod_idx < self.frames.len() { mod_idx } else { cycle_frames - mod_idx }
                }
            }
        };
        self.current_frame = frame_index;
        self.elapsed = clamped;
        self.accumulator = clamped - (frame_index as f32 * frame_duration);
    }

    /// 已播放时间（秒）
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// 总时长（秒）
    pub fn total_duration(&self) -> f32 {
        if self.fps <= 0.0 {
            return 0.0;
        }
        self.frames.len() as f32 / self.fps
    }

    /// 当前播放进度（0.0 ~ 1.0）
    pub fn progress(&self) -> f32 {
        let total = self.total_duration();
        if total <= 0.0 {
            0.0
        } else {
            (self.elapsed / total).min(1.0)
        }
    }

    // endregion

    // region: 速率与模式

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps.max(0.0);
    }

    pub fn loop_mode(&self) -> LoopMode {
        self.loop_mode
    }

    pub fn set_loop(&mut self, mode: LoopMode) {
        self.loop_mode = mode;
        // 重置方向，避免模式切换导致的状态混乱
        self.direction = 1;
        self.finished = false;
    }

    // endregion

    // region: 更新与绘制

    pub fn update(&mut self, dt: f32) {
        if !self.is_playing || self.frames.is_empty() || self.fps <= 0.0 {
            return;
        }
        let frame_time = 1.0 / self.fps;
        self.accumulator += dt;
        self.elapsed += dt;

        // 避免一次巨大的 dt 跳过大量帧
        let max_steps = 5;
        let mut steps = 0;
        while self.accumulator >= frame_time && steps < max_steps {
            self.accumulator -= frame_time;
            self.advance_frame();
            steps += 1;
        }
        // 若积累过多则重置到当前帧
        if steps >= max_steps && self.accumulator >= frame_time {
            self.accumulator = 0.0;
        }
    }

    fn advance_frame(&mut self) {
        match self.loop_mode {
            LoopMode::Once => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.finished = true;
                    self.is_playing = false;
                }
            }
            LoopMode::Loop => {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
            }
            LoopMode::PingPong => {
                let next = self.current_frame as i32 + self.direction;
                let last = (self.frames.len() as i32).saturating_sub(1);
                if next >= last {
                    self.current_frame = last as usize;
                    self.direction = -1;
                } else if next <= 0 {
                    self.current_frame = 0;
                    self.direction = 1;
                } else {
                    self.current_frame = next as usize;
                }
            }
        }
    }

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

    pub fn draw_ex(&self, _ctx: &super::RenderContext, position: Vec2, params: DrawParams) {
        if self.frames.is_empty() {
            return;
        }
        let frame_rect = self.frames[self.current_frame];
        let sprite = Sprite::from_texture(self.atlas.clone())
            .with_source_rect(frame_rect)
            .with_color(self.color.mul(params.color));
        sprite.draw_ex(_ctx, position, params);
    }

    // endregion

    // region: 颜色与帧区域

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn current_frame_rect(&self) -> Option<super::Rect> {
        self.frames.get(self.current_frame).copied()
    }

    pub fn get_frame_rect(&self, index: usize) -> Option<super::Rect> {
        self.frames.get(index).copied()
    }

    pub fn set_frames(&mut self, frames: Vec<super::Rect>) {
        self.frames = frames;
        if self.current_frame >= self.frames.len() {
            self.current_frame = 0;
        }
    }

    pub fn frames(&self) -> &[super::Rect] {
        &self.frames
    }

    // endregion

    // region: 纹理与大小

    pub fn atlas(&self) -> TextureHandle {
        self.atlas.clone()
    }

    pub fn current_frame_size(&self) -> Vec2 {
        if let Some(rect) = self.current_frame_rect() {
            Vec2::new(rect.width, rect.height)
        } else {
            Vec2::ZERO
        }
    }

    // endregion
}

impl Default for AnimatedSprite {
    fn default() -> Self {
        let handle = Handle::<crate::Texture2D>::null();
        Self::new(handle, 12.0, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texture::Texture2D;
    use crate::Rect;
    use engine_utils::Handle;

    fn make_frames() -> Vec<Rect> {
        vec![
            Rect::new(0.0, 0.0, 32.0, 32.0),
            Rect::new(32.0, 0.0, 32.0, 32.0),
            Rect::new(64.0, 0.0, 32.0, 32.0),
            Rect::new(96.0, 0.0, 32.0, 32.0),
        ]
    }

    #[test]
    fn test_new_and_defaults() {
        let anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, make_frames());
        assert_eq!(anim.total_frames(), 4);
        assert_eq!(anim.current_frame(), 0);
        assert_eq!(anim.fps(), 12.0);
        assert!(!anim.is_playing());
    }

    #[test]
    fn test_play_pause_stop() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        assert!(!anim.is_playing());
        anim.play();
        assert!(anim.is_playing());
        assert!(!anim.is_finished());
        anim.pause();
        assert!(!anim.is_playing());
        assert!(anim.is_paused());
        anim.stop();
        assert_eq!(anim.current_frame(), 0);
        assert_eq!(anim.elapsed(), 0.0);
    }

    #[test]
    fn test_update_once_mode_stops() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.set_loop(LoopMode::Once);
        anim.play();
        // 每个帧 0.5s，4 个帧共 2.0s
        anim.update(2.5); // 超过总时长
        assert!(!anim.is_playing());
        assert!(anim.is_finished());
    }

    #[test]
    fn test_update_loop_mode() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.set_loop(LoopMode::Loop);
        anim.play();
        anim.update(0.5); // 前进一帧
        assert_eq!(anim.current_frame(), 1);
        anim.update(1.5); // 再前进 3 帧 回到 frame 0
        assert_eq!(anim.current_frame(), 0);
    }

    #[test]
    fn test_update_pingpong() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.set_loop(LoopMode::PingPong);
        anim.play();
        anim.update(1.5); // 到第 3 帧
        assert!(anim.current_frame() >= 1 && anim.current_frame() < 4);
    }

    #[test]
    fn test_update_no_frames() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, Vec::new());
        anim.play();
        anim.update(100.0);
        // 应保持安全
        assert_eq!(anim.current_frame(), 0);
    }

    #[test]
    fn test_jump_to_frame() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, make_frames());
        anim.jump_to_frame(2);
        assert_eq!(anim.current_frame(), 2);
    }

    #[test]
    fn test_seek_to_time_loop() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.set_loop(LoopMode::Loop);
        anim.seek(1.5); // 1.5 / 0.5 = 第 3 帧
        assert_eq!(anim.current_frame(), 3);
    }

    #[test]
    fn test_seek_to_time_once() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.set_loop(LoopMode::Once);
        anim.seek(10.0); // 远超总时长
        assert!(anim.is_finished());
        assert_eq!(anim.current_frame(), 3); // 停留在最后一帧
    }

    #[test]
    fn test_restart() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.play();
        anim.update(1.5);
        anim.restart();
        assert_eq!(anim.current_frame(), 0);
        assert!(anim.is_playing());
        assert_eq!(anim.elapsed(), 0.0);
    }

    #[test]
    fn test_progress() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.seek(1.0); // 1 秒
        let total = anim.total_duration();
        assert_eq!(total, 2.0); // 4 帧 / 2fps = 2s
        assert!(anim.progress() > 0.4 && anim.progress() <= 1.0);
    }

    #[test]
    fn test_set_fps() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, make_frames());
        anim.set_fps(60.0);
        assert_eq!(anim.fps(), 60.0);
        anim.set_fps(-1.0);
        assert_eq!(anim.fps(), 0.0);
    }

    #[test]
    fn test_current_frame_rect() {
        let frames = make_frames();
        let anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, frames.clone());
        assert_eq!(anim.current_frame_rect(), Some(frames[0]));
    }

    #[test]
    fn test_color() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, make_frames());
        anim.set_color(Color::RED);
        assert_eq!(anim.color(), Color::RED);
    }

    #[test]
    fn test_set_frames() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, make_frames());
        anim.set_frame(3);
        anim.set_frames(vec![Rect::new(0.0, 0.0, 1.0, 1.0)]);
        assert_eq!(anim.current_frame(), 0);
        assert_eq!(anim.total_frames(), 1);
    }

    #[test]
    fn test_frames_accessor() {
        let frames = make_frames();
        let anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, frames.clone());
        assert_eq!(anim.frames().len(), 4);
    }

    #[test]
    fn test_current_frame_size() {
        let anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, make_frames());
        let size = anim.current_frame_size();
        assert_eq!(size.x, 32.0);
        assert_eq!(size.y, 32.0);
    }

    #[test]
    fn test_empty_anim_frame_size() {
        let anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 12.0, Vec::new());
        let size = anim.current_frame_size();
        assert_eq!(size.x, 0.0);
        assert_eq!(size.y, 0.0);
    }

    #[test]
    fn test_total_duration() {
        let anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 4.0, make_frames());
        assert_eq!(anim.total_duration(), 1.0);
    }

    #[test]
    fn test_loop_mode_change_resets_direction() {
        let mut anim = AnimatedSprite::new(Handle::<Texture2D>::null(), 2.0, make_frames());
        anim.set_loop(LoopMode::PingPong);
        anim.play();
        anim.update(2.0); // 足够让它转向
        anim.set_loop(LoopMode::Loop);
        // 切回循环模式后应重置到 1
        assert!(anim.is_playing());
        assert!(!anim.is_finished());
    }

    #[test]
    fn test_default_anim() {
        let _: AnimatedSprite = Default::default();
    }
}
